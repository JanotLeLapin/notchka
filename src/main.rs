mod md;
mod html;
mod util;

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
    std::fs::remove_dir_all(OUT)?;

    for file in util::walk_dir("content") {
        let content = std::fs::read_to_string(&file.path)?;
        let page = html::make_page(md::parse_markdown(&content));

        let out = std::path::Path::new(OUT).join(file.path.replace("content/", ""));
        let out = out.parent().unwrap();
        std::fs::create_dir_all(out)?;
        std::fs::write(out.join(format!("{}.html", file.name)), page)?;
    }

    let options = grass::Options::default().style(grass::OutputStyle::Compressed);
    for file in util::walk_dir("style") {
        if let Ok(result) = grass::from_path(&file.path, &options) {
            let out = std::path::Path::new(OUT).join("dist").join(&file.path);
            let out = out.parent().unwrap();
            std::fs::create_dir_all(out)?;
            std::fs::write(out.join(format!("{}.css", file.name)), result)?;
        }
    }

    for file in util::walk_dir("dist") {
        let out = std::path::Path::new(OUT).join(&file.path);
        std::fs::create_dir_all(out.parent().unwrap())?;
        std::fs::copy(&file.path, out)?;
    }

    Ok(())
}
