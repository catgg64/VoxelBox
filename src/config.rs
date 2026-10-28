use crate::blob::{function::FunctionId, pin::Pin};

pub enum InstructionPin {
    Pin(Pin),
    InputPin(Pin),
}

pub enum ConfigInstruction {
    SPP(InstructionPin, InstructionPin), //Set pin x to pin y
    SPI(InstructionPin, InstructionPin), //Set pin x to input pin y
    SIP(InstructionPin, InstructionPin), //Set input x to pin y
    FOR(InstructionPin), // Function Output Return
    FOP(FunctionId, Vec<InstructionPin>, InstructionPin), //Call FunctionId x on pin y, Output to pin z
    JMP(u32),
}

pub struct ConfigScript {
    pub script: Vec<ConfigInstruction>,
}