//! 📦️ Worker-owned render preparation and UI-authorized presentation contract.

use semio_framework_ui_draw_contract::{DrawLayer, DrawList, ScissorRect};
use semio_framework_job::{InteractiveJob, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView, StepContext};
use semio_framework_job_payload::{JobPayloadStream, RetainedPayloadBuilder};
use std::mem::size_of;
use semio_framework_ui_prepared_staging::retirement::*;
use std::rc::Rc;
use std::sync::atomic::{AtomicPtr, AtomicU64, AtomicU8, AtomicUsize, Ordering};
use std::sync::Mutex;
use semio_framework_job::{InteractiveJobCloseStep as CloseStep, RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_value::{ValueError, ValueRefusalKind};

#[path = "📊️credits-limits-and-fixed-lists/🦀️.rs"]
mod credits_limits_and_fixed_lists;
use credits_limits_and_fixed_lists::{
    prepared_abandoned_close, prepared_abandoned_demands, prepared_render_units,
    PREPARED_RENDER_BACKING_MASK, PREPARED_RENDER_BACKING_SHIFT, PREPARED_RENDER_ITEM_MASK,
    PREPARED_RENDER_ITEM_SHIFT, PREPARED_RENDER_JOB_ABANDONMENT_OWNER,
    PREPARED_RENDER_JOB_ABANDONMENT_STATE, PREPARED_RENDER_PACKET_ABANDONMENT_OWNER,
    PREPARED_RENDER_PACKET_ABANDONMENT_STATE, PREPARED_RENDER_PAGE_MASK, PREPARED_RENDER_PAGE_SHIFT,
    PREPARED_RENDER_PROCESS_PAGES, PREPARED_RENDER_PROCESS_SLOTS,
};
pub use credits_limits_and_fixed_lists::{
    PreparedFixedList, PreparedFixedPage, PreparedRenderLimits, PreparedRenderProcessPermit,
    PreparedRenderUsage, PREPARED_RENDER_COMMAND_PAGES, PREPARED_RENDER_COMMAND_PAGE_ITEMS,
    PREPARED_RENDER_INPUT_ABANDONMENT_OWNER, PREPARED_RENDER_INPUT_ABANDONMENT_STATE,
    PREPARED_RENDER_METADATA_ITEMS, PREPARED_RENDER_METADATA_PAGE_ITEMS,
    PREPARED_RENDER_PROCESS_PERMITS,
};

#[path = "🧾️render-commands-and-packet/🦀️.rs"]
mod render_commands_and_packet;
pub use render_commands_and_packet::{
    PreparedRasterKeepCursorV1, PreparedRasterKeepStepV1, PreparedRasterProducers,
    PreparedRenderCommand, PreparedRenderCommandKind, PreparedRenderCommandPages,
    PreparedRenderDirectives, PreparedRenderEvictions, PreparedRenderPacket, PreparedRenderScissors,
    PreparedRenderUploads,
};
use render_commands_and_packet::{
    PreparedRenderCommandDirectory, PreparedRenderCommandPage, PREPARED_RENDER_COMMAND_DIRECTORIES,
};

#[path = "📮️render-input-and-mailbox/🦀️.rs"]
mod render_input_and_mailbox;
pub use render_input_and_mailbox::{
    PreparedRenderInput, PreparedRenderInputRejected, PreparedRenderReceiver,
    PREPARED_RENDER_MAILBOX,
};

#[path = "🔧️render-job-state-and-close/🦀️.rs"]
mod render_job_state_and_close;
pub use render_job_state_and_close::{
    DrawMeasureCursor, PreparedRenderJob, PreparedRenderJobRejected,
};
use render_job_state_and_close::PreparationSection;

#[path = "📏️draw-usage-measuring/🦀️.rs"]
mod draw_usage_measuring;

#[path = "🏁️job-completion-and-interactive-job/🦀️.rs"]
mod job_completion_and_interactive_job;

#[path = "🛡️presentation-gate/🦀️.rs"]
mod presentation_gate;
pub use presentation_gate::{
    PreparedPresenterWitness, PreparedRenderGate, PreparedRenderRejection,
    PreparedRenderReplacement, UiPresentToken,
};
#[cfg(all(target_arch="wasm32",not(target_env="p2")))]
pub use presentation_gate::OffscreenPresentToken;

pub use semio_framework_ui_prepared_staging::*;

//#endregion 📦️Packet

#[cfg(test)]
extern crate self as semio_framework_ui_prepared;
#[cfg(any(test,feature="testkit"))]
#[path="🧪️testing/🦀️.rs"]
pub mod testing;
