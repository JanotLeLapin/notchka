pub struct FsPage {
    pub name: String,
    pub path: String,
}

pub fn file_structure(files: &Vec<crate::File>) -> Vec<FsPage> {
    files.into_iter().map(|f| {
        let (_, meta) = parse_meta(&std::fs::read_to_string(&f.path).unwrap());
        let mut buf = std::path::PathBuf::new();
        let mut tmp = "/";
        for elem in f.path.split("/").skip(1) {
            buf.push(tmp);
            tmp = elem;
        }
        buf.push(format!("{}.html", f.name));
        FsPage {
            name: meta.unwrap_or_default().title.unwrap_or(f.name.to_string()),
            path: buf.to_string_lossy().into_owned(),
        }
    }).collect()
}

pub fn parse_meta(src: &str) -> (String, Option<crate::Meta>) {
    if src.starts_with("---") {
        let mut iter = src.split("---").skip(1);
        let meta = iter.next().unwrap();
        let meta = serde_yaml::from_str(meta).ok();
        (iter.next().unwrap().to_string(), meta)
    } else { (src.to_string(), None) }
}

pub fn parse_markdown(src: &str, meta: crate::Meta) -> Result<crate::Page, String> {
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
