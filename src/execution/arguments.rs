use color_eyre::eyre::Context;
use tera::Tera;

pub fn render_template_with_args(
    template: &str,
    content: Option<&str>,
    escape: bool,
) -> color_eyre::Result<String> {
    let mut ctx = tera::Context::new();
    if let Some(text) = content {
        ctx.insert("content", text);
    }

    Tera::one_off(template, &ctx, escape).wrap_err("Failed to render template")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_template_with_args_works_with_valid_input() {
        let template = "<body>{{ content }}</body>";

        let result = render_template_with_args(
            template,
            Some("This is an example"),
            false,
        );
        assert!(result.is_ok());

        let output = result.expect("Failed to render template");
        assert_eq!(output, "<body>This is an example</body>");
    }

    #[test]
    fn render_template_with_args_works_with_no_input() {
        let template = r#"<body>{{ "Hello, " ~ "world" ~ `!`}}</body>"#;

        let result = render_template_with_args(template, None, false);
        assert!(result.is_ok());

        let output = result.expect("Failed to render template");
        assert_eq!(output, "<body>Hello, world!</body>");
    }

    #[test]
    fn render_template_with_args_works_with_unnecesary_input() {
        let template = r#"<body>{{ "Hello, " ~ "world" ~ `!`}}</body>"#;

        let result = render_template_with_args(
            template,
            Some("This will be ignored"),
            false,
        );
        assert!(result.is_ok());

        let output = result.expect("Failed to render template");
        assert_eq!(output, "<body>Hello, world!</body>");
    }

    #[test]
    fn render_template_with_args_fails_with_invalid_input() {
        let template = "<body>{{ content }}</body>";

        let result = render_template_with_args(template, None, false);
        assert!(result.is_err());
    }

    #[test]
    fn render_template_with_args_correctly_escapes_input() {
        let template = "<body>{{ content }}</body>";

        let result = render_template_with_args(
            template,
            Some("<h1>This is dangerous HTML</h1>"),
            true,
        );
        assert!(result.is_ok());

        let output = result.expect("Failed to render template");
        assert_eq!(
            output,
            "<body>&lt;h1&gt;This is dangerous HTML&lt;&#x2F;h1&gt;</body>"
        );
    }
}
