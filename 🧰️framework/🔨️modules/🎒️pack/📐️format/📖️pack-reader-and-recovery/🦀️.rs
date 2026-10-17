//! 📖️ Pack reader and recovery: the superblock, the pack file with its identity chunk cursor, the footer-only reader and the forward-scan recovery. See [`super`].

use super::*;

//#region 🔖️Reader
/// 🪪️ The two fixed-size, always-present anchors of a pack file.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Superblock {
    pub header: Header,
    pub footer: Footer,
}

/// 📂️ Random-access pack file reader with three progressively-deeper open levels:
/// `open_superblock` (header+footer only), `open_manifest` (+manifest+symbols+chunk table),
/// and `body_bytes`/`read_chunk` (full content, decompressed and optionally content-hash
/// verified).
pub struct PackFile<S: PackSource> {
    pub(super) source: S,
    pub(super) limits: PackLimits,
    pub(super) superblock: Superblock,
    pub(super) manifest: Option<Manifest>,
    pub(super) symbols: Vec<String>,
    chunk_table: Vec<ChunkTableEntry>,
}

/// 🧩️ Retained identity-chunk reader that advances by one caller-owned fragment.
pub struct PackIdentityChunkCursor<'file, S: PackSource> {
    pub(super) source: &'file S,
    pub(super) entry: ChunkTableEntry,
    pub(super) verification: VerificationLevel,
    pub(super) offset: u64,
    pub(super) crc: semio_framework_replication_codec::Crc32cCursor,
    pub(super) hash: semio_framework_hash::Hasher,
    terminal: bool,
}

impl<'file, S: PackSource> PackIdentityChunkCursor<'file, S> {
    pub fn len(&self) -> u64 {
        self.entry.raw_len
    }

    pub fn is_empty(&self) -> bool {
        self.entry.raw_len == 0
    }

    pub fn remaining(&self) -> u64 {
        self.entry.raw_len.saturating_sub(self.offset)
    }

    pub async fn read_fragment(&mut self, target: &mut [u8]) -> Result<usize, S::Error> {
        if self.terminal {
            return Ok(0);
        }
        let count = usize::try_from(self.remaining().min(target.len() as u64)).map_err(|_| PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"identity chunk fragment length"})?;
        if count == 0 {
            if self.verification.checks_crc() && self.crc.finish() != self.entry.crc32 {
                return Err((PackRefusal::ChecksumMismatch { segment: "chunk", offset: self.entry.offset }).into());
            }
            if self.verification.checks_content_hash() && self.hash.finalize().as_bytes() != &self.entry.blake3 {
                return Err((PackRefusal::ContentHashMismatch).into());
            }
            self.terminal = true;
            return Ok(0);
        }
        self.source.read_exact_at(self.entry.offset + self.offset, &mut target[..count]).await?;
        self.crc.update_page(&target[..count]);
        self.hash.update(&target[..count]);
        self.offset += count as u64;
        Ok(count)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.terminal
    }
}

