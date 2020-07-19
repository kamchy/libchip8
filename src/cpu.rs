pub type Addr = u16;
pub type Instr = u16;
pub type Reg = u8;
pub type Regs = [Reg; 16];

#[derive(Default, PartialEq, Debug)]
pub struct CPU {
    ///
    pub pc: Addr,
    /// I register stroring address for sprites
    pub i: Addr,
    /// 16 registers
    pub regs: Regs,
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
        regs: Regs,
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

    pub fn skip_neq_reg(&mut self, vx: u8, vy: u8) {
        self.skip_if(self.regs[vx as usize] != self.regs[vy as usize]);
    }

    pub fn load(&mut self, vx: u8, byte: u8) {
        self.regs[vx as usize] = byte;
    }

    pub fn load_r(&mut self, vx: u8, vy: u8) {
        self.regs[vx as usize] = self.regs[vy as usize];
    }

    pub fn add(&mut self, vx: u8, byte: u8) {
        let sum = self.regs[vx as usize].wrapping_add(byte);
        self.regs[vx as usize] = sum;
    }

    pub fn or(&mut self, vx: u8, vy: u8) {
        self.regs[vx as usize] |= self.regs[vy as usize];
    }

    pub fn and(&mut self, vx: u8, vy: u8) {
        self.regs[vx as usize] &= self.regs[vy as usize];
    }

    pub fn xor(&mut self, vx: u8, vy: u8) {
        self.regs[vx as usize] ^= self.regs[vy as usize];
    }

    pub fn addr(&mut self, vx: u8, vy: u8) {
        let (sum, overflow) = self.regs[vx as usize].overflowing_add(self.regs[vy as usize]);
        self.regs[0xF] = if overflow { 1 } else { 0 };
        self.regs[vx as usize] = sum;
    }

    pub fn subr(&mut self, vx: u8, vy: u8) {
        let (diff, overflow) = self.regs[vx as usize].overflowing_sub(self.regs[vy as usize]);
        self.regs[0xF] = if !overflow { 1 } else { 0 };
        self.regs[vx as usize] = diff;
    }

    pub fn shr(&mut self, vx: u8) {
        let (res, overflow) = self.regs[vx as usize].overflowing_shr(1);
        self.regs[0xF] = if overflow { 1 } else { 0 };
        self.regs[vx as usize] = res;
    }

    pub fn subrn(&mut self, vx: u8, vy: u8) {
        let (diff, overflow) = self.regs[vy as usize].overflowing_sub(self.regs[vx as usize]);
        self.regs[0xF] = if !overflow { 1 } else { 0 };
        self.regs[vx as usize] = diff;
    }

    pub fn shl(&mut self, vx: u8) {
        let (res, overflow) = self.regs[vx as usize].overflowing_shl(1);
        self.regs[0xF] = if overflow { 1 } else { 0 };
        self.regs[vx as usize] = res;
    }
    pub fn ldi(&mut self, addr: Addr) {
        self.i = addr;
    }

    pub fn jpoff(&mut self, addr: Addr) {
        self.pc = self.regs[0] as u16 + addr;
    }

    pub fn rnd(&mut self, vx: u8, byte: u8) {
        self.regs[vx as usize] = rand::random::<u8>() & byte;
    }

    pub fn dtset(&mut self, vx: u8) {
        self.dt = self.regs[vx as usize];
    }

    pub fn dtget(&mut self, vx: u8) {
        self.regs[vx as usize] = self.dt;
    }

    pub fn stset(&mut self, vx: u8) {
        self.st = self.regs[vx as usize];
    }

