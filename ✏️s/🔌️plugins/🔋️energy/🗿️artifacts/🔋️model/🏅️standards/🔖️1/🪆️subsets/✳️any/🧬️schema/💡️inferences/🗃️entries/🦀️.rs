//! 🗃 `entries` — one named inference: a census over the working-scene `Model` behind a snapshot's
//! composed `structure`/`zones` children (real top-level `Model`-field count, real JSON byte size,
//! real content digest). Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM: the old opaque
//! `model_json` field is gone — since a composed child slot can only ever hold a real, typed
//! `Model` (never arbitrary/malformed text), this leaf now census over
//! `crate::energy_model(snapshot)`'s own first-party `pack::json` serialization
//! (`Model` derives `ToValue`), which is ALWAYS a full JSON object (`Model` derives `Default`,
//! every field always present) rather than treating the body as an opaque, possibly-malformed byte
//! string.

use crate::EnergyModelSnapshot;
use semio_framework_os_kernel::{DslValue, ToValue};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

//#region 🔖️Entries
/// 🗃️ Census of the working-scene `Model` behind a snapshot's composed children.
#[derive(Clone, Debug, Default, PartialEq, ToValueDerive, FromValueDerive)]
#[value(rename_all = "camelCase")]
pub struct EnergyModelEntries {
    pub entry_count: u32,
    pub byte_size: u32,
    pub content_digest: String,
}

/// 🗃️ `entryCount` = number of top-level `Model` fields (its own `DslValue::Object` always has
/// one key per field — `Model` derives `Default`, never a partial object); `byteSize` = real UTF-8
/// byte length of that JSON; `contentDigest` = a deterministic (within-process) fingerprint over
/// those same bytes. Std-only (`DefaultHasher`), same reasoning as `🏠️home/🆔digest`: no external
/// hash crate needed for a single scalar byte-string digest.
pub fn compute_energy_model_entries(snapshot: &EnergyModelSnapshot) -> EnergyModelEntries {
    let model = crate::energy_model(snapshot);
    let json = pack::json::to_json_string(&model);
    let bytes = json.as_bytes();
    let entry_count = match model.to_value() {
        DslValue::Object(entries) => entries.len() as u32,
        _ => 0,
    };
    let mut hasher = DefaultHasher::new();
    bytes.hash(&mut hasher);
    EnergyModelEntries { entry_count, byte_size: bytes.len() as u32, content_digest: format!("{:016x}", hasher.finish()) }
}
//#endregion 🔖️Entries

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
