extern crate rand;
use rand::thread_rng;
use rand::Rng;

pub const BOARD_SIZE: usize = 4;

#[derive(Debug, PartialEq)]
pub struct Game {
    pub board: [u8; BOARD_SIZE * BOARD_SIZE],
}

pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Game {
    pub fn new() -> Game {
        let mut game = Game { board: [0; 16] };
        let mut rng = thread_rng();

        // pick two random (but different) positions on the board
        let pos1: usize = rng.gen_range(0, 16);
        let mut pos2: usize = rng.gen_range(0, 15);
        if pos2 >= pos1 {
            pos2 += 1;
        }

        game.board[pos1] = gen_number();
        game.board[pos2] = gen_number();

        game
    }

    pub fn step(&self, direction: &Direction) -> (Game, usize, bool) {
        let (mut next, reward) = self.shift(direction);
        next.add_number();
        let done = next.is_done();
        (next, reward, done)
    }

    fn shift(&self, direction: &Direction) -> (Game, usize) {
        let mut next = Game { board: [0; 16] };
        let mut reward: usize = 0;

        for row in 0..BOARD_SIZE {
            let mut previous_val = 0;
            let mut out_col = 0;
            for col in 0..BOARD_SIZE {
                let val = self.board[translate(row, col, &direction)];
                if val != 0 {
                    if previous_val != 0 && val == previous_val {
                        // merge two numbers
                        next.board[translate(row, out_col - 1, &direction)] += 1;
                        reward += 2 << val;
                        previous_val = 0;
                    } else {
                        next.board[translate(row, out_col, &direction)] = val;
                        previous_val = val;
                        out_col += 1;
                    }
                }
            }
        }

        (next, reward)
    }

    fn add_number(&mut self) {
        let num_possibilities = self.board.iter().filter(|&x| *x == 0).count();
        if num_possibilities > 0 {
            let i = thread_rng().gen_range(0, num_possibilities);
            self.board[find_ith_occurrence(&self.board, i, 0).unwrap()] = gen_number();
        }
    }

    fn is_done(&self) -> bool {
        // this could be sped up by checking for 0s or adjacent numbers directly
        for direction in vec![
            Direction::Left,
            Direction::Up,
            Direction::Right,
            Direction::Down,
        ] {
            if *self != self.shift(&direction).0 {
                return false;
            }
        }
        true
    }
}

fn translate(row: usize, col: usize, direction: &Direction) -> usize {
    match direction {
        Direction::Left => row * BOARD_SIZE + col,
        Direction::Up => col * BOARD_SIZE + row,
        Direction::Right => (BOARD_SIZE - row - 1) * BOARD_SIZE + (BOARD_SIZE - col - 1),
        Direction::Down => (BOARD_SIZE - col - 1) * BOARD_SIZE + (BOARD_SIZE - row - 1),
    }
}

fn gen_number() -> u8 {
    if thread_rng().gen_range(0, 8) == 0 {
        2
    } else {
        1
    }
}

fn find_ith_occurrence(
    board: &[u8; BOARD_SIZE * BOARD_SIZE],
    mut i: usize,
    value: u8,
) -> Option<usize> {
    board.iter().position(|&x| {
        if x == value {
            if i == 0 {
                true
            } else {
                i -= 1;
                false
            }
        } else {
            false
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg_attr(rustfmt, rustfmt_skip)]
    #[test]
    fn test_shift() {
        let board = Game{board: [0, 0, 1, 3,
                                 0, 2, 2, 1,
                                 2, 2, 2, 2,
                                 1, 0, 1, 0]};

        assert_eq!(board.shift(&Direction::Left),
                   (Game{board: [1, 3, 0, 0,
                                 3, 1, 0, 0,
                                 3, 3, 0, 0,
                                 2, 0, 0, 0]}, 8 * 3 + 4));

        assert_eq!(board.shift(&Direction::Up),
                   (Game{board: [2, 3, 1, 3,
                                 1, 0, 3, 1,
                                 0, 0, 1, 2,
                                 0, 0, 0, 0]}, 8 * 2));

        assert_eq!(board.shift(&Direction::Right),
                   (Game{board: [0, 0, 1, 3,
                                 0, 0, 3, 1,
                                 0, 0, 3, 3,
                                 0, 0, 0, 2]}, 8 * 3 + 4));

        assert_eq!(board.shift(&Direction::Down),
                   (Game{board: [0, 0, 0, 0,
                                 0, 0, 1, 3,
                                 2, 0, 3, 1,
                                 1, 3, 1, 2]}, 8 * 2));
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    #[test]
    fn test_is_done() {
        assert!(!Game{board: [0, 0, 0, 0,
                              0, 0, 1, 3,
                              2, 0, 3, 1,
                              1, 3, 1, 2]}.is_done());

        assert!(Game{board: [1, 2, 1, 2,
                             2, 1, 2, 1,
                             1, 2, 1, 2,
                             2, 1, 2, 1]}.is_done());
    }
}
