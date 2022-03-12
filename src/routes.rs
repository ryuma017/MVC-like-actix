use crate::controllers::{create_user, index};
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(index::index))
        .route("/create_user", web::post().to(create_user::create));
}
