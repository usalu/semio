//! 👤️ Actor, frontier, touched region and commit indexes: the actor sequence index, the frontier index, the touched region index and the commit index. See [`super`].

use super::*;

//#region 🔖️ActorSeqIndex
/// 👤️ `(actor, actor_seq) -> command_seq` — resolves an actor's own local operation sequence
/// number (idempotency / causal-order checks at admission) to the document's global command
/// sequence. Keys are `actor_bytes || 0x00 || actor_seq(8, BE)`; `actor`'s id must not itself
/// contain a NUL byte (validated) so the `0x00` separator stays unambiguous and prefix scans by
/// actor (`latest_for_actor`) can't spill into a neighboring actor's entries.
pub struct ActorSeqIndex<'a, S: IndexStorage> {
    pub handle: IndexHandle<'a, S>,
}

pub(super) async fn validate_actor_key_safe(actor: &ActorId) -> Result<(), DbError> {
    if actor.0.as_bytes().contains(&0u8) {
        return Err(DbError::InvalidArgument("actor id must not contain a NUL byte to be index-key safe".to_string()));
    }
    Ok(())
}

async fn actor_seq_key(actor: &ActorId, actor_seq: u64) -> Result<Vec<u8>, DbError> {
    validate_actor_key_safe(actor).await?;
    let mut key = Vec::with_capacity(actor.0.len() + 1 + 8);
    key.extend_from_slice(actor.0.as_bytes());
    key.push(0u8);
    key.extend_from_slice(&actor_seq.to_be_bytes());
    Ok(key)
}

// 🚫️async: E1 pure accessor consumed by sync Option::map/closures — see R9
fn decode_u64_le(reader: &mut RunPageReader<'_>) -> Result<u64, DbError> {
    Ok(u64::from_le_bytes(reader.array()?))
}

impl<'a, S: IndexStorage> ActorSeqIndex<'a, S> {
    pub async fn new(storage: &'a S, document: ArtifactId) -> Self {
        Self { handle: IndexHandle::new(storage, document, IndexKind::ActorSeq).await }
    }

    pub async fn record(&self, actor: &ActorId, actor_seq: u64, command_seq: u64) -> Result<(), DbError> {
        let mut control = self.handle.operation_control(8_192)?;
        let value = admit_generated_index_bytes(command_seq.to_le_bytes().to_vec(), MAX_VALUE_LEN, &mut control).await?;
        let key_bytes = actor_seq_key(actor, actor_seq).await?;
        let key = IndexBytes::copy_for_operation(value.operation(), &key_bytes, &mut control).await?;
        self.handle.put(key, value, &mut control).await
    }

    pub async fn lookup(&self, actor: &ActorId, actor_seq: u64) -> Result<Option<u64>, DbError> {
        let mut control = self.handle.operation_control(8_192)?;
        let key = admit_generated_index_bytes(actor_seq_key(actor, actor_seq).await?, MAX_KEY_LEN, &mut control).await?;
        let result = self.handle.get(&key, &mut control).await?;
        close_index_bytes(key, &mut control).await?;
        match result {
            Some(bytes) => Ok(Some(decode_index_bytes(bytes, &mut control, decode_u64_le).await?)),
            None => Ok(None),
        }
    }

    /// 📥️ Records `(actor, actor_seq, command_seq)` triples as ONE run (at most
    /// `MAX_RUN_ENTRIES`) of `runs`, keyed and ordered as `record` keys them, without listing.
    pub async fn record_owned_run(&self, runs: &mut OwnedRuns, entries: &[(ActorId, u64, u64)]) -> Result<(), DbError> {
        let mut encoded = Vec::with_capacity(entries.len());
        for (actor, actor_seq, command_seq) in entries {
            encoded.push((actor_seq_key(actor, *actor_seq).await?, command_seq.to_le_bytes()));
        }
        encoded.sort_unstable_by(|left, right| left.0.cmp(&right.0));
        let slices: Vec<(&[u8], &[u8])> = encoded.iter().map(|(key, value)| (&key[..], &value[..])).collect();
        let mut control = self.handle.operation_control(65_536)?;
        self.handle.append_owned_sorted_run(runs, &slices, &mut control).await
    }

