/// Address in chip-8 memory  (4096 B, byte-addressable)
/// - used by pc, i, sp and stack etc.
pub type Addr = u16;
///
pub type Instr = u16;
/// Type of value stored in chip-8 register (u8)
pub type Reg = u8;
/// Number of cpu registers
const REGS_COUNT: usize = 0x10;

#[derive(Default, PartialEq, Debug)]
pub struct CPU {
    ///
    pub pc: Addr,
    /// I register stroring address for sprites
    pub i: Addr,
    /// 16 registers
    pub regs: [Reg; REGS_COUNT],
    /// stack pointer
    pub sp: Addr,
    /// stack of return addresses for subroutines
    stack: Vec<Addr>,
    /// fetched instruction to be executed
    pub instr: Option<Opcode>,
    /// delay timer regiter
    pub dt: Reg,
    /// sound timer register
    pub st: Reg,
}

impl CPU {
    pub fn from(
        pc: Addr,
        i: Addr,
        regs: [Reg; REGS_COUNT],
        sp: Addr,
        instr: Option<Opcode>,
        dt: Reg,
        st: Reg,
    ) -> Self {
        CPU {
            pc,
            i,
            regs,
            sp,
            stack: vec![],
            instr,
            dt,
            st,
        }
    }

    pub fn new() -> Self {
        Default::default()
    }

    pub fn pc(&mut self, pc: Addr) -> &Self {
        self.pc = pc;
        self
    }

    pub fn inc_pc(&mut self) {
        self.pc += 2;
    }

    pub fn ret(&mut self) {
        if let Some(addr) = self.stack.pop() {
            self.sp -= 1;
            self.pc = addr;
            assert_eq!(self.sp as usize, self.stack.len());
        }
    }

    pub fn call(&mut self, a: Addr) {
        self.stack.push(self.pc);
        self.sp += 1;
        self.pc = a;
    }

    pub fn skip_if(&mut self, pred: bool) {
        self.pc += if pred { 4 } else { 2 };
    }

    pub fn skip_eq(&mut self, vx: usize, byte: Reg) {
        self.skip_if(self.regs[vx] == byte);
    }

    pub fn skip_neq(&mut self, vx: usize, byte: Reg) {
        self.skip_if(self.regs[vx] != byte);
    }

    pub fn skip_eq_reg(&mut self, vx: usize, vy: usize) {
        self.skip_if(self.regs[vx] == self.regs[vy]);
    }

    pub fn skip_neq_reg(&mut self, vx: usize, vy: usize) {
        self.skip_if(self.regs[vx] != self.regs[vy]);
    }

    pub fn load(&mut self, vx: usize, byte: u8) {
        self.regs[vx] = byte;
    }

    pub fn load_r(&mut self, vx: usize, vy: usize) {
        self.regs[vx] = self.regs[vy];
    }

    pub fn add(&mut self, vx: usize, byte: u8) {
        let sum = self.regs[vx].wrapping_add(byte);
        self.regs[vx] = sum;
    }

    pub fn or(&mut self, vx: usize, vy: usize) {
        self.regs[vx] |= self.regs[vy];
    }

    pub fn and(&mut self, vx: usize, vy: usize) {
        self.regs[vx] &= self.regs[vy];
    }

    pub fn xor(&mut self, vx: usize, vy: usize) {
        self.regs[vx] ^= self.regs[vy];
    }

    pub fn addr(&mut self, vx: usize, vy: usize) {
        let (sum, overflow) = self.regs[vx].overflowing_add(self.regs[vy]);
        self.regs[0xF] = if overflow { 1 } else { 0 };
        self.regs[vx] = sum;
    }

    pub fn subr(&mut self, vx: usize, vy: usize) {
        let (diff, overflow) = self.regs[vx].overflowing_sub(self.regs[vy]);
        self.regs[0xF] = if !overflow { 1 } else { 0 };
        self.regs[vx] = diff;
    }

    pub fn shr(&mut self, vx: usize) {
        let (res, overflow) = self.regs[vx].overflowing_shr(1);
        self.regs[0xF] = if overflow { 1 } else { 0 };
        self.regs[vx] = res;
    }

