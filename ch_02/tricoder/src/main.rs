use thiserror::Error;

mod subdomains;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Usage: tricoder <kerkour.com>")]
    CliUsage,
}

fn main() -> Result<(), anyhow::Error> {
    println!("Hello, world!");

    Ok(())
}
