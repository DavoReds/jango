mod arguments;
mod markdown;

use self::{
    arguments::render_template_with_args, markdown::render_template_with_md,
};
use crate::interface::Command;
use camino::Utf8Path;
use jango::parsing::process_md_file;

pub fn execute_application(command: Command) -> color_eyre::Result<()> {
    match command {
        Command::Args(args) => create_file_with_args(
            &args.template,
            &args.output,
            args.content.as_deref(),
            args.escape,
        ),
        Command::Markdown(args) => {
            create_file_with_markdown(&args.template, &args.input, &args.output)
        }
    }
}

fn create_file_with_args(
    template_path: &Utf8Path,
    output_path: &Utf8Path,
    content: Option<&str>,
    escape: bool,
) -> color_eyre::Result<()> {
    let template = std::fs::read_to_string(template_path)?;

    let output = render_template_with_args(&template, content, escape)?;
    std::fs::write(output_path, output)?;

    Ok(())
}

fn create_file_with_markdown(
    template_path: &Utf8Path,
    markdown_path: &Utf8Path,
    output_path: &Utf8Path,
) -> color_eyre::Result<()> {
    let template = std::fs::read_to_string(template_path)?;
    let markdown = std::fs::read_to_string(markdown_path)?;

    let (frontmatter, content) = process_md_file(&markdown)?;
    let output =
        render_template_with_md(&template, &frontmatter, &content, false)?;

    std::fs::write(output_path, output)?;

    Ok(())
}
