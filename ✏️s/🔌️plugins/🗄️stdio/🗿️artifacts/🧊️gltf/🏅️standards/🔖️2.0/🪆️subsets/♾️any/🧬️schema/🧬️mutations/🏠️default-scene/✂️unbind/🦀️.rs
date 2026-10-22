//! 🧬️ Direct unbind-default-scene mutation owner: payload, validation, typed diff, inverse, and outcomes.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::diff::*;
#[cfg(test)] use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level_collections::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.unbind-default-scene.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/scene"];
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct GltfUnbindDefaultScenePayload {}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(_payload: &GltfUnbindDefaultScenePayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    if base.document.scene.is_none() {
        return Err(reject("gltf.mutation.relation-absent", "document/scene", "no default scene is bound"));
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfUnbindDefaultScenePayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    Ok(GltfDiff { scene: Some(None), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(p: &GltfUnbindDefaultScenePayload, base: &GltfSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::remove_used_extension::RemoveUsedExtensionMutation>,
{
    if validate(p, base).is_err() {
        return Vec::new();
    }
    match base.document.scene {
        Some(current) => vec![crate::standards::v2_0::subsets::any::schema::mutations::bind_default_scene::mutation(crate::standards::v2_0::subsets::any::schema::mutations::bind_default_scene::GltfBindDefaultScenePayload { scene: current })],
        None => Vec::new(),
    }
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum UnbindDefaultSceneMutation {
    Apply(GltfUnbindDefaultScenePayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation<Op>(payload: GltfUnbindDefaultScenePayload) -> Op
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::unbind_default_scene::UnbindDefaultSceneMutation>,
{
    Op::from(UnbindDefaultSceneMutation::Apply(payload))
}

impl<Op> protocol::MutationKind<GltfSnapshot, Op> for UnbindDefaultSceneMutation
where
    Op: protocol::Mutation<GltfSnapshot, Diff = GltfDiff> + From<UnbindDefaultSceneMutation>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "unbind", entity: "default-scene", kind: "unbind-default-scene", record: "UnboundDefaultScene" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Unbind Default Scene", "Bindung der Standardszene aufheben")
    }

    fn target(&self) -> Vec<String> {
        vec!["unbind-default-scene".to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<GltfSnapshot, Op> for UnbindDefaultSceneMutation
where
    Op: protocol::ColdMutation<GltfSnapshot, ReferenceDiff = GltfDiff> + From<UnbindDefaultSceneMutation>,
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
