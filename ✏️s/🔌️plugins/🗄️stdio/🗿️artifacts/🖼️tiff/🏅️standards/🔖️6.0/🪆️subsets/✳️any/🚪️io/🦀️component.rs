//! 🚪️ IO stdio.tiff (6.0/✳️any) — registration now flows through 🎹️composer::register
//! (called once from 🔌️plugin/🔧️setup via ⚙️engine::register), not per-leaf register().
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::tiff::standards::v6_0::subsets::any::schema::TiffAnalyzer;
    use crate::artifacts::tiff::TiffSnapshot;
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
use crate::artifacts::tiff::schema::snapshot::{
    TiffByteOrder, TiffFieldType, TiffIfd, TiffSnapshot, TiffTag, TiffValues, TAG_BITS_PER_SAMPLE, TAG_COMPRESSION, TAG_IMAGE_LENGTH, TAG_IMAGE_WIDTH, TAG_PHOTOMETRIC, TAG_ROWS_PER_STRIP, TAG_SAMPLES_PER_PIXEL, TAG_STRIP_BYTE_COUNTS,
    TAG_STRIP_OFFSETS,
};
use crate::artifacts::tiff::STDIO_TIFF_DOCUMENT_SCHEMA;

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
fn tag_values<'a>(ifd: &'a TiffIfd, tag: u16) -> Option<&'a TiffValues> {
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
            out.extend(std::iter::repeat(b).take(count));
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
            entries.push(TiffTag { tag: TAG_STRIP_OFFSETS, kind: TiffFieldType::Long, values: TiffValues::Long(vec![0]) }); // placeholder, patched below
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
mod tests {
    use super::*;
    use crate::artifacts::tiff::schema::demo_tiff_snapshot;
    use crate::artifacts::tiff::schema::snapshot::{TiffFieldType, TiffValues};

    async fn gradient_checkerboard_rgba(w: u32, h: u32) -> Vec<u8> {
        let mut out = Vec::with_capacity((w * h * 4) as usize);
        for y in 0..h {
            for x in 0..w {
                let checker = if (x + y) % 2 == 0 { 255u8 } else { 0u8 };
                out.extend_from_slice(&[checker, ((x * 37) % 256) as u8, ((y * 53) % 256) as u8, 255]);
            }
        }
        out
    }

    async fn ifd0_snapshot(width: u32, height: u32) -> TiffIfd {
        TiffIfd { pixels: Vec::new(), entries: vec![TiffTag { tag: TAG_IMAGE_WIDTH, kind: TiffFieldType::Long, values: TiffValues::Long(vec![width]) }, TiffTag { tag: TAG_IMAGE_LENGTH, kind: TiffFieldType::Long, values: TiffValues::Long(vec![height]) }] }
    }

    /// 🔬 Load-bearing regression: non-solid 9x5 checkerboard/gradient round-tripped through the
    /// real uncompressed IFD codec.
    #[semio_framework_async_macros::async_test]
    async fn gradient_checkerboard_uncompressed_round_trip() {
        let (w, h) = (9u32, 5u32);
        let rgba = gradient_checkerboard_rgba(w, h).await;
        let snap = TiffSnapshot { schema: STDIO_TIFF_DOCUMENT_SCHEMA.into(), byte_order: TiffByteOrder::LittleEndian, ifds: vec![ifd0_snapshot(w, h).await], pixels: rgba.clone() };
        let encoded = encode_tiff(&snap).expect("encode");
        let decoded = decode_tiff(&encoded).expect("decode");
        assert_eq!(decoded.width(), Some(w));
        assert_eq!(decoded.height(), Some(h));
        assert_eq!(decoded.pixels, rgba, "decoded pixels must exactly match the original");
    }

