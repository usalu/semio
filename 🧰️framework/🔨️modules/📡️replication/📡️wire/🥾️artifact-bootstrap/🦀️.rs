//! 🥾️ Artifact bootstrap: the bootstrap limits and constants, the pair, the bootstrap, the rebootstrap request, the progress and control and the assembler. See [`super`].

//#region 🔖️ServerFrame
pub const ARTIFACT_BOOTSTRAP_FORMAT_VERSION: u32 = 1;

pub const ARTIFACT_BOOTSTRAP_CHUNK_BYTES: usize = 4 * 1024;

pub const ARTIFACT_BOOTSTRAP_MAX_TOTAL_BYTES: u64 = 64 * 1024 * 1024;

pub const ARTIFACT_BOOTSTRAP_MAX_CHUNKS: u32 = 16 * 1024;

pub const REBOOTSTRAP_SCOPE_MAX_BYTES: usize = 256;

/// 🧩️ The indivisible canonical artifact payload: pack state plus SPR history.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArtifactBootstrapPair {
    pub pack: Vec<u8>,
    pub spr: Vec<u8>,
}

/// 📦️ Descriptor-bound metadata and optional inline pair for one public artifact bootstrap.
#[derive(Clone, Debug, PartialEq)]
pub struct ArtifactBootstrap {
    pub format_version: u32,
    pub descriptor_hash: [u8; 32],
    pub artifact_schema: String,
    pub artifact_kind: String,
    pub pack_schema_hash: [u8; 32],
    pub baseline_frontier: semio_framework_replication_causal::FrontierSummary,
    pub pack_hash: [u8; 32],
    pub spr_hash: [u8; 32],
    pub pack_length: u64,
    pub spr_length: u64,
    pub chunk_count: u32,
    pub aggregate_hash: [u8; 32],
    pub required_tail_frontier: semio_framework_replication_causal::FrontierSummary,
    pub inline: Option<ArtifactBootstrapPair>,
}

/// 🛟️ Storage-key-free checkpoint identity sent before a lagged live stream closes.
#[derive(Clone, Debug, PartialEq)]
pub struct RebootstrapRequired {
    pub space_id: String,
    pub document_id: String,
    pub checkpoint_id: [u8; 32],
    pub descriptor_hash: [u8; 32],
    pub baseline_frontier: semio_framework_replication_causal::FrontierSummary,
}

/// 🌱️ Whether `baseline` is a document's genesis — no edit and no commit yet — the one baseline that names no head edit: a
/// document created and rebuilt before its first check-in has only its creation checkpoint.
fn rebootstrap_baseline_is_genesis(baseline: &semio_framework_replication_causal::FrontierSummary) -> bool {
    baseline.head_edit_ordinal == 0 && baseline.last_commit_seq == 0
}

pub(super) fn validate_rebootstrap_required(control: &RebootstrapRequired) -> Result<(), semio_framework_replication_error::ProtocolError> {
    if control.space_id.is_empty()
        || control.document_id.is_empty()
        || control.space_id.len() > REBOOTSTRAP_SCOPE_MAX_BYTES
        || control.document_id.len() > REBOOTSTRAP_SCOPE_MAX_BYTES
        || !artifact_bootstrap_nonzero(&control.checkpoint_id)
        || !artifact_bootstrap_nonzero(&control.descriptor_hash)
        || control.baseline_frontier.document_id.0 != control.document_id
        || (control.baseline_frontier.head_edit_id.is_empty() && !rebootstrap_baseline_is_genesis(&control.baseline_frontier))
    {
        return Err(artifact_bootstrap_error("rebootstrap control identity is invalid"));
    }
    Ok(())
}

/// 🛡️ Caller-selected assembly ceilings, always constrained by the wire chunk maximum.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArtifactBootstrapLimits {
    pub max_total_bytes: u64,
    pub max_chunks: u32,
    pub max_chunk_bytes: usize,
}

impl Default for ArtifactBootstrapLimits {
    fn default() -> Self {
        Self { max_total_bytes: ARTIFACT_BOOTSTRAP_MAX_TOTAL_BYTES, max_chunks: ARTIFACT_BOOTSTRAP_MAX_CHUNKS, max_chunk_bytes: ARTIFACT_BOOTSTRAP_CHUNK_BYTES }
    }
}

/// 📈️ Observable transfer progress; byte and chunk counts never decrease within one assembler.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArtifactBootstrapProgress {
    pub received_bytes: u64,
    pub total_bytes: u64,
    pub received_chunks: u32,
    pub total_chunks: u32,
}

