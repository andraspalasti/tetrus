use rand::Rng;

#[derive(Clone, PartialEq, Eq)]
pub enum Colors {
    None,
    Cyan,
    Blue,
    Orange,
    Yellow,
    Green,
    Purple,
    Red,
}

#[derive(Clone)]
pub struct Tetromino {
    color: Colors,
    dir: usize,
    blocks: [u16; 4],
}

impl Tetromino {
    /// Returns a random tetromino from the TETROMINOS constant
    pub fn new() -> Tetromino {
        let idx = rand::thread_rng().gen_range(0..TETROMINOS.len());
        TETROMINOS[idx].clone()
    }

    pub fn blocks(self: &Self) -> u16 {
        self.blocks[self.dir]
    }

    pub fn rotate(self: &mut Self) {
        self.dir = (self.dir + 1) % self.blocks.len();
    }

    pub fn rotate_back(self: &mut Self) {
        if self.dir == self.blocks.len() - 1 {
            self.dir = 0;
        } else {
            self.dir += 1;
        }
    }

    pub fn each_block<F>(self: &Self, mut func: F)
    where
        F: FnMut(i32, i32),
    {
        let blocks = self.blocks();
        let (mut dx, mut dy) = (0, 0);
        for i in 0..16 {
            if blocks & (0x8000 >> i) != 0 {
                func(dx, dy);
            }

            dx += 1;
            if dx == 4 {
                dx = 0;
                dy += 1;
            }
        }
    }
}

const TETROMINOS: [Tetromino; 7] = [
    Tetromino {
        color: Colors::Cyan,
        dir: 0,
        blocks: [0, 0, 0, 0],
    },
    Tetromino {
        color: Colors::Blue,
        dir: 0,
        blocks: [0, 0, 0, 0],
    },
    Tetromino {
        color: Colors::Orange,
        dir: 0,
        blocks: [0, 0, 0, 0],
    },
    Tetromino {
        color: Colors::Yellow,
        dir: 0,
        blocks: [0, 0, 0, 0],
    },
    Tetromino {
        color: Colors::Green,
        dir: 0,
        blocks: [0, 0, 0, 0],
    },
    Tetromino {
        color: Colors::Purple,
        dir: 0,
        blocks: [0, 0, 0, 0],
    },
    Tetromino {
        color: Colors::Red,
        dir: 0,
        blocks: [0, 0, 0, 0],
    },
];
