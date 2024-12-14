//! Hash Table
//!
//! Hash table (or hash map, or just map, or dictionary) is a data structure that implements
//! an associative array , where _keys_ are mapped to _values_. You should use a hash map when:
//! - You want to associate arbitrary keys with an arbitrary value
//! - You want a cache
//! - You want a map, with no extra functionality
//!
//! ![picture](https://upload.wikimedia.org/wikipedia/commons/thumb/7/7d/Hash_table_3_1_1_0_1_0_0_SP.svg/315px-Hash_table_3_1_1_0_1_0_0_SP.svg.png)
//!
//! ## References
//! - [Hash table- Wikipedia](https://en.wikipedia.org/wiki/Hash_table)
//! - [Implementing a simple Hashmap in Rust](https://edgl.dev/blog/rust-hashmap/)

use std::{
    collections::hash_map::RandomState,
    hash::{BuildHasher, Hash},
};

// Can make our own trait vs std::hash::Hash for supporting a hashable type
pub trait Hashable {
    fn hash(&self) -> usize;
}

/// Probing type to use
#[derive(Debug, Clone, PartialEq)]
pub enum ProbeType {
    Quadratic,
    Linear,
}

/// HashTable object which uses the `std` `RandomState` to create unique instances of `Hasher` and
/// generic over Key and Value types.
#[derive(Debug, Clone)]
pub struct HashTable<K, V, S = RandomState> {
    /// Internal hash map storage. The slots of this array will either be nothing or key/value
    /// tuple pair, so wrap in `Option`.
    storage: Vec<Option<(K, V)>>,

    /// Length for quick querying
    len: usize,

    /// State of the hasher for hashing keys
    state: S,

    /// Probing type to use
    probing: ProbeType,
}

/// Basic utility methods (no trait bounds to keep generic)
impl<K, V, S> HashTable<K, V, S> {
    pub fn new(hasher: S, probing: ProbeType) -> Self {
        Self {
            storage: (0..8).map(|_| None).collect(),
            len: 0,
            state: hasher,
            probing,
        }
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn is_quadratic(&self) -> bool {
        self.probing == ProbeType::Quadratic
    }

    /// For hashmap performance, it should keep a good load factor, which is defined as:
    /// `load factor = number of entries / number of slots`
    /// [Some papers](https://dl.acm.org/doi/10.1145/356643.356645) say when load factor approaches
    /// 0.7 - 0.8 it should be resized.
    fn should_resize(&self) -> bool {
        (self.len as f64 / self.storage.len() as f64) > 0.7
    }
}

/// Impl for probing and searching: when searching for a slot (when inserting or searching), we
/// need to handle when a hash collision happens.
impl<K, V, S> HashTable<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    /// Use linear or quadratic probing, starting with `i = 0`:
    /// - If the slot `hash(key) + i % capacity` (for linear) or `hash(key) + (i + i^2) / 2 % capacity` (for quadratic) is unused, use it.
    /// - If the slot `hash(key) + i % capacity` (for linear) or `hash(key) + (i + i^2) / 2 % capacity` is used, check if the key matches:
    ///   + If it matches, use it
    ///   + If it doesn't, repeat probing with `i += offset` (w/`offset` usually == 1)
    fn find_slot_index(&self, key: &K) -> usize {
        let start_idx = self.state.hash_one(key) as usize;
        let mut iter_idx = 0;
        let len = self.storage.len();

        loop {
            let idx_mod: usize = if self.probing == ProbeType::Linear {
                (start_idx + iter_idx) % len
            } else {
                (start_idx + (iter_idx + iter_idx.pow(2)) / 2) % len
            };
            match &self.storage[idx_mod] {
                Some(kv) if kv.0.eq(key) => break idx_mod,
                None => break idx_mod,
                _ => {
                    iter_idx += 1;

                    assert!(
                        iter_idx <= len,
                        "find_slot called without a matching key and full storage"
                    );
                }
            }
        }
    }

    /// Find slot for given Key and return key/value pair. Uses linear or quadratic probing, rather
    /// than simple `index = key.hash() % self.storage.len()` to resolve hash collisions (where two
    /// different keys resolve to the same hash or index post modulo)
    fn find_slot(&self, key: &K) -> &Option<(K, V)> {
        &self.storage[self.find_slot_index(key)]
    }

    /// Find slot for given Key and return mutable key/value pair. Uses linear or quadratic probing, rather
    /// than simple `index = key.hash() % self.storage.len()`
    fn find_slot_mut(&mut self, key: &K) -> &mut Option<(K, V)> {
        let slot_idx = self.find_slot_index(key);
        &mut self.storage[slot_idx]
    }