    pub fn iinc(&mut self, vx: u8) {
        self.i += self.regs[vx as usize] as u16;
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
    SE(u8, Reg),
    /// skips if not equals
    SNE(usize, Reg),
    /// skip next instr if contents of registers
    /// with given indices are equal
    SER(usize, usize),
    /// sets vx=kk
    LD(u8, Reg),
    /// sets Vx = Vx + kk
    ADD(u8, Reg),
    /// sets Vx = Vy
    LDR(u8, u8),

    OR(u8, u8),
    AND(u8, u8),
    XOR(u8, u8),
    ADDR(u8, u8),
    SUBR(u8, u8),
    SHR(u8),
    SUBRN(u8, u8),
    SHL(u8),
    SNER(u8, u8),
    LDI(u16),
    JPOFF(u16),
    RND(u8, u8),
    DRW(u8, u8, u8),
    SKP(u8),
    SKNP(u8),
    // F
    KEYSET(u8),
    DTSET(u8),
    DTGET(u8),
    STSET(u8),
    IINC(u8),
    IDIG(u8),
    BCD(u8),
    REGSSTORE(u8),
    REGLOAD(u8),
}

impl Opcode {
    pub fn from(op: u16) -> Option<Opcode> {
        if op & 0xF000 == 0x1000 {
            Some(Opcode::JP(op & 0x0FFF))
        } else if op & 0xF000 == 0x2000 {
            Some(Opcode::CALL(op & 0x0FFF))
        } else if op == 0x00E0 {
            Some(Opcode::CLS)
        } else if op == 0x00EE {
            Some(Opcode::RET)
        } else if op & 0xF000 == 0x3000 {
            Some(Opcode::SE(((op & 0x0F00) >> 8) as u8, (op & 0x00FF) as u8))
        } else if op & 0xF000 == 0x4000 {
            Some(Opcode::SNE(
                ((op & 0x0F00) >> 8).into(),
                (op & 0x00FF) as u8,
            ))
        } else if op & 0xF00F == 0x5000 {
            Some(Opcode::SER(
                ((op & 0x0F00) >> 8).into(),
                ((op & 0x00F0) >> 4).into(),
            ))
        } else if op & 0xF000 == 0x6000 {
            Some(Opcode::LD((op >> 8 & 0xF) as u8, (op & 0xFF) as u8))
        } else if op & 0xF000 == 0x7000 {
            Some(Opcode::ADD((op >> 8 & 0xF) as u8, (op & 0xFF) as u8))
        } else if op & 0xF00F == 0x8000 {
            Some(Opcode::LDR((op >> 8 & 0xF) as u8, (op >> 4 & 0xF) as u8))
        } else if op & 0xF00F == 0x8001 {
            Some(Opcode::OR((op >> 8 & 0xF) as u8, (op >> 4 & 0xF) as u8))
        } else if op & 0xF00F == 0x8002 {
            Some(Opcode::AND((op >> 8 & 0xF) as u8, (op >> 4 & 0xF) as u8))
        } else if op & 0xF00F == 0x8003 {
            Some(Opcode::XOR((op >> 8 & 0xF) as u8, (op >> 4 & 0xF) as u8))
        } else if op & 0xF00F == 0x8004 {
            Some(Opcode::ADDR((op >> 8 & 0xF) as u8, (op >> 4 & 0xF) as u8))
        } else if op & 0xF00F == 0x8005 {
            Some(Opcode::SUBR((op >> 8 & 0xF) as u8, (op >> 4 & 0xF) as u8))
        } else if op & 0xF00F == 0x8006 {
            Some(Opcode::SHR((op >> 8 & 0xF) as u8))
        } else if op & 0xF00F == 0x8007 {
            Some(Opcode::SUBRN((op >> 8 & 0xF) as u8, (op >> 4 & 0xF) as u8))
        } else if op & 0xF00F == 0x800E {
            Some(Opcode::SHL((op >> 8 & 0xF) as u8))
        } else if op & 0xF00F == 0x9000 {
            Some(Opcode::SNER((op >> 8 & 0xF) as u8, (op >> 4 & 0xF) as u8))
        } else if op & 0xF000 == 0xA000 {
            Some(Opcode::LDI(op & 0x0FFF))
        } else if op & 0xF000 == 0xB000 {
            Some(Opcode::JPOFF(op & 0x0FFF))
        } else if op & 0xF000 == 0xC000 {
            Some(Opcode::RND((op >> 8 & 0xF) as u8, (op >> 4 & 0xF) as u8))
        } else if op & 0xF000 == 0xD000 {
            Some(Opcode::DRW(
                (op >> 8 & 0xF) as u8,
                (op >> 4 & 0xF) as u8,
                (op & 0xF) as u8,
            ))
        } else if op & 0xF0FF == 0xE09E {
            Some(Opcode::SKP((op >> 8 & 0x0F) as u8))
        } else if op & 0xF0FF == 0xE0A1 {
            Some(Opcode::SKNP((op >> 8 & 0x0F) as u8))
        } else if op & 0xF0FF == 0xF00A {
            Some(Opcode::KEYSET((op >> 8 & 0x0F) as u8))
        } else if op & 0xF0FF == 0xF015 {
            Some(Opcode::DTSET((op >> 8 & 0x0F) as u8))
        } else if op & 0xF0FF == 0xF007 {
            Some(Opcode::DTGET((op >> 8 & 0x0F) as u8))
        } else if op & 0xF0FF == 0xF018 {
            Some(Opcode::STSET((op >> 8 & 0x0F) as u8))
        } else if op & 0xF0FF == 0xF01E {
            Some(Opcode::IINC((op >> 8 & 0x0F) as u8))
        } else if op & 0xF0FF == 0xF029 {
            Some(Opcode::IDIG((op >> 8 & 0x0F) as u8))
        } else if op & 0xF0FF == 0xF033 {
            Some(Opcode::BCD((op >> 8 & 0x0F) as u8))
        } else if op & 0xF0FF == 0xF055 {
            Some(Opcode::REGSSTORE((op >> 8 & 0x0F) as u8))
        } else if op & 0xF0FF == 0xF065 {
            Some(Opcode::REGLOAD((op >> 8 & 0x0F) as u8))
        } else {
            None
        }
    }

