//! 🎞️ Protocol app-engine channel: the `AppCommand`/`AppFrame` binary frame taxonomy every app,
//! once turned into a headless engine driven by bidirectional streaming of typed binary commands,
//! exchanges with its client (a UI or a headless runner) — every UI interaction becomes a
//! forwarded `AppCommand`, every engine reaction a returned `AppFrame`. Ticket:
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️01/HEADLESS-APP-ENGINE-BINARY-COMMAND-PROTOCOL-FOUNDATIONS/`.
//!
//! 🎯️ Mirrors `protocol_wire`'s W5 hand-rolled binary layout exactly: `tag: u8` (assigned
//! sequentially in match-arm declaration order below, NOT the enum's own discriminant) followed by
//! its fields in declaration order, no per-field tags, no body-length prefix — one frame per
//! channel message. `protocol::wire::🔖️WireCodec` supplies the primitive codec
//! (`write_varint_u64`/`write_str`/`write_bytes`/`write_bool` and their `read_*` twins); this crate
//! adds only the option/vec combinators and the two enums' tag dispatch below. Unlike
//! `protocol::wire::ClientFrame`/`ServerFrame`, `AppCommand`/`AppFrame` carry no `Lane` byte —
//! the app-engine channel is a single logical stream, not split into causally-ordered vs.
//! best-effort lanes.

//#region 🔖️Version
/// 🔢️ The channel wire format's own version, pinned against the shared cross-language
/// fixture `🔖️channel-version.json` so a half-done bump fails a test instead of drifting silently.
/// Channel v12 (`📓️design-abi.md` §2 "`exchange` collapse") retires the `AppCommand::Hello` /
/// `AppFrame::Welcome` handshake entirely — lifecycle now arrives through the reactor ABI's
/// `Event::InstanceOpen`/`InstanceClose`, so this constant is no longer carried on the wire by any
/// frame; it exists purely as the drift guard the tests below assert against.
pub const CHANNEL_VERSION: u32 = 23;
//#endregion 🔖️Version

//#region 🔖️ChannelHandshake
/// 🤝️ Fault code of a guest compiled for another app-channel version than its host. The host reads the guest's version
/// (`reactor.channel-version`, owned twin `semio_owned_channel_version_v1`) once, right after instantiation and before any
/// frame; its framework notice names `{guest}` and `{host}`.
pub const CHANNEL_MISMATCH_CODE: &str = "plugin.channel-mismatch";

/// 🤝️ Admits a guest compiled for the host's app-channel version; any other version, older or newer, is refused before a single
/// frame is exchanged, so a stale component is never misread under a stale or forged descriptor. Corpus
/// `🧫️fixtures/🧫️channel-handshake`; TS twin `admitGuestChannelVersion`.
pub fn admit_guest_channel_version(guest: u32, host: u32) -> Result<(), semio_framework_diagnostic::Fault> {
    if guest == host {
        return Ok(());
    }
    Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new(CHANNEL_MISMATCH_CODE), format!("the guest speaks app channel {guest}, the host app channel {host}"))
        .with_param("guest", guest.to_string())
        .with_param("host", host.to_string()))
}
//#endregion 🔖️ChannelHandshake

#[path = "🪪️wire-entries/🦀️.rs"]
mod wire_entries;
pub use wire_entries::{AppDocumentIdentity, ChildHeadPackEntry, ChildPackEntry, MediaExportHandleWire, MediaExportStateWire, WindowConfigPackEntry};



//#region 🔖️DocumentArchive
#[path="📦️archive/🧬️schema/🦀️.rs"]
pub mod archive_schema;
pub use archive_schema::{DocumentArchiveArtifactRef,DocumentArchiveOwnerRef,OwnedDocumentMemberPackEntry,DocumentArchivePack,DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS,DOCUMENT_ARCHIVE_MAXIMUM_BYTES,DOCUMENT_ARCHIVE_VERSION};
#[path="📦️archive/🛬️decode/🦀️.rs"]
pub mod archive_decoding;
pub use archive_decoding::{decode_document_archive_bytes_controlled,decode_document_archive_head_controlled};

#[path = "🚚️archive-load/🦀️.rs"]
mod archive_load;
pub use archive_load::{DocumentArchiveLoadHost, DocumentArchiveLoadOutcome, DocumentArchiveLoadRefusal, DocumentArchiveLoadState, DocumentArchiveLoadStatus, DocumentArchiveLoadStep};
//#endregion 🔖️DocumentArchive




#[path = "👥️presence-roster/🦀️.rs"]
mod presence;
pub use presence::{PRESENCE_COMMAND_MAXIMUM_BYTES, PRESENCE_ROSTER_MAXIMUM_BYTES, PRESENCE_ROSTER_MAXIMUM_ENTRY_BYTES, PRESENCE_ROSTER_MAXIMUM_ITEMS, PresenceCommandCursor, PresenceRosterPushRejected, PresenceRosterWire};

#[path = "🧩️command-decode/🦀️.rs"]
mod command_decode;
use command_decode::TRANSACTION_PREPARED_OPS_MAXIMUM;
pub use command_decode::{DecodedAppCommandOwner, PagedAppCommandDecodeCursor};

#[path = "📨️app-command/🦀️.rs"]
mod app_command;
pub use app_command::AppCommand;

#[path = "🎞️app-frame/🦀️.rs"]
mod app_frame;
pub use app_frame::{AppFrame,AppEmitFrameView,AppEmitFrameWireCursor,AppEmitFrameWireProgress};


#[path = "🗜️codec/🦀️.rs"]
mod codec;
use codec::{document_archive_into_fields, malformed};
pub use codec::{decode_app_command, decode_app_frame, decode_document_archive_bytes, encode_app_command, encode_app_frame, encode_document_archive_bytes, encode_local_interaction_query_frame_into, presence_command_sequence};


/// 🪝️ Test hook of the companion crate `semio-framework-os-channel-tests`: the private codec modules, public only behind the `testkit` feature.
#[cfg(feature = "testkit")]
#[doc(hidden)]
pub mod testkit {
    pub mod wire_entries {
        pub use crate::wire_entries::*;
    }
    pub mod archive_load {
        pub use crate::archive_load::*;
    }
    pub mod presence {
        pub use crate::presence::*;
    }
    pub mod command_decode {
        pub use crate::command_decode::*;
    }
    pub mod app_command {
        pub use crate::app_command::*;
    }
    pub mod app_frame {
        pub use crate::app_frame::*;
    }
    pub mod codec {
        pub use crate::codec::*;
    }
}
