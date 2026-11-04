//! 📥️ Items of `sync` that belong to `os-store-sync-native` (PLAN-OS-KERNEL-CRATES.md, Appendix C): lifted out of `🏪️store/🔄️sync/🦀️.rs`.
use semio_framework_os_store_text_io::ArtifactPackFiles;
use semio_framework_os_store_text_io::text_format::ArtifactTextFiles;
#[cfg(any(test, feature = "testing"))]
use semio_framework_os_store_sync::fixtures::*;
use semio_framework_os_store_sync::is_repository_root;
use semio_framework_os_store_sync::now_ms;
//#region 🔖️Fixtures

/// 📜️ Owned append-only folder event log. Documents are written as indivisible
/// `(schema, pack, spr)` snapshot events; blobs are content-addressed put/delete events. Reads fold
/// the log deterministically, so persistence follows the repo's event-sourced model without a CRUD
/// database. Every record is length-delimited and checksummed; an incomplete final record from a
/// process interruption is ignored, while corruption in a complete record is reported.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone)]
pub struct FolderEventLogStorage {
    folder: std::path::PathBuf,
    writer: std::sync::Arc<std::sync::Mutex<()>>,
}

#[cfg(not(target_arch = "wasm32"))]
pub const FOLDER_EVENT_MAGIC: &[u8; 8] = b"SEMIOEL1";

#[cfg(not(target_arch = "wasm32"))]
const MAX_FOLDER_EVENT_BYTES: u64 = 16 * 1024 * 1024 * 1024;

#[cfg(not(target_arch = "wasm32"))]
const DOCUMENT_PUT_EVENT: u8 = 1;

#[cfg(not(target_arch = "wasm32"))]
const BLOB_PUT_EVENT: u8 = 2;

#[cfg(not(target_arch = "wasm32"))]
const BLOB_DELETE_EVENT: u8 = 3;

#[cfg(not(target_arch = "wasm32"))]
const DOCUMENT_ARCHIVE_PUT_EVENT: u8 = 4;

#[cfg(not(target_arch = "wasm32"))]
struct FolderEvent {
    kind: u8,
    updated_at_ms: u64,
    key: String,
    metadata: String,
    primary: Vec<u8>,
    secondary: Vec<u8>,
}

#[cfg(not(target_arch = "wasm32"))]
struct FolderEventReader<'a> {
    bytes: &'a [u8],
    cursor: usize,
}

#[cfg(not(target_arch = "wasm32"))]
impl<'a> FolderEventReader<'a> {
    pub fn take(&mut self, len: usize) -> Result<&'a [u8], semio_framework_os_vcs::VcsError> {
        let end = self.cursor.checked_add(len).ok_or_else(|| semio_framework_os_vcs::VcsError::Backbone("folder event length overflow".into()))?;
        let value = self.bytes.get(self.cursor..end).ok_or_else(|| semio_framework_os_vcs::VcsError::Backbone("truncated folder event".into()))?;
        self.cursor = end;
        Ok(value)
    }

    fn u8(&mut self) -> Result<u8, semio_framework_os_vcs::VcsError> {
        Ok(self.take(1)?[0])
    }

    pub fn u32(&mut self) -> Result<u32, semio_framework_os_vcs::VcsError> {
        Ok(u32::from_le_bytes(self.take(4)?.try_into().unwrap()))
    }

    pub fn u64(&mut self) -> Result<u64, semio_framework_os_vcs::VcsError> {
        Ok(u64::from_le_bytes(self.take(8)?.try_into().unwrap()))
    }

    pub fn text(&mut self) -> Result<String, semio_framework_os_vcs::VcsError> {
        let len = self.u32()? as usize;
        String::from_utf8(self.take(len)?.to_vec()).map_err(|error| semio_framework_os_vcs::VcsError::Backbone(error.to_string()))
    }

    fn data(&mut self) -> Result<Vec<u8>, semio_framework_os_vcs::VcsError> {
        let len = usize::try_from(self.u64()?).map_err(|error| semio_framework_os_vcs::VcsError::Backbone(error.to_string()))?;
        Ok(self.take(len)?.to_vec())
    }
}

