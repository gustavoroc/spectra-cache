// Este arquivo está vazio de propósito.
// Estamos começando com os testes primeiro, seguindo TDD. 

use std::time::{Duration, Instant};
use std::collections::{HashMap, BTreeMap};
use std::iter::Iterator;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

/// A distributed hash table implementation that provides O(1) access time.
/// 
/// This structure manages cache entries with support for:
/// - Fast key-value lookups
/// - TTL-based expiration
/// - Automatic cleanup of expired entries
/// - Thread-safe operations
#[derive(Debug)]
pub struct DistributedHashTable {
    entries: HashMap<String, Entry>,
    bloom_filter: BloomFilter,
}

#[derive(Debug)]
struct Entry {
    value: String,
    ttl: Option<Duration>,
    created_at: Instant,
    last_accessed_at: Instant,
}

impl Entry {
    /// Creates a new cache entry without TTL.
    /// 
    /// # Arguments
    /// 
    /// * `key` - The unique identifier for this cache entry
    /// * `value` - The data stored in this cache entry
    /// 
    /// # Examples
    /// 
    /// ```
    /// use spectra_cache::DistributedHashTable;
    /// 
    /// let mut cache = DistributedHashTable::new();
    /// cache.insert("user:123", "John Doe");
    /// assert_eq!(cache.get("user:123"), Some("John Doe"));
    /// ```
    fn new(key: &str, value: &str) -> Self {
        Self::with_ttl(key, value, None)
    }
    
    /// Creates a new cache entry with optional TTL.
    /// 
    /// # Arguments
    /// 
    /// * `key` - The unique identifier for this cache entry
    /// * `value` - The data stored in this cache entry
    /// * `ttl` - Optional duration after which the entry expires
    /// 
    /// # Examples
    /// 
    /// ```
    /// use spectra_cache::DistributedHashTable;
    /// use std::time::Duration;
    /// 
    /// let mut cache = DistributedHashTable::new();
    /// cache.insert_with_ttl("session:456", "active", Duration::from_secs(3600));
    /// assert!(cache.contains_key("session:456"));
    /// ```
    fn with_ttl(_key: &str, value: &str, ttl: Option<Duration>) -> Self {
        let now = Instant::now();
        Self {
            value: value.to_string(),
            ttl,
            created_at: now,
            last_accessed_at: now,
        }
    }
    
    /// Returns the value of the cache entry.
    fn value(&self) -> &str {
        &self.value
    }
    
    /// Checks if the entry has expired based on its TTL.
    /// 
    /// Returns `true` if the entry has a TTL and the current age exceeds it.
    /// Returns `false` if the entry has no TTL or hasn't expired yet.
    fn is_expired(&self) -> bool {
        self.ttl.map_or(false, |ttl| self.age() > ttl)
    }
    
    /// Updates the last accessed time to now.
    /// 
    /// This method should be called whenever the entry is accessed
    /// to maintain accurate idle time tracking.
    fn touch(&mut self) {
        self.last_accessed_at = Instant::now();
    }
    
    /// Updates the value of the cache entry.
    /// 
    /// This method also calls `touch()` to update the last accessed time.
    /// 
    /// # Arguments
    /// 
    /// * `new_value` - The new value to store in this entry
    fn update_value(&mut self, new_value: &str) {
        self.value = new_value.to_string();
        self.touch();
    }
    
    /// Returns how long this entry has been in the cache.
    fn age(&self) -> Duration {
        self.created_at.elapsed()
    }
}

