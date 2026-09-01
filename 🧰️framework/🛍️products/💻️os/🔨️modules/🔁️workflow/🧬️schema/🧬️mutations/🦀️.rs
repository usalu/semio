use super::{WorkflowDiff, WorkflowSnapshot};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Leaves
#[path = "➕️add-node/🦀️.rs"]
mod add_node;
#[path = "🗑️remove-node/🦀️.rs"]
mod remove_node;
#[path = "🔗connect-ports/🦀️.rs"]
mod connect_ports;
#[path = "✂️disconnect-edge/🦀️.rs"]
mod disconnect_edge;
#[path = "↔️move-node/🦀️.rs"]
mod move_node;
#[path = "✏️rename-node/🦀️.rs"]
mod rename_node;
#[path = "🧩add-parameter/🦀️.rs"]
mod add_parameter;
#[path = "🧹remove-parameter/🦀️.rs"]
mod remove_parameter;
#[path = "🩹change-parameter/🦀️.rs"]
mod change_parameter;
#[path = "🔒bind-parameter-field/🦀️.rs"]
mod bind_parameter_field;
#[path = "🔓unbind-parameter-field/🦀️.rs"]
mod unbind_parameter_field;
#[path = "🔄update-node-ports/🦀️.rs"]
mod update_node_ports;
#[path = "📥add-input/🦀️.rs"]
mod add_input;
#[path = "🚮remove-input/🦀️.rs"]
mod remove_input;
#[path = "🔌bind-input/🦀️.rs"]
mod bind_input;
#[path = "🚪unbind-input/🦀️.rs"]
mod unbind_input;
#[path = "📤bind-output/🦀️.rs"]
mod bind_output;
#[path = "⛔️unbind-output/🦀️.rs"]
mod unbind_output;

pub use add_node::AddNode;
pub use remove_node::RemoveNode;
pub use connect_ports::ConnectPorts;
pub use disconnect_edge::DisconnectEdge;
pub use move_node::MoveNode;
pub use rename_node::RenameNode;
pub use add_parameter::AddParameter;
pub use remove_parameter::RemoveParameter;
pub use change_parameter::ChangeParameter;
pub use bind_parameter_field::BindParameterField;
pub use unbind_parameter_field::UnbindParameterField;
pub use update_node_ports::UpdateNodePorts;
pub use add_input::AddInput;
pub use remove_input::RemoveInput;
pub use bind_input::BindInput;
pub use unbind_input::UnbindInput;
pub use bind_output::BindOutput;
pub use unbind_output::UnbindOutput;
//#endregion 🔖️Leaves

//#region 🔖️Aggregate
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::Mutations, dsl::DslOps)]
#[serde(tag = "operation", rename_all = "camelCase", deny_unknown_fields)]
#[value(tag = "operation", rename_all = "camelCase", deny_unknown_fields)]
#[mutations(snapshot = WorkflowSnapshot, diff = WorkflowDiff, schema = "os.workflow")]
pub enum WorkflowMutation {
    AddNode(AddNode),
    RemoveNode(RemoveNode),
    ConnectPorts(ConnectPorts),
    DisconnectEdge(DisconnectEdge),
    MoveNode(MoveNode),
    RenameNode(RenameNode),
    AddParameter(AddParameter),
    RemoveParameter(RemoveParameter),
    ChangeParameter(ChangeParameter),
    BindParameterField(BindParameterField),
    UnbindParameterField(UnbindParameterField),
    UpdateNodePorts(UpdateNodePorts),
    AddInput(AddInput),
    RemoveInput(RemoveInput),
    BindInput(BindInput),
    UnbindInput(UnbindInput),
    BindOutput(BindOutput),
    UnbindOutput(UnbindOutput),
}
//#endregion 🔖️Aggregate

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::{Mutation, SemanticMutation};
    #[test]
    fn descriptors_follow_the_canonical_workflow_roster() {
        assert_eq!(WorkflowMutation::kinds().iter().map(|value| value.kind).collect::<Vec<_>>(), vec!["add-node", "remove-node", "connect-ports", "disconnect-edge", "move-node", "rename-node", "add-parameter", "remove-parameter", "change-parameter", "bind-parameter-field", "unbind-parameter-field", "update-node-ports", "add-input", "remove-input", "bind-input", "unbind-input", "bind-output", "unbind-output"]);
        assert_eq!(<WorkflowMutation as Mutation<WorkflowSnapshot>>::DESCRIPTORS.iter().map(|value| value.binary_tag).collect::<Vec<_>>(), vec![Some(0), Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), Some(16), Some(17)]);
    }
}
