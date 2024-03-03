use proconio::input;
use proconio::source::once::OnceSource;

pub struct Input {
    pub t: usize,
    pub n: usize,
    pub v: Vec<Vec<i32>>,
    pub h: Vec<Vec<i32>>,
    pub a: Vec<Vec<i32>>,
}

pub struct Output {
    pub max_step: usize,
    pub takahashi_first: Point,
    pub aoki_first: Point,
    pub walks: Vec<(bool, Direction, Direction)>,
}

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Stop,
}

#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

pub fn parse_input( input: &str) -> Input {
    let source = OnceSource::from( input) ;
    input! {
        from source,
        _t: usize,
        n: usize,
        v_input: [String; n],
        h_input: [String; n-1],
        a: [[i32; n]; n],
    }

    let mut v: Vec<Vec<i32>> = vec![];
    let mut h: Vec<Vec<i32>> = vec![];

    for v_ in v_input {
        let mut _v: Vec<i32> = vec![];
        for v_char in v_.chars() {
            _v.push(v_char as i32 - 48);
        }
        v.push(_v);
    }

    for h_ in h_input {
        let mut _h: Vec<i32> = vec![];
        for h_char in h_.chars() {
            _h.push(h_char as i32 - 48);
        }
        h.push(_h);
    }

    Input{t: _t,n,v,h, a}
}

pub fn parse_output(output:  &String) -> Output {
    let lines = output.lines().collect::<Vec<&str>>();
    let numbers: Vec<usize> = lines[0]
        .split_whitespace() // 空白で文字列を分割
        .filter_map(|s| s.parse().ok()) // 各文字列をusizeにパースし、結果がOkなら値を取得
        .collect(); // 結果をVec<usize>にまとめる

    let takahashi_first = Point { x: numbers[1], y: numbers[0] };
    let aoki_first = Point { x: numbers[3], y: numbers[2] };

    let walks: Vec<(bool, Direction, Direction)> = lines[1..]
        .iter().filter_map( |line| Some(translate(line)) )
        .collect();

    Output{
        max_step: walks.len(),
        takahashi_first,
        aoki_first,
        walks
    }

}

fn translate(line: &str ) -> (bool, Direction, Direction) {
    let ss : Vec<String> = line.split_whitespace()
        .filter_map(|s| s.parse().ok()) // 各文字列をusizeにパースし、結果がOkなら値を取得
        .collect(); // 結果をVec<usize>にまとめる

    (
        if ss[0] == "0" { false} else if ss[0] == "1" { true} else { panic!()},
        seek_direction(&ss[1]),
        seek_direction(&ss[2])
    )
}

fn seek_direction( text: &String ) -> Direction {
    if text == "U" {
        return Direction::Up
    } else if text == "D" {
        return Direction::Down
    } else if text == "L" {
        return Direction::Left
    } else if text == "R" {
        return Direction::Right
    } else if text == "."{
        return Direction::Stop
    } else {
        panic!()
    }
}
