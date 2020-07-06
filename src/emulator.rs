use crate::cpu;
use crate::mem;
use crate::display;

use cpu::Addr;

pub struct Emulator {
    pub cpu: cpu::CPU,
    pub mem: mem::Mem,
    pub scr: display::Screen,
}


impl Emulator {
    pub fn new() -> Self {
        Emulator { 
            cpu: cpu::CPU::new(), 
            mem: mem::Mem::new(),
            scr: display::Screen::new()
        }
    }

    pub fn load(&mut self, addr: Addr, v: &[u8]) {
        for (idx, e) in v.into_iter().enumerate() {
            self.mem.store(addr + idx as u16, *e)
        }
    }
}
