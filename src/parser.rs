use markdown;

markup::define! {
    Template(page: Page) {
        @markup::doctype()
        html[lang="fr"] {
            head {
                @if let Some(title) = &page.meta.title {
                    title { @title }
                }
            }
            body {
                @markup::raw(page.src.clone())
            }
        }
    }
}

#[derive(Debug, Default, serde::Deserialize)]
pub struct Meta {
    title: Option<String>,
}

#[derive(Debug)]
pub struct Page {
    src: String,
    meta: Meta,
}

pub fn parse_markdown(src: &str) -> Page {
    let mut src = src.trim().to_string();
    let mut meta: Option<Meta> = None;
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

    Page {
        src: markdown::to_html(&src),
        meta: meta.unwrap_or(Meta::default()),
    }
}

pub fn make_page(page: Page) -> String {
    Template { page }.to_string()
}