    /// 🗂️ This kind's runs, listed once for its append-only owner.
    pub async fn owned_runs(&self) -> Result<OwnedRuns, DbError> {
        let mut control = self.handle.operation_control(8_192)?;
        self.handle.owned_runs(&mut control).await
    }

    /// 🔝️ The highest command seq any entry of the newest run points at, `0` when none —
    /// for an owner that records runs in ascending command order.
    pub async fn indexed_through(&self) -> Result<u64, DbError> {
        let mut control = self.handle.operation_control(8_192)?;
        let Some(view) = self.handle.newest_run(&mut control).await? else { return Ok(0) };
        let mut highest = Ok(0u64);
        for index in 0..view.entries.len() {
            match view.value_u64_le(index) {
                Ok(command_seq) => highest = highest.map(|highest| highest.max(command_seq)),
                Err(error) => {
                    highest = Err(error);
                    break;
                }
            }
        }
        view.close()?;
        highest
    }

    /// 🥇️ The highest `(actor_seq, command_seq)` pair recorded for `actor`, or `None` if
    /// `actor` has never been recorded.
    pub async fn latest_for_actor(&self, actor: &ActorId) -> Result<Option<(u64, u64)>, DbError> {
        validate_actor_key_safe(actor).await?;
        let mut prefix = actor.0.as_bytes().to_vec();
        prefix.push(0u8);
        let mut control = self.handle.operation_control(16_384)?;
        let prefix_owner = admit_generated_index_bytes(prefix, MAX_KEY_LEN, &mut control).await?;
        let latest = self.handle.last_live_in_range(&prefix_owner, None, &mut control).await;
        close_index_bytes(prefix_owner, &mut control).await?;
        let Some((key, value)) = latest? else { return Ok(None) };
        let mut suffix = [0u8; 8];
        let suffix_read = key.read_fragment(key.len().saturating_sub(8), &mut suffix);
        close_index_bytes(key, &mut control).await?;
        if suffix_read != 8 {
            close_index_bytes(value, &mut control).await?;
            return Err(DbError::Corrupt("actor-seq index key has a malformed suffix".to_string()));
        }
        let command = decode_index_bytes(value, &mut control, decode_u64_le).await?;
        Ok(Some((u64::from_be_bytes(suffix), command)))
    }
}
//#endregion 🔖️ActorSeqIndex

//#region 🔖️FrontierIndex
/// 🧭️ `commit_seq -> Frontier` — a per-commit snapshot of `Frontier`, letting
/// `Consistency::Historical`/replica resume resolve "what did the frontier look like at commit N"
/// without replaying.
pub struct FrontierIndex<'a, S: IndexStorage> {
    pub handle: IndexHandle<'a, S>,
}

async fn encode_frontier(frontier: &Frontier) -> Vec<u8> {
    let mut writer = ByteWriter::new();
    let document_bytes = frontier.document.0.as_bytes();
    writer.write_varint_u64(document_bytes.len() as u64);
    writer.write_bytes(document_bytes);
    writer.write_varint_u64(frontier.head_seq);
    writer.write_varint_u64(frontier.commit_seq);
    writer.write_bytes(&frontier.chain_hash);
    writer.write_varint_u64(frontier.epoch);
    writer.into_bytes()
}

fn decode_frontier(reader: &mut RunPageReader<'_>) -> Result<Frontier, DbError> {
    let document_len = reader.varint()?;
    check_len(document_len, MAX_KEY_LEN, "db_index::frontier_document")?;
    let mut document_bytes = vec![0u8; document_len as usize];
    let mut written = 0usize;
    while written < document_bytes.len() {
        let fragment = reader.fragment()?;
        let count = (document_bytes.len() - written).min(fragment.len());
        document_bytes[written..written + count].copy_from_slice(&fragment[..count]);
        reader.position += count;
        written += count;
    }
    let document = ArtifactId(String::from_utf8(document_bytes).map_err(|_| DbError::Corrupt("frontier document id is not valid utf-8".to_string()))?);
    let head_seq = reader.varint()?;
    let commit_seq = reader.varint()?;
    let chain_hash = reader.array()?;
    let epoch = reader.varint()?;
    Ok(Frontier { document, head_seq, commit_seq, chain_hash, epoch })
}

impl<'a, S: IndexStorage> FrontierIndex<'a, S> {
    pub async fn new(storage: &'a S, document: ArtifactId) -> Self {
        Self { handle: IndexHandle::new(storage, document, IndexKind::Frontier).await }
    }

