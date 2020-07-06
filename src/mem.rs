use std::slice::SliceIndex;
use crate::cpu::Addr;

pub struct Mem {
    cells: [u8;4096],
}

impl Mem {
    pub fn new() -> Self {
        Mem { cells: [0; 4096] }
    }
   
    pub fn store(&mut self, i: Addr, v: u8) {
        self.cells[i as usize] = v;
    }

    pub fn load(&mut self, i: Addr) -> u8 {
        self.cells[i as usize]
    }

    pub fn get<I>(&self, index: I) -> Option<&<I as SliceIndex<[u8]>>::Output> 
        where I: SliceIndex<[u8]>, {
        self.cells.get(index)
    }

}