    pub fn subrn(&mut self, vx: usize, vy: usize) {
        let (diff, overflow) = self.regs[vy].overflowing_sub(self.regs[vx]);
        self.regs[0xF] = if !overflow { 1 } else { 0 };
        self.regs[vx] = diff;
    }

    pub fn shl(&mut self, vx: usize) {
        let (res, overflow) = self.regs[vx].overflowing_shl(1);
        self.regs[0xF] = if overflow { 1 } else { 0 };
        self.regs[vx] = res;
    }
    pub fn ldi(&mut self, addr: Addr) {
        self.i = addr;
    }

    pub fn jpoff(&mut self, addr: Addr) {
        self.pc = self.regs[0] as u16 + addr;
    }

    pub fn rnd(&mut self, vx: usize, byte: u8) {
        self.regs[vx] = rand::random::<u8>() & byte;
    }

    pub fn dtset(&mut self, vx: usize) {
        self.dt = self.regs[vx];
    }

    pub fn dtget(&mut self, vx: usize) {
        self.regs[vx] = self.dt;
    }

    pub fn stset(&mut self, vx: usize) {
        self.st = self.regs[vx];
    }

    pub fn iinc(&mut self, vx: usize) {
        self.i += self.regs[vx] as u16;
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Opcode {
    /// clear screen
    CLS,
    /// return from subroutine
    RET,
    // jump tp address
    JP(Addr),
    /// call subroutine from address
    CALL(Addr),
    /// skip next instr if register with given index
    /// equals given value
    SE(usize, Reg),
    /// skips if not equals
    SNE(usize, Reg),
    /// skip next instr if contents of registers
    /// with given indices are equal
    SER(usize, usize),
    /// sets vx=kk
    LD(usize, Reg),
    /// sets Vx = Vx + kk
    ADD(usize, Reg),
    /// sets Vx = Vy
    LDR(usize, usize),

    OR(usize, usize),
    AND(usize, usize),
    XOR(usize, usize),
    ADDR(usize, usize),
    SUBR(usize, usize),
    /// shift right contents of given register
    SHR(usize, usize),
    SUBRN(usize, usize),
    SHL(usize, usize),
    SNER(usize, usize),
    LDI(u16),
    JPOFF(u16),
    RND(usize, u8),
    DRW(usize, usize, u8),
    SKP(usize),
    SKNP(usize),
    // F
    KEYSET(usize),
    DTSET(usize),
    DTGET(usize),
    STSET(usize),
    IINC(usize),
    IDIG(usize),
    BCD(usize),
    REGSSTORE(usize),
    REGLOAD(usize),
}

impl Opcode {
    fn xyn(op: u16) -> (usize, usize, u8) {
        (
            (op >> 8 & 0xF) as usize,
            (op >> 4 & 0xF) as usize,
            (op & 0xF) as u8,
        )
    }

    /// extracts three nibbles at the end (masks with 0x0FFF)
    fn nnn(op: u16) -> u16 {
        op & 0x0FFF
    }

    fn xkk(op: u16) -> (usize, u8) {
        ((op >> 8 & 0xF) as usize, (op & 0x00FF) as u8)
    }

    fn xy(op: u16) -> (usize, usize) {
        ((op >> 8 & 0xF) as usize, (op >> 4 & 0xF) as usize)
    }

    fn xs(op: u16) -> usize {
        (op >> 8 & 0xF) as usize
    }

    pub fn from(op: Instr) -> Option<Opcode> {
        let nnn = Opcode::nnn(op);
        let (x, kk) = Opcode::xkk(op);
        let (xm, ym) = Opcode::xy(op);
        let (xn, yn, nn) = Opcode::xyn(op);
        let xs = Opcode::xs(op);
        match op & 0xF000 {
            0x0000 => match op {
                0x00E0 => Some(Opcode::CLS),
                0x00EE => Some(Opcode::RET),
                _ => None,
            },
            0x1000 => Some(Opcode::JP(nnn)),
            0x2000 => Some(Opcode::CALL(nnn)),
            0x3000 => Some(Opcode::SE(x, kk)),
            0x4000 => Some(Opcode::SNE(x, kk)),
            0x5000 => match op & 0xF {
                0 => Some(Opcode::SER(xm, ym)),
                _ => None,
            },
            0x6000 => Some(Opcode::LD(x, kk)),
            0x7000 => Some(Opcode::ADD(x, kk)),
            0x8000 => match op & 0xF {
                0x0 => Some(Opcode::LDR(xm, ym)),
                0x1 => Some(Opcode::OR(xm, ym)),
                0x2 => Some(Opcode::AND(xm, ym)),
                0x3 => Some(Opcode::XOR(xm, ym)),
                0x4 => Some(Opcode::ADDR(xm, ym)),
                0x5 => Some(Opcode::SUBR(xm, ym)),
                0x6 => Some(Opcode::SHR(xm, ym)),
                0x7 => Some(Opcode::SUBRN(xm, ym)),
                0xE => Some(Opcode::SHL(xm, ym)),
                _ => None,
            },
            0x9000 => Some(Opcode::SNER(xm, ym)),
            0xA000 => Some(Opcode::LDI(nnn)),
            0xB000 => Some(Opcode::JPOFF(nnn)),
            0xC000 => Some(Opcode::RND(x, kk)),
            0xD000 => Some(Opcode::DRW(xn, yn, nn)),
            0xE000 => match op & 0xFF {
                0x9E => Some(Opcode::SKP(xs)),
                0xA1 => Some(Opcode::SKNP(xs)),
                _ => None,
            },
            0xF000 => match op & 0xFF {
                0x07 => Some(Opcode::DTGET(xs)),
                0x0A => Some(Opcode::KEYSET(xs)),
                0x15 => Some(Opcode::DTSET(xs)),
                0x18 => Some(Opcode::STSET(xs)),
                0x1E => Some(Opcode::IINC(xs)),
                0x29 => Some(Opcode::IDIG(xs)),
                0x33 => Some(Opcode::BCD(xs)),
                0x55 => Some(Opcode::REGSSTORE(xs)),
                0x65 => Some(Opcode::REGLOAD(xs)),
                _ => None,
            },

            _ => None,
        }
    }

    fn vx_byte(mask: u16, vx: &usize, byte: &u8) -> Instr {
        mask | (*vx as u16) << 8 | *byte as u16
    }

    fn vx_vy(mask: u16, vx: &usize, vy: &usize) -> Instr {
        mask | (*vx as u16) << 8 | (*vy as u16) << 4
    }

    fn innn(mask: u16, a: &u16) -> Instr {
        mask | a
    }

    fn vx_vy_n(mask: u16, vx: &usize, vy: &usize, n: &u8) -> Instr {
        mask | (*vx as u16) << 8 | (*vy as u16) << 4 | (*n as u16)
    }

    fn ibyte(mask: u16, vx: &usize) -> Instr {
        mask | (*vx as u16) << 8
    }

    pub fn to_instr(&self) -> Instr {
        let res = match self {
            Opcode::CLS => 0x00E0,
            Opcode::RET => 0x00EE,
            Opcode::JP(a) => Opcode::innn(0x1000, a),
            Opcode::CALL(a) => Opcode::innn(0x2000, a),
            Opcode::SE(vx, byte) => Opcode::vx_byte(0x3000, vx, byte),
            Opcode::SNE(vx, byte) => Opcode::vx_byte(0x4000, vx, byte),
            Opcode::SER(vx, vy) => Opcode::vx_vy(0x5000, vx, vy),
            Opcode::LD(vx, byte) => Opcode::vx_byte(0x6000, vx, byte),
            Opcode::ADD(vx, byte) => Opcode::vx_byte(0x7000, vx, byte),
            Opcode::LDR(vx, vy) => Opcode::vx_vy(0x8000, vx, vy),
            Opcode::OR(vx, vy) => Opcode::vx_vy(0x8001, vx, vy),
            Opcode::AND(vx, vy) => Opcode::vx_vy(0x8002, vx, vy),
            Opcode::XOR(vx, vy) => Opcode::vx_vy(0x8003, vx, vy),
            Opcode::ADDR(vx, vy) => Opcode::vx_vy(0x8004, vx, vy),
            Opcode::SUBR(vx, vy) => Opcode::vx_vy(0x8005, vx, vy),
            Opcode::SHR(vx, _) => Opcode::ibyte(0x8006, vx),
            Opcode::SUBRN(vx, vy) => Opcode::vx_vy(0x8007, vx, vy),
            Opcode::SHL(vx, _) => Opcode::ibyte(0x800E, vx),
            Opcode::SNER(vx, vy) => Opcode::vx_vy(0x9000, vx, vy),
            Opcode::LDI(a) => Opcode::innn(0xA000, a),
            Opcode::JPOFF(a) => Opcode::innn(0xB000, a),
            Opcode::RND(vx, byte) => Opcode::vx_byte(0xC000, vx, byte),
            Opcode::DRW(vx, vy, n) => Opcode::vx_vy_n(0xD000, vx, vy, n),
            Opcode::SKP(a) => Opcode::ibyte(0xE09E, a),
            Opcode::SKNP(a) => Opcode::ibyte(0xE0A1, a),
            Opcode::KEYSET(a) => Opcode::ibyte(0xF00A, a),
            Opcode::DTSET(a) => Opcode::ibyte(0xF015, a),
            Opcode::DTGET(a) => Opcode::ibyte(0xF007, a),
            Opcode::STSET(a) => Opcode::ibyte(0xF018, a),
            Opcode::IINC(a) => Opcode::ibyte(0xF01E, a),
            Opcode::IDIG(a) => Opcode::ibyte(0xF029, a),
            Opcode::BCD(a) => Opcode::ibyte(0xF033, a),
            Opcode::REGSSTORE(a) => Opcode::ibyte(0xF055, a),
            Opcode::REGLOAD(a) => Opcode::ibyte(0xF065, a),
        };
        res
    }
}

#[cfg(test)]
mod test {
    use super::Opcode;

    #[test]
    fn cls_test() {
        assert_eq!(Opcode::from(0x00E0), Some(Opcode::CLS));
        assert_eq!(0x00E0, Opcode::CLS.to_instr());
    }

    #[test]
    fn ret_test() {
        assert_eq!(Opcode::from(0x00EE), Some(Opcode::RET));
        assert_eq!(0x00EE, Opcode::RET.to_instr());
    }

    #[test]
    fn jp_test() {
        assert_eq!(Opcode::from(0x1ABC), Some(Opcode::JP(0xABC)));
        assert_eq!(0x1ABC, Opcode::JP(0xABC).to_instr());
    }

    #[test]
    fn call_test() {
        assert_eq!(Opcode::from(0x2DEF), Some(Opcode::CALL(0xDEF)));
        assert_eq!(0x2DEF, Opcode::CALL(0xDEF).to_instr());
    }

    #[test]
    fn se_test() {
        assert_eq!(Opcode::from(0x30AB), Some(Opcode::SE(0, 0xAB)));
        assert_eq!(0x30AB, Opcode::SE(0, 0xAB).to_instr());
    }

    #[test]
    fn sne_test() {
        assert_eq!(Opcode::from(0x40AB), Some(Opcode::SNE(0, 0xAB)));
        assert_eq!(0x40AB, Opcode::SNE(0, 0xAB).to_instr());
    }

    #[test]
    fn ser_test() {
        assert_eq!(Opcode::from(0x5DA0), Some(Opcode::SER(0xD, 0xA)));
        assert_eq!(0x5DA0, Opcode::SER(0xD, 0xA).to_instr());
    }

    #[test]
    fn ld_test() {
        assert_eq!(Opcode::from(0x6DA0), Some(Opcode::LD(0xD, 0xA0)));
        assert_eq!(0x6DA0, Opcode::LD(0xD, 0xA0).to_instr());
    }

    #[test]
    fn add_test() {
        assert_eq!(Opcode::from(0x7DA0), Some(Opcode::ADD(0xD, 0xA0)));
        assert_eq!(0x7DA0, Opcode::ADD(0xD, 0xA0).to_instr());
    }

    #[test]
    fn add_76ff_test() {
        assert_eq!(Opcode::from(0x76FF), Some(Opcode::ADD(0x6, 0xFF)));
        assert_eq!(0x76FF, Opcode::ADD(0x6, 0xFF).to_instr());
    }

    #[test]
    fn ldr_test() {
        assert_eq!(Opcode::from(0x8DA0), Some(Opcode::LDR(0xD, 0xA)));
        assert_eq!(0x8DA0, Opcode::LDR(0xD, 0xA).to_instr());
    }

    #[test]
    fn or_test() {
        assert_eq!(Opcode::from(0x8DA1), Some(Opcode::OR(0xD, 0xA)));
        assert_eq!(0x8DA1, Opcode::OR(0xD, 0xA).to_instr());
    }

    #[test]
    fn and_test() {
        assert_eq!(Opcode::from(0x8DA2), Some(Opcode::AND(0xD, 0xA)));
        assert_eq!(0x8DA2, Opcode::AND(0xD, 0xA).to_instr());
    }

    #[test]
    fn xor_test() {
        assert_eq!(Opcode::from(0x8DA3), Some(Opcode::XOR(0xD, 0xA)));
        assert_eq!(0x8DA3, Opcode::XOR(0xD, 0xA).to_instr());
    }

    #[test]
    fn addr_test() {
        assert_eq!(Opcode::from(0x8DA4), Some(Opcode::ADDR(0xD, 0xA)));
        assert_eq!(0x8DA4, Opcode::ADDR(0xD, 0xA).to_instr());
    }

    #[test]
    fn subr_test() {
        assert_eq!(Opcode::from(0x8DA5), Some(Opcode::SUBR(0xD, 0xA)));
        assert_eq!(0x8DA5, Opcode::SUBR(0xD, 0xA).to_instr());
    }

    #[test]
    fn shr_test() {
        assert_eq!(Opcode::from(0x8DA6), Some(Opcode::SHR(0xD, 0xA)));
        assert_eq!(0x8D06, Opcode::SHR(0xD, 0xA).to_instr());
    }

    #[test]
    fn subrn_test() {
        assert_eq!(Opcode::from(0x8DA7), Some(Opcode::SUBRN(0xD, 0xA)));
        assert_eq!(0x8DA7, Opcode::SUBRN(0xD, 0xA).to_instr());
    }

    #[test]
    fn shl_test() {
        assert_eq!(Opcode::from(0x8DAE), Some(Opcode::SHL(0xD, 0xA)));
        assert_eq!(0x8D0E, Opcode::SHL(0xD, 0xA).to_instr());
    }

    #[test]
    fn sner_test() {
        assert_eq!(Opcode::from(0x9DA0), Some(Opcode::SNER(0xD, 0xA)));
        assert_eq!(0x9DA0, Opcode::SNER(0xD, 0xA).to_instr());
    }

    #[test]
    fn ldi_test() {
        assert_eq!(Opcode::from(0xADA0), Some(Opcode::LDI(0xDA0)));
        assert_eq!(0xADA0, Opcode::LDI(0xDA0).to_instr());
    }

    #[test]
    fn jpoff_test() {
        assert_eq!(Opcode::from(0xBDB0), Some(Opcode::JPOFF(0xDB0)));
        assert_eq!(0xBDB0, Opcode::JPOFF(0xDB0).to_instr());
    }

    #[test]
    fn drw_test() {
        assert_eq!(Opcode::from(0xDDB1), Some(Opcode::DRW(0xD, 0xB, 1)));
        assert_eq!(0xDDB1, Opcode::DRW(0xD, 0xB, 1).to_instr());
    }

    #[test]
    fn skp_test() {
        assert_eq!(Opcode::from(0xE19E), Some(Opcode::SKP(1)));
        assert_eq!(0xE19E, Opcode::SKP(1).to_instr());
    }

    #[test]
    fn sknp_test() {
        assert_eq!(Opcode::from(0xE1A1), Some(Opcode::SKNP(1)));
        assert_eq!(0xE1A1, Opcode::SKNP(1).to_instr());
    }
}
