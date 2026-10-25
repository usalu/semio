//! 💾️ The append-only event store.

use super::{digest, Sha256};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

//#region 🗄️Store

pub const SCHEMA: &str = "semio.event/1";
pub(super) const STAGE_SCHEMA: &str = "semio.event.stage/1";
pub const MAX_EVENT_SIZE: usize = 1 << 20;
pub const MAX_BATCH_SIZE: usize = 64 << 20;
pub const MAX_BATCH_EVENTS: usize = 100_000;

/// ⚠️ Every way an append or a replay can refuse.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StoreError {
    Duplicate(String),
    Corrupt(String),
    TooLarge(String),
    Cancelled,
    Invalid(String),
    Io(String),
}

impl fmt::Display for StoreError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Duplicate(detail) => write!(formatter, "duplicate event: {detail}"),
            Self::Corrupt(detail) => write!(formatter, "corrupt event log: {detail}"),
            Self::TooLarge(detail) => write!(formatter, "event exceeds maximum size: {detail}"),
            Self::Cancelled => write!(formatter, "cancelled"),
            Self::Invalid(detail) => write!(formatter, "{detail}"),
            Self::Io(detail) => write!(formatter, "{detail}"),
        }
    }
}

impl std::error::Error for StoreError {}

impl From<std::io::Error> for StoreError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error.to_string())
    }
}

/// 🎒️ A JSON document carried as the exact text its owner encoded, the twin of Go's
/// `json.RawMessage`.
///
/// A record's member order is part of its wire shape: a domain struct marshals in declaration order
/// while a generic document marshals sorted, and `serde_json::Value` cannot tell the two apart
/// because it sorts everything. Carrying the encoded text is what lets a payload keep the order the
/// owning module wrote it in, so a log written by one implementation checksums identically in the
/// other.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Payload(pub(super) Box<serde_json::value::RawValue>);

impl Payload {
    /// 🔃️ Encodes one value into its payload, in the member order its own `Serialize` writes.
    pub fn of<T: Serialize + ?Sized>(value: &T) -> Result<Payload, StoreError> {
        serde_json::value::to_raw_value(value).map(Payload).map_err(|error| StoreError::Invalid(error.to_string()))
    }

    /// 📄️ The encoded text, byte for byte as it reaches the log.
    pub fn text(&self) -> &str {
        self.0.get()
    }
}

impl PartialEq for Payload {
    fn eq(&self, other: &Self) -> bool {
        self.text() == other.text()
    }
}

impl Eq for Payload {}

/// 🧾️ One committed record of the append-only log.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StoreEvent {
    pub schema: String,
    pub sequence: u64,
    pub id: String,
    pub kind: String,
    pub data: Payload,
    pub checksum: String,
}

/// 📨️ An uncommitted record offered to the log.
///
/// The reference declares `Data interface{}` here and `json.RawMessage` on the committed record: an
/// offered document is re-encoded on the way in — compact, and sorted when it is a generic
/// document — while a committed one is carried verbatim. `normalized_payload` is that difference.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Input {
    pub id: String,
    pub kind: String,
    #[serde(deserialize_with = "normalized_payload")]
    pub data: Payload,
}

/// 📘️ Decodes an offered document and re-encodes it the way the reference's `interface{}` does.
fn normalized_payload<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<Payload, D::Error> {
    let value = serde_json::Value::deserialize(deserializer)?;
    Payload::of(&value).map_err(serde::de::Error::custom)
}

/// ⏳️ One step of an append or a replay.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Progress {
    pub current: usize,
    pub total: usize,
    pub step: String,
}

/// ⏸️ Cooperative cancellation and progress reporting, the Rust shape of Go's `context` + callback.
pub trait Interrupt {
    /// 🛑️ True once the caller has asked to stop.
    fn cancelled(&self) -> bool {
        false
    }
    /// 📣️ Receives one progress step.
    fn report(&self, _progress: Progress) {}
}

