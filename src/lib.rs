// Este arquivo está vazio de propósito.
// Estamos começando com os testes primeiro, seguindo TDD. 

use std::time::{Duration, Instant};

/// Represents a single entry in the cache with metadata about its lifecycle
pub struct CacheEntry {
    key: String,
    value: String,
    ttl: Option<Duration>,
    created_at: Instant,
    last_accessed_at: Instant,
}

impl CacheEntry {
    /// Creates a new cache entry without TTL
    pub fn new(key: &str, value: &str) -> Self {
        let now = Instant::now();
        Self {
            key: key.to_string(),
            value: value.to_string(),
            ttl: None,
            created_at: now,
            last_accessed_at: now,
        }
    }
    
    /// Creates a new cache entry with TTL
    pub fn with_ttl(key: &str, value: &str, ttl: Duration) -> Self {
        let now = Instant::now();
        Self {
            key: key.to_string(),
            value: value.to_string(),
            ttl: Some(ttl),
            created_at: now,
            last_accessed_at: now,
        }
    }
    
    /// Returns the key of the cache entry
    pub fn key(&self) -> &str {
        &self.key
    }
    
    /// Returns the value of the cache entry
    pub fn value(&self) -> &str {
        &self.value
    }
    
    /// Returns the TTL if set
    pub fn ttl(&self) -> Option<Duration> {
        self.ttl
    }
    
    /// Checks if the entry has expired based on its TTL
    pub fn is_expired(&self) -> bool {
        if let Some(ttl) = self.ttl {
            self.age() > ttl
        } else {
            false
        }
    }
    
    /// Returns the last time this entry was accessed
    pub fn last_accessed_at(&self) -> Duration {
        self.last_accessed_at.elapsed()
    }
    
    /// Updates the last accessed time to now
    pub fn touch(&mut self) {
        self.last_accessed_at = Instant::now();
    }
    
    /// Updates the value of the cache entry
    pub fn update_value(&mut self, new_value: &str) {
        self.value = new_value.to_string();
        self.touch(); // Atualiza o último acesso ao modificar o valor
    }
    
    /// Returns how long this entry has been in the cache
    pub fn age(&self) -> Duration {
        self.created_at.elapsed()
    }
    
    /// Returns how long this entry has been idle (not accessed)
    pub fn idle_time(&self) -> Duration {
        self.last_accessed_at.elapsed()
    }
} 