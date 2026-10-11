//! 🧬️ Direct bind-default-scene mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::schema::diff::*;
use crate::schema::modules::mutation_support::top_level_collections::*;
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
use crate::GltfSnapshot;
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
pub fn inverse(p: &GltfBindDefaultScenePayload, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
    if validate(p, base).is_err() {
        return Vec::new();
    }
    match base.document.scene {
        Some(current) if current == p.scene => Vec::new(),
        Some(current) => vec![super::bind_default_scene::mutation(super::bind_default_scene::GltfBindDefaultScenePayload { scene: current })],
        None => vec![super::unbind_default_scene::mutation(super::unbind_default_scene::GltfUnbindDefaultScenePayload {  })],
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
pub fn mutation(payload: GltfBindDefaultScenePayload) -> super::GltfMutation {
    super::GltfMutation::BindDefaultScene(BindDefaultSceneMutation::Apply(payload))
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for BindDefaultSceneMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "bind", entity: "default-scene", kind: "bind-default-scene", record: "BoundDefaultScene" };

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
        semio_framework_ui_locale::LocalizedLabel::native("Bind Default Scene", "Standardszene binden")
    }

    fn target(&self) -> Vec<String> {
        vec!["bind-default-scene".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🎞️moves-the-0e42fe/🦀️.rs"]
mod case_moves_the_0e42fe;
//#endregion 🧪️Tests
