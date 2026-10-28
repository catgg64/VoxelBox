use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pin {
    pub id: usize,
}

#[derive(Clone)]
pub struct PinBoard<T> {
    pins: Vec<Option<T>>,
}

impl<T> PinBoard<T> {
    pub fn new() -> Self {
        let pins = vec![];
        
        Self { pins }
    }

    pub fn get_value(&self, pin: &Pin) -> &Option<T> {
        &self.pins[pin.id]
    }

    pub fn get_mut_value(&mut self, pin: &Pin) -> &mut Option<T> {
        &mut self.pins[pin.id]
    }


    pub fn set_value(&mut self, pin: Pin, value: Option<T>) {
        self.pins[pin.id] =  value;
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

    pub fn set_value(&mut self, pin: Pin, value: Option<T>) {
        self.pins.insert(pin, value);
    }
}