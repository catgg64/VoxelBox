use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pin {
    pub id: usize,
}

pub struct PinBoard<T> {
    pins: HashMap<Pin, Option<T>>,
}

impl<T> PinBoard<T> {
    fn new(size: usize) -> Self {
        let mut pins = HashMap::new();
        
        for i in 0..size {
            pins.insert(Pin{ id: i }, None);
        }

        Self { pins }
    }
}