//! 🧬️ Imperative snapshot schema — persistent fields only.

use crate::artifacts::imperative::{ImperativeFlowChild, ImperativeTextChild};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted imperative document snapshot (persistent fields of the artifact). Ticket
/// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (`imperative→C:text,flow`): the inline `path:
/// Path` (the ordered/nested `Step` control-flow tree) and `seed: BTreeMap<String, Value>` (the
/// initial variable dictionary) content fields are replaced by two fixed composed CHILD slots —
/// this plugin no longer defines its own program-graph or seed-content model, it composes stdio's
/// `flow` and `text` subsets instead. `#[child(...)]` drives `#[derive(ArtifactSchema)]`'s
/// slot-table emission; never hand-written.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.imperative.imperative")]
pub struct ImperativeSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.flow")]
    pub flow: ImperativeFlowChild,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.text")]
    pub text: ImperativeTextChild,
}

impl Default for ImperativeSnapshot {
    fn default() -> Self {
        crate::artifacts::imperative::imperative_snapshot_with_content("imperative.document", &crate::artifacts::imperative::Path::new(), &std::collections::BTreeMap::new())
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

fn enc_flow_child(c: &ImperativeFlowChild) -> String {
    format!("[{},{}]", enc_str(&c.child_id), enc_ref(&c.target))
}
fn dec_flow_child(s: &str) -> Result<ImperativeFlowChild, String> {
    let inner = s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))?;
    let parts: Vec<&str> = inner.splitn(2, ',').collect();
    let [child_id, target] = parts.as_slice() else { return Err(format!("flow child handle: expected 2 fields, got {}", parts.len())) };
    Ok(store::ArtifactChild::new(dec_str(child_id)?, dec_ref(target)?))
}
fn enc_text_child(c: &ImperativeTextChild) -> String {
    format!("[{},{}]", enc_str(&c.child_id), enc_ref(&c.target))
}
fn dec_text_child(s: &str) -> Result<ImperativeTextChild, String> {
    let inner = s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))?;
    let parts: Vec<&str> = inner.splitn(2, ',').collect();
    let [child_id, target] = parts.as_slice() else { return Err(format!("text child handle: expected 2 fields, got {}", parts.len())) };
    Ok(store::ArtifactChild::new(dec_str(child_id)?, dec_ref(target)?))
}
//#endregion 🔖️ChildCodecPrimitives

//#region 🔖️TextPrimitives
fn print_imperative_snapshot_body(s: &ImperativeSnapshot) -> String {
    format!("schema={}\nflow={}\ntext={}", enc_str(&s.schema), enc_flow_child(&s.flow), enc_text_child(&s.text))
}
fn parse_imperative_snapshot_body(body: &str) -> Result<ImperativeSnapshot, String> {
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
    Ok(ImperativeSnapshot {
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
fn write_flow_child(out: &mut Vec<u8>, c: &ImperativeFlowChild) {
    write_str_lp(out, &c.child_id);
    write_ref(out, &c.target);
}
fn read_flow_child(reader: &mut store::ByteReader<'_>) -> Result<ImperativeFlowChild, String> {
    let child_id = read_str_lp(reader)?;
    let target = read_ref(reader)?;
    Ok(store::ArtifactChild::new(child_id, target))
}
fn write_text_child(out: &mut Vec<u8>, c: &ImperativeTextChild) {
    write_str_lp(out, &c.child_id);
    write_ref(out, &c.target);
}
fn read_text_child(reader: &mut store::ByteReader<'_>) -> Result<ImperativeTextChild, String> {
    let child_id = read_str_lp(reader)?;
    let target = read_ref(reader)?;
    Ok(store::ArtifactChild::new(child_id, target))
}

fn encode_imperative_snapshot_binary(s: &ImperativeSnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut out = vec![PACK_BINARY_FORMAT];
    write_str_lp(&mut out, &s.schema);
    write_flow_child(&mut out, &s.flow);
    write_text_child(&mut out, &s.text);
    out
}
fn decode_imperative_snapshot_binary(bytes: &[u8]) -> Result<ImperativeSnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    let schema = read_str_lp(&mut reader)?;
    let flow = read_flow_child(&mut reader)?;
    let text = read_text_child(&mut reader)?;
    Ok(ImperativeSnapshot { schema, flow, text })
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactCodecs
/// 🎁 Handcrafted `ArtifactDsl`/`ArtifactPack`, real hex/bracket text + LEB128 binary primitives —
/// the same upgrade `📐️cad`/`💠️lowpoly`/`✒️writer`/`🌊️flow` made when they gained a real
/// `ArtifactChild<S>` slot (the old `dsl::DslRecord`-derive-driven `Self::__dsl_spec()` path cannot
/// express a composed child slot, which has no `dsl::DslField` impl reachable from this crate).
impl store::ArtifactDsl for ImperativeSnapshot {
    const EXTENSION: &'static str = "imperative";
    fn envelope_id() -> &'static str {
        "imperative.imperative"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_imperative_snapshot_body(body).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let body = print_imperative_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        )
        .expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for ImperativeSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_imperative_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        )
        .map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let _ = options;
        decode_imperative_snapshot_binary(&inner).map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs
