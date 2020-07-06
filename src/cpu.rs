pub type Addr = u16;
pub type Opcode = u16;
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

}
