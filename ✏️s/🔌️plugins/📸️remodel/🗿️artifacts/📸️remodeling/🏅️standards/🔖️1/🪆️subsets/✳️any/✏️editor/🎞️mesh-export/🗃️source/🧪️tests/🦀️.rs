//! 🧪️ Neutral mesh handle counts compare the paid parser with Rust's integer parser.
use super::*;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::default_remodeling_scene;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::ByteBuffer;

#[test]
fn original_handle_count_is_incremental_and_matches_the_reference_parser() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️counts/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let text = case["text"].as_str().unwrap();
        let reference = text.parse::<u64>();
        assert_eq!(reference.is_err(), case["refused"] == true, "{text}");
        let mut scene = default_remodeling_scene();
        let handle = &mut scene.results.mesh.mesh;
        handle.target.artifact_id = format!("remodeling-mesh-content:{text}");
        let mut parser = HandleCountCursor::new();
        let mut actual = None;
        let mut refused = false;
        for _ in 0..=handle.target.artifact_id.len() {
            for (items, copy, depth, fuel) in [(0, 64, 1, 1), (1, 0, 1, 1), (1, 64, 0, 1), (1, 64, 1, 0)] {
                let denied = parser.advance(handle, items, copy, depth, fuel).unwrap();
                assert_eq!((denied.complete, denied.copied_bytes, denied.work), (false, 0, 0));
            }
            match parser.advance(handle, 1, 64, 1, 1) {
                Ok(turn) => { assert_eq!(turn.work, 1); assert!(turn.copied_bytes <= 64); if turn.complete { actual = turn.value; break; } }
                Err(error) => { assert_eq!(error.work, 1); refused = true; break; }
            }
        }
        assert_eq!(refused, reference.is_err(), "{text}");
        if let Ok(value) = reference {
            assert_eq!(actual, Some(value), "{text}");
            assert_eq!(value.to_string(), case["value"].as_str().unwrap());
            let other = handle.clone();
            assert!(matches!(parser.advance(&other, 1, 64, 1, 1), Err(SourceFailure { reason: "mesh handle source changed", work: 1 })));
        }
    }
}

#[test]
fn constant_handle_comparisons_report_bounded_byte_work() {
    for (id, empty, boxed) in [("remodeling-mesh-constant-empty", true, false), ("remodeling-mesh-constant-box", false, true), ("remodeling-mesh-constant-emptX", false, false), ("foreign", false, false)] {
        let (actual_empty, actual_boxed, work) = constant_mesh(id);
        assert_eq!((actual_empty, actual_boxed), (empty, boxed));
        assert!((2..=64).contains(&work));
    }
}

#[test]
fn original_empty_handle_needs_no_artifact_and_obeys_denied_turns() {
    let handle = semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::empty_remodeling_mesh_handle();
    let mut index = MeshIndex::new();
    for (items, copy, depth, fuel) in [(0, 128, 1, 80), (1, 0, 1, 80), (1, 128, 0, 80), (1, 128, 1, 0), (1, 128, 1, 79)] {
        let denied = index.advance(&handle, 0, None, items, copy, 0, depth, fuel).unwrap();
        assert_eq!((denied.complete, denied.copied_bytes, denied.work), (false, 0, 0));
    }
    assert!(index.advance(&handle, 0, None, 1, 128, 0, 1, 80).unwrap().complete);
    let source = index.source(None);
    assert!(source.valid_layout());
    assert_eq!((source.vertices(), source.indices()), (0, 0));
    index.begin_close();
    assert!(index.terminal_is_empty());
}

