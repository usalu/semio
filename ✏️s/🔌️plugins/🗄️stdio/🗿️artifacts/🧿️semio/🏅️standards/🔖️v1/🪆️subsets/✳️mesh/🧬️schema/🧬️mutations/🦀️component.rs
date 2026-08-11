//! 🧬️ SemioMeshMutation — mesh mutation dispatch. Every variant's `diff()` is handcrafted
//! (calls exactly one `diff_*` constructor from `schema::diff`, never apply-and-capture) and
//! every variant's `inverse()` is handcrafted, id-aware (looks up the pre-mutation value in
//! `base` and builds the undoing mutation from it).

use crate::artifacts::semio::standards::v1::engine::geometry::{SemioPoint3, SemioRgba, SemioUv};
use crate::artifacts::semio::standards::v1::engine::triples::{split_top_level, strip_brackets};
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::{
    dec_list, dec_material, dec_mesh, dec_point3, dec_primitive, dec_rgba, dec_str, dec_texture, dec_topology, dec_uv, decode_option,
    diff_add_material, diff_add_mesh, diff_add_primitive, diff_add_texture, diff_remove_material, diff_remove_mesh, diff_remove_primitive,
    diff_remove_texture, diff_set_material_base_color, diff_set_material_pbr, diff_set_primitive_geometry, diff_set_primitive_material,
    diff_set_primitive_topology, diff_set_snapshot, diff_set_texture_bytes, enc_list, enc_material, enc_mesh, enc_point3, enc_primitive,
    enc_rgba, enc_str, enc_texture, enc_topology, enc_uv, encode_option, hex_decode, hex_encode, SemioMeshDiff,
};
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::{SemioMaterial, SemioMesh, SemioMeshSnapshot, SemioPrimitive, SemioTexture, SemioTopology};
use protocol::{Mutation, OpText};
#[cfg(test)]
use protocol::OpBinary;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `s.stdio.semio.mesh`. Beyond the baseline
/// `{NoMutation, SetSnapshot}`, addresses meshes/primitives/materials/textures by id.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum SemioMeshMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: SemioMeshSnapshot,
    },
    /// ➕️ Inserts a mesh (fails-safe no-op semantics live in the diff/apply layer; a duplicate id
    /// is a caller error, same convention as every other named-collection mutation in the repo).
    AddMesh {
        mesh: SemioMesh,
    },
    /// ➖️ Removes the mesh with id `id`.
    RemoveMesh {
        id: String,
    },
    /// ➕️ Inserts `primitive` into mesh `mesh_id`.
    AddPrimitive {
        mesh_id: String,
        primitive: SemioPrimitive,
    },
    /// ➖️ Removes the primitive `primitive_id` from mesh `mesh_id`.
    RemovePrimitive {
        mesh_id: String,
        primitive_id: String,
    },
    /// 🔺️ Sets a primitive's draw mode.
    SetPrimitiveTopology {
        mesh_id: String,
        primitive_id: String,
        topology: SemioTopology,
    },
    /// 📐️ Replaces a primitive's full vertex-buffer set (positions/normals/uvs/colors/indices) —
    /// whole-value replaced, per the recipe (never sub-diffed per vertex).
    SetPrimitiveGeometry {
        mesh_id: String,
        primitive_id: String,
        positions: Vec<SemioPoint3>,
        normals: Vec<SemioPoint3>,
        uvs: Vec<SemioUv>,
        colors: Vec<SemioRgba>,
        indices: Vec<u32>,
    },
    /// 🔗 Sets (or, if `None`, clears) a primitive's `material_id` reference.
    SetPrimitiveMaterial {
        mesh_id: String,
        primitive_id: String,
        material_id: Option<String>,
    },
    /// ➕️ Inserts a material.
    AddMaterial {
        material: SemioMaterial,
    },
    /// ➖️ Removes the material with id `id`.
    RemoveMaterial {
        id: String,
    },
    /// 🎨️ Sets a material's PBR base color.
    SetMaterialBaseColor {
        id: String,
        base_color: SemioRgba,
    },
    /// 🎚️ Sets a material's metallic/roughness PBR factors.
    SetMaterialPbr {
        id: String,
        metallic: f32,
        roughness: f32,
    },
    /// ➕️ Inserts a texture.
    AddTexture {
        texture: SemioTexture,
    },
    /// ➖️ Removes the texture with id `id`.
    RemoveTexture {
        id: String,
    },
    /// ✍️ Replaces a texture's mime type and raw bytes.
    SetTextureBytes {
        id: String,
        mime: String,
        bytes: Vec<u8>,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`: `let d = mutation.diff(&*snapshot); *snapshot =
/// d.apply(snapshot); d` — the diff is the single semantics source, never a separate imperative
/// apply path (apply-and-capture is banned).
pub fn apply_semio_mesh_mutation(snapshot: &mut SemioMeshSnapshot, mutation: &SemioMeshMutation) -> SemioMeshDiff {
    let diff = Mutation::diff(mutation, snapshot);
    *snapshot = protocol::MutationDiff::apply(&diff, snapshot);
    diff
}
//#endregion 🔖️Apply

//#region 🔖️Helpers
fn mesh_at<'a>(base: &'a SemioMeshSnapshot, mesh_id: &str) -> Option<&'a SemioMesh> {
    base.meshes.iter().find(|m| m.id == mesh_id)
}
fn primitive_at<'a>(base: &'a SemioMeshSnapshot, mesh_id: &str, primitive_id: &str) -> Option<&'a SemioPrimitive> {
    mesh_at(base, mesh_id)?.primitives.iter().find(|p| p.id == primitive_id)
}
fn material_at<'a>(base: &'a SemioMeshSnapshot, id: &str) -> Option<&'a SemioMaterial> {
    base.materials.iter().find(|m| m.id == id)
}
fn texture_at<'a>(base: &'a SemioMeshSnapshot, id: &str) -> Option<&'a SemioTexture> {
    base.textures.iter().find(|t| t.id == id)
}
//#endregion 🔖️Helpers

//#region 🔖️MutationTrait
impl Mutation<SemioMeshSnapshot> for SemioMeshMutation {
    type Diff = SemioMeshDiff;

