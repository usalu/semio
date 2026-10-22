//! 🧬️ Direct unbind-node-mesh mutation owner: payload, validation, typed diff, inverse, and outcomes.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::diff::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level_collections::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::structure_geometry::checked_index;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.unbind-node-mesh.v1";
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct GltfUnbindNodeMeshPayload {
    pub node: usize,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfUnbindNodeMeshPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    checked_index(payload.node, base.document.nodes.len(), "document/nodes")?;
    if base.document.nodes[payload.node].mesh.is_none() {
        return Err(reject("gltf.mutation.relation-absent", format!("document/nodes/{}/mesh", payload.node), "node has no binding"));
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfUnbindNodeMeshPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    Ok(GltfDiff { nodes: patch(p.node, GltfNodeDiff { mesh: Some(None), ..Default::default() }), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(p: &GltfUnbindNodeMeshPayload, base: &GltfSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::remove_used_extension::RemoveUsedExtensionMutation>,
{
    if validate(p, base).is_err() {
        return Vec::new();
    }
    match base.document.nodes[p.node].mesh {
        Some(current) => vec![crate::standards::v2_0::subsets::any::schema::mutations::bind_node_mesh::mutation(crate::standards::v2_0::subsets::any::schema::mutations::bind_node_mesh::GltfBindNodeMeshPayload { node: p.node, mesh: current })],
        None => Vec::new(),
    }
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum UnbindNodeMeshMutation {
    Apply(GltfUnbindNodeMeshPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation<Op>(payload: GltfUnbindNodeMeshPayload) -> Op
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::unbind_node_mesh::UnbindNodeMeshMutation>,
{
    Op::from(UnbindNodeMeshMutation::Apply(payload))
}

impl<Op> protocol::MutationKind<GltfSnapshot, Op> for UnbindNodeMeshMutation
where
    Op: protocol::Mutation<GltfSnapshot, Diff = GltfDiff> + From<UnbindNodeMeshMutation>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "unbind", entity: "node-mesh", kind: "unbind-node-mesh", record: "UnboundNodeMesh" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Unbind Node Mesh", "Bindung des Knotennetzes aufheben")
    }

    fn target(&self) -> Vec<String> {
        vec!["unbind-node-mesh".to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<GltfSnapshot, Op> for UnbindNodeMeshMutation
where
    Op: protocol::ColdMutation<GltfSnapshot, ReferenceDiff = GltfDiff> + From<UnbindNodeMeshMutation>,
{
    fn diff(&self, base: &GltfSnapshot) -> protocol::MutationOutcome<GltfDiff> {
        match self {
            Self::Apply(payload) => match plan(payload, base) {
                Ok(diff) => protocol::MutationOutcome::new(diff),
                Err(error) => rejection_outcome(&error.code, &error.path, error.detail),
            },
        }
    }
    fn inverse(&self, base: &GltfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        match self {
            Self::Apply(payload) => Ok(inverse(payload, base)),
        }
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests

//#endregion 🧪️Tests
