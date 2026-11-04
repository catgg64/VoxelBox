use std::{fs::File, io::{BufRead, BufReader}, pin};

use serde::{Deserialize, Serialize};

use crate::{blob::{Blob, function::FunctionId, pin::{Pin, PinBoard, PinHeap}}, bundle::BundleProgram, config::ConfigInstruction::{BFP, BGV, FOP, FOR, JMP, MOV, MVI}};

#[derive(Serialize, Deserialize)]
pub enum ConfigInstruction {
    MOV(Pin, Pin), //Set pin x to pin y
    MVI(Pin, Pin), //Set pin x to input pin y
    FOR(Pin), // Function Output Return
    FOP(FunctionId, Pin, PinBoard<Pin>, Pin), //Call FunctionId x on pin y, Output to pin z
    BGV(Pin, Pin), // Set x to self's value y
    BFP(FunctionId, PinBoard<Pin>, Pin), // Set z to self's function x, output y
    JMP(usize), // Jump to
}

#[derive(Serialize, Deserialize)]
pub struct ConfigScript {
    pub script: Vec<ConfigInstruction>,
}

pub fn interpret(mut current_blob: Option<&mut Blob>, script: &ConfigScript, pin_heap: &mut PinHeap<Blob>, input: &mut PinBoard<Blob>, bundles: &BundleProgram) -> Option<Blob> {
    let mut current_line = 0;
    for _ in 0..script.script.len() {
        let line = &script.script[current_line];
        match line {
            MOV(a, b) => {
                let v = pin_heap.take_value(&b).take();
                pin_heap.set_value(*a, v);
            }
            MVI(a, b) => {
                pin_heap.set_value(*a, input.get_mut_value(*b).take());
            }
            FOR(a) => {
                return pin_heap.take_value(a).take()
            }
            FOP(f, pi, i, po) => {
                let mut values_taken = PinBoard::take_from_board(&i, pin_heap);
                let mut got_value = pin_heap.get_mut_value(&pi).take();
                if let Some(ref mut got_value) = got_value {
                    let got_value_blob_type = got_value.blob_type.clone();
                    pin_heap.set_value(*po, interpret(Some(got_value), &bundles.bundles[&got_value_blob_type.bundle_id].blob_types[&got_value_blob_type.blob_type_id].functions[f].script, &mut PinHeap::new(), &mut values_taken, bundles));
                }
                pin_heap.set_value(*pi, got_value);
            }
            BGV(b, v) => {
                match current_blob {
                    Some(ref mut current_blob) => {
                        pin_heap.set_value(*v, current_blob.params.get_mut_value(*v).take());
                    }
                    None => panic!("current blob is none")
                }
                if let Some(mut bb) = pin_heap.get_mut_value(b).take() {
                    if let Some(param) = bundles.bundles[&bb.blob_type.bundle_id].blob_types[&bb.blob_type.blob_type_id].type_params.get_value(*v) {
                        if param.0 {
                            if let Some(_) = bb.params.get_value(*v) {
                                pin_heap.set_value(*b, bb.params.get_mut_value(*v).take());
                            }
                        }
                    }
                    pin_heap.set_value(*b, Some(bb));
                }
            }
            BFP(f, i, o) => {
                match current_blob {
                    None => panic!("current blob is none"),
                    Some(ref mut got_value) => {
                        let mut values_taken = PinBoard::take_from_board(&i, pin_heap);
                        let got_value_blob_type = got_value.blob_type.clone();
                        pin_heap.set_value(*o, interpret(Some(got_value), &bundles.bundles[&got_value_blob_type.bundle_id].blob_types[&got_value_blob_type.blob_type_id].functions[f].script, &mut PinHeap::new(), &mut values_taken, bundles));
                        //pin_heap.set_value(*o, interpret(Some(&mut got_value), &bundles.bundles[&got_value.blob_type.bundle_id].blob_types[&got_value.blob_type.blob_type_id].functions[f].script, &mut PinHeap::new(), &mut PinBoard::take_from_board(i, pin_heap), bundles));
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

// pub fn rustify_dot_pin(code: &File) -> (ConfigScript, Vec<Blob>) {
//     let buffer = BufReader::new(code);
//     let mut output_script = ConfigScript{ script: vec![] };
//     for line in buffer.lines() {

//     }
//     (output_script)
// }