use colored::*;

fn color_message(msg: &str, ext: &str) -> ColoredString {
    match ext {
        "md" => msg.green(),
        "css" | "scss" | "sass" => msg.purple(),
        _ => msg.white(),
    }
}

pub fn info_compiled(path: &std::path::Path, time: &std::time::Instant) {
    println!(
        "{} '{}' ({:?})",
        color_message("Compiled", &path.extension().unwrap().to_string_lossy().into_owned()),
        path.display(),
        time.elapsed(),
    );
}

pub fn error_compiled(path: &std::path::Path, err: Box<impl std::fmt::Display>) {
    println!(
        "{} on {}: {}",
        color_message("Failed", &path.extension().unwrap().to_string_lossy().into_owned()),
        path.display(),
        err,
    );
}
