//! 📝️ Text representation codec surface for `stdio.semio.text` (snapshot) — grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::text::schema::snapshot::*;
use crate::standards::v1::subsets::base::schema::triples::{split_top_level, strip_brackets};
use framework_schema::ArtifactSchema;

/// 🧪️ Real hex/bracket-encoded value primitives backing the hand-rolled `ArtifactDsl` below — same
/// style `🖼️image`'s/`🔊️audio`'s own `📸️snapshot`/`🔺️diff`/`🧬️mutations` facets already establish,
/// duplicated locally (not imported across facets) to keep each facet module independently
/// compilable, per that precedent's own doc comment.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if !s.len().is_multiple_of(2) {
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
pub(crate) fn enc_list<T>(items: &[T], enc: impl Fn(&T) -> String) -> String {
    format!("[{}]", items.iter().map(enc).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_list<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Vec<T>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec).collect()
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
pub(crate) fn print_text_snapshot_body(s: &SemioTextSnapshot) -> String {
    format!("schema={}\nruns={}", enc_str(&s.schema), enc_list(&s.runs, enc_run))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_text_snapshot_body(body: &str) -> Result<SemioTextSnapshot, String> {
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

/// 🎁 Real structured text/binary codecs, wrapped in the repo-wide `store::semio_format` envelope.
impl store::ArtifactDsl for SemioTextSnapshot {
    const EXTENSION: &'static str = "semio";
    fn envelope_id() -> &'static str {
        STDIO_SEMIOTEXT_DOCUMENT_SCHEMA
    }

    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_text_snapshot_body(body).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let body = print_text_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📤️ This subset's own `#[value(rename_all = "camelCase")]` structural JSON projection of
/// `s.stdio.semio.text` — the shape `🔤️mutate-semio-text` compares under `ordered-json-v1`, derived
/// from the snapshot type itself rather than hand-written a second time in the adapter, where it
/// could drift away from the type it claims to project. A thin `pack::to_json_string` wrapper
/// (first-party, over `ToValue`/`DslValue`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_semio_text_snapshot_json(snapshot: &SemioTextSnapshot) -> String {
    semio_framework_pack_json::to_json_string(snapshot)
}

/// 📥️ The `pack::from_json_str` inverse of [`encode_semio_text_snapshot_json`] — decodes the committed
/// `../🧬️mutations/<kind>/🧪️tests/<fixture>/📸️snapshot/{⬅️before,➡️after}/🔣️.json`
/// specification vectors into real [`SemioTextSnapshot`] values, so `🔤️mutate-semio-text`'s adapter
/// reads the committed fixture instead of re-declaring it as a Rust literal beside it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_text_snapshot_json(text: &str) -> Result<SemioTextSnapshot, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// 📝️ Parses `s.stdio.semio.text` DSL text into a [`SemioTextSnapshot`] — a named pass-through of this snapshot's own
/// `store::ArtifactDsl` impl above, whose trait and error type are both unnameable outside this
/// crate, so `🔤️mutate-semio-text`'s `identity-round-trip` scenario reaches the real committed
/// artifact (`../../🖼️assets/📃️note/🗣️.dsl.semio`) through this instead.
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
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use super::*;
use crate::standards::v1::subsets::text::schema::snapshot::*;
use crate::audio::io::text::diff::{strip_brackets};
use crate::audio::io::text::diff::{split_top_level};
use framework_schema::ArtifactSchema;




}
pub use snapshot_wire_codec::*;
