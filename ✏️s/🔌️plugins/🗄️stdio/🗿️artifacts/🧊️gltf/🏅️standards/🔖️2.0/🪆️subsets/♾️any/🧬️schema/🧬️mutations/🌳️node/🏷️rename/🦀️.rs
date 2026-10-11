//! 🧬️ Direct change-node-name mutation owner.
use crate::schema::diff::*;
use crate::schema::modules::mutation_support::structure_geometry::checked_index;
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
use crate::schema::modules::mutation_support::top_level_collections::*;
use crate::GltfSnapshot;

//#region 🔖️Payload
pub const ID: &str = "s.stdio.gltf.mutation.change-node-name.v1";

/// 🕳️ `value` carries `#[value(required)]`: this derive, like `serde`'s, decodes a MISSING `Option<T>` key as `None` unless
/// the field says otherwise, and `required` is exactly the opt-out — a present wire key is mandatory even for an `Option<T>`,
/// so clearing the name (`{"value": null}`) and a wire that forgot the value never decode to the same payload.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct GltfChangeNodeNamePayload {
    pub node: u32,
    #[value(required)]
    pub value: Option<String>,
}
//#endregion 🔖️Payload

//#region ⚙️Validation
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn node_path(node: u32) -> String {
    format!("document/nodes/{node}/name")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn node_index(node: u32, base: &GltfSnapshot) -> Result<usize, GltfTopLevelMutationRejection> {
    let index = usize::try_from(node).map_err(|_| reject("gltf.mutation.index-out-of-range", "document/nodes", format!("index {node} is not representable on this platform")))?;
    checked_index(index, base.document.nodes.len(), "document/nodes")?;
    Ok(index)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfChangeNodeNamePayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    let index = node_index(payload.node, base)?;
    if base.document.nodes[index].name == payload.value {
        return Err(reject("gltf.mutation.no-observable-change", node_path(payload.node), "name already has the requested presence and value"));
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfChangeNodeNamePayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let node = node_index(p.node, base)?;
    Ok(GltfDiff { nodes: patch(node, GltfNodeDiff { name: Some(p.value.clone()), ..Default::default() }), ..Default::default() })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(p: &GltfChangeNodeNamePayload, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
    if validate(p, base).is_err() {
        return Vec::new();
    }
    let Ok(node) = node_index(p.node, base) else {
        return Vec::new();
    };
    vec![super::change_node_name::mutation(super::change_node_name::GltfChangeNodeNamePayload { node: p.node, value: base.document.nodes[node].name.clone() })]
}
//#endregion ⚙️Validation

//#region 🧬️Operation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase", deny_unknown_fields)]
pub enum ChangeNodeNameMutation {
    Apply(GltfChangeNodeNamePayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation(payload: GltfChangeNodeNamePayload) -> super::GltfMutation {
    super::GltfMutation::ChangeNodeName(ChangeNodeNameMutation::Apply(payload))
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for ChangeNodeNameMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "node-name", kind: "change-node-name", record: "ChangedNodeName" };

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
        semio_framework_ui_locale::LocalizedLabel::native("Change Node Name", "Knotenname ändern")
    }
    fn target(&self) -> Vec<String> {
        match self {
            Self::Apply(payload) => vec![node_path(payload.node)],
        }
    }
}
//#endregion 🧬️Operation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/✏️renames-the-root-f1e002/🦀️.rs"]
mod case_renames_the_root_f1e002;
//#endregion 🧪️Tests

#[cfg(test)]
#[path = "📜️contract/🧪️tests/🔬️unit/🦀️.rs"]
mod contract;