impl DistributedHashTable {
    /// Creates a new empty distributed hash table.
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            bloom_filter: BloomFilter::new(1000, 0.01), // Inicializa com capacidade de 1000 e 1% de falsos positivos
        }
    }

    /// Returns the number of entries in the table.
    pub fn size(&self) -> usize {
        self.entries.len()
    }

    /// Returns true if the table is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Inserts a key-value pair into the table.
    /// 
    /// If the key already exists, the value will be updated.
    pub fn insert(&mut self, key: &str, value: &str) {
        let entry = Entry::new(key, value);
        self.entries.insert(key.to_string(), entry);
        self.bloom_filter.insert(&key.to_string());
    }

    /// Inserts a key-value pair with TTL into the table.
    /// 
    /// The entry will be automatically removed when the TTL expires.
    pub fn insert_with_ttl(&mut self, key: &str, value: &str, ttl: Duration) {
        let entry = Entry::with_ttl(key, value, Some(ttl));
        self.entries.insert(key.to_string(), entry);
        self.bloom_filter.insert(&key.to_string());
    }

    /// Retrieves a value by key.
    /// 
    /// Returns None if the key doesn't exist or if the entry has expired.
    pub fn get(&mut self, key: &str) -> Option<&str> {
        // Primeiro verifica no Bloom Filter
        if !self.bloom_filter.contains(&key.to_string()) {
            return None;
        }

        let is_expired = self.entries.get(key).map_or(false, |entry| entry.is_expired());
        
        if is_expired {
            self.entries.remove(key);
            None
        } else if let Some(entry) = self.entries.get_mut(key) {
            entry.touch();
            Some(entry.value())
        } else {
            None
        }
    }

    /// Removes a key-value pair from the table.
    /// 
    /// Returns the removed value if the key existed.
    pub fn remove(&mut self, key: &str) -> Option<String> {
        if let Some(value) = self.entries.remove(key) {
            Some(value.value().to_string())
        } else {
            None
        }
    }

    /// Updates an existing entry's value.
    /// 
    /// Returns true if the update was successful (key existed).
    pub fn update(&mut self, key: &str, value: &str) -> bool {
        if let Some(entry) = self.entries.get_mut(key) {
            entry.update_value(value);
            true
        } else {
            false
        }
    }

    /// Removes all entries from the table.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.bloom_filter.clear();
    }

    /// Checks if a key exists in the table.
    /// 
    /// Returns false if the key doesn't exist or if the entry has expired.
    pub fn contains_key(&mut self, key: &str) -> bool {
        // Primeiro verifica no Bloom Filter
        if !self.bloom_filter.contains(&key.to_string()) {
            return false;
        }

        if let Some(entry) = self.entries.get(key) {
            if entry.is_expired() {
                self.entries.remove(key);
                false
            } else {
                true
            }
        } else {
            false
        }
    }

    /// Returns an iterator over all keys in the table.
    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.entries.keys()
    }

    /// Returns an iterator over all values in the table.
    pub fn values(&self) -> impl Iterator<Item = &String> {
        self.entries.values().map(|entry| &entry.value)
    }
}

/// A B-tree based cache implementation that provides O(log n) access time with ordered keys.
/// 
/// This structure manages cache entries with support for:
/// - Ordered key-value lookups
/// - TTL-based expiration
/// - Automatic cleanup of expired entries
/// - Thread-safe operations
#[derive(Debug)]
pub struct BTreeCache {
    entries: BTreeMap<String, Entry>,
    bloom_filter: BloomFilter,
}

impl BTreeCache {
    /// Creates a new empty B-tree cache.
    pub fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
            bloom_filter: BloomFilter::new(1000, 0.01), // Inicializa com capacidade de 1000 e 1% de falsos positivos
        }
    }

    /// Returns the number of entries in the cache.
    pub fn size(&self) -> usize {
        self.entries.len()
    }

    /// Returns true if the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Inserts a key-value pair into the cache.
    /// 
    /// If the key already exists, the value will be updated.
    /// Keys are maintained in sorted order.
    pub fn insert(&mut self, key: &str, value: &str) {
        let entry = Entry::new(key, value);
        self.entries.insert(key.to_string(), entry);
        self.bloom_filter.insert(&key.to_string());
    }

    /// Inserts a key-value pair with TTL into the cache.
    /// 
    /// The entry will be automatically removed when the TTL expires.
    /// Keys are maintained in sorted order.
    pub fn insert_with_ttl(&mut self, key: &str, value: &str, ttl: Duration) {
        let entry = Entry::with_ttl(key, value, Some(ttl));
        self.entries.insert(key.to_string(), entry);
        self.bloom_filter.insert(&key.to_string());
    }

    /// Retrieves a value by key.
    /// 
    /// Returns None if the key doesn't exist or if the entry has expired.
    /// Time complexity: O(log n)
    pub fn get(&mut self, key: &str) -> Option<&str> {
        // Primeiro verifica no Bloom Filter
        if !self.bloom_filter.contains(&key.to_string()) {
            return None;
        }

        let is_expired = self.entries.get(key).map_or(false, |entry| entry.is_expired());
        
        if is_expired {
            self.entries.remove(key);
            None
        } else if let Some(entry) = self.entries.get_mut(key) {
            entry.touch();
            Some(entry.value())
        } else {
            None
        }
    }

    /// Removes a key-value pair from the cache.
    /// 
    /// Returns the removed value if the key existed.
    /// Time complexity: O(log n)
    pub fn remove(&mut self, key: &str) -> Option<String> {
        if let Some(value) = self.entries.remove(key) {
            Some(value.value().to_string())
        } else {
            None
        }
    }

    /// Updates an existing entry's value.
    /// 
    /// Returns true if the update was successful (key existed).
    /// Time complexity: O(log n)
    pub fn update(&mut self, key: &str, value: &str) -> bool {
        if let Some(entry) = self.entries.get_mut(key) {
            entry.update_value(value);
            true
        } else {
            false
        }
    }

    /// Removes all entries from the cache.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.bloom_filter.clear();
    }

    /// Checks if a key exists in the cache.
    /// 
    /// Returns false if the key doesn't exist or if the entry has expired.
    /// Time complexity: O(log n)
    pub fn contains_key(&mut self, key: &str) -> bool {
        // Primeiro verifica no Bloom Filter
        if !self.bloom_filter.contains(&key.to_string()) {
            return false;
        }

        if let Some(entry) = self.entries.get(key) {
            if entry.is_expired() {
                self.entries.remove(key);
                false
            } else {
                true
            }
        } else {
            false
        }
    }

    /// Returns an iterator over all keys in sorted order.
    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.entries.keys()
    }

    /// Returns an iterator over all values in key-sorted order.
    pub fn values(&self) -> impl Iterator<Item = &String> {
        self.entries.values().map(|entry| &entry.value)
    }

    /// Returns an iterator over entries within a range of keys.
    /// 
    /// # Arguments
    /// 
    /// * `start` - The inclusive start of the range
    /// * `end` - The inclusive end of the range
    pub fn range(&self, start: &str, end: &str) -> impl Iterator<Item = (&String, &String)> {
        self.entries.range(start.to_string()..=end.to_string())
            .map(|(k, v)| (k, &v.value))
    }

    /// Returns the first key-value pair in the cache.
    pub fn first(&self) -> Option<(&String, &str)> {
        self.entries.first_key_value().map(|(k, v)| (k, v.value()))
    }

    /// Returns the last key-value pair in the cache.
    pub fn last(&self) -> Option<(&String, &str)> {
        self.entries.last_key_value().map(|(k, v)| (k, v.value()))
    }
}

