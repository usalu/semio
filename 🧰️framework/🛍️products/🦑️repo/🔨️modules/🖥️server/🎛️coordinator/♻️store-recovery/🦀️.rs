//! ♻️ Store recovery of the coordinator: the recovery protocol, the inspection and replay of the event file and the store guard. See [`super`].

use super::*;

//#region 🗄️EventStore
impl EventStore {
    pub(super) fn recover_locked(&self) -> Result<(), StoreError> {
        self.set_recovery(RecoveryStatus::default());
        remove_durably(&self.suffixed("stage.next"))?;
        let staged = match std::fs::read_to_string(self.stage_path()) {
            Err(failure) if failure.kind() == std::io::ErrorKind::NotFound => {
                remove_durably(&self.next_path())?;
                let backup = self.inspect(&self.backup_path())?;
                if backup.exists {
                    return Err(StoreError::Corrupt("orphan backup".to_string()));
                }
                return Ok(());
            }
            Err(failure) => return Err(StoreError::Unavailable(format!("read stage: {failure}"))),
            Ok(text) => text,
        };
        let stage = serde_json::from_str::<Value>(&staged)
            .ok()
            .as_ref()
            .and_then(Stage::from_value)
            .ok_or_else(|| StoreError::Corrupt("invalid append stage".to_string()))?;
        let current = self.inspect(&self.path)?;
        let backup = self.inspect(&self.backup_path())?;
        if matches(&current, stage.next_size, &stage.next_checksum) {
            self.cleanup_unfaulted()?;
            self.set_recovery(RecoveryStatus { recovered: true, action: "committed-cleanup".to_string() });
            return Ok(());
        }
        if matches(&backup, stage.prior_size, &stage.prior_checksum) {
            remove_durably(&self.path)?;
            rename_durably(&self.backup_path(), &self.path)?;
            self.cleanup_unfaulted()?;
            self.set_recovery(RecoveryStatus { recovered: true, action: "prior-restored".to_string() });
            return Ok(());
        }
        if stage.prior_exists && matches(&current, stage.prior_size, &stage.prior_checksum) {
            self.cleanup_unfaulted()?;
            self.set_recovery(RecoveryStatus { recovered: true, action: "prior-cleanup".to_string() });
            return Ok(());
        }
        if !stage.prior_exists && !current.exists {
            self.cleanup_unfaulted()?;
            self.set_recovery(RecoveryStatus { recovered: true, action: "empty-cleanup".to_string() });
            return Ok(());
        }
        Err(StoreError::Corrupt("neither committed nor prior log is valid".to_string()))
    }

    fn set_recovery(&self, status: RecoveryStatus) {
        *self.recovery.lock().expect("recovery") = status;
    }

    pub(super) fn inspect(&self, path: &Path) -> Result<PersistedFile, StoreError> {
        match std::fs::read(path) {
            Err(failure) if failure.kind() == std::io::ErrorKind::NotFound => Ok(PersistedFile { exists: false, size: 0, checksum: digest(&[]) }),
            Err(failure) => Err(StoreError::Unavailable(failure.to_string())),
            Ok(data) => Ok(PersistedFile { exists: true, size: data.len() as u64, checksum: digest(&data) }),
        }
    }

    pub(super) fn read_optional(&self) -> Result<Option<Vec<u8>>, StoreError> {
        match std::fs::read(&self.path) {
            Err(failure) if failure.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(failure) => Err(StoreError::Unavailable(failure.to_string())),
            Ok(data) => {
                if data.len() as u64 > self.limits.max_log_bytes {
                    return Err(StoreError::Limit(format!("log bytes {} > {}", data.len(), self.limits.max_log_bytes)));
                }
                Ok(Some(data))
            }
        }
    }

    /// ⏪️ Recovers and returns a deterministic copy of every valid event.
    pub fn replay(&self) -> Result<Vec<EventEnvelope>, StoreError> {
        let guard = self.lock()?;
        let outcome = self.recover_locked().and_then(|()| self.replay_locked(true));
        drop(guard);
        outcome
    }

    pub(super) fn replay_locked(&self, recover_tail: bool) -> Result<Vec<EventEnvelope>, StoreError> {
        let data = match self.read_optional()? {
            None => return Ok(Vec::new()),
            Some(data) if data.is_empty() => return Ok(Vec::new()),
            Some(data) => data,
        };
        let mut data = data;
        if data.last() != Some(&b'\n') {
            if !recover_tail {
                return Err(StoreError::Corrupt("partial tail".to_string()));
            }
            let keep = data.iter().rposition(|byte| *byte == b'\n').map_or(0, |index| index + 1);
            let file = OpenOptions::new().write(true).open(&self.path).map_err(|failure| StoreError::Unavailable(failure.to_string()))?;
            file.set_len(keep as u64).map_err(|failure| StoreError::Unavailable(failure.to_string()))?;
            file.sync_all().map_err(|failure| StoreError::Unavailable(failure.to_string()))?;
            data.truncate(keep);
        }
        let text = String::from_utf8(data).map_err(|_| StoreError::Corrupt("log is not UTF-8".to_string()))?;
        let mut events: Vec<EventEnvelope> = Vec::new();
        let mut sequences: HashMap<String, u64> = HashMap::new();
        for (index, line) in text.lines().enumerate() {
            if events.len() >= self.limits.max_replay_events {
                return Err(StoreError::Limit(format!("replay events exceed {}", self.limits.max_replay_events)));
            }
            if line.len() > self.limits.max_append_bytes {
                return Err(StoreError::Limit(format!("encoded event bytes {} > {}", line.len(), self.limits.max_append_bytes)));
            }
            let event = decode_event(line).map_err(|_| StoreError::Corrupt(format!("malformed event {}", index + 1)))?;
            if encode_event(&event) != format!("{line}\n") {
                return Err(StoreError::Corrupt(format!("non-canonical event {}", index + 1)));
            }
            let expected = sequences.get(&event.stream).copied().unwrap_or(0) + 1;
            if !valid_scalar(&event.stream) || !valid_scalar(&event.id) || event.generation == 0 || !valid_scalar(&event.kind) || event.sequence != expected || event.checksum != event_checksum(&event) {
                return Err(StoreError::Corrupt(format!("invalid event {}", index + 1)));
            }
            validate_payload(&event.payload, self.limits.max_json_depth).map_err(|_| StoreError::Corrupt(format!("invalid payload {}", index + 1)))?;
            if events.iter().any(|seen| seen.id == event.id) {
                return Err(StoreError::Duplicate(event.id));
            }
            sequences.insert(event.stream.clone(), event.sequence);
            events.push(event);
        }
        Ok(events)
    }
}

pub(super) fn matches(file: &PersistedFile, size: u64, checksum: &str) -> bool {
    file.exists && file.size == size && file.checksum == checksum
}

/// 🔓️ Releases the on-disk lock file and the in-process holder together.
pub(super) struct StoreGuard {
    pub(super) holder: PathBuf,
    pub(super) lock_path: PathBuf,
}

impl Drop for StoreGuard {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.lock_path);
        let (holders, condition) = process_locks();
        holders.lock().expect("locks").remove(&self.holder);
        condition.notify_all();
    }
}
//#endregion 🗄️EventStore
