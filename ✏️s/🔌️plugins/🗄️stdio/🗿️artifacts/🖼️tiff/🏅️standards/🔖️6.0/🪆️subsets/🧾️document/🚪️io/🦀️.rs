//! 🚪️ IO stdio.tiff (6.0/✳️any) — registration now flows through 🎹️composer::register
//! (called once from 🔌️plugin/🔧️setup via ⚙️engine::register), not per-leaf register().
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v6_0::subsets::document::schema::TiffAnalyzer;
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
// zero real callers) were deleted outright. `empty_tiff_snapshot`/`demo_tiff_snapshot` moved to
// `../🧬️schema`.
use crate::schema::snapshot::{
    TiffByteOrder, TiffFieldType, TiffIfd, TiffSnapshot, TiffTag, TiffValues, TAG_BITS_PER_SAMPLE, TAG_COMPRESSION, TAG_IMAGE_LENGTH, TAG_IMAGE_WIDTH, TAG_PHOTOMETRIC, TAG_ROWS_PER_STRIP, TAG_SAMPLES_PER_PIXEL, TAG_STRIP_BYTE_COUNTS,
    TAG_STRIP_OFFSETS,
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
        TiffFieldType::Ascii => {
            let text = String::from_utf8_lossy(src);
            TiffValues::Ascii(text.trim_end_matches('\u{0}').to_string())
        }
        TiffFieldType::Short => TiffValues::Short((0..count).map(|i| e.u16(&src[i * 2..i * 2 + 2])).collect()),
        TiffFieldType::Long => TiffValues::Long((0..count).map(|i| e.u32(&src[i * 4..i * 4 + 4])).collect()),
        TiffFieldType::Rational => TiffValues::Rational((0..count).map(|i| (e.u32(&src[i * 8..i * 8 + 4]), e.u32(&src[i * 8 + 4..i * 8 + 8]))).collect()),
        TiffFieldType::SByte => TiffValues::SByte(src.iter().map(|&b| b as i8).collect()),
        TiffFieldType::Undefined => TiffValues::Undefined(src.to_vec()),
        TiffFieldType::SShort => TiffValues::SShort((0..count).map(|i| e.u16(&src[i * 2..i * 2 + 2]) as i16).collect()),
        TiffFieldType::SLong => TiffValues::SLong((0..count).map(|i| e.u32(&src[i * 4..i * 4 + 4]) as i32).collect()),
        TiffFieldType::SRational => TiffValues::SRational((0..count).map(|i| (e.u32(&src[i * 8..i * 8 + 4]) as i32, e.u32(&src[i * 8 + 4..i * 8 + 8]) as i32)).collect()),
        TiffFieldType::Float => TiffValues::Float((0..count).map(|i| f32::from_bits(e.u32(&src[i * 4..i * 4 + 4]))).collect()),
        TiffFieldType::Double => TiffValues::Double((0..count).map(|i| f64::from_bits(e.u64(&src[i * 8..i * 8 + 8]))).collect()),
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
/// 🧵 Concatenates one directory's strips VERBATIM, in `StripOffsets` order — no decompression, no
/// photometric interpretation, nothing that could fail on a layout this codec does not model. A
/// directory that declares no strips at all (metadata-only, or a tiled one whose payload lives in
/// `TileOffsets`) honestly yields an empty payload rather than a fabricated one.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_raw_strips(data: &[u8], ifd: &TiffIfd) -> Result<Vec<u8>, String> {
    let offsets = tag_u32_list(ifd, TAG_STRIP_OFFSETS);
    let counts = tag_u32_list(ifd, TAG_STRIP_BYTE_COUNTS);
    if offsets.is_empty() || counts.len() != offsets.len() {
        return Ok(Vec::new());
    }
    let mut strips = Vec::with_capacity(counts.iter().map(|&c| c as usize).sum());
    for (&offset, &count) in offsets.iter().zip(counts.iter()) {
        let start = offset as usize;
        strips.extend_from_slice(data.get(start..start + count as usize).ok_or("tiff: strip data truncated")?);
    }
    Ok(strips)
}

