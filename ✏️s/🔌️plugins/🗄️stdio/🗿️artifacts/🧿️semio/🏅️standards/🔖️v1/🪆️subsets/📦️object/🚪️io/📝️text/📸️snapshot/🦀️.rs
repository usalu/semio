//! 📝️ Text representation codec surface for `s.stdio.semio.object` (snapshot) — grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::object::schema::snapshot::*;
use crate::standards::v1::subsets::base::schema::geometry::SemioTransform;
use crate::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use crate::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use crate::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
use framework_schema::ArtifactSchema;

/// 🧪️ Real hex/bracket child-handle codec — a handle is exactly two strings (`child_id`, the
/// target's `ArtifactRef` flattened via `to_uri()`), never the child's own content (composition
/// rule: "a child handle is two strings; that is all the parent stores").
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
pub(crate) fn enc_ref(r: &semio_framework_artifact_reference::ArtifactRef) -> String {
    format!("[{},{},{},{}]",enc_str(&r.artifact_id),enc_str(&r.dialect.artifact_kind),enc_str(&r.dialect.standard),enc_str(&r.dialect.subset))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_ref(s: &str) -> Result<semio_framework_artifact_reference::ArtifactRef, String> {
    let parts=crate::standards::v1::subsets::base::io::text::snapshot::split_top_level(crate::standards::v1::subsets::base::io::text::snapshot::strip_brackets(s)?,',');
    let[id,kind,standard,subset]=parts.as_slice()else{return Err("reference requires four literal fields".into())};
    Ok(semio_framework_artifact_reference::ArtifactRef{artifact_id:dec_str(id)?,dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:dec_str(kind)?,standard:dec_str(standard)?,subset:dec_str(subset)?}})
}

