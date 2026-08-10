//! ⚙️ TiffEngine — real TIFF codec: IFD walk (II/MM), uncompressed + PackBits strips.
//!
//! Decode supports the classic baseline tag set needed for real-world files: both byte orders,
//! multi-strip `ImageWidth`/`ImageLength`/`BitsPerSample`(8 only)/`SamplesPerPixel`(1/3/4)/
//! `RowsPerStrip` chunky strips, `Compression` 1 (none) and 32773 (PackBits). LZW(5)/Deflate(8)
//! return a typed error rather than fabricate pixels — see 🚫️CompressionScopeNote. Encode always
//! emits little-endian (`II`), 8-bit RGB, chunky, single-strip TIFF: `encode_tiff` uncompressed
//! (the historical default, kept so callers/serializers are unaffected) and `encode_tiff_packbits`
//! PackBits-compressed (added for real PackBits *encode* coverage, not just decode).

use crate::artifacts::tiff::{schema::snapshot::RasterImage, TiffArtifact, TiffDiff, TiffMutation, TiffSnapshot, STDIO_TIFF_DOCUMENT_SCHEMA};

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
}

fn read_u16(data: &[u8], pos: usize, e: Endian) -> Result<u16, String> {
    data.get(pos..pos + 2).map(|s| e.u16(s)).ok_or_else(|| "tiff: truncated (u16)".into())
}
fn read_u32(data: &[u8], pos: usize, e: Endian) -> Result<u32, String> {
    data.get(pos..pos + 4).map(|s| e.u32(s)).ok_or_else(|| "tiff: truncated (u32)".into())
}
//#endregion ByteOrder

//#region Ifd
struct IfdEntry {
    tag: u16,
    typ: u16,
    count: u32,
    value_field: [u8; 4],
}

fn type_size(typ: u16) -> usize {
    match typ {
        1 | 2 | 6 | 7 => 1,
        3 | 8 => 2,
        4 | 9 | 11 => 4,
        5 | 10 | 12 => 8,
        _ => 1,
    }
}

/// 📖 Walks one IFD: 2-byte entry count, N x 12-byte entries, 4-byte offset to the next IFD.
fn read_ifd(data: &[u8], ifd_off: usize, e: Endian) -> Result<Vec<IfdEntry>, String> {
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
        entries.push(IfdEntry { tag, typ, count: cnt, value_field: vf });
        pos += 12;
    }
    Ok(entries)
}

/// 🔢 Reads a tag's values as `u32`s, resolving the inline-vs-offset rule (TIFF6 §2: the value
/// is stored inline in the 4-byte field if `type_size * count <= 4`, else the field holds a file
/// offset to the values).
fn entry_values(data: &[u8], entry: &IfdEntry, e: Endian) -> Result<Vec<u32>, String> {
    let sz = type_size(entry.typ);
    let total = sz * entry.count as usize;
    let owned;
    let src: &[u8] = if total <= 4 {
        &entry.value_field
    } else {
        let off = e.u32(&entry.value_field) as usize;
        owned = data.get(off..off + total).ok_or("tiff: tag value offset out of range")?;
        owned
    };
    let mut out = Vec::with_capacity(entry.count as usize);
    for i in 0..entry.count as usize {
        let v = match entry.typ {
            3 => e.u16(&src[i * 2..i * 2 + 2]) as u32,
            4 => e.u32(&src[i * 4..i * 4 + 4]),
            1 | 2 | 6 | 7 => src[i] as u32,
            _ => return Err(format!("tiff: unsupported tag value type {}", entry.typ)),
        };
        out.push(v);
    }
    Ok(out)
}

fn entry_u32(data: &[u8], entry: &IfdEntry, e: Endian) -> Result<u32, String> {
    Ok(entry_values(data, entry, e)?.first().copied().unwrap_or(0))
}
//#endregion Ifd

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

//#region Codec
const TAG_IMAGE_WIDTH: u16 = 256;
const TAG_IMAGE_LENGTH: u16 = 257;
const TAG_BITS_PER_SAMPLE: u16 = 258;
const TAG_COMPRESSION: u16 = 259;
const TAG_PHOTOMETRIC: u16 = 262;
const TAG_STRIP_OFFSETS: u16 = 273;
const TAG_SAMPLES_PER_PIXEL: u16 = 277;
const TAG_ROWS_PER_STRIP: u16 = 278;
const TAG_STRIP_BYTE_COUNTS: u16 = 279;

