use actix_web::{
    post, get, body::BoxBody, web, App, HttpResponse, HttpServer, Responder
};
use serde_json;
use actix_web::HttpRequest;
use actix_web::http::header::ContentType;
use actix_cors::Cors;
use actix_web::http::header;
use serde::Deserialize;
use env_logger;

mod importdataservice;
mod kmeansservice;

#[derive(Deserialize)]
struct ImportUriRequest {
    uri: String,
    k: usize,
}

#[derive(Deserialize)]
struct ImportDataRequest {
    data: Vec<String>,
    k: usize,
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

#[get("/import")]
async fn import_data(import_data_req: web::Query<ImportUriRequest>) -> impl Responder {
    let data = importdataservice::import_data(&import_data_req.uri, import_data_req.k).await.expect("Should be json");
    data
}

#[post("/import")]
async fn import_data_post(import_data_req: web::Json<ImportDataRequest>) -> impl Responder {
    let data = importdataservice::import_data_snippets(&import_data_req.data, import_data_req.k).await.expect("Should be json");
    data
}

#[get("/render/uri")]
async fn render_uri(import_data_req: web::Query<ImportUriRequest>) -> impl Responder {
    let html = importdataservice::render_uri(&import_data_req.uri, import_data_req.k).await.expect("Should be html");
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html)
}

#[post("/render/data")]
async fn render_data(import_data_req: web::Json<ImportDataRequest>) -> impl Responder {
    let html = importdataservice::render_data(&import_data_req.data, import_data_req.k).await.expect("Should be html");
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init();

    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);
      App::new()
        .wrap(cors)
        .service(import_data)
        .service(import_data_post)
        .service(render_data)
        .service(render_uri)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
