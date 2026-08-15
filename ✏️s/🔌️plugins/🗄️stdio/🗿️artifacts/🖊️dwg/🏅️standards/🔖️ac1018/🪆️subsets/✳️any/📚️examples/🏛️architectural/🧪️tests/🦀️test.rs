//! 🧪️ Tests for example `🏛️architectural` — real fixture, real D1/D2 decode assertions.

use crate::artifacts::binary::{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};
use crate::artifacts::dwg::examples::architectural::{FIXTURE_BYTES, source};
use crate::artifacts::dwg::schema::diff::DwgDiff;
use crate::artifacts::dwg::schema::mutations::{DwgMutation, apply_dwg_mutation};
use crate::artifacts::dwg::schema::snapshot::{DwgSnapshot, decode_dwg, encode_dwg};
use crate::artifacts::dwg::standards::v_ac1024::engine as dwg_engine;
use crate::artifacts::dwg::standards::v_ac1024::subsets::any::schema::DwgAnalyzer;
use crate::artifacts::dwg::standards::v_ac1024::subsets::any::io::export::serializers::artifacts::binary::v_raw::any as raw_export;
use crate::artifacts::dwg::standards::v_ac1024::subsets::any::io::import::deserializers::artifacts::binary::v_raw::any as raw_import;
use protocol::command::DiffAlgebra;
use protocol::{Mutation, MutationDiff};
use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeSource, Dialect, StandardId, SubsetId};
use store::{ArtifactDsl, ArtifactPack};

fn assert_fixture_bytes(actual: &[u8], label: &str) {
    if actual == FIXTURE_BYTES {
        return;
    }
    let offset = actual.iter().zip(FIXTURE_BYTES).position(|(left, right)| left != right).unwrap_or(actual.len().min(FIXTURE_BYTES.len()));
    let start = offset.saturating_sub(8);
    let actual_end = (offset + 8).min(actual.len());
    let expected_end = (offset + 8).min(FIXTURE_BYTES.len());
    panic!("{label}: bytes differ at offset {offset}; actual len={} window={:02x?}; expected len={} window={:02x?}", actual.len(), &actual[start..actual_end], FIXTURE_BYTES.len(), &FIXTURE_BYTES[start..expected_end]);
}

#[test]
fn fixture_is_real_ac1024_not_a_stub() {
    assert!(FIXTURE_BYTES.len() > 100_000, "architectural.dwg must be the real ~145KB fixture, got {} bytes", FIXTURE_BYTES.len());
    assert_eq!(&FIXTURE_BYTES[0..6], b"AC1024", "fixture must start with the AC1024 version marker");
}

#[test]
fn source_nonempty() {
    let _ = source();
}

/// 🧪️ The real file is projected into standard logical concepts without retaining its container.
#[test]
fn real_decode_projects_logical_state() {
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
    assert!(dsl.contains("xrecord"), "structured DSL must carry tagged XRecord bodies");
    let dsl_objects = DwgSnapshot::parse_dsl(&dsl).expect("typed DWG DSL must decode").drawing.objects;
    if dsl_objects != snap.drawing.objects {
        let index = dsl_objects.iter().zip(&snap.drawing.objects).position(|(left, right)| left != right).unwrap_or(dsl_objects.len().min(snap.drawing.objects.len()));
        panic!("typed object bodies must survive DSL; first mismatch index={index} decoded={:?} original={:?}", dsl_objects.get(index), snap.drawing.objects.get(index));
    }
    let pack = snap.encode_pack();
    assert_eq!(DwgSnapshot::decode_pack(&pack).expect("typed DWG pack must decode").drawing.objects, snap.drawing.objects, "typed object bodies must survive pack");
    assert!(!snap.classes.is_empty(), "real fixture must project its standard class definitions");
    assert!(!snap.dependencies.is_empty(), "real fixture must project its standard file dependencies");
    assert!(!snap.application.name.is_empty(), "real fixture must project its standard application information");
}

