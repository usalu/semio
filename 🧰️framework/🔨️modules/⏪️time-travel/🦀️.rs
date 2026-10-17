//! ⏪️ Domain-neutral time-travel session: the pure, target-neutral reducer of one ephemeral,
//! local-only history edit (session identity, generation and base stamp, stages, accepted drafts
//! in history order, the pending draft with its return stage, the replay report, progress and
//! fault), its events, effects, refusals and EN/DE labels.
//!
//! Pure: no async, no store, no locale type. Schema of record: `🧬️schema/🔣️.json`; law fixture:
//! `🧫️fixtures/🧫️lifecycle-law/🔣️.json`; contract:
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/📋️design.md` §4.
//!
//! Driver contract (the host owning a [`TimeTravelSession`]):
//! - [`TimeTravelEvent::Begin`] carries the target's applied `position` resolved at the session base.
//! - Every change of the store's content revision is forwarded as [`TimeTravelEvent::BaseMoved`], in every
//!   stage (also `Inactive`), with the re-resolved positions of every session target; an unchanged base is
//!   `timeTravel.stale`. The store's local generation is no part of the base.
//! - [`TimeTravelEvent::BeginWithdrawn`] is a history row's Withdraw: `Begin` whose pending draft starts as
//!   `Withdrawn`, legal exactly where `Begin` is; the host sends [`TimeTravelEvent::Withdraw`] instead for the
//!   mutation already being edited.
//! - [`TimeTravelEvent::Restore`] is a history row's Restore: it takes the accepted draft of one mutation back while
//!   `Reviewing` and replays the rest; taking the only one back leaves time travel with zero trace.
//! - Every event except `Begin`, `BeginWithdrawn`, `BaseMoved` and `Exit` carries the session `generation` the
//!   host last observed; any other generation is `timeTravel.stale` (a silent no-op).
//! - [`TimeTravelSession::review`] classifies `Reviewing`: nothing accepted is "no changes" (the
//!   preview is the committed head); accepted drafts without a report (a cancelled or faulted replay)
//!   "need a replay", which [`TimeTravelEvent::Rerun`] starts; otherwise the report blocks or allows
//!   finalizing.
//! - Fault codes are non-empty, whitespace-free and at most [`TIME_TRAVEL_TEXT_MAX_BYTES`]; alternative
//!   names are not blank and at most [`TIME_TRAVEL_TEXT_MAX_BYTES`]. A malformed event is `timeTravel.illegal`.

use protocol::{InputReplacement, MutationId, ReplayReport, SupersededInput};
use semio_framework_value_derive::{FromValue, ToValue};
use std::cmp::Ordering;

#[path = "🔖️limits-and-codes/🦀️.rs"]
mod limits_and_codes;
pub use limits_and_codes::*;

#[path = "🪪️identity-and-session/🦀️.rs"]
mod identity_and_session;
pub use identity_and_session::*;

#[path = "📨️events/🦀️.rs"]
mod events;
pub use events::*;

#[path = "🎬️effects-and-refusals/🦀️.rs"]
mod effects_and_refusals;
pub use effects_and_refusals::*;

#[path = "⚙️reducer/🦀️.rs"]
mod reducer;

#[path = "🌍️labels/🦀️.rs"]
mod labels;
pub use labels::*;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[path="♻️retirement/🦀️.rs"]
mod original_retirement;

#[path="🎮️decision/🗑️discard/🦀️.rs"]
mod discard_decision;
pub use discard_decision::TimeTravelDiscardCursor;
#[path="🎮️decision/📨️command/🦀️.rs"]
mod pending_command;
pub use pending_command::TimeTravelCommandCustody;