    /// 🔬 Same fixture through real PackBits encode+decode — proves PackBits compression is
    /// actually exercised (not just pass-through), by asserting the compressed strip is smaller
    /// than the raw RGB and that decode reconstructs the exact original pixels.
    #[semio_framework_async_macros::async_test]
    async fn gradient_checkerboard_packbits_round_trip() {
        let (w, h) = (9u32, 5u32);
        let rgba = gradient_checkerboard_rgba(w, h).await;
        let snap = TiffSnapshot { schema: STDIO_TIFF_DOCUMENT_SCHEMA.into(), byte_order: TiffByteOrder::LittleEndian, ifds: vec![ifd0_snapshot(w, h).await], pixels: rgba.clone() };
        let encoded = encode_tiff_packbits(&snap).expect("encode packbits");
        let decoded = decode_tiff(&encoded).expect("decode packbits");
        assert_eq!(decoded.width(), Some(w));
        assert_eq!(decoded.height(), Some(h));
        assert_eq!(decoded.pixels, rgba, "packbits round trip must exactly match the original");
    }

    /// 🔬 PackBits actually runs real repeat/literal RLE, not a pass-through: a solid-color strip
    /// (long repeat runs) must compress to fewer bytes than the raw RGB.
    #[semio_framework_async_macros::async_test]
    async fn packbits_compresses_repetitive_data() {
        let (w, h) = (20u32, 10u32);
        let rgba: Vec<u8> = (0..w * h).flat_map(|_| [128u8, 128, 128, 255]).collect();
        let snap = TiffSnapshot { schema: STDIO_TIFF_DOCUMENT_SCHEMA.into(), byte_order: TiffByteOrder::LittleEndian, ifds: vec![ifd0_snapshot(w, h).await], pixels: rgba.clone() };
        let uncompressed = encode_tiff(&snap).expect("encode uncompressed");
        let encoded = encode_tiff_packbits(&snap).expect("encode packbits");
        assert!(encoded.len() < uncompressed.len(), "packbits must shrink a byte-repetitive strip below the uncompressed encoding ({} !< {})", encoded.len(), uncompressed.len());
        let decoded = decode_tiff(&encoded).expect("decode packbits");
        assert_eq!(decoded.pixels, rgba);
    }

    #[semio_framework_async_macros::async_test]
    async fn packbits_hand_decode_control_bytes() {
        // literal run of 3 (10,20,30), then repeat run of 5x99, then literal run of 2 (1,2)
        let encoded: [u8; 9] = [2, 10, 20, 30, 0xFC, 99, 1, 1, 2];
        let expected: Vec<u8> = vec![10, 20, 30, 99, 99, 99, 99, 99, 1, 2];
        let decoded = packbits_decode(&encoded, expected.len()).expect("decode");
        assert_eq!(decoded, expected);
        let re_encoded = packbits_encode(&expected);
        let re_decoded = packbits_decode(&re_encoded, expected.len()).expect("re-decode");
        assert_eq!(re_decoded, expected);
    }

    /// 🔬 Big-endian (`MM`) byte order must decode correctly too, AND encode must round-trip
    /// `byte_order` itself (real round-trip, not always-little-endian).
    #[semio_framework_async_macros::async_test]
    async fn big_endian_round_trip() {
        let (w, h) = (2u32, 1u32);
        let rgba = vec![10u8, 20, 30, 255, 40, 50, 60, 255];
        let snap = TiffSnapshot { schema: STDIO_TIFF_DOCUMENT_SCHEMA.into(), byte_order: TiffByteOrder::BigEndian, ifds: vec![ifd0_snapshot(w, h).await], pixels: rgba.clone() };
        let encoded = encode_tiff(&snap).expect("encode big-endian");
        assert_eq!(&encoded[0..2], b"MM", "encode must honor byte_order, not always little-endian");
        let decoded = decode_tiff(&encoded).expect("decode big-endian tiff");
        assert_eq!(decoded.byte_order, TiffByteOrder::BigEndian);
        assert_eq!(decoded.width(), Some(w));
        assert_eq!(decoded.height(), Some(h));
        assert_eq!(decoded.pixels, rgba);
    }