#[test]
fn xrecord_terminal_fill_policy_is_uniform_and_derived() {
    let classes = dwg_engine::decode_r2004_classes(FIXTURE_BYTES).expect("classes must decode");
    let fills = dwg_engine::decode_r2004_xrecord_terminal_fills(FIXTURE_BYTES, &classes).expect("XRecord terminal fills must decode");
    let mut histogram = std::collections::BTreeMap::new();
    for fill in &fills {
        *histogram.entry(*fill).or_insert(0usize) += 1;
    }
    assert_eq!(fills.len(), 145, "every XRecord frame must contribute one terminal-fill observation");
    assert_eq!(histogram, std::collections::BTreeMap::from([((4, 15), 145)]), "all fixture XRecords must share the same derived terminal-fill width and value");
    assert!(fills.iter().all(|(width, value)| *width <= 7 && *value == (1u8 << *width).saturating_sub(1)), "terminal fill must be the single derived AC1024 all-ones policy");
}

#[test]
fn xrecord_logical_frames_reencode_exactly() {
    let classes = dwg_engine::decode_r2004_classes(FIXTURE_BYTES).expect("classes must decode");
    let verified = dwg_engine::verify_r2004_xrecord_frames(FIXTURE_BYTES, &classes).expect("typed XRECORD frames must reencode exactly");
    assert_eq!(verified, 145, "every typed XRECORD frame must be compared");
}

#[test]
fn dictionary_logical_frames_reencode_exactly() {
    let classes = dwg_engine::decode_r2004_classes(FIXTURE_BYTES).expect("classes must decode");
    let verified = dwg_engine::verify_r2004_dictionary_frames(FIXTURE_BYTES, &classes).expect("typed DICTIONARY/WDFLT frames must reencode exactly");
    assert_eq!(verified, 84, "all fixed and custom dictionary frames must be compared");
}

#[test]
fn every_imported_object_has_a_typed_standard_body() {
    let snap = decode_dwg(FIXTURE_BYTES).expect("real fixture must decode");
    let missing = snap.drawing.objects.iter().filter(|object| object.body.is_none()).map(|object| format!("{:#x}:{}", object.handle, object.class_name)).collect::<Vec<_>>();
    assert!(missing.is_empty(), "{} of 652 imported objects remain identity-only; first missing bodies: {:?}", missing.len(), &missing[..missing.len().min(24)]);
}

#[test]
fn table_control_logical_frames_reencode_exactly() {
    let classes = dwg_engine::decode_r2004_classes(FIXTURE_BYTES).expect("classes must decode");
    let verified = dwg_engine::verify_r2004_table_control_frames(FIXTURE_BYTES, &classes).expect("typed table-control frames must reencode exactly");
    assert_eq!(verified, 9, "all nine fixed table-control families must be compared");
}

#[test]
fn table_record_logical_frames_reencode_exactly() {
    let classes = dwg_engine::decode_r2004_classes(FIXTURE_BYTES).expect("classes must decode");
    let verified = dwg_engine::verify_r2004_table_record_frames(FIXTURE_BYTES, &classes).expect("typed table-record frames must reencode exactly");
    assert_eq!(verified, 50, "all fifty fixed table records must be compared");
}

#[test]
fn line_logical_frames_reencode_exactly() {
    let classes = dwg_engine::decode_r2004_classes(FIXTURE_BYTES).expect("fixture classes must decode");
    let verified = dwg_engine::verify_r2004_line_frames(FIXTURE_BYTES, &classes).expect("typed LINE frames must reencode exactly");
    assert_eq!(verified, 40, "all fixture LINE frames must be typed and exact");
}

#[test]
fn arc_logical_frames_reencode_exactly() {
    let classes = dwg_engine::decode_r2004_classes(FIXTURE_BYTES).expect("fixture classes must decode");
    let verified = dwg_engine::verify_r2004_arc_frames(FIXTURE_BYTES, &classes).expect("typed ARC frames must reencode exactly");
    assert_eq!(verified, 12, "all fixture ARC frames must be typed and exact");
}

