#[cfg(debug_assertions)]
const ROOT: &str = ".";

#[cfg(not(debug_assertions))]
const ROOT: &str = "/uni";

markup::define! {
    Template(page: crate::Page) {
        @markup::doctype()
        html[lang="fr"] {
            head {
                link[rel="stylesheet", href=format!("{}/style/base.css", ROOT)];

                @if let Some(title) = &page.meta.title {
                    title { @title }
                }

                @if let Some(css) = &page.meta.css {
                    link[rel="stylesheet", href=format!("{}/style/{}.css", ROOT, css)];
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
                    ] {}

                    script[] { @std::fs::read_to_string("./script/maths.js").unwrap() }
                }
            }
            body {
                @markup::raw(page.src.clone())
            }
        }
    }
}

pub fn make_page(page: crate::Page) -> String {
    Template { page }.to_string()
}
