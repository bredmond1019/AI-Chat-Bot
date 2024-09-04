use crate::services::chat_server::ChatServer;
use crate::services::chat_session::ChatSession;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

pub async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<actix::Addr<ChatServer>>,
) -> Result<HttpResponse, Error> {
    ws::start(ChatSession::new(srv.get_ref().clone()), &req, stream)
}
