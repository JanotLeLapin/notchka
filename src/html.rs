markup::define! {
    Tree<'a>(prefix: &'a str, pages: Vec<(String, String)>) {
        nav.tree {
            h4 { "Pages" }
            ul {
                @for (name, path) in pages.into_iter() {
                    li {
                        a[href=&format!("{}{}.html", prefix, path)] { @name }
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

pub fn make_tree<'a>(prefix: &'a str, files: &Vec<String>) -> Tree<'a> {
    let pages = files.into_iter().map(|f| {
        let path = std::path::Path::new(&f);
        let (_, meta) = crate::md::parse_meta(&std::fs::read_to_string(path).unwrap());
        (
            meta.unwrap_or_default().title.unwrap_or(path.file_stem().unwrap().to_string_lossy().into_owned()),
            path.with_extension("").to_string_lossy().into_owned().split("/").skip(1).map(|s| format!("/{}", s)).collect(),
        )
    }).collect();
    Tree { prefix, pages }
}

pub fn make_page(page: crate::Page, prefix: Option<&str>, tree: &Tree) -> String {
    Template { page, prefix: prefix.unwrap_or(""), tree }.to_string()
}
