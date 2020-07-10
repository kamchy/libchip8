#[cfg(test)]
mod xtests {

    use libchip8::emulator::Emulator;
    use libchip8::loader::load;

    #[test]
    fn load_test() {
        let mut e = Emulator::new();
        load(&mut e, &String::from("tests/hex.b"));
        e.run();
        assert_eq!(e.cpu.regs[1], 0xE);
    }
}
