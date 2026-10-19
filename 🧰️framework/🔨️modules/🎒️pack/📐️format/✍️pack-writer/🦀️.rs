//! ✍️ Pack writer: the write options, the writer, the identity segment and chunk writers and the writer implementation. See [`super`].

use super::*;

//#region 🔖️Writer
/// ⚙️ Header flags and the codec to compress every segment/chunk with when building a
/// pack file. `REQUIRED_COMPRESSED` is set automatically in the written header whenever `codec`
/// is non-identity, so callers only need to set it themselves for other reasons (e.g. signaling
/// intent before any segment is written).
#[derive(Clone, Copy, Debug)]
pub struct WriteOptions {
    pub required_flags: u32,
    pub optional_flags: u32,
    pub codec: CodecId,
}

/// ✒️ Sequential pack file builder. Write segments/chunks in any order, then `finish`
/// with a `Manifest` — `finish` fills in `symbols_span`/`chunk_table_span`/`chunk_count`/
/// `symbol_count` authoritatively from what was actually written (the caller-supplied values in
/// those fields are ignored), and resolves `manifest.schema_name` to a symref against the last
/// `write_segment(KIND_SYMBOLS, ...)` call, so a symbols segment must be written (containing
/// `schema_name`, unless it's empty) before `finish` is called.
pub struct PackWriter<S: PackSink> {
    sink: S,
    pub(super) options: WriteOptions,
    pub(super) chunks: Vec<ChunkTableEntry>,
    pub(super) symbols: Vec<String>,
    pub(super) symbols_span: Option<ByteRange>,
    document_hasher: semio_framework_hash::Hasher,
}

pub struct PackIdentitySegment<'a, S: PackSink> {
    pub(super) owner: &'a mut PackWriter<S>,
    pub(super) kind: u8,
    payload_len: usize,
    written: usize,
    pub(super) crc: semio_framework_replication_codec::Crc32cCursor,
}

pub struct PackIdentityChunk<'a, S: PackSink> {
    pub(super) owner: &'a mut PackWriter<S>,
    pub(super) payload_offset: u64,
    payload_len: usize,
    written: usize,
    pub(super) segment_crc: semio_framework_replication_codec::Crc32cCursor,
    pub(super) payload_crc: semio_framework_replication_codec::Crc32cCursor,
    pub(super) hash: semio_framework_hash::Hasher,
}

impl<S: PackSink> PackIdentityChunk<'_, S> {
    pub async fn write_fragment(&mut self, fragment: &[u8]) -> Result<(), S::Error> {
        self.written = self.written.checked_add(fragment.len()).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"chunk payload length overflow"})?;
        if self.written > self.payload_len {
            return Err((PackRefusal::LimitExceeded{kind:ValueRefusalKind::InvariantViolated,limit:"chunk exceeded retained payload reservation"}).into());
        }
        self.segment_crc.update_page(fragment);
        self.payload_crc.update_page(fragment);
        self.hash.update(fragment);
        self.owner.sink.write_all(fragment).await
    }

    pub async fn finish(self) -> Result<ChunkId, S::Error> {
        if self.written != self.payload_len {
            return Err((PackRefusal::LimitExceeded{kind:ValueRefusalKind::InvariantViolated,limit:"chunk ended before retained payload reservation"}).into());
        }
        self.owner.sink.write_all(&self.segment_crc.finish().to_le_bytes()).await?;
        let id = ChunkId(self.owner.chunks.len() as u32);
        self.owner.chunks.push(ChunkTableEntry { offset: self.payload_offset, stored_len: self.payload_len as u64, raw_len: self.payload_len as u64, crc32: self.payload_crc.finish(), blake3: *self.hash.finalize().as_bytes() });
        Ok(id)
    }

    pub fn close(self) {}
}

