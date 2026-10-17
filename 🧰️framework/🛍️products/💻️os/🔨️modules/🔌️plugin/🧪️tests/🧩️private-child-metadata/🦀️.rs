//! 🧳️ Original private child metadata/genesis custody law at its actual defining owner.
#[test]
fn child_emission_private_input_metadata_and_genesis_retain_exact_request_and_original_batch() {
    use crate::app_emission::{ChildEmit, ChildEmitGenesis};
    use crate::child_emit_preparation::OwnedChildEmit;
    use crate::child_content::PrivateChildPublicationInput;
    use semio_framework_artifact_reference::{ArtifactRef, ArtifactDialect};
    use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneStep};
    static DECLARATIONS: &[store::MemberOpenDeclaration] = &[
        store::MemberOpenDeclaration { kind: "test", standard: "1", subset: "*", schema: "demo/v1",history_limits:semio_framework_os_kernel::os_store::HistoryReadLimits{file_bytes:67108864,frame_body_bytes:1048576,records:8192,maximum_fields:128,maximum_text_bytes:1048576,maximum_binary_bytes:1048576,maximum_json_bytes:1048576,maximum_json_allocation_bytes:8388608,maximum_json_items:65536,maximum_json_depth:64},maximum_history_allocation_bytes:8388608 },
        store::MemberOpenDeclaration { kind: "kind-δ", standard: "1", subset: "*", schema: "demo/v1",history_limits:semio_framework_os_kernel::os_store::HistoryReadLimits{file_bytes:67108864,frame_body_bytes:1048576,records:8192,maximum_fields:128,maximum_text_bytes:1048576,maximum_binary_bytes:1048576,maximum_json_bytes:1048576,maximum_json_allocation_bytes:8388608,maximum_json_items:65536,maximum_json_depth:64},maximum_history_allocation_bytes:8388608 },
    ];
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../🏪️store/🧩️composition/🚪️open/🌱️genesis/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let reference = |value: &serde_json::Value| ArtifactRef { artifact_id: value["artifact_id"].as_str().unwrap().into(), dialect: ArtifactDialect { artifact_kind: value["dialect"]["artifact_kind"].as_str().unwrap().into(), standard: value["dialect"]["standard"].as_str().unwrap().into(), subset: value["dialect"]["subset"].as_str().unwrap().into() } };
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 64, maximum_capacity_bytes: 262144, maximum_release_bytes: 262144, maximum_depth: 64 };
    for row in corpus["cases"].as_array().unwrap() {
        let expected = reference(&row["expected"]);
        let parent = reference(&row["owner"]["parent"]);
        let slot = row["owner"]["slot"].as_str().unwrap();
        let child = "local-private-α";
        let pack_segment = row["initialPackHex"].as_str().unwrap().as_bytes().as_chunks::<2>().0.iter().map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()).collect::<Vec<_>>();
        for stop in [Some(0usize), Some(1), Some(3), Some(17), Some(65), Some(257), None] {
            let pack = pack_segment.repeat(row["packRepeats"].as_u64().unwrap() as usize);
            let pack_pointer = pack.as_ptr();
            let original = vec![17i32, 3, 9];
            let mutation_pointer = original.as_ptr();
            let (batch, _) = store::MemberStoreOwnedBatch::try_new(original, grant).unwrap();
            let metadata = ChildEmit { genesis: Some(ChildEmitGenesis { reference: reference(&row["expected"]), initial_pack: pack }), owner: String::new(), slot: slot.into(), child_id: child.into(), ops: Vec::new(), op_schema: semio_framework::kernel::SchemaId("private.owned".into()), labels: Vec::new() };
            let (mut input, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| PrivateChildPublicationInput::new(OwnedChildEmit::new(metadata, batch)));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            for turn in 0..100000 {
                if stop.is_some_and(|stop| turn >= stop) { break; }
                let capacity = input.next_capacity_byte_demand(&parent.artifact_id, &parent.dialect, None, "actor:private-genesis", None, None).unwrap();
                assert!(capacity <= grant.maximum_capacity_bytes);
                let paused = RetainedCloneGrant { maximum_items: 0, ..grant };
                let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| input.advance(&parent.artifact_id, &parent.dialect, None, "actor:private-genesis", None, None, DECLARATIONS, semio_framework_job::OperationId(7311), semio_framework_job::Generation(3), 1_000_000, 1, paused).unwrap());
                assert_eq!(step.progress(), Default::default());
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                if capacity != 0 {
                    let denied = RetainedCloneGrant { maximum_capacity_bytes: capacity - 1, ..grant };
                    let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| input.advance(&parent.artifact_id, &parent.dialect, None, "actor:private-genesis", None, None, DECLARATIONS, semio_framework_job::OperationId(7311), semio_framework_job::Generation(3), 1_000_000, 1, denied).unwrap());
                    assert_eq!(step.progress(), Default::default());
                    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                }
                let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| input.advance(&parent.artifact_id, &parent.dialect, None, "actor:private-genesis", None, None, DECLARATIONS, semio_framework_job::OperationId(7311), semio_framework_job::Generation(3), 1_000_000, 1, grant).unwrap());
                assert!(step.progress().fits(grant));
                assert_eq!((heap.requested_bytes, heap.released_bytes), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
                assert_eq!(input.source().unwrap().genesis.as_ref().unwrap().initial_pack.as_ptr(), pack_pointer);
                if matches!(step, RetainedCloneStep::Complete(_)) { break; }
            }
            if stop.is_none() {
                assert!(input.ready());
                assert!(input.take_ready(RetainedCloneGrant { maximum_items: 0, ..grant }).is_none());
                let (mut parts, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| input.take_ready(grant).unwrap());
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                assert!(input.terminal_is_empty());
                assert_eq!(parts.source.mutations::<i32>().unwrap().as_ptr(), mutation_pointer);
                let identities = parts.metadata.parts().unwrap();
                assert_eq!(identities.key.child_id, child);
                assert_eq!(identities.prepared_identity.reference, expected);
                assert_eq!(identities.registry_owner.child_id, child);
                let request = parts.request.as_ref().unwrap();
                assert_eq!(request.admitted_expected().unwrap(), &expected);
                assert_eq!(request.owner().unwrap().parent, parent);
                assert_eq!(request.owner().unwrap().child_id, child);
                assert_eq!(request.actor().0.as_str(), "actor:private-genesis");
                for _ in 0..100000 {
                    let request = parts.request.as_mut().unwrap();
                    if request.terminal_is_empty() { break; }
                    let demand = request.next_release_byte_demand().unwrap();
                    assert!(demand <= grant.maximum_release_bytes);
                    let turn = RetainedCloneGrant { maximum_release_bytes: demand, ..grant };
                    let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| request.close_step(turn).unwrap());
                    assert!(step.progress().fits(turn));
                    let bytes = step.progress().released_bytes;
                    assert_eq!(heap.requested_bytes, 0);
                    assert_eq!(heap.released_bytes, bytes);
                }
                assert!(parts.request.as_ref().unwrap().terminal_is_empty());
                parts.request.take();
                for _ in 0..100000 {
                    if parts.metadata.terminal_is_empty() { break; }
                    let demand = parts.metadata.next_close_byte_demand();
                    let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| parts.metadata.close_granted(RetainedCloneGrant { maximum_release_bytes: demand, ..grant }).unwrap());
                    assert_eq!((heap.requested_bytes, heap.released_bytes), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
                }
                assert!(parts.metadata.terminal_is_empty());
                for _ in 0..100000 {
                    if parts.source.terminal_is_empty() { break; }
                    let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| parts.source.close_granted(grant).unwrap());
                    assert!(step.progress().fits(grant));
                    assert_eq!((heap.requested_bytes, heap.released_bytes), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
                }
                assert!(parts.source.terminal_is_empty());
            } else {
                for _ in 0..100000 {
                    if input.terminal_is_empty() { break; }
                    let demand = input.retirement_demands(grant.maximum_copy_bytes).unwrap();
                    assert!(demand.release_bytes <= grant.maximum_release_bytes && demand.depth <= grant.maximum_depth);
                    let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| input.close_granted(grant).unwrap());
                    assert!(step.progress().fits(grant));
                    assert_eq!((heap.requested_bytes, heap.released_bytes), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
                }
                assert!(input.terminal_is_empty());
            }
        }
    }
}
