//! 🧭️ Chooses how a stored folder archive reopens: the contiguous decoder below the former ceiling, bounded paged receiving above it.
use super::archive_schema::{DOCUMENT_ARCHIVE_MAXIMUM_BYTES, DOCUMENT_ARCHIVE_VERSION};

/// 📏️ The largest archive the contiguous reopen path admits: a quarter of the persisted authority, the former 4 MiB archive ceiling.
pub const DOCUMENT_ARCHIVE_CONTIGUOUS_REOPEN_MAXIMUM_BYTES: usize = DOCUMENT_ARCHIVE_MAXIMUM_BYTES / 4;

/// 🧭️ The reopen path selected for one stored archive.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DocumentArchiveReopenRoute {
    Contiguous,
    Paged,
    Refused,
}

/// 🧭️ Selects from the encoded extent and the archive's first and last bytes alone; only a memberless version-1 archive can reopen paged.
pub fn select_document_archive_reopen_route(extent: usize, first: u8, last: u8) -> DocumentArchiveReopenRoute {
    if extent == 0 || extent > DOCUMENT_ARCHIVE_MAXIMUM_BYTES || first != DOCUMENT_ARCHIVE_VERSION {
        DocumentArchiveReopenRoute::Refused
    } else if extent <= DOCUMENT_ARCHIVE_CONTIGUOUS_REOPEN_MAXIMUM_BYTES {
        DocumentArchiveReopenRoute::Contiguous
    } else if last == 0 {
        DocumentArchiveReopenRoute::Paged
    } else {
        DocumentArchiveReopenRoute::Refused
    }
}
