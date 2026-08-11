//! 🧬️ CsvMutation — document mutation dispatch. Every variant's `diff()` is handcrafted
//! (constructs the sparse `CsvDiff` directly — apply-and-capture is banned); `inverse()` is
//! handcrafted per variant, index-aware, reading the pre-state it needs from `base`.

use crate::artifacts::csv::schema::diff::{
    dec_record, dec_str, diff_set_snapshot, enc_record, enc_str, split_top_level, strip_brackets,
    CsvDiff, CsvFieldDiff, CsvRecordAdded, CsvRecordDiff, CsvRecordModified, CsvRecordsDiff,
};
use crate::artifacts::csv::schema::snapshot::CsvRecord;
use crate::artifacts::csv::CsvSnapshot;
use protocol::{Mutation, MutationDiff, OpText};
use serde::{Deserialize, Serialize};
#[cfg(test)]
use protocol::OpBinary;

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.csv`.
/// 🧪️ F6: `#[derive(dsl::DslOps)]` on this enum CANNOT be used — confirmed via a real `cargo
/// check` error, and NOT one of the recon report's documented §3a/§3b failure modes: it is a
/// genuine derive-macro hygiene bug. `InsertRecord`'s field is literally named `record`, and
/// `dsl_derive::dsl_variants_codegen`'s generated `to_named_arms` match-arm body shadows any
/// field bound by that same name with its own internal accumulator —
/// `let mut record = ::dsl::RecordValue::default();` — declared AFTER the match pattern destructures
/// the variant's fields. The subsequent `record.fields.insert(#id, ::dsl::DslField::to_value(record))`
/// statement for the `record` field then resolves `record` to the SHADOWING `RecordValue`, not the
/// `&CsvRecord` binding, giving: `error[E0308]: mismatched types … expected reference `&_`, found
/// struct `RecordValue`` at this variant's `record: CsvRecord` field (verified: renaming the field
/// to `csvrec` alone made the same derive attempt compile clean). Renaming the field back would fix
/// the derive but changes the Mutation enum's wire shape, which is out of scope here — `OpText`/
/// `OpBinary` hand-rolled below instead, reusing `CsvDiff`'s `pub(crate)` grammar primitives.
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
/// 🧪️ F6: **hand-rolled** `OpText`/`OpBinary` for `CsvMutation` (`#[derive(dsl::DslOps)]`
/// confirmed rejected above — a macro hygiene bug, not §3a/§3b) — reuses `CsvDiff`'s
/// `pub(crate)` grammar primitives (`hex`/`split_top_level`/`encode_option`/`enc_record`/...)
/// rather than duplicating them a second time in this file. Grammar: `keyword arg=value ...`
/// (space-separated), same convention gif89a's/svg's own hand-rolled `OpText` impls use, one
/// match arm per variant (no `DslVariants` scaffolding available since nothing here derives it).
fn enc_csv_snapshot(s: &CsvSnapshot) -> String {
    format!(
        "[{},{},[{}]]",
        enc_str(&s.schema),
        if s.has_header { 1 } else { 0 },
        s.records.iter().map(enc_record).collect::<Vec<_>>().join(","),
    )
}
fn dec_csv_snapshot(s: &str) -> Result<CsvSnapshot, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [schema, has_header, records] = parts.as_slice() else {
        return Err(format!("csv snapshot: expected 3 fields, got {}", parts.len()));
    };
    let records = split_top_level(strip_brackets(records)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(dec_record)
        .collect::<Result<Vec<_>, String>>()?;
    Ok(CsvSnapshot { schema: dec_str(schema)?, has_header: *has_header == "1", records })
}

