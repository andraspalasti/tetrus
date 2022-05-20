use rand::Rng;

use crossterm::style;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Color {
    None,
    Cyan,
    Blue,
    Orange,
    Yellow,
    Green,
    Purple,
    Red,
}

impl Color {
    /// Returns the associated crossterm color
    pub fn ctcolor(self: &Self) -> style::Color {
        match self {
            &Self::Cyan => style::Color::Cyan,
            &Self::Blue => style::Color::Blue,
            &Self::Orange => style::Color::DarkYellow,
            &Self::Yellow => style::Color::Yellow,
            &Self::Green => style::Color::Green,
            &Self::Purple => style::Color::Magenta,
            &Self::Red => style::Color::Red,
            _ => style::Color::Reset,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Tetromino {
    color: Color,
    dir: usize,
    blocks: [u16; 4],
}

impl Tetromino {
    /// Returns a random tetromino from the TETROMINOS constant
    pub fn new() -> Tetromino {
        let idx = rand::thread_rng().gen_range(0..TETROMINOS.len());
        TETROMINOS[idx].clone()
    }

    pub fn color(self: &Self) -> Color {
        self.color.clone()
    }

    pub fn blocks(self: &Self) -> u16 {
        self.blocks[self.dir]
    }

    pub fn rotate(self: &mut Self) {
        self.dir = (self.dir + 1) % self.blocks.len();
    }

    pub fn rotate_back(self: &mut Self) {
        if self.dir == 0 {
            self.dir = self.blocks.len() - 1;
        } else {
            self.dir -= 1;
        }
    }
}

impl IntoIterator for Tetromino {
    type Item = (i32, i32);
    type IntoIter = TetrominoIter;

    fn into_iter(self: Self) -> Self::IntoIter {
        return TetrominoIter {
            blocks: self.blocks(),
            idx: 0,
        };
    }
}

pub struct TetrominoIter {
    blocks: u16,
    idx: i32,
}

impl Iterator for TetrominoIter {
    type Item = (i32, i32);

    fn next(self: &mut Self) -> Option<Self::Item> {
        for i in self.idx..16 {
            if (self.blocks & (0x8000 >> i)) != 0 {
                self.idx = i + 1;
                return Some((i % 4, i / 4));
            }
        }
        None
    }
}

const TETROMINOS: [Tetromino; 7] = [
    Tetromino {
        color: Color::Cyan,
        dir: 0,
        blocks: [0x0F00, 0x2222, 0x00F0, 0x4444],
    },
    Tetromino {
        color: Color::Blue,
        dir: 0,
        blocks: [0x44C0, 0x8E00, 0x6440, 0x0E20],
    },
    Tetromino {
        color: Color::Orange,
        dir: 0,
        blocks: [0x4460, 0x0E80, 0xC440, 0x2E00],
    },
    Tetromino {
        color: Color::Yellow,
        dir: 0,
        blocks: [0xCC00, 0xCC00, 0xCC00, 0xCC00],
    },
    Tetromino {
        color: Color::Green,
        dir: 0,
        blocks: [0x06C0, 0x8C40, 0x6C00, 0x4620],
    },
    Tetromino {
        color: Color::Purple,
        dir: 0,
        blocks: [0x0E40, 0x4C40, 0x4E00, 0x4640],
    },
    Tetromino {
        color: Color::Red,
        dir: 0,
        blocks: [0x0C60, 0x4C80, 0xC600, 0x2640],
    },
];