impl<S: PackSource> PackFile<S> {
    /// 🛬️ Reads owned manifest/catalog state with interior source, checksum, inflation and copy control.
    pub async fn open_manifest_controlled(source:S,limits:&PackLimits,verification:VerificationLevel,control:&mut NativeDecodeControl<'_>)->Result<Self,S::Error>{
        control.checkpoint().map_err(PackRefusal::ValueRefusal)?;let len=source.len().await;
        if len>limits.max_file_len{return Err((PackRefusal::LimitExceeded{kind:ValueRefusalKind::WorkLimit,limit:"source length exceeds max_file_len"}).into());}
        if len<(HEADER_SIZE+FOOTER_SIZE) as u64{return Err((PackRefusal::Truncated(len)).into());}
        let mut header_bytes=[0;HEADER_SIZE];let mut footer_bytes=[0;FOOTER_SIZE];source.read_exact_at(0,&mut header_bytes).await?;source.read_exact_at(len-FOOTER_SIZE as u64,&mut footer_bytes).await?;
        let header=Header::parse(&header_bytes).await?;let footer=Footer::parse(&footer_bytes).await?;
        if footer.file_len!=len{return Err((PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"footer",offset:len-FOOTER_SIZE as u64,detail:"file_len differs from source".into()}).into());}
        let mut this=Self{source,limits:limits.clone(),superblock:Superblock{header,footer},manifest:None,symbols:Vec::new(),chunk_table:Vec::new()};
        let mut manifest_bytes=Vec::new();let(kind,consumed)=append_controlled_segment(&this.source,this.superblock.footer.manifest_offset,&mut manifest_bytes,None,limits,verification,control).await?;
        if kind!=semio_framework_replication_codec::KIND_MANIFEST||consumed!=this.superblock.footer.manifest_len{return Err((PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"manifest",offset:this.superblock.footer.manifest_offset,detail:"manifest framing differs from footer".into()}).into());}
        let raw=parse_raw_manifest(&manifest_bytes).await?;
        if raw.symbols_span.len>0{
            let mut bytes=Vec::new();let(kind,consumed)=append_controlled_segment(&this.source,raw.symbols_span.offset,&mut bytes,None,limits,verification,control).await?;
            if kind!=semio_framework_replication_codec::KIND_SYMBOLS||consumed!=raw.symbols_span.len{return Err((PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"symbols",offset:raw.symbols_span.offset,detail:"symbol framing differs from manifest".into()}).into());}
            let mut position=0;let count=read_varint_u64(&bytes,&mut position)?;
            if count>u64::from(limits.max_symbols){return Err((PackRefusal::LimitExceeded{kind:ValueRefusalKind::WorkLimit,limit:"controlled symbol count exceeds source/limits"}).into());}
            if count>bytes.len() as u64{return Err((PackRefusal::LimitExceeded{kind:ValueRefusalKind::InvalidValue,limit:"controlled symbol count exceeds source/limits"}).into());}
            this.symbols=controlled_vec(control,limits,usize::try_from(count).map_err(|_|PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"symbol count exceeds address space"})?)?;control.begin_stage(count as usize).map_err(PackRefusal::ValueRefusal)?;
            for _ in 0..count{
                control.step().map_err(PackRefusal::ValueRefusal)?;let length=usize::try_from(read_varint_u64(&bytes,&mut position)?).map_err(|_|PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"symbol length exceeds address space"})?;
                let end=position.checked_add(length).filter(|end|*end<=bytes.len()).ok_or(PackRefusal::Truncated(position as u64))?;check_controlled_allocation(control,limits,length)?;
                let text=control.borrow_text(&bytes[position..end]).map_err(PackRefusal::ValueRefusal)?;this.symbols.push(control.copy_text(text).map_err(PackRefusal::ValueRefusal)?);position=end;
            }
            if position!=bytes.len(){return Err((PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"symbols",offset:position as u64,detail:"trailing symbol data".into()}).into());}
        }
        if raw.chunk_table_span.len>0{
            let mut bytes=Vec::new();let(kind,consumed)=append_controlled_segment(&this.source,raw.chunk_table_span.offset,&mut bytes,None,limits,verification,control).await?;
            if kind!=semio_framework_replication_codec::KIND_CHUNK_TABLE||consumed!=raw.chunk_table_span.len{return Err((PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"chunk_table",offset:raw.chunk_table_span.offset,detail:"chunk framing differs from manifest".into()}).into());}
            let mut position=0;let count=read_varint_u64(&bytes,&mut position)?;
            if count>limits.max_items{return Err((PackRefusal::LimitExceeded{kind:ValueRefusalKind::WorkLimit,limit:"controlled chunk table exceeds source/limits"}).into());}
            if count>bytes.len() as u64/39{return Err((PackRefusal::LimitExceeded{kind:ValueRefusalKind::InvalidValue,limit:"controlled chunk table exceeds source/limits"}).into());}
            this.chunk_table=controlled_vec(control,limits,usize::try_from(count).map_err(|_|PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"chunk count exceeds address space"})?)?;control.begin_stage(count as usize).map_err(PackRefusal::ValueRefusal)?;
            for _ in 0..count{
                control.step().map_err(PackRefusal::ValueRefusal)?;let offset=read_varint_u64(&bytes,&mut position)?;let stored_len=read_varint_u64(&bytes,&mut position)?;let raw_len=read_varint_u64(&bytes,&mut position)?;
                if stored_len>limits.max_segment_len||raw_len>limits.max_segment_len{return Err((PackRefusal::LimitExceeded{kind:ValueRefusalKind::WorkLimit,limit:"controlled chunk length exceeds max_segment_len"}).into());}
                let end=position.checked_add(36).filter(|end|*end<=bytes.len()).ok_or(PackRefusal::Truncated(position as u64))?;let crc32=u32::from_le_bytes(bytes[position..position+4].try_into().unwrap());let mut blake3=[0;32];blake3.copy_from_slice(&bytes[position+4..end]);position=end;
                this.chunk_table.push(ChunkTableEntry{offset,stored_len,raw_len,crc32,blake3});
            }
            if position!=bytes.len(){return Err((PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"chunk_table",offset:position as u64,detail:"trailing chunk data".into()}).into());}
        }
        if raw.symbol_count!=this.symbols.len() as u64||raw.chunk_count!=this.chunk_table.len() as u64{return Err((PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"manifest",offset:0,detail:"declared catalog counts differ from contents".into()}).into());}
        let mut manifest=Manifest{schema_name:String::new(),schema_hash:raw.schema_hash,doc_span:raw.doc_span,doc_frame_count:raw.doc_frame_count,symbols_span:raw.symbols_span,chunk_table_span:raw.chunk_table_span,field_index_span:raw.field_index_span,uncompressed_body_len:raw.uncompressed_body_len,field_count:raw.field_count,chunk_count:raw.chunk_count,symbol_count:raw.symbol_count};
        if !this.symbols.is_empty(){let index=usize::try_from(raw.schema_symref).map_err(|_|PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"schema symbol exceeds address space"})?;let text=this.symbols.get(index).ok_or(PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"manifest",offset:0,detail:"schema symbol out of range".into()})?;check_controlled_allocation(control,limits,text.len())?;manifest.schema_name=control.copy_text(text).map_err(PackRefusal::ValueRefusal)?;}
        else if raw.schema_symref!=0{return Err((PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"manifest",offset:0,detail:"schema symbol out of range".into()}).into());}
        this.manifest=Some(manifest);Ok(this)
    }

    /// 📄️ Concatenates directly into one admitted document allocation and controls each frame interior.
    pub async fn body_bytes_controlled(&self,verification:VerificationLevel,control:&mut NativeDecodeControl<'_>)->Result<Vec<u8>,S::Error>{
        let manifest=self.manifest.as_ref().ok_or_else(||PackRefusal::RetainedMalformed{kind:ValueRefusalKind::InvariantViolated,what:"manifest",offset:0,detail:"controlled manifest not loaded"})?;
        let length=usize::try_from(manifest.uncompressed_body_len).map_err(|_|PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"document exceeds address space"})?;
        let mut output=controlled_vec::<u8>(control,&self.limits,length)?;
        if manifest.doc_span.len>0{
            let frames=manifest.doc_frame_count.max(1);if frames>self.limits.max_items{return Err((PackRefusal::LimitExceeded{kind:ValueRefusalKind::WorkLimit,limit:"document frame count exceeds source/limits"}).into());}
            if frames>manifest.doc_span.len/7{return Err((PackRefusal::LimitExceeded{kind:ValueRefusalKind::InvalidValue,limit:"document frame count exceeds source/limits"}).into());}
            let mut offset=manifest.doc_span.offset;let end=offset.checked_add(manifest.doc_span.len).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"document span overflow"})?;
            for _ in 0..frames{let(kind,consumed)=append_controlled_segment(&self.source,offset,&mut output,Some(length),&self.limits,verification,control).await?;if kind!=semio_framework_replication_codec::KIND_DOCUMENT{return Err((PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"document",offset,detail:"unexpected document segment".into()}).into());}offset=offset.checked_add(consumed).filter(|offset|*offset<=end).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::InvalidValue,limit:"document frame exceeds manifest span"})?;}
            if offset!=end{return Err((PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"document",offset,detail:"document frames differ from declared span".into()}).into());}
        }
        if output.len()!=length{return Err((PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"document",offset:manifest.doc_span.offset,detail:"decoded document differs from declared length".into()}).into());}
        if verification.checks_content_hash(){control.begin_stage(output.len()).map_err(PackRefusal::ValueRefusal)?;let mut hash=semio_framework_hash::Hasher::new();for page in output.chunks(4096){hash.update(page);control.advance(page.len()).map_err(PackRefusal::ValueRefusal)?;}if hash.finalize().as_bytes()!=&self.superblock.footer.content_hash.0{return Err((PackRefusal::ContentHashMismatch).into());}}
        Ok(output)
    }

    /// 1⃣ Level 1: parses and CRC-validates the header and footer only, and cross-checks
    /// the footer's `file_len` against the actual source length.
    pub async fn open_superblock(source: S, limits: &PackLimits) -> Result<Self, S::Error> {
        let len = source.len().await;
        if len > limits.max_file_len {
            return Err((PackRefusal::LimitExceeded{kind:ValueRefusalKind::WorkLimit,limit:"source length exceeds max_file_len"}).into());
        }
        if len < FOOTER_SIZE as u64 {
            return Err((PackRefusal::Truncated(len)).into());
        }
        let mut header_bytes = [0u8; HEADER_SIZE];
        source.read_exact_at(0, &mut header_bytes).await?;
        let header = Header::parse(&header_bytes).await?;
        let mut footer_bytes = vec![0u8; FOOTER_SIZE];
        source.read_exact_at(len - FOOTER_SIZE as u64, &mut footer_bytes).await?;
        let footer = Footer::parse(&footer_bytes).await?;
        if footer.file_len != len {
            return Err((PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"footer", offset: len - FOOTER_SIZE as u64, detail: "file_len does not match actual source length".to_string() }).into());
        }
        Ok(Self { source, limits: limits.clone(), superblock: Superblock { header, footer }, manifest: None, symbols: Vec::new(), chunk_table: Vec::new() })
    }

    /// 2⃣ Level 2: `open_superblock` plus decoding the manifest, its symbol table (used
    /// to resolve `manifest().schema_name`), and the chunk table (if present).
    pub async fn open_manifest(source: S, limits: &PackLimits, verification: VerificationLevel) -> Result<Self, S::Error> {
        let mut this = Self::open_superblock(source, limits).await?;
        let verify_crc = verification.checks_crc();
        let manifest_seg = decode_segment_at(&this.source, this.superblock.footer.manifest_offset, &this.limits, verify_crc).await?;
        if manifest_seg.kind != semio_framework_replication_codec::KIND_MANIFEST {
            return Err((PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"manifest", offset: this.superblock.footer.manifest_offset, detail: "expected KIND_MANIFEST segment".to_string() }).into());
        }
        if manifest_seg.consumed != this.superblock.footer.manifest_len {
            return Err((PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"manifest", offset: this.superblock.footer.manifest_offset, detail: "manifest_len mismatch".to_string() }).into());
        }
        let raw = parse_raw_manifest(&manifest_seg.payload).await?;
        let symbols = if raw.symbols_span.len > 0 {
            let seg = decode_segment_at(&this.source, raw.symbols_span.offset, &this.limits, verify_crc).await?;
            if seg.kind != semio_framework_replication_codec::KIND_SYMBOLS {
                return Err((PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"symbols", offset: raw.symbols_span.offset, detail: "expected KIND_SYMBOLS segment".to_string() }).into());
            }
            decode_symbols(&seg.payload, &this.limits).await?
        } else {
            Vec::new()
        };
        let chunk_table = if raw.chunk_table_span.len > 0 {
            let seg = decode_segment_at(&this.source, raw.chunk_table_span.offset, &this.limits, verify_crc).await?;
            if seg.kind != semio_framework_replication_codec::KIND_CHUNK_TABLE {
                return Err((PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"chunk_table", offset: raw.chunk_table_span.offset, detail: "expected KIND_CHUNK_TABLE segment".to_string() }).into());
            }
            decode_chunk_table(&seg.payload, &this.limits).await?
        } else {
            Vec::new()
        };
        let manifest = resolve_manifest(&raw, &symbols)?;
        this.manifest = Some(manifest);
        this.symbols = symbols;
        this.chunk_table = chunk_table;
        Ok(this)
    }

    pub fn superblock(&self) -> &Superblock {
        &self.superblock
    }

    pub fn manifest(&self) -> Option<&Manifest> {
        self.manifest.as_ref()
    }

    /// 🧮️ Counts retained catalog slots and UTF-8 bytes before constructing document values.
    pub fn owned_catalog_bytes(&self) -> Result<u64, PackRefusal> {
        let symbol_slots = (self.symbols.capacity() as u64).checked_mul(size_of::<String>() as u64).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"catalog symbol allocation overflow"})?;
        let chunk_slots = (self.chunk_table.capacity() as u64).checked_mul(size_of::<ChunkTableEntry>() as u64).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"catalog chunk allocation overflow"})?;
        let mut bytes = symbol_slots.checked_add(chunk_slots).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"catalog allocation overflow"})?;
        for symbol in &self.symbols {
            bytes = bytes.checked_add(symbol.capacity() as u64).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"catalog text allocation overflow"})?;
        }
        if let Some(manifest) = &self.manifest {
            bytes = bytes.checked_add(manifest.schema_name.capacity() as u64).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"catalog schema allocation overflow"})?;
        }
        Ok(bytes)
    }

    /// 🔤️ Resolves a symref (index into the symbol table loaded by `open_manifest`).
    pub fn symbol(&self, symref: u64) -> Result<&str, PackRefusal> {
        self.symbols.get(symref as usize).map(String::as_str).ok_or(PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"symref", offset: symref, detail: "symref out of range".to_string() })
    }

    pub fn chunk_count(&self) -> u64 {
        self.chunk_table.len() as u64
    }

    /// 📏️ The `(offset, stored_len)` range of a chunk's on-disk (possibly compressed)
    /// payload bytes — suitable for a range-fetch (see `pack_http`) without decoding.
    pub fn chunk_range(&self, id: ChunkId) -> Result<ByteRange, PackRefusal> {
        self.chunk_table.get(id.0 as usize).map(|entry| ByteRange { offset: entry.offset, len: entry.stored_len }).ok_or(PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"chunk_id", offset: id.0 as u64, detail: "unknown chunk id".to_string() })
    }

    /// 📏️ Declared decoded bytes let callers admit ownership before reading or decompressing a chunk.
    pub fn chunk_decoded_len(&self,id:ChunkId)->Result<u64,PackRefusal>{self.chunk_table.get(id.0 as usize).map(|entry|entry.raw_len).ok_or(PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"chunk_id",offset:id.0 as u64,detail:"unknown chunk id".into()})}

    /// 🧩️ Opens an identity chunk without allocating or materializing its payload.
    pub fn identity_chunk_cursor(&self, id: ChunkId, verification: VerificationLevel) -> Result<PackIdentityChunkCursor<'_, S>, S::Error> {
        let entry = self.chunk_table.get(id.0 as usize).cloned().ok_or(PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"chunk_id", offset: id.0 as u64, detail: "unknown chunk id".to_string() })?;
        if entry.stored_len != entry.raw_len {
            return Err((PackRefusal::Malformed { kind:ValueRefusalKind::UnsupportedOwner, what: "chunk", offset: entry.offset, detail: "retained fragment cursor requires an identity chunk".to_string() }).into());
        }
        if entry.raw_len > self.limits.max_segment_len {
            return Err((PackRefusal::LimitExceeded{kind:ValueRefusalKind::WorkLimit,limit:"chunk length exceeds max_segment_len"}).into());
        }
        Ok(PackIdentityChunkCursor { source: &self.source, entry, verification, offset: 0, crc: semio_framework_replication_codec::Crc32cCursor::new(), hash: semio_framework_hash::Hasher::new(), terminal: false })
    }

    /// 🧩️ Appends one verified chunk into a previously admitted output buffer with interior source and inflation control.
    pub async fn append_chunk_controlled(&self,id:ChunkId,verification:VerificationLevel,output:&mut Vec<u8>,control:&mut NativeDecodeControl<'_>)->Result<(),S::Error>{
        control.checkpoint().map_err(PackRefusal::ValueRefusal)?;let entry=self.chunk_table.get(id.0 as usize).ok_or(PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"chunk_id",offset:id.0 as u64,detail:"unknown chunk id".into()})?;
        if entry.stored_len>self.limits.max_segment_len||entry.raw_len>self.limits.max_segment_len{return Err((PackRefusal::LimitExceeded{kind:ValueRefusalKind::WorkLimit,limit:"controlled chunk exceeds max_segment_len"}).into());}let end=entry.offset.checked_add(entry.stored_len).filter(|end|*end<=self.superblock.footer.file_len).ok_or(PackRefusal::Truncated(entry.offset))?;if end>self.source.len().await{return Err((PackRefusal::Truncated(entry.offset)).into());}
        if verification.checks_crc()&&controlled_crc(&self.source,entry.offset,entry.stored_len,control).await?!=entry.crc32{return Err((PackRefusal::ChecksumMismatch{segment:"chunk",offset:entry.offset}).into());}
        let start=output.len();let codec=if entry.stored_len==entry.raw_len{CodecId(0)}else if self.superblock.header.required_flags&REQUIRED_COMPRESSED!=0{CodecId(1)}else{CodecId(0)};append_controlled_payload(&self.source,entry.offset,entry.stored_len,entry.raw_len,codec,output,&self.limits,control).await?;
        if verification.checks_content_hash(){let bytes=&output[start..];control.begin_stage(bytes.len()).map_err(PackRefusal::ValueRefusal)?;let mut hash=semio_framework_hash::Hasher::new();for page in bytes.chunks(4096){hash.update(page);control.advance(page.len()).map_err(PackRefusal::ValueRefusal)?;}if hash.finalize().as_bytes()!=&entry.blake3{return Err((PackRefusal::ContentHashMismatch).into());}}
        Ok(())
    }

    /// 3⃣ Level 3: reads, optionally CRC-verifies (`Standard`+) and decompresses one
    /// chunk; at `Full` also verifies its blake3 content hash.
    pub async fn read_chunk(&self, id: ChunkId, verification: VerificationLevel) -> Result<Vec<u8>, S::Error> {
        let entry = self.chunk_table.get(id.0 as usize).ok_or(PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"chunk_id", offset: id.0 as u64, detail: "unknown chunk id".to_string() })?;
        if entry.stored_len > self.limits.max_segment_len || entry.raw_len > self.limits.max_segment_len {
            return Err((PackRefusal::LimitExceeded{kind:ValueRefusalKind::WorkLimit,limit:"chunk length exceeds max_segment_len"}).into());
        }
        let total_len = self.source.len().await;
        let end = entry.offset.checked_add(entry.stored_len).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"chunk range overflow"})?;
        if end > total_len {
            return Err((PackRefusal::Truncated(entry.offset)).into());
        }
        let mut stored = vec![0u8; entry.stored_len as usize];
        self.source.read_exact_at(entry.offset, &mut stored).await?;
        if verification.checks_crc() {
            let computed = crc32c(&stored);
            if computed != entry.crc32 {
                return Err((PackRefusal::ChecksumMismatch { segment: "chunk", offset: entry.offset }).into());
            }
        }
        let raw = if entry.stored_len == entry.raw_len {
            stored
        } else {
            let codec = if self.superblock.header.required_flags & REQUIRED_COMPRESSED != 0 { CodecId(1) } else { CodecId(0) };
            codec_decompress(codec, &stored, entry.raw_len, self.limits.max_segment_len).await?
        };
        if verification.checks_content_hash() {
            let hash = semio_framework_hash::hash(&raw);
            if hash.as_bytes() != &entry.blake3 {
                return Err((PackRefusal::ContentHashMismatch).into());
            }
        }
        Ok(raw)
    }

    /// 📄️ Level 3: reads and concatenates the `doc_frame_count` `KIND_DOCUMENT` segments
    /// starting at `manifest().doc_span.offset`; at `Full` also verifies the result's blake3
    /// hash against the footer's `content_hash`.
    pub async fn body_bytes(&self, verification: VerificationLevel) -> Result<Vec<u8>, S::Error> {
        let manifest = self.manifest.as_ref().ok_or_else(|| PackRefusal::RetainedMalformed{kind:ValueRefusalKind::InvariantViolated,what:"manifest",offset:0,detail:"manifest not loaded; call open_manifest first"})?;
        let catalog=self.owned_catalog_bytes()?;
        let remaining=self.limits.max_total_alloc.checked_sub(catalog).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"catalog exceeds max_total_alloc"})?;
        if manifest.uncompressed_body_len>remaining{return Err((PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"document body exceeds max_total_alloc"}).into());}
        let body_len=usize::try_from(manifest.uncompressed_body_len).map_err(|_|PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"document body exceeds address space"})?;
        let mut out = Vec::new();
        check_allocation_layout::<u8>(body_len,"document body allocation failed")?;
        out.try_reserve_exact(body_len).map_err(|_|PackRefusal::LimitExceeded{kind:ValueRefusalKind::AllocationFailed,limit:"document body allocation failed"})?;
        let mut frame_limits=self.limits.clone();frame_limits.max_total_alloc=remaining-manifest.uncompressed_body_len;
        if manifest.doc_span.len > 0 {
            let mut offset = manifest.doc_span.offset;
            let frames = manifest.doc_frame_count.max(1);
            for _ in 0..frames {
                let seg = decode_segment_at(&self.source, offset, &frame_limits, verification.checks_crc()).await?;
                if seg.kind != semio_framework_replication_codec::KIND_DOCUMENT {
                    return Err((PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"document", offset, detail: "expected KIND_DOCUMENT segment".to_string() }).into());
                }
                if seg.payload.len()>body_len-out.len(){return Err((PackRefusal::LimitExceeded{kind:ValueRefusalKind::InvalidValue,limit:"document frames exceed declared body length"}).into());}
                out.extend_from_slice(&seg.payload);
                offset=offset.checked_add(seg.consumed).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"document frame offset overflow"})?;
            }
        }
        if out.len()!=body_len{return Err((PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"document",offset:manifest.doc_span.offset,detail:"decoded body length differs from manifest".into()}).into());}
        if verification.checks_content_hash() {
            let hash = semio_framework_hash::hash(&out);
            if hash.as_bytes() != &self.superblock.footer.content_hash.0 {
                return Err((PackRefusal::ContentHashMismatch).into());
            }
        }
        Ok(out)
    }

    /// #⃣ The footer's content hash — no decode needed.
    pub fn content_hash(&self) -> ContentHash {
        self.superblock.footer.content_hash
    }
}

