# `git2ext`

[![](https://docs.rs/git2ext/badge.svg)](https://docs.rs/git2ext/) [![](https://img.shields.io/crates/v/git2ext.svg)](https://crates.io/crates/git2ext) [![](https://img.shields.io/crates/d/git2ext.png)](https://crates.io/crates/git2ext) [![Build Status](https://travis-ci.org/Phrohdoh/git2ext-rs.png?branch=master)](https://travis-ci.org/Phrohdoh/git2ext-rs)

`git2ext` provides a high-level, semantic API for dealing with git objects based on `git2`.

One example of the API is the ability to query a repository to determine which commit, if any, modified a path in a given way.

This project is very much a work-in-progress.

Please feel free to contribute, open issues, and discuss improvements!

## Building

To build `git2ext` you will need the Rust toolchain which can be acquired via `rustup` at [https://rustup.rs](https://rustup.rs).

You may need to restart your shell before the command-line tools are available (also make sure `$HOME/.cargo/bin` is on your `$PATH`).

Finally you may run the following command from the root of the project's source to build `git2ext`

```sh
$ cargo build
```

## Getting help

I am often present in the `#orcaware` channel on `irc.freenode.net`.

Note that this channel has many topics of discussion so you may see conversation about other projects there too.

If IRC is not your thing or you don't get a good response I am happy to respond to [GitHub issues](https://github.com/Phrohdoh/git2ext-rs/issues/new) as well.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.