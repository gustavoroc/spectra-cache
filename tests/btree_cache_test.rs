use spectra_cache::BTreeCache;
use std::time::Duration;

#[test]
fn test_create_empty_cache() {
    let cache = BTreeCache::new();
    assert_eq!(cache.size(), 0);
    assert!(cache.is_empty());
}

#[test]
fn test_insert_and_get() {
    let mut cache = BTreeCache::new();
    let key = "test_key";
    let value = "test_value";
    
    cache.insert(key, value);
    assert_eq!(cache.size(), 1);
    assert!(!cache.is_empty());
    
    let retrieved = cache.get(key).unwrap();
    assert_eq!(retrieved, value);
}

#[test]
fn test_insert_with_ttl() {
    let mut cache = BTreeCache::new();
    let key = "test_key";
    let value = "test_value";
    let ttl = Duration::from_millis(50);
    
    cache.insert_with_ttl(key, value, ttl);
    assert_eq!(cache.size(), 1);
    
    // Verifica se o valor está disponível antes do TTL expirar
    assert_eq!(cache.get(key).unwrap(), value);
    
    // Espera o TTL expirar
    std::thread::sleep(Duration::from_millis(100));
    
    // Verifica se o valor foi removido após o TTL expirar
    assert!(cache.get(key).is_none());
    assert!(cache.is_empty());
}

#[test]
fn test_remove() {
    let mut cache = BTreeCache::new();
    let key = "test_key";
    let value = "test_value";
    
    cache.insert(key, value);
    assert_eq!(cache.size(), 1);
    
    let removed = cache.remove(key);
    assert_eq!(removed, Some(value.to_string()));
    assert!(cache.is_empty());
    
    // Tentar remover uma chave que não existe
    let removed = cache.remove("non_existent");
    assert!(removed.is_none());
}

#[test]
fn test_update_existing() {
    let mut cache = BTreeCache::new();
    let key = "test_key";
    let value1 = "value1";
    let value2 = "value2";
    
    cache.insert(key, value1);
    assert_eq!(cache.get(key).unwrap(), value1);
    
    cache.insert(key, value2);
    assert_eq!(cache.get(key).unwrap(), value2);
    assert_eq!(cache.size(), 1); // Tamanho não deve mudar ao atualizar
}

#[test]
fn test_clear() {
    let mut cache = BTreeCache::new();
    
    cache.insert("key1", "value1");
    cache.insert("key2", "value2");
    assert_eq!(cache.size(), 2);
    
    cache.clear();
    assert!(cache.is_empty());
    assert_eq!(cache.size(), 0);
}

#[test]
fn test_contains_key() {
    let mut cache = BTreeCache::new();
    let key = "test_key";
    let value = "test_value";
    
    assert!(!cache.contains_key(key));
    
    cache.insert(key, value);
    assert!(cache.contains_key(key));
    
    cache.remove(key);
    assert!(!cache.contains_key(key));
}

#[test]
fn test_keys() {
    let mut cache = BTreeCache::new();
    
    cache.insert("key1", "value1");
    cache.insert("key2", "value2");
    
    let keys: Vec<_> = cache.keys().collect();
    assert_eq!(keys.len(), 2);
    assert!(keys.contains(&&"key1".to_string()));
    assert!(keys.contains(&&"key2".to_string()));
}

#[test]
fn test_values() {
    let mut cache = BTreeCache::new();
    
    cache.insert("key1", "value1");
    cache.insert("key2", "value2");
    
    let values: Vec<_> = cache.values().collect();
    assert_eq!(values.len(), 2);
    assert!(values.contains(&&"value1".to_string()));
    assert!(values.contains(&&"value2".to_string()));
}

#[test]
fn test_ordered_operations() {
    let mut cache = BTreeCache::new();
    
    // Insert out of order
    cache.insert("c", "3");
    cache.insert("a", "1");
    cache.insert("b", "2");
    
    // Test first/last
    assert_eq!(cache.first(), Some((&"a".to_string(), "1")));
    assert_eq!(cache.last(), Some((&"c".to_string(), "3")));
    
    // Test range
    let range: Vec<_> = cache.range("a", "b").collect();
    assert_eq!(range.len(), 2);
    assert_eq!(range[0].1, "1");
    assert_eq!(range[1].1, "2");
    
    // Test ordered iteration
    let keys: Vec<_> = cache.keys().collect();
    assert_eq!(keys, vec!["a", "b", "c"]);
    
    let values: Vec<_> = cache.values().collect();
    assert_eq!(values, vec!["1", "2", "3"]);
}

#[test]
fn test_range_queries() {
    let mut cache = BTreeCache::new();
    
    // Inserir alguns números
    for i in 0..10 {
        cache.insert(&format!("key{}", i), &i.to_string());
    }
    
    // Testar range no meio
    let mid_range: Vec<_> = cache.range("key3", "key6")
        .map(|(_, v)| v.to_string())
        .collect();
    assert_eq!(mid_range, vec!["3", "4", "5", "6"]);
    
    // Testar range no início
    let start_range: Vec<_> = cache.range("key0", "key2")
        .map(|(_, v)| v.to_string())
        .collect();
    assert_eq!(start_range, vec!["0", "1", "2"]);
    
    // Testar range no fim
    let end_range: Vec<_> = cache.range("key7", "key9")
        .map(|(_, v)| v.to_string())
        .collect();
    assert_eq!(end_range, vec!["7", "8", "9"]);
} 