impl<'a, S: PackSink> PackIdentitySegment<'a, S> {
    pub async fn write_fragment(&mut self, fragment: &[u8]) -> Result<(), S::Error> {
        self.written = self.written.checked_add(fragment.len()).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"segment payload length overflow"})?;
        if self.written > self.payload_len {
            return Err((PackRefusal::LimitExceeded{kind:ValueRefusalKind::InvariantViolated,limit:"segment exceeded retained payload reservation"}).into());
        }
        self.crc.update_page(fragment);
        if self.kind == semio_framework_replication_codec::KIND_DOCUMENT {
            self.owner.document_hasher.update(fragment);
        }
        self.owner.sink.write_all(fragment).await
    }

    pub async fn finish(self) -> Result<(), S::Error> {
        if self.written != self.payload_len {
            return Err((PackRefusal::LimitExceeded{kind:ValueRefusalKind::InvariantViolated,limit:"segment ended before retained payload reservation"}).into());
        }
        self.owner.sink.write_all(&self.crc.finish().to_le_bytes()).await
    }
}

impl<S: PackSink> PackWriter<S> {
    /// 🚀️ Writes the 32-byte header and returns a writer positioned right after it.
    pub async fn begin(mut sink: S, options: &WriteOptions) -> Result<Self, S::Error> {
        let mut required_flags = options.required_flags;
        if options.codec.0 != 0 {
            required_flags |= REQUIRED_COMPRESSED;
        }
        let unknown = required_flags & !REQUIRED_KNOWN_MASK;
        if unknown != 0 {
            return Err((PackRefusal::UnknownRequiredFlags(unknown)).into());
        }
        let header = Header { version_major: FORMAT_VERSION_MAJOR, version_minor: FORMAT_VERSION_MINOR, required_flags, optional_flags: options.optional_flags };
        sink.write_all(&header.write_bytes().await).await?;
        Ok(Self { sink, options: WriteOptions { required_flags, optional_flags: options.optional_flags, codec: options.codec }, chunks: Vec::new(), symbols: Vec::new(), symbols_span: None, document_hasher: semio_framework_hash::Hasher::new() })
    }

    /// 📍️ Current absolute write position — the offset the next segment/chunk will start
    /// at. Callers building a `Manifest` (e.g. `doc_span`/`field_index_span`) call this before
    /// and after their own `write_segment` calls to record spans this writer doesn't track
    /// automatically.
    pub async fn position(&self) -> u64 {
        self.sink.position().await
    }

    pub async fn begin_identity_segment(&mut self, kind: u8, payload_len: usize) -> Result<PackIdentitySegment<'_, S>, S::Error> {
        let mut length = [0u8; 10];
        let mut remaining = payload_len as u64;
        let mut count = 0;
        loop {
            let mut byte = (remaining & 0x7f) as u8;
            remaining >>= 7;
            if remaining != 0 {
                byte |= 0x80;
            }
            length[count] = byte;
            count += 1;
            if remaining == 0 {
                break;
            }
        }
        let fixed = [kind, 0];
        let mut crc = semio_framework_replication_codec::Crc32cCursor::new();
        crc.update_page(&fixed);
        crc.update_page(&length[..count]);
        self.sink.write_all(&fixed).await?;
        self.sink.write_all(&length[..count]).await?;
        Ok(PackIdentitySegment { owner: self, kind, payload_len, written: 0, crc })
    }

    /// 🧱️ Opens one `KIND_CHUNK` segment written RAW, whatever codec the pack as a whole
    /// uses. A chunk is framed with its own `flags = 0`, so a reader takes its codec from the
    /// segment, not from the pack — and the chunk table indexes each chunk by absolute payload
    /// offset plus a content hash over the stored bytes, which only lines up while those bytes are
    /// the payload. Refusing here instead made every large `Bytes64` field unencodable under the
    /// DEFAULT `EncodeOptions` (`codec: CodecId(1)`): `large_bytes_field_is_chunked_and_round_trips`
    /// failed at `encode: UnsupportedCodec(1)`, i.e. no document with a blob past
    /// `chunk_threshold` could be written at all.
    pub async fn begin_identity_chunk(&mut self, payload_len: usize) -> Result<PackIdentityChunk<'_, S>, S::Error> {
                if self.options.required_flags&REQUIRED_CHUNKED==0{return Err(PackRefusal::RetainedMalformed{kind:ValueRefusalKind::InvalidValue,what:"required_chunked",offset:12,detail:"append-only chunk writer requires declared REQUIRED_CHUNKED"}.into());}
