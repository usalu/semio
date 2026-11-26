use super::*;
use semio_framework_job::{Generation, OperationId, OriginalJobFixtureCaller, receive_original_retained_work};
use semio_framework_job_payload::{JobPayloadStream, RetainedPayloadBuilder};

#[test]
fn checkpoint_becomes_visible_only_after_its_last_page() {
    let law: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    let pages: Vec<Vec<u8>> = serde_json::from_value(law["pages"].clone()).unwrap();
    let expected = law["expectedHex"].as_str().unwrap();
    let stock = RetainedCloneGrant { maximum_items: 4096, maximum_copy_bytes: 1048576, maximum_capacity_bytes: 1048576, maximum_release_bytes: 1048576, maximum_depth: 64 };
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(71), Generation(2), stock, ||Some(1), u64::MAX, 100000);
    let mut builder = RetainedPayloadBuilder::new(JobPayloadStream::CheckpointState);
    let source: Vec<u8> = pages.concat();
    let mut cursor = 0;
    let birth = RetainedCloneGrant { maximum_capacity_bytes: 3, ..stock };
    for _ in 0..1024 {
        if builder.published().is_some() { break }
        caller.turn(birth, |cx| {
            let turn = if !builder.is_initialized() { builder.advance_initialization(cx) }
                else if cursor < source.len() { builder.append_original(cx, &source, &mut cursor) }
                else { builder.seal(cx) };
            receive_original_retained_work(cx, turn)?;
            Ok(())
        }).unwrap();
    }
    let payload = builder.published().unwrap();
    assert!(payload.page_count() > 1);
    let mut checkpoint = ToolRunCheckpoint { published: vec![99], visible: true, ..Default::default() };
    let mut received = false;
    let copy = size_of::<ToolRunCheckpoint>() + law["copyBytesPerTurn"].as_u64().unwrap() as usize;
    for _ in 0..64 {
        assert_eq!(checkpoint.as_deref(), Some([99].as_slice()));
        caller.turn(RetainedCloneGrant { maximum_copy_bytes: copy, ..stock }, |cx| {
            checkpoint.receive(71, payload, &mut received, cx)
        }).unwrap();
        if received { break }
    }
    assert!(received);
    let hex: String = checkpoint.as_deref().unwrap().iter().map(|byte| format!("{byte:02x}")).collect();
    assert_eq!(hex, expected);
    while !builder.terminal_is_empty() {
        caller.turn(stock, |cx| {
            semio_framework_job::close_original_effect_step(cx, builder.retirement_demands()?, |effect| {
                semio_framework_value::RetainedWorkPoll::Complete(builder.close_step(effect))
            }).map(|_| ())
        }).unwrap();
    }
    caller.retire(stock);
}
