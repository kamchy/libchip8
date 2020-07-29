/// number of collumns in chip-8 display
pub const COLS: usize = 64;

/// number of rows in chip-8 display
pub const ROWS: usize = 32;

pub trait Scr {
    fn xor(&mut self, x: usize, y: usize, v: bool) -> bool;
    fn xor_bytes(&mut self, x: usize, y: usize, bytes: &[u8]) -> bool;
    fn get(&self, x: usize, y: usize) -> bool;
    fn clear(&mut self);
}
/// Screen is an 2d array of bool values
pub struct Screen {
    pixels: [[bool; COLS]; ROWS],
}
pub struct BitScreen {
    pixels: [u64; 32],
}
impl BitScreen {
    pub fn new() -> Self {
        BitScreen { pixels: [0u64; 32] }
    }
}
impl Scr for BitScreen {
    fn xor(&mut self, x: usize, y: usize, v: bool) -> bool {
        let prev = self.get(x, y);
        let val_with_bit = 1u64.rotate_right((x as u32) + 1);
        self.pixels[y] ^= val_with_bit;
        prev & !(prev ^ v)
    }

    fn get(&self, x: usize, y: usize) -> bool {
        self.pixels[y].rotate_left((x as u32).saturating_add(1) % 64) & 1 == 1
    }
    fn clear(&mut self) {
        self.pixels.iter_mut().for_each(|e| *e = 0);
    }

    fn xor_bytes(&mut self, x: usize, y: usize, bytes: &[u8]) -> bool {
        let x = x as u32;
        let mut overflow = false;
        for (bidx, b) in bytes.iter().enumerate() {
            let val_to_xor = (*b as u64).rotate_right(x.saturating_add(8) % 64);
            let old_line = self.pixels[y + bidx];
            let new_line = old_line ^ val_to_xor;
            self.pixels[y + bidx] = new_line;
            overflow = overflow || (old_line & new_line > 0);
        }
        overflow
    }
}
fn bools_from_byte(v: u8) -> [bool; 8] {
    let mut b = [false; 8];
    for x in 0..8_usize {
        b[7 - x] = (1u8 << x) & v > 0;
    }
    b
}

fn byte_from_bools(v: &[bool]) -> u8 {
    let mut r = 0u8;
    for i in 0..8_usize {
        if v[i] {
            r += 1 << (7 - i);
        }
    }
    r
}

impl Screen {
    pub fn new() -> Self {
        Screen {
            pixels: [[false; COLS]; ROWS],
        }
    }
}
impl Scr for Screen {
    /// Xors value v with value at [x, y] coors.
    /// Returns true if [x,y] changed value from true to false
    fn xor(&mut self, x: usize, y: usize, v: bool) -> bool {
        let x = x % COLS;
        let y = y % ROWS;
        let was_pixel = self.pixels[y][x];
        self.pixels[y][x] = was_pixel ^ v;
        was_pixel && !self.pixels[y][x]
    }

    fn xor_bytes(&mut self, x: usize, y: usize, bytes: &[u8]) -> bool {
        let mut overflow = false;
        for (bidx, b) in bytes.iter().enumerate() {
            if let Some(pix_bool_arr) = self.pixels[y + bidx].get_mut(x..x + 8) {
                let pix_byte = byte_from_bools(pix_bool_arr);
                let xored = pix_byte ^ *b;

                let muts = pix_bool_arr;
                muts.copy_from_slice(&bools_from_byte(xored)[..]);
                overflow = overflow || (pix_byte & byte_from_bools(muts) > 0);
            }
        }
        overflow
    }

    fn get(&self, x: usize, y: usize) -> bool {
        let x = x % COLS;
        let y = y % ROWS;
        self.pixels[y][x]
    }

    fn clear(&mut self) {
        for c in 0..COLS {
            for r in 0..ROWS {
                self.pixels[r][c] = false;
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
