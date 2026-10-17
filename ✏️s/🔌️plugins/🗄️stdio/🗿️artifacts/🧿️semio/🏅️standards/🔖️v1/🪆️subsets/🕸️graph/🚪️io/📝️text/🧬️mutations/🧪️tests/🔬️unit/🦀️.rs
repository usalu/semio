use crate::schema::mutations::SemioGraphMutation;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint2;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_base::io::text::codec::split_top_level;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_base::io::text::codec::strip_brackets;
#[allow(unused_imports)]
use crate::{schema::mutations::add_node_port::AddNodePort, schema::mutations::add_node_property::AddNodeProperty, schema::mutations::change_node_kind::ChangeNodeKind, schema::mutations::change_node_label::ChangeNodeLabel, schema::mutations::create_edge::CreateEdge, schema::mutations::create_node::CreateNode, schema::mutations::delete_edge::DeleteEdge, schema::mutations::delete_node::DeleteNode, schema::mutations::drag_nodes::DragNodes, schema::mutations::move_node::MoveNode, schema::mutations::remove_node_port::RemoveNodePort, schema::mutations::remove_node_property::RemoveNodeProperty, schema::mutations::rename_node::RenameNode, schema::mutations::resize_node::ResizeNode, schema::mutations::set_node_property::SetNodeProperty, schema::mutations::add_edge_property::AddEdgeProperty, schema::mutations::remove_edge_property::RemoveEdgeProperty, schema::mutations::set_edge_property::SetEdgeProperty};
#[allow(unused_imports)]
use crate::{schema::snapshot::GraphEdgeId, schema::snapshot::GraphNodeId, schema::snapshot::SemioGraphPort};
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_value_model::io::text::diff::dec_semio_value_entry;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_value_model::io::text::diff::enc_semio_value_entry;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueEntry;
use crate::io::text::mutations::*;
use protocol::OpText;

#[semio_framework_async_macros::async_test]
async fn op_text_roundtrip_law() {
    for mutation in demo_mutation_cases() {
        let printed = mutation.print_op();
        assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
        let parsed = <SemioGraphMutation as OpText>::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
        assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch (printed {printed:?})");
    }
}
