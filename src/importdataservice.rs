use std::{str, cmp};
use std::error::Error;
use actix_web::rt::task::spawn_blocking;
use log::debug;
use reqwest;
use rust_bert::pipelines::sentence_embeddings::{SentenceEmbeddingsBuilder, SentenceEmbeddingsModelType};
use smartcore::decomposition::svd::SVD;
use smartcore::linalg::basic::matrix::DenseMatrix;
use smartcore::linalg::basic::arrays::Array2;
use smartcore::decomposition::svd::SVDParameters;
use serde::Serialize;
use plotly::{ common::Mode, Scatter, Plot};

use crate::kmeansservice;

#[derive(Serialize)]
pub struct VisData {
    centroids: Vec<(f32, f32)>,
    data: Vec<DataPoint>
}

#[derive(Serialize)]
pub struct DataPoint {
    centroid_index: usize,
    point: (f32, f32),
    content: String
}


/// downloads data over http from the specified uri
/// no auth
/// chunks, encodes, and stores in vector db
///
/// # Arguments
/// * `uri` - the location of the data
pub async fn import_data(uri: &String) -> Result<VisData, Box<dyn Error>> {
    return download_data(uri).await; // TODO poll?
}

pub async fn render_data(uri: &String) -> Result<String, Box<dyn Error>>{
    let data = download_data(uri).await.expect("asdf");

    let trace = Scatter::new(
        data.data.iter().map(|d| d.point.0).collect(),
        data.data.iter().map(|d| d.point.1).collect(),
    ).mode(Mode::Markers);
    let mut plot = Plot::new();
    plot.add_trace(trace);
    return Ok(plot.to_html());
}

async fn download_data(uri: &String) -> Result<VisData, Box<dyn Error>> {
    let body = reqwest::get(uri).await.expect("body").text().await.unwrap();

    // chunk the text
    let chunks = chunk_text(&body, 4000);

    // encode the text
    let encoded_cs = encode_chunks(&chunks).await?;

    let service = kmeansservice::init(&encoded_cs);

    let memberships = service.memberships;

    let encoded_cs_matrix = DenseMatrix::from_2d_vec(&encoded_cs);
    let svd = SVD::fit(&encoded_cs_matrix, SVDParameters::default().with_n_components(2)).unwrap();

    let reduced_encodings = svd.transform(&encoded_cs_matrix).unwrap();

    let mut centroids_vec = vec!(); // TODO sad
    for row in service.model.centroids().rows() {
        centroids_vec.push(row.to_vec());
    }

    let centroids_matrix = DenseMatrix::from_2d_vec(&centroids_vec);
    let reduced_centroids = svd.transform(&centroids_matrix).unwrap();

    let data = reduced_encodings.row_iter().enumerate().map(|(index, point)| {
        DataPoint {
            centroid_index: memberships[index],
            point: (*point.get(0), *point.get(1)),
            content: str::from_utf8(chunks[index]).unwrap().to_string()
        }
    }).collect();

    return Ok(VisData {
        centroids: reduced_centroids.row_iter().map(|r| { (*r.get(0), *r.get(1)) }).collect(),
        data: data
    });
}

async fn encode_chunks(chunks: &Vec<&[u8]>) -> Result<Vec<Vec<f32>>, Box<dyn Error>> {
    // sentenceembeddingsbuilder::remote is blocking and we are in an async runtime
    // so must explicitly spawn a thread for the blocking computation or panic
    let model = spawn_blocking( move || { SentenceEmbeddingsBuilder::remote(
            SentenceEmbeddingsModelType::AllMiniLmL12V2
        ).create_model() }).await?;

    let sentences: Vec<&str> = chunks.iter().map(|c| {
        match str::from_utf8(c) {
            Ok(s) => s,
            Err(e) => {
                debug!("Failed to transform bytes to utf8 string: error {}, {:?}", e, c);
                ""
            }
        }
    }).collect();
    return Ok(model?.encode(&sentences)?);
}

fn chunk_text(text: &String, chunk_size: usize) -> Vec<&[u8]> {
    let text_bytes: &[u8] = text.as_bytes();
    let text_size = text_bytes.len();

    let mut i: usize = 0;
    let mut res: Vec<&[u8]> = vec![];
    while i < text_size {
        let end: usize = cmp::min(i+chunk_size, text_size);
        res.push(&text_bytes[i..end]);

        i = end;
    }
    return res;
}
