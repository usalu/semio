//! 📝️ Text representation grammar surface for `s.stdio.semio.cad` (snapshot).

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::cad::schema::snapshot::*;
use crate::standards::v1::subsets::base::schema::geometry::native;
use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use crate::standards::v1::subsets::base::io::text::snapshot::{split_top_level, strip_brackets};
use framework_schema::ArtifactSchema;

/// 🧪️ ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION cad wave (following the
/// flow/brep pilots' proven template, `ws-codec-workflow-report.md`/`ws-codec-brep-report.md`):
/// real hex/bracket-encoded value primitives backing the hand-rolled `ArtifactDsl` below — same
/// style as this subset's own `🔺️diff`/`🧬️mutations` facets, duplicated here (not imported from
/// `schema::diff`) to keep `snapshot` — the base type `diff`/`mutations` both depend ON — free of a
/// reverse dependency on either sibling facet.
///
/// 🧩️ The `#[derive(dsl::DslArtifact)]` path was reconsidered now that `SemioPoint2` derives
/// `dsl::DslRecord`. Still blocked: `CadEntity` is a data-carrying TAGGED ENUM (9 variants, each
/// with a DIFFERENT field set) — the derive machinery's `DslVariants`/`DslEnum` support targets
/// one-spec-per-variant BINARY layouts, not a single alternated TEXT grammar production set (the
/// `semio-tagged-enum-heterogeneous-variants-no-dslenum-text-path` gap brep's wave first hit).
/// Hand-rolled instead, matching the established hex/bracket convention this subset's own `🔺️diff`
/// facet already uses for exactly this enum.
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
pub(crate) fn parse_f64(s: &str) -> Result<f64, String> {
    native::parse(s)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_i32(s: &str) -> Result<i32, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_bool(b: bool) -> &'static str {
    if b {
        "1"
    } else {
        "0"
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_bool(s: &str) -> Result<bool, String> {
    match s {
        "1" => Ok(true),
        "0" => Ok(false),
        other => Err(format!("bad bool {other:?}")),
    }
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
pub(crate) fn enc_point2(p: &SemioPoint2) -> String {
    format!("[{},{}]", native::NativeF64(p.x), native::NativeF64(p.y))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_point2(s: &str) -> Result<SemioPoint2, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y] = parts.as_slice() else { return Err(format!("point2: expected 2 fields, got {}", parts.len())) };
    Ok(SemioPoint2 { x: parse_f64(x)?, y: parse_f64(y)? })
}

/// 📐️ `L`ine/`A`rc/`C`ircle/`E`llipse/`P`olyline/`T`ext/`I`nsert/`S`olid/`D`imension — single-letter
/// tag prefix, same convention this subset's own `🔺️diff/🦀️.rs`'s `enc_entity` uses
/// (duplicated here, field-for-field).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_entity(e: &CadEntity) -> String {
    match e {
        CadEntity::Line { a, b } => format!("L[{},{}]", enc_point2(a), enc_point2(b)),
        CadEntity::Arc { center, radius, start_angle, end_angle } => format!("A[{},{},{},{}]", enc_point2(center), native::NativeF64(*radius), native::NativeF64(*start_angle), native::NativeF64(*end_angle)),
        CadEntity::Circle { center, radius } => format!("C[{},{}]", enc_point2(center), native::NativeF64(*radius)),
        CadEntity::Ellipse { center, major_axis_end, ratio, start_param, end_param } => {
            format!("E[{},{},{},{},{}]", enc_point2(center), enc_point2(major_axis_end), native::NativeF64(*ratio), native::NativeF64(*start_param), native::NativeF64(*end_param))
        }
        CadEntity::Polyline { vertices, closed } => format!("P[{},{}]", enc_list(vertices, enc_point2), enc_bool(*closed)),
        CadEntity::Text { position, height, rotation, content } => format!("T[{},{},{},{}]", enc_point2(position), native::NativeF64(*height), native::NativeF64(*rotation), enc_str(content)),
        CadEntity::Insert { block_name, insertion_point, scale, rotation } => format!("I[{},{},{},{}]", enc_str(block_name), enc_point2(insertion_point), enc_point2(scale), native::NativeF64(*rotation)),
        CadEntity::Solid { p1, p2, p3, p4 } => format!("S[{},{},{},{}]", enc_point2(p1), enc_point2(p2), enc_point2(p3), enc_point2(p4)),
        CadEntity::Dimension { def_point, text_position, measurement, text } => format!("D[{},{},{},{}]", enc_point2(def_point), enc_point2(text_position), native::NativeF64(*measurement), enc_str(text)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_entity(s: &str) -> Result<CadEntity, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    let parts = split_top_level(inner, ',');
    match tag {
        "L" => {
            let [a, b] = parts.as_slice() else { return Err(format!("line: expected 2 fields, got {}", parts.len())) };
            Ok(CadEntity::Line { a: dec_point2(a)?, b: dec_point2(b)? })
        }
        "A" => {
            let [center, radius, start_angle, end_angle] = parts.as_slice() else { return Err(format!("arc: expected 4 fields, got {}", parts.len())) };
            Ok(CadEntity::Arc { center: dec_point2(center)?, radius: parse_f64(radius)?, start_angle: parse_f64(start_angle)?, end_angle: parse_f64(end_angle)? })
        }
        "C" => {
            let [center, radius] = parts.as_slice() else { return Err(format!("circle: expected 2 fields, got {}", parts.len())) };
            Ok(CadEntity::Circle { center: dec_point2(center)?, radius: parse_f64(radius)? })
        }
        "E" => {
            let [center, major_axis_end, ratio, start_param, end_param] = parts.as_slice() else { return Err(format!("ellipse: expected 5 fields, got {}", parts.len())) };
            Ok(CadEntity::Ellipse { center: dec_point2(center)?, major_axis_end: dec_point2(major_axis_end)?, ratio: parse_f64(ratio)?, start_param: parse_f64(start_param)?, end_param: parse_f64(end_param)? })
        }
        "P" => {
            let [vertices, closed] = parts.as_slice() else { return Err(format!("polyline: expected 2 fields, got {}", parts.len())) };
            Ok(CadEntity::Polyline { vertices: dec_list(vertices, dec_point2)?, closed: parse_bool(closed)? })
        }
        "T" => {
            let [position, height, rotation, content] = parts.as_slice() else { return Err(format!("text: expected 4 fields, got {}", parts.len())) };
            Ok(CadEntity::Text { position: dec_point2(position)?, height: parse_f64(height)?, rotation: parse_f64(rotation)?, content: dec_str(content)? })
        }
        "I" => {
            let [block_name, insertion_point, scale, rotation] = parts.as_slice() else { return Err(format!("insert: expected 4 fields, got {}", parts.len())) };
            Ok(CadEntity::Insert { block_name: dec_str(block_name)?, insertion_point: dec_point2(insertion_point)?, scale: dec_point2(scale)?, rotation: parse_f64(rotation)? })
        }
        "S" => {
            let [p1, p2, p3, p4] = parts.as_slice() else { return Err(format!("solid: expected 4 fields, got {}", parts.len())) };
            Ok(CadEntity::Solid { p1: dec_point2(p1)?, p2: dec_point2(p2)?, p3: dec_point2(p3)?, p4: dec_point2(p4)? })
        }
        "D" => {
            let [def_point, text_position, measurement, text] = parts.as_slice() else { return Err(format!("dimension: expected 4 fields, got {}", parts.len())) };
            Ok(CadEntity::Dimension { def_point: dec_point2(def_point)?, text_position: dec_point2(text_position)?, measurement: parse_f64(measurement)?, text: dec_str(text)? })
        }
        other => Err(format!("entity: unknown tag {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_layer(l: &CadLayer) -> String {
    format!("[{},{},{},{}]", enc_str(&l.name), l.color_index, enc_str(&l.line_type), enc_bool(l.visible))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_layer(s: &str) -> Result<CadLayer, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, color_index, line_type, visible] = parts.as_slice() else { return Err(format!("layer: expected 4 fields, got {}", parts.len())) };
    Ok(CadLayer { name: dec_str(name)?, color_index: parse_i32(color_index)?, line_type: dec_str(line_type)?, visible: parse_bool(visible)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_entity_record(r: &CadEntityRecord) -> String {
    format!("[{},{},{}]", enc_str(&r.handle), enc_str(&r.layer), enc_entity(&r.entity))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_entity_record(s: &str) -> Result<CadEntityRecord, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [handle, layer, entity] = parts.as_slice() else { return Err(format!("entity record: expected 3 fields, got {}", parts.len())) };
    Ok(CadEntityRecord { handle: dec_str(handle)?, layer: dec_str(layer)?, entity: dec_entity(entity)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_block(b: &CadBlock) -> String {
    format!("[{},{},{}]", enc_str(&b.name), enc_point2(&b.base_point), enc_list(&b.entities, enc_entity_record))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_block(s: &str) -> Result<CadBlock, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, base_point, entities] = parts.as_slice() else { return Err(format!("block: expected 3 fields, got {}", parts.len())) };
    Ok(CadBlock { name: dec_str(name)?, base_point: dec_point2(base_point)?, entities: dec_list(entities, dec_entity_record)? })
}

/// 📄️ The real structured text body: four lines — `schema=<hex>`, `layers=[...]`, `blocks=[...]`,
/// `entities=[...]` — matching the grammar's `document = artifact-mark schema-line layers-line
/// blocks-line entities-line`. Newlines are pure lexer trivia in the shared dialect, so this is
/// genuinely recognizable by `dsl::Recognizer`, not merely readable.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_cad_snapshot_body(s: &SemioCadSnapshot) -> String {
    format!(
        "schema={}\nlayers=[{}]\nblocks=[{}]\nentities=[{}]",
        enc_str(&s.schema),
        s.layers.iter().map(enc_layer).collect::<Vec<_>>().join(","),
        s.blocks.iter().map(enc_block).collect::<Vec<_>>().join(","),
        s.entities.iter().map(enc_entity_record).collect::<Vec<_>>().join(","),
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_cad_snapshot_body(body: &str) -> Result<SemioCadSnapshot, String> {
    let mut schema = None;
    let mut layers = Vec::new();
    let mut blocks = Vec::new();
    let mut entities = Vec::new();
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("schema=") {
            schema = Some(dec_str(rest)?);
        } else if let Some(rest) = line.strip_prefix("layers=") {
            layers = dec_list(rest, dec_layer)?;
        } else if let Some(rest) = line.strip_prefix("blocks=") {
            blocks = dec_list(rest, dec_block)?;
        } else if let Some(rest) = line.strip_prefix("entities=") {
            entities = dec_list(rest, dec_entity_record)?;
        } else {
            return Err(format!("cad snapshot: unknown line {line:?}"));
        }
    }
    let schema = schema.ok_or_else(|| "cad snapshot: missing schema line".to_string())?;
    Ok(SemioCadSnapshot { schema, layers, blocks, entities })
}

/// 🎁 Real structured text/binary codecs (cad wave — off the old hex-dump-of-`serde_json`
/// shortcut, following the flow/brep pilots' proven template). Wrapped in the repo-wide
/// `store::semio_format` envelope, unchanged.
impl store::ArtifactDsl for SemioCadSnapshot {
    const EXTENSION: &'static str = "semio";
    fn envelope_id() -> &'static str {
        STDIO_SEMIOCAD_DOCUMENT_SCHEMA
    }

    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_cad_snapshot_body(body).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let body = print_cad_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📥️ Parses this subset's own committed `.dsl.semio` text into a real [`SemioCadSnapshot`] — a thin
/// wrapper over `store::ArtifactDsl::parse_dsl` so external Rust callers that cannot name this
/// crate's private `store` extern-crate item (the `📐️mutate-semio-cad` test adapter, which reads the
/// REAL committed example artifact rather than a hand-transcribed Rust literal of it) can still
/// drive the same codec production does. Same rationale as `🧰️kit`'s `decode_kit_snapshot_json`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_semio_cad_dsl(text: &str) -> Result<SemioCadSnapshot, String> {
    <SemioCadSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| error.to_string())
}

/// 📤️ The `store::ArtifactDsl::print_dsl` inverse of [`parse_semio_cad_dsl`] — same rationale.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn print_semio_cad_dsl(snapshot: &SemioCadSnapshot) -> String {
    <SemioCadSnapshot as store::ArtifactDsl>::print_dsl(snapshot)
}

/// 📤️ This subset's own `#[value(rename_all = "camelCase")]` structural JSON projection of
/// `s.stdio.semio.cad` — the shape the `📐️mutate-semio-cad` case compares under `ordered-json-v1`. A
/// thin `pack::to_json_string` wrapper (first-party, over `ToValue`/`DslValue` — see
/// `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS/`),
/// so a projection is derived from the snapshot type itself rather than hand-written a second time
/// in the adapter, where it could drift.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_semio_cad_snapshot_json(snapshot: &SemioCadSnapshot) -> String {
    semio_framework_pack_json::to_json_string(snapshot)
}

/// 📥️ The `pack::from_json_str` inverse of [`encode_semio_cad_snapshot_json`] — decodes the
/// `before`/`after` halves of `📐️mutate-semio-cad`'s committed specification vectors
/// (`../../../../../🧪️tests/📐️mutate-semio-cad/🧫️fixtures/🦠️<kind>.json`) into real [`SemioCadSnapshot`]
/// values, so the adapter never hand-transcribes a fixture into a Rust literal that could silently
/// drift away from the JSON it claims to mirror.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_cad_snapshot_json(text: &str) -> Result<SemioCadSnapshot, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use super::*;
use crate::standards::v1::subsets::cad::schema::snapshot::*;
use crate::standards::v1::subsets::base::schema::geometry::native;
use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use crate::standards::v1::subsets::audio::io::text::diff::{strip_brackets};
use crate::standards::v1::subsets::audio::io::text::diff::{split_top_level};
use framework_schema::ArtifactSchema;




}
pub use snapshot_wire_codec::*;
