//! 🧪️ Tests for example `🏛️architectural` — real fixture, real D1/D2 decode assertions.

use crate::artifacts::dwg::examples::architectural::{source, FIXTURE_BYTES};
use crate::artifacts::binary::{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};
use crate::artifacts::dwg::schema::diff::DwgDiff;
use crate::artifacts::dwg::schema::mutations::{apply_dwg_mutation, DwgMutation};
use crate::artifacts::dwg::schema::snapshot::{decode_dwg, encode_dwg, DwgDecodeStatus, DwgSnapshot};
use crate::artifacts::dwg::standards::v_ac1024::subsets::any::io::export::serializers::artifacts::binary::v_raw::any as raw_export;
use crate::artifacts::dwg::standards::v_ac1024::subsets::any::io::import::deserializers::artifacts::binary::v_raw::any as raw_import;
use protocol::command::DiffAlgebra;
use protocol::{Mutation, MutationDiff};
use store::{ArtifactDsl, ArtifactPack};

#[test]
fn fixture_is_real_ac1024_not_a_stub() {
    assert!(FIXTURE_BYTES.len() > 100_000, "architectural.dwg must be the real ~145KB fixture, got {} bytes", FIXTURE_BYTES.len());
    assert_eq!(&FIXTURE_BYTES[0..6], b"AC1024", "fixture must start with the AC1024 version marker");
}

#[test]
fn source_nonempty() {
    let _ = source();
}

/// 🧪️ The actual regression test for "sentinel + passthrough" (pre-ticket behavior, which never
/// located a single real section on this file): decode must reach D2 (`SectionsDecompressed`)
/// and every real AutoCAD section name must be present.
#[test]
fn real_decode_reaches_d2_with_every_named_section() {
    let snap = decode_dwg(FIXTURE_BYTES).expect("real fixture must decode");
    assert_eq!(snap.version, "AC1024");
    assert_eq!(snap.decode_status, DwgDecodeStatus::SectionsDecompressed, "every page on this well-formed real fixture must decompress cleanly");
    let expected_names = [
        "AcDb:Header", "AcDb:AuxHeader", "AcDb:Classes", "AcDb:Handles", "AcDb:Template",
        "AcDb:ObjFreeSpace", "AcDb:AcDbObjects", "AcDb:RevHistory", "AcDb:SummaryInfo",
        "AcDb:Preview", "AcDb:AppInfo", "AcDb:AppInfoHistory", "AcDb:FileDepList",
    ];
    for name in expected_names {
        assert!(snap.section_names.iter().any(|n| n == name), "missing real section {name}");
        let section = snap.sections.iter().find(|s| s.name == name).unwrap();
        assert!(!section.pages.is_empty(), "section {name} has no pages");
        let total_decoded: usize = section.pages.iter().map(|p| p.decoded.len()).sum();
        assert!(total_decoded > 0, "section {name} decoded to zero bytes");
    }
}

/// 🧪️ Logical decode followed by deterministic materialization is byte-identical.
#[test]
fn real_decode_stays_lossless_on_reencode() {
    let snap = decode_dwg(FIXTURE_BYTES).expect("real fixture must decode");
    let reencoded = crate::artifacts::dwg::schema::snapshot::encode_dwg(&snap).expect("re-encode");
    assert_eq!(reencoded, FIXTURE_BYTES, "re-encode must be byte-identical to the original file");
}

#[test]
fn exact_fixture_roundtrips_through_snapshot_diff_mutation_and_raw_io() {
    let binary = BinarySnapshot { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes: FIXTURE_BYTES.to_vec() };
    let original = raw_import::deserialize(&binary).expect("raw DWG import");
    let exported = raw_export::serialize(&original).expect("raw DWG export").bytes;
    eprintln!("[DEBUG] DWG raw import/export bytes={} identical={}", exported.len(), exported == FIXTURE_BYTES);
    assert_eq!(exported, FIXTURE_BYTES);

    let dsl = original.print_dsl();
    let from_dsl = DwgSnapshot::parse_dsl(&dsl).expect("snapshot DSL roundtrip");
    assert_eq!(encode_dwg(&from_dsl).expect("DSL-restored export"), FIXTURE_BYTES);

    let pack = original.encode_pack().expect("snapshot pack roundtrip");
    let from_pack = DwgSnapshot::decode_pack(&pack).expect("snapshot unpack");
    assert_eq!(encode_dwg(&from_pack).expect("pack-restored export"), FIXTURE_BYTES);

    let empty = DwgDiff::between(&original, &original);
    assert!(empty.is_empty());
    assert_eq!(encode_dwg(&empty.apply(&original)).expect("empty diff export"), FIXTURE_BYTES);

    let mut no_op = original.clone();
    let no_op_diff = apply_dwg_mutation(&mut no_op, &DwgMutation::NoMutation);
    assert!(no_op_diff.is_empty());
    assert_eq!(encode_dwg(&no_op).expect("no-op mutation export"), FIXTURE_BYTES);

    let header_change = DwgMutation::SetVersionInfo {
        version: original.version.clone(),
        maintenance_version: original.maintenance_version.wrapping_add(1),
        codepage: original.codepage.wrapping_add(1),
    };
    let header_diff = header_change.diff(&original);
    let changed = header_diff.apply(&original);
    let changed_bytes = encode_dwg(&changed).expect("byte-patched header mutation export");
    let redecoded = decode_dwg(&changed_bytes).expect("mutated header re-decode");
    assert_eq!(redecoded.maintenance_version, changed.maintenance_version);
    assert_eq!(redecoded.codepage, changed.codepage);

    let inverse_diff = header_diff.inverse(&original);
    assert_eq!(encode_dwg(&inverse_diff.apply(&changed)).expect("inverse diff export"), FIXTURE_BYTES);
    let mut absorbed = header_diff;
    absorbed.absorb(inverse_diff);
    assert_eq!(encode_dwg(&absorbed.apply(&original)).expect("absorbed inverse export"), FIXTURE_BYTES);
}

#[test]
fn semantic_section_edits_materialize_from_logical_content() {
    let original = decode_dwg(FIXTURE_BYTES).expect("decode exact fixture");
    let section = original.sections.first().expect("real fixture section");
    let mutation = DwgMutation::SetSectionData {
        name: section.name.clone(),
        compressed: section.compressed,
        declared_size: section.declared_size.wrapping_add(1),
        pages: section.pages.clone(),
    };
    let mut dirty = original.clone();
    apply_dwg_mutation(&mut dirty, &mutation);
    let dirty_bytes = encode_dwg(&dirty).expect("logical section export");
    let dirty_roundtrip = decode_dwg(&dirty_bytes).expect("logical section re-import");
    assert_eq!(dirty_roundtrip.sections, dirty.sections);

    for inverse in mutation.inverse(&original) {
        apply_dwg_mutation(&mut dirty, &inverse);
    }
    assert_eq!(dirty, original);
    assert_eq!(encode_dwg(&dirty).expect("inverse mutation export"), FIXTURE_BYTES);
}
