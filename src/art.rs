//! Configure and draw simple ASCII or ASCII-like* sprites to a 2D grid.
//!
//! [`grux::art`][`crate::art`] provides:
//! - A uniform interface for drawing art to a 2D grid: [`Sprite`].
//! - Built-in art types e.g. [`Line`], [`FilledRect`], [`BorderRect`].
//!
//! > ⓘ **NOTE**: The art types in this module are _not_ limited to ASCII characters.
//! >
//! > For example, you can use [`Line`] to draw a line of unicode characters, or even a line of a
//! > more complex type; for example a custom `Cell` struct in your own data structure. ASCII is
//! > just a way to understand the way the sprites are drawn.

use std::fmt::Display;

use crate::GridWriter;

/// A trait for types that can be drawn to a 2D grid.
///
/// # Examples
///
/// A custom type that implements [`Sprite`], for example a pre-configured unicode box:
///
/// ```
/// # use grux::art::Sprite;
/// # use grux::{GridWriter};
/// struct AsciiBoxExample;
///
/// // Not a super useful example, but it's a start.
/// impl Sprite for AsciiBoxExample {
///    type Element = char;
///
///    fn width(&self) -> usize {
///        3
///    }
///
///    fn height(&self) -> usize {
///        3
///    }
///
///    fn draw_to(&self, position: (usize, usize), to: &mut impl GridWriter<Element = Self::Element>) {
///        to.set((position.0 + 0, position.1 + 0), '╔');
///        to.set((position.0 + 1, position.1 + 0), '═');
///        to.set((position.0 + 2, position.1 + 0), '╗');
///        to.set((position.0 + 0, position.1 + 1), '║');
///        to.set((position.0 + 2, position.1 + 1), '║');
///        to.set((position.0 + 0, position.1 + 2), '╚');
///        to.set((position.0 + 1, position.1 + 2), '═');
///        to.set((position.0 + 2, position.1 + 2), '╝');
///        to.set((position.0 + 1, position.1 + 1), ' ');
///    }
/// }
///
/// let mut grid = [[' '; 3]; 3];
/// AsciiBoxExample.draw_to((0, 0), &mut grid);
///
/// assert_eq!(grid, [
///    ['╔', '═', '╗'],
///    ['║', ' ', '║'],
///    ['╚', '═', '╝'],
/// ]);
/// ```
pub trait Sprite {
    type Element: Clone;

    /// The width of the element.
    #[must_use]
    fn width(&self) -> usize;

    /// The height of the element.
    #[must_use]
    fn height(&self) -> usize;

    /// Draws the given element to the grid at the given `(x. y)` position.
    fn draw_to(&self, position: (usize, usize), to: &mut impl GridWriter<Element = Self::Element>);
}

/// A structured way to draw a line to a 2D grid.
///
/// # Examples
///
/// ```
/// # use grux::art::{Line, Sprite};
/// # use grux::{GridWriter};
/// let mut grid = [[' '; 3]; 4];
///
/// let line = Line::horizontal(3, '═');
/// line.draw_to((0, 0), &mut grid);
/// line.draw_to((0, 3), &mut grid);
///
/// let line = Line::vertical(2, '║');
/// line.draw_to((0, 1), &mut grid);
/// line.draw_to((2, 1), &mut grid);
///
/// assert_eq!(grid, [
///     ['═', '═', '═'],
///     ['║', ' ', '║'],
///     ['║', ' ', '║'],
///     ['═', '═', '═']
/// ]);
/// ```
pub struct Line<T: Display> {
    length: usize,
    render: T,
    orientation: Orientation,
}

/// Options for drawing a line to a 2D grid.
enum Orientation {
    /// Left to right.
    Horizontal,

    /// Top to bottom.
    Vertical,
}

impl<T: Display> Line<T> {
    /// Configures a horizontal line of the given length.
    #[must_use]
    pub fn horizontal(length: usize, render: T) -> Self {
        Self {
            length,
            render,
            orientation: Orientation::Horizontal,
        }
    }

    /// Configures a vertical line of the given length.
    #[must_use]
    pub fn vertical(length: usize, render: T) -> Self {
        Self {
            length,
            render,
            orientation: Orientation::Vertical,
        }
    }
}

impl<T: Display + Clone> Sprite for Line<T> {
    type Element = T;

    fn width(&self) -> usize {
        match self.orientation {
            Orientation::Horizontal => self.length,
            Orientation::Vertical => 1,
        }
    }

    fn height(&self) -> usize {
        match self.orientation {
            Orientation::Horizontal => 1,
            Orientation::Vertical => self.length,
        }
    }

