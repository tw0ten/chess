pub mod def;

use def::*;

const fn d(x: usize, y: usize) -> usize {
	x.abs_diff(y)
}

impl Board {
	// ret board state
	// is valid board? or something. if in check invalid, cuz our move
	pub fn can_move(&self, from: Square, to: Square) -> bool {
		if from == to {
			return false;
		}
		let piece = self.at(&from);
		if let Some(piece) = piece {
			if piece.color != self.player {
				// moving opponent piece
				return false;
			}
			let capture = self.at(&to);
			if let Some(capture) = capture {
				if capture.color == piece.color {
					// capturing own piece
					return false;
				}
			}

			let check = false; // todo

			return match piece.kind {
				Kind::King => (d(from.x, to.x) == 1 || d(from.y, to.y) == 1) && false, // is target in check?
				Kind::Pawn => {
					// implement promotion somehow
					if capture.is_some() {
						d(to.x, from.x) == 1
							&& match piece.color {
								true => to.y - from.y == 1,
								false => from.y - to.y == 1,
							}
					} else {
						from.x == to.x
							&& match piece.color {
								true => to.y - from.y == 1 || (from.y == 1 && to.y == from.y + 2),
								false => from.y - to.y == 1 || (from.y == 6 && to.y == from.y - 2),
							}
					}
				}
				Kind::Horse => match (d(from.x, to.x), d(from.y, to.y)) {
					(2, 1) | (1, 2) => true,
					_ => false,
				},
				Kind::Tower => (from.y == to.y) ^ (from.x == to.x),
				Kind::Elephant => d(from.x, to.x) == d(from.y, to.y),
				Kind::Queen => {
					(from.y == to.y) ^ (from.x == to.x) || d(from.x, to.x) == d(from.y, to.y)
					// maybe move piece matching into a function to do Elephant || Tower
				}
			};
		}

		// moving empty square
		false
	}

	fn pass(&self, f: Square, t: Square, v: (isize, isize)) -> bool {
		//move in line
		todo!()
	}

	pub fn legal_moves(&self) -> Vec<Move> {
		let mut v = vec![];
		for x in 0..BOARD_SIZE {
			for y in 0..BOARD_SIZE {
				if let Some(p) = &self.at(&Square { x, y }) {
					if p.color == self.player {
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

	pub fn make_move(&mut self, mv: Move) {
		// case move (promotion, castling, enpassant)
		self.pieces[mv.to.x][mv.to.y] = self.pieces[mv.from.x][mv.from.y].clone();
		self.pieces[mv.from.x][mv.from.y] = None;
	}

	pub const fn at(&self, square: &Square) -> &Option<Piece> {
		&self.pieces[square.x][square.y]
	}
}

impl Board {
	pub const fn default() -> Board {
		Board {
			pieces: [
				[
					Some(Piece {
						kind: Kind::Tower,
						color: false,
					}),
					Some(Piece {
						kind: Kind::Horse,
						color: false,
					}),
					Some(Piece {
						kind: Kind::Elephant,
						color: false,
					}),
					Some(Piece {
						kind: Kind::Queen,
						color: false,
					}),
					Some(Piece {
						kind: Kind::King,
						color: false,
					}),
					Some(Piece {
						kind: Kind::Elephant,
						color: false,
					}),
					Some(Piece {
						kind: Kind::Horse,
						color: false,
					}),
					Some(Piece {
						kind: Kind::Tower,
						color: false,
					}),
				],
				[const {
					Some(Piece {
						kind: Kind::Pawn,
						color: false,
					})
				}; BOARD_SIZE],
				[const { None }; BOARD_SIZE],
				[const { None }; BOARD_SIZE],
				[const { None }; BOARD_SIZE],
				[const { None }; BOARD_SIZE],
				[const {
					Some(Piece {
						kind: Kind::Pawn,
						color: true,
					})
				}; BOARD_SIZE],
				[
					Some(Piece {
						kind: Kind::Tower,
						color: true,
					}),
					Some(Piece {
						kind: Kind::Horse,
						color: true,
					}),
					Some(Piece {
						kind: Kind::Elephant,
						color: true,
					}),
					Some(Piece {
						kind: Kind::Queen,
						color: true,
					}),
					Some(Piece {
						kind: Kind::King,
						color: true,
					}),
					Some(Piece {
						kind: Kind::Elephant,
						color: true,
					}),
					Some(Piece {
						kind: Kind::Horse,
						color: true,
					}),
					Some(Piece {
						kind: Kind::Tower,
						color: true,
					}),
				],
			],
			player: true,
		}
	}
}
