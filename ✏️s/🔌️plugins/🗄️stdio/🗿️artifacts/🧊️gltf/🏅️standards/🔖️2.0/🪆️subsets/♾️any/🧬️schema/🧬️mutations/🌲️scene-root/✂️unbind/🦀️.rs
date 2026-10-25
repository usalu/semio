//! 🧬️ Direct unbind-scene-root-node mutation owner: payload, validation, typed diff, inverse, and outcomes.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::diff::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level_collections::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::structure_geometry::checked_index;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.unbind-scene-root-node.v1";
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct GltfUnbindSceneRootNodePayload {
    pub scene: usize,
    pub node: usize,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfUnbindSceneRootNodePayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    checked_index(payload.scene, base.document.scenes.len(), "document/scenes")?;
    checked_index(payload.node, base.document.nodes.len(), "document/nodes")?;
    if !base.document.scenes[payload.scene].nodes.contains(&payload.node) {
        return Err(reject("gltf.mutation.relation-absent", format!("document/scenes/{}/nodes", payload.scene), "node is not a root of this scene"));
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfUnbindSceneRootNodePayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let Some(index) = base.document.scenes[p.scene].nodes.iter().position(|node| *node == p.node) else {
        return Err(reject("gltf.mutation.relation-absent", format!("document/scenes/{}/nodes", p.scene), "node is not a root of this scene"));
    };
    Ok(GltfDiff { scenes: patch(p.scene, GltfSceneDiff { nodes: Some(GltfRefsDelta::removal(&refs(&base.document.scenes[p.scene].nodes), index)), ..Default::default() }), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(p: &GltfUnbindSceneRootNodePayload, base: &GltfSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::bind_scene_root_node::BindSceneRootNodeMutation>,
{
    if validate(p, base).is_err() {
        return Vec::new();
    }
    let Some(index) = base.document.scenes[p.scene].nodes.iter().position(|node| *node == p.node) else {
        return Vec::new();
    };
    vec![crate::standards::v2_0::subsets::any::schema::mutations::bind_scene_root_node::mutation(crate::standards::v2_0::subsets::any::schema::mutations::bind_scene_root_node::GltfBindSceneRootNodePayload { scene: p.scene, node: p.node, position: index })]
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum UnbindSceneRootNodeMutation {
    Apply(GltfUnbindSceneRootNodePayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation<Op>(payload: GltfUnbindSceneRootNodePayload) -> Op
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::unbind_scene_root_node::UnbindSceneRootNodeMutation>,
{
    Op::from(UnbindSceneRootNodeMutation::Apply(payload))
}

impl<Op> protocol::MutationKind<GltfSnapshot, Op> for UnbindSceneRootNodeMutation
where
    Op: protocol::Mutation<GltfSnapshot, Diff = GltfDiff> + From<UnbindSceneRootNodeMutation>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "unbind", entity: "scene-root-node", kind: "unbind-scene-root-node", record: "UnboundSceneRootNode" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Unbind Scene Root Node", "Bindung des Szenenwurzelknotens aufheben")
    }

    fn target(&self) -> Vec<String> {
        vec!["unbind-scene-root-node".to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<GltfSnapshot, Op> for UnbindSceneRootNodeMutation
where
    Op: protocol::ColdMutation<GltfSnapshot, ReferenceDiff = GltfDiff> + From<UnbindSceneRootNodeMutation> + From<crate::standards::v2_0::subsets::any::schema::mutations::bind_scene_root_node::BindSceneRootNodeMutation>,
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


