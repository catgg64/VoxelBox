use crate::{blob::{Blob, function::FunctionId, pin::{InfinitePinBoard, Pin, PinBoard}}, config::ConfigInstruction::{FOP, FOR, JMP, MOV, MVI}};

#[derive(Clone)]
pub enum ConfigInstruction {
    MOV(Pin, Pin), //Set pin x to pin y
    MVI(Pin, Pin), //Set pin x to input pin y
    FOR(Pin), // Function Output Return
    FOP(FunctionId, Pin, PinBoard<Blob>, Pin), //Call FunctionId x on pin y, Output to pin z
    JMP(u32),
}

#[derive(Clone)]
pub struct ConfigScript {
    pub script: Vec<ConfigInstruction>,
}

pub fn interpret(script: &mut ConfigScript, pin_board: &mut InfinitePinBoard<Blob>, input: &mut PinBoard<Blob>) -> Option<Blob> {
    let mut current_line = 0;
    for n in 0..script.script.len() {
        let line = &mut script.script[current_line];
        match line {
            MOV(a, b) => {
                let v = pin_board.take_value(&b).take();
                pin_board.set_value(*a, v);
            }
            MVI(a, b) => {
                pin_board.set_value(*a, input.get_mut_value(&b).take());
            }
            FOR(a) => {
                return pin_board.take_value(a).take()
            }
            FOP(f, pi, i, po) => {
                let got_value = pin_board.get_value(&pi);
                if let Some(got_value) = got_value {
                    if let Some(func) = got_value.blob_type.functions.get(&f) {
                        pin_board.set_value(*po, interpret(&mut func.script.clone(), &mut InfinitePinBoard::new(), i));
                    }
                }
            }
            JMP(v) => {
                
            }
        }
        current_line += 1;
    }
    return None;
}