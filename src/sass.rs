pub fn compile_scss(file: &crate::util::File) -> Option<grass::Result<String>> {
    let syntax = match file.ext.as_str() {
        "css" => grass::InputSyntax::Css,
        "sass" => grass::InputSyntax::Sass,
        "scss" => grass::InputSyntax::Scss,
        _ => return None,
    };
    let options = grass::Options::default()
        .style(grass::OutputStyle::Compressed)
        .quiet(true)
        .input_syntax(syntax);

    Some(grass::from_path(&file.path, &options))
}