    pub fn to_instr(&self) -> Instr {
        let res = match self {
            Opcode::CLS => 0x00E0,
            Opcode::RET => 0x00EE,
            Opcode::JP(a) => 0x1000 | a,
            Opcode::CALL(a) => 0x2000 | a,
            Opcode::SE(vx, byte) => 0x3000 | (*vx as u16) << 8 | *byte as u16,
            Opcode::SNE(vx, byte) => 0x4000 | (*vx as u16) << 8 | *byte as u16,
            Opcode::SER(vx, vy) => 0x5000 | (*vx as u16) << 8 | (*vy as u16) << 4,
            Opcode::LD(vx, byte) => 0x6000 | (*vx as u16) << 8 | *byte as u16,
            Opcode::ADD(vx, byte) => 0x7000 | (*vx as u16) << 8 | *byte as u16,
            Opcode::LDR(vx, vy) => 0x8000 | (*vx as u16) << 8 | (*vy as u16) << 4,
            Opcode::OR(vx, vy) => 0x8001 | (*vx as u16) << 8 | (*vy as u16) << 4,
            Opcode::AND(vx, vy) => 0x8002 | (*vx as u16) << 8 | (*vy as u16) << 4,
            Opcode::XOR(vx, vy) => 0x8003 | (*vx as u16) << 8 | (*vy as u16) << 4,
            Opcode::ADDR(vx, vy) => 0x8004 | (*vx as u16) << 8 | (*vy as u16) << 4,
            Opcode::SUBR(vx, vy) => 0x8005 | (*vx as u16) << 8 | (*vy as u16) << 4,
            Opcode::SHR(vx) => 0x8006 | (*vx as u16) << 8,
            Opcode::SUBRN(vx, vy) => 0x8007 | (*vx as u16) << 8 | (*vy as u16) << 4,
            Opcode::SHL(vx) => 0x800E | (*vx as u16) << 8,
            Opcode::SNER(vx, vy) => 0x9000 | (*vx as u16) << 8 | (*vy as u16) << 4,
            Opcode::LDI(a) => 0xA000 | a,
            Opcode::JPOFF(a) => 0xB000 | a,
            Opcode::RND(vx, vy) => 0xC000 | (*vx as u16) << 8 | (*vy as u16) << 4,
            Opcode::DRW(vx, vy, n) => 0xD000 | (*vx as u16) << 8 | (*vy as u16) << 4 | (*n as u16),
            Opcode::SKP(a) => 0xE09E | (*a as u16) << 8,
            Opcode::SKNP(a) => 0xE0A1 | (*a as u16) << 8,

            Opcode::KEYSET(a) => 0xF00A | (*a as u16) << 8,
            Opcode::DTSET(a) => 0xF015 | (*a as u16) << 8,
            Opcode::DTGET(a) => 0xF007 | (*a as u16) << 8,
            Opcode::STSET(a) => 0xF018 | (*a as u16) << 8,
            Opcode::IINC(a) => 0xF01E | (*a as u16) << 8,
            Opcode::IDIG(a) => 0xF029 | (*a as u16) << 8,
            Opcode::BCD(a) => 0xF033 | (*a as u16) << 8,
            Opcode::REGSSTORE(a) => 0xF055 | (*a as u16) << 8,
            Opcode::REGLOAD(a) => 0xF065 | (*a as u16) << 8,
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
        assert_eq!(Opcode::from(0x8DA6), Some(Opcode::SHR(0xD)));
        assert_eq!(0x8D06, Opcode::SHR(0xD).to_instr());
    }

    #[test]
    fn subrn_test() {
        assert_eq!(Opcode::from(0x8DA7), Some(Opcode::SUBRN(0xD, 0xA)));
        assert_eq!(0x8DA7, Opcode::SUBRN(0xD, 0xA).to_instr());
    }

    #[test]
    fn shl_test() {
        assert_eq!(Opcode::from(0x8DAE), Some(Opcode::SHL(0xD)));
        assert_eq!(0x8D0E, Opcode::SHL(0xD).to_instr());
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
