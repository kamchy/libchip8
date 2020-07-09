use crate::cpu;
use crate::cpu::Instr;
use crate::cpu::Opcode;
use crate::display;
use crate::mem;

use cpu::Addr;
/// Emulator capable of running chip-8 binaries
pub struct Emulator {
    pub cpu: cpu::CPU,
    pub mem: mem::Mem,
    pub scr: display::Screen,
}

impl Emulator {
    /// Creates emulator with empty memory.
    pub fn new() -> Self {
        Emulator {
            cpu: cpu::CPU::new(),
            mem: mem::Mem::new(),
            scr: display::Screen::new(),
        }
    }

    pub fn start_addr(&self) -> Addr {
        0x200
    }

    fn store_instr(mem: &mut mem::Mem, addr: Addr, v: &[Instr]) {
        let mut a = addr;
        for instr in v.iter() {
            mem.store(a, (instr >> 8) as u8);
            mem.store(a + 1, (instr & 0x00ff) as u8);
            a += 2;
        }
    }

    fn load_instr(mem: &mem::Mem, i: Addr) -> Instr {
        let bh: Instr = mem.load(i).into();
        let bl: Instr = mem.load(i + 1).into();
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
        Opcode::from(instr)
    }

    /// Runs instructions from start memory location in a loop
    pub fn run(&mut self) {
        self.cpu.pc(self.start_addr());
        loop {
            if let Some(op) = self.fetch() {
                print!("Running opcode {:?}", op);
                match op {
                    Opcode::CLS => {
                        self.scr.clear();
                        self.cpu.inc_pc();
                    }
                    Opcode::RET => {
                        self.cpu.ret();
                        self.cpu.inc_pc();
                    }
                    Opcode::JP(addr) => self.cpu.pc = addr,
                    Opcode::CALL(addr) => self.cpu.call(addr),
                    Opcode::SE(vx, byte) => self.cpu.skip_eq(vx.into(), byte),
                    Opcode::SNE(vx, byte) => self.cpu.skip_neq(vx, byte),
                    Opcode::SER(vx, vy) => self.cpu.skip_eq_reg(vx, vy),
                    Opcode::LD(vx, byte) => {
                        self.cpu.load(vx, byte);
                        self.cpu.inc_pc();
                    }
                    Opcode::ADD(vx, byte) => self.cpu.add(vx, byte),
                    Opcode::LDR(vx, vy) => self.cpu.load_r(vx, vy),
                }
            } else {
                break;
            }
        }
    }
}

impl Default for Emulator {
    fn default() -> Self {
        Self::new()
    }
}
