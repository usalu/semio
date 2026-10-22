//! 🧬️ Direct delete-sampler mutation owner: payload, validation, typed diff, inverse, and outcomes.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::diff::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level_collections::*;
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.delete-sampler.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/samplers"];
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct GltfDeleteSamplerPayload {
    pub index: usize,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfDeleteSamplerPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    if payload.index >= base.document.samplers.len() {
        return Err(reject("gltf.mutation.index-out-of-range", "document/samplers", "index must address an item"));
    }
    require_unreferenced(base, GltfTopLevelFamily::Samplers, payload.index)?;
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfDeleteSamplerPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let mut diff = rewire(base, GltfTopLevelFamily::Samplers, &mut after_delete(p.index));
    let slot = diff.samplers.get_or_insert_with(Default::default);
    slot.removed.push(p.index);
    Ok(diff)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(p: &GltfDeleteSamplerPayload, base: &GltfSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::remove_used_extension::RemoveUsedExtensionMutation>,
{
    if validate(p, base).is_err() {
        return Vec::new();
    }
    let mut rows = vec![crate::standards::v2_0::subsets::any::schema::mutations::create_sampler::mutation(crate::standards::v2_0::subsets::any::schema::mutations::create_sampler::GltfCreateSamplerPayload { position: p.index, sampler: Some(Box::new(base.document.samplers[p.index].clone())) })];

    rows.reverse();
    rows
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum DeleteSamplerMutation {
    Apply(GltfDeleteSamplerPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation<Op>(payload: GltfDeleteSamplerPayload) -> Op
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::delete_sampler::DeleteSamplerMutation>,
{
    Op::from(DeleteSamplerMutation::Apply(payload))
}

impl<Op> protocol::MutationKind<GltfSnapshot, Op> for DeleteSamplerMutation
where
    Op: protocol::Mutation<GltfSnapshot, Diff = GltfDiff> + From<DeleteSamplerMutation>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "sampler", kind: "delete-sampler", record: "DeletedSampler" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Delete Sampler", "Sampler löschen")
    }

    fn target(&self) -> Vec<String> {
        vec!["delete-sampler".to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<GltfSnapshot, Op> for DeleteSamplerMutation
where
    Op: protocol::ColdMutation<GltfSnapshot, ReferenceDiff = GltfDiff> + From<DeleteSamplerMutation>,
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
