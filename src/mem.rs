use crate::cpu::Addr;
use std::slice::SliceIndex;

const FONT: [[u8; 5]; 16] = [
    [0xF0, 0x90, 0x90, 0x90, 0xF0],
    [0x20, 0x60, 0x20, 0x20, 0x70],
    [0xF0, 0x10, 0xF0, 0x80, 0xF0],
    [0xF0, 0x10, 0xF0, 0x10, 0xF0],
    [0x90, 0x90, 0xF0, 0x10, 0x10],
    [0xF0, 0x80, 0xF0, 0x10, 0xF0],
    [0xF0, 0x80, 0xF0, 0x90, 0xF0],
    [0xF0, 0x10, 0x20, 0x40, 0x40],
    [0xF0, 0x90, 0xF0, 0x90, 0xF0],
    [0xF0, 0x90, 0xF0, 0x10, 0xF0],
    [0xF0, 0x90, 0xF0, 0x90, 0x90],
    [0xE0, 0x90, 0xE0, 0x90, 0xE0],
    [0xF0, 0x80, 0x80, 0x80, 0xF0],
    [0xE0, 0x90, 0x90, 0x90, 0xE0],
    [0xF0, 0x80, 0xF0, 0x80, 0xF0],
    [0xF0, 0x80, 0xF0, 0x80, 0x80],
];

pub struct Mem {
    cells: [u8; 4096],
    start_addr: Addr,
}

impl Mem {
    const FONT_SIZE_BYTES: u16 = 5;

    pub fn new() -> Self {
        Mem {
            cells: [0; 4096],
            start_addr: 0x0000,
        }
    }

    pub fn store(&mut self, i: Addr, v: u8) {
        self.cells[i as usize] = v;
    }

    pub fn load(&self, i: Addr) -> u8 {
        self.cells[i as usize]
    }

    pub fn get<I>(&self, index: I) -> Option<&<I as SliceIndex<[u8]>>::Output>
    where
        I: SliceIndex<[u8]>,
    {
        self.cells.get(index)
    }

    pub fn store_arr(&mut self, addr: Addr, v: &[u8]) {
        for (idx, e) in v.iter().enumerate() {
            self.store(addr + idx as u16, *e)
        }
    }

    pub fn store_font(&mut self, start: Addr) {
        self.start_addr = start;
        for i in 0..16 {
            let a: Addr = start + i * Mem::FONT_SIZE_BYTES;
            self.store_arr(a, &FONT[i as usize]);
        }
    }

    pub fn addr_of_font(&self, digit: u8) -> u16 {
        self.start_addr + Mem::FONT_SIZE_BYTES * digit as u16
    }
}

impl Default for Mem {
    fn default() -> Self {
        Self::new()
    }
}
