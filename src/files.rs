use actix_web::Responder;
use include_dir::{include_dir, Dir};

pub fn service<T>(app: actix_web::App<T>) -> actix_web::App<T>
where
    T: actix_web::dev::ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Error = actix_web::Error,
        InitError = (),
    >,
{
    app.service(favicon)
        .service(src)
        .service(media)
        .service(security_txt)
}

#[actix_web::get("/src/{path}")]
async fn src(path: actix_web::web::Path<String>) -> impl Responder {
    static_responder(&path)
}

#[actix_web::get("/media/{path}")]
async fn media(path: actix_web::web::Path<String>) -> impl Responder {
    media_responder(&path)
}

#[actix_web::get("/favicon.ico")]
async fn favicon() -> impl Responder {
    media_responder("cf-icon.svg")
}

#[actix_web::get("/.well-known/security.txt")]
async fn security_txt() -> impl Responder {
    actix_web::HttpResponse::Ok()
        .content_type(actix_web::http::header::ContentType::plaintext())
        .body(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/security.txt"
        )))
}

const STATIC: Dir = include_dir!("$CARGO_MANIFEST_DIR/static");
fn static_responder(path: &str) -> Option<impl Responder> {
    dir_responder(&STATIC, path)
}

const MEDIA: Dir = include_dir!("$CARGO_MANIFEST_DIR/media");
fn media_responder(path: &str) -> Option<impl Responder> {
    dir_responder(&MEDIA, path)
}

fn dir_responder(dir: &'static Dir, path: &str) -> Option<impl Responder> {
    let file = dir.get_file(path)?;
    Some(
        actix_web::HttpResponse::Ok()
            .content_type(mime_guess::from_path(file.path()).first_or_octet_stream())
            .body(file.contents()),
    )
}
