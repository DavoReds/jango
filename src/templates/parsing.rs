use super::default_md_options;
use color_eyre::eyre::Context;

pub fn parse_md_frontmatter(
    frontmatter: &str,
) -> color_eyre::Result<toml::Table> {
    toml::from_str(frontmatter).wrap_err("Failed to parse TOML fragment")
}

pub fn parse_md_content(input: &str, inline_html: bool) -> String {
    markdown::to_html_with_options(input, &default_md_options(inline_html))
        .expect("This should never fail")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    #[once]
    fn md_test() -> &'static str {
        include_str!("test.md")
    }

    #[fixture]
    #[once]
    fn html_test() -> &'static str {
        include_str!("html_test.md")
    }

    #[test]
    fn parse_frontmatter_works_with_valid_toml() {
        let input = "title = \"This is for a test\"\nvalid = true";

        let result = parse_md_frontmatter(input);
        assert!(result.is_ok());

        let result = result.expect("Failed to parse frontmatter");
        assert_eq!(result["title"].as_str(), Some("This is for a test"));
        assert_eq!(result["valid"].as_bool(), Some(true));
    }

    #[test]
    fn parse_frontmatter_works_with_empty_frontmatter() {
        let input = "";

        let result = parse_md_frontmatter(input);
        assert!(result.is_ok());
    }

    #[test]
    fn parse_frontmatter_errors_with_invalid_toml() {
        let input = "This is for a test\nvalid = false";

        let result = parse_md_frontmatter(input);
        assert!(result.is_err());
    }

    #[test]
    fn parse_contents_works_with_simple_markdown() {
        let input =
            "# This is a title\n\nThis is a paragraph with a **bold** word.";

        let result = parse_md_content(input, false);
        assert_eq!(
            result,
            "<h1>This is a title</h1>\n<p>This is a paragraph \
            with a <strong>bold</strong> word.</p>"
        );
    }

    #[rstest]
    fn parse_contents_works_on_a_file_with_a_frontmatter(md_test: &str) {
        let result = parse_md_content(md_test, false);
        assert_eq!(
            result,
            "<h1>Lorem ipsum dolor sit amet</h1>\n<p>Lorem \
            <del>ipsum</del> <em>dolor</em> sit amet, officia excepteur ex \
            fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur \
            mollit ex esse <strong>exercitation</strong> amet. Nisi anim \
            cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum \
            Lorem est aliquip amet voluptate voluptate dolor minim nulla est \
            proident. Nostrud officia pariatur ut officia. Sit irure elit esse \
            ea nulla sunt ex occaecat reprehenderit commodo officia dolor \
            Lorem duis laboris cupidatat officia voluptate. Culpa proident \
            adipisicing id nulla nisi laboris ex in Lorem sunt duis officia \
            eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt \
            velit enim. Voluptate laboris sint cupidatat ullamco ut ea \
            consectetur et est culpa et culpa duis.</p>\n"
        );
    }

    #[rstest]
    fn parse_contents_works_with_valid_inline_html(html_test: &str) {
        let result = parse_md_content(html_test, true);
        assert_eq!(
            result,
            "<h1>Lorem ipsum dolor sit amet</h1>\n<p>Lorem \
            <del>ipsum</del> <em>dolor</em> sit amet, officia excepteur ex \
            fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur \
            mollit ex esse <strong>exercitation</strong> amet. Nisi anim \
            cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum \
            Lorem est aliquip amet voluptate voluptate dolor minim nulla est \
            proident. Nostrud officia pariatur ut officia. Sit irure elit esse \
            ea nulla sunt ex occaecat reprehenderit commodo officia dolor \
            Lorem duis laboris cupidatat officia voluptate. Culpa proident \
            adipisicing id nulla nisi laboris ex in Lorem sunt duis officia \
            eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt \
            velit enim. Voluptate laboris sint cupidatat ullamco ut ea \
            consectetur et est culpa et culpa duis.</p>\n"
        );
    }
}
