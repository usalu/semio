//! 🧬️ SemioTextSnapshot — the neutral interchange shape for plain and inline-marked text: a
//! sequence of language-tagged runs, each carrying content plus inline marks (bold/italic/code/
//! link). LEAF subset (no child slots, no link slots) per the master plan's stdio target
//! vocabulary — `document`/`drawing`/`presentation`/etc. compose this for their textual content
//! (a later wave). Absorbs the duplicated `LocalizedText` types that currently exist twice inside
//! the norm plugin (ticket UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM, W2a/text).
//!
//! Modeled on `✳️image`'s hand-rolled `ArtifactDsl`/`ArtifactPack` convention (real hex/bracket
//! text codec + real varint-length-prefixed binary codec, both wrapped in the shared
//! `store::semio_format` envelope) — `✳️document`'s `DocRun`/`DocStyle` is the structural cousin
//! (run + inline marks), but `text` owns runs standalone rather than nested inside block
//! structure, per this ticket's brief.

use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::{split_top_level, strip_brackets};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Ids
/// 🏷️ Document schema / DSL envelope id AND `ArtifactSchema` descriptor id — same literal for
/// both, per the master plan's "Schema descriptor ids `s.stdio.semio` + `s.stdio.semio.<subset>`"
/// convention, one per subset.
pub const STDIO_SEMIOTEXT_DOCUMENT_SCHEMA: &str = "s.stdio.semio.text";
//#endregion 🔖️Ids

//#region 🔖️MarkKind
/// 🖊️ The closed inline-mark vocabulary this leaf carries — bold/italic/code (flag-only) and link
/// (carries an `href`). `href` on a non-`Link` mark is always the empty string.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum SemioTextMarkKind {
    #[default]
    Bold,
    Italic,
    Code,
    Link,
}
//#endregion 🔖️MarkKind

//#region 🔖️Mark
/// 🔖️ One inline mark applied to a run. Strong entity, index-addressed within its owning run's
/// `marks` (an intrinsically ordered, anonymous collection — see `➕add-mark`/`➖remove-mark`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SemioTextMark {
    pub kind: SemioTextMarkKind,
    /// 🔗️ Populated only when `kind == Link`; empty string otherwise.
    #[serde(default)]
    pub href: String,
}
//#endregion 🔖️Mark

//#region 🔖️Run
/// 🏃️ One run of text: a BCP-47 `language` tag (`""` = unspecified, inherits from context), the
/// authored `content`, and its ordered `marks`. Runs themselves are index-addressed (no stable
/// id — an intrinsically ordered, anonymous collection, `📓️taxonomy.md` addressing rule #3), the
/// same shape `insert-run`/`remove-run`/`reorder-runs` operate on.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SemioTextRun {
    #[serde(default)]
    pub language: String,
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub marks: Vec<SemioTextMark>,
}
//#endregion 🔖️Run

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.text")]
pub struct SemioTextSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[serde(default)]
    pub runs: Vec<SemioTextRun>,
}

