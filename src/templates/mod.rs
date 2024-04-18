mod extraction;
mod parsing;
mod processing;

pub use extraction::*;
use markdown::{CompileOptions, Constructs, Options, ParseOptions};
pub use parsing::*;
pub use processing::*;

fn default_md_options(inline_html: bool) -> Options {
    Options {
        parse: default_md_parse_options(inline_html),
        compile: CompileOptions {
            allow_dangerous_html: inline_html,
            ..CompileOptions::gfm()
        },
    }
}

fn default_md_parse_options(inline_html: bool) -> ParseOptions {
    ParseOptions {
        constructs: Constructs {
            frontmatter: true,
            html_flow: inline_html,
            html_text: inline_html,
            ..Constructs::gfm()
        },
        ..ParseOptions::gfm()
    }
}
