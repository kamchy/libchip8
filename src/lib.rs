pub mod cpu;
pub mod mem;
pub mod emulator;
pub mod display;

#[cfg(test)]
mod tests {
    use super::cpu;
    use super::emulator;
    use super::mem;
    use super::display;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn correct_init() {
        let cpu = cpu::CPU::new();
        assert_eq!(cpu.pc, 0);
    }

    #[test]
    fn emulator_test() {
       let e = emulator::Emulator::new();
       assert_eq!(e.cpu, cpu::CPU::new());
    }

    #[test]
    fn pc_test() {
       let mut cpu = cpu::CPU::new();
       cpu.pc(0x234);
       assert_eq!(cpu.pc, 0x234);
    }

    #[test]
    fn mem_test() {
        let mut mem = mem::Mem::new();
        mem.store(3, 34);
        assert_eq!(mem.load(3), 34u8);

    }

    #[test]
    fn memget_test() {
        let mut mem = mem::Mem::new();
        mem.store(3, 34);
        mem.store(5, 4);
        assert_eq!(mem.get(2..=5), Some(&[0, 34u8, 0, 4][..]));
    }

    #[test]
    fn font_test() {
        let mut mem = mem::Mem::new();
        mem.store_font(0);
        assert_eq!(mem.get(2..=4), Some(&[0x90, 0x90, 0xF0][..]));
    }

    #[test]
    fn display_test() {
       let mut d = display::Screen::new();
       d.switch(2, 2);
       d.switch(4, 4);
       d.switch(4, 4);
       d.switch(100, 100);

       assert_eq!(d.get(2, 2), true);
       assert_eq!(d.get(36, 4), true);
       assert_eq!(d.get(4, 4), false);
    }

    #[test]
    fn exec_test() {
        let mut e = emulator::Emulator::new();
        e.mem.store_font(0);

        let prog = [cpu::Opcode::JP(0x0123)];
        e.store(&prog);
        e.run();
        assert_eq!(e.cpu.pc, 0x0123);
    }

    #[test]
    fn store_instr_test() {
        let mut e = emulator::Emulator::new();
        e.store(&[
            cpu::Opcode::JP(0x0105),
            cpu::Opcode::JP(0x0ABC)]);
        assert_eq!(e.mem.get(200..=203),
                   Some(&[0x11, 0x05, 0x1A, 0xBC][..]));
    }
}

