use std::sync::Arc;

use actix_web::{
    App, HttpServer,
    web::{self, ServiceConfig},
};
use clap::Parser;
use fxbilibili::{
    models::config::ProgramArgs,
    routes,
    services::bilibili::{BilibiliService, BilibiliServiceTrait},
};

fn init(config: &mut ServiceConfig) {
    let bili_service: web::Data<Arc<dyn BilibiliServiceTrait>> =
        web::Data::new(Arc::new(BilibiliService::new()));
    config.app_data(bili_service);
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    env_logger::init();
    let args = ProgramArgs::parse();

    let addrs = (args.http_host, args.http_port);

    HttpServer::new(|| App::new().configure(init).configure(routes::configure))
        .bind(addrs)?
        .run()
        .await?;
    Ok(())
}