    /// 🔬 A non-core tag (`Artist`, ASCII, out-of-line since its value exceeds 4 bytes) set on
    /// `ifds[0]` must survive an encode/decode round trip verbatim — proves the generic
    /// tag/type/value model, not just the hardcoded strip-geometry tags.
    #[semio_framework_async_macros::async_test]
    async fn carried_ascii_tag_round_trips() {
        let (w, h) = (2u32, 2u32);
        let rgba = vec![1u8; (w * h * 4) as usize];
        let mut ifd = ifd0_snapshot(w, h).await;
        ifd.entries.push(TiffTag { tag: 315, kind: TiffFieldType::Ascii, values: TiffValues::Ascii("A Real Artist".into()) });
        let snap = TiffSnapshot { schema: STDIO_TIFF_DOCUMENT_SCHEMA.into(), byte_order: TiffByteOrder::LittleEndian, ifds: vec![ifd], pixels: rgba };
        let encoded = encode_tiff(&snap).expect("encode");
        let decoded = decode_tiff(&encoded).expect("decode");
        let artist = decoded.tag(315).expect("Artist tag must survive round trip");
        assert_eq!(artist.values, TiffValues::Ascii("A Real Artist".into()));
    }

    /// 🔬 A short (inline) non-core numeric tag also survives.
    #[semio_framework_async_macros::async_test]
    async fn carried_short_tag_round_trips() {
        let (w, h) = (2u32, 2u32);
        let rgba = vec![1u8; (w * h * 4) as usize];
        let mut ifd = ifd0_snapshot(w, h).await;
        ifd.entries.push(TiffTag { tag: 296, kind: TiffFieldType::Short, values: TiffValues::Short(vec![2]) }); // ResolutionUnit
        let snap = TiffSnapshot { schema: STDIO_TIFF_DOCUMENT_SCHEMA.into(), byte_order: TiffByteOrder::LittleEndian, ifds: vec![ifd], pixels: rgba };
        let encoded = encode_tiff(&snap).expect("encode");
        let decoded = decode_tiff(&encoded).expect("decode");
        assert_eq!(decoded.tag(296).expect("ResolutionUnit must survive").values, TiffValues::Short(vec![2]));
    }

    /// 🔬 Real multi-IFD encode: a genuinely two-IFD snapshot round-trips through
    /// `encode_tiff`/`decode_tiff` with BOTH directories intact — the `next IFD offset` chain
    /// `decode_tiff` walks is actually written, not dropped, and IFD 1's own non-strip tags
    /// survive verbatim even though it carries no backing pixel data.
    #[semio_framework_async_macros::async_test]
    async fn multi_ifd_round_trip_preserves_every_ifd() {
        let (w, h) = (2u32, 2u32);
        let rgba = vec![7u8; (w * h * 4) as usize];
        let ifd1 = TiffIfd { pixels: Vec::new(), entries: vec![TiffTag { tag: 270, kind: TiffFieldType::Ascii, values: TiffValues::Ascii("second page".into()) }] };
        let snap = TiffSnapshot { schema: STDIO_TIFF_DOCUMENT_SCHEMA.into(), byte_order: TiffByteOrder::LittleEndian, ifds: vec![ifd0_snapshot(w, h).await, ifd1], pixels: rgba.clone() };
        let encoded = encode_tiff(&snap).expect("encode multi-ifd");
        let decoded = decode_tiff(&encoded).expect("decode multi-ifd");
        assert_eq!(decoded.ifds.len(), 2, "both IFDs must survive the real chain");
        assert_eq!(decoded.pixels, rgba, "IFD 0's raster must be unaffected by a second IFD existing");
        let second = decoded.ifds[1].entries.iter().find(|t| t.tag == 270).expect("IFD 1's own tag must survive");
        assert_eq!(second.values, TiffValues::Ascii("second page".into()));
    }

