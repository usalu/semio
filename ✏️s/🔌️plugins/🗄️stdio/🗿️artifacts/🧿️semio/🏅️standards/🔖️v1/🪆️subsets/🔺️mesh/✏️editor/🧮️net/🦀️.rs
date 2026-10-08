//! 🧮️ Net of one snapshot edit as mesh domain leaves: the delta between the edited snapshot and the base, expressed as the
//! concrete `mesh` mutations that carry the base to it. A change the vocabulary cannot express yields leaves whose fold differs from
//! the edit, which the editor's publication check refuses.

use crate::standards::v1::subsets::base::schema::triples::net_keyed;
use crate::standards::v1::subsets::mesh::schema::mutations::{
    change_material_base, change_material_metallic, change_material_roughness, change_texture_mime, create_material, create_mesh, create_primitive, create_texture, delete_material, delete_mesh, delete_primitive, delete_texture, replace_primitive_geometry, replace_texture_bytes,
    set_primitive_material, set_primitive_topology, SemioMeshMutation,
};
use crate::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn net(base: &SemioMeshSnapshot, next: &SemioMeshSnapshot) -> Vec<SemioMeshMutation> {
    let meshes = net_keyed(&base.meshes, &next.meshes, |mesh| mesh.id.clone());
    let materials = net_keyed(&base.materials, &next.materials, |material| material.id.clone());
    let textures = net_keyed(&base.textures, &next.textures, |texture| texture.id.clone());
    let mut out = Vec::new();
    out.extend(meshes.removed.iter().map(|mesh| SemioMeshMutation::DeleteMesh(delete_mesh::DeleteMesh { id: mesh.id.clone() })));
    out.extend(textures.added.iter().map(|texture| SemioMeshMutation::CreateTexture(create_texture::CreateTexture { texture: (*texture).clone(), at: None })));
    for (before, after) in &textures.modified {
        if before.mime != after.mime {
            out.push(SemioMeshMutation::ChangeTextureMime(change_texture_mime::ChangeTextureMime { id: after.id.clone(), new_mime: after.mime.clone() }));
        }
        if before.bytes != after.bytes {
            out.push(SemioMeshMutation::ReplaceTextureBytes(replace_texture_bytes::ReplaceTextureBytes { id: after.id.clone(), new_bytes: after.bytes.clone() }));
        }
    }
    out.extend(materials.added.iter().map(|material| SemioMeshMutation::CreateMaterial(create_material::CreateMaterial { material: (*material).clone(), at: None })));
    for (before, after) in &materials.modified {
        if before.base_color != after.base_color {
            out.push(SemioMeshMutation::ChangeMaterialBaseColor(change_material_base::ChangeMaterialBaseColor { id: after.id.clone(), new_base_color: after.base_color }));
        }
        if before.metallic != after.metallic {
            out.push(SemioMeshMutation::ChangeMaterialMetallic(change_material_metallic::ChangeMaterialMetallic { id: after.id.clone(), new_metallic: after.metallic }));
        }
        if before.roughness != after.roughness {
            out.push(SemioMeshMutation::ChangeMaterialRoughness(change_material_roughness::ChangeMaterialRoughness { id: after.id.clone(), new_roughness: after.roughness }));
        }
    }
    out.extend(meshes.added.iter().map(|mesh| SemioMeshMutation::CreateMesh(create_mesh::CreateMesh { mesh: (*mesh).clone(), at: None })));
    for (before, after) in &meshes.modified {
        let primitives = net_keyed(&before.primitives, &after.primitives, |primitive| primitive.id.clone());
        out.extend(primitives.removed.iter().map(|primitive| SemioMeshMutation::DeletePrimitive(delete_primitive::DeletePrimitive { mesh_id: after.id.clone(), primitive_id: primitive.id.clone() })));
        for (old, new) in &primitives.modified {
            if old.topology != new.topology {
                out.push(SemioMeshMutation::SetPrimitiveTopology(set_primitive_topology::SetPrimitiveTopology { mesh_id: after.id.clone(), primitive_id: new.id.clone(), topology: new.topology }));
            }
            if old.material_id != new.material_id {
                out.push(SemioMeshMutation::SetPrimitiveMaterial(set_primitive_material::SetPrimitiveMaterial { mesh_id: after.id.clone(), primitive_id: new.id.clone(), material_id: new.material_id.clone() }));
            }
            if (&old.positions, &old.normals, &old.uvs, &old.colors, &old.indices) != (&new.positions, &new.normals, &new.uvs, &new.colors, &new.indices) {
                out.push(SemioMeshMutation::ReplacePrimitiveGeometry(replace_primitive_geometry::ReplacePrimitiveGeometry {
                    mesh_id: after.id.clone(),
                    primitive_id: new.id.clone(),
                    positions: new.positions.clone(),
                    normals: new.normals.clone(),
                    uvs: new.uvs.clone(),
                    colors: new.colors.clone(),
                    indices: new.indices.clone(),
                }));
            }
        }
        out.extend(primitives.added.iter().map(|primitive| SemioMeshMutation::CreatePrimitive(create_primitive::CreatePrimitive { mesh_id: after.id.clone(), primitive: (*primitive).clone(), at: None })));
    }
    out.extend(materials.removed.iter().map(|material| SemioMeshMutation::DeleteMaterial(delete_material::DeleteMaterial { id: material.id.clone() })));
    out.extend(textures.removed.iter().map(|texture| SemioMeshMutation::DeleteTexture(delete_texture::DeleteTexture { id: texture.id.clone() })));
    out
}
