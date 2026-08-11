//! 🧬️ CsvMutation — document mutation dispatch. Every variant's `diff()` is handcrafted
//! (constructs the sparse `CsvDiff` directly — apply-and-capture is banned); `inverse()` is
//! handcrafted per variant, index-aware, reading the pre-state it needs from `base`.

use crate::artifacts::csv::schema::diff::{
    diff_set_snapshot, CsvDiff, CsvFieldDiff, CsvRecordAdded, CsvRecordDiff, CsvRecordModified,
    CsvRecordsDiff,
};
use crate::artifacts::csv::schema::snapshot::CsvRecord;
use crate::artifacts::csv::CsvSnapshot;
use protocol::{Mutation, MutationDiff};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.csv`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum CsvMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: CsvSnapshot,
    },
    /// 📑 Toggles RFC 4180's optional header-row convention.
    SetHasHeader {
        has_header: bool,
    },
    /// ➕️ Inserts a whole record at `index` (clamped to the end on apply).
    InsertRecord {
        index: usize,
        record: CsvRecord,
    },
    /// ➖️ Removes the record at `index`.
    RemoveRecord {
        index: usize,
    },
    /// ✏️ Patches one field's value and quoted-retention flag in place.
    SetField {
        record_index: usize,
        field_index: usize,
        value: String,
        quoted: bool,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`: `let d = mutation.diff(&*snapshot); *snapshot =
/// d.apply(snapshot); d` — the diff is the single semantics source.
pub fn apply_csv_mutation(snapshot: &mut CsvSnapshot, mutation: &CsvMutation) -> CsvDiff {
    let diff = <CsvMutation as protocol::Mutation<CsvSnapshot>>::diff(mutation, snapshot);
    *snapshot = <CsvDiff as protocol::MutationDiff<CsvSnapshot>>::apply(&diff, snapshot);
    diff
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<CsvSnapshot> for CsvMutation {
    type Diff = CsvDiff;

    fn diff(&self, base: &CsvSnapshot) -> Self::Diff {
        match self {
            CsvMutation::NoMutation => CsvDiff::default(),
            CsvMutation::SetSnapshot { snapshot } => diff_set_snapshot(base, snapshot),
            CsvMutation::SetHasHeader { has_header } => {
                CsvDiff { has_header: Some(*has_header), records: None }
            }
            CsvMutation::InsertRecord { index, record } => CsvDiff {
                has_header: None,
                records: Some(CsvRecordsDiff {
                    removed: Vec::new(),
                    modified: Vec::new(),
                    added: vec![CsvRecordAdded { index: *index, record: record.clone() }],
                }),
            },
            CsvMutation::RemoveRecord { index } => CsvDiff {
                has_header: None,
                records: Some(CsvRecordsDiff {
                    removed: vec![*index],
                    modified: Vec::new(),
                    added: Vec::new(),
                }),
            },
            CsvMutation::SetField { record_index, field_index, value, quoted } => {
                let mut fields = vec![None; field_index + 1];
                fields[*field_index] = Some(CsvFieldDiff {
                    value: Some(value.clone()),
                    quoted: Some(*quoted),
                });
                CsvDiff {
                    has_header: None,
                    records: Some(CsvRecordsDiff {
                        removed: Vec::new(),
                        modified: vec![CsvRecordModified {
                            index: *record_index,
                            diff: CsvRecordDiff { fields: Some(fields) },
                        }],
                        added: Vec::new(),
                    }),
                }
            }
        }
    }

    fn inverse(&self, base: &CsvSnapshot) -> Vec<Self> {
        match self {
            CsvMutation::NoMutation => vec![CsvMutation::NoMutation],
            CsvMutation::SetSnapshot { .. } => {
                vec![CsvMutation::SetSnapshot { snapshot: base.clone() }]
            }
            CsvMutation::SetHasHeader { .. } => {
                vec![CsvMutation::SetHasHeader { has_header: base.has_header }]
            }
            CsvMutation::InsertRecord { index, .. } => {
                vec![CsvMutation::RemoveRecord { index: *index }]
            }
            CsvMutation::RemoveRecord { index } => match base.records.get(*index) {
                Some(record) => vec![CsvMutation::InsertRecord { index: *index, record: record.clone() }],
                None => vec![CsvMutation::NoMutation],
            },
            CsvMutation::SetField { record_index, field_index, .. } => {
                match base.records.get(*record_index).and_then(|r| r.fields.get(*field_index)) {
                    Some(field) => vec![CsvMutation::SetField {
                        record_index: *record_index,
                        field_index: *field_index,
                        value: field.value.clone(),
                        quoted: field.quoted,
                    }],
                    None => vec![CsvMutation::NoMutation],
                }
            }
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
impl protocol::OpText for CsvMutation {
    fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".into())
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line.trim()).map_err(|e| {
            store::TextError::new(format!("op parse: {e}"), dsl::TextSpan::at(1, 1))
        })
    }
}

impl protocol::OpBinary for CsvMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        serde_json::to_vec(self).map_err(|e| protocol::ProtocolError::Malformed {
            what: "op encode",
            offset: 0,
            detail: e.to_string(),
        })
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        serde_json::from_slice(bytes).map_err(|e| protocol::ProtocolError::Malformed {
            what: "op decode",
            offset: 0,
            detail: e.to_string(),
        })
    }
}
//#endregion OpCodecs

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::csv::schema::snapshot::CsvField;
    use protocol::command::DiffAlgebra;

    //#region 🔖️Fixtures
    fn field(value: &str, quoted: bool) -> CsvField {
        CsvField { value: value.into(), quoted }
    }
    fn record(fields: &[(&str, bool)]) -> CsvRecord {
        CsvRecord { fields: fields.iter().map(|(v, q)| field(v, *q)).collect() }
    }
    fn base_snapshot() -> CsvSnapshot {
        CsvSnapshot {
            schema: "stdio.csv".into(),
            has_header: true,
            records: vec![
                record(&[("name", false), ("note", true)]),
                record(&[("a", false), ("b", false)]),
                record(&[("x", false), ("y", false)]),
            ],
        }
    }
    //#endregion 🔖️Fixtures

    //#region 🔖️FieldSweepFixtures
    /// 🧬️ Canonical "differs in every mutable field" snapshot A: 3 records — one that will
    /// be removed, one that will be modified in every field, one untouched (so `sweep_b`'s
    /// added record has something stable to anchor its own index against).
    fn sweep_a() -> CsvSnapshot {
        CsvSnapshot {
            schema: "stdio.csv".into(),
            has_header: true,
            records: vec![
                record(&[("gone", false), ("also-gone", true)]),
                record(&[("old-a", false), ("old-b", true)]),
                record(&[("stable", false)]),
            ],
        }
    }
    /// 🧬️ Sweep B: `has_header` flips, record 0 is removed, record 1 (now index 0) is
    /// modified in every field (value AND quoted), record 2 (now index 1) is untouched, and
    /// a brand-new record is added at the end.
    fn sweep_b() -> CsvSnapshot {
        CsvSnapshot {
            schema: "stdio.csv".into(),
            has_header: false,
            records: vec![
                record(&[("new-a", true), ("new-b", false)]),
                record(&[("stable", false)]),
                record(&[("brand-new", true)]),
            ],
        }
    }
    //#endregion 🔖️FieldSweepFixtures

    //#region 🔖️MutationDiffLaw
    #[test]
    fn mutation_diff_law() {
        let base = base_snapshot();
        let variants = vec![
            CsvMutation::NoMutation,
            CsvMutation::SetSnapshot { snapshot: sweep_b() },
            CsvMutation::SetHasHeader { has_header: false },
            CsvMutation::InsertRecord { index: 1, record: record(&[("new", true)]) },
            CsvMutation::RemoveRecord { index: 0 },
            CsvMutation::SetField { record_index: 1, field_index: 0, value: "changed".into(), quoted: true },
        ];
        for m in variants {
            let diff = m.diff(&base);
            let expected = diff.apply(&base);

            let mut via_apply = base.clone();
            let returned_diff = apply_csv_mutation(&mut via_apply, &m);

            assert_eq!(via_apply, expected, "apply_csv_mutation mismatch for {m:?}");
            assert_eq!(returned_diff, diff, "returned diff mismatch for {m:?}");
        }
    }
    //#endregion 🔖️MutationDiffLaw

    //#region 🔖️InverseLaw
    #[test]
    fn inverse_law() {
        let base = base_snapshot();
        let variants = vec![
            CsvMutation::NoMutation,
            CsvMutation::SetSnapshot { snapshot: sweep_b() },
            CsvMutation::SetHasHeader { has_header: false },
            CsvMutation::InsertRecord { index: 1, record: record(&[("new", true)]) },
            CsvMutation::RemoveRecord { index: 0 },
            CsvMutation::SetField { record_index: 1, field_index: 0, value: "changed".into(), quoted: true },
        ];
        for m in variants {
            // 🔁️ mutation-level round trip
            let mut forward = base.clone();
            apply_csv_mutation(&mut forward, &m);
            for inv in m.inverse(&base) {
                apply_csv_mutation(&mut forward, &inv);
            }
            assert_eq!(forward, base, "mutation-level inverse round trip failed for {m:?}");

            // 🔁️ diff-level round trip
            let d = m.diff(&base);
            let mid = d.apply(&base);
            let back = d.inverse(&base).apply(&mid);
            assert_eq!(back, base, "diff-level inverse round trip failed for {m:?}");
        }
    }
    //#endregion 🔖️InverseLaw

    //#region 🔖️AbsorbLaw
    #[test]
    fn absorb_law() {
        let base = base_snapshot();

        // 🧩 Insert(2) + Remove(0): the two-op sequence base → mid → after.
        let d1 = CsvMutation::InsertRecord { index: 2, record: record(&[("ins", false)]) }.diff(&base);
        let mid = d1.apply(&base);
        let d2 = CsvMutation::RemoveRecord { index: 0 }.diff(&mid);
        let after = d2.apply(&mid);
        let mut composed = d1.clone();
        composed.absorb(d2.clone());
        assert_eq!(composed.apply(&base), after, "Insert+Remove-before absorb mismatch");

        // 🧩 Insert(2,f) + Insert(2,g): both must survive (fixes the old op-slot LWW bug).
        let d1 = CsvMutation::InsertRecord { index: 2, record: record(&[("f", false)]) }.diff(&base);
        let mid = d1.apply(&base);
        let d2 = CsvMutation::InsertRecord { index: 2, record: record(&[("g", false)]) }.diff(&mid);
        let after = d2.apply(&mid);
        let mut composed = d1.clone();
        composed.absorb(d2.clone());
        assert_eq!(composed.apply(&base), after, "Insert+Insert-same-index absorb mismatch");
        assert_eq!(after.records.len(), base.records.len() + 2, "both inserts must survive");

        // 🧩 Add + SetField: patch into the added payload.
        let d1 = CsvMutation::InsertRecord { index: 1, record: record(&[("orig", false)]) }.diff(&base);
        let mid = d1.apply(&base);
        let d2 = CsvMutation::SetField { record_index: 1, field_index: 0, value: "patched".into(), quoted: true }.diff(&mid);
        let after = d2.apply(&mid);
        let mut composed = d1.clone();
        composed.absorb(d2.clone());
        assert_eq!(composed.apply(&base), after, "Add+SetField absorb mismatch");
        assert_eq!(after.records[1].fields[0].value, "patched");

        // 🧩 Modify + Remove: modifying then removing the same record collapses to a removal.
        let d1 = CsvMutation::SetField { record_index: 1, field_index: 0, value: "will-vanish".into(), quoted: false }.diff(&base);
        let mid = d1.apply(&base);
        let d2 = CsvMutation::RemoveRecord { index: 1 }.diff(&mid);
        let after = d2.apply(&mid);
        let mut composed = d1.clone();
        composed.absorb(d2.clone());
        assert_eq!(composed.apply(&base), after, "Modify+Remove absorb mismatch");

        // 🧩 Associativity over a triple.
        let base = base_snapshot();
        let d1 = CsvMutation::InsertRecord { index: 0, record: record(&[("a", false)]) }.diff(&base);
        let s1 = d1.apply(&base);
        let d2 = CsvMutation::SetField { record_index: 0, field_index: 0, value: "a2".into(), quoted: true }.diff(&s1);
        let s2 = d2.apply(&s1);
        let d3 = CsvMutation::RemoveRecord { index: 2 }.diff(&s2);
        let s3 = d3.apply(&s2);

        let mut left = d1.clone();
        left.absorb(d2.clone());
        left.absorb(d3.clone());

        let mut d23 = d2.clone();
        d23.absorb(d3.clone());
        let mut right = d1.clone();
        right.absorb(d23);

        assert_eq!(left.apply(&base), s3);
        assert_eq!(right.apply(&base), s3);
        assert_eq!(left.apply(&base), right.apply(&base), "absorb must be associative");
    }
    //#endregion 🔖️AbsorbLaw

    //#region 🔖️BetweenRoundtripLaw
    #[test]
    fn between_roundtrip_law() {
        let a = base_snapshot();
        let b = sweep_b();
        assert_eq!(CsvDiff::between(&a, &b).apply(&a), b);
        assert_eq!(CsvDiff::between(&b, &a).apply(&b), a);

        // synthetic: differing field counts within an overlapping index (record replace path).
        let mut c = a.clone();
        c.records[0] = record(&[("only-one-field", false)]);
        assert_eq!(CsvDiff::between(&a, &c).apply(&a), c);
        assert_eq!(CsvDiff::between(&c, &a).apply(&c), a);

        assert!(CsvDiff::between(&a, &a).is_empty());
    }
    //#endregion 🔖️BetweenRoundtripLaw

    //#region 🔖️FieldSweep
    #[test]
    fn field_sweep_every_mutable_field_changes() {
        let a = sweep_a();
        let b = sweep_b();

        let d_ab = CsvDiff::between(&a, &b);
        assert_eq!(d_ab.apply(&a), b, "between(a,b).apply(a) == b");

        let d_ba = CsvDiff::between(&b, &a);
        assert_eq!(d_ba.apply(&b), a, "between(b,a).apply(b) == a");

        // 🔍 Hand-written per-field assertion: every field of `CsvDiff` is populated.
        assert!(d_ab.has_header.is_some(), "has_header must be populated");
        let records = d_ab.records.as_ref().expect("records diff must be populated");
        assert!(!records.removed.is_empty(), "removed must be non-empty (record 0 dropped)");
        assert!(!records.modified.is_empty(), "modified must be non-empty (record 1 changed in every field)");
        assert!(!records.added.is_empty(), "added must be non-empty (brand-new record)");
        let modified = &records.modified[0];
        let field_patches = modified.diff.fields.as_ref().expect("record 1's field patch list must be populated");
        assert!(field_patches.iter().all(|f| f.is_some()), "every field of the modified record must be patched");
        for patch in field_patches.iter().flatten() {
            assert!(patch.value.is_some() && patch.quoted.is_some(), "every field patch must set BOTH value and quoted");
        }

        assert!(CsvDiff::between(&a, &a).is_empty());
    }
    //#endregion 🔖️FieldSweep
}
//#endregion 🧪️Tests
