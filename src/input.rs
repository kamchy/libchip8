const KEY_COUNT: usize = 0x10;
#[derive(Debug, Default)]
pub struct Keyboard {
    pub states: [bool; KEY_COUNT],
}

impl Keyboard {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn switch(&mut self, idx: usize) {
        self.states[idx] = !self.states[idx];
    }

    pub fn get(&self, idx: usize) -> bool {
        self.states[idx]
    }

    pub fn down_key(&self) -> Option<usize> {
        self.states.iter().position(|&i| i == true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn switch_test() {
        let mut k = Keyboard::new();
        k.switch(3);
        assert_eq!(k.get(3), true);
    }

    #[test]
    fn down_key_test() {
        let mut k = Keyboard::new();
        k.switch(3);
        assert_eq!(Some(3), k.down_key());
    }
}
