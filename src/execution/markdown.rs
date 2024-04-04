use color_eyre::eyre::Context;
use tera::Tera;

pub fn render_template_with_markdown(
    template: &str,
    frontmatter: &toml::Table,
    content: &str,
    escape: bool,
) -> color_eyre::Result<String> {
    let mut context = tera::Context::from_serialize(frontmatter)?;
    context.insert("content", content);

    Tera::one_off(template, &context, escape)
        .wrap_err("Failed to render template")
}

#[cfg(test)]
mod tests {
    use super::*;
    use jango::parsing::process_md_file;

    #[test]
    fn render_template_works_with_valid_inputs() {
        let template = include_str!("template.html");
        let md = include_str!("test.md");

        let (frontmatter, content) =
            process_md_file(md).expect("Failed to process markdown file");

        let result = render_template_with_markdown(
            template,
            &frontmatter,
            &content,
            false,
        );
        assert!(result.is_ok());

        let output = result.expect("Failed to render template");
        assert_eq!(
            output,
            "<html>\n  <head>\n    <title>Test</title>\n  \
            </head>\n  <body><h1>This is a test</h1></body>\n</html>\n"
        )
    }

    #[test]
    fn render_template_errors_on_empty_content() {
        let template = include_str!("template.html");
        let md = "+++\n+++";

        let (frontmatter, content) =
            process_md_file(md).expect("Failed to process markdown file");

        let result = render_template_with_markdown(
            template,
            &frontmatter,
            &content,
            false,
        );
        assert!(result.is_err());
    }

    #[test]
    fn render_template_errors_on_template_mismatch() {
        let template = include_str!("template2.html");
        let md = include_str!("test.md");

        let (frontmatter, content) =
            process_md_file(md).expect("Failed to process markdown file");

        let result = render_template_with_markdown(
            template,
            &frontmatter,
            &content,
            false,
        );
        assert!(result.is_err(), "{result:?}");
    }
}
