//! 🧬️ Direct reorder-nodes mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::schema::diff::*;
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::schema::modules::mutation_support::top_level_collections::*;
use crate::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.reorder-nodes.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/nodes"];
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfReorderNodesPayload {
    pub order: Vec<usize>,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfReorderNodesPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    if payload.order.len() != base.document.nodes.len() || payload.order.iter().collect::<std::collections::BTreeSet<_>>().len() != payload.order.len() || payload.order.iter().any(|index| *index >= base.document.nodes.len()) {
        return Err(reject("gltf.mutation.invalid-permutation", "document/nodes", "order must contain every index once"));
    }
    if payload.order.iter().enumerate().all(|(index, value)| index == *value) {
        return Err(reject("gltf.mutation.no-observable-change", "document/nodes", "order already matches"));
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfReorderNodesPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let mut remap = after_reorder(&p.order);
    let mut diff = rewire(base, GltfTopLevelFamily::Nodes, &mut remap);
    let slot = diff.nodes.get_or_insert_with(Default::default);
    slot.modified.retain(|entry| p.order[entry.index] == entry.index);
    for (new, old) in p.order.iter().enumerate() {
        if new != *old {
            let mut item = base.document.nodes[*old].clone();
            item.children = item.children.iter().filter_map(|child| remap(*child)).collect();
            slot.removed.push(*old);
            slot.added.push(GltfAdded { index: new, item });
        }
    }
    slot.removed.sort_unstable();
    Ok(diff)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(p: &GltfReorderNodesPayload, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
    if validate(p, base).is_err() {
        return Vec::new();
    }
    vec![super::reorder_nodes::mutation(super::reorder_nodes::GltfReorderNodesPayload { order: inverse_order(&p.order) })]
}

//#region 🧬️DirectMutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum ReorderNodesMutation {
    Apply(GltfReorderNodesPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation(payload: GltfReorderNodesPayload) -> super::GltfMutation {
    super::GltfMutation::ReorderNodes(ReorderNodesMutation::Apply(payload))
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for ReorderNodesMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "reorder", entity: "nodes", kind: "reorder-nodes", record: "ReorderedNodes" };

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
        semio_framework_ui_locale::LocalizedLabel::native("Reorder Nodes", "Knoten umordnen")
    }

    fn target(&self) -> Vec<String> {
        vec!["reorder-nodes".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔢️flips-the-two-f25cfb/🦀️.rs"]
mod case_flips_the_two_f25cfb;
//#endregion 🧪️Tests
