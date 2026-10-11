//! 🧬️ Direct delete-node mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::schema::diff::*;
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::schema::modules::mutation_support::top_level_collections::*;
use crate::GltfSnapshot;
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
pub fn inverse(p: &GltfDeleteNodePayload, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
    if validate(p, base).is_err() {
        return Vec::new();
    }
    let mut rows = vec![super::create_node::mutation(super::create_node::GltfCreateNodePayload { position: p.index, node: Some(Box::new(base.document.nodes[p.index].clone())) })];
    for (scene, entry) in base.document.scenes.iter().enumerate() {
        if let Some(position) = entry.nodes.iter().position(|node| *node == p.index) {
            rows.push(super::bind_scene_root_node::mutation(super::bind_scene_root_node::GltfBindSceneRootNodePayload { scene, node: p.index, position }));
        }
    }
    for (parent, entry) in base.document.nodes.iter().enumerate() {
        if let Some(position) = entry.children.iter().position(|child| *child == p.index) {
            rows.push(super::bind_node_child::mutation(super::bind_node_child::GltfBindNodeChildPayload { parent, child: p.index, position }));
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
pub fn mutation(payload: GltfDeleteNodePayload) -> super::GltfMutation {
    super::GltfMutation::DeleteNode(DeleteNodeMutation::Apply(payload))
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for DeleteNodeMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "node", kind: "delete-node", record: "DeletedNode" };

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
        semio_framework_ui_locale::LocalizedLabel::native("Delete Node", "Knoten löschen")
    }

    fn target(&self) -> Vec<String> {
        vec!["delete-node".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🚫️removes-the-root-516263/🦀️.rs"]
mod case_removes_the_root_516263;
#[cfg(test)]
#[path = "🧪️tests/🔬️middle-row/🦀️.rs"]
mod case_middle_row;
//#endregion 🧪️Tests
