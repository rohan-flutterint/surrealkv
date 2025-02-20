use std::path::PathBuf;

use revision::revisioned;

#[revisioned(revision = 1)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum IsolationLevel {
    SnapshotIsolation,
    SerializableSnapshotIsolation,
}

impl IsolationLevel {
    pub fn from_u64(value: u64) -> Option<Self> {
        match value {
            0 => Some(IsolationLevel::SnapshotIsolation),
            1 => Some(IsolationLevel::SerializableSnapshotIsolation),
            _ => None,
        }
    }
}

#[revisioned(revision = 4)]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Options {
    // Required options.
    pub dir: PathBuf, // Directory path for storing the database files.

    // Usually modified options.
    pub isolation_level: IsolationLevel, // Isolation level for transactions.

    // Deprecated field.
    #[revision(end = 2, convert_fn = "convert_max_key_size")]
    max_key_size: u64,
    // Deprecated field.
    #[revision(end = 2, convert_fn = "convert_max_value_size")]
    max_value_size: u64,

    // Fine tuning options.
    pub max_value_threshold: usize, // Threshold to decide value should be stored and read from memory or from log value files.
    pub max_segment_size: u64,      // Maximum size of a single segment.
    pub max_value_cache_size: u64,  // Maximum size of the value cache.
    pub max_compaction_segment_size: u64, // Maximum size of a single compaction.

    // Field to indicate whether the data should be stored completely in memory
    pub disk_persistence: bool, // If false, data will be stored completely in memory. If true, data will be stored on disk too.

    // Used to enable or disable versioned values.
    #[revision(start = 3)]
    pub enable_versions: bool,

    // Field to indicate whether the values should be cached during writes instead of reads.
    #[revision(start = 4)]
    pub cache_on_write: bool,
}

impl Default for Options {
    /// Creates a new set of options with default values.
    fn default() -> Self {
        Self {
            dir: PathBuf::from(""),
            max_value_threshold: 64, // 64 bytes
            isolation_level: IsolationLevel::SnapshotIsolation,
            max_segment_size: 1 << 29, // 512 MB
            max_value_cache_size: 100000,
            disk_persistence: true,
            max_compaction_segment_size: 1 << 30, // 1 GB
            enable_versions: true,
            cache_on_write: false,
        }
    }
}

impl Options {
    /// Creates a new set of options with default values.
    pub fn new() -> Self {
        Self::default()
    }
    /// Returns true if the data should be persisted on disk.
    pub fn should_persist_data(&self) -> bool {
        self.disk_persistence
    }

    fn convert_max_key_size(&self, _revision: u16, _value: u64) -> Result<(), revision::Error> {
        // We don't use this deprecated field anymore so let's completely ignore it.
        Ok(())
    }

    fn convert_max_value_size(&self, _revision: u16, _value: u64) -> Result<(), revision::Error> {
        // We don't use this deprecated field anymore so let's completely ignore it.
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn default_options() {
        let options = Options::default();

        assert_eq!(options.dir, PathBuf::from(""));
        assert_eq!(options.max_value_threshold, 64);
        assert_eq!(options.isolation_level, IsolationLevel::SnapshotIsolation);
        assert_eq!(options.max_segment_size, 1 << 29);
        assert_eq!(options.max_value_cache_size, 100000);
        assert_eq!(options.max_compaction_segment_size, 1 << 30);
        assert!(options.disk_persistence);
        assert!(options.enable_versions);
        assert!(!options.cache_on_write);
    }
}
