use super::*;
use protocol::MutationLeaf;
#[test]
fn descriptor_has_set_count_identity() {
    assert_eq!(<SetCount as MutationLeaf>::DESCRIPTOR.semantic_kind, "set-count");
    assert_eq!(<SetCount as MutationLeaf>::DESCRIPTOR.text_opcode, Some("set-count"));
    assert_eq!(<SetCount as MutationLeaf>::DESCRIPTOR.binary_tag, Some(0));
}