/// 🔎️ Standalone helper (used by `crate::content_hash`) that reads and parses only the
/// last `FOOTER_SIZE` bytes of `source`, without touching the header or any segment.
pub async fn read_footer_only<S: PackSource>(source: &S) -> Result<Footer, S::Error> {
    let len = source.len().await;
    if len < FOOTER_SIZE as u64 {
        return Err((PackRefusal::Truncated(len)).into());
    }
    let mut buf = vec![0u8; FOOTER_SIZE];
    source.read_exact_at(len - FOOTER_SIZE as u64, &mut buf).await?;
    Footer::parse(&buf).await.map_err(S::Error::from)
}
//#endregion 🔖️Reader

//#region 🔖️Recover
/// 🩹️ What a forward-scan recovery pass managed to salvage.
#[derive(Clone, Debug)]
pub struct RecoveryReport {
    pub segments_recovered: u64,
    pub bytes_recovered: u64,
    pub manifest: Option<Manifest>,
}

/// 🩺️ Forward-scans from byte `HEADER_SIZE` (right after the header), CRC-validating and
/// accumulating one segment at a time until the first invalid/truncated segment, a `KIND_END`
/// segment, or end of file — whichever comes first. Unrecognized segment kinds are accumulated,
/// not rejected. If a `KIND_MANIFEST` and (when its `schema_name` is non-empty) a matching
/// `KIND_SYMBOLS` segment were both recovered, the manifest is resolved and returned too. Used
/// when the footer itself fails to parse/validate.
pub async fn recover<S: PackSource>(source: &S, limits: &PackLimits) -> Result<RecoveryReport, S::Error> {
    let len = source.len().await;
    if len < HEADER_SIZE as u64 {
        return Err((PackRefusal::Truncated(len)).into());
    }
    let mut header_bytes = [0u8; HEADER_SIZE];
    source.read_exact_at(0, &mut header_bytes).await?;
    Header::parse(&header_bytes).await?;

    let mut offset = HEADER_SIZE as u64;
    let mut segments_recovered = 0u64;
    let mut bytes_recovered = 0u64;
    let mut found: Vec<(u8, Vec<u8>)> = Vec::new();
    while offset < len {
        match decode_segment_at(source, offset, limits, true).await {
            Ok(seg) => {
                segments_recovered += 1;
                bytes_recovered += seg.consumed;
                let is_end = seg.kind == semio_framework_replication_codec::KIND_END;
                offset += seg.consumed;
                found.push((seg.kind, seg.payload));
                if is_end {
                    break;
                }
            }
            Err(_) => break,
        }
    }

    let mut symbols = Vec::new();
    if let Some((_, payload)) = found.iter().find(|(kind, _)| *kind == semio_framework_replication_codec::KIND_SYMBOLS) {
        if let Ok(decoded) = decode_symbols(payload, limits).await {
            symbols = decoded;
        }
    }
    let mut manifest = None;
    if let Some((_, payload)) = found.iter().find(|(kind, _)| *kind == semio_framework_replication_codec::KIND_MANIFEST) {
        if let Ok(raw) = parse_raw_manifest(payload).await {
            manifest = resolve_manifest(&raw, &symbols).ok();
        }
    }

    Ok(RecoveryReport { segments_recovered, bytes_recovered, manifest })
}
//#endregion 🔖️Recover