    pub async fn record(&self, frontier: &Frontier) -> Result<(), DbError> {
        let mut control = self.handle.operation_control(8_192)?;
        let value = admit_generated_index_bytes(encode_frontier(frontier).await, MAX_VALUE_LEN, &mut control).await?;
        let key = IndexBytes::copy_for_operation(value.operation(), &frontier.commit_seq.to_be_bytes(), &mut control).await?;
        self.handle.put(key, value, &mut control).await
    }

    pub async fn lookup(&self, commit_seq: u64) -> Result<Option<Frontier>, DbError> {
        let mut control = self.handle.operation_control(8_192)?;
        let key = admit_generated_index_bytes(commit_seq.to_be_bytes().to_vec(), MAX_KEY_LEN, &mut control).await?;
        let result = self.handle.get(&key, &mut control).await?;
        close_index_bytes(key, &mut control).await?;
        match result {
            Some(bytes) => Ok(Some(decode_index_bytes(bytes, &mut control, decode_frontier).await?)),
            None => Ok(None),
        }
    }

    /// 📥️ Records ascending frontiers (by `commit_seq`) as ONE run (at most `MAX_RUN_ENTRIES`) of `runs`, without listing.
    pub async fn record_owned_run(&self, runs: &mut OwnedRuns, frontiers: &[Frontier]) -> Result<(), DbError> {
        let mut encoded = Vec::with_capacity(frontiers.len());
        for frontier in frontiers {
            encoded.push((frontier.commit_seq.to_be_bytes(), encode_frontier(frontier).await));
        }
        let slices: Vec<(&[u8], &[u8])> = encoded.iter().map(|(key, value)| (&key[..], &value[..])).collect();
        let mut control = self.handle.operation_control(65_536)?;
        self.handle.append_owned_sorted_run(runs, &slices, &mut control).await
    }

    /// 🗂️ This kind's runs, listed once for its append-only owner.
    pub async fn owned_runs(&self) -> Result<OwnedRuns, DbError> {
        let mut control = self.handle.operation_control(8_192)?;
        self.handle.owned_runs(&mut control).await
    }

    /// 🔝️ The highest `commit_seq` recorded, `0` when none — from the newest run alone.
    pub async fn indexed_through(&self) -> Result<u64, DbError> {
        let mut control = self.handle.operation_control(8_192)?;
        let Some(view) = self.handle.newest_run(&mut control).await? else { return Ok(0) };
        let highest = view.entries.len().checked_sub(1).map_or(Ok(0), |last| view.key_u64_be(last));
        view.close()?;
        highest
    }

    /// 🥇️ The frontier recorded under the highest `commit_seq`, or `None` if none recorded.
    pub async fn latest(&self) -> Result<Option<Frontier>, DbError> {
        let mut control = self.handle.operation_control(16_384)?;
        let prefix = admit_generated_index_bytes(Vec::new(), MAX_KEY_LEN, &mut control).await?;
        let latest = self.handle.last_live_in_range(&prefix, None, &mut control).await;
        close_index_bytes(prefix, &mut control).await?;
        let Some((key, value)) = latest? else { return Ok(None) };
        close_index_bytes(key, &mut control).await?;
        Ok(Some(decode_index_bytes(value, &mut control, decode_frontier).await?))
    }
}
//#endregion 🔖️FrontierIndex

//#region 🔖️TouchedRegionIndex
/// 🎯️ `region -> [command_seq]` (ascending, deduplicated) — `db_conflict`'s reverse index:
/// given a region a new command is about to touch, which prior commands also touched it (the
/// candidate set for touched-region-intersection conflict checks).
pub struct TouchedRegionIndex<'a, S: IndexStorage> {
    pub(super) handle: IndexHandle<'a, S>,
}

pub(super) async fn encode_postings(postings: &db_io_core::DbIoU64List) -> Vec<u8> {
    let mut writer = ByteWriter::new();
    writer.write_varint_u64(postings.len() as u64);
    for posting in postings.as_slice() {
        writer.write_varint_u64(*posting);
    }
    writer.into_bytes()
}

