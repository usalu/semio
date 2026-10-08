//! 🧬️ Direct reorder-node-children mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::schema::diff::*;
use crate::schema::modules::mutation_support::top_level_collections::*;
use crate::schema::modules::mutation_support::structure_geometry::checked_index;
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
use crate::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.reorder-node-children.v1";
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfReorderNodeChildrenPayload {
    pub parent: usize,
    pub order: Vec<usize>,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfReorderNodeChildrenPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    checked_index(payload.parent, base.document.nodes.len(), "document/nodes")?;
    let children = &base.document.nodes[payload.parent].children;
    if payload.order.len() != children.len() || payload.order.iter().any(|child| !children.contains(child)) || {
        let mut order = payload.order.clone();
        order.sort_unstable();
        order.dedup();
        order.len() != children.len()
    } {
        return Err(reject("gltf.mutation.invalid-permutation", "document/nodes/children", "order must contain every child identity once"));
    }
    if payload.order == *children {
        return Err(reject("gltf.mutation.no-observable-change", "document/nodes/children", "reorder must change order"));
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfReorderNodeChildrenPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    Ok(GltfDiff { nodes: patch(p.parent, GltfNodeDiff { children: Some(p.order.clone()), ..Default::default() }), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(p: &GltfReorderNodeChildrenPayload, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
    if validate(p, base).is_err() {
        return Vec::new();
    }
    vec![super::reorder_node_children::mutation(super::reorder_node_children::GltfReorderNodeChildrenPayload { parent: p.parent, order: base.document.nodes[p.parent].children.clone() })]
}

//#region 🧬️DirectMutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum ReorderNodeChildrenMutation {
    Apply(GltfReorderNodeChildrenPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation(payload: GltfReorderNodeChildrenPayload) -> super::GltfMutation {
    super::GltfMutation::ReorderNodeChildren(ReorderNodeChildrenMutation::Apply(payload))
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for ReorderNodeChildrenMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "reorder", entity: "node-children", kind: "reorder-node-children", record: "ReorderedNodeChildren" };

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
        semio_framework_ui_locale::LocalizedLabel::native("Reorder Node Children", "Kindknoten umordnen")
    }

    fn target(&self) -> Vec<String> {
        vec!["reorder-node-children".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🚫️refuses-an-order-dd1195/🦀️.rs"]
mod case_refuses_an_order_dd1195;
//#endregion 🧪️Tests
