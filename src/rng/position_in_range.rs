pub fn position_in_range(min: &u64, max: &u64, hash: &u64) -> u64 {
    let span = max - min + 1;
    min + (hash % span)
}

#[test]
fn position_in_range_is_within_bounds() {
    let min = 10;
    let max = 15;

    for hash in 0..10_000u64 {
        let result = position_in_range(&min, &max, &hash);
        assert!(result >= min && result <= max, "hash={hash} gave {result}");
    }
}

#[test]
fn position_in_range_can_hit_endpoints() {
    let min = 10;
    let max = 15;

    let mut found_min = false;
    let mut found_max = false;

    for hash in 0..10_000u64 {
        let v = position_in_range(&min, &max, &hash);
        if v == min {
            found_min = true;
        }
        if v == max {
            found_max = true;
        }
        if found_min && found_max {
            break;
        }
    }

    assert!(found_min, "never hit min");
    assert!(found_max, "never hit max");
}
