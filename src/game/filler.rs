use rand::seq::IteratorRandom;
use std::collections::VecDeque;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct Filler {
    board: [[Cell; COLS]; ROWS],
    pub p1: (Cell, u8),
    pub p2: (Cell, u8),
    pub turn: Turn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Green,
    Blue,
    Yellow,
    Purple,
    Red,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Turn {
    P1,
    P2,
}

impl Turn {
    fn flip(&self) -> Self {
        match self {
            Turn::P1 => Turn::P2,
            Turn::P2 => Turn::P1,
        }
    }
    fn starting_position(&self) -> (usize, usize) {
        match self {
            Turn::P1 => (0, 0),
            Turn::P2 => (ROWS - 1, COLS - 1),
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let b = match &self {
            Cell::Green => "32",
            Cell::Blue => "36",
            Cell::Yellow => "93",
            Cell::Purple => "35",
            Cell::Red => "31",
            Cell::Black => "30",
        };
        write!(f, "\x1b[{b}m██\x1b[0m")
    }
}

impl Display for Filler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "turn {:?}", self.turn)?;
        for row in self.board {
            for cell in row.iter().rev() {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub const ROWS: usize = 7;
pub const COLS: usize = 8;

impl Cell {
    pub const CELLS: [Cell; 6] = [
        Cell::Green,
        Cell::Blue,
        Cell::Yellow,
        Cell::Purple,
        Cell::Red,
        Cell::Black,
    ];

    pub fn from_input(s: char) -> Option<Cell> {
        Some(match s {
            'g' => Cell::Green,
            'b' => Cell::Blue,
            'y' => Cell::Yellow,
            'p' => Cell::Purple,
            'r' => Cell::Red,
            'l' => Cell::Black,
            _ => return None,
        })
    }
}



impl Filler {
    pub fn new() -> Filler {
        let mut r = rand::rng();
        let mut board = [[Cell::Black; COLS]; ROWS];
        for row in 0..ROWS {
            for col in 0..COLS {
                let mut exclude = vec![];
                if row > 0 {
                    exclude.push(board[row - 1][col]);
                }
                if col > 0 {
                    exclude.push(board[row][col - 1]);
                }
                if row == ROWS - 1 && col == COLS - 1 {
                    exclude.push(board[0][0]);
                }
                board[row][col] = Cell::CELLS
                    .into_iter()
                    .filter(|e| !exclude.contains(e))
                    .choose(&mut r)
                    .unwrap();
            }
        }
        Filler::from(board)
    }

    pub fn is_over(&self) -> bool {
        self.p1.1 + self.p2.1 >= (COLS * ROWS) as u8
    }

    pub fn do_move(&self, color: Cell) -> Filler {
        if color == self.p1.0 || color == self.p2.0 {
            panic!("invalid move {} for \n{}", color, self);
        }
        let mut board = self.board.clone();
        let mut moves = VecDeque::new();
        let curr_color = match self.turn {
            Turn::P1 => self.p1.0,
            Turn::P2 => self.p2.0,
        };
        let start = self.turn.starting_position();
        let mut visited = [[false; COLS]; ROWS];
        moves.push_back(start);
        visited[start.0][start.1] = true;
        board[start.0][start.1] = color;
        let mut score = 1;
        while let Some((r, c)) = moves.pop_front() {
            let deltas: [(bool, fn(usize) -> usize, fn(usize) -> usize); 4] = [
                (r > 0,        |r| r - 1, |c| c),
                (r < ROWS - 1, |r| r + 1, |c| c),
                (c > 0,        |r| r, |c| c - 1),
                (c < COLS - 1, |r| r, |c| c + 1),
            ];
            for (valid, dr, dc) in deltas {
                if !valid { continue }
                let r = dr(r);
                let c = dc(c);
                if !visited[r][c] {
                    visited[r][c] = true;
                    if color == board[r][c] {
                        score += 1;
                    } else if curr_color == board[r][c] {
                        board[r][c] = color;
                        score += 1;
                        moves.push_back((r, c));
                    }
                }
            }
        }
        match self.turn {
            Turn::P1 => Filler { board, p1: (color, score), p2: self.p2, turn: self.turn.flip() },
            Turn::P2 => Filler { board, p2: (color, score), p1: self.p1, turn: self.turn.flip() },
        }
    }
    pub fn get_options(&self) -> Vec<Cell> {
        let mut options = Cell::CELLS.to_vec();
        options.retain(|c| ![self.p1.0, self.p2.0].contains(&c));
        options
    }
}

impl From<[[Cell; COLS]; ROWS]> for Filler {
    fn from(board: [[Cell; COLS]; ROWS]) -> Self {
        let p1_start = Turn::P1.starting_position();
        let p2_start = Turn::P2.starting_position();
        Filler {
            board,
            p1: (board[p1_start.0][p1_start.1], 1),
            p2: (board[p2_start.0][p2_start.1], 1),
            turn: Turn::P1,
        }
    }
}
