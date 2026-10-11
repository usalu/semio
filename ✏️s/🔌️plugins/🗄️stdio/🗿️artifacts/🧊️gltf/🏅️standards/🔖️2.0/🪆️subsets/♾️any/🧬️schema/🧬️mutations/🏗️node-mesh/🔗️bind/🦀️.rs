//! 🧬️ Direct bind-node-mesh mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::schema::diff::*;
use crate::schema::modules::mutation_support::top_level_collections::*;
use crate::schema::modules::mutation_support::structure_geometry::checked_index;
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::schema::modules::mutation_support::top_level::GltfTopLevelMutationRejection;
use crate::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.bind-node-mesh.v1";
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct GltfBindNodeMeshPayload {
    pub node: usize,
    pub mesh: usize,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfBindNodeMeshPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    checked_index(payload.node, base.document.nodes.len(), "document/nodes")?;
    checked_index(payload.mesh, base.document.meshes.len(), "document/meshes")?;
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfBindNodeMeshPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let current = base.document.nodes[p.node].mesh;
    Ok(GltfDiff { nodes: patch(p.node, GltfNodeDiff { mesh: (current != Some(p.mesh)).then_some(Some(p.mesh)), ..Default::default() }), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(p: &GltfBindNodeMeshPayload, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
    if validate(p, base).is_err() {
        return Vec::new();
    }
    match base.document.nodes[p.node].mesh {
        Some(current) if current == p.mesh => Vec::new(),
        Some(current) => vec![super::bind_node_mesh::mutation(super::bind_node_mesh::GltfBindNodeMeshPayload { node: p.node, mesh: current })],
        None => vec![super::unbind_node_mesh::mutation(super::unbind_node_mesh::GltfUnbindNodeMeshPayload { node: p.node })],
    }
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum BindNodeMeshMutation {
    Apply(GltfBindNodeMeshPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation(payload: GltfBindNodeMeshPayload) -> super::GltfMutation {
    super::GltfMutation::BindNodeMesh(BindNodeMeshMutation::Apply(payload))
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for BindNodeMeshMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "bind", entity: "node-mesh", kind: "bind-node-mesh", record: "BoundNodeMesh" };

    fn diff(&self, base: &GltfSnapshot) -> protocol::MutationOutcome<crate::schema::diff::GltfDiff> {
        match self {
            Self::Apply(payload) => match plan(payload, base) {
                Ok(diff) => protocol::MutationOutcome::new(diff),
                Err(error) => rejection_outcome(&error.code, &error.path, error.detail),
            },
        }
    }

    fn inverse(&self, base: &GltfSnapshot) -> Result<Vec<super::GltfMutation>, semio_framework_value::ValueError> {
        match self {
            Self::Apply(payload) => Ok(inverse(payload, base)),
        }
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Bind Node Mesh", "Knotennetz binden")
    }

    fn target(&self) -> Vec<String> {
        vec!["bind-node-mesh".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔗️binds-the-hull-fe46fd/🦀️.rs"]
mod case_binds_the_hull_fe46fd;
//#endregion 🧪️Tests
