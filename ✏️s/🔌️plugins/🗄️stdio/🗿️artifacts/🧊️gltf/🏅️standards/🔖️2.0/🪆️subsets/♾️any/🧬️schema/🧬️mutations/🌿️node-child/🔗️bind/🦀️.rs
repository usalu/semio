//! 🧬️ Direct bind-node-child mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::schema::diff::*;
use crate::schema::modules::mutation_support::top_level_collections::*;
use crate::schema::modules::mutation_support::structure_geometry::{checked_index, checked_position};
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
use crate::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.bind-node-child.v1";
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct GltfBindNodeChildPayload {
    pub parent: usize,
    pub child: usize,
    pub position: usize,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfBindNodeChildPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    checked_index(payload.parent, base.document.nodes.len(), "document/nodes")?;
    checked_index(payload.child, base.document.nodes.len(), "document/nodes")?;
    checked_position(payload.position, base.document.nodes[payload.parent].children.len(), "document/nodes/children")?;
    if payload.parent == payload.child || base.document.nodes[payload.parent].children.contains(&payload.child) {
        return Err(reject("gltf.mutation.invalid-child-link", format!("document/nodes/{}/children", payload.parent), "self and duplicate child links are forbidden"));
    }
    let mut pending = vec![payload.child];
    let mut seen = std::collections::BTreeSet::new();
    while let Some(node) = pending.pop() {
        if node == payload.parent {
            return Err(reject("gltf.mutation.node-cycle", format!("document/nodes/{}/children", payload.parent), "relationship closes a cycle"));
        }
        if seen.insert(node) {
            let current = base.document.nodes.get(node).ok_or_else(|| reject("gltf.mutation.invalid-reference", format!("document/nodes/{}", node), "child graph contains a missing node"))?;
            pending.extend(current.children.iter().copied());
        }
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfBindNodeChildPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    Ok(GltfDiff { nodes: patch(p.parent, GltfNodeDiff { children: Some(GltfRefsDelta::insertion(p.position, GltfRef(p.child))), ..Default::default() }), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(p: &GltfBindNodeChildPayload, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
    if validate(p, base).is_err() {
        return Vec::new();
    }
    vec![super::unbind_node_child::mutation(super::unbind_node_child::GltfUnbindNodeChildPayload { parent: p.parent, child: p.child })]
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum BindNodeChildMutation {
    Apply(GltfBindNodeChildPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation(payload: GltfBindNodeChildPayload) -> super::GltfMutation {
    super::GltfMutation::BindNodeChild(BindNodeChildMutation::Apply(payload))
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for BindNodeChildMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "bind", entity: "node-child", kind: "bind-node-child", record: "BoundNodeChild" };

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
        semio_framework_ui_locale::LocalizedLabel::native("Bind Node Child", "Kindknoten binden")
    }

    fn target(&self) -> Vec<String> {
        vec!["bind-node-child".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔗️adopts-the-child-aebf46/🦀️.rs"]
mod case_adopts_the_child_aebf46;
//#endregion 🧪️Tests

#[cfg(test)]
#[path = "📜️contract/🧪️tests/🔬️unit/🦀️.rs"]
mod contract;