    /// Allocate new storage and rehash all entries into new storage
    fn resize(&mut self) {
        // simply double current size
        let new_storage: Vec<Option<(K, V)>> = (0..self.storage.len() * 2).map(|_| None).collect();

        // Replace the storage with the new one, so we can use self methods to rehash
        let old_storage = std::mem::replace(&mut self.storage, new_storage);
        self.len = 0; // zero out length since we will reindex/rehash new_storage

        // Insert back and re-hash all previous entries
        for (k, v) in old_storage.into_iter().flatten() {
            self.insert(k, v);
        }
    }

    /// To insert a key/value pair:
    /// - Check if load factor is good, other resize backing structure to increase size
    /// - Search a slot using one of the probing/search methods
    pub fn insert(&mut self, key: K, value: V) {
        if self.should_resize() {
            self.resize();
        }

        let slot = self.find_slot_mut(&key);
        // if none, there wasn't an element at that index
        let inc_len = slot.is_none();
        *slot = Some((key, value));

        // increment length as we just added a new element
        if inc_len {
            self.len += 1;
        }
    }

    /// Search a slot using probing method and return value for given key
    pub fn search(&self, key: &K) -> Option<&V> {
        self.find_slot(key).as_ref().map(|kv| &kv.1)
    }

    /// Search a slot using probing method and return mutable value for given key
    pub fn search_mut(&mut self, key: &K) -> Option<&mut V> {
        self.find_slot_mut(key).as_mut().map(|kv| &mut kv.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let map: HashTable<u32, u32> = HashTable::new(RandomState::new(), ProbeType::Linear);
        assert!(map.is_empty());
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn insert() {
        let mut map: HashTable<u32, u32> = HashTable::new(RandomState::new(), ProbeType::Linear);
        map.insert(1, 2);
        map.insert(2, 2);
        map.insert(3, 2);
        map.insert(4, 2);
        map.insert(1, 3);
        map.insert(2, 4);
        map.insert(3, 5);
        map.insert(4, 6);
        assert_eq!(map.len(), 4);
    }

    #[test]
    fn search() {
        let mut map: HashTable<u32, u32> = HashTable::new(RandomState::new(), ProbeType::Linear);
        map.insert(1, 3);
        map.insert(2, 2);
        map.insert(3, 1);
        assert_eq!(map.search(&1), Some(&3));
        assert_eq!(map.search(&2), Some(&2));
        assert_eq!(map.search(&3), Some(&1));
        assert_eq!(map.search(&6), None);
        assert_eq!(map.len(), 3);
    }

    #[test]
    fn search_mut() {
        let mut map: HashTable<u32, u32> = HashTable::new(RandomState::new(), ProbeType::Linear);
        map.insert(1, 3);
        map.insert(2, 2);
        map.insert(3, 1);
        assert_eq!(map.search(&1), Some(&3));
        let val = map.search_mut(&1);

        if let Some(val) = val {
            *val = 4;
        }
        assert_eq!(map.search(&1), Some(&4));
    }

    #[test]
    fn insert_lots() {
        let mut map: HashTable<u32, u32> = HashTable::new(RandomState::new(), ProbeType::Linear);

        for x in 0..100000 {
            map.insert(x, x);
            assert_eq!(map.search(&x).unwrap(), &x)
        }
    }

    #[test]
    fn insert_quadratic() {
        let mut map: HashTable<u32, u32> = HashTable::new(RandomState::new(), ProbeType::Quadratic);
        map.insert(1, 2);
        map.insert(2, 2);
        map.insert(3, 2);
        map.insert(4, 2);
        map.insert(1, 3);
        map.insert(2, 4);
        map.insert(3, 5);
        map.insert(4, 6);
        assert_eq!(map.len(), 4);
    }

    #[test]
    fn search_quadratic() {
        let mut map: HashTable<u32, u32> = HashTable::new(RandomState::new(), ProbeType::Quadratic);
        map.insert(1, 3);
        map.insert(2, 2);
        map.insert(3, 1);
        assert_eq!(map.search(&1), Some(&3));
        assert_eq!(map.search(&2), Some(&2));
        assert_eq!(map.search(&3), Some(&1));
        assert_eq!(map.search(&6), None);
        assert_eq!(map.len(), 3);
    }

    #[test]
    fn search_mut_quadratic() {
        let mut map: HashTable<u32, u32> = HashTable::new(RandomState::new(), ProbeType::Quadratic);
        map.insert(1, 3);
        map.insert(2, 2);
        map.insert(3, 1);
        assert_eq!(map.search(&1), Some(&3));
        let val = map.search_mut(&1);

        if let Some(val) = val {
            *val = 4;
        }
        assert_eq!(map.search(&1), Some(&4));
    }

    #[test]
    fn insert_lots_quadratic() {
        let mut map: HashTable<u32, u32> = HashTable::new(RandomState::new(), ProbeType::Quadratic);

        for x in 0..100000 {
            map.insert(x, x);
            assert_eq!(map.search(&x).unwrap(), &x)
        }
    }
}
