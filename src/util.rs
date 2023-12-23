#[derive(Debug)]
pub struct File {
    pub name: String,
    pub ext: String,
    pub path: String,
}

pub fn walk_dir(path: &str) -> Vec<File> {
    let mut res = Vec::new();
    for entry in walkdir::WalkDir::new(path) {
        if let Ok(file) = entry {
            if !file.file_type().is_file() {
                continue;
            }

            if let Some(file_name) = file.file_name().to_str() {
                let mut iter = file_name.split(".");
                let mut name = String::new();
                let mut ext = String::new();

                while let Some(elem) = iter.next() {
                    name.push_str(&ext);
                    ext = elem.to_string();
                }

                res.push(File {
                    name,
                    ext,
                    path: file.path().to_str().unwrap().to_string(),
                })
            }
        }
    }
    res
}
