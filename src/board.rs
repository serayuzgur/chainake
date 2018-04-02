/// # Board draws itself.
use std::io::Write;
use termion::cursor;

/// Empty char for default.
const EMPTY: &'static str = "";
/// Top-left corner
const TL: &'static str = "╔";
/// Top-right corner
const TR: &'static str = "╗";
/// Bottom-left corner
const BL: &'static str = "╚";
/// Bottom-right corner
const BR: &'static str = "╝";
/// Horizontal line
const HL: &'static str = "═";
/// Vertical line
const VL: &'static str = "║";

pub struct Board {
    /// Width of the board
    pub width: u16,
    /// Height of the board
    pub height: u16,
}

impl Board {
    /// Draws the board.
    pub fn draw<W: Write>(&self, stdout: &mut W) {
        // traverses all width and height and prints border chracters.
        for x in 0..self.width {
            for y in 0..self.height {
                let mut ch = EMPTY;
                if x == 0 {
                    if y == 0 {
                        ch = TL;
                    } else if y == (self.height - 1) {
                        ch = BL;
                    } else {
                        ch = VL;
                    }
                } else if y == 0 {
                    if x == (self.width - 1) {
                        ch = TR;
                    } else {
                        ch = HL;
                    }
                } else if x == self.width - 1 {
                    if y == (self.height - 1) {
                        ch = BR;
                    } else {
                        ch = VL;
                    }
                } else if y == self.height - 1 {
                    ch = HL;
                }
                //set cursor position and write the character.
                write!(stdout, "{}{}", cursor::Goto(x + 1, y + 1), ch)
                    .expect("Can't write to terminal");
            }
        }
    }
}
