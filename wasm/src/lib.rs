use wasm_bindgen::prelude::*;
use crate::visualize::visualize;

mod parse;
mod visualize;

#[wasm_bindgen]
pub fn gen(seed: i32) -> String {
    // util::gen(seed as u64).to_string()
    "".to_string()
}

#[wasm_bindgen(getter_with_clone)]
pub struct Ret {
    pub score: i64,
    pub err: String,
    pub svg: String,
}

#[wasm_bindgen]
pub fn vis(_input: String, _output: String, turn: usize) -> Ret {
    let input = parse::parse_input( &_input ) ;
    let output = parse::parse_output(&_output) ;
    let( score, err, svg ) = visualize(input, output, turn) ;

    Ret {
        score,
        err,
        svg,
    }
}

#[wasm_bindgen]
pub fn get_max_turn(_input: String, _output: String) -> usize {
    parse::parse_output( &_output ).max_step
}