/// 🔒️ The one in-process writer lock of a folder, shared by every storage over it.
#[cfg(not(target_arch = "wasm32"))]
fn folder_writer(folder: &std::path::Path) -> std::sync::Arc<std::sync::Mutex<()>> {
    static WRITERS: std::sync::OnceLock<std::sync::Mutex<std::collections::HashMap<std::path::PathBuf, std::sync::Weak<std::sync::Mutex<()>>>>> = std::sync::OnceLock::new();
    let canonical_key = std::fs::canonicalize(folder).unwrap_or_else(|_| folder.to_path_buf());
    let mut writers = WRITERS.get_or_init(Default::default).lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    writers.get(&canonical_key).and_then(std::sync::Weak::upgrade).unwrap_or_else(|| {
        let writer = std::sync::Arc::new(std::sync::Mutex::new(()));
        writers.insert(canonical_key, std::sync::Arc::downgrade(&writer));
        writer
    })
}

/// 💾️ Replaces `target` atomically: the bytes go to a uniquely named staged file beside it, are flushed to disk and renamed over the target, then the directory entry is flushed; a crash leaves the old or the new file, never a torn one, and concurrent writers never share a staged name.
#[cfg(not(target_arch = "wasm32"))]
fn atomic_replace(target: &std::path::Path, bytes: &[u8]) -> Result<(), semio_framework_os_vcs::VcsError> {
    use std::io::Write;
    static STAGED: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let backbone = |error: std::io::Error| semio_framework_os_vcs::VcsError::Backbone(error.to_string());
    let name = target.file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or_default();
    let staged = target.with_file_name(format!(".{name}.{}.{}.stage", std::process::id(), STAGED.fetch_add(1, std::sync::atomic::Ordering::Relaxed)));
    let published = (|| {
        let mut file = std::fs::File::create(&staged)?;
        file.write_all(bytes)?;
        file.sync_all()?;
        drop(file);
        std::fs::rename(&staged, target)
    })();
    if let Err(error) = published {
        let _ = std::fs::remove_file(&staged);
        return Err(backbone(error));
    }
    match target.parent() {
        Some(parent) => FolderEventLogStorage::sync_directory(parent),
        None => Ok(()),
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl FolderEventLogStorage {
    pub fn new(folder: std::path::PathBuf) -> Self {
        let writer = folder_writer(&folder);
        Self { folder, writer }
    }

    pub fn event_path(&self) -> std::path::PathBuf {
        self.folder.join(".semio").join("events.semio")
    }

    fn checksum(bytes: &[u8]) -> u64 {
        bytes.iter().fold(0xcbf29ce484222325, |hash, byte| (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3))
    }

    fn encode_event(event: &FolderEvent) -> Result<Vec<u8>, semio_framework_os_vcs::VcsError> {
        let mut bytes = Vec::new();
        bytes.push(event.kind);
        bytes.extend_from_slice(&event.updated_at_ms.to_le_bytes());
        Self::encode_text(&mut bytes, &event.key)?;
        Self::encode_text(&mut bytes, &event.metadata)?;
        Self::encode_data(&mut bytes, &event.primary)?;
        Self::encode_data(&mut bytes, &event.secondary)?;
        Ok(bytes)
    }

    fn encode_text(output: &mut Vec<u8>, value: &str) -> Result<(), semio_framework_os_vcs::VcsError> {
        let len = u32::try_from(value.len()).map_err(|error| semio_framework_os_vcs::VcsError::Backbone(error.to_string()))?;
        output.extend_from_slice(&len.to_le_bytes());
        output.extend_from_slice(value.as_bytes());
        Ok(())
    }

    fn encode_data(output: &mut Vec<u8>, value: &[u8]) -> Result<(), semio_framework_os_vcs::VcsError> {
        let len = u64::try_from(value.len()).map_err(|error| semio_framework_os_vcs::VcsError::Backbone(error.to_string()))?;
        output.extend_from_slice(&len.to_le_bytes());
        output.extend_from_slice(value);
        Ok(())
    }

    fn decode_event(bytes: &[u8]) -> Result<FolderEvent, semio_framework_os_vcs::VcsError> {
        let mut reader = FolderEventReader { bytes, cursor: 0 };
        let event = FolderEvent { kind: reader.u8()?, updated_at_ms: reader.u64()?, key: reader.text()?, metadata: reader.text()?, primary: reader.data()?, secondary: reader.data()? };
        if !matches!(event.kind, DOCUMENT_PUT_EVENT | BLOB_PUT_EVENT | BLOB_DELETE_EVENT | DOCUMENT_ARCHIVE_PUT_EVENT) {
            return Err(semio_framework_os_vcs::VcsError::Backbone(format!("unknown folder event kind {}", event.kind)));
        }
        if reader.cursor != bytes.len() {
            return Err(semio_framework_os_vcs::VcsError::Backbone("folder event has trailing bytes".into()));
        }
        Ok(event)
    }

    /// 🚫 The repository root is not a document space. Paired build authority and a package workspace declaration identify this folder as the repository workspace, and writing `.semio/events.semio` there recreates a tracked event log.
    fn refuse_repository_root(&self) -> Result<(), semio_framework_os_vcs::VcsError> {
        if is_repository_root(&self.folder).map_err(semio_framework_os_vcs::VcsError::Backbone)? {
            return Err(semio_framework_os_vcs::VcsError::Backbone(format!("refusing to write {} at the repository root", self.event_path().display())));
        }
        Ok(())
    }

    pub fn append(&self, event: &FolderEvent) -> Result<(), semio_framework_os_vcs::VcsError> {
        use std::io::{Seek, Write};
        self.refuse_repository_root()?;
        let payload = Self::encode_event(event)?;
        if payload.len() as u64 > MAX_FOLDER_EVENT_BYTES {
            return Err(semio_framework_os_vcs::VcsError::Backbone("folder event exceeds the 16 GiB record boundary".into()));
        }
        let _guard = self.writer.lock().map_err(|error| semio_framework_os_vcs::VcsError::Backbone(error.to_string()))?;
        let semio_dir = self.folder.join(".semio");
        let created = !semio_dir.exists();
        std::fs::create_dir_all(&semio_dir).map_err(|error| semio_framework_os_vcs::VcsError::Backbone(error.to_string()))?;
        let mut file = std::fs::OpenOptions::new().create(true).read(true).write(true).open(self.event_path()).map_err(|error| semio_framework_os_vcs::VcsError::Backbone(error.to_string()))?;
        let (valid, torn) = Self::scan_tail(&mut file)?;
        if torn > 0 {
            file.set_len(valid).map_err(|error| semio_framework_os_vcs::VcsError::Backbone(error.to_string()))?;
        }
        file.seek(std::io::SeekFrom::Start(valid)).map_err(|error| semio_framework_os_vcs::VcsError::Backbone(error.to_string()))?;
        let mut record = Vec::with_capacity(payload.len() + 24);
        record.extend_from_slice(FOLDER_EVENT_MAGIC);
        record.extend_from_slice(&(payload.len() as u64).to_le_bytes());
        record.extend_from_slice(&Self::checksum(&payload).to_le_bytes());
        record.extend_from_slice(&payload);
        file.write_all(&record).map_err(|error| semio_framework_os_vcs::VcsError::Backbone(error.to_string()))?;
        file.sync_data().map_err(|error| semio_framework_os_vcs::VcsError::Backbone(error.to_string()))?;
        if created {
            Self::sync_directory(&semio_dir)?;
            Self::sync_directory(&self.folder)?;
        }
        Ok(())
    }

    #[cfg(unix)]
    pub(crate) fn sync_directory(path: &std::path::Path) -> Result<(), semio_framework_os_vcs::VcsError> {
        std::fs::File::open(path).and_then(|directory| directory.sync_all()).map_err(|error| semio_framework_os_vcs::VcsError::Backbone(error.to_string()))
    }

    #[cfg(not(unix))]
    fn sync_directory(_path: &std::path::Path) -> Result<(), semio_framework_os_vcs::VcsError> {
        Ok(())
    }

    /// 🩹️ Walks the record headers without reading payloads and returns `(valid_len, torn_bytes)`: the byte length of the complete records and the length of an incomplete or unverifiable final record. Only the last record's checksum is verified; corruption inside the log is an error.
    fn scan_tail(file: &mut std::fs::File) -> Result<(u64, u64), semio_framework_os_vcs::VcsError> {
        use std::io::{Read, Seek, SeekFrom};
        let backbone = |error: std::io::Error| semio_framework_os_vcs::VcsError::Backbone(error.to_string());
        let total = file.metadata().map_err(backbone)?.len();
        let mut cursor = 0u64;
        let mut last: Option<(u64, u64, u64)> = None;
        while cursor < total {
            let remaining = total - cursor;
            file.seek(SeekFrom::Start(cursor)).map_err(backbone)?;
            let mut header = [0u8; 24];
            let available = usize::try_from(remaining.min(24)).unwrap_or(24);
            file.read_exact(&mut header[..available]).map_err(backbone)?;
            if available >= 8 && &header[..8] != FOLDER_EVENT_MAGIC {
                return Err(semio_framework_os_vcs::VcsError::Backbone("invalid folder event magic".into()));
            }
            if available < 24 {
                return Ok((cursor, remaining));
            }
            let len = u64::from_le_bytes(header[8..16].try_into().unwrap());
            if len > MAX_FOLDER_EVENT_BYTES {
                return Err(semio_framework_os_vcs::VcsError::Backbone(format!("folder event length {len} exceeds the record boundary")));
            }
            if remaining - 24 < len {
                return Ok((cursor, remaining));
            }
            last = Some((cursor, len, u64::from_le_bytes(header[16..24].try_into().unwrap())));
            cursor += 24 + len;
        }
        if let Some((start, len, expected)) = last {
            file.seek(SeekFrom::Start(start + 24)).map_err(backbone)?;
            let mut payload = vec![0u8; usize::try_from(len).map_err(|error| semio_framework_os_vcs::VcsError::Backbone(error.to_string()))?];
            file.read_exact(&mut payload).map_err(backbone)?;
            if Self::checksum(&payload) != expected {
                return Ok((start, total - start));
            }
        }
        Ok((cursor, 0))
    }

    /// 🩹️ The byte length of an incomplete or unverifiable final record that the next append truncates; `0` for an intact log.
    pub fn torn_tail_bytes(&self) -> Result<u64, semio_framework_os_vcs::VcsError> {
        let _guard = self.writer.lock().map_err(|error| semio_framework_os_vcs::VcsError::Backbone(error.to_string()))?;
        match std::fs::File::open(self.event_path()) {
            Ok(mut file) => Self::scan_tail(&mut file).map(|(_, torn)| torn),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(0),
            Err(error) => Err(semio_framework_os_vcs::VcsError::Backbone(error.to_string())),
        }
    }

    pub fn events(&self) -> Result<Vec<FolderEvent>, semio_framework_os_vcs::VcsError> {
        use std::io::Read;
        let _guard = self.writer.lock().map_err(|error| semio_framework_os_vcs::VcsError::Backbone(error.to_string()))?;
        let mut file = match std::fs::File::open(self.event_path()) {
            Ok(file) => file,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(error) => return Err(semio_framework_os_vcs::VcsError::Backbone(error.to_string())),
        };
        let mut events = Vec::new();
        loop {
            let mut magic = [0; 8];
            match file.read_exact(&mut magic) {
                Ok(()) => {}
                Err(error) if error.kind() == std::io::ErrorKind::UnexpectedEof => break,
                Err(error) => return Err(semio_framework_os_vcs::VcsError::Backbone(error.to_string())),
            }
            if &magic != FOLDER_EVENT_MAGIC {
                return Err(semio_framework_os_vcs::VcsError::Backbone("invalid folder event magic".into()));
            }
            let mut header = [0; 16];
            match file.read_exact(&mut header) {
                Ok(()) => {}
                Err(error) if error.kind() == std::io::ErrorKind::UnexpectedEof => break,
                Err(error) => return Err(semio_framework_os_vcs::VcsError::Backbone(error.to_string())),
            }
            let len = u64::from_le_bytes(header[..8].try_into().unwrap());
            if len > MAX_FOLDER_EVENT_BYTES {
                return Err(semio_framework_os_vcs::VcsError::Backbone(format!("folder event length {len} exceeds the record boundary")));
            }
            let expected_checksum = u64::from_le_bytes(header[8..].try_into().unwrap());
            let mut payload = vec![0; usize::try_from(len).map_err(|error| semio_framework_os_vcs::VcsError::Backbone(error.to_string()))?];
            match file.read_exact(&mut payload) {
                Ok(()) => {}
                Err(error) if error.kind() == std::io::ErrorKind::UnexpectedEof => break,
                Err(error) => return Err(semio_framework_os_vcs::VcsError::Backbone(error.to_string())),
            }
            if Self::checksum(&payload) != expected_checksum {
                let mut probe = [0u8; 1];
                if file.read(&mut probe).map_err(|error| semio_framework_os_vcs::VcsError::Backbone(error.to_string()))? == 0 {
                    break;
                }
                return Err(semio_framework_os_vcs::VcsError::Backbone("folder event checksum mismatch".into()));
            }
            events.push(Self::decode_event(&payload)?);
        }
        Ok(events)
    }

    /// 📖️ Folds the latest stored `(pack, spr)` event for `document_id`.
    pub async fn read(&self, document_id: &str) -> Result<Option<(Vec<u8>, Vec<u8>)>, semio_framework_os_vcs::VcsError> {
        Ok(self.events()?.into_iter().rev().find(|event| event.kind == DOCUMENT_PUT_EVENT && event.key == document_id).map(|event| (event.primary, event.secondary)))
    }

    /// ✍️ Appends an indivisible document snapshot event.
    pub async fn write(&self, document_id: &str, schema: &str, pack: &[u8], spr: &[u8]) -> Result<(), semio_framework_os_vcs::VcsError> {
        self.append(&FolderEvent { kind: DOCUMENT_PUT_EVENT, updated_at_ms: now_ms().await, key: document_id.into(), metadata: schema.into(), primary: pack.into(), secondary: spr.into() })
    }

    /// 🗃️ Folds the latest exact recursive archive for `document_id`.
    pub async fn read_archive(&self, document_id: &str) -> Result<Option<Vec<u8>>, semio_framework_os_vcs::VcsError> {
        Ok(self.events()?.into_iter().rev().find(|event| event.kind == DOCUMENT_ARCHIVE_PUT_EVENT && event.key == document_id).map(|event| event.primary))
    }

    /// 🗃️ Appends one indivisible recursive-document archive event.
    pub async fn write_archive(&self, document_id: &str, schema: &str, archive: &[u8]) -> Result<(), semio_framework_os_vcs::VcsError> {
        semio_framework_os_spr::decode_document_archive_bytes(archive).await.map_err(|error| semio_framework_os_vcs::VcsError::Backbone(error.to_string()))?;
        self.append(&FolderEvent { kind: DOCUMENT_ARCHIVE_PUT_EVENT, updated_at_ms: now_ms().await, key: document_id.into(), metadata: schema.into(), primary: archive.into(), secondary: Vec::new() })
    }

    /// 📇️ Lists latest document events in newest-write-first order.
    pub async fn document_ids(&self) -> Result<Vec<String>, semio_framework_os_vcs::VcsError> {
        let mut latest = std::collections::HashMap::<String, u64>::new();
        for event in self.events()? {
            if matches!(event.kind, DOCUMENT_PUT_EVENT | DOCUMENT_ARCHIVE_PUT_EVENT) {
                latest.insert(event.key, event.updated_at_ms);
            }
        }
        let mut ids = latest.into_iter().collect::<Vec<_>>();
        ids.sort_by(|(left_id, left_time), (right_id, right_time)| right_time.cmp(left_time).then_with(|| left_id.cmp(right_id)));
        Ok(ids.into_iter().map(|(id, _)| id).collect())
    }
}

/// 🗃️ Textual persistence for one folder of documents: `<id>.<ext>` holds the DSL text (initial
/// snapshot), `<id>.<ext>.ops` holds the append-only op log (see `crate::os_store::text_format::print_document_text`/
/// `crate::os_store::text_format::parse_document_text`). No `Backbone` impl: like `FolderEventLogStorage` above, this actor
/// layer drives it from its own thread; this crate only owns the file format. Additive alongside the
/// sqlite storage today — a technology adopts it by implementing `ArtifactDsl`/`OpText` and having
/// its sync endpoint construct one of these instead; nothing currently reads or writes through it
/// automatically.
#[cfg(not(target_arch = "wasm32"))]
pub struct FolderTextStorage {
    folder: std::path::PathBuf,
    writer: std::sync::Arc<std::sync::Mutex<()>>,
}

#[cfg(not(target_arch = "wasm32"))]
impl FolderTextStorage {
    pub async fn new(folder: std::path::PathBuf) -> Self {
        let writer = folder_writer(&folder);
        Self { folder, writer }
    }

    fn lock(&self) -> Result<std::sync::MutexGuard<'_, ()>, semio_framework_os_vcs::VcsError> {
        self.writer.lock().map_err(|error| semio_framework_os_vcs::VcsError::Backbone(error.to_string()))
    }

    async fn dsl_path(&self, document_id: &str, envelope_id: &str) -> std::path::PathBuf {
        self.folder.join(semio_framework_os_semio::semio_filename(document_id, envelope_id, semio_framework_os_semio::Component::Dsl))
    }

    async fn ops_path(&self, document_id: &str, envelope_id: &str) -> std::path::PathBuf {
        self.folder.join(semio_framework_os_semio::semio_filename(document_id, envelope_id, semio_framework_os_semio::Component::Op))
    }

    /// 🏷️ Path of the authoritative binary pack file.
    pub async fn pack_path(&self, document_id: &str, envelope_id: &str) -> std::path::PathBuf {
        self.folder.join(semio_framework_os_semio::semio_filename(document_id, envelope_id, semio_framework_os_semio::Component::Pack))
    }

    /// 🏷️ Path of the authoritative binary op-log file.
    pub async fn spr_path(&self, document_id: &str, envelope_id: &str) -> std::path::PathBuf {
        self.folder.join(semio_framework_os_semio::semio_filename(document_id, envelope_id, semio_framework_os_semio::Component::Spr))
    }

    /// 🗃️ Path of the authoritative recursive-document archive.
    pub async fn archive_path(&self, document_id: &str, envelope_id: &str) -> std::path::PathBuf {
        self.folder.join(format!("{document_id}.{envelope_id}.archive.semio"))
    }

    /// 🗃️ Reads one exact recursive-document archive, or `None` when none was persisted.
    pub async fn read_archive(&self, document_id: &str, envelope_id: &str) -> Result<Option<Vec<u8>>, semio_framework_os_vcs::VcsError> {
        let bytes = match std::fs::read(self.archive_path(document_id, envelope_id).await) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(error) => return Err(semio_framework_os_vcs::VcsError::Backbone(error.to_string())),
        };
        semio_framework_os_spr::decode_document_archive_bytes(&bytes).await.map_err(|error| semio_framework_os_vcs::VcsError::Backbone(error.to_string()))?;
        Ok(Some(bytes))
    }

    /// 🗃️ Publishes derived root mirrors before atomically replacing archive authority.
    pub async fn write_archive(&self, document_id: &str, envelope_id: &str, archive: &[u8], files: &ArtifactPackFiles, dsl_mirror: &str) -> Result<(), semio_framework_os_vcs::VcsError> {
        semio_framework_os_spr::decode_document_archive_bytes(archive).await.map_err(|error| semio_framework_os_vcs::VcsError::Backbone(error.to_string()))?;
        let paths = self.pack_targets(document_id, envelope_id).await;
        let archive_path = self.archive_path(document_id, envelope_id).await;
        let _guard = self.lock()?;
        std::fs::create_dir_all(&self.folder).map_err(|error| semio_framework_os_vcs::VcsError::Backbone(error.to_string()))?;
        Self::write_pack_files(&paths, files, dsl_mirror)?;
        atomic_replace(&archive_path, archive)
    }

    /// 📖️ Reads both files for `document_id`, or `None` if the DSL file does not exist yet.
    pub async fn read(&self, document_id: &str, envelope_id: &str) -> Result<Option<ArtifactTextFiles>, semio_framework_os_vcs::VcsError> {
        let dsl = match std::fs::read_to_string(self.dsl_path(document_id, envelope_id).await) {
            Ok(text) => text,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(err) => return Err(semio_framework_os_vcs::VcsError::Backbone(err.to_string())),
        };
        let ops = match std::fs::read_to_string(self.ops_path(document_id, envelope_id).await) {
            Ok(text) => text,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => String::new(),
            Err(err) => return Err(semio_framework_os_vcs::VcsError::Backbone(err.to_string())),
        };
        Ok(Some(ArtifactTextFiles { dsl, ops }))
    }

    /// ✍️ Overwrites both files wholesale — the structural-command cold path (undo/redo/
    /// checkpoint/alternative).
    pub async fn write(&self, document_id: &str, envelope_id: &str, files: &ArtifactTextFiles) -> Result<(), semio_framework_os_vcs::VcsError> {
        let (dsl, ops) = (self.dsl_path(document_id, envelope_id).await, self.ops_path(document_id, envelope_id).await);
        let _guard = self.lock()?;
        std::fs::create_dir_all(&self.folder).map_err(|e| semio_framework_os_vcs::VcsError::Backbone(e.to_string()))?;
        atomic_replace(&dsl, files.dsl.as_bytes())?;
        atomic_replace(&ops, files.ops.as_bytes())
    }

    /// 📖️ pack+spr-first read: reads the AUTHORITATIVE pair for `document_id`, or `None` if
    /// the `.pack` file itself doesn't exist (unlike `read`, the DSL mirror's existence alone
    /// doesn't count — pack+spr are authoritative per the disk-layout LAW, the DSL file is
    /// import-only). A present `.pack` with a missing `.spr` is a hard error — no legacy: they are
    /// always written together (see `write_pack`), so a missing `.spr` means corruption or a
    /// manual edit, never a valid state to silently recover from.
    pub async fn read_pack(&self, document_id: &str, envelope_id: &str) -> Result<Option<ArtifactPackFiles>, semio_framework_os_vcs::VcsError> {
        let pack = match std::fs::read(self.pack_path(document_id, envelope_id).await) {
            Ok(bytes) => bytes,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(err) => return Err(semio_framework_os_vcs::VcsError::Backbone(err.to_string())),
        };
        let spr = std::fs::read(self.spr_path(document_id, envelope_id).await).map_err(|err| {
            if err.kind() == std::io::ErrorKind::NotFound {
                semio_framework_os_vcs::VcsError::Backbone(format!("{document_id} pack.semio exists but spr.semio is missing for envelope {envelope_id}"))
            } else {
                semio_framework_os_vcs::VcsError::Backbone(err.to_string())
            }
        })?;
        let ops = match std::fs::read_to_string(self.ops_path(document_id, envelope_id).await) {
            Ok(text) => text,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => String::new(),
            Err(err) => return Err(semio_framework_os_vcs::VcsError::Backbone(err.to_string())),
        };
        Ok(Some(ArtifactPackFiles { pack, spr, ops }))
    }

    /// ✍️ Overwrites all four files: the AUTHORITATIVE `.pack` + `.spr` pair, the shared
    /// `.ops` text mirror, and the always-written DSL mirror `dsl_mirror` (`print_dsl` on the
    /// initial snapshot) — the pack-aware sibling of `write`.
    pub async fn write_pack(&self, document_id: &str, envelope_id: &str, files: &ArtifactPackFiles, dsl_mirror: &str) -> Result<(), semio_framework_os_vcs::VcsError> {
        let paths = self.pack_targets(document_id, envelope_id).await;
        let _guard = self.lock()?;
        std::fs::create_dir_all(&self.folder).map_err(|e| semio_framework_os_vcs::VcsError::Backbone(e.to_string()))?;
        Self::write_pack_files(&paths, files, dsl_mirror)
    }

    async fn pack_targets(&self, document_id: &str, envelope_id: &str) -> [std::path::PathBuf; 4] {
        [self.pack_path(document_id, envelope_id).await, self.spr_path(document_id, envelope_id).await, self.ops_path(document_id, envelope_id).await, self.dsl_path(document_id, envelope_id).await]
    }

    fn write_pack_files(paths: &[std::path::PathBuf; 4], files: &ArtifactPackFiles, dsl_mirror: &str) -> Result<(), semio_framework_os_vcs::VcsError> {
        atomic_replace(&paths[0], &files.pack)?;
        atomic_replace(&paths[1], &files.spr)?;
        atomic_replace(&paths[2], files.ops.as_bytes())?;
        atomic_replace(&paths[3], dsl_mirror.as_bytes())
    }

    /// ➕️ Appends already-printed op-log lines (one complete {@link print_edit_lines} unit:
    /// edit + inverse + metadata) to the `.ops` file without rewriting it — O(new edit).
    pub async fn append_ops(&self, document_id: &str, envelope_id: &str, lines: &str) -> Result<(), semio_framework_os_vcs::VcsError> {
        use std::io::{Read, Seek, SeekFrom, Write};
        let backbone = |error: std::io::Error| semio_framework_os_vcs::VcsError::Backbone(error.to_string());
        let ops = self.ops_path(document_id, envelope_id).await;
        let _guard = self.lock()?;
        std::fs::create_dir_all(&self.folder).map_err(backbone)?;
        let mut file = std::fs::OpenOptions::new().create(true).read(true).write(true).open(ops).map_err(backbone)?;
        let mut existing = Vec::new();
        file.read_to_end(&mut existing).map_err(backbone)?;
        let complete = existing.iter().rposition(|byte| *byte == b'\n').map_or(0, |index| index + 1);
        if complete < existing.len() {
            file.set_len(complete as u64).map_err(backbone)?;
        }
        file.seek(SeekFrom::Start(complete as u64)).map_err(backbone)?;
        file.write_all(lines.as_bytes()).map_err(backbone)?;
        file.sync_data().map_err(backbone)
    }

    /// 📇️ Lists every stored document id (by DSL `.semio` file stem) for a given envelope id.
    pub async fn document_ids(&self, envelope_id: &str) -> Result<Vec<String>, semio_framework_os_vcs::VcsError> {
        let suffix = format!(".{envelope_id}.dsl.semio");
        let entries = match std::fs::read_dir(&self.folder) {
            Ok(entries) => entries,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(err) => return Err(semio_framework_os_vcs::VcsError::Backbone(err.to_string())),
        };
        let mut ids = Vec::new();
        for entry in entries {
            let entry = entry.map_err(|e| semio_framework_os_vcs::VcsError::Backbone(e.to_string()))?;
            if let Some(name) = entry.file_name().to_str() {
                if let Some(id) = name.strip_suffix(&suffix) {
                    ids.push(id.to_string());
                }
            }
        }
        Ok(ids)
    }
}



