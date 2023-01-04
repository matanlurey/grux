//! A library for drawing grid-based user interfaces using ASCII characters.
//!
//! Note that all that is provided is the [`GridWriter`] trait, which is implemented for a few
//! common data structures. The trait is intentionally simple, and is intended to be implemented
//! for other data structures as needed.
//!
//! # Examples
//!
//! ## Using a fixed-size nested array
//!
//! > 💡 **TIP**: Use a fixed-size nested array for a grid dimensions known ahead of time.
//! >
//! > - Nested arrays will be faster and more efficient than a growable grid
//! > - Nested arrays support `Display` trait for cells, which means graphemes are supported and
//! >   ANSI escape codes can be used for colors (see `examples/emojis.rs` and `examples/ansi.rs`).
//!
//! ```
//! use grux::GridWriter;
//!
//! // Create a 2x2 array of zeros.
//! let mut array = [[0; 2]; 2];
//!
//! // Set the element at (1, 1) to 1.
//! array.draw((1, 1), 1);
//! assert_eq!(array, [[0, 0], [0, 1]]);
//! ```
//!
//! ## Using a growable nested vector
//!
//! > 💡 **TIP**: Use a growable nested vector for a grid dimensions not known ahead of time.
//! >
//! > - Nested vectors will be slower and less efficient than a fixed-size grid
//! > - Nested vectors support `Display` trait for cells, which means graphemes are supported
//! > - A rectangular grid is not guaranteed
//!
//! ```
//! use grux::GridWriter;
//!
//! // Create an empty vector (of vectors)
//! let mut vec: Vec<Vec<i32>> = Vec::new();
//!
//! // Set the element at (1, 1) to 1.
//! // This will grow the vector to fit the position, adding empty default vectors as needed.
//! vec.draw((1, 1), 1);
//! assert_eq!(vec, vec![vec![], vec![0, 1]]);
//! ```
//!
//! ## Using a growable string
//!
//! > ⚠️ **WARNING**: Only supports ASCII characters (`char`) and not graphemes or ANSI escape codes.
//! >
//! > - Strings are not as efficient or flexible as nested arrays or vectors
//! > - Strings do not support graphemes or ANSI escape codes
//! > - A rectangular grid is not guaranteed
//! >
//! > See [print any grid to a output stream](#print-any-grid-to-a-output-stream) for alternatives.
//!
//! ```
//! use grux::GridWriter;
//!
//! // Create an empty string.
//! let mut string = String::new();
//!
//! // Set the element at (1, 2) to '1'.
//! // This will grow the string to fit the position, adding empty lines as needed.
//! string.draw((1, 2), '1');
//! assert_eq!(string, "\n\n 1");
//! ```
//!
//! ## Print any grid to a output stream
//!
//! Any type that implements [`DisplayGrid`] can be printed to a output stream _or_ a new string.
//!
//! ```
//! use grux::DisplayGrid;
//!
//! // Create a 3x3 array of the letters 'A' - 'I'.
//! let mut array = [['A', 'B', 'C'], ['D', 'E', 'F'], ['G', 'H', 'I']];
//!
//! // Convert the array to a string.
//! // TIP: Use `print` instead if you want to print to a output stream.
//! let string = array.to_string().unwrap();
//!
//! assert_eq!(string, "ABC\nDEF\nGHI\n");
//! ```

use std::{fmt::Display, string::FromUtf8Error};

#[cfg(test)]
mod tests;

/// A trait for a grid-like writable buffer, typically with a fixed width and height.
///
/// The grid is indexed by `(x, y)` coordinates, where `x` is the column and `y` is the row.
///
/// # Examples
///
/// The provided structs and implementations are likely sufficient, but as an example:
///
/// ```
/// # use grux::GridWriter;
/// struct MyGrid {
///     width: usize,
///     height: usize,
///     data: Vec<char>,
/// }
///
/// impl GridWriter for MyGrid {
///     type Element = char;
///
///     fn draw(&mut self, position: (usize, usize), element: Self::Element) {
///         let (x, y) = position;
///         self.data[y * self.width + x] = element;
///     }
/// }
/// ```
pub trait GridWriter {
    /// The type of the elements in the grid, e.g. `char`; must implement `Display`.
    type Element: Display;

