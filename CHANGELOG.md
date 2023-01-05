# Changelog

## 0.2.0

- Renamed `GridWriter::draw` to `GridWriter::set` (as it's more accurate):

  ```diff
  - grid.draw((0, 0), 'a');
  + grid.set((0, 0), 'a');
  ```

- Renamed `DisplayGrid::print` to `DisplayGrid::write_to`:

  ```diff
  - grid.print(&mut stdout).unwrap();
  + grid.write_to(&mut stdout).unwrap();
  ```

- Added the `grux::art` , for ease of creating sprites, with some built-ins:

  ```rs
  use grux::DisplayGrid;
  use grux::art::{BorderRect, FillRect, Sprite};

  let mut grid = [[' '; 8]; 4];
  let rect = BorderRect::new(8, 4, ['╔', '═', '╗', '║', '║', '╚', '═', '╝']);
  rect.draw(&mut grid, (0, 0)).unwrap();

  // ╔══════╗
  // ║      ║
  // ║      ║
  // ╚══════╝
  println!("{}", grid.to_string().unwrap());
  ```

## 0.1.0

- Initial release
