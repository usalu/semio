//! 🧬️ Direct reorder-used-extensions mutation owner: payload, validation, typed diff, inverse, and outcomes.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::diff::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level_collections::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.reorder-used-extensions.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/extensionsUsed"];
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct GltfReorderUsedExtensionsPayload {
    pub order: Vec<String>,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfReorderUsedExtensionsPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    if payload.order.len() != base.document.extensions_used.len() || payload.order.iter().collect::<std::collections::BTreeSet<_>>().len() != payload.order.len() || payload.order.iter().any(|value| !base.document.extensions_used.contains(value)) {
        return Err(reject("gltf.mutation.invalid-permutation", "document/extensionsUsed", "order must contain every declaration exactly once"));
    }
    if payload.order == base.document.extensions_used {
        return Err(reject("gltf.mutation.no-observable-change", "document/extensionsUsed", "order already matches"));
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfReorderUsedExtensionsPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    Ok(GltfDiff { extensions_used: Some(GltfStringsDelta::rows(Vec::new(), Vec::new(), moves_to_keys(&base.document.extensions_used, &p.order))), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(p: &GltfReorderUsedExtensionsPayload, base: &GltfSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::remove_used_extension::RemoveUsedExtensionMutation>,
{
    if validate(p, base).is_err() {
        return Vec::new();
    }
    vec![mutation(GltfReorderUsedExtensionsPayload { order: base.document.extensions_used.clone() })]
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum ReorderUsedExtensionsMutation {
    Apply(GltfReorderUsedExtensionsPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation<Op>(payload: GltfReorderUsedExtensionsPayload) -> Op
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::reorder_used_extensions::ReorderUsedExtensionsMutation>,
{
    Op::from(ReorderUsedExtensionsMutation::Apply(payload))
}

impl<Op> protocol::MutationKind<GltfSnapshot, Op> for ReorderUsedExtensionsMutation
where
    Op: protocol::Mutation<GltfSnapshot, Diff = GltfDiff> + From<ReorderUsedExtensionsMutation>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "reorder", entity: "used-extensions", kind: "reorder-used-extensions", record: "ReorderedUsedExtensions" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Reorder Used Extensions", "Verwendete Erweiterungen umordnen")
    }

    fn target(&self) -> Vec<String> {
        vec!["reorder-used-extensions".to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<GltfSnapshot, Op> for ReorderUsedExtensionsMutation
where
    Op: protocol::ColdMutation<GltfSnapshot, ReferenceDiff = GltfDiff> + From<ReorderUsedExtensionsMutation>,
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
