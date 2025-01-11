mod api;

use actix_cors::Cors;
use actix_web::*;
use api::*;
use common::def::*;
use std::time::{Duration, Instant};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	let state = app_state();
	HttpServer::new(move || {
		App::new()
			.wrap(
				Cors::default()
					.allow_any_origin()
					.allow_any_method()
					.allow_any_header(),
			)
			.app_data(web::Data::from(state.clone()))
			.service(client)
			.service(
				web::scope("api")
					.service(api)
					.service(web::scope("*").service(name).service(send)),
			)
	})
	.bind(ADDRESS)?
	.run()
	.await
}

struct Session {
	white: Option<usize>,
	black: usize,
	board: Board,
	moves: Vec<usize>, // move to board?
	expire: Instant,
}

impl Session {
	pub fn new(token: usize) -> Self {
		Session {
			white: None,
			black: token,
			board: Board::default(),
			moves: vec![],
			expire: Instant::now(),
		}
	}

	pub fn expire(&mut self, i: Duration) {
		self.expire = Instant::now() + i
	}

	pub fn expired(&self) -> bool {
		Instant::now() > self.expire
	}
}
