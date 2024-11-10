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

	pub fn can_move(&self, from: Square, to: Square) -> bool {
		let piece = self.at(&from);
		if let Some(piece) = piece {
			let capture = self.at(&to);
			if let Some(capture) = capture {
				if capture.color == piece.color {
					return false;
				}
			}

			return match piece.kind {
				Kind::King => {
					(diff(from.x, to.x) == 1 || diff(from.y, to.y) == 1) && todo!("not check")
				}
				Kind::Pawn => {
					if capture.is_some() {
						diff(to.x, from.x) == 1
							&& match piece.color {
								Color::White => to.y - from.y == 1,
								Color::Black => from.y - to.y == 1,
							}
					} else {
						from.x == to.x
							&& match piece.color {
								Color::White => {
									to.y - from.y == 1 || (from.y == 1 && to.y == from.y + 2)
								}
								Color::Black => {
									from.y - to.y == 1 || (from.y == 6 && to.y == from.y - 2)
								}
							}
					}
				}
				Kind::Horse => {
					(diff(from.x, to.x) == 1 && diff(from.y, to.y) == 2)
						|| (diff(from.x, to.x) == 2 && diff(from.y, to.y) == 1)
				}
				Kind::Tower => (from.y == to.y) ^ (from.x == to.x),
				Kind::Elephant => diff(from.x, to.x) == diff(from.y, to.y),
				Kind::Queen => {
					(from.y == to.y) ^ (from.x == to.x) || diff(from.x, to.x) == diff(from.y, to.y)
				}
			};
		}
		false
	}

	pub fn legal_moves(&self) -> Vec<Move> {
		let mut v = vec![];
		for x in 0..BOARD_SIZE {
			for y in 0..BOARD_SIZE {
				if let Some(p) = &self.at(&Square { x, y }) {
					if p.color == self.curr_move {
						for x1 in 0..BOARD_SIZE {
							for y1 in 0..BOARD_SIZE {
								if self.can_move(Square { x, y }, Square { x: x1, y: y1 }) {
									v.push(Move {
										from: Square { x, y },
										to: Square { x: x1, y: y1 },
									});
								}
							}
						}
					}
				}
			}
		}
		v
	}
}

impl Board {
	pub const fn default() -> Board {
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
				[const { None }; BOARD_SIZE],
				[const { None }; BOARD_SIZE],
				[const { None }; BOARD_SIZE],
				[const { None }; BOARD_SIZE],
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
