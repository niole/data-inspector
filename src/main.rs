use actix_web::{
    /*body::BoxBody,*/ post, web, App, HttpResponse, HttpServer, Responder
};
use serde::{Deserialize, /*Serialize*/};
use env_logger;

mod importdataservice;
mod kmeansservice;

#[derive(Deserialize)]
struct ImportDataRequest {
    uri: String,
}

//#[derive(Serialize)]
//struct VisData {
//    centroids: Vec<(f32, f32)>,
//    data: Vec<DataPoint>
//}
//
//#[derive(Serialize)]
//struct DataPoint {
//    centroid_index: usize,
//    x: f32,
//    y: f32,
//    content: String
//}
//
//impl Responder for VisData {
//    type Body = BoxBody;
//
//    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
//        let body = serde_json::to_string(&self).unwrap();
//
//        HttpResponse::Ok()
//            .content_type(ContentType::json())
//            .body(body)
//    }
//}

#[post("/import")]
async fn import_data(import_data_req: web::Json<ImportDataRequest>) -> impl Responder {
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
