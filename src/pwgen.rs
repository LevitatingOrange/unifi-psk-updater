use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

pub fn generate_password(len: usize) -> String {
    thread_rng()
    .sample_iter(&Alphanumeric)
    .take(len)
    .collect()
}