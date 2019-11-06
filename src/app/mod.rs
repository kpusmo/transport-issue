use actix_web::web;

mod solution;

pub fn config(cfg: &mut web::ServiceConfig) {
    solution::config(cfg);
}