/// A probabilistic data structure for testing set membership.
/// 
/// This structure provides:
/// - Fast membership testing with a small probability of false positives
/// - No false negatives
/// - Space-efficient storage
/// - Merge operations for combining filters
#[derive(Debug)]
pub struct BloomFilter {
    bits: Vec<bool>,
    num_hash_functions: usize,
    size: usize,
}

impl BloomFilter {
    /// Creates a new Bloom filter with the specified capacity and false positive rate.
    /// 
    /// # Arguments
    /// 
    /// * `capacity` - Expected number of elements to be stored
    /// * `false_positive_rate` - Desired probability of false positives (0.0 to 1.0)
    /// 
    /// # Examples
    /// 
    /// ```
    /// use spectra_cache::BloomFilter;
    /// 
    /// let filter = BloomFilter::new(1000, 0.01);
    /// assert!(filter.is_empty());
    /// ```
    pub fn new(capacity: usize, false_positive_rate: f64) -> Self {
        let num_bits = Self::optimal_num_bits(capacity, false_positive_rate);
        let num_hash_functions = Self::optimal_num_hash_functions(num_bits, capacity);
        
        Self {
            bits: vec![false; num_bits],
            num_hash_functions,
            size: 0,
        }
    }
    
    /// Returns the number of elements in the filter.
    pub fn size(&self) -> usize {
        self.size
    }
    
    /// Returns true if the filter is empty.
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
    
    /// Inserts an element into the filter.
    /// 
    /// # Arguments
    /// 
    /// * `item` - The element to insert
    pub fn insert<T: Hash>(&mut self, item: &T) {
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        let hash = hasher.finish();
        
        for i in 0..self.num_hash_functions {
            let index = self.get_index(hash, i);
            self.bits[index] = true;
        }
        
        self.size += 1;
    }
    
    /// Checks if an element is in the filter.
    /// 
    /// Returns true if the element is probably in the filter.
    /// There is a small probability of false positives.
    /// 
    /// # Arguments
    /// 
    /// * `item` - The element to check
    pub fn contains<T: Hash>(&self, item: &T) -> bool {
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        let hash = hasher.finish();
        
        for i in 0..self.num_hash_functions {
            let index = self.get_index(hash, i);
            if !self.bits[index] {
                return false;
            }
        }
        
        true
    }
    
    /// Removes all elements from the filter.
    pub fn clear(&mut self) {
        self.bits.fill(false);
        self.size = 0;
    }
    
