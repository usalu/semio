//! 🔤️ Native paged key and scalar text parity under exact allocation-free copy grants.
use super::*;
use semio_framework_value::{paged::PagedUtf8, retained_clone::{RetainedCloneGrant, RetainedCloneStep}, retirement::controlled::ControlledRetirement};

#[derive(semio_framework_value_derive::RetireOwned)]
struct TextRoot { key: PagedUtf8<{usize::MAX}>, value: PagedUtf8<{usize::MAX}> }
impl super::ArtifactCanonicalJson for TextRoot {
    fn canonical_json_node(&self, path: &[usize]) -> Result<super::ArtifactCanonicalJsonNode<'_>, semio_framework_value::ValueError> {
        match path {
            [] => Ok(super::ArtifactCanonicalJsonNode::Object(1)),
            [0] => Ok(super::ArtifactCanonicalJsonNode::Text(ArtifactCanonicalJsonText::Native(&self.value))),
            _ => Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "native text fixture path")),
        }
    }
    fn canonical_json_key(&self, path: &[usize], index: usize) -> Result<ArtifactCanonicalJsonText<'_>, semio_framework_value::ValueError> {
        if path.is_empty() && index == 0 { Ok(ArtifactCanonicalJsonText::Native(&self.key)) } else { Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "native text fixture key")) }
    }
}

#[test]
fn canonical_native_paged_utf8_key_and_scalar_match_serde_with_zero_heap_copy_turns() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🎒️pack/🔤️json/🛫️encode/🔤️text/🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let key: String = row["keyChunks"].as_array().unwrap().iter().map(|part| part.as_str().unwrap()).collect();
        let value: String = row["valueChunks"].as_array().unwrap().iter().map(|part| part.as_str().unwrap()).collect();
        let oracle = serde_json::to_vec(&serde_json::json!({ key.as_str(): value })).unwrap();
        let root = TextRoot { key: PagedUtf8::from(key), value: PagedUtf8::from(value) };
        for maximum in fixture["copyGrants"].as_array().unwrap() {
            let maximum = maximum.as_u64().unwrap() as usize;
            let mut cursor = super::ArtifactCanonicalJsonCursor::default();
            let mut output = Vec::new();
            while !cursor.is_complete() {
                let mut chunk = [0; 256];
                let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| cursor.encode_chunk_admitted(&root, &mut chunk[..maximum.min(256)],RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64}).map(|step|step.written_bytes));
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                let count = result.unwrap();
                assert!(count <= maximum.min(256));
                output.extend_from_slice(&chunk[..count]);
            }
            assert_eq!(output, oracle);
        }
        let mut retirement = ControlledRetirement::new(root).unwrap_or_else(|_| panic!("native text record retirement"));
        for turn in 0..100_000 {
            if retirement.terminal_is_empty() { println!("[DEBUG] native canonical key/scalar Unicode/NUL/escaping matches Serde; every copy turn zero heap, actual owner closure {turn} turns"); break; }
            let copy = retirement.next_copy_byte_demand().unwrap();
            let capacity = retirement.next_capacity_byte_demand(copy).unwrap();
            assert!(copy + capacity <= 4096);
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: capacity, maximum_release_bytes: retirement.next_release_byte_demand().unwrap(), maximum_depth: retirement.next_depth_demand().unwrap() };
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| retirement.step(grant).unwrap());
            assert!(step.progress().fits(grant));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
            if matches!(step, RetainedCloneStep::Complete(_)) { assert!(retirement.terminal_is_empty()); }
        }
        assert!(retirement.terminal_is_empty());
    }
}