fn decode_pixels_from_ifd(data: &[u8], ifd: &TiffIfd) -> Result<Vec<u8>, String> {
    let width = tag_u32(ifd, TAG_IMAGE_WIDTH).ok_or("tiff: missing ImageWidth")?;
    let height = tag_u32(ifd, TAG_IMAGE_LENGTH).ok_or("tiff: missing ImageLength")?;
    if width == 0 || height == 0 {
        return Err("tiff: zero dimension".into());
    }
    let bits_per_sample = tag_u32(ifd, TAG_BITS_PER_SAMPLE).unwrap_or(8);
    if bits_per_sample != 8 {
        return Err(format!("tiff: unsupported BitsPerSample {bits_per_sample} (only 8 is implemented)"));
    }
    let samples_per_pixel = tag_u32(ifd, TAG_SAMPLES_PER_PIXEL).unwrap_or(1);
    if samples_per_pixel != 1 && samples_per_pixel != 3 && samples_per_pixel != 4 {
        return Err(format!("tiff: unsupported SamplesPerPixel {samples_per_pixel}"));
    }
    let compression = tag_u32(ifd, TAG_COMPRESSION).unwrap_or(1);
    if compression != 1 && compression != 32773 {
        return Err(format!("tiff: unsupported compression {compression} (only uncompressed/PackBits are implemented)"));
    }
    let photometric = tag_u32(ifd, TAG_PHOTOMETRIC).unwrap_or(1);
    let strip_offsets = tag_u32_list(ifd, TAG_STRIP_OFFSETS);
    if strip_offsets.is_empty() {
        return Err("tiff: missing StripOffsets".into());
    }
    let strip_byte_counts = tag_u32_list(ifd, TAG_STRIP_BYTE_COUNTS);
    let rows_per_strip = tag_u32(ifd, TAG_ROWS_PER_STRIP).unwrap_or(height);

    let row_bytes = width as usize * samples_per_pixel as usize;
    let mut raster = vec![0u8; row_bytes * height as usize];
    let mut row_cursor = 0usize;
    for (i, &offset) in strip_offsets.iter().enumerate() {
        if row_cursor >= height as usize {
            break;
        }
        let rows_here = rows_per_strip.min(height - row_cursor as u32) as usize;
        let strip_len = rows_here * row_bytes;
        let start = offset as usize;
        let decoded: Vec<u8> = if compression == 32773 {
            let byte_count = *strip_byte_counts.get(i).ok_or("tiff: missing StripByteCounts entry")? as usize;
            let compressed = data.get(start..start + byte_count).ok_or("tiff: strip data truncated")?;
            packbits_decode(compressed, strip_len)?
        } else {
            data.get(start..start + strip_len).ok_or("tiff: strip data truncated")?.to_vec()
        };
        let dst_start = row_cursor * row_bytes;
        raster[dst_start..dst_start + strip_len].copy_from_slice(&decoded);
        row_cursor += rows_here;
    }

    let mut rgba = vec![0u8; width as usize * height as usize * 4];
    for p in 0..(width as usize * height as usize) {
        let so = p * samples_per_pixel as usize;
        let o = p * 4;
        match samples_per_pixel {
            1 => {
                let mut g = raster[so];
                if photometric == 0 {
                    g = 255 - g; // WhiteIsZero
                }
                rgba[o] = g;
                rgba[o + 1] = g;
                rgba[o + 2] = g;
                rgba[o + 3] = 255;
            }
            3 => {
                rgba[o] = raster[so];
                rgba[o + 1] = raster[so + 1];
                rgba[o + 2] = raster[so + 2];
                rgba[o + 3] = 255;
            }
            4 => rgba[o..o + 4].copy_from_slice(&raster[so..so + 4]),
            _ => unreachable!("validated above"),
        }
    }
    Ok(rgba)
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
            entries.push(TiffTag { tag: entry.tag, kind, values });
        }
        entries.sort_by_key(|t| t.tag); // TIFF6 §2: entries "must be sorted in ascending order by Tag".
        ifds.push(TiffIfd { pixels: Vec::new(), entries });
    }

    // 🖼️ Every directory BEYOND the first keeps its raster as RAW STRIP BYTES (see `TiffIfd::pixels`).
    // No interpretation happens: the strips are concatenated in `StripOffsets` order and stored
    // exactly as the file spells them, so a secondary page whose photometric layout this codec does
    // not decode still survives a round trip intact. IFD 0's raster is the snapshot's own decoded
    // `pixels`, so `ifds[0].pixels` deliberately stays empty.
    // 🧭 `StripOffsets`/`StripByteCounts` are LAYOUT, not content: once the payload itself is held
    // in `pixels`, keeping the source file's byte offsets in `entries` would be keeping a pointer
    // into a file this snapshot is no longer bound to — and `decode(encode(x)) == x` could never
    // hold, since the writer necessarily lays those strips out somewhere else. They are dropped
    // here and recomputed at encode time, exactly as IFD 0's already are.
    for ifd in ifds.iter_mut().skip(1) {
        let strips = read_raw_strips(data, ifd)?;
        if !strips.is_empty() {
            ifd.entries.retain(|t| t.tag != TAG_STRIP_OFFSETS && t.tag != TAG_STRIP_BYTE_COUNTS);
        }
        ifd.pixels = strips;
    }

    // Pixel decode only runs against IFD 0 — see module doc's normalization note.
    let pixels = decode_pixels_from_ifd(data, &ifds[0])?;

    Ok(TiffSnapshot { schema: STDIO_TIFF_DOCUMENT_SCHEMA.into(), byte_order, ifds, pixels })
}
//#endregion Decode

