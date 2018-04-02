extern crate termion;

use termion::{clear, cursor, style};
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;

use std::env;
use std::io::{self, Read, Write};
use std::process;
use std::iter::Iterator;

mod board;

use board::Board;

/// The string printed for flagged cells.
const SNAKE: &'static str = "▓";

/// The string printed for concealed cells.
const APPLE: &'static str = "▒";

/// The game over screen.
const WELCOME: &'static str = "╔═══════════════════╗\n\r\
                                 ║──────WELCOME──────║\n\r\
                                 ║ space :  to start ║\n\r\
                                 ║ q     :  quit     ║\n\r\
                                 ╚═══════════════════╝";

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
        self.stdout.flush();
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
                                        &65 => { println!("TOP") }
                                        /*BOTTOM*/
                                        &66 => { println!("BOTTOM") }
                                        /*RIGHT*/
                                        &67 => { println!("RIGHT") }
                                        /*LEFT*/
                                        &68 => { println!("LEFT") }
                                        _ => {}
                                    }
                                }
                                &0 => { break; }
                                _ => {}
                            }
                        }
                        /*W*/
                        &119=>{}
                        /*S*/
                        &115=>{}
                        /*D*/
                        &100=>{}
                        /*A*/
                        &97=>{}
                        _ => {
                            self.stdout.write(&format!("Pressed: {:?}\n", &buf).as_bytes());
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
    let mut stdout = stdout.lock();
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let stderr = io::stderr();
    let mut stderr = stderr.lock();
    let stdout = stdout.into_raw_mode().unwrap();
    let term_size = termion::terminal_size().ok();
    let term_width = term_size.map(|(w, _)| w - 2);
    let term_height = term_size.map(|(_, h)| h - 2);
    let mut game = Chainake {
        board: Board { width: term_width.unwrap(), height: term_height.unwrap() },
        stdin,
        stdout,
    };
    game.play(1);
}
