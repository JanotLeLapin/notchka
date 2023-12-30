pub mod md;
pub mod html;
pub mod sass;

pub mod logging;
pub mod server;

pub const OUT: &str = "build";

pub fn walk_dir(path: &str) -> Vec<String> {
    let mut res = Vec::new();
    for entry in walkdir::WalkDir::new(path) {
        if let Ok(file) = entry {
            if !file.file_type().is_file() {
                continue;
            }

            res.push(file.path().to_str().unwrap().to_string().into())
        }
    }
    res
}

pub fn create_path<'a>(base: &str, iter: impl Iterator<Item = &'a str>) -> std::path::PathBuf {
    let mut buf = std::path::PathBuf::new();
    buf.push(base);
    for item in iter {
        buf.push(item);
    }
    buf
}

#[derive(Debug, Default, serde::Deserialize)]
pub struct Meta {
    title: Option<String>,
    css: Option<Vec<String>>,
    js: Option<Vec<String>>,
    #[serde(default = "bool::default")] katex: bool,
    #[serde(default = "bool::default")] prism: bool,
}

#[derive(Debug)]
pub struct Page {
    src: String,
    meta: Meta,
    sections: Vec<String>,
}
