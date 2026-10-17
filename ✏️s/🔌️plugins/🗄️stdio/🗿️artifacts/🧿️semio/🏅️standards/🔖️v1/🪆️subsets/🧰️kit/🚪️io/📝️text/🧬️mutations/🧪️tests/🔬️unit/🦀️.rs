use crate::schema::mutations::SemioKitMutation;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_base::io::text::codec::split_top_level;
#[allow(unused_imports)]
use crate::schema::mutations::{add_design::AddDesign, add_type::AddType, bind_representation::BindRepresentation, change_representation_pin::ChangeRepresentationPin, create_model::CreateModel, create_object::CreateObject, create_properties::CreateProperties, delete_model::DeleteModel, delete_object::DeleteObject, delete_properties::DeleteProperties, edit_design::EditDesign, remove_design::RemoveDesign, remove_type::RemoveType, rename_type::RenameType, unbind_representation::UnbindRepresentation};
#[allow(unused_imports)]
use crate::io::text::snapshot::dec_connection;
#[allow(unused_imports)]
use crate::io::text::snapshot::enc_connection;
#[allow(unused_imports)]
use crate::io::text::snapshot::dec_piece;
#[allow(unused_imports)]
use crate::io::text::snapshot::enc_piece;
#[allow(unused_imports)]
use crate::io::text::snapshot::dec_pin;
#[allow(unused_imports)]
use crate::io::text::snapshot::enc_pin;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_base::io::text::codec::dec_ref;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_base::io::text::codec::enc_ref;
#[allow(unused_imports)]
use crate::io::text::snapshot::dec_str;
#[allow(unused_imports)]
use crate::io::text::snapshot::enc_str;
use crate::io::text::mutations::*;
use protocol::OpText;

#[semio_framework_async_macros::async_test]
async fn op_text_roundtrip_law() {
    for mutation in demo_mutation_cases() {
        let printed = mutation.print_op();
        assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
        let parsed = <SemioKitMutation as OpText>::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
        assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch (printed {printed:?})");
    }
}
