use crate::parser::{Direction, Input, Output, Point};
use crate::visualizer::{InputData, OutputData};

pub fn convert_input(input: &Input) -> InputData {
    InputData{
        t: input.t,
        n: input.n,
        v: input.v.clone(),
        h: input.h.clone(),
    }
}

pub fn convert_output(input: &Input, output: Output) -> OutputData {

    let mut takahashi_history : Vec<Point> = vec![ output.takahashi_first ];
    let mut aoki_history : Vec<Point> = vec![ output.aoki_first ];
    for (_, dir1, dir2) in output.walks.iter() {
        let before = takahashi_history.last().unwrap();
        let next :Point  = next_position( before, dir1) ;
        takahashi_history.push(next ) ;

        let before = aoki_history.last().unwrap();
        let next :Point  = next_position( before, dir2) ;
        aoki_history.push(next ) ;
    }

    if output.walks.len() != takahashi_history.len() -1 || output.walks.len() != aoki_history.len() -1 {
        panic!()
    }

    let mut aaa = input.a.clone();
    let mut result_history = vec![input.a.clone()] ;
    for i in 0..output.walks.len() {
        if output.walks[i].0  {
            aaa = update_board(aaa, takahashi_history[i], aoki_history[i]);
        }
        result_history.push( aaa.clone()) ;
    }

    OutputData{
        takahashi_history,
        aoki_history,
        result_history,
    }
}


fn next_position( before : &Point, dir : &Direction) -> Point{
    let next :Point = if *dir == Direction::Up {
        Point {x: before.x, y:before.y -1 }
    } else if *dir == Direction::Down {
        Point {x: before.x, y:before.y +1}
    } else if *dir == Direction::Left {
        Point {x: before.x -1, y:before.y }
    } else if *dir == Direction::Right {
        Point {x: before.x+1, y:before.y }
    } else if *dir == Direction::Stop {
        Point {x: before.x, y:before.y }
    } else {
        panic!()
    };

    next
}

fn update_board( a:Vec<Vec<i32>>, takahashi: Point, aoki:Point ) -> Vec<Vec<i32>> {
    let mut copy = a.clone() ;
    let tmp =copy[takahashi.y][takahashi.x] ;
    copy[takahashi.y][takahashi.x] = copy[aoki.y][aoki.x] ;
    copy[aoki.y][aoki.x] = tmp;
    copy
}
