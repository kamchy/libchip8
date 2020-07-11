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

    fn wrpx(x: u8) -> usize {
        x as usize % COLS
    }

    fn wrpy(y: u8) -> usize {
        y as usize % ROWS
    }

    /// Switches state of a value at x, y coords,
    /// i.e. xor-draws a pixel at [x, y] location
    /// Returns true if collision was detected
    pub fn switch(&mut self, x: u8, y: u8) -> bool {
        let xi = Self::wrpx(x);
        let yi = Self::wrpy(y);
        let was_pixel = self.pixels[xi][yi];
        self.pixels[xi][yi] = !was_pixel;
        was_pixel && !self.pixels[xi][yi]
    }

    pub fn get(&self, x: u8, y: u8) -> bool {
        let xi = Self::wrpx(x);
        let yi = Self::wrpy(y);
        self.pixels[xi][yi]
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
        let of = a.switch(10, 12);
        assert_eq!(true, a.get(10, 12));
        assert_eq!(false, of);
    }

    #[test]
    fn setget_xor_test() {
        let mut a = Screen::new();
        let of = a.switch(10, 12);
        let of = of | a.switch(10, 12);
        assert_eq!(false, a.get(10, 12));
        assert_eq!(true, of);
    }
}
