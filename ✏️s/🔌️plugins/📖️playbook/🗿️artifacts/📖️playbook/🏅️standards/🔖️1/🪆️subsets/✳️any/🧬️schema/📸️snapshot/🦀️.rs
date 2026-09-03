//! 🧬️ Playbook snapshot schema — artifact-lane fields only.
//!
//! Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (`playbook→C:document,flow`): `steps:
//! Vec<PlaybookStep>` is replaced by the composed `document`/`flow` child slots (see the artifact
//! root's `🔖️ContentBridge` region). This struct no longer maps 1:1 onto the kernel `PlaybookSpec`,
//! so `store::ArtifactDsl`/`ArtifactPack` are hand-rolled directly (the codec wall every composed
//! subset in this ticket hits — `ArtifactChild<S>` has no `DslField` impl) rather than delegating to
//! `PlaybookSpec::__dsl_to_record`/`__dsl_spec` as before.

use crate::artifacts::playbook::PlaybookStep;
use schema::ArtifactSchema;
// 🔬️ `Serialize`/`Deserialize` survive ONLY as a `#[cfg(test)]` differential oracle (see
// `scene_owner_fixture_proves_identity_isolation_aba_wire_omission_and_bounded_close`'s
// "third-party serde oracle" case in `../../🦀️.rs`, which checks `ArtifactChild::local_owner`'s
// `#[serde(skip)]` treatment against real serde) — never a production dependency of this crate.
// `store::ArtifactChild<S>` itself only derives `Serialize`/`Deserialize` under the same
// `#[cfg(test)]` gate, so this struct's own derive must mirror it exactly.
#[cfg(test)]
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted playbook document snapshot (persistent fields of the artifact). `#[child(...)]`
/// drives `#[derive(ArtifactSchema)]`'s slot-table emission; never hand-written.
#[derive(Clone, Debug, PartialEq, ArtifactSchema)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[artifact_schema(id = "s.playbook.playbook")]
pub struct PlaybookSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub id: String,
    #[state(artifact)]
    pub version: String,
    #[state(artifact)]
    pub title: Option<String>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.document")]
    pub document: crate::artifacts::playbook::PlaybookDocumentChild,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.flow")]
    pub flow: crate::artifacts::playbook::PlaybookFlowChild,
}

impl Default for PlaybookSnapshot {
    fn default() -> Self {
        let kernel = crate::playbook::empty_playbook_snapshot();
        Self::from_kernel(crate::playbook::PlaybookSpec { schema: kernel.schema, id: kernel.id, version: kernel.version, title: kernel.title, steps: kernel.steps })
    }
}

impl PlaybookSnapshot {
    /// 🌉️ Builds a plugin snapshot from the shared kernel `PlaybookSpec`, minting/caching the
    /// composed `document`/`flow` children from its `steps`.
    pub fn from_kernel(spec: crate::playbook::PlaybookSpec) -> Self {
        crate::artifacts::playbook::playbook_snapshot_with_steps(&spec.schema, &spec.id, &spec.version, spec.title, spec.steps)
    }

    /// 🌉️ Lowers this snapshot into the kernel `PlaybookSpec` for shared domain helpers — reads
    /// steps off the `flow` child's working-scene cache (see the artifact root's `🔖️WorkingScene`).
    pub fn to_kernel(self) -> crate::playbook::PlaybookSpec {
        self.as_kernel()
    }

    /// 🌉️ Borrows as kernel spec without consuming `self`.
    pub fn as_kernel(&self) -> crate::playbook::PlaybookSpec {
        crate::playbook::PlaybookSpec { schema: self.schema.clone(), id: self.id.clone(), version: self.version.clone(), title: self.title.clone(), steps: crate::artifacts::playbook::playbook_steps(self) }
    }

