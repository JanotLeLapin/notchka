markup::define! {
    Tree<'a>(fs: &'a Vec<crate::md::FsPage>) {
        nav.tree {
            h4 { "Pages" }
            ul {
                @for file in fs.into_iter() {
                    li {
                        a[href=&file.path] { @file.name }
                    }
                }
            }
        }
    }
    Template<'a>(page: crate::Page, prefix: &'a str, tree: &'a Tree<'a>) {
        @markup::doctype()
        html[lang="fr"] {
            head {
                link[rel="stylesheet", href=format!("{}/dist/style/base.css", prefix)];

                meta[name="viewport", content="width=device-width, initial-scale=1.0"];

                @if let Some(title) = &page.meta.title {
                    title { @title }
                }

                @if let Some(css) = &page.meta.css {
                    @for href in css {
                        link[rel="stylesheet", href=format!("{}/dist/style/{}.css", prefix, href)];
                    }
                }

                @if let Some(js) = &page.meta.js {
                    @for src in js {
                        script[defer, src=format!("{}/dist/script/{}.js", prefix, src)] {}
                    }
                }

                @if page.meta.katex {
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
                }

                @if page.meta.prism {
                    script[
                        defer,
                        src="https://cdn.jsdelivr.net/npm/prismjs@v1.29/components/prism-core.min.js",
                        integrity="sha384-MXybTpajaBV0AkcBaCPT4KIvo0FzoCiWXgcihYsw4FUkEz0Pv3JGV6tk2G8vJtDc",
                        crossorigin="anonymous",
                    ] {}

                    script[
                        defer,
                        src="https://cdn.jsdelivr.net/npm/prismjs@v1.29/plugins/autoloader/prism-autoloader.min.js",
                        integrity="sha384-Uq05+JLko69eOiPr39ta9bh7kld5PKZoU+fF7g0EXTAriEollhZ+DrN8Q/Oi8J2Q",
                        crossorigin="anonymous",
                    ] {}
                }
            }
            body {
                @tree
                nav.sections {
                    h4 { "Contenu" }
                    ul {
                        @for section in &page.sections {
                            li { a[href=format!("#{}", section)] { @section } }
                        }
                    }
                }
                div.content { @markup::raw(page.src.clone()) }
            }
        }
    }
}

pub fn make_tree<'a>(fs: &'a Vec<crate::md::FsPage>) -> Tree<'a> {
    Tree { fs }
}

pub fn make_page(page: crate::Page, prefix: Option<&str>, tree: &Tree<'_>) -> String {
    Template { page, prefix: prefix.unwrap_or(""), tree }.to_string()
}
