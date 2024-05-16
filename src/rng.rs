use std::hash::{DefaultHasher, Hash, Hasher};

pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub fn coin_flip(hash: u64) {
    // Todo: what to return? we're gonna modulo the hash such that we basically just return even
    //  (0) or odd (1), or whatever else makes actual sense.
}

pub fn position_in_range(max: u64, hash: u64) -> u64 {
    hash % max
}
