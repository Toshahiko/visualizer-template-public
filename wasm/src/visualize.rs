use crate::parse::{Direction, Input, Point};
use crate::parse::Output;
use svg::node::element::{Circle, Line, Rectangle, Style};
use svg::node::Text;
pub fn visualize(input: Input, output: Output, turn: usize) ->(i64, String, String) {

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
    let mut result_history = vec![input.a] ;
    for i in 0..output.walks.len() {
        if output.walks[i].0  {
            aaa = update_board(aaa, takahashi_history[i], aoki_history[i]);
        }
        result_history.push( aaa.clone()) ;
    }





    let scale = 30;
    let W = input.n*scale;
    let H = input.n*scale;
    let w = scale;
    let h = scale;
    let mut doc = svg::Document::new()
        .set("id", "vis")
        .set("viewBox", (-5, -5, W + 10, H + 10))
        .set("width", W + 10)
        .set("height", H + 10)
        .set("style", "background-color:white");

    doc = doc.add(Style::new(format!(
        "text {{text-anchor: middle;dominant-baseline: central; font-size: {}}}",
        6
    )));
    for y in 0..input.n {
        for x in 0..input.n {
            doc = doc.add(
                rect(
                    x * w,
                    y * h,
                    w,
                    h,
                    &generate_color(result_history[turn][y][x] as usize, input.n*input.n), // 点数に応じて色を帰る。
                )
                    .set("stroke", "gray")
                    .set("stroke-width", 1)
                    .set("class", "box"),
            );
        }
    }

    for y in 0..input.n {
        for x in 0..input.n {
            doc = doc.add(
                text( x*w + scale/2, y * h + scale/2, result_history[turn][y][x])
            )
        }
    }

    for y in 0..input.n {
        for x in 0..input.n {
            if x == input.n - 1 {continue;}
            if input.v[y][x] == 1 {
                doc = doc.add(
                    line( x*w +scale, y*h, x*w +scale, y*h + scale )
                )
            }
        }
    }

    for y in 0..input.n {
        if y == input.n - 1 {continue;}
        for x in 0..input.n {
            if input.h[y][x] == 1 {
                doc = doc.add(
                    line( x*w, y*h+scale, x*w + scale, y*h +scale )
                )
            }
        }
    }

    doc = doc.add(
        circle( takahashi_history[turn].x*w + scale/2,takahashi_history[turn].y*h + scale/2, scale/2,"black" )
    );

    doc = doc.add(
        circle(  aoki_history[turn].x*w + scale/2,aoki_history[turn].y*h + scale/2, scale/2, "white" )
    );

    (100, "".to_string(), doc.to_string() )
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

pub fn rect(x: usize, y: usize, w: usize, h: usize, fill: &str) -> Rectangle {
    Rectangle::new()
        .set("x", x)
        .set("y", y)
        .set("width", w)
        .set("height", h)
        .set("fill", fill)
}

pub fn line(x1: usize, y1: usize, x2: usize, y2: usize) -> Line {
    Line::new()
        .set("x1", x1)
        .set("y1", y1)
        .set("x2", x2)
        .set("y2", y2)
        .set("stroke", "black")
        .set("stroke-width", 3)
}

pub fn text(x: usize, y: usize, value: i32) -> svg::node::element::Text {
    let text_node =Text::new(value.to_string());
    svg::node::element::Text::new()
        .set("x", x)
        .set("y", y)
        .add(text_node)
}

pub fn circle( x: usize, y:usize, radius:usize, stroke:&str) -> Circle{
    Circle::new()
        .set("cx", x)
        .set("cy", y)
        .set("r", radius)
        .set("stroke", stroke)
        .set("fill", "transparent")
        .set( "stroke-width", 2)
}

fn generate_dark_color(code: usize) -> String {
    // 入力値に基づいてHue（色相）を計算
    let hue = (code as f32 * 36.0) % 360.0;

    // Saturation（彩度）を低めに、Lightness（明度）を固定値で低く設定
    let saturation = 30.0;
    let lightness = 30.0;

    // HSL to RGB 変換
    let hue_normalized = hue / 360.0;
    let q = if lightness < 0.5 {
        lightness * (1.0 + saturation)
    } else {
        lightness + saturation - (lightness * saturation)
    };

    let p = 2.0 * lightness - q;

    let r = hue_to_rgb(p, q, hue_normalized + 1.0 / 3.0);
    let g = hue_to_rgb(p, q, hue_normalized);
    let b = hue_to_rgb(p, q, hue_normalized - 1.0 / 3.0);

    // RGB を 16 進数に変換して文字列を返す
    format!(
        "#{:02X}{:02X}{:02X}",
        (r * 255.0) as u8,
        (g * 255.0) as u8,
        (b * 255.0) as u8
    )
}

fn generate_color(code: usize, max_number: usize) -> String {
    // 入力値に基づいてHue（色相）を計算
    let hue = (code as f32 / max_number as f32) * 360.0 % 360.0;

    // Saturation（彩度）とLightness（明度）を固定値で設定
    let saturation = 10.0;
    let lightness = 0.1;

    // HSL to RGB 変換
    let hue_normalized = hue / 360.0;
    let q = if lightness < 0.5 {
        lightness * (1.0 + saturation)
    } else {
        lightness + saturation - (lightness * saturation)
    };

    let p = 2.0 * lightness - q;

    let r = hue_to_rgb(p, q, hue_normalized + 1.0 / 3.0);
    let g = hue_to_rgb(p, q, hue_normalized);
    let b = hue_to_rgb(p, q, hue_normalized - 1.0 / 3.0);

    // RGB を 16 進数に変換して文字列を返す
    format!(
        "#{:02X}{:02X}{:02X}",
        (r * 255.0) as u8,
        (g * 255.0) as u8,
        (b * 255.0) as u8
    )
}

fn hue_to_rgb(p: f32, q: f32, t: f32) -> f32 {
    let t = if t < 0.0 {
        t + 1.0
    } else if t > 1.0 {
        t - 1.0
    } else {
        t
    };

    if t < 1.0 / 6.0 {
        p + (q - p) * 6.0 * t
    } else if t < 1.0 / 2.0 {
        q
    } else if t < 2.0 / 3.0 {
        p + (q - p) * (2.0 / 3.0 - t) * 6.0
    } else {
        p
    }
}