#[test]
fn lwpolyline_logical_frames_reencode_exactly() {
    let classes = dwg_engine::decode_r2004_classes(FIXTURE_BYTES).expect("fixture classes must decode");
    let verified = dwg_engine::verify_r2004_lwpolyline_frames(FIXTURE_BYTES, &classes).expect("typed LWPOLYLINE frames must reencode exactly");
    assert_eq!(verified, 16, "all fixture LWPOLYLINE frames must be typed and exact");
}

#[test]
fn block_marker_logical_frames_reencode_exactly() {
    let classes = dwg_engine::decode_r2004_classes(FIXTURE_BYTES).expect("fixture classes must decode");
    let (blocks, end_blocks) = dwg_engine::verify_r2004_block_marker_frames(FIXTURE_BYTES, &classes).expect("typed BLOCK and ENDBLK frames must reencode exactly");
    assert_eq!((blocks, end_blocks), (10, 10), "all fixture BLOCK and ENDBLK frames must be typed and exact");
}

#[test]
fn insert_logical_frames_reencode_exactly() {
    let classes = dwg_engine::decode_r2004_classes(FIXTURE_BYTES).expect("fixture classes must decode");
    let verified = dwg_engine::verify_r2004_insert_frames(FIXTURE_BYTES, &classes).expect("typed INSERT frames must reencode exactly");
    assert_eq!(verified, 12, "all fixture INSERT frames must be typed and exact");
}

#[test]
fn dimension_linear_logical_frames_reencode_exactly() {
    let classes = dwg_engine::decode_r2004_classes(FIXTURE_BYTES).expect("fixture classes must decode");
    let verified = dwg_engine::verify_r2004_dimension_linear_frames(FIXTURE_BYTES, &classes).expect("typed DIMENSION_LINEAR frames must reencode exactly");
    assert_eq!(verified, 14, "all fixture DIMENSION_LINEAR frames, including the recovered second handle-map block, must be typed and exact");
}

#[test]
fn viewport_logical_frames_reencode_exactly() {
    let classes = dwg_engine::decode_r2004_classes(FIXTURE_BYTES).expect("fixture classes must decode");
    let verified = dwg_engine::verify_r2004_viewport_frames(FIXTURE_BYTES, &classes).expect("typed VIEWPORT frames must reencode exactly");
    assert_eq!(verified, 2, "both fixture VIEWPORT frames must be typed and exact");
}

#[test]
fn visual_style_logical_frames_reencode_exactly() {
    let classes = dwg_engine::decode_r2004_classes(FIXTURE_BYTES).expect("fixture classes must decode");
    let verified = dwg_engine::verify_r2004_visual_style_frames(FIXTURE_BYTES, &classes).expect("typed VISUALSTYLE frames must reencode exactly");
    assert_eq!(verified, 19, "all fixture VISUALSTYLE frames must be typed and exact");
}

#[test]
fn block_parameter_dependency_body_frames_reencode_exactly() {
    let classes = dwg_engine::decode_r2004_classes(FIXTURE_BYTES).expect("fixture classes must decode");
    let verified = dwg_engine::verify_r2004_block_parameter_dependency_body_frames(FIXTURE_BYTES, &classes).expect("typed BLOCKPARAMDEPENDENCYBODY frames must reencode exactly");
    assert_eq!(verified, 6, "all fixture BLOCKPARAMDEPENDENCYBODY frames must be typed and exact");
}

#[test]
fn block_representation_data_frames_reencode_exactly() {
    let classes = dwg_engine::decode_r2004_classes(FIXTURE_BYTES).expect("fixture classes must decode");
    let verified = dwg_engine::verify_r2004_block_representation_data_frames(FIXTURE_BYTES, &classes).expect("typed ACDB_BLOCKREPRESENTATION_DATA frames must reencode exactly");
    assert_eq!(verified, 12, "all fixture ACDB_BLOCKREPRESENTATION_DATA frames must be typed and exact");
}

