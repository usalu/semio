//! 🧵️ Snapshot segment io: the pack sub sources, the descriptor reader and decoder, the page sink and the page sources. See [`super`].

use super::*;

//#region 🔖️SegmentIo
/// 🪟️ A length-bounded, base-shifted view over a borrowed byte buffer that implements
/// `semio_framework_pack_source::PackSource` — the mechanism this crate uses to open one generation's own pack structure
/// (`PackFile::open_manifest`, `read_footer_only`) at an arbitrary offset inside a larger
/// multi-generation concatenation, without touching any `pack_format` private internals.
#[cfg(any(test, feature = "testkit"))]
pub struct SubSource<'a> {
    pub(super) inner: &'a [u8],
    pub(super) base: u64,
    pub(super) len: u64,
}

#[cfg(any(test, feature = "testkit"))]
impl<'a> semio_framework_pack_source::PackSource for SubSource<'a> {
    async fn len(&self) -> u64 {
        self.len
    }

    async fn read_at(&self, offset: u64, buf: &mut [u8]) -> Result<usize, pack::PackError> {
        if offset > self.len {
            return Err(pack::PackError::Refusal(pack::PackRefusal::Truncated(offset)));
        }
        let available = ((self.len - offset) as usize).min(buf.len());
        self.inner.read_at(self.base + offset, &mut buf[..available]).await
    }
}

#[derive(Clone, Copy)]
pub(super) struct PageSubSource<'pages> {
    pub(super) inner: &'pages db_io_core::DbIoPages,
    pub(super) base: u64,
    pub(super) len: u64,
}

impl semio_framework_pack_source::PackSource for PageSubSource<'_> {
    async fn len(&self) -> u64 {
        self.len
    }

    async fn read_at(&self, offset: u64, output: &mut [u8]) -> Result<usize, pack::PackError> {
        if offset > self.len {
            return Err(pack::PackError::Refusal(pack::PackRefusal::Truncated(offset)));
        }
        let absolute = self.base.checked_add(offset).ok_or(pack::PackError::Refusal(pack::PackRefusal::Truncated(offset)))? as usize;
        let available = ((self.len - offset) as usize).min(output.len());
        let mut base = 0usize;
        let mut written = 0usize;
        for fragment in self.inner.fragments() {
            let end = base + fragment.len();
            if end <= absolute {
                base = end;
                continue;
            }
            let start = absolute.saturating_sub(base);
            let count = (available - written).min(fragment.len() - start);
            output[written..written + count].copy_from_slice(&fragment[start..start + count]);
            written += count;
            base = end;
            if written == available {
                break;
            }
        }
        Ok(written)
    }
}

struct SnapshotDescriptorReader<'source, 'control, S: semio_framework_pack_source::PackSource> {
    pub(super) source: &'source S,
    pub(super) cursor: u64,
    end: u64,
    pub(super) crc: pack::codec::Crc32cCursor,
    pub(super) control: &'control mut SnapshotCursorControl,
}

impl<S: semio_framework_pack_source::PackSource> SnapshotDescriptorReader<'_, '_, S> {
    async fn fixed<const N: usize>(&mut self) -> Result<[u8; N], DbError> {
        if self.cursor.checked_add(N as u64).is_none_or(|end| end > self.end) {
            return Err(DbError::Corrupt("snapshot descriptor ended early".to_string()));
        }
        self.control.grant()?;
        let mut output = [0u8; N];
        self.source.read_exact_at(self.cursor, &mut output).await?;
        self.cursor += N as u64;
        self.crc.update_page(&output);
        Ok(output)
    }

    pub(super) async fn byte(&mut self) -> Result<u8, DbError> {
        Ok(self.fixed::<1>().await?[0])
    }

    async fn varint(&mut self) -> Result<u64, DbError> {
        let mut value = 0u64;
        for shift in (0..70).step_by(7) {
            let byte = self.byte().await?;
            value |= u64::from(byte & 0x7f) << shift;
            if byte & 0x80 == 0 {
                return Ok(value);
            }
        }
        Err(DbError::Corrupt("snapshot descriptor varint overflow".to_string()))
    }

    async fn option_u64(&mut self) -> Result<Option<u64>, DbError> {
        match self.byte().await? {
            0 => Ok(None),
            1 => Ok(Some(self.varint().await?)),
            tag => Err(DbError::Corrupt(format!("bad snapshot option tag {tag}"))),
        }
    }

