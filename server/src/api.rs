use crate::*;
use actix_web::*;
use std::sync::Arc;
use std::{collections::HashMap, sync::Mutex};

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
			return HttpResponse::Conflict().body("ongoing");
		}

		session.white = Some(rand::random::<usize>());
		return HttpResponse::Ok().body(format!(
			"joined.\ntoken:{}\ngame state:{}",
			session.white.unwrap(),
			session.board
		));
	}

	let session = Session::new(rand::random::<usize>());
	state.insert(n.clone(), session);
	let session = state.get(&n).unwrap();
	HttpResponse::Ok().body(format!(
		"created.\ntoken:{}\ngame state:{}",
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
			// if current player token = provided token, make req.move
			return HttpResponse::Ok();
		}
		return HttpResponse::Unauthorized();
	}
	HttpResponse::BadRequest()
}

#[get("")]
async fn api() -> impl Responder {
	HttpResponse::Ok().body("ok")
}

#[get("")]
async fn client() -> impl Responder {
	// provide own ip as argument
	web::Redirect::to("https://tw0ten.github.io/chess?s=127.0.0.1").permanent()
}