    /// 🔎️ The current steps, read through the composed `flow` child's working-scene cache — the
    /// single call site every render/inference/export path in this plugin uses instead of the old
    /// `.steps` field access.
    pub fn steps(&self) -> Vec<PlaybookStep> {
        crate::artifacts::playbook::playbook_steps(self)
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️ValueCodec
/// 🔀️ Hand-written, not derived — mirrors the sibling `PlaybookArtifact` impl one region up
/// (`../🦀️.rs`'s `🔖️ValueCodec`): `document`/`flow` are `store::ArtifactChild<S>` composed-artifact
/// handles, bridged per-field through the pre-existing `to_dsl_value`/`from_dsl_value` seam instead
/// of widening the derive macro to understand child-slot handles.
impl ::semio_framework_os_kernel::ToValue for PlaybookSnapshot {
    fn to_value(&self) -> ::semio_framework_os_kernel::DslValue {
        ::semio_framework_os_kernel::DslValue::object([
            ("schema".to_string(), ::semio_framework_os_kernel::ToValue::to_value(&self.schema)),
            ("id".to_string(), ::semio_framework_os_kernel::ToValue::to_value(&self.id)),
            ("version".to_string(), ::semio_framework_os_kernel::ToValue::to_value(&self.version)),
            ("title".to_string(), ::semio_framework_os_kernel::ToValue::to_value(&self.title)),
            ("document".to_string(), ::semio_framework_os_kernel::to_dsl_value(&self.document).expect("ArtifactChild serializes")),
            ("flow".to_string(), ::semio_framework_os_kernel::to_dsl_value(&self.flow).expect("ArtifactChild serializes")),
        ])
    }
}
impl ::semio_framework_os_kernel::FromValue for PlaybookSnapshot {
    fn from_value(value: ::semio_framework_os_kernel::DslValue) -> Result<Self, ::semio_framework_os_kernel::ValueError> {
        let entries = value.into_object()?;
        let get = |key: &str| entries.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone());
        let field = |key: &str| get(key).ok_or_else(|| ::semio_framework_os_kernel::ValueError::new(format!("missing field `{key}`")));
        Ok(Self {
            schema: ::semio_framework_os_kernel::FromValue::from_value(field("schema")?)?,
            id: ::semio_framework_os_kernel::FromValue::from_value(field("id")?)?,
            version: ::semio_framework_os_kernel::FromValue::from_value(field("version")?)?,
            title: ::semio_framework_os_kernel::FromValue::from_value(field("title")?)?,
            document: ::semio_framework_os_kernel::from_dsl_value(field("document")?).map_err(::semio_framework_os_kernel::ValueError::new)?,
            flow: ::semio_framework_os_kernel::from_dsl_value(field("flow")?).map_err(::semio_framework_os_kernel::ValueError::new)?,
        })
    }
}
//#endregion 🔖️ValueCodec

//#region 🔖️ChildCodecPrimitives
/// 🧪️ Real hex/bracket child-handle codec (mirrors writer's/raster's own `enc_child`/`dec_child`) —
/// a handle is exactly two strings (`child_id`, the target's `ArtifactRef` flattened via
/// `to_uri()`), never the child's own content. Generic over the phantom `S` so one pair of helpers
/// backs both the `document` and `flow` slots.
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
fn enc_opt_str(v: &Option<String>) -> String {
    match v {
        None => "[0]".to_string(),
        Some(s) => format!("[1,{}]", enc_str(s)),
    }
}
fn dec_opt_str(s: &str) -> Result<Option<String>, String> {
    let inner = s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))?;
    match inner.splitn(2, ',').collect::<Vec<_>>().as_slice() {
        ["0"] => Ok(None),
        [tag, value] if *tag == "1" => Ok(Some(dec_str(value)?)),
        other => Err(format!("option: bad shape {other:?}")),
    }
}
fn enc_ref(r: &store::os_io::ArtifactRef) -> String {
    enc_str(&r.to_uri())
}
fn dec_ref(s: &str) -> Result<store::os_io::ArtifactRef, String> {
    store::os_io::ArtifactRef::parse_uri(&dec_str(s)?)
}
fn enc_child<S>(c: &store::ArtifactChild<S>) -> String {
    format!("[{},{}]", enc_str(&c.child_id), enc_ref(&c.target))
}
fn dec_child<S>(s: &str) -> Result<store::ArtifactChild<S>, String> {
    let inner = s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))?;
    let parts: Vec<&str> = inner.splitn(2, ',').collect();
    let [child_id, target] = parts.as_slice() else { return Err(format!("child handle: expected 2 fields, got {}", parts.len())) };
    Ok(store::ArtifactChild::new(dec_str(child_id)?, dec_ref(target)?))
}
//#endregion 🔖️ChildCodecPrimitives

