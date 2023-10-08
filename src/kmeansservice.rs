use linfa::DatasetBase;
use linfa::traits::Fit;
use linfa_nn::distance::L2Dist;
use linfa_clustering::KMeans;
use ndarray::Array;
use rand_xoshiro::Xoshiro256Plus;
use rand_xoshiro::rand_core::SeedableRng;

/**
 * plan: will categorize points based on what centroid they are closest to
 * train the model on the encoded vectors, get the centroids, and then predict the category for
 * each vector
 */

pub struct KMeansService {
    pub model: KMeans<f32, L2Dist>
}

pub fn init(data: Vec<Vec<f32>>) -> KMeansService {
    // Our random number generator, seeded for reproducibility
    let rng = Xoshiro256Plus::seed_from_u64(42);

    let records = Array::from_shape_fn(
        (data.len(), data[0].len()),
        |(i, j)| { data[i][j] }
    );

    //let mut records = Array::zeros((data.len(), data[0].len()));
    //for (i, x) in data.iter().enumerate() {
    //    records.row_mut(i).assign(&Array::from_iter(x));
    //}
    // let records = Array2::from_vec(data);

    let targets = Array::from_iter(0..data[0].len());
    let observations = DatasetBase::new(records, targets);

    let model = KMeans::params_with_rng(10, rng.clone())
        .tolerance(1e-2)
        .fit(&observations)
        .expect("KMeans fitted");

    return KMeansService {
        model: model
    };
}
