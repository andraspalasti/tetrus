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
            offset: ((w / 2).try_into().unwrap(), 0),
            curr_piece: Tetromino::new(),
            board: vec![vec![Colors::None; w]; h],
        })
    }

    fn is_occupied(self: &Self, x: i32, y: i32) -> bool {
        let mut result = false;
        self.curr_piece.each_block(|dx, dy| {
            if result {
                return;
            }
            let pos = ((x + dx) as usize, (y + dy) as usize);
            let block = self.board.get(pos.0).and_then(|row| row.get(pos.1));
            match block {
                Some(color) if color != &Colors::None => result = true,
                None => result = true,
                _ => (),
            }
        });
        result
    }
}
