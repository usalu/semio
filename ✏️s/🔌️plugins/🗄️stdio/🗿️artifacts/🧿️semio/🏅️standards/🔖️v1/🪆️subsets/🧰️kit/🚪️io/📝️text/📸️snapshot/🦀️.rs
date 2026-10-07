//! 📝️ Text representation codec surface for `s.stdio.semio.kit` (snapshot) — grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::kit::schema::snapshot::*;
use crate::standards::v1::subsets::base::schema::geometry::SemioTransform;
use crate::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot;
use crate::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use crate::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
use framework_schema::ArtifactSchema;
#[cfg(test)]
use serde::{Deserialize, Serialize};
use crate::standards::v1::subsets::base::io::text::snapshot::{split_top_level, strip_brackets};

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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_child<S>(c: &store::ArtifactChild<S>) -> String {
    format!("[{},{}]", enc_str(&c.child_id), enc_ref(&c.target))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_child<S>(s: &str) -> Result<store::ArtifactChild<S>, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [child_id, target] = parts.as_slice() else { return Err(format!("child handle: expected 2 fields, got {}", parts.len())) };
    Ok(store::ArtifactChild::new(dec_str(child_id)?, dec_ref(target)?))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_child_list<S>(list: &[store::ArtifactChild<S>]) -> String {
    format!("[{}]", list.iter().map(enc_child).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_child_list<S>(s: &str) -> Result<Vec<store::ArtifactChild<S>>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_child).collect()
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

/// 📌️ `LinkPin`: `h` (Head) | `c,<hex id>` (Checkpoint) | `s,<hex hash>,<size>,<hex media_type>` (Snapshot).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_pin(p: &store::LinkPin) -> String {
    match p {
        store::LinkPin::Head => "[h]".to_string(),
        store::LinkPin::Checkpoint { id } => format!("[c,{}]", enc_str(id)),
        store::LinkPin::Snapshot { blob } => format!("[s,{},{},{}]", enc_str(&blob.hash), blob.size, enc_str(&blob.media_type)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_pin(s: &str) -> Result<store::LinkPin, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    match parts.as_slice() {
        [tag] if *tag == "h" => Ok(store::LinkPin::Head),
        [tag, id] if *tag == "c" => Ok(store::LinkPin::Checkpoint { id: dec_str(id)? }),
        [tag, hash, size, media_type] if *tag == "s" => Ok(store::LinkPin::Snapshot { blob: store::BlobRef { hash: dec_str(hash)?, size: size.trim().parse().map_err(|e: std::num::ParseIntError| e.to_string())?, media_type: dec_str(media_type)? } }),
        _ => Err(format!("link pin: unrecognized {s:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_link(l: &store::ArtifactLink) -> String {
    format!("[{},{},{}]", enc_ref(&l.target), enc_pin(&l.pin), enc_str(&l.role))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_link(s: &str) -> Result<store::ArtifactLink, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [target, pin, role] = parts.as_slice() else { return Err(format!("link: expected 3 fields, got {}", parts.len())) };
    Ok(store::ArtifactLink { target: dec_ref(target)?, pin: dec_pin(pin)?, role: dec_str(role)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_link_list(list: &[store::ArtifactLink]) -> String {
    format!("[{}]", list.iter().map(enc_link).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_link_list(s: &str) -> Result<Vec<store::ArtifactLink>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_link).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_transform(t: &SemioTransform) -> String {
    use crate::standards::v1::subsets::base::schema::geometry::native::NativeF64;
    format!("[{},{},{},{},{},{},{},{},{},{}]", NativeF64(t.translation.x), NativeF64(t.translation.y), NativeF64(t.translation.z), NativeF64(t.rotation.x), NativeF64(t.rotation.y), NativeF64(t.rotation.z), NativeF64(t.rotation.w), NativeF64(t.scale.x), NativeF64(t.scale.y), NativeF64(t.scale.z))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_transform(s: &str) -> Result<SemioTransform, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [tx, ty, tz, rx, ry, rz, rw, sx, sy, sz] = parts.as_slice() else {
        return Err(format!("transform: expected 10 fields, got {}", parts.len()));
    };
    let f = crate::standards::v1::subsets::base::schema::geometry::native::parse;
    use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint3, SemioQuaternion};
    Ok(SemioTransform { translation: SemioPoint3 { x: f(tx)?, y: f(ty)?, z: f(tz)? }, rotation: SemioQuaternion { x: f(rx)?, y: f(ry)?, z: f(rz)?, w: f(rw)? }, scale: SemioPoint3 { x: f(sx)?, y: f(sy)?, z: f(sz)? } })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_type(t: &SemioKitType) -> String {
    format!("[{},{},{}]", enc_str(&t.id), enc_str(&t.name), enc_str(&t.category))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_type(s: &str) -> Result<SemioKitType, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, name, category] = parts.as_slice() else { return Err(format!("type: expected 3 fields, got {}", parts.len())) };
    Ok(SemioKitType { id: dec_str(id)?, name: dec_str(name)?, category: dec_str(category)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_type_list(list: &[SemioKitType]) -> String {
    format!("[{}]", list.iter().map(enc_type).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_type_list(s: &str) -> Result<Vec<SemioKitType>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_type).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_piece(p: &SemioKitPiece) -> String {
    format!("[{},{},{}]", enc_str(&p.id), enc_str(&p.type_id), enc_transform(&p.transform))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_piece(s: &str) -> Result<SemioKitPiece, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, type_id, transform] = parts.as_slice() else { return Err(format!("piece: expected 3 fields, got {}", parts.len())) };
    Ok(SemioKitPiece { id: dec_str(id)?, type_id: dec_str(type_id)?, transform: dec_transform(transform)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_connection(c: &SemioKitConnection) -> String {
    format!("[{},{},{},{},{}]", enc_str(&c.id), enc_str(&c.connecting_piece_id), enc_str(&c.connecting_port), enc_str(&c.connected_piece_id), enc_str(&c.connected_port))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_connection(s: &str) -> Result<SemioKitConnection, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, cp_id, cp_port, cd_id, cd_port] = parts.as_slice() else { return Err(format!("connection: expected 5 fields, got {}", parts.len())) };
    Ok(SemioKitConnection { id: dec_str(id)?, connecting_piece_id: dec_str(cp_id)?, connecting_port: dec_str(cp_port)?, connected_piece_id: dec_str(cd_id)?, connected_port: dec_str(cd_port)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_design(d: &SemioKitDesign) -> String {
    let pieces = d.pieces.iter().map(enc_piece).collect::<Vec<_>>().join(",");
    let connections = d.connections.iter().map(enc_connection).collect::<Vec<_>>().join(",");
    format!("[{},{},[{}],[{}]]", enc_str(&d.id), enc_str(&d.name), pieces, connections)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_design(s: &str) -> Result<SemioKitDesign, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, name, pieces, connections] = parts.as_slice() else { return Err(format!("design: expected 4 fields, got {}", parts.len())) };
    let pieces = split_top_level(strip_brackets(pieces)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_piece).collect::<Result<Vec<_>, String>>()?;
    let connections = split_top_level(strip_brackets(connections)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_connection).collect::<Result<Vec<_>, String>>()?;
    Ok(SemioKitDesign { id: dec_str(id)?, name: dec_str(name)?, pieces, connections })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_design_list(list: &[SemioKitDesign]) -> String {
    format!("[{}]", list.iter().map(enc_design).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_design_list(s: &str) -> Result<Vec<SemioKitDesign>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_design).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_kit_snapshot_body(s: &SemioKitSnapshot) -> String {
    format!(
        "schema={}\ntypes={}\ndesigns={}\nobjects={}\nmodels={}\nproperties={}\nrepresentations={}",
        enc_str(&s.schema),
        enc_type_list(&s.types),
        enc_design_list(&s.designs),
        enc_child_list(&s.objects),
        enc_child_list(&s.models),
        enc_child_opt(&s.properties),
        enc_link_list(&s.representations),
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_kit_snapshot_body(body: &str) -> Result<SemioKitSnapshot, String> {
    let mut schema = None;
    let mut types = Vec::new();
    let mut designs = Vec::new();
    let mut objects = Vec::new();
    let mut models = Vec::new();
    let mut properties = None;
    let mut representations = Vec::new();
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("schema=") {
            schema = Some(dec_str(rest)?);
        } else if let Some(rest) = line.strip_prefix("types=") {
            types = dec_type_list(rest)?;
        } else if let Some(rest) = line.strip_prefix("designs=") {
            designs = dec_design_list(rest)?;
        } else if let Some(rest) = line.strip_prefix("objects=") {
            objects = dec_child_list(rest)?;
        } else if let Some(rest) = line.strip_prefix("models=") {
            models = dec_child_list(rest)?;
        } else if let Some(rest) = line.strip_prefix("properties=") {
            properties = dec_child_opt(rest)?;
        } else if let Some(rest) = line.strip_prefix("representations=") {
            representations = dec_link_list(rest)?;
        } else {
            return Err(format!("semio kit snapshot: unknown line {line:?}"));
        }
    }
    Ok(SemioKitSnapshot { schema: schema.ok_or_else(|| "semio kit snapshot: missing schema line".to_string())?, types, designs, objects, models, properties, representations })
}

impl store::ArtifactDsl for SemioKitSnapshot {
    const EXTENSION: &'static str = "semio";
    fn envelope_id() -> &'static str {
        STDIO_SEMIOKIT_DOCUMENT_SCHEMA
    }

    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_kit_snapshot_body(body).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let body = print_kit_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📥️ Decodes this subset's own `#[value(rename_all = "camelCase")]`-shaped JSON projection — the
/// exact shape the committed `../🧬️mutations/<kind>/🧪️tests/<fixture>/📸️snapshot/{⬅️before,➡️after}/
/// 🔣️.json` specification-vector fixtures carry — into a real `SemioKitSnapshot`, using
/// the snapshot's own hand-written `ToValue`/`FromValue` (§ValueCodec above). A thin
/// `pack::from_json_str` wrapper (first-party, over `ToValue`/`DslValue`) so external Rust callers
/// that cannot name this crate's private `store` extern-crate item (e.g. `mutate-semio-kit`'s test
/// adapter — see its own doc comment for why) can still decode a snapshot from committed fixture
/// text without hand-transcribing one field at a time, which is both laborious and a place for the
/// transcription to silently drift away from the fixture it claims to mirror.
pub fn decode_kit_snapshot_json(text: &str) -> Result<SemioKitSnapshot, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// 📤️ The `pack::to_json_string` inverse of `decode_kit_snapshot_json` — same rationale.
pub fn encode_kit_snapshot_json(snapshot: &SemioKitSnapshot) -> String {
    semio_framework_pack_json::to_json_string(snapshot)
}

/// 📝️ Parses `s.stdio.semio.kit` DSL text into a [`SemioKitSnapshot`] — a named pass-through of this snapshot's own
/// `store::ArtifactDsl` impl above, whose trait and error type are both unnameable outside this
/// crate, so `🧰️mutate-semio-kit`'s `identity-round-trip` scenario reaches the real committed
/// artifact (`../../🖼️assets/🪑️furniture/🗣️.dsl.semio`) through this instead.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_semio_kit_dsl(text: &str) -> Result<SemioKitSnapshot, String> {
    <SemioKitSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| error.to_string())
}

/// 📝️ Renders a [`SemioKitSnapshot`] back as `s.stdio.semio.kit` DSL text — the inverse of
/// [`parse_semio_kit_dsl`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn print_semio_kit_dsl(snapshot: &SemioKitSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use super::*;
use crate::standards::v1::subsets::kit::schema::snapshot::*;
use crate::standards::v1::subsets::base::schema::geometry::SemioTransform;
use crate::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot;
use crate::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use crate::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
use framework_schema::ArtifactSchema;
#[cfg(test)]
use serde::{Deserialize, Serialize};
use crate::standards::v1::subsets::audio::io::text::diff::{strip_brackets};
use crate::standards::v1::subsets::audio::io::text::diff::{split_top_level};




}
pub use snapshot_wire_codec::*;
