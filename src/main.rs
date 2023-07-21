use crate::cli::CLI;

mod parser;
mod network;
mod cli;

fn main() {
    let cli = CLI::new();
    if let Err(e) = cli {
        eprintln!("Failed -> {}", e);
    } else {
        let cli = cli.unwrap();
        cli.delete_all();
    }
}