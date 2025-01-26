mod files;
mod hpot;
mod pages;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    println!("Starting server");
    actix_web::HttpServer::new(|| {
        let mut app = actix_web::App::new()
            .default_service(actix_web::web::to(|| pages::not_found()))
            .wrap(actix_web::middleware::NormalizePath::new(
                actix_web::middleware::TrailingSlash::Trim,
            ));
        app = files::service(app);
        app = pages::service(app);
        app = hpot::service(app);
        app
    })
    .bind_rustls_0_23(
        "0.0.0.0:443",
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
