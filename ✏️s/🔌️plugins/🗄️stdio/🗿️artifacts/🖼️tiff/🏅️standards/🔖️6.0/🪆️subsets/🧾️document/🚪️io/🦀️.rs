//! 🚪️ IO stdio.tiff (6.0/✳️any) — registration now flows through 🎹️composer::register
//! (called once from 🔌️plugin/🔧️setup via ⚙️engine::register), not per-leaf register().
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v6_0::subsets::document::io::TiffAnalyzer;
    use crate::TiffSnapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

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
    TiffBinary32, TiffBinary64, TiffByteOrder, TiffFieldType, TiffIfd, TiffSnapshot, TiffStorage, TiffStorageKind, TiffTag, TiffValues, TAG_BITS_PER_SAMPLE, TAG_COMPRESSION, TAG_IMAGE_LENGTH, TAG_IMAGE_WIDTH,
    TAG_PHOTOMETRIC, TAG_ROWS_PER_STRIP, TAG_SAMPLES_PER_PIXEL, TAG_STRIP_BYTE_COUNTS, TAG_STRIP_OFFSETS, TAG_TILE_BYTE_COUNTS, TAG_TILE_LENGTH, TAG_TILE_OFFSETS, TAG_TILE_WIDTH,
};
use crate::STDIO_TIFF_DOCUMENT_SCHEMA;

//#region ByteOrder
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
        TiffFieldType::Ascii => TiffValues::Ascii(src.to_vec()),
        TiffFieldType::Short => TiffValues::Short((0..count).map(|i| e.u16(&src[i * 2..i * 2 + 2])).collect()),
        TiffFieldType::Long => TiffValues::Long((0..count).map(|i| e.u32(&src[i * 4..i * 4 + 4])).collect()),
        TiffFieldType::Rational => TiffValues::Rational((0..count).map(|i| (e.u32(&src[i * 8..i * 8 + 4]), e.u32(&src[i * 8 + 4..i * 8 + 8]))).collect()),
        TiffFieldType::SByte => TiffValues::SByte(src.iter().map(|&b| b as i8).collect()),
        TiffFieldType::Undefined => TiffValues::Undefined(src.to_vec()),
        TiffFieldType::SShort => TiffValues::SShort((0..count).map(|i| e.u16(&src[i * 2..i * 2 + 2]) as i16).collect()),
        TiffFieldType::SLong => TiffValues::SLong((0..count).map(|i| e.u32(&src[i * 4..i * 4 + 4]) as i32).collect()),
        TiffFieldType::SRational => TiffValues::SRational((0..count).map(|i| (e.u32(&src[i * 8..i * 8 + 4]) as i32, e.u32(&src[i * 8 + 4..i * 8 + 8]) as i32)).collect()),
        TiffFieldType::Float => TiffValues::Float((0..count).map(|i| TiffBinary32 { bits: e.u32(&src[i * 4..i * 4 + 4]) }).collect()),
        TiffFieldType::Double => TiffValues::Double((0..count).map(|i| TiffBinary64 { bits: e.u64(&src[i * 8..i * 8 + 8]) }).collect()),
    })
}
//#endregion IfdRead

