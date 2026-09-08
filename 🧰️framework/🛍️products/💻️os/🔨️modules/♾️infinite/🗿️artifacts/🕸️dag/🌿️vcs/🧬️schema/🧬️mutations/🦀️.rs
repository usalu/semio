use super::{DagDiff, DagSnapshot};
use semio_framework_value_derive::{FromValue, ToValue};

#[path = "🔡️change-node-abbreviation/🦀️.rs"]
mod change_node_abbreviation;
#[path = "🖼️change-node-icon/🦀️.rs"]
mod change_node_icon;
#[path = "🔤️change-node-name/🦀️.rs"]
mod change_node_name;
#[path = "🧮️change-node-operator-kind/🦀️.rs"]
mod change_node_operator_kind;
#[path = "🔗️connect-nodes/🦀️.rs"]
mod connect_nodes;
#[path = "➕️create-node/🦀️.rs"]
mod create_node;
#[path = "🗑️delete-node/🦀️.rs"]
mod delete_node;
#[path = "✂️disconnect-nodes/🦀️.rs"]
mod disconnect_nodes;
#[path = "↔️move-node/🦀️.rs"]
mod move_node;
#[path = "✏️rename-node/🦀️.rs"]
mod rename_node;
#[path = "🔀️reorder-nodes/🦀️.rs"]
mod reorder_nodes;
#[path = "🔁️replace-node-kind/🦀️.rs"]
mod replace_node_kind;
#[path = "🗃️replace-node-properties/🦀️.rs"]
mod replace_node_properties;
#[path = "📐️resize-node/🦀️.rs"]
mod resize_node;

pub use change_node_abbreviation::ChangeNodeAbbreviation;
pub use change_node_icon::ChangeNodeIcon;
pub use change_node_name::ChangeNodeName;
pub use change_node_operator_kind::ChangeNodeOperatorKind;
pub use connect_nodes::ConnectNodes;
pub use create_node::CreateNode;
pub use delete_node::DeleteNode;
pub use disconnect_nodes::DisconnectNodes;
pub use move_node::MoveNode;
pub use rename_node::RenameNode;
pub use reorder_nodes::ReorderNodes;
pub use replace_node_kind::ReplaceNodeKind;
pub use replace_node_properties::ReplaceNodeProperties;
pub use resize_node::ResizeNode;

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::Mutations, dsl::DslOps)]
#[value(tag = "operation", rename_all = "camelCase", deny_unknown_fields)]
#[mutations(snapshot = DagSnapshot, diff = DagDiff, schema = "dag.dag")]
pub enum DagMutation {
    CreateNode(CreateNode),
    DeleteNode(DeleteNode),
    RenameNode(RenameNode),
    ChangeNodeName(ChangeNodeName),
    MoveNode(MoveNode),
    ResizeNode(ResizeNode),
    ChangeNodeIcon(ChangeNodeIcon),
    ChangeNodeAbbreviation(ChangeNodeAbbreviation),
    ChangeNodeOperatorKind(ChangeNodeOperatorKind),
    ReplaceNodeKind(ReplaceNodeKind),
    ReplaceNodeProperties(ReplaceNodeProperties),
    ReorderNodes(ReorderNodes),
    ConnectNodes(ConnectNodes),
    DisconnectNodes(DisconnectNodes),
}
