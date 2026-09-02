//! 🏷️ `kind` — one named inference: which of the 18 wrapped domain subsets this envelope
//! dispatches to, as the same `(tag, ordinal)` pair `📸️snapshot/🦀️.rs`'s own DSL header
//! line and binary pack header already compute from `SemioSubsetSnapshot` — reused via
//! `subset_tag`/`subset_ordinal`, not re-derived, so this facet can never drift from the real
//! wire dispatch. A pure O(1) tag read off an already-decoded enum discriminant — no
//! `InferredField` needed.

use crate::artifacts::semio::standards::v1::subsets::any::schema::snapshot::{subset_ordinal, subset_tag, SemioSnapshot};

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
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::any::schema::snapshot::{SemioSubsetSnapshot, STDIO_SEMIO_DOCUMENT_SCHEMA};
    use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn snapshot(subset: SemioSubsetSnapshot) -> SemioSnapshot {
        SemioSnapshot { schema: STDIO_SEMIO_DOCUMENT_SCHEMA.into(), subset }
    }

    #[semio_framework_async_macros::async_test]
    async fn image_dispatches_to_its_own_tag_and_ordinal() {
        let kind = compute_semio_kind(&snapshot(SemioSubsetSnapshot::Image(SemioImageSnapshot::default())));
        assert_eq!(kind, SemioKind { tag: "image".into(), ordinal: 7 });
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_determinism_law() {
        let snap = snapshot(SemioSubsetSnapshot::Image(SemioImageSnapshot::default()));
        assert_eq!(compute_semio_kind(&snap), compute_semio_kind(&snap));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(compute_semio_kind(&SemioSnapshot::default()), SemioKind::default());
    }
}
//#endregion 🧪️Tests
