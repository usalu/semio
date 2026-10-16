//! 🪟️ Wrapper namespace of the window laws: the imports of the host's window module and the browser seam that the laws name through `super`.

use semio_framework_abi::{AbiBytes, AbiControl, AbiErrorCode, AbiEvent, AbiMessage, AbiOperation, AbiPage, AbiPort, AbiPortPoll, AbiReply, AbiReplyLedger, AbiRequest, AbiRequestId, AbiStatusCode, AbiWorkBudget};
use ui_render::{CursorRequest, DispatchEvent, FrameScheduler, ImeDirective, InvalidationReason, PhysicalSize};

pub use semio_framework_ui_host::*;

pub mod browser {
    pub use semio_framework_ui_host::linear_memory_test_import;
}

#[path = "../../🔬️window-unit/🦀️.rs"]
mod unit;
