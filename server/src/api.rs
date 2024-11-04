use crate::*;
use rand::random;
use std::{
	collections::HashMap,
	sync::{Arc, Mutex},
};

pub const ADDRESS: (&str, u16) = ("127.0.0.1", 8080);

pub struct AppState {
	sessions: Mutex<HashMap<String, Session>>,
}

pub fn app_state() -> Arc<AppState> {
	Arc::new(AppState {
		sessions: Mutex::new(HashMap::new()),
	})
}

#[get("!/{name}")]
async fn name(path: web::Path<String>, state: web::Data<AppState>) -> impl Responder {
	let mut state = state.sessions.lock().unwrap();
	let n = path.into_inner();

	if let Some(session) = state.get_mut(&n) {
		if session.white.is_some() {
			return HttpResponse::Conflict().finish();
		}

		session.white = Some(random());
		return HttpResponse::Ok().body(format!(
			"token:{}\ngame state:\n{}",
			session.white.unwrap(),
			session.board
		));
	}

	let session = Session::new(random());
	state.insert(n.clone(), session);
	let session = state.get(&n).unwrap();
	HttpResponse::Ok().body(format!(
		"token:{}\ngame state:\n{}",
		session.black, session.board
	))
}

#[post("!/{name}")]
async fn send(
	path: web::Path<String>,
	req: HttpRequest,
	state: web::Data<AppState>,
) -> impl Responder {
	let token = req
		.headers()
		.get(http::header::AUTHORIZATION)
		.and_then(|value| value.to_str().ok())
		.and_then(|value| value.parse::<usize>().ok());

	if let Some(token) = token {
		let mut state = state.sessions.lock().unwrap();
		if let Some(session) = state.get_mut(&path.into_inner()) {
			if match session.board.curr_move {
				Color::White => session.white.unwrap(),
				Color::Black => session.black,
			} != token
			{
				return HttpResponse::Unauthorized();
			}
			// process move
		}
	}

	HttpResponse::BadRequest()
}

#[get("")]
async fn api() -> impl Responder {
	HttpResponse::Ok().body("ok")
}

#[get("/")]
async fn client() -> impl Responder {
	let (ip, port) = ADDRESS;
	web::Redirect::to(format!("https://tw0ten.github.io/chess?s={}:{}", ip, port)).permanent()
}
