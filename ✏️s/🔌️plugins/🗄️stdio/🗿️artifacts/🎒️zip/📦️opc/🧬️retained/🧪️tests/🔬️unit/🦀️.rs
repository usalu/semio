use super::*;
use semio_framework_value::{NativeEncodeControl, SnapshotRetirementStep, ValueRefusalKind, native_encoding::NativeEncodeProgress, retained_clone::{RetainedClone, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneSource, RetainedCloneStep}, retirement::owned_retirement};
use std::sync::Arc;

fn package(fixture: &serde_json::Value) -> OpcPackage {
    let payload = &fixture["payload"];
    let length = payload["length"].as_u64().expect("payload length") as usize;
    let multiplier = payload["multiplier"].as_u64().expect("payload multiplier") as usize;
    let increment = payload["increment"].as_u64().expect("payload increment") as usize;
    let pattern: Vec<u8> = (0..length).map(|ordinal| ((ordinal * multiplier + increment) & 255) as u8).collect();
    let parts = fixture["parts"].as_array().expect("parts").iter().map(|part| OpcPart {
        path: part["path"].as_str().expect("part path").into(),
        content_type: part["contentType"].as_str().expect("part content type").into(),
        bytes: if part["payload"] == "pattern" { pattern.clone() } else { Vec::new() },
    }).collect();
    let pairs = |name: &str| fixture["contentTypes"][name].as_array().expect("content type entries").iter().map(|pair| {
        let pair = pair.as_array().expect("content type pair");
        (pair[0].as_str().expect("content type name").into(), pair[1].as_str().expect("content type value").into())
    }).collect();
    let mut relationships = OpcRelationshipOwners::new();
    for owner in fixture["relationships"].as_array().expect("relationship owners") {
        let entries = owner["entries"].as_array().expect("relationships").iter().map(|relationship| OpcRelationship {
            id: relationship["id"].as_str().expect("relationship id").into(),
            rel_type: relationship["relType"].as_str().expect("relationship type").into(),
            target: relationship["target"].as_str().expect("relationship target").into(),
            target_mode: if relationship["targetMode"] == "external" { OpcTargetMode::External } else { OpcTargetMode::Internal },
        }).collect();
        relationships.replace_owner(owner["owner"].as_str().expect("relationship owner").into(), entries);
    }
    OpcPackage {
        parts,
        content_types: OpcContentTypes { defaults: pairs("defaults"), overrides: pairs("overrides") },
        relationships,
        comment: fixture["comment"].as_str().expect("comment").into(),
    }
}

fn close_cursor(cursor: &mut <RetainedOpcPackage as RetainedClone>::Cursor, grant: RetainedCloneGrant) -> usize {
    assert!(cursor.begin_close());
    let mut turns = 0usize;
    loop {
        turns += 1;
        assert!(turns < 100_000, "retained OPC cursor close must terminate");
        match cursor.close_step(grant.maximum_items, grant.maximum_capacity_bytes).expect("retained OPC cursor close") {
            SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= grant.maximum_items);
                assert!(released_bytes <= grant.maximum_capacity_bytes);
            }
            SnapshotRetirementStep::Blocked => panic!("exact retained OPC cursor close grant blocked"),
            SnapshotRetirementStep::Complete => break,
        }
    }
    assert!(cursor.terminal_is_empty());
    turns
}

