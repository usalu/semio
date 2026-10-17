use semio_framework_value_derive::{FromValue, ToValue};
use protocol::MutationKind;
use crate::test_app_mutation_fixture::document::mutations::set_count::SetCount;
use protocol::MutationLeaf;
#[test]
fn descriptor_has_set_count_identity() {
    assert_eq!(<SetCount as MutationLeaf>::DESCRIPTOR.semantic_kind, "set-count");
    assert_eq!(<SetCount as MutationLeaf>::DESCRIPTOR.text_opcode, Some("set-count"));
    assert_eq!(<SetCount as MutationLeaf>::DESCRIPTOR.binary_tag, Some(0));
}
