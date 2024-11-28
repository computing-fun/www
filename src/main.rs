use actix_web::{
    get, http::header::ContentType, web::Path, App, HttpResponse, HttpServer, Responder,
};
use askama::Template;
//use askama_actix::TemplateToResponse;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    println!("Starting server");
    HttpServer::new(|| {
        App::new()
            .service(src_file)
            .service(media_file)
            .service(favicon)
            .service(security_txt)
            .service(index)
            .default_service(actix_web::web::route().to(default_service))
    })
    .bind_rustls_0_23(
        "127.0.0.1:8443",
        rustls::ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(
                rustls_pemfile::certs(&mut std::io::BufReader::new(std::fs::File::open(
                    "tls/cert.pem",
                )?))
                .collect::<Result<Vec<_>, _>>()?,
                rustls_pemfile::pkcs8_private_keys(&mut std::io::BufReader::new(
                    std::fs::File::open("tls/key.pem")?,
                ))
                .collect::<Result<Vec<_>, _>>()?
                .remove(0)
                .into(),
            )
            .unwrap(),
    )?
    .run()
    .await
}

#[derive(Template)]
#[template(path = "404.html")]
struct NotFound {}
fn not_found() -> HttpResponse {
    HttpResponse::NotFound().body(NotFound {}.render().unwrap())
}

async fn default_service() -> impl Responder {
    not_found()
}

#[derive(Template)]
#[template(path = "construction.html")]
struct Construction {}
fn construction() -> impl Responder {
    HttpResponse::ServiceUnavailable().body(Construction {}.render().unwrap())
}

const STATIC: include_dir::Dir = include_dir::include_dir!("$CARGO_MANIFEST_DIR/static");
#[get("/src/{path}")]
async fn src_file(path: Path<String>) -> impl Responder {
    match STATIC.get_file(path.as_str()) {
        Some(file) => HttpResponse::Ok()
            .content_type(mime_guess::from_path(path.as_str()).first_or_octet_stream())
            .body(file.contents()),
        _ => not_found(),
    }
}

const MEDIA: include_dir::Dir = include_dir::include_dir!("$CARGO_MANIFEST_DIR/media");
#[get("/media/{path}")]
async fn media_file(path: Path<String>) -> impl Responder {
    match MEDIA.get_file(path.as_str()) {
        Some(file) => HttpResponse::Ok()
            .content_type(mime_guess::from_path(path.as_str()).first_or_octet_stream())
            .body(file.contents()),
        _ => not_found(),
    }
}

#[get("/favicon.ico")]
async fn favicon() -> impl Responder {
    match MEDIA.get_file("cf-icon.svg") {
        Some(file) => HttpResponse::Ok()
            .content_type("image/svg+xml")
            .body(file.contents()),
        None => not_found(),
    }
}

#[get("/.well-known/security.txt")]
async fn security_txt() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body(include_str!("../security.txt"))
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {}
#[get("/")]
async fn index() -> impl Responder {
    //Index {}.to_response()
    construction()
}
