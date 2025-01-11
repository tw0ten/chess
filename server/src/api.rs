use crate::*;

use rand::random;
use std::{
	collections::HashMap,
	sync::{Arc, Mutex},
};

pub const ADDRESS: (&str, u16) = ("127.0.0.1", 8080);

pub fn app_state() -> Arc<AppState> {
	Arc::new(AppState {
		sessions: Mutex::new(HashMap::new()),
	})
}

pub struct AppState {
	sessions: Mutex<HashMap<String, Session>>,
}

#[get("{name}")]
async fn name(path: web::Path<String>, state: web::Data<AppState>) -> impl Responder {
	if let Some(mut state) = state.sessions.lock().ok() {
		let n = path.into_inner();

		if let Some(session) = state.get_mut(&n) {
			if !session.expired() {
				if session.white.is_some() {
					return HttpResponse::Conflict().finish();
				}

				session.white = Some(random());
				session.expire(Duration::from_secs(90));
				return HttpResponse::Ok().body(format!(
					"{}\n{}",
					session.white.unwrap(),
					session.board
				));
			}
		}

		let mut session = Session::new(random());
		session.expire(Duration::from_secs(90));
		state.insert(n.clone(), session);
		let session = state.get(&n).unwrap();
		HttpResponse::Ok().body(format!("{}\n{}", session.black, session.board))
	} else {
		HttpResponse::Locked().finish()
	}
}

#[post("{name}")]
async fn send(
	path: web::Path<String>,
	req: HttpRequest,
	state: web::Data<AppState>,
) -> impl Responder {
	let token = req
		.headers()
		.get(http::header::AUTHORIZATION)
		.and_then(|v| v.to_str().ok())
		.and_then(|v| v.parse::<usize>().ok());

	if let Some(token) = token {
		if let Some(mut state) = state.sessions.lock().ok() {
			let n = path.into_inner();
			if let Some(session) = state.get_mut(&n) {
				if session.expired() {
					state.remove(&n);
					return HttpResponse::Gone();
				}
				if session.white.is_some() {
					if match session.board.player {
						false => session.black,
						true => session.white.unwrap(),
					} != token
					{
						return HttpResponse::Unauthorized();
					}
					//if session.board.move() {
					session.expire(Duration::from_secs(90));
					return HttpResponse::Ok();
				}
			}
		}
	}

	HttpResponse::BadRequest()
}

#[get("/")]
async fn api() -> impl Responder {
	HttpResponse::Ok().finish()
}

#[get("/")]
async fn client() -> impl Responder {
	let (ip, port) = ADDRESS;
	web::Redirect::to(format!("https://tw0ten.github.io/chess?s={}:{}", ip, port)).permanent()
}
