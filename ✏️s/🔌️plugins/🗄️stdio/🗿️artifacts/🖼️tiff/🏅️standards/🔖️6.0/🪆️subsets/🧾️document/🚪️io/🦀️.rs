//! 🚪️ IO stdio.tiff (6.0/✳️any) — registration now flows through 🎹️composer::register
//! (called once from 🔌️plugin/🔧️setup via ⚙️engine::register), not per-leaf register().
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v6_0::subsets::document::io::TiffAnalyzer;
    use crate::TiffSnapshot;
    use {semio_framework_plugin::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::ComposeError,semio_framework_plugin::ComposeSource,semio_framework_plugin::Composition,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.tiff", standard: StandardId("6.0"), subset: SubsetId("*") };
    const DEP_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };

    pub struct TiffComposerComposition;

    impl ArtifactComposition for TiffComposerComposition {
        type Snapshot = TiffSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_BINARY]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            // 🌱 Every listed read dialect's payload is raw text/bytes that this artifact's own
            // analyzer already round-trips through `store::Document{Dsl,Pack}` -- including bytes
            // claiming a dependency's dialect, since (for a single-standard DAG-adjacent dependency
            // like binary) that payload IS the same byte/text shape `analyze` already accepts.
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT || s.dialect == DEP_BINARY)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "TiffComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = TiffAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "TiffComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

// 🐜️ `⚙️engine/` dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): the
// real TIFF codec — full IFD-chain walk (II/MM), generic typed tag/value decode for every TIFF
// 6.0 field type, uncompressed + PackBits strip pixel decode — relocated here verbatim
// (destination rule 2: codecs → `🚪️io/`; rule 6: pure format algorithms with no snapshot
// dependency stay WITH the codec here, since they're TIFF-specific, not artifact-independent).
//
// **Decode** is fully generic: it walks the WHOLE `next IFD offset` chain (not just the first
// IFD) and decodes every entry's typed value via [`TiffFieldType`]/[`TiffValues`], regardless of
// whether the codec specially interprets that tag id — this is the "unknown tags stay typed-raw"
// completeness promise. Pixel decode itself only runs against IFD 0 (documented normalization: a
// multi-IFD file's later IFDs — e.g. thumbnails — keep their real tags but don't get a second
// decoded raster).
//
// **Encode** (ticket 26/08/23/END-TO-END-TESTING-REFACTOR, wave 8) walks the WHOLE `ifds` vector
// and writes a REAL `next IFD offset` chain (🚫 `MultiIfdEncodeScopeNote`): every IFD `snap.ifds`
// carries is re-serialized, in order, each ending with a 4-byte offset to the next directory and
// the last with `0` — mirroring `decode_tiff`'s own chain walk exactly, so `InsertIfd`/`RemoveIfd`
// are genuinely observable in the bytes, not just in memory. `ifds[0]` alone gets the baseline
// strip/geometry tags freshly computed from `pixels` (`Compression`/`PhotometricInterpretation`/
// `BitsPerSample`/chunky/single-strip layout canonicalized, exactly like png's encoder
// canonicalizes color type/bit depth/interlace); every OTHER tag `ifds[0]` carries verbatim (so a
// caller's `SetTag`-set metadata genuinely round-trips). `byte_order` itself DOES round-trip.
// `ifds[1..]` carry every entry verbatim EXCEPT the three strip tags, which are recomputed from the
// directory's OWN raw strip bytes — `TiffIfd::pixels`, added 2026-08-25 by ticket
// 26/08/23/END-TO-END-TESTING-REFACTOR wave 17, exactly the "per-IFD raw-strip field this snapshot
// does not have" the previous revision of this note flagged. `decode_tiff` fills it for every
// directory beyond the first (verbatim strips, no interpretation, so an undecodable photometric
// layout still round-trips); `encode_tiff_with` writes those strips back and emits the
// `StripOffsets`/`RowsPerStrip`/`StripByteCounts` triple TIFF6 §Baseline REQUIRES of a
// strip-organised directory, `RowsPerStrip` forced to `ImageLength` because this writer always
// re-lays a directory out as one combined strip. Before that field existed the two pointer tags
// were omitted for every IFD beyond the first, which meant a real multi-page file lost every page
// after the first on every single round trip — measured by nothing, since the semantic projection
// only decodes IFD 0's raster. A directory carrying no strip bytes is still metadata-only and
// still gets no invented pointer. `TiffEngine` (zero
// construction sites) and the dead `register`/`register_pilot_languages`/
// `register_artifact_inferences` cluster (superseded by `declaration()` in the artifact root,
// zero real callers) were deleted outright. `blank_tiff_snapshot`/`demo_tiff_snapshot` moved to
// `../🧬️schema`.
use crate::schema::snapshot::{
    TiffBinary32, TiffWord64, TiffFieldType, TiffIfd, TiffSnapshot, TiffSampleBlock, TiffTag, TiffValues, TAG_BITS_PER_SAMPLE, TAG_IMAGE_LENGTH, TAG_IMAGE_WIDTH, TAG_PHOTOMETRIC, TAG_SAMPLES_PER_PIXEL,
};
use crate::STDIO_TIFF_DOCUMENT_SCHEMA;

