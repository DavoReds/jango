# Jango

A CLI application to populate your templates.

## Features

- Create templates with Jinja-inspired [Tera](https://keats.github.io/tera/)
  syntax.
- Custom `content` field that can be populated with the command line.
- Arbitrary template fields with Markdown TOML frontmatters.
- Written in Rust, btw.

## Installation

You need to have the Rust stable toolchain installed and run:

```
cargo build --release
```

This will leave the file in `target/release/jango`. You can put this file
wherever you want and run the program.

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

If you are manipulating HTML, you can use the `--escape` flag if you wish
to escape the input given to the command.

### With a Markdown file

When dealing with more complex templates, you can use a Markdown file with
a [TOML](https://toml.io/en/) frontmatter to pass any arbitrary variables
beyond the file's `content`.

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
<!DOCTYPE html>
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

You're going to need to have [Rust](https://www.rust-lang.org/) installed. I
also recommend having [Bacon](https://dystroy.org/bacon/) (to handle
recompilation) and [cargo-nextest](https://nexte.st/) (for a nicer interface
when running tests) installed.

We also try to [conventional commit](https://www.conventionalcommits.org/en/v1.0.0/) conventions when contributing to the repo.
