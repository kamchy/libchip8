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

    fn load_instr(mem: &mem::Mem, i: Addr) -> Instr {
        let bh: Instr = mem.load(i).into();
        let bl: Instr = mem.load(i+1).into();
        (bh << 8) | bl
    }

    /// Stores slice of opcodes at start address
    pub fn store(&mut self, v: &[Opcode]) {
        let mut instrs: Vec<Instr> = vec![];
        for op in v {
            instrs.push(Opcode::to_instr(op));
        }
        let start = self.start_addr();
        Emulator::store_instr(&mut self.mem, start, &instrs[..]);
    }



    /// Fetches next instruction (Opcode enum) from location
    /// pointed to by cpu pc register
    pub fn fetch(&mut self) -> Option<Opcode> {
        let instr = Emulator::load_instr(&self.mem, self.cpu.pc);
        let opt_optcode = Opcode::from(instr);
        opt_optcode
    }


    /// Runs instructions from start memory location in a loop
    pub fn run(&mut self) {
        self.cpu.pc(self.start_addr());
        loop {
            if let Some(op) = self.fetch() {
                match op {
                    Opcode::JP(addr) => self.cpu.pc = addr,
                }

            } else {
                break;
            }
        }
    }
}