impl Default for SemioTextSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_SEMIOTEXT_DOCUMENT_SCHEMA.into(), runs: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️TextPrimitives
/// 🧪️ Real hex/bracket-encoded value primitives backing the hand-rolled `ArtifactDsl` below — same
/// style `✳️image`'s/`✳️audio`'s own `📸️snapshot`/`🔺️diff`/`🧬️mutations` facets already establish,
/// duplicated locally (not imported across facets) to keep each facet module independently
/// compilable, per that precedent's own doc comment.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_list<T>(items: &[T], enc: impl Fn(&T) -> String) -> String {
    format!("[{}]", items.iter().map(|it| enc(it)).collect::<Vec<_>>().join(","))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_list<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Vec<T>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|entry| dec(entry)).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_mark_kind(k: SemioTextMarkKind) -> char {
    match k {
        SemioTextMarkKind::Bold => 'b',
        SemioTextMarkKind::Italic => 'i',
        SemioTextMarkKind::Code => 'c',
        SemioTextMarkKind::Link => 'l',
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_mark_kind(s: &str) -> Result<SemioTextMarkKind, String> {
    match s {
        "b" => Ok(SemioTextMarkKind::Bold),
        "i" => Ok(SemioTextMarkKind::Italic),
        "c" => Ok(SemioTextMarkKind::Code),
        "l" => Ok(SemioTextMarkKind::Link),
        other => Err(format!("bad mark kind {other:?}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_mark(m: &SemioTextMark) -> String {
    format!("[{},{}]", enc_mark_kind(m.kind), enc_str(&m.href))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_mark(s: &str) -> Result<SemioTextMark, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [kind, href] = parts.as_slice() else { return Err(format!("mark: expected 2 fields, got {}", parts.len())) };
    Ok(SemioTextMark { kind: dec_mark_kind(kind)?, href: dec_str(href)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_run(r: &SemioTextRun) -> String {
    format!("[{},{},{}]", enc_str(&r.language), enc_str(&r.content), enc_list(&r.marks, enc_mark))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_run(s: &str) -> Result<SemioTextRun, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [language, content, marks] = parts.as_slice() else { return Err(format!("run: expected 3 fields, got {}", parts.len())) };
    Ok(SemioTextRun { language: dec_str(language)?, content: dec_str(content)?, marks: dec_list(marks, dec_mark)? })
}

/// 📄️ The real structured text body: two lines — `schema=<hex>`, `runs=[<run>,...]` — matching the
/// grammar's `document = artifact-mark schema-line runs-line`. Newlines are pure lexer trivia in
/// the shared dialect, so this is genuinely recognizable by `dsl::Recognizer`, not merely readable.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_text_snapshot_body(s: &SemioTextSnapshot) -> String {
    format!("schema={}\nruns={}", enc_str(&s.schema), enc_list(&s.runs, enc_run))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_text_snapshot_body(body: &str) -> Result<SemioTextSnapshot, String> {
    let mut schema = None;
    let mut runs = Vec::new();
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("schema=") {
            schema = Some(dec_str(rest)?);
        } else if let Some(rest) = line.strip_prefix("runs=") {
            runs = dec_list(rest, dec_run)?;
        } else {
            return Err(format!("semio text snapshot: unknown line {line:?}"));
        }
    }
    Ok(SemioTextSnapshot { schema: schema.ok_or_else(|| "semio text snapshot: missing schema line".to_string())?, runs })
}
//#endregion 🔖️TextPrimitives

//#region 🔖️BinaryPrimitives
/// 🧪️ Real LEB128-varint-length-prefixed binary primitives (`store::pack_rt::write_varint_u64` /
/// `store::ByteReader`, same helpers every other real semio codec in this standard uses).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn mark_kind_tag(k: SemioTextMarkKind) -> u8 {
    match k {
        SemioTextMarkKind::Bold => 0,
        SemioTextMarkKind::Italic => 1,
        SemioTextMarkKind::Code => 2,
        SemioTextMarkKind::Link => 3,
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn mark_kind_from_tag(tag: u8) -> Result<SemioTextMarkKind, String> {
    match tag {
        0 => Ok(SemioTextMarkKind::Bold),
        1 => Ok(SemioTextMarkKind::Italic),
        2 => Ok(SemioTextMarkKind::Code),
        3 => Ok(SemioTextMarkKind::Link),
        other => Err(format!("unsupported mark kind tag {other}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_mark(out: &mut Vec<u8>, m: &SemioTextMark) {
    out.push(mark_kind_tag(m.kind));
    write_str_lp(out, &m.href);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_mark(reader: &mut store::ByteReader<'_>) -> Result<SemioTextMark, String> {
    let kind = mark_kind_from_tag(reader.read_u8().map_err(|e| e.to_string())?)?;
    let href = read_str_lp(reader)?;
    Ok(SemioTextMark { kind, href })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_run(out: &mut Vec<u8>, r: &SemioTextRun) {
    write_str_lp(out, &r.language);
    write_str_lp(out, &r.content);
    store::pack_rt::write_varint_u64(out, r.marks.len() as u64);
    for m in &r.marks {
        write_mark(out, m);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_run(reader: &mut store::ByteReader<'_>) -> Result<SemioTextRun, String> {
    let language = read_str_lp(reader)?;
    let content = read_str_lp(reader)?;
    let mark_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut marks = Vec::with_capacity(mark_count as usize);
    for _ in 0..mark_count {
        marks.push(read_mark(reader)?);
    }
    Ok(SemioTextRun { language, content, marks })
}

/// 🎁 `format u8` + varint-length-prefixed `schema` UTF-8 — both genuinely, individually
/// protocol-walkable, matching `📡️component.protocol.semio`'s header/segment fields exactly —
/// then `runs` (varint count + per-run language/content/marks) as the honest opaque `payload`
/// tail (`protocol-array-of-records` gap — homogeneous, variable-length repeated records).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_text_snapshot_binary(s: &SemioTextSnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut out = Vec::new();
    out.push(PACK_BINARY_FORMAT);
    write_str_lp(&mut out, &s.schema);
    store::pack_rt::write_varint_u64(&mut out, s.runs.len() as u64);
    for r in &s.runs {
        write_run(&mut out, r);
    }
    out
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_text_snapshot_binary(bytes: &[u8]) -> Result<SemioTextSnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    let schema = read_str_lp(&mut reader)?;
    let run_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut runs = Vec::with_capacity(run_count as usize);
    for _ in 0..run_count {
        runs.push(read_run(&mut reader)?);
    }
    Ok(SemioTextSnapshot { schema, runs })
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactCodecs
/// 🎁 Real structured text/binary codecs, wrapped in the repo-wide `store::semio_format` envelope.
impl store::ArtifactDsl for SemioTextSnapshot {
    const EXTENSION: &'static str = "semio";
    fn envelope_id() -> &'static str {
        STDIO_SEMIOTEXT_DOCUMENT_SCHEMA
    }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_text_snapshot_body(body).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let body = print_text_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for SemioTextSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_text_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let _ = options;
        decode_text_snapshot_binary(&inner).map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🌉️ExternalCodecBridge
/// 📤️ This subset's own `#[serde(rename_all = "camelCase")]` structural JSON projection of
/// `s.stdio.semio.text` — the shape `mutate-semio-text` compares under `ordered-json-v1`, derived
/// from the snapshot type itself rather than hand-written a second time in the adapter, where it
/// could drift away from the type it claims to project. A thin `serde_json` wrapper (already a
/// direct dependency of this crate, used behind this interface per CLAUDE.md's "external libraries
/// behind an interface" rule, never a new one).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_semio_text_snapshot_json(snapshot: &SemioTextSnapshot) -> String {
    serde_json::to_string(snapshot).expect("SemioTextSnapshot serialization is infallible")
}

/// 📥️ The `serde_json` inverse of [`encode_semio_text_snapshot_json`] — decodes the committed
/// `../🧬️mutations/<kind>/🧪️tests/<fixture>/📸️snapshot/{⬅️before,➡️after}/🔣️component.json`
/// specification vectors into real [`SemioTextSnapshot`] values, so `mutate-semio-text`'s adapter
/// reads the committed fixture instead of re-declaring it as a Rust literal beside it. Reaching
/// `serde_json` from that adapter is impossible — the generated test host links only this crate —
/// which is why the bridge belongs here rather than there.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_text_snapshot_json(text: &str) -> Result<SemioTextSnapshot, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}
//#endregion 🌉️ExternalCodecBridge

//#region 🔖️Wire
/// 📝️ Parses `s.stdio.semio.text` DSL text into a [`SemioTextSnapshot`] — a named pass-through of this snapshot's own
/// `store::ArtifactDsl` impl above, whose trait and error type are both unnameable outside this
/// crate, so `mutate-semio-text`'s `identity-round-trip` scenario reaches the real committed
/// artifact (`../../📚️examples/📃️note/🖼️assets/🗣️example.dsl.semio`) through this instead.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_semio_text_dsl(text: &str) -> Result<SemioTextSnapshot, String> {
    <SemioTextSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| error.to_string())
}

/// 📝️ Renders a [`SemioTextSnapshot`] back as `s.stdio.semio.text` DSL text — the inverse of
/// [`parse_semio_text_dsl`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn print_semio_text_dsl(snapshot: &SemioTextSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}

/// 📦️ Encodes a [`SemioTextSnapshot`] as a semio pack envelope — the binary twin of the DSL text, produced by a
/// SEPARATE codec, which is what makes the two committed encodings of one document able to
/// contradict each other.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_semio_text_pack(snapshot: &SemioTextSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(snapshot)
}

/// 📦️ Decodes a semio pack envelope into a [`SemioTextSnapshot`] — the inverse of
/// [`encode_semio_text_pack`], reading `../../📚️examples/📃️note/🖼️assets/🎒️example.pack.semio`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_text_pack(bytes: &[u8]) -> Result<SemioTextSnapshot, String> {
    <SemioTextSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| error.to_string())
}
//#endregion 🔖️Wire

//#region 🔖️Demo
/// 🌱 The demo `s.stdio.semio.text` document — three runs (plain, bold, and a link mark) across
/// two languages, exercising every leaf/collection shape at least once. Single source of truth for
/// `📚️examples/…/🖼️assets/🗣️example.dsl.semio`/`🎒️example.pack.semio` and for the conformance-law
/// tests in `🚪️io/🦀️component.rs`.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_text_snapshot() -> SemioTextSnapshot {
    SemioTextSnapshot {
        schema: STDIO_SEMIOTEXT_DOCUMENT_SCHEMA.into(),
        runs: vec![
            SemioTextRun { language: "en".into(), content: "Hello, ".into(), marks: vec![] },
            SemioTextRun { language: "en".into(), content: "world".into(), marks: vec![SemioTextMark { kind: SemioTextMarkKind::Bold, href: String::new() }] },
            SemioTextRun { language: "de".into(), content: "semio.tech".into(), marks: vec![SemioTextMark { kind: SemioTextMarkKind::Link, href: "https://semio.tech".into() }] },
        ],
    }
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn populated() -> SemioTextSnapshot {
        demo_text_snapshot()
    }

    #[semio_framework_async_macros::async_test]
    async fn json_pack_round_trips() {
        let snap = SemioTextSnapshot::default();
        let bytes = <SemioTextSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <SemioTextSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);
    }

    #[semio_framework_async_macros::async_test]
    async fn dsl_text_round_trips() {
        let snap = SemioTextSnapshot::default();
        let text = <SemioTextSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let back = <SemioTextSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(snap, back);
    }

    /// 🧪️ codec_retention_law: decode(encode(snapshot)) is byte-for-byte structurally identical
    /// on a fully-populated snapshot (runs/marks non-empty), not just the default.
    #[semio_framework_async_macros::async_test]
    async fn codec_retention_law() {
        let snap = populated();
        let bytes = <SemioTextSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <SemioTextSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);
        let text = <SemioTextSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let back_text = <SemioTextSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(snap, back_text);
    }
}
//#endregion 🔖️Tests
