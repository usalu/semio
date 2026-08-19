//! 🧬️ SemioImageSnapshot — complete per the master plan's image subset spec: width/height/
//! colorspace/bit-depth + frames{delay_ms, rgba8 pixels} + embedded ICC profile + metadata
//! entries. Informed by png's typed IHDR/ancillary model and gif 89a's frame sequence; replaces
//! the pre-migration `RasterImage`. Ticket
//! 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT (W2b/image).
//! ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION's image wave replaces the old
//! hex-of-`serde_json` envelope passthrough with real hand-rolled text/binary codecs (this is a
//! NEUTRAL semio type, not itself an on-disk file format — real per-format bytes for png/gif/bmp/
//! jpg/tiff are produced by the semio↔format `🚪️io` leaves, W4).

use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::{split_top_level, strip_brackets};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Ids
/// 🏷️ Document schema / DSL envelope id AND `ArtifactSchema` descriptor id — the semio design
/// (unlike gif 87a/89a's deliberately-split convention) uses the SAME literal for both, per the
/// master plan's "Schema descriptor ids `s.stdio.semio` + `s.stdio.semio.<subset>`" note, one per
/// subset. Must stay repo-wide unique — `register_document_codec` duplicate-id detection is a
/// static policy check.
pub const STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA: &str = "s.stdio.semio.image";
//#endregion 🔖️Ids

//#region 🔖️Colorspace
/// 🎨️ Source pixel colorspace — every frame's `rgba8` buffer is always normalized to RGBA8 on
/// decode (per the master plan's snapshot spec), so this field records the SOURCE colorspace for
/// honest round-trip/re-encode decisions, not a second in-memory pixel layout.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum SemioColorspace {
    #[default]
    Rgb,
    Rgba,
    Grayscale,
    GrayscaleAlpha,
    Indexed,
}
//#endregion 🔖️Colorspace

//#region 🔖️Frame
/// 🖼️ One decoded frame: always-RGBA8 pixels (row-major, `width*height*4` bytes) plus its
/// animation delay. A single-frame image (png/jpg/bmp/tiff) has exactly one `SemioImageFrame`
/// with `delay_ms: 0`. Strong entity — per-field diffable (see `🔺️diff`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SemioImageFrame {
    pub delay_ms: u32,
    #[serde(default)]
    pub rgba8: Vec<u8>,
}
//#endregion 🔖️Frame

//#region 🔖️Metadata
/// 🏷️ One textual metadata entry (png tEXt/iTXt, exif-as-text, gif comment-extension-derived, …)
/// — name-keyed by `key`. Weak/value entity: its "diff" is the whole new value, never sub-diffed.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SemioImageMetadataEntry {
    pub key: String,
    #[serde(default)]
    pub value: String,
}
//#endregion 🔖️Metadata

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.image")]
pub struct SemioImageSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub width: u32,
    #[state(artifact)]
    pub height: u32,
    #[state(artifact)]
    #[serde(default)]
    pub colorspace: SemioColorspace,
    #[state(artifact)]
    #[serde(default)]
    pub bit_depth: u8,
    #[state(artifact)]
    #[serde(default)]
    pub frames: Vec<SemioImageFrame>,
    /// 🎨️ Embedded ICC color profile bytes, verbatim — `None` when the source carried none.
    #[state(artifact)]
    #[serde(default)]
    pub icc: Option<Vec<u8>>,
    #[state(artifact)]
    #[serde(default)]
    pub metadata: Vec<SemioImageMetadataEntry>,
}

impl Default for SemioImageSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA.into(), width: 0, height: 0, colorspace: SemioColorspace::default(), bit_depth: 0, frames: Vec::new(), icc: None, metadata: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️TextPrimitives
/// 🧪️ Real hex/bracket-encoded value primitives backing the hand-rolled `ArtifactDsl` below — same
/// style as this subset's own `🔺️diff`/`🧬️mutations` facets (`GifDiff`/`SvgDiff`/`DocxDiff`'s
/// established hand-rolled convention), duplicated here (not imported from `schema::diff`) to keep
/// `snapshot` — the base type `diff`/`mutations` both depend ON — free of a reverse dependency on
/// either sibling facet (same rationale `✳️flow`'s/`✳️mesh`'s own pilots document).
///
/// 🧩️ The `#[derive(dsl::DslArtifact)]` path was tried first per this ticket's brief. It is
/// blocked here: `SemioImageSnapshot.icc: Option<Vec<u8>>` is a BARE `Option<T>` field directly on
/// the snapshot struct — `dsl` has no blanket `Option<T>: DslField` impl (the exact same shape
/// this subset's own `🔺️diff`/`🧬️mutations` facets already document as blocking their derive path,
/// matching gif's/docx's established precedent — `f6-final-summary.md` §4.3/§4.4). Hand-rolled
/// instead, same boundary this ticket's other semio pilots hit for their own bare-`Option`/nested-
/// buffer collection shapes.
async fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
async fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
async fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
async fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
async fn enc_bytes(b: &[u8]) -> String {
    hex_encode(b)
}
async fn dec_bytes(s: &str) -> Result<Vec<u8>, String> {
    hex_decode(s)
}
async fn parse_u8(s: &str) -> Result<u8, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
async fn parse_u32(s: &str) -> Result<u32, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}

