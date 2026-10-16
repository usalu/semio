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

#[cfg(not(target_arch = "wasm32"))]
impl FolderEventLogStorage {
    pub fn new(folder: std::path::PathBuf) -> Self {
        static WRITERS: std::sync::OnceLock<std::sync::Mutex<std::collections::HashMap<std::path::PathBuf, std::sync::Weak<std::sync::Mutex<()>>>>> = std::sync::OnceLock::new();
        let canonical_key = std::fs::canonicalize(&folder).unwrap_or_else(|_| folder.clone());
        let mut writers = WRITERS.get_or_init(Default::default).lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        let writer = writers.get(&canonical_key).and_then(std::sync::Weak::upgrade).unwrap_or_else(|| {
            let writer = std::sync::Arc::new(std::sync::Mutex::new(()));
            writers.insert(canonical_key, std::sync::Arc::downgrade(&writer));
            writer
        });
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
        use std::io::Write;
        self.refuse_repository_root()?;
        let payload = Self::encode_event(event)?;
        if payload.len() as u64 > MAX_FOLDER_EVENT_BYTES {
            return Err(semio_framework_os_vcs::VcsError::Backbone("folder event exceeds the 16 GiB record boundary".into()));
        }
        let _guard = self.writer.lock().map_err(|error| semio_framework_os_vcs::VcsError::Backbone(error.to_string()))?;
        let semio_dir = self.folder.join(".semio");
        std::fs::create_dir_all(&semio_dir).map_err(|error| semio_framework_os_vcs::VcsError::Backbone(error.to_string()))?;
        let mut file = std::fs::OpenOptions::new().create(true).append(true).open(self.event_path()).map_err(|error| semio_framework_os_vcs::VcsError::Backbone(error.to_string()))?;
        file.write_all(FOLDER_EVENT_MAGIC).map_err(|error| semio_framework_os_vcs::VcsError::Backbone(error.to_string()))?;
        file.write_all(&(payload.len() as u64).to_le_bytes()).map_err(|error| semio_framework_os_vcs::VcsError::Backbone(error.to_string()))?;
        file.write_all(&Self::checksum(&payload).to_le_bytes()).map_err(|error| semio_framework_os_vcs::VcsError::Backbone(error.to_string()))?;
        file.write_all(&payload).map_err(|error| semio_framework_os_vcs::VcsError::Backbone(error.to_string()))?;
        file.sync_data().map_err(|error| semio_framework_os_vcs::VcsError::Backbone(error.to_string()))
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
}

#[cfg(not(target_arch = "wasm32"))]
impl FolderTextStorage {
    pub async fn new(folder: std::path::PathBuf) -> Self {
        Self { folder }
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
        std::fs::create_dir_all(&self.folder).map_err(|error| semio_framework_os_vcs::VcsError::Backbone(error.to_string()))?;
        let target = self.archive_path(document_id, envelope_id).await;
        let staged = target.with_extension("semio.stage");
        std::fs::write(&staged, archive).map_err(|error| semio_framework_os_vcs::VcsError::Backbone(error.to_string()))?;
        if let Err(error) = self.write_pack(document_id, envelope_id, files, dsl_mirror).await {
            let _ = std::fs::remove_file(&staged);
            return Err(error);
        }
        if let Err(error) = std::fs::rename(&staged, &target) {
            let _ = std::fs::remove_file(&staged);
            return Err(semio_framework_os_vcs::VcsError::Backbone(error.to_string()));
        }
        Ok(())
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
        std::fs::create_dir_all(&self.folder).map_err(|e| semio_framework_os_vcs::VcsError::Backbone(e.to_string()))?;
        std::fs::write(self.dsl_path(document_id, envelope_id).await, &files.dsl).map_err(|e| semio_framework_os_vcs::VcsError::Backbone(e.to_string()))?;
        std::fs::write(self.ops_path(document_id, envelope_id).await, &files.ops).map_err(|e| semio_framework_os_vcs::VcsError::Backbone(e.to_string()))
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
        std::fs::create_dir_all(&self.folder).map_err(|e| semio_framework_os_vcs::VcsError::Backbone(e.to_string()))?;
        std::fs::write(self.pack_path(document_id, envelope_id).await, &files.pack).map_err(|e| semio_framework_os_vcs::VcsError::Backbone(e.to_string()))?;
        std::fs::write(self.spr_path(document_id, envelope_id).await, &files.spr).map_err(|e| semio_framework_os_vcs::VcsError::Backbone(e.to_string()))?;
        std::fs::write(self.ops_path(document_id, envelope_id).await, &files.ops).map_err(|e| semio_framework_os_vcs::VcsError::Backbone(e.to_string()))?;
        std::fs::write(self.dsl_path(document_id, envelope_id).await, dsl_mirror).map_err(|e| semio_framework_os_vcs::VcsError::Backbone(e.to_string()))
    }

    /// ➕️ Appends already-printed op-log lines (one complete {@link print_edit_lines} unit:
    /// edit + inverse + metadata) to the `.ops` file without rewriting it — O(new edit).
    pub async fn append_ops(&self, document_id: &str, envelope_id: &str, lines: &str) -> Result<(), semio_framework_os_vcs::VcsError> {
        use std::io::Write;
        std::fs::create_dir_all(&self.folder).map_err(|e| semio_framework_os_vcs::VcsError::Backbone(e.to_string()))?;
        let mut file = std::fs::OpenOptions::new().create(true).append(true).open(self.ops_path(document_id, envelope_id).await).map_err(|e| semio_framework_os_vcs::VcsError::Backbone(e.to_string()))?;
        file.write_all(lines.as_bytes()).map_err(|e| semio_framework_os_vcs::VcsError::Backbone(e.to_string()))
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
