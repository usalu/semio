use super::ProgramFault;

#[test]
fn original_host_program_fault_retains_exact_typed_frame_cause_without_heap() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🔌️plugin/🖥️host/🧵️shard/🎟️grant/🧫️fixtures/🔣️.json")).unwrap();
    let cause = "operation identity is absent";
    let error = semio_framework_actor::pack::PackError::InvalidRetainedTurn(cause);
    let (failure, physical) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| ProgramFault::from(error));
    assert_eq!((physical.requested_bytes, physical.released_bytes), (law["dispatch"]["heapBytes"].as_u64().unwrap() as usize, 0));
    assert_eq!(failure.frame, Some(error));
    assert!(failure.fault.is_none());
    assert_eq!((failure.text.len(), failure.text.capacity()), (0, 0));
    let semio_framework_actor::pack::PackError::InvalidRetainedTurn(original) = failure.frame.unwrap() else { unreachable!() };
    assert_eq!(original.as_ptr(), cause.as_ptr());
    eprintln!("[DEBUG] Actual renderer ProgramFault response retains exact original typed PackError/static cause pointer heap0 with no String error adapter; physical frame publication accounting remains separate");
}

#[test]
fn original_host_program_fault_resource_override_and_single_wire_preserve_original_authority_without_heap() {
    use semio_framework_plugin_host::shard::{grant::ShardResourceBudget, ShardFrame};
    let law: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🔌️plugin/🖥️host/🧵️shard/🎟️grant/🧫️fixtures/🔣️.json")).unwrap();
    let input = serde_json::from_value(law["input"].clone()).unwrap();
    let before = &law["dispatch"]["resourcesBefore"];
    let budget = semio_framework_actor::Budget { retained: input, fuel: before["fuel"].as_u64().unwrap(), wall_ms: before["wallMs"].as_u64().unwrap() as u32, memory_bytes: before["memoryBytes"].as_u64().unwrap(), ui_nodes: before["uiNodes"].as_u64().unwrap() as u32, mailbox_len: before["mailboxLen"].as_u64().unwrap() as u16, max_effects: before["maxEffects"].as_u64().unwrap() as u32, max_patch_bytes: before["maxPatchBytes"].as_u64().unwrap() as u32 };
    let grant = semio_framework_actor::TurnGrant { actor: semio_framework_actor::ActorId(71), shard: semio_framework_actor::ShardId(0), budget, envelopes: Vec::new() };
    let original = grant.original_input() as *const _;
    let after = &law["dispatch"]["resourcesAfter"];
    let resources = ShardResourceBudget { fuel: after["fuel"].as_u64().unwrap(), wall_ms: after["wallMs"].as_u64().unwrap() as u32, memory_bytes: after["memoryBytes"].as_u64().unwrap(), ui_nodes: after["uiNodes"].as_u64().unwrap() as u32, mailbox_len: after["mailboxLen"].as_u64().unwrap() as u16, max_effects: after["maxEffects"].as_u64().unwrap() as u32, max_patch_bytes: after["maxPatchBytes"].as_u64().unwrap() as u32 };
    let hex = law["dispatch"]["singleAuthorityWire"]["hex"].as_str().unwrap();
    let expected: Vec<u8> = (0..hex.len()).step_by(2).map(|index| u8::from_str_radix(&hex[index..index + 2], 16).unwrap()).collect();
    let mut bytes = Vec::with_capacity(256);
    let backing = bytes.as_ptr();
    let (encoded, physical) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| semio_framework_async::poll::resolve_ready(ShardFrame::pack_encode_grant(&grant, resources, &mut bytes)));
    encoded.unwrap();
    assert_eq!((physical.requested_bytes, physical.released_bytes), (0, 0));
    assert_eq!(grant.original_input() as *const _, original);
    assert_eq!(*grant.original_input(), input);
    assert_eq!(bytes.as_ptr(), backing);
    assert_eq!(bytes.len(), law["dispatch"]["singleAuthorityWire"]["byteLength"].as_u64().unwrap() as usize);
    assert_eq!(bytes, expected);
    let mut position = 0;
    let (decoded, physical) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| semio_framework_async::poll::resolve_ready(ShardFrame::pack_decode(&bytes, &mut position)));
    assert_eq!((physical.requested_bytes, physical.released_bytes), (0, 0));
    let ShardFrame::Grant { budget, .. } = decoded.unwrap() else { unreachable!() };
    assert_eq!(*budget.original_input(), input);
    assert_eq!(ShardResourceBudget::from_original(&budget), resources);
    assert_eq!(position, bytes.len());
    eprintln!("[DEBUG] Actual borrowed Host grant resource-only override preserves original71/3/19 pointer and all5axes; one108byte wire equals independent neutral Python struct/Node Buffer golden encode/decode heap0 on supplied backing");
}
