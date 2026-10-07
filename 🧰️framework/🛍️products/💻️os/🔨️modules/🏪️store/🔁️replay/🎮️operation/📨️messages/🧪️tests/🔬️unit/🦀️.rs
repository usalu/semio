//! 🧪️ Native UTF-8 copying and exact cancellation match the neutral diagnostic corpus.

use super::*;

fn close(cursor: &mut MessageCopyCursor) {
    cursor.begin_close();
    for _ in 0..100000 {
        let step = cursor.close_step(1, 7).unwrap();
        if let SnapshotRetirementStep::Pending { released_items, released_bytes } = step { assert!(released_items <= 1); assert!(released_bytes <= 7); }
        if step == SnapshotRetirementStep::Complete { assert!(cursor.terminal_is_empty()); return; }
    }
    panic!("message copy exact close must terminate");
}

#[test]
fn cooperative_message_copy_obeys_the_neutral_law() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for row in law["cases"].as_array().unwrap() {
        let source = MutationMessage::fatal(row["code"].as_str().unwrap().to_owned(), row["message"].as_str().unwrap()).at(row["target"].as_array().unwrap().iter().map(|segment| segment.as_str().unwrap().to_owned()));
        for bytes in law["byteGrants"].as_array().unwrap() {
            let grant = MessageCopyGrant { maximum_items: 1, maximum_copy_bytes: bytes.as_u64().unwrap() as usize, maximum_capacity_bytes: law["capacityGrant"].as_u64().unwrap() as usize };
            let mut cursor = MessageCopyCursor::new();
            let mut turns = 0;
            loop {
                turns += 1;
                assert!(turns < 100000);
                match cursor.advance(&source, grant).unwrap() {
                    MessageCopyStep::Blocked => panic!("neutral nonzero UTF-8 grant must progress"),
                    MessageCopyStep::Pending(progress) => { assert!(progress.items <= 1); assert!(progress.copied_bytes <= grant.maximum_copy_bytes); assert!(progress.retained_capacity_bytes <= grant.maximum_capacity_bytes); }
                    MessageCopyStep::Complete(_) => break,
                }
            }
            assert!(turns > source.target.len());
            assert_eq!(cursor.take().unwrap(), source);
            close(&mut cursor);
            for cancelled_at in law["cancelAt"].as_array().unwrap() {
                let mut cancelled = MessageCopyCursor::new();
                for _ in 0..cancelled_at.as_u64().unwrap() { cancelled.advance(&source, grant).unwrap(); }
                cancelled.cancel();
                assert_eq!(cancelled.advance(&source, grant).unwrap(), MessageCopyStep::Blocked);
                assert!(cancelled.take().is_none());
                close(&mut cancelled);
            }
        }
    }
    let source = MutationMessage::fatal("empty-targets", "").at(std::iter::repeat_n(String::new(), law["emptySegments"]["count"].as_u64().unwrap() as usize));
    let mut cursor = MessageCopyCursor::new();
    let accepted = cursor.advance(&source, MessageCopyGrant { maximum_items: 1, maximum_copy_bytes: 7, maximum_capacity_bytes: law["capacityGrant"].as_u64().unwrap() as usize }).is_ok();
    assert_eq!(accepted, law["emptySegments"]["accepted"].as_bool().unwrap());
    close(&mut cursor);
    eprintln!("[DEBUG] cooperative message-copy neutral UTF-8 grants/cancel/empty-target refusal verified");
}
