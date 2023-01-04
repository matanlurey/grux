fn main() {
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

    // Print the grid.
    println!("{}", grid.to_string().unwrap());
}
