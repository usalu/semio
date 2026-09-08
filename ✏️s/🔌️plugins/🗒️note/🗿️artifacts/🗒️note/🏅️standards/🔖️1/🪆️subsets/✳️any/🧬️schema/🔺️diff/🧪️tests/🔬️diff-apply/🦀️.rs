
use super::*;

#[semio_framework_async_macros::async_test]
async fn malformed_nested_parent_rejects_without_changing_the_base() {
    let base = NoteSnapshot::default();
    let diff = NoteDiff {
        blocks: Some(NoteBlocksDelta {
            added: vec![NoteAddedBlockEntry { parent_id: Some("missing-group".into()), index: Some(0), block: crate::schema::create_block_by_kind(&mut crate::schema::NoteIdOwner::new("diff-hostile-test", 0), "text", 0.0, 0.0) }],
            ..Default::default()
        }),
        ..Default::default()
    };
    let error = diff.apply(&base).expect_err("missing nested parent must reject");
    assert_eq!(error.code, "mutation.apply.missing-target");
    assert_eq!(error.target, ["blocks", "added", "0", "parentId"]);
    assert!(base.blocks.is_empty());
}
