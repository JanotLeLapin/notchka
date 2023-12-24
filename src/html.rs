markup::define! {
    Template<'a>(page: crate::Page, dist_prefix: &'a str) {
        @markup::doctype()
        html[lang="fr"] {
            head {
                link[rel="stylesheet", href=format!("{}/style/base.css", dist_prefix)];

                @if let Some(title) = &page.meta.title {
                    title { @title }
                }

                @if let Some(css) = &page.meta.css {
                    link[rel="stylesheet", href=format!("{}/style/{}.css", dist_prefix, css)];
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

                    script[
                        defer,
                        src=format!("{}/script/maths.js", dist_prefix),
                    ] {}
                }
            }
            body {
                @markup::raw(page.src.clone())
            }
        }
    }
}

pub fn make_page(page: crate::Page, dist_prefix: Option<&str>) -> String {
    Template { page, dist_prefix: dist_prefix.unwrap_or("./dist") }.to_string()
}
