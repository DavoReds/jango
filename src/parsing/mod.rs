use color_eyre::eyre::{eyre, Context};
use markdown::{mdast::Node, Constructs, Options, ParseOptions};

fn default_md_options() -> Options {
    Options {
        parse: default_md_parse_options(),
        ..Options::gfm()
    }
}

fn default_md_parse_options() -> ParseOptions {
    ParseOptions {
        constructs: Constructs {
            frontmatter: true,
            ..Constructs::gfm()
        },
        ..ParseOptions::gfm()
    }
}

#[allow(clippy::option_if_let_else)]
fn extract_md_frontmatter(root: &Node) -> color_eyre::Result<String> {
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

fn parse_md_frontmatter(frontmatter: &str) -> color_eyre::Result<toml::Table> {
    toml::from_str(frontmatter).wrap_err("Failed to parse TOML fragment")
}

fn parse_md_content(input: &str) -> String {
    markdown::to_html_with_options(input, &default_md_options())
        .expect("This should never fail")
}

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
) -> color_eyre::Result<(toml::Table, String)> {
    let ast = markdown::to_mdast(input, &default_md_parse_options())
        .expect("This should never fail");
    let frontmatter = extract_md_frontmatter(&ast)?;
    let frontmatter = parse_md_frontmatter(&frontmatter)?;

    let content = parse_md_content(input);

    Ok((frontmatter, content))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_frontmatter_works_on_a_valid_file() {
        let input = include_str!("test.md");
        let tree = markdown::to_mdast(input, &default_md_parse_options())
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
        let tree = markdown::to_mdast(input, &default_md_parse_options())
            .expect("This should not fail");

        let result = extract_md_frontmatter(&tree);
        assert!(result.is_ok());
    }

    #[test]
    fn extract_frontmatter_errors_when_no_frontmatter_is_present() {
        let input = "# This is a title\n\nThis is a paragraph";
        let tree = markdown::to_mdast(input, &default_md_parse_options())
            .expect("This should not fail");

        let result = extract_md_frontmatter(&tree);
        assert!(result.is_err());
    }

    #[test]
    fn extract_frontmatter_errors_on_empty_input() {
        let input = "";
        let tree = markdown::to_mdast(input, &default_md_parse_options())
            .expect("This should not fail");

        let result = extract_md_frontmatter(&tree);
        assert!(result.is_err());
    }

    #[test]
    fn extract_frontmatter_errors_on_frontmatter_of_different_type() {
        let input = include_str!("yaml_test.md");
        let tree = markdown::to_mdast(input, &default_md_parse_options())
            .expect("This should not fail");

        let result = extract_md_frontmatter(&tree);
        assert!(result.is_err());
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
    fn parse_frontmatter_errors_with_invalid_yaml() {
        let input = "This is for a test\nvalid: false";

        let result = parse_md_frontmatter(input);
        assert!(result.is_err());
    }

    #[test]
    fn parse_contents_works_with_simple_markdown() {
        let input =
            "# This is a title\n\nThis is a paragraph with a **bold** word.";

        let result = parse_md_content(input);
        assert_eq!(
            result,
            "<h1>This is a title</h1>\n<p>This is a paragraph \
            with a <strong>bold</strong> word.</p>"
        );
    }

    #[test]
    fn parse_contents_works_on_a_file_with_a_frontmatter() {
        let input = include_str!("test.md");

        let result = parse_md_content(input);
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

    #[test]
    fn process_md_file_works_on_a_file_with_a_frontmatter() {
        let input = include_str!("test.md");

        let result = process_md_file(input);
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
        )
    }

    #[test]
    fn process_md_file_works_on_a_file_with_an_empty_frontmatter() {
        let input = "+++\n+++\n# This is a heading\n\nThis is a paragraph";

        let result = process_md_file(input);
        assert!(result.is_ok());

        let (_, content) = result.expect("Failed to parse markdown input");
        assert_eq!(
            content,
            "<h1>This is a heading</h1>\n<p>This is a paragraph</p>"
        );
    }

    #[test]
    fn process_md_file_errors_on_a_file_with_an_invalid_frontmatter() {
        let input = include_str!("yaml_test.md");

        let result = process_md_file(input);
        assert!(result.is_err());
    }

    #[test]
    fn process_md_file_errors_with_empty_input() {
        let input = "";

        let result = process_md_file(input);
        assert!(result.is_err());
    }

    #[test]
    fn process_md_file_errors_when_frontmatter_is_not_present() {
        let input = "# This is a heading\n\nThis is a paragraph";

        let result = process_md_file(input);
        assert!(result.is_err());
    }
}
