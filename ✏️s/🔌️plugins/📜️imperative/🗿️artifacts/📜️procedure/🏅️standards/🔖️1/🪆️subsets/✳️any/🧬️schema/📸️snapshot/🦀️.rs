//! 🧬️ Imperative snapshot schema — artifact-lane fields only.

use crate::artifacts::procedure::{ProcedureFlowChild, ProcedureTextChild};
use schema::ArtifactSchema;

//#region 🔖️Snapshot
/// 📸️ Persisted imperative document snapshot (persistent fields of the artifact). Ticket
/// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (`imperative→C:text,flow`): the inline `path:
/// Path` (the ordered/nested `Step` control-flow tree) and `seed: BTreeMap<String, Value>` (the
/// initial variable dictionary) content fields are replaced by two fixed composed CHILD slots —
/// this plugin no longer defines its own program-graph or seed-content model, it composes stdio's
/// `flow` and `text` subsets instead. `#[child(...)]` drives `#[derive(ArtifactSchema)]`'s
/// slot-table emission; never hand-written.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.imperative.procedure")]
pub struct ProcedureSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.flow")]
    pub flow: ProcedureFlowChild,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.text")]
    pub text: ProcedureTextChild,
}

impl Default for ProcedureSnapshot {
    fn default() -> Self {
        crate::artifacts::procedure::procedure_snapshot_with_content("procedure.document", &crate::artifacts::procedure::Path::new(), &std::collections::BTreeMap::new())
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️ChildCodecPrimitives
/// 🧪️ Real hex/bracket child-handle codec (mirrors `📐️cad`'s/`✒️writer`'s own `enc_child`/
/// `dec_child`) — a handle is exactly two strings (`child_id`, the target's `ArtifactRef`
/// flattened via `to_uri()`), never the child's own content.
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
fn enc_ref(r: &store::os_io::ArtifactRef) -> String {
    enc_str(&r.to_uri())
}
fn dec_ref(s: &str) -> Result<store::os_io::ArtifactRef, String> {
    store::os_io::ArtifactRef::parse_uri(&dec_str(s)?)
}

fn enc_flow_child(c: &ProcedureFlowChild) -> String {
    format!("[{},{}]", enc_str(&c.child_id), enc_ref(&c.target))
}
fn dec_flow_child(s: &str) -> Result<ProcedureFlowChild, String> {
    let inner = s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))?;
    let parts: Vec<&str> = inner.splitn(2, ',').collect();
    let [child_id, target] = parts.as_slice() else { return Err(format!("flow child handle: expected 2 fields, got {}", parts.len())) };
    Ok(store::ArtifactChild::new(dec_str(child_id)?, dec_ref(target)?))
}
fn enc_text_child(c: &ProcedureTextChild) -> String {
    format!("[{},{}]", enc_str(&c.child_id), enc_ref(&c.target))
}
fn dec_text_child(s: &str) -> Result<ProcedureTextChild, String> {
    let inner = s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))?;
    let parts: Vec<&str> = inner.splitn(2, ',').collect();
    let [child_id, target] = parts.as_slice() else { return Err(format!("text child handle: expected 2 fields, got {}", parts.len())) };
    Ok(store::ArtifactChild::new(dec_str(child_id)?, dec_ref(target)?))
}
//#endregion 🔖️ChildCodecPrimitives

//#region 🔖️TextPrimitives
fn print_procedure_snapshot_body(s: &ProcedureSnapshot) -> String {
    format!("schema={}\nflow={}\ntext={}", enc_str(&s.schema), enc_flow_child(&s.flow), enc_text_child(&s.text))
}
fn parse_procedure_snapshot_body(body: &str) -> Result<ProcedureSnapshot, String> {
    let mut schema = None;
    let mut flow = None;
    let mut text = None;
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("schema=") {
            schema = Some(dec_str(rest)?);
        } else if let Some(rest) = line.strip_prefix("flow=") {
            flow = Some(dec_flow_child(rest)?);
        } else if let Some(rest) = line.strip_prefix("text=") {
            text = Some(dec_text_child(rest)?);
        } else {
            return Err(format!("imperative snapshot: unknown line {line:?}"));
        }
    }
    Ok(ProcedureSnapshot {
        schema: schema.ok_or_else(|| "imperative snapshot: missing schema line".to_string())?,
        flow: flow.ok_or_else(|| "imperative snapshot: missing flow line".to_string())?,
        text: text.ok_or_else(|| "imperative snapshot: missing text line".to_string())?,
    })
}
//#endregion 🔖️TextPrimitives