/// ▶️ An interrupt that never cancels and ignores progress.
#[derive(Debug, Clone, Copy, Default)]
pub struct Uninterrupted;

impl Interrupt for Uninterrupted {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct Stage {
    pub(super) schema: String,
    #[serde(rename = "priorExists")]
    pub(super) prior_exists: bool,
    #[serde(rename = "priorSize")]
    pub(super) prior_size: u64,
    #[serde(rename = "priorChecksum")]
    pub(super) prior_checksum: String,
    #[serde(rename = "batchSize")]
    pub(super) batch_size: usize,
    #[serde(rename = "batchChecksum")]
    pub(super) batch_checksum: String,
}

/// 🔏️ The SHA-256 identity of a record over schema, sequence, id, kind and data.
pub fn checksum(event: &StoreEvent) -> String {
    let mut hasher = Sha256::new();
    hasher.update(
        format!("{}\u{0}{}\u{0}{}\u{0}{}\u{0}", event.schema, event.sequence, event.id, event.kind)
            .as_bytes(),
    );
    hasher.update(event.data.text().as_bytes());
    hasher.finish()
}

/// 🗄️ A recoverable append-only JSONL log with deterministic replay.
#[derive(Debug, Clone)]
pub struct Store {
    pub path: PathBuf,
}

impl Store {
    /// 🆕️ A store over one log file.
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        Self { path: path.into() }
    }

    pub(super) fn stage_path(&self) -> PathBuf {
        let mut value = self.path.clone().into_os_string();
        value.push(".stage");
        PathBuf::from(value)
    }

    fn next_path(&self) -> PathBuf {
        let mut value = self.stage_path().into_os_string();
        value.push(".next");
        PathBuf::from(value)
    }

