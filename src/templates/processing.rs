use super::{
    default_md_parse_options, extract_md_frontmatter, parse_md_content,
    parse_md_frontmatter,
};

/// Processes a markdown `&str` containing a TOML frontmatter and returns a tuple with a `Table`
/// that corresponds to the frontmatter and a String corresponding to the compiled HTML of the
/// input's contents.
///
/// # Errors
///
/// This function returns an error if it's unable to extract the TOML frontmatter or if the input's
/// frontmatter is not valid TOML.
#[allow(clippy::missing_panics_doc)]
pub fn process_md_file(
    input: &str,
    inline_html: bool,
) -> color_eyre::Result<(toml::Table, String)> {
    let ast = markdown::to_mdast(input, &default_md_parse_options(inline_html))
        .expect("This should never fail");
    let frontmatter = extract_md_frontmatter(&ast)?;
    let frontmatter = parse_md_frontmatter(&frontmatter)?;

    let content = parse_md_content(input, inline_html);

    Ok((frontmatter, content))
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
    fn yaml_test() -> &'static str {
        include_str!("yaml_test.md")
    }

    #[fixture]
    #[once]
    fn html_test() -> &'static str {
        include_str!("html_test.md")
    }

    #[rstest]
    fn process_md_file_works_on_a_file_with_a_frontmatter(md_test: &str) {
        let result = process_md_file(md_test, false);
        assert!(result.is_ok());

        let (frontmatter, content) =
            result.expect("Failed to parse markdown file");

        assert_eq!(
            frontmatter["description"].as_str(),
            Some("This is a test note")
        );
        assert_eq!(frontmatter["date"].as_str(), Some("2024-04-03"));

        assert_eq!(
            content,
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

    #[test]
    fn process_md_file_works_on_a_file_with_an_empty_frontmatter() {
        let input = "+++\n+++\n# This is a heading\n\nThis is a paragraph";

        let result = process_md_file(input, false);
        assert!(result.is_ok());

        let (_, content) = result.expect("Failed to parse markdown input");
        assert_eq!(
            content,
            "<h1>This is a heading</h1>\n<p>This is a paragraph</p>"
        );
    }

    #[rstest]
    fn process_md_file_errors_on_a_file_with_an_invalid_frontmatter(
        yaml_test: &str,
    ) {
        let result = process_md_file(yaml_test, false);
        assert!(result.is_err());
    }

    #[test]
    fn process_md_file_errors_with_empty_input() {
        let input = "";

        let result = process_md_file(input, false);
        assert!(result.is_err());
    }

    #[test]
    fn process_md_file_errors_when_frontmatter_is_not_present() {
        let input = "# This is a heading\n\nThis is a paragraph";

        let result = process_md_file(input, false);
        assert!(result.is_err());
    }
}