pub(super) fn decode_postings(reader: &mut RunPageReader<'_>) -> Result<db_io_core::DbIoU64List, DbError> {
    let count = reader.varint()?;
    check_len(count, MAX_RUN_ENTRIES, "db_index::postings")?;
    let mut postings = db_io_core::DbIoU64List::new();
    for _ in 0..count {
        postings.push(reader.varint()?)?;
    }
    Ok(postings)
}

impl<'a, S: IndexStorage> TouchedRegionIndex<'a, S> {
    pub async fn new(storage: &'a S, document: ArtifactId) -> Self {
        Self { handle: IndexHandle::new(storage, document, IndexKind::TouchedRegion).await }
    }

    /// ➕️ Records that `command_seq` touched `region` — read-modify-write over the region's
    /// current posting list, kept sorted and deduplicated.
    pub async fn record_touch(&self, region: &[u8], command_seq: u64) -> Result<(), DbError> {
        let mut control = self.handle.operation_control(16_384)?;
        let mut postings = self.touching_with_control(region, &mut control).await?;
        let mut updated = db_io_core::DbIoU64List::new();
        let mut inserted = false;
        for posting in postings.as_slice() {
            if !inserted && command_seq < *posting {
                updated.push(command_seq)?;
                inserted = true;
            }
            if *posting == command_seq {
                inserted = true;
            }
            updated.push(*posting)?;
        }
        if !inserted {
            updated.push(command_seq)?;
        }
        control.grant()?;
        let _ = postings.close_step();
        drop(postings);
        let value = admit_generated_index_bytes(encode_postings(&updated).await, MAX_VALUE_LEN, &mut control).await?;
        control.grant()?;
        let _ = updated.close_step();
        drop(updated);
        let key = IndexBytes::copy_for_operation(value.operation(), region, &mut control).await?;
        self.handle.put(key, value, &mut control).await
    }

    async fn touching_with_control(&self, region: &[u8], control: &mut IndexCursorControl) -> Result<db_io_core::DbIoU64List, DbError> {
        let key = admit_generated_index_bytes(region.to_vec(), MAX_KEY_LEN, control).await?;
        let result = self.handle.get(&key, control).await?;
        close_index_bytes(key, control).await?;
        match result {
            Some(bytes) => decode_index_bytes(bytes, control, decode_postings).await,
            None => Ok(db_io_core::DbIoU64List::new()),
        }
    }

    pub async fn touching(&self, region: &[u8]) -> Result<db_io_core::DbIoU64List, DbError> {
        let mut control = self.handle.operation_control(16_384)?;
        self.touching_with_control(region, &mut control).await
    }
}
//#endregion 🔖️TouchedRegionIndex

//#region 🔖️CommitIndex
/// 🏁️ `commit_id -> command_seq` — resolves a VCS-facing commit id (`vcs::Checkpoint.id`,
/// per the contract's content-addressed `ck-<hex16>` scheme) to the command sequence it was cut at,
/// for `Consistency::Historical(commit_id)` query resolution.
pub struct CommitIndex<'a, S: IndexStorage> {
    pub handle: IndexHandle<'a, S>,
}

impl<'a, S: IndexStorage> CommitIndex<'a, S> {
    pub async fn new(storage: &'a S, document: ArtifactId) -> Self {
        Self { handle: IndexHandle::new(storage, document, IndexKind::Commit).await }
    }

    pub async fn record(&self, commit_id: &str, command_seq: u64) -> Result<(), DbError> {
        let mut control = self.handle.operation_control(8_192)?;
        let value = admit_generated_index_bytes(command_seq.to_le_bytes().to_vec(), MAX_VALUE_LEN, &mut control).await?;
        let key = IndexBytes::copy_for_operation(value.operation(), commit_id.as_bytes(), &mut control).await?;
        self.handle.put(key, value, &mut control).await
    }

    pub async fn lookup(&self, commit_id: &str) -> Result<Option<u64>, DbError> {
        let mut control = self.handle.operation_control(8_192)?;
        let key = admit_generated_index_bytes(commit_id.as_bytes().to_vec(), MAX_KEY_LEN, &mut control).await?;
        let result = self.handle.get(&key, &mut control).await?;
        close_index_bytes(key, &mut control).await?;
        match result {
            Some(bytes) => Ok(Some(decode_index_bytes(bytes, &mut control, decode_u64_le).await?)),
            None => Ok(None),
        }
    }
}
//#endregion 🔖️CommitIndex
