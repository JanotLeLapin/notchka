use http_body_util::Full;
use hyper::{Request, Response};
use hyper::body::Bytes;

fn find_file(name: &str, path: &str) -> Option<crate::util::File> {
    crate::util::walk_dir(path).into_iter().find(|f| f.name == name)
}

fn mime(ext: &str) -> String {
    let mime = match ext {
        "css" => "text/css",
        "html" => "text/html",
        "js" => "application/javascript",
        _ => "text/document",
    };

    format!("{}; charset=utf-8", mime)
}

pub async fn service(req: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, std::convert::Infallible> {
    let (content_type, content) = {
        let uri: String = req.uri().to_string().chars().skip(1).collect();
        let parent = std::path::Path::new(&uri).parent().unwrap().to_string_lossy().into_owned();

        let file: crate::util::File = uri.to_string().into();
        let content = match file.ext.as_str() {
            "css" => {
                let dir: String = parent.split("/").skip(2).collect();
                let dir = std::path::Path::new("style").join(dir);
                match find_file(&file.name, &dir.to_string_lossy().into_owned()) {
                    Some(file) => {
                        let start = std::time::Instant::now();
                        match crate::sass::compile_scss(&file) {
                            Some(result) => match result {
                                Ok(content) => {
                                    crate::logging::info_compiled(&file, &start);
                                    content
                                },
                                Err(err) => {
                                    crate::logging::error_compiled(&file, err);
                                    String::new()
                                }
                            },
                            None => {
                                crate::logging::error_compiled(&file, Box::new(format!("Unknown format '{}'", file.ext)));
                                String::new()
                            },
                        }
                    },
                    None => {
                        String::new()
                    }
                }
            },
            "html" => {
                let dir = std::path::Path::new("content").join(&parent);
                match find_file(&file.name, &dir.to_string_lossy().into_owned()) {
                    Some(file) => {
                        let start = std::time::Instant::now();
                        let content = crate::html::make_page(crate::md::parse_markdown(&std::fs::read_to_string(&file.path).unwrap()), None);
                        crate::logging::info_compiled(&file, &start);
                        content
                    },
                    None => "File not found".to_string(),
                }
            },
            _ => match std::fs::read_to_string(uri) {
                Ok(content) => content,
                Err(_) => String::new(),
            },
        };

        (file.ext, content)
    };

    let response = Response::builder()
        .header("Content-Type", mime(&content_type))
        .body(Full::new(Bytes::from(content)))
        .unwrap();

    Ok(response)
}