let base = self.sink.position().await;
        let mut length = [0u8; 10];
        let mut remaining = payload_len as u64;
        let mut count = 0usize;
        loop {
            let mut byte = (remaining & 0x7f) as u8;
            remaining >>= 7;
            if remaining != 0 {
                byte |= 0x80;
            }
            length[count] = byte;
            count += 1;
            if remaining == 0 {
                break;
            }
        }
        let fixed = [semio_framework_replication_codec::KIND_CHUNK, 0];
        let mut segment_crc = semio_framework_replication_codec::Crc32cCursor::new();
        segment_crc.update_page(&fixed);
        segment_crc.update_page(&length[..count]);
        self.sink.write_all(&fixed).await?;
        self.sink.write_all(&length[..count]).await?;
        Ok(PackIdentityChunk { owner: self, payload_offset: base + fixed.len() as u64 + count as u64, payload_len, written: 0, segment_crc, payload_crc: semio_framework_replication_codec::Crc32cCursor::new(), hash: semio_framework_hash::Hasher::new() })
    }

    /// 🖇️ Frames, compresses (per `options.codec`), CRCs, and writes one segment. A
    /// `KIND_SYMBOLS` segment is parsed and remembered for `schema_name` resolution in `finish`;
    /// a `KIND_DOCUMENT` segment's raw bytes are folded into the running content-hash used for
    /// the footer.
    pub async fn write_segment(&mut self, kind: u8, payload: &[u8]) -> Result<(), S::Error> {
        if self.options.codec.0 == 0 && kind != semio_framework_replication_codec::KIND_SYMBOLS {
            let mut segment = self.begin_identity_segment(kind, payload.len()).await?;
            segment.write_fragment(payload).await?;
            return segment.finish().await;
        }
        let base = self.sink.position().await;
        let encoded = encode_segment(kind, self.options.codec, payload).await?;
        self.sink.write_all(&encoded.bytes).await?;
        if kind == semio_framework_replication_codec::KIND_SYMBOLS {
            self.symbols = decode_symbols(payload, &PackLimits::default()).await?;
            self.symbols_span = Some(ByteRange { offset: base, len: encoded.bytes.len() as u64 });
        }
        if kind == semio_framework_replication_codec::KIND_DOCUMENT {
            self.document_hasher.update(payload);
        }
        Ok(())
    }

    /// 🧱️ Writes a `KIND_CHUNK` segment and records its offset/lengths/hashes for the
    /// chunk table `finish` will emit.
    #[cfg(any(test, feature = "testkit"))]
    pub async fn write_chunk(&mut self, payload: &[u8]) -> Result<ChunkId, S::Error> {
                if self.options.required_flags&REQUIRED_CHUNKED==0{return Err(PackRefusal::RetainedMalformed{kind:ValueRefusalKind::InvalidValue,what:"required_chunked",offset:12,detail:"append-only chunk writer requires declared REQUIRED_CHUNKED"}.into());}
if self.options.codec.0 == 0 {
            let mut chunk = self.begin_identity_chunk(payload.len()).await?;
            chunk.write_fragment(payload).await?;
            return chunk.finish().await;
        }
        let base = self.sink.position().await;
        let encoded = encode_segment(semio_framework_replication_codec::KIND_CHUNK, self.options.codec, payload).await?;
        let payload_offset = base + encoded.header_len as u64;
        let stored_bytes = &encoded.bytes[encoded.header_len..encoded.header_len + encoded.stored_len];
        let stored_crc = crc32c(stored_bytes);
        let raw_hash = semio_framework_hash::hash(payload);
        self.sink.write_all(&encoded.bytes).await?;
        let id = ChunkId(self.chunks.len() as u32);
        self.chunks.push(ChunkTableEntry { offset: payload_offset, stored_len: encoded.stored_len as u64, raw_len: payload.len() as u64, crc32: stored_crc, blake3: *raw_hash.as_bytes() });
        Ok(id)
    }

    /// 🏁️ Writes the chunk table (if any chunks were written), the manifest, an `End`
    /// segment, then the footer — and returns the underlying sink.
    pub async fn finish(mut self, manifest: &Manifest) -> Result<S, S::Error> {
                if (self.options.required_flags&REQUIRED_CHUNKED!=0)!=!self.chunks.is_empty(){return Err(PackRefusal::RetainedMalformed{kind:ValueRefusalKind::InvalidValue,what:"required_chunked",offset:12,detail:"writer REQUIRED_CHUNKED disagrees with completed chunk custody"}.into());}
let chunk_count = self.chunks.len() as u64;
        let mut chunk_table_span = ByteRange { offset: 0, len: 0 };
        if !self.chunks.is_empty() {
            let base = self.sink.position().await;
            let chunks = std::mem::take(&mut self.chunks);
            let payload_len = chunks.iter().try_fold(retained_varint_len(chunks.len() as u64), |length, entry| {
                length
                    .checked_add(retained_varint_len(entry.offset))
                    .and_then(|length| length.checked_add(retained_varint_len(entry.stored_len)))
                    .and_then(|length| length.checked_add(retained_varint_len(entry.raw_len)))
                    .and_then(|length| length.checked_add(4 + 32))
                    .ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"chunk table retained length overflow"})
            })?;
            let mut segment = self.begin_identity_segment(semio_framework_replication_codec::KIND_CHUNK_TABLE, payload_len).await?;
            let mut varint = [0u8; 10];
            segment.write_fragment(retained_varint(chunks.len() as u64, &mut varint)).await?;
            for entry in chunks {
                segment.write_fragment(retained_varint(entry.offset, &mut varint)).await?;
                segment.write_fragment(retained_varint(entry.stored_len, &mut varint)).await?;
                segment.write_fragment(retained_varint(entry.raw_len, &mut varint)).await?;
                segment.write_fragment(&entry.crc32.to_le_bytes()).await?;
                segment.write_fragment(&entry.blake3).await?;
            }
            segment.finish().await?;
            chunk_table_span = ByteRange { offset: base, len: self.sink.position().await - base };
        }
        let schema_symref = if manifest.schema_name.is_empty() {
            0u64
        } else {
            self.symbols.iter().position(|symbol| symbol == &manifest.schema_name).ok_or_else(|| PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"manifest",offset:0,detail:format!("schema_name {:?} not found in written symbols table",manifest.schema_name)})? as u64
        };
        let final_manifest = Manifest {
            schema_name: manifest.schema_name.clone(),
            schema_hash: manifest.schema_hash,
            doc_span: manifest.doc_span,
            doc_frame_count: manifest.doc_frame_count,
            symbols_span: self.symbols_span.unwrap_or(ByteRange { offset: 0, len: 0 }),
            chunk_table_span,
            field_index_span: manifest.field_index_span,
            uncompressed_body_len: manifest.uncompressed_body_len,
            field_count: manifest.field_count,
            chunk_count,
            symbol_count: self.symbols.len() as u64,
        };
        let manifest_bytes = encode_manifest_bytes(schema_symref, &final_manifest).await;
        let manifest_base = self.sink.position().await;
        self.write_segment(semio_framework_replication_codec::KIND_MANIFEST, &manifest_bytes).await?;
        let manifest_span = ByteRange { offset: manifest_base, len: self.sink.position().await - manifest_base };

        let mut end = self.begin_identity_segment(semio_framework_replication_codec::KIND_END, 0).await?;
        end.write_fragment(&[]).await?;
        end.finish().await?;

        let content_hash = ContentHash(*self.document_hasher.finalize().as_bytes());
        let file_len = self.sink.position().await + FOOTER_SIZE as u64;
        let footer = Footer {
            version_major: FORMAT_VERSION_MAJOR,
            version_minor: FORMAT_VERSION_MINOR,
            required_flags: self.options.required_flags,
            manifest_offset: manifest_span.offset,
            manifest_len: manifest_span.len,
            file_len,
            content_hash,
            prev_footer_offset: 0,
        };
        self.sink.write_all(&footer.write_bytes().await).await?;
        self.sink.flush().await?;
        Ok(self.sink)
    }
}
//#endregion 🔖️Writer
