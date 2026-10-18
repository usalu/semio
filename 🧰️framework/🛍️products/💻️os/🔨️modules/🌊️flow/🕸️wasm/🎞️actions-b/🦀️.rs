//! 🎞️ Flow program actions 2561 to 2610: one retained step machine per operation.

use semio_framework_os_infinite::board::ports::directed_dag as dag;
use semio_framework_os_flow_host_core::*;
use semio_framework_os_flow_drawing::*;
use semio_framework_os_flow_wasm_domain::flow_result;
use semio_framework_os_flow_wasm_domain::protocol::{FlowFailure, FlowFeatureAdmission, FlowFeatureStep};
use semio_framework_os_flow_wasm_domain::*;
use semio_framework_value::{retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}, retirement::controlled::ControlledRetirement};
use semio_framework_abi::{AbiErrorCode, AbiWorkBudget};
use std::rc::Rc;

#[path = "⏪️undo-redo-camera-and-wheel/🦀️.rs"]
mod undo_redo_camera_and_wheel;
pub use undo_redo_camera_and_wheel::{
    FlowAction2561, FlowAction2562, FlowAction2563, FlowAction2564, FlowAction2565, FlowAction2566,
    FlowAction2567,
};

#[path = "🔭️level-of-detail/🦀️.rs"]
mod level_of_detail;
pub use level_of_detail::{
    FlowAction2568, FlowAction2569, FlowAction2570, FlowAction2571, FlowAction2572, FlowAction2573,
    FlowAction2574,
};

#[path = "🖼️surface-theme-reorganize-and-render/🦀️.rs"]
mod surface_theme_reorganize_and_render;
pub use surface_theme_reorganize_and_render::{
    FlowAction2575, FlowAction2576, FlowAction2577, FlowAction2578, FlowAction2579, FlowAction2580,
    FlowAction2581,
};

#[path = "🎯️pointer-and-picking/🦀️.rs"]
mod pointer_and_picking;
pub use pointer_and_picking::{
    FlowAction2582, FlowAction2583, FlowAction2584, FlowAction2585, FlowAction2586, FlowAction2587,
};

#[path = "✅️selection/🦀️.rs"]
mod selection;
pub use selection::{
    FlowAction2588, FlowAction2589, FlowAction2590, FlowAction2591, FlowAction2592, FlowAction2593,
    FlowAction2594, FlowAction2595, FlowAction2596, FlowAction2597, FlowAction2598,
};

#[path = "📤️drawing-export-and-snapshots/🦀️.rs"]
mod drawing_export_and_snapshots;
pub use drawing_export_and_snapshots::{
    FlowAction2599, FlowAction2600, FlowAction2601, FlowAction2602, FlowAction2605, FlowAction2606,
    FlowAction2607, FlowAction2608, FlowAction2609, FlowAction2610,
};
