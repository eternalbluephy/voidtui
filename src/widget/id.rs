use std::sync::atomic::{AtomicUsize, Ordering};

/// A unique identifier that auto-increments on creation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ID(usize);

impl ID {
    /// Creates a new unique ID
    pub fn new() -> Self {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        ID(COUNTER.fetch_add(1, Ordering::Relaxed))
    }

    /// Returns the raw value of the ID
    pub fn value(&self) -> usize {
        self.0
    }
}

impl Default for ID {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_increment() {
        let id1 = ID::new();
        let id2 = ID::new();
        let id3 = ID::new();
        
        assert_ne!(id1, id2);
        assert_ne!(id2, id3);
        assert!(id2.value() > id1.value());
        assert!(id3.value() > id2.value());
    }
}