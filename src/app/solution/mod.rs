mod calculate;

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        actix_web::web::scope("/solutions")
            .route("", actix_web::web::post().to(calculate::index))
    );
}