async fn enc_list<T>(items: &[T], enc: impl Fn(&T) -> String) -> String {
    format!("[{}]", items.iter().map(|it| enc(it)).collect::<Vec<_>>().join(","))
}
async fn dec_list<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Vec<T>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|entry| dec(entry)).collect()
}
pub(crate) async fn encode_option<T>(opt: &Option<T>, enc: impl Fn(&T) -> String) -> String {
    match opt {
        None => "[0]".to_string(),
        Some(v) => format!("[1,{}]", enc(v)),
    }
}
pub(crate) async fn decode_option<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Option<T>, String> {
    let inner = strip_brackets(s)?;
    match split_top_level(inner, ',').as_slice() {
        ["0"] => Ok(None),
        [tag, value] if *tag == "1" => Ok(Some(dec(value)?)),
        other => Err(format!("option decode: bad shape {other:?}")),
    }
}

pub(crate) async fn enc_colorspace(c: SemioColorspace) -> char {
    match c {
        SemioColorspace::Rgb => 'r',
        SemioColorspace::Rgba => 'a',
        SemioColorspace::Grayscale => 'g',
        SemioColorspace::GrayscaleAlpha => 'y',
        SemioColorspace::Indexed => 'i',
    }
}
pub(crate) async fn dec_colorspace(s: &str) -> Result<SemioColorspace, String> {
    match s {
        "r" => Ok(SemioColorspace::Rgb),
        "a" => Ok(SemioColorspace::Rgba),
        "g" => Ok(SemioColorspace::Grayscale),
        "y" => Ok(SemioColorspace::GrayscaleAlpha),
        "i" => Ok(SemioColorspace::Indexed),
        other => Err(format!("bad colorspace {other:?}")),
    }
}
pub(crate) async fn enc_frame(f: &SemioImageFrame) -> String {
    format!("[{},{}]", f.delay_ms, hex_encode(&f.rgba8))
}
pub(crate) async fn dec_frame(s: &str) -> Result<SemioImageFrame, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [delay, rgba] = parts.as_slice() else { return Err(format!("frame: expected 2 fields, got {}", parts.len())) };
    Ok(SemioImageFrame { delay_ms: parse_u32(delay)?, rgba8: hex_decode(rgba)? })
}
pub(crate) async fn enc_metadata_entry(e: &SemioImageMetadataEntry) -> String {
    format!("[{},{}]", enc_str(&e.key), enc_str(&e.value))
}
pub(crate) async fn dec_metadata_entry(s: &str) -> Result<SemioImageMetadataEntry, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [key, value] = parts.as_slice() else { return Err(format!("metadata entry: expected 2 fields, got {}", parts.len())) };
    Ok(SemioImageMetadataEntry { key: dec_str(key)?, value: dec_str(value)? })
}