//#region Encode
fn rgba_to_rgb(pixels: &[u8], width: u32, height: u32) -> Result<Vec<u8>, String> {
    let n = width as usize * height as usize;
    if pixels.len() != n * 4 {
        return Err(format!("tiff: pixels length mismatch (got {}, expected {} for {width}x{height} RGBA)", pixels.len(), n * 4));
    }
    let mut rgb = Vec::with_capacity(n * 3);
    for px in pixels.chunks(4) {
        rgb.extend_from_slice(&px[0..3]);
    }
    Ok(rgb)
}

fn value_bytes(values: &TiffValues, bo: TiffByteOrder) -> Vec<u8> {
    let mut out = Vec::new();
    match values {
        TiffValues::Byte(v) | TiffValues::Undefined(v) => out.extend_from_slice(v),
        TiffValues::Ascii(s) => {
            out.extend_from_slice(s.as_bytes());
            out.push(0);
        }
        TiffValues::Short(v) => v.iter().for_each(|&x| {
            write_u16(&mut out, x, bo);
        }),
        TiffValues::Long(v) => v.iter().for_each(|&x| {
            write_u32(&mut out, x, bo);
        }),
        TiffValues::Rational(v) => v.iter().for_each(|&(n, d)| {
            write_u32(&mut out, n, bo);
            write_u32(&mut out, d, bo);
        }),
        TiffValues::SByte(v) => out.extend(v.iter().map(|&x| x as u8)),
        TiffValues::SShort(v) => v.iter().for_each(|&x| {
            write_u16(&mut out, x as u16, bo);
        }),
        TiffValues::SLong(v) => v.iter().for_each(|&x| {
            write_u32(&mut out, x as u32, bo);
        }),
        TiffValues::SRational(v) => v.iter().for_each(|&(n, d)| {
            write_u32(&mut out, n as u32, bo);
            write_u32(&mut out, d as u32, bo);
        }),
        TiffValues::Float(v) => v.iter().for_each(|&x| {
            write_u32(&mut out, x.to_bits(), bo);
        }),
        TiffValues::Double(v) => v.iter().for_each(|&x| {
            write_u64(&mut out, x.to_bits(), bo);
        }),
    }
    out
}