//#region ByteOrder
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TiffByteOrder { #[default] LittleEndian, BigEndian }
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TiffCompression { #[default] None, PackBits, Lzw, Deflate, ModifiedHuffman, Group3, Group4 }
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TiffNativeLayout { #[default] SingleStrip, Strips { rows: u32 }, Tiles { width: u32, height: u32 } }
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TiffNativeOptions { pub byte_order: TiffByteOrder, pub compression: TiffCompression, pub layout: TiffNativeLayout }
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum NativeStorageKind { #[default] None, Strips, Tiles }
#[derive(Clone, Debug, PartialEq, Eq)]
struct NativeStorage { kind: NativeStorageKind, offsets_kind: TiffFieldType, byte_counts_kind: TiffFieldType, chunks: Vec<Vec<u8>> }
impl Default for NativeStorage { fn default()->Self { Self {kind:NativeStorageKind::None,offsets_kind:TiffFieldType::Long,byte_counts_kind:TiffFieldType::Long,chunks:Vec::new()} } }
#[derive(Clone, Debug, PartialEq)]
struct NativeIfd { entries: Vec<TiffTag>, storage: NativeStorage }
#[derive(Clone, Debug, PartialEq)]
struct NativeSnapshot { schema:String, byte_order:TiffByteOrder, ifds:Vec<NativeIfd> }
pub const TAG_COMPRESSION:u16=259;
pub const TAG_STRIP_OFFSETS:u16=273;
pub const TAG_ROWS_PER_STRIP:u16=278;
pub const TAG_STRIP_BYTE_COUNTS:u16=279;
pub const TAG_TILE_WIDTH:u16=322;
pub const TAG_TILE_LENGTH:u16=323;
pub const TAG_TILE_OFFSETS:u16=324;
pub const TAG_TILE_BYTE_COUNTS:u16=325;
pub(crate) trait NativeFieldType {
 fn from_u16(code:u16)->Result<Self,String> where Self:Sized;
 fn to_u16(self)->u16;
 fn element_size(self)->usize;
}
impl NativeFieldType for TiffFieldType {
 fn from_u16(code:u16)->Result<Self,String>{Ok(match code{1=>Self::Byte,2=>Self::Ascii,3=>Self::Short,4=>Self::Long,5=>Self::Rational,6=>Self::SByte,7=>Self::Undefined,8=>Self::SShort,9=>Self::SLong,10=>Self::SRational,11=>Self::Float,12=>Self::Double,_=>return Err("tiff: unknown native field type".into())})}
 fn to_u16(self)->u16{match self{Self::Byte=>1,Self::Ascii=>2,Self::Short=>3,Self::Long=>4,Self::Rational=>5,Self::SByte=>6,Self::Undefined=>7,Self::SShort=>8,Self::SLong=>9,Self::SRational=>10,Self::Float=>11,Self::Double=>12}}
 fn element_size(self)->usize{match self{Self::Byte|Self::Ascii|Self::SByte|Self::Undefined=>1,Self::Short|Self::SShort=>2,Self::Long|Self::SLong|Self::Float=>4,Self::Rational|Self::SRational|Self::Double=>8}}
}
fn native_value_count(values:&TiffValues)->Result<u32,String>{let count=match values{TiffValues::Ascii(texts)=>texts.iter().try_fold(0usize,|sum,text|sum.checked_add(text.len())?.checked_add(1)).ok_or("tiff: ASCII native count overflow")?,TiffValues::Byte(v)|TiffValues::Undefined(v)=>v.len(),TiffValues::Short(v)=>v.len(),TiffValues::Long(v)=>v.len(),TiffValues::Rational(v)=>v.len(),TiffValues::SByte(v)=>v.len(),TiffValues::SShort(v)=>v.len(),TiffValues::SLong(v)=>v.len(),TiffValues::SRational(v)=>v.len(),TiffValues::Float(v)=>v.len(),TiffValues::Double(v)=>v.len()};u32::try_from(count).map_err(|_|"tiff: native value count exceeds classic TIFF width".into())}
fn read_ascii(bytes:&[u8])->Result<Vec<String>,String>{if bytes.is_empty(){return Ok(Vec::new())}if bytes.last()!=Some(&0)||!bytes.is_ascii(){return Err("tiff: native ASCII metadata is invalid or unterminated".into())}bytes[..bytes.len()-1].split(|byte|*byte==0).map(|text|String::from_utf8(text.to_vec()).map_err(|error|error.to_string())).collect()}

#[derive(Clone, Copy)]
enum Endian {
    Little,
    Big,
}

impl Endian {
    fn u16(self, b: &[u8]) -> u16 {
        match self {
            Endian::Little => u16::from_le_bytes([b[0], b[1]]),
            Endian::Big => u16::from_be_bytes([b[0], b[1]]),
        }
    }
    fn u32(self, b: &[u8]) -> u32 {
        match self {
            Endian::Little => u32::from_le_bytes([b[0], b[1], b[2], b[3]]),
            Endian::Big => u32::from_be_bytes([b[0], b[1], b[2], b[3]]),
        }
    }
    fn u64(self, b: &[u8]) -> u64 {
        match self {
            Endian::Little => u64::from_le_bytes(b.try_into().expect("8 bytes")),
            Endian::Big => u64::from_be_bytes(b.try_into().expect("8 bytes")),
        }
    }
}

fn read_u16(data: &[u8], pos: usize, e: Endian) -> Result<u16, String> {
    data.get(pos..pos + 2).map(|s| e.u16(s)).ok_or_else(|| "tiff: truncated (u16)".into())
}
fn read_u32(data: &[u8], pos: usize, e: Endian) -> Result<u32, String> {
    data.get(pos..pos + 4).map(|s| e.u32(s)).ok_or_else(|| "tiff: truncated (u32)".into())
}

fn write_u16(out: &mut Vec<u8>, v: u16, bo: TiffByteOrder) {
    match bo {
        TiffByteOrder::LittleEndian => out.extend_from_slice(&v.to_le_bytes()),
        TiffByteOrder::BigEndian => out.extend_from_slice(&v.to_be_bytes()),
    }
}
fn write_u32(out: &mut Vec<u8>, v: u32, bo: TiffByteOrder) {
    match bo {
        TiffByteOrder::LittleEndian => out.extend_from_slice(&v.to_le_bytes()),
        TiffByteOrder::BigEndian => out.extend_from_slice(&v.to_be_bytes()),
    }
}
fn write_u64(out: &mut Vec<u8>, v: u64, bo: TiffByteOrder) {
    match bo {
        TiffByteOrder::LittleEndian => out.extend_from_slice(&v.to_le_bytes()),
        TiffByteOrder::BigEndian => out.extend_from_slice(&v.to_be_bytes()),
    }
}
//#endregion ByteOrder

//#region IfdRead
struct RawEntry {
    tag: u16,
    typ: u16,
    count: u32,
    value_field: [u8; 4],
}

struct RawIfd {
    entries: Vec<RawEntry>,
    next: u32,
}

/// 📖️ Walks one IFD: 2-byte entry count, N x 12-byte entries, 4-byte offset to the next IFD.
fn read_ifd_raw(data: &[u8], ifd_off: usize, e: Endian) -> Result<RawIfd, String> {
    let count = read_u16(data, ifd_off, e)? as usize;
    let mut entries = Vec::with_capacity(count);
    let mut pos = ifd_off + 2;
    for _ in 0..count {
        if pos + 12 > data.len() {
            return Err("tiff: truncated IFD entry".into());
        }
        let tag = read_u16(data, pos, e)?;
        let typ = read_u16(data, pos + 2, e)?;
        let cnt = read_u32(data, pos + 4, e)?;
        let mut vf = [0u8; 4];
        vf.copy_from_slice(&data[pos + 8..pos + 12]);
        entries.push(RawEntry { tag, typ, count: cnt, value_field: vf });
        pos += 12;
    }
    let next = read_u32(data, pos, e)?;
    Ok(RawIfd { entries, next })
}

/// 🔗️ Walks the WHOLE `next IFD offset` chain starting at `first_off` (0 = none). Cycle-
/// guarded so a malformed/adversarial chain errors instead of looping forever.
fn read_ifd_chain(data: &[u8], first_off: usize, e: Endian) -> Result<Vec<RawIfd>, String> {
    let mut out = Vec::new();
    let mut off = first_off;
    let mut seen = std::collections::HashSet::new();
    while off != 0 {
        if !seen.insert(off) {
            return Err("tiff: IFD offset cycle detected".into());
        }
        let raw = read_ifd_raw(data, off, e)?;
        let next = raw.next as usize;
        out.push(raw);
        off = next;
    }
    Ok(out)
}

/// 🔢️ Reads one entry's real typed value, resolving the inline-vs-offset rule (TIFF6 §2: the
/// value is stored inline in the 4-byte field if `element_size * count <= 4`, else the field
/// holds a file offset to the values) GENERICALLY for all 12 field types.
fn read_tag_values(data: &[u8], entry: &RawEntry, e: Endian, kind: TiffFieldType) -> Result<TiffValues, String> {
    let elem = kind.element_size();
    let count = entry.count as usize;
    let total = elem * count;
    let owned;
    let src: &[u8] = if total <= 4 {
        &entry.value_field[..total]
    } else {
        let off = e.u32(&entry.value_field) as usize;
        owned = data.get(off..off + total).ok_or("tiff: tag value offset out of range")?;
        owned
    };
    Ok(match kind {
        TiffFieldType::Byte => TiffValues::Byte(src.to_vec()),
        TiffFieldType::Ascii => TiffValues::Ascii(read_ascii(src)?),
        TiffFieldType::Short => TiffValues::Short((0..count).map(|i| e.u16(&src[i * 2..i * 2 + 2])).collect()),
        TiffFieldType::Long => TiffValues::Long((0..count).map(|i| e.u32(&src[i * 4..i * 4 + 4])).collect()),
        TiffFieldType::Rational => TiffValues::Rational((0..count).map(|i| (e.u32(&src[i * 8..i * 8 + 4]), e.u32(&src[i * 8 + 4..i * 8 + 8]))).collect()),
        TiffFieldType::SByte => TiffValues::SByte(src.iter().map(|&b| b as i8).collect()),
        TiffFieldType::Undefined => TiffValues::Undefined(src.to_vec()),
        TiffFieldType::SShort => TiffValues::SShort((0..count).map(|i| e.u16(&src[i * 2..i * 2 + 2]) as i16).collect()),
        TiffFieldType::SLong => TiffValues::SLong((0..count).map(|i| e.u32(&src[i * 4..i * 4 + 4]) as i32).collect()),
        TiffFieldType::SRational => TiffValues::SRational((0..count).map(|i| (e.u32(&src[i * 8..i * 8 + 4]) as i32, e.u32(&src[i * 8 + 4..i * 8 + 8]) as i32)).collect()),
        TiffFieldType::Float => TiffValues::Float((0..count).map(|i| TiffBinary32 { bits: e.u32(&src[i * 4..i * 4 + 4]) }).collect()),
        TiffFieldType::Double => TiffValues::Double((0..count).map(|i| TiffWord64::from_word(e.u64(&src[i * 8..i * 8 + 8]))).collect()),
    })
}
//#endregion IfdRead

//#region TagLookup
fn tag_values(ifd: &NativeIfd, tag: u16) -> Option<&TiffValues> {
    ifd.entries.iter().find(|t| t.tag == tag).map(|t| &t.values)
}
fn tag_u32_list(ifd: &NativeIfd, tag: u16) -> Vec<u32> {
    match tag_values(ifd, tag) {
        Some(TiffValues::Short(v)) => v.iter().map(|&x| x as u32).collect(),
        Some(TiffValues::Long(v)) => v.clone(),
        _ => Vec::new(),
    }
}
fn tag_u32(ifd: &NativeIfd, tag: u16) -> Option<u32> {
    tag_u32_list(ifd, tag).first().copied()
}
//#endregion TagLookup

//#region PackBits
/// 📦 PackBits (TIFF compression scheme 32773, TIFF6 §9): signed control byte `n` — `n >= 0`
/// copies the next `n+1` literal bytes; `n < 0` (and `n != -128`) repeats the next byte
/// `1-n` times; `n == -128` is a no-op.
fn packbits_decode(data: &[u8], expected_len: usize) -> Result<Vec<u8>, String> {
    let mut out = Vec::with_capacity(expected_len);
    let mut i = 0usize;
    while i < data.len() && out.len() < expected_len {
        let n = data[i] as i8;
        i += 1;
        if n >= 0 {
            let count = n as usize + 1;
            let end = i + count;
            if end > data.len() {
                return Err("tiff: packbits literal run overruns strip".into());
            }
            out.extend_from_slice(&data[i..end]);
            i = end;
        } else if n != -128 {
            let count = (1 - n as i32) as usize;
            if i >= data.len() {
                return Err("tiff: packbits repeat run missing byte".into());
            }
            let b = data[i];
            i += 1;
            out.extend(std::iter::repeat_n(b, count));
        }
    }
    if out.len() != expected_len {
        return Err(format!("tiff: packbits decoded length mismatch (got {}, expected {expected_len})", out.len()));
    }
    Ok(out)
}

fn packbits_encode(data: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut i = 0usize;
    let n = data.len();
    while i < n {
        let mut run_len = 1usize;
        while i + run_len < n && data[i + run_len] == data[i] && run_len < 128 {
            run_len += 1;
        }
        if run_len >= 2 {
            out.push((1i32 - run_len as i32) as u8);
            out.push(data[i]);
            i += run_len;
        } else {
            let start = i;
            let mut lit_len = 1usize;
            i += 1;
            while i < n && lit_len < 128 {
                if i + 1 < n && data[i] == data[i + 1] {
                    break;
                }
                lit_len += 1;
                i += 1;
            }
            out.push((lit_len - 1) as u8);
            out.extend_from_slice(&data[start..start + lit_len]);
        }
    }
    out
}
//#endregion PackBits


//#region Decode
/// 🚫 CompressionScopeNote: only uncompressed(1)/PackBits(32773) are decoded for real —
/// LZW(5)/Deflate(8)/CCITT(2/3/4)/others deliberately fail rather than fabricate pixels.
/// 🧵 Captures each authored strip or tile as an independent canonical chunk.
fn read_storage(data: &[u8], ifd: &NativeIfd) -> Result<NativeStorage, String> {
    let strips = tag_u32_list(ifd, TAG_STRIP_OFFSETS);
    let tiles = tag_u32_list(ifd, TAG_TILE_OFFSETS);
    if !strips.is_empty() && !tiles.is_empty() {
        return Err("tiff: one IFD cannot own both strips and tiles".into());
    }
    let (storage_kind, offset_tag, count_tag, offsets) = if !tiles.is_empty() {
        (NativeStorageKind::Tiles, TAG_TILE_OFFSETS, TAG_TILE_BYTE_COUNTS, tiles)
    } else if !strips.is_empty() {
        (NativeStorageKind::Strips, TAG_STRIP_OFFSETS, TAG_STRIP_BYTE_COUNTS, strips)
    } else {
        return Ok(NativeStorage::default());
    };
    let offset_entry = ifd.entries.iter().find(|entry| entry.tag == offset_tag).ok_or("tiff: missing storage offsets")?;
    let count_entry = ifd.entries.iter().find(|entry| entry.tag == count_tag).ok_or("tiff: missing storage byte counts")?;
    let counts = tag_u32_list(ifd, count_tag);
    if counts.len() != offsets.len() {
        return Err("tiff: storage offset/count cardinality mismatch".into());
    }
    let offsets_kind = offset_entry.values.kind();
    let byte_counts_kind = count_entry.values.kind();
    if !matches!(offsets_kind, TiffFieldType::Short | TiffFieldType::Long) || !matches!(byte_counts_kind, TiffFieldType::Short | TiffFieldType::Long) {
        return Err("tiff: storage offsets and byte counts must be SHORT or LONG".into());
    }
    let mut chunks = Vec::with_capacity(offsets.len());
    for (offset, count) in offsets.into_iter().zip(counts) {
        let start = usize::try_from(offset).map_err(|_| "tiff: storage offset width")?;
        let length = usize::try_from(count).map_err(|_| "tiff: storage byte-count width")?;
        chunks.push(data.get(start..start.checked_add(length).ok_or("tiff: storage range overflow")?).ok_or("tiff: storage chunk truncated")?.to_vec());
    }
    Ok(NativeStorage { kind: storage_kind, offsets_kind, byte_counts_kind, chunks })
}

#[derive(Clone, Debug, PartialEq)]
pub struct TiffRgbaPage {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TiffPngPage {
    pub width: u32,
    pub height: u32,
    pub bytes: Vec<u8>,
}


fn read_native_uncontrolled(data: &[u8]) -> Result<NativeSnapshot, String> {
    if data.len() < 8 {
        return Err("tiff: truncated header".into());
    }
    let (e, byte_order) = match &data[0..2] {
        b"II" => (Endian::Little, TiffByteOrder::LittleEndian),
        b"MM" => (Endian::Big, TiffByteOrder::BigEndian),
        _ => return Err("tiff: bad byte-order mark".into()),
    };
    if read_u16(data, 2, e)? != 42 {
        return Err("tiff: bad magic number".into());
    }
    let first_off = read_u32(data, 4, e)? as usize;
    let raw_ifds = read_ifd_chain(data, first_off, e)?;
    if raw_ifds.is_empty() {
        return Err("tiff: no IFD present".into());
    }

    let mut ifds = Vec::with_capacity(raw_ifds.len());
    for raw in &raw_ifds {
        let mut entries = Vec::with_capacity(raw.entries.len());
        for entry in &raw.entries {
            let kind = TiffFieldType::from_u16(entry.typ)?;
            let values = read_tag_values(data, entry, e, kind)?;
            entries.push(TiffTag { tag: entry.tag, values });
        }
        entries.sort_by_key(|t| t.tag); // TIFF6 §2: entries "must be sorted in ascending order by Tag".
        ifds.push(NativeIfd { entries, storage: NativeStorage::default() });
    }

    for ifd in &mut ifds {
        ifd.storage = read_storage(data, ifd)?;
        ifd.entries.retain(|entry| !matches!(entry.tag, TAG_STRIP_OFFSETS | TAG_STRIP_BYTE_COUNTS | TAG_TILE_OFFSETS | TAG_TILE_BYTE_COUNTS));
    }
    Ok(NativeSnapshot { schema: STDIO_TIFF_DOCUMENT_SCHEMA.into(), byte_order, ifds })
}
//#endregion Decode

//#region Encode
fn value_bytes(values: &TiffValues, bo: TiffByteOrder) -> Vec<u8> {
    let mut out = Vec::new();
    match values {
        TiffValues::Byte(v) | TiffValues::Undefined(v) => out.extend_from_slice(v),
        TiffValues::Ascii(texts) => { for text in texts { out.extend_from_slice(text.as_bytes()); out.push(0); } },
        TiffValues::Short(v) => v.iter().for_each(|&x| write_u16(&mut out, x, bo)),
        TiffValues::Long(v) => v.iter().for_each(|&x| write_u32(&mut out, x, bo)),
        TiffValues::Rational(v) => v.iter().for_each(|&(n, d)| { write_u32(&mut out, n, bo); write_u32(&mut out, d, bo); }),
        TiffValues::SByte(v) => out.extend(v.iter().map(|&x| x as u8)),
        TiffValues::SShort(v) => v.iter().for_each(|&x| write_u16(&mut out, x as u16, bo)),
        TiffValues::SLong(v) => v.iter().for_each(|&x| write_u32(&mut out, x as u32, bo)),
        TiffValues::SRational(v) => v.iter().for_each(|&(n, d)| { write_u32(&mut out, n as u32, bo); write_u32(&mut out, d as u32, bo); }),
        TiffValues::Float(v) => v.iter().for_each(|x| write_u32(&mut out, x.bits, bo)),
        TiffValues::Double(v) => v.iter().for_each(|x| write_u64(&mut out, x.word(), bo)),
    }
    out
}

fn dir_size(count: usize) -> Result<usize, String> {
    count.checked_mul(12).and_then(|size| size.checked_add(6)).ok_or_else(|| "tiff: directory size overflow".into())
}

fn values_owned_bytes(value:&TiffValues)->Result<usize,semio_framework_value::ValueError>{
 use semio_framework_value::{ValueError,ValueRefusalKind};let overflow=||ValueError::new(ValueRefusalKind::OwnershipLimit,"TIFF owned metadata allocation overflow");
 let bytes=match value{TiffValues::Byte(v)|TiffValues::Undefined(v)=>v.len(),TiffValues::Ascii(v)=>v.iter().try_fold(v.len().checked_mul(std::mem::size_of::<String>()).ok_or_else(overflow)?,|n,text|n.checked_add(text.len()).ok_or_else(overflow))?,TiffValues::SByte(v)=>v.len(),TiffValues::Short(v)=>v.len()*2,TiffValues::SShort(v)=>v.len()*2,TiffValues::Long(v)=>v.len()*4,TiffValues::SLong(v)=>v.len()*4,TiffValues::Float(v)=>v.len()*4,TiffValues::Double(v)=>v.len()*8,TiffValues::Rational(v)=>v.len()*8,TiffValues::SRational(v)=>v.len()*8};Ok(bytes)
}

fn out_of_line_size(entries: &[TiffTag], bo: TiffByteOrder) -> Result<usize, String> {
    entries.iter().try_fold(0usize, |total, tag| {
        let length = native_value_count(&tag.values)? as usize * tag.values.kind().element_size();
        total.checked_add(if length <= 4 { 0 } else { length + length % 2 }).ok_or_else(|| "tiff: value layout overflow".into())
    })
}

fn storage_values(kind: TiffFieldType, values: &[u32], label: &str) -> Result<TiffValues, String> {
    match kind {
        TiffFieldType::Short => values.iter().map(|value| u16::try_from(*value).map_err(|_| format!("tiff: {label} exceeds SHORT width"))).collect::<Result<Vec<_>, _>>().map(TiffValues::Short),
        TiffFieldType::Long => Ok(TiffValues::Long(values.to_vec())),
        _ => Err(format!("tiff: {label} type must be SHORT or LONG")),
    }
}

fn storage_tags(ifd: &NativeIfd) -> Result<Vec<TiffTag>, String> {
    if ifd.storage.kind == NativeStorageKind::None {
        if !ifd.storage.chunks.is_empty() {
            return Err("tiff: none storage cannot own chunks".into());
        }
        return Ok(Vec::new());
    }
    if ifd.storage.chunks.is_empty() {
        return Err("tiff: image storage requires at least one chunk".into());
    }
    let (offset_tag, count_tag) = match ifd.storage.kind {
        NativeStorageKind::Strips => (TAG_STRIP_OFFSETS, TAG_STRIP_BYTE_COUNTS),
        NativeStorageKind::Tiles => (TAG_TILE_OFFSETS, TAG_TILE_BYTE_COUNTS),
        NativeStorageKind::None => unreachable!(),
    };
    let zeroes = vec![0; ifd.storage.chunks.len()];
    let counts = ifd.storage.chunks.iter().map(|chunk| u32::try_from(chunk.len()).map_err(|_| "tiff: storage chunk exceeds TIFF6 byte-count width".to_string())).collect::<Result<Vec<_>, _>>()?;
    Ok(vec![
        TiffTag { tag: offset_tag, values: storage_values(ifd.storage.offsets_kind, &zeroes, "storage offset")? },
        TiffTag { tag: count_tag, values: storage_values(ifd.storage.byte_counts_kind, &counts, "storage byte count")? },
    ])
}

fn encode_tiff_preserving_storage(snapshot: &NativeSnapshot,c:&mut semio_framework_value::NativeEncodeControl<'_>) -> Result<Vec<u8>, String> {
    if snapshot.ifds.is_empty() {
        return Err("tiff: encode requires at least one IFD".into());
    }
    let mut entries_per_ifd = c.allocate_vec::<Vec<TiffTag>>(snapshot.ifds.len()).map_err(|e|e.to_string())?;
    for ifd in &snapshot.ifds {
        let mut entries=c.allocate_vec::<TiffTag>(ifd.entries.len()+2).map_err(|e|e.to_string())?;c.begin_stage(ifd.entries.len()).map_err(|e|e.to_string())?;for tag in &ifd.entries{c.charge(values_owned_bytes(&tag.values).map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;entries.push(tag.clone());c.step().map_err(|e|e.to_string())?;}c.charge(2*std::mem::size_of::<TiffTag>()+ifd.storage.chunks.len()*16).map_err(|e|e.to_string())?;
        entries.extend(storage_tags(ifd)?);
        entries.sort_by_key(|tag| tag.tag);
        if entries.windows(2).any(|pair| pair[0].tag == pair[1].tag) {
            return Err("tiff: duplicate IFD tag".into());
        }
        u16::try_from(entries.len()).map_err(|_| "tiff: directory exceeds TIFF6 entry-count width")?;
        entries_per_ifd.push(entries);
    }

    let mut cursor = 8usize;
    let mut directory_offsets=c.allocate_vec::<usize>(entries_per_ifd.len()).map_err(|e|e.to_string())?;
    for entries in &entries_per_ifd {
        directory_offsets.push(cursor);
        cursor = cursor.checked_add(dir_size(entries.len())?).and_then(|value| value.checked_add(out_of_line_size(entries, snapshot.byte_order).ok()?)).ok_or("tiff: directory layout overflow")?;
    }
    for (ifd_index, ifd) in snapshot.ifds.iter().enumerate() {
        if ifd.storage.kind == NativeStorageKind::None {
            continue;
        }
        let offset_tag = match ifd.storage.kind { NativeStorageKind::Strips => TAG_STRIP_OFFSETS, NativeStorageKind::Tiles => TAG_TILE_OFFSETS, NativeStorageKind::None => unreachable!() };
        let mut offsets=c.allocate_vec::<u32>(ifd.storage.chunks.len()).map_err(|e|e.to_string())?;
        for chunk in &ifd.storage.chunks {
            offsets.push(u32::try_from(cursor).map_err(|_| "tiff: storage offset exceeds TIFF6 width")?);
            cursor = cursor.checked_add(chunk.len()).ok_or("tiff: storage layout overflow")?;
        }
        let entry = entries_per_ifd[ifd_index].iter_mut().find(|entry| entry.tag == offset_tag).ok_or("tiff: derived storage offset tag missing")?;
        entry.values = storage_values(ifd.storage.offsets_kind, &offsets, "storage offset")?;
    }
    u32::try_from(cursor).map_err(|_| "tiff: file exceeds TIFF6 offset width")?;

    let mut out=c.allocate_vec::<u8>(cursor).map_err(|e|e.to_string())?;c.begin_stage(cursor).map_err(|e|e.to_string())?;
    out.extend_from_slice(match snapshot.byte_order { TiffByteOrder::LittleEndian => b"II", TiffByteOrder::BigEndian => b"MM" });
    write_u16(&mut out, 42, snapshot.byte_order);
    write_u32(&mut out, if snapshot.ifds.is_empty(){0}else{8}, snapshot.byte_order);c.advance(8).map_err(|e|e.to_string())?;
    for (ifd_index, entries) in entries_per_ifd.iter().enumerate() {
        let start=out.len();
        if out.len() != directory_offsets[ifd_index] {
            return Err("tiff: directory layout invariant".into());
        }
        write_u16(&mut out, u16::try_from(entries.len()).map_err(|_| "tiff: entry-count width")?, snapshot.byte_order);
        let mut value_cursor = directory_offsets[ifd_index].checked_add(dir_size(entries.len())?).ok_or("tiff: value layout overflow")?;
        for tag in entries {
            write_u16(&mut out, tag.tag, snapshot.byte_order);
            write_u16(&mut out, tag.values.kind().to_u16(), snapshot.byte_order);
            write_u32(&mut out, native_value_count(&tag.values)?, snapshot.byte_order);
            c.charge(native_value_count(&tag.values)? as usize*tag.values.kind().element_size()).map_err(|e|e.to_string())?;let bytes = value_bytes(&tag.values, snapshot.byte_order);
            if bytes.len() <= 4 {
                let mut inline = [0; 4];
                inline[..bytes.len()].copy_from_slice(&bytes);
                out.extend_from_slice(&inline);
            } else {
                write_u32(&mut out, u32::try_from(value_cursor).map_err(|_| "tiff: value offset width")?, snapshot.byte_order);
                value_cursor = value_cursor.checked_add(bytes.len() + bytes.len() % 2).ok_or("tiff: value layout overflow")?;
            }
        }
        write_u32(&mut out, directory_offsets.get(ifd_index + 1).copied().map(u32::try_from).transpose().map_err(|_| "tiff: next IFD offset width")?.unwrap_or(0), snapshot.byte_order);
        for tag in entries {
            c.charge(native_value_count(&tag.values)? as usize*tag.values.kind().element_size()).map_err(|e|e.to_string())?;let bytes = value_bytes(&tag.values, snapshot.byte_order);
            if bytes.len() > 4 {
                out.extend_from_slice(&bytes);
                if bytes.len() % 2 == 1 { out.push(0); }
            }
        }
        c.advance(out.len()-start).map_err(|e|e.to_string())?;
    }
    for ifd in &snapshot.ifds {
        for chunk in &ifd.storage.chunks {
            for span in chunk.chunks(65536){out.extend_from_slice(span);c.advance(span.len()).map_err(|e|e.to_string())?;}
        }
    }
    if out.len() != cursor {
        return Err("tiff: final output length invariant".into());
    }
    Ok(out)
}

fn write_native(snapshot: &NativeSnapshot,c:&mut semio_framework_value::NativeEncodeControl<'_>) -> Result<Vec<u8>, String> {
    encode_tiff_preserving_storage(snapshot,c)
}


/// 📖️ Admits native TIFF once into exact owned sample identities.
pub fn decode_tiff(data:&[u8])->Result<TiffSnapshot,String>{
 controlled_decoding::decode_tiff_controlled(data,&mut semio_framework_value::NativeDecodeControl::new(usize::MAX,&mut |_|true),usize::MAX).map_err(|error|error.message)
}
/// 📤️ Emits an owned TIFF page under explicit physical policy.
pub fn encode_tiff_with(snapshot:&TiffSnapshot,options:TiffNativeOptions)->Result<Vec<u8>,String>{let mut progress=|_|true;let mut c=semio_framework_value::NativeEncodeControl::new(usize::MAX,&mut progress);let native=owned_samples::lower(snapshot,options,&mut c).map_err(|e|e.to_string())?;write_native(&native,&mut c)}
pub fn encode_tiff(snapshot:&TiffSnapshot)->Result<Vec<u8>,String>{encode_tiff_with(snapshot,TiffNativeOptions::default())}
pub fn encode_tiff_packbits(snapshot:&TiffSnapshot)->Result<Vec<u8>,String>{encode_tiff_with(snapshot,TiffNativeOptions{compression:TiffCompression::PackBits,..Default::default()})}
/// 🖼️ Computes an ephemeral display projection from exact owned sample words.
pub fn decode_tiff_page_rgba(snapshot:&TiffSnapshot,ifd_index:usize)->Result<TiffRgbaPage,String>{
 snapshot.validate()?;let ifd=snapshot.ifds.get(ifd_index).ok_or("tiff: page index outside owned document")?;
 let block=ifd.blocks.first().ok_or("tiff: page has no owned samples")?;let channels=block.channels as usize;let mut depths=ifd.integers(TAG_BITS_PER_SAMPLE);if depths.is_empty(){depths.push(1)}if depths.len()==1{depths.resize(channels,depths[0])}let mut formats=ifd.integers(339);if formats.is_empty(){formats.push(1)}if formats.len()==1{formats.resize(channels,formats[0])}
 let photo=ifd.integer(TAG_PHOTOMETRIC).unwrap_or(1);let palette=ifd.integers(320);let mut pixels=Vec::with_capacity(block.width as usize*block.height as usize*4);
 for sample in block.samples.chunks_exact(channels){
  let display=|lane:usize|->u8{let word=sample[lane].word();let value=match formats[lane]{3=>if depths[lane]==16{let sign=if word&32768!=0{-1.0}else{1.0};let exponent=(word>>10)&31;let fraction=word&1023;sign*match exponent{0=>fraction as f64*2f64.powi(-24),31=>if fraction==0{f64::INFINITY}else{f64::NAN},_=> (1.0+fraction as f64/1024.0)*2f64.powi(exponent as i32-15)}}else if depths[lane]==32{f32::from_bits(word as u32)as f64}else{f64::from_bits(word)},2=>{let sign=1u64<<(depths[lane]-1);let maximum=(sign-1)as f64;let signed=if depths[lane]==64{word as i64}else{((word<<(64-depths[lane]))as i64)>>(64-depths[lane])};((signed as f64+sign as f64)/(maximum+sign as f64)).clamp(0.0,1.0)},_=>{let maximum=if depths[lane]==64{u64::MAX}else{(1u64<<depths[lane])-1};word as f64/maximum as f64}};if value.is_finite(){(value.clamp(0.0,1.0)*255.0).round()as u8}else{0}};
  let rgba=match photo{
   0|1 if channels==1=>{let gray=display(0);let gray=if photo==0{255-gray}else{gray};[gray,gray,gray,255]},
   2 if channels==3||channels==4=>[display(0),display(1),display(2),if channels==4{display(3)}else{255}],
   3 if channels==1=>{let count=palette.len()/3;let index=sample[0].word()as usize;if count*3!=palette.len()||index>=count{return Err("tiff: indexed color table extent".into())}[(palette[index]as u64*255/65535)as u8,(palette[count+index]as u64*255/65535)as u8,(palette[2*count+index]as u64*255/65535)as u8,255]},
   _=>return Err("tiff: page photometric display requires its typed projector".into())
  };pixels.extend_from_slice(&rgba);
 }
 Ok(TiffRgbaPage{width:block.width,height:block.height,pixels})
}
pub fn encode_tiff_page_png(snapshot:&TiffSnapshot,ifd_index:usize)->Result<TiffPngPage,String>{
 use semio_s_artifact_stdio_png::standards::v1_2::subsets::any::schema::snapshot::{PngSnapshot,PngImage,PngColorType};
 let page=decode_tiff_page_rgba(snapshot,ifd_index)?;
 let image=PngImage{width:page.width,height:page.height,bit_depth:8,color_type:PngColorType::Rgba,samples:page.pixels.into_iter().map(u16::from).collect(),..PngImage::default()};
 let bytes=semio_s_artifact_stdio_png::standards::v1_2::subsets::any::io::encode_png(&PngSnapshot{schema:"stdio.png".into(),image})?;
 Ok(TiffPngPage{width:page.width,height:page.height,bytes})
}

#[path="💾️binary/📸️snapshot/🧬️owned-samples/🦀️.rs"]
mod owned_samples;
//#region Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion Tests

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::standards::v6_0::subsets::baseline::io::TiffBaselineComposer;
    use crate::standards::v6_0::subsets::document::io::TiffComposer as TiffRawAnyComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<TiffRawAnyComposer>(), composer_entry_of::<TiffBaselineComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry

#[path = "🛬️decoding/🦀️.rs"]
pub mod controlled_decoding;

#[path = "🛫️encoding/🦀️.rs"]
pub mod controlled_encoding;

#[path="🗂️ordering/🦀️.rs"] mod controlled_ordering;

#[path = "💾️binary/🦀️.rs"]
pub mod binary;

#[path = "📝️text/🦀️.rs"]
pub mod text;

#[path = "🪶️sqlite/🦀️.rs"]
pub mod sqlite;

pub mod derived_construction {
    use crate::{TiffDiff, TiffMutation, TiffSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.tiff` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct TiffBuilderConstruction {
        snapshot: TiffSnapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
    }

    impl ArtifactBuilder for TiffBuilderConstruction {
        type Snapshot = TiffSnapshot;
        type Mutation = TiffMutation;
        type Diff = TiffDiff;
        fn empty() -> Self {
            Self { snapshot: TiffSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<TiffSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<TiffSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::schema::mutations::apply_tiff_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <TiffDiff as protocol::MutationDiff<TiffSnapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>> {
            if self.diagnostics.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(self.diagnostics)
            }
        }
    }
    //#endregion 🔖️Builder
}
pub use derived_construction::*;

pub mod derived_analysis {
    use crate::TiffSnapshot;
    use {semio_framework_plugin::Analysis,semio_framework_plugin::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_plugin::IoConfidence,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.tiff` parts.
    #[derive(Clone, Debug, Default)]
    pub struct TiffParts {
        pub snapshot: Option<TiffSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.tiff` (6.0/✳️any) sources.
    pub struct TiffAnalyzerAnalysis;

    impl ArtifactAnalysis for TiffAnalyzerAnalysis {
        type Parts = TiffParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.tiff", standard: StandardId("6.0"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            const SIG_LE: [u8; 4] = [0x49, 0x49, 0x2A, 0x00]; // "II*\0" little-endian
            const SIG_BE: [u8; 4] = [0x4D, 0x4D, 0x00, 0x2A]; // "MM\0*" big-endian
            match source {
                AnalyzeSource::Binary(bytes) => {
                    if bytes.len() >= 4 && (bytes[0..4] == SIG_LE || bytes[0..4] == SIG_BE) {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
                AnalyzeSource::Text(text) => {
                    // 🔍 stdio.tiff's text envelope is a hex dump of the raw bytes after the
                    // `semio ...` preamble line — decode the first 4 bytes to sniff the real signature.
                    let body = match store::semio_format::split_text_preamble(text) {
                        Ok((_, rest)) => rest,
                        Err(_) => text,
                    };
                    let hex: String = body.chars().filter(|c| !c.is_whitespace()).take(8).collect();
                    if hex.len() < 8 {
                        return IoConfidence::Low;
                    }
                    let mut decoded = [0u8; 4];
                    for (i, byte) in decoded.iter_mut().enumerate() {
                        match u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16) {
                            Ok(b) => *byte = b,
                            Err(_) => return IoConfidence::Low,
                        }
                    }
                    if decoded == SIG_LE || decoded == SIG_BE {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = TiffParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <TiffSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <TiffSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.binary", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
    //#endregion 🔖️Analyzer
}
pub use derived_analysis::*;

semio_framework_plugin::derive_artifact_facets!(
    pub spec TiffBuilderFacets {
        construction: TiffBuilderConstruction,
        analysis: TiffAnalyzerAnalysis,
        composition: crate::standards::v6_0::subsets::document::io::derived_composition::TiffComposerComposition,
    }
    builder: TiffBuilder,
    analyzer: TiffAnalyzer,
    composer: TiffComposer,
);

#[path="💾️binary/📸️snapshot/📠️fax/🦀️.rs"]
mod fax;