    /// 🔬 THE regression this wave exists for (`📓️w13-final-audit.md` §2.2(12)): a secondary
    /// directory that CARRIES raster must come back with its raster AND with the three strip tags
    /// TIFF6 §Baseline makes required of it — `StripOffsets`, `RowsPerStrip` (forced to the page's
    /// own `ImageLength`, since this writer always re-lays a directory out as one combined strip)
    /// and `StripByteCounts`. Before `TiffIfd::pixels` existed the encoder had nothing to back a
    /// non-primary raster with, dropped the two pointer tags and never emitted `RowsPerStrip`, so
    /// every round trip of a real multi-page file silently destroyed page 2 — measured by nothing,
    /// because the semantic projection only decodes IFD 0's raster.
    #[semio_framework_async_macros::async_test]
    async fn secondary_ifd_raster_and_its_required_strip_tags_survive_the_codec() {
        let (w, h) = (2u32, 2u32);
        let rgba = vec![7u8; (w * h * 4) as usize];
        let page2: Vec<u8> = (0u8..12).collect(); // 2x2 chunky RGB = 12 bytes
        let ifd1 = TiffIfd {
            entries: vec![
                TiffTag { tag: TAG_IMAGE_WIDTH, kind: TiffFieldType::Long, values: TiffValues::Long(vec![2]) },
                TiffTag { tag: TAG_IMAGE_LENGTH, kind: TiffFieldType::Long, values: TiffValues::Long(vec![2]) },
                TiffTag { tag: TAG_BITS_PER_SAMPLE, kind: TiffFieldType::Short, values: TiffValues::Short(vec![8, 8, 8]) },
                TiffTag { tag: TAG_SAMPLES_PER_PIXEL, kind: TiffFieldType::Short, values: TiffValues::Short(vec![3]) },
            ],
            pixels: page2.clone(),
        };
        let snap = TiffSnapshot { schema: STDIO_TIFF_DOCUMENT_SCHEMA.into(), byte_order: TiffByteOrder::LittleEndian, ifds: vec![ifd0_snapshot(w, h).await, ifd1], pixels: rgba.clone() };
        let decoded = decode_tiff(&encode_tiff(&snap).expect("encode")).expect("decode");
        assert_eq!(decoded.ifds.len(), 2);
        assert_eq!(decoded.pixels, rgba, "IFD 0's raster must be unaffected");
        assert_eq!(decoded.ifds[1].pixels, page2, "IFD 1's own strip bytes must survive the round trip");
        let rows_per_strip = decoded.ifds[1].entries.iter().find(|t| t.tag == TAG_ROWS_PER_STRIP).expect("a strip-organised IFD must declare RowsPerStrip");
        assert_eq!(rows_per_strip.values, TiffValues::Long(vec![2]), "one combined strip means RowsPerStrip == ImageLength");
        // 🧭 The two pointer tags are layout, recomputed on write and folded back into `pixels` on
        // read, so a second round trip is a fixpoint rather than a drift of stale offsets.
        assert!(!decoded.ifds[1].entries.iter().any(|t| t.tag == TAG_STRIP_OFFSETS || t.tag == TAG_STRIP_BYTE_COUNTS), "strip pointers belong to the layout, not to the snapshot");
        let twice = decode_tiff(&encode_tiff(&decoded).expect("re-encode")).expect("re-decode");
        assert_eq!(twice.ifds, decoded.ifds, "decode(encode(x)) must be a fixpoint for every directory");
    }

    /// 🔬 `InsertIfd`/`RemoveIfd` genuinely observable THROUGH THE CODEC, not merely in the
    /// in-memory `TiffSnapshot`: apply the mutation, encode to real bytes, decode those bytes back
    /// with the independent `decode_tiff` chain walk, and see the directory actually appear/vanish.
    #[semio_framework_async_macros::async_test]
    async fn insert_ifd_and_remove_ifd_are_observable_through_the_codec() {
        use crate::artifacts::tiff::schema::mutations::apply_tiff_mutation;
        use crate::artifacts::tiff::TiffMutation;

        let (w, h) = (2u32, 2u32);
        let rgba = vec![3u8; (w * h * 4) as usize];
        let base = TiffSnapshot { schema: STDIO_TIFF_DOCUMENT_SCHEMA.into(), byte_order: TiffByteOrder::LittleEndian, ifds: vec![ifd0_snapshot(w, h).await], pixels: rgba };
        let mut snapshot = decode_tiff(&encode_tiff(&base).expect("encode base")).expect("decode base");
        assert_eq!(snapshot.ifds.len(), 1);

        let inserted = TiffIfd { pixels: Vec::new(), entries: vec![TiffTag { tag: 270, kind: TiffFieldType::Ascii, values: TiffValues::Ascii("inserted page".into()) }] };
        apply_tiff_mutation(&mut snapshot, &TiffMutation::InsertIfd { index: 1, ifd: inserted });
        let after_insert = decode_tiff(&encode_tiff(&snapshot).expect("encode after insert")).expect("decode after insert");
        assert_eq!(after_insert.ifds.len(), 2, "InsertIfd must add a real, decodable second directory");
        let tag = after_insert.ifds[1].entries.iter().find(|t| t.tag == 270).expect("inserted IFD's tag must survive the codec");
        assert_eq!(tag.values, TiffValues::Ascii("inserted page".into()));

        apply_tiff_mutation(&mut snapshot, &TiffMutation::RemoveIfd { index: 1 });
        let after_remove = decode_tiff(&encode_tiff(&snapshot).expect("encode after remove")).expect("decode after remove");
        assert_eq!(after_remove.ifds.len(), 1, "RemoveIfd must genuinely drop the directory from the encoded chain");
    }