//#region TagLookup
fn tag_values(ifd: &TiffIfd, tag: u16) -> Option<&TiffValues> {
    ifd.entries.iter().find(|t| t.tag == tag).map(|t| &t.values)
}
fn tag_u32_list(ifd: &TiffIfd, tag: u16) -> Vec<u32> {
    match tag_values(ifd, tag) {
        Some(TiffValues::Short(v)) => v.iter().map(|&x| x as u32).collect(),
        Some(TiffValues::Long(v)) => v.clone(),
        _ => Vec::new(),
    }
}
fn tag_u32(ifd: &TiffIfd, tag: u16) -> Option<u32> {
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
fn read_storage(data: &[u8], ifd: &TiffIfd) -> Result<TiffStorage, String> {
    let strips = tag_u32_list(ifd, TAG_STRIP_OFFSETS);
    let tiles = tag_u32_list(ifd, TAG_TILE_OFFSETS);
    if !strips.is_empty() && !tiles.is_empty() {
        return Err("tiff: one IFD cannot own both strips and tiles".into());
    }
    let (storage_kind, offset_tag, count_tag, offsets) = if !tiles.is_empty() {
        (TiffStorageKind::Tiles, TAG_TILE_OFFSETS, TAG_TILE_BYTE_COUNTS, tiles)
    } else if !strips.is_empty() {
        (TiffStorageKind::Strips, TAG_STRIP_OFFSETS, TAG_STRIP_BYTE_COUNTS, strips)
    } else {
        return Ok(TiffStorage::default());
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
    Ok(TiffStorage { kind: storage_kind, offsets_kind, byte_counts_kind, chunks })
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct TiffRgba8Layout {
    width: u32,
    height: u32,
    samples: usize,
    compression: u32,
    photometric: u32,
    chunk_width: u32,
    chunk_height: u32,
    chunks_across: u32,
    chunks_down: u32,
}

fn tiff_rgba8_layout(ifd: &TiffIfd) -> Result<TiffRgba8Layout, String> {
    let width = tag_u32(ifd, TAG_IMAGE_WIDTH).ok_or("tiff: missing ImageWidth")?;
    let height = tag_u32(ifd, TAG_IMAGE_LENGTH).ok_or("tiff: missing ImageLength")?;
    if width == 0 || height == 0 {
        return Err("tiff: zero dimension".into());
    }
    let samples_per_pixel = tag_u32(ifd, TAG_SAMPLES_PER_PIXEL).unwrap_or(1);
    if !matches!(samples_per_pixel, 1 | 3 | 4) {
        return Err(format!("tiff: unsupported SamplesPerPixel {samples_per_pixel}"));
    }
    let bits_per_sample = tag_u32_list(ifd, TAG_BITS_PER_SAMPLE);
    if bits_per_sample.len() != usize::try_from(samples_per_pixel).map_err(|_| "tiff: sample count exceeds address space")? || bits_per_sample.iter().any(|value| *value != 8) {
        return Err(format!("tiff: unsupported BitsPerSample {bits_per_sample:?} (only 8 is implemented)"));
    }
    let compression = tag_u32(ifd, TAG_COMPRESSION).unwrap_or(1);
    if !matches!(compression, 1 | 32773) {
        return Err(format!("tiff: unsupported compression {compression} (only uncompressed/PackBits are implemented)"));
    }
    if tag_u32(ifd, 317).unwrap_or(1) != 1 {
        return Err("tiff: horizontal prediction is not implemented".into());
    }
    if tag_u32_list(ifd, 339).into_iter().any(|sample_format| sample_format != 1) {
        return Err("tiff: only unsigned integer samples are implemented".into());
    }
    let photometric = tag_u32(ifd, TAG_PHOTOMETRIC).unwrap_or(1);
    if samples_per_pixel == 1 && !matches!(photometric, 0 | 1) {
        return Err(format!("tiff: unsupported grayscale photometric {photometric}"));
    }
    if samples_per_pixel != 1 && photometric != 2 {
        return Err(format!("tiff: unsupported RGB photometric {photometric}"));
    }
    if tag_u32(ifd, 284).unwrap_or(1) != 1 {
        return Err("tiff: planar display projection is not implemented".into());
    }
    if samples_per_pixel == 4 && tag_u32_list(ifd, 338) != [2] {
        return Err("tiff: four-sample display projection requires one unassociated alpha ExtraSample".into());
    }
    if tag_u32(ifd, 274).unwrap_or(1) != 1 {
        return Err("tiff: oriented display projection is not implemented".into());
    }
    if ifd.storage.chunks.is_empty() {
        return Err("tiff: display projection requires authored image storage".into());
    }
    let (chunk_width, chunk_height, chunks_across, chunks_down) = match ifd.storage.kind {
        TiffStorageKind::Strips => {
            let rows = tag_u32(ifd, TAG_ROWS_PER_STRIP).unwrap_or(height);
            if rows == 0 { return Err("tiff: RowsPerStrip must be positive".into()); }
            (width, rows, 1, height.checked_add(rows - 1).ok_or("tiff: strip count overflow")? / rows)
        }
        TiffStorageKind::Tiles => {
            let tile_width = tag_u32(ifd, TAG_TILE_WIDTH).ok_or("tiff: tiled image is missing TileWidth")?;
            let tile_length = tag_u32(ifd, TAG_TILE_LENGTH).ok_or("tiff: tiled image is missing TileLength")?;
            if tile_width == 0 || tile_length == 0 { return Err("tiff: tile dimensions must be positive".into()); }
            let across = width.checked_add(tile_width - 1).ok_or("tiff: tile column count overflow")? / tile_width;
            let down = height.checked_add(tile_length - 1).ok_or("tiff: tile row count overflow")? / tile_length;
            (tile_width, tile_length, across, down)
        }
        TiffStorageKind::None => return Err("tiff: display projection requires authored image storage".into()),
    };
    let expected_chunks = chunks_across.checked_mul(chunks_down).ok_or("tiff: chunk count overflow")?;
    if ifd.storage.chunks.len() != usize::try_from(expected_chunks).map_err(|_| "tiff: chunk count exceeds address space")? {
        return Err(format!("tiff: chunk count {} does not match expected {expected_chunks}", ifd.storage.chunks.len()));
    }
    Ok(TiffRgba8Layout { width, height, samples: usize::try_from(samples_per_pixel).map_err(|_| "tiff: sample count exceeds address space")?, compression, photometric, chunk_width, chunk_height, chunks_across, chunks_down })
}

fn write_rgba(source: &[u8], samples: usize, photometric: u32, target: &mut [u8]) {
    match samples {
        1 => {
            let gray = if photometric == 0 { 255 - source[0] } else { source[0] };
            target.copy_from_slice(&[gray, gray, gray, 255]);
        }
        3 => target.copy_from_slice(&[source[0], source[1], source[2], 255]),
        4 => target.copy_from_slice(&source[..4]),
        _ => unreachable!("validated TIFF sample count"),
    }
}

fn decoded_chunk<'a>(chunk: &'a [u8], compression: u32, expected: usize, decoded: &'a mut Vec<u8>) -> Result<&'a [u8], String> {
    if compression == 32773 {
        *decoded = packbits_decode(chunk, expected)?;
        Ok(decoded)
    } else if chunk.len() == expected {
        Ok(chunk)
    } else {
        Err(format!("tiff: uncompressed chunk has {} bytes, expected {expected}", chunk.len()))
    }
}

/// 🖼️ Projects one canonical strip- or tile-organized IFD into ephemeral RGBA8 display pixels.
pub fn decode_tiff_page_rgba(snapshot: &TiffSnapshot, ifd_index: usize) -> Result<TiffRgbaPage, String> {
    let ifd = snapshot.ifds.get(ifd_index).ok_or_else(|| format!("tiff: IFD {ifd_index} is outside the document"))?;
    let layout = tiff_rgba8_layout(ifd)?;
    let width_usize = usize::try_from(layout.width).map_err(|_| "tiff: image width exceeds address space")?;
    let height_usize = usize::try_from(layout.height).map_err(|_| "tiff: image height exceeds address space")?;
    let pixel_count = width_usize.checked_mul(height_usize).ok_or("tiff: pixel count overflow")?;
    let mut rgba = vec![0u8; pixel_count.checked_mul(4).ok_or("tiff: RGBA byte count overflow")?];
    let chunk_width = usize::try_from(layout.chunk_width).map_err(|_| "tiff: chunk width exceeds address space")?;
    let chunk_height = usize::try_from(layout.chunk_height).map_err(|_| "tiff: chunk height exceeds address space")?;
    let chunk_bytes = chunk_width.checked_mul(chunk_height).and_then(|pixels| pixels.checked_mul(layout.samples)).ok_or("tiff: chunk byte count overflow")?;
    for (index, chunk) in ifd.storage.chunks.iter().enumerate() {
        let chunk_row = u32::try_from(index).map_err(|_| "tiff: chunk ordinal width")? / layout.chunks_across;
        let chunk_column = u32::try_from(index).map_err(|_| "tiff: chunk ordinal width")? % layout.chunks_across;
        let origin_x = chunk_column.checked_mul(layout.chunk_width).ok_or("tiff: chunk x overflow")?;
        let origin_y = chunk_row.checked_mul(layout.chunk_height).ok_or("tiff: chunk y overflow")?;
        let visible_width = usize::try_from(layout.chunk_width.min(layout.width - origin_x)).map_err(|_| "tiff: visible chunk width")?;
        let visible_height = usize::try_from(layout.chunk_height.min(layout.height - origin_y)).map_err(|_| "tiff: visible chunk height")?;
        let expected = if ifd.storage.kind == TiffStorageKind::Tiles { chunk_bytes } else { visible_height.checked_mul(chunk_width).and_then(|pixels| pixels.checked_mul(layout.samples)).ok_or("tiff: strip byte count overflow")? };
        let mut decoded = Vec::new();
        let source = decoded_chunk(chunk, layout.compression, expected, &mut decoded)?;
        for local_y in 0..visible_height {
            for local_x in 0..visible_width {
                let source_offset = local_y.checked_mul(chunk_width).and_then(|row| row.checked_add(local_x)).and_then(|pixel| pixel.checked_mul(layout.samples)).ok_or("tiff: source sample offset overflow")?;
                let x = usize::try_from(origin_x).map_err(|_| "tiff: chunk x width")?.checked_add(local_x).ok_or("tiff: target x overflow")?;
                let y = usize::try_from(origin_y).map_err(|_| "tiff: chunk y width")?.checked_add(local_y).ok_or("tiff: target y overflow")?;
                let target = y.checked_mul(width_usize).and_then(|row| row.checked_add(x)).and_then(|pixel| pixel.checked_mul(4)).ok_or("tiff: target sample offset overflow")?;
                write_rgba(&source[source_offset..source_offset + layout.samples], layout.samples, layout.photometric, &mut rgba[target..target + 4]);
            }
        }
    }
    Ok(TiffRgbaPage { width: layout.width, height: layout.height, pixels: rgba })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TiffRegion {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

pub const TIFF_MAXIMUM_INTERACTIVE_PAINT_ROWS: u32 = 65_536;

pub fn tiff_revision(snapshot: &TiffSnapshot) -> String {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in semio_framework_pack_json::to_json_string(snapshot).bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

fn stored_color(layout: TiffRgba8Layout, color: [u8; 4]) -> Result<Vec<u8>, String> {
    match layout.samples {
        1 if color[0] == color[1] && color[1] == color[2] && color[3] == 255 => Ok(vec![if layout.photometric == 0 { 255 - color[0] } else { color[0] }]),
        1 => Err("tiff: grayscale paint requires equal RGB channels and opaque alpha".into()),
        3 if color[3] == 255 => Ok(color[..3].to_vec()),
        3 => Err("tiff: RGB paint requires opaque alpha".into()),
        4 => Ok(color.to_vec()),
        _ => Err("tiff: unsupported paint sample profile".into()),
    }
}

/// 🛡️ Validates one exact raw-sample tiled paint without allocating a derived raster or snapshot.
pub fn validate_tiff_region_paint(snapshot: &TiffSnapshot, ifd_index: usize, region: TiffRegion, color: [u8; 4]) -> Result<(), String> {
    let ifd = snapshot.ifds.get(ifd_index).ok_or_else(|| format!("tiff: IFD {ifd_index} is outside the document"))?;
    let layout = tiff_rgba8_layout(ifd)?;
    if ifd.storage.kind != TiffStorageKind::Tiles { return Err("tiff: region paint requires tiled storage".into()); }
    if layout.compression != 1 { return Err("tiff: region paint requires uncompressed tiles".into()); }
    if region.width == 0 || region.height == 0 { return Err("tiff: paint region must be nonempty".into()); }
    if region.height > TIFF_MAXIMUM_INTERACTIVE_PAINT_ROWS { return Err(format!("tiff: paint region height exceeds {TIFF_MAXIMUM_INTERACTIVE_PAINT_ROWS}")); }
    let end_x = region.x.checked_add(region.width).ok_or("tiff: paint region x overflow")?;
    let end_y = region.y.checked_add(region.height).ok_or("tiff: paint region y overflow")?;
    if end_x > layout.width || end_y > layout.height { return Err(format!("tiff: paint region exceeds {}x{} page", layout.width, layout.height)); }
    stored_color(layout, color)?;
    Ok(())
}

/// 🖌️ Paints raw samples in one uncompressed tiled IFD while retaining every other canonical byte.
pub fn paint_tiff_region_controlled(snapshot: &TiffSnapshot, revision: &str, ifd_index: usize, region: TiffRegion, color: [u8; 4], progress: &mut dyn FnMut(usize, usize) -> bool) -> Result<TiffSnapshot, String> {
    let actual = tiff_revision(snapshot);
    if revision != actual { return Err(format!("tiff: stale document revision {revision}; expected {actual}")); }
    validate_tiff_region_paint(snapshot, ifd_index, region, color)?;
    let ifd = &snapshot.ifds[ifd_index];
    let layout = tiff_rgba8_layout(ifd)?;
    let end_x = region.x.checked_add(region.width).ok_or("tiff: paint region x overflow")?;
    let stored = stored_color(layout, color)?;
    let total = usize::try_from(region.height).map_err(|_| "tiff: paint height exceeds address space")?;
    let mut next = snapshot.clone();
    let chunks = &mut next.ifds[ifd_index].storage.chunks;
    let tile_width = usize::try_from(layout.chunk_width).map_err(|_| "tiff: tile width exceeds address space")?;
    for local_y in 0..total {
        if !progress(local_y, total) { return Err("tiff: tiled paint cancelled".into()); }
        let y = region.y.checked_add(u32::try_from(local_y).map_err(|_| "tiff: paint row width")?).ok_or("tiff: paint row overflow")?;
        for x in region.x..end_x {
            let tile_row = y / layout.chunk_height;
            let tile_column = x / layout.chunk_width;
            let tile_index = usize::try_from(tile_row.checked_mul(layout.chunks_across).and_then(|row| row.checked_add(tile_column)).ok_or("tiff: tile ordinal overflow")?).map_err(|_| "tiff: tile ordinal width")?;
            let local_x = usize::try_from(x % layout.chunk_width).map_err(|_| "tiff: tile local x width")?;
            let local_tile_y = usize::try_from(y % layout.chunk_height).map_err(|_| "tiff: tile local y width")?;
            let offset = local_tile_y.checked_mul(tile_width).and_then(|row| row.checked_add(local_x)).and_then(|pixel| pixel.checked_mul(layout.samples)).ok_or("tiff: tile sample offset overflow")?;
            chunks[tile_index][offset..offset + layout.samples].copy_from_slice(&stored);
        }
    }
    if !progress(total, total) { return Err("tiff: tiled paint cancelled".into()); }
    Ok(next)
}

/// 🌐 Encodes one TIFF page as a browser-displayable PNG without changing document authority.
pub fn encode_tiff_page_png(snapshot: &TiffSnapshot, ifd_index: usize) -> Result<TiffPngPage, String> {
    let page = decode_tiff_page_rgba(snapshot, ifd_index)?;
    let bytes = semio_framework_pixels::encode_png(&semio_framework_pixels::RasterImage { width: page.width, height: page.height, pixels: page.pixels }).map_err(|error| error.to_string())?;
    Ok(TiffPngPage { width: page.width, height: page.height, bytes })
}

pub fn decode_tiff(data: &[u8]) -> Result<TiffSnapshot, String> {
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
        ifds.push(TiffIfd { entries, storage: TiffStorage::default() });
    }

    for ifd in &mut ifds {
        ifd.storage = read_storage(data, ifd)?;
        ifd.entries.retain(|entry| !matches!(entry.tag, TAG_STRIP_OFFSETS | TAG_STRIP_BYTE_COUNTS | TAG_TILE_OFFSETS | TAG_TILE_BYTE_COUNTS));
    }
    Ok(TiffSnapshot { schema: STDIO_TIFF_DOCUMENT_SCHEMA.into(), byte_order, ifds })
}
//#endregion Decode

//#region Encode
fn value_bytes(values: &TiffValues, bo: TiffByteOrder) -> Vec<u8> {
    let mut out = Vec::new();
    match values {
        TiffValues::Byte(v) | TiffValues::Ascii(v) | TiffValues::Undefined(v) => out.extend_from_slice(v),
        TiffValues::Short(v) => v.iter().for_each(|&x| write_u16(&mut out, x, bo)),
        TiffValues::Long(v) => v.iter().for_each(|&x| write_u32(&mut out, x, bo)),
        TiffValues::Rational(v) => v.iter().for_each(|&(n, d)| { write_u32(&mut out, n, bo); write_u32(&mut out, d, bo); }),
        TiffValues::SByte(v) => out.extend(v.iter().map(|&x| x as u8)),
        TiffValues::SShort(v) => v.iter().for_each(|&x| write_u16(&mut out, x as u16, bo)),
        TiffValues::SLong(v) => v.iter().for_each(|&x| write_u32(&mut out, x as u32, bo)),
        TiffValues::SRational(v) => v.iter().for_each(|&(n, d)| { write_u32(&mut out, n as u32, bo); write_u32(&mut out, d as u32, bo); }),
        TiffValues::Float(v) => v.iter().for_each(|x| write_u32(&mut out, x.bits, bo)),
        TiffValues::Double(v) => v.iter().for_each(|x| write_u64(&mut out, x.bits, bo)),
    }
    out
}

fn dir_size(count: usize) -> Result<usize, String> {
    count.checked_mul(12).and_then(|size| size.checked_add(6)).ok_or_else(|| "tiff: directory size overflow".into())
}

fn out_of_line_size(entries: &[TiffTag], bo: TiffByteOrder) -> Result<usize, String> {
    entries.iter().try_fold(0usize, |total, tag| {
        let length = value_bytes(&tag.values, bo).len();
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

fn storage_tags(ifd: &TiffIfd) -> Result<Vec<TiffTag>, String> {
    if ifd.storage.kind == TiffStorageKind::None {
        if !ifd.storage.chunks.is_empty() {
            return Err("tiff: none storage cannot own chunks".into());
        }
        return Ok(Vec::new());
    }
    if ifd.storage.chunks.is_empty() {
        return Err("tiff: image storage requires at least one chunk".into());
    }
    let (offset_tag, count_tag) = match ifd.storage.kind {
        TiffStorageKind::Strips => (TAG_STRIP_OFFSETS, TAG_STRIP_BYTE_COUNTS),
        TiffStorageKind::Tiles => (TAG_TILE_OFFSETS, TAG_TILE_BYTE_COUNTS),
        TiffStorageKind::None => unreachable!(),
    };
    let zeroes = vec![0; ifd.storage.chunks.len()];
    let counts = ifd.storage.chunks.iter().map(|chunk| u32::try_from(chunk.len()).map_err(|_| "tiff: storage chunk exceeds TIFF6 byte-count width".to_string())).collect::<Result<Vec<_>, _>>()?;
    Ok(vec![
        TiffTag { tag: offset_tag, values: storage_values(ifd.storage.offsets_kind, &zeroes, "storage offset")? },
        TiffTag { tag: count_tag, values: storage_values(ifd.storage.byte_counts_kind, &counts, "storage byte count")? },
    ])
}

fn encode_tiff_preserving_storage(snapshot: &TiffSnapshot) -> Result<Vec<u8>, String> {
    if snapshot.ifds.is_empty() {
        return Err("tiff: encode requires at least one IFD".into());
    }
    let mut entries_per_ifd = Vec::with_capacity(snapshot.ifds.len());
    for ifd in &snapshot.ifds {
        let mut entries: Vec<TiffTag> = ifd.entries.iter().filter(|tag| !matches!(tag.tag, TAG_STRIP_OFFSETS | TAG_STRIP_BYTE_COUNTS | TAG_TILE_OFFSETS | TAG_TILE_BYTE_COUNTS)).cloned().collect();
        entries.extend(storage_tags(ifd)?);
        entries.sort_by_key(|tag| tag.tag);
        if entries.windows(2).any(|pair| pair[0].tag == pair[1].tag) {
            return Err("tiff: duplicate IFD tag".into());
        }
        u16::try_from(entries.len()).map_err(|_| "tiff: directory exceeds TIFF6 entry-count width")?;
        entries_per_ifd.push(entries);
    }

    let mut cursor = 8usize;
    let mut directory_offsets = Vec::with_capacity(entries_per_ifd.len());
    for entries in &entries_per_ifd {
        directory_offsets.push(cursor);
        cursor = cursor.checked_add(dir_size(entries.len())?).and_then(|value| value.checked_add(out_of_line_size(entries, snapshot.byte_order).ok()?)).ok_or("tiff: directory layout overflow")?;
    }
    for (ifd_index, ifd) in snapshot.ifds.iter().enumerate() {
        if ifd.storage.kind == TiffStorageKind::None {
            continue;
        }
        let offset_tag = match ifd.storage.kind { TiffStorageKind::Strips => TAG_STRIP_OFFSETS, TiffStorageKind::Tiles => TAG_TILE_OFFSETS, TiffStorageKind::None => unreachable!() };
        let mut offsets = Vec::with_capacity(ifd.storage.chunks.len());
        for chunk in &ifd.storage.chunks {
            offsets.push(u32::try_from(cursor).map_err(|_| "tiff: storage offset exceeds TIFF6 width")?);
            cursor = cursor.checked_add(chunk.len()).ok_or("tiff: storage layout overflow")?;
        }
        let entry = entries_per_ifd[ifd_index].iter_mut().find(|entry| entry.tag == offset_tag).ok_or("tiff: derived storage offset tag missing")?;
        entry.values = storage_values(ifd.storage.offsets_kind, &offsets, "storage offset")?;
    }
    u32::try_from(cursor).map_err(|_| "tiff: file exceeds TIFF6 offset width")?;

    let mut out = Vec::with_capacity(cursor);
    out.extend_from_slice(match snapshot.byte_order { TiffByteOrder::LittleEndian => b"II", TiffByteOrder::BigEndian => b"MM" });
    write_u16(&mut out, 42, snapshot.byte_order);
    write_u32(&mut out, 8, snapshot.byte_order);
    for (ifd_index, entries) in entries_per_ifd.iter().enumerate() {
        if out.len() != directory_offsets[ifd_index] {
            return Err("tiff: directory layout invariant".into());
        }
        write_u16(&mut out, u16::try_from(entries.len()).map_err(|_| "tiff: entry-count width")?, snapshot.byte_order);
        let mut value_cursor = directory_offsets[ifd_index].checked_add(dir_size(entries.len())?).ok_or("tiff: value layout overflow")?;
        for tag in entries {
            write_u16(&mut out, tag.tag, snapshot.byte_order);
            write_u16(&mut out, tag.values.kind().to_u16(), snapshot.byte_order);
            write_u32(&mut out, tag.values.count(), snapshot.byte_order);
            let bytes = value_bytes(&tag.values, snapshot.byte_order);
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
            let bytes = value_bytes(&tag.values, snapshot.byte_order);
            if bytes.len() > 4 {
                out.extend_from_slice(&bytes);
                if bytes.len() % 2 == 1 { out.push(0); }
            }
        }
    }
    for ifd in &snapshot.ifds {
        for chunk in &ifd.storage.chunks {
            out.extend_from_slice(chunk);
        }
    }
    if out.len() != cursor {
        return Err("tiff: final output length invariant".into());
    }
    Ok(out)
}

pub fn encode_tiff(snapshot: &TiffSnapshot) -> Result<Vec<u8>, String> {
    encode_tiff_preserving_storage(snapshot)
}

/// 📦 Explicitly converts uncompressed strip chunks to PackBits before preserving the new carrier.
pub fn encode_tiff_packbits(snapshot: &TiffSnapshot) -> Result<Vec<u8>, String> {
    let mut converted = snapshot.clone();
    let ifd = converted.ifds.first_mut().ok_or("tiff: PackBits conversion requires IFD 0")?;
    if ifd.storage.kind != TiffStorageKind::Strips {
        return Err("tiff: PackBits conversion requires strip storage".into());
    }
    let compression = ifd.entries.iter_mut().find(|tag| tag.tag == TAG_COMPRESSION);
    if let Some(tag) = compression {
        tag.values = TiffValues::Short(vec![32773]);
    } else {
        ifd.entries.push(TiffTag { tag: TAG_COMPRESSION, values: TiffValues::Short(vec![32773]) });
    }
    ifd.storage.chunks = ifd.storage.chunks.iter().map(|chunk| packbits_encode(chunk)).collect();
    encode_tiff_preserving_storage(&converted)
}
//#endregion Encode

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
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

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