#[test]
fn dynamic_block_purge_preventer_frames_reencode_exactly() {
    let classes = dwg_engine::decode_r2004_classes(FIXTURE_BYTES).expect("fixture classes must decode");
    let verified = dwg_engine::verify_r2004_dynamic_block_purge_preventer_frames(FIXTURE_BYTES, &classes).expect("typed ACDB_DYNAMICBLOCKPURGEPREVENTER_VERSION frames must reencode exactly");
    assert_eq!(verified, 2, "both fixture ACDB_DYNAMICBLOCKPURGEPREVENTER_VERSION frames must be typed and exact");
}

#[test]
fn evaluation_graph_frames_reencode_exactly() {
    let classes = dwg_engine::decode_r2004_classes(FIXTURE_BYTES).expect("fixture classes must decode");
    let verified = dwg_engine::verify_r2004_evaluation_graph_frames(FIXTURE_BYTES, &classes).expect("typed ACAD_EVALUATION_GRAPH frames must reencode exactly");
    assert_eq!(verified, 2, "fixture must expose both typed evaluation graphs");
}

#[test]
fn block_flip_parameter_frames_reencode_exactly() {
    let classes = dwg_engine::decode_r2004_classes(FIXTURE_BYTES).expect("fixture classes must decode");
    let verified = dwg_engine::verify_r2004_block_flip_parameter_frames(FIXTURE_BYTES, &classes).expect("typed BLOCKFLIPPARAMETER frames must reencode exactly");
    assert_eq!(verified, 3, "fixture must expose all three typed block flip parameters");
}

#[test]
fn block_visibility_parameter_frames_reencode_exactly() {
    let classes = dwg_engine::decode_r2004_classes(FIXTURE_BYTES).expect("fixture classes must decode");
    let verified = dwg_engine::verify_r2004_block_visibility_parameter_frames(FIXTURE_BYTES, &classes).expect("typed BLOCKVISIBILITYPARAMETER frame must reencode exactly");
    assert_eq!(verified, 1, "fixture must expose its typed block visibility parameter");
}

#[test]
fn placeholder_and_dictionary_variable_frames_reencode_exactly() {
    let classes = dwg_engine::decode_r2004_classes(FIXTURE_BYTES).expect("fixture classes must decode");
    let placeholders = dwg_engine::verify_r2004_placeholder_frames(FIXTURE_BYTES, &classes).expect("typed ACDBPLACEHOLDER frame must reencode exactly");
    let variables = dwg_engine::verify_r2004_dictionary_variable_frames(FIXTURE_BYTES, &classes).expect("typed DICTIONARYVAR frames must reencode exactly");
    assert_eq!(placeholders, 1, "fixture must expose its typed placeholder");
    assert_eq!(variables, 8, "fixture must expose all typed dictionary variables");
}

#[test]
fn annotation_scale_frames_reencode_exactly() {
    let classes = dwg_engine::decode_r2004_classes(FIXTURE_BYTES).expect("fixture classes must decode");
    let verified = dwg_engine::verify_r2004_annotation_scale_frames(FIXTURE_BYTES, &classes).expect("typed SCALE frames must reencode exactly");
    assert_eq!(verified, 17, "fixture must expose all typed annotation scales");
}

#[test]
fn sort_entities_table_frames_reencode_exactly() {
    let classes = dwg_engine::decode_r2004_classes(FIXTURE_BYTES).expect("fixture classes must decode");
    let verified = dwg_engine::verify_r2004_sort_entities_table_frames(FIXTURE_BYTES, &classes).expect("typed SORTENTSTABLE frames must reencode exactly");
    assert_eq!(verified, 7, "fixture must expose all typed sort-entities tables");
}

