//! 🗃 `entries` — one named inference: a census over the native `EnergyModelState` behind a snapshot's
//! `model` field (real top-level field count, real canonical JSON byte size, real content digest),
//! always a full JSON object rather than a possibly-malformed opaque body.

#[cfg(test)] use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Entries
/// 🗃️ Census of the native model state behind a snapshot.
#[derive(Clone, Debug, Default, PartialEq, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct EnergyModelEntries {
    pub entry_count: u32,
    pub byte_size: u32,
    pub content_digest: String,
}


//#endregion 🔖️Entries

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
