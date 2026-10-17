use semio_framework_value_derive::{FromValue, ToValue};
use protocol::MutationKind;
use crate::test_app_mutation_fixture::document::mutations::set_slot_children::SetSlotChildren;
use protocol::MutationLeaf;
#[test]
fn descriptor_has_set_slot_children_identity() {
    assert_eq!(<SetSlotChildren as MutationLeaf>::DESCRIPTOR.semantic_kind, "set-slot-children");
    assert_eq!(<SetSlotChildren as MutationLeaf>::DESCRIPTOR.text_opcode, Some("set-slot-children"));
    assert_eq!(<SetSlotChildren as MutationLeaf>::DESCRIPTOR.binary_tag, Some(2));
}