#[test]
fn table_style_frames_reencode_exactly() {
    let classes = dwg_engine::decode_r2004_classes(FIXTURE_BYTES).expect("fixture classes must decode");
    let verified = dwg_engine::verify_r2004_table_style_frames(FIXTURE_BYTES, &classes).expect("typed TABLESTYLE frame must reencode exactly");
    assert_eq!(verified, 1, "fixture must expose its typed table style");
}

#[test]
fn mline_and_mleader_style_frames_reencode_exactly() {
    let classes = dwg_engine::decode_r2004_classes(FIXTURE_BYTES).expect("fixture classes must decode");
    let mline = dwg_engine::verify_r2004_mline_style_frames(FIXTURE_BYTES, &classes).expect("typed MLINESTYLE frame must reencode exactly");
    let mleader = dwg_engine::verify_r2004_mleader_style_frames(FIXTURE_BYTES, &classes).expect("typed MLEADERSTYLE frame must reencode exactly");
    assert_eq!(mline, 1, "fixture must expose its typed multiline style");
    assert_eq!(mleader, 1, "fixture must expose its typed multileader style");
}

#[test]
fn material_frames_reencode_exactly() {
    let classes = dwg_engine::decode_r2004_classes(FIXTURE_BYTES).expect("fixture classes must decode");
    let verified = dwg_engine::verify_r2004_material_frames(FIXTURE_BYTES, &classes).expect("typed MATERIAL frames must reencode exactly");
    assert_eq!(verified, 3, "fixture must expose all typed materials");
}

#[test]
fn block_move_action_frames_reencode_exactly() {
    let classes = dwg_engine::decode_r2004_classes(FIXTURE_BYTES).expect("fixture classes must decode");
    let verified = dwg_engine::verify_r2004_block_move_action_frames(FIXTURE_BYTES, &classes).expect("typed BLOCKMOVEACTION frames must reencode exactly");
    assert_eq!(verified, 2, "fixture must expose both typed block move actions");
}

#[test]
fn assoc_network_frames_reencode_exactly() {
    let classes = dwg_engine::decode_r2004_classes(FIXTURE_BYTES).expect("fixture classes must decode");
    let verified = dwg_engine::verify_r2004_assoc_network_frames(FIXTURE_BYTES, &classes).expect("typed ACDBASSOCNETWORK frames must reencode exactly");
    assert_eq!(verified, 5, "fixture must expose all typed associative networks");
}

#[test]
fn assoc_2d_constraint_group_frames_reencode_exactly() {
    let classes = dwg_engine::decode_r2004_classes(FIXTURE_BYTES).expect("fixture classes must decode");
    let verified = dwg_engine::verify_r2004_assoc_2d_constraint_group_frames(FIXTURE_BYTES, &classes).expect("typed ACDBASSOC2DCONSTRAINTGROUP frames must reencode exactly");
    assert_eq!(verified, 4, "fixture must expose all typed 2D constraint groups");
}

#[test]
fn dynamic_linear_and_grip_frames_reencode_exactly() {
    let classes = dwg_engine::decode_r2004_classes(FIXTURE_BYTES).expect("fixture classes must decode");
    let verified = dwg_engine::verify_r2004_dynamic_linear_grip_frames(FIXTURE_BYTES, &classes).expect("typed dynamic linear/grip frames must reencode exactly");
    assert_eq!(verified, (2, 4, 3, 1), "fixture must expose all typed dynamic linear/grip frames");
}

#[test]
fn alignment_and_action_frames_reencode_exactly() {
    let classes = dwg_engine::decode_r2004_classes(FIXTURE_BYTES).expect("fixture classes must decode");
    let verified = dwg_engine::verify_r2004_alignment_action_frames(FIXTURE_BYTES, &classes).expect("typed alignment/action frames must reencode exactly");
    assert_eq!(verified, (2, 2, 6, 1, 3), "fixture must expose all typed alignment/action frames");
}

