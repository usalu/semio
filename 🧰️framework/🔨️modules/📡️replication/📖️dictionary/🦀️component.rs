//! 📖 Deterministic string dictionary builder and reader.

use crate::ProtocolError;

//#region 🔖️Dictionary
/// @emoji 📚️ In-memory dictionary builder — deterministic first-use interning order — shared by
/// `protocol_history`'s `REC_ACTOR_DICT`/`REC_STR_DICT` codec and `protocol_format`'s dict-aware
/// frame helpers.
#[derive(Clone, Debug, Default)]
pub struct DictBuilder {
    entries: Vec<String>,
    index: std::collections::HashMap<String, u32>,
}

impl DictBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// @emoji ➕️ Returns `s`'s existing index, or appends it and returns the new index.
    pub fn intern(&mut self, s: &str) -> u32 {
        if let Some(&idx) = self.index.get(s) {
            return idx;
        }
        let idx = self.entries.len() as u32;
        self.entries.push(s.to_string());
        self.index.insert(s.to_string(), idx);
        idx
    }

    pub fn len(&self) -> u32 {
        self.entries.len() as u32
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// @emoji ✂️ The entries appended since `base_count` — the delta a `REC_*_DICT` record stores.
    pub fn entries_since(&self, base_count: u32) -> &[String] {
        &self.entries[base_count as usize..]
    }
}

/// @emoji 📖️ Read-side twin of `DictBuilder`: replays `REC_*_DICT` deltas in file order.
#[derive(Clone, Debug, Default)]
pub struct DictReader {
    entries: Vec<String>,
}

impl DictReader {
    pub fn new() -> Self {
        Self::default()
    }

    /// @emoji ➕️ Appends a dictionary delta. `base_count` must equal the reader's current length
    /// — a mismatch means the stream's dictionary deltas arrived out of order.
    pub fn extend(&mut self, base_count: u32, new_entries: impl IntoIterator<Item = String>) -> Result<(), ProtocolError> {
        let expected = self.entries.len() as u32;
        if base_count != expected {
            return Err(ProtocolError::DictOutOfOrder { expected, actual: base_count });
        }
        self.entries.extend(new_entries);
        Ok(())
    }

    pub fn resolve(&self, index: u32) -> Result<&str, ProtocolError> {
        self.entries.get(index as usize).map(String::as_str).ok_or(ProtocolError::DictMiss(index))
    }

    pub fn len(&self) -> u32 {
        self.entries.len() as u32
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
//#endregion 🔖️Dictionary
