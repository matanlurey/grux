fn main() {
    // Create a 3x3 2D array.
    // Alternatives provided by `grux`: `Vec<Vec<T>>` and `String`.
    let mut grid = [[' '; 4]; 4];

    // Draw some shapes using the `Sprite` trait.
    use grux::art::{BorderRect, FillRect, Sprite};

    // Draw a bordered rectangle.
    let rect = BorderRect::new(4, 4, ['╔', '═', '╗', '║', '║', '╚', '═', '╝']);
    rect.draw_to((0, 0), &mut grid);

    // Draw a filled rectangle in the middle.
    let rect = FillRect::new(2, 2, '█');
    rect.draw_to((1, 1), &mut grid);

    // Provides a uniform interface for displaying a 2D grid.
    use grux::DisplayGrid;

    // Print the grid.
    // ╔══╗
    // ║██║
    // ║██║
    // ╚══╝
    println!("{}", grid.to_string().unwrap());
}
