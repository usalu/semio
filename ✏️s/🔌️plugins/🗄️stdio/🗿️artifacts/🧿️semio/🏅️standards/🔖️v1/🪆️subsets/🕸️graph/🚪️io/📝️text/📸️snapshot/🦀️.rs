//! 📝️ Text representation codec surface for `stdio.semio.graph` (snapshot) — grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::graph::schema::snapshot::*;
use crate::standards::v1::subsets::base::schema::geometry::native;
use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use crate::standards::v1::subsets::base::io::text::snapshot::{split_top_level, strip_brackets};
use crate::standards::v1::subsets::value::io::text::diff::{dec_semio_value_entry};
use crate::standards::v1::subsets::value::io::text::diff::{enc_semio_value_entry};
use crate::standards::v1::subsets::value::io::binary::diff::{dec_semio_value_bin};
use crate::standards::v1::subsets::value::io::binary::diff::{enc_semio_value_bin};
use crate::standards::v1::subsets::value::schema::snapshot::SemioValueEntry;
use framework_schema::ArtifactSchema;

/// 🧪️ Real hex/bracket-encoded value primitives backing the hand-rolled `ArtifactDsl` below — same
/// style `🔤️text`'s own `📸️snapshot`/`🔺️diff`/`🧬️mutations` facets already establish, duplicated
/// locally (not imported across facets) to keep each facet module independently compilable.
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

