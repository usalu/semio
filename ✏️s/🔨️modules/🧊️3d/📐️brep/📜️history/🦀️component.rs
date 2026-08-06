//! 📜️ Operation provenance: a [`PersistentLabel`] assigned once at an entity's birth and never
//! reused, plus the [`OpDelta`] every mutating operation in [`crate::brep::euler`] returns. This is what
//! lets the document layer (a later phase) build exact `generated`/`modified`/`deleted` maps for
//! `backwards()` inversion without needing to diff whole-body snapshots.

// #region 🔖️Labels

/// 📜️ A stable identity for one topological entity, assigned from a per-`Body` monotonically
/// increasing counter at birth. Unlike an arena [`crate::brep::arena::ArenaId`] (which can be reused
/// after removal once its generation increments), a label is never reused — it survives arena
/// compaction and is the identity the document layer's persistent naming keys off of.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct PersistentLabel(pub u64);

/// 📜️ Issues fresh, never-repeating labels for one `Body`.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct LabelSource {
    next: u64,
}

impl LabelSource {
    pub fn new() -> Self {
        LabelSource { next: 0 }
    }
    pub fn next_label(&mut self) -> PersistentLabel {
        let label = PersistentLabel(self.next);
        self.next += 1;
        label
    }
}

// #endregion 🔖️Labels

// #region 🔖️Delta

/// 📜️ The provenance of one mutating operation, in terms of stable [`PersistentLabel`]s rather
/// than arena ids (which can be reused after removal): every entity the operation created, every
/// entity it modified (paired with its label so the same entity's before/after states are
/// linkable), and every entity it deleted.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OpDelta {
    pub generated: Vec<PersistentLabel>,
    pub modified: Vec<PersistentLabel>,
    pub deleted: Vec<PersistentLabel>,
}

impl OpDelta {
    pub fn is_empty(&self) -> bool {
        self.generated.is_empty() && self.modified.is_empty() && self.deleted.is_empty()
    }
    pub fn merge(&mut self, other: OpDelta) {
        self.generated.extend(other.generated);
        self.modified.extend(other.modified);
        self.deleted.extend(other.deleted);
    }
}

/// 📜️ Accumulates an [`OpDelta`] as a checked editor runs; passed by every [`crate::brep::euler`]
/// operator so no operation can forget to log what it touched. `record_deleted` and friends are
/// idempotent against duplicate reporting within one operation, since some editors touch the same
/// entity more than once (e.g. splitting an edge modifies the vertex on both sides).
#[derive(Clone, Debug, Default)]
pub struct OpRecorder {
    delta: OpDelta,
}

impl OpRecorder {
    pub fn new() -> Self {
        OpRecorder::default()
    }
    pub fn record_generated(&mut self, label: PersistentLabel) {
        if !self.delta.generated.contains(&label) {
            self.delta.generated.push(label);
        }
    }
    pub fn record_modified(&mut self, label: PersistentLabel) {
        if !self.delta.modified.contains(&label) && !self.delta.generated.contains(&label) {
            self.delta.modified.push(label);
        }
    }
    pub fn record_deleted(&mut self, label: PersistentLabel) {
        self.delta.generated.retain(|l| *l != label);
        self.delta.modified.retain(|l| *l != label);
        if !self.delta.deleted.contains(&label) {
            self.delta.deleted.push(label);
        }
    }
    pub fn into_delta(self) -> OpDelta {
        self.delta
    }
}

// #endregion 🔖️Delta

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn label_source_never_repeats() {
        let mut source = LabelSource::new();
        let a = source.next_label();
        let b = source.next_label();
        assert_ne!(a, b);
        assert_eq!(a.0, 0);
        assert_eq!(b.0, 1);
    }

    #[test]
    fn recorder_generated_then_deleted_cancels_out() {
        let mut rec = OpRecorder::new();
        let label = PersistentLabel(5);
        rec.record_generated(label);
        rec.record_deleted(label);
        let delta = rec.into_delta();
        assert!(delta.generated.is_empty());
        assert_eq!(delta.deleted, vec![label]);
    }

    #[test]
    fn recorder_generated_entity_is_not_also_reported_modified() {
        let mut rec = OpRecorder::new();
        let label = PersistentLabel(1);
        rec.record_generated(label);
        rec.record_modified(label);
        let delta = rec.into_delta();
        assert_eq!(delta.generated, vec![label]);
        assert!(delta.modified.is_empty());
    }

    #[test]
    fn recorder_deduplicates_repeated_reports() {
        let mut rec = OpRecorder::new();
        let label = PersistentLabel(2);
        rec.record_modified(label);
        rec.record_modified(label);
        let delta = rec.into_delta();
        assert_eq!(delta.modified.len(), 1);
    }

    #[test]
    fn op_delta_merge_concatenates_all_three_lists() {
        let mut a = OpDelta { generated: vec![PersistentLabel(1)], modified: vec![PersistentLabel(2)], deleted: vec![] };
        let b = OpDelta { generated: vec![], modified: vec![], deleted: vec![PersistentLabel(3)] };
        a.merge(b);
        assert_eq!(a.generated, vec![PersistentLabel(1)]);
        assert_eq!(a.modified, vec![PersistentLabel(2)]);
        assert_eq!(a.deleted, vec![PersistentLabel(3)]);
    }

    #[test]
    fn empty_delta_reports_is_empty() {
        assert!(OpDelta::default().is_empty());
        assert!(!OpDelta { generated: vec![PersistentLabel(0)], ..Default::default() }.is_empty());
    }
}
// #endregion 🔖️Tests
