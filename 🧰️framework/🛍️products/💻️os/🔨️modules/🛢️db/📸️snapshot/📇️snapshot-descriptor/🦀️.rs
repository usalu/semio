//! 📇️ Snapshot descriptor: the descriptor format limits, the snapshot descriptor with its encoding and the varint, segment and field writers and readers. See [`super`].

use super::*;

//#region 🔖️Descriptor
/// 🔢️ Wire format tag for `SnapshotDescriptor::encode`/`decode` — bumped on any
/// incompatible field layout change so a stale reader fails loudly (`DbError::Corrupt`) instead
/// of misparsing.
pub(super) const DESCRIPTOR_FORMAT_VERSION: u8 = 2;

/// 🛡️ Ceiling on `roots`/`new_pages` entry counts and on any embedded string's byte
/// length, validated before allocating the destination `Vec`/`String` — mirrors `pack_core`'s
/// stated "validate before allocating" invariant.
pub(super) const MAX_HASH_LIST_LEN: u64 = 4_000_000;

pub(super) const MAX_STRING_BYTES: u64 = 1024 * 1024;

/// 📇️ One generation's manifest of a document's snapshot state: the frontier it was taken
/// at, protocol provenance, the root page hashes needed to reconstruct the document tree, and
/// the hashes of the pages whose `KIND_CHUNK` bytes live *in this generation's own blob*
/// (everything else reachable from `roots` is expected to resolve via `parent_generation`'s chain
/// — see `resolve_page`). This is exactly the payload written into the `KIND_SNAPSHOT` segment.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SnapshotDescriptor {
    pub document: ArtifactId,
    pub generation: u64,
    /// 🌳️ `None` for a full baseline (self-sufficient, chains to nothing); `Some(g)` for an
    /// incremental generation whose unlisted pages must be resolved from generation `g` onward.
    pub parent_generation: Option<u64>,
    pub head_seq: u64,
    pub commit_seq: u64,
    pub epoch: u64,
    pub chain_hash: [u8; 32],
    pub protocol_version: u32,
    pub base_pack_hash: Option<ContentHash>,
    pub roots: Vec<ContentHash>,
    /// 🧱️ Page hashes with a `KIND_CHUNK` in *this* blob, in the exact order they were
    /// written — index `i` here is `pack::ChunkId(i)` in this generation's own chunk table.
    pub new_pages: Vec<ContentHash>,
    pub created_at_ms: u64,
}

impl SnapshotDescriptor {
    /// 🧭️ Reconstructs the `Frontier` this generation was taken at.
    pub async fn frontier(&self) -> Frontier {
        Frontier { document: self.document.clone(), head_seq: self.head_seq, commit_seq: self.commit_seq, chain_hash: self.chain_hash, epoch: self.epoch }
    }

    /// ✍️ Serializes this descriptor to the exact bytes written into the `KIND_SNAPSHOT`
    /// segment — a flat, versioned, varint-framed encoding (this crate's own choice; the contract
    /// fixes only the segment kind, not the payload layout).
    #[cfg(test)]
    pub async fn encode(&self) -> Vec<u8> {
        let mut w = pack::ByteWriter::new();
        w.write_u8(DESCRIPTOR_FORMAT_VERSION);
        write_string(&mut w, &self.document.0).await;
        w.write_varint_u64(self.generation);
        write_option_u64(&mut w, self.parent_generation).await;
        w.write_varint_u64(self.head_seq);
        w.write_varint_u64(self.commit_seq);
        w.write_varint_u64(self.epoch);
        w.write_bytes(&self.chain_hash);
        w.write_varint_u64(self.protocol_version as u64);
        match &self.base_pack_hash {
            Some(hash) => {
                w.write_u8(1);
                w.write_bytes(&hash.0);
            }
            None => w.write_u8(0),
        }
        write_hash_list(&mut w, &self.roots).await;
        write_hash_list(&mut w, &self.new_pages).await;
        w.write_varint_u64(self.created_at_ms);
        w.into_bytes()
    }