#[test]
fn final_parameter_and_layout_frames_reencode_exactly() {
    let classes = dwg_engine::decode_r2004_classes(FIXTURE_BYTES).expect("fixture classes must decode");
    let verified = dwg_engine::verify_r2004_final_parameter_layout_frames(FIXTURE_BYTES, &classes).expect("typed final parameter/layout frames must reencode exactly");
    assert_eq!(verified, (1, 1, 1, 2), "fixture must expose all final parameter/layout frames");
}

#[test]
fn object_and_handle_sections_materialize_exactly() {
    let classes = dwg_engine::decode_r2004_classes(FIXTURE_BYTES).expect("fixture classes must decode");
    let verified = dwg_engine::verify_r2004_object_handle_sections(FIXTURE_BYTES, &classes).expect("logical objects and derived Handles must materialize exactly");
    assert_eq!(verified, (663, 213_182, 2_085));
}

#[test]
fn d2_payloads_materialize_exactly() {
    assert_eq!(dwg_engine::verify_r2004_d2_payloads(FIXTURE_BYTES).expect("all ordinary D2 payloads must materialize exactly"), 15);
}

#[test]
fn d2_pages_materialize_exactly() {
    assert_eq!(dwg_engine::verify_r2004_d2_pages(FIXTURE_BYTES).expect("all ordinary D2 pages must materialize exactly"), 15);
}

#[test]
fn remaining_named_semantic_sections_reencode_exactly() {
    assert_eq!(dwg_engine::verify_r2004_named_semantic_sections(FIXTURE_BYTES).expect("typed named sections must reencode exactly"), (123, 16, 86_191, 1_390));
}

#[test]
fn header_semantic_section_reencodes_exactly() {
    assert_eq!(dwg_engine::verify_r2004_header_semantic(FIXTURE_BYTES).expect("typed Header section must reencode exactly"), (896, 8_845));
}

#[test]
fn twelve_named_sections_materialize_from_snapshot_exactly() {
    let snapshot = decode_dwg(FIXTURE_BYTES).expect("fixture snapshot");
    assert_eq!(dwg_engine::verify_r2004_materialized_named_sections(&snapshot, FIXTURE_BYTES).expect("logical named-section materialization"), 12);
}

#[test]
fn first_nineteen_ordinary_pages_materialize_exactly() {
    let snapshot = decode_dwg(FIXTURE_BYTES).expect("fixture snapshot");
    assert_eq!(dwg_engine::verify_r2004_ordinary_prefix(&snapshot, FIXTURE_BYTES).expect("ordinary native prefix"), (19, 0x23b80));
}

#[test]
fn system_directories_materialize_exactly() {
    let snapshot = decode_dwg(FIXTURE_BYTES).expect("fixture snapshot");
    assert_eq!(dwg_engine::verify_r2004_system_directories(&snapshot, FIXTURE_BYTES).expect("derived Section Info and Section Map"), (14, 20, 1_684, 176));
}

#[test]
fn associative_dependency_logical_frames_reencode_exactly() {
    let classes = dwg_engine::decode_r2004_classes(FIXTURE_BYTES).expect("fixture classes must decode");
    let verified = dwg_engine::verify_r2004_associative_dependency_frames(FIXTURE_BYTES, &classes).expect("typed ACDBASSOCDEPENDENCY frames must reencode exactly");
    assert_eq!(verified, 20, "all fixture ACDBASSOCDEPENDENCY frames must be typed and exact");
}

#[test]
fn associative_value_dependency_logical_frames_reencode_exactly() {
    let classes = dwg_engine::decode_r2004_classes(FIXTURE_BYTES).expect("fixture classes must decode");
    let verified = dwg_engine::verify_r2004_associative_value_dependency_frames(FIXTURE_BYTES, &classes).expect("typed ACDBASSOCVALUEDEPENDENCY frames must reencode exactly");
    assert_eq!(verified, 26, "all fixture ACDBASSOCVALUEDEPENDENCY frames must be typed and exact");
}

