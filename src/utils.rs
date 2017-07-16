//! This module contains the random_state function used in random_grid

/// Return true with 25% of probability
pub fn random_state() -> bool {
    use rand;
    use rand::distributions::{IndependentSample, Range};

    let step = Range::new(0, 4);
    let mut rng = rand::thread_rng();
    let choice = step.ind_sample(&mut rng);
    choice == 0
}