/// 🆔️ `GraphNodeId`/`GraphEdgeId` encode as a bare hex token directly — same convention a run's
/// `language` field uses in `🔤️text`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_node_id(id: &GraphNodeId) -> String {
    enc_str(&id.value)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_node_id(s: &str) -> Result<GraphNodeId, String> {
    Ok(GraphNodeId::new(dec_str(s)?))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_edge_id(id: &GraphEdgeId) -> String {
    enc_str(&id.value)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_edge_id(s: &str) -> Result<GraphEdgeId, String> {
    Ok(GraphEdgeId::new(dec_str(s)?))
}

/// 🔢 `SemioPoint2`'s `x`/`y` are `f64`; encoded as exact numeric words wrapped in hex
/// (text-lexeme style — never round-tripped through a binary float type in the TEXT codec), parsed
/// back with the explicit numeric-word parser. Two flat comma-separated tokens, no wrapping brackets, so they slot
/// directly into an outer bracketed field list (matches this facet's committed grammar).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_point2_fields(p: &SemioPoint2) -> String {
    format!("{},{}", enc_str(&native::NativeF64(p.x).to_string()), enc_str(&native::NativeF64(p.y).to_string()))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_f64_hex(s: &str) -> Result<f64, String> {
    native::parse(&dec_str(s)?)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_port_kind(k: SemioGraphPortKind) -> char {
    match k {
        SemioGraphPortKind::In => 'i',
        SemioGraphPortKind::Out => 'o',
        SemioGraphPortKind::InOut => 'x',
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_port_kind(s: &str) -> Result<SemioGraphPortKind, String> {
    match s {
        "i" => Ok(SemioGraphPortKind::In),
        "o" => Ok(SemioGraphPortKind::Out),
        "x" => Ok(SemioGraphPortKind::InOut),
        other => Err(format!("bad port kind {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_port(p: &SemioGraphPort) -> String {
    format!("[{},{},{},{}]", enc_str(&p.name), enc_port_kind(p.kind), enc_str(&p.category), enc_list(&p.properties, enc_property))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_port(s: &str) -> Result<SemioGraphPort, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, kind, category, properties] = parts.as_slice() else { return Err(format!("port: expected 4 fields, got {}", parts.len())) };
    Ok(SemioGraphPort { name: dec_str(name)?, kind: dec_port_kind(kind)?, category:dec_str(category)?, properties:dec_list(properties,dec_property)? })
}

/// 🍃️ A property list element is `enc_semio_value_entry(&p)`'s raw output (`hexkey:value`),
/// embedded directly as one comma-separated list element — its own internal `:` never collides
/// with the outer `,`/`[]` delimiters, so no extra wrapping brackets are needed (REUSE of
/// `🔢️value`'s diff-facet helpers, not a locally reinvented codec).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_property(p: &SemioValueEntry) -> String {
    enc_semio_value_entry(p)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_property(s: &str) -> Result<SemioValueEntry, String> {
    dec_semio_value_entry(s)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_node(n: &SemioGraphNode) -> String {
    format!("[{},{},{},{},{},{},{},{}]", enc_node_id(&n.id), enc_str(&n.kind), enc_str(&n.label), enc_point2_fields(&n.position), enc_str(&native::NativeF64(n.width).to_string()), enc_str(&native::NativeF64(n.height).to_string()), enc_list(&n.ports, enc_port), enc_list(&n.properties, enc_property))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_node(s: &str) -> Result<SemioGraphNode, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, kind, label, x, y, width, height, ports, properties] = parts.as_slice() else {
        return Err(format!("node: expected 9 fields, got {}", parts.len()));
    };
    Ok(SemioGraphNode { id: dec_node_id(id)?, kind: dec_str(kind)?, label: dec_str(label)?, position: SemioPoint2 { x: dec_f64_hex(x)?, y: dec_f64_hex(y)? }, width:dec_f64_hex(width)?,height:dec_f64_hex(height)?, ports: dec_list(ports, dec_port)?, properties: dec_list(properties, dec_property)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_edge(e: &SemioGraphEdge) -> String {
    format!("[{},{},{},{},{},{},{},{}]", enc_edge_id(&e.id), enc_node_id(&e.source), enc_node_id(&e.target), enc_str(&e.kind), enc_str(&e.label), enc_optional(e.source_port.as_deref()), enc_optional(e.target_port.as_deref()), enc_list(&e.properties,enc_property))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_edge(s: &str) -> Result<SemioGraphEdge, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, source, target, kind, label, source_port, target_port, properties] = parts.as_slice() else {
        return Err(format!("edge: expected 8 fields, got {}", parts.len()));
    };
    Ok(SemioGraphEdge { id: dec_edge_id(id)?, source: dec_node_id(source)?, target: dec_node_id(target)?, kind: dec_str(kind)?, label: dec_str(label)?, source_port:dec_optional(source_port)?,target_port:dec_optional(target_port)?,properties:dec_list(properties,dec_property)? })
}

pub(crate) fn enc_optional(value:Option<&str>)->String{match value{Some(value)=>format!("[{}]",enc_str(value)),None=>"-".into()}}

pub(crate) fn dec_optional(value:&str)->Result<Option<String>,String>{if value=="-"{Ok(None)}else{Ok(Some(dec_str(strip_brackets(value)?)?))}}

/// 📄️ The real structured graph body: three lines — `schema=<hex>`, `nodes=[<node>,...]`,
/// `edges=[<edge>,...]` — matching the grammar's `document = artifact-mark schema-line nodes-line
/// edges-line`. Newlines are pure lexer trivia in the shared dialect.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_graph_snapshot_body(s: &SemioGraphSnapshot) -> String {
    format!("schema={}\nnodes={}\nedges={}", enc_str(&s.schema), enc_list(&s.nodes, enc_node), enc_list(&s.edges, enc_edge))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_graph_snapshot_body(body: &str) -> Result<SemioGraphSnapshot, String> {
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
            nodes = dec_list(rest, dec_node)?;
        } else if let Some(rest) = line.strip_prefix("edges=") {
            edges = dec_list(rest, dec_edge)?;
        } else {
            return Err(format!("semio graph snapshot: unknown line {line:?}"));
        }
    }
    Ok(SemioGraphSnapshot { schema: schema.ok_or_else(|| "semio graph snapshot: missing schema line".to_string())?, nodes, edges })
}

/// 🎁 Real structured text/binary codecs, wrapped in the repo-wide `store::semio_format` envelope.
impl store::ArtifactDsl for SemioGraphSnapshot {
    const EXTENSION: &'static str = "semio";
    fn envelope_id() -> &'static str {
        STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA
    }

    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_graph_snapshot_body(body).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let body = print_graph_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📤️ Projects the complete declared graph JSON with exact binary64 geometry words,
/// literal endpoint components and all nine intrinsic property families.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_semio_graph_snapshot_json(snapshot: &SemioGraphSnapshot) -> Result<String, semio_framework_value::ValueError> {
    super::snapshot_wire2_codec::encode(snapshot)
}

/// 📥️ Reads the closed declared graph JSON through the first-party Reject-member parser
/// and checked role binding, preserving raw geometry words and typed refusal origins.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_graph_snapshot_json(text: &str) -> Result<SemioGraphSnapshot, semio_framework_value::ValueError> {
    super::snapshot_wire2_codec::decode(text)
}

/// 📝️ Parses `s.stdio.semio.graph` DSL text into a [`SemioGraphSnapshot`] — a named pass-through of this snapshot's own
/// `store::ArtifactDsl` impl above, whose trait and error type are both unnameable outside this
/// crate, so `🌳️mutate-semio-graph`'s `identity-round-trip` scenario reaches the real committed
/// artifact (`../../🖼️assets/🕸️wires/🗣️.dsl.semio`) through this instead.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_semio_graph_dsl(text: &str) -> Result<SemioGraphSnapshot, String> {
    <SemioGraphSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| error.to_string())
}

/// 📝️ Renders a [`SemioGraphSnapshot`] back as `s.stdio.semio.graph` DSL text — the inverse of
/// [`parse_semio_graph_dsl`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn print_semio_graph_dsl(snapshot: &SemioGraphSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use super::*;
use crate::standards::v1::subsets::graph::schema::snapshot::*;
use crate::standards::v1::subsets::base::schema::geometry::native;
use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use crate::standards::v1::subsets::audio::io::text::diff::{strip_brackets};
use crate::standards::v1::subsets::audio::io::text::diff::{split_top_level};
use crate::standards::v1::subsets::value::io::text::diff::{dec_semio_value_entry};
use crate::standards::v1::subsets::value::io::text::diff::{enc_semio_value_entry};
use crate::standards::v1::subsets::value::io::binary::diff::{dec_semio_value_bin};
use crate::standards::v1::subsets::value::io::binary::diff::{enc_semio_value_bin};
use crate::standards::v1::subsets::value::schema::snapshot::SemioValueEntry;
use framework_schema::ArtifactSchema;




}
pub use snapshot_wire_codec::*;

#[allow(unused_imports)]
mod snapshot_wire2_codec {

use semio_framework_value::{DslValue,FromValue,ToValue};
use semio_framework_value::{ValueError,ValueRefusalKind};

pub(crate) fn invalid(message:&str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}

pub(crate) fn fields(value:&mut DslValue,required:&[&str],optional:&[&str])->Result<(),ValueError>{
 let DslValue::Object(entries)=value else{return Err(invalid("Semio graph declared object required"))};
 if required.iter().any(|key|!entries.iter().any(|(name,_)|name==key))||entries.iter().any(|(name,_)|!required.contains(&name.as_str())&&!optional.contains(&name.as_str())){return Err(invalid("Semio graph declared fields invalid"))}Ok(())
}

pub(crate) fn member<'a>(value:&'a mut DslValue,key:&str)->Result<&'a mut DslValue,ValueError>{let DslValue::Object(entries)=value else{return Err(invalid("Semio graph object required"))};entries.iter_mut().find(|(name,_)|name==key).map(|(_,value)|value).ok_or_else(||invalid("Semio graph role missing"))}

pub(crate) fn word(value:&mut DslValue,decode:bool)->Result<(),ValueError>{if decode{
 fields(value,&["bits"],&[])?;let bits=member(value,"bits")?.as_str().ok_or_else(||invalid("Semio graph hex64 text required"))?;
 if bits.len()!=16||!bits.bytes().all(|byte|byte.is_ascii_digit()||(b'a'..=b'f').contains(&byte)){return Err(invalid("Semio graph lowercase hex64 required"))}
 *value=DslValue::float(f64::from_bits(u64::from_str_radix(bits,16).map_err(|_|invalid("Semio graph word invalid"))?));
 }else{let bits=value.as_f64().ok_or_else(||invalid("Semio graph binary64 owner required"))?.to_bits();*value=DslValue::object([("bits".into(),DslValue::String(format!("{bits:016x}")))]);}Ok(())
}

pub fn convert(mut value:DslValue,decode:bool)->Result<DslValue,ValueError>{
 fields(&mut value,&["schema","nodes","edges"],&[])?;
 let DslValue::Array(nodes)=member(&mut value,"nodes")? else{return Err(invalid("Semio graph nodes required"))};
 for node in nodes{
  fields(node,&["id","kind","label","position","width","height","ports","properties"],&[])?;
  let point=member(node,"position")?;fields(point,&["x","y"],&[])?;word(member(point,"x")?,decode)?;word(member(point,"y")?,decode)?;word(member(node,"width")?,decode)?;word(member(node,"height")?,decode)?;
 }
 let DslValue::Array(edges)=member(&mut value,"edges")? else{return Err(invalid("Semio graph edges required"))};for edge in edges{fields(edge,&["id","source","target","kind","label","properties"],&["sourcePort","targetPort"])?;}Ok(value)
}

pub fn encode(snapshot:&crate::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot)->Result<String,ValueError>{let value=convert(snapshot.to_value(),false)?;Ok(semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&value)))}

pub fn decode(text:&str)->Result<crate::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot,ValueError>{let parsed=semio_framework_pack_json::parse(text,semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error|invalid(&error.to_string()))?;crate::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot::from_value(convert(semio_framework_pack_json::to_dsl_value(&parsed),true)?)}
}
pub use snapshot_wire2_codec::*;
