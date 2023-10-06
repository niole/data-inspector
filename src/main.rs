use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use log::debug;
use serde::Deserialize;
use env_logger;

mod importdataservice;

#[derive(Deserialize)]
struct ImportDataRequest {
    uri: String,
}

#[post("/import")]
async fn import_data(import_data_req: web::Json<ImportDataRequest>) -> impl Responder {
    debug!("START");
    importdataservice::import_data(&import_data_req.uri).await;
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(|| {
      App::new()
        .service(import_data)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
