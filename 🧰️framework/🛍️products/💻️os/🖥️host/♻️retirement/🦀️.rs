//! ♻️ Exact field ownership for persisted OS space, collection, and workflow documents.

use crate::{space, workflow_kernel as workflow};
use store::os_store::retirement::{RetireOwned, RetirementCursor, leaf, sequence};
use store::{artifact_retire_leaf as retire_leaf, artifact_retire_struct as retire_struct, artifact_retirement_sequence as seq};

retire_leaf!(space::SpaceKind, space::SpaceVisibility, space::SpaceRole);
retire_struct!(space::SpaceUser { id, name, avatar, role });
retire_struct!(space::CollectionRef { id, name, document_id });
retire_struct!(space::InstalledExtension { extension_id, version, source_uri, package_hash, enabled });
retire_struct!(space::SpaceSnapshot { schema, name, kind, visibility, users, collections, programs, extensions });
retire_struct!(space::CollectionFolder { id, parent_id, name });
retire_struct!(space::CollectionEntry { id, folder_id, name, kind_id, body });
retire_struct!(space::CollectionSnapshot { schema, name, folders, entries });

impl RetireOwned for space::ArtifactBody {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self { Self::Document { schema, document_id } => seq![schema, document_id], Self::Blob { blob } => blob.retirement() }
    }
}

impl RetireOwned for space::SpaceMutation {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::SetName { name } => name.retirement(),
            Self::SetKind { kind } => kind.retirement(),
            Self::SetVisibility { visibility } => visibility.retirement(),
            Self::UpsertUser { user } => user.retirement(),
            Self::RemoveUser { user_id } => user_id.retirement(),
            Self::AddCollection { collection } => collection.retirement(),
            Self::RemoveCollection { collection_id } => collection_id.retirement(),
            Self::RenameCollection { collection_id, name } => seq![collection_id, name],
            Self::InstallProgram { plugin_id } | Self::UninstallProgram { plugin_id } => plugin_id.retirement(),
            Self::InstallExtension { extension_id, version, source_uri, package_hash, enabled } => seq![extension_id, version, source_uri, package_hash, enabled],
            Self::UninstallExtension { extension_id } => extension_id.retirement(),
            Self::SetExtensionEnabled { extension_id, enabled } => seq![extension_id, enabled],
        }
    }
}

impl RetireOwned for space::CollectionMutation {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::RenameCollection { new_name } => new_name.retirement(),
            Self::CreateFolder { folder, index } => seq![folder, index],
            Self::DeleteFolder { folder_id } => folder_id.retirement(),
            Self::MoveToCollection { folder_id, new_parent } => seq![folder_id, new_parent],
            Self::RenameFolder { folder_id, new_name } => seq![folder_id, new_name],
            Self::CreateEntry { entry, index } => seq![entry, index],
            Self::DeleteEntry { entry_id } => entry_id.retirement(),
            Self::MoveToFolder { entry_id, new_folder } => seq![entry_id, new_folder],
            Self::RenameEntry { entry_id, new_name } => seq![entry_id, new_name],
            Self::ReplaceEntryBody { entry_id, new_body } => seq![entry_id, new_body],
        }
    }
}

retire_struct!(workflow::WorkflowNode { id, plugin_id, app_id, label, yields, artifact_ref, config_ref, x, y, width, height, inputs, outputs });
retire_struct!(workflow::WorkflowEdge { id, source_node_id, source_port_id, target_node_id, target_port_id, contract });
retire_struct!(workflow::Workflow { schema, nodes, edges });
retire_struct!(workflow::WorkflowParameterBinding { parameter_id, node_id, field_path });
retire_struct!(workflow::WorkflowInputBinding { input_id, node_id, port_id });
retire_struct!(workflow::WorkflowOutputBinding { node_id, port_id, path_template });
retire_struct!(workflow::WorkflowSnapshot { schema, graph, parameters, parameter_bindings, inputs, input_bindings, output_bindings });

impl RetireOwned for workflow::MediaContract {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        let Self { kind_id, media_type, wire, conversion } = self;
        let wire = match wire { semio_framework::MediaWireFormat::Document { schema } => schema, semio_framework::MediaWireFormat::Binary { format_kind } => format_kind };
        sequence(vec![kind_id.retirement(), leaf(media_type), wire.retirement(), leaf(conversion)])
    }
}

impl RetireOwned for workflow::WorkflowMediaPort {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        let Self { id, spec } = self;
        let semio_framework::MediaPortSpec { id: spec_id, label, direction, media_type, kind_id, required, multiplicity } = spec;
        sequence(vec![id.retirement(), spec_id.retirement(), label.retirement(), leaf(direction), leaf(media_type), kind_id.retirement(), required.retirement(), leaf(multiplicity)])
    }
}

impl RetireOwned for workflow::WorkflowInput {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        let Self { id, kind_id, selector, required, multiplicity } = self;
        sequence(vec![id.retirement(), kind_id.retirement(), selector.retirement(), required.retirement(), leaf(multiplicity)])
    }
}

impl RetireOwned for workflow::WorkflowParameter {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::Numeric { id, name, value, min, max, step } => seq![id, name, value, min, max, step],
            Self::Categorical { id, name, value, options } => seq![id, name, value, options],
            Self::Toggle { id, name, value } => seq![id, name, value],
            Self::Text { id, name, value } => seq![id, name, value],
        }
    }
}

impl RetireOwned for workflow::WorkflowMutation {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::AddNode(workflow::AddNode { node }) => node.retirement(),
            Self::RemoveNode(workflow::RemoveNode { node_id }) => node_id.retirement(),
            Self::ConnectPorts(workflow::ConnectPorts { edge }) => edge.retirement(),
            Self::DisconnectEdge(workflow::DisconnectEdge { edge_id }) => edge_id.retirement(),
            Self::MoveNode(workflow::MoveNode { node_id, x, y }) => seq![node_id, x, y],
            Self::RenameNode(workflow::RenameNode { node_id, label }) => seq![node_id, label],
            Self::AddParameter(workflow::AddParameter { parameter }) => parameter.retirement(),
            Self::RemoveParameter(workflow::RemoveParameter { parameter_id }) => parameter_id.retirement(),
            Self::ChangeParameter(workflow::ChangeParameter { parameter_id, parameter }) => seq![parameter_id, parameter],
            Self::BindParameterField(workflow::BindParameterField { binding }) => binding.retirement(),
            Self::UnbindParameterField(workflow::UnbindParameterField { node_id, field_path }) => seq![node_id, field_path],
            Self::UpdateNodePorts(workflow::UpdateNodePorts {}) => seq![],
            Self::AddInput(workflow::AddInput { input }) => input.retirement(),
            Self::RemoveInput(workflow::RemoveInput { input_id }) | Self::UnbindInput(workflow::UnbindInput { input_id }) => input_id.retirement(),
            Self::BindInput(workflow::BindInput { binding }) => binding.retirement(),
            Self::BindOutput(workflow::BindOutput { binding }) => binding.retirement(),
            Self::UnbindOutput(workflow::UnbindOutput { node_id, port_id }) => seq![node_id, port_id],
        }
    }
}
