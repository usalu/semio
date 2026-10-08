//! 🧬️ Direct change-node-extra-data mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::schema::diff::*;
use crate::schema::modules::mutation_support::top_level_collections::*;
use crate::schema::snapshot::*;
use crate::schema::modules::mutation_support::structure_geometry::checked_index;
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
use crate::schema::snapshot::GltfJson;
use crate::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.change-node-extra-data.v1";
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "state", rename_all = "camelCase")]
pub enum GltfDataPresence {
    Absent,
    Present { value: GltfJson },
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfChangeNodeExtraDataPayload {
    pub node: usize,
    pub data: GltfDataPresence,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfChangeNodeExtraDataPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    checked_index(payload.node, base.document.nodes.len(), "document/nodes")?;
    let unchanged = match &payload.data {
        GltfDataPresence::Absent => base.document.nodes[payload.node].extras.is_none(),
        GltfDataPresence::Present { value } => base.document.nodes[payload.node].extras.as_ref() == Some(value),
    };
    if unchanged {
        return Err(reject("gltf.mutation.no-observable-change", format!("document/nodes/{}/extras", payload.node), "extras already has the requested presence and value"));
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn requested(payload: &GltfChangeNodeExtraDataPayload) -> Option<GltfJson> {
    match &payload.data {
        GltfDataPresence::Absent => None,
        GltfDataPresence::Present { value } => Some(value.clone()),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn presence(value: &Option<GltfJson>) -> GltfDataPresence {
    match value {
        None => GltfDataPresence::Absent,
        Some(value) => GltfDataPresence::Present { value: value.clone() },
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfChangeNodeExtraDataPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let current = &base.document.nodes[p.node].extras;
    Ok(GltfDiff { nodes: patch(p.node, GltfNodeDiff { extras: (current != &requested(p)).then(|| requested(p)), ..Default::default() }), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(p: &GltfChangeNodeExtraDataPayload, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
    if validate(p, base).is_err() {
        return Vec::new();
    }
    let current = &base.document.nodes[p.node].extras;
    if current == &requested(p) {
        return Vec::new();
    }
    vec![super::change_node_extra_data::mutation(super::change_node_extra_data::GltfChangeNodeExtraDataPayload { node: p.node, data: presence(current) })]
}

//#region 🧬️DirectMutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum ChangeNodeExtraDataMutation {
    Apply(GltfChangeNodeExtraDataPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation(payload: GltfChangeNodeExtraDataPayload) -> super::GltfMutation {
    super::GltfMutation::ChangeNodeExtraData(ChangeNodeExtraDataMutation::Apply(payload))
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for ChangeNodeExtraDataMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "node-extra-data", kind: "change-node-extra-data", record: "ChangedNodeExtraData" };

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
        semio_framework_ui_locale::LocalizedLabel::native("Change Node Extra Data", "Zusatzdaten des Knotens ändern")
    }

    fn target(&self) -> Vec<String> {
        vec!["change-node-extra-data".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/📝️attaches-a-unit-5159f9/🦀️.rs"]
mod case_attaches_a_unit_5159f9;
//#endregion 🧪️Tests

#[cfg(test)]
#[path = "📜️contract/🧪️tests/🔬️unit/🦀️.rs"]
mod contract;
