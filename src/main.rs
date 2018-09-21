extern crate rand;
extern crate termion;

mod lib;
use lib::{Direction, Game, BOARD_SIZE};

use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, style};

fn draw(stdout: &mut termion::raw::RawTerminal<std::io::Stdout>, game: &Game, score: usize) {
    write!(
        stdout,
        "{goto}{clear}{bold}score: {score}{reset}\r\n",
        goto = termion::cursor::Goto(1, 1),
        clear = clear::CurrentLine,
        bold = style::Bold,
        score = score,
        reset = style::Reset
    ).unwrap();
    for row in 0..BOARD_SIZE {
        write!(stdout, "\r\n-----------------\r\n|").unwrap();
        for col in 0..BOARD_SIZE {
            let val = game.board[BOARD_SIZE * row + col];
            write!(stdout, " {} |", val).unwrap();
        }
    }
    write!(stdout, "\r\n-----------------\r\n").unwrap();
    stdout.flush().unwrap();
}

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut game = Game::new();
    let mut score: usize = 0;

    write!(stdout, "{}", termion::clear::All);
    draw(&mut stdout, &game, score);

    for c in stdin.keys() {
        let direction = match c.unwrap() {
            Key::Char('q') | Key::Ctrl('c') => {
                write!(stdout, "\r\nQuit\r\n").unwrap();
                break;
            }
            Key::Char('h') | Key::Left => Some(Direction::Left),
            Key::Char('k') | Key::Up => Some(Direction::Up),
            Key::Char('l') | Key::Right => Some(Direction::Right),
            Key::Char('j') | Key::Down => Some(Direction::Down),
            _ => None,
        };

        if let Some(direction) = direction {
            let (next, reward, done) = game.step(&direction);
            game = next;
            score += reward;

            draw(&mut stdout, &game, score);

            if done {
                write!(stdout, "\r\nGame over!\r\n").unwrap();
                stdout.flush().unwrap();
                break;
            }
        }
    }
}
