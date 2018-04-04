/// # Snake draws itself.
use std::io::Write;
use termion::cursor;
use super::cell::Cell;

const EMPTY: char = ' ';
const SNAKE: char = '█';

#[derive(Debug)]
pub enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

pub struct Snake {
    /// Holds body cells of the snake.
    pub body: Vec<Cell>,
    /// Hold the direction of the snake.
    pub direction: Direction,
    /// Holds the deleted cells of the snake for clearing grid.
    pub ghost: Vec<Cell>
}
fn give_last(vec:Vec<Cell>) -> Cell {
    return vec.last().unwrap().clone();
}

impl Snake {
    pub fn new(x:u16, y:u16) -> Snake {
        Snake {
            body: vec![
                Cell {
                    x,
                    y,
                    content:SNAKE
                },
                Cell {
                    x:x.saturating_add(1),
                    y,
                    content:SNAKE
                },
                Cell {
                    x:x.saturating_add(2),
                    y,
                    content:SNAKE
                },
            ],
            direction: Direction::RIGHT,
            ghost:Vec::<Cell>::new()
        }
    }
    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }
    pub fn move_1(&mut self){
        let body = &mut self.body;
        let last = give_last(body.to_vec());
        let mut next = Cell { 
            x: last.x,
            y: last.y,
            content: SNAKE
        };
        
        match &self.direction {
            Direction::UP => { next.y = last.y.saturating_sub(1);},
            Direction::RIGHT => { next.x = last.x.saturating_add(1);},
            Direction::DOWN => { next.y = last.y.saturating_add(1);},
            Direction::LEFT => { next.x = last.x.saturating_sub(1);},
        };        
        let deleted = &body.remove(0);
        &body.push(next.clone());
        &self.ghost.push(deleted.clone());
    }
    pub fn draw<W: Write>(&mut self, stdout: &mut W) {
        // Draw snakes body.
        for cell in &self.body {
            write!(
                stdout,
                "{}{}",
                cursor::Goto(cell.x , cell.y),
                cell.content
            ).expect("Can't write to terminal");
        }

        // Clear snakes ghost (backtrace).
        for cell in &self.ghost {
            write!(
                stdout,
                "{}{}",
                cursor::Goto(cell.x , cell.y),
                EMPTY
            ).expect("Can't write to terminal");
        }
        &self.ghost.clear();
    }
}
