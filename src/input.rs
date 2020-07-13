#[derive(Debug, Default)]
pub struct Keyboard {
    pub states: [bool; 16],
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
}
