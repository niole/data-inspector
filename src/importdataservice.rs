use std::{str, cmp};
use log::debug;
use reqwest;
use rust_bert::RustBertError;
use rust_bert::pipelines::sentence_embeddings::{SentenceEmbeddingsBuilder, SentenceEmbeddingsModelType};

/// downloads data over http from the specified uri
/// no auth
/// chunks, encodes, and stores in vector db
///
/// # Arguments
/// * `uri` - the location of the data
pub async fn  import_data(uri: &String) {
    download_data(uri).await; // TODO poll?
}

async fn download_data(uri: &String) {
    let body = reqwest::get(uri).await.expect("body").text().await.unwrap();

    // chunk the text
    let chunks = chunk_text(&body, 4000);

    // encode the text TODO make this work
    let encoded_cs: Vec<Vec<f32>> = encode_chunks(chunks).unwrap();
    print!("{:?}", encoded_cs.get(0).unwrap_or(&vec![]));
}

fn encode_chunks(chunks: Vec<&[u8]>) -> Result<Vec<Vec<f32>>, RustBertError> {
    let model = SentenceEmbeddingsBuilder::remote(
            SentenceEmbeddingsModelType::AllMiniLmL12V2
        ).create_model()?;

    let sentences: Vec<&str> = chunks.iter().map(|c| {
        match str::from_utf8(c) {
            Ok(s) => s,
            Err(e) => {
                debug!("Failed to transform bytes to utf8 string: error {}, {:?}", e, c);
                ""
            }
        }
    }).collect();

    return model.encode(&sentences);
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
