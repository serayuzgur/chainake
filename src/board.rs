/// # Board draws itself.

use std::io::Write;
use termion::cursor;
use super::cell::Cell;

/// Empty char for default.
const EMPTY: char = ' ';
/// Top-left corner
const TL: char = '╔';
/// Top-right corner
const TR: char = '╗';
/// Bottom-left corner
const BL: char = '╚';
/// Bottom-right corner
const BR: char = '╝';
/// Horizontal line
const HL: char = '═';
/// Vertical line
const VL: char = '║';

pub struct Board {
    /// Width of the board
    pub width: u16,
    /// Height of the board
    pub height: u16,

    pub grid : Vec<Cell>
}

impl Board {

    pub fn new(width:u16, height:u16) -> Board {
        let mut grid = Vec::<Cell>::new();
        for x in 0..width {
            for y in 0..height {
                let mut ch:char = EMPTY;
                if x == 0 {
                    if y == 0 {
                        ch = TL;
                    } else if y == (height - 1) {
                        ch = BL;
                    } else {
                        ch = VL;
                    }
                } else if y == 0 {
                    if x == (width - 1) {
                        ch = TR;
                    } else {
                        ch = HL;
                    }
                } else if x == width - 1 {
                    if y == (height - 1) {
                        ch = BR;
                    } else {
                        ch = VL;
                    }
                } else if y == height - 1 {
                    ch = HL;
                }
                //set cursor position and write the character.
                grid.push(Cell { x, y, content:ch});
            }
        }
        Board {
            width,
            height,
            grid
        }
    }
    /// Draws the board.
    pub fn draw<W: Write>(&self, stdout: &mut W) {
        // traverses all width and height and prints border chracters.
        for cell in &self.grid {
             //set cursor position and write the character.
             write!(stdout, "{}{}", cursor::Goto(cell.x as u16 + 1, cell.y as u16 + 1), cell.content)
                .expect("Can't write to terminal");
        }
    }
}
