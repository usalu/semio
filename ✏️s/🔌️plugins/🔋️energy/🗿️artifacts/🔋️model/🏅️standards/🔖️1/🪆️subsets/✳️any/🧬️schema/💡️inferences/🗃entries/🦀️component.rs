//! 🗃 `entries` — one named inference: an opaque-container census of the persisted `model_json`
//! body (real top-level JSON key count, real byte size, real content digest). `model_json` is not
//! guaranteed to decode into the typed `crate::model::Model` for every persisted snapshot (the
//! default snapshot's `"{}"` has none of `Model`'s required fields), so this leaf never assumes a
//! successful `Model` decode — it treats the field as an opaque JSON body, the same honest
//! treatment an archive/container facet gives its own opaque payload.

use crate::artifacts::model::EnergyModelSnapshot;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

//#region 🔖️Entries
/// 🗃️ Opaque-container census of `model_json`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnergyModelEntries {
    pub entry_count: u32,
    pub byte_size: u32,
    pub content_digest: String,
}

/// 🗃️ `entryCount` = number of top-level keys when `model_json` parses as a JSON object (`0`
/// otherwise — malformed/non-object bodies still get a valid, deterministic census rather than an
/// error); `byteSize` = real UTF-8 byte length; `contentDigest` = a deterministic (within-process)
/// fingerprint over those same bytes. Std-only (`DefaultHasher`), same reasoning as
/// `🏠️home/🆔digest`: no external hash crate needed for a single scalar byte-string digest.
pub fn compute_energy_model_entries(snapshot: &EnergyModelSnapshot) -> EnergyModelEntries {
    let bytes = snapshot.model_json.as_bytes();
    let entry_count = match serde_json::from_str::<serde_json::Value>(&snapshot.model_json) {
        Ok(serde_json::Value::Object(map)) => map.len() as u32,
        _ => 0,
    };
    let mut hasher = DefaultHasher::new();
    bytes.hash(&mut hasher);
    EnergyModelEntries { entry_count, byte_size: bytes.len() as u32, content_digest: format!("{:016x}", hasher.finish()) }
}
//#endregion 🔖️Entries

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;

    #[test]
    fn default_model_json_yields_zero_entries_and_a_real_byte_size() {
        let entries = compute_energy_model_entries(&EnergyModelSnapshot::default());
        assert_eq!(entries.entry_count, 0);
        assert_eq!(entries.byte_size, "{}".len() as u32);
    }

    #[test]
    fn top_level_keys_are_counted_exactly() {
        let snapshot = EnergyModelSnapshot { model_json: r#"{"a":1,"b":2,"c":3}"#.into(), ..EnergyModelSnapshot::default() };
        let entries = compute_energy_model_entries(&snapshot);
        assert_eq!(entries.entry_count, 3);
    }

    #[test]
    fn malformed_json_still_yields_a_deterministic_census() {
        let snapshot = EnergyModelSnapshot { model_json: "not json".into(), ..EnergyModelSnapshot::default() };
        let entries = compute_energy_model_entries(&snapshot);
        assert_eq!(entries.entry_count, 0);
        assert_eq!(entries, compute_energy_model_entries(&snapshot));
    }

    #[test]
    fn different_bodies_yield_different_digests() {
        let a = EnergyModelSnapshot { model_json: "{}".into(), ..EnergyModelSnapshot::default() };
        let b = EnergyModelSnapshot { model_json: r#"{"x":1}"#.into(), ..EnergyModelSnapshot::default() };
        assert_ne!(compute_energy_model_entries(&a).content_digest, compute_energy_model_entries(&b).content_digest);
    }
}
//#endregion 🧪️Tests
