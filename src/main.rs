mod execution;
mod interface;

use clap::Parser;
use interface::Cli;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();
    println!("{cli:?}");

    Ok(())
}
