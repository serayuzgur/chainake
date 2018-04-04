extern crate termion;

use termion::{clear,style,cursor,async_stdin};
use termion::raw::IntoRawMode;

use std::io::{self, Read, Write};
use std::thread;
use std::time::Duration;


mod board;
mod snake;
mod cell;

use board::Board;
use snake::Snake;

/// The string printed for concealed cells.
const APPLE: &'static str = "@";

struct Chainake {
    board: Board,
    snake: Snake,
}
/// A snake application written with rust
impl Chainake {
    /// Play the game
    fn play( &mut self, difficulty: usize) {
        self.start();
    }

    fn draw(&mut self){
        let stdout = io::stdout();
        let mut stdout = stdout.lock().into_raw_mode().unwrap();
        write!(stdout, "{}{}{}", clear::All, style::Reset, cursor::Goto(1, 1)).unwrap();
        self.board.draw(&mut stdout);
        self.snake.draw(&mut stdout);
        stdout.flush();
    }

    fn start(&mut self) {
  
        self.draw();
        let mut stdin = async_stdin();
        loop {
            let mut buf: [u8; 3] = [0, 0, 0];
            match stdin.read(&mut buf) {
                Ok(_) => {
                    match buf.get(0).unwrap() {
                        //113=q 27 starting of arrow and esc keys
                        &113 | &27 => {
                            match buf.get(1).unwrap() {
                                &91 => {
                                    match buf.get(2).unwrap() {
                                        /*TOP*/
                                        &65 => self.snake.set_direction(snake::Direction::UP),
                                        /*BOTTOM*/
                                        &66 => self.snake.set_direction(snake::Direction::DOWN),
                                        /*RIGHT*/
                                        &67 => self.snake.set_direction(snake::Direction::RIGHT),
                                        /*LEFT*/
                                        &68 => self.snake.set_direction(snake::Direction::LEFT),
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
                        &119 => self.snake.set_direction(snake::Direction::UP),
                        /*S*/
                        &115 => self.snake.set_direction(snake::Direction::DOWN),
                        /*D*/
                        &100 => self.snake.set_direction(snake::Direction::RIGHT),
                        /*A*/
                        &97 => self.snake.set_direction(snake::Direction::LEFT),
                        &32 => {/*space for starting*/},
                        _ => {
                            // Some keys pressed.
                            // self.stdout
                            //     .write(&format!("Pressed: {:?}\n", &buf).as_bytes());
                            // self.stdout.flush();
                        }
                    }
                }
                Err(error) => {
                    println!("{}",error);
                }
            }
            thread::sleep(Duration::from_millis(500));
            &self.snake.move_1();
            // &self.draw();
            let stdout = io::stdout();
            let mut stdout = stdout.lock().into_raw_mode().unwrap();
            self.snake.draw(&mut stdout);
            stdout.flush();
        }

    }
}

fn main() {
    let term_size = termion::terminal_size().ok();
    let term_width = term_size.map(|(w, _)| w - 2);
    let term_height = term_size.map(|(_, h)| h - 2);

    let stdout = io::stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    write!(stdout, "{}", cursor::Hide).unwrap();

    let mut game = Chainake {
        board: Board::new(
            term_width.unwrap(),
            term_height.unwrap(),
        ),
        snake: Snake::new(term_width.unwrap() / 2 ,term_height.unwrap() / 2)
    };
    game.play(1);
}
