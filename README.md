# dvach
dvach is a simple client cli tool for the [2ch.hk](http://2ch.hk) imageboard.

# Installation

If you already have [rust toolchain](https://rustup.rs/) installed:
```
cargo install dvach
```
Also ensure that you have `~/.cargo/bin/` in your `$PATH`.

# Usage
```bash
$ dvach # list boards
$ dvach pr # list threads for the "pr" board
$ dvach pr 1299618 # show selected thread
```

# Why?
It's just a tiny exercise in writing cli tools in rust.
