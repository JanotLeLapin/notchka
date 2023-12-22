use markdown;

const KATEX_SCRIPT: &str = r#"
document.addEventListener('DOMContentLoaded', function() {
    renderMathInElement(document.body, {
        delimiters: [
          {left: '$$', right: '$$', display: true},
          {left: '$', right: '$', display: false},
        ],
        throwOnError : false
    });
});
"#;

markup::define! {
    Template(page: Page) {
        @markup::doctype()
        html[lang="fr"] {
            head {
                link[rel="stylesheet", href="../style/base.css"];

                @if let Some(title) = &page.meta.title {
                    title { @title }
                }
                @if page.meta.maths {
                    link[
                        rel="stylesheet",
                        href="https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/katex.min.css",
                        integrity="sha384-n8MVd4RsNIU0tAv4ct0nTaAbDJwPJzDEaqSD1odI+WdtXRGWt2kTvGFasHpSy3SV",
                        crossorigin="anonymous"
                    ];

                    script[
                        defer,
                        src="https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/katex.min.js",
                        integrity="sha384-XjKyOOlGwcjNTAIQHIpgOno0Hl1YQqzUOEleOLALmuqehneUG+vnGctmUb0ZY0l8",
                        crossorigin="anonymous"
                    ] {}

                    script[
                        defer,
                        src="https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/contrib/auto-render.min.js",
                        integrity="sha384-+VBxd3r6XgURycqtZ117nYw44OOcIax56Z4dCRWbxyPt0Koah1uHoK0o4+/RRE05",
                        crossorigin="anonymous",
                        onload="renderMathInElement(document.body);"
                    ] {}

                    script { @KATEX_SCRIPT }
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
    #[serde(default = "bool::default")]
    maths: bool,
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
        src: markdown::to_html(&src).replace("\n", ""),
        meta: meta.unwrap_or(Meta::default()),
    }
}

pub fn make_page(page: Page) -> String {
    Template { page }.to_string()
}
