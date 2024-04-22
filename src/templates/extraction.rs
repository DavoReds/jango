use color_eyre::eyre::eyre;
use markdown::mdast::Node;

/// Given the root node of a Markdown ast, returns the String representing its TOML frontmatter if
/// present.
///
/// # Errors
///
/// Returns an error if the input is an empty tree, if the tree contains no frontmatter node or if
/// this frontmatter is in YAML instead of TOML.
#[allow(clippy::option_if_let_else)]
pub fn extract_md_frontmatter(root: &Node) -> color_eyre::Result<String> {
    match root.children() {
        Some(children) => match children.first() {
            Some(Node::Toml(frontmatter)) => Ok(frontmatter.value.clone()),
            Some(Node::Yaml(_)) => {
                Err(eyre!("Invalid frontmatter type. It must be TOML"))
            }
            _ => Err(eyre!("Frontmatter not present")),
        },
        None => Err(eyre!("Invalid MarkDown file")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::templates::default_md_parse_options;
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

    #[rstest]
    fn extract_frontmatter_works_on_a_valid_file(md_test: &str) {
        let tree =
            markdown::to_mdast(md_test, &default_md_parse_options(false))
                .expect("This should not fail");

        let result = extract_md_frontmatter(&tree);
        assert!(result.is_ok());

        let result = result.expect("Failed to extract frontmatter");
        assert_eq!(
            result,
            "description = \"This is a test note\"\ndate = \"2024-04-03\""
        );
    }

    #[test]
    fn extract_frontmatter_works_with_empty_frontmatter() {
        let input = "+++\n+++\n\n# This is a heading\n\nThis is a paragraph";
        let tree = markdown::to_mdast(input, &default_md_parse_options(false))
            .expect("This should not fail");

        let result = extract_md_frontmatter(&tree);
        assert!(result.is_ok());
    }

    #[test]
    fn extract_frontmatter_errors_when_no_frontmatter_is_present() {
        let input = "# This is a title\n\nThis is a paragraph";
        let tree = markdown::to_mdast(input, &default_md_parse_options(false))
            .expect("This should not fail");

        let result = extract_md_frontmatter(&tree);
        assert!(result.is_err());
    }

    #[test]
    fn extract_frontmatter_errors_on_empty_input() {
        let input = "";
        let tree = markdown::to_mdast(input, &default_md_parse_options(false))
            .expect("This should not fail");

        let result = extract_md_frontmatter(&tree);
        assert!(result.is_err());
    }

    #[rstest]
    fn extract_frontmatter_errors_on_frontmatter_of_different_type(
        yaml_test: &str,
    ) {
        let tree =
            markdown::to_mdast(yaml_test, &default_md_parse_options(false))
                .expect("This should not fail");

        let result = extract_md_frontmatter(&tree);
        assert!(result.is_err());
    }
}