/// ⏱️ Host-provided cancellation, monotonic clock, and progress boundary.
pub trait ArtifactBootstrapControl {
    fn is_cancelled(&mut self) -> bool;
    fn now_ms(&mut self) -> u64;
    fn on_progress(&mut self, progress: ArtifactBootstrapProgress);
}

#[derive(Debug)]
enum ArtifactBootstrapStorage {
    Inline(ArtifactBootstrapPair),
    Chunked(Vec<u8>),
}

/// 🧱️ Bounded, cancellable staging owner that yields a pair only after complete integrity validation.
#[derive(Debug)]
pub struct ArtifactBootstrapAssembler {
    pub(super) bootstrap: ArtifactBootstrap,
    pub(super) limits: ArtifactBootstrapLimits,
    deadline_ms: Option<u64>,
    storage: Option<ArtifactBootstrapStorage>,
    pub(super) received_bytes: u64,
    next_index: u32,
    pub(super) inline: bool,
}

pub(super) fn artifact_bootstrap_error(detail: impl Into<String>) -> semio_framework_replication_error::ProtocolError {
    semio_framework_replication_error::ProtocolError::Malformed { what: "artifact bootstrap", offset: 0, detail: detail.into() }
}

fn artifact_bootstrap_nonzero(hash: &[u8; 32]) -> bool {
    hash.iter().any(|byte| *byte != 0)
}

fn artifact_bootstrap_total(bootstrap: &ArtifactBootstrap) -> Result<u64, semio_framework_replication_error::ProtocolError> {
    if bootstrap.pack_length == 0 || bootstrap.spr_length == 0 {
        return Err(artifact_bootstrap_error("pack and SPR bytes must both be present"));
    }
    bootstrap.pack_length.checked_add(bootstrap.spr_length).ok_or_else(|| artifact_bootstrap_error("total bytes overflow"))
}

pub(super) fn validate_artifact_bootstrap_header(bootstrap: &ArtifactBootstrap, inline: bool, limits: ArtifactBootstrapLimits) -> Result<u64, semio_framework_replication_error::ProtocolError> {
    if limits.max_total_bytes == 0 || limits.max_chunks == 0 || limits.max_chunk_bytes == 0 || limits.max_chunk_bytes > ARTIFACT_BOOTSTRAP_CHUNK_BYTES {
        return Err(artifact_bootstrap_error("assembler limits are invalid"));
    }
    if bootstrap.format_version != ARTIFACT_BOOTSTRAP_FORMAT_VERSION {
        return Err(artifact_bootstrap_error(format!("version {} is unsupported", bootstrap.format_version)));
    }
    if !artifact_bootstrap_nonzero(&bootstrap.descriptor_hash) || !artifact_bootstrap_nonzero(&bootstrap.pack_schema_hash) || !artifact_bootstrap_nonzero(&bootstrap.pack_hash) || !artifact_bootstrap_nonzero(&bootstrap.spr_hash) || !artifact_bootstrap_nonzero(&bootstrap.aggregate_hash) {
        return Err(artifact_bootstrap_error("required hash must be nonzero"));
    }
    if bootstrap.artifact_schema.is_empty() || bootstrap.artifact_schema.len() > 256 || bootstrap.artifact_kind.is_empty() || bootstrap.artifact_kind.len() > 256 {
        return Err(artifact_bootstrap_error("artifact schema or kind length is invalid"));
    }
    if bootstrap.baseline_frontier.document_id.0.is_empty() || bootstrap.baseline_frontier.document_id != bootstrap.required_tail_frontier.document_id {
        return Err(artifact_bootstrap_error("frontier document mismatch"));
    }
    if bootstrap.required_tail_frontier.head_edit_ordinal < bootstrap.baseline_frontier.head_edit_ordinal || bootstrap.required_tail_frontier.last_commit_seq < bootstrap.baseline_frontier.last_commit_seq {
        return Err(artifact_bootstrap_error("required tail frontier precedes baseline"));
    }
    let total = artifact_bootstrap_total(bootstrap)?;
    if total > limits.max_total_bytes || usize::try_from(total).is_err() {
        return Err(artifact_bootstrap_error("total bytes exceed assembler budget"));
    }
    if inline {
        if bootstrap.chunk_count != 0 {
            return Err(artifact_bootstrap_error("inline pair must declare zero chunks"));
        }
    } else {
        let chunk_count = u64::from(bootstrap.chunk_count);
        if bootstrap.chunk_count == 0 || bootstrap.chunk_count > limits.max_chunks || chunk_count > total || total > chunk_count.saturating_mul(limits.max_chunk_bytes as u64) {
            return Err(artifact_bootstrap_error("chunk count cannot cover declared bytes within budget"));
        }
    }
    Ok(total)
}

