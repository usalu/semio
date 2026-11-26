//! 📡️ The canonical borrowed actor wire paths retain each typed input under its original turn.
use semio_framework_job::{OriginalTurnFailure,StepContext};
use semio_framework_value::{RetainedCloneProgress,ValueError,ValueRefusalKind};
use semio_plugin_abi::owned_abi::ActorInput;

#[path="🌀️empty-poll.rs"]
pub mod empty_poll;

#[path="🌀️poll-wire.rs"]
mod poll_wire;

#[path="🚪️ordinary-open.rs"]
pub mod ordinary_open;

#[path="🪪️lifecycle-ack.rs"]
pub mod lifecycle_ack;

#[path="🛬️decode.rs"]
pub mod decode;
