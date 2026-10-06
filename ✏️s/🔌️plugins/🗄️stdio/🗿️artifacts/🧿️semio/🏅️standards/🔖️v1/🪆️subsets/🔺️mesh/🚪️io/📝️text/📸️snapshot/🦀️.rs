//! 📝️ Text representation codec surface for `s.stdio.semio.mesh` (snapshot) — grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::mesh::schema::snapshot::*;
use crate::standards::v1::subsets::base::schema::geometry::native;
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint3, SemioRgba, SemioUv};
use crate::standards::v1::subsets::base::schema::triples::{split_top_level, strip_brackets};
use framework_schema::ArtifactSchema;

/// 🧪️ Real hex/bracket-encoded value primitives backing the hand-rolled `ArtifactDsl` below —
/// same style as this subset's own `🔺️diff`/`🧬️mutations` facets (`GifDiff`/`SvgDiff`/`DocxDiff`'s
/// established hand-rolled convention), duplicated here (not imported from `schema::diff`) to keep
/// `snapshot` — the base type `diff`/`mutations` both depend ON — free of a reverse dependency on
/// either sibling facet (same rationale `🌊️flow`'s own pilot documents).
///
/// 🧩️ The `#[derive(dsl::DslArtifact)]` path was tried first per this ticket's brief now that the
/// shared `⚙️engine/🧮️geometry` types derive `dsl::DslRecord`. It is still blocked here: this
/// subset's `SemioPrimitive`/`SemioMesh`/`SemioMaterial`/`SemioTexture` hold `Vec<SemioPoint3>`/
/// `Vec<SemioUv>`/`Vec<SemioRgba>`/`Vec<u8>` buffer fields nested two collections deep
/// (`meshes[].primitives[].positions[]`) — the derive macro's `#[dsl(table)]`/`Vec<Record>` support
/// covers one level of id-keyed collection, not a doubly-nested buffer-of-records-of-buffers shape,
/// and `SemioPrimitive.material_id: Option<String>` sits alongside those buffers in the same
/// record. Hand-rolled instead, same boundary this ticket's other semio pilots hit for their own
/// structurally-nested collections.
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
pub(crate) fn enc_bytes(b: &[u8]) -> String {
    hex_encode(b)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_bytes(s: &str) -> Result<Vec<u8>, String> {
    hex_decode(s)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_f32(s: &str) -> Result<f32, String> {
    native::parse32(s)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_f64(s: &str) -> Result<f64, String> {
    native::parse(s)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_u32(s: &str) -> Result<u32, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
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
pub(crate) fn encode_option<T>(opt: &Option<T>, enc: impl Fn(&T) -> String) -> String {
    match opt {
        None => "[0]".to_string(),
        Some(v) => format!("[1,{}]", enc(v)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn decode_option<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Option<T>, String> {
    let inner = strip_brackets(s)?;
    match split_top_level(inner, ',').as_slice() {
        ["0"] => Ok(None),
        [tag, value] if *tag == "1" => Ok(Some(dec(value)?)),
        other => Err(format!("option decode: bad shape {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_point3(p: &SemioPoint3) -> String {
    format!("[{},{},{}]", native::NativeF64(p.x), native::NativeF64(p.y), native::NativeF64(p.z))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_point3(s: &str) -> Result<SemioPoint3, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y, z] = parts.as_slice() else { return Err(format!("point3: expected 3 fields, got {}", parts.len())) };
    Ok(SemioPoint3 { x: parse_f64(x)?, y: parse_f64(y)?, z: parse_f64(z)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_uv(v: &SemioUv) -> String {
    format!("[{},{}]", native::NativeF64(v.u), native::NativeF64(v.v))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_uv(s: &str) -> Result<SemioUv, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [u, v] = parts.as_slice() else { return Err(format!("uv: expected 2 fields, got {}", parts.len())) };
    Ok(SemioUv { u: parse_f64(u)?, v: parse_f64(v)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_rgba(c: &SemioRgba) -> String {
    format!("[{},{},{},{}]", native::NativeF32(c.r), native::NativeF32(c.g), native::NativeF32(c.b), native::NativeF32(c.a))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_rgba(s: &str) -> Result<SemioRgba, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [r, g, b, a] = parts.as_slice() else { return Err(format!("rgba: expected 4 fields, got {}", parts.len())) };
    Ok(SemioRgba { r: parse_f32(r)?, g: parse_f32(g)?, b: parse_f32(b)?, a: parse_f32(a)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_topology(t: &SemioTopology) -> String {
    match t {
        SemioTopology::Points => "P".to_string(),
        SemioTopology::Lines => "L".to_string(),
        SemioTopology::LineStrip => "S".to_string(),
        SemioTopology::Triangles => "T".to_string(),
        SemioTopology::TriangleStrip => "X".to_string(),
        SemioTopology::TriangleFan => "F".to_string(),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_topology(s: &str) -> Result<SemioTopology, String> {
    match s {
        "P" => Ok(SemioTopology::Points),
        "L" => Ok(SemioTopology::Lines),
        "S" => Ok(SemioTopology::LineStrip),
        "T" => Ok(SemioTopology::Triangles),
        "X" => Ok(SemioTopology::TriangleStrip),
        "F" => Ok(SemioTopology::TriangleFan),
        other => Err(format!("topology: unknown tag {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_primitive(p: &SemioPrimitive) -> String {
    format!(
        "[{},{},{},{},{},{},{},{}]",
        enc_str(&p.id),
        enc_topology(&p.topology),
        enc_list(&p.positions, enc_point3),
        enc_list(&p.normals, enc_point3),
        enc_list(&p.uvs, enc_uv),
        enc_list(&p.colors, enc_rgba),
        enc_list(&p.indices, |v: &u32| v.to_string()),
        encode_option(&p.material_id, |v: &String| enc_str(v)),
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_primitive(s: &str) -> Result<SemioPrimitive, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, topology, positions, normals, uvs, colors, indices, material_id] = parts.as_slice() else {
        return Err(format!("primitive: expected 8 fields, got {}", parts.len()));
    };
    Ok(SemioPrimitive {
        id: dec_str(id)?,
        topology: dec_topology(topology)?,
        positions: dec_list(positions, dec_point3)?,
        normals: dec_list(normals, dec_point3)?,
        uvs: dec_list(uvs, dec_uv)?,
        colors: dec_list(colors, dec_rgba)?,
        indices: dec_list(indices, parse_u32)?,
        material_id: decode_option(material_id, dec_str)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_mesh(m: &SemioMesh) -> String {
    format!("[{},{}]", enc_str(&m.id), enc_list(&m.primitives, enc_primitive))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_mesh(s: &str) -> Result<SemioMesh, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, primitives] = parts.as_slice() else { return Err(format!("mesh: expected 2 fields, got {}", parts.len())) };
    Ok(SemioMesh { id: dec_str(id)?, primitives: dec_list(primitives, dec_primitive)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_material(m: &SemioMaterial) -> String {
    format!("[{},{},{},{},{},{},{},{},{}]", enc_str(&m.id), enc_rgba(&m.base_color), native::NativeF32(m.metallic), native::NativeF32(m.roughness), encode_option(&m.base_color_texture, |v: &String| enc_str(v)), encode_option(&m.metallic_roughness_texture, |v: &String| enc_str(v)), encode_option(&m.normal_texture, |v: &String| enc_str(v)), encode_option(&m.occlusion_texture, |v: &String| enc_str(v)), encode_option(&m.emissive_texture, |v: &String| enc_str(v)))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_material(s: &str) -> Result<SemioMaterial, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, base_color, metallic, roughness, base_color_texture, metallic_roughness_texture, normal_texture, occlusion_texture, emissive_texture] = parts.as_slice() else { return Err(format!("material: expected 9 fields, got {}", parts.len())) };
    Ok(SemioMaterial { id: dec_str(id)?, base_color: dec_rgba(base_color)?, metallic: parse_f32(metallic)?, roughness: parse_f32(roughness)?, base_color_texture: decode_option(base_color_texture, dec_str)?, metallic_roughness_texture: decode_option(metallic_roughness_texture, dec_str)?, normal_texture: decode_option(normal_texture, dec_str)?, occlusion_texture: decode_option(occlusion_texture, dec_str)?, emissive_texture: decode_option(emissive_texture, dec_str)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_texture(t: &SemioTexture) -> String {
    format!("[{},{},{}]", enc_str(&t.id), enc_str(&t.mime), enc_bytes(&t.bytes))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_texture(s: &str) -> Result<SemioTexture, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, mime, bytes] = parts.as_slice() else { return Err(format!("texture: expected 3 fields, got {}", parts.len())) };
    Ok(SemioTexture { id: dec_str(id)?, mime: dec_str(mime)?, bytes: dec_bytes(bytes)? })
}

/// 📄️ The real structured text body: four lines — `schema=<hex>`, `meshes=[<mesh>,...]`,
/// `materials=[<material>,...]`, `textures=[<texture>,...]` — matching the grammar's
/// `document = artifact-mark schema-line meshes-line materials-line textures-line`. Newlines are
/// pure lexer trivia in the shared dialect, so this is genuinely recognizable by `dsl::Recognizer`,
/// not merely readable.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_mesh_snapshot_body(s: &SemioMeshSnapshot) -> String {
    format!("schema={}\nmeshes={}\nmaterials={}\ntextures={}", enc_str(&s.schema), enc_list(&s.meshes, enc_mesh), enc_list(&s.materials, enc_material), enc_list(&s.textures, enc_texture),)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_mesh_snapshot_body(body: &str) -> Result<SemioMeshSnapshot, String> {
    let mut schema = None;
    let mut meshes = Vec::new();
    let mut materials = Vec::new();
    let mut textures = Vec::new();
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("schema=") {
            schema = Some(dec_str(rest)?);
        } else if let Some(rest) = line.strip_prefix("meshes=") {
            meshes = dec_list(rest, dec_mesh)?;
        } else if let Some(rest) = line.strip_prefix("materials=") {
            materials = dec_list(rest, dec_material)?;
        } else if let Some(rest) = line.strip_prefix("textures=") {
            textures = dec_list(rest, dec_texture)?;
        } else {
            return Err(format!("semio mesh snapshot: unknown line {line:?}"));
        }
    }
    let schema = schema.ok_or_else(|| "semio mesh snapshot: missing schema line".to_string())?;
    Ok(SemioMeshSnapshot { schema, meshes, materials, textures })
}

/// 🎁 Real structured text/binary codecs — replaces the old hex-dump-of-`serde_json` shortcut.
/// Wrapped in the repo-wide `store::semio_format` envelope, unchanged.
impl store::ArtifactDsl for SemioMeshSnapshot {
    const EXTENSION: &'static str = "semio";
    fn envelope_id() -> &'static str {
        STDIO_SEMIOMESH_DOCUMENT_SCHEMA
    }

    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_mesh_snapshot_body(body).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let body = print_mesh_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📤️ This subset's own `#[value(rename_all = "camelCase")]` structural JSON projection of
/// `s.stdio.semio.mesh` — the shape `🔺️mutate-semio-mesh` compares under `ordered-json-v1`, derived from the
/// snapshot type itself rather than hand-written a second time in the adapter, where it could drift
/// away from the type it claims to project. A mesh snapshot is dominated by bulk arrays — per-primitive `positions`/`normals`/`uvs`/
/// `indices` — where transcribing a fixture into a Rust literal is both the most laborious and the
/// least reviewable option available.
/// A thin `pack::to_json_string` wrapper (first-party, over `ToValue`/`DslValue`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_semio_mesh_snapshot_json(snapshot: &SemioMeshSnapshot) -> String {
    semio_framework_pack_json::to_json_string(snapshot)
}

/// 📥️ The `pack::from_json_str` inverse of [`encode_semio_mesh_snapshot_json`] — decodes the committed
/// `../🧬️mutations/<kind>/🧪️tests/<fixture>/📸️snapshot/{⬅️before,➡️after}/🔣️.json`
/// specification vectors into real [`SemioMeshSnapshot`] values, so `🔺️mutate-semio-mesh`'s adapter reads the
/// committed fixture instead of re-declaring it as a Rust literal beside it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_mesh_snapshot_json(text: &str) -> Result<SemioMeshSnapshot, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// 🦑 Dissolved out of the former `⚙️engine` — thin pass-throughs of this snapshot's own
/// `ArtifactDsl`/`ArtifactPack` impls above, kept as named convenience wrappers for callers that
/// want the mesh-subset-specific names.
/// 📝 Parse mesh subset DSL text into a `SemioMeshSnapshot`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_mesh_dsl(text: &str) -> Result<SemioMeshSnapshot, semio_framework_diagnostic::TextError> {
    <SemioMeshSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 📝 Render a `SemioMeshSnapshot` as mesh subset DSL text.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn print_mesh_dsl(snapshot: &SemioMeshSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use super::*;
use crate::standards::v1::subsets::mesh::schema::snapshot::*;
use crate::standards::v1::subsets::base::schema::geometry::native;
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint3, SemioRgba, SemioUv};
use crate::audio::io::text::diff::{strip_brackets};
use crate::audio::io::text::diff::{split_top_level};
use framework_schema::ArtifactSchema;




}
pub use snapshot_wire_codec::*;
