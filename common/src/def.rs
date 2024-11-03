use std::fmt;

pub const BOARD_SIZE: usize = 8;

pub type Player = bool;

pub enum Kind {
    King,
    Queen,
    Tower,
    Elephant,
    Horse,
    Pawn,
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Kind::King => 'k',
                Kind::Queen => 'q',
                Kind::Tower => 't',
                Kind::Elephant => 'e',
                Kind::Horse => 'h',
                Kind::Pawn => 'p',
            }
        )
    }
}

pub struct Piece {
    pub kind: Kind,
    pub player: Player,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let i = self.kind.to_string();
        write!(
            f,
            "{}",
            if self.player {
                i.to_lowercase()
            } else {
                i.to_uppercase()
            }
        )
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} 1 2 3 4 5 6 7 8", if self.player { 'w' } else { 'B' })?;
        for i in 0..BOARD_SIZE {
            writeln!(f)?;
            write!(f, "{}", 8 - i)?;
            for cell in &self.pieces[i] {
                match cell {
                    Some(piece) => write!(f, " {}", piece)?,
                    None => write!(f, "  ")?,
                };
            }
        }
        Ok(())
    }
}

pub struct Board {
    pub pieces: [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE],
    pub player: Player,
}
