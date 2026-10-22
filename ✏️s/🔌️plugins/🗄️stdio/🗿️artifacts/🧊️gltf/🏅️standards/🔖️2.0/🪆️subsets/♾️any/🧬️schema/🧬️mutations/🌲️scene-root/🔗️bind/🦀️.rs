//! 🧬️ Direct bind-scene-root-node mutation owner: payload, validation, typed diff, inverse, and outcomes.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::diff::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level_collections::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::structure_geometry::{checked_index, checked_position};
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.bind-scene-root-node.v1";
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct GltfBindSceneRootNodePayload {
    pub scene: usize,
    pub node: usize,
    pub position: usize,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfBindSceneRootNodePayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    checked_index(payload.scene, base.document.scenes.len(), "document/scenes")?;
    checked_index(payload.node, base.document.nodes.len(), "document/nodes")?;
    checked_position(payload.position, base.document.scenes[payload.scene].nodes.len(), "document/scenes/nodes")?;
    if base.document.scenes[payload.scene].nodes.contains(&payload.node) {
        return Err(reject("gltf.mutation.duplicate-scene-root", format!("document/scenes/{}/nodes", payload.scene), "node is already a scene root"));
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfBindSceneRootNodePayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    Ok(GltfDiff { scenes: patch(p.scene, GltfSceneDiff { nodes: Some(GltfRefsDelta::insertion(p.position, GltfRef(p.node))), ..Default::default() }), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(p: &GltfBindSceneRootNodePayload, base: &GltfSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::remove_used_extension::RemoveUsedExtensionMutation>,
{
    if validate(p, base).is_err() {
        return Vec::new();
    }
    vec![crate::standards::v2_0::subsets::any::schema::mutations::unbind_scene_root_node::mutation(crate::standards::v2_0::subsets::any::schema::mutations::unbind_scene_root_node::GltfUnbindSceneRootNodePayload { scene: p.scene, node: p.node })]
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum BindSceneRootNodeMutation {
    Apply(GltfBindSceneRootNodePayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation<Op>(payload: GltfBindSceneRootNodePayload) -> Op
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::bind_scene_root_node::BindSceneRootNodeMutation>,
{
    Op::from(BindSceneRootNodeMutation::Apply(payload))
}

impl<Op> protocol::MutationKind<GltfSnapshot, Op> for BindSceneRootNodeMutation
where
    Op: protocol::Mutation<GltfSnapshot, Diff = GltfDiff> + From<BindSceneRootNodeMutation>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "bind", entity: "scene-root-node", kind: "bind-scene-root-node", record: "BoundSceneRootNode" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Bind Scene Root Node", "Szenenwurzelknoten binden")
    }

    fn target(&self) -> Vec<String> {
        vec!["bind-scene-root-node".to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<GltfSnapshot, Op> for BindSceneRootNodeMutation
where
    Op: protocol::ColdMutation<GltfSnapshot, ReferenceDiff = GltfDiff> + From<BindSceneRootNodeMutation>,
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


