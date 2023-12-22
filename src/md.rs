use markdown;

pub fn parse_markdown(src: &str) -> crate::Page {
    let mut src = src.trim().to_string();
    let mut meta: Option<crate::Meta> = None;
    if src.starts_with("---") {
        // Frontmatter parsing
        let trim = src.chars().skip(4).collect::<String>();
        let mut iter = trim.split("\n---");

        if let Some(frontmatter) = iter.next() {
            meta = serde_yaml::from_str(frontmatter).ok();
        }

        src = match iter.next() {
            Some(value) => value.to_string(),
            None => String::new(),
        };
    }

    let opts = markdown::Options {
        compile: markdown::CompileOptions {
            allow_dangerous_html: true,
            ..Default::default()
        },
        parse: markdown::ParseOptions::default(),
    };

    crate::Page {
        src: markdown::to_html_with_options(&src, &opts).unwrap().replace("\n", ""),
        meta: meta.unwrap_or(crate::Meta::default()),
    }
}