    fn draw_to(&self, position: (usize, usize), to: &mut impl GridWriter<Element = Self::Element>) {
        let (x, y) = position;

        match self.orientation {
            Orientation::Horizontal => {
                for i in 0..self.length {
                    to.set((x + i, y), self.render.clone());
                }
            }
            Orientation::Vertical => {
                for i in 0..self.length {
                    to.set((x, y + i), self.render.clone());
                }
            }
        }
    }
}

/// A structured way to draw a filled rectangle to a 2D grid.
///
/// If you want to draw a rectangle that is just a border, see [`BorderRect`].
///
/// # Examples
///
/// ```
/// # use grux::art::{FillRect, Sprite};
/// # use grux::{GridWriter};
/// let mut grid = [[' '; 4]; 4];
///
/// let rect = FillRect::new(2, 2, '█');
///
/// rect.draw_to((1, 1), &mut grid);
///
/// assert_eq!(grid, [
///     [' ', ' ', ' ', ' '],
///     [' ', '█', '█', ' '],
///     [' ', '█', '█', ' '],
///     [' ', ' ', ' ', ' '],
/// ]);
/// ```
pub struct FillRect<T: Display> {
    width: usize,
    height: usize,
    render: T,
}

impl<T: Display> FillRect<T> {
    /// Configures a filled rectangle of the given width and height.
    #[must_use]
    pub fn new(width: usize, height: usize, render: T) -> Self {
        Self {
            width,
            height,
            render,
        }
    }
}

impl<T: Display + Clone> Sprite for FillRect<T> {
    type Element = T;

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn draw_to(&self, position: (usize, usize), to: &mut impl GridWriter<Element = Self::Element>) {
        let (x, y) = position;

        for i in 0..self.width {
            for j in 0..self.height {
                to.set((x + i, y + j), self.render.clone());
            }
        }
    }
}

/// A structured way to draw a bordered rectangle to a 2D grid.
///
/// # Examples
///
/// ```
/// # use grux::art::{BorderRect, Sprite};
/// # use grux::{GridWriter};
/// let mut grid = [[' '; 4]; 4];
///
/// let rect = BorderRect::new(4, 4, ['╔', '═', '╗', '║', '║', '╚', '═', '╝']);
/// rect.draw_to((0, 0), &mut grid);
///
/// assert_eq!(grid, [
///     ['╔', '═', '═', '╗'],
///     ['║', ' ', ' ', '║'],
///     ['║', ' ', ' ', '║'],
///     ['╚', '═', '═', '╝'],
/// ]);
/// ```
pub struct BorderRect<T: Display> {
    width: usize,
    height: usize,
    render: [T; 8],
}

impl<T: Display> BorderRect<T> {
    /// Configures a bordered rectangle of the given width and height.
    ///
    /// The render array should be in the following order:
    /// - Top left corner
    /// - Top border
    /// - Top right corner
    /// - Left border
    /// - Right border
    /// - Bottom left corner
    /// - Bottom border
    /// - Bottom right corner
    ///
    /// # Panics
    ///
    /// If the width or height is less than 2.
    #[must_use]
    pub fn new(width: usize, height: usize, render: [T; 8]) -> Self {
        assert!(width >= 2, "Width must be at least 2");
        assert!(height >= 2, "Height must be at least 2");
        Self {
            width,
            height,
            render,
        }
    }
}
impl<T: Display + Clone> BorderRect<T> {
    fn top_left(&self) -> T {
        self.render[0].clone()
    }

    fn top(&self) -> T {
        self.render[1].clone()
    }

    fn top_right(&self) -> T {
        self.render[2].clone()
    }

    fn left(&self) -> T {
        self.render[3].clone()
    }

    fn right(&self) -> T {
        self.render[4].clone()
    }

    fn bottom_left(&self) -> T {
        self.render[5].clone()
    }

    fn bottom(&self) -> T {
        self.render[6].clone()
    }

    fn bottom_right(&self) -> T {
        self.render[7].clone()
    }
}

impl<T: Display + Clone> Sprite for BorderRect<T> {
    type Element = T;

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn draw_to(&self, position: (usize, usize), to: &mut impl GridWriter<Element = Self::Element>) {
        let (x, y) = position;
        let width = self.width();
        let height = self.height();

        // Top Middle and Bottom Middle
        for i in 1..width - 1 {
            to.set((x + i, y), self.top());
            to.set((x + i, y + height - 1), self.bottom());
        }

        // Left Side and Right Side
        for i in 1..height - 1 {
            to.set((x, y + i), self.left());
            to.set((x + width - 1, y + i), self.right());
        }

        // Corners
        to.set((x, y), self.top_left());
        to.set((x + width - 1, y), self.top_right());
        to.set((x, y + height - 1), self.bottom_left());
        to.set((x + width - 1, y + height - 1), self.bottom_right());
    }
}
