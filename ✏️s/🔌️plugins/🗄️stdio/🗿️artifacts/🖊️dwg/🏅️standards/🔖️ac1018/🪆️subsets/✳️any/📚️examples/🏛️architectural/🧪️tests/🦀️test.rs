//! 🧪️ Tests for example `🏛️architectural` — real fixture, real D1/D2 decode assertions.

use crate::artifacts::binary::{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};
use crate::artifacts::dwg::examples::architectural::{source, FIXTURE_BYTES};
use crate::artifacts::dwg::schema::diff::DwgDiff;
use crate::artifacts::dwg::schema::mutations::{apply_dwg_mutation, DwgMutation};
use crate::artifacts::dwg::schema::snapshot::{decode_dwg, encode_dwg, DwgSnapshot};
use crate::artifacts::dwg::standards::v_ac1024::subsets::any::io::export::serializers::artifacts::binary::v_raw::any as raw_export;
use crate::artifacts::dwg::standards::v_ac1024::subsets::any::io::import::deserializers::artifacts::binary::v_raw::any as raw_import;
use crate::artifacts::dwg::standards::v_ac1024::subsets::any::schema::DwgAnalyzer;
use protocol::command::DiffAlgebra;
use protocol::{Mutation, MutationDiff};
use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeSource, Dialect, StandardId, SubsetId};
use store::{ArtifactDsl, ArtifactPack};

async fn assert_fixture_bytes(actual: &[u8], label: &str) {
    if actual == FIXTURE_BYTES {
        return;
    }
    let offset = actual.iter().zip(FIXTURE_BYTES).position(|(left, right)| left != right).unwrap_or(actual.len().min(FIXTURE_BYTES.len()));
    let start = offset.saturating_sub(8);
    let actual_end = (offset + 8).min(actual.len());
    let expected_end = (offset + 8).min(FIXTURE_BYTES.len());
    panic!("{label}: bytes differ at offset {offset}; actual len={} window={:02x?}; expected len={} window={:02x?}", actual.len(), &actual[start..actual_end], FIXTURE_BYTES.len(), &FIXTURE_BYTES[start..expected_end]);
}

#[semio_framework_async_macros::async_test]
async fn fixture_is_real_ac1024_not_a_stub() {
    assert!(FIXTURE_BYTES.len() > 100_000, "architectural.dwg must be the real ~145KB fixture, got {} bytes", FIXTURE_BYTES.len());
    assert_eq!(&FIXTURE_BYTES[0..6], b"AC1024", "fixture must start with the AC1024 version marker");
}

#[semio_framework_async_macros::async_test]
async fn source_nonempty() {
    let _ = source();
}

