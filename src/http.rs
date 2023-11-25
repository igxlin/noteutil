use axum::routing::get;

#[derive(Clone)]
pub struct ServerState {
    pub config: crate::Config,
}

pub async fn serve(cx: crate::Context) {
    log::info!("HTTP server is listening 10428");
    let http_server =
        axum::Server::bind(&"0.0.0.0:10428".parse().unwrap()).serve(router(cx).into_make_service());

    http_server.await.unwrap();
}

fn router(cx: crate::Context) -> axum::Router {
    axum::Router::new()
        // TODO The members of ServerState are be cloned every time. Refactor the members to make
        // them as pointers.
        .route("/pages/*path", get(crate::html::serve_page))
        .with_state(ServerState { config: cx.config })
}
