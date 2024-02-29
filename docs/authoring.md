# Authoring Guide

## Markdown formatting

* Use ATX-style heading with sentence case.

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
