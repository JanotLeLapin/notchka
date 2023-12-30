pub fn compile_scss(path: &std::path::Path) -> grass::Result<String> {
    let syntax = match path.extension().map(|s| s.to_string_lossy().into_owned()).as_deref() {
        Some("css") => grass::InputSyntax::Css,
        Some("sass") => grass::InputSyntax::Sass,
        Some("scss") => grass::InputSyntax::Scss,
        _ => return Ok(std::fs::read_to_string(path)?),
    };
    let options = grass::Options::default()
        .style(grass::OutputStyle::Compressed)
        .quiet(true)
        .input_syntax(syntax);

    grass::from_path(&path, &options)
}
