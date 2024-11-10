use std::fmt;

pub const BOARD_SIZE: usize = 8;

pub struct Board {
	pub pieces: [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE],
	pub curr_move: Color,
	pub white: Player,
	pub black: Player,
	pub idle_moves: usize,
	pub enpassant: Option<usize>,
}

pub enum Kind {
	King,
	Queen,
	Tower,
	Elephant,
	Horse,
	Pawn,
}

pub struct Piece {
	pub kind: Kind,
	pub color: Color,
}

pub enum Castle {
	Both,
	Queen,
	King,
}

#[derive(PartialEq)]
pub enum Color {
	White,
	Black,
}

pub struct Player {
	pub castle: Option<Castle>,
}

pub struct Square {
	pub x: usize,
	pub y: usize,
}

pub struct Move {
	pub from: Square,
	pub to: Square,
}

impl Square {
	pub const fn new(x: usize, y: usize) -> Self {
		Square {
			x: if x < BOARD_SIZE { x } else { 0 },
			y: if y < BOARD_SIZE { y } else { 0 },
		}
	}
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

impl fmt::Display for Piece {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let i = self.kind.to_string();
		write!(
			f,
			"{}",
			match self.color {
				Color::White => i.to_lowercase(),
				Color::Black => i.to_uppercase(),
			}
		)
	}
}

impl fmt::Display for Board {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for i in 0..BOARD_SIZE {
			for cell in &self.pieces[i] {
				match cell {
					Some(piece) => write!(f, "{}", piece)?,
					None => write!(f, " ")?,
				}
			}
		}
		Ok(())
	}
}
