//! 🧬️ Direct reorder-primitives mutation owner: payload, validation, typed diff, inverse, and outcomes.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::diff::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level_collections::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::structure_geometry::checked_index;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.reorder-primitives.v1";
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct GltfReorderPrimitivesPayload {
    pub mesh: usize,
    pub order: Vec<usize>,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfReorderPrimitivesPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    checked_index(payload.mesh, base.document.meshes.len(), "document/meshes")?;
    let length = base.document.meshes[payload.mesh].primitives.len();
    if payload.order.len() != length || payload.order.iter().any(|index| *index >= length) || {
        let mut order = payload.order.clone();
        order.sort_unstable();
        order.dedup();
        order.len() != length
    } {
        return Err(reject("gltf.mutation.invalid-permutation", "document/meshes/primitives", "order must contain each primitive once"));
    }
    if payload.order.iter().enumerate().all(|(index, value)| *value == index) {
        return Err(reject("gltf.mutation.no-observable-change", "document/meshes/primitives", "reorder must change order"));
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfReorderPrimitivesPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let moved: Vec<(usize, usize)> = p.order.iter().copied().enumerate().filter(|(new, old)| new != old).collect();
    let mut removed: Vec<usize> = moved.iter().map(|(_, old)| *old).collect();
    removed.sort_unstable();
    let added = moved.iter().map(|(new, old)| GltfAdded { index: *new, item: base.document.meshes[p.mesh].primitives[*old].clone() }).collect();
    Ok(GltfDiff { meshes: primitives_rows(p.mesh, GltfPrimitivesDiff { removed, added, ..Default::default() }), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(p: &GltfReorderPrimitivesPayload, base: &GltfSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::reorder_primitives::ReorderPrimitivesMutation>,
{
    if validate(p, base).is_err() {
        return Vec::new();
    }
    vec![mutation(GltfReorderPrimitivesPayload { mesh: p.mesh, order: inverse_order(&p.order) })]
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum ReorderPrimitivesMutation {
    Apply(GltfReorderPrimitivesPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation<Op>(payload: GltfReorderPrimitivesPayload) -> Op
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::reorder_primitives::ReorderPrimitivesMutation>,
{
    Op::from(ReorderPrimitivesMutation::Apply(payload))
}

impl<Op> protocol::MutationKind<GltfSnapshot, Op> for ReorderPrimitivesMutation
where
    Op: protocol::Mutation<GltfSnapshot, Diff = GltfDiff> + From<ReorderPrimitivesMutation>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "reorder", entity: "primitives", kind: "reorder-primitives", record: "ReorderedPrimitives" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Reorder Primitives", "Primitive umordnen")
    }

    fn target(&self) -> Vec<String> {
        vec!["reorder-primitives".to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<GltfSnapshot, Op> for ReorderPrimitivesMutation
where
    Op: protocol::ColdMutation<GltfSnapshot, ReferenceDiff = GltfDiff> + From<ReorderPrimitivesMutation>,
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