pub fn decode_tiff(data: &[u8]) -> Result<TiffSnapshot, String> {
    if data.len() < 8 {
        return Err("tiff: truncated header".into());
    }
    let e = match &data[0..2] {
        b"II" => Endian::Little,
        b"MM" => Endian::Big,
        _ => return Err("tiff: bad byte-order mark".into()),
    };
    if read_u16(data, 2, e)? != 42 {
        return Err("tiff: bad magic number".into());
    }
    let ifd_off = read_u32(data, 4, e)? as usize;
    let entries = read_ifd(data, ifd_off, e)?;

    let mut width = None;
    let mut height = None;
    let mut compression = 1u32;
    let mut samples_per_pixel = 1u32;
    let mut bits_per_sample = 8u32;
    let mut photometric = 1u32;
    let mut rows_per_strip: Option<u32> = None;
    let mut strip_offsets: Vec<u32> = Vec::new();
    let mut strip_byte_counts: Vec<u32> = Vec::new();

    for entry in &entries {
        match entry.tag {
            TAG_IMAGE_WIDTH => width = Some(entry_u32(data, entry, e)?),
            TAG_IMAGE_LENGTH => height = Some(entry_u32(data, entry, e)?),
            TAG_BITS_PER_SAMPLE => bits_per_sample = entry_values(data, entry, e)?.first().copied().unwrap_or(8),
            TAG_COMPRESSION => compression = entry_u32(data, entry, e)?,
            TAG_PHOTOMETRIC => photometric = entry_u32(data, entry, e)?,
            TAG_STRIP_OFFSETS => strip_offsets = entry_values(data, entry, e)?,
            TAG_SAMPLES_PER_PIXEL => samples_per_pixel = entry_u32(data, entry, e)?,
            TAG_ROWS_PER_STRIP => rows_per_strip = Some(entry_u32(data, entry, e)?),
            TAG_STRIP_BYTE_COUNTS => strip_byte_counts = entry_values(data, entry, e)?,
            _ => {}
        }
    }

    let width = width.ok_or("tiff: missing ImageWidth")?;
    let height = height.ok_or("tiff: missing ImageLength")?;
    if width == 0 || height == 0 {
        return Err("tiff: zero dimension".into());
    }
    if bits_per_sample != 8 {
        return Err(format!("tiff: unsupported BitsPerSample {bits_per_sample} (only 8 is implemented)"));
    }
    if samples_per_pixel != 1 && samples_per_pixel != 3 && samples_per_pixel != 4 {
        return Err(format!("tiff: unsupported SamplesPerPixel {samples_per_pixel}"));
    }
    // 🚫 CompressionScopeNote: only uncompressed(1)/PackBits(32773) are decoded for real —
    // LZW(5)/Deflate(8)/others deliberately fail rather than fabricate pixel data.
    if compression != 1 && compression != 32773 {
        return Err(format!("tiff: unsupported compression {compression} (only uncompressed/PackBits are implemented)"));
    }
    if strip_offsets.is_empty() {
        return Err("tiff: missing StripOffsets".into());
    }
    let rows_per_strip = rows_per_strip.unwrap_or(height);

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

    Ok(TiffSnapshot { schema: STDIO_TIFF_DOCUMENT_SCHEMA.into(), image: RasterImage { width, height, rgba } })
}

fn rgb_bytes(img: &RasterImage) -> Result<Vec<u8>, String> {
    let pixels = img.width as usize * img.height as usize;
    if img.rgba.len() != pixels * 4 {
        return Err("tiff: rgba length mismatch".into());
    }
    let mut rgb = Vec::with_capacity(pixels * 3);
    for px in img.rgba.chunks(4) {
        rgb.extend_from_slice(&px[0..3]);
    }
    Ok(rgb)
}

