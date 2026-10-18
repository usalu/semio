//! 📄️ Flow document: widgets, host_snapshot, and DAG snapshot helpers.

use neural_engine as neural;
use protocol::causal::transition::HistoryFoldIndex;

use crate::{OrderedMap, OrderedSet};
use std::collections::HashSet;

use semio_framework_graph_manifest::{PropertyBag, PropertyValue};
use neural::{cluster_operator_info, Atom, ChannelSpec, Dictionary, Neuron, OperatorInfo, Synapse, Tree, Value as NeuralValue, CLUSTER_KIND, INPUT_KIND, OUTPUT_KIND};
use semio_framework_os_infinite_directed_presentation::{normalize_node_display, DagNodeKind, DagNodeSpec, DagPreviewContent, IoPortSpec};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

use crate::os_spr::{Identified, Patchable};
use crate::widget_id_for;
#[path = "../💡️inferences/🌳️cluster-tree/🦀️.rs"]
mod cluster_tree;
pub use cluster_tree::{cluster_tree_from_property, cluster_tree_property};

// #region 🔖️Document

#[path = "🧱️document-model/🦀️.rs"]
mod document_model;
use document_model::{
    default_export_format, default_slider_value, default_variable_name, default_variable_schema,
    export_widget_display_meta,
};
pub use document_model::{
    CameraJson, FlowArtifact, FlowChannelRef, FlowGui, FlowHostComponents, FlowLayoutEntry,
    FlowNodeGui, FlowPreviewGui, FlowUi, NodeChrome, SynapseSpec, Widget, WidgetLayout,
};

#[path = "🪪️collection-identity-and-widget-chrome/🦀️.rs"]
mod collection_identity_and_widget_chrome;
pub use collection_identity_and_widget_chrome::widget_chrome;

#[path = "🏷️widget-labels-and-io-ports/🦀️.rs"]
mod widget_labels_and_io_ports;
pub use widget_labels_and_io_ports::{
    cluster_io_layout, default_neuron_input_ports, default_neuron_output_ports, neuron_io_layout,
    variable_io_ports, widget_display_meta, widget_io_ports, widget_label,
};
use widget_labels_and_io_ports::{
    default_neuron_input_ports_from_info, default_neuron_output_ports_from_info, io_port_typed,
};

#[path = "🗺️dag-node-conversion/🦀️.rs"]
mod dag_node_conversion;
pub use dag_node_conversion::{parse_port_endpoint, widget_to_dag_node};

#[path = "🎚️slider-parameters-and-preview-content/🦀️.rs"]
mod slider_parameters_and_preview_content;
pub use slider_parameters_and_preview_content::{
    dag_preview_content_from_dict, preview_content_summary, sensible_slider_range,
    set_widget_slider_value, FLOW_SLIDER_MAX, FLOW_SLIDER_MIN, FLOW_SLIDER_STEP,
};
use slider_parameters_and_preview_content::resolve_input_slider_fields;

#[path = "🧩️widget-descriptors/🦀️.rs"]
mod widget_descriptors;
pub use widget_descriptors::{
    default_neuron_params_from_info, descriptor_explicit_id, generated_widget_id,
    widget_from_descriptor, widget_from_descriptor_with_info, WidgetDescriptor,
};

//#region 🎚️ParameterValue

#[cfg(test)]
#[path = "../../🎚️parameter/🧪️tests/🎚️parameter/🦀️.rs"]
mod parameter_tests;
//#endregion 🎚️ParameterValue

// #endregion 🔖️Document

//#region 🧪️AuthoredSliderLabels
#[cfg(test)]
#[path = "🧪️tests/🔬️slider-label/🦀️.rs"]
mod slider_label_tests;
//#endregion 🧪️AuthoredSliderLabels

//#region 🧪️SinkPorts
#[cfg(test)]
#[path = "🧪️tests/🔬️sink-ports/🦀️.rs"]
mod sink_port_tests;
//#endregion 🧪️SinkPorts
