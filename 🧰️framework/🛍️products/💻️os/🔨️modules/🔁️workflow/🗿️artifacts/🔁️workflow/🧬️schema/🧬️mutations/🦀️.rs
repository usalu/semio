use super::{WorkflowDiff, WorkflowSnapshot};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Leaves
#[path = "📥add-input/🦀️.rs"]
mod add_input;
#[path = "➕️add-node/🦀️.rs"]
mod add_node;
#[path = "🧩add-parameter/🦀️.rs"]
mod add_parameter;
#[path = "🔌bind-input/🦀️.rs"]
mod bind_input;
#[path = "📤bind-output/🦀️.rs"]
mod bind_output;
#[path = "🔒bind-parameter-field/🦀️.rs"]
mod bind_parameter_field;
#[path = "🩹change-parameter/🦀️.rs"]
mod change_parameter;
#[path = "🔗connect-ports/🦀️.rs"]
mod connect_ports;
#[path = "✂️disconnect-edge/🦀️.rs"]
mod disconnect_edge;
#[path = "↔️move-node/🦀️.rs"]
mod move_node;
#[path = "🚮remove-input/🦀️.rs"]
mod remove_input;
#[path = "🗑️remove-node/🦀️.rs"]
mod remove_node;
#[path = "🧹remove-parameter/🦀️.rs"]
mod remove_parameter;
#[path = "✏️rename-node/🦀️.rs"]
mod rename_node;
#[path = "🚪unbind-input/🦀️.rs"]
mod unbind_input;
#[path = "⛔️unbind-output/🦀️.rs"]
mod unbind_output;
#[path = "🔓unbind-parameter-field/🦀️.rs"]
mod unbind_parameter_field;
#[path = "🔄update-node-ports/🦀️.rs"]
mod update_node_ports;

pub use add_input::AddInput;
pub use add_node::AddNode;
pub use add_parameter::AddParameter;
pub use bind_input::BindInput;
pub use bind_output::BindOutput;
pub use bind_parameter_field::BindParameterField;
pub use change_parameter::ChangeParameter;
pub use connect_ports::ConnectPorts;
pub use disconnect_edge::DisconnectEdge;
pub use move_node::MoveNode;
pub use remove_input::RemoveInput;
pub use remove_node::RemoveNode;
pub use remove_parameter::RemoveParameter;
pub use rename_node::RenameNode;
pub use unbind_input::UnbindInput;
pub use unbind_output::UnbindOutput;
pub use unbind_parameter_field::UnbindParameterField;
pub use update_node_ports::UpdateNodePorts;
//#endregion 🔖️Leaves

//#region 🔖️Aggregate
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::Mutations, dsl::DslOps)]
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
