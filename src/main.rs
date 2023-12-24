mod md;
mod html;
mod sass;
mod util;

mod server;

use colored::*;

const OUT: &str = "build";

#[derive(Debug, Default, serde::Deserialize)]
pub struct Meta {
    title: Option<String>,
    css: Option<String>,
    #[serde(default = "bool::default")]
    maths: bool,
}

#[derive(Debug)]
pub struct Page {
    src: String,
    meta: Meta,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    /*
    let start = std::time::Instant::now();

    let _ = std::fs::remove_dir_all(OUT);

    for file in util::walk_dir("content") {
        let content = std::fs::read_to_string(&file.path)?;
        let now = std::time::Instant::now();
        let page = html::make_page(md::parse_markdown(&content));
        println!("{} page '{}' (in {:?})", "Compiled".green(), file.path, now.elapsed());

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
                    println!("{} stylesheet '{}' (in {:?})", "Compiled".green(), file.path, now.elapsed());
                    let out = std::path::Path::new(OUT).join("dist").join(&file.path);
                    let out = out.parent().unwrap();
                    std::fs::create_dir_all(out)?;
                    std::fs::write(out.join(format!("{}.css", file.name)), result)?;
                },
                Err(err) => println!("{} for stylesheet '{}': {}", "Compilation failed".red(), file.path, err),
            },
            None => println!("{} for stylesheet '{}': Unknown file type", "Compilation failed".red(), file.path),
        };
    }

    for file in util::walk_dir("dist") {
        let out = std::path::Path::new(OUT).join(&file.path);
        std::fs::create_dir_all(out.parent().unwrap())?;
        std::fs::copy(&file.path, out)?;
    }

    println!("{} (took {:?})", "Done".blue(), start.elapsed());
    */

    let _ = server::server().await;

    Ok(())
}
