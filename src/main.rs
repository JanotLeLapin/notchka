mod md;
mod html;
mod util;

const OUT: &str = "dist";

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
    let _ = std::fs::create_dir_all(format!("{}/style", OUT));

    for file in util::walk_dir("content") {
        let content = std::fs::read_to_string(&file.path)?;
        let page = html::make_page(md::parse_markdown(&content));

        let out = file.path.replace("content/", "");
        let out = std::path::Path::new(OUT).join(out);
        let out = out.parent().unwrap();
        std::fs::create_dir_all(out)?;
        std::fs::write(out.join(format!("{}.html", file.name)), page)?;
    }

    for file in util::walk_dir("style") {
        let options = grass::Options::default().style(grass::OutputStyle::Compressed);
        if let Ok(result) = grass::from_path(&file.path, &options) {
            let out = std::path::Path::new(OUT).join(&file.path);
            let out = out.parent().unwrap();
            std::fs::create_dir_all(out)?;
            std::fs::write(out.join(format!("{}.css", file.name)), result)?;
        }
    }

    Ok(())
}
