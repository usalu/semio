//! 🧪️ Language-neutral identity vectors independently decoded with serde_json.
use super::*;



/// 👁️ Strict borrowed visitation admits whole identities before callbacks and owns no heap storage.
#[test]
fn scoped_identity_visitor_is_atomic_ordered_and_allocation_free() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let law: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/👁️visitor/🔣️.json")).unwrap();
    for (index, row) in vectors["valid"].as_array().unwrap().iter().enumerate() {
        let identity = row["identity"].as_str().unwrap();
        let mut visited = [""; SCOPED_IDENTITY_MAX_SEGMENTS];
        let mut callbacks = 0;
        let (count, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| visit_scoped_identity_segments(identity, |segment| { visited[callbacks] = segment; callbacks += 1; }).unwrap());
        assert_eq!(count, law["callbackCounts"][index].as_u64().unwrap() as usize);
        assert_eq!(count, callbacks);
        for (index, segment) in visited[..count].iter().enumerate() {
            assert_eq!(*segment, row["segments"][index].as_str().unwrap());
            assert!((segment.as_ptr() as usize) >= identity.as_ptr() as usize && (segment.as_ptr() as usize + segment.len()) <= identity.as_ptr() as usize + identity.len());
        }
        assert_eq!((heap.requested_bytes, heap.released_bytes), (law["requestedBytes"].as_u64().unwrap() as usize, law["releasedBytes"].as_u64().unwrap() as usize));
        let (encoded, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| encode_scoped_identity(&visited[..count]).unwrap());
        assert_eq!(encoded, identity);
        assert_eq!((heap.requested_bytes, heap.released_bytes), (encoded.capacity(), 0));
    }
    let maximum = format!("{PREFIX}{}", "1:x".repeat(SCOPED_IDENTITY_MAX_SEGMENTS));
    let mut callbacks = 0;
    assert_eq!(visit_scoped_identity_segments(&maximum, |_| callbacks += 1).unwrap(), law["maximumCallbacks"].as_u64().unwrap() as usize);
    assert_eq!(callbacks, SCOPED_IDENTITY_MAX_SEGMENTS);
    for identity in vectors["invalidIdentities"].as_array().unwrap().iter().map(|row| row.as_str().unwrap()).chain(std::iter::once("")) {
        let mut callbacks = 0;
        let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| visit_scoped_identity_segments(identity, |_| callbacks += 1));
        assert!(result.is_err());
        assert_eq!(callbacks, law["invalidCallbacks"].as_u64().unwrap() as usize);
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    }
    let overflow = format!("{PREFIX}{}", "1:x".repeat(SCOPED_IDENTITY_MAX_SEGMENTS + 1));
    let mut callbacks = 0;
    assert!(visit_scoped_identity_segments(&overflow, |_| callbacks += 1).is_err());
    assert_eq!(callbacks, law["overflowCallbacks"].as_u64().unwrap() as usize);
    println!("[DEBUG] Borrowed scoped visitor preserves ordered original slices, callbacks0 on whole-input refusal,32segments maximum and zero heap bytes; encoder owns one exact allocation");
}

#[test]
fn scoped_identities_follow_portable_vectors_and_bounds() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).expect("third-party JSON oracle");
    for row in fixture["valid"].as_array().expect("valid vectors") {
        let segments: Vec<_> = row["segments"].as_array().expect("segments").iter().map(|segment| segment.as_str().expect("segment")).collect();
        let identity = row["identity"].as_str().expect("identity");
        assert_eq!(encode_scoped_identity(&segments).expect("encode"), identity);
        assert_eq!(decode_scoped_identity(identity).expect("decode"), segments);
    }
    for row in fixture["invalidSegments"].as_array().expect("invalid segments") {
        let segments: Vec<_> = row.as_array().expect("segments").iter().map(|segment| segment.as_str().expect("segment")).collect();
        assert!(encode_scoped_identity(&segments).is_err());
    }
    for row in fixture["invalidIdentities"].as_array().expect("invalid identities") { assert!(decode_scoped_identity(row.as_str().expect("identity")).is_err(), "{row}"); }
    assert!(encode_scoped_identity(&["x"; SCOPED_IDENTITY_MAX_SEGMENTS + 1]).is_err());
    assert!(decode_scoped_identity(&format!("{PREFIX}{}", "1:x".repeat(SCOPED_IDENTITY_MAX_SEGMENTS + 1))).is_err());
    assert!(encode_scoped_identity(&[&"x".repeat(SCOPED_IDENTITY_MAX_BYTES)]).is_err());
    println!("[DEBUG] Scoped interaction identities preserve Unicode byte boundaries, case ownership, delimiters and bounded canonical refusal");
}
