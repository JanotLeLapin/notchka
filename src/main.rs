mod md;
mod html;
mod util;

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

fn main() -> std::io::Result<()> {
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
        let syntax = match file.ext.as_str() {
            "css" => grass::InputSyntax::Css,
            "sass" => grass::InputSyntax::Sass,
            "scss" => grass::InputSyntax::Scss,
            _ => continue,
        };
        let options = grass::Options::default()
            .style(grass::OutputStyle::Compressed)
            .quiet(true)
            .input_syntax(syntax);

        let now = std::time::Instant::now();
        match grass::from_path(&file.path, &options) {
            Ok(result) => {
                println!("{} stylesheet '{}' (in {:?})", "Compiled".green(), file.path, now.elapsed());
                let out = std::path::Path::new(OUT).join("dist").join(&file.path);
                let out = out.parent().unwrap();
                std::fs::create_dir_all(out)?;
                std::fs::write(out.join(format!("{}.css", file.name)), result)?;
            },
            Err(err) => {
                println!("{} for stylesheet '{}': {}", "Compilation failed".red(), file.path, err);
            }
        };
    }

    for file in util::walk_dir("dist") {
        let out = std::path::Path::new(OUT).join(&file.path);
        std::fs::create_dir_all(out.parent().unwrap())?;
        std::fs::copy(&file.path, out)?;
    }

    println!("{} (took {:?})", "Done".blue(), start.elapsed());

    Ok(())
}
