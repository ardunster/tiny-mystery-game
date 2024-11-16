use std::hash::{DefaultHasher, Hash, Hasher};

pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub fn coin_flip(hash: &u64) -> bool {
    hash % 2 == 0
}

pub fn position_in_range(max: &u64, hash: &u64) -> u64 {
    hash % max
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn coin_flip_even() {
        assert_eq!(coin_flip(&2), true);
        assert_eq!(coin_flip(&4), true);
        assert_eq!(coin_flip(&6), true);
        assert_eq!(coin_flip(&10), true);
        assert_eq!(coin_flip(&4222), true);
        assert_eq!(coin_flip(&932747129874), true);
    }

    #[test]
    fn coin_flip_odd() {
        assert_eq!(coin_flip(&1), false);
        assert_eq!(coin_flip(&3), false);
        assert_eq!(coin_flip(&5), false);
        assert_eq!(coin_flip(&15), false);
        assert_eq!(coin_flip(&1237681), false);
        assert_eq!(coin_flip(&4489329749), false);
    }

    #[test]
    fn coin_flip_same() {
        assert_eq!(coin_flip(&0), coin_flip(&0));
        assert_eq!(coin_flip(&1), coin_flip(&1));
        assert_eq!(coin_flip(&2), coin_flip(&2));
        assert_eq!(coin_flip(&7), coin_flip(&7));
        assert_eq!(coin_flip(&45), coin_flip(&45));
        assert_eq!(coin_flip(&802), coin_flip(&802));
        assert_eq!(coin_flip(&9001), coin_flip(&9001));
        assert_eq!(coin_flip(&14703), coin_flip(&14703));
        assert_eq!(coin_flip(&7458379), coin_flip(&7458379));
    }
}
