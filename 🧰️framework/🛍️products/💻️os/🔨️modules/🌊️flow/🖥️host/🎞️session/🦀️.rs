//! 🎞️ Flow host session: the evaluation session over a retained host, its tick and preview turns, publication custody and intake receivers.

use neural_engine as neural;
use std::collections::BTreeSet;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering as AtomicOrdering};
use protocol::causal::transition::HistoryFoldIndex;
use semio_framework::MeshData;
use semio_framework_value::retirement::{RetireOwned, RetirementCursor};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}, retirement::controlled::ControlledRetirement, retirement_queue::RetirementQueue};
use semio_framework_artifact_flow_flow::{artifact::*, widget_id_for};
use neural::{Atom, Dictionary, EvalChannels, EvalStepBudget, NeuralCache, Neuron, OperatorInfo, Tree, TreeSnapshot, Value as NeuralValue};
use serde::{Deserialize, Serialize};
use semio_framework_os_flow_host_core::*;
use semio_framework_os_flow_operators::bridge::is_global_eval_error_json;
use semio_framework_os_flow_operators::catalogue::FLOW_LOD_MODE_AUTOMATIC;
use semio_framework_os_flow_operators::registry::{flow_extension_registry_generation, seed_flow_eval_node_cache};
use semio_framework_os_flow_geometry::{GeometryPort, GeometryPortRetirement};
use semio_framework_os_flow_host_source::{seeds::FlowEvaluationSeeds, work::FlowEvaluationWork, FlowEvaluationSource};
use semio_framework_os_flow_host_read::FlowSourceBaseline;

// #region 🔖️EvalSession
#[path = "../🧾️eval-session-state/🦀️.rs"]
mod eval_session_state;
pub use eval_session_state::*;
#[path = "../⏱️eval-session/🦀️.rs"]
mod eval_session;

#[path = "../🔭️preview-status/🦀️.rs"]
mod preview_status;
pub use preview_status::*;

#[path = "../🚦️eval-status/🦀️.rs"]
mod eval_status;
pub use eval_status::*;
#[path = "../🌊️node-graph-extras/🦀️.rs"]
mod node_graph_extras;
pub use node_graph_extras::*;
// #endregion 🔖️EvalSession

#[path = "../📤️publication/🎟️session/🦀️.rs"]
mod session_publication;
use session_publication::SessionLivePublication;
#[path = "../📤️publication/📡️invocation/🦀️.rs"]
pub mod invocation_publication;
pub use invocation_publication::{FlowInvocationIdentity, FlowInvocationOrigin, FlowInvocationOriginLease};
use invocation_publication::FlowInvocationEntry;
#[path = "../📥️preview/🦀️.rs"]
pub mod preview_receiving;
pub use preview_receiving::{FlowPreviewCompletedAddress, FlowPreviewCompletedReceiver, FlowPreviewCompletedIntakeSource, FlowPreviewLedgerFold};
#[path = "../📥️evaluation/🦀️.rs"]
pub mod evaluation_receiving;
pub use evaluation_receiving::{FlowEvaluationReplyAddress, FlowEvaluationReplyReceiver, FlowEvaluationReplyIntakeSource};
pub use semio_framework_os_flow_host_read::{FlowCurrentOutput, FlowCurrentOutputRead, FlowPreviewSource, FlowPreviewSourceTransfer, FlowPreviewTessellationRequest, FlowPreviewTessellationWriter, FlowSourceRead, FlowSourceReadFailure, FlowSourceReadRequest, FlowSourceReadTarget, FlowSourceReadStep};

#[path = "../🚪️io/🦀️.rs"]
pub mod io;
