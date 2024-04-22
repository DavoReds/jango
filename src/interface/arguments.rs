use super::parsing::parse_key_val;
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
    /// Where to pull the information from
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
    /// Use arguments to populate the template
    #[command(arg_required_else_help = true, visible_alias = "a")]
    Args(CommandLineArgs),

    /// Use a Markdown file to populate the template
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
pub struct CommandLineArgs {
    /// Whether to escape the input
    #[arg(short, long)]
    pub escape: bool,

    /// `key=value` pairs to pass to the template
    #[arg(short, long, value_parser = parse_key_val::<String, String>)]
    pub data: Option<Vec<(String, String)>>,

    /// Path to the template file
    #[arg(value_hint = clap::ValueHint::FilePath)]
    pub template: Utf8PathBuf,

    /// Path for the output file. Prints to stdout if not present
    pub output: Option<Utf8PathBuf>,
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
    /// Don't escape input's inline HTML
    #[arg(short, long = "inline")]
    pub inline_html: bool,

    /// Path to the template file
    #[arg(value_hint = clap::ValueHint::FilePath)]
    pub template: Utf8PathBuf,

    /// Path to the Markdown file
    #[arg(value_hint = clap::ValueHint::FilePath)]
    pub input: Utf8PathBuf,

    /// Path for the output file. Prints to stdout if not present
    pub output: Option<Utf8PathBuf>,
}
