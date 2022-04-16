use bitvec::prelude::*;
use fnv::FnvHasher;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

trait Filter {
    fn insert(&mut self, item: &str);
    fn test(&self, item: &str) -> bool;
    fn hash(&self, item: &str) -> Vec<usize>;
}

#[derive(Debug)]
struct BloomFilter {
    bit_arr: BitArray,
}

impl BloomFilter {
    fn new() -> Self {
        Self {
            bit_arr: bitarr![0; 32],
        }
    }
}

fn std_hash(item: &str, len: u64) -> usize {
    let mut hasher = DefaultHasher::new();
    hasher.write(item.as_bytes());

    let res = hasher.finish() % len;
    res.try_into().unwrap()
}

fn fnv_hash(item: &str, len: u64) -> usize {
    let mut hasher = FnvHasher::default();
    // write input message
    hasher.write(item.as_bytes());
    let res = hasher.finish() % len;

    res.try_into().unwrap()
}

impl Filter for BloomFilter {
    fn insert(&mut self, item: &str) {
        let positions = self.hash(item);

        println!("positions: {:?}", positions);

        for position in positions {
            self.bit_arr.set(position, true);
        }
    }

    fn test(&self, item: &str) -> bool {
        let positions = self.hash(item);

        for position in positions.into_iter() {
            if self.bit_arr.get(position).as_deref() == Some(&false) {
                return false;
            }
        }
        true
    }

    fn hash(&self, item: &str) -> Vec<usize> {
        let mut hashes = Vec::with_capacity(2);
        hashes.push(std_hash(item, 32));
        hashes.push(fnv_hash(item, 32));

        hashes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut filter = BloomFilter::new();
        assert!(!filter.test(&"Insert"));
        let expected_bit_arr = bitarr![
            0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0
        ];
        filter.insert(&"Insert");
        assert_eq!(filter.bit_arr, expected_bit_arr);
        assert!(filter.test(&"Insert"));
    }
}
