//! 🔬️ Original typed Brep source bytes match ordered serde JSON and ordinary operation frames without heap work.
use crate::standards::v1::subsets::brep::schema::{mutations::{SemioBrepMutation, set_snapshot::SetSnapshot}, snapshot::SemioBrepSnapshot};
use semio_framework_plugin::plugin_app_close_prelude::store::{ArtifactPreparedOperationCursor, ArtifactPreparedOperationProgress};
use semio_framework_value::{DslValue, FromValue, ToValue};
use semio_framework_value::retained_clone::RetainedCloneGrant;
use serde::ser::{SerializeMap, SerializeSeq};

struct Ordered<'a>(&'a DslValue);
impl serde::Serialize for Ordered<'_> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self.0 {
            DslValue::Null => serializer.serialize_none(),
            DslValue::Bool(value) => serializer.serialize_bool(*value),
            DslValue::Number(value) => match value {
                semio_framework_value::Number::UInt(value) => serializer.serialize_u64(*value),
                semio_framework_value::Number::Int(value) => serializer.serialize_i64(*value),
                semio_framework_value::Number::Float(value) => serializer.serialize_f64(*value),
            },
            DslValue::String(value) => serializer.serialize_str(value),
            DslValue::Bytes(value) => serializer.serialize_bytes(value),
            DslValue::Array(values) => {
                let mut sequence = serializer.serialize_seq(Some(values.len()))?;
                for value in values { sequence.serialize_element(&Ordered(value))?; }
                sequence.end()
            },
            DslValue::Object(values) => {
                let mut object = serializer.serialize_map(Some(values.len()))?;
                for (key, value) in values { object.serialize_entry(key, &Ordered(value))?; }
                object.end()
            },
        }
    }
}

#[test]
fn brep_borrowed_set_snapshot_imported_obj_retains_original_topology_and_wire_owner() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let input = &fixture["importedObj"];
    let mut kernel = semio_framework_3d::brep::engine::Brep::new();
    kernel.import_obj_sync(input["text"].as_str().unwrap(), input["tolerance"].as_f64().unwrap()).unwrap();
    let snapshot = crate::standards::v1::subsets::brep::schema::snapshot::body::snapshot_from_body(kernel.representation());
    let counts = [("vertices", snapshot.vertices.len()), ("edges", snapshot.edges.len()), ("loops", snapshot.loops.len()), ("faces", snapshot.faces.len()), ("shells", snapshot.shells.len()), ("solids", snapshot.solids.len()), ("coedges", snapshot.coedges.len())];
    for (key, count) in counts { assert_eq!(count, input["counts"][key].as_u64().unwrap() as usize); }
    let value = snapshot.to_value();
    let json = serde_json::to_vec(&Ordered(&value)).unwrap();
    assert_eq!(semio_framework_pack_json::to_json_string(&snapshot).as_bytes(), json);
    let mutation = SemioBrepMutation::SetSnapshot(SetSnapshot { snapshot });
    let expected = protocol::OpBinary::encode_op(&mutation).unwrap();
    let SemioBrepMutation::SetSnapshot(payload) = &mutation else { unreachable!() };
    let identity = &payload.snapshot as *const SemioBrepSnapshot;
    let mut cursor = ArtifactPreparedOperationCursor::default();
    let mut actual = Vec::with_capacity(expected.len());
    for _ in 0..expected.len() + 8 {
        let mut output = [0; 64];
        let (progress, heap) = crate::snapshot_test_backing::measure(|| cursor.advance(super::prepared_operation_wire_source(&mutation).unwrap(), &mut output, RetainedCloneGrant::one_payload_turn(64, 64)).unwrap());
        assert_eq!((heap.bytes, heap.released_bytes), (0, 0));
        assert_eq!((progress.retained_capacity_bytes, progress.released_bytes), (0, 0));
        assert!(progress.copied_bytes <= 64);
        actual.extend_from_slice(&output[..progress.written_bytes]);
        if progress.complete { break; }
    }
    assert_eq!(actual, expected);
    let semio_framework_plugin::plugin_app_close_prelude::store::ArtifactPreparedOperationSource::HexJson { body, .. } = super::prepared_operation_wire_source(&mutation).unwrap() else { panic!("original imported snapshot hex source"); };
    assert_eq!(body as *const dyn semio_framework_plugin::plugin_app_close_prelude::store::ArtifactCanonicalJson as *const (), identity as *const ());
    cursor.close(RetainedCloneGrant { maximum_items: 1, ..Default::default() }).unwrap();
    assert_eq!(payload.snapshot.to_value(), value);
    eprintln!("[DEBUG] imported OBJ original SetSnapshot exact4vertices6edges4faces12coedges wireBytes={} typedOwnerUnchanged=true heapCopy=0", actual.len());
}

