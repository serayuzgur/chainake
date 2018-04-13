use super::cell::Cell;
/// # Snake draws itself.
use std::io::Write;
use termion::cursor;

use rand::{Rng, StdRng};

const APPLE: char = '@';

pub struct Apple {
    pub position: Cell,
    pub ghost: Option<Cell>,
    pub max: Cell,
}

impl Apple {
    pub fn new(max_x: u16, max_y: u16) -> Apple {
        let mut rng = StdRng::new().unwrap();

        //Generate a random number within range.
        let x: u16 = rng.gen_range(1, max_x - 1);
        let y: u16 = rng.gen_range(1, max_y - 1);
        Apple {
            position: Cell {
                x,
                y,
                content: APPLE,
            },
            ghost: None,
            max: Cell {
                x: max_x,
                y: max_y,
                content: ' ',
            },
        }
    }

    pub fn renew(&mut self) {
        // TODO: get board for borders (range), get snake to prevent overlaping,
        let mut rng = StdRng::new().unwrap();
        //Generate a random number within range.
        let x: u16 = rng.gen_range(2, &self.max.x - 1);
        let y: u16 = rng.gen_range(2, &self.max.y - 1);
        self.ghost = Some(self.position.clone());
        self.position.x = x;
        self.position.y = y;
    }

    pub fn draw<W: Write>(&mut self, stdout: &mut W) {
        // Draw apple.
        write!(
            stdout,
            "{}{}",
            cursor::Goto(self.position.x, self.position.y),
            &self.position.content
        ).expect("Can't write to terminal");

        //Clear ghost.
        match &self.ghost {
            Some(ghost) => {
                write!(
                    stdout,
                    "{}{}",
                    cursor::Goto(ghost.x, ghost.y),
                    ' '
                ).expect("Can't write to terminal");
            }
            _=>{}
        }
        self.ghost = None;
    }
}