#[test]
fn retained_opc_copy_and_materialization_preserve_package_authority() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).expect("retained OPC fixture");
    let expected = package(&fixture);
    let retained = RetainedOpcPackage::try_from_package(expected.clone()).expect("retained OPC owner");
    let mut progress = Vec::new();
    let mut callback = |event: NativeEncodeProgress| { progress.push(event); true };
    let mut materialization = NativeEncodeControl::new(1_000_000, &mut callback);
    assert_eq!(retained.materialize_package(&mut materialization).expect("controlled retained OPC materialization"), expected);
    assert!(progress.iter().any(|event| event.completed > 0));
    let payload_length = fixture["payload"]["length"].as_u64().expect("payload length") as usize;
    let mut reached_cancellation = false;
    let mut cancel = |event: NativeEncodeProgress| {
        if event.total == payload_length && event.completed >= 256 && event.completed < event.total {
            reached_cancellation = true;
            false
        } else {
            true
        }
    };
    let error = retained.materialize_package(&mut NativeEncodeControl::new(1_000_000, &mut cancel)).expect_err("interior retained OPC materialization cancellation");
    assert_eq!(error.kind, ValueRefusalKind::Canceled);
    assert!(reached_cancellation);
    assert_eq!(retained.parts.iter().map(|part| part.path.to_string_owner()).collect::<Vec<_>>(), expected.parts.iter().map(|part| part.path.clone()).collect::<Vec<_>>());
    assert_eq!(retained.content_types.defaults.iter().map(|entry| entry.name.to_string_owner()).collect::<Vec<_>>(), expected.content_types.defaults.iter().map(|entry| entry.0.clone()).collect::<Vec<_>>());
    assert_eq!(retained.relationships.keys().cloned().collect::<Vec<_>>(), expected.relationships.groups().map(|(owner, _)| owner.clone()).collect::<Vec<_>>());
    assert_eq!(retained.relationships_for("word/document.xml").expect("document relationships").iter().map(|relationship| relationship.id.to_string_owner()).collect::<Vec<_>>(), vec!["rId2", "rId1"]);
    assert_eq!(retained.part("/word/media/image-large.bin").expect("retained part").bytes.len(), fixture["payload"]["length"].as_u64().expect("payload length") as usize);

    let grant_fixture = &fixture["grant"];
    let grant = RetainedCloneGrant {
        maximum_items: grant_fixture["maximumItems"].as_u64().expect("maximum items") as usize,
        maximum_copy_bytes: grant_fixture["maximumCopyBytes"].as_u64().expect("maximum copy bytes") as usize,
        maximum_capacity_bytes: grant_fixture["maximumCapacityBytes"].as_u64().expect("maximum capacity bytes") as usize,
        maximum_depth: grant_fixture["maximumDepth"].as_u64().expect("maximum depth") as usize, maximum_release_bytes: grant_fixture["maximumReleaseBytes"].as_u64().expect("maximum capacity bytes") as usize };
    let source = RetainedCloneSource::from_authority(Arc::new(retained), ());
    let mut cursor = RetainedOpcPackage::retained_clone_cursor();
    let mut turns = 0usize;
    loop {
        turns += 1;
        assert!(turns < 100_000, "retained OPC copy must terminate");
        if matches!(cursor.advance(source.borrow(), grant).expect("retained OPC copy"), RetainedCloneStep::Complete(_)) {
            break;
        }
    }
    assert!(turns > 1);
    let copied = cursor.take().expect("retained OPC copy owner");
    let mut callback = |_| true;
    assert_eq!(copied.materialize_package(&mut NativeEncodeControl::new(1_000_000, &mut callback)).expect("copied retained OPC materialization"), expected);
    assert_eq!(close_cursor(&mut cursor, grant), 1, "a completed spent cursor already drained every child scaffold during admitted copy turns");

    let mut interrupted = RetainedOpcPackage::retained_clone_cursor();
    assert!(matches!(interrupted.advance(source.borrow(), grant).expect("partial retained OPC copy"), RetainedCloneStep::Progress(_)));
    assert!(close_cursor(&mut interrupted, RetainedCloneGrant { maximum_items: 1, ..grant }) > 1);

    let mut retirement = owned_retirement(copied);
    for turn in 0..100_000 {
        match retirement.close_step(grant.maximum_items, grant.maximum_capacity_bytes).expect("retained OPC retirement") {
            SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= grant.maximum_items);
                assert!(released_bytes <= grant.maximum_capacity_bytes);
            }
            SnapshotRetirementStep::Blocked => panic!("exact retained OPC retirement grant blocked at turn {turn}"),
            SnapshotRetirementStep::Complete => break,
        }
    }
    assert!(retirement.terminal_is_empty());
    drop(source);
}
