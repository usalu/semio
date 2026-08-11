//! 🧬️ DwgMutation — document mutation dispatch for `ac1018`. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION F5: real vocabulary
//! beyond the universal `{NoMutation, SetSnapshot}` stub, matching ac1018's honest scope exactly
//! (Decision #5 — a deliberately frozen legacy shim, never brought to ac1024 decode parity).
//! `InsertSectionName`/`RemoveSectionName` operate on the flat opaque name list (no per-section
//! `data` payload exists to set — deliberately NOT named `InsertSection`/`SetSectionData` like
//! ac1024's richer vocabulary, since that would imply content this standard never decodes).
//! Every variant's `diff()` is handcrafted via the `schema::diff` builders — apply-and-capture is
//! never used.

// ⚠️ NOT `crate::artifacts::dwg::schema`/`crate::artifacts::dwg::DwgSnapshot` — those top-level
// re-exports are aliased to the CANONICAL richer standard (ac1024, per S-6). ac1018's own
// `diff`/`DwgSnapshot` must be reached through its own fully-qualified standard path (mirrors
// gif 87a's precedent).
use crate::artifacts::dwg::standards::v_ac1018::subsets::any::schema::diff::{self, DwgDiff};
use crate::artifacts::dwg::standards::v_ac1018::subsets::any::schema::snapshot::DwgSnapshot;
use protocol::Mutation;
#[cfg(test)]
use protocol::{OpBinary, OpText};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.dwg` (ac1018).
///
/// 🧪️ F6: `dsl::DslOps` derive added — emits `dsl::DslVariants` only (P6: `OpText`/`OpBinary` are
/// always handcrafted, see the `OpCodecs` region below). DERIVE-eligible: every variant's payload
/// walk (incl. `SetSnapshot`'s whole `DwgSnapshot`, `dsl::DslRecord`-derived) hits zero
/// data-carrying enums, per `f6-recon-report.md` §3's decision rule.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum DwgMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        #[dsl(block)]
        snapshot: DwgSnapshot,
    },
    /// 🗓️🌐 Sets the version/maintenance/codepage header fields, patching `bytes` at the matching
    /// plain-preamble offsets to keep the typed mirrors and the byte-level ground truth in sync.
    SetVersionInfo {
        version: String,
        maintenance_version: u8,
        codepage: u16,
    },
    /// ➕️ Inserts a detected section name at `index` (final position, clamped to `len`; may
    /// duplicate an existing entry, matching real byte-scan behavior where a name could
    /// plausibly be found more than once). Positional (not append-only) so `RemoveSectionName`'s
    /// inverse can restore the exact original position.
    InsertSectionName {
        index: usize,
        name: String,
    },
    /// ➖️ Removes one occurrence of a section name (no-op if absent).
    RemoveSectionName {
        name: String,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`.
