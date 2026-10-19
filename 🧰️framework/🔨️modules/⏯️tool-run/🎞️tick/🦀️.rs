//! 🎞️ Tool-run tick wire: the canonical tick record encoder and the funded native receiving of ticks, identities, steps, progress, entity columns and trace pages over the tool-run contract.

use semio_framework_tool_run::payload_stream;
use semio_framework_tool_run::{ToolRunCounter, ToolRunId, ToolRunIdentity, ToolRunPayloadUpdate, ToolRunProgress, ToolRunReceivingLimits, ToolRunState, ToolRunStep, ToolRunStepArg, ToolRunStepKind, ToolRunStepRing, ToolRunTick, ToolRunTickWriter, ToolRunTraceOp, ToolRunTracePage, ToolRunTraceSubject, ToolRunVerdict, TOOL_RUN_COUNTERS_MAX, TOOL_RUN_STEP_ARGS_MAX, TOOL_RUN_STEP_RING_CAPACITY, TOOL_RUN_TICK_BYTES_MAX, TOOL_RUN_TRACE_PAGE_BYTES_MAX, TOOL_RUN_TRACE_PAGE_OPS_MAX};
use pack::{PackLimits, record::RetainedRecordBodyCursor};

#[path="🛬️receive/🪪️identity/🦀️.rs"]
mod identity_receiving;
pub use identity_receiving::ToolRunIdentityReceiving;
#[path="🛬️receive/📝️step/🦀️.rs"]
mod step_receiving;
pub use step_receiving::ToolRunStepReceiving;
#[path="🛬️receive/🏗️native/🦀️.rs"]
mod native_tick_preparation;
pub use native_tick_preparation::{ToolRunNativeTickPreparation,ToolRunReceivedTick,ToolRunReceivedTickSource,ToolRunReceivedProgress,ToolRunReceivedTracePage};
#[path="🛬️receive/🔢️entities/🦀️.rs"]
mod entity_column_receiving;
pub use entity_column_receiving::ToolRunEntityColumnReceiving;
#[path="🛬️receive/📊️progress/🦀️.rs"]
mod progress_receiving;
pub use progress_receiving::{ToolRunProgressReceiving,ToolRunReceivedProgressSource};
#[path="🛬️receive/🦀️.rs"]
pub mod tick_receive;
#[path="🛫️encode/🦀️.rs"]
mod original_tick_encoding;
pub use original_tick_encoding::{ToolRunTickWireCursor,ToolRunTickPayloadSource};

#[cfg(feature = "testkit")]
#[doc(hidden)]
pub mod testkit {
    pub mod progress_receiving {
        pub use super::super::progress_receiving::Phase;
    }

    pub mod entity_column_receiving {
        pub use super::super::entity_column_receiving::Phase;
    }
}

#[cfg(test)]
#[path = "../📨️payload/🌊️stream/📡️wire/🧪️tests/🦀️.rs"]
mod payload_stream_wire_tests;
