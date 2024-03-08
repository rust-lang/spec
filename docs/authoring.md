# Authoring Guide

## Markdown formatting

* Use [ATX-style headings][atx] (not Setext) with [sentence case].
* Do not use tabs, only spaces.
* Files must end with a newline.
* Lines must not end with spaces. Double spaces have semantic meaning, but can be invisible. Use a trailing backslash if you need a hard line break.
* If possible, avoid double blank lines.
* Do not use indented code blocks, use 3+ backticks code blocks instead.
* Code blocks should have an explicit language tag.
* Do not wrap long lines. This helps with reviewing diffs of the source.
* Use [smart punctuation] instead of Unicode characters. For example, use `---` for em-dash instead of the Unicode character. Characters like em-dash can be difficult to see in a fixed-width editor, and some editors may not have easy methods to enter such characters.

There are automated checks for some of these rules. Run `cargo run --manifest-path style-check/Cargo.toml -- spec` to run them locally.

[atx]: https://spec.commonmark.org/0.31.2/#atx-headings
[sentence case]: https://apastyle.apa.org/style-grammar-guidelines/capitalization/sentence-case
[smart punctuation]: https://rust-lang.github.io/mdBook/format/markdown.html#smart-punctuation

## Special markdown constructs

### Rules

Most clauses should be preceded with a rule.
Rules can be specified in the markdown source with the following on a line by itself:

```
r[foo.bar]
```

The rule name should be lowercase, with periods separating from most general to most specific (like `r[array.repeat.zero]`).

Rules can be linked to by their ID using markdown such as `[foo.bar]`. There are automatic link references so that any rule can be referred to from any page in the book.

In the HTML, the rules are clickable just like headers.

### Admonitions

Admonitions use a style similar to GitHub-flavored markdown, where the style name is placed at the beginning of a blockquote, such as:

```
> [!WARNING]
> This is a warning.
```

All this does is apply a CSS class to the blockquote. You should define the color or style of the rule in the `css/custom.css` file if it isn't already defined.