    fn diff(&self, base: &SemioMeshSnapshot) -> Self::Diff {
        match self {
            SemioMeshMutation::NoMutation => SemioMeshDiff::default(),
            SemioMeshMutation::SetSnapshot { snapshot } => diff_set_snapshot(base, snapshot),
            SemioMeshMutation::AddMesh { mesh } => diff_add_mesh(base, mesh.clone()),
            SemioMeshMutation::RemoveMesh { id } => diff_remove_mesh(id),
            SemioMeshMutation::AddPrimitive { mesh_id, primitive } => diff_add_primitive(base, mesh_id, primitive.clone()),
            SemioMeshMutation::RemovePrimitive { mesh_id, primitive_id } => diff_remove_primitive(mesh_id, primitive_id),
            SemioMeshMutation::SetPrimitiveTopology { mesh_id, primitive_id, topology } => diff_set_primitive_topology(mesh_id, primitive_id, *topology),
            SemioMeshMutation::SetPrimitiveGeometry { mesh_id, primitive_id, positions, normals, uvs, colors, indices } => {
                diff_set_primitive_geometry(mesh_id, primitive_id, positions.clone(), normals.clone(), uvs.clone(), colors.clone(), indices.clone())
            }
            SemioMeshMutation::SetPrimitiveMaterial { mesh_id, primitive_id, material_id } => diff_set_primitive_material(mesh_id, primitive_id, material_id.clone()),
            SemioMeshMutation::AddMaterial { material } => diff_add_material(base, material.clone()),
            SemioMeshMutation::RemoveMaterial { id } => diff_remove_material(id),
            SemioMeshMutation::SetMaterialBaseColor { id, base_color } => diff_set_material_base_color(id, *base_color),
            SemioMeshMutation::SetMaterialPbr { id, metallic, roughness } => diff_set_material_pbr(id, *metallic, *roughness),
            SemioMeshMutation::AddTexture { texture } => diff_add_texture(base, texture.clone()),
            SemioMeshMutation::RemoveTexture { id } => diff_remove_texture(id),
            SemioMeshMutation::SetTextureBytes { id, mime, bytes } => diff_set_texture_bytes(id, mime.clone(), bytes.clone()),
        }
    }

    fn inverse(&self, base: &SemioMeshSnapshot) -> Vec<Self> {
        match self {
            SemioMeshMutation::NoMutation => vec![SemioMeshMutation::NoMutation],
            SemioMeshMutation::SetSnapshot { .. } => vec![SemioMeshMutation::SetSnapshot { snapshot: base.clone() }],
            SemioMeshMutation::AddMesh { mesh } => vec![SemioMeshMutation::RemoveMesh { id: mesh.id.clone() }],
            // ↩️ `AddMesh` always APPENDS (see `diff_add_mesh`), so naively reinverting a `RemoveMesh`
            // to a single `AddMesh` would restore the mesh's VALUE but lose its ORIGINAL POSITION
            // whenever other meshes originally followed it — restore exact position by first
            // removing every mesh that originally followed `id`, then re-adding `id` and each of
            // them back in original order (every re-add is an append, landing them exactly where
            // they started). Same shape `object`'s `RemoveMapEntry` inverse documents.
            SemioMeshMutation::RemoveMesh { id } => match base.meshes.iter().position(|m| &m.id == id) {
                Some(pos) => {
                    let tail: Vec<SemioMesh> = base.meshes[pos + 1..].to_vec();
                    let mut steps: Vec<SemioMeshMutation> = tail.iter().rev().map(|m| SemioMeshMutation::RemoveMesh { id: m.id.clone() }).collect();
                    steps.push(SemioMeshMutation::AddMesh { mesh: base.meshes[pos].clone() });
                    steps.extend(tail.into_iter().map(|m| SemioMeshMutation::AddMesh { mesh: m }));
                    steps
                }
                None => vec![SemioMeshMutation::NoMutation],
            },
            SemioMeshMutation::AddPrimitive { mesh_id, primitive } => vec![SemioMeshMutation::RemovePrimitive { mesh_id: mesh_id.clone(), primitive_id: primitive.id.clone() }],
            // ↩️ Same position-preserving technique as `RemoveMesh` above, scoped to `mesh_id`'s
            // own `primitives` collection.
            SemioMeshMutation::RemovePrimitive { mesh_id, primitive_id } => match mesh_at(base, mesh_id).and_then(|m| m.primitives.iter().position(|p| &p.id == primitive_id).map(|pos| (m, pos))) {
                Some((mesh, pos)) => {
                    let tail: Vec<SemioPrimitive> = mesh.primitives[pos + 1..].to_vec();
                    let mut steps: Vec<SemioMeshMutation> = tail.iter().rev().map(|p| SemioMeshMutation::RemovePrimitive { mesh_id: mesh_id.clone(), primitive_id: p.id.clone() }).collect();
                    steps.push(SemioMeshMutation::AddPrimitive { mesh_id: mesh_id.clone(), primitive: mesh.primitives[pos].clone() });
                    steps.extend(tail.into_iter().map(|p| SemioMeshMutation::AddPrimitive { mesh_id: mesh_id.clone(), primitive: p }));
                    steps
                }
                None => vec![SemioMeshMutation::NoMutation],
            },
            SemioMeshMutation::SetPrimitiveTopology { mesh_id, primitive_id, .. } => match primitive_at(base, mesh_id, primitive_id) {
                Some(primitive) => vec![SemioMeshMutation::SetPrimitiveTopology { mesh_id: mesh_id.clone(), primitive_id: primitive_id.clone(), topology: primitive.topology }],
                None => vec![SemioMeshMutation::NoMutation],
            },
            SemioMeshMutation::SetPrimitiveGeometry { mesh_id, primitive_id, .. } => match primitive_at(base, mesh_id, primitive_id) {
                Some(primitive) => vec![SemioMeshMutation::SetPrimitiveGeometry {
                    mesh_id: mesh_id.clone(),
                    primitive_id: primitive_id.clone(),
                    positions: primitive.positions.clone(),
                    normals: primitive.normals.clone(),
                    uvs: primitive.uvs.clone(),
                    colors: primitive.colors.clone(),
                    indices: primitive.indices.clone(),
                }],
                None => vec![SemioMeshMutation::NoMutation],
            },
            SemioMeshMutation::SetPrimitiveMaterial { mesh_id, primitive_id, .. } => match primitive_at(base, mesh_id, primitive_id) {
                Some(primitive) => vec![SemioMeshMutation::SetPrimitiveMaterial { mesh_id: mesh_id.clone(), primitive_id: primitive_id.clone(), material_id: primitive.material_id.clone() }],
                None => vec![SemioMeshMutation::NoMutation],
            },
            SemioMeshMutation::AddMaterial { material } => vec![SemioMeshMutation::RemoveMaterial { id: material.id.clone() }],
            // ↩️ Same position-preserving technique as `RemoveMesh` above.
            SemioMeshMutation::RemoveMaterial { id } => match base.materials.iter().position(|m| &m.id == id) {
                Some(pos) => {
                    let tail: Vec<SemioMaterial> = base.materials[pos + 1..].to_vec();
                    let mut steps: Vec<SemioMeshMutation> = tail.iter().rev().map(|m| SemioMeshMutation::RemoveMaterial { id: m.id.clone() }).collect();
                    steps.push(SemioMeshMutation::AddMaterial { material: base.materials[pos].clone() });
                    steps.extend(tail.into_iter().map(|m| SemioMeshMutation::AddMaterial { material: m }));
                    steps
                }
                None => vec![SemioMeshMutation::NoMutation],
            },
            SemioMeshMutation::SetMaterialBaseColor { id, .. } => match material_at(base, id) {
                Some(material) => vec![SemioMeshMutation::SetMaterialBaseColor { id: id.clone(), base_color: material.base_color }],
                None => vec![SemioMeshMutation::NoMutation],
            },
            SemioMeshMutation::SetMaterialPbr { id, .. } => match material_at(base, id) {
                Some(material) => vec![SemioMeshMutation::SetMaterialPbr { id: id.clone(), metallic: material.metallic, roughness: material.roughness }],
                None => vec![SemioMeshMutation::NoMutation],
            },
            SemioMeshMutation::AddTexture { texture } => vec![SemioMeshMutation::RemoveTexture { id: texture.id.clone() }],
            // ↩️ Same position-preserving technique as `RemoveMesh` above.
            SemioMeshMutation::RemoveTexture { id } => match base.textures.iter().position(|t| &t.id == id) {
                Some(pos) => {
                    let tail: Vec<SemioTexture> = base.textures[pos + 1..].to_vec();
                    let mut steps: Vec<SemioMeshMutation> = tail.iter().rev().map(|t| SemioMeshMutation::RemoveTexture { id: t.id.clone() }).collect();
                    steps.push(SemioMeshMutation::AddTexture { texture: base.textures[pos].clone() });
                    steps.extend(tail.into_iter().map(|t| SemioMeshMutation::AddTexture { texture: t }));
                    steps
                }
                None => vec![SemioMeshMutation::NoMutation],
            },
            SemioMeshMutation::SetTextureBytes { id, .. } => match texture_at(base, id) {
                Some(texture) => vec![SemioMeshMutation::SetTextureBytes { id: id.clone(), mime: texture.mime.clone(), bytes: texture.bytes.clone() }],
                None => vec![SemioMeshMutation::NoMutation],
            },
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🧪️ Hand-rolled `OpText`/`OpBinary` for `SemioMeshMutation` (per the ticket's dsl-derive-gaps
/// note — collection-diff generics and this artifact's own snapshot tree have no `DslField`
/// bridge; see `f6-final-summary.md` §4.4). Reuses `schema::diff`'s `pub(crate)` grammar
/// primitives/value codecs rather than duplicating them a second time in this file. Grammar:
/// `keyword arg=value ...` (space-separated), same shape docx's own hand-rolled convention uses.
fn enc_mesh_snapshot(s: &SemioMeshSnapshot) -> String {
    format!("[{},{},{},{}]", enc_str(&s.schema), enc_list(&s.meshes, enc_mesh), enc_list(&s.materials, enc_material), enc_list(&s.textures, enc_texture))
}
fn dec_mesh_snapshot(s: &str) -> Result<SemioMeshSnapshot, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [schema, meshes, materials, textures] = parts.as_slice() else { return Err(format!("snapshot: expected 4 fields, got {}", parts.len())) };
    Ok(SemioMeshSnapshot {
        schema: dec_str(schema)?,
        meshes: dec_list(meshes, dec_mesh)?,
        materials: dec_list(materials, dec_material)?,
        textures: dec_list(textures, dec_texture)?,
    })
}

fn enc_positions(v: &[SemioPoint3]) -> String { enc_list(v, enc_point3) }
fn dec_positions(s: &str) -> Result<Vec<SemioPoint3>, String> { dec_list(s, dec_point3) }
fn enc_uvs(v: &[SemioUv]) -> String { enc_list(v, enc_uv) }
fn dec_uvs(s: &str) -> Result<Vec<SemioUv>, String> { dec_list(s, dec_uv) }
fn enc_colors(v: &[SemioRgba]) -> String { enc_list(v, enc_rgba) }
fn dec_colors(s: &str) -> Result<Vec<SemioRgba>, String> { dec_list(s, dec_rgba) }
fn enc_indices(v: &[u32]) -> String { enc_list(v, |x: &u32| x.to_string()) }
fn dec_indices(s: &str) -> Result<Vec<u32>, String> { dec_list(s, |t| t.parse::<u32>().map_err(|e: std::num::ParseIntError| e.to_string())) }

fn print_semio_mesh_mutation(m: &SemioMeshMutation) -> String {
    match m {
        SemioMeshMutation::NoMutation => "no-mutation".to_string(),
        SemioMeshMutation::SetSnapshot { snapshot } => format!("set-snapshot snapshot={}", enc_mesh_snapshot(snapshot)),
        SemioMeshMutation::AddMesh { mesh } => format!("add-mesh mesh={}", enc_mesh(mesh)),
        SemioMeshMutation::RemoveMesh { id } => format!("remove-mesh id={}", enc_str(id)),
        SemioMeshMutation::AddPrimitive { mesh_id, primitive } => format!("add-primitive mesh-id={} primitive={}", enc_str(mesh_id), enc_primitive(primitive)),
        SemioMeshMutation::RemovePrimitive { mesh_id, primitive_id } => format!("remove-primitive mesh-id={} primitive-id={}", enc_str(mesh_id), enc_str(primitive_id)),
        SemioMeshMutation::SetPrimitiveTopology { mesh_id, primitive_id, topology } => {
            format!("set-primitive-topology mesh-id={} primitive-id={} topology={}", enc_str(mesh_id), enc_str(primitive_id), enc_topology(topology))
        }
        SemioMeshMutation::SetPrimitiveGeometry { mesh_id, primitive_id, positions, normals, uvs, colors, indices } => format!(
            "set-primitive-geometry mesh-id={} primitive-id={} positions={} normals={} uvs={} colors={} indices={}",
            enc_str(mesh_id), enc_str(primitive_id), enc_positions(positions), enc_positions(normals), enc_uvs(uvs), enc_colors(colors), enc_indices(indices)
        ),
        SemioMeshMutation::SetPrimitiveMaterial { mesh_id, primitive_id, material_id } => {
            format!("set-primitive-material mesh-id={} primitive-id={} material-id={}", enc_str(mesh_id), enc_str(primitive_id), encode_option(material_id, |v: &String| enc_str(v)))
        }
        SemioMeshMutation::AddMaterial { material } => format!("add-material material={}", enc_material(material)),
        SemioMeshMutation::RemoveMaterial { id } => format!("remove-material id={}", enc_str(id)),
        SemioMeshMutation::SetMaterialBaseColor { id, base_color } => format!("set-material-base-color id={} base-color={}", enc_str(id), enc_rgba(base_color)),
        SemioMeshMutation::SetMaterialPbr { id, metallic, roughness } => format!("set-material-pbr id={} metallic={} roughness={}", enc_str(id), metallic, roughness),
        SemioMeshMutation::AddTexture { texture } => format!("add-texture texture={}", enc_texture(texture)),
        SemioMeshMutation::RemoveTexture { id } => format!("remove-texture id={}", enc_str(id)),
        SemioMeshMutation::SetTextureBytes { id, mime, bytes } => format!("set-texture-bytes id={} mime={} bytes={}", enc_str(id), enc_str(mime), hex_encode(bytes)),
    }
}
fn parse_semio_mesh_mutation(line: &str) -> Result<SemioMeshMutation, String> {
    if line == "no-mutation" {
        return Ok(SemioMeshMutation::NoMutation);
    }
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|tok| tok.split_once('=').ok_or_else(|| format!("semio mesh mutation: bad arg token {tok:?}")))
        .collect::<Result<Vec<_>, String>>()?
        .into_iter()
        .collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("semio mesh mutation: missing arg '{k}' for '{keyword}'"));
    match keyword {
        "set-snapshot" => Ok(SemioMeshMutation::SetSnapshot { snapshot: dec_mesh_snapshot(arg("snapshot")?)? }),
        "add-mesh" => Ok(SemioMeshMutation::AddMesh { mesh: dec_mesh(arg("mesh")?)? }),
        "remove-mesh" => Ok(SemioMeshMutation::RemoveMesh { id: dec_str(arg("id")?)? }),
        "add-primitive" => Ok(SemioMeshMutation::AddPrimitive { mesh_id: dec_str(arg("mesh-id")?)?, primitive: dec_primitive(arg("primitive")?)? }),
        "remove-primitive" => Ok(SemioMeshMutation::RemovePrimitive { mesh_id: dec_str(arg("mesh-id")?)?, primitive_id: dec_str(arg("primitive-id")?)? }),
        "set-primitive-topology" => Ok(SemioMeshMutation::SetPrimitiveTopology {
            mesh_id: dec_str(arg("mesh-id")?)?,
            primitive_id: dec_str(arg("primitive-id")?)?,
            topology: dec_topology(arg("topology")?)?,
        }),
        "set-primitive-geometry" => Ok(SemioMeshMutation::SetPrimitiveGeometry {
            mesh_id: dec_str(arg("mesh-id")?)?,
            primitive_id: dec_str(arg("primitive-id")?)?,
            positions: dec_positions(arg("positions")?)?,
            normals: dec_positions(arg("normals")?)?,
            uvs: dec_uvs(arg("uvs")?)?,
            colors: dec_colors(arg("colors")?)?,
            indices: dec_indices(arg("indices")?)?,
        }),
        "set-primitive-material" => Ok(SemioMeshMutation::SetPrimitiveMaterial {
            mesh_id: dec_str(arg("mesh-id")?)?,
            primitive_id: dec_str(arg("primitive-id")?)?,
            material_id: decode_option(arg("material-id")?, dec_str)?,
        }),
        "add-material" => Ok(SemioMeshMutation::AddMaterial { material: dec_material(arg("material")?)? }),
        "remove-material" => Ok(SemioMeshMutation::RemoveMaterial { id: dec_str(arg("id")?)? }),
        "set-material-base-color" => Ok(SemioMeshMutation::SetMaterialBaseColor { id: dec_str(arg("id")?)?, base_color: dec_rgba(arg("base-color")?)? }),
        "set-material-pbr" => Ok(SemioMeshMutation::SetMaterialPbr {
            id: dec_str(arg("id")?)?,
            metallic: arg("metallic")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())?,
            roughness: arg("roughness")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())?,
        }),
        "add-texture" => Ok(SemioMeshMutation::AddTexture { texture: dec_texture(arg("texture")?)? }),
        "remove-texture" => Ok(SemioMeshMutation::RemoveTexture { id: dec_str(arg("id")?)? }),
        "set-texture-bytes" => Ok(SemioMeshMutation::SetTextureBytes { id: dec_str(arg("id")?)?, mime: dec_str(arg("mime")?)?, bytes: hex_decode(arg("bytes")?)? }),
        other => Err(format!("semio mesh mutation: unknown keyword {other:?}")),
    }
}

impl OpText for SemioMeshMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_semio_mesh_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    fn print_op(&self) -> String {
        print_semio_mesh_mutation(self)
    }
}

impl protocol::OpBinary for SemioMeshMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(self.print_op().into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let line = std::str::from_utf8(bytes).map_err(|e| protocol::ProtocolError::Malformed { what: "op utf8", offset: 0, detail: e.to_string() })?;
        Self::parse_op(line).map_err(|e| protocol::ProtocolError::Malformed { what: "op text", offset: 0, detail: e.to_string() })
    }
}
//#endregion OpCodecs

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::{SemioMaterialsDiff, SemioMeshesDiff, SemioPrimitivesDiff, SemioTexturesDiff};
    use protocol::command::DiffAlgebra;
    use protocol::MutationDiff;

    fn fixture() -> SemioMeshSnapshot {
        SemioMeshSnapshot {
            meshes: vec![SemioMesh {
                id: "mesh-a".into(),
                primitives: vec![SemioPrimitive {
                    id: "prim-a".into(),
                    topology: SemioTopology::Triangles,
                    positions: vec![SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 }, SemioPoint3 { x: 1.0, y: 0.0, z: 0.0 }, SemioPoint3 { x: 0.0, y: 1.0, z: 0.0 }],
                    normals: vec![SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 }; 3],
                    uvs: vec![SemioUv { u: 0.0, v: 0.0 }; 3],
                    colors: vec![],
                    indices: vec![0, 1, 2],
                    material_id: Some("mat-a".into()),
                }],
            }],
            materials: vec![SemioMaterial { id: "mat-a".into(), base_color: SemioRgba { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }, metallic: 0.0, roughness: 0.5 }],
            textures: vec![],
            ..Default::default()
        }
    }

    #[test]
    fn add_then_remove_mesh_apply_and_inverse() {
        let base = fixture();
        let add = SemioMeshMutation::AddMesh { mesh: SemioMesh { id: "mesh-b".into(), primitives: vec![] } };
        let mut after = base.clone();
        apply_semio_mesh_mutation(&mut after, &add);
        assert_eq!(after.meshes.len(), 2);
        for inv in Mutation::inverse(&add, &base) {
            apply_semio_mesh_mutation(&mut after, &inv);
        }
        assert_eq!(after, base);
    }

    #[test]
    fn remove_mesh_inverse_restores_removed_mesh() {
        let base = fixture();
        let remove = SemioMeshMutation::RemoveMesh { id: "mesh-a".into() };
        let mut after = base.clone();
        apply_semio_mesh_mutation(&mut after, &remove);
        assert!(after.meshes.is_empty());
        for inv in Mutation::inverse(&remove, &base) {
            apply_semio_mesh_mutation(&mut after, &inv);
        }
        assert_eq!(after, base);
    }

    #[test]
    fn primitive_mutations_apply_and_inverse() {
        let base = fixture();
        let add = SemioMeshMutation::AddPrimitive { mesh_id: "mesh-a".into(), primitive: SemioPrimitive { id: "prim-b".into(), ..Default::default() } };
        let mut after = base.clone();
        apply_semio_mesh_mutation(&mut after, &add);
        assert_eq!(after.meshes[0].primitives.len(), 2);
        for inv in Mutation::inverse(&add, &base) {
            apply_semio_mesh_mutation(&mut after, &inv);
        }
        assert_eq!(after, base);

        let topo = SemioMeshMutation::SetPrimitiveTopology { mesh_id: "mesh-a".into(), primitive_id: "prim-a".into(), topology: SemioTopology::Lines };
        let mut after2 = base.clone();
        apply_semio_mesh_mutation(&mut after2, &topo);
        assert_eq!(after2.meshes[0].primitives[0].topology, SemioTopology::Lines);
        for inv in Mutation::inverse(&topo, &base) {
            apply_semio_mesh_mutation(&mut after2, &inv);
        }
        assert_eq!(after2, base);

        let mat = SemioMeshMutation::SetPrimitiveMaterial { mesh_id: "mesh-a".into(), primitive_id: "prim-a".into(), material_id: None };
        let mut after3 = base.clone();
        apply_semio_mesh_mutation(&mut after3, &mat);
        assert_eq!(after3.meshes[0].primitives[0].material_id, None);
        for inv in Mutation::inverse(&mat, &base) {
            apply_semio_mesh_mutation(&mut after3, &inv);
        }
        assert_eq!(after3, base);
    }

    #[test]
    fn material_and_texture_mutations_apply_and_inverse() {
        let base = fixture();
        let add_mat = SemioMeshMutation::AddMaterial { material: SemioMaterial { id: "mat-b".into(), base_color: SemioRgba { r: 0.0, g: 0.0, b: 1.0, a: 1.0 }, metallic: 1.0, roughness: 0.0 } };
        let mut after = base.clone();
        apply_semio_mesh_mutation(&mut after, &add_mat);
        assert_eq!(after.materials.len(), 2);
        for inv in Mutation::inverse(&add_mat, &base) {
            apply_semio_mesh_mutation(&mut after, &inv);
        }
        assert_eq!(after, base);

        let add_tex = SemioMeshMutation::AddTexture { texture: SemioTexture { id: "tex-a".into(), mime: "image/png".into(), bytes: vec![1, 2, 3] } };
        let mut with_tex = base.clone();
        apply_semio_mesh_mutation(&mut with_tex, &add_tex);
        let set_bytes = SemioMeshMutation::SetTextureBytes { id: "tex-a".into(), mime: "image/jpeg".into(), bytes: vec![9, 9] };
        let mut after2 = with_tex.clone();
        apply_semio_mesh_mutation(&mut after2, &set_bytes);
        assert_eq!(after2.textures[0].mime, "image/jpeg");
        for inv in Mutation::inverse(&set_bytes, &with_tex) {
            apply_semio_mesh_mutation(&mut after2, &inv);
        }
        assert_eq!(after2, with_tex);
    }

    //#region 🔖️Fixtures
    /// 🌱 `sweep_a`/`sweep_b` differ in EVERY mutable field. `meshes`: a removed mesh, a
    /// modified mesh (whose OWN nested `primitives` exercises removed+modified+added), an added
    /// mesh. `materials`/`textures`: one removed, one modified-in-every-field, one added each.
    /// `material_id` tri-state: cleared going `a -> b` (`Some(None)`), set going `b -> a`
    /// (`Some(Some(_))`).
    fn sweep_a() -> SemioMeshSnapshot {
        SemioMeshSnapshot {
            meshes: vec![
                SemioMesh { id: "toRemove".into(), primitives: vec![] },
                SemioMesh {
                    id: "toModify".into(),
                    primitives: vec![
                        SemioPrimitive { id: "toRemove".into(), topology: SemioTopology::Points, ..Default::default() },
                        SemioPrimitive {
                            id: "toModify".into(),
                            topology: SemioTopology::Triangles,
                            positions: vec![SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 }],
                            normals: vec![SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 }],
                            uvs: vec![SemioUv { u: 0.0, v: 0.0 }],
                            colors: vec![SemioRgba { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }],
                            indices: vec![0],
                            material_id: Some("matKeep".into()),
                        },
                    ],
                },
                SemioMesh { id: "keep".into(), primitives: vec![] },
            ],
            materials: vec![
                SemioMaterial { id: "toRemove".into(), base_color: SemioRgba { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }, metallic: 0.0, roughness: 0.0 },
                SemioMaterial { id: "toModify".into(), base_color: SemioRgba { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }, metallic: 0.0, roughness: 0.0 },
                SemioMaterial { id: "matKeep".into(), base_color: SemioRgba { r: 0.2, g: 0.2, b: 0.2, a: 1.0 }, metallic: 0.5, roughness: 0.5 },
            ],
            textures: vec![
                SemioTexture { id: "toRemove".into(), mime: "image/png".into(), bytes: vec![0] },
                SemioTexture { id: "toModify".into(), mime: "image/png".into(), bytes: vec![1, 2, 3] },
            ],
            ..Default::default()
        }
    }

    fn sweep_b() -> SemioMeshSnapshot {
        SemioMeshSnapshot {
            meshes: vec![
                SemioMesh {
                    id: "toModify".into(),
                    primitives: vec![
                        SemioPrimitive {
                            id: "toModify".into(),
                            topology: SemioTopology::Lines,
                            positions: vec![SemioPoint3 { x: 9.0, y: 9.0, z: 9.0 }],
                            normals: vec![SemioPoint3 { x: 1.0, y: 0.0, z: 0.0 }],
                            uvs: vec![SemioUv { u: 1.0, v: 1.0 }],
                            colors: vec![SemioRgba { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }],
                            indices: vec![0, 0],
                            material_id: None,
                        },
                        SemioPrimitive { id: "added".into(), topology: SemioTopology::Points, ..Default::default() },
                    ],
                },
                SemioMesh { id: "keep".into(), primitives: vec![] },
                SemioMesh { id: "added".into(), primitives: vec![] },
            ],
            materials: vec![
                SemioMaterial { id: "toModify".into(), base_color: SemioRgba { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }, metallic: 1.0, roughness: 1.0 },
                SemioMaterial { id: "matKeep".into(), base_color: SemioRgba { r: 0.2, g: 0.2, b: 0.2, a: 1.0 }, metallic: 0.5, roughness: 0.5 },
                SemioMaterial { id: "added".into(), base_color: SemioRgba { r: 0.0, g: 1.0, b: 0.0, a: 1.0 }, metallic: 0.3, roughness: 0.7 },
            ],
            textures: vec![
                SemioTexture { id: "toModify".into(), mime: "image/jpeg".into(), bytes: vec![9, 8, 7] },
                SemioTexture { id: "added".into(), mime: "image/webp".into(), bytes: vec![5] },
            ],
            ..Default::default()
        }
    }
    //#endregion 🔖️Fixtures

    //#region 🔖️MutationDiffLaw
    fn sample_mutations() -> Vec<SemioMeshMutation> {
        vec![
            SemioMeshMutation::NoMutation,
            SemioMeshMutation::SetSnapshot { snapshot: sweep_b() },
            SemioMeshMutation::AddMesh { mesh: SemioMesh { id: "x".into(), primitives: vec![] } },
            SemioMeshMutation::RemoveMesh { id: "toRemove".into() },
            SemioMeshMutation::AddPrimitive { mesh_id: "toModify".into(), primitive: SemioPrimitive { id: "y".into(), ..Default::default() } },
            SemioMeshMutation::RemovePrimitive { mesh_id: "toModify".into(), primitive_id: "toRemove".into() },
            SemioMeshMutation::SetPrimitiveTopology { mesh_id: "toModify".into(), primitive_id: "toModify".into(), topology: SemioTopology::TriangleFan },
            SemioMeshMutation::SetPrimitiveGeometry {
                mesh_id: "toModify".into(),
                primitive_id: "toModify".into(),
                positions: vec![SemioPoint3 { x: 2.0, y: 2.0, z: 2.0 }],
                normals: vec![],
                uvs: vec![],
                colors: vec![],
                indices: vec![],
            },
            SemioMeshMutation::SetPrimitiveMaterial { mesh_id: "toModify".into(), primitive_id: "toModify".into(), material_id: Some("matKeep".into()) },
            SemioMeshMutation::AddMaterial { material: SemioMaterial { id: "z".into(), ..Default::default() } },
            SemioMeshMutation::RemoveMaterial { id: "toRemove".into() },
            SemioMeshMutation::SetMaterialBaseColor { id: "matKeep".into(), base_color: SemioRgba { r: 1.0, g: 0.0, b: 1.0, a: 1.0 } },
            SemioMeshMutation::SetMaterialPbr { id: "matKeep".into(), metallic: 0.9, roughness: 0.1 },
            SemioMeshMutation::AddTexture { texture: SemioTexture { id: "w".into(), mime: "image/png".into(), bytes: vec![] } },
            SemioMeshMutation::RemoveTexture { id: "toRemove".into() },
            SemioMeshMutation::SetTextureBytes { id: "toModify".into(), mime: "image/bmp".into(), bytes: vec![1] },
        ]
    }

    #[test]
    fn mutation_diff_law() {
        for mutation in sample_mutations() {
            let base = sweep_a();
            let diff_direct = Mutation::diff(&mutation, &base);
            let applied_via_diff = MutationDiff::apply(&diff_direct, &base);

            let mut via_apply = base.clone();
            let diff_from_apply = apply_semio_mesh_mutation(&mut via_apply, &mutation);

            assert_eq!(applied_via_diff, via_apply, "mutation_diff_law: apply mismatch for {mutation:?}");
            assert_eq!(diff_direct, diff_from_apply, "mutation_diff_law: diff mismatch for {mutation:?}");
        }
    }
    //#endregion 🔖️MutationDiffLaw

    //#region 🔖️InverseLaw
    #[test]
    fn inverse_law() {
        for mutation in sample_mutations() {
            let base = sweep_a();

            let mut round_tripped = base.clone();
            apply_semio_mesh_mutation(&mut round_tripped, &mutation);
            for inverse_mutation in <SemioMeshMutation as Mutation<SemioMeshSnapshot>>::inverse(&mutation, &base) {
                apply_semio_mesh_mutation(&mut round_tripped, &inverse_mutation);
            }
            assert_eq!(round_tripped, base, "inverse_law (mutation-level) failed for {mutation:?}");

            let diff = Mutation::diff(&mutation, &base);
            let next = MutationDiff::apply(&diff, &base);
            let inverse_diff = DiffAlgebra::inverse(&diff, &base);
            let restored = MutationDiff::apply(&inverse_diff, &next);
            assert_eq!(restored, base, "inverse_law (diff-level) failed for {mutation:?}");
        }
    }
    //#endregion 🔖️InverseLaw

    //#region 🔖️AbsorbLaw
    fn assert_absorb_matches_sequential(base: &SemioMeshSnapshot, d1: &SemioMeshDiff, d2: &SemioMeshDiff) -> SemioMeshDiff {
        let sequential = MutationDiff::apply(d2, &MutationDiff::apply(d1, base));
        let mut absorbed = d1.clone();
        MutationDiff::absorb(&mut absorbed, d2.clone());
        assert_eq!(MutationDiff::apply(&absorbed, base), sequential, "absorb_law: apply(absorb(d1,d2), base) != sequential");
        absorbed
    }

    fn materials_triple(diff: &SemioMeshDiff) -> &SemioMaterialsDiff {
        diff.materials.as_ref().expect("materials diff present")
    }

    #[test]
    fn absorb_law() {
        let base = fixture();

        // Canonical: Add+Remove(same key) -> annihilates the add.
        {
            let d1 = Mutation::diff(&SemioMeshMutation::AddMaterial { material: SemioMaterial { id: "temp".into(), ..Default::default() } }, &base);
            let mid = MutationDiff::apply(&d1, &base);
            let d2 = Mutation::diff(&SemioMeshMutation::RemoveMaterial { id: "temp".into() }, &mid);
            let absorbed = assert_absorb_matches_sequential(&base, &d1, &d2);
            let triple = materials_triple(&absorbed);
            assert!(triple.added.is_empty() && triple.removed.is_empty(), "add+remove of the same never-persisted key must annihilate cleanly");
        }

        // Canonical: Add(k1)+Add(k2) -> both survive.
        {
            let d1 = Mutation::diff(&SemioMeshMutation::AddMaterial { material: SemioMaterial { id: "m1".into(), ..Default::default() } }, &base);
            let mid = MutationDiff::apply(&d1, &base);
            let d2 = Mutation::diff(&SemioMeshMutation::AddMaterial { material: SemioMaterial { id: "m2".into(), ..Default::default() } }, &mid);
            let absorbed = assert_absorb_matches_sequential(&base, &d1, &d2);
            let triple = materials_triple(&absorbed);
            assert_eq!(triple.added.len(), 2, "both independent adds must survive absorb");
            assert!(triple.added.iter().any(|a| a.item.id == "m1") && triple.added.iter().any(|a| a.item.id == "m2"));
        }

        // Canonical: Add(k)+SetField(k) -> patch into the added payload.
        {
            let d1 = Mutation::diff(&SemioMeshMutation::AddMaterial { material: SemioMaterial { id: "m3".into(), metallic: 0.0, ..Default::default() } }, &base);
            let mid = MutationDiff::apply(&d1, &base);
            let d2 = Mutation::diff(&SemioMeshMutation::SetMaterialPbr { id: "m3".into(), metallic: 1.0, roughness: 1.0 }, &mid);
            let absorbed = assert_absorb_matches_sequential(&base, &d1, &d2);
            let triple = materials_triple(&absorbed);
            assert!(triple.modified.is_empty(), "patch-into-added must not surface as a separate modified entry");
            assert_eq!(triple.added.len(), 1);
            assert_eq!(triple.added[0].item.metallic, 1.0);
        }

        // Canonical: Modify(k)+Remove(k) -> the modify is annihilated by the later remove.
        {
            let d1 = Mutation::diff(&SemioMeshMutation::SetMaterialPbr { id: "mat-a".into(), metallic: 0.9, roughness: 0.9 }, &base);
            let mid = MutationDiff::apply(&d1, &base);
            let d2 = Mutation::diff(&SemioMeshMutation::RemoveMaterial { id: "mat-a".into() }, &mid);
            let absorbed = assert_absorb_matches_sequential(&base, &d1, &d2);
            let triple = materials_triple(&absorbed);
            assert!(triple.modified.is_empty(), "modify of a since-removed item must not survive absorb");
            assert_eq!(triple.removed, vec!["mat-a".to_string()]);
        }

        // Associativity over a triple.
        {
            let d1 = Mutation::diff(&SemioMeshMutation::AddMaterial { material: SemioMaterial { id: "m1".into(), ..Default::default() } }, &base);
            let mid1 = MutationDiff::apply(&d1, &base);
            let d2 = Mutation::diff(&SemioMeshMutation::AddMaterial { material: SemioMaterial { id: "m2".into(), ..Default::default() } }, &mid1);
            let mid2 = MutationDiff::apply(&d2, &mid1);
            let d3 = Mutation::diff(&SemioMeshMutation::RemoveMaterial { id: "mat-a".into() }, &mid2);
            let sequential = MutationDiff::apply(&d3, &mid2);

            let mut left = d1.clone();
            MutationDiff::absorb(&mut left, d2.clone());
            MutationDiff::absorb(&mut left, d3.clone());

            let mut d2_then_d3 = d2.clone();
            MutationDiff::absorb(&mut d2_then_d3, d3.clone());
            let mut right = d1.clone();
            MutationDiff::absorb(&mut right, d2_then_d3);

            assert_eq!(MutationDiff::apply(&left, &base), sequential, "absorb associativity (left) failed");
            assert_eq!(MutationDiff::apply(&right, &base), sequential, "absorb associativity (right) failed");
        }
    }
    //#endregion 🔖️AbsorbLaw

    //#region 🔖️BetweenRoundtripLaw
    #[test]
    fn between_roundtrip_law() {
        let a = sweep_a();
        let b = sweep_b();
        assert_eq!(MutationDiff::apply(&<SemioMeshDiff as DiffAlgebra<SemioMeshSnapshot>>::between(&a, &b), &a), b);
        assert_eq!(MutationDiff::apply(&<SemioMeshDiff as DiffAlgebra<SemioMeshSnapshot>>::between(&b, &a), &b), a);

        let sample = fixture();
        assert_eq!(MutationDiff::apply(&<SemioMeshDiff as DiffAlgebra<SemioMeshSnapshot>>::between(&sample, &sample), &sample), sample);

        let mut mutated = sample.clone();
        apply_semio_mesh_mutation(&mut mutated, &SemioMeshMutation::SetPrimitiveTopology { mesh_id: "mesh-a".into(), primitive_id: "prim-a".into(), topology: SemioTopology::LineStrip });
        assert_ne!(sample, mutated);
        assert_eq!(MutationDiff::apply(&<SemioMeshDiff as DiffAlgebra<SemioMeshSnapshot>>::between(&sample, &mutated), &sample), mutated);
        assert_eq!(MutationDiff::apply(&<SemioMeshDiff as DiffAlgebra<SemioMeshSnapshot>>::between(&mutated, &sample), &mutated), sample);
    }
    //#endregion 🔖️BetweenRoundtripLaw

    //#region 🔖️CodecRetentionLaw
    #[test]
    fn codec_retention_law() {
        let snap = fixture();
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <SemioMeshSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }
    //#endregion 🔖️CodecRetentionLaw

    //#region 🔖️FieldSweep
    /// 🎯️ THE acceptance criterion: `sweep_a`/`sweep_b` differ in every mutable field across
    /// `meshes` (incl. nested `primitives`), `materials`, and `textures` — see the fixtures' doc
    /// comment for exactly how each collection flavor and the `material_id` tri-state are
    /// exercised.
    #[test]
    fn field_sweep() {
        let a = sweep_a();
        let b = sweep_b();

        let diff_ab = <SemioMeshDiff as DiffAlgebra<SemioMeshSnapshot>>::between(&a, &b);
        assert_eq!(MutationDiff::apply(&diff_ab, &a), b);
        let diff_ba = <SemioMeshDiff as DiffAlgebra<SemioMeshSnapshot>>::between(&b, &a);
        assert_eq!(MutationDiff::apply(&diff_ba, &b), a);
        assert!(<SemioMeshDiff as DiffAlgebra<SemioMeshSnapshot>>::between(&a, &a).is_empty());

        let meshes: &SemioMeshesDiff = diff_ab.meshes.as_ref().expect("meshes diff present");
        assert!(!meshes.removed.is_empty(), "meshes: removed not exercised");
        assert!(!meshes.added.is_empty(), "meshes: added not exercised");
        let mesh_mod = meshes.modified.iter().find(|m| m.key == "toModify").expect("toModify mesh modified");
        let prims: &SemioPrimitivesDiff = mesh_mod.diff.primitives.as_ref().expect("primitives diff present");
        assert!(!prims.removed.is_empty(), "primitives: removed not exercised");
        assert!(!prims.added.is_empty(), "primitives: added not exercised");
        let prim_mod = prims.modified.iter().find(|p| p.key == "toModify").expect("toModify primitive modified");
        assert!(prim_mod.diff.topology.is_some() && prim_mod.diff.positions.is_some() && prim_mod.diff.normals.is_some());
        assert!(prim_mod.diff.uvs.is_some() && prim_mod.diff.colors.is_some() && prim_mod.diff.indices.is_some());
        assert_eq!(prim_mod.diff.material_id, Some(None), "material_id tri-state Some(None) not exercised");

        let materials: &SemioMaterialsDiff = diff_ab.materials.as_ref().expect("materials diff present");
        assert!(!materials.removed.is_empty() && !materials.added.is_empty(), "materials: removed/added not exercised");
        let mat_mod = materials.modified.iter().find(|m| m.key == "toModify").expect("toModify material modified");
        assert!(mat_mod.diff.base_color.is_some() && mat_mod.diff.metallic.is_some() && mat_mod.diff.roughness.is_some());

        let textures: &SemioTexturesDiff = diff_ab.textures.as_ref().expect("textures diff present");
        assert!(!textures.removed.is_empty() && !textures.added.is_empty(), "textures: removed/added not exercised");
        let tex_mod = textures.modified.iter().find(|t| t.key == "toModify").expect("toModify texture modified");
        assert!(tex_mod.diff.mime.is_some() && tex_mod.diff.bytes.is_some());

        // b -> a: the opposite tri-state, Some(Some(_)).
        let mesh_mod_ba = diff_ba.meshes.as_ref().unwrap().modified.iter().find(|m| m.key == "toModify").expect("toModify mesh modified (b->a)");
        let prim_mod_ba = mesh_mod_ba.diff.primitives.as_ref().unwrap().modified.iter().find(|p| p.key == "toModify").expect("toModify primitive modified (b->a)");
        assert_eq!(prim_mod_ba.diff.material_id, Some(Some("matKeep".to_string())), "material_id tri-state Some(Some(_)) not exercised");
    }
    //#endregion 🔖️FieldSweep

    //#region 🔖️OpTextBinaryRoundtripLaw
    #[test]
    fn op_text_binary_roundtrip_law() {
        let mutations = sample_mutations();
        for mutation in mutations {
            let printed = mutation.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = SemioMeshMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch for {mutation:?} (printed {printed:?})");

            let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op({mutation:?}) failed: {e}"));
            let decoded = SemioMeshMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch for {mutation:?}");
        }
    }
    //#endregion 🔖️OpTextBinaryRoundtripLaw
}
//#endregion 🔖️Tests
