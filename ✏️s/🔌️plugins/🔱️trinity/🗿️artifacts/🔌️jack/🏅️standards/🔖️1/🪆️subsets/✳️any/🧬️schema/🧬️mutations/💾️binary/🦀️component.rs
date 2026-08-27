//! 📡️ Trinity graph mutation binary framing and registry surface.

pub use crate::artifacts::jack::standards::v1::subsets::any::schema::wire_runtime::*;

/// 🧾️ Direct-owner binary tags in aggregate declaration order.
pub const BINARY_TAG_REGISTRY: &[(&str, u8)] = &[
    ("CreateNode", super::create_node::binary::BINARY_TAG),
    ("DeleteNode", super::delete_node::binary::BINARY_TAG),
    ("CreateEdge", super::create_edge::binary::BINARY_TAG),
    ("DeleteEdge", super::delete_edge::binary::BINARY_TAG),
    ("RenameNode", super::rename_node::binary::BINARY_TAG),
    ("MoveNode", super::move_node::binary::BINARY_TAG),
    ("ChangeDataProperty", super::change_data_property::binary::BINARY_TAG),
    ("RemoveDataProperty", super::remove_data_property::binary::BINARY_TAG),
];
