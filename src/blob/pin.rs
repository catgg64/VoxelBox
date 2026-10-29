use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pin {
    pub id: usize,
}

#[derive(Debug, Clone)]
pub struct PinBoard<T> {
    values: Vec<Option<T>>,
}

impl<T> PinBoard<T> {
    pub fn new() -> Self {
        let values = vec![];
        
        Self { values }
    }

    pub fn get_value(&self, pin: &Pin) -> &Option<T> {
        &self.values[pin.id]
    }

    pub fn get_mut_value(&mut self, pin: &Pin) -> &mut Option<T> {
        &mut self.values[pin.id]
    }


    pub fn set_value(&mut self, pin: Pin, value: Option<T>) {
        self.values[pin.id] =  value;
    }

    pub fn take(&mut self) -> Self {
        let mut values = vec![];
        for v in &mut self.values {
            values.push(v.take());
        }
        Self { values }
    }
}


pub struct InfinitePinBoard<T> {
    pins: HashMap<Pin, Option<T>>,
}

impl<T> InfinitePinBoard<T> {
    pub fn new() -> Self {
        let pins = HashMap::new();
        
        Self { pins }
    }

    pub fn take_value(&mut self, pin: &Pin) -> Option<T> {
        self.pins.remove(pin)?
    }

    pub fn get_value(&self, pin: &Pin) -> &Option<T> {
        &self.pins[pin]
    }

    pub fn get_mut_value(&mut self, pin: &Pin) -> &mut Option<T> {
        self.pins.get_mut(pin).unwrap()
    }

    pub fn set_value(&mut self, pin: Pin, value: Option<T>) {
        self.pins.insert(pin, value);
    }
}