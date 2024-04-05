# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.2](https://github.com/DavoReds/jango/compare/v0.1.1...v0.1.2) - 2024-04-05

### Other
- release

## [0.1.1](https://github.com/DavoReds/jango/compare/v0.1.0...v0.1.1) - 2024-04-05

### Other
- Configure release-plz
- release

## [0.1.0](https://github.com/DavoReds/jango/releases/tag/v0.1.0) - 2024-04-04

### Added
- Implement the execution of the Markdown command
- Implement template rendering with markdown files
- Implement extracting frontmatter and content from markdown input
- Implement an interface for the markdown command
- Write parsings functions to extract information from markdown files
- Use color_eyre to handle errors

### Fixed
- Apply even more clippy lints
- Apply clippy lint suggestions
- Change the wording on the help command

### Other
- Add badges to README
- Create README file
- Create release-plz action to automate releases
- Run cargo-dist on the project
- Add repository to manifest
- Remove walkdir from dependencies
- Create Github actions file for CI
- Create license file
- Write a description for the program
- Replace YAML format with TOML format
- Write tests for processing a markdown file
- Add tests for an empty frontmatter
- Add rustfmt configuration
- Initial commit
