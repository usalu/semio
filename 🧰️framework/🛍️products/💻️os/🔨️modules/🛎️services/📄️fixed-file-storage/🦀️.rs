//! 📄️ Fixed-size file pages and retained fixed-file documents with exact owner custody and atomic publication.

use std::collections::{BTreeMap, VecDeque};
use std::sync::atomic::{AtomicU64, Ordering};

//#region 💾️Storage
//#region 📄️FixedFilePage
/// 📄️ Maximum logical payload admitted by one host-storage worker turn.
pub const STORAGE_FIXED_FILE_PAGE_BYTES: usize = 16 * 1024;

/// 📖️ Reads exactly one bounded file page from a caller-owned path. This synchronous
/// primitive is deliberately owned by the host-storage service and must be invoked only by an
/// already-admitted [`Lane::Io`] worker job; presentation code may retain paths and results but
/// never performs the platform operation itself.
#[cfg(not(target_arch = "wasm32"))]
pub fn storage_worker_read_fixed_file_page(path: &std::path::Path, maximum_bytes: usize) -> std::io::Result<Vec<u8>> {
    use std::io::Read;
    if maximum_bytes > STORAGE_FIXED_FILE_PAGE_BYTES {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "fixed file page limit exceeds host-storage authority"));
    }
    let mut file = std::fs::File::open(path)?;
    let logical_bytes = usize::try_from(file.metadata()?.len()).map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "fixed file page length is not representable"))?;
    if logical_bytes > maximum_bytes {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "fixed file page exceeds admitted bytes"));
    }
    let mut page = [0u8; STORAGE_FIXED_FILE_PAGE_BYTES];
    file.read_exact(&mut page[..logical_bytes])?;
    let mut trailing = [0u8; 1];
    if file.read(&mut trailing)? != 0 {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "fixed file page grew beyond admitted bytes"));
    }
    Ok(page[..logical_bytes].to_vec())
}

/// 📝️ Writes exactly one bounded file page from an already-admitted [`Lane::Io`] worker
/// job. Byte admission precedes directory creation, file opening, truncation, and publication.
#[cfg(not(target_arch = "wasm32"))]
pub fn storage_worker_write_fixed_file_page(path: &std::path::Path, bytes: &[u8], maximum_bytes: usize) -> std::io::Result<()> {
    use std::io::Write;
    if maximum_bytes > STORAGE_FIXED_FILE_PAGE_BYTES || bytes.len() > maximum_bytes {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "fixed file page exceeds admitted bytes"));
    }
    let parent = path.parent().ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "fixed file page has no parent"))?;
    std::fs::create_dir_all(parent)?;
    let mut file = std::fs::OpenOptions::new().create(true).truncate(true).write(true).open(path)?;
    file.write_all(bytes)
}
//#endregion 📄️FixedFilePage

//#region 📚️RetainedFixedFileDocument
/// 📚️ Maximum logical single-file document retained by the host storage service.
pub const STORAGE_FIXED_FILE_DOCUMENT_MAX_BYTES: usize = 64 * 1024;
/// 📚️ Maximum number of bounded file-page turns in one logical document write.
pub const STORAGE_FIXED_FILE_DOCUMENT_MAX_WRITE_STEPS: usize = STORAGE_FIXED_FILE_DOCUMENT_MAX_BYTES / STORAGE_FIXED_FILE_PAGE_BYTES;

/// 🪪️ Exact owner and generation of one inactive fixed-file document.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FixedFileDocumentWriteToken {
    pub(super) owner: u64,
    pub(super) generation: u64,
}

impl FixedFileDocumentWriteToken {
    pub const fn new(owner: u64, generation: u64) -> Self {
        Self { owner, generation }
    }
}

/// 📄️ Result of one bounded retained-document write turn.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FixedFileDocumentWriteStep {
    pub written_bytes: usize,
    pub ready_to_publish: bool,
}

#[cfg(not(target_arch = "wasm32"))]
struct RetainedFixedFileDocumentWrite {
    pub(super) destination: std::path::PathBuf,
    pub(super) temporary: std::path::PathBuf,
    file: Option<std::fs::File>,
    pub(super) pages: VecDeque<Vec<u8>>,
    expected_bytes: usize,
    pub(super) written_bytes: usize,
    pub(super) cancelled: bool,
}

/// 📚️ One bounded queue of inactive single-file documents. Only the exact current token may write,
/// publish, or cancel. A destination retains its terminal generation floor so delayed owners cannot
/// replay after publication or cancellation; publication replaces the committed raw file only after
/// exact-length validation.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Default)]
pub struct RetainedFixedFileDocuments {
    pub(super) owners: BTreeMap<std::path::PathBuf, FixedFileDocumentWriteToken>,
    generations: BTreeMap<std::path::PathBuf, u64>,
    writes: BTreeMap<FixedFileDocumentWriteToken, RetainedFixedFileDocumentWrite>,
}

