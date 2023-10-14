use std::error::Error;
use linfa::DatasetBase;
use linfa::traits::Fit;
use linfa_nn::distance::L2Dist;
use linfa_tsne::TSneParams;
use linfa::prelude::Transformer;
use linfa_clustering::KMeans;
use linfa::prelude::PredictInplace;
use ndarray::{Array, Array2, Array1, Axis};
use rand_xoshiro::Xoshiro256Plus;
use rand_xoshiro::rand_core::SeedableRng;

/**
 * most of the time, the caller knows what chunks they want you to use
 * how to render the UI?
 * should i send back data with the categorization?
 *
 * plan: will categorize points based on what centroid they are closest to
 * train the model on the encoded vectors, get the centroids, and then predict the category for
 * each vector
 */

pub struct KMeansService {
    pub model: KMeans<f32, L2Dist>,
    pub memberships: Array1<usize>,
    pub points: Array2<f32>,
    pub centroid_points: Array2<f32>,
}

pub fn init(encoded_cs: &Vec<Vec<f32>>, k: usize) -> Result<KMeansService, Box<dyn Error>> {
    // Our random number generator, seeded for reproducibility
    let rng = Xoshiro256Plus::seed_from_u64(42);

    let records = Array::from_shape_fn(
        (encoded_cs.len(), encoded_cs[0].len()),
        |(i, j)| { encoded_cs[i][j] }
    );

    let targets = Array::from_iter(0..records.len_of(Axis(0)));
    let observations = DatasetBase::new(records.clone(), targets);

    // TODO can we do a dynamic number of centroids? Right now it's always 10,
    // when maybe we would want more or less groups?
    let model = KMeans::params_with_rng(k, rng.clone())
        .tolerance(1e-2)
        .fit(&observations)
        .expect("KMeans fitted");

    let mut memberships = Array::from_iter(0..encoded_cs.len());

    model.predict_inplace(&records, &mut memberships);

    // TODO what is perplecity?
    let perplexity = 0.5;

    let points = TSneParams::embedding_size(2)
        .perplexity(perplexity)
        .approx_threshold(0.1)
        .transform(records)?;

    let centroid_points = TSneParams::embedding_size(2)
        .perplexity(perplexity)
        .approx_threshold(0.1)
        .transform(model.centroids().clone())?;

    return Ok(KMeansService {
        model: model,
        memberships: memberships,
        points: points,
        centroid_points: centroid_points,
    });
}
