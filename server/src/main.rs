use common::def::*;

fn main() {
    let board = setup_board();
    println!("{}", board);
}

fn setup_board() -> Board {
    const EMPTY: [Option<Piece>; BOARD_SIZE] = [const { None }; BOARD_SIZE];
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
            EMPTY,
            EMPTY,
            EMPTY,
            EMPTY,
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
