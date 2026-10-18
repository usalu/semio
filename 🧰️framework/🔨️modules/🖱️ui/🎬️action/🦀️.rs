//! 🧾 Fixed-credit action construction and FIFO ownership for interactive renderer input.
//!
//! 🎬️ Two channels leave this target, exactly as React's own Interpreter splits them
//! (`🗣️Interpreter/🟦️.tsx`'s `UiInterpreterContext`): every SEMANTIC control
//! (Button/Input/Select/Toggle/Slider/NumberStepper/Ring/IconSelect/Tree row) fires a
//! [`UiIntentCommand`] — the renderer-side twin of [`ui_contract::UiIntent`] — while the bare
//! [`ActionDescriptor`] stays reserved for the unowned scene-host elements and the chrome's own
//! immediate-mode widgets, which address no published node. An intent additionally carries the
//! addressing (`surface`/`revision`/`node`/`node_key`) that makes [`intent_is_stale`] and
//! [`BoundedActionQueue::admit_intent`] possible; a descriptor never could.

use semio_framework_ui_layout::layout::ActionDescriptor;
use semio_framework_value::{DslValue, Number};

#[path = "🧭️intent-address-and-sequencer/🦀️.rs"]
mod intent_address_and_sequencer;
pub use intent_address_and_sequencer::{
    intent_is_stale, UiIntentAddress, UiIntentBindings, UiIntentCommand, UiIntentSequencer,
    ACTION_BATCH_ITEM_CAPACITY, ACTION_CLAIM_BATCH_CAPACITY, ACTION_CLAIM_CAPACITY,
    ACTION_DEPTH_CAPACITY, ACTION_ITEM_BYTE_CAPACITY, ACTION_NODE_CAPACITY,
    ACTION_QUEUE_BYTE_CAPACITY, ACTION_QUEUE_ITEM_CAPACITY, ACTION_STRING_BYTE_CAPACITY,
    INTENT_DELTA_FIELD, INTENT_VALUE_FIELD, RETAINED_ACTION_STRING_BYTE_CAPACITY,
    RETAINED_ACTION_STRING_PAGE_BYTES,
};

#[path = "🧱️bounded-action-model/🦀️.rs"]
mod bounded_action_model;
pub use bounded_action_model::{
    checked_action_string_bytes, ActionQueueReceipt, ActionQueueReceiptSource, BoundedAction,
    BoundedActionBuilder, BoundedActionFault, QueuedActionDescriptor, RetainedStringAction,
    RetainedStringActionStep,
};
use bounded_action_model::{FlatNode, FlatValue, TextSpan};

#[path = "🏗️bounded-action-builder/🦀️.rs"]
mod bounded_action_builder;

#[path = "🎟️claims-and-reservations/🦀️.rs"]
mod claims_and_reservations;
pub use claims_and_reservations::{
    BoundedActionBatchReservation, BoundedActionClaim, BoundedActionClaimBatch,
    BoundedActionReservation, BoundedClaimedActionDraft, BoundedClaimedActionReservation,
    PreparedClaimedAction, PreparedClaimedActionBatch,
};
use claims_and_reservations::BoundedActionClaimSlot;

#[path = "📬️action-queue/🦀️.rs"]
mod action_queue;
pub use action_queue::{BoundedActionQueue, UiIntentAdmission};

#[cfg(test)]
#[path = "../🧪️tests/🔬️targets-wgpu-action-unit/🦀️.rs"]
mod tests;