    /// Sets the element at the given `(x, y)` position.
    ///
    /// How the position is interpreted is up to the implementor; for example, it could grow the
    /// grid to fit the position, or it could panic if the position is out of bounds. See the
    /// documentation for the implementor for more information.
    fn draw(&mut self, position: (usize, usize), element: Self::Element);
}

/// A trait that can be used to display a grid-like buffer to a output stream or a new string.
pub trait DisplayGrid {
    /// Returns a UTF-8 string representation of the grid.
    ///
    /// Each row is separated by a newline (`\n`), including the last row.
    ///
    /// # Performance
    ///
    /// Equivalent to calling `print` with a new vector, but is provided for convenience. If...
    ///
    /// - The grid is large
    /// - The grid will be printed to an output stream (e.g. `stdout`)
    /// - Memory is a concern
    ///
    /// ... then it is recommended to use `print` instead (or provide a custom `to_string`).
    ///
    /// # Errors
    ///
    /// Returns an error if the grid contains invalid UTF-8.
    ///
    /// # Examples
    ///
    /// ```
    /// # use grux::DisplayGrid;
    /// let mut grid = [['A', 'B', 'C'], ['D', 'E', 'F'], ['G', 'H', 'I']];
    ///
    /// assert_eq!(grid.to_string().unwrap(), "ABC\nDEF\nGHI\n");
    /// ```
    fn to_string(&self) -> Result<String, FromUtf8Error> {
        let mut output = Vec::new();
        self.print(&mut output).unwrap();
        String::from_utf8(output)
    }

    /// Formats the grid into the given formatter.
    ///
    /// Each row is separated by a newline (`\n`), including the last row.
    ///
    /// # Errors
    ///
    /// Returns an error if the output stream returns an error.
    ///
    /// # Examples
    ///
    /// ```
    /// # use grux::DisplayGrid;
    /// let mut grid = [['A', 'B', 'C'], ['D', 'E', 'F'], ['G', 'H', 'I']];
    ///
    /// // Print the grid to a vector (which can be substituted for say, stdout).
    /// let mut output = Vec::new();
    /// grid.print(&mut output).unwrap();
    ///
    /// assert_eq!(output, b"ABC\nDEF\nGHI\n");
    /// ```
    fn print(&self, stream: &mut impl std::io::Write) -> std::io::Result<()>;
}

/// Provides [`GridWriter`] for a fixed-size nested array of elements.
///
/// The outer array is assumed to be the rows, and the inner array is assumed to be the columns.
///
/// > ⓘ **NOTE**: While this doesn't seem like an intuitive way to index arrays (since it would be
/// > more natural to index by `[y][x]`, this implementation allows nested arrays to be used the
/// > same way as other data structures, i.e. the point of this library.
///
/// # Examples
///
/// ```
/// # use grux::GridWriter;
/// let mut array = [[0; 2]; 2];
///
/// // Set the element at (1, 1) to 1.
/// array.draw((1, 1), 1);
///
/// assert_eq!(array, [[0, 0], [0, 1]]);
/// ```
impl<const W: usize, const H: usize, T> GridWriter for [[T; W]; H]
where
    T: Display,
{
    type Element = T;

    /// Sets the element at the given `(x, y)` position.
    ///
    /// # Panics
    ///
    /// If the position is out of bounds.
    fn draw(&mut self, position: (usize, usize), element: Self::Element) {
        let (x, y) = position;
        self[y][x] = element;
    }
}

/// Provides [`DisplayGrid`] for a fixed-size nested array of elements.
impl<const W: usize, const H: usize, T> DisplayGrid for [[T; W]; H]
where
    T: Display,
{
    fn print(&self, stream: &mut impl std::io::Write) -> std::io::Result<()> {
        for row in self {
            for element in row {
                write!(stream, "{}", element)?;
            }
            writeln!(stream)?;
        }
        Ok(())
    }
}

