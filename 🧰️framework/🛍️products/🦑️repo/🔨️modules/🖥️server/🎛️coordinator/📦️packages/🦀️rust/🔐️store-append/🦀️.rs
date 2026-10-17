//! 🔐️ Store append of the coordinator: the lock, the append, the prepare, commit, rollback and cleanup of one staged append. See [`super`].

use super::*;

//#region 🗄️EventStore
impl EventStore {
    pub(super) fn lock(&self) -> Result<StoreGuard, StoreError> {
        let absolute = std::fs::canonicalize(self.path.parent().unwrap_or(Path::new(".")))
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(self.path.file_name().unwrap_or_default());
        let (holders, condition) = process_locks();
        let mut held = holders.lock().expect("locks");
        while !held.insert(absolute.clone()) {
            held = condition.wait(held).expect("locks");
        }
        drop(held);
        let lock_path = self.suffixed("lock");
        loop {
            match OpenOptions::new().create_new(true).write(true).open(&lock_path) {
                Ok(mut file) => {
                    let _ = writeln!(file, "{}", std::process::id());
                    let _ = file.sync_all();
                    return Ok(StoreGuard { holder: absolute, lock_path });
                }
                Err(failure) if failure.kind() == std::io::ErrorKind::AlreadyExists => {
                    if let Ok(metadata) = std::fs::metadata(&lock_path) {
                        let stale = metadata
                            .modified()
                            .ok()
                            .and_then(|when| SystemTime::now().duration_since(when).ok())
                            .is_some_and(|age| age > Duration::from_secs(30));
                        if stale {
                            let _ = std::fs::remove_file(&lock_path);
                            continue;
                        }
                    }
                    std::thread::sleep(Duration::from_millis(2));
                }
                Err(failure) => {
                    let (holders, condition) = process_locks();
                    holders.lock().expect("locks").remove(&absolute);
                    condition.notify_all();
                    return Err(StoreError::Unavailable(format!("acquire lock: {failure}")));
                }
            }
        }
    }

    /// ➕️ Stages and atomically commits events at the expected stream sequence.
    pub fn append(&self, expected_sequence: u64, inputs: &[EventInput]) -> Result<AppendResult, StoreError> {
        if inputs.is_empty() {
            return Ok(AppendResult::default());
        }
        if inputs.len() > self.limits.max_append_events {
            return Err(StoreError::Limit(format!("events {} > {}", inputs.len(), self.limits.max_append_events)));
        }
        let guard = self.lock()?;
        let outcome = self.append_locked(expected_sequence, inputs);
        drop(guard);
        outcome
    }

    fn append_locked(&self, expected_sequence: u64, inputs: &[EventInput]) -> Result<AppendResult, StoreError> {
        self.recover_locked()?;
        let existing = self.replay_locked(true)?;
        let (created, duplicate) = self.prepare(expected_sequence, &existing, inputs)?;
        if duplicate {
            return Ok(AppendResult { events: created, duplicate: true, committed: true, pending_cleanup: false });
        }
        let encoded: String = created.iter().map(encode_event).collect();
        let prior = self.read_optional()?;
        let prior_bytes = prior.clone().unwrap_or_default();
        if (prior_bytes.len() + encoded.len()) as u64 > self.limits.max_log_bytes {
            return Err(StoreError::Limit(format!("log bytes exceed {}", self.limits.max_log_bytes)));
        }
        let mut combined = prior_bytes.clone();
        combined.extend_from_slice(encoded.as_bytes());
        let stage = Stage {
            prior_exists: prior.is_some(),
            prior_size: prior_bytes.len() as u64,
            prior_checksum: digest(&prior_bytes),
            next_size: combined.len() as u64,
            next_checksum: digest(&combined),
        };
        self.commit(&stage, &prior_bytes, encoded.as_bytes())?;
        Ok(AppendResult { events: created, duplicate: false, committed: true, pending_cleanup: false })
    }