fn print_csv_mutation(m: &CsvMutation) -> String {
    match m {
        CsvMutation::NoMutation => "no-mutation".to_string(),
        CsvMutation::SetSnapshot { snapshot } => format!("set-snapshot snapshot={}", enc_csv_snapshot(snapshot)),
        CsvMutation::SetHasHeader { has_header } => format!("set-has-header has-header={}", if *has_header { 1 } else { 0 }),
        CsvMutation::InsertRecord { index, record } => format!("insert-record index={index} record={}", enc_record(record)),
        CsvMutation::RemoveRecord { index } => format!("remove-record index={index}"),
        CsvMutation::SetField { record_index, field_index, value, quoted } => format!(
            "set-field record-index={record_index} field-index={field_index} value={} quoted={}",
            enc_str(value), if *quoted { 1 } else { 0 },
        ),
    }
}
fn parse_csv_mutation(line: &str) -> Result<CsvMutation, String> {
    if line == "no-mutation" {
        return Ok(CsvMutation::NoMutation);
    }
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|tok| tok.split_once('=').ok_or_else(|| format!("csv mutation: bad arg token {tok:?}")))
        .collect::<Result<Vec<_>, String>>()?
        .into_iter()
        .collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("csv mutation: missing arg '{k}' for '{keyword}'"));
    let usize_arg = |k: &str| -> Result<usize, String> { arg(k)?.parse().map_err(|e: std::num::ParseIntError| e.to_string()) };
    match keyword {
        "set-snapshot" => Ok(CsvMutation::SetSnapshot { snapshot: dec_csv_snapshot(arg("snapshot")?)? }),
        "set-has-header" => Ok(CsvMutation::SetHasHeader { has_header: arg("has-header")? == "1" }),
        "insert-record" => Ok(CsvMutation::InsertRecord { index: usize_arg("index")?, record: dec_record(arg("record")?)? }),
        "remove-record" => Ok(CsvMutation::RemoveRecord { index: usize_arg("index")? }),
        "set-field" => Ok(CsvMutation::SetField {
            record_index: usize_arg("record-index")?,
            field_index: usize_arg("field-index")?,
            value: dec_str(arg("value")?)?,
            quoted: arg("quoted")? == "1",
        }),
        other => Err(format!("csv mutation: unknown keyword {other:?}")),
    }
}

impl OpText for CsvMutation {
    fn print_op(&self) -> String {
        print_csv_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_csv_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
}

/// ⚡️ Binary = the text bytes verbatim, same simplification as `CsvDiff`'s hand-rolled codec.
impl protocol::OpBinary for CsvMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(self.print_op().into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let line = std::str::from_utf8(bytes).map_err(|e| protocol::ProtocolError::Malformed { what: "op utf8", offset: 0, detail: e.to_string() })?;
        Self::parse_op(line).map_err(|e| protocol::ProtocolError::Malformed { what: "op text", offset: 0, detail: e.to_string() })
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

    //#region 🔖️OpTextBinaryRoundtripLaw
    /// 🧪️ F6: `OpText`/`OpBinary` round-trip laws for the hand-rolled `CsvMutation` grammar —
    /// exercises every variant, incl. a `SetSnapshot` payload whose record fields contain the
    /// grammar's own reserved separator characters (`,`/`[`/`]`/space) to prove hex-encoding
    /// sidesteps escaping entirely.
    #[test]
    fn op_text_binary_roundtrip_law() {
        let mutations = vec![
            CsvMutation::NoMutation,
            CsvMutation::SetSnapshot { snapshot: sweep_b() },
            CsvMutation::SetSnapshot {
                snapshot: CsvSnapshot {
                    schema: "stdio.csv".into(),
                    has_header: false,
                    records: vec![record(&[("a, tricky [value]", true), ("plain", false)])],
                },
            },
            CsvMutation::SetHasHeader { has_header: true },
            CsvMutation::SetHasHeader { has_header: false },
            CsvMutation::InsertRecord { index: 1, record: record(&[("new, [tricky]", true)]) },
            CsvMutation::RemoveRecord { index: 0 },
            CsvMutation::SetField { record_index: 1, field_index: 0, value: "changed".into(), quoted: true },
            CsvMutation::SetField { record_index: 0, field_index: 2, value: "with, comma [and] brackets".into(), quoted: false },
        ];
        for m in mutations {
            let printed = m.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = CsvMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, m, "print_op/parse_op round-trip mismatch for {m:?} (printed {printed:?})");

            let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op({m:?}) failed: {e}"));
            let decoded = CsvMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, m, "encode_op/decode_op round-trip mismatch for {m:?}");
        }
    }
    //#endregion 🔖️OpTextBinaryRoundtripLaw
}
//#endregion 🧪️Tests
