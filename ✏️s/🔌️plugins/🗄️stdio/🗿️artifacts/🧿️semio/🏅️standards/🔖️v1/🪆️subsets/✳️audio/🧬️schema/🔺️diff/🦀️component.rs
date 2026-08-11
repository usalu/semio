//! 🔺️ SemioAudioDiff — 🚧 scaffolded by W1b: a minimal, genuinely law-abiding full-replace
//! diff (between/apply/inverse/absorb all tested below). W2 replaces this with a sparse
//! per-field diff following the bcf/docx `enc_named_triple`/`enc_indexed_triple` pattern (see
//! `crate::artifacts::semio::standards::v1::engine::triples` for the shared generic codec).

use crate::artifacts::semio::standards::v1::subsets::audio::schema::snapshot::SemioAudioSnapshot;
use protocol::MutationDiff;
use protocol::command::DiffAlgebra;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemioAudioDiff {
    /// 🚧 Full-snapshot replacement slot — `None` = no change. W2 replaces this with sparse
    /// per-field triples (see module doc comment).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replacement: Option<SemioAudioSnapshot>,
}

impl MutationDiff<SemioAudioSnapshot> for SemioAudioDiff {
    fn apply(&self, base: &SemioAudioSnapshot) -> SemioAudioSnapshot {
        self.replacement.clone().unwrap_or_else(|| base.clone())
    }
    fn absorb(&mut self, other: Self) {
        if let Some(r) = other.replacement {
            self.replacement = Some(r);
        }
    }
}

impl DiffAlgebra<SemioAudioSnapshot> for SemioAudioDiff {
    fn between(base: &SemioAudioSnapshot, other: &SemioAudioSnapshot) -> Self {
        if base == other { Self { replacement: None } } else { Self { replacement: Some(other.clone()) } }
    }
    fn inverse(&self, base: &SemioAudioSnapshot) -> Self {
        Self { replacement: Some(base.clone()) }
    }
    fn is_empty(&self) -> bool {
        self.replacement.is_none()
    }
}

/// 🧩 Set-snapshot diff helper — used by the `📄set-snapshot/🔺️diff` leaf.
pub fn diff_set_snapshot(base: &SemioAudioSnapshot, snapshot: &SemioAudioSnapshot) -> SemioAudioDiff {
    <SemioAudioDiff as DiffAlgebra<SemioAudioSnapshot>>::between(base, snapshot)
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
        let a = SemioAudioSnapshot::default();
        let mut b = SemioAudioSnapshot::default();
        b.schema = format!("{}-swept", a.schema);
        let d = <SemioAudioDiff as DiffAlgebra<SemioAudioSnapshot>>::between(&a, &b);
        assert_eq!(d.apply(&a), b);
        let inv = d.inverse(&a);
        assert_eq!(inv.apply(&d.apply(&a)), a);
        assert!(<SemioAudioDiff as DiffAlgebra<SemioAudioSnapshot>>::between(&a, &a).is_empty());
    }

    #[test]
    fn absorb_composes_two_sequential_diffs() {
        let a = SemioAudioSnapshot::default();
        let mut mid = SemioAudioSnapshot::default();
        mid.schema = format!("{}-mid", a.schema);
        let mut after = SemioAudioSnapshot::default();
        after.schema = format!("{}-after", a.schema);
        let mut d1 = <SemioAudioDiff as DiffAlgebra<SemioAudioSnapshot>>::between(&a, &mid);
        let d2 = <SemioAudioDiff as DiffAlgebra<SemioAudioSnapshot>>::between(&mid, &after);
        let applied_before_absorb = d1.apply(&a);
        d1.absorb(d2.clone());
        assert_eq!(d1.apply(&a), d2.apply(&applied_before_absorb));
        assert_eq!(d1.apply(&a), after);
    }
}
//#endregion 🔖️Tests
