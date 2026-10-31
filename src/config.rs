use crate::{blob::{Blob, function::FunctionId, pin::{InfinitePinBoard, Pin, PinBoard}}, config::ConfigInstruction::{FOP, FOR, JMP, MOV, MVI}};

pub enum ConfigInstruction {
    MOV(Pin, Pin), //Set pin x to pin y
    MVI(Pin, Pin), //Set pin x to input pin y
    FOR(Pin), // Function Output Return
    FOP(FunctionId, Pin, PinBoard<Pin>, Pin), //Call FunctionId x on pin y, Output to pin z
    JMP(usize),
}

pub struct ConfigScript {
    pub script: Vec<ConfigInstruction>,
}

pub fn interpret(script: &ConfigScript, pin_board: &mut InfinitePinBoard<Blob>, input: &mut PinBoard<Blob>) -> Option<Blob> {
    let mut current_line = 0;
    for _ in 0..script.script.len() {
        let line = &script.script[current_line];
        match line {
            MOV(a, b) => {
                let v = pin_board.take_value(&b).take();
                pin_board.set_value(*a, v);
            }
            MVI(a, b) => {
                pin_board.set_value(*a, input.get_mut_value(*b).take());
            }
            FOR(a) => {
                return pin_board.take_value(a).take()
            }
            FOP(f, pi, i, po) => {
                let mut values_taken = PinBoard::take_from_board(&i, pin_board);
                let got_value = pin_board.get_value(&pi);
                if let Some(got_value) = got_value {
                    if let Some(func) = got_value.blob_type.functions.get(&f) {
                        pin_board.set_value(*po, interpret(&func.script, &mut InfinitePinBoard::new(), &mut values_taken));
                    }
                }
            }
            JMP(v) => {
                current_line = *v;
            }
        }
        current_line += 1;
    }
    return None;
}