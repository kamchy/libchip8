use crate::cpu;
use crate::cpu::Instr;
use crate::cpu::Opcode;
use crate::display;
use crate::input;
use crate::mem;

use cpu::Addr;
///
/// Emulator capable of running chip-8 binaries
pub struct Emulator {
    pub cpu: cpu::CPU,
    pub mem: mem::Mem,
    pub scr: display::Screen,
    pub kbd: input::Keyboard,
}

impl Emulator {
    /// Creates emulator with empty memory.
    pub fn new() -> Self {
        Emulator {
            cpu: cpu::CPU::new(),
            mem: mem::Mem::new(),
            scr: display::Screen::new(),
            kbd: input::Keyboard::new(),
        }
    }

    pub fn start_addr(&self) -> Addr {
        0x200
    }

    pub fn store_font(&mut self) {
        self.mem.store_font(0);
        self.cpu.i = 0;
    }
    pub fn store_instr(&mut self, v: &[Instr]) {
        let mut a = self.start_addr();
        for instr in v.iter() {
            self.mem.store(a, (instr >> 8) as u8);
            self.mem.store(a + 1, (instr & 0x00ff) as u8);
            a += 2;
        }
        self.cpu.pc(self.start_addr());
    }

    fn load_instr(&self, i: Addr) -> Instr {
        let bh: u16 = self.mem.load(i).into();
        let bl: u16 = self.mem.load(i + 1).into();
        (bh << 8) | bl
    }

    /// stores slice of bytes at start_addr
    pub fn store_bytes(&mut self, v: &[u8]) {
        self.mem.store_arr(self.start_addr(), v);
        self.cpu.pc(self.start_addr());
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
        let instr = self.load_instr(self.cpu.pc);
        let op = Opcode::from(instr);
        self.cpu.instr = op;
        op
    }

    pub fn step(&mut self) {
        if let Some(op) = self.fetch() {
            self.exec(op);
        }
    }

    pub fn key_pressed(&mut self, oldk: Option<usize>, k: usize) {
        if let Some(oldidx) = oldk {
            if oldidx != k {
                self.kbd.switch(oldidx);
                self.kbd.switch(k);
            }
        } else {
            self.kbd.switch(k);
        }
    }

    fn exec(&mut self, op: Opcode) {
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
            Opcode::LDI(a) => {
                self.cpu.ldi(a);
                self.cpu.inc_pc();
            }
            Opcode::JPOFF(a) => self.cpu.jpoff(a),
            Opcode::RND(vx, byte) => {
                self.cpu.rnd(vx, byte);
                self.cpu.inc_pc();
            }
            Opcode::DRW(vx, vy, n) => {
                self.draw(vx, vy, n);
                self.cpu.inc_pc();
            }
            Opcode::SKP(vx) => self.cpu.skip_if(self.kbd.get(vx as usize)),
            Opcode::SKNP(vx) => self.cpu.skip_if(!self.kbd.get(vx as usize)),
            Opcode::KEYSET(vx) => {
                self.keyset(vx);
                self.cpu.inc_pc();
            }
            Opcode::DTSET(vx) => {
                self.cpu.dtset(vx);
                self.cpu.inc_pc();
            }
            Opcode::DTGET(vx) => {
                self.cpu.dtget(vx);
                self.cpu.inc_pc();
            }
            Opcode::STSET(vx) => {
                self.cpu.stset(vx);
                self.cpu.inc_pc();
            }
            Opcode::IINC(vx) => {
                self.cpu.iinc(vx);
                self.cpu.inc_pc();
            }
            Opcode::IDIG(vx) => {
                self.idig(vx);
                self.cpu.inc_pc();
            }
            Opcode::BCD(vx) => {
                self.bcd(vx);
                self.cpu.inc_pc();
            }
            Opcode::REGSSTORE(vx) => {
                self.regsstore(vx);
                self.cpu.inc_pc();
            }
            Opcode::REGLOAD(vx) => {
                self.regsload(vx);
                self.cpu.inc_pc();
            }
        }
    }

    fn regsstore(&mut self, vx: u8) {
        self.mem
            .store_arr(self.cpu.i, &self.cpu.regs[0..=vx as usize]);
    }

    fn regsload(&mut self, vx: u8) {
        for i_offset in 0..=vx {
            if let Some(val) = self.mem.get((self.cpu.i + i_offset as u16) as usize) {
                self.cpu.regs[i_offset as usize] = *val;
            }
        }
    }

    fn split_val(v: u8) -> [u8; 3] {
        [v / 100, v / 10, v % 10]
    }

    fn bcd(&mut self, vx: u8) {
        let val = self.cpu.regs[vx as usize];
        match Emulator::split_val(val) {
            [h, t, d] => {
                self.mem.store(self.cpu.i, h);
                self.mem.store(self.cpu.i + 1, t);
                self.mem.store(self.cpu.i + 2, d);
            }
        }
    }

    fn idig(&mut self, vx: u8) {
        self.cpu.i = self.mem.addr_of_font(self.cpu.regs[vx as usize]);
    }

    fn keyset(&mut self, vx: u8) {
        if let Some(idx) = self.kbd.down_key() {
            self.cpu.regs[vx as usize] = idx as u8;
        }
    }

    fn draw(&mut self, vx: u8, vy: u8, n: u8) {
        let mut collision = false;
        let vx = self.cpu.regs[vx as usize];
        let vy = self.cpu.regs[vy as usize];
        for line_num in 0..n {
            let memloc = self.cpu.i + line_num as u16;
            let byte = self.mem.load(memloc);
            for bit in 0..8 {
                if byte & (1 << (7 - bit)) != 0 {
                    collision = collision | self.scr.switch(vx + bit, vy + line_num);
                }
            }
        }

        self.cpu.regs[0xF] = if collision { 1 } else { 0 };
    }

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

    #[test]
    fn ldi_test() {
        let mut e = Emulator::new();
        e.store_instr(&[0xA124]);
        assert_eq!(0xA124, e.load_instr(0x200));
        e.run();
        assert_eq!(e.cpu.i, 0x124);
    }
    #[test]
    fn jpoff_test() {
        let mut e = Emulator::new();
        e.store_instr(&[0x6001, 0xB124]);
        assert_eq!(0x6001, e.load_instr(0x200));
        assert_eq!(0xB124, e.load_instr(0x202));
        e.run();
        assert_eq!(e.cpu.pc, 0x125);
    }

    #[test]
    fn draw_test() {
        let mut e = Emulator::new();
        e.store_font();
        e.store_instr(&[0x6201, 0x6302, 0xD232]);
        e.run();
        assert_eq!(0, e.cpu.i);
        assert_eq!(true, e.scr.get(1, 2), "checking scr(1,2) is true");
        assert_eq!(e.cpu.pc, 0x200 + 6);
    }
}
