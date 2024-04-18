use clap::Parser;
use jango::{execution::execute_application, interface::Cli};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();
    execute_application(cli.command)?;

    Ok(())
}