/// 🧪️ The real file is projected into standard logical concepts without retaining its container.
#[semio_framework_async_macros::async_test]
async fn real_decode_projects_logical_state() {
    let snap = decode_dwg(FIXTURE_BYTES).expect("real fixture must decode");
    assert_eq!(snap.version, "AC1024");
    assert_eq!(snap.schema, crate::artifacts::dwg::STDIO_DWG_DOCUMENT_SCHEMA);
    assert!(snap.codepage > 0);
    assert!(snap.drawing.layers.len() >= 7, "real fixture must project its standard layer table records");
    assert_eq!(snap.drawing.entities().len(), 68, "real fixture must derive LINE, ARC and LWPOLYLINE projections from handle-keyed bodies");
    assert_eq!(snap.drawing.objects.len(), 663, "every framed object from both independently based handle-map blocks must retain its logical identity");
    assert!(snap.drawing.objects.iter().all(|object| !object.class_name.is_empty()), "every framed object must resolve to a fixed or custom class name");
    let relation_bodies =
        snap.drawing.objects.iter().filter(|object| object.owner_handle.is_some() || !object.reactor_handles.is_empty() || object.extension_dictionary_handle.is_some() || !object.referenced_handles.is_empty() || object.body.is_some()).count();
    let named_dictionaries = snap.drawing.objects.iter().filter(|object| matches!(&object.body, Some(crate::artifacts::dwg::schema::snapshot::DwgLogicalObjectBody::Dictionary(body)) if !body.entries.is_empty())).count();
    let named_records = snap.drawing.objects.iter().filter(|object| matches!(&object.body, Some(crate::artifacts::dwg::schema::snapshot::DwgLogicalObjectBody::TableRecord(_)))).count();
    let table_controls = snap.drawing.objects.iter().filter(|object| matches!(&object.body, Some(crate::artifacts::dwg::schema::snapshot::DwgLogicalObjectBody::TableControl(body)) if !body.entry_handles().is_empty())).count();
    let xrecords = snap.drawing.objects.iter().filter(|object| matches!(&object.body, Some(crate::artifacts::dwg::schema::snapshot::DwgLogicalObjectBody::XRecord(_)))).count();
    let typed_xrecords = snap.drawing.objects.iter().filter(|object| matches!(&object.body, Some(crate::artifacts::dwg::schema::snapshot::DwgLogicalObjectBody::XRecord(body)) if !body.values.is_empty())).count();
    let xrecord_values = snap
        .drawing
        .objects
        .iter()
        .filter_map(|object| match &object.body {
            Some(crate::artifacts::dwg::schema::snapshot::DwgLogicalObjectBody::XRecord(body)) => Some(body.values.len()),
            _ => None,
        })
        .sum::<usize>();
    let xrecord_object_ids = snap
        .drawing
        .objects
        .iter()
        .filter_map(|object| match &object.body {
            Some(crate::artifacts::dwg::schema::snapshot::DwgLogicalObjectBody::XRecord(body)) => Some(body.object_id_handles.len()),
            _ => None,
        })
        .sum::<usize>();
    let xrecord_inline_object_ids = snap
        .drawing
        .objects
        .iter()
        .filter_map(|object| match &object.body {
            Some(crate::artifacts::dwg::schema::snapshot::DwgLogicalObjectBody::XRecord(body)) => Some(body.values.iter().filter(|value| matches!(value, crate::artifacts::dwg::schema::snapshot::DwgXRecordValue::ObjectId { .. })).count()),
            _ => None,
        })
        .sum::<usize>();
    assert!(relation_bodies >= 615, "typed common relations must be projected without publishing partial unsupported entities, got {relation_bodies}");
    assert!(named_dictionaries >= 77, "dictionary entry names and handles must be projected");
    assert_eq!(named_records, 50, "all table-record names must be projected");
    assert!(table_controls >= 8, "table-control owned handles must be projected");
    assert_eq!(xrecords, 145, "all XRecord cloning flags must be projected");
    assert_eq!(typed_xrecords, 145, "all XRecords must retain typed values");
    assert_eq!(xrecord_values, 2_044, "all ordered XRecord group values must be projected");
    assert_eq!(xrecord_object_ids, 0, "this fixture's bounded XRecord handle vectors are empty");
    assert_eq!(xrecord_inline_object_ids, 38, "inline XRecord ObjectId values must remain typed even when the trailing handle vector is empty");
    assert!(snap.drawing.objects.iter().any(|object| !object.extended_data.is_empty()), "semantic EED application records must be typed rather than silently discarded");
    let dsl = snap.print_dsl();
    assert!(dsl.await.contains("xrecord"), "structured DSL must carry tagged XRecord bodies");
    let dsl_objects = DwgSnapshot::parse_dsl(&dsl).await.expect("typed DWG DSL must decode").drawing.objects;
    if dsl_objects != snap.drawing.objects {
        let index = dsl_objects.iter().zip(&snap.drawing.objects).position(|(left, right)| left != right).unwrap_or(dsl_objects.len().min(snap.drawing.objects.len()));
        panic!("typed object bodies must survive DSL; first mismatch index={index} decoded={:?} original={:?}", dsl_objects.get(index), snap.drawing.objects.get(index));
    }
    let pack = snap.encode_pack();
    assert_eq!(DwgSnapshot::decode_pack(&pack).await.expect("typed DWG pack must decode").drawing.objects, snap.drawing.objects, "typed object bodies must survive pack");
    assert!(!snap.classes.is_empty(), "real fixture must project its standard class definitions");
    assert!(!snap.dependencies.is_empty(), "real fixture must project its standard file dependencies");
    assert!(!snap.application.name.is_empty(), "real fixture must project its standard application information");
}