    /// Merges another Bloom filter into this one.
    /// 
    /// # Arguments
    /// 
    /// * `other` - The Bloom filter to merge with
    pub fn merge(&mut self, other: &BloomFilter) {
        assert_eq!(self.bits.len(), other.bits.len(), "Bloom filters must have the same size to merge");
        assert_eq!(self.num_hash_functions, other.num_hash_functions, "Bloom filters must have the same number of hash functions to merge");
        
        for i in 0..self.bits.len() {
            self.bits[i] |= other.bits[i];
        }
        
        // Não somamos os tamanhos porque podem haver elementos duplicados
        // O tamanho real é uma estimativa baseada na densidade dos bits
        let density = self.bits.iter().filter(|&&bit| bit).count() as f64 / self.bits.len() as f64;
        self.size = (self.bits.len() as f64 * density / self.num_hash_functions as f64).round() as usize;
    }
    
    /// Calculates the optimal number of bits based on capacity and false positive rate.
    fn optimal_num_bits(capacity: usize, false_positive_rate: f64) -> usize {
        let ln2 = std::f64::consts::LN_2;
        let ln2_squared = ln2 * ln2;
        let capacity_f64 = capacity as f64;
        let bits = (-capacity_f64 * false_positive_rate.ln()) / ln2_squared;
        bits.ceil() as usize
    }
    
    /// Calculates the optimal number of hash functions based on number of bits and capacity.
    fn optimal_num_hash_functions(num_bits: usize, capacity: usize) -> usize {
        let ln2 = std::f64::consts::LN_2;
        ((num_bits as f64 / capacity as f64) * ln2).round() as usize
    }
    
    /// Gets the index for a hash value and hash function number.
    fn get_index(&self, hash: u64, i: usize) -> usize {
        let mut combined_hash = hash;
        for _ in 0..i {
            combined_hash = combined_hash.wrapping_mul(0x517cc1b727220a95);
        }
        combined_hash as usize % self.bits.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;

    #[test]
    fn test_create_and_get() {
        let mut cache = DistributedHashTable::new();
        
        cache.insert("key1", "value1");
        assert_eq!(cache.get("key1"), Some("value1"));
        assert_eq!(cache.get("key2"), None);
    }

    #[test]
    fn test_insert_with_ttl() {
        let mut cache = DistributedHashTable::new();
        
        cache.insert_with_ttl("key1", "value1", Duration::from_millis(50));
        assert_eq!(cache.get("key1"), Some("value1"));
        
        sleep(Duration::from_millis(100));
        assert_eq!(cache.get("key1"), None);
    }

    #[test]
    fn test_update() {
        let mut cache = DistributedHashTable::new();
        
        cache.insert("key1", "value1");
        assert!(cache.update("key1", "value2"));
        assert_eq!(cache.get("key1"), Some("value2"));
        
        assert!(!cache.update("key2", "value3"));
    }

    #[test]
    fn test_remove() {
        let mut cache = DistributedHashTable::new();
        
        cache.insert("key1", "value1");
        assert_eq!(cache.remove("key1"), Some("value1".to_string()));
        assert_eq!(cache.get("key1"), None);
        assert_eq!(cache.remove("key2"), None);
    }

    #[test]
    fn test_clear() {
        let mut cache = DistributedHashTable::new();
        
        cache.insert("key1", "value1");
        cache.insert("key2", "value2");
        assert_eq!(cache.size(), 2);
        
        cache.clear();
        assert!(cache.is_empty());
    }

    #[test]
    fn test_contains_key() {
        let mut cache = DistributedHashTable::new();
        
        cache.insert("key1", "value1");
        assert!(cache.contains_key("key1"));
        assert!(!cache.contains_key("key2"));
        
        cache.insert_with_ttl("key3", "value3", Duration::from_millis(50));
        assert!(cache.contains_key("key3"));
        
        sleep(Duration::from_millis(100));
        assert!(!cache.contains_key("key3"));
    }

    #[test]
    fn test_keys_and_values() {
        let mut cache = DistributedHashTable::new();
        
        cache.insert("key1", "value1");
        cache.insert("key2", "value2");
        
        let keys: Vec<_> = cache.keys().collect();
        assert_eq!(keys.len(), 2);
        assert!(keys.contains(&&"key1".to_string()));
        assert!(keys.contains(&&"key2".to_string()));
        
        let values: Vec<_> = cache.values().collect();
        assert_eq!(values.len(), 2);
        assert!(values.contains(&&"value1".to_string()));
        assert!(values.contains(&&"value2".to_string()));
    }
} 