/// 📄️ The real structured text body: eight lines — `schema=<hex>`, `width=<N>`, `height=<N>`,
/// `colorspace=<c>`, `bitDepth=<N>`, `icc=<option-hex>`, `frames=[<frame>,...]`,
/// `metadata=[<entry>,...]` — matching the grammar's `document = artifact-mark schema-line
/// width-line height-line colorspace-line bit-depth-line icc-line frames-line metadata-line`.
/// Newlines are pure lexer trivia in the shared dialect, so this is genuinely recognizable by
/// `dsl::Recognizer`, not merely readable.
async fn print_image_snapshot_body(s: &SemioImageSnapshot) -> String {
    format!(
        "schema={}\nwidth={}\nheight={}\ncolorspace={}\nbitDepth={}\nicc={}\nframes={}\nmetadata={}",
        enc_str(&s.schema),
        s.width,
        s.height,
        enc_colorspace(s.colorspace),
        s.bit_depth,
        encode_option(&s.icc, |b| enc_bytes(b)),
        enc_list(&s.frames, enc_frame),
        enc_list(&s.metadata, enc_metadata_entry),
    )
}
async fn parse_image_snapshot_body(body: &str) -> Result<SemioImageSnapshot, String> {
    let mut schema = None;
    let mut width = None;
    let mut height = None;
    let mut colorspace = None;
    let mut bit_depth = None;
    let mut icc = None;
    let mut frames = Vec::new();
    let mut metadata = Vec::new();
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("schema=") {
            schema = Some(dec_str(rest)?);
        } else if let Some(rest) = line.strip_prefix("width=") {
            width = Some(parse_u32(rest)?);
        } else if let Some(rest) = line.strip_prefix("height=") {
            height = Some(parse_u32(rest)?);
        } else if let Some(rest) = line.strip_prefix("colorspace=") {
            colorspace = Some(dec_colorspace(rest)?);
        } else if let Some(rest) = line.strip_prefix("bitDepth=") {
            bit_depth = Some(parse_u8(rest)?);
        } else if let Some(rest) = line.strip_prefix("icc=") {
            icc = Some(decode_option(rest, dec_bytes)?);
        } else if let Some(rest) = line.strip_prefix("frames=") {
            frames = dec_list(rest, dec_frame)?;
        } else if let Some(rest) = line.strip_prefix("metadata=") {
            metadata = dec_list(rest, dec_metadata_entry)?;
        } else {
            return Err(format!("semio image snapshot: unknown line {line:?}"));
        }
    }
    Ok(SemioImageSnapshot {
        schema: schema.ok_or_else(|| "semio image snapshot: missing schema line".to_string())?,
        width: width.ok_or_else(|| "semio image snapshot: missing width line".to_string())?,
        height: height.ok_or_else(|| "semio image snapshot: missing height line".to_string())?,
        colorspace: colorspace.unwrap_or_default(),
        bit_depth: bit_depth.unwrap_or(0),
        icc: icc.unwrap_or(None),
        frames,
        metadata,
    })
}
//#endregion 🔖️TextPrimitives

//#region 🔖️BinaryPrimitives
/// 🧪️ Real LEB128-varint-length-prefixed binary primitives (`store::pack_rt::write_varint_u64` /
/// `store::ByteReader`, same helpers `✳️flow`'s/`✳️mesh`'s own upgraded `ArtifactPack` uses)
/// backing the real `ArtifactPack` below — replaces the old `serde_json::to_vec`-in-envelope
/// shortcut.
async fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}
async fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}
async fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}
async fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}

async fn colorspace_tag(c: SemioColorspace) -> u8 {
    match c {
        SemioColorspace::Rgb => 0,
        SemioColorspace::Rgba => 1,
        SemioColorspace::Grayscale => 2,
        SemioColorspace::GrayscaleAlpha => 3,
        SemioColorspace::Indexed => 4,
    }
}
async fn colorspace_from_tag(tag: u8) -> Result<SemioColorspace, String> {
    match tag {
        0 => Ok(SemioColorspace::Rgb),
        1 => Ok(SemioColorspace::Rgba),
        2 => Ok(SemioColorspace::Grayscale),
        3 => Ok(SemioColorspace::GrayscaleAlpha),
        4 => Ok(SemioColorspace::Indexed),
        other => Err(format!("unsupported colorspace tag {other}")),
    }
}

