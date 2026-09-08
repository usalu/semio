//! 🏷️ `kind` — one named inference: which of the 18 wrapped domain subsets this envelope
//! dispatches to, as the same `(tag, ordinal)` pair `📸️snapshot/🦀️.rs`'s own DSL header
//! line and binary pack header already compute from `SemioSubsetSnapshot` — reused via
//! `subset_tag`/`subset_ordinal`, not re-derived, so this facet can never drift from the real
//! wire dispatch. A pure O(1) tag read off an already-decoded enum discriminant — no
//! `InferredField` needed.

use crate::standards::v1::subsets::base::schema::snapshot::{subset_ordinal, subset_tag, SemioSnapshot};

//#region 🔖️Kind
/// 🏷️ The envelope's wrapped-subset dispatch tag/ordinal.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemioKind {
    pub tag: String,
    pub ordinal: u32,
}

/// 🩹 Hand-rolled: `SemioSubsetSnapshot::default()` is `Brep(..)` (the first-declared variant,
/// per its own hand-written `impl Default`), so the honest default `kind` is `("brep", 0)`, not a
/// derive-produced empty string / zero-ordinal-by-coincidence — this makes that agreement
/// explicit rather than relying on `subset_tag`/`subset_ordinal`'s enum-declaration-order
/// happening to put `Brep` first.
impl Default for SemioKind {
    fn default() -> Self {
        Self { tag: "brep".into(), ordinal: 0 }
    }
}

/// 🏷️ Computes [`SemioKind`] via the envelope's own `subset_tag`/`subset_ordinal` dispatch —
/// pure, total, O(1).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn compute_semio_kind(snapshot: &SemioSnapshot) -> SemioKind {
    SemioKind { tag: subset_tag(&snapshot.subset).to_string(), ordinal: subset_ordinal(&snapshot.subset) as u32 }
}
//#endregion 🔖️Kind

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
