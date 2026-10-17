//! 🌉️ Flow editor owned byte/message bridge and primitive linear-memory exports.

use semio_framework_test_fixtures::fixture_path;

use semio_framework_abi::{
    AbiBytes, AbiControl, AbiCursorStep, AbiError, AbiErrorCode, AbiEvent, AbiEventCode, AbiHandle, AbiHandleTable, AbiMessage, AbiMessageBytes, AbiPage, AbiPageReader, AbiPort, AbiPortPoll, AbiPortRejection, AbiReply, AbiReplyLedger,
    AbiRequest, AbiRequestId, AbiStatus, AbiStatusCode, AbiWorkBudget, decode_abi_message, encode_abi_message,
};
use semio_framework_value::{retained_clone::{RetainedCloneStep, RetainedCloneProgress, RetainedCloneGrant, admit_retained_clone_close}, retirement::controlled::ControlledRetirement};
use semio_framework_os_flow_vcs::{FlowRetainedVcs, FlowVcsCloseDemands};
use semio_framework_os_flow_host_core::*;
use semio_framework_os_flow_wasm_domain::*;
use semio_framework_os_flow_wasm_actions_a::*;
use semio_framework_os_flow_wasm_actions_a::retained_dag_input;
use semio_framework_os_flow_wasm_actions_b::*;
use semio_framework_os_flow_wasm_domain::protocol::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
pub use semio_framework_os_flow_wasm_domain::protocol;

//#region 🔖️ReactiveFeatures
#[path = "🌉️domain-impl/🦀️.rs"]
mod domain_impl;
#[path = "🧭️dispatch/🦀️.rs"]
mod dispatch;
use dispatch::*;
//#endregion 🔖️ReactiveFeatures

//#region 🔖️Runtime
#[path = "📡️protocol/🪢️domain/🦀️.rs"]
mod domain_trait;
pub use domain_trait::*;
#[path = "📡️protocol/🌉️bridge/🦀️.rs"]
mod bridge;
pub use bridge::*;
//#endregion 🔖️Runtime

#[path = "🔗️linear-memory/🦀️.rs"]
mod linear_memory;
pub use linear_memory::*;

//#region 🧪️Laws
#[cfg(test)] use semio_framework_artifact_flow_flow::artifact::Widget;
#[cfg(test)] use semio_framework_os_flow_drawing::*;
#[cfg(test)] use semio_framework_os_flow_host::*;
#[cfg(test)] use semio_framework_os_infinite::board::ports::directed_dag as dag;
#[cfg(test)] use semio_framework_canvas as canvas;
#[cfg(test)] use protocol::{FlowBridge, FlowDomain, FlowFailure, FlowFeature, FlowFeatureAdmission, FlowFeatureStep, FlowPayloadReader, FlowPayloadWriter};
#[cfg(test)] use semio_framework_value::retained_clone::{RetainedCloneStep, RetainedCloneProgress, RetainedCloneGrant};
#[cfg(test)] use semio_framework_value::retirement::controlled::ControlledRetirement;
#[cfg(test)] use semio_framework_abi::{decode_abi_message, AbiErrorCode, AbiMessage, AbiPortPoll, AbiWorkBudget};
#[cfg(test)] use serde_json::json;
#[cfg(test)] use serde_json::Value;
#[cfg(test)] use ui_webgpu::{CanvasMetrics, SurfaceGeneration, SurfaceId};
/// 🧾️ Reads one source or schema document below the wasm session node at test time instead of compiling it into the crate.
#[cfg(test)]
fn read_source(path: &str) -> String {
    std::fs::read_to_string(fixture_path!(path)).unwrap_or_else(|error| panic!("{path}: {error}"))
}

/// 🧾️ The concatenated production source of the listed files below the wasm session node.
#[cfg(test)]
fn production_source(files: &[&str]) -> String {
    files.iter().map(|file| read_source(&format!("../../{file}"))).collect::<Vec<_>>().join("\n")
}

#[cfg(test)]
#[path = "🧪️tests/🔬️protocol-unit/🦀️.rs"]
mod protocol_tests;
//#endregion 🧪️Laws

//#region 🧪️DomainLaws
#[cfg(test)]
#[path = "🧪️tests/🔬️component-domain-laws/🦀️.rs"]
mod domain_laws;

#[cfg(test)]
#[path = "🧪️tests/🎬️draw-list/🦀️.rs"]
mod draw_list_laws;
//#endregion 🧪️DomainLaws
