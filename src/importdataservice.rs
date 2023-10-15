use std::{str, cmp};
use plotly::color::NamedColor;
use plotly::common::Marker;
use std::error::Error;
use actix_web::rt::task::spawn_blocking;
use log::debug;
use reqwest;
use rust_bert::pipelines::sentence_embeddings::{SentenceEmbeddingsBuilder, SentenceEmbeddingsModelType};
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
pub async fn import_data(uri: &String, k: usize) -> Result<VisData, Box<dyn Error>> {
    return download_data(uri, k).await; // TODO poll?
}

pub async fn render_data(uri: &String, k: usize) -> Result<String, Box<dyn Error>>{
    let colors = vec![NamedColor::Azure, NamedColor::Black, NamedColor::Crimson, NamedColor:: Brown, NamedColor::Goldenrod, NamedColor::HotPink, NamedColor::Lavender, NamedColor::Grey, NamedColor::Orange, NamedColor::Plum, NamedColor::Tomato];
    let data = download_data(uri, k).await.expect("asdf");

    let trace = Scatter::new(
        data.data.iter().map(|d| d.point.0).collect(),
        data.data.iter().map(|d| d.point.1).collect(),
    )
    .hover_text_array(data.data.iter().map(|d| format!("{}", d.content)).collect())
    .mode(Mode::Markers)
    .marker(
        Marker::new()
        .color_array(data.data.iter().map(|d| colors[d.centroid_index] ).collect()),
    );
    let mut plot = Plot::new();
    plot.add_trace(trace);
    return Ok(plot.to_html());
}

async fn download_data(uri: &String, k: usize) -> Result<VisData, Box<dyn Error>> {
    let body = reqwest::get(uri).await.expect("body").text().await.unwrap();

    // chunk the text
    let chunks = chunk_text(&body, 4000);

    // encode the text
    let encoded_cs = encode_chunks(&chunks).await?;

    let service = kmeansservice::init(&encoded_cs, k)?;

    let memberships = service.memberships;

    let data = service.points.rows().into_iter().enumerate().map(|(index, point)| {
        DataPoint {
            centroid_index: memberships[index],
            point: (*point.get(0).expect("Should have 0 index"), *point.get(1).expect("Should have 1 index")),
            content: str::from_utf8(chunks[index]).unwrap().to_string()
        }
    }).collect();

    return Ok(VisData {
        centroids: service.centroid_points.rows().into_iter().map(|r| { (*r.get(0).expect("Should have 0 index"), *r.get(1).expect("Should have 1 index")) }).collect(),
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
