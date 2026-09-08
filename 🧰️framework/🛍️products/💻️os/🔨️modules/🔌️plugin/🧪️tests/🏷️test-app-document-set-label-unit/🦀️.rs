use super::*;
use protocol::MutationLeaf;
#[test]
fn descriptor_has_set_label_identity() {
    assert_eq!(<SetLabel as MutationLeaf>::DESCRIPTOR.semantic_kind, "set-label");
    assert_eq!(<SetLabel as MutationLeaf>::DESCRIPTOR.text_opcode, Some("set-label"));
    assert_eq!(<SetLabel as MutationLeaf>::DESCRIPTOR.binary_tag, Some(1));
}