    #[semio_framework_async_macros::async_test]
    async fn sniff_rejects_non_tiff_bytes() {
        let err = decode_tiff(b"not a tiff at all").unwrap_err();
        assert!(err.contains("byte-order"));
    }

    #[semio_framework_async_macros::async_test]
    async fn unsupported_compression_is_a_typed_error() {
        let (w, h) = (2u32, 2u32);
        let mut ifd = ifd0_snapshot(w, h).await;
        ifd.entries.push(TiffTag { tag: TAG_BITS_PER_SAMPLE, kind: TiffFieldType::Short, values: TiffValues::Short(vec![8]) });
        ifd.entries.push(TiffTag { tag: TAG_COMPRESSION, kind: TiffFieldType::Short, values: TiffValues::Short(vec![5]) }); // LZW — intentionally unsupported
        ifd.entries.push(TiffTag { tag: TAG_SAMPLES_PER_PIXEL, kind: TiffFieldType::Short, values: TiffValues::Short(vec![3]) });
        ifd.entries.push(TiffTag { tag: TAG_STRIP_OFFSETS, kind: TiffFieldType::Long, values: TiffValues::Long(vec![0]) });
        ifd.entries.sort_by_key(|t| t.tag);

        // Hand-encode a minimal file carrying this IFD (bypassing `encode_tiff`, which always
        // canonicalizes `Compression` itself) to exercise `decode_tiff`'s own rejection path.
        let snap = TiffSnapshot { schema: STDIO_TIFF_DOCUMENT_SCHEMA.into(), byte_order: TiffByteOrder::LittleEndian, ifds: vec![ifd], pixels: Vec::new() };
        let dir_offset = 8usize;
        let entries = &snap.ifds[0].entries;
        let mut out = Vec::new();
        out.extend_from_slice(b"II");
        write_u16(&mut out, 42, TiffByteOrder::LittleEndian);
        write_u32(&mut out, dir_offset as u32, TiffByteOrder::LittleEndian);
        write_u16(&mut out, entries.len() as u16, TiffByteOrder::LittleEndian);
        for t in entries {
            write_u16(&mut out, t.tag, TiffByteOrder::LittleEndian);
            write_u16(&mut out, t.kind.to_u16(), TiffByteOrder::LittleEndian);
            write_u32(&mut out, t.values.count(), TiffByteOrder::LittleEndian);
            let vb = value_bytes(&t.values, TiffByteOrder::LittleEndian);
            let mut field = [0u8; 4];
            field[..vb.len().min(4)].copy_from_slice(&vb[..vb.len().min(4)]);
            out.extend_from_slice(&field);
        }
        write_u32(&mut out, 0, TiffByteOrder::LittleEndian);
        let err = decode_tiff(&out).unwrap_err();
        assert!(err.contains("unsupported compression"), "unexpected error: {err}");
    }