/// 🎁 `format u8` + varint-length-prefixed `schema` UTF-8 + real fixed-width `width`/`height`
/// (`u32` LE) + `colorspace` (`u8` tag) + `bit_depth` (`u8`) — all genuinely, individually
/// protocol-walkable, matching the real `📡️component.protocol.semio` header/segment fields
/// exactly — then `icc` (presence `u8` + optional varint-length-prefixed bytes), `frames`
/// (varint count + per-frame `delay_ms`/`rgba8`), and `metadata` (varint count + per-entry
/// `key`/`value`) as the honest opaque `payload` tail (`protocol-array-of-records` gap — `frames`/
/// `metadata` are homogeneous variable-length repeated records).
async fn encode_image_snapshot_binary(s: &SemioImageSnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut out = Vec::new();
    out.push(PACK_BINARY_FORMAT);
    write_str_lp(&mut out, &s.schema);
    out.extend_from_slice(&s.width.to_le_bytes());
    out.extend_from_slice(&s.height.to_le_bytes());
    out.push(colorspace_tag(s.colorspace));
    out.push(s.bit_depth);
    match &s.icc {
        Some(bytes) => {
            out.push(1);
            write_bytes_lp(&mut out, bytes);
        }
        None => out.push(0),
    }
    store::pack_rt::write_varint_u64(&mut out, s.frames.len() as u64);
    for f in &s.frames {
        out.extend_from_slice(&f.delay_ms.to_le_bytes());
        write_bytes_lp(&mut out, &f.rgba8);
    }
    store::pack_rt::write_varint_u64(&mut out, s.metadata.len() as u64);
    for entry in &s.metadata {
        write_str_lp(&mut out, &entry.key);
        write_str_lp(&mut out, &entry.value);
    }
    out
}
async fn decode_image_snapshot_binary(bytes: &[u8]) -> Result<SemioImageSnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    let schema = read_str_lp(&mut reader)?;
    let width = reader.read_u32_le().map_err(|e| e.to_string())?;
    let height = reader.read_u32_le().map_err(|e| e.to_string())?;
    let colorspace = colorspace_from_tag(reader.read_u8().map_err(|e| e.to_string())?)?;
    let bit_depth = reader.read_u8().map_err(|e| e.to_string())?;
    let icc = match reader.read_u8().map_err(|e| e.to_string())? {
        0 => None,
        1 => Some(read_bytes_lp(&mut reader)?),
        other => return Err(format!("unsupported icc presence tag {other}")),
    };
    let frame_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut frames = Vec::with_capacity(frame_count as usize);
    for _ in 0..frame_count {
        let delay_ms = reader.read_u32_le().map_err(|e| e.to_string())?;
        let rgba8 = read_bytes_lp(&mut reader)?;
        frames.push(SemioImageFrame { delay_ms, rgba8 });
    }
    let metadata_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut metadata = Vec::with_capacity(metadata_count as usize);
    for _ in 0..metadata_count {
        let key = read_str_lp(&mut reader)?;
        let value = read_str_lp(&mut reader)?;
        metadata.push(SemioImageMetadataEntry { key, value });
    }
    Ok(SemioImageSnapshot { schema, width, height, colorspace, bit_depth, icc, frames, metadata })
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactCodecs
/// 🎁 Real structured text/binary codecs — replaces the old hex-dump-of-`serde_json` shortcut.
/// Wrapped in the repo-wide `store::semio_format` envelope, unchanged.
impl store::ArtifactDsl for SemioImageSnapshot {
    const EXTENSION: &'static str = "semio";
    async fn envelope_id() -> &'static str {
        STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA
    }

    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_image_snapshot_body(body).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }

    async fn print_dsl(&self) -> String {
        let body = print_image_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for SemioImageSnapshot {
    async fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_image_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    async fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let _ = options;
        decode_image_snapshot_binary(&inner).map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️Demo
/// 🌱 The demo `s.stdio.semio.image` document — one frame (16-byte RGBA8 pixel sweep across
/// red/green/blue/white), a non-default colorspace/bit-depth, a set ICC profile, and one metadata
/// entry — exercising every leaf/collection shape at least once. Single source of truth for
/// `📚️examples/…/🖼️assets/🗣️example.dsl.semio`/`🎒️example.pack.semio` and for the conformance-law
/// tests in `🎹️composer/🦀️component.rs`.
#[cfg(test)]
pub(crate) async fn demo_image_snapshot() -> SemioImageSnapshot {
    SemioImageSnapshot {
        schema: STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA.into(),
        width: 2,
        height: 2,
        colorspace: SemioColorspace::Rgba,
        bit_depth: 8,
        frames: vec![SemioImageFrame { delay_ms: 100, rgba8: vec![255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 255, 255, 255, 255] }],
        icc: Some(vec![1, 2, 3, 4]),
        metadata: vec![SemioImageMetadataEntry { key: "Title".into(), value: "Demo".into() }],
    }
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// 🌱 Reuses `demo_image_snapshot()` (single source of truth, also feeds the shipped fixtures
    /// and `🎹️composer/🦀️component.rs`'s conformance-law tests) rather than an independent copy.
    async fn populated() -> SemioImageSnapshot {
        demo_image_snapshot()
    }

    #[semio_framework_async_macros::async_test]
    async fn json_pack_round_trips() {
        let snap = SemioImageSnapshot::default();
        let bytes = <SemioImageSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <SemioImageSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);
    }

    #[semio_framework_async_macros::async_test]
    async fn dsl_text_round_trips() {
        let snap = SemioImageSnapshot::default();
        let text = <SemioImageSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let back = <SemioImageSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(snap, back);
    }

    /// 🧪️ codec_retention_law: decode(encode(snapshot)) is byte-for-byte structurally identical
    /// on a fully-populated snapshot (frames/icc/metadata all non-empty), not just the default.
    #[semio_framework_async_macros::async_test]
    async fn codec_retention_law() {
        let snap = populated();
        let bytes = <SemioImageSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <SemioImageSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);
        let text = <SemioImageSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let back_text = <SemioImageSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(snap, back_text);
    }
}
//#endregion 🔖️Tests
