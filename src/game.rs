use crate::tetromino::{Color, Tetromino};

const WIDTH: usize = 10;
const HEIGHT: usize = 20;

pub struct Game {
    board: Vec<Vec<Color>>,
    curr_piece: Tetromino,
    offset: (i32, i32),
}

impl Game {
    pub fn new() -> Self {
        Game {
            offset: ((WIDTH / 2) as i32 - 1, 0),
            curr_piece: Tetromino::new(),
            board: vec![vec![Color::None; WIDTH]; HEIGHT],
        }
    }

    pub fn board(self: &Self) -> &Vec<Vec<Color>> {
        &self.board
    }

    pub fn offset(self: &Self) -> &(i32, i32) {
        &self.offset
    }

    pub fn piece(self: &Self) -> &Tetromino {
        &self.curr_piece
    }

    pub fn height(self: &Self) -> usize {
        self.board.len()
    }

    pub fn width(self: &Self) -> usize {
        if self.height() < 1 {
            return 0;
        }
        self.board[0].len()
    }

    pub fn move_left(self: &mut Self) {
        if !self.is_occupied(self.offset.0 - 1, self.offset.1) {
            self.offset.0 -= 1;
        }
    }

    pub fn move_right(self: &mut Self) {
        if !self.is_occupied(self.offset.0 + 1, self.offset.1) {
            self.offset.0 += 1;
        }
    }

    pub fn move_down(self: &mut Self) {
        if !self.is_occupied(self.offset.0, self.offset.1 + 1) {
            self.offset.1 += 1;
        }
    }

    pub fn rotate_piece(self: &mut Self) {
        self.curr_piece.rotate();
        if self.is_occupied(self.offset.0, self.offset.1) {
            self.curr_piece.rotate_back();
        }
    }

    pub fn tick(self: &mut Self) {
        // check if we can move the piece down
        if !self.is_occupied(self.offset.0, self.offset.1 + 1) {
            self.offset.1 += 1;
            return;
        }

        // move the colors from the piece to the board
        for (dx, dy) in self.curr_piece.clone() {
            self.board[(self.offset.1 + dy) as usize][(self.offset.0 + dx) as usize] =
                self.curr_piece.color();
        }
        self.curr_piece = Tetromino::new();
        self.offset = ((self.board[0].len() / 2) as i32 - 1, 0);

        // check for full rows
        for i in 0..self.board.len() {
            let is_full = self.board[i].iter().all(|c| *c != Color::None);
            if !is_full {
                continue;
            }

            // clear the current rows colors
            for c in &mut self.board[i] {
                *c = Color::None;
            }

            // swap the elements from down to up
            for j in 0..i {
                self.board.swap(i - j, i - j - 1);
            }
        }
    }

    fn is_occupied(self: &Self, x: i32, y: i32) -> bool {
        for (dx, dy) in self.curr_piece.clone() {
            let (new_x, new_y) = ((x + dx) as usize, (y + dy) as usize);
            if self.height() <= new_y
                || self.width() <= new_x
                || self.board[new_y][new_x] != Color::None
            {
                return true;
            }
        }
        false
    }
}
