use color_eyre::eyre::Context;
use tera::Tera;

pub fn render_template_with_md(
    template: &str,
    frontmatter: &impl serde::Serialize,
    content: &str,
    escape: bool,
) -> color_eyre::Result<String> {
    let mut ctx = tera::Context::from_serialize(frontmatter)?;
    ctx.insert("content", content);

    Tera::one_off(template, &ctx, escape).wrap_err("Failed to render template")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    #[once]
    fn template() -> &'static str {
        include_str!("template.html")
    }

    #[fixture]
    #[once]
    fn template_2() -> &'static str {
        include_str!("template2.html")
    }

    #[fixture]
    #[once]
    fn html_content() -> &'static str {
        "<h1>This is a test</h1>"
    }

    #[fixture]
    fn empty_table() -> toml::Table {
        toml::Table::new()
    }

    #[fixture]
    fn title_frontmatter(mut empty_table: toml::Table) -> toml::Table {
        empty_table.insert("title".into(), "Test".into());
        empty_table
    }

    #[rstest]
    fn render_template_works_with_valid_inputs(
        template: &str,
        title_frontmatter: toml::Table,
        html_content: &str,
    ) {
        let result = render_template_with_md(
            template,
            &title_frontmatter,
            html_content,
            false,
        );
        assert!(result.is_ok());

        let output = result.expect("Failed to render template");
        assert_eq!(
            output,
            "<html>\n  <head>\n    <title>Test</title>\n  \
                </head>\n  <body><h1>This is a test</h1></body>\n</html>\n"
        );
    }

    #[rstest]
    fn render_template_errors_on_empty_content(
        template: &str,
        empty_table: toml::Table,
    ) {
        let result = render_template_with_md(template, &empty_table, "", false);
        assert!(result.is_err());
    }

    #[rstest]
    fn render_template_errors_on_template_mismatch(
        template_2: &str,
        title_frontmatter: toml::Table,
        html_content: &str,
    ) {
        let result = render_template_with_md(
            template_2,
            &title_frontmatter,
            html_content,
            false,
        );
        assert!(result.is_err(), "{result:?}");
    }
}