    pub(super) fn retained_len(&self) -> Result<usize, DbError> {
        let mut len = 1usize;
        for field in [
            snapshot_field_len(self.document.0.as_bytes()),
            snapshot_varint_len(self.generation),
            1 + self.parent_generation.map_or(0, snapshot_varint_len),
            snapshot_varint_len(self.head_seq),
            snapshot_varint_len(self.commit_seq),
            snapshot_varint_len(self.epoch),
            32,
            snapshot_varint_len(self.protocol_version as u64),
            1 + self.base_pack_hash.map_or(0, |_| 32),
            snapshot_varint_len(self.roots.len() as u64) + self.roots.len().checked_mul(32).ok_or(DbError::LimitExceeded("snapshot roots bytes"))?,
            snapshot_varint_len(self.new_pages.len() as u64) + self.new_pages.len().checked_mul(32).ok_or(DbError::LimitExceeded("snapshot new pages bytes"))?,
            snapshot_varint_len(self.created_at_ms),
        ] {
            len = len.checked_add(field).ok_or(DbError::LimitExceeded("snapshot descriptor bytes"))?;
        }
        Ok(len)
    }

    pub(super) async fn write_retained(&self, segment: &mut pack::PackIdentitySegment<'_, SnapshotPageSink>) -> Result<(), DbError> {
        snapshot_segment_write(segment, &[DESCRIPTOR_FORMAT_VERSION]).await?;
        snapshot_segment_write_field(segment, self.document.0.as_bytes()).await?;
        snapshot_segment_write_varint(segment, self.generation).await?;
        snapshot_segment_write(segment, &[u8::from(self.parent_generation.is_some())]).await?;
        if let Some(parent) = self.parent_generation {
            snapshot_segment_write_varint(segment, parent).await?;
        }
        snapshot_segment_write_varint(segment, self.head_seq).await?;
        snapshot_segment_write_varint(segment, self.commit_seq).await?;
        snapshot_segment_write_varint(segment, self.epoch).await?;
        snapshot_segment_write(segment, &self.chain_hash).await?;
        snapshot_segment_write_varint(segment, self.protocol_version as u64).await?;
        snapshot_segment_write(segment, &[u8::from(self.base_pack_hash.is_some())]).await?;
        if let Some(hash) = self.base_pack_hash {
            snapshot_segment_write(segment, &hash.0).await?;
        }
        snapshot_segment_write_varint(segment, self.roots.len() as u64).await?;
        for hash in &self.roots {
            snapshot_segment_write(segment, &hash.0).await?;
        }
        snapshot_segment_write_varint(segment, self.new_pages.len() as u64).await?;
        for hash in &self.new_pages {
            snapshot_segment_write(segment, &hash.0).await?;
        }
        snapshot_segment_write_varint(segment, self.created_at_ms).await
    }

    /// 📖️ Inverse of `encode`. Never panics on malformed input — every field read is
    /// bounds-checked by `pack::ByteReader` and every count is checked against
    /// `MAX_HASH_LIST_LEN`/`MAX_STRING_BYTES` before the corresponding `Vec`/`String` is allocated.
    #[cfg(test)]
    pub async fn decode(bytes: &[u8]) -> Result<SnapshotDescriptor, DbError> {
        let mut r = pack::ByteReader::new(bytes);
        let version = r.read_u8()?;
        if version != DESCRIPTOR_FORMAT_VERSION {
            return Err(DbError::Corrupt(format!("unsupported snapshot descriptor format version {version}")));
        }
        let document = ArtifactId(read_string(&mut r).await?);
        let generation = r.read_varint_u64()?;
        let parent_generation = read_option_u64(&mut r).await?;
        let head_seq = r.read_varint_u64()?;
        let commit_seq = r.read_varint_u64()?;
        let epoch = r.read_varint_u64()?;
        let chain_hash = r.read_array32()?;
        let protocol_version = r.read_varint_u64()? as u32;
        let base_pack_hash = match r.read_u8()? {
            0 => None,
            1 => Some(ContentHash(r.read_array32()?)),
            other => return Err(DbError::Corrupt(format!("bad option tag {other}"))),
        };
        let roots = read_hash_list(&mut r).await?;
        let new_pages = read_hash_list(&mut r).await?;
        let created_at_ms = r.read_varint_u64()?;
        Ok(SnapshotDescriptor { document, generation, parent_generation, head_seq, commit_seq, epoch, chain_hash, protocol_version, base_pack_hash, roots, new_pages, created_at_ms })
    }
}

