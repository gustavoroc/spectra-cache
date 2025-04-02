use spectra_cache::BloomFilter;

#[test]
fn test_create_empty_filter() {
    let filter = BloomFilter::new(1000, 0.01);
    assert_eq!(filter.size(), 0);
    assert!(filter.is_empty());
}

#[test]
fn test_insert_and_contains() {
    let mut filter = BloomFilter::new(1000, 0.01);
    let key = String::from("test_key");
    
    assert!(!filter.contains(&key));
    filter.insert(&key);
    assert!(filter.contains(&key));
    assert_eq!(filter.size(), 1);
    assert!(!filter.is_empty());
}

#[test]
fn test_multiple_insertions() {
    let mut filter = BloomFilter::new(1000, 0.01);
    
    // Insert multiple keys
    let keys = vec![
        String::from("key1"),
        String::from("key2"),
        String::from("key3"),
        String::from("key4"),
        String::from("key5")
    ];
    for key in &keys {
        filter.insert(key);
    }
    
    // Check that all inserted keys are present
    for key in &keys {
        assert!(filter.contains(key));
    }
    
    // Check that non-inserted keys are not present
    assert!(!filter.contains(&String::from("non_existent_key")));
    assert_eq!(filter.size(), 5);
}

#[test]
fn test_false_positive_rate() {
    let mut filter = BloomFilter::new(1000, 0.01);
    let num_insertions = 100;
    let num_checks = 1000;
    
    // Insert some keys
    for i in 0..num_insertions {
        filter.insert(&format!("key{}", i));
    }
    
    // Check for non-existent keys
    let mut false_positives = 0;
    for i in 0..num_checks {
        if filter.contains(&format!("non_existent{}", i)) {
            false_positives += 1;
        }
    }
    
    // Calculate actual false positive rate
    let actual_rate = false_positives as f64 / num_checks as f64;
    assert!(actual_rate <= 0.01, "False positive rate {} exceeds expected 0.01", actual_rate);
}

#[test]
fn test_clear() {
    let mut filter = BloomFilter::new(1000, 0.01);
    
    filter.insert(&String::from("key1"));
    filter.insert(&String::from("key2"));
    assert_eq!(filter.size(), 2);
    
    filter.clear();
    assert!(filter.is_empty());
    assert_eq!(filter.size(), 0);
    assert!(!filter.contains(&String::from("key1")));
    assert!(!filter.contains(&String::from("key2")));
}

#[test]
fn test_capacity() {
    let capacity = 100;
    let mut filter = BloomFilter::new(capacity, 0.01);
    
    // Insert up to capacity
    for i in 0..capacity {
        filter.insert(&format!("key{}", i));
    }
    
    assert_eq!(filter.size(), capacity);
    
    // Try to insert more
    filter.insert(&String::from("extra_key"));
    assert_eq!(filter.size(), capacity + 1);
}

#[test]
fn test_merge() {
    let mut filter1 = BloomFilter::new(1000, 0.01);
    let mut filter2 = BloomFilter::new(1000, 0.01);
    
    filter1.insert(&String::from("key1"));
    filter1.insert(&String::from("key2"));
    
    filter2.insert(&String::from("key2"));
    filter2.insert(&String::from("key3"));
    
    filter1.merge(&filter2);
    
    assert!(filter1.contains(&String::from("key1")));
    assert!(filter1.contains(&String::from("key2")));
    assert!(filter1.contains(&String::from("key3")));
    assert_eq!(filter1.size(), 3);
} 