pub fn parse_meta(src: &str) -> (String, Option<crate::Meta>) {
    if src.starts_with("---") {
        let mut iter = src.split("---").skip(1);
        let meta = iter.next().unwrap();
        let meta = serde_yaml::from_str(meta).ok();
        (iter.next().unwrap().to_string(), meta)
    } else { (src.to_string(), None) }
}

#[cfg(feature = "katex")]
fn latex(src: &str) -> katex::Result<String> {
    let mut result = String::new();
    let mut iter = src.chars().peekable();

    let mut katex = false;
    let mut tmp = String::new();

    let opts = katex::Opts::builder().display_mode(false).output_type(katex::OutputType::HtmlAndMathml).build().unwrap();

    while let Some(c) = iter.next() {
        if katex {
            if c == '$' {
                katex = false;
                result.push_str(&katex::render_with_opts(&tmp, &opts)?);
                tmp.clear();
            } else { tmp.push(c); }
        }
        else {
            if c == '$' { katex = true; }
            else { result.push(c); }
        }
    }

    Ok(result)
}

#[cfg(not(feature = "katex"))]
fn latex(src: &str) -> std::result::Result<String, std::convert::Infallible> {
    println!("LaTeX function was called, but Notchka was compiled without the katex feature enabled!");
    Ok(src.to_string())
}

pub fn parse_markdown(src: &str, meta: crate::Meta) -> Result<crate::Page, String> {
    let src = if meta.katex {
        match latex(src) {
            Ok(res) => res,
            Err(e) => {
                println!("{}", e);
                src.to_string()
            },
        }
    } else { src.to_string() };

    let parser = pulldown_cmark::Parser::new(&src);
    let mut events = Vec::new();
    let mut sections = Vec::new();
    let mut heading_lvl = None;
    for event in parser {
        use pulldown_cmark::Event::*;
        use pulldown_cmark::Tag::*;

        match event {
            Start(Heading(lvl, _, _)) => heading_lvl = Some(lvl),
            Text(txt) => {
                if let Some(lvl) = heading_lvl {
                    use pulldown_cmark::CowStr::*;
                    let id = match txt {
                        Borrowed(s) => {
                            sections.push(s.to_string());
                            Some(s)
                        },
                        _ => None,
                    };
                    events.push(Start(Heading(lvl, id, vec![])));
                    heading_lvl = None;
                }
                events.push(Text(txt));
            },
            event => events.push(event),
        }
    }

    let mut buf = String::new();
    pulldown_cmark::html::push_html(&mut buf, events.into_iter());

    Ok(crate::Page {
        src: buf,
        meta,
        sections,
    })
}