fn snapshot_varint(mut value: u64, output: &mut [u8; 10]) -> &[u8] {
    let mut len = 0;
    loop {
        let mut byte = (value & 0x7f) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        output[len] = byte;
        len += 1;
        if value == 0 {
            return &output[..len];
        }
    }
}

pub(super) fn snapshot_varint_len(value: u64) -> usize {
    let mut output = [0u8; 10];
    snapshot_varint(value, &mut output).len()
}

pub(super) fn snapshot_field_len(bytes: &[u8]) -> usize {
    snapshot_varint_len(bytes.len() as u64) + bytes.len()
}

pub(super) async fn snapshot_segment_write(segment: &mut pack::PackIdentitySegment<'_, SnapshotPageSink>, bytes: &[u8]) -> Result<(), DbError> {
    segment.write_fragment(bytes).await.map_err(DbError::from)
}

pub(super) async fn snapshot_segment_write_varint(segment: &mut pack::PackIdentitySegment<'_, SnapshotPageSink>, value: u64) -> Result<(), DbError> {
    let mut output = [0u8; 10];
    snapshot_segment_write(segment, snapshot_varint(value, &mut output)).await
}

pub(super) async fn snapshot_segment_write_field(segment: &mut pack::PackIdentitySegment<'_, SnapshotPageSink>, bytes: &[u8]) -> Result<(), DbError> {
    snapshot_segment_write_varint(segment, bytes.len() as u64).await?;
    snapshot_segment_write(segment, bytes).await
}

#[cfg(test)]
async fn write_string(w: &mut pack::ByteWriter, s: &str) {
    w.write_varint_u64(s.len() as u64);
    w.write_bytes(s.as_bytes());
}

#[cfg(test)]
async fn read_string(r: &mut pack::ByteReader<'_>) -> Result<String, DbError> {
    let len = r.read_varint_u64()?;
    check_len(len, MAX_STRING_BYTES, "snapshot descriptor string")?;
    let bytes = r.read_bytes(len as usize)?;
    String::from_utf8(bytes.to_vec()).map_err(|_| DbError::Corrupt("invalid utf8 in snapshot descriptor".to_string()))
}

#[cfg(test)]
async fn write_option_u64(w: &mut pack::ByteWriter, value: Option<u64>) {
    match value {
        Some(v) => {
            w.write_u8(1);
            w.write_varint_u64(v);
        }
        None => w.write_u8(0),
    }
}

#[cfg(test)]
async fn read_option_u64(r: &mut pack::ByteReader<'_>) -> Result<Option<u64>, DbError> {
    match r.read_u8()? {
        0 => Ok(None),
        1 => Ok(Some(r.read_varint_u64()?)),
        other => Err(DbError::Corrupt(format!("bad option tag {other}"))),
    }
}

#[cfg(test)]
async fn write_hash_list(w: &mut pack::ByteWriter, hashes: &[ContentHash]) {
    w.write_varint_u64(hashes.len() as u64);
    for hash in hashes {
        w.write_bytes(&hash.0);
    }
}

#[cfg(test)]
async fn read_hash_list(r: &mut pack::ByteReader<'_>) -> Result<Vec<ContentHash>, DbError> {
    let count = r.read_varint_u64()?;
    check_len(count, MAX_HASH_LIST_LEN, "snapshot descriptor hash list")?;
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        out.push(ContentHash(r.read_array32()?));
    }
    Ok(out)
}
//#endregion 🔖️Descriptor
