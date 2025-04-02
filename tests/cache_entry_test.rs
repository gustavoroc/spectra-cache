use spectra_cache::CacheEntry;
use std::time::Duration;
use std::thread::sleep;

// Criamos uma struct falsa CacheEntry para que o compilador não recuse o teste
// mas o teste ainda vai falhar, pois esta implementação não tem a funcionalidade real
struct CacheEntry {
    key: &'static str,
    value: &'static str,
}

impl CacheEntry {
    fn new(key: &'static str, value: &'static str) -> Self {
        Self { key, value }
    }
    
    fn with_ttl(key: &'static str, value: &'static str, _ttl: Duration) -> Self {
        Self { key, value }
    }
    
    fn key(&self) -> &'static str {
        self.key
    }
    
    fn value(&self) -> &'static str {
        self.value
    }
    
    fn ttl(&self) -> Option<Duration> {
        None
    }
    
    fn is_expired(&self) -> bool {
        false
    }
    
    fn last_accessed_at(&self) -> Duration {
        Duration::from_secs(0)
    }
    
    fn touch(&mut self) {
    }
    
    fn update_value(&mut self, _new_value: &'static str) {
    }
    
    fn age(&self) -> Duration {
        Duration::from_secs(0)
    }
    
    fn idle_time(&self) -> Duration {
        Duration::from_secs(0)
    }
}

#[test]
fn test_create_new_cache_entry() {
    let key = "test_key";
    let value = "test_value";
    
    let entry = CacheEntry::new(key, value);
    
    assert_eq!(entry.key(), key);
    assert_eq!(entry.value(), value);
    assert!(entry.ttl().is_none());
    assert!(!entry.is_expired());
}

#[test]
fn test_cache_entry_with_ttl() {
    let key = "test_key";
    let value = "test_value";
    let ttl = Duration::from_millis(50);
    
    let entry = CacheEntry::with_ttl(key, value, ttl);
    
    assert_eq!(entry.ttl(), Some(ttl));
    assert!(!entry.is_expired());
    
    sleep(Duration::from_millis(100));
    assert!(entry.is_expired());
}

#[test]
fn test_touch_updates_last_accessed() {
    let key = "test_key";
    let value = "test_value";
    let mut entry = CacheEntry::new(key, value);
    let initial_access_time = entry.last_accessed_at();
    
    sleep(Duration::from_millis(10));
    entry.touch();
    
    assert!(entry.last_accessed_at() < initial_access_time);
}

#[test]
fn test_update_value() {
    let key = "test_key";
    let value1 = "value1";
    let value2 = "value2";
    let mut entry = CacheEntry::new(key, value1);
    
    entry.update_value(value2);
    
    assert_eq!(entry.value(), value2);
}

#[test]
fn test_age_and_idle_time() {
    let key = "test_key";
    let value = "test_value";
    let mut entry = CacheEntry::new(key, value);
    
    sleep(Duration::from_millis(20));
    let age = entry.age();
    let idle_time = entry.idle_time();
    
    assert!(age.as_millis() >= 20);
    assert!(idle_time.as_millis() >= 20);
    
    entry.touch();
    assert!(entry.idle_time() < idle_time);
    assert!(entry.age() >= age);
} 