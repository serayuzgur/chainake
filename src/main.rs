extern crate termion;

use termion::clear;
use termion::raw::IntoRawMode;

use std::io::{self, Read, Write};

mod board;
mod snake;

use board::Board;
use snake::Snake;

/// The string printed for concealed cells.
const APPLE: &'static str = "▒";

struct Chainake<R: Read, W: Write> {
    board: Board,
    snake: Snake,
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

    fn draw(&mut self){
        self.board.draw(&mut self.stdout);
        self.snake.draw(&mut self.stdout);
        self.stdout.flush();
    }

    fn start(&mut self) {
        write!(&mut self.stdout, "{}", clear::All).unwrap();
        self.draw();
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
                                        &65 => self.snake.move_1(snake::Direction::UP),
                                        /*BOTTOM*/
                                        &66 => self.snake.move_1(snake::Direction::DOWN),
                                        /*RIGHT*/
                                        &67 => self.snake.move_1(snake::Direction::RIGHT),
                                        /*LEFT*/
                                        &68 => self.snake.move_1(snake::Direction::LEFT),
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
                        &119 => self.snake.move_1(snake::Direction::UP),
                        /*S*/
                        &115 => self.snake.move_1(snake::Direction::DOWN),
                        /*D*/
                        &100 => self.snake.move_1(snake::Direction::RIGHT),
                        /*A*/
                        &97 => self.snake.move_1(snake::Direction::LEFT),
                        &32 => println!("START"),
                        _ => {
                            self.stdout
                                .write(&format!("Pressed: {:?}\n", &buf).as_bytes());
                            self.stdout.flush();
                        }
                    }
                    self.draw();
                }
                Err(error) => {
                    self.stdout.write(&format!("Error: {}", error).as_bytes());
                    self.stdout.flush();
                }
            }
        }
        self.stdout.write("Exiting".as_bytes());
        write!(&mut self.stdout, "{}", clear::All).unwrap();
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
        snake: Snake {
            body: vec![
                snake::Point {
                    x: (term_width.unwrap() / 2) as i8,
                    y: (term_height.unwrap() / 2) as i8,
                },
            ],
            direction: snake::Direction::RIGHT,
        },
        stdin,
        stdout,
    };
    game.play(1);
}