//#region 🔖️BinaryPrimitives
fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}
fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}
fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}
fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}
fn write_ref(out: &mut Vec<u8>, r: &store::os_io::ArtifactRef) {
    write_str_lp(out, &r.to_uri());
}
fn read_ref(reader: &mut store::ByteReader<'_>) -> Result<store::os_io::ArtifactRef, String> {
    store::os_io::ArtifactRef::parse_uri(&read_str_lp(reader)?)
}
fn write_flow_child(out: &mut Vec<u8>, c: &ProcedureFlowChild) {
    write_str_lp(out, &c.child_id);
    write_ref(out, &c.target);
}
fn read_flow_child(reader: &mut store::ByteReader<'_>) -> Result<ProcedureFlowChild, String> {
    let child_id = read_str_lp(reader)?;
    let target = read_ref(reader)?;
    Ok(store::ArtifactChild::new(child_id, target))
}
fn write_text_child(out: &mut Vec<u8>, c: &ProcedureTextChild) {
    write_str_lp(out, &c.child_id);
    write_ref(out, &c.target);
}
fn read_text_child(reader: &mut store::ByteReader<'_>) -> Result<ProcedureTextChild, String> {
    let child_id = read_str_lp(reader)?;
    let target = read_ref(reader)?;
    Ok(store::ArtifactChild::new(child_id, target))
}

fn encode_procedure_snapshot_binary(s: &ProcedureSnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut out = vec![PACK_BINARY_FORMAT];
    write_str_lp(&mut out, &s.schema);
    write_flow_child(&mut out, &s.flow);
    write_text_child(&mut out, &s.text);
    out
}
fn decode_procedure_snapshot_binary(bytes: &[u8]) -> Result<ProcedureSnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    let schema = read_str_lp(&mut reader)?;
    let flow = read_flow_child(&mut reader)?;
    let text = read_text_child(&mut reader)?;
    Ok(ProcedureSnapshot { schema, flow, text })
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactCodecs
/// 🎁 Handcrafted `ArtifactDsl`/`ArtifactPack`, real hex/bracket text + LEB128 binary primitives —
/// the same upgrade `📐️cad`/`💠️lowpoly`/`✒️writer`/`🌊️flow` made when they gained a real
/// `ArtifactChild<S>` slot (the old `dsl::DslRecord`-derive-driven `Self::__dsl_spec()` path cannot
/// express a composed child slot, which has no `dsl::DslField` impl reachable from this crate).
impl store::ArtifactDsl for ProcedureSnapshot {
    const EXTENSION: &'static str = "imperative";
    fn envelope_id() -> &'static str {
        "imperative.imperative"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_procedure_snapshot_body(body).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let body = print_procedure_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for ProcedureSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_procedure_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let _ = options;
        decode_procedure_snapshot_binary(&inner).map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🌉️ExternalCodecBridge
/// 📤️ Renders an [`ProcedureSnapshot`] as this facet's own camelCase JSON projection — the
/// comparison surface `🛟️mutate-procedure-1`'s scenarios are measured through, and the shape the
/// committed `../🧬️mutations/<slug>/🧪️tests/<fixture>/📸️snapshot/{⬅️before,➡️after}/🔣️.json`
/// specification vectors are written in. It carries `flow` and `text` as content-addressed HANDLES,
/// never as content, which is what makes it a usable observability surface here: the `flow` handle
/// moves if and only if the program moved.
///
/// A thin `dsl::os_pack::json` wrapper over `ProcedureSnapshot`'s own `ToValue` impl — first-party,
/// infallible (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS).
pub fn encode_procedure_snapshot_json(snapshot: &ProcedureSnapshot) -> String {
    dsl::os_pack::json::to_json_string(snapshot)
}

/// 📥️ The inverse of [`encode_procedure_snapshot_json`] — decodes those committed specification
/// vectors into real [`ProcedureSnapshot`] values, so `🛟️mutate-procedure-1`'s adapter reads the
/// committed fixture rather than re-declaring it as a Rust literal beside it.
pub fn decode_procedure_snapshot_json(text: &str) -> Result<ProcedureSnapshot, String> {
    dsl::os_pack::json::from_json_str(text).map_err(|error| error.to_string())
}

/// 📝️ Parses `.imperative.dsl.semio` text into an [`ProcedureSnapshot`] — a named, non-async
/// pass-through of this type's own `store::ArtifactDsl` impl above, whose trait and error type are
/// both unnameable outside this crate, so `🛟️mutate-procedure-1`'s `identity-round-trip` scenario
/// reaches the real committed artifact (`../../📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`)
/// through this instead.
pub fn parse_procedure_dsl(text: &str) -> Result<ProcedureSnapshot, String> {
    <ProcedureSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}

/// 📝️ Renders an [`ProcedureSnapshot`] back as `.imperative.dsl.semio` text — the inverse of
/// [`parse_procedure_dsl`], preamble and both hex-encoded child-handle lines included.
pub fn print_procedure_dsl(snapshot: &ProcedureSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}
//#endregion 🌉️ExternalCodecBridge
