//! 🧬️ Direct bind-node-camera mutation owner: payload, validation, typed diff, inverse, and outcomes.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::diff::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level_collections::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::structure_geometry::checked_index;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::GltfTopLevelMutationRejection;
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.bind-node-camera.v1";
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct GltfBindNodeCameraPayload {
    pub node: usize,
    pub camera: usize,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfBindNodeCameraPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    checked_index(payload.node, base.document.nodes.len(), "document/nodes")?;
    checked_index(payload.camera, base.document.cameras.len(), "document/cameras")?;
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfBindNodeCameraPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let current = base.document.nodes[p.node].camera;
    Ok(GltfDiff { nodes: patch(p.node, GltfNodeDiff { camera: (current != Some(p.camera)).then_some(Some(p.camera)), ..Default::default() }), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(p: &GltfBindNodeCameraPayload, base: &GltfSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::unbind_node_camera::UnbindNodeCameraMutation> + From<crate::standards::v2_0::subsets::any::schema::mutations::bind_node_camera::BindNodeCameraMutation>,
{
    if validate(p, base).is_err() {
        return Vec::new();
    }
    match base.document.nodes[p.node].camera {
        Some(current) if current == p.camera => Vec::new(),
        Some(current) => vec![mutation(GltfBindNodeCameraPayload { node: p.node, camera: current })],
        None => vec![crate::standards::v2_0::subsets::any::schema::mutations::unbind_node_camera::mutation(crate::standards::v2_0::subsets::any::schema::mutations::unbind_node_camera::GltfUnbindNodeCameraPayload { node: p.node })],
    }
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum BindNodeCameraMutation {
    Apply(GltfBindNodeCameraPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation<Op>(payload: GltfBindNodeCameraPayload) -> Op
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::bind_node_camera::BindNodeCameraMutation>,
{
    Op::from(BindNodeCameraMutation::Apply(payload))
}

impl<Op> protocol::MutationKind<GltfSnapshot, Op> for BindNodeCameraMutation
where
    Op: protocol::Mutation<GltfSnapshot, Diff = GltfDiff> + From<BindNodeCameraMutation>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "bind", entity: "node-camera", kind: "bind-node-camera", record: "BoundNodeCamera" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Bind Node Camera", "Knotenkamera binden")
    }

    fn target(&self) -> Vec<String> {
        vec!["bind-node-camera".to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<GltfSnapshot, Op> for BindNodeCameraMutation
where
    Op: protocol::ColdMutation<GltfSnapshot, ReferenceDiff = GltfDiff> + From<BindNodeCameraMutation> + From<crate::standards::v2_0::subsets::any::schema::mutations::unbind_node_camera::UnbindNodeCameraMutation>,
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
