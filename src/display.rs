/// number of collumns in chip-8 display
pub const COLS: usize = 64;

/// number of rows in chip-8 display
pub const ROWS: usize = 32;

/// Screen is an 2d array of bool values
pub struct Screen {
    pub pixels: [[bool; ROWS]; COLS],
}

impl Screen {
    pub fn new() -> Self {
        Screen {
            pixels: [[false; ROWS]; COLS],
        }
    }

    /// Switches state of a value at x, y coords,
    /// i.e. xor-draws a pixel at [x, y] location
    /// Returns true if collision was detected
    pub fn xor(&mut self, x: usize, y: usize, v: bool) -> bool {
        let x = x % COLS;
        let y = y % ROWS;
        let was_pixel = self.pixels[x][y];
        self.pixels[x][y] = was_pixel ^ v;
        was_pixel && !self.pixels[x][y]
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        let x = x % COLS;
        let y = y % ROWS;
        self.pixels[x][y]
    }

    pub fn clear(&mut self) {
        for c in 0..COLS {
            for r in 0..ROWS {
                self.pixels[c][r] = false;
            }
        }
    }
}

impl Default for Screen {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setget_no_xor_test() {
        let mut a = Screen::new();
        let of = a.xor(10, 12, true);
        assert_eq!(true, a.get(10, 12));
        assert_eq!(false, of);
    }

    #[test]
    fn setget_xor_test() {
        let mut a = Screen::new();
        let of = a.xor(10, 12, true);
        let of = of | a.xor(10, 12, true);
        assert_eq!(false, a.get(10, 12));
        assert_eq!(true, of);
    }
    #[test]
    fn display_test() {
        let mut d = Screen::new();
        d.xor(2, 2, true);
        d.xor(4, 4, true);
        d.xor(4, 4, true);
        d.xor(100, 100, true);

        assert_eq!(d.get(2, 2), true);
        assert_eq!(d.get(36, 4), true);
        assert_eq!(d.get(4, 4), false);
    }
}