    fn prepare(&self, expected_sequence: u64, existing: &[EventEnvelope], inputs: &[EventInput]) -> Result<(Vec<EventEnvelope>, bool), StoreError> {
        let mut normalized = Vec::with_capacity(inputs.len());
        for input in inputs {
            if !valid_scalar(&input.stream) || !valid_scalar(&input.id) || !valid_scalar(&input.kind) || input.generation == 0 {
                return Err(StoreError::Corrupt("event stream, id, generation, and type are required".to_string()));
            }
            let encoded = to_go_json(&input.payload);
            if encoded.len() > self.limits.max_payload_bytes {
                return Err(StoreError::Limit(format!("payload bytes {} > {}", encoded.len(), self.limits.max_payload_bytes)));
            }
            let mut input = input.clone();
            input.payload = canonical_payload(&input.payload, self.limits.max_json_depth)?;
            normalized.push(input);
        }
        let mut by_id: HashMap<&str, &EventEnvelope> = HashMap::new();
        let mut sequences: HashMap<&str, u64> = HashMap::new();
        for event in existing {
            by_id.insert(event.id.as_str(), event);
            sequences.insert(event.stream.as_str(), event.sequence);
        }
        let mut duplicates = Vec::new();
        let mut all_duplicate = true;
        for input in &normalized {
            match by_id.get(input.id.as_str()) {
                None => all_duplicate = false,
                Some(event) => {
                    if event.stream != input.stream || event.generation != input.generation || event.kind != input.kind || event.payload != input.payload {
                        return Err(StoreError::Duplicate(format!("id {:?} has different content", input.id)));
                    }
                    duplicates.push((*event).clone());
                }
            }
        }
        if all_duplicate {
            return Ok((duplicates, true));
        }
        if !duplicates.is_empty() {
            return Err(StoreError::Duplicate("mixed duplicate and new batch".to_string()));
        }
        let stream = normalized[0].stream.clone();
        let actual = sequences.get(stream.as_str()).copied().unwrap_or(0);
        if actual != expected_sequence {
            return Err(StoreError::SequenceConflict(format!("stream {stream:?} expected {expected_sequence} actual {actual}")));
        }
        let mut created = Vec::with_capacity(normalized.len());
        let mut seen: Vec<String> = Vec::new();
        let mut encoded_bytes = 0usize;
        for (index, input) in normalized.iter().enumerate() {
            if input.stream != stream {
                return Err(StoreError::Corrupt("one append batch must target one stream".to_string()));
            }
            if seen.contains(&input.id) {
                return Err(StoreError::Duplicate(input.id.clone()));
            }
            let mut event = EventEnvelope {
                stream: stream.clone(),
                sequence: expected_sequence + index as u64 + 1,
                id: input.id.clone(),
                generation: input.generation,
                kind: input.kind.clone(),
                payload: input.payload.clone(),
                checksum: String::new(),
            };
            event.checksum = event_checksum(&event);
            encoded_bytes += encode_event(&event).len();
            if encoded_bytes > self.limits.max_append_bytes {
                return Err(StoreError::Limit(format!("append bytes {encoded_bytes} > {}", self.limits.max_append_bytes)));
            }
            seen.push(event.id.clone());
            created.push(event);
        }
        Ok((created, false))
    }

    pub(super) fn commit(&self, stage: &Stage, prior: &[u8], appended: &[u8]) -> Result<(), StoreError> {
        let next = self.next_path();
        if let Err(failure) = self.write(&next, &[prior, appended]) {
            return Err(self.rollback_after(failure));
        }
        if let Err(failure) = self.write_stage(stage) {
            return Err(self.rollback_after(failure));
        }
        if stage.prior_exists {
            if let Err(failure) = self.remove(&self.backup_path()) {
                return Err(self.rollback_after(failure));
            }
            if let Err(failure) = self.rename(&self.path, &self.backup_path()) {
                return Err(self.rollback_after(failure));
            }
        }
        if let Err(failure) = self.rename(&next, &self.path) {
            return Err(self.rollback_after(failure));
        }
        self.cleanup().map_err(|failure| StoreError::PendingCleanup(failure.to_string()))
    }

    fn rollback_after(&self, failure: StoreError) -> StoreError {
        let _ = self.rollback();
        failure
    }

    fn write_stage(&self, stage: &Stage) -> Result<(), StoreError> {
        let staged = format!("{}\n", to_go_json(&stage.to_value()));
        let next = self.suffixed("stage.next");
        self.write(&next, &[staged.as_bytes()])?;
        self.rename(&next, &self.stage_path())
    }

    fn rollback(&self) -> Result<(), StoreError> {
        let stage = match std::fs::read_to_string(self.stage_path()) {
            Err(_) => return remove_durably(&self.next_path()),
            Ok(text) => match serde_json::from_str::<Value>(&text).ok().as_ref().and_then(Stage::from_value) {
                None => return remove_durably(&self.next_path()),
                Some(stage) => stage,
            },
        };
        let backup = self.inspect(&self.backup_path())?;
        if matches(&backup, stage.prior_size, &stage.prior_checksum) {
            remove_durably(&self.path)?;
            rename_durably(&self.backup_path(), &self.path)?;
        } else if !stage.prior_exists {
            remove_durably(&self.path)?;
        }
        self.cleanup_unfaulted()
    }

    fn cleanup(&self) -> Result<(), StoreError> {
        for path in [self.next_path(), self.backup_path(), self.stage_path(), self.suffixed("stage.next")] {
            self.remove(&path)?;
        }
        Ok(())
    }

    pub(super) fn cleanup_unfaulted(&self) -> Result<(), StoreError> {
        for path in [self.next_path(), self.backup_path(), self.stage_path(), self.suffixed("stage.next")] {
            remove_durably(&path)?;
        }
        Ok(())
    }
}
//#endregion 🗄️EventStore
