use actix_web::{get, http::StatusCode, Responder};
use askama::Template;
use askama_actix::TemplateToResponse;

pub fn service<T>(app: actix_web::App<T>) -> actix_web::App<T>
where
    T: actix_web::dev::ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Error = actix_web::Error,
        InitError = (),
    >,
{
    app.service(index)
}

#[derive(Template)]
#[template(path = "404.html")]
struct NotFound {}
pub async fn not_found() -> impl Responder {
    NotFound {}.customize().with_status(StatusCode::NOT_FOUND)
}

#[allow(dead_code)]
#[derive(Template)]
#[template(path = "503.html")]
struct ServiceUnavailable {}
#[allow(dead_code)]
pub async fn service_unavailable() -> impl Responder {
    ServiceUnavailable {}
        .customize()
        .with_status(StatusCode::SERVICE_UNAVAILABLE)
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index {}
#[get("/")]
async fn index() -> impl Responder {
    Index {}.to_response()
}
