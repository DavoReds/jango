mod arguments;
mod markdown;

use self::{
    arguments::render_template_with_args, markdown::render_template_with_md,
};
use crate::{interface::Command, templates::process_md_file};
use camino::Utf8Path;

#[allow(clippy::missing_errors_doc)]
pub fn execute_application(command: Command) -> color_eyre::Result<()> {
    match command {
        Command::Args(args) => create_file_with_args(
            &args.template,
            args.output.as_deref(),
            args.data.as_deref(),
            args.escape,
        ),
        Command::Markdown(args) => create_file_with_markdown(
            &args.template,
            &args.input,
            args.output.as_deref(),
            args.inline_html,
        ),
    }
}

fn create_file_with_args(
    template_path: &Utf8Path,
    output_path: Option<&Utf8Path>,
    data: Option<&[(String, String)]>,
    escape: bool,
) -> color_eyre::Result<()> {
    let template = std::fs::read_to_string(template_path)?;
    let output = render_template_with_args(&template, data, escape)?;

    match output_path {
        Some(path) => {
            std::fs::write(path, output)?;
        }
        None => {
            println!("{output}");
        }
    }

    Ok(())
}

fn create_file_with_markdown(
    template_path: &Utf8Path,
    markdown_path: &Utf8Path,
    output_path: Option<&Utf8Path>,
    inline_html: bool,
) -> color_eyre::Result<()> {
    let template = std::fs::read_to_string(template_path)?;
    let markdown = std::fs::read_to_string(markdown_path)?;

    let (frontmatter, content) = process_md_file(&markdown, inline_html)?;
    let output =
        render_template_with_md(&template, &frontmatter, &content, false)?;

    match output_path {
        Some(path) => {
            std::fs::write(path, output)?;
        }
        None => {
            println!("{output}");
        }
    }

    Ok(())
}
