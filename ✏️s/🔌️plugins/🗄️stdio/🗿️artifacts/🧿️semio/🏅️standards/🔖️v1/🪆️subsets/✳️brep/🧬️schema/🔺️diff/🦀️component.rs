//! 🔺️ SemioBrepDiff — 🚧 scaffolded by W1b: a minimal, genuinely law-abiding full-replace
//! diff (between/apply/inverse/absorb all tested below). W2 replaces this with a sparse
//! per-field diff following the bcf/docx `enc_named_triple`/`enc_indexed_triple` pattern (see
//! `crate::artifacts::semio::standards::v1::engine::triples` for the shared generic codec).

use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use protocol::MutationDiff;
use protocol::command::DiffAlgebra;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemioBrepDiff {
    /// 🚧 Full-snapshot replacement slot — `None` = no change. W2 replaces this with sparse
    /// per-field triples (see module doc comment).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replacement: Option<SemioBrepSnapshot>,
}

impl MutationDiff<SemioBrepSnapshot> for SemioBrepDiff {
    fn apply(&self, base: &SemioBrepSnapshot) -> SemioBrepSnapshot {
        self.replacement.clone().unwrap_or_else(|| base.clone())
    }
    fn absorb(&mut self, other: Self) {
        if let Some(r) = other.replacement {
            self.replacement = Some(r);
        }
    }
}

impl DiffAlgebra<SemioBrepSnapshot> for SemioBrepDiff {
    fn between(base: &SemioBrepSnapshot, other: &SemioBrepSnapshot) -> Self {
        if base == other { Self { replacement: None } } else { Self { replacement: Some(other.clone()) } }
    }
    fn inverse(&self, base: &SemioBrepSnapshot) -> Self {
        Self { replacement: Some(base.clone()) }
    }
    fn is_empty(&self) -> bool {
        self.replacement.is_none()
    }
}

/// 🧩 Set-snapshot diff helper — used by the `📄set-snapshot/🔺️diff` leaf.
pub fn diff_set_snapshot(base: &SemioBrepSnapshot, snapshot: &SemioBrepSnapshot) -> SemioBrepDiff {
    <SemioBrepDiff as DiffAlgebra<SemioBrepSnapshot>>::between(base, snapshot)
}
//#endregion 🔖️Diff

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// 🧪️ field_sweep law (script.ts policy `field-sweep-presence`): between(a,b).apply(a)==b
    /// across every currently-mutable field — today that's `schema` alone (the snapshot's only
    /// scaffold-era scalar); W2 widens this test as real fields land.
    #[test]
    fn field_sweep_full_replace_round_trip() {
        let a = SemioBrepSnapshot::default();
        let mut b = SemioBrepSnapshot::default();
        b.schema = format!("{}-swept", a.schema);
        let d = <SemioBrepDiff as DiffAlgebra<SemioBrepSnapshot>>::between(&a, &b);
        assert_eq!(d.apply(&a), b);
        let inv = d.inverse(&a);
        assert_eq!(inv.apply(&d.apply(&a)), a);
        assert!(<SemioBrepDiff as DiffAlgebra<SemioBrepSnapshot>>::between(&a, &a).is_empty());
    }

    #[test]
    fn absorb_composes_two_sequential_diffs() {
        let a = SemioBrepSnapshot::default();
        let mut mid = SemioBrepSnapshot::default();
        mid.schema = format!("{}-mid", a.schema);
        let mut after = SemioBrepSnapshot::default();
        after.schema = format!("{}-after", a.schema);
        let mut d1 = <SemioBrepDiff as DiffAlgebra<SemioBrepSnapshot>>::between(&a, &mid);
        let d2 = <SemioBrepDiff as DiffAlgebra<SemioBrepSnapshot>>::between(&mid, &after);
        let applied_before_absorb = d1.apply(&a);
        d1.absorb(d2.clone());
        assert_eq!(d1.apply(&a), d2.apply(&applied_before_absorb));
        assert_eq!(d1.apply(&a), after);
    }
}
//#endregion 🔖️Tests
