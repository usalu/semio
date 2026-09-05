//! 🧬️ SemioMeshSnapshot — meshes -> primitives{topology, positions/normals/uvs/colors, indices,
//! material} + materials (PBR base_color/metallic/roughness) + textures{mime, bytes}. Informed by
//! gltf 2.0's `GltfMesh`/`GltfPrimitive`/`GltfAccessor`/`GltfMaterial`, per the master plan's
//! "Subset snapshot cores" table. Owned types (w1b-type-ownership.md): `SemioMesh`,
//! `SemioPrimitive`, `SemioMaterial`, `SemioTexture` (`SemioPrimitive` was RESERVED at W1b —
//! this file is where it lands).

use crate::artifacts::semio::standards::v1::subsets::base::schema::geometry::{SemioPoint3, SemioRgba, SemioUv};
use crate::artifacts::semio::standards::v1::subsets::base::schema::triples::{split_top_level, strip_brackets};

//#region 🔖️Topology
/// 🔺️ Primitive draw mode — the gltf 2.0 `mode` enumeration, named (never a bare integer tag).
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum SemioTopology {
    Points,
    Lines,
    LineStrip,
    Triangles,
    TriangleStrip,
    TriangleFan,
}

impl Default for SemioTopology {
    fn default() -> Self {
        Self::Triangles
    }
}
//#endregion 🔖️Topology

//#region 🔖️Primitive
/// 🔷️ One drawable primitive inside a `SemioMesh` — id-keyed (the strong entity gltf's
/// `mesh.primitives` array lacks; every W2 subset id-keys its repeating structures per the
/// schema-design.md recipe). `positions`/`normals`/`uvs`/`colors`/`indices` are weak, parallel
/// buffer-shaped data — whole-value replaced in diffs, never sub-diffed per vertex.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemioPrimitive {
    pub id: String,
    #[value(default)]
    pub topology: SemioTopology,
    #[value(default)]
    pub positions: Vec<SemioPoint3>,
    #[value(default)]
    pub normals: Vec<SemioPoint3>,
    #[value(default)]
    pub uvs: Vec<SemioUv>,
    #[value(default)]
    pub colors: Vec<SemioRgba>,
    #[value(default)]
    pub indices: Vec<u32>,
    #[value(default)]
    pub material_id: Option<String>,
}
//#endregion 🔖️Primitive

//#region 🔖️Mesh
/// 🕸️ A mesh is an id-keyed collection of `SemioPrimitive`s (gltf's `mesh.primitives`).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemioMesh {
    pub id: String,
    #[value(default)]
    pub primitives: Vec<SemioPrimitive>,
}
//#endregion 🔖️Mesh

//#region 🔖️Material
/// 🎨️ PBR metallic-roughness material (gltf's `material.pbrMetallicRoughness`, the spec-mandated
/// field set per the master plan's row: "materials (PBR base_color/metallic/roughness)").
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemioMaterial {
    pub id: String,
    #[value(default)]
    pub base_color: SemioRgba,
    #[value(default)]
    pub metallic: f32,
    #[value(default)]
    pub roughness: f32,
}
//#endregion 🔖️Material

//#region 🔖️Texture
/// 🖼️ Raw texture payload (gltf's `image` + embedded `bufferView`/data-uri collapsed into one
/// typed-raw-retention entity — mime + bytes, per the master plan's row: "textures{mime, bytes}").
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemioTexture {
    pub id: String,
    #[value(default)]
    pub mime: String,
    #[value(default)]
    pub bytes: Vec<u8>,
}
//#endregion 🔖️Texture

use schema::ArtifactSchema;

//#region 🔖️Ids
pub const STDIO_SEMIOMESH_DOCUMENT_SCHEMA: &str = "stdio.semio.mesh";
//#endregion 🔖️Ids

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.mesh")]
pub struct SemioMeshSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub meshes: Vec<SemioMesh>,
    #[state(artifact)]
    #[value(default)]
    pub materials: Vec<SemioMaterial>,
    #[state(artifact)]
    #[value(default)]
    pub textures: Vec<SemioTexture>,
}

