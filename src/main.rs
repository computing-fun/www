use warp::{reply::Reply, Filter};

#[tokio::main]
async fn main() {
    warp::serve(index().or(well_known()))
        .tls()
        .cert_path("tls/cert.pem")
        .key_path("tls/key.rsa")
        .run(([127, 0, 0, 1], 8443))
        .await
}

type RouteFilter = warp::filters::BoxedFilter<(warp::reply::Response,)>;

fn index() -> RouteFilter {
    warp::path::end()
        .map(|| warp::reply::html("[Index Page]").into_response())
        .boxed()
}

fn well_known() -> RouteFilter {
    let root = warp::path(".well-known");

    let security = root
        .and(warp::path("security.txt"))
        .and(warp::path::end())
        .map(|| {
            warp::reply::with_header(
                include_str!("../security.txt"),
                warp::http::header::CONTENT_TYPE,
                "text/plain",
            )
            .into_response()
        });

    security.boxed()
}
