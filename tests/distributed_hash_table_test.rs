use spectra_cache::DistributedHashTable;
use std::time::Duration;

#[test]
fn test_create_empty_table() {
    let table = DistributedHashTable::new();
    assert_eq!(table.size(), 0);
    assert!(table.is_empty());
}

#[test]
fn test_insert_and_get() {
    let mut table = DistributedHashTable::new();
    let key = "test_key";
    let value = "test_value";
    
    table.insert(key, value);
    assert_eq!(table.size(), 1);
    assert!(!table.is_empty());
    
    let retrieved = table.get(key).unwrap();
    assert_eq!(retrieved, value);
}

#[test]
fn test_insert_with_ttl() {
    let mut table = DistributedHashTable::new();
    let key = "test_key";
    let value = "test_value";
    let ttl = Duration::from_millis(50);
    
    table.insert_with_ttl(key, value, ttl);
    assert_eq!(table.size(), 1);
    
    // Verifica se o valor está disponível antes do TTL expirar
    assert_eq!(table.get(key).unwrap(), value);
    
    // Espera o TTL expirar
    std::thread::sleep(Duration::from_millis(100));
    
    // Verifica se o valor foi removido após o TTL expirar
    assert!(table.get(key).is_none());
    assert!(table.is_empty());
}

#[test]
fn test_remove() {
    let mut table = DistributedHashTable::new();
    let key = "test_key";
    let value = "test_value";
    
    table.insert(key, value);
    assert_eq!(table.size(), 1);
    
    let removed = table.remove(key);
    assert_eq!(removed, Some(value.to_string()));
    assert!(table.is_empty());
    
    // Tentar remover uma chave que não existe
    let removed = table.remove("non_existent");
    assert!(removed.is_none());
}

#[test]
fn test_update_existing() {
    let mut table = DistributedHashTable::new();
    let key = "test_key";
    let value1 = "value1";
    let value2 = "value2";
    
    table.insert(key, value1);
    assert_eq!(table.get(key).unwrap(), value1);
    
    table.insert(key, value2);
    assert_eq!(table.get(key).unwrap(), value2);
    assert_eq!(table.size(), 1); // Tamanho não deve mudar ao atualizar
}

#[test]
fn test_clear() {
    let mut table = DistributedHashTable::new();
    
    table.insert("key1", "value1");
    table.insert("key2", "value2");
    assert_eq!(table.size(), 2);
    
    table.clear();
    assert!(table.is_empty());
    assert_eq!(table.size(), 0);
}

#[test]
fn test_contains_key() {
    let mut table = DistributedHashTable::new();
    let key = "test_key";
    let value = "test_value";
    
    assert!(!table.contains_key(key));
    
    table.insert(key, value);
    assert!(table.contains_key(key));
    
    table.remove(key);
    assert!(!table.contains_key(key));
}

#[test]
fn test_keys() {
    let mut table = DistributedHashTable::new();
    
    table.insert("key1", "value1");
    table.insert("key2", "value2");
    
    let keys: Vec<_> = table.keys().collect();
    assert_eq!(keys.len(), 2);
    assert!(keys.contains(&&"key1".to_string()));
    assert!(keys.contains(&&"key2".to_string()));
}

#[test]
fn test_values() {
    let mut table = DistributedHashTable::new();
    
    table.insert("key1", "value1");
    table.insert("key2", "value2");
    
    let values: Vec<_> = table.values().collect();
    assert_eq!(values.len(), 2);
    assert!(values.contains(&&"value1".to_string()));
    assert!(values.contains(&&"value2".to_string()));
} 