#[cfg(not(target_arch = "wasm32"))]
static FIXED_FILE_DOCUMENT_TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(1);

#[cfg(not(target_arch = "wasm32"))]
fn fixed_file_document_error(kind: std::io::ErrorKind, message: &'static str) -> std::io::Error {
    std::io::Error::new(kind, message)
}

/// 📖️ Reads one committed logical document in at most four service-sized chunks.
#[cfg(not(target_arch = "wasm32"))]
pub fn storage_worker_read_fixed_file_document(path: &std::path::Path, maximum_bytes: usize) -> std::io::Result<Vec<u8>> {
    use std::io::Read;
    if maximum_bytes > STORAGE_FIXED_FILE_DOCUMENT_MAX_BYTES {
        return Err(fixed_file_document_error(std::io::ErrorKind::InvalidInput, "fixed file document limit exceeds host-storage authority"));
    }
    let mut file = std::fs::File::open(path)?;
    let logical_bytes = usize::try_from(file.metadata()?.len()).map_err(|_| fixed_file_document_error(std::io::ErrorKind::InvalidData, "fixed file document length is not representable"))?;
    if logical_bytes > maximum_bytes {
        return Err(fixed_file_document_error(std::io::ErrorKind::InvalidData, "fixed file document exceeds admitted bytes"));
    }
    let mut document = vec![0u8; logical_bytes];
    for page in document.chunks_mut(STORAGE_FIXED_FILE_PAGE_BYTES) {
        file.read_exact(page)?;
    }
    let mut trailing = [0u8; 1];
    if file.read(&mut trailing)? != 0 {
        return Err(fixed_file_document_error(std::io::ErrorKind::InvalidData, "fixed file document grew beyond admitted bytes"));
    }
    Ok(document)
}

#[cfg(not(target_arch = "wasm32"))]
impl RetainedFixedFileDocuments {
    /// 📝️ Admits one complete logical value before any file is opened. A strictly newer generation for the
    /// same destination supersedes and retires the prior inactive file while preserving the commit.
    pub fn begin_write(&mut self, destination: &std::path::Path, owner: u64, generation: u64, bytes: Vec<u8>, maximum_bytes: usize) -> std::io::Result<FixedFileDocumentWriteToken> {
        if maximum_bytes > STORAGE_FIXED_FILE_DOCUMENT_MAX_BYTES || bytes.len() > maximum_bytes {
            return Err(fixed_file_document_error(std::io::ErrorKind::InvalidInput, "fixed file document exceeds retained service authority"));
        }
        let page_count = bytes.len().max(1).div_ceil(STORAGE_FIXED_FILE_PAGE_BYTES);
        if page_count > STORAGE_FIXED_FILE_DOCUMENT_MAX_WRITE_STEPS {
            return Err(fixed_file_document_error(std::io::ErrorKind::InvalidInput, "fixed file document exceeds retained page authority"));
        }
        let token = FixedFileDocumentWriteToken::new(owner, generation);
        if self.writes.contains_key(&token) {
            return Err(fixed_file_document_error(std::io::ErrorKind::AlreadyExists, "fixed file document token is already active"));
        }
        let destination = destination.to_path_buf();
        if self.generations.get(&destination).is_some_and(|current| generation <= *current) {
            return Err(fixed_file_document_error(std::io::ErrorKind::InvalidInput, "fixed file document generation is stale"));
        }
        if let Some(stale) = self.owners.get(&destination).copied() {
            self.retire_write(stale)?;
        }
        let parent = destination.parent().ok_or_else(|| fixed_file_document_error(std::io::ErrorKind::InvalidInput, "fixed file document has no parent"))?;
        std::fs::create_dir_all(parent)?;
        let name = destination.file_name().ok_or_else(|| fixed_file_document_error(std::io::ErrorKind::InvalidInput, "fixed file document has no file name"))?;
        let sequence = FIXED_FILE_DOCUMENT_TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let temporary = destination.with_file_name(format!("{}.pending-{owner}-{generation}-{sequence}", name.to_string_lossy()));
        let file = std::fs::OpenOptions::new().create_new(true).write(true).open(&temporary)?;
        let expected_bytes = bytes.len();
        let pages = if bytes.is_empty() { VecDeque::from([Vec::new()]) } else { bytes.chunks(STORAGE_FIXED_FILE_PAGE_BYTES).map(|page| page.to_vec()).collect() };
        self.generations.insert(destination.clone(), generation);
        self.owners.insert(destination.clone(), token);
        self.writes.insert(token, RetainedFixedFileDocumentWrite { destination, temporary, file: Some(file), pages, expected_bytes, written_bytes: 0, cancelled: false });
        Ok(token)
    }

