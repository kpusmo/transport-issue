use actix_web::web;

pub use state::AppState;
pub use state::factory as state;

mod state;
mod solution;

pub fn config(cfg: &mut web::ServiceConfig) {
    solution::config(cfg);
}