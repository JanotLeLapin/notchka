pub mod md;
pub mod html;
pub mod sass;

pub mod logging;
pub mod server;

pub const OUT: &str = "build";

#[derive(Debug)]
pub struct File {
    pub name: String,
    pub ext: String,
    pub path: String,
}

impl Into<File> for String {
    fn into(self) -> File {
        let file_name = self.split("/").last().unwrap();

        let mut iter = file_name.split(".");
        let mut name = String::new();
        let mut ext = String::new();

        while let Some(elem) = iter.next() {
            name.push_str(&ext);
            ext = elem.to_string();
        }

        if name.is_empty() {
            File {
                name: ext,
                ext: name,
                path: self,
            }
        } else {
            File {
                name,
                ext,
                path: self,
            }
        }
    }
}

pub fn walk_dir(path: &str) -> Vec<File> {
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

#[derive(Debug, Default, serde::Deserialize)]
pub struct Meta {
    title: Option<String>,
    css: Option<String>,
    #[serde(default = "bool::default")]
    maths: bool,
}

#[derive(Debug)]
pub struct Page {
    src: String,
    meta: Meta,
    sections: Vec<String>,
}
