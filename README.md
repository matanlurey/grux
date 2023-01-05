# grux

A library for drawing grid-based user interfaces using ASCII characters.

[![Rust Checks](https://github.com/matanlurey/grux/actions/workflows/rust.yml/badge.svg)](https://github.com/matanlurey/grux/actions/workflows/rust.yml)
[![Coverage Status](https://coveralls.io/repos/github/matanlurey/grux/badge.svg)](https://coveralls.io/github/matanlurey/grux)
[![Current Crates.io Version](https://img.shields.io/crates/v/grux.svg)](https://crates.io/crates/grux)
[![Docs](https://docs.rs/grux/badge.svg)](https://docs.rs/grux/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

<!-- TODO: Sync this somehow with examples/array.rs -->

```rs
// Provides a uniform interface for drawing to a 2D grid.
use grux::GridWriter;

// Create a 3x3 2D array.
// Alternatives provided by `grux`: `Vec<Vec<T>>` and `String`.
let mut grid = [[' '; 3]; 3];

// Draw some random stuff. In practice, you'd probably use the `Sprite` trait.
grid.set((0, 0), '╔');
grid.set((1, 0), '═');
grid.set((2, 0), '╗');
grid.set((0, 1), '║');
grid.set((2, 1), '║');
grid.set((0, 2), '╚');
grid.set((1, 2), '═');
grid.set((2, 2), '╝');

// Provides a uniform interface for displaying a 2D grid.
use grux::DisplayGrid;

// ╔═╗
// ║ ║
// ╚═╝
println!("{}", grid.to_string().unwrap());
```

See the [examples](examples/) directory for more, including built-in sprites.

## Why Grux?

There are plenty of existing libraries for terminal-based user interfaces, but
none of them are quite what I wanted. I wanted a library that would let me draw
a grid of cells, each of which could contain a single character.

Importantly, **Grux _isn't_ a UI framework**.

It doesn't handle input or even output. It just lets you draw to a grid-like
structure, which could be:

- A `Vec<Vec<T>>`
- A `String`
- A fixed-size 2D array (i.e. `[[T; 10]; 10]`)
- Your custom data structure (just implement `GridWriter` and/or `DisplayGrid`)

**tl;dr**: Draw to whatever you want, and build on top of it (or replace it).
