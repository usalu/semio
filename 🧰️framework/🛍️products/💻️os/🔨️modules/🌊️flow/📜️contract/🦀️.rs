//! 🌊️ Flow model contracts for plugin-level crates: the explicit Flow names a plugin shares with the Flow artifact without depending on the artifact crate.

pub use semio_framework_artifact_flow_flow::{CameraJson, FlowChannelRef, FlowHostSnapshot, FlowNodeGui, FlowPreviewGui, FlowUi, NodeChrome, OrderedMap, OrderedSet, SynapseSpec, Widget, WidgetLayout};

pub mod neural {
    pub use semio_framework_artifact_flow_flow::neural::{Atom, ColdDictionaryBuilder, ColdOwner, ColdRetire, Dictionary, Neuron, Synapse, Tree, Value};
}

pub mod retained {
    pub use semio_framework_artifact_flow_flow::retained::{FlowOwner, FlowRetirement};
}
