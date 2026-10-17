//! 🕸️ Workflow graph model: media contracts and ports, nodes and edges, declared inputs, validation, planning and parameters.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;

use semio_framework::{AppDefinition, MediaClass, MediaForm, MediaPortDirection, MediaPortSpec, MediaType, MediaWireFormat, PortMultiplicity};
use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::Terminology;
use std::collections::{HashMap, HashSet};

pub const WORKFLOW_SCHEMA: &str = "workflow.graph";

/// 🪶️ W3 "the inversion" document schema — the persisted `os.workflow` artifact (graph + parameters +
/// inputs/bindings, see [`WorkflowSnapshot`]), distinct from `WORKFLOW_SCHEMA` (the bare graph-only
/// sub-shape still embedded as `WorkflowSnapshot.graph`). Registered as a builtin artifact kind by
/// os-core's `seed_builtin_artifact_kinds`.
pub const S_WORKFLOW_SCHEMA: &str = "os.workflow";

#[path = "🤝️media-contract/🦀️.rs"]
mod media_contract;
pub use media_contract::*;

#[path = "🔢️media-enum-ordinals/🦀️.rs"]
mod media_enum_ordinals;
pub use media_enum_ordinals::*;

#[path = "🫳️borrowed-record-specs/🦀️.rs"]
mod borrowed_record_specs;
pub use borrowed_record_specs::*;

#[path = "📐️media-contract-record/🦀️.rs"]
mod media_contract_record;
pub use media_contract_record::*;

#[path = "🔌️media-port/🦀️.rs"]
mod media_port;
pub use media_port::*;

#[path = "🧱️graph-model/🦀️.rs"]
mod graph_model;
pub use graph_model::*;

#[path = "✅️validation-and-planning/🦀️.rs"]
mod validation_and_planning;
pub use validation_and_planning::*;

#[path = "🎛️parameters/🦀️.rs"]
mod parameters;
pub use parameters::*;

#[path = "📥️inputs/🦀️.rs"]
mod inputs;
pub use inputs::*;

//#region 🔖️MediaContractDsl

// 🚫️async: E4 fn-pointer slot — value goes into `dsl::Shape::Record(fn() -> RecordSpec)`.
use semio_framework_dsl_record::{BorrowedFieldSpec,BorrowedShape};

//#endregion 🔖️MediaContractDsl
