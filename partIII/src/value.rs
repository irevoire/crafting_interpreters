pub type Value = f64;

#[derive(Debug, Default, Clone)]
pub struct ValueArray(Vec<Value>);

impl ValueArray {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.0.push(value);
        self.0.len() - 1
    }
}
