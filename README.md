![GitHub License](https://img.shields.io/github/license/DavoReds/jango?style=flat-square&color=%2389dceb)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/DavoReds/jango/ci.yml?style=flat-square&logo=github&label=CI&color=%23a6e3a1)
![GitHub Release](https://img.shields.io/github/v/release/DavoReds/jango?sort=semver&display_name=release&style=flat-square&logo=github&label=github%20release&color=%2389b4fa)
![Crates.io Version](https://img.shields.io/crates/v/jango?style=flat-square&logo=rust&color=%23f9e2af)
![Crates.io MSRV](https://img.shields.io/crates/msrv/jango?style=flat-square&logo=rust&color=%23f38ba8)

# Jango

A CLI application to populate your templates.

## Features

- Create templates with Jinja-inspired [Tera](https://keats.github.io/tera/) syntax.
- Custom `content` field that can be populated with the command line.
- Arbitrary template fields with Markdown TOML frontmatters.
- Written in Rust, btw.

## Installation

### Prebuilt Binaries

#### Install prebuilt binaries via shell script

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/DavoReds/jango/releases/download/v0.1.5/jango-installer.sh | sh
```

This works for both Linux & MacOS.

#### Install prebuilt binaries via powershell script

```sh
powershell -c "irm https://github.com/DavoReds/jango/releases/download/v0.1.5/jango-installer.ps1 | iex"
```

You can also download the prebuild binaries manually from the [releases](https://github.com/DavoReds/jango/releases) page.

### Crates.io

If you have Rust installed, you can build **Jango** from source using the [crates.io](https://crates.io) registry.

```sh
cargo install jango
```

Or download a prebuilt binary with [cargo-binstall](https://github.com/cargo-bins/cargo-binstall).

```sh
cargo binstall jango
```

## Usage

### With CLI arguments

This mode allows for a template containing any of [Tera](https://keats.github.io/tera/docs/#built-ins)'s builtins and a variable called `content` that corresponds to the command line argument of the same name.

The template could look something like this:

```markdown
---
date: {{ now() | date(format="%F") }}
---

# This is a title

{{ content }}
```

And you would populate it with this command:

```sh
jango args template.md output.md -c "Hello, **world\!**"
```

Which would result in this file:

```markdown
---
date: 2024-04-05
---

# This is a title

Hello, **world!**
```

If you are manipulating HTML, you can use the `--escape` flag if you wish to escape the input given to the command.

### With a Markdown file

When dealing with more complex templates, you can use a Markdown file with a [TOML](https://toml.io/en/) frontmatter to pass any arbitrary variables beyond the file's `content`.

The template can look something like this:

```htmldjango
<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>{{ title }}</title>
  </head>
  <body>
      {{ content | indent }}
  </body>
</html>
```

You can write a Markdown file like this:

```markdown
+++
title = "This is a title"
+++

# This is a heading

This is a paragraph.
```

Then execute a command like this:

```sh
jango markdown template.html markdown.md output.html
```

Which would result in this file:

```html
<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>This is a title</title>
  </head>
  <body>
    <h1>This is a heading</h1>
    <p>This is a paragraph.</p>
  </body>
</html>
```

## Contributing

Contributions are always welcome!

You're going to need to have [Rust](https://www.rust-lang.org/) installed. I also recommend having [Bacon](https://dystroy.org/bacon/) (to handle recompilation) and [cargo-nextest](https://nexte.st/) (for a nicer interface when running tests) installed.

We also try to [conventional commit](https://www.conventionalcommits.org/en/v1.0.0/) conventions when contributing to the repo.

## (Un)License

[Unlicense](https://unlicense.org/)