    //#region 🔖️ConformanceLaws
    /// 🧪️ P2-FG2: per-artifact conformance laws (`📖️grammar-recipe.md` §4's checklist item 6) —
    /// grammar/protocol parseability, `Recognizer` against real fixtures AND real `print_op`/
    /// `print_diff` output, `walk_protocol` against real `encode_pack`/`encode_op`/`encode_diff`
    /// bytes, and the fixture-honesty round-trip. Relocated verbatim from `⚙️engine`'s own test
    /// region (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — mirrors png's own
    /// `conformance_laws` module shape exactly.
    mod conformance_laws {
        use super::*;
        use crate::artifacts::tiff::schema::{diff, mutations, snapshot};
        use protocol::{DiffCodec, OpBinary, OpText};

        /// ✅️ "committed files parse": all 6 handcrafted `.grammar.semio`/`.protocol.semio`
        /// files parse under the real dialect — independent of, and cheaper than, the two
        /// `recognize`/`walk_protocol` laws below (a parse failure here fails fast with a
        /// clearer message).
        #[semio_framework_async_macros::async_test]
        async fn committed_facet_files_parse() {
            for (label, text) in [("snapshot grammar", snapshot::text::COMPONENT_GRAMMAR_SEMIO), ("mutations grammar", mutations::text::COMPONENT_GRAMMAR_SEMIO), ("diff grammar", diff::text::COMPONENT_GRAMMAR_SEMIO)] {
                let grammar = dsl::parse_grammar(text).unwrap_or_else(|e| panic!("{label}: parse_grammar failed: {e:?}"));
                assert_eq!(grammar.dialect, dsl::SemioDialect::Grammar, "{label}: expected grammar dialect");
            }
            for (label, text) in [("snapshot protocol", snapshot::binary::COMPONENT_PROTOCOL_SEMIO), ("mutations protocol", mutations::binary::COMPONENT_PROTOCOL_SEMIO), ("diff protocol", diff::binary::COMPONENT_PROTOCOL_SEMIO)] {
                dsl::parse_protocol(text).unwrap_or_else(|e| panic!("{label}: parse_protocol failed: {e:?}"));
            }
        }

        /// ✅️ `grammar_conformance_law`: the snapshot grammar (a hex-dump grammar — TIFF has no
        /// textual syntax of its own, see that file's own doc comment) recognizes real
        /// `print_dsl` output for the demo snapshot — same preamble-stripped body
        /// reconstruction `m5_handcrafted_grammar_conformance`'s own `dsl_body_from_fixture`
        /// uses, so this is a direct proof this artifact will pass that harness once graduated,
        /// not merely an analogue.
        #[semio_framework_async_macros::async_test]
        async fn grammar_conformance_law() {
            let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            let text = store::ArtifactDsl::print_dsl(&demo_tiff_snapshot());
            let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
            let reconstructed = format!("{}\n{body}", envelope.envelope_id());
            assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");
        }

