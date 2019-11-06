use actix_cors::Cors;
use actix_web::{App, HttpServer, http};
use listenfd::ListenFd;

mod app;
mod error;

fn main() {
    let mut listenfd = ListenFd::from_env();
    let data = app::state();
    let mut server = HttpServer::new(move || {
        App::new().wrap(Cors::new()
            .allowed_origin("http://transport.local.net:8888")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600)
        )
            .register_data(data.clone()).configure(app::config)
    });
    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind("0.0.0.0:80").unwrap()
    };

    server.run().unwrap();
}
