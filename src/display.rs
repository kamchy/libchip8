/// number of collumns in chip-8 display
const COLS: usize = 64;

/// number of rows in chip-8 display
const ROWS: usize = 32;

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
        (x as usize % COLS) as usize
    }

    fn wrpy(y: u8) -> usize {
        (y as usize % ROWS) as usize
    }

    /// Switches state of a value at x, y coords,
    /// i.e. xor-draws a pixel at [x, y] location
    pub fn switch(&mut self, x: u8, y: u8) -> &Self {
        let xi = Self::wrpx(x);
        let yi = Self::wrpy(y);
        self.pixels[xi][yi] = !self.pixels[xi][yi];
        self
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
