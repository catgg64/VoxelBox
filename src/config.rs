use crate::{blob::{Blob, function::FunctionId, pin::{InfinitePinBoard, Pin, PinBoard}}, config::ConfigInstruction::{FOP, FOR, MOV, MVI}};

pub enum ConfigInstruction {
    MOV(Pin, Pin), //Set pin x to pin y
    MVI(Pin, Pin), //Set pin x to input pin y
    FOR(Pin), // Function Output Return
    FOP(FunctionId, Pin, Vec<Pin>, Pin), //Call FunctionId x on pin y, Output to pin z
    JMP(u32),
}

pub struct ConfigScript {
    pub script: Vec<ConfigInstruction>,
}

pub fn interpret(script: &ConfigScript, pin_board: InfinitePinBoard<Blob>, input: PinBoard<Blob>) -> Option<Blob> {
    for line in script.script {
        match line {
            MOV(a, b) => {
                pin_board.set_value(a, pin_board.get_value(&b).take());
            }
            MVI(a, b) => {
                pin_board.set_value(a, input.get_value(&b).take());
            }
            FOR(a) => {
                return *pin_board.get_value(&a)
            }
            FOP(f, pi, i, po) => {
                let got_value = pin_board.get_value(&pi);
                if let Some(got_value) = got_value {
                    if let Some(func) = got_value.blob_type.functions.get(&f) {
                        interpret(func.script, pin_board, input)
                    }
                }
                pin_board.set_value(po, );
            }
        }
    }
    return None;
}