impl Default for SemioMeshSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_SEMIOMESH_DOCUMENT_SCHEMA.into(), meshes: Default::default(), materials: Default::default(), textures: Default::default() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️TextPrimitives
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
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_bytes(b: &[u8]) -> String {
    hex_encode(b)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_bytes(s: &str) -> Result<Vec<u8>, String> {
    hex_decode(s)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_f32(s: &str) -> Result<f32, String> {
    s.parse().map_err(|e: std::num::ParseFloatError| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_f64(s: &str) -> Result<f64, String> {
    s.parse().map_err(|e: std::num::ParseFloatError| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_u32(s: &str) -> Result<u32, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_list<T>(items: &[T], enc: impl Fn(&T) -> String) -> String {
    format!("[{}]", items.iter().map(|it| enc(it)).collect::<Vec<_>>().join(","))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_list<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Vec<T>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|entry| dec(entry)).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_option<T>(opt: &Option<T>, enc: impl Fn(&T) -> String) -> String {
    match opt {
        None => "[0]".to_string(),
        Some(v) => format!("[1,{}]", enc(v)),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_option<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Option<T>, String> {
    let inner = strip_brackets(s)?;
    match split_top_level(inner, ',').as_slice() {
        ["0"] => Ok(None),
        [tag, value] if *tag == "1" => Ok(Some(dec(value)?)),
        other => Err(format!("option decode: bad shape {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_point3(p: &SemioPoint3) -> String {
    format!("[{},{},{}]", p.x, p.y, p.z)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_point3(s: &str) -> Result<SemioPoint3, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y, z] = parts.as_slice() else { return Err(format!("point3: expected 3 fields, got {}", parts.len())) };
    Ok(SemioPoint3 { x: parse_f64(x)?, y: parse_f64(y)?, z: parse_f64(z)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_uv(v: &SemioUv) -> String {
    format!("[{},{}]", v.u, v.v)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_uv(s: &str) -> Result<SemioUv, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [u, v] = parts.as_slice() else { return Err(format!("uv: expected 2 fields, got {}", parts.len())) };
    Ok(SemioUv { u: parse_f64(u)?, v: parse_f64(v)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_rgba(c: &SemioRgba) -> String {
    format!("[{},{},{},{}]", c.r, c.g, c.b, c.a)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_rgba(s: &str) -> Result<SemioRgba, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [r, g, b, a] = parts.as_slice() else { return Err(format!("rgba: expected 4 fields, got {}", parts.len())) };
    Ok(SemioRgba { r: parse_f32(r)?, g: parse_f32(g)?, b: parse_f32(b)?, a: parse_f32(a)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_topology(t: &SemioTopology) -> String {
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
fn dec_topology(s: &str) -> Result<SemioTopology, String> {
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
fn enc_primitive(p: &SemioPrimitive) -> String {
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
fn dec_primitive(s: &str) -> Result<SemioPrimitive, String> {
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
fn enc_mesh(m: &SemioMesh) -> String {
    format!("[{},{}]", enc_str(&m.id), enc_list(&m.primitives, enc_primitive))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_mesh(s: &str) -> Result<SemioMesh, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, primitives] = parts.as_slice() else { return Err(format!("mesh: expected 2 fields, got {}", parts.len())) };
    Ok(SemioMesh { id: dec_str(id)?, primitives: dec_list(primitives, dec_primitive)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_material(m: &SemioMaterial) -> String {
    format!("[{},{},{},{}]", enc_str(&m.id), enc_rgba(&m.base_color), m.metallic, m.roughness)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_material(s: &str) -> Result<SemioMaterial, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, base_color, metallic, roughness] = parts.as_slice() else { return Err(format!("material: expected 4 fields, got {}", parts.len())) };
    Ok(SemioMaterial { id: dec_str(id)?, base_color: dec_rgba(base_color)?, metallic: parse_f32(metallic)?, roughness: parse_f32(roughness)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_texture(t: &SemioTexture) -> String {
    format!("[{},{},{}]", enc_str(&t.id), enc_str(&t.mime), enc_bytes(&t.bytes))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_texture(s: &str) -> Result<SemioTexture, String> {
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
fn print_mesh_snapshot_body(s: &SemioMeshSnapshot) -> String {
    format!("schema={}\nmeshes={}\nmaterials={}\ntextures={}", enc_str(&s.schema), enc_list(&s.meshes, enc_mesh), enc_list(&s.materials, enc_material), enc_list(&s.textures, enc_texture),)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_mesh_snapshot_body(body: &str) -> Result<SemioMeshSnapshot, String> {
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
//#endregion 🔖️TextPrimitives

//#region 🔖️BinaryPrimitives
/// 🧪️ Real LEB128-varint-length-prefixed binary primitives (`store::pack_rt::write_varint_u64` /
/// `store::ByteReader`, same helpers `🌊️flow`'s own upgraded `ArtifactPack` uses) backing the
/// real `ArtifactPack` below — replaces the old `serde_json::to_vec`-in-envelope shortcut.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_f32_le(reader: &mut store::ByteReader<'_>) -> Result<f32, String> {
    let bytes = reader.read_bytes(4).map_err(|e| e.to_string())?;
    let arr: [u8; 4] = bytes.try_into().map_err(|_| "f32 read: truncated".to_string())?;
    Ok(f32::from_le_bytes(arr))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_point3_list(out: &mut Vec<u8>, items: &[SemioPoint3]) {
    store::pack_rt::write_varint_u64(out, items.len() as u64);
    for p in items {
        out.extend_from_slice(&p.x.to_le_bytes());
        out.extend_from_slice(&p.y.to_le_bytes());
        out.extend_from_slice(&p.z.to_le_bytes());
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_point3_list(reader: &mut store::ByteReader<'_>) -> Result<Vec<SemioPoint3>, String> {
    let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut out = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let x = reader.read_f64_le().map_err(|e| e.to_string())?;
        let y = reader.read_f64_le().map_err(|e| e.to_string())?;
        let z = reader.read_f64_le().map_err(|e| e.to_string())?;
        out.push(SemioPoint3 { x, y, z });
    }
    Ok(out)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_uv_list(out: &mut Vec<u8>, items: &[SemioUv]) {
    store::pack_rt::write_varint_u64(out, items.len() as u64);
    for v in items {
        out.extend_from_slice(&v.u.to_le_bytes());
        out.extend_from_slice(&v.v.to_le_bytes());
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_uv_list(reader: &mut store::ByteReader<'_>) -> Result<Vec<SemioUv>, String> {
    let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut out = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let u = reader.read_f64_le().map_err(|e| e.to_string())?;
        let v = reader.read_f64_le().map_err(|e| e.to_string())?;
        out.push(SemioUv { u, v });
    }
    Ok(out)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_rgba(out: &mut Vec<u8>, c: &SemioRgba) {
    out.extend_from_slice(&c.r.to_le_bytes());
    out.extend_from_slice(&c.g.to_le_bytes());
    out.extend_from_slice(&c.b.to_le_bytes());
    out.extend_from_slice(&c.a.to_le_bytes());
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_rgba(reader: &mut store::ByteReader<'_>) -> Result<SemioRgba, String> {
    Ok(SemioRgba { r: read_f32_le(reader)?, g: read_f32_le(reader)?, b: read_f32_le(reader)?, a: read_f32_le(reader)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_rgba_list(out: &mut Vec<u8>, items: &[SemioRgba]) {
    store::pack_rt::write_varint_u64(out, items.len() as u64);
    for c in items {
        write_rgba(out, c);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_rgba_list(reader: &mut store::ByteReader<'_>) -> Result<Vec<SemioRgba>, String> {
    let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut out = Vec::with_capacity(n as usize);
    for _ in 0..n {
        out.push(read_rgba(reader)?);
    }
    Ok(out)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_mesh_snapshot_binary(s: &SemioMeshSnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut out = Vec::new();
    out.push(PACK_BINARY_FORMAT);
    write_str_lp(&mut out, &s.schema);
    store::pack_rt::write_varint_u64(&mut out, s.meshes.len() as u64);
    for m in &s.meshes {
        write_str_lp(&mut out, &m.id);
        store::pack_rt::write_varint_u64(&mut out, m.primitives.len() as u64);
        for p in &m.primitives {
            write_str_lp(&mut out, &p.id);
            out.push(match p.topology {
                SemioTopology::Points => 0u8,
                SemioTopology::Lines => 1,
                SemioTopology::LineStrip => 2,
                SemioTopology::Triangles => 3,
                SemioTopology::TriangleStrip => 4,
                SemioTopology::TriangleFan => 5,
            });
            write_point3_list(&mut out, &p.positions);
            write_point3_list(&mut out, &p.normals);
            write_uv_list(&mut out, &p.uvs);
            write_rgba_list(&mut out, &p.colors);
            store::pack_rt::write_varint_u64(&mut out, p.indices.len() as u64);
            for idx in &p.indices {
                out.extend_from_slice(&idx.to_le_bytes());
            }
            match &p.material_id {
                Some(v) => {
                    out.push(1);
                    write_str_lp(&mut out, v);
                }
                None => out.push(0),
            }
        }
    }
    store::pack_rt::write_varint_u64(&mut out, s.materials.len() as u64);
    for mat in &s.materials {
        write_str_lp(&mut out, &mat.id);
        write_rgba(&mut out, &mat.base_color);
        out.extend_from_slice(&mat.metallic.to_le_bytes());
        out.extend_from_slice(&mat.roughness.to_le_bytes());
    }
    store::pack_rt::write_varint_u64(&mut out, s.textures.len() as u64);
    for t in &s.textures {
        write_str_lp(&mut out, &t.id);
        write_str_lp(&mut out, &t.mime);
        write_bytes_lp(&mut out, &t.bytes);
    }
    out
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_mesh_snapshot_binary(bytes: &[u8]) -> Result<SemioMeshSnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    let schema = read_str_lp(&mut reader)?;
    let mesh_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut meshes = Vec::with_capacity(mesh_count as usize);
    for _ in 0..mesh_count {
        let id = read_str_lp(&mut reader)?;
        let primitive_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
        let mut primitives = Vec::with_capacity(primitive_count as usize);
        for _ in 0..primitive_count {
            let pid = read_str_lp(&mut reader)?;
            let topology_tag = reader.read_u8().map_err(|e| e.to_string())?;
            let topology = match topology_tag {
                0 => SemioTopology::Points,
                1 => SemioTopology::Lines,
                2 => SemioTopology::LineStrip,
                3 => SemioTopology::Triangles,
                4 => SemioTopology::TriangleStrip,
                5 => SemioTopology::TriangleFan,
                other => return Err(format!("unsupported topology tag {other}")),
            };
            let positions = read_point3_list(&mut reader)?;
            let normals = read_point3_list(&mut reader)?;
            let uvs = read_uv_list(&mut reader)?;
            let colors = read_rgba_list(&mut reader)?;
            let index_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
            let mut indices = Vec::with_capacity(index_count as usize);
            for _ in 0..index_count {
                indices.push(reader.read_u32_le().map_err(|e| e.to_string())?);
            }
            let material_id = match reader.read_u8().map_err(|e| e.to_string())? {
                0 => None,
                1 => Some(read_str_lp(&mut reader)?),
                other => return Err(format!("unsupported material_id tag {other}")),
            };
            primitives.push(SemioPrimitive { id: pid, topology, positions, normals, uvs, colors, indices, material_id });
        }
        meshes.push(SemioMesh { id, primitives });
    }
    let material_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut materials = Vec::with_capacity(material_count as usize);
    for _ in 0..material_count {
        let id = read_str_lp(&mut reader)?;
        let base_color = read_rgba(&mut reader)?;
        let metallic = read_f32_le(&mut reader)?;
        let roughness = read_f32_le(&mut reader)?;
        materials.push(SemioMaterial { id, base_color, metallic, roughness });
    }
    let texture_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut textures = Vec::with_capacity(texture_count as usize);
    for _ in 0..texture_count {
        let id = read_str_lp(&mut reader)?;
        let mime = read_str_lp(&mut reader)?;
        let bytes = read_bytes_lp(&mut reader)?;
        textures.push(SemioTexture { id, mime, bytes });
    }
    Ok(SemioMeshSnapshot { schema, meshes, materials, textures })
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactCodecs
/// 🎁 Real structured text/binary codecs — replaces the old hex-dump-of-`serde_json` shortcut.
/// Wrapped in the repo-wide `store::semio_format` envelope, unchanged.
impl store::ArtifactDsl for SemioMeshSnapshot {
    const EXTENSION: &'static str = "semio";
    fn envelope_id() -> &'static str {
        STDIO_SEMIOMESH_DOCUMENT_SCHEMA
    }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_mesh_snapshot_body(body).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let body = print_mesh_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for SemioMeshSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_mesh_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let _ = options;
        decode_mesh_snapshot_binary(&inner).map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🌉️ExternalCodecBridge
/// 📤️ This subset's own `#[value(rename_all = "camelCase")]` structural JSON projection of
/// `s.stdio.semio.mesh` — the shape `🔺️mutate-semio-mesh` compares under `ordered-json-v1`, derived from the
/// snapshot type itself rather than hand-written a second time in the adapter, where it could drift
/// away from the type it claims to project. A mesh snapshot is dominated by bulk arrays — per-primitive `positions`/`normals`/`uvs`/
/// `indices` — where transcribing a fixture into a Rust literal is both the most laborious and the
/// least reviewable option available.
/// A thin `pack::to_json_string` wrapper (first-party, over `ToValue`/`DslValue`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_semio_mesh_snapshot_json(snapshot: &SemioMeshSnapshot) -> String {
    pack::to_json_string(snapshot)
}

/// 📥️ The `pack::from_json_str` inverse of [`encode_semio_mesh_snapshot_json`] — decodes the committed
/// `../🧬️mutations/<kind>/🧪️tests/<fixture>/📸️snapshot/{⬅️before,➡️after}/🔣️.json`
/// specification vectors into real [`SemioMeshSnapshot`] values, so `🔺️mutate-semio-mesh`'s adapter reads the
/// committed fixture instead of re-declaring it as a Rust literal beside it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_mesh_snapshot_json(text: &str) -> Result<SemioMeshSnapshot, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}
//#endregion 🌉️ExternalCodecBridge

//#region 🔖️Demo
/// 🦑 Dissolved out of the former `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-
/// STATE-MACHINES) — pure snapshot constructor, no codec/IO concern.
///
/// 🌱 The demo `s.stdio.semio.mesh` document — single source of truth for
/// `📚️examples/🧊️cube/🖼️assets/🗣️.dsl.semio`/`🎒️.pack.semio` and conformance laws.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_mesh_snapshot() -> SemioMeshSnapshot {
    SemioMeshSnapshot {
        schema: STDIO_SEMIOMESH_DOCUMENT_SCHEMA.into(),
        meshes: vec![SemioMesh {
            id: "mesh-1".into(),
            primitives: vec![SemioPrimitive {
                id: "prim-1".into(),
                topology: SemioTopology::Triangles,
                positions: vec![SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 }, SemioPoint3 { x: 1.0, y: 0.0, z: 0.0 }, SemioPoint3 { x: 0.0, y: 1.0, z: 0.0 }],
                normals: vec![SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 }; 3],
                uvs: vec![SemioUv { u: 0.0, v: 0.0 }, SemioUv { u: 1.0, v: 0.0 }, SemioUv { u: 0.0, v: 1.0 }],
                colors: vec![SemioRgba { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }; 3],
                indices: vec![0, 1, 2],
                material_id: Some("mat-1".into()),
            }],
        }],
        materials: vec![SemioMaterial { id: "mat-1".into(), base_color: SemioRgba { r: 0.8, g: 0.2, b: 0.2, a: 1.0 }, metallic: 0.1, roughness: 0.6 }],
        textures: vec![SemioTexture { id: "tex-1".into(), mime: "image/png".into(), bytes: vec![0x89, 0x50, 0x4e, 0x47] }],
    }
}
//#endregion 🔖️Demo

//#region 🔖️Wire
/// 🦑 Dissolved out of the former `⚙️engine` — thin pass-throughs of this snapshot's own
/// `ArtifactDsl`/`ArtifactPack` impls above, kept as named convenience wrappers for callers that
/// want the mesh-subset-specific names.
/// 📝 Parse mesh subset DSL text into a `SemioMeshSnapshot`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_mesh_dsl(text: &str) -> Result<SemioMeshSnapshot, store::TextError> {
    <SemioMeshSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 📝 Render a `SemioMeshSnapshot` as mesh subset DSL text.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn print_mesh_dsl(snapshot: &SemioMeshSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}

/// 📦 Encode a `SemioMeshSnapshot` as a semio pack envelope.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_mesh_pack(snapshot: &SemioMeshSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(snapshot)
}

/// 📦 Decode a semio pack envelope into a `SemioMeshSnapshot`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_mesh_pack(bytes: &[u8]) -> Result<SemioMeshSnapshot, store::PackError> {
    <SemioMeshSnapshot as store::ArtifactPack>::decode_pack(bytes)
}
//#endregion 🔖️Wire

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// 🌱 Reuses `demo_mesh_snapshot()` (single source of truth, also feeds the shipped fixtures
    /// and `🎹️composer/🦀️.rs`'s conformance-law tests) rather than an independent copy.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn populated() -> SemioMeshSnapshot {
        demo_mesh_snapshot()
    }

    #[semio_framework_async_macros::async_test]
    async fn json_pack_round_trips() {
        let snap = populated();
        let bytes = <SemioMeshSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <SemioMeshSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);
    }

    #[semio_framework_async_macros::async_test]
    async fn dsl_text_round_trips() {
        let snap = populated();
        let text = <SemioMeshSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let back = <SemioMeshSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(snap, back);
    }

    #[semio_framework_async_macros::async_test]
    async fn default_snapshot_has_no_meshes_materials_or_textures() {
        let snap = SemioMeshSnapshot::default();
        assert!(snap.meshes.is_empty() && snap.materials.is_empty() && snap.textures.is_empty());
    }
}
//#endregion 🔖️Tests
