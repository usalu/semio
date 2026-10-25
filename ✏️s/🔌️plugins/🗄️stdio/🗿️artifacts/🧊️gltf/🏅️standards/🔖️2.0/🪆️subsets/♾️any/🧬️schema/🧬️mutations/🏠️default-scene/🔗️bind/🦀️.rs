//! 🧬️ Direct bind-default-scene mutation owner: payload, validation, typed diff, inverse, and outcomes.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::diff::*;
#[cfg(test)] use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level_collections::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.bind-default-scene.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/scene"];
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct GltfBindDefaultScenePayload {
    pub scene: usize,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfBindDefaultScenePayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    if payload.scene >= base.document.scenes.len() {
        return Err(reject("gltf.mutation.index-out-of-range", "document/scenes", "scene must exist"));
    }
    if Some(payload.scene) == base.document.scene {
        return Err(reject("gltf.mutation.no-observable-change", "document/scene", "scene is already default"));
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfBindDefaultScenePayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    Ok(GltfDiff { scene: Some(Some(p.scene)), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(p: &GltfBindDefaultScenePayload, base: &GltfSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::unbind_default_scene::UnbindDefaultSceneMutation> + From<crate::standards::v2_0::subsets::any::schema::mutations::bind_default_scene::BindDefaultSceneMutation>,
{
    if validate(p, base).is_err() {
        return Vec::new();
    }
    match base.document.scene {
        Some(current) if current == p.scene => Vec::new(),
        Some(current) => vec![mutation(GltfBindDefaultScenePayload { scene: current })],
        None => vec![crate::standards::v2_0::subsets::any::schema::mutations::unbind_default_scene::mutation(crate::standards::v2_0::subsets::any::schema::mutations::unbind_default_scene::GltfUnbindDefaultScenePayload {  })],
    }
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum BindDefaultSceneMutation {
    Apply(GltfBindDefaultScenePayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation<Op>(payload: GltfBindDefaultScenePayload) -> Op
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::bind_default_scene::BindDefaultSceneMutation>,
{
    Op::from(BindDefaultSceneMutation::Apply(payload))
}

impl<Op> protocol::MutationKind<GltfSnapshot, Op> for BindDefaultSceneMutation
where
    Op: protocol::Mutation<GltfSnapshot, Diff = GltfDiff> + From<BindDefaultSceneMutation>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "bind", entity: "default-scene", kind: "bind-default-scene", record: "BoundDefaultScene" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Bind Default Scene", "Standardszene binden")
    }

    fn target(&self) -> Vec<String> {
        vec!["bind-default-scene".to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<GltfSnapshot, Op> for BindDefaultSceneMutation
where
    Op: protocol::ColdMutation<GltfSnapshot, ReferenceDiff = GltfDiff> + From<BindDefaultSceneMutation> + From<crate::standards::v2_0::subsets::any::schema::mutations::unbind_default_scene::UnbindDefaultSceneMutation>,
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