pub fn apply_dwg_mutation(snapshot: &mut DwgSnapshot, mutation: &DwgMutation) -> DwgDiff {
    let __diff = <DwgMutation as protocol::Mutation<DwgSnapshot>>::diff(mutation, snapshot);
    match mutation {
        DwgMutation::NoMutation => {}
        DwgMutation::SetSnapshot { snapshot: next } => *snapshot = next.clone(),
        DwgMutation::SetVersionInfo { version, maintenance_version, codepage } => {
            snapshot.bytes = diff::patch_version_info_bytes(&snapshot.bytes, version, *maintenance_version, *codepage);
            snapshot.version = version.clone();
            snapshot.maintenance_version = *maintenance_version;
            snapshot.codepage = *codepage;
        }
        DwgMutation::InsertSectionName { index, name } => {
            let at = (*index).min(snapshot.section_names.len());
            snapshot.section_names.insert(at, name.clone());
        }
        DwgMutation::RemoveSectionName { name } => {
            if let Some(pos) = snapshot.section_names.iter().position(|n| n == name) {
                snapshot.section_names.remove(pos);
            }
        }
    }

    __diff
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<DwgSnapshot> for DwgMutation {
    type Diff = DwgDiff;

    fn diff(&self, base: &DwgSnapshot) -> Self::Diff {
        match self {
            DwgMutation::NoMutation => DwgDiff::default(),
            DwgMutation::SetSnapshot { snapshot } => diff::diff_set_snapshot(base, snapshot),
            DwgMutation::SetVersionInfo { version, maintenance_version, codepage } => {
                diff::diff_set_version_info(base, version, *maintenance_version, *codepage)
            }
            DwgMutation::InsertSectionName { index, name } => diff::diff_insert_section_name(base, *index, name),
            DwgMutation::RemoveSectionName { name } => diff::diff_remove_section_name(base, name),
        }
    }

    /// ↩️ Handcrafted, key-aware mutation-level inverses. `RemoveSectionName` looks up the prior
    /// occurrence in `base`; a stale/absent name inverts to `NoMutation` (nothing to undo).
    fn inverse(&self, base: &DwgSnapshot) -> Vec<Self> {
        match self {
            DwgMutation::NoMutation => vec![DwgMutation::NoMutation],
            DwgMutation::SetSnapshot { .. } => vec![DwgMutation::SetSnapshot { snapshot: base.clone() }],
            DwgMutation::SetVersionInfo { .. } => vec![DwgMutation::SetVersionInfo {
                version: base.version.clone(),
                maintenance_version: base.maintenance_version,
                codepage: base.codepage,
            }],
            DwgMutation::InsertSectionName { name, .. } => vec![DwgMutation::RemoveSectionName { name: name.clone() }],
            DwgMutation::RemoveSectionName { name } => match base.section_names.iter().position(|n| n == name) {
                Some(index) => vec![DwgMutation::InsertSectionName { index, name: name.clone() }],
                None => vec![DwgMutation::NoMutation],
            },
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🎙️ Handcrafted `OpText` (P6: `dsl::DslOps` emits `DslVariants` only) — one-line grammar via
/// the derived `RecordSpec`/`DslVariants`. Body is the same ~15-line shape every `DslOps`-derived
/// enum's `OpText` impl uses (see `BinaryMutation`, `SpaceMutation`, `FlowMutationDsl` for
/// precedent this copies verbatim). Replaces the prior `serde_json` stub.
impl protocol::OpText for DwgMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

/// ⚡️ Handcrafted `OpBinary` (P6) — pure forward to `dsl::variants_binary`, the generic
/// `format u8 (=1) | variant ordinal varint | record body` layout shared by every `DslVariants`
/// type. Zero per-artifact logic. Replaces the prior `serde_json` stub.
impl protocol::OpBinary for DwgMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion OpCodecs

//#region Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::MutationDiff;
    use protocol::command::DiffAlgebra;

    //#region Fixtures
    /// 🧪️ 22 bytes: 6-byte version sentinel + 16 trailing bytes, long enough to reach the 0x15
    /// preamble boundary (`maintenance_version`/`codepage`) without triggering the zero-pad path.
    fn base_snapshot() -> DwgSnapshot {
        let mut bytes = b"AC1018\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00".to_vec();
        bytes[0x12] = 2;
        bytes[0x13] = 30;
        bytes[0x14] = 0;
        DwgSnapshot {
            schema: "stdio.dwg".into(),
            version: "AC1018".into(),
            maintenance_version: 2,
            codepage: 30,
            bytes,
            section_names: vec!["AcDb:Header".into(), "AcDb:Classes".into(), "AcDb:Handles".into()],
        }
    }
    //#endregion Fixtures

    //#region 🔖️mutation_diff_law
    fn assert_mutation_diff_law(base: &DwgSnapshot, mutation: DwgMutation) {
        let expected_diff = mutation.diff(base);
        let mut applied_snapshot = base.clone();
        let returned_diff = apply_dwg_mutation(&mut applied_snapshot, &mutation);
        assert_eq!(returned_diff, expected_diff, "apply_dwg_mutation must return mutation.diff(base) for {mutation:?}");
        assert_eq!(expected_diff.apply(base), applied_snapshot, "diff.apply(base) must equal the imperative mutation result for {mutation:?}");
    }

    #[test]
    fn mutation_diff_law() {
        let base = base_snapshot();
        assert_mutation_diff_law(&base, DwgMutation::NoMutation);
        let mut alt = base.clone();
        alt.codepage = 1;
        assert_mutation_diff_law(&base, DwgMutation::SetSnapshot { snapshot: alt });
        assert_mutation_diff_law(&base, DwgMutation::SetVersionInfo { version: "AC1018".into(), maintenance_version: 5, codepage: 1252 });
        assert_mutation_diff_law(&base, DwgMutation::InsertSectionName { index: 3, name: "AcDb:Template".into() });
        assert_mutation_diff_law(&base, DwgMutation::RemoveSectionName { name: "AcDb:Classes".into() });
        // Out-of-range name: graceful no-op, still law-compliant.
        assert_mutation_diff_law(&base, DwgMutation::RemoveSectionName { name: "does-not-exist".into() });
    }
    //#endregion 🔖️mutation_diff_law

    //#region 🔖️inverse_law
    #[test]
    fn inverse_law() {
        let base = base_snapshot();
        let variants = vec![
            DwgMutation::NoMutation,
            DwgMutation::SetVersionInfo { version: "AC1018".into(), maintenance_version: 9, codepage: 65001 },
            DwgMutation::InsertSectionName { index: 3, name: "AcDb:Template".into() },
            DwgMutation::RemoveSectionName { name: "AcDb:Classes".into() },
        ];
        for m in variants {
            // Mutation-level round trip.
            let mut snap = base.clone();
            apply_dwg_mutation(&mut snap, &m);
            for inv in m.inverse(&base) {
                apply_dwg_mutation(&mut snap, &inv);
            }
            assert_eq!(snap, base, "mutation-level inverse must restore base for {m:?}");

            // Diff-level round trip.
            let d = m.diff(&base);
            let mutated = d.apply(&base);
            let inv_d = d.inverse(&base);
            assert_eq!(inv_d.apply(&mutated), base, "diff-level inverse must restore base for {m:?}");
        }
    }
    //#endregion 🔖️inverse_law

    //#region 🔖️absorb_law
    fn assert_absorb_law(base: &DwgSnapshot, m1: DwgMutation, m2: DwgMutation) {
        let d1 = m1.diff(base);
        let mid = d1.apply(base);
        let d2 = m2.diff(&mid);
        let sequential = d2.apply(&mid);

        let mut merged = d1.clone();
        merged.absorb(d2.clone());
        assert_eq!(merged.apply(base), sequential, "absorb(d1,d2).apply(base) must equal sequential application for {m1:?} + {m2:?}");
    }

    #[test]
    fn absorb_law() {
        let base = base_snapshot();

        // Insert+Remove-before (canonical case): the just-added name is immediately removed
        // again — nets to nothing. Whole-value LWW composition handles this automatically since
        // `d2` was computed against the post-insert mid-state.
        assert_absorb_law(&base, DwgMutation::InsertSectionName { index: 3, name: "AcDb:Template".into() }, DwgMutation::RemoveSectionName { name: "AcDb:Template".into() });

        // Insert+Insert-same-name: both survive (appended twice — a real multiset, not a set).
        assert_absorb_law(&base, DwgMutation::InsertSectionName { index: 3, name: "AcDb:Template".into() }, DwgMutation::InsertSectionName { index: 3, name: "AcDb:Template".into() });

        // Remove-then-remove-a-different-name: both real removals compose.
        assert_absorb_law(&base, DwgMutation::RemoveSectionName { name: "AcDb:Classes".into() }, DwgMutation::RemoveSectionName { name: "AcDb:Handles".into() });

        // Two unrelated scalar sets absorb via LWW.
        assert_absorb_law(
            &base,
            DwgMutation::SetVersionInfo { version: "AC1018".into(), maintenance_version: 1, codepage: 1 },
            DwgMutation::SetVersionInfo { version: "AC1018".into(), maintenance_version: 2, codepage: 2 },
        );
    }

    #[test]
    fn absorb_law_associativity() {
        let base = base_snapshot();
        let d1 = DwgMutation::SetVersionInfo { version: "AC1018".into(), maintenance_version: 1, codepage: 1 }.diff(&base);
        let mid1 = d1.apply(&base);
        let d2 = DwgMutation::InsertSectionName { index: 3, name: "AcDb:Template".into() }.diff(&mid1);
        let mid2 = d2.apply(&mid1);
        let d3 = DwgMutation::RemoveSectionName { name: "AcDb:Handles".into() }.diff(&mid2);

        // (d1∘d2)∘d3
        let mut left = d1.clone();
        left.absorb(d2.clone());
        left.absorb(d3.clone());

        // d1∘(d2∘d3)
        let mut d23 = d2.clone();
        d23.absorb(d3.clone());
        let mut right = d1.clone();
        right.absorb(d23);

        assert_eq!(left.apply(&base), right.apply(&base), "absorb must associate");
        assert_eq!(left.apply(&base), d3.apply(&mid2), "associated absorb must match full sequential application");
    }
    //#endregion 🔖️absorb_law

    //#region 🔖️between_roundtrip_law
    #[test]
    fn between_roundtrip_law() {
        let a = base_snapshot();
        let mut b = base_snapshot();
        b.maintenance_version = 9;
        b.codepage = 65001;
        b.section_names.remove(0); // remove AcDb:Header
        b.section_names.push("AcDb:Template".into()); // add AcDb:Template

        let d = DwgDiff::between(&a, &b);
        assert_eq!(d.apply(&a), b, "between(a,b).apply(a) must equal b");
        let d_rev = DwgDiff::between(&b, &a);
        assert_eq!(d_rev.apply(&b), a, "between(b,a).apply(b) must equal a");
        assert!(DwgDiff::between(&a, &a).is_empty(), "between(a,a) must be empty");
    }
    //#endregion 🔖️between_roundtrip_law

    //#region 🔖️codec_retention_law
    #[test]
    fn codec_retention_law() {
        use crate::artifacts::dwg::standards::v_ac1018::subsets::any::schema::snapshot::{decode_dwg, encode_dwg};
        let stub = b"AC1018\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
        let decoded = decode_dwg(stub).expect("decode stub");
        let reencoded = encode_dwg(&decoded).expect("re-encode");
        let redecoded = decode_dwg(&reencoded).expect("re-decode");
        assert_eq!(decoded, redecoded, "decode -> encode -> decode must be identity");
        assert_eq!(reencoded, stub, "re-encode must be byte-identical to the original stub");
    }
    //#endregion 🔖️codec_retention_law

    //#region 🔖️field_sweep
    /// 🌪️ `sweep_a`/`sweep_b` differ in EVERY mutable field: `version`/`maintenance_version`/
    /// `codepage`/`bytes` all change, and `section_names` has one name removed and one added.
    fn sweep_a() -> DwgSnapshot {
        let mut bytes = b"AC1018\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00".to_vec();
        bytes[0x12] = 1;
        bytes[0x13] = 10;
        DwgSnapshot {
            schema: "stdio.dwg".into(),
            version: "AC1018".into(),
            maintenance_version: 1,
            codepage: 10,
            bytes,
            section_names: vec!["AcDb:Header".into(), "AcDb:Classes".into()],
        }
    }

    fn sweep_b() -> DwgSnapshot {
        let mut bytes = b"AC1032\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\xff".to_vec();
        bytes[0x12] = 9;
        bytes[0x13] = 30;
        DwgSnapshot {
            schema: "stdio.dwg".into(),
            version: "AC1032".into(),
            maintenance_version: 9,
            codepage: 30,
            bytes,
            // "AcDb:Header" removed, "AcDb:Handles" added, "AcDb:Classes" survives.
            section_names: vec!["AcDb:Classes".into(), "AcDb:Handles".into()],
        }
    }

    #[test]
    fn field_sweep_covers_every_mutable_field() {
        let a = sweep_a();
        let b = sweep_b();

        let forward = DwgDiff::between(&a, &b);
        assert_eq!(forward.apply(&a), b, "between(a,b).apply(a) must equal b");
        let backward = DwgDiff::between(&b, &a);
        assert_eq!(backward.apply(&b), a, "between(b,a).apply(b) must equal a");
        assert!(DwgDiff::between(&a, &a).is_empty(), "between(a,a) must be empty");

        // Structural per-field assertion: every mutable field actually changed in the diff.
        assert!(forward.version.is_some(), "version must be diffed");
        assert!(forward.maintenance_version.is_some(), "maintenance_version must be diffed");
        assert!(forward.codepage.is_some(), "codepage must be diffed");
        assert!(forward.bytes.is_some(), "bytes must be diffed");
        assert_eq!(
            forward.section_names.as_ref().expect("section_names diff must be present"),
            &b.section_names,
            "section_names must be diffed as the whole new value"
        );
    }
    //#endregion 🔖️field_sweep

    #[test]
    fn out_of_range_section_name_mutation_is_noop_not_panic() {
        let base = base_snapshot();
        let mut snap = base.clone();
        apply_dwg_mutation(&mut snap, &DwgMutation::RemoveSectionName { name: "missing".into() });
        assert_eq!(snap, base);
    }

    //#region 🔖️op_text_binary_roundtrip_law
    /// 🧪️ F6: `OpText`/`OpBinary` round-trip laws (handcrafted impls over the `dsl::DslOps`-derived
    /// `DslVariants`), every variant incl. `SetSnapshot`'s nested-record payload.
    #[test]
    fn op_text_binary_roundtrip_law() {
        let base = base_snapshot();
        let variants = vec![
            DwgMutation::NoMutation,
            DwgMutation::SetSnapshot { snapshot: base.clone() },
            DwgMutation::SetVersionInfo { version: "AC1018".into(), maintenance_version: 5, codepage: 1252 },
            DwgMutation::InsertSectionName { index: 1, name: "AcDb:Template".into() },
            DwgMutation::RemoveSectionName { name: "AcDb:Classes".into() },
        ];
        for m in variants {
            let printed = m.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = DwgMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, m, "print_op/parse_op round-trip mismatch for {m:?} (printed {printed:?})");

            let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op({m:?}) failed: {e}"));
            let decoded = DwgMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, m, "encode_op/decode_op round-trip mismatch for {m:?}");
        }
    }
    //#endregion 🔖️op_text_binary_roundtrip_law
}
//#endregion Tests