        /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op`
        /// output for every `TiffMutation` variant (`mutations::demo_mutation_cases()`).
        #[semio_framework_async_macros::async_test]
        async fn ops_grammar_conformance_law() {
            let grammar = dsl::parse_grammar(mutations::text::COMPONENT_GRAMMAR_SEMIO).expect("parse mutations grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            for mutation in mutations::demo_mutation_cases() {
                let printed = mutation.print_op();
                assert!(recognizer.recognize(&printed).unwrap_or(false), "mutations grammar did not recognize {printed:?} (from {mutation:?})");
            }
        }

        /// ✅️ `diff_grammar_conformance_law`: the diff grammar recognizes real `print_diff`
        /// output for every representative `TiffDiff` (`diff::demo_diff_cases()`), incl. the
        /// empty diff, every IFD-level/tag-level collection-triple shape, and every
        /// `TiffValues` field-type family.
        #[semio_framework_async_macros::async_test]
        async fn diff_grammar_conformance_law() {
            let grammar = dsl::parse_grammar(diff::text::COMPONENT_GRAMMAR_SEMIO).expect("parse diff grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            for d in diff::demo_diff_cases() {
                let printed = d.print_diff();
                assert!(recognizer.recognize(&printed).unwrap_or(false), "diff grammar did not recognize {printed:?} (from {d:?})");
            }
        }

        /// ✅️ `protocol_walk_law`: `walk_protocol` against REAL bytes for all three facets —
        /// snapshot pack (`encode_pack`, envelope-unwrapped first, matching how
        /// `m5_handcrafted_protocol_conformance` itself feeds `walk_protocol`), every demo
        /// mutation's `encode_op`, and every demo diff's `encode_diff`. The snapshot protocol
        /// only describes the real 8-byte header + first-IFD entry-count field as INDIVIDUALLY
        /// typed fields (§ this standard's own protocol.semio doc comment: IFD-entry-array/
        /// out-of-line-offset/IFD-chain resolution are honest mechanism gaps) before the
        /// trailing `chain rest bytes` consumes everything past that point — so `consumed ==
        /// bytes.len()` still holds exactly for every facet, same as the op/diff protocols.
        #[semio_framework_async_macros::async_test]
        async fn protocol_walk_law() {
            let pack_spec = dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
            let packed = store::ArtifactPack::encode_pack(&demo_tiff_snapshot());
            let (_, inner) = store::semio_format::unwrap_binary(&packed).expect("unwrap semio envelope");
            let trace = dsl::walk_protocol(&pack_spec, &inner).unwrap_or_else(|e| panic!("walk_protocol(pack) failed @{}: {}", e.offset, e.message));
            assert_eq!(trace.consumed, inner.len(), "pack walk did not consume every byte");

            let op_spec = dsl::parse_protocol(mutations::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse mutations protocol");
            for mutation in mutations::demo_mutation_cases() {
                let bytes = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op failed for {mutation:?}: {e:?}"));
                let trace = dsl::walk_protocol(&op_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(op) failed for {mutation:?} @{}: {}", e.offset, e.message));
                assert_eq!(trace.consumed, bytes.len(), "op walk did not consume every byte for {mutation:?}");
            }

            let diff_spec = dsl::parse_protocol(diff::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse diff protocol");
            for d in diff::demo_diff_cases() {
                let bytes = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed for {d:?}: {e:?}"));
                let trace = dsl::walk_protocol(&diff_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(diff) failed for {d:?} @{}: {}", e.offset, e.message));
                assert_eq!(trace.consumed, bytes.len(), "diff walk did not consume every byte for {d:?}");
            }
        }

        /// ✅️ `fixture_honesty_law`: the shipped `.dsl.semio`/`.pack.semio` fixtures are
        /// GENUINE `print_dsl`/`encode_pack` output of `demo_tiff_snapshot()` —
        /// `parse_dsl(fixture) == demo()`, `print_dsl(demo()) == fixture` (byte-for-byte), and
        /// the pack twin — so the fixtures can never silently drift back to a fake again.
        #[semio_framework_async_macros::async_test]
        async fn fixture_honesty_law() {
            const FIXTURE_DSL: &str = include_str!("../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");
            const FIXTURE_PACK: &[u8] = include_bytes!("../📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio");

            let demo = demo_tiff_snapshot();

            let parsed = <TiffSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
            assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_tiff_snapshot()");
            assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_tiff_snapshot()) drifted from the shipped .dsl.semio fixture");

            let decoded = <TiffSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
            assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_tiff_snapshot()");
            assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_tiff_snapshot()) drifted from the shipped .pack.semio fixture");

            let native = encode_tiff(&demo).expect("encode native tiff");
            assert_eq!(native.as_slice(), include_bytes!("../📚️examples/🎬️demo/🖼️assets/🖼️example.tiff"), "encode_tiff(demo) drifted from 🖼️example.tiff");
        }

        #[semio_framework_async_macros::async_test]
        #[ignore]
        async fn zzz_write_native_tiff_fixture() {
            let demo = demo_tiff_snapshot();
            let native = encode_tiff(&demo).expect("encode");
            let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🖼️example.tiff");
            std::fs::write(path, native).expect("write 🖼️example.tiff");
        }
    }
    //#endregion 🔖️ConformanceLaws
}
//#endregion Tests

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::artifacts::tiff::standards::v6_0::subsets::any::schema::TiffComposer as TiffRawAnyComposer;
    use crate::artifacts::tiff::standards::v6_0::subsets::baseline::schema::TiffBaselineComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<TiffRawAnyComposer>(), composer_entry_of::<TiffBaselineComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
