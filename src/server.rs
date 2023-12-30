use http_body_util::Full;
use hyper::{Request, Response};
use hyper::body::Bytes;

fn mime(ext: &str) -> String {
    let mime = match ext {
        "css" => "text/css",
        "html" => "text/html",
        "js" => "application/javascript",
        _ => "text/document",
    };

    format!("{}; charset=utf-8", mime)
}

fn create_path<'a>(base: &str, iter: impl Iterator<Item = &'a str>) -> std::path::PathBuf {
    let mut buf = std::path::PathBuf::new();
    buf.push(base);
    for item in iter {
        buf.push(item);
    }
    buf
}

fn find_by_name(mut readdir: std::fs::ReadDir, name: &str) -> Option<String> {
    readdir.find_map(|entry| entry.ok().and_then(|file| {
        if file.path().file_stem().map(|file_name| file_name.to_string_lossy().into_owned() == name).unwrap_or(false) {
            Some(file.path().to_string_lossy().into_owned())
        } else { None }
    }))
}

pub async fn service(req: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, std::convert::Infallible> {
    let uri = req.uri().to_string();
    let mut iter = uri.split("/").skip(1).peekable();
    let result = {
        if let Some(_) = iter.next_if_eq(&"dist") {
            let path = create_path("", iter.clone());
            let path = std::path::Path::new(&path);
            match path.extension().map(|s| s.to_string_lossy().into_owned()).as_deref() {
                Some("css") => {
                    if let Ok(readdir) = std::fs::read_dir(path.parent().unwrap()) {
                        if let Some(path) = find_by_name(readdir, &path.file_stem().unwrap().to_string_lossy().into_owned()) {
                            crate::sass::compile_scss(&std::path::Path::new(&path)).ok().map(|content| ("css".to_string(), content))
                        } else { None }
                    } else { None }
                },
                _ => std::fs::read_to_string(path.to_string_lossy().into_owned()).ok().map(|content| (path.extension().unwrap().to_string_lossy().into_owned(), content)),
            }
        } else {
            // Render HTML page
            let path = create_path("content", iter);
            let path = if path.is_dir() {
                Some(path.join("index.md").to_string_lossy().into_owned())
            } else {
                if let Some(readdir) = path.parent().and_then(|parent| std::fs::read_dir(parent).ok()) {
                    find_by_name(readdir, &path.file_stem().unwrap().to_string_lossy().into_owned())
                } else { None }
            };
            if let Some(path) = path {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    let files = crate::walk_dir("content");
                    let tree = crate::html::make_tree("", &files);
                    let (content, meta) = crate::md::parse_meta(&content);
                    let page = match crate::md::parse_markdown(&content, meta.unwrap_or_default()) {
                        Ok(md) => crate::html::make_page(md, None, &tree),
                        Err(err) => {
                            crate::logging::error_compiled(&std::path::Path::new(&path), Box::new(&err));
                            "File not found".to_string()
                        },
                    };
                    Some(("html".to_string(), page))
                } else { None }
            } else { None }
        }
    };

    if let Some((content_type, content)) = result {
        Ok(Response::builder()
            .header("Content-Type", mime(&content_type))
            .body(Full::new(Bytes::from(content))).unwrap()
        )
    } else {
        Ok(Response::builder()
            .status(500)
            .body(Full::new(Bytes::new())).unwrap()
        )
    }
}