pub(super) fn validate_artifact_bootstrap(bootstrap: &ArtifactBootstrap, limits: ArtifactBootstrapLimits) -> Result<u64, semio_framework_replication_error::ProtocolError> {
    let inline = bootstrap.inline.is_some();
    let total = validate_artifact_bootstrap_header(bootstrap, inline, limits)?;
    if let Some(pair) = &bootstrap.inline {
        if pair.pack.len() as u64 != bootstrap.pack_length || pair.spr.len() as u64 != bootstrap.spr_length {
            return Err(artifact_bootstrap_error("inline pair lengths do not match metadata"));
        }
    }
    Ok(total)
}

/// 🔗️ SHA-256 over the exact declared `pack || spr` content stream.
pub fn artifact_bootstrap_aggregate_hash(pack: &[u8], spr: &[u8]) -> [u8; 32] {
    let mut hash = semio_framework_hash::Sha256::new();
    hash.update(pack);
    hash.update(spr);
    hash.finalize()
}

impl ArtifactBootstrapAssembler {
    /// 🆕️ Validates metadata and reserves no more than the caller's explicit budget.
    pub fn new(mut bootstrap: ArtifactBootstrap, expected_descriptor_hash: [u8; 32], limits: ArtifactBootstrapLimits, deadline_ms: Option<u64>, control: &mut impl ArtifactBootstrapControl) -> Result<Self, semio_framework_replication_error::ProtocolError> {
        let total = validate_artifact_bootstrap(&bootstrap, limits)?;
        if !artifact_bootstrap_nonzero(&expected_descriptor_hash) || bootstrap.descriptor_hash != expected_descriptor_hash {
            return Err(artifact_bootstrap_error("descriptor mismatch"));
        }
        if control.is_cancelled() {
            return Err(artifact_bootstrap_error("cancelled"));
        }
        if deadline_ms.is_some_and(|deadline| control.now_ms() >= deadline) {
            return Err(artifact_bootstrap_error("deadline exceeded"));
        }
        let inline_pair = bootstrap.inline.take();
        let inline = inline_pair.is_some();
        let storage = match inline_pair {
            Some(mut pair) => {
                pair.pack.shrink_to_fit();
                pair.spr.shrink_to_fit();
                if pair.pack.capacity().saturating_add(pair.spr.capacity()) as u64 > limits.max_total_bytes {
                    return Err(artifact_bootstrap_error("inline allocation exceeds assembler budget"));
                }
                ArtifactBootstrapStorage::Inline(pair)
            }
            None => {
                let storage = Vec::with_capacity(total as usize);
                if storage.capacity() as u64 > limits.max_total_bytes {
                    return Err(artifact_bootstrap_error("chunk allocation exceeds assembler budget"));
                }
                ArtifactBootstrapStorage::Chunked(storage)
            }
        };
        let mut assembler = Self { bootstrap, limits, deadline_ms, storage: Some(storage), received_bytes: 0, next_index: 0, inline };
        control.on_progress(assembler.progress());
        if inline {
            assembler.received_bytes = total;
            control.on_progress(assembler.progress());
        }
        Ok(assembler)
    }

    /// 💾️ Exact payload allocation debit currently owned by the assembler.
    pub fn retained_bytes(&self) -> usize {
        match self.storage.as_ref() {
            Some(ArtifactBootstrapStorage::Inline(pair)) => pair.pack.capacity().saturating_add(pair.spr.capacity()),
            Some(ArtifactBootstrapStorage::Chunked(bytes)) => bytes.capacity(),
            None => 0,
        }
    }

    /// 📸️ Current monotonic progress snapshot.
    pub fn progress(&self) -> ArtifactBootstrapProgress {
        ArtifactBootstrapProgress { received_bytes: self.received_bytes, total_bytes: self.bootstrap.pack_length + self.bootstrap.spr_length, received_chunks: self.next_index, total_chunks: self.bootstrap.chunk_count }
    }

    /// 🧹️ Drops all staged bytes without producing a completion value.
    pub fn abort(&mut self) {
        self.storage = None;
    }

    fn fail<T>(&mut self, detail: impl Into<String>) -> Result<T, semio_framework_replication_error::ProtocolError> {
        self.abort();
        Err(artifact_bootstrap_error(detail))
    }