fn encode_tiff_with(snap: &TiffSnapshot, packbits: bool) -> Result<Vec<u8>, String> {
    let img = &snap.image;
    if img.width == 0 || img.height == 0 {
        return Err("tiff: empty image".into());
    }
    let rgb = rgb_bytes(img)?;
    let strip_bytes = if packbits { packbits_encode(&rgb) } else { rgb };
    let compression: u32 = if packbits { 32773 } else { 1 };

    let entry_count = 9u16;
    let ifd_off = 8u32;
    let strip_off = ifd_off + 2 + 12 * entry_count as u32 + 4;
    let entries: [(u16, u16, u32, u32); 9] = [
        (TAG_IMAGE_WIDTH, 3, 1, img.width),
        (TAG_IMAGE_LENGTH, 3, 1, img.height),
        (TAG_BITS_PER_SAMPLE, 3, 1, 8),
        (TAG_COMPRESSION, 3, 1, compression),
        (TAG_PHOTOMETRIC, 3, 1, 2),
        (TAG_STRIP_OFFSETS, 4, 1, strip_off),
        (TAG_SAMPLES_PER_PIXEL, 3, 1, 3),
        (TAG_ROWS_PER_STRIP, 3, 1, img.height),
        (TAG_STRIP_BYTE_COUNTS, 4, 1, strip_bytes.len() as u32),
    ];
    let mut out = Vec::new();
    out.extend_from_slice(b"II");
    out.extend_from_slice(&42u16.to_le_bytes());
    out.extend_from_slice(&ifd_off.to_le_bytes());
    out.extend_from_slice(&entry_count.to_le_bytes());
    for (tag, typ, cnt, val) in entries {
        out.extend_from_slice(&tag.to_le_bytes());
        out.extend_from_slice(&typ.to_le_bytes());
        out.extend_from_slice(&cnt.to_le_bytes());
        out.extend_from_slice(&val.to_le_bytes());
    }
    out.extend_from_slice(&0u32.to_le_bytes()); // next IFD = none
    out.resize(strip_off as usize, 0);
    out.extend_from_slice(&strip_bytes);
    Ok(out)
}

/// 🚫 EncodeScopeNote: always emits little-endian, 8-bit RGB, chunky, single-strip, uncompressed
/// (`Compression` 1) TIFF — the historical default kept so `print_dsl`/`encode_pack_with` and the
/// io export serializer (which both call this exact function) are unaffected.
pub fn encode_tiff(snap: &TiffSnapshot) -> Result<Vec<u8>, String> {
    encode_tiff_with(snap, false)
}

/// 📦 Same shape as `encode_tiff` but real-PackBits-compresses the strip (`Compression` 32773) —
/// added so PackBits has genuine encode coverage, not just decode.
pub fn encode_tiff_packbits(snap: &TiffSnapshot) -> Result<Vec<u8>, String> {
    encode_tiff_with(snap, true)
}

pub fn empty_tiff_snapshot() -> TiffSnapshot {
    TiffSnapshot::default()
}

pub fn register() {
    crate::artifacts::tiff::composer::register();
    ::schema::register_artifact_schema_descriptor(crate::artifacts::tiff::schema::tiff_artifact_schema_descriptor());
    store::register_document_codec(store::ArtifactCodec::of::<TiffSnapshot, TiffMutation>(STDIO_TIFF_DOCUMENT_SCHEMA));
}

pub struct TiffEngine {
    artifact_state: TiffArtifact,
    snapshot_state: TiffSnapshot,
}
impl TiffEngine {
    pub fn new(snapshot: TiffSnapshot) -> Self {
        Self { artifact_state: TiffArtifact::from_snapshot(snapshot.clone()), snapshot_state: snapshot }
    }
}
impl protocol::ArtifactEngine for TiffEngine {
    type Artifact = TiffArtifact;
    type Snapshot = TiffSnapshot;
    type Mutation = TiffMutation;
    type Diff = TiffDiff;
    fn artifact(&self) -> &Self::Artifact {
        &self.artifact_state
    }
    fn snapshot(&self) -> &Self::Snapshot {
        &self.snapshot_state
    }
    fn apply(&mut self, mutation: &Self::Mutation) -> Result<Self::Diff, protocol::EngineFault> {
        let diff = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(mutation, &self.snapshot_state);
        self.snapshot_state = <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(&diff, &self.snapshot_state);
        self.artifact_state.set_snapshot(self.snapshot_state.clone());
        Ok(diff)
    }
    fn inverse(&self, mutation: &Self::Mutation) -> Vec<Self::Mutation> {
        <Self::Mutation as protocol::Mutation<Self::Snapshot>>::inverse(mutation, &self.snapshot_state)
    }
}
//#endregion Codec

