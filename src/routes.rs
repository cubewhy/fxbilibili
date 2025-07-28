use actix_web::web::ServiceConfig;

pub mod index;
pub mod video;

pub fn configure(config: &mut ServiceConfig) {
    config.service(video::scope()).service(index::scope());
}
