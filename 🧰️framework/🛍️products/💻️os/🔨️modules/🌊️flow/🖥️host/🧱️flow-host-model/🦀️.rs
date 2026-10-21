//! 🧱️ Flow host core: the retained `FlowHost` value, its graph editing, evaluation steps, history and close ladder, with host errors and shared payloads.

use semio_framework_os_infinite::board::schema::dag_input::{DagSelectionDomains, DagNodeStatuses};
use semio_framework_os_infinite as infinite;
use semio_framework_os_infinite::board::ports::directed_dag as dag;
use semio_framework_canvas as canvas;
use neural_engine as neural;
use semio_framework_os_infinite_directed_presentation::dag_host_snapshot_to_wire_literal;

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::sync::Arc;
use protocol::causal::transition::HistoryFoldIndex;
use semio_framework_value_ordered::ordered::OrderedMap;
use semio_framework_value::retirement::{RetireOwned, RetirementCursor};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}, retirement::controlled::ControlledRetirement};

use semio_framework_os_infinite::board::schema::layout::DagLayoutOptions;
use dag::{fit_node_size, would_create_cycle, DagHost};
use semio_framework_os_infinite_directed_presentation::{dag_host_snapshot_execution_rows, DagHostSnapshot, DagHostSnapshotEdge};

use semio_framework_os_infinite_directed_presentation::{DagNodeKind, DagNodeSpec, EdgeRouteStyle, IoPortSpec};
use semio_framework_artifact_flow_flow::{retire_flow_mutation, widget_id_for, AddSynapse, AddWidget, ChangeLayout, ChangeSynapse, ChangeWidget, FlowLayoutEntry, FlowMutation, FlowStore, RemoveSynapse, RemoveWidget};
use semio_framework_graph_dsl::{WireEdge, WireNode};
use semio_framework_graph_manifest::{PropertyBag, PropertyValue};
use neural::{
    channel_output, compute_dirty_set, Atom, BudgetedEval, ColdRetire, Dictionary, EvalChannels, EvalError, EvalStepBudget, Evaluator, NeuralCache, Neuron, OperatorInfo, Synapse, Tree, TreeSnapshot,
    Value as NeuralValue, CLUSTER_KIND, INPUT_KIND, OUTPUT_KIND,
};

use semio_framework_artifact_flow_flow::{artifact::*, retained};
use semio_framework_os_flow_operators::bridge::*;
use semio_framework_os_flow_operators::catalogue::{node_graph_operator_record_to_operator_info, static_catalogue_sections, CatalogueSection};
use semio_framework_os_flow_operators::registry::{flow_extension_registry_generation, flow_operator_registry};
use semio_framework_os_flow_extensions::evaluation_progress;
use semio_framework_os_flow_geometry::{GeometryPort, GeometryPortRetirement};
use semio_framework_os_flow_vcs::history_identity;
use semio_framework_os_kernel::{os_spr, os_store};
use semio_framework_os_kernel::os_store::store_schema::ArtifactCommand;
use semio_framework_os_flow_host_source::display::FlowDisplayCursor;
use semio_framework_os_flow_host_source::work::FlowEvaluationWork;

#[path = "../🧹️retirement/🦀️.rs"]
mod original_retirement;
use original_retirement::FlowHostPayload;

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum FlowEvaluationStep{Working,Complete}

#[path = "../⚠️errors/🦀️.rs"]
mod errors;
pub use errors::*;

// #region 🔖️FlowHost
#[path = "../🏠️flow-host/🦀️.rs"]
mod flow_host;
pub use flow_host::*;
#[path = "../✏️host-authoring/🦀️.rs"]
mod host_authoring;
#[path = "../🌳️host-evaluation-history/🦀️.rs"]
mod host_evaluation_history;

#[path = "../🏁️host-close/🦀️.rs"]
mod host_close;
pub use host_close::*;
#[path = "../🛍️host-catalogue/🦀️.rs"]
mod host_catalogue;
pub use host_catalogue::*;

#[path = "../🔌️ports/🦀️.rs"]
pub mod ports;
pub use ports::*;
// #endregion 🔖️FlowHost

#[path = "../📐️widget-size/🦀️.rs"]
mod widget_size;
pub use widget_size::*;
