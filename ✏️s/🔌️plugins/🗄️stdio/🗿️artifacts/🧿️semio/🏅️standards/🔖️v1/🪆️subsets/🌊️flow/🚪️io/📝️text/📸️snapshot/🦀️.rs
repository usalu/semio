//! 📝️ Text representation codec surface for `s.stdio.semio.flow` (snapshot) — grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::flow::schema::snapshot::*;
use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use crate::standards::v1::subsets::base::schema::triples::{split_top_level, strip_brackets};
use framework_schema::ArtifactSchema;

/// 🧪️ P2 pilot (flow, the FIRST semio subset upgraded): real hex/bracket-encoded value
/// primitives backing the hand-rolled `ArtifactDsl` below — same style as this subset's own
/// `🔺️diff`/`🧬️mutations` facets (`GifDiff`/`SvgDiff`/`DocxDiff`'s established hand-rolled
/// convention), duplicated here (not imported from `schema::diff`) to keep `snapshot` — the base
/// type `diff`/`mutations` both depend ON — free of a reverse dependency on either sibling facet.
///
/// 🧩️ The `#[derive(dsl::DslArtifact)]` path was tried first per this ticket's brief and hits a
/// real mechanism gap: `position: SemioPoint2` would need `SemioPoint2` (`engine::geometry`,
/// OUTSIDE this ticket's `🌊️flow/`-only edit scope) to implement `dsl::DslField`/`DslRecord`,
/// which it does not. Hand-rolled instead — see this wave's report `mechanism_gaps`.
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
pub(crate) fn enc_f64(v: f64) -> String {
    if v.is_nan(){format!("nan64_{:016x}",v.to_bits())}else{v.to_string()}
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_f64(s: &str) -> Result<f64, String> {
    if let Some(word)=s.strip_prefix("nan64_"){if word.len()!=16||!word.bytes().all(|byte|byte.is_ascii_hexdigit()){return Err("invalid Flow binary64 NaN word".into())}let bits=u64::from_str_radix(word,16).map_err(|error|error.to_string())?;let value=f64::from_bits(bits);if !value.is_nan(){return Err("Flow binary64 NaN word has a non-NaN class".into())}return Ok(value)}
    s.parse().map_err(|e: std::num::ParseFloatError| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_point2(p: &SemioPoint2) -> String {
    format!("[{},{}]", enc_f64(p.x), enc_f64(p.y))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_point2(s: &str) -> Result<SemioPoint2, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [x, y] = parts.as_slice() else { return Err(format!("point2: expected 2 fields, got {}", parts.len())) };
    Ok(SemioPoint2 { x: dec_f64(x)?, y: dec_f64(y)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_port_ref(p: &PortRef) -> String {
    format!("[{},{}]", enc_str(&p.node), enc_str(&p.port))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_port_ref(s: &str) -> Result<PortRef, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [node, port] = parts.as_slice() else { return Err(format!("port ref: expected 2 fields, got {}", parts.len())) };
    Ok(PortRef { node: dec_str(node)?, port: dec_str(port)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_param(p: &FlowParam) -> String {
    format!("[{},{}]", enc_str(&p.key), enc_str(&p.value))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_param(s: &str) -> Result<FlowParam, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [key, value] = parts.as_slice() else { return Err(format!("param: expected 2 fields, got {}", parts.len())) };
    Ok(FlowParam { key: dec_str(key)?, value: dec_str(value)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_node(n: &FlowNode) -> String {
    format!("[{},{},{},{},{}]", enc_str(&n.id), enc_str(&n.kind), enc_str(&n.label), format_args!("[{}]", n.params.iter().map(enc_param).collect::<Vec<_>>().join(",")), enc_point2(&n.position))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_node(s: &str) -> Result<FlowNode, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [id, kind, label, params, position] = parts.as_slice() else { return Err(format!("node: expected 5 fields, got {}", parts.len())) };
    let params = split_top_level(strip_brackets(params)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_param).collect::<Result<Vec<_>, String>>()?;
    Ok(FlowNode { id: dec_str(id)?, kind: dec_str(kind)?, label: dec_str(label)?, params, position: dec_point2(position)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_edge(e: &FlowEdge) -> String {
    format!("[{},{},{},{}]", enc_str(&e.id), enc_port_ref(&e.from), enc_port_ref(&e.to), enc_str(&e.kind))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_edge(s: &str) -> Result<FlowEdge, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [id, from, to, kind] = parts.as_slice() else { return Err(format!("edge: expected 4 fields, got {}", parts.len())) };
    Ok(FlowEdge { id: dec_str(id)?, from: dec_port_ref(from)?, to: dec_port_ref(to)?, kind: dec_str(kind)? })
}

/// 📄️ The real structured text body: three lines — `schema=<hex>`, `nodes=[<node>,...]`,
/// `edges=[<edge>,...]` — matching the grammar's `document = artifact-mark schema-line nodes-line
/// edges-line`. Newlines are pure lexer trivia in the shared dialect, so this is genuinely
/// recognizable by `dsl::Recognizer`, not merely readable.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_flow_snapshot_body(s: &SemioFlowSnapshot) -> String {
    format!("schema={}\nnodes=[{}]\nedges=[{}]", enc_str(&s.schema), s.nodes.iter().map(enc_node).collect::<Vec<_>>().join(","), s.edges.iter().map(enc_edge).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_flow_snapshot_body(body: &str) -> Result<SemioFlowSnapshot, String> {
    let mut schema = None;
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("schema=") {
            schema = Some(dec_str(rest)?);
        } else if let Some(rest) = line.strip_prefix("nodes=") {
            let inner = strip_brackets(rest)?;
            nodes = split_top_level(inner, ',').into_iter().filter(|s| !s.is_empty()).map(dec_node).collect::<Result<Vec<_>, String>>()?;
        } else if let Some(rest) = line.strip_prefix("edges=") {
            let inner = strip_brackets(rest)?;
            edges = split_top_level(inner, ',').into_iter().filter(|s| !s.is_empty()).map(dec_edge).collect::<Result<Vec<_>, String>>()?;
        } else {
            return Err(format!("flow snapshot: unknown line {line:?}"));
        }
    }
    let schema = schema.ok_or_else(|| "flow snapshot: missing schema line".to_string())?;
    Ok(SemioFlowSnapshot { schema, nodes, edges })
}

/// 🎁 Real structured text/binary codecs (P2 pilot — first semio subset upgraded off the old
/// hex-dump-of-`serde_json` shortcut). Wrapped in the repo-wide `store::semio_format` envelope,
/// unchanged.
impl store::ArtifactDsl for SemioFlowSnapshot {
    const EXTENSION: &'static str = "semio";
    fn envelope_id() -> &'static str {
        STDIO_SEMIOFLOW_DOCUMENT_SCHEMA
    }

    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_flow_snapshot_body(body).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let body = print_flow_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📥️ Parses this subset's own committed `.dsl.semio` text into a real [`SemioFlowSnapshot`] — a thin
/// wrapper over `store::ArtifactDsl::parse_dsl` so external Rust callers that cannot name this
/// crate's private `store` extern-crate item (the `🌊️mutate-semio-flow` test adapter, which reads the
/// REAL committed example artifact rather than a hand-transcribed Rust literal of it) can still
/// drive the same codec production does. Same rationale as `🧰️kit`'s `decode_kit_snapshot_json`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_semio_flow_dsl(text: &str) -> Result<SemioFlowSnapshot, String> {
    <SemioFlowSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| error.to_string())
}

/// 📤️ The `store::ArtifactDsl::print_dsl` inverse of [`parse_semio_flow_dsl`] — same rationale.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn print_semio_flow_dsl(snapshot: &SemioFlowSnapshot) -> String {
    <SemioFlowSnapshot as store::ArtifactDsl>::print_dsl(snapshot)
}

/// 📤️ This subset's own `#[value(rename_all = "camelCase")]` structural JSON projection of
/// `s.stdio.semio.flow` — the shape the `🌊️mutate-semio-flow` case compares under `ordered-json-v1`. A thin
/// `pack::to_json_string` wrapper (over `ToValue`/`FromValue`, first-party, per this ticket's
/// serde→value conversion), so a projection is derived from the snapshot type itself rather than
/// hand-written a second time in the adapter, where it could drift.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_semio_flow_snapshot_json(snapshot: &SemioFlowSnapshot) -> String {
    semio_framework_pack_json::to_json_string(snapshot)
}

/// 📥️ The `pack::from_json_str` inverse of [`encode_semio_flow_snapshot_json`] — decodes the
/// `before`/`after` halves of `🌊️mutate-semio-flow`'s committed specification vectors
/// (`../../../../../🧪️tests/🌊️mutate-semio-flow/🧫️fixtures/🦠️<kind>.json`) into real [`SemioFlowSnapshot`]
/// values, so the adapter never hand-transcribes a fixture into a Rust literal that could silently
/// drift away from the JSON it claims to mirror.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_flow_snapshot_json(text: &str) -> Result<SemioFlowSnapshot, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use super::*;
use crate::standards::v1::subsets::flow::schema::snapshot::*;
use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use crate::audio::io::text::diff::{strip_brackets};
use crate::audio::io::text::diff::{split_top_level};
use framework_schema::ArtifactSchema;




}
pub use snapshot_wire_codec::*;
