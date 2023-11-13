pub struct Server {}

impl Server {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn serve(self) {
        log::info!("HTTP server is listening 3000");
        let http_server = axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
            .serve(router().into_make_service());

        http_server.await.unwrap();
    }
}

fn router() -> axum::Router {
    axum::Router::new().route("/", axum::routing::get(|| async { "Hello, World!" }))
}