#[test]
fn associative_geometry_dependency_logical_frames_reencode_exactly() {
    let classes = dwg_engine::decode_r2004_classes(FIXTURE_BYTES).expect("fixture classes must decode");
    let verified = dwg_engine::verify_r2004_associative_geometry_dependency_frames(FIXTURE_BYTES, &classes).expect("typed ACDBASSOCGEOMDEPENDENCY frames must reencode exactly");
    assert_eq!(verified, 31, "all fixture ACDBASSOCGEOMDEPENDENCY frames must be typed and exact");
}

#[test]
fn block_grip_location_component_logical_frames_reencode_exactly() {
    let classes = dwg_engine::decode_r2004_classes(FIXTURE_BYTES).expect("fixture classes must decode");
    let verified = dwg_engine::verify_r2004_block_grip_location_component_frames(FIXTURE_BYTES, &classes).expect("typed BLOCKGRIPLOCATIONCOMPONENT frames must reencode exactly");
    assert_eq!(verified, 23, "all fixture BLOCKGRIPLOCATIONCOMPONENT frames must be typed and exact");
}

#[test]
fn dynamic_block_proxy_node_logical_frame_reencodes_exactly() {
    let classes = dwg_engine::decode_r2004_classes(FIXTURE_BYTES).expect("fixture classes must decode");
    let verified = dwg_engine::verify_r2004_dynamic_block_proxy_node_frames(FIXTURE_BYTES, &classes).expect("typed ACDB_DYNAMICBLOCKPROXYNODE frame must reencode exactly");
    assert_eq!(verified, 1, "the fixture ACDB_DYNAMICBLOCKPROXYNODE frame must be typed and exact");
}

#[test]
fn associative_variable_logical_frames_reencode_exactly() {
    let classes = dwg_engine::decode_r2004_classes(FIXTURE_BYTES).expect("fixture classes must decode");
    let verified = dwg_engine::verify_r2004_associative_variable_frames(FIXTURE_BYTES, &classes).expect("typed ACDBASSOCVARIABLE frames must reencode exactly");
    assert_eq!(verified, 20, "all fixture ACDBASSOCVARIABLE frames must be typed and exact");
}

#[test]
fn associative_dimension_dependency_body_frames_reencode_exactly() {
    let classes = dwg_engine::decode_r2004_classes(FIXTURE_BYTES).expect("fixture classes must decode");
    let verified = dwg_engine::verify_r2004_associative_dimension_dependency_body_frames(FIXTURE_BYTES, &classes).expect("typed ASSOCDIMDEPENDENCYBODY frames must reencode exactly");
    assert_eq!(verified, 14, "all fixture ASSOCDIMDEPENDENCYBODY frames must be typed and exact");
}

/// 🧪️ Logical decode followed by deterministic materialization is byte-identical.
#[test]
fn real_decode_stays_lossless_on_reencode() {
    let snap = decode_dwg(FIXTURE_BYTES).expect("real fixture must decode");
    let reencoded = crate::artifacts::dwg::schema::snapshot::encode_dwg(&snap).expect("re-encode");
    assert_fixture_bytes(&reencoded, "re-encode");
}

#[test]
fn snapshot_pack_preserves_signed_zero_semantics() {
    let original=decode_dwg(FIXTURE_BYTES).expect("real fixture must decode");
    let restored=DwgSnapshot::decode_pack(&original.encode_pack()).expect("snapshot pack roundtrip");
    let expected=serde_json::to_string(&original.drawing).expect("original drawing JSON");
    let actual=serde_json::to_string(&restored.drawing).expect("restored drawing JSON");
    assert!(expected.contains("\"value\":-0.0"), "fixture must exercise negative zero");
    assert_eq!(actual, expected, "snapshot pack must preserve signed zero");
}

