mod parser;
mod util;

const OUT: &str = "dist";

fn compile_markdown(name: &str, _ext: &str, path: &std::path::Path) -> std::io::Result<()> {
    let content = std::fs::read_to_string(path)?;
    let html = parser::parse_markdown(&content);
    let page = parser::make_page(html);

    let out = path.strip_prefix("content").unwrap();
    let out = std::path::Path::new(OUT).join(out);
    let out = out.parent().unwrap();
    std::fs::create_dir_all(out)?;
    std::fs::write(out.join(format!("{}.html", name)), page)?;

    Ok(())
}

fn compile_scss(name: &str, _ext: &str, path: &std::path::Path) -> std::io::Result<()> {
    let options = grass::Options::default().style(grass::OutputStyle::Compressed);
    if let Ok(result) = grass::from_path(path, &options) {
        let out = std::path::Path::new(OUT).join(path);
        let out = out.parent().unwrap();
        std::fs::create_dir_all(out)?;
        std::fs::write(out.join(format!("{}.css", name)), result)?;
    }
    Ok(())
}

fn main() {
    let _ = std::fs::create_dir_all(format!("{}/style", OUT));

    util::walk_dir("content", compile_markdown);
    util::walk_dir("style", compile_scss);
}
