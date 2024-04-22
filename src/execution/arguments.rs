use color_eyre::eyre::Context;
use tera::Tera;

pub fn render_template_with_args(
    template: &str,
    data: Option<&[(String, String)]>,
    escape: bool,
) -> color_eyre::Result<String> {
    let mut ctx = tera::Context::new();
    if let Some(pairs) = data {
        for (key, value) in pairs {
            ctx.insert(key, value);
        }
    }

    Tera::one_off(template, &ctx, escape).wrap_err("Failed to render template")
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use tera::escape_html;

    proptest! {
        #[test]
        fn render_template_works_with_valid_input(input in "\\PC*") {
            let template = "<body>{{ content }}</body>";

            let result = render_template_with_args(template, Some(&[("content".to_string(), input.clone())]), false);
            assert!(result.is_ok());

            let output = result.expect("Failed to render template");
            assert_eq!(output, format!("<body>{input}</body>"));
        }

        #[test]
        fn render_template_with_args_works_with_unnecesary_input(input in "\\PC*") {
            let template = r#"<body>{{ "Hello, " ~ "world" ~ `!`}}</body>"#;

            let result = render_template_with_args(
                template,
                Some(&[("content".to_string(), input)]),
                false,
            );
            assert!(result.is_ok());

            let output = result.expect("Failed to render template");
            assert_eq!(output, "<body>Hello, world!</body>");
        }

        #[test]
        fn render_template_with_args_correctly_escapes_input(input in "\\PC*") {
            let template = "<body>{{ content }}</body>";

            let result = render_template_with_args(
                template,
                Some(&[("content".to_string(), format!("<h1>{input}</h1>"))]),
                true,
            );
            assert!(result.is_ok());

            let output = result.expect("Failed to render template");
            assert_eq!(
                output,
                format!("<body>&lt;h1&gt;{}&lt;&#x2F;h1&gt;</body>", escape_html(&input))
            );
        }
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
    fn render_template_with_args_fails_without_necessary_input() {
        let template = "<body>{{ content }}</body>";

        let result = render_template_with_args(template, None, false);
        assert!(result.is_err());
    }
}
