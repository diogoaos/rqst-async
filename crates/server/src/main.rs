use miniserve::{http::StatusCode, Content, Request, Response};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Messages {
    messages: Vec<String>,
}

fn index(_req: Request) -> Response {
    let content = include_str!("../index.html").to_string();
    Ok(Content::Html(content))
}

fn chat(_req: Request) -> Response {
    match _req {
        Request::Get => Err(StatusCode::METHOD_NOT_ALLOWED),
        Request::Post(body) => {
            let content = body;
            let mut request_dto: Messages = serde_json::from_str(&content).unwrap();
            let mut last_word = request_dto.messages.last().unwrap().clone();

            last_word.push_str(" -> well said");
            request_dto.messages.push(last_word);
            Ok(Content::Json(serde_json::to_string(&request_dto).unwrap()))
        }
    }
}

fn main() {
    miniserve::Server::new()
        .route("/", index)
        .route("/chat", chat)
        .run()
}
