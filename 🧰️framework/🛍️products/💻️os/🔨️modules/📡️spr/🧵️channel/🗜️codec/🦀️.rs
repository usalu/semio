//! 🔣️ Binary codec: option and vec combinators, pack readers and writers, and the `AppCommand` and `AppFrame` tag dispatch.

use ::protocol::wire::command_ingress::{FixedCommandPage, CommandPageSet, PagedCommand, COMMAND_PAGE_MAXIMUM_BYTES, COMMAND_MAXIMUM_PAGES, INVOCATION_RESULT_PACK_MAXIMUM_BYTES};
use super::{AppCommand, AppDocumentIdentity, AppFrame, ChildHeadPackEntry, ChildPackEntry, DOCUMENT_ARCHIVE_MAXIMUM_BYTES, DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS, DOCUMENT_ARCHIVE_VERSION, DocumentArchiveArtifactRef, DocumentArchiveLoadState, DocumentArchiveLoadStatus, DocumentArchiveOwnerRef, DocumentArchivePack, MediaExportHandleWire, MediaExportStateWire, OwnedDocumentMemberPackEntry, PresenceRosterWire, TRANSACTION_PREPARED_OPS_MAXIMUM, WindowConfigPackEntry};

#[path = "🧩️wire-combinators-and-writers/🦀️.rs"]
mod wire_combinators_and_writers;
pub(super) use wire_combinators_and_writers::*;

#[path = "📤️command-encoder/🦀️.rs"]
mod command_encoder;
pub use command_encoder::*;

#[path = "📚️archive-and-pack-codecs/🦀️.rs"]
mod archive_and_pack_codecs;
pub use archive_and_pack_codecs::*;

#[path = "📥️command-decoder/🦀️.rs"]
mod command_decoder;
pub use command_decoder::*;

#[path = "🎞️frame-codec/🦀️.rs"]
mod frame_codec;
pub use frame_codec::*;
