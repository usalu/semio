//! 🧬️ DwgMutation — document mutation dispatch for `ac1024`. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION F5: real vocabulary
//! beyond the universal `{NoMutation, SetSnapshot}` stub, enriching the snapshot/diff/mutation
//! layer within ac1024's existing honest D1/D2 decode boundary (no new codec depth — D3-D5 stay
//! out of scope). Every variant's `diff()` is handcrafted via the `schema::diff` builders —
//! apply-and-capture is never used. `section_names`/`decode_status` are recomputed from
//! `sections` after every section-mutating arm (see `schema::snapshot::derive_*`), never set
//! directly.

use crate::artifacts::dwg::schema::diff::{self, DwgDiff};
use crate::artifacts::dwg::schema::snapshot::{derive_decode_status, derive_section_names, DwgSection, DwgSectionPage};
use crate::artifacts::dwg::DwgSnapshot;
use protocol::Mutation;
#[cfg(test)]
use protocol::{OpBinary, OpText};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.dwg` (ac1024).
///
/// 🧪️ F6: `dsl::DslOps` derive added — emits `dsl::DslVariants` only (P6: `OpText`/`OpBinary` are
/// always handcrafted, see `OpCodecs` below). Verified DERIVE-eligible for real: `SetSnapshot`'s
/// whole `DwgSnapshot` tree and every other variant's payload walk contain zero data-carrying
/// enums (`DwgDecodeStatus` inside `DwgSnapshot` is unit-variant-only, `DslScalar`-derived).
/// `#[dsl(block)]` on the two struct-valued variant fields matches the `BinaryMutation`/
/// `GifMutation` precedent for readability.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum DwgMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        #[dsl(block)]
        snapshot: DwgSnapshot,
    },
    /// 🗓️🌐 Sets logical version, maintenance-version, and codepage metadata.
    SetVersionInfo {
        version: String,
        maintenance_version: u8,
        codepage: u16,
    },
    /// ➕️ Inserts a fully-specified section at `index` (final position, clamped to `len`).
    InsertSection {
        index: usize,
        #[dsl(block)]
        section: DwgSection,
    },
    /// ➖️ Removes the section named `name` (no-op if absent).
    RemoveSection {
        name: String,
    },
    /// 🗜️ Whole-value-replaces a section's content (`compressed`/`declared_size`/`pages` together
    /// — the recipe's "no splice mechanism" rule for this artifact's section payloads).
    SetSectionData {
        name: String,
        compressed: bool,
        declared_size: u64,
        pages: Vec<DwgSectionPage>,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`. Every section-targeted variant is a graceful no-op when
/// `name` doesn't exist. `section_names`/`decode_status` are always recomputed from the
/// post-mutation `sections`, never set independently.
pub fn apply_dwg_mutation(snapshot: &mut DwgSnapshot, mutation: &DwgMutation) -> DwgDiff {
    let __diff = <DwgMutation as protocol::Mutation<DwgSnapshot>>::diff(mutation, snapshot);
    match mutation {
        DwgMutation::NoMutation => {}
        DwgMutation::SetSnapshot { snapshot: next } => *snapshot = next.clone(),
        DwgMutation::SetVersionInfo { version, maintenance_version, codepage } => {
            crate::artifacts::dwg::schema::snapshot::synchronize_version_info(
                snapshot,
                version,
                *maintenance_version,
                *codepage,
            )
            .expect("SetVersionInfo requires a valid DWG version sentinel");
        }
        DwgMutation::InsertSection { index, section } => {
            let at = (*index).min(snapshot.sections.len());
            snapshot.sections.insert(at, section.clone());
            snapshot.section_names = derive_section_names(&snapshot.sections);
            snapshot.decode_status = derive_decode_status(&snapshot.sections);
        }
        DwgMutation::RemoveSection { name } => {
            snapshot.sections.retain(|s| &s.name != name);
            snapshot.section_names = derive_section_names(&snapshot.sections);
            snapshot.decode_status = derive_decode_status(&snapshot.sections);
        }
        DwgMutation::SetSectionData { name, compressed, declared_size, pages } => {
            if let Some(s) = snapshot.sections.iter_mut().find(|s| &s.name == name) {
                s.compressed = *compressed;
                s.declared_size = *declared_size;
                s.pages = pages.clone();
            }
            snapshot.decode_status = derive_decode_status(&snapshot.sections);
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
            DwgMutation::InsertSection { index, section } => diff::diff_insert_section(*index, section.clone()),
            DwgMutation::RemoveSection { name } => diff::diff_remove_section(name),
            DwgMutation::SetSectionData { name, compressed, declared_size, pages } => {
                diff::diff_set_section_data(name, *compressed, *declared_size, pages.clone())
            }
        }
    }

    /// ↩️ Handcrafted, key-aware mutation-level inverses. Section-targeted variants look the
    /// prior state up in `base`; a stale/absent name inverts to `NoMutation`.
    fn inverse(&self, base: &DwgSnapshot) -> Vec<Self> {
        let section = |name: &str| base.sections.iter().find(|s| s.name == name);
        match self {
            DwgMutation::NoMutation => vec![DwgMutation::NoMutation],
            DwgMutation::SetSnapshot { .. } => vec![DwgMutation::SetSnapshot { snapshot: base.clone() }],
            DwgMutation::SetVersionInfo { .. } => vec![DwgMutation::SetVersionInfo {
                version: base.version.clone(),
                maintenance_version: base.maintenance_version,
                codepage: base.codepage,
            }],
            DwgMutation::InsertSection { section, .. } => vec![DwgMutation::RemoveSection { name: section.name.clone() }],
            DwgMutation::RemoveSection { name } => match base.sections.iter().position(|s| &s.name == name) {
                Some(index) => vec![DwgMutation::InsertSection { index, section: base.sections[index].clone() }],
                None => vec![DwgMutation::NoMutation],
            },
            DwgMutation::SetSectionData { name, .. } => match section(name) {
                Some(s) => vec![DwgMutation::SetSectionData {
                    name: name.clone(),
                    compressed: s.compressed,
                    declared_size: s.declared_size,
                    pages: s.pages.clone(),
                }],
                None => vec![DwgMutation::NoMutation],
            },
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🎙️ Handcrafted `OpText` (P6: `dsl::DslOps` emits `DslVariants` only) — one-line grammar via
/// the derived `RecordSpec`/`DslVariants`. Same ~15-line shape every `DslOps`-derived enum's
/// `OpText` impl uses (`BinaryMutation`, `GifMutation`, `SpaceMutation`, `FlowMutationDsl`).
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
/// type. Zero per-artifact logic.
impl protocol::OpBinary for DwgMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion OpCodecs

//#region 🔖️DemoCases
/// 🎬️ Representative `DwgMutation` cases, one per variant — reused by `op_text_binary_roundtrip_law`
/// below AND by `⚙️engine`'s `conformance_laws::ops_grammar_conformance_law`/`protocol_walk_law`
/// (mirrors `BinaryMutation::demo_mutation_cases`, `💾️binary/…/🧬️mutations/🦀️component.rs`).
#[cfg(test)]
pub(crate) fn demo_mutation_cases() -> Vec<DwgMutation> {
    let demo_section = |name: &str, compressed: bool, declared_size: u64, page_number: i32, file_address: u64, compressed_size: u32, decoded: &[u8]| DwgSection {
        name: name.into(),
        compressed,
        declared_size,
        pages: vec![DwgSectionPage { page_number, start_offset: file_address, decompressed_size: compressed_size, decoded: decoded.to_vec(), error: None }],
        ..Default::default()
    };
    let mut base = crate::artifacts::dwg::schema::snapshot::decode_dwg(
        b"AC1024\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x02\x1e\x00\x00",
    )
    .expect("decode demo source");
    base.sections = vec![demo_section("AcDb:Header", true, 100, 0, 0x200, 50, b"header-bytes"), demo_section("AcDb:Classes", true, 200, 1, 0x300, 80, b"classes-bytes")];
    base.section_names = derive_section_names(&base.sections);
    base.decode_status = derive_decode_status(&base.sections);
    vec![
        DwgMutation::NoMutation,
        DwgMutation::SetSnapshot { snapshot: base },
        DwgMutation::SetVersionInfo { version: "AC1024".into(), maintenance_version: 9, codepage: 65001 },
        DwgMutation::InsertSection { index: 1, section: demo_section("AcDb:Template", true, 10, 9, 0x900, 10, b"new") },
        DwgMutation::RemoveSection { name: "AcDb:Classes".into() },
        DwgMutation::SetSectionData { name: "AcDb:Header".into(), compressed: false, declared_size: 999, pages: vec![DwgSectionPage { page_number: 0, start_offset: 0x999, decompressed_size: 5, decoded: b"patched".to_vec(), error: None }] },
    ]
}
//#endregion 🔖️DemoCases

//#region Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::dwg::schema::diff::DwgSectionsDiff;
    use crate::artifacts::dwg::schema::snapshot::DwgDecodeStatus;
    use protocol::MutationDiff;
    use protocol::command::DiffAlgebra;

    //#region Fixtures
    fn page(n: i32, addr: u64, size: u32, decoded: &[u8]) -> DwgSectionPage {
        DwgSectionPage { page_number: n, start_offset: addr, decompressed_size: size, decoded: decoded.to_vec(), error: None }
    }

    fn section(name: &str, compressed: bool, declared_size: u64, pages: Vec<DwgSectionPage>) -> DwgSection {
        DwgSection { name: name.into(), compressed, declared_size, pages, ..Default::default() }
    }

    fn snapshot_with_sections(bytes: &[u8], sections: Vec<DwgSection>) -> DwgSnapshot {
        let mut snapshot = crate::artifacts::dwg::schema::snapshot::decode_dwg(bytes).expect("decode synthetic source");
        snapshot.section_names = derive_section_names(&sections);
        snapshot.decode_status = derive_decode_status(&sections);
        snapshot.sections = sections;
        snapshot
    }

    /// 🧪️ 22 bytes: 6-byte version sentinel + 16 trailing bytes, long enough to reach the 0x15
    /// preamble boundary (`maintenance_version`/`codepage`) without triggering the zero-pad path.
    fn base_snapshot() -> DwgSnapshot {
        let mut bytes = b"AC1024\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00".to_vec();
        bytes[0x12] = 2;
        bytes[0x13] = 30;
        let sections = vec![
            section("AcDb:Header", true, 100, vec![page(0, 0x200, 50, b"header-bytes")]),
            section("AcDb:Classes", true, 200, vec![page(1, 0x300, 80, b"classes-bytes")]),
            section("AcDb:Handles", false, 40, vec![page(2, 0x400, 40, b"handles-bytes")]),
        ];
        snapshot_with_sections(&bytes, sections)
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
        assert_mutation_diff_law(&base, DwgMutation::SetVersionInfo { version: "AC1024".into(), maintenance_version: 5, codepage: 1252 });
        assert_mutation_diff_law(&base, DwgMutation::InsertSection { index: 1, section: section("AcDb:Template", true, 10, vec![page(9, 0x900, 10, b"new")]) });
        assert_mutation_diff_law(&base, DwgMutation::RemoveSection { name: "AcDb:Classes".into() });
        assert_mutation_diff_law(&base, DwgMutation::SetSectionData {
            name: "AcDb:Header".into(),
            compressed: false,
            declared_size: 999,
            pages: vec![page(0, 0x999, 5, b"patched")],
        });
        // Out-of-range name: graceful no-op, still law-compliant.
        assert_mutation_diff_law(&base, DwgMutation::RemoveSection { name: "does-not-exist".into() });
    }
    //#endregion 🔖️mutation_diff_law

    //#region 🔖️inverse_law
    #[test]
    fn inverse_law() {
        let base = base_snapshot();
        let variants = vec![
            DwgMutation::NoMutation,
            DwgMutation::SetVersionInfo { version: "AC1024".into(), maintenance_version: 9, codepage: 65001 },
            DwgMutation::InsertSection { index: 1, section: section("AcDb:Template", true, 10, vec![page(9, 0x900, 10, b"new")]) },
            DwgMutation::RemoveSection { name: "AcDb:Classes".into() },
            DwgMutation::SetSectionData { name: "AcDb:Header".into(), compressed: false, declared_size: 999, pages: vec![page(0, 0x999, 5, b"patched")] },
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
        let new_section = || section("AcDb:Template", true, 10, vec![page(9, 0x900, 10, b"new")]);

        // Insert+Remove-before (canonical case): an earlier-positioned base survivor is removed
        // right after the insert — the surviving added section's final index must shift.
        assert_absorb_law(&base, DwgMutation::InsertSection { index: 1, section: new_section() }, DwgMutation::RemoveSection { name: "AcDb:Header".into() });

        // Insert+Insert-same-index: both survive, later insert lands at the lower final index.
        assert_absorb_law(
            &base,
            DwgMutation::InsertSection { index: 1, section: new_section() },
            DwgMutation::InsertSection { index: 1, section: section("AcDb:AuxHeader", false, 5, vec![]) },
        );

        // Add+SetField: the second mutation patches directly into the still-pending added section.
        assert_absorb_law(
            &base,
            DwgMutation::InsertSection { index: 0, section: new_section() },
            DwgMutation::SetSectionData { name: "AcDb:Template".into(), compressed: false, declared_size: 1, pages: vec![] },
        );

        // Modify+Remove: a pending field patch on a since-removed base section vanishes.
        assert_absorb_law(
            &base,
            DwgMutation::SetSectionData { name: "AcDb:Header".into(), compressed: false, declared_size: 1, pages: vec![] },
            DwgMutation::RemoveSection { name: "AcDb:Header".into() },
        );

        // Insert then annihilate the very same insert.
        assert_absorb_law(&base, DwgMutation::InsertSection { index: 0, section: new_section() }, DwgMutation::RemoveSection { name: "AcDb:Template".into() });

        // Two unrelated scalar sets absorb via LWW.
        assert_absorb_law(
            &base,
            DwgMutation::SetVersionInfo { version: "AC1024".into(), maintenance_version: 1, codepage: 1 },
            DwgMutation::SetVersionInfo { version: "AC1024".into(), maintenance_version: 2, codepage: 2 },
        );
    }

    #[test]
    fn absorb_law_associativity() {
        let base = base_snapshot();
        let d1 = DwgMutation::SetVersionInfo { version: "AC1024".into(), maintenance_version: 1, codepage: 1 }.diff(&base);
        let mid1 = d1.apply(&base);
        let d2 = DwgMutation::InsertSection { index: 0, section: section("AcDb:Template", true, 10, vec![]) }.diff(&mid1);
        let mid2 = d2.apply(&mid1);
        let d3 = DwgMutation::RemoveSection { name: "AcDb:Handles".into() }.diff(&mid2);

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
        b.sections.remove(0); // remove AcDb:Header
        b.sections[0].declared_size = 12345; // modify AcDb:Classes (now index 0)
        b.sections.push(section("AcDb:Template", true, 77, vec![page(5, 0x500, 20, b"template")])); // add
        b.section_names = derive_section_names(&b.sections);
        b.decode_status = derive_decode_status(&b.sections);

        let d = DwgDiff::between(&a, &b);
        assert_eq!(d.apply(&a), b, "between(a,b).apply(a) must equal b");
        let d_rev = DwgDiff::between(&b, &a);
        assert_eq!(d_rev.apply(&b), a, "between(b,a).apply(b) must equal a");
        assert!(DwgDiff::between(&a, &a).is_empty(), "between(a,a) must be empty");
    }
    //#endregion 🔖️between_roundtrip_law

    //#region 🔖️codec_retention_law
    /// 🧪️ The real fixture regression test: logical decode -> deterministic composition must be
    /// byte-identical on the actual 145KB `architectural.dwg` file.
    #[test]
    fn codec_retention_law() {
        let bytes = crate::artifacts::dwg::examples::architectural::FIXTURE_BYTES;
        let decoded = crate::artifacts::dwg::schema::snapshot::decode_dwg(bytes).expect("decode real fixture");
        assert_eq!(decoded.decode_status, DwgDecodeStatus::SectionsDecompressed, "real fixture must reach D2");
        let reencoded = crate::artifacts::dwg::schema::snapshot::encode_dwg(&decoded).expect("re-encode");
        assert_eq!(reencoded, bytes, "re-encode must be byte-identical to the original fixture");
        let redecoded = crate::artifacts::dwg::schema::snapshot::decode_dwg(&reencoded).expect("re-decode");
        assert_eq!(decoded, redecoded, "decode -> encode -> decode must be identity");
    }
    //#endregion 🔖️codec_retention_law

    //#region 🔖️field_sweep
    /// 🌪️ `sweep_a`/`sweep_b` differ in EVERY mutable field: `version`/`maintenance_version`/
    /// `codepage` all change, and `sections` has one removed, one modified in every field,
    /// and one added.
    fn sweep_a() -> DwgSnapshot {
        let mut bytes = b"AC1024\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00".to_vec();
        bytes[0x12] = 1;
        bytes[0x13] = 10;
        let sections = vec![
            section("gone", true, 10, vec![page(0, 0x10, 5, b"gone-bytes")]),
            section("stay", true, 20, vec![page(1, 0x20, 5, b"before")]),
        ];
        snapshot_with_sections(&bytes, sections)
    }

    fn sweep_b() -> DwgSnapshot {
        let mut bytes = b"AC1032\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\xff".to_vec();
        bytes[0x12] = 9;
        bytes[0x13] = 30;
        let sections = vec![
            // "stay" key unchanged (between() matches sections by name; a rename would show as
            // remove+add — same documented convention as zip's own between()), every OTHER field
            // changed to exercise `modified`.
            section("stay", false, 999, vec![page(9, 0x900, 50, b"after")]),
            section("new", true, 5, vec![page(2, 0x30, 5, b"brand new")]),
        ];
        snapshot_with_sections(&bytes, sections)
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
        let sd: &DwgSectionsDiff = forward.sections.as_ref().expect("sections diff must be present");
        assert_eq!(sd.removed, vec!["gone".to_string()], "the removed section must be tracked");
        assert_eq!(sd.added.len(), 1, "exactly one section must be added");
        assert_eq!(sd.added[0].section.name, "new");
        assert_eq!(sd.modified.len(), 1, "exactly one section must be modified");
        assert_eq!(sd.modified[0].name, "stay");
        let md = &sd.modified[0].diff;
        assert!(md.compressed.is_some(), "compressed must be diffed");
        assert!(md.declared_size.is_some(), "declared_size must be diffed");
        assert!(md.pages.is_some(), "pages must be diffed");
    }
    //#endregion 🔖️field_sweep

    #[test]
    fn out_of_range_section_mutation_is_noop_not_panic() {
        let base = base_snapshot();
        let mut snap = base.clone();
        apply_dwg_mutation(&mut snap, &DwgMutation::SetSectionData { name: "missing".into(), compressed: true, declared_size: 1, pages: vec![] });
        assert_eq!(snap, base);
        apply_dwg_mutation(&mut snap, &DwgMutation::RemoveSection { name: "missing".into() });
        assert_eq!(snap, base);
    }

    //#region 🔖️op_text_binary_roundtrip_law
    /// 🧪️ F6: `OpText`/`OpBinary` round-trip law (handcrafted impls over the `dsl::DslOps`-derived
    /// `DslVariants`) — every `demo_mutation_cases()` variant, including `SetSnapshot`'s
    /// whole-nested-`DwgSnapshot` payload, PLUS one extra `InsertSection` case exercising a
    /// `DwgSectionPage` with an `Option<String>` `error` field (`demo_mutation_cases()` itself
    /// never sets `error`, since it doubles as the engine's own `protocol_walk_law`/
    /// `ops_grammar_conformance_law` fixture and a genuinely-failed page is not representative of
    /// a "successfully decoded" demo section).
    #[test]
    fn op_text_binary_roundtrip_law() {
        let mut variants = demo_mutation_cases();
        variants.push(DwgMutation::InsertSection {
            index: 1,
            section: {
                let mut s = section("AcDb:Template", true, 10, vec![page(9, 0x900, 10, b"new")]);
                s.pages.push({
                    let mut p = page(10, 0x901, 3, b"");
                    p.error = Some("bad page".into());
                    p
                });
                s
            },
        });
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
