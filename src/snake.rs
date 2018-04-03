/// # Snake draws itself.
use std::io::Write;
use termion::cursor;

const EMPTY: &'static str = "";
const SNAKE: &'static str = "▓";

pub enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

#[derive(Clone)]
pub struct Point {
    pub x: i8,
    pub y: i8,
}

pub struct Snake {
    pub body: Vec<Point>,
    pub direction: Direction,
}
fn give_last(vec:Vec<Point>) -> Point {
    return vec.last().unwrap().clone();
}

impl Snake {
    pub fn move_1(&mut self,direction: Direction){
        let diff = match direction {
            Direction::UP => (0, -1),
            Direction::RIGHT => (1, 0),
            Direction::DOWN => (0, 1),
            Direction::LEFT => (-1, 0),
        };
        
        let body = &mut self.body;
        let last = give_last(body.to_vec());
        let next = Point { 
            x: last.x + diff.0, 
            y: last.y+diff.1
        };
        &body.remove(0);
        &body.push(next);
    }
    pub fn draw<W: Write>(&mut self, stdout: &mut W) {
        for point in &self.body {
            write!(
                stdout,
                "{}{}",
                cursor::Goto(point.x as u16 , point.y as u16),
                SNAKE
            ).expect("Can't write to terminal");
        }
    }
}
