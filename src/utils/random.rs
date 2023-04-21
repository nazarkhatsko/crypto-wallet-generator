use rand::Rng;

pub fn get_random_n(min: usize, max: usize) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

pub fn get_random_vec(min: u8, max: u8, len: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut v = Vec::with_capacity(len);
    v.iter_mut().map(|_| rng.gen_range(min..max));
    v
}