#[semio_framework_async_macros::async_test]
async fn every_imported_object_has_a_typed_standard_body() {
    let snap = decode_dwg(FIXTURE_BYTES).expect("real fixture must decode");
    let missing = snap.drawing.objects.iter().filter(|object| object.body.is_none()).map(|object| format!("{:#x}:{}", object.handle, object.class_name)).collect::<Vec<_>>();
    assert!(missing.is_empty(), "{} of 652 imported objects remain identity-only; first missing bodies: {:?}", missing.len(), &missing[..missing.len().min(24)]);
}

#[semio_framework_async_macros::async_test]
async fn real_decode_stays_lossless_on_reencode() {
    let snap = decode_dwg(FIXTURE_BYTES).expect("real fixture must decode");
    let reencoded = encode_dwg(&snap).expect("re-encode");
    assert_fixture_bytes(&reencoded, "re-encode");
}

#[semio_framework_async_macros::async_test]
async fn snapshot_pack_preserves_signed_zero_semantics() {
    let original = decode_dwg(FIXTURE_BYTES).expect("real fixture must decode");
    let restored = DwgSnapshot::decode_pack(&original.encode_pack()).await.expect("snapshot pack roundtrip");
    let expected = serde_json::to_string(&original.drawing).expect("original drawing JSON");
    let actual = serde_json::to_string(&restored.drawing).expect("restored drawing JSON");
    assert!(expected.contains("\"value\":-0.0"), "fixture must exercise negative zero");
    assert_eq!(actual, expected, "snapshot pack must preserve signed zero");
}

#[semio_framework_async_macros::async_test]
async fn exact_fixture_roundtrips_through_snapshot_diff_mutation_and_raw_io() {
    let binary = BinarySnapshot { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes: FIXTURE_BYTES.to_vec() };
    let original = raw_import::deserialize(&binary).expect("raw DWG import");
    let exported = raw_export::serialize(&original).expect("raw DWG export").bytes;
    assert_fixture_bytes(&exported, "raw import/export");

    let dsl = original.print_dsl();
    for forbidden in ["section-names", "decode-status", "compressed", "encrypted", "page-number", "start-offset", "declared-size", "decompressed-size", "bytes=", "drawing={entities="] {
        assert!(!dsl.await.contains(forbidden), "snapshot DSL retained forbidden DWG shadow term {forbidden}");
    }
    let from_dsl = DwgSnapshot::parse_dsl(&dsl).await.expect("snapshot DSL roundtrip");
    assert_fixture_bytes(&encode_dwg(&from_dsl).expect("DSL-restored export"), "DSL-restored export");

    let pack = original.encode_pack();
    let pack_text = String::from_utf8_lossy(&pack);
    for forbidden in ["sectionNames", "decodeStatus", "compressed", "encrypted", "pageNumber", "startOffset", "declaredSize", "decompressedSize", "bytes_wire", "DwgLogicalEntity"] {
        assert!(!pack_text.contains(forbidden), "snapshot pack retained forbidden DWG shadow term {forbidden}");
    }
    let from_pack = DwgSnapshot::decode_pack(&pack).await.expect("snapshot unpack");
    let restored_json = serde_json::to_string(&from_pack.drawing).expect("restored drawing JSON");
    let expected_json = serde_json::to_string(&original.drawing).expect("original drawing JSON");
    assert_eq!(restored_json, expected_json, "snapshot pack must preserve signed numeric semantics");
    assert_fixture_bytes(&encode_dwg(&from_pack).expect("pack-restored export"), "pack-restored export");

    let analysis = DwgAnalyzer::analyze(&[AnalyzeSource::Text(&dsl)]);
    let analyzed = analysis.await.parts.snapshot.expect("analyzer snapshot");
    assert_fixture_bytes(&encode_dwg(&analyzed).expect("analyzer export"), "analyzer export");
    let dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1024"), subset: SubsetId("*") };
    let composition = crate::artifacts::dwg::standards::v_ac1024::subsets::any::io::derived_composition::DwgComposerComposition::compose(&[ComposeSource { dialect, payload: AnalyzeSource::Binary(&pack) }]).await.expect("composer snapshot");
    assert_fixture_bytes(&encode_dwg(&composition.snapshot).expect("composer export"), "composer export");

    let empty = DwgDiff::between(&original, &original);
    assert!(empty.is_empty());
    assert_fixture_bytes(&encode_dwg(&empty.apply(&original).expect("empty diff must apply")).expect("empty diff export"), "empty diff export");

    let mut no_op = original.clone();
    let no_op_diff = apply_dwg_mutation(&mut no_op, &DwgMutation::NoMutation);
    assert!(no_op_diff.diff().is_empty());
    assert_fixture_bytes(&encode_dwg(&no_op).expect("no-op mutation export"), "no-op mutation export");

    let header_change = DwgMutation::SetVersionInfo { version: original.version.clone(), maintenance_version: original.maintenance_version.wrapping_add(1), codepage: original.codepage.wrapping_add(1) };
    let header_diff = header_change.diff(&original);
    let changed = header_diff.await.diff().apply(&original).expect("header diff must apply");
    let changed_bytes = encode_dwg(&changed).expect("byte-patched header mutation export");
    let redecoded = decode_dwg(&changed_bytes).expect("mutated header re-decode");
    assert_eq!(redecoded.maintenance_version, changed.maintenance_version);
    assert_eq!(redecoded.codepage, changed.codepage);

    let inverse_diff = header_diff.await.diff().inverse(&original);
    assert_fixture_bytes(&encode_dwg(&inverse_diff.apply(&changed).expect("inverse diff must apply")).expect("inverse diff export"), "inverse diff export");
    let mut absorbed = header_diff.await.diff().clone();
    absorbed.absorb(inverse_diff);
    assert_fixture_bytes(&encode_dwg(&absorbed.apply(&original).expect("absorbed diff must apply")).expect("absorbed inverse export"), "absorbed inverse export");
}

