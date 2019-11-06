use std::sync::Mutex;
use actix_web::web::Data;

pub struct AppState {
    pub aaa: String,
}

unsafe impl Send for AppState {}
unsafe impl Sync for AppState {}

pub fn factory() -> Data<Mutex<AppState>>{
    Data::new(Mutex::new(AppState {
        aaa: "AAAAAAAAaaaaaaaaaaaaaaAAAAAAAAAAAAAAAAAAA".to_string(),
    }))
}