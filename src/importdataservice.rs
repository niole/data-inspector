use log::info;

/// downloads data over http from the specified uri
/// no auth
/// chunks, encodes, and stores in vector db
///
/// # Arguments
/// * `uri` - the location of the data
pub fn  import_data(uri: &String) {
    info!("{}", uri);
}