#[test]
fn brep_borrowed_set_snapshot_matches_complete_neutral_serde_wire_without_heap_work() {
    let text = include_str!("🧫️fixtures/🔣️.json");
    let fixture: DslValue = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let policy: serde_json::Value = serde_json::from_str(text).unwrap();
    let snapshot = SemioBrepSnapshot::from_value(fixture.get("snapshot").unwrap().clone()).unwrap();
    let canonical = snapshot.to_value();
    let ordinary = semio_framework_pack_json::to_json_string(&snapshot);
    let independent = serde_json::to_vec(&Ordered(&canonical)).unwrap();
    assert_eq!(ordinary.as_bytes(), independent);
    let DslValue::Object(root) = &canonical else { panic!("typed snapshot object"); };
    let keys: Vec<_> = root.iter().map(|(key, _)| key.as_str()).collect();
    let expected_keys: Vec<_> = policy["rootKeys"].as_array().unwrap().iter().map(|key| key.as_str().unwrap()).collect();
    assert_eq!(keys, expected_keys);
    let mutation = SemioBrepMutation::SetSnapshot(SetSnapshot { snapshot });
    let expected = protocol::OpBinary::encode_op(&mutation).unwrap();
    assert_eq!(&expected[..2], &[1, 13]);
    assert_eq!(&expected[2..11], b"snapshot=");
    let independent_hex: String = independent.iter().map(|byte| format!("{byte:02x}")).collect();
    assert_eq!(&expected[11..], independent_hex.as_bytes());
    let source = || super::prepared_operation_wire_source(&mutation).expect("original SetSnapshot must publish its borrowed source");
    for maximum in policy["copyBytes"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as usize) {
        let (mut cursor, birth) = crate::snapshot_test_backing::measure(ArtifactPreparedOperationCursor::default);
        assert_eq!((birth.bytes, birth.released_bytes), (0, 0));
        let mut actual = Vec::with_capacity(expected.len());
        for _ in 0..expected.len() * 2 + 8 {
            let mut output = [0; 256];
            for grant in [RetainedCloneGrant::default(), RetainedCloneGrant { maximum_items: 0, maximum_copy_bytes: maximum, ..Default::default() }] {
                let (progress, heap) = crate::snapshot_test_backing::measure(|| cursor.advance(source(), &mut output, grant).unwrap());
                assert_eq!(progress, ArtifactPreparedOperationProgress::default());
                assert_eq!((heap.bytes, heap.released_bytes), (0, 0));
            }
            let (progress, heap) = crate::snapshot_test_backing::measure(|| cursor.advance(source(), &mut output, RetainedCloneGrant::one_payload_turn(maximum, 64)).unwrap());
            assert_eq!((heap.bytes, heap.released_bytes), (0, 0));
            assert!(!heap.overflow);
            assert_eq!(progress.processed_items, 1);
            assert!(progress.written_bytes <= maximum.min(64));
            assert!(progress.copied_bytes <= maximum.min(64));
            assert_eq!((progress.retained_capacity_bytes, progress.released_bytes), (0, 0));
            actual.extend_from_slice(&output[..progress.written_bytes]);
            if progress.complete { break; }
        }
        assert_eq!(actual, expected);
        assert_eq!(cursor.next_minimum_copy_bytes(), 0);
        assert_eq!((cursor.next_capacity_byte_demand().unwrap(), cursor.next_close_byte_demand().unwrap()), (0, 0));
        let (_, heap) = crate::snapshot_test_backing::measure(|| cursor.close(RetainedCloneGrant { maximum_items: 1, ..Default::default() }).unwrap());
        assert_eq!((heap.bytes, heap.released_bytes), (0, 0));
        eprintln!("[DEBUG] original Brep SetSnapshot serde wire bytes={} copy={} heapBirth=0 heapFree=0", actual.len(), maximum);
    }
    for stop in policy["cancelStops"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as usize) {
        let mut cursor = ArtifactPreparedOperationCursor::default();
        for _ in 0..stop { cursor.advance(source(), &mut [0; 64], RetainedCloneGrant::one_payload_turn(64, 64)).unwrap(); }
        let (progress, heap) = crate::snapshot_test_backing::measure(|| cursor.close(RetainedCloneGrant { maximum_items: 1, ..Default::default() }).unwrap());
        assert!(progress.complete);
        assert_eq!((progress.copied_bytes, progress.retained_capacity_bytes, progress.released_bytes), (0, 0, 0));
        assert_eq!((heap.bytes, heap.released_bytes), (0, 0));
        let SemioBrepMutation::SetSnapshot(payload) = &mutation else { unreachable!() };
        assert_eq!(payload.snapshot.to_value(), canonical);
        eprintln!("[DEBUG] original Brep SetSnapshot cancellation stop={} heapBirth=0 heapFree=0 ownerUnchanged=true", stop);
    }
}
