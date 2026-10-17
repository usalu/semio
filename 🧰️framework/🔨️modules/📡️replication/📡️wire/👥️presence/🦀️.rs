//! 👥️ Peer presence wire contract: window views, typing, tool runs, history edits and the typed interaction roster, with the borrowed reader and the peer overlay.
//!
//! Presence travels inside the lane-tagged frames only as opaque blobs; this crate owns what the blobs contain.

use std::collections::BTreeMap;
use semio_framework_replication_interaction::HoverSpec;
use semio_framework_replication_interaction::InteractionState;
use semio_framework_replication_interaction::SelectionSpec;
use semio_framework_replication_wire_core::malformed;
use semio_framework_replication_wire_core::write_opt_str;

//#region 🔖️Presence
// 🎯️ W6 kernel unification: `PayloadHash`/`MutationEnvelope`/`MutationDagError`/`MutationDag`/`InsertResult`
// (the local causal-sync types) and `HubClientFrame`/`HubServerFrame` (the local semio_hub wire frames)
// are DELETED — `store`/`store_sync` (their only consumers outside this crate) now speak
// `protocol::{MutationEnvelope, MutationDag, MutationDagError, InsertResult}`/`protocol::{ClientFrame,
// ServerFrame}` directly (W5 already made these real binary types; this wave just stops
// duplicating them here). `PresencePoint`/`PresenceViewport`/`PresencePeer` below are NOT
// duplicates of anything in `protocol` — no equivalent exists there — so they stay, kept in their
// own region since the `🔖️HubProtocol` name they used to share with the now-deleted frame enums no
// longer fits. 🎯️ ticket 26/08/17/SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION
// C7.1: `PresencePoint`/`PresenceViewport` and `PresencePeer.cursor`/`.viewport` are DELETED —
// replaced by `views: Vec<PresenceWindowView>` (one entry per open window/surface, artifact-scope,
// matched by `space`) plus `ui: Option<PresenceUi>` (app-scope `data-ui-path` hover/focus/press).

#[path = "🪟️view-records-and-codec/🦀️.rs"]
mod view_records_and_codec;
pub use view_records_and_codec::*;

#[path = "👤️peer-records/🦀️.rs"]
mod peer_records;
pub use peer_records::*;

#[path = "📤️peer-wire-encode/🦀️.rs"]
mod peer_wire_encode;
pub use peer_wire_encode::*;

#[path = "📥️peer-wire-decode/🦀️.rs"]
mod peer_wire_decode;
pub use peer_wire_decode::*;

#[path = "🕹️interaction-and-domain/🦀️.rs"]
mod interaction_and_domain;
pub use interaction_and_domain::*;

#[cfg(test)]
#[path = "../🧪️tests/🔬️presence-codec/🦀️.rs"]
mod presence_codec_tests;

//#endregion 🔖️Presence

//#region 🔖️PresenceInteraction

//#region 🔖️Assemble

#[cfg(test)]
#[path = "../🧪️tests/🔬️assemble-presence-interaction/🦀️.rs"]
mod assemble_presence_interaction_tests;
//#endregion 🔖️Assemble
//#endregion 🔖️PresenceInteraction

#[path = "../../🚪️io/👥️presence/🔍️borrowed/🦀️.rs"]
pub mod borrowed;
pub use borrowed::{borrow_presence_metadata, BorrowedPresenceMetadata, BorrowedPresenceInteraction, BorrowedPresenceDomain, BorrowedPresenceStrings};

#[path = "../../👕️peer-overlay/🦀️.rs"]
pub mod peer_overlay;
pub use peer_overlay::*;
