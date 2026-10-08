//! 🧬️ Direct change-node-transform mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::schema::diff::*;
use crate::schema::modules::mutation_support::top_level_collections::*;
use crate::schema::modules::mutation_support::structure_geometry::checked_index;
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
use crate::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.change-node-transform.v1";
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfTransformNodePayload {
    pub node: usize,
    pub transform: GltfNodeTransform,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum GltfNodeTransform {
    Matrix { matrix: [f64; 16] },
    Trs { translation: Option<[f64; 3]>, rotation: Option<[f64; 4]>, scale: Option<[f64; 3]> },
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfTransformNodePayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    checked_index(payload.node, base.document.nodes.len(), "document/nodes")?;
    let finite = match &payload.transform {
        GltfNodeTransform::Matrix { matrix } => matrix.iter().all(|value| value.is_finite()),
        GltfNodeTransform::Trs { translation, rotation, scale } => translation.iter().flatten().chain(rotation.iter().flatten()).chain(scale.iter().flatten()).all(|value| value.is_finite()),
    };
    if !finite {
        return Err(reject("gltf.mutation.invalid-transform", format!("document/nodes/{}/transform", payload.node), "transform values must be finite"));
    }
    let node = &base.document.nodes[payload.node];
    if node.matrix.is_some() && (node.translation.is_some() || node.rotation.is_some() || node.scale.is_some()) {
        return Err(reject("gltf.mutation.invalid-transform", format!("document/nodes/{}/transform", payload.node), "node holds both a matrix and translation/rotation/scale"));
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfTransformNodePayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let (matrix, translation, rotation, scale) = match &p.transform {
        GltfNodeTransform::Matrix { matrix } => (Some(*matrix), None, None, None),
        GltfNodeTransform::Trs { translation, rotation, scale } => (None, *translation, *rotation, *scale),
    };
    let node = &base.document.nodes[p.node];
    Ok(GltfDiff { nodes: patch(p.node, GltfNodeDiff { matrix: (node.matrix != matrix).then_some(matrix), translation: (node.translation != translation).then_some(translation), rotation: (node.rotation != rotation).then_some(rotation), scale: (node.scale != scale).then_some(scale), ..Default::default() }), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(p: &GltfTransformNodePayload, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
    if validate(p, base).is_err() {
        return Vec::new();
    }
    let (matrix, translation, rotation, scale) = match &p.transform {
        GltfNodeTransform::Matrix { matrix } => (Some(*matrix), None, None, None),
        GltfNodeTransform::Trs { translation, rotation, scale } => (None, *translation, *rotation, *scale),
    };
    let node = &base.document.nodes[p.node];
    if node.matrix == matrix && node.translation == translation && node.rotation == rotation && node.scale == scale {
        return Vec::new();
    }
    let transform = match node.matrix {
        Some(matrix) => GltfNodeTransform::Matrix { matrix },
        None => GltfNodeTransform::Trs { translation: node.translation, rotation: node.rotation, scale: node.scale },
    };
    vec![super::change_node_transform::mutation(super::change_node_transform::GltfTransformNodePayload { node: p.node, transform })]
}

//#region 🧬️DirectMutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum ChangeNodeTransformMutation {
    Apply(GltfTransformNodePayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation(payload: GltfTransformNodePayload) -> super::GltfMutation {
    super::GltfMutation::ChangeNodeTransform(ChangeNodeTransformMutation::Apply(payload))
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for ChangeNodeTransformMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "node-transform", kind: "change-node-transform", record: "ChangedNodeTransform" };

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
        semio_framework_ui_locale::LocalizedLabel::native("Change Node Transform", "Knotentransformation ändern")
    }

    fn target(&self) -> Vec<String> {
        vec!["change-node-transform".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔀️replaces-a-f09fa4/🦀️.rs"]
mod case_replaces_a_f09fa4;
//#endregion 🧪️Tests
