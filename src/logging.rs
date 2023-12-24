use colored::*;

fn color_message(msg: &str, ext: &str) -> ColoredString {
    match ext {
        "md" => msg.green(),
        "css" | "scss" | "sass" => msg.purple(),
        _ => msg.white(),
    }
}

pub fn info_compiled(file: &crate::util::File, time: &std::time::Instant) {
    println!(
        "{} '{}' ({:?})",
        color_message("Compiled", &file.ext),
        file.path,
        time.elapsed(),
    );
}

pub fn error_compiled(file: &crate::util::File, err: Box<impl std::fmt::Display>) {
    println!(
        "{} on {}: {}",
        color_message("Failed", &file.ext),
        file.path,
        err,
    );
}
