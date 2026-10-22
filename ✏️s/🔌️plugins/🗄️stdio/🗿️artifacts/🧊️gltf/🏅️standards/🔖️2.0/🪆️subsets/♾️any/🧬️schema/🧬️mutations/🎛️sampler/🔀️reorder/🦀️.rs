//! 🧬️ Direct reorder-samplers mutation owner: payload, validation, typed diff, inverse, and outcomes.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::diff::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level_collections::*;
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.reorder-samplers.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/samplers"];
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct GltfReorderSamplersPayload {
    pub order: Vec<usize>,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfReorderSamplersPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    if payload.order.len() != base.document.samplers.len() || payload.order.iter().collect::<std::collections::BTreeSet<_>>().len() != payload.order.len() || payload.order.iter().any(|index| *index >= base.document.samplers.len()) {
        return Err(reject("gltf.mutation.invalid-permutation", "document/samplers", "order must contain every index once"));
    }
    if payload.order.iter().enumerate().all(|(index, value)| index == *value) {
        return Err(reject("gltf.mutation.no-observable-change", "document/samplers", "order already matches"));
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfReorderSamplersPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let mut diff = rewire(base, GltfTopLevelFamily::Samplers, &mut after_reorder(&p.order));
    let slot = diff.samplers.get_or_insert_with(Default::default);
    for (new, old) in p.order.iter().enumerate() {
        if new != *old {
            slot.removed.push(*old);
            slot.added.push(GltfAdded { index: new, item: base.document.samplers[*old].clone() });
        }
    }
    slot.removed.sort_unstable();
    Ok(diff)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(p: &GltfReorderSamplersPayload, base: &GltfSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::remove_used_extension::RemoveUsedExtensionMutation>,
{
    if validate(p, base).is_err() {
        return Vec::new();
    }
    vec![mutation(GltfReorderSamplersPayload { order: inverse_order(&p.order) })]
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum ReorderSamplersMutation {
    Apply(GltfReorderSamplersPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation<Op>(payload: GltfReorderSamplersPayload) -> Op
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::reorder_samplers::ReorderSamplersMutation>,
{
    Op::from(ReorderSamplersMutation::Apply(payload))
}

impl<Op> protocol::MutationKind<GltfSnapshot, Op> for ReorderSamplersMutation
where
    Op: protocol::Mutation<GltfSnapshot, Diff = GltfDiff> + From<ReorderSamplersMutation>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "reorder", entity: "samplers", kind: "reorder-samplers", record: "ReorderedSamplers" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Reorder Samplers", "Sampler umordnen")
    }

    fn target(&self) -> Vec<String> {
        vec!["reorder-samplers".to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<GltfSnapshot, Op> for ReorderSamplersMutation
where
    Op: protocol::ColdMutation<GltfSnapshot, ReferenceDiff = GltfDiff> + From<ReorderSamplersMutation>,
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
