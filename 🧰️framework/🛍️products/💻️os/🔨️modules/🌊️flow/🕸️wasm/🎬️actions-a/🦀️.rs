//! 🎬️ Flow program actions 2504 to 2560: one retained step machine per operation.

use semio_framework_os_infinite::board::ports::directed_dag as dag;
use semio_framework_os_flow_host_core::*;
use semio_framework_os_flow_wasm_domain::flow_result;
use semio_framework_os_flow_wasm_domain::protocol;
use semio_framework_os_flow_wasm_domain::protocol::{FlowFailure, FlowFeatureAdmission, FlowFeatureStep};
use semio_framework_os_flow_wasm_domain::*;
use semio_framework_value::{retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}, retirement::controlled::ControlledRetirement};
use semio_framework_abi::{AbiErrorCode, AbiWorkBudget};
use std::rc::Rc;
#[path = "../🧵️dag-input/🦀️.rs"]
pub mod retained_dag_input;

#[path = "🗒️catalogue-ports-and-evaluation/🦀️.rs"]
mod catalogue_ports_and_evaluation;
pub use catalogue_ports_and_evaluation::{
    FlowAction2504, FlowAction2505, FlowAction2506, FlowAction2507, FlowAction2508, FlowAction2509,
    FlowAction2510, FlowAction2511, FlowAction2512, FlowAction2513,
};

#[path = "🖱️computing-hover-and-preview/🦀️.rs"]
mod computing_hover_and_preview;
use computing_hover_and_preview::dag_input_failure;
pub use computing_hover_and_preview::{
    FlowAction2516, FlowAction2517, FlowAction2521, FlowAction2524, FlowAction2526, FlowAction2527,
    FlowAction2529,
};

#[path = "🗜️preview-toggle-clusters-and-export/🦀️.rs"]
mod preview_toggle_clusters_and_export;
pub use preview_toggle_clusters_and_export::{
    FlowAction2530, FlowAction2531, FlowAction2532, FlowAction2533, FlowAction2534, FlowAction2535,
};

#[path = "📝️slider-and-note-editing/🦀️.rs"]
mod slider_and_note_editing;
pub use slider_and_note_editing::{
    FlowAction2536, FlowAction2537, FlowAction2538, FlowAction2539, FlowAction2540, FlowAction2541,
    FlowAction2542, FlowAction2543, FlowAction2544, FlowAction2545,
};

#[path = "🖼️image-schemas-and-variables/🦀️.rs"]
mod image_schemas_and_variables;
pub use image_schemas_and_variables::{
    FlowAction2546, FlowAction2547, FlowAction2548, FlowAction2549,
};

#[path = "🔗️widget-graph-editing/🦀️.rs"]
mod widget_graph_editing;
pub use widget_graph_editing::{
    FlowAction2550, FlowAction2551, FlowAction2552, FlowAction2553, FlowAction2554, FlowAction2555,
    FlowAction2556, FlowAction2557, FlowAction2558, FlowAction2559, FlowAction2560,
};
