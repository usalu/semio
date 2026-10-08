//! 🧭️ The mesh editor's edit rules: which snapshot pointer raises which ONE concrete mesh mutation kind. An edit no kind expresses is refused, never approximated.

use crate::editor::semio_base::edit_plumbing::{complete, ent, find, ins, keyed, named, rem, resolve, Entries, Reshape};
use crate::standards::v1::subsets::mesh::schema::mutations::SemioMeshMutation;
use crate::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use semio_framework_plugin::Fault;
use semio_framework_value::DslValue;
use semio_s_artifact_stdio_contract::editing::{EditRules, Selector, SnapshotEditEvent};

const ID: Selector = named("id", "id");
const MESH: Selector = named("mesh_id", "id");
const PRIMITIVE: Selector = named("primitive_id", "id");
const VERTEX: Selector = Selector::Index("vertex_index");

/// 📚 Every pointer a mesh editor edit resolves, one rule per kind that sets, inserts or removes through it.
pub const EDIT_RULES: EditRules = EditRules {
    entities: &[
        ent("/materials/*/baseColor", "change-material-base-color", &[ID], "new_base_color"),
        ent("/materials/*/metallic", "change-material-metallic", &[ID], "new_metallic"),
        ent("/materials/*/roughness", "change-material-roughness", &[ID], "new_roughness"),
        ent("/textures/*/mime", "change-texture-mime", &[ID], "new_mime"),
        ent("/textures/*/bytes", "replace-texture-bytes", &[ID], "new_bytes"),
        ent("/meshes/*/primitives/*/topology", "set-primitive-topology", &[MESH, PRIMITIVE], "topology"),
        ent("/meshes/*/primitives/*/materialId", "set-primitive-material", &[MESH, PRIMITIVE], "material_id"),
        ent("/meshes/*/primitives/*/positions/*", "move-vertex", &[MESH, PRIMITIVE, VERTEX], "new_point"),
        ent("/meshes/*/primitives/*/positions", "replace-primitive-geometry", &[MESH, PRIMITIVE], "positions"),
        ent("/meshes/*/primitives/*/normals", "replace-primitive-geometry", &[MESH, PRIMITIVE], "normals"),
        ent("/meshes/*/primitives/*/uvs", "replace-primitive-geometry", &[MESH, PRIMITIVE], "uvs"),
        ent("/meshes/*/primitives/*/colors", "replace-primitive-geometry", &[MESH, PRIMITIVE], "colors"),
        ent("/meshes/*/primitives/*/indices", "replace-primitive-geometry", &[MESH, PRIMITIVE], "indices"),
    ],
    inserts: &[
        ins("/meshes", "create-mesh", &[], Some("at"), "mesh"),
        ins("/materials", "create-material", &[], Some("at"), "material"),
        ins("/textures", "create-texture", &[], Some("at"), "texture"),
        ins("/meshes/*/primitives", "create-primitive", &[MESH], Some("at"), "primitive"),
    ],
    removes: &[
        rem("/meshes", "delete-mesh", &[], keyed("id", "id")),
        rem("/materials", "delete-material", &[], keyed("id", "id")),
        rem("/textures", "delete-texture", &[], keyed("id", "id")),
        rem("/meshes/*/primitives", "delete-primitive", &[MESH], keyed("primitive_id", "id")),
    ],
};

const RESHAPES: &[(&str, Reshape)] = &[("replace-primitive-geometry", geometry)];

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn geometry(tree: &DslValue, entries: &mut Entries) -> Result<(), String> {
    let named_entry = |name: &str| entries.iter().find(|(key, _)| key == name).map(|(_, value)| value.clone()).ok_or_else(|| format!("the edit carries no '{name}'"));
    let (mesh_id, primitive_id) = (named_entry("mesh_id")?, named_entry("primitive_id")?);
    let mesh = find(tree, "meshes", "id", &mesh_id).ok_or("the mesh does not exist")?;
    let primitive = find(mesh, "primitives", "id", &primitive_id).ok_or("the primitive does not exist")?;
    complete(entries, primitive, &["positions", "normals", "uvs", "colors", "indices"])
}

/// 🧩️ Every edit goes through [`EDIT_RULES`]; a geometry edit carries the buffers it keeps.
pub(crate) fn special(event: &SnapshotEditEvent, snapshot: &SemioMeshSnapshot) -> Result<Option<Vec<SemioMeshMutation>>, Fault> {
    resolve::<SemioMeshSnapshot, SemioMeshMutation>(&EDIT_RULES, RESHAPES, snapshot, event).map(Some)
}
