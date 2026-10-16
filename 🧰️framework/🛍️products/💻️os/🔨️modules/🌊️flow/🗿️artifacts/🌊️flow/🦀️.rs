//! 🌊️ Independently composable Flow document, mutation, and ownership contracts: the facade over the model, document and copy producers.

pub use semio_framework_artifact_flow_flow_model::{neural, os_dsl, os_pack, os_spr, os_store, widget_id_for, OrderedMap, OrderedSet};

pub use semio_framework_artifact_flow_flow_model::{
    cluster_io_layout, cluster_tree_from_property, cluster_tree_property, dag_preview_content_from_dict, default_neuron_input_ports, default_neuron_output_ports, default_neuron_params_from_info, descriptor_explicit_id, generated_widget_id, neuron_io_layout, parse_port_endpoint, preview_content_summary, sensible_slider_range, set_widget_slider_value, variable_io_ports, widget_display_meta, widget_from_descriptor, widget_from_descriptor_with_info, widget_io_ports, widget_label, widget_to_dag_node,
    CameraJson, FlowArtifact, FlowChannelRef, FlowGui, FlowHostComponents, FlowLayoutEntry, FlowNodeGui, FlowPreviewGui, FlowUi, NodeChrome, SynapseSpec, Widget, WidgetDescriptor, WidgetLayout, FLOW_SLIDER_MAX, FLOW_SLIDER_MIN, FLOW_SLIDER_STEP,
};

pub use semio_framework_artifact_flow_flow_document::{
    FlowGraphStep, FlowHostSnapshot, keyboard_order, keyboard_step, tree_from_host_snapshot,
    AddSynapse, AddWidget, ChangeLayout, ChangeSynapse, ChangeWidget, empty_flow_snapshot, FlowCollectionDelta, FlowDelta, FlowDiff, FlowEnvelope, FlowMutation, FlowSnapshotRetirementFactory, FlowStore, FLOW_DOCUMENT_SCHEMA, FLOW_HISTORY_STORE_FACTORY, MoveSynapse, MoveWidget, original_flow_history_envelope, original_flow_history_store, prepared_operation_wire_source, RemoveSynapse, RemoveWidget, retire_flow_mutation, retire_flow_store_cold,
};

pub mod artifact {
    pub use super::{
        cluster_io_layout, cluster_tree_from_property, cluster_tree_property, dag_preview_content_from_dict, default_neuron_input_ports, default_neuron_output_ports, default_neuron_params_from_info, descriptor_explicit_id, generated_widget_id, neuron_io_layout, parse_port_endpoint, preview_content_summary, sensible_slider_range, set_widget_slider_value, variable_io_ports, widget_display_meta, widget_from_descriptor, widget_from_descriptor_with_info, widget_io_ports, widget_label, widget_to_dag_node,
        CameraJson, FlowArtifact, FlowChannelRef, FlowGui, FlowHostComponents, FlowLayoutEntry, FlowNodeGui, FlowPreviewGui, FlowUi, NodeChrome, SynapseSpec, Widget, WidgetDescriptor, WidgetLayout, FLOW_SLIDER_MAX, FLOW_SLIDER_MIN, FLOW_SLIDER_STEP,
        FlowGraphStep, FlowHostSnapshot, keyboard_order, keyboard_step, tree_from_host_snapshot,
    };
}

pub mod graph_parameter {
    pub use semio_framework_artifact_flow_flow_model::graph_parameter::{SetGraphParameter, SetGraphParameterRetirement};
}

pub mod retained {
    pub use semio_framework_artifact_flow_flow_model::retained::{FlowOwner, FlowRetirement};
    pub use semio_framework_artifact_flow_flow_copy as copy;
    pub use semio_framework_artifact_flow_flow_copy::{FlowCopyAllocations, FlowHostSnapshotCopy, FlowSnapshotCopyIntakeSource, FlowSnapshotIntakeProducer, FlowSynapseCopy, FlowWidgetCopy};
}

pub mod vcs {
    pub use super::{
        AddSynapse, AddWidget, ChangeLayout, ChangeSynapse, ChangeWidget, empty_flow_snapshot, FlowCollectionDelta, FlowDelta, FlowDiff, FlowEnvelope, FlowMutation, FlowSnapshotRetirementFactory, FlowStore, FLOW_DOCUMENT_SCHEMA, FLOW_HISTORY_STORE_FACTORY, MoveSynapse, MoveWidget, original_flow_history_envelope, original_flow_history_store, prepared_operation_wire_source, RemoveSynapse, RemoveWidget, retire_flow_mutation, retire_flow_store_cold,
        FlowLayoutEntry,
    };
}

pub mod io {
    pub mod binary {
        pub mod snapshot {
            pub use semio_framework_artifact_flow_flow_document::io::binary::snapshot::FlowSnapshotPackProducer;
        }
    }
}