    /// ➕️ Stages, writes and syncs a batch, leaving the prior log untouched on any failure.
    pub fn append(
        &self,
        inputs: &[Input],
        interrupt: &dyn Interrupt,
    ) -> Result<Vec<StoreEvent>, StoreError> {
        if self.path.as_os_str().is_empty() {
            return Err(StoreError::Invalid("event log path is required".into()));
        }
        if interrupt.cancelled() {
            return Err(StoreError::Cancelled);
        }
        self.recover()?;
        let existing =
            if self.path.exists() { self.replay_locked(&Uninterrupted)? } else { Vec::new() };
        if inputs.len() > MAX_BATCH_EVENTS {
            return Err(StoreError::TooLarge(format!(
                "{} events > {MAX_BATCH_EVENTS}",
                inputs.len()
            )));
        }
        let mut seen: Vec<String> = existing.iter().map(|event| event.id.clone()).collect();
        let mut created = Vec::with_capacity(inputs.len());
        let mut batch = String::new();
        for (index, input) in inputs.iter().enumerate() {
            if interrupt.cancelled() {
                return Err(StoreError::Cancelled);
            }
            if input.id.is_empty() || input.kind.is_empty() {
                return Err(StoreError::Invalid("event id and kind are required".into()));
            }
            if seen.iter().any(|id| id == &input.id) {
                return Err(StoreError::Duplicate(input.id.clone()));
            }
            let data = input.data.text();
            if data.len() > MAX_EVENT_SIZE {
                return Err(StoreError::TooLarge(format!("{} > {MAX_EVENT_SIZE}", data.len())));
            }
            let mut event = StoreEvent {
                schema: SCHEMA.to_string(),
                sequence: (existing.len() + index + 1) as u64,
                id: input.id.clone(),
                kind: input.kind.clone(),
                data: input.data.clone(),
                checksum: String::new(),
            };
            event.checksum = checksum(&event);
            batch.push_str(
                &serde_json::to_string(&event)
                    .map_err(|error| StoreError::Invalid(error.to_string()))?,
            );
            batch.push('\n');
            if batch.len() > MAX_BATCH_SIZE {
                return Err(StoreError::TooLarge(format!("batch > {MAX_BATCH_SIZE}")));
            }
            seen.push(input.id.clone());
            created.push(event);
            interrupt.report(Progress {
                current: index + 1,
                total: inputs.len(),
                step: "encoded".into(),
            });
        }
        if interrupt.cancelled() {
            return Err(StoreError::Cancelled);
        }
        if created.is_empty() {
            return Ok(created);
        }
        if let Some(parent) = self.path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)?;
            }
        }
        let prior = self.inspect_prior()?;
        let staged = Stage {
            schema: STAGE_SCHEMA.to_string(),
            prior_exists: prior.0,
            prior_size: prior.1,
            prior_checksum: prior.2,
            batch_size: batch.len(),
            batch_checksum: digest(batch.as_bytes()),
        };
        self.write_stage(&staged)?;
        interrupt.report(Progress { current: 0, total: created.len(), step: "staged".into() });
        if interrupt.cancelled() {
            self.rollback(&staged)?;
            return Err(StoreError::Cancelled);
        }
        let write = fs::OpenOptions::new().create(true).append(true).open(&self.path);
        let mut file = match write {
            Ok(file) => file,
            Err(error) => {
                self.rollback(&staged)?;
                return Err(error.into());
            }
        };
        let mut failure = file.write_all(batch.as_bytes()).err().map(StoreError::from);
        interrupt.report(Progress {
            current: batch.len(),
            total: batch.len(),
            step: "appended".into(),
        });
        if failure.is_none() && interrupt.cancelled() {
            failure = Some(StoreError::Cancelled);
        }
        if failure.is_none() {
            failure = file.sync_all().err().map(StoreError::from);
        }
        if failure.is_none() {
            interrupt.report(Progress {
                current: created.len(),
                total: created.len(),
                step: "synced".into(),
            });
            if interrupt.cancelled() {
                failure = Some(StoreError::Cancelled);
            }
        }
        drop(file);
        if let Some(error) = failure {
            self.rollback(&staged)?;
            return Err(error);
        }
        let _ = fs::remove_file(self.stage_path());
        interrupt.report(Progress {
            current: created.len(),
            total: created.len(),
            step: "committed".into(),
        });
        Ok(created)
    }

    /// ⏪️ Recovers any interrupted append, then reads and validates the whole log.
    pub fn replay(&self, interrupt: &dyn Interrupt) -> Result<Vec<StoreEvent>, StoreError> {
        if self.path.as_os_str().is_empty() {
            return Err(StoreError::Invalid("event log path is required".into()));
        }
        if interrupt.cancelled() {
            return Err(StoreError::Cancelled);
        }
        self.recover()?;
        self.replay_locked(interrupt)
    }

    fn replay_locked(&self, interrupt: &dyn Interrupt) -> Result<Vec<StoreEvent>, StoreError> {
        let content = fs::read(&self.path)?;
        let mut events: Vec<StoreEvent> = Vec::new();
        let mut seen: Vec<String> = Vec::new();
        let mut start = 0usize;
        while start < content.len() {
            if interrupt.cancelled() {
                return Err(StoreError::Cancelled);
            }
            let end = match content[start..].iter().position(|byte| *byte == b'\n') {
                Some(offset) => start + offset,
                None => {
                    return Err(StoreError::Corrupt(format!(
                        "incomplete event at sequence {}",
                        events.len() + 1
                    )))
                }
            };
            let line = &content[start..end];
            if line.len() > MAX_EVENT_SIZE * 2 {
                return Err(StoreError::Corrupt("encoded event too large".into()));
            }
            let expected = (events.len() + 1) as u64;
            let event: StoreEvent = serde_json::from_slice(line)
                .map_err(|error| StoreError::Corrupt(format!("at sequence {expected}: {error}")))?;
            if event.schema != SCHEMA
                || event.sequence != expected
                || event.id.is_empty()
                || event.kind.is_empty()
                || event.checksum != checksum(&event)
            {
                return Err(StoreError::Corrupt(format!("at sequence {expected}")));
            }
            if seen.iter().any(|id| id == &event.id) {
                return Err(StoreError::Duplicate(event.id));
            }
            seen.push(event.id.clone());
            events.push(event);
            interrupt.report(Progress {
                current: events.len(),
                total: 0,
                step: "replayed".into(),
            });
            start = end + 1;
        }
        Ok(events)
    }

    fn inspect_prior(&self) -> Result<(bool, u64, String), StoreError> {
        match fs::read(&self.path) {
            Ok(content) => Ok((true, content.len() as u64, digest(&content))),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                Ok((false, 0, digest(&[])))
            }
            Err(error) => Err(error.into()),
        }
    }

    fn write_stage(&self, staged: &Stage) -> Result<(), StoreError> {
        let next = self.next_path();
        let _ = fs::remove_file(&next);
        let mut encoded =
            serde_json::to_vec(staged).map_err(|error| StoreError::Invalid(error.to_string()))?;
        encoded.push(b'\n');
        let mut file = fs::File::create(&next)?;
        file.write_all(&encoded)?;
        file.sync_all()?;
        drop(file);
        fs::rename(&next, self.stage_path())?;
        Ok(())
    }

    fn recover(&self) -> Result<(), StoreError> {
        let _ = fs::remove_file(self.next_path());
        let data = match fs::read(self.stage_path()) {
            Ok(data) => data,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
            Err(error) => return Err(error.into()),
        };
        let staged: Stage = serde_json::from_slice(&data)
            .map_err(|_| StoreError::Corrupt("invalid append stage".into()))?;
        if staged.schema != STAGE_SCHEMA || staged.batch_size == 0 {
            return Err(StoreError::Corrupt("invalid append stage".into()));
        }
        let content = match fs::read(&self.path) {
            Ok(content) => content,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                if !staged.prior_exists {
                    fs::remove_file(self.stage_path())?;
                    return Ok(());
                }
                return Err(StoreError::Corrupt(format!("staged log missing: {error}")));
            }
            Err(error) => return Err(error.into()),
        };
        let size = content.len() as u64;
        if size < staged.prior_size || size > staged.prior_size + staged.batch_size as u64 {
            return Err(StoreError::Corrupt("staged append size".into()));
        }
        let prior_size = staged.prior_size as usize;
        if digest(&content[..prior_size]) != staged.prior_checksum {
            return Err(StoreError::Corrupt("staged append prefix".into()));
        }
        let committed = size == staged.prior_size + staged.batch_size as u64
            && digest(&content[prior_size..]) == staged.batch_checksum;
        if !committed && size != staged.prior_size {
            let file = fs::OpenOptions::new().write(true).open(&self.path)?;
            file.set_len(staged.prior_size)?;
            file.sync_all()?;
        }
        if !staged.prior_exists && staged.prior_size == 0 && !committed {
            match fs::remove_file(&self.path) {
                Ok(()) => {}
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(error) => return Err(error.into()),
            }
        }
        fs::remove_file(self.stage_path())?;
        Ok(())
    }

    fn rollback(&self, staged: &Stage) -> Result<(), StoreError> {
        match fs::OpenOptions::new().write(true).open(&self.path) {
            Ok(file) => {
                file.set_len(staged.prior_size)?;
                file.sync_all()?;
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                if !staged.prior_exists {
                    let _ = fs::remove_file(self.stage_path());
                    return Ok(());
                }
                return Err(error.into());
            }
            Err(error) => return Err(error.into()),
        }
        if !staged.prior_exists {
            match fs::remove_file(&self.path) {
                Ok(()) => {}
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(error) => return Err(error.into()),
            }
        }
        match fs::remove_file(self.stage_path()) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.into()),
        }
        Ok(())
    }
}

//#endregion 🗄️Store
