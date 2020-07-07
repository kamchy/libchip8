use crate::cpu;
use crate::mem;
use crate::display;
use crate::cpu::Opcode;
use crate::cpu::Instr;

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


    pub fn start_addr(&self) -> Addr {
        200
    }


    fn store_instr(mem: &mut mem::Mem, addr: Addr, v: &[Instr]) {
        let mut a = addr;
        for instr in v.into_iter() {
            mem.store(a, (instr >> 8) as u8);
            mem.store(a + 1, (instr & 0x00ff) as u8);
            a += 2;
        }
    }
    pub fn store(&mut self, v: &[Opcode]) {
        let mut instrs: Vec<Instr> = vec![];
        for op in v {
            instrs.push(Opcode::to_instr(op));
        }
        let start = self.start_addr();
        Emulator::store_instr(&mut self.mem, start, &instrs[..]);
    }

    pub fn run(&mut self) {
        self.cpu.pc(self.start_addr());
        loop {
            if let Some(op) = self.cpu.fetch(&self.mem){
                match op {
                    Opcode::JP(addr) => self.cpu.pc = addr,
                }

            } else {
                break;
            }
        }
    }
}
