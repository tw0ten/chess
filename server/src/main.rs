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
	pub fn new(token: usize) -> Self {
		Session {
			white: None,
			black: token,
			board: Board::default(),
			moves: vec![],
		}
	}
}
