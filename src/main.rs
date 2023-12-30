use notchka::*;
use clap::{Parser, Subcommand};

#[derive(Subcommand)]
enum Commands {
    Build {
        #[arg(long)]
        prefix: Option<String>,
    },
    Dev {
        #[arg(short, long)]
        port: Option<u16>,
    },
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    path: Option<String>,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    use Commands::*;
    let cli = Cli::parse();

    match cli.command {
        Build { prefix } => {
            let _ = std::fs::remove_dir_all(OUT);
            let files = walk_dir("content");

            for file in &files {
                let tree = html::make_tree(prefix.as_deref().unwrap_or(""), &files);
                let path = std::path::Path::new(file);
                let content = std::fs::read_to_string(&file)?;
                let now = std::time::Instant::now();
                let (content, meta) = md::parse_meta(&content);
                let page = match md::parse_markdown(&content, meta.unwrap_or_default()) {
                    Ok(md) => html::make_page(md, prefix.as_deref(), &tree),
                    Err(err) => {
                        logging::error_compiled(&path, Box::new(&err));
                        continue;
                    },
                };
                crate::logging::info_compiled(&path, &now);

                let out = std::path::Path::new(OUT).join(file.replace("content/", ""));
                let out = out.parent().unwrap();
                std::fs::create_dir_all(out)?;
                std::fs::write(out.join(format!("{}.html", path.file_stem().unwrap().to_string_lossy().into_owned())), page)?;
            }

            for file in walk_dir("style") {
                let path = std::path::Path::new(&file);
                let now = std::time::Instant::now();
                match sass::compile_scss(&std::path::Path::new(&file)) {
                    Ok(result) => {
                        crate::logging::info_compiled(&path, &now);
                        let out = std::path::Path::new(OUT).join("dist").join(&file);
                        let out = out.parent().unwrap();
                        std::fs::create_dir_all(out)?;
                        std::fs::write(out.join(format!("{}.css", path.file_stem().unwrap().to_string_lossy().to_owned())), result)?;
                    },
                    Err(err) => logging::error_compiled(&path, err),
                };
            }

            for file in walk_dir("script") {
                let out = std::path::Path::new(OUT).join("dist").join(&file);
                std::fs::create_dir_all(out.parent().unwrap())?;
                std::fs::write(out, std::fs::read_to_string(&file)?)?;
            }

            for file in walk_dir("dist") {
                let out = std::path::Path::new(OUT).join(&file);
                std::fs::create_dir_all(out.parent().unwrap())?;
                std::fs::copy(&file, out)?;
            }
        },
        Dev { port } => {
            let addr = std::net::SocketAddr::from(([127, 0, 0, 1], port.unwrap_or(8080)));
            let listener = tokio::net::TcpListener::bind(addr).await?;

            loop {
                let (stream, _) = listener.accept().await?;
                let io = hyper_util::rt::TokioIo::new(stream);

                tokio::spawn(async move {
                    if let Err(err) = hyper::server::conn::http1::Builder::new()
                        .serve_connection(io, hyper::service::service_fn(server::service))
                            .await
                            {
                                println!("Could not start dev server: {}", err);
                            }
                });
            }
        },
    };

    Ok(())
}
