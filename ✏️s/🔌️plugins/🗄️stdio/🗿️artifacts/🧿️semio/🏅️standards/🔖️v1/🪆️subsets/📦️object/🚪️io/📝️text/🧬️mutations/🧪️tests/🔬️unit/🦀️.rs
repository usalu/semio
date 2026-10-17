use crate::schema::mutations::SemioObjectMutation;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint3;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioQuaternion;
#[allow(unused_imports)]
use crate::schema::mutations::{create_brep::CreateBrep, create_mesh::CreateMesh, create_properties::CreateProperties, delete_brep::DeleteBrep, delete_mesh::DeleteMesh, delete_properties::DeleteProperties, move_object::MoveObject, rotate_object::RotateObject, scale_object::ScaleObject};
use crate::io::text::mutations::*;
use protocol::OpText;

#[semio_framework_async_macros::async_test]
async fn op_text_roundtrip_law() {
    for mutation in demo_mutation_cases() {
        let printed = mutation.print_op();
        assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
        let parsed = <SemioObjectMutation as OpText>::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
        assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch (printed {printed:?})");
    }
}
