//! 🧬️ Direct change-node-transform mutation owner: payload, validation, typed diff, inverse, and outcomes.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::diff::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level_collections::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::structure_geometry::checked_index;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.change-node-transform.v1";
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct GltfTransformNodePayload {
    pub node: usize,
    pub transform: GltfNodeTransform,
}
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
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
pub fn inverse<Op>(p: &GltfTransformNodePayload, base: &GltfSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::change_node_transform::ChangeNodeTransformMutation>,
{
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
    vec![mutation(GltfTransformNodePayload { node: p.node, transform })]
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum ChangeNodeTransformMutation {
    Apply(GltfTransformNodePayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation<Op>(payload: GltfTransformNodePayload) -> Op
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::change_node_transform::ChangeNodeTransformMutation>,
{
    Op::from(ChangeNodeTransformMutation::Apply(payload))
}

impl<Op> protocol::MutationKind<GltfSnapshot, Op> for ChangeNodeTransformMutation
where
    Op: protocol::Mutation<GltfSnapshot, Diff = GltfDiff> + From<ChangeNodeTransformMutation>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "node-transform", kind: "change-node-transform", record: "ChangedNodeTransform" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change Node Transform", "Knotentransformation ändern")
    }

    fn target(&self) -> Vec<String> {
        vec!["change-node-transform".to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<GltfSnapshot, Op> for ChangeNodeTransformMutation
where
    Op: protocol::ColdMutation<GltfSnapshot, ReferenceDiff = GltfDiff> + From<ChangeNodeTransformMutation>,
{
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
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests

//#endregion 🧪️Tests
