pub fn walk_dir(path: &str, f: fn(name: &str, ext: &str, path: &std::path::Path) -> std::io::Result<()>) {
    for entry in walkdir::WalkDir::new(path) {
        if let Ok(file) = entry {
            if !file.file_type().is_file() {
                continue;
            }

            if let Some(file_name) = file.file_name().to_str() {
                let mut iter = file_name.split(".");
                let mut name = String::new();
                let mut ext = "";

                while let Some(elem) = iter.next() {
                    name.push_str(ext);
                    ext = elem;
                }

                let _ = f(&name, ext, file.path());
            }
        }
    }
}
