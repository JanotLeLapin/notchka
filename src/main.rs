use clap::{Parser, Subcommand};

#[derive(Subcommand)]
enum Commands {
    Build,
    Dev,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    path: Option<String>,
}

#[tokio::main]
async fn main() {
    use Commands::*;

    let cli = Cli::parse();
    match cli.command {
        Build => ssg::compiler::compile().unwrap(),
        Dev => ssg::server::server().await.unwrap(),
    };
}
