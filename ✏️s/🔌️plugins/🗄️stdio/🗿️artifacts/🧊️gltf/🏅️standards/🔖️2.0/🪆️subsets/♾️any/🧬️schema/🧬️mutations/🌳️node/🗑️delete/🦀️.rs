//! 🧬️ Direct delete-node mutation owner: payload, validation, typed diff, inverse, and outcomes.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::diff::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level_collections::*;
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.delete-node.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/nodes"];
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct GltfDeleteNodePayload {
    pub index: usize,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfDeleteNodePayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    if payload.index >= base.document.nodes.len() {
        return Err(reject("gltf.mutation.index-out-of-range", "document/nodes", "index must address an item"));
    }
    require_unreferenced(base, GltfTopLevelFamily::Nodes, payload.index)?;
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfDeleteNodePayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let mut diff = rewire(base, GltfTopLevelFamily::Nodes, &mut after_delete(p.index));
    let slot = diff.nodes.get_or_insert_with(Default::default);
    slot.modified.retain(|entry| entry.index != p.index);
    slot.removed.push(p.index);
    Ok(diff)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(p: &GltfDeleteNodePayload, base: &GltfSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::remove_used_extension::RemoveUsedExtensionMutation>,
{
    if validate(p, base).is_err() {
        return Vec::new();
    }
    let mut rows = vec![crate::standards::v2_0::subsets::any::schema::mutations::create_node::mutation(crate::standards::v2_0::subsets::any::schema::mutations::create_node::GltfCreateNodePayload { position: p.index, node: Some(Box::new(base.document.nodes[p.index].clone())) })];
    for (scene, entry) in base.document.scenes.iter().enumerate() {
        if let Some(position) = entry.nodes.iter().position(|node| *node == p.index) {
            rows.push(crate::standards::v2_0::subsets::any::schema::mutations::bind_scene_root_node::mutation(crate::standards::v2_0::subsets::any::schema::mutations::bind_scene_root_node::GltfBindSceneRootNodePayload { scene, node: p.index, position }));
        }
    }
    for (parent, entry) in base.document.nodes.iter().enumerate() {
        if let Some(position) = entry.children.iter().position(|child| *child == p.index) {
            rows.push(crate::standards::v2_0::subsets::any::schema::mutations::bind_node_child::mutation(crate::standards::v2_0::subsets::any::schema::mutations::bind_node_child::GltfBindNodeChildPayload { parent, child: p.index, position }));
        }
    }
    rows.reverse();
    rows
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum DeleteNodeMutation {
    Apply(GltfDeleteNodePayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation<Op>(payload: GltfDeleteNodePayload) -> Op
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::delete_node::DeleteNodeMutation>,
{
    Op::from(DeleteNodeMutation::Apply(payload))
}

impl<Op> protocol::MutationKind<GltfSnapshot, Op> for DeleteNodeMutation
where
    Op: protocol::Mutation<GltfSnapshot, Diff = GltfDiff> + From<DeleteNodeMutation>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "node", kind: "delete-node", record: "DeletedNode" };

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

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Delete Node", "Knoten löschen")
    }

    fn target(&self) -> Vec<String> {
        vec!["delete-node".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests


//#endregion 🧪️Tests