/// 🪪️ One literal child identity and four literal reference fields.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_child<S>(c: &store::ArtifactChild<S>) -> String {
    format!("[{},{}]", enc_str(&c.child_id), enc_ref(&c.target))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_child<S>(s: &str) -> Result<store::ArtifactChild<S>, String> {
    let parts = crate::standards::v1::subsets::base::io::text::snapshot::split_top_level(crate::standards::v1::subsets::base::io::text::snapshot::strip_brackets(s)?, ',');
    let [child_id, target] = parts.as_slice() else { return Err(format!("child handle: expected 2 fields, got {}", parts.len())) };
    Ok(store::ArtifactChild::new(dec_str(child_id)?, dec_ref(target)?))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_child_opt<S>(c: &Option<store::ArtifactChild<S>>) -> String {
    match c {
        Some(c) => enc_child(c),
        None => "[]".to_string(),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_child_opt<S>(s: &str) -> Result<Option<store::ArtifactChild<S>>, String> {
    if s == "[]" {
        return Ok(None);
    }
    Ok(Some(dec_child(s)?))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_transform(t: &SemioTransform) -> String {
    use crate::standards::v1::subsets::base::schema::geometry::native::NativeF64;
    format!("[{},{},{},{},{},{},{},{},{},{}]", NativeF64(t.translation.x), NativeF64(t.translation.y), NativeF64(t.translation.z), NativeF64(t.rotation.x), NativeF64(t.rotation.y), NativeF64(t.rotation.z), NativeF64(t.rotation.w), NativeF64(t.scale.x), NativeF64(t.scale.y), NativeF64(t.scale.z))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_transform(s: &str) -> Result<SemioTransform, String> {
    let parts = crate::standards::v1::subsets::base::io::text::snapshot::split_top_level(crate::standards::v1::subsets::base::io::text::snapshot::strip_brackets(s)?, ',');
    let [tx, ty, tz, rx, ry, rz, rw, sx, sy, sz] = parts.as_slice() else {
        return Err(format!("transform: expected 10 fields, got {}", parts.len()));
    };
    let f = crate::standards::v1::subsets::base::schema::geometry::native::parse;
    use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint3, SemioQuaternion};
    Ok(SemioTransform { translation: SemioPoint3 { x: f(tx)?, y: f(ty)?, z: f(tz)? }, rotation: SemioQuaternion { x: f(rx)?, y: f(ry)?, z: f(rz)?, w: f(rw)? }, scale: SemioPoint3 { x: f(sx)?, y: f(sy)?, z: f(sz)? } })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_object_snapshot_body(s: &SemioObjectSnapshot) -> String {
    format!("schema={}\ntransform={}\nbrep={}\nmesh={}\nproperties={}", enc_str(&s.schema), enc_transform(&s.transform), enc_child_opt(&s.brep), enc_child_opt(&s.mesh), enc_child_opt(&s.properties),)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_object_snapshot_body(body: &str) -> Result<SemioObjectSnapshot, String> {
    let mut schema = None;
    let mut transform = None;
    let mut brep = None;
    let mut mesh = None;
    let mut properties = None;
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("schema=") {
            schema = Some(dec_str(rest)?);
        } else if let Some(rest) = line.strip_prefix("transform=") {
            transform = Some(dec_transform(rest)?);
        } else if let Some(rest) = line.strip_prefix("brep=") {
            brep = dec_child_opt(rest)?;
        } else if let Some(rest) = line.strip_prefix("mesh=") {
            mesh = dec_child_opt(rest)?;
        } else if let Some(rest) = line.strip_prefix("properties=") {
            properties = dec_child_opt(rest)?;
        } else {
            return Err(format!("semio object snapshot: unknown line {line:?}"));
        }
    }
    Ok(SemioObjectSnapshot { schema: schema.ok_or_else(|| "semio object snapshot: missing schema line".to_string())?, transform: transform.ok_or_else(|| "semio object snapshot: missing transform line".to_string())?, brep, mesh, properties })
}

impl store::ArtifactDsl for SemioObjectSnapshot {
    const EXTENSION: &'static str = "semio";
    fn envelope_id() -> &'static str {
        STDIO_SEMIOOBJECT_DOCUMENT_SCHEMA
    }

    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_object_snapshot_body(body).and_then(|snapshot| { snapshot.validate()?; Ok(snapshot) }).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let body = print_object_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📤️ This subset's own `#[value(rename_all = "camelCase")]`-shaped structural JSON projection of
/// `s.stdio.semio.object` — the shape `📦️mutate-semio-object` compares under `ordered-json-v1`,
/// derived from the snapshot type's own hand-written `ToValue` impl above (§ValueCodec) rather
/// than hand-written a second time in the adapter, where it could drift away from the type it
/// claims to project. This is the bridge that makes the CHILD slots reachable at all — `ToValue`
/// bridges each `brep`/`mesh`/`properties` field through `to_dsl_value` (real ArtifactChild data:
/// `child_id` + `target` URI, never embedded content).
/// A thin `pack::to_json_string` wrapper (first-party, over `ToValue`/`DslValue`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_semio_object_snapshot_json(snapshot: &SemioObjectSnapshot) -> String {
    semio_framework_pack_json::to_json_string(snapshot)
}

/// 📥️ The `pack::from_json_str` inverse of [`encode_semio_object_snapshot_json`] — decodes the
/// committed `../🧬️mutations/<kind>/🧪️tests/<fixture>/📸️snapshot/{⬅️before,➡️after}/🔣️.json`
/// specification vectors into real [`SemioObjectSnapshot`] values, so `📦️mutate-semio-object`'s
/// adapter reads the committed fixture instead of re-declaring it as a Rust literal beside it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_object_snapshot_json(text: &str) -> Result<SemioObjectSnapshot, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// 📝️ Parses `s.stdio.semio.object` DSL text into a [`SemioObjectSnapshot`] — a named pass-through of this snapshot's own
/// `store::ArtifactDsl` impl above, whose trait and error type are both unnameable outside this
/// crate, so `📦️mutate-semio-object`'s `identity-round-trip` scenario reaches the real committed
/// artifact (`../../🖼️assets/📦️crate/🗣️.dsl.semio`) through this instead.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_semio_object_dsl(text: &str) -> Result<SemioObjectSnapshot, String> {
    <SemioObjectSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| error.to_string())
}

/// 📝️ Renders a [`SemioObjectSnapshot`] back as `s.stdio.semio.object` DSL text — the inverse of
/// [`parse_semio_object_dsl`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn print_semio_object_dsl(snapshot: &SemioObjectSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use super::*;
use crate::standards::v1::subsets::object::schema::snapshot::*;
use crate::standards::v1::subsets::base::schema::geometry::SemioTransform;
use crate::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use crate::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use crate::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
use framework_schema::ArtifactSchema;




}
pub use snapshot_wire_codec::*;
