use actix_web::{
    body::BoxBody, post, web, App, HttpResponse, HttpServer, Responder
};
use serde_json;
use actix_web::HttpRequest;
use actix_web::http::header::ContentType;
use serde::Deserialize;
use env_logger;

mod importdataservice;
mod kmeansservice;

#[derive(Deserialize)]
struct ImportDataRequest {
    uri: String,
}

impl Responder for importdataservice::VisData {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

#[post("/import")]
async fn import_data(import_data_req: web::Json<ImportDataRequest>) -> impl Responder {
    let data = importdataservice::import_data(&import_data_req.uri).await;
    data
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
