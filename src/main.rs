mod parser;

const OUT: &str = "dist";

fn main() {
    let dir = walkdir::WalkDir::new("content");
    for entry in dir {
        if let Ok(file) = entry {
            if !file.file_type().is_file() {
                continue;
            }

            if let Ok(content) = std::fs::read_to_string(file.path()) {
                let file_name = file.file_name().to_str().unwrap();
                if !file_name.ends_with(".md") {
                    continue;
                }

                let file_name = file_name.chars().take(file_name.len() - 3).collect::<String>();

                let html = parser::parse_markdown(&content);
                let page = parser::make_page(html);

                let parent = format!("{}/{}", OUT, file.path().parent().unwrap().display());
                let _ = std::fs::create_dir_all(&parent);
                let _ = std::fs::write(format!("{}/{}", &parent, format!("{}.html", file_name)), page);
            }
        }
    }
}