/// Provides [`GridWriter`] for a growable nested vector of elements.
///
/// The outer vector is assumed to be the rows, and the inner vector is assumed to be the columns.
///
/// Unlike fixed-size nested arrays, this implementation will grow the grid to fit the position;
/// this is useful for drawing to a grid that is not known ahead of time. As such, the element is
/// required to implement [`Default`] and [`Clone`].
///
/// > ⓘ **NOTE**: While this doesn't seem like an intuitive way to index vectors (since it would be
/// > more natural to index by `[y][x]`, this implementation allows nested vectors to be used the
/// > same way as other data structures, i.e. the point of this library.
///
/// # Limitations
///
/// A rectangular grid is not guaranteed. See the examples below for details.
///
/// # Examples
///
/// ```
/// # use grux::GridWriter;
/// let mut vec: Vec<Vec<i32>> = Vec::new();
///
/// // Set the element at (1, 1) to 1.
/// // This will grow the vector to fit the position, adding empty default vectors as needed.
/// vec.draw((1, 1), 1);
///
/// assert_eq!(vec, vec![vec![], vec![0, 1]]);
/// ```
impl<T> GridWriter for Vec<Vec<T>>
where
    T: Display + Default + Clone,
{
    type Element = T;

    /// Sets the element at the given `(x, y)` position.
    ///
    /// If the position is out of bounds, the grid will be resized to fit the position.
    fn draw(&mut self, position: (usize, usize), element: Self::Element) {
        let (x, y) = position;

        if y >= self.len() {
            self.resize_with(y + 1, Vec::new);
        }

        let row = &mut self[y];

        if x >= row.len() {
            row.resize(x + 1, T::default());
        }

        row[x] = element;
    }
}

/// Provides [`DisplayGrid`] for a growable nested vector of elements.
impl<T> DisplayGrid for Vec<Vec<T>>
where
    T: Display + Default + Clone,
{
    fn print(&self, stream: &mut impl std::io::Write) -> std::io::Result<()> {
        for row in self {
            for element in row {
                write!(stream, "{}", element)?;
            }
            writeln!(stream)?;
        }
        Ok(())
    }
}

/// Provides [`GridWriter`] for a growable string of characters.
///
/// Unlike fixed-size nested arrays, this implementation will grow the grid to fit the position;
/// this is useful for drawing to a grid that is not known ahead of time. "Empty" characters are
/// assumed to be spaces (`' '`).
///
/// # Limitations
///
/// This implementation assumes that the string is a grid of characters, where each line is a row
/// and each character is a column. This means that the string must be a valid UTF-8 string, and
/// that the string cannot contain multi-byte characters (i.e. graphemes or ANSI escape sequences).
///
/// Additionally, a rectangular grid is not guaranteed. See the examples below for details.
///
/// # Performance
///
/// This implementation is not optimized for performance, and is intended for use in small grids
/// (e.g. a 10x10 grid) or for prototyping. For larger grids, consider using a fixed-size nested
/// array or a growable vector.
///
/// # Examples
///
/// ```
/// # use grux::GridWriter;
/// let mut string = String::new();
///
/// // Set the element at (1, 1) to 'X'.
/// // This will grow the string to fit the position, adding empty spaces as needed.
/// string.draw((1, 1), 'X');
///
/// assert_eq!(string, "\n X");
/// ```
impl GridWriter for String {
    type Element = char;

    /// Sets the element at the given `(x, y)` position.
    ///
    /// If the position is out of bounds, the grid will be resized to fit the position.
    fn draw(&mut self, position: (usize, usize), element: Self::Element) {
        let (x, y) = position;

        // Create a vector of the rows (i.e lines) in the string.
        let mut rows: Vec<&str> = self.lines().collect();

        // Grow the rows if necessary.
        while rows.len() <= y {
            rows.push("");
        }

        // Replace the y-th row with a new row that is the same as the old row, but with the element
        // at the x-th position replaced with the new element.
        let mut row = rows[y].to_string();

        // Grow the row if necessary, using spaces for the new characters.
        while row.len() <= x {
            row.push(' ');
        }

        // Replace the x-th character with the new element.
        row.replace_range(x..=x, &element.to_string());

        // Replace the y-th row with the new row, trimming any trailing whitespace.
        rows[y] = row.trim_end();

        // Replace the string with the new rows.
        *self = rows.join("\n");
    }
}

/// Provides [`DisplayGrid`] for a growable string of characters.
///
/// > ⓘ **NOTE**: This implementation is provided for consistency, but it's already a string, so...
impl DisplayGrid for String {
    fn to_string(&self) -> Result<String, FromUtf8Error> {
        Ok(self.clone())
    }

    fn print(&self, stream: &mut impl std::io::Write) -> std::io::Result<()> {
        write!(stream, "{}", self)
    }
}
