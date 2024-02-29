# The Rust Specification

## Building

The specification uses [mdBook] to process the source into HTML. See [mdBook Installation] for more information on installing mdBook. To build the book, run:

```sh
mdbook build
```

This will output the HTML into a directory called `book`.

For authors, consider using the server functionality which supports automatic reload:

```sh
mdbook serve --open
```

This will open a browser with a websocket live-link to automatically reload whenever the source is updated.

[mdBook]: https://rust-lang.github.io/mdBook/
[mdBook Insallation]: https://rust-lang.github.io/mdBook/guide/installation.html

## License

The Rust Specification is distributed under the terms of both the MIT license and the Apache license (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.
