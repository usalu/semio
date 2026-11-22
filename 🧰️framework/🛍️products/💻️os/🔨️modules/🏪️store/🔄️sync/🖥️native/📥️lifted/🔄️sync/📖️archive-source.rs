//! 📖 Selects the durable folder record and lends its exact bytes through bounded reads.
use super::{FolderEventLogStorage, archive_readback::{DocumentRecord, read_pages, scan_folder}};
use std::fs::File;
use std::io::{self, ErrorKind, Read, Seek, SeekFrom};

pub const FOLDER_ARCHIVE_PAGE_BYTES: usize = 4096;

/// 🔏 Identifies the selected archive in the verified folder prefix.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NativeFolderArchiveSeal {
    start: u64,
    pub extent: usize,
    pub sha256: [u8; 32],
}

/// 📄 Holds the descriptor of the verified record without borrowing a writer lock across guest turns.
pub struct NativeFolderArchiveSource {
    file: File,
    seal: NativeFolderArchiveSeal,
    offered: usize,
}

impl NativeFolderArchiveSource {
    pub fn seal(&self) -> NativeFolderArchiveSeal { self.seal }
    pub fn offered(&self) -> usize { self.offered }

    /// 🪟 Fills at most the next archive window and advances after a complete read.
    pub fn read_next(&mut self, output: &mut [u8], cancelled: &impl Fn() -> bool) -> io::Result<usize> {
        if cancelled() {
            return Err(io::Error::new(ErrorKind::Interrupted, "folder archive source cancelled"));
        }
        let count = output.len().min(FOLDER_ARCHIVE_PAGE_BYTES).min(self.seal.extent - self.offered);
        if count == 0 {
            return Ok(0);
        }
        self.file.seek(SeekFrom::Start(self.seal.start + self.offered as u64))?;
        self.file.read_exact(&mut output[..count])?;
        self.offered += count;
        Ok(count)
    }
}

impl FolderEventLogStorage {
    /// 🧾 Captures the latest exact schema-bound record in the folder's append-only event log.
    pub fn open_archive_source(
        &self, document: &str, schema: &str, maximum_bytes: usize, cancelled: &impl Fn() -> bool,
    ) -> io::Result<Option<NativeFolderArchiveSource>> {
        if document.is_empty() || schema.is_empty() || document.len() > 2048 || schema.len() > 2048 {
            return Err(io::Error::new(ErrorKind::InvalidInput, "folder archive source requires a bounded document and schema"));
        }
        let _guard = self.writer.try_lock().map_err(|_| io::Error::new(ErrorKind::WouldBlock, "folder event writer busy"))?;
        let mut file = match File::open(self.event_path()) {
            Ok(file) => file,
            Err(error) if error.kind() == ErrorKind::NotFound => return Ok(None),
            Err(error) => return Err(error),
        };
        file.try_lock_shared().map_err(|error| io::Error::new(ErrorKind::WouldBlock, error))?;
        let file_len = file.metadata()?.len();
        let scan = scan_folder(&mut file, file_len, document, Some(schema), cancelled)?;
        let Some(DocumentRecord::Archive { start, length, schema_matches }) = scan.latest else { return Ok(None); };
        if !schema_matches {
            return Err(io::Error::new(ErrorKind::InvalidData, "stored archive schema does not match the mounted instance"));
        }
        let extent = usize::try_from(length).ok().filter(|length| (6..=maximum_bytes).contains(length))
            .ok_or_else(|| io::Error::new(ErrorKind::InvalidData, "stored archive exceeds its encoded source authority"))?;
        file.seek(SeekFrom::Start(start))?;
        let mut page = [0; FOLDER_ARCHIVE_PAGE_BYTES];
        let mut digest = semio_framework_hash::Sha256::new();
        read_pages(&mut file, length, &mut page, cancelled, |part| {
            digest.update(part);
            Ok(())
        })?;
        let seal = NativeFolderArchiveSeal { start, extent, sha256: digest.finalize() };
        file.unlock()?;
        Ok(Some(NativeFolderArchiveSource { file, seal, offered: 0 }))
    }

    /// 🪪 A superseding record, changed schema, or changed bytes invalidates the previous record seal.
    pub fn archive_source_is_current(
        &self, document: &str, schema: &str, seal: NativeFolderArchiveSeal, maximum_bytes: usize, cancelled: &impl Fn() -> bool,
    ) -> io::Result<bool> {
        Ok(self.open_archive_source(document, schema, maximum_bytes, cancelled)?
            .is_some_and(|source| source.seal() == seal))
    }
}
