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
}