#[semio_framework_async_macros::async_test]
async fn persisted_dwg_facets_have_no_parallel_entity_projection() {
    let artifact_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧬️schema");
    for relative in
        ["🟦️component.ts", "🔗️component.graphql", "🛰️component.proto", "🔣️component.json", "📸️snapshot/🔗️component.graphql", "📸️snapshot/🛰️component.proto", "🔺️diff/🟦️component.ts", "🔺️diff/🔗️component.graphql", "🔺️diff/🛰️component.proto"]
    {
        let facet = std::fs::read_to_string(artifact_root.join(relative)).unwrap_or_else(|error| panic!("{relative}: {error}"));
        assert!(!facet.contains("DwgLogicalEntity"), "{relative} retains DwgLogicalEntity");
        assert!(!facet.contains(" entities:"), "{relative} retains the parallel entities field");
        assert!(!facet.contains("\"entities\""), "{relative} retains the parallel entities property");
    }
    let rust = std::fs::read_to_string(artifact_root.join("📸️snapshot/🦀️component.rs")).expect("Rust snapshot schema");
    let drawing = rust.split("pub struct DwgLogicalDrawing").nth(1).and_then(|tail| tail.split("impl DwgLogicalDrawing").next()).expect("logical drawing definition");
    assert!(!drawing.contains("pub entities:"), "Rust persisted drawing retains the parallel entities field");
}

#[semio_framework_async_macros::async_test]
async fn semantic_metadata_edits_materialize_from_logical_content() {
    let original = decode_dwg(FIXTURE_BYTES).expect("decode exact fixture");
    let mut changed = original.clone();
    changed.summary.title = "Architectural Example".into();
    let mutation = DwgMutation::SetSnapshot { snapshot: changed };
    let mut dirty = original.clone();
    apply_dwg_mutation(&mut dirty, &mutation);
    let dirty_bytes = encode_dwg(&dirty).expect("logical metadata export");
    let dirty_roundtrip = decode_dwg(&dirty_bytes).expect("logical section re-import");
    assert_eq!(dirty_roundtrip.version, dirty.version);

    for inverse in mutation.inverse(&original) {
        apply_dwg_mutation(&mut dirty, &inverse);
    }
    assert_eq!(dirty, original);
    assert_fixture_bytes(&encode_dwg(&dirty).expect("inverse mutation export"), "inverse mutation export");
}
