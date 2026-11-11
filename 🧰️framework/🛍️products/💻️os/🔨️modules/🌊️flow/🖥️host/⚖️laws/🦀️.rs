//! ⚖️ Flow host retirement, work continuation and source read laws: the suites that drive a retained host together with its evaluation session.

#[cfg(test)] use semio_framework_test_fixtures::fixture_text;

pub use semio_os_flow_host_core::*;
pub use semio_os_flow_host_read::*;
pub use semio_os_flow_host_session::*;
pub use semio_os_flow_host_source::{seeds::FlowEvaluationSeeds, work::FlowEvaluationWork, FlowEvaluationSource};

#[cfg(test)] use semio_os_infinite::board::ports::directed_dag as dag;
#[cfg(test)] use neural_engine as neural;
#[cfg(test)] use std::collections::{BTreeMap, HashMap};
#[cfg(test)] use std::sync::Arc;
#[cfg(test)] use protocol::causal::transition::HistoryFoldIndex;
#[cfg(test)] use semio_framework::MeshData;
#[cfg(test)] use semio_framework_value::retirement::RetireOwned;
#[cfg(test)] use semio_framework_value::RetirementDemand;
#[cfg(test)] use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
#[cfg(test)] use semio_framework_value::retirement::controlled::ControlledRetirement;
#[cfg(test)] use dag::DagHost;
#[cfg(test)] use semio_os_infinite_directed_presentation::DagHostSnapshot;
#[cfg(test)] use semio_os_infinite_directed_presentation::{DagNodeKind, DagNodeSpec, IoPortSpec};
#[cfg(test)] use semio_framework_artifact_flow_flow::{FlowStore, FLOW_DOCUMENT_SCHEMA};
#[cfg(test)] use semio_framework_graph_manifest::PropertyBag;
#[cfg(test)] use neural::{Atom, Dictionary, EvalChannels, Neuron, OperatorInfo, Tree, TreeSnapshot, Value as NeuralValue};
#[cfg(test)] use semio_framework_artifact_flow_flow::artifact::*;
#[cfg(test)] use semio_os_flow_operators::bridge::*;
#[cfg(test)] use semio_os_flow_operators::catalogue::*;
#[cfg(test)] use semio_os_flow_drawing::*;
#[cfg(test)] use semio_os_kernel::os_store::document_materialize::create_document_envelope;
#[cfg(test)] use semio_os_flow_operators::registry::*;

#[cfg(test)]
#[path = "../🧹️retirement/🧪️tests/🧹️retirement/🦀️.rs"]
mod session_retirement_tests;
#[cfg(test)]
#[path = "../📥️evaluation-source/🧵️work/🧪️tests/🦀️.rs"]
mod work_tests;
#[cfg(test)]
#[path = "../📥️evaluation-source/🔎️read/🧪️tests/🦀️.rs"]
mod read_tests;

#[cfg(test)]
#[path = "../📤️publication/📡️invocation/🧪️tests/🦀️.rs"]
mod invocation_tests;
#[cfg(test)]
#[path = "../📥️evaluation/🧪️tests/🦀️.rs"]
mod evaluation_tests;
#[cfg(test)]
#[path = "../📥️preview/🧪️tests/🦀️.rs"]
mod preview_tests;

/// 🧾️ Reads the actual host fixture through its existing runtime fixture-reader authority.
#[cfg(test)]
fn read_node(path:&str)->&'static str{fixture_text!(format!("../../../{path}"))}
#[cfg(test)]
#[path="../📤️publication/🧵️work/🧪️tests/🦀️.rs"]
mod publication_work_tests;

#[cfg(test)] use std::mem::size_of;


#[cfg(test)] use neural::ColdRetire;
#[cfg(test)] use semio_os_flow_host_session::invocation_publication::{FlowInferenceDependencyWork,FlowInvocationRequestWork,FlowInvocationOriginWork};
#[cfg(test)]
#[path="../📤️publication/📡️invocation/🌳️dependency-work/🧪️laws/🦀️.rs"]
mod dependency_work_tests;
#[cfg(test)]
#[path="../📤️publication/📡️invocation/🧾️request-work/🧪️laws/🦀️.rs"]
mod invocation_request_work_tests;
#[cfg(test)]
#[path="../📤️publication/📡️invocation/📥️origin-work/🧪️laws/🦀️.rs"]
mod invocation_origin_work_tests;


#[cfg(test)]
#[path="../📥️evaluation-source/🔣️receive/🧪️laws/🦀️.rs"]
mod original_json_field_laws;

#[cfg(test)] use semio_os_flow_host_source::receiving::JsonField;

#[cfg(test)]
#[path="../📥️evaluation-source/🧵️work/🧪️tests/⏱️opportunity/🦀️.rs"]
mod original_work_opportunity_laws;

#[cfg(test)] use semio_os_flow_host_session::invocation_publication::FlowInvocationDispatchWork;
#[cfg(test)] use semio_os_flow_operators::registry::catalogue_work::FlowOperatorCatalogueWork;
#[cfg(test)] use neural::{SharedRegistry,RegistryIdentity};
#[cfg(test)] type Infos=HistoryFoldIndex<String,OperatorInfo>;
#[cfg(test)]
#[path="../📤️publication/📡️invocation/📨️dispatch-work/🧪️laws/🦀️.rs"]
mod invocation_dispatch_work_tests;
#[cfg(test)]
#[path="../../📔️registry/📇️catalogue-work/🧪️laws/🦀️.rs"]
mod operator_catalogue_work_tests;
#[cfg(test)]
#[path="../📤️publication/📡️invocation/📥️origin-work/🧪️laws/📏️opportunity/🦀️.rs"]
mod invocation_opportunity_tests;
