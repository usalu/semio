
use super::*;

/// 🧪️ Relocated from the deleted `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES).
#[semio_framework_async_macros::async_test]
async fn clone_block_reids_group_children() {
    let mut ids = NoteIdOwner::new("schema-clone-test", 0);
    let child = create_block_by_kind(&mut ids, "text", 0.0, 0.0);
    let child_id = block_id(&child).to_string();
    let group = NoteBlockNode::Group { id: "group-1".into(), name: "Group".into(), x: 0.0, y: 0.0, width: 100.0, height: 100.0, rotation: 0.0, visible: true, locked: false, children: vec![child] };
    let cloned = clone_block(&mut ids, &group);
    if let NoteBlockNode::Group { children, .. } = &cloned {
        assert_ne!(block_id(&children[0]), child_id);
    } else {
        panic!("expected group block");
    }
}
