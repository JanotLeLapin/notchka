pub fn parse_markdown(mut src: &str) -> Result<crate::Page, String> {
    // Metadata parsing
    let meta = if src.starts_with("---") {
        let mut iter = src.split("---").skip(1);
        let meta = iter.next().unwrap();
        let meta = serde_yaml::from_str(meta).ok();
        src = iter.next().unwrap();
        meta
    } else {
        None
    }.unwrap_or(crate::Meta::default());

    let parser = pulldown_cmark::Parser::new(src);
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