    async fn text(&mut self) -> Result<String, DbError> {
        let len = self.varint().await?;
        check_len(len, MAX_STRING_BYTES, "snapshot descriptor string")?;
        if self.cursor.checked_add(len).is_none_or(|end| end > self.end) {
            return Err(DbError::Corrupt("snapshot descriptor string ended early".to_string()));
        }
        let mut remaining = len as usize;
        let mut output = Vec::with_capacity(remaining);
        let mut fragment = [0u8; 4096];
        while remaining != 0 {
            self.control.grant()?;
            let count = remaining.min(fragment.len());
            self.source.read_exact_at(self.cursor, &mut fragment[..count]).await?;
            self.cursor += count as u64;
            remaining -= count;
            self.crc.update_page(&fragment[..count]);
            output.extend_from_slice(&fragment[..count]);
        }
        String::from_utf8(output).map_err(|_| DbError::Corrupt("invalid utf8 in snapshot descriptor".to_string()))
    }

    pub(super) async fn hashes(&mut self) -> Result<Vec<ContentHash>, DbError> {
        let count = self.varint().await?;
        check_len(count, MAX_HASH_LIST_LEN, "snapshot descriptor hash list")?;
        if count.checked_mul(32).and_then(|bytes| self.cursor.checked_add(bytes)).is_none_or(|end| end > self.end) {
            return Err(DbError::Corrupt("snapshot descriptor hash list ended early".to_string()));
        }
        let mut output = Vec::with_capacity(count as usize);
        for _ in 0..count {
            output.push(ContentHash(self.fixed::<32>().await?));
        }
        Ok(output)
    }
}

pub(super) async fn decode_snapshot_descriptor(source: &impl semio_framework_pack_source::PackSource, control: &mut SnapshotCursorControl) -> Result<SnapshotDescriptor, DbError> {
    let offset = pack::HEADER_SIZE as u64;
    let mut prefix = [0u8; 12];
    control.grant()?;
    source.read_exact_at(offset, &mut prefix).await?;
    let kind = prefix[0];
    let flags = prefix[1];
    if flags != 0 {
        return Err(DbError::Corrupt("snapshot descriptor segment must use the identity codec".to_string()));
    }
    let mut position = 2usize;
    let segment_len = pack::os_pack::read_varint_u64(&prefix, &mut position)?;
    check_len(segment_len, 64 * 1024 * 1024, "snapshot descriptor segment length")?;
    let header_len = position as u64;
    let end = offset.checked_add(header_len).and_then(|cursor| cursor.checked_add(segment_len)).ok_or(DbError::LimitExceeded("snapshot descriptor segment range"))?;
    let mut crc = pack::codec::Crc32cCursor::new();
    crc.update_page(&prefix[..position]);
    let mut reader = SnapshotDescriptorReader { source, cursor: offset + header_len, end, crc, control };
    let version = reader.byte().await?;
    if version != DESCRIPTOR_FORMAT_VERSION {
        return Err(DbError::Corrupt(format!("unsupported snapshot descriptor format version {version}")));
    }
    let document = ArtifactId(reader.text().await?);
    let generation = reader.varint().await?;
    let parent_generation = reader.option_u64().await?;
    let head_seq = reader.varint().await?;
    let commit_seq = reader.varint().await?;
    let epoch = reader.varint().await?;
    let chain_hash = reader.fixed::<32>().await?;
    let protocol_version = u32::try_from(reader.varint().await?).map_err(|_| DbError::Corrupt("snapshot protocol version exceeds u32".to_string()))?;
    let base_pack_hash = match reader.byte().await? {
        0 => None,
        1 => Some(ContentHash(reader.fixed::<32>().await?)),
        tag => return Err(DbError::Corrupt(format!("bad snapshot option tag {tag}"))),
    };
    let roots = reader.hashes().await?;
    let new_pages = reader.hashes().await?;
    let created_at_ms = reader.varint().await?;
    if reader.cursor != reader.end {
        return Err(DbError::Corrupt("snapshot descriptor segment has trailing bytes".to_string()));
    }
    let mut crc_bytes = [0u8; 4];
    reader.control.grant()?;
    source.read_exact_at(end, &mut crc_bytes).await?;
    if u32::from_le_bytes(crc_bytes) != reader.crc.finish() {
        return Err(DbError::Corrupt("snapshot descriptor segment checksum mismatch".to_string()));
    }
    if kind != pack::KIND_SNAPSHOT {
        return Err(DbError::Corrupt(format!("expected KIND_SNAPSHOT segment (0x{:02x}), found 0x{kind:02x}", pack::KIND_SNAPSHOT)));
    }
    Ok(SnapshotDescriptor { document, generation, parent_generation, head_seq, commit_seq, epoch, chain_hash, protocol_version, base_pack_hash, roots, new_pages, created_at_ms })
}
//#endregion 🔖️SegmentIo
