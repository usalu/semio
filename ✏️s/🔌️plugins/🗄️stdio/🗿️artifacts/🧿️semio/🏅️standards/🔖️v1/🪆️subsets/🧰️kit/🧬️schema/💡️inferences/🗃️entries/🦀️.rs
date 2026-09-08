//! 🗃 `entries` — one named inference: the kit catalog's own census. `objects`/`models`/
//! `properties` are owned CHILD slots (handles only — never embedded content, per this composite
//! subset's own module doc comment), `representations` is a LINK slot; none of the four are
//! honestly resolvable from THIS snapshot alone (resolving a handle is a cross-artifact read, out
//! of scope for a pure snapshot->inference fold), so the honest inference here is a real fold over
//! what IS owned outright: `types`/`designs` (including every design's nested `pieces`/
//! `connections`) plus a plain count/presence read of the four handle slots.

use crate::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Entries
/// 🗃️ Semio kit catalog census.
#[derive(Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemioKitEntries {
    pub type_count: u32,
    pub design_count: u32,
    /// 🧩️ Total pieces across EVERY design (a real fold, not a length read of `designs` itself).
    pub piece_count: u32,
    /// 🔌️ Total connections across every design.
    pub connection_count: u32,
    pub object_count: u32,
    pub model_count: u32,
    pub has_properties: bool,
    pub representation_count: u32,
}

/// 🗃️ Computes [`SemioKitEntries`] — pure, total, O(types + designs + pieces + connections +
/// objects + models + representations).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn compute_semio_kit_entries(snapshot: &SemioKitSnapshot) -> SemioKitEntries {
    let piece_count = snapshot.designs.iter().map(|d| d.pieces.len() as u32).sum();
    let connection_count = snapshot.designs.iter().map(|d| d.connections.len() as u32).sum();
    SemioKitEntries {
        type_count: snapshot.types.len() as u32,
        design_count: snapshot.designs.len() as u32,
        piece_count,
        connection_count,
        object_count: snapshot.objects.len() as u32,
        model_count: snapshot.models.len() as u32,
        has_properties: snapshot.properties.is_some(),
        representation_count: snapshot.representations.len() as u32,
    }
}
//#endregion 🔖️Entries

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
