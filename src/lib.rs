pub mod cpu;
pub mod display;
pub mod emulator;
pub mod input;
pub mod loader;
pub mod mem;

#[cfg(test)]
/// Tests
/// TODO should be moved to relevant modules
mod tests {
    use super::cpu;
    use super::display;
    use super::emulator;
    use super::mem;

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

        e.store_instr(&[cpu::Opcode::JP(0x0123).to_instr()]);

        e.run();
        assert_eq!(e.cpu.pc, 0x0123);
    }

    #[test]
    fn exec_jump_ret_test() {
        let mut e = emulator::Emulator::new();
        e.store(&[
            cpu::Opcode::CALL(0x204),
            cpu::Opcode::JP(0x209),
            cpu::Opcode::CLS,
            cpu::Opcode::RET,
        ]);
        e.run();
        assert_eq!(
            e.mem.get(0x200..=0x208),
            Some(&[0x22, 0x04, 0x12, 0x09, 0x00, 0xE0, 0x00, 0xEE, 0x00][..])
        );
        ae(e.cpu.pc, 0x0209);
    }
    fn ae(l: u16, r: u16) {
        assert_eq!(l, r, "\nl=0x{:04X}, r=0x{:04X}", l, r);
    }

    #[test]
    fn store_instr_test() {
        let mut e = emulator::Emulator::new();
        e.store(&[
            cpu::Opcode::JP(0x0105),
            cpu::Opcode::JP(0x0ABC),
            cpu::Opcode::CALL(0x0123),
            cpu::Opcode::SE(0x4, 0xFF),
        ]);
        assert_eq!(
            e.mem.get(0x200..=0x207),
            Some(&[0x11, 0x05, 0x1A, 0xBC, 0x21, 0x23, 0x34, 0xFF][..])
        );
    }
}