//#region 🔖️TextPrimitives
fn print_playbook_snapshot_body(s: &PlaybookSnapshot) -> String {
    format!("schema={}\nid={}\nversion={}\ntitle={}\ndocument={}\nflow={}", enc_str(&s.schema), enc_str(&s.id), enc_str(&s.version), enc_opt_str(&s.title), enc_child(&s.document), enc_child(&s.flow))
}
fn parse_playbook_snapshot_body(body: &str) -> Result<PlaybookSnapshot, String> {
    let mut snapshot = PlaybookSnapshot::default();
    let mut saw_schema = false;
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("schema=") {
            snapshot.schema = dec_str(rest)?;
            saw_schema = true;
        } else if let Some(rest) = line.strip_prefix("id=") {
            snapshot.id = dec_str(rest)?;
        } else if let Some(rest) = line.strip_prefix("version=") {
            snapshot.version = dec_str(rest)?;
        } else if let Some(rest) = line.strip_prefix("title=") {
            snapshot.title = dec_opt_str(rest)?;
        } else if let Some(rest) = line.strip_prefix("document=") {
            snapshot.document = dec_child(rest)?;
        } else if let Some(rest) = line.strip_prefix("flow=") {
            snapshot.flow = dec_child(rest)?;
        } else {
            return Err(format!("playbook snapshot: unknown line {line:?}"));
        }
    }
    if !saw_schema {
        return Err("playbook snapshot: missing schema line".to_string());
    }
    Ok(snapshot)
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
fn write_opt_str(out: &mut Vec<u8>, v: &Option<String>) {
    match v {
        None => out.push(0),
        Some(s) => {
            out.push(1);
            write_str_lp(out, s);
        }
    }
}
fn read_opt_str(reader: &mut store::ByteReader<'_>) -> Result<Option<String>, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(None),
        1 => Ok(Some(read_str_lp(reader)?)),
        other => Err(format!("opt str: bad tag {other}")),
    }
}
fn write_ref(out: &mut Vec<u8>, r: &store::os_io::ArtifactRef) {
    write_str_lp(out, &r.to_uri());
}
fn read_ref(reader: &mut store::ByteReader<'_>) -> Result<store::os_io::ArtifactRef, String> {
    store::os_io::ArtifactRef::parse_uri(&read_str_lp(reader)?)
}
fn write_child<S>(out: &mut Vec<u8>, c: &store::ArtifactChild<S>) {
    write_str_lp(out, &c.child_id);
    write_ref(out, &c.target);
}
fn read_child<S>(reader: &mut store::ByteReader<'_>) -> Result<store::ArtifactChild<S>, String> {
    let child_id = read_str_lp(reader)?;
    let target = read_ref(reader)?;
    Ok(store::ArtifactChild::new(child_id, target))
}

fn encode_playbook_snapshot_binary(s: &PlaybookSnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut out = vec![PACK_BINARY_FORMAT];
    write_str_lp(&mut out, &s.schema);
    write_str_lp(&mut out, &s.id);
    write_str_lp(&mut out, &s.version);
    write_opt_str(&mut out, &s.title);
    write_child(&mut out, &s.document);
    write_child(&mut out, &s.flow);
    out
}
fn decode_playbook_snapshot_binary(bytes: &[u8]) -> Result<PlaybookSnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    let mut snapshot = PlaybookSnapshot::default();
    snapshot.schema = read_str_lp(&mut reader)?;
    snapshot.id = read_str_lp(&mut reader)?;
    snapshot.version = read_str_lp(&mut reader)?;
    snapshot.title = read_opt_str(&mut reader)?;
    snapshot.document = read_child(&mut reader)?;
    snapshot.flow = read_child(&mut reader)?;
    Ok(snapshot)
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactCodecs
impl store::ArtifactDsl for PlaybookSnapshot {
    const EXTENSION: &'static str = "playbook";
    fn envelope_id() -> &'static str {
        "playbook.playbook"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_playbook_snapshot_body(body).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let body = print_playbook_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for PlaybookSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_playbook_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let _ = options;
        decode_playbook_snapshot_binary(&inner).map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️ExternalBridges
/// 📖️ Parses `.playbook` DSL text with a plain-`String` error, reachable from OUTSIDE this crate —
/// `store` is a private `extern crate` alias (`🦀️.rs`), so `store::TextError` cannot be named
/// by the exhaustive mutation case's test adapter that has to read the committed
/// `🗣️.dsl.semio` artifact.
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn parse_playbook_dsl(text: &str) -> Result<PlaybookSnapshot, String> {
    <PlaybookSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| error.to_string())
}

/// 🖨️ Prints a [`PlaybookSnapshot`] back to `.playbook` DSL text under a name an external caller can reach, paired
/// with [`parse_playbook_dsl`].
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn print_playbook_dsl(snapshot: &PlaybookSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}
//#endregion 🔖️ExternalBridges
