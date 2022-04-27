use crate::tetromino::{Colors, Tetromino};

pub struct Game {
    board: Vec<Vec<Colors>>,
    curr_piece: Tetromino,
    offset: (i32, i32),
}

impl Game {
    pub fn new(w: usize, h: usize) -> Result<Game, &'static str> {
        // check for min width
        if w < 10 {
            return Err("The width of the game has to be at least 10 blocks");
        }
        // check for min height
        if h < 20 {
            return Err("The height of the game has to be at least 20 blocks");
        }
        Ok(Game {
            offset: ((w / 2) as i32 - 1, 0),
            curr_piece: Tetromino::new(),
            board: vec![vec![Colors::None; w]; h],
        })
    }

    pub fn board(self: &Self) -> &Vec<Vec<Colors>> {
        &self.board
    }

    pub fn offset(self: &Self) -> &(i32, i32) {
        &self.offset
    }

    pub fn piece(self: &Self) -> &Tetromino {
        &self.curr_piece
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
        let coords = self.curr_piece.clone().into_iter();
        for (dx, dy) in coords {
            self.board[(self.offset.1 + dy) as usize][(self.offset.0 + dx) as usize] =
                self.curr_piece.color();
        }

        // check for full rows
        for i in 0..self.board.len() {
            let is_full = self.board[i].iter().all(|c| *c != Colors::None);
            if !is_full {
                continue;
            }

            // clear the current rows colors
            for c in &mut self.board[i] {
                *c = Colors::None;
            }

            // swap the elements from down to up
            for j in 0..i {
                self.board.swap(i - j, i - j - 1);
            }
        }
    }

    fn is_occupied(self: &Self, x: i32, y: i32) -> bool {
        let coords = self.curr_piece.clone().into_iter();
        for (dx, dy) in coords {
            let offset = ((x + dx) as usize, (y + dy) as usize);
            if self.board.len() <= offset.1
                || self.board[0].len() <= offset.0
                || self.board[offset.1][offset.0] != Colors::None
            {
                return true;
            }
        }
        false
    }
}