#[test]
fn original_mesh_leaves_are_indexed_under_bounded_turns() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🎞️mesh-export/🔣️.json")).unwrap();
    let positions: Vec<f32> = fixture["positions"].as_array().unwrap().iter().map(|value| value.as_f64().unwrap() as f32).collect();
    let indices: Vec<u32> = fixture["indices"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as u32).collect();
    let mut position_leaf = vec![0];
    for value in &positions { position_leaf.extend_from_slice(&value.to_le_bytes()); }
    let mut index_leaf = vec![3];
    for value in &indices { index_leaf.extend_from_slice(&value.to_le_bytes()); }
    let artifact = RemodelingDurableArtifact { kind: "mesh".into(), mime: None, width: 0, height: 0, chunks: vec![ByteBuffer::from_u8_slice(&position_leaf), ByteBuffer::from_u8_slice(&index_leaf)] };
    let mut scene = default_remodeling_scene();
    let handle = &mut scene.results.mesh.mesh;
    handle.child_id = "original-mesh".into();
    let mut index = MeshIndex::new();
    let bad = RemodelingDurableArtifact { kind: "mesh".into(), mime: None, width: 0, height: 0, chunks: vec![ByteBuffer::from_u8_slice(&[255])] };
    assert!(matches!(index.advance(handle, 1, Some(&bad), 1, 128, 6144, 1, 80), Err(SourceFailure { reason: "invalid mesh field order", work }) if work > 0 && work <= 80));
    let mut complete = false;
    for _ in 0..=positions.len() + indices.len() + 7 {
        for (items, copy, depth, fuel) in [(0, 128, 1, 80), (1, 0, 1, 80), (1, 128, 0, 80), (1, 128, 1, 0), (1, 128, 1, 79)] {
            let denied = index.advance(handle, 2, Some(&artifact), items, copy, 6144, depth, fuel).unwrap();
            assert_eq!((denied.complete, denied.copied_bytes, denied.retained_capacity_bytes, denied.work), (false, 0, 0, 0));
        }
        if index.next_capacity_demand() != 0 {
            let denied = index.advance(handle, 2, Some(&artifact), 1, 128, 0, 1, 80).unwrap();
            assert_eq!((denied.complete, denied.copied_bytes, denied.retained_capacity_bytes, denied.work), (false, 0, 0, 0));
        }
        let turn = index.advance(handle, 2, Some(&artifact), 1, 128, 6144, 1, 80).unwrap();
        assert!(turn.copied_bytes <= 128 && turn.retained_capacity_bytes <= 6144 && turn.work <= 80);
        if turn.complete { complete = true; break; }
    }
    assert!(complete);
    let source = index.source(Some(&artifact));
    assert!(source.valid_layout());
    for (i, expected) in positions.chunks_exact(3).enumerate() { let (actual, work) = source.position(i); assert_eq!(actual.unwrap().as_slice(), expected); assert!(work <= source.read_work()); }
    for (i, expected) in indices.iter().enumerate() { let (actual, work) = source.index(i); assert_eq!(actual, Some(*expected)); assert!(work <= source.read_work()); }
    let other = artifact.clone();
    assert!(!index.source(Some(&other)).valid_layout());
    index.begin_close();
    while !index.terminal_is_empty() {
        let release = index.next_release_demand();
        let denied = index.close_step(1, size_of::<Vec<u32>>(), 0, 1, 1).unwrap();
        assert_eq!((denied.complete, denied.copied_bytes, denied.released_bytes, denied.work), (false, 0, 0, 0));
        let turn = index.close_step(1, size_of::<Vec<u32>>(), release, 1, 1).unwrap();
        assert_eq!((turn.released_bytes, turn.work), (release, 1));
    }
}

#[test]
fn ordered_lookup_retains_the_real_root_and_closes_before_its_store() {
    use semio_framework_value::retirement::controlled::ControlledRetirement;
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 4096, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 1 << 20, maximum_depth: 4096 };
    let mut store = RemodelingOrderedDurableArtifactStore::new();
    let artifact = RemodelingDurableArtifact { kind: "mesh".into(), mime: None, width: 0, height: 0, chunks: vec![ByteBuffer::from_u8_slice(&[0, 1, 2, 3, 4])] };
    assert!(store.insert("mesh-original".into(), artifact.clone()).is_none());
    let mut scene = default_remodeling_scene();
    let handle = &mut scene.results.mesh.mesh;
    handle.child_id = "mesh-original".into();
    let denied = OriginalMeshLookup::admit(&store, handle, grant, 0);
    assert_eq!(denied.work, 0);
    assert!(denied.result.is_err());
    let born = OriginalMeshLookup::admit(&store, handle, grant, 1);
    assert_eq!(born.work, 1);
    let (mut lookup, progress) = born.result.unwrap();
    assert!(progress.fits(grant));
    while !lookup.is_complete() {
        let demand = lookup.next_demand(handle).unwrap();
        assert!(demand.copy_bytes <= grant.maximum_copy_bytes && demand.depth <= grant.maximum_depth);
        let denied = lookup.advance(handle, RetainedCloneGrant { maximum_items: 0, ..grant }, 1);
        assert_eq!(denied.work, 0);
        assert_eq!(denied.result.unwrap().progress(), RetainedCloneProgress::default());
        let turn = lookup.advance(handle, grant, 1);
        assert!(turn.work <= 1);
        assert!(turn.result.unwrap().progress().fits(grant));
    }
    assert_eq!(lookup.result(), Some(&artifact));
    let other = handle.clone();
    assert!(lookup.next_demand(&other).is_err());
    lookup.begin_close();
    while !lookup.terminal_is_empty() {
        let (step, work) = lookup.close_step(grant);
        assert!(work <= 1);
        match step {
            RetirementStep::Progress { released_bytes, .. } => assert!(released_bytes <= grant.maximum_release_bytes),
            RetirementStep::ProcessedBytes(bytes) => assert!(bytes <= grant.maximum_copy_bytes),
            RetirementStep::Complete => {},
            _ => panic!("original lookup did not retire under its grant"),
        }
    }
    drop(lookup);
    let mut close = ControlledRetirement::new(store).unwrap_or_else(|_| panic!("ordered store supports paid close"));
    while !close.terminal_is_empty() { assert!(close.step(grant).unwrap().progress().fits(grant)); }
}
