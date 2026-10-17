use semio_framework_value::ErasedSnapshotRetirement;
use semio_framework_value_derive::{FromValue, ToValue};
use protocol::MutationKind;
use crate::test_app_mutation_fixture::document::mutations::set_label::SetLabel;
use protocol::MutationLeaf;
#[test]
fn descriptor_has_set_label_identity() {
    assert_eq!(<SetLabel as MutationLeaf>::DESCRIPTOR.semantic_kind, "set-label");
    assert_eq!(<SetLabel as MutationLeaf>::DESCRIPTOR.text_opcode, Some("set-label"));
    assert_eq!(<SetLabel as MutationLeaf>::DESCRIPTOR.binary_tag, Some(1));
}
