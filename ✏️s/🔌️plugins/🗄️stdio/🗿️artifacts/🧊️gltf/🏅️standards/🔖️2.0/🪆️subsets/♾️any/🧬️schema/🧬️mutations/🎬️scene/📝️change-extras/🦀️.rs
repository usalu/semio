//! 🧬️ Direct change-scene-extra-data mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::schema::diff::*;
use crate::schema::modules::mutation_support::top_level_collections::*;
use crate::schema::modules::mutation_support::structure_geometry::checked_index;
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::schema::modules::mutation_support::top_level::GltfTopLevelMutationRejection;
use crate::schema::snapshot::*;
use crate::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.change-scene-extra-data.v1";
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "state", rename_all = "camelCase")]
pub enum GltfDataPresence {
    Absent,
    Present { value: GltfJson },
}
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct GltfChangeSceneExtraDataPayload {
    pub scene: usize,
    pub data: GltfDataPresence,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfChangeSceneExtraDataPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    checked_index(payload.scene, base.document.scenes.len(), "document/scenes")?;
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn requested(payload: &GltfChangeSceneExtraDataPayload) -> Option<GltfJson> {
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
pub fn plan(p: &GltfChangeSceneExtraDataPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let current = &base.document.scenes[p.scene].extras;
    Ok(GltfDiff { scenes: patch(p.scene, GltfSceneDiff { extras: (current != &requested(p)).then(|| requested(p)), ..Default::default() }), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(p: &GltfChangeSceneExtraDataPayload, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
    if validate(p, base).is_err() {
        return Vec::new();
    }
    let current = &base.document.scenes[p.scene].extras;
    if current == &requested(p) {
        return Vec::new();
    }
    vec![super::change_scene_extra_data::mutation(super::change_scene_extra_data::GltfChangeSceneExtraDataPayload { scene: p.scene, data: presence(current) })]
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum ChangeSceneExtraDataMutation {
    Apply(GltfChangeSceneExtraDataPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation(payload: GltfChangeSceneExtraDataPayload) -> super::GltfMutation {
    super::GltfMutation::ChangeSceneExtraData(ChangeSceneExtraDataMutation::Apply(payload))
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for ChangeSceneExtraDataMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "scene-extra-data", kind: "change-scene-extra-data", record: "ChangedSceneExtraData" };

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
        semio_framework_ui_locale::LocalizedLabel::native("Change Scene Extra Data", "Zusatzdaten der Szene ändern")
    }

    fn target(&self) -> Vec<String> {
        vec!["change-scene-extra-data".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/📏️attaches-a-units-e2653c/🦀️.rs"]
mod case_attaches_a_units_e2653c;
//#endregion 🧪️Tests
