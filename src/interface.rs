use camino::Utf8PathBuf;
use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "Jango",
    author,
    version,
    about,
    long_about = None,
    help_template = "\
{before-help}{name} {version}
{author-with-newline}{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
#[command(help_template(
    "\
{name}
{about-with-newline}
{usage-heading} {usage}

{all-args}"
))]
pub enum Command {
    #[command(arg_required_else_help = true, visible_alias = "md")]
    Markdown(MarkdownArgs),
}

#[derive(Debug, Args)]
#[command(help_template(
    "\
{name}
{about-with-newline}
{usage-heading} {usage}

{all-args}"
))]
pub struct MarkdownArgs {
    /// Path to the template file
    pub template: Utf8PathBuf,

    /// Path to the input markdown file
    pub input: Utf8PathBuf,

    /// Path to put the output file
    pub output: Utf8PathBuf,
}
