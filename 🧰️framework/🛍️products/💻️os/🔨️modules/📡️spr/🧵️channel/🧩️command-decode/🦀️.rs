//! 🧩️ Paged decoders for document archives, route fields, media-export handles and `AppCommand`s over bounded command pages.

use ::protocol::wire::command_ingress::{PagedCommand, PagedCommandReader, COMMAND_MAXIMUM_BYTES};
use super::{AppCommand, DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS, DocumentArchiveArtifactRef, DocumentArchiveOwnerRef, DocumentArchivePack, MediaExportHandleWire, OwnedDocumentMemberPackEntry, WindowConfigPackEntry, document_archive_into_fields};

#[path = "📦️archive-decode/🦀️.rs"]
mod archive_decode;
pub(super) use archive_decode::*;

#[path = "🛣️route-and-media-decode/🦀️.rs"]
mod route_and_media_decode;
use route_and_media_decode::*;

#[path = "🧾️command-owner/🦀️.rs"]
mod command_owner;
pub use command_owner::*;

#[path = "🧭️command-cursor/🦀️.rs"]
mod command_cursor;