    /// 📝️ Writes at most one retained 16 KiB page for the exact current generation.
    pub fn write_step(&mut self, token: FixedFileDocumentWriteToken) -> std::io::Result<FixedFileDocumentWriteStep> {
        let destination = self.writes.get(&token).map(|write| write.destination.clone()).ok_or_else(|| fixed_file_document_error(std::io::ErrorKind::NotFound, "fixed file document token is not active"))?;
        if self.owners.get(&destination) != Some(&token) {
            return Err(fixed_file_document_error(std::io::ErrorKind::PermissionDenied, "fixed file document token no longer owns its destination"));
        }
        let write = self.writes.get_mut(&token).expect("active fixed file document token");
        if write.cancelled {
            return Err(fixed_file_document_error(std::io::ErrorKind::Interrupted, "fixed file document token is cancelled"));
        }
        let page = write.pages.front().ok_or_else(|| fixed_file_document_error(std::io::ErrorKind::InvalidInput, "fixed file document is already ready to publish"))?;
        use std::io::Write;
        write.file.as_mut().ok_or_else(|| fixed_file_document_error(std::io::ErrorKind::BrokenPipe, "fixed file document writer is closed"))?.write_all(page)?;
        let written_bytes = page.len();
        write.pages.pop_front();
        write.written_bytes += written_bytes;
        Ok(FixedFileDocumentWriteStep { written_bytes, ready_to_publish: write.pages.is_empty() })
    }

    /// ✅️ Flushes, validates, and atomically replaces the committed destination for the exact owner.
    pub fn publish(&mut self, token: FixedFileDocumentWriteToken) -> std::io::Result<()> {
        let destination = self.writes.get(&token).map(|write| write.destination.clone()).ok_or_else(|| fixed_file_document_error(std::io::ErrorKind::NotFound, "fixed file document token is not active"))?;
        if self.owners.get(&destination) != Some(&token) {
            return Err(fixed_file_document_error(std::io::ErrorKind::PermissionDenied, "fixed file document token no longer owns its destination"));
        }
        let temporary = {
            let write = self.writes.get_mut(&token).expect("active fixed file document token");
            if write.cancelled || !write.pages.is_empty() || write.written_bytes != write.expected_bytes {
                return Err(fixed_file_document_error(std::io::ErrorKind::InvalidInput, "fixed file document is incomplete"));
            }
            use std::io::Write;
            let mut file = write.file.take().ok_or_else(|| fixed_file_document_error(std::io::ErrorKind::BrokenPipe, "fixed file document writer is closed"))?;
            file.flush()?;
            file.sync_all()?;
            if file.metadata()?.len() != write.expected_bytes as u64 {
                write.file = Some(file);
                return Err(fixed_file_document_error(std::io::ErrorKind::InvalidData, "inactive fixed file document length changed before publication"));
            }
            drop(file);
            write.temporary.clone()
        };
        std::fs::rename(&temporary, &destination)?;
        self.writes.remove(&token);
        if self.owners.get(&destination) == Some(&token) {
            self.owners.remove(&destination);
        }
        Ok(())
    }

    /// 🛑️ Marks the exact inactive generation non-publishable without touching the committed file.
    pub fn cancel(&mut self, token: FixedFileDocumentWriteToken) -> std::io::Result<()> {
        let write = self.writes.get_mut(&token).ok_or_else(|| fixed_file_document_error(std::io::ErrorKind::NotFound, "fixed file document token is not active"))?;
        write.cancelled = true;
        write.file.take();
        if self.owners.get(&write.destination) == Some(&token) {
            self.owners.remove(&write.destination);
        }
        Ok(())
    }

    /// 🧹 Removes one cancelled inactive file and retires its exact generation.
    pub fn cancel_step(&mut self, token: FixedFileDocumentWriteToken) -> std::io::Result<bool> {
        let write = self.writes.get(&token).ok_or_else(|| fixed_file_document_error(std::io::ErrorKind::NotFound, "fixed file document token is not active"))?;
        if !write.cancelled {
            return Err(fixed_file_document_error(std::io::ErrorKind::InvalidInput, "fixed file document token is not cancelled"));
        }
        let temporary = write.temporary.clone();
        match std::fs::remove_file(&temporary) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error),
        }
        self.writes.remove(&token);
        Ok(true)
    }

    /// 📖️ Reads only the committed destination, so an inactive write cannot expose mixed bytes.
    pub fn read(&self, path: &std::path::Path, maximum_bytes: usize) -> std::io::Result<Vec<u8>> {
        storage_worker_read_fixed_file_document(path, maximum_bytes)
    }

    fn retire_write(&mut self, token: FixedFileDocumentWriteToken) -> std::io::Result<()> {
        let Some(mut write) = self.writes.remove(&token) else { return Ok(()) };
        write.file.take();
        match std::fs::remove_file(&write.temporary) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => {
                self.writes.insert(token, write);
                return Err(error);
            }
        }
        if self.owners.get(&write.destination) == Some(&token) {
            self.owners.remove(&write.destination);
        }
        Ok(())
    }
}
//#endregion 📚️RetainedFixedFileDocument
//#endregion 💾️Storage