#[test]
fn exact_fixture_roundtrips_through_snapshot_diff_mutation_and_raw_io() {
    let binary = BinarySnapshot { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes: FIXTURE_BYTES.to_vec() };
    let original = raw_import::deserialize(&binary).expect("raw DWG import");
    let exported = raw_export::serialize(&original).expect("raw DWG export").bytes;
    assert_fixture_bytes(&exported, "raw import/export");

    let dsl = original.print_dsl();
    for forbidden in ["section-names", "decode-status", "compressed", "encrypted", "page-number", "start-offset", "declared-size", "decompressed-size", "bytes=", "drawing={entities="] {
        assert!(!dsl.contains(forbidden), "snapshot DSL retained forbidden DWG shadow term {forbidden}");
    }
    let from_dsl = DwgSnapshot::parse_dsl(&dsl).expect("snapshot DSL roundtrip");
    assert_fixture_bytes(&encode_dwg(&from_dsl).expect("DSL-restored export"), "DSL-restored export");

    let pack = original.encode_pack();
    let pack_text = String::from_utf8_lossy(&pack);
    for forbidden in ["sectionNames", "decodeStatus", "compressed", "encrypted", "pageNumber", "startOffset", "declaredSize", "decompressedSize", "bytes_wire", "DwgLogicalEntity"] {
        assert!(!pack_text.contains(forbidden), "snapshot pack retained forbidden DWG shadow term {forbidden}");
    }
    let from_pack = DwgSnapshot::decode_pack(&pack).expect("snapshot unpack");
    let restored_json=serde_json::to_string(&from_pack.drawing).expect("restored drawing JSON");
    let expected_json=serde_json::to_string(&original.drawing).expect("original drawing JSON");
    assert_eq!(restored_json, expected_json, "snapshot pack must preserve signed numeric semantics");
    assert_fixture_bytes(&encode_dwg(&from_pack).expect("pack-restored export"), "pack-restored export");

    let analysis=DwgAnalyzer::analyze(&[AnalyzeSource::Text(&dsl)]);
    let analyzed=analysis.parts.snapshot.expect("analyzer snapshot");
    assert_fixture_bytes(&encode_dwg(&analyzed).expect("analyzer export"), "analyzer export");
    let dialect=Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1024"), subset: SubsetId("*") };
    let composition=dwg_engine::DwgComposerComposition::compose(&[ComposeSource { dialect, payload: AnalyzeSource::Binary(&pack) }]).expect("composer snapshot");
    assert_fixture_bytes(&encode_dwg(&composition.snapshot).expect("composer export"), "composer export");

    let empty = DwgDiff::between(&original, &original);
    assert!(empty.is_empty());
    assert_fixture_bytes(&encode_dwg(&empty.apply(&original)).expect("empty diff export"), "empty diff export");

    let mut no_op = original.clone();
    let no_op_diff = apply_dwg_mutation(&mut no_op, &DwgMutation::NoMutation);
    assert!(no_op_diff.is_empty());
    assert_fixture_bytes(&encode_dwg(&no_op).expect("no-op mutation export"), "no-op mutation export");

    let header_change = DwgMutation::SetVersionInfo { version: original.version.clone(), maintenance_version: original.maintenance_version.wrapping_add(1), codepage: original.codepage.wrapping_add(1) };
    let header_diff = header_change.diff(&original);
    let changed = header_diff.apply(&original);
    let changed_bytes = encode_dwg(&changed).expect("byte-patched header mutation export");
    let redecoded = decode_dwg(&changed_bytes).expect("mutated header re-decode");
    assert_eq!(redecoded.maintenance_version, changed.maintenance_version);
    assert_eq!(redecoded.codepage, changed.codepage);

    let inverse_diff = header_diff.inverse(&original);
    assert_fixture_bytes(&encode_dwg(&inverse_diff.apply(&changed)).expect("inverse diff export"), "inverse diff export");
    let mut absorbed = header_diff;
    absorbed.absorb(inverse_diff);
    assert_fixture_bytes(&encode_dwg(&absorbed.apply(&original)).expect("absorbed inverse export"), "absorbed inverse export");
}

#[test]
fn persisted_dwg_facets_have_no_parallel_entity_projection() {
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

#[test]
fn semantic_metadata_edits_materialize_from_logical_content() {
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
