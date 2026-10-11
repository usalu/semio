//! 🧬️ Direct move-node mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::schema::diff::*;
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::schema::modules::mutation_support::top_level_collections::*;
use crate::schema::snapshot::GltfNode;
use crate::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.move-node.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/nodes"];
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct GltfMoveNodePayload {
    pub index: usize,
    pub position: usize,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfMoveNodePayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    if payload.index >= base.document.nodes.len() || payload.position >= base.document.nodes.len() {
        return Err(reject("gltf.mutation.index-out-of-range", "document/nodes", "indices must address items"));
    }
    if payload.index == payload.position {
        return Err(reject("gltf.mutation.no-observable-change", "document/nodes", "destination equals source"));
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfMoveNodePayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let mut remap = after_move(p.index, p.position);
    let mut diff = rewire(base, GltfTopLevelFamily::Nodes, &mut remap);
    let source = &base.document.nodes[p.index];
    let item = GltfNode { children: source.children.iter().filter_map(|child| remap(*child)).collect(), ..source.clone() };
    let slot = diff.nodes.get_or_insert_with(Default::default);
    slot.modified.retain(|entry| entry.index != p.index);
    slot.removed.push(p.index);
    slot.added.push(GltfAdded { index: p.position, item });
    Ok(diff)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(p: &GltfMoveNodePayload, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
    if validate(p, base).is_err() {
        return Vec::new();
    }
    vec![super::move_node::mutation(super::move_node::GltfMoveNodePayload { index: p.position, position: p.index })]
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum MoveNodeMutation {
    Apply(GltfMoveNodePayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation(payload: GltfMoveNodePayload) -> super::GltfMutation {
    super::GltfMutation::MoveNode(MoveNodeMutation::Apply(payload))
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for MoveNodeMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "node", kind: "move-node", record: "MovedNode" };

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
        semio_framework_ui_locale::LocalizedLabel::native("Move Node", "Knoten verschieben")
    }

    fn target(&self) -> Vec<String> {
        vec!["move-node".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🚚️swaps-the-two-scene-d73df8/🦀️.rs"]
mod case_swaps_the_two_scene_d73df8;
//#endregion 🧪️Tests
