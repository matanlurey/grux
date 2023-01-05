use super::art::*;
use super::*;

#[test]
fn display_grid_default_impl_to_string() {
    struct MyGrid;

    impl DisplayGrid for MyGrid {
        fn write_to(&self, stream: &mut impl std::io::Write) -> std::io::Result<()> {
            write!(stream, "Hello ")?;
            write!(stream, "World!")?;
            Ok(())
        }
    }

    let string = MyGrid.to_string().unwrap();
    assert_eq!(string, "Hello World!");
}

#[test]
fn grid_writer_fixed_array() {
    let mut grid = [[0, 1, 2], [3, 4, 5], [6, 7, 8]];

    grid.set((1, 1), 9);

    assert_eq!(grid, [[0, 1, 2], [3, 9, 5], [6, 7, 8]]);
}

#[test]
fn display_grid_fixed_array() {
    let grid = [[0, 1, 2], [3, 4, 5], [6, 7, 8]];

    let string = grid.to_string().unwrap();
    assert_eq!(string, "012\n345\n678\n");
}

#[test]
fn grid_writer_fixed_vec() {
    let mut grid = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];

    grid.set((1, 1), 9);

    assert_eq!(grid, vec![vec![0, 1, 2], vec![3, 9, 5], vec![6, 7, 8]]);
}

#[test]
fn grid_writer_grown_vec() {
    let mut grid = vec![];

    grid.set((3, 3), 9);

    assert_eq!(grid, vec![vec![], vec![], vec![], vec![0, 0, 0, 9]]);
}

#[test]
fn display_grid_fixed_vec() {
    let grid = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];

    let string = grid.to_string().unwrap();
    assert_eq!(string, "012\n345\n678\n");
}

#[test]
fn display_grid_grown_vec() {
    let grid = vec![vec![], vec![], vec![], vec![0, 0, 0, 9]];

    let string = grid.to_string().unwrap();
    assert_eq!(string, "\n\n\n0009\n");
}

#[test]
fn grid_writer_fixed_string() {
    let mut grid = String::from("012\n345\n678\n");

    grid.set((1, 1), '9');

    assert_eq!(grid, "012\n395\n678");
}

#[test]
fn grid_writer_grown_string() {
    let mut grid = String::new();

    grid.set((3, 3), '9');

    assert_eq!(grid, "\n\n\n   9");
}

#[test]
fn display_grid_string() {
    let grid = String::from("012\n345\n678\n");

    let string = DisplayGrid::to_string(&grid).unwrap();
    assert_eq!(string, "012\n345\n678\n");
}

#[test]
fn display_grid_string_print() {
    let grid = String::from("012\n345\n678\n");
    let mut buffer = Vec::new();

    grid.write_to(&mut buffer).unwrap();
    assert_eq!(buffer, b"012\n345\n678\n");
}

#[test]
fn sprite_line_horizontal() {
    let mut grid = [[' '; 3]; 3];

    let line = Line::horizontal(3, '═');
    line.draw_to((0, 0), &mut grid);

    #[rustfmt::skip]
    assert_eq!(grid, [
        ['═', '═', '═'],
        [' ', ' ', ' '],
        [' ', ' ', ' '],
    ]);

    assert_eq!(line.width(), 3);
    assert_eq!(line.height(), 1);
}

#[test]
fn sprite_line_vertical() {
    let mut grid = [[' '; 3]; 3];

    let line = Line::vertical(3, '║');
    line.draw_to((0, 0), &mut grid);

    #[rustfmt::skip]
    assert_eq!(grid, [
        ['║', ' ', ' '],
        ['║', ' ', ' '],
        ['║', ' ', ' '],
    ]);

    assert_eq!(line.width(), 1);
    assert_eq!(line.height(), 3);
}

#[test]
fn sprite_fill_rect() {
    let mut grid = [[' '; 3]; 3];

    let rect = FillRect::new(3, 3, '█');
    rect.draw_to((0, 0), &mut grid);

    #[rustfmt::skip]
    assert_eq!(grid, [
        ['█', '█', '█'],
        ['█', '█', '█'],
        ['█', '█', '█'],
    ]);

    assert_eq!(rect.width(), 3);
    assert_eq!(rect.height(), 3);
}

#[test]
fn sprite_border_rect() {
    let mut grid = [[' '; 3]; 3];

    let rect = BorderRect::new(3, 3, ['╔', '═', '╗', '║', '║', '╚', '═', '╝']);
    rect.draw_to((0, 0), &mut grid);

    #[rustfmt::skip]
    assert_eq!(grid, [
        ['╔', '═', '╗'],
        ['║', ' ', '║'],
        ['╚', '═', '╝'],
    ]);

    assert_eq!(rect.width(), 3);
    assert_eq!(rect.height(), 3);
}

#[test]
#[should_panic]
fn sprite_border_width_too_small() {
    let _ = BorderRect::new(1, 3, ['╔', '═', '╗', '║', '║', '╚', '═', '╝']);
}

#[test]
#[should_panic]
fn sprite_border_height_too_small() {
    let _ = BorderRect::new(3, 1, ['╔', '═', '╗', '║', '║', '╚', '═', '╝']);
}
