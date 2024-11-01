use std::fmt;

const BOARD_SIZE: usize = 8;

pub type Player = bool;

mod piece;
use piece::*;

struct Board {
    pieces: [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE],
    player: Player,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        _ = write!(f, "{} 1 2 3 4 5 6 7 8", if self.player { 'w' } else { 'B' });
        for i in 0..BOARD_SIZE {
            _ = writeln!(f);
            _ = write!(f, "{}", 8 - i);
            for cell in &self.pieces[i] {
                _ = match cell {
                    Some(piece) => write!(f, " {}", piece),
                    None => write!(f, "  "),
                };
            }
        }
        Ok(())
    }
}

impl Board {
    /*
        fn won(&self) -> Option<Option<Player>> {
            return Some(Some(true));
            return Some(None);
            None
        }

        fn mv() {}
    */

    fn new() -> Self {
        fn line(player: Player) -> [Option<Piece>; BOARD_SIZE] {
            let piece = |kind: Kind| Some(Piece { kind, player });
            [
                piece(Kind::Tower),
                piece(Kind::Horse),
                piece(Kind::Elephant),
                piece(Kind::Queen),
                piece(Kind::King),
                piece(Kind::Elephant),
                piece(Kind::Horse),
                piece(Kind::Tower),
            ]
        }

        Board {
            pieces: [
                line(false),
                [const {
                    Some(Piece {
                        kind: Kind::Pawn,
                        player: false,
                    })
                }; BOARD_SIZE],
                [const { None }; BOARD_SIZE],
                [const { None }; BOARD_SIZE],
                [const { None }; BOARD_SIZE],
                [const { None }; BOARD_SIZE],
                [const {
                    Some(Piece {
                        kind: Kind::Pawn,
                        player: true,
                    })
                }; BOARD_SIZE],
                line(true),
            ],
            player: true,
        }
    }
}

pub fn main() {
    let board = Board::new();
    println!("{}", board);
}
