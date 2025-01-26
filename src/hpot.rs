use actix_web::{get, post, HttpRequest, HttpResponse, Responder};
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
    app.service(wp).service(wp_post).service(wp_wild)
}

#[derive(Template)]
#[template(path = "hpot/wp.html")]
struct WP {
    error: String,
}
#[get("/wp")]
async fn wp() -> impl Responder {
    WP {
        error: String::new(),
    }
    .to_response()
}

#[post("/wp")]
async fn wp_post(req: HttpRequest, payload: actix_web::web::Payload) -> impl Responder {
    println!(
        "{:?}\n{}",
        req,
        payload.to_bytes().await.unwrap_or_default().escape_ascii()
    );
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    WP {
        error: String::from("Username and Password don't match"),
    }
    .to_response()
}

#[get("/wp-{param:.*}")]
async fn wp_wild() -> impl Responder {
    HttpResponse::Found()
        .insert_header(("Location", "https://www.computingfun.org/wp"))
        .finish()
}
