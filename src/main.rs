mod execution;
mod interface;

use clap::Parser;
use execution::execute_application;
use interface::Cli;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();
    execute_application(cli.command)?;

    Ok(())
}
