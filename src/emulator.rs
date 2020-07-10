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

    pub fn store_instr(&mut self, v: &[Instr]) {
        let mut a = self.start_addr();
        for instr in v.iter() {
            print!("store_inst: storing 0x{:04X} at 0x{:04X}\n", instr, a);
            self.mem.store(a, (instr >> 8) as u8);
            self.mem.store(a + 1, (instr & 0x00ff) as u8);
            a += 2;
        }
    }

    fn load_instr(&self, i: Addr) -> Instr {
        let bh: u16 = self.mem.load(i).into();
        let bl: u16 = self.mem.load(i + 1).into();
        (bh << 8) | bl
    }

    /// stores slice of bytes at start_addr
    pub fn store_bytes(&mut self, v: &[u8]) {
        self.mem.store_arr(self.start_addr(), v);
    }

    /// Stores slice of opcodes at start address
    pub fn store(&mut self, v: &[Opcode]) {
        let mut instrs: Vec<Instr> = vec![];
        for op in v {
            instrs.push(Opcode::to_instr(op));
        }
        self.store_instr(&instrs[..]);
    }

    /// Fetches next instruction (Opcode enum) from location
    /// pointed to by cpu pc register
    fn fetch(&mut self) -> Option<Opcode> {
        println!("fecting... from addr 0x{:02X}", self.cpu.pc);

        let instr = self.load_instr(self.cpu.pc);
        println!(
            "feched... {:?} -> opcode is {:?}",
            instr,
            Opcode::from(instr)
        );

        Opcode::from(instr)
    }

    fn exec(&mut self, op: Opcode) {
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
            Opcode::ADD(vx, byte) => {
                self.cpu.add(vx, byte);
                self.cpu.inc_pc();
            }
            Opcode::LDR(vx, vy) => {
                self.cpu.load_r(vx, vy);
                self.cpu.inc_pc();
            }
            Opcode::AND(vx, vy) => {
                self.cpu.and(vx, vy);
                self.cpu.inc_pc();
            }
            Opcode::OR(vx, vy) => {
                self.cpu.or(vx, vy);
                self.cpu.inc_pc();
            }
            Opcode::XOR(vx, vy) => {
                self.cpu.xor(vx, vy);
                self.cpu.inc_pc();
            }
            Opcode::ADDR(vx, vy) => {
                self.cpu.addr(vx, vy);
                self.cpu.inc_pc();
            }
            Opcode::SUBR(vx, vy) => {
                self.cpu.subr(vx, vy);
                self.cpu.inc_pc();
            }
            Opcode::SHR(vx) => {
                self.cpu.shr(vx);
                self.cpu.inc_pc();
            }

            Opcode::SUBRN(vx, vy) => {
                self.cpu.subrn(vx, vy);
                self.cpu.inc_pc();
            }

            Opcode::SHL(vx) => {
                self.cpu.shl(vx);
                self.cpu.inc_pc();
            }
            Opcode::SNER(vx, vy) => self.cpu.skip_neq_reg(vx, vy),
        }
    }
    /// Runs instructions from start memory location in a loop
    pub fn run(&mut self) {
        self.cpu.pc(self.start_addr());
        loop {
            if let Some(op) = self.fetch() {
                self.exec(op);
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

#[cfg(test)]
mod loadingtest {
    use super::Emulator;
    #[test]
    fn simple_test() {
        let mut e = Emulator::new();
        e.store_bytes(&vec![0x61, 0x05, 0x62, 0x09, 0x81, 0x24]);
        assert_eq!(0x6105, e.load_instr(0x200));
        e.run();
        assert_eq!(e.cpu.regs[1], 14);
    }
}
