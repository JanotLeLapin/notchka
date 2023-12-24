use ssg::*;
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

            for file in util::walk_dir("content") {
                let content = std::fs::read_to_string(&file.path)?;
                let now = std::time::Instant::now();
                let page = html::make_page(md::parse_markdown(&content), prefix.as_deref());
                crate::logging::info_compiled(&file, &now);

                let out = std::path::Path::new(OUT).join(file.path.replace("content/", ""));
                let out = out.parent().unwrap();
                std::fs::create_dir_all(out)?;
                std::fs::write(out.join(format!("{}.html", file.name)), page)?;
            }

            for file in util::walk_dir("style") {
                let now = std::time::Instant::now();
                match sass::compile_scss(&file) {
                    Some(result) => match result {
                        Ok(result) => {
                            crate::logging::info_compiled(&file, &now);
                            let out = std::path::Path::new(OUT).join("dist").join(&file.path);
                            let out = out.parent().unwrap();
                            std::fs::create_dir_all(out)?;
                            std::fs::write(out.join(format!("{}.css", file.name)), result)?;
                        },
                        Err(err) => logging::error_compiled(&file, err),
                    },
                    None => logging::error_compiled(&file, Box::new(format!("Unknown format '{}'", file.ext))),
                };
            }

            for file in util::walk_dir("dist") {
                let out = std::path::Path::new(OUT).join(&file.path);
                std::fs::create_dir_all(out.parent().unwrap())?;
                std::fs::copy(&file.path, out)?;
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
