//! 🧩 `composition` — one named inference: which owned CHILD slots are present (`brep`/`mesh`/
//! `properties` — handles only, never embedded content per this subset's own module doc comment,
//! so presence is the honest limit of what a handle tells you) plus the object's own real
//! `transform.translation`, read directly (never a fabricated geometry bounding box — resolving a
//! child handle into its target snapshot is a cross-artifact read, out of scope for a pure
//! snapshot->inference fold).

use crate::standards::v1::subsets::base::schema::geometry::SemioPoint3;
use crate::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Composition
/// 🧩️ Semio object composition census.
#[derive(Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemioObjectComposition {
    pub has_brep: bool,
    pub has_mesh: bool,
    pub has_properties: bool,
    pub position: SemioPoint3,
}

/// 🧩️ Computes [`SemioObjectComposition`] — pure, total, O(1).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn compute_semio_object_composition(snapshot: &SemioObjectSnapshot) -> SemioObjectComposition {
    SemioObjectComposition { has_brep: snapshot.brep.is_some(), has_mesh: snapshot.mesh.is_some(), has_properties: snapshot.properties.is_some(), position: snapshot.transform.translation }
}
//#endregion 🔖️Composition

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
