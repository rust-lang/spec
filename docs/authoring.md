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

### Standard library links

You should link to the standard library without specifying a URL in a fashion similar to [rustdoc intra-doc links][intra]. Some examples:

```
Link to Option is [`std::option::Option`]

You can include generics, they are ignored, like [`std::option::Option<T>`]

You can shorthand things if you don't want the full path in the text,
like [`Option`](std::option::Option).

Macros can use `!`, which also works for disambiguation,
like [`alloc::vec!`] is the macro, not the module.

Explicit namespace disambiguation is also supported, such as [`std::vec`](mod@std::vec).
```

[intra]: https://doc.rust-lang.org/rustdoc/write-documentation/linking-to-items-by-name.html

### Admonitions

Admonitions use a style similar to GitHub-flavored markdown, where the style name is placed at the beginning of a blockquote, such as:

```
> [!WARNING]
> This is a warning.
```

All this does is apply a CSS class to the blockquote. You should define the color or style of the rule in the `css/custom.css` file if it isn't already defined.