/// 🗄️ Content-addressed blob events in [`FolderEventLogStorage`].
#[cfg(not(target_arch = "wasm32"))]
impl semio_framework_os_store_space_member::lifted_store::BlobStore for FolderEventLogStorage {
    async fn put(&self, bytes: &[u8], media_type: &str) -> Result<semio_framework_os_store_blob_identity::BlobRef, semio_framework_os_vcs::VcsError> {
        let hash = semio_framework_hash::hash_bytes(bytes);
        self.append(&FolderEvent { kind: BLOB_PUT_EVENT, updated_at_ms: now_ms().await, key: hash.clone(), metadata: media_type.into(), primary: bytes.into(), secondary: Vec::new() })?;
        Ok(semio_framework_os_store_blob_identity::BlobRef { hash, size: bytes.len() as u64, media_type: media_type.into() })
    }

    async fn get(&self, hash: &str) -> Result<Option<Vec<u8>>, semio_framework_os_vcs::VcsError> {
        for event in self.events()?.into_iter().rev() {
            if event.key == hash {
                return match event.kind {
                    BLOB_PUT_EVENT => Ok(Some(event.primary)),
                    BLOB_DELETE_EVENT => Ok(None),
                    _ => continue,
                };
            }
        }
        Ok(None)
    }

    async fn has(&self, hash: &str) -> Result<bool, semio_framework_os_vcs::VcsError> {
        Ok(self.get(hash).await?.is_some())
    }

    async fn delete(&self, hash: &str) -> Result<(), semio_framework_os_vcs::VcsError> {
        self.append(&FolderEvent { kind: BLOB_DELETE_EVENT, updated_at_ms: now_ms().await, key: hash.into(), metadata: String::new(), primary: Vec::new(), secondary: Vec::new() })
    }
}
//#endregion 🔖️Fixtures
