//! 🧬️ Direct move-node-child mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::schema::diff::*;
use crate::schema::modules::mutation_support::top_level_collections::*;
use crate::schema::modules::mutation_support::structure_geometry::checked_index;
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
use crate::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.move-node-child.v1";
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct GltfMoveNodeChildPayload {
    pub parent: usize,
    pub child: usize,
    pub position: usize,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfMoveNodeChildPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    checked_index(payload.parent, base.document.nodes.len(), "document/nodes")?;
    let children = &base.document.nodes[payload.parent].children;
    let index = children.iter().position(|child| *child == payload.child).ok_or_else(|| reject("gltf.mutation.relation-absent", "document/nodes/children", "child is not linked to parent"))?;
    checked_index(payload.position, children.len(), "document/nodes/children")?;
    if index == payload.position {
        return Err(reject("gltf.mutation.no-observable-change", "document/nodes/children", "destination equals source"));
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfMoveNodeChildPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let Some(index) = base.document.nodes[p.parent].children.iter().position(|child| *child == p.child) else {
        return Err(reject("gltf.mutation.relation-absent", "document/nodes/children", "child is not linked to parent"));
    };
    Ok(GltfDiff { nodes: patch(p.parent, GltfNodeDiff { children: Some(GltfRefsDelta::relocation(&refs(&base.document.nodes[p.parent].children), index, p.position)), ..Default::default() }), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(p: &GltfMoveNodeChildPayload, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
    if validate(p, base).is_err() {
        return Vec::new();
    }
    let Some(index) = base.document.nodes[p.parent].children.iter().position(|child| *child == p.child) else {
        return Vec::new();
    };
    vec![super::move_node_child::mutation(super::move_node_child::GltfMoveNodeChildPayload { parent: p.parent, child: p.child, position: index })]
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum MoveNodeChildMutation {
    Apply(GltfMoveNodeChildPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation(payload: GltfMoveNodeChildPayload) -> super::GltfMutation {
    super::GltfMutation::MoveNodeChild(MoveNodeChildMutation::Apply(payload))
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for MoveNodeChildMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "node-child", kind: "move-node-child", record: "MovedNodeChild" };

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
        semio_framework_ui_locale::LocalizedLabel::native("Move Node Child", "Kindknoten verschieben")
    }

    fn target(&self) -> Vec<String> {
        vec!["move-node-child".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️t045/🦀️.rs"]
mod case_t045;
//#endregion 🧪️Tests