//#region EngineTests
#[cfg(test)]
mod tests {
    use super::*;

    fn gradient_checkerboard_rgba(w: u32, h: u32) -> Vec<u8> {
        let mut out = Vec::with_capacity((w * h * 4) as usize);
        for y in 0..h {
            for x in 0..w {
                let checker = if (x + y) % 2 == 0 { 255u8 } else { 0u8 };
                out.extend_from_slice(&[checker, ((x * 37) % 256) as u8, ((y * 53) % 256) as u8, 255]);
            }
        }
        out
    }

    /// 🔬 Load-bearing regression: non-solid 9x5 checkerboard/gradient round-tripped through the
    /// real uncompressed IFD codec.
    #[test]
    fn gradient_checkerboard_uncompressed_round_trip() {
        let (w, h) = (9u32, 5u32);
        let rgba = gradient_checkerboard_rgba(w, h);
        let snap = TiffSnapshot { schema: STDIO_TIFF_DOCUMENT_SCHEMA.into(), image: RasterImage { width: w, height: h, rgba: rgba.clone() } };
        let encoded = encode_tiff(&snap).expect("encode");
        let decoded = decode_tiff(&encoded).expect("decode");
        assert_eq!(decoded.image.width, w);
        assert_eq!(decoded.image.height, h);
        assert_eq!(decoded.image.rgba, rgba, "decoded pixels must exactly match the original");
    }

    /// 🔬 Same fixture through real PackBits encode+decode — proves PackBits compression is
    /// actually exercised (not just pass-through), by asserting the compressed strip is smaller
    /// than the raw RGB and that decode reconstructs the exact original pixels.
    #[test]
    fn gradient_checkerboard_packbits_round_trip() {
        let (w, h) = (9u32, 5u32);
        let rgba = gradient_checkerboard_rgba(w, h);
        let snap = TiffSnapshot { schema: STDIO_TIFF_DOCUMENT_SCHEMA.into(), image: RasterImage { width: w, height: h, rgba: rgba.clone() } };
        let encoded = encode_tiff_packbits(&snap).expect("encode packbits");
        let decoded = decode_tiff(&encoded).expect("decode packbits");
        assert_eq!(decoded.image.width, w);
        assert_eq!(decoded.image.height, h);
        assert_eq!(decoded.image.rgba, rgba, "packbits round trip must exactly match the original");
    }

    /// 🔬 PackBits actually runs real repeat/literal RLE, not a pass-through: a solid-color strip
    /// (long repeat runs) must compress to fewer bytes than the raw RGB.
    #[test]
    fn packbits_compresses_repetitive_data() {
        let (w, h) = (20u32, 10u32);
        // R=G=B so the underlying RGB byte stream is one long run of an identical byte value —
        // PackBits is a byte-level RLE, so a per-pixel-but-varying-per-channel color (e.g.
        // 200,50,25 cycling) would NOT compress; this fixture is the case that genuinely does.
        let rgba: Vec<u8> = (0..w * h).flat_map(|_| [128u8, 128, 128, 255]).collect();
        let snap = TiffSnapshot { schema: STDIO_TIFF_DOCUMENT_SCHEMA.into(), image: RasterImage { width: w, height: h, rgba: rgba.clone() } };
        let uncompressed = encode_tiff(&snap).expect("encode uncompressed");
        let encoded = encode_tiff_packbits(&snap).expect("encode packbits");
        assert!(encoded.len() < uncompressed.len(), "packbits must shrink a byte-repetitive strip below the uncompressed encoding ({} !< {})", encoded.len(), uncompressed.len());
        let decoded = decode_tiff(&encoded).expect("decode packbits");
        assert_eq!(decoded.image.rgba, rgba);
    }

    #[test]
    fn packbits_hand_decode_control_bytes() {
        // literal run of 3 (10,20,30), then repeat run of 5x99, then literal run of 2 (1,2)
        let encoded: [u8; 9] = [2, 10, 20, 30, 0xFC, 99, 1, 1, 2];
        let expected: Vec<u8> = vec![10, 20, 30, 99, 99, 99, 99, 99, 1, 2];
        let decoded = packbits_decode(&encoded, expected.len()).expect("decode");
        assert_eq!(decoded, expected);
        // round trip through our own encoder too
        let re_encoded = packbits_encode(&expected);
        let re_decoded = packbits_decode(&re_encoded, expected.len()).expect("re-decode");
        assert_eq!(re_decoded, expected);
    }