    fn guard(&mut self, control: &mut impl ArtifactBootstrapControl) -> Result<(), semio_framework_replication_error::ProtocolError> {
        if control.is_cancelled() {
            return self.fail("cancelled");
        }
        if self.deadline_ms.is_some_and(|deadline| control.now_ms() >= deadline) {
            return self.fail("deadline exceeded");
        }
        if self.storage.is_none() {
            return Err(artifact_bootstrap_error("is not active"));
        }
        Ok(())
    }

    /// ⏭️ Appends exactly the next descriptor-bound, nonempty bounded chunk.
    pub fn push(&mut self, descriptor_hash: [u8; 32], index: u32, bytes: &[u8], control: &mut impl ArtifactBootstrapControl) -> Result<ArtifactBootstrapProgress, semio_framework_replication_error::ProtocolError> {
        self.guard(control)?;
        if self.inline {
            return self.fail("inline transfer cannot accept chunks");
        }
        if descriptor_hash != self.bootstrap.descriptor_hash {
            return self.fail("chunk descriptor mismatch");
        }
        if index != self.next_index {
            return self.fail(format!("chunk index {index} does not equal expected {}", self.next_index));
        }
        if bytes.is_empty() || bytes.len() > self.limits.max_chunk_bytes {
            return self.fail("chunk bytes exceed assembler budget");
        }
        if index >= self.bootstrap.chunk_count {
            return self.fail("chunk index exceeds declared count");
        }
        let Some(end) = self.received_bytes.checked_add(bytes.len() as u64) else { return self.fail("received bytes overflow") };
        if end > self.progress().total_bytes {
            return self.fail("chunk bytes exceed declared total");
        }
        match self.storage.as_mut() {
            Some(ArtifactBootstrapStorage::Chunked(storage)) => storage.extend_from_slice(bytes),
            _ => return self.fail("chunk storage is unavailable"),
        }
        self.received_bytes = end;
        self.next_index += 1;
        let progress = self.progress();
        control.on_progress(progress);
        Ok(progress)
    }

    /// ✅️ Verifies completion and all hashes, transfers ownership of the pair, then retires staging.
    pub fn finish(&mut self, done: Option<([u8; 32], u32)>, control: &mut impl ArtifactBootstrapControl) -> Result<ArtifactBootstrapPair, semio_framework_replication_error::ProtocolError> {
        self.guard(control)?;
        if !self.inline {
            let Some((descriptor_hash, chunk_count)) = done else { return self.fail("is incomplete without done frame") };
            if descriptor_hash != self.bootstrap.descriptor_hash {
                return self.fail("done descriptor mismatch");
            }
            if chunk_count != self.bootstrap.chunk_count {
                return self.fail("done chunk count mismatch");
            }
        } else if done.is_some_and(|(descriptor_hash, chunk_count)| descriptor_hash != self.bootstrap.descriptor_hash || chunk_count != 0) {
            return self.fail("inline done metadata mismatch");
        }
        if self.next_index != self.bootstrap.chunk_count || self.received_bytes != self.progress().total_bytes {
            return self.fail("is incomplete");
        }
        let (pack_hash, spr_hash, aggregate_hash) = match self.storage.as_ref() {
            Some(ArtifactBootstrapStorage::Inline(pair)) => (semio_framework_hash::Sha256::digest(&pair.pack), semio_framework_hash::Sha256::digest(&pair.spr), artifact_bootstrap_aggregate_hash(&pair.pack, &pair.spr)),
            Some(ArtifactBootstrapStorage::Chunked(bytes)) => {
                let split = self.bootstrap.pack_length as usize;
                let (pack, spr) = bytes.split_at(split);
                (semio_framework_hash::Sha256::digest(pack), semio_framework_hash::Sha256::digest(spr), artifact_bootstrap_aggregate_hash(pack, spr))
            }
            None => return Err(artifact_bootstrap_error("is not active")),
        };
        self.guard(control)?;
        if pack_hash != self.bootstrap.pack_hash {
            return self.fail("pack hash mismatch");
        }
        if spr_hash != self.bootstrap.spr_hash {
            return self.fail("SPR hash mismatch");
        }
        if aggregate_hash != self.bootstrap.aggregate_hash {
            return self.fail("aggregate hash mismatch");
        }
        let storage = self.storage.take().ok_or_else(|| artifact_bootstrap_error("is not active"))?;
        match storage {
            ArtifactBootstrapStorage::Inline(pair) => Ok(pair),
            ArtifactBootstrapStorage::Chunked(mut bytes) => {
                let spr = bytes.split_off(self.bootstrap.pack_length as usize);
                Ok(ArtifactBootstrapPair { pack: bytes, spr })
            }
        }
    }
}
//#endregion 🔖️ServerFrame
