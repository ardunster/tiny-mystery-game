use std::hash::{DefaultHasher, Hash, Hasher};

pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub fn coin_flip(hash: u64) -> bool {
    hash % 2 == 0
}

pub fn position_in_range(max: u64, hash: u64) -> u64 {
    hash % max
}
