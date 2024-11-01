use crate::Player;
use std::fmt;

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
        if self.player {
            write!(f, "{}", self.kind.to_string().to_lowercase())
        } else {
            write!(f, "{}", self.kind.to_string().to_uppercase())
        }
    }
}
