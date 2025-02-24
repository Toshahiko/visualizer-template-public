#![allow(non_snake_case, unused_macros)]

use std::fmt::Debug;
use wasm_bindgen::prelude::*;
use crate::visualizer::visualize;
use itertools::Itertools;
use proconio::{input, marker::Chars};
use rand::prelude::*;
use std::ops::RangeBounds;

mod parser;
mod visualizer;
mod converter;

#[wasm_bindgen]
pub fn gen(seed: i32) -> String {
    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(seed as u64);
    let ty = seed % 20;
    let mut input = parse_input_fixed(FIXED[ty as usize]);
    let mut nums = (1..=input.n * input.n).collect_vec();
    nums.shuffle(&mut rng);
    input.a = crate::mat![0; input.n; input.n];
    for i in 0..input.n {
        for j in 0..input.n {
            input.a[i][j] = nums[i * input.n + j] as i32;
        }
    }
    format!("{}", input)
}

#[wasm_bindgen(getter_with_clone)]
pub struct Ret {
    pub score: i64,
    pub err: String,
    pub svg: String,
}

#[wasm_bindgen]
pub fn vis(_input: String, _output: String, turn: usize) -> Ret {
    let input = parser::parse_input( &_input ) ;
    let input_data = converter::convert_input(&input);

    let output = parser::parse_output(&_output) ;
    let output_data = converter::convert_output(&input, output);
    let( score, err, svg ) = visualize(input_data, output_data, turn) ;

    Ret {
        score,
        err,
        svg,
    }
}

#[wasm_bindgen]
pub fn get_max_turn(_input: String, _output: String) -> usize {
    parser::parse_output( &_output ).max_step
}

#[macro_export]
macro_rules! mat {
	($($e:expr),*) => { Vec::from(vec![$($e),*]) };
	($($e:expr,)*) => { Vec::from(vec![$($e),*]) };
	($e:expr; $d:expr) => { Vec::from(vec![$e; $d]) };
	($e:expr; $d:expr $(; $ds:expr)+) => { Vec::from(vec![mat![$e $(; $ds)*]; $d]) };
}

#[derive(Clone, Debug)]
pub struct Input {
    pub ty: u64,
    pub n: usize,
    pub a: Vec<Vec<i32>>,
    pub vs: Vec<Vec<char>>,
    pub hs: Vec<Vec<char>>,
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {}", self.ty, self.n)?;
        for i in 0..self.n {
            writeln!(f, "{}", self.vs[i].iter().collect::<String>())?;
        }
        for i in 0..self.n - 1 {
            writeln!(f, "{}", self.hs[i].iter().collect::<String>())?;
        }
        for i in 0..self.n {
            writeln!(f, "{}", self.a[i].iter().join(" "))?;
        }
        Ok(())
    }
}
pub fn parse_input_fixed(f: &str) -> Input {
    let f = proconio::source::once::OnceSource::from(f);
    input! {
        from f,
        ty: u64, n: usize,
        vs: [Chars; n],
        hs: [Chars; n - 1],
    }
    for i in 0..n {
        assert_eq!(vs[i].len(), n - 1);
    }
    for i in 0..n - 1 {
        assert_eq!(hs[i].len(), n);
    }
    Input {
        ty,
        n,
        a: vec![],
        vs,
        hs,
    }
}

const FIXED: [&'static str; 20] = [
    include_str!("../in_fixed/0.txt"),
    include_str!("../in_fixed/1.txt"),
    include_str!("../in_fixed/2.txt"),
    include_str!("../in_fixed/3.txt"),
    include_str!("../in_fixed/4.txt"),
    include_str!("../in_fixed/5.txt"),
    include_str!("../in_fixed/6.txt"),
    include_str!("../in_fixed/7.txt"),
    include_str!("../in_fixed/8.txt"),
    include_str!("../in_fixed/9.txt"),
    include_str!("../in_fixed/10.txt"),
    include_str!("../in_fixed/11.txt"),
    include_str!("../in_fixed/12.txt"),
    include_str!("../in_fixed/13.txt"),
    include_str!("../in_fixed/14.txt"),
    include_str!("../in_fixed/15.txt"),
    include_str!("../in_fixed/16.txt"),
    include_str!("../in_fixed/17.txt"),
    include_str!("../in_fixed/18.txt"),
    include_str!("../in_fixed/19.txt"),
];