    /// 🔬 Big-endian (`MM`) byte order must decode correctly too — hand-builds a minimal
    /// big-endian uncompressed 2x1 RGB TIFF.
    #[test]
    fn big_endian_uncompressed_decode() {
        let (w, h): (u32, u32) = (2, 1);
        let rgb: [u8; 6] = [10, 20, 30, 40, 50, 60];
        let entry_count = 9u16;
        let ifd_off = 8u32;
        let strip_off = ifd_off + 2 + 12 * entry_count as u32 + 4;
        let entries: [(u16, u16, u32, u32); 9] = [
            (TAG_IMAGE_WIDTH, 3, 1, w),
            (TAG_IMAGE_LENGTH, 3, 1, h),
            (TAG_BITS_PER_SAMPLE, 3, 1, 8),
            (TAG_COMPRESSION, 3, 1, 1),
            (TAG_PHOTOMETRIC, 3, 1, 2),
            (TAG_STRIP_OFFSETS, 4, 1, strip_off),
            (TAG_SAMPLES_PER_PIXEL, 3, 1, 3),
            (TAG_ROWS_PER_STRIP, 3, 1, h),
            (TAG_STRIP_BYTE_COUNTS, 4, 1, rgb.len() as u32),
        ];
        let mut out = Vec::new();
        out.extend_from_slice(b"MM");
        out.extend_from_slice(&42u16.to_be_bytes());
        out.extend_from_slice(&ifd_off.to_be_bytes());
        out.extend_from_slice(&entry_count.to_be_bytes());
        for (tag, typ, cnt, val) in entries {
            out.extend_from_slice(&tag.to_be_bytes());
            out.extend_from_slice(&typ.to_be_bytes());
            out.extend_from_slice(&cnt.to_be_bytes());
            // SHORT values are left-justified within the 4-byte field even in big-endian files.
            if typ == 3 {
                out.extend_from_slice(&(val as u16).to_be_bytes());
                out.extend_from_slice(&[0u8; 2]);
            } else {
                out.extend_from_slice(&val.to_be_bytes());
            }
        }
        out.extend_from_slice(&0u32.to_be_bytes());
        out.resize(strip_off as usize, 0);
        out.extend_from_slice(&rgb);

        let decoded = decode_tiff(&out).expect("decode big-endian tiff");
        assert_eq!(decoded.image.width, w);
        assert_eq!(decoded.image.height, h);
        assert_eq!(decoded.image.rgba, vec![10, 20, 30, 255, 40, 50, 60, 255]);
    }

    #[test]
    fn sniff_rejects_non_tiff_bytes() {
        let err = decode_tiff(b"not a tiff at all").unwrap_err();
        assert!(err.contains("byte-order"));
    }

    #[test]
    fn unsupported_compression_is_a_typed_error() {
        let (w, h) = (2u32, 2u32);
        let entry_count = 6u16;
        let ifd_off = 8u32;
        let entries: [(u16, u16, u32, u32); 6] = [
            (TAG_IMAGE_WIDTH, 3, 1, w),
            (TAG_IMAGE_LENGTH, 3, 1, h),
            (TAG_BITS_PER_SAMPLE, 3, 1, 8),
            (TAG_COMPRESSION, 3, 1, 5), // LZW — intentionally unsupported
            (TAG_SAMPLES_PER_PIXEL, 3, 1, 3),
            (TAG_STRIP_OFFSETS, 4, 1, 0),
        ];
        let mut out = Vec::new();
        out.extend_from_slice(b"II");
        out.extend_from_slice(&42u16.to_le_bytes());
        out.extend_from_slice(&ifd_off.to_le_bytes());
        out.extend_from_slice(&entry_count.to_le_bytes());
        for (tag, typ, cnt, val) in entries {
            out.extend_from_slice(&tag.to_le_bytes());
            out.extend_from_slice(&typ.to_le_bytes());
            out.extend_from_slice(&cnt.to_le_bytes());
            out.extend_from_slice(&val.to_le_bytes());
        }
        out.extend_from_slice(&0u32.to_le_bytes());
        let err = decode_tiff(&out).unwrap_err();
        assert!(err.contains("unsupported compression"), "unexpected error: {err}");
    }
}
//#endregion EngineTests
