use crate::mem;
pub type Addr = u16;
pub type Instr = u16;
pub type Regs = [u8; 16];

#[derive(Default, PartialEq, Debug)]
pub struct CPU {
    pub pc: Addr,
    pub i: Addr,
    pub v: Regs,
    pub sp: Addr,
    pub instr: Option<Opcode>,
}

impl CPU {
    pub fn from(pc: Addr, i: Addr, v: Regs, sp: Addr, instr: Option<Opcode>) -> Self {
        CPU { pc, i, v, sp, instr}
    }

    pub fn new() -> Self {
        Default::default()
    }

    pub fn pc(&mut self, pc: Addr) -> &Self {
        self.pc = pc;
        self
    }

    fn load_instr(mem: &mem::Mem, i: Addr) -> Instr {
        let bh: Instr = mem.load(i).into();
        let bl: Instr = mem.load(i+1).into();
        (bh << 8) | bl
    }



    /// Fetches next instruction (Opcode enum) from location
    /// pointed to by cpu pc register
    pub fn fetch(&mut self, mem: &mem::Mem) -> Option<Opcode> {
        let instr = CPU::load_instr(mem, self.pc);
        let opt_optcode = Opcode::from(instr);
        opt_optcode
    }

}

#[derive(Debug, PartialEq)]
pub enum Opcode {
    JP(Addr),
}

impl Opcode {
    pub fn from(op: u16) -> Option<Opcode> {
       if op >> 12 == 0x0001 {
           Some(Opcode::JP(op & 0x0FFF))
       } else {
           None
       }
    }

    pub fn to_instr(&self) -> Instr {
        match self {
            Opcode::JP(a) => 0x1000 | a,
        }


    }

}
