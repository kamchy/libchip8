pub type Addr = u16;
pub type Instr = u16;
pub type Reg = u8;
pub type Regs = [Reg; 16];

#[derive(Default, PartialEq, Debug)]
pub struct CPU {
    pub pc: Addr,
    pub i: Addr,
    pub regs: Regs,
    pub sp: Addr,
    stack: Vec<Addr>,
    pub instr: Option<Opcode>,
}

impl CPU {
    pub fn from(pc: Addr, i: Addr, regs: Regs, sp: Addr, instr: Option<Opcode>) -> Self {
        CPU {
            pc,
            i,
            regs,
            sp,
            stack: vec![],
            instr,
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

    fn skip_if(&mut self, pred: bool) {
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
        self.regs[vx as usize] += byte;
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
}

#[derive(Debug, PartialEq)]
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
        };
        println!("\nto_instr for {:?} - {:02X}\n", &self, res);
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
}
