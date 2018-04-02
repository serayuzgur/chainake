extern crate termion;

use termion::{clear};
use termion::raw::IntoRawMode;

use std::io::{self, Read, Write};

mod board;

use board::Board;

/// The string printed for flagged cells.
const SNAKE: &'static str = "▓";

/// The string printed for concealed cells.
const APPLE: &'static str = "▒";

struct Chainake<R: Read, W: Write> {
    board: Board,
    stdout: W,
    stdin: R,
}

/// A snake application written with rust
impl<R: Read, W: Write> Chainake<R, W> {
    /// Play the game
    fn play(&mut self, difficulty: usize) {
        println!("Hello All Difficulty is {}", difficulty);
        self.start();
    }

    fn start(&mut self) {
        write!(&mut self.stdout, "{}", clear::All).unwrap();
        self.board.draw(&mut self.stdout);
        loop {
            let mut buf: [u8; 3] = [0, 0, 0];
            match self.stdin.read(&mut buf) {
                Ok(_) => {
                    match buf.get(0).unwrap() {
                        //113=q 27 starting of arrow and esc keys
                        &113 | &27 => {
                            match buf.get(1).unwrap() {
                                &91 => {
                                    match buf.get(2).unwrap() {
                                        /*TOP*/
                                        &65 => println!("UP"),
                                        /*BOTTOM*/
                                        &66 => println!("DOWN"),
                                        /*RIGHT*/
                                        &67 => println!("RIGHT"),
                                        /*LEFT*/
                                        &68 => println!("LEFT"),
                                        _ => {}
                                    }
                                }
                                &0 => {
                                    break;
                                }
                                _ => {}
                            }
                        }
                        /*W*/
                        &119 => println!("UP"),
                        /*S*/
                        &115 => println!("DOWN"),
                        /*D*/
                        &100 => println!("RIGHT"),
                        /*A*/
                        &97 => println!("LEFT"),
                        &32 => println!("START"),
                        _ => {
                            self.stdout
                                .write(&format!("Pressed: {:?}\n", &buf).as_bytes());
                            self.stdout.flush();
                        }
                    }
                }
                Err(error) => {
                    self.stdout.write(&format!("Error: {}", error).as_bytes());
                    self.stdout.flush();
                }
            }
        }
        self.stdout.write("Exiting".as_bytes());
        self.stdout.flush();
    }
}

fn main() {
    let stdout = io::stdout();
    stdout.lock();
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let stderr = io::stderr();
    stderr.lock();
    let stdout = stdout.into_raw_mode().unwrap();
    let term_size = termion::terminal_size().ok();
    let term_width = term_size.map(|(w, _)| w - 2);
    let term_height = term_size.map(|(_, h)| h - 2);
    let mut game = Chainake {
        board: Board {
            width: term_width.unwrap(),
            height: term_height.unwrap(),
        },
        stdin,
        stdout,
    };
    game.play(1);
}
