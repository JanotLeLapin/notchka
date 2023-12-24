pub mod md;
pub mod html;
pub mod sass;
pub mod util;

pub mod server;

pub const OUT: &str = "build";

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
}
