pub mod def;

use def::*;

fn diff(x: usize, y: usize) -> usize {
	if x < y {
		y - x
	} else {
		x - y
	}
}

impl Board {
	pub fn at(&self, square: &Square) -> &Option<Piece> {
		&self.pieces[square.x][square.y]
	}

	pub fn canmove(&self, from: Square, to: Square) -> bool {
		let piece = self.at(&from);
		if let Some(piece) = piece {
			if let Some(capture) = self.at(&to) {
				if piece.color == capture.color {
					return false;
				}
			}

			// this will be cancer
			return match piece.kind {
				Kind::King => {
					(diff(from.x, to.x) == 1 || diff(from.y, to.y) == 1) && todo!("not check")
				}
				Kind::Pawn => match piece.color {
					Color::White => to.y - from.y == 1 || (from.y == 1 && to.y == from.y + 2),
					Color::Black => false,
				},
				_ => false,
			};
		}
		false
	}

	pub fn legal_moves(&self) -> Vec<Move> {
		vec![]
	}
}

// i want to leave it abstract enough to introduce new pieces.
// maybe even increase board size?
impl Board {
	pub fn default() -> Board {
		const EMPTY: [Option<Piece>; BOARD_SIZE] = [const { None }; BOARD_SIZE];
		Board {
			pieces: [
				[
					Some(Piece {
						kind: Kind::Tower,
						color: Color::Black,
					}),
					Some(Piece {
						kind: Kind::Horse,
						color: Color::Black,
					}),
					Some(Piece {
						kind: Kind::Elephant,
						color: Color::Black,
					}),
					Some(Piece {
						kind: Kind::Queen,
						color: Color::Black,
					}),
					Some(Piece {
						kind: Kind::King,
						color: Color::Black,
					}),
					Some(Piece {
						kind: Kind::Elephant,
						color: Color::Black,
					}),
					Some(Piece {
						kind: Kind::Horse,
						color: Color::Black,
					}),
					Some(Piece {
						kind: Kind::Tower,
						color: Color::Black,
					}),
				],
				[const {
					Some(Piece {
						kind: Kind::Pawn,
						color: Color::Black,
					})
				}; BOARD_SIZE],
				EMPTY,
				EMPTY,
				EMPTY,
				EMPTY,
				[const {
					Some(Piece {
						kind: Kind::Pawn,
						color: Color::White,
					})
				}; BOARD_SIZE],
				[
					Some(Piece {
						kind: Kind::Tower,
						color: Color::White,
					}),
					Some(Piece {
						kind: Kind::Horse,
						color: Color::White,
					}),
					Some(Piece {
						kind: Kind::Elephant,
						color: Color::White,
					}),
					Some(Piece {
						kind: Kind::Queen,
						color: Color::White,
					}),
					Some(Piece {
						kind: Kind::King,
						color: Color::White,
					}),
					Some(Piece {
						kind: Kind::Elephant,
						color: Color::White,
					}),
					Some(Piece {
						kind: Kind::Horse,
						color: Color::White,
					}),
					Some(Piece {
						kind: Kind::Tower,
						color: Color::White,
					}),
				],
			],
			curr_move: Color::White,
			white: Player {
				castle: Some(Castle::Both),
			},
			black: Player {
				castle: Some(Castle::Both),
			},
			idle_moves: 0,
			enpassant: None,
		}
	}
}
