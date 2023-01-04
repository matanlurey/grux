# grux

A library for drawing grid-based user interfaces using ASCII characters.

<!-- TODO: Sync this somehow with examples/array.rs -->

```rs
// Provides a uniform interface for drawing to a 2D grid.
use grux::GridWriter;

// Create a 3x3 2D array.
// Alternatives provided by `grux`: `Vec<Vec<T>>` and `String`.
let mut grid = [[' '; 3]; 3];

// Draw some random stuff. In practice, you'd probably use a loop :P.
grid.draw((0, 0), '╔');
grid.draw((1, 0), '═');
grid.draw((2, 0), '╗');
grid.draw((0, 1), '║');
grid.draw((2, 1), '║');
grid.draw((0, 2), '╚');
grid.draw((1, 2), '═');
grid.draw((2, 2), '╝');

// Provides a uniform interface for displaying a 2D grid.
use grux::DisplayGrid;

// ╔═╗
// ║ ║
// ╚═╝
println!("{}", grid.to_string().unwrap());
```

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