const CORE_STRIP_TAGS: [u16; 9] = [TAG_IMAGE_WIDTH, TAG_IMAGE_LENGTH, TAG_BITS_PER_SAMPLE, TAG_COMPRESSION, TAG_PHOTOMETRIC, TAG_STRIP_OFFSETS, TAG_SAMPLES_PER_PIXEL, TAG_ROWS_PER_STRIP, TAG_STRIP_BYTE_COUNTS];

fn dir_size(n: usize) -> usize {
    2 + 12 * n + 4
}
fn out_of_line_size(entries: &[TiffTag], bo: TiffByteOrder) -> usize {
    entries
        .iter()
        .map(|t| {
            let len = value_bytes(&t.values, bo).len();
            if len <= 4 {
                0
            } else {
                len + (len % 2)
            }
        })
        .sum()
}

/// 🚫 MultiIfdEncodeScopeNote (see module doc): writes the REAL whole-`ifds` chain — `ifds[0]`'s
/// baseline strip/geometry tags recomputed fresh from `pixels`, every OTHER `ifds[0]` tag carried
/// over verbatim so `SetTag`-set metadata round-trips, and `ifds[1..]` carried verbatim minus
/// `StripOffsets`/`StripByteCounts` (this codec has no per-IFD raw-strip storage beyond IFD 0 —
/// see module doc). `byte_order` is honored (real round-trip, unlike the pre-migration engine
/// which always emitted little-endian).
fn encode_tiff_with(snap: &TiffSnapshot, packbits: bool) -> Result<Vec<u8>, String> {
    let width = snap.width().ok_or("tiff: encode requires an ImageWidth tag in ifds[0] (e.g. via SetTag)")?;
    let height = snap.height().ok_or("tiff: encode requires an ImageLength tag in ifds[0] (e.g. via SetTag)")?;
    if width == 0 || height == 0 {
        return Err("tiff: empty image".into());
    }
    let rgb = rgba_to_rgb(&snap.pixels, width, height)?;
    let strip_bytes = if packbits { packbits_encode(&rgb) } else { rgb };
    let compression: u32 = if packbits { 32773 } else { 1 };

    // IFD 0: baseline strip/geometry tags recomputed fresh from `pixels`, every other tag
    // carried over verbatim (unchanged from the pre-multi-IFD behavior).
    let carried: Vec<TiffTag> = snap.ifds.first().map(|ifd| ifd.entries.iter().filter(|t| !CORE_STRIP_TAGS.contains(&t.tag)).cloned().collect()).unwrap_or_default();
    let mut ifd0 = carried;
    ifd0.push(TiffTag { tag: TAG_IMAGE_WIDTH, kind: TiffFieldType::Long, values: TiffValues::Long(vec![width]) });
    ifd0.push(TiffTag { tag: TAG_IMAGE_LENGTH, kind: TiffFieldType::Long, values: TiffValues::Long(vec![height]) });
    // 🎨 TIFF6 §Baseline Fields (p.29): BitsPerSample's COUNT is SamplesPerPixel, one entry per
    // channel — not one entry for the image. This encoder always writes chunky 8-bit RGB
    // (`SamplesPerPixel` 3, two lines below), so the field is `[8, 8, 8]`. A single `[8]` is a
    // count/SamplesPerPixel contradiction that lenient readers paper over and a conformant one
    // reports verbatim; three SHORTs are 6 bytes, so the value moves out of line, which the layout
    // pass below already sizes through `out_of_line_size`.
    ifd0.push(TiffTag { tag: TAG_BITS_PER_SAMPLE, kind: TiffFieldType::Short, values: TiffValues::Short(vec![8, 8, 8]) });
    ifd0.push(TiffTag { tag: TAG_COMPRESSION, kind: TiffFieldType::Short, values: TiffValues::Short(vec![compression as u16]) });
    ifd0.push(TiffTag { tag: TAG_PHOTOMETRIC, kind: TiffFieldType::Short, values: TiffValues::Short(vec![2]) });
    ifd0.push(TiffTag { tag: TAG_SAMPLES_PER_PIXEL, kind: TiffFieldType::Short, values: TiffValues::Short(vec![3]) });
    ifd0.push(TiffTag { tag: TAG_ROWS_PER_STRIP, kind: TiffFieldType::Long, values: TiffValues::Long(vec![height]) });
    ifd0.push(TiffTag { tag: TAG_STRIP_BYTE_COUNTS, kind: TiffFieldType::Long, values: TiffValues::Long(vec![strip_bytes.len() as u32]) });
    ifd0.push(TiffTag { tag: TAG_STRIP_OFFSETS, kind: TiffFieldType::Long, values: TiffValues::Long(vec![0]) }); // placeholder, patched below
    ifd0.sort_by_key(|t| t.tag);

    // IFD 1..N: every entry the snapshot holds for that directory, with the three layout-dependent
    // strip tags recomputed from the strip bytes the directory itself carries (`TiffIfd::pixels`).
    // TIFF6 §Baseline makes `StripOffsets`/`RowsPerStrip`/`StripByteCounts` REQUIRED fields of a
    // strip-organised image directory, and this writer always re-lays a directory out as ONE
    // combined strip — so `RowsPerStrip` is forced to the full `ImageLength`, exactly as the single
    // `StripOffsets`/`StripByteCounts` entry it emits implies (a reader computing
    // `ceil(height / RowsPerStrip)` must arrive at 1). A directory that carries no strip bytes is
    // metadata only: its two pointer tags are dropped rather than pointed at nothing, and no
    // `RowsPerStrip` is invented for a raster that does not exist.
    let mut entries_per_ifd: Vec<Vec<TiffTag>> = Vec::with_capacity(snap.ifds.len().max(1));
    entries_per_ifd.push(ifd0);
    for ifd in snap.ifds.iter().skip(1) {
        let mut entries: Vec<TiffTag> = ifd.entries.iter().filter(|t| t.tag != TAG_STRIP_OFFSETS && t.tag != TAG_STRIP_BYTE_COUNTS).cloned().collect();
        if !ifd.pixels.is_empty() {
            if let Some(length) = entries.iter().find(|t| t.tag == TAG_IMAGE_LENGTH).and_then(|t| t.values.first_u32()) {
                entries.retain(|t| t.tag != TAG_ROWS_PER_STRIP);
                entries.push(TiffTag { tag: TAG_ROWS_PER_STRIP, kind: TiffFieldType::Long, values: TiffValues::Long(vec![length]) });
            }
            entries.push(TiffTag { tag: TAG_STRIP_BYTE_COUNTS, kind: TiffFieldType::Long, values: TiffValues::Long(vec![ifd.pixels.len() as u32]) });
            entries.push(TiffTag { tag: TAG_STRIP_OFFSETS, kind: TiffFieldType::Long, values: TiffValues::Long(vec![0]) });
            // placeholder, patched below
        }
        entries.sort_by_key(|t| t.tag); // TIFF6 §2: entries "must be sorted in ascending order by Tag".
        entries_per_ifd.push(entries);
    }

    // Layout pass: every IFD's directory + its own out-of-line value block, back to back
    // (mirroring `decode_tiff`'s own chain — each directory ends with a 4-byte next-IFD offset,
    // the last with 0), THEN every directory's strip payload in IFD order — IFD 0's decoded raster
    // first, then each later directory's own raw strips.
    let mut cursor = 8usize;
    let mut dir_offsets = Vec::with_capacity(entries_per_ifd.len());
    for entries in &entries_per_ifd {
        dir_offsets.push(cursor);
        cursor += dir_size(entries.len()) + out_of_line_size(entries, snap.byte_order);
    }
    let pixel_data_offset = cursor;
    if let Some(t) = entries_per_ifd[0].iter_mut().find(|t| t.tag == TAG_STRIP_OFFSETS) {
        t.values = TiffValues::Long(vec![pixel_data_offset as u32]); // Long/count1 stays inline: doesn't move the layout.
    }
    cursor += strip_bytes.len();
    for (i, ifd) in snap.ifds.iter().enumerate().skip(1) {
        if ifd.pixels.is_empty() {
            continue;
        }
        if let Some(t) = entries_per_ifd[i].iter_mut().find(|t| t.tag == TAG_STRIP_OFFSETS) {
            t.values = TiffValues::Long(vec![cursor as u32]);
        }
        cursor += ifd.pixels.len();
    }

    let mut out = Vec::new();
    match snap.byte_order {
        TiffByteOrder::LittleEndian => out.extend_from_slice(b"II"),
        TiffByteOrder::BigEndian => out.extend_from_slice(b"MM"),
    }
    write_u16(&mut out, 42, snap.byte_order);
    write_u32(&mut out, 8, snap.byte_order); // first IFD offset

    for (i, entries) in entries_per_ifd.iter().enumerate() {
        debug_assert_eq!(out.len(), dir_offsets[i], "computed IFD layout must match actual bytes written");
        write_u16(&mut out, entries.len() as u16, snap.byte_order);
        let out_of_line_start = dir_offsets[i] + dir_size(entries.len());
        let mut oo_cursor = out_of_line_start;
        for t in entries {
            write_u16(&mut out, t.tag, snap.byte_order);
            write_u16(&mut out, t.kind.to_u16(), snap.byte_order);
            write_u32(&mut out, t.values.count(), snap.byte_order);
            let vb = value_bytes(&t.values, snap.byte_order);
            if vb.len() <= 4 {
                let mut field = [0u8; 4];
                field[..vb.len()].copy_from_slice(&vb);
                out.extend_from_slice(&field);
            } else {
                write_u32(&mut out, oo_cursor as u32, snap.byte_order);
                oo_cursor += vb.len() + (vb.len() % 2);
            }
        }
        let next_ifd_offset = if i + 1 < dir_offsets.len() { dir_offsets[i + 1] as u32 } else { 0 };
        write_u32(&mut out, next_ifd_offset, snap.byte_order);
        for t in entries {
            let vb = value_bytes(&t.values, snap.byte_order);
            if vb.len() > 4 {
                out.extend_from_slice(&vb);
                if vb.len() % 2 == 1 {
                    out.push(0);
                }
            }
        }
    }
    debug_assert_eq!(out.len(), pixel_data_offset, "computed layout must match actual bytes written");
    out.extend_from_slice(&strip_bytes);
    for ifd in snap.ifds.iter().skip(1) {
        out.extend_from_slice(&ifd.pixels);
    }
    debug_assert_eq!(out.len(), cursor, "computed strip layout must match actual bytes written");
    Ok(out)
}

/// 🚫 MultiIfdEncodeScopeNote: see `encode_tiff_with`. Uncompressed (`Compression` 1) variant —
/// the historical default kept so `print_dsl`/`encode_pack_with` and the io export serializer
/// (which both call this exact function) are unaffected.
pub fn encode_tiff(snap: &TiffSnapshot) -> Result<Vec<u8>, String> {
    encode_tiff_with(snap, false)
}

/// 📦 Same shape as `encode_tiff` but real-PackBits-compresses the strip (`Compression` 32773).
pub fn encode_tiff_packbits(snap: &TiffSnapshot) -> Result<Vec<u8>, String> {
    encode_tiff_with(snap, true)
}
//#endregion Encode

//#region Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion Tests

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::standards::v6_0::subsets::baseline::schema::TiffBaselineComposer;
    use crate::standards::v6_0::subsets::document::schema::TiffComposer as TiffRawAnyComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<TiffRawAnyComposer>(), composer_entry_of::<TiffBaselineComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
