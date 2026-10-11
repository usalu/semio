//! 🧬️ Direct change-primitive-topology-mode mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::schema::diff::*;
use crate::schema::modules::mutation_support::top_level_collections::*;
use crate::schema::modules::mutation_support::structure_geometry::checked_index;
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
use crate::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.change-primitive-topology-mode.v1";
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct GltfChangePrimitiveTopologyModePayload {
    pub mesh: usize,
    pub primitive: usize,
    #[value(required)]
    pub mode: Option<u64>,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfChangePrimitiveTopologyModePayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    checked_index(payload.mesh, base.document.meshes.len(), "document/meshes")?;
    checked_index(payload.primitive, base.document.meshes[payload.mesh].primitives.len(), "document/meshes/primitives")?;
    if payload.mode.is_some_and(|mode| mode > 6) {
        return Err(reject("gltf.mutation.invalid-topology-mode", "document/meshes/primitives/mode", "mode must be in the glTF topology domain"));
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfChangePrimitiveTopologyModePayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let value = p.mode;
    Ok(GltfDiff { meshes: primitive_patch(p.mesh, p.primitive, GltfPrimitiveDiff { mode: (value != base.document.meshes[p.mesh].primitives[p.primitive].mode).then(|| value), ..Default::default() }), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(p: &GltfChangePrimitiveTopologyModePayload, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
    if validate(p, base).is_err() {
        return Vec::new();
    }
    if base.document.meshes[p.mesh].primitives[p.primitive].mode == p.mode {
        return Vec::new();
    }
    vec![super::change_primitive_topology_mode::mutation(super::change_primitive_topology_mode::GltfChangePrimitiveTopologyModePayload { mesh: p.mesh, primitive: p.primitive, mode: base.document.meshes[p.mesh].primitives[p.primitive].mode })]
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum ChangePrimitiveTopologyModeMutation {
    Apply(GltfChangePrimitiveTopologyModePayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation(payload: GltfChangePrimitiveTopologyModePayload) -> super::GltfMutation {
    super::GltfMutation::ChangePrimitiveTopologyMode(ChangePrimitiveTopologyModeMutation::Apply(payload))
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for ChangePrimitiveTopologyModeMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "primitive-topology-mode", kind: "change-primitive-topology-mode", record: "ChangedPrimitiveTopologyMode" };

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
        semio_framework_ui_locale::LocalizedLabel::native("Change Primitive Topology Mode", "Topologiemodus des Primitivs ändern")
    }

    fn target(&self) -> Vec<String> {
        vec!["change-primitive-topology-mode".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/📐️triangle-81bc44/🦀️.rs"]
mod case_triangle_81bc44;
//#endregion 🧪️Tests
