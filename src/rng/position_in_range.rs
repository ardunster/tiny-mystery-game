pub fn position_in_range(min: &u64, max: &u64, hash: &u64) -> u64 {
    let span = max - min + 1;
    min + (hash % span)
}
