mod api;
use actix_web::*;
use api::*;
use common::def::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	let state = app_state();
	HttpServer::new(move || {
		App::new()
			.app_data(web::Data::from(state.clone()))
			.service(client)
			.service(web::scope("api").service(api).service(name).service(send))
	})
	.bind(ADDRESS)?
	.run()
	.await
}

struct Session {
	white: Option<usize>,
	black: usize,
	board: Board,
	moves: Vec<usize>,
}

impl Session {
	fn board() -> Board {
		const EMPTY: [Option<Piece>; BOARD_SIZE] = [const { None }; BOARD_SIZE];
		fn line(player: bool) -> [Option<Piece>; BOARD_SIZE] {
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
			idle_moves: 0,
			enpassant: None,
		}
	}

	pub fn new(token: usize) -> Self {
		Session {
			white: None,
			black: token,
			board: Self::board(),
			moves: vec![],
		}
	}
}
