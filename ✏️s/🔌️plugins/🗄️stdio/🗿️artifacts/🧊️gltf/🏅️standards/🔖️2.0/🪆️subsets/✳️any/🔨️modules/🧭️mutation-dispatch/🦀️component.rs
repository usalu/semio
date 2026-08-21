//! 🧭️ Open GLTF mutation runtime: immutable descriptor registry and typed envelopes.

use crate::artifacts::gltf::schema::mutations::{gltf_mutation_leaf_descriptors, GltfMutationLeafDescriptor, GltfMutationLeafError, GltfMutationLeafPlan};
use crate::artifacts::gltf::GltfSnapshot;
use protocol::{Mutation, MutationApplyError, MutationDiff, MutationOutcome};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::OnceLock;

//#region 🔖️Envelope
pub const GLTF_MUTATION_MAX_COMMAND_ID_BYTES: usize = 160;
pub const GLTF_MUTATION_MAX_PAYLOAD_BYTES: usize = 64 * 1024;
pub const GLTF_MUTATION_MAX_DIFF_ENVELOPES: usize = 256;
pub const GLTF_MUTATION_MAX_TOUCHED_PATHS: usize = 64;
pub const GLTF_MUTATION_MAX_TOUCHED_PATH_BYTES: usize = 512;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GltfMutationPhase {
    Mutation,
    Diff,
    Inverse,
}

impl GltfMutationPhase {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn binary_tag(self) -> u8 {
        match self {
            Self::Mutation => 1,
            Self::Diff => 2,
            Self::Inverse => 3,
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn from_binary_tag(tag: u8) -> Result<Self, GltfMutationRegistryError> {
        match tag {
            1 => Ok(Self::Mutation),
            2 => Ok(Self::Diff),
            3 => Ok(Self::Inverse),
            _ => Err(GltfMutationRegistryError::Malformed(format!("unknown mutation phase tag {tag}"))),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GltfMutationEnvelope {
    pub command_id: String,
    pub version: u32,
    pub phase: GltfMutationPhase,
    pub payload: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GltfDiffEnvelope {
    pub command_id: String,
    pub version: u32,
    pub phase: GltfMutationPhase,
    pub payload: Vec<u8>,
    pub touched_paths: Vec<String>,
}
//#endregion 🔖️Envelope

//#region 🔖️Registry
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GltfMutationRegistryError {
    DuplicateCommand(String),
    UnknownCommand(String),
    StaleVersion { command_id: String, expected: u32, actual: u32 },
    InvalidPhase { command_id: String, phase: GltfMutationPhase },
    BudgetExceeded(&'static str),
    Malformed(String),
    Leaf(GltfMutationLeafError),
}

impl From<GltfMutationLeafError> for GltfMutationRegistryError {
    fn from(error: GltfMutationLeafError) -> Self {
        Self::Leaf(error)
    }
}

impl std::fmt::Display for GltfMutationRegistryError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DuplicateCommand(id) => write!(formatter, "duplicate GLTF mutation command {id}"),
            Self::UnknownCommand(id) => write!(formatter, "unknown GLTF mutation command {id}"),
            Self::StaleVersion { command_id, expected, actual } => write!(formatter, "stale GLTF mutation command {command_id}: expected v{expected}, got v{actual}"),
            Self::InvalidPhase { command_id, phase } => write!(formatter, "invalid GLTF mutation phase {phase:?} for {command_id}"),
            Self::BudgetExceeded(what) => write!(formatter, "GLTF mutation budget exceeded: {what}"),
            Self::Malformed(detail) => write!(formatter, "malformed GLTF mutation: {detail}"),
            Self::Leaf(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for GltfMutationRegistryError {}

pub struct GltfMutationRegistry {
    descriptors: BTreeMap<&'static str, GltfMutationLeafDescriptor>,
}

impl GltfMutationRegistry {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_descriptors(descriptors: impl IntoIterator<Item = GltfMutationLeafDescriptor>) -> Result<Self, GltfMutationRegistryError> {
        let mut registered = BTreeMap::new();
        for descriptor in descriptors {
            if descriptor.command_id.is_empty() || descriptor.command_id.len() > GLTF_MUTATION_MAX_COMMAND_ID_BYTES || descriptor.version == 0 {
                return Err(GltfMutationRegistryError::Malformed("descriptor has an invalid canonical id or version".into()));
            }
            if registered.insert(descriptor.command_id, descriptor).is_some() {
                return Err(GltfMutationRegistryError::DuplicateCommand(descriptor.command_id.into()));
            }
        }
        Ok(Self { descriptors: registered })
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn descriptor(&self, command_id: &str, version: u32) -> Result<&GltfMutationLeafDescriptor, GltfMutationRegistryError> {
        let descriptor = self.descriptors.get(command_id).ok_or_else(|| GltfMutationRegistryError::UnknownCommand(command_id.into()))?;
        if descriptor.version != version {
            return Err(GltfMutationRegistryError::StaleVersion { command_id: command_id.into(), expected: descriptor.version, actual: version });
        }
        Ok(descriptor)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn validate_envelope(&self, envelope: &GltfMutationEnvelope) -> Result<(), GltfMutationRegistryError> {
        if envelope.command_id.is_empty() || envelope.command_id.len() > GLTF_MUTATION_MAX_COMMAND_ID_BYTES {
            return Err(GltfMutationRegistryError::BudgetExceeded("command id"));
        }
        if envelope.payload.len() > GLTF_MUTATION_MAX_PAYLOAD_BYTES {
            return Err(GltfMutationRegistryError::BudgetExceeded("payload"));
        }
        self.descriptor(&envelope.command_id, envelope.version)?;
        Ok(())
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validate_plan(&self, plan: &GltfMutationLeafPlan) -> Result<(), GltfMutationRegistryError> {
        if plan.diff_payload.len() > GLTF_MUTATION_MAX_PAYLOAD_BYTES || plan.inverse_payload.len() > GLTF_MUTATION_MAX_PAYLOAD_BYTES {
            return Err(GltfMutationRegistryError::BudgetExceeded("planned payload"));
        }
        if plan.touched_paths.len() > GLTF_MUTATION_MAX_TOUCHED_PATHS || plan.touched_paths.iter().any(|path| path.len() > GLTF_MUTATION_MAX_TOUCHED_PATH_BYTES) {
            return Err(GltfMutationRegistryError::BudgetExceeded("planned touched paths"));
        }
        Ok(())
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn plan(&self, envelope: &GltfMutationEnvelope, base: &GltfSnapshot) -> Result<GltfMutationLeafPlan, GltfMutationRegistryError> {
        self.validate_envelope(envelope)?;
        let descriptor = self.descriptor(&envelope.command_id, envelope.version)?;
        let plan = match envelope.phase {
            GltfMutationPhase::Mutation => (descriptor.plan)(&envelope.payload, base).map_err(Into::into),
            GltfMutationPhase::Inverse => (descriptor.plan_inverse)(&envelope.payload, base).map_err(Into::into),
            GltfMutationPhase::Diff => Err(GltfMutationRegistryError::InvalidPhase { command_id: envelope.command_id.clone(), phase: envelope.phase }),
        }?;
        self.validate_plan(&plan)?;
        Ok(plan)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn apply(&self, envelope: &GltfDiffEnvelope, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfMutationRegistryError> {
        let input = GltfMutationEnvelope { command_id: envelope.command_id.clone(), version: envelope.version, phase: envelope.phase, payload: envelope.payload.clone() };
        self.validate_envelope(&input)?;
        if envelope.touched_paths.len() > GLTF_MUTATION_MAX_TOUCHED_PATHS || envelope.touched_paths.iter().any(|path| path.len() > GLTF_MUTATION_MAX_TOUCHED_PATH_BYTES) {
            return Err(GltfMutationRegistryError::BudgetExceeded("touched paths"));
        }
        let descriptor = self.descriptor(&envelope.command_id, envelope.version)?;
        let application = match envelope.phase {
            GltfMutationPhase::Diff => (descriptor.apply_diff)(&envelope.payload, base).map_err(GltfMutationRegistryError::from)?,
            GltfMutationPhase::Inverse => (descriptor.apply_inverse)(&envelope.payload, base).map_err(GltfMutationRegistryError::from)?,
            GltfMutationPhase::Mutation => return Err(GltfMutationRegistryError::InvalidPhase { command_id: envelope.command_id.clone(), phase: envelope.phase }),
        };
        if application.touched_paths != envelope.touched_paths {
            return Err(GltfMutationLeafError { code: "gltf.mutation.invalid-touched-paths".into(), path: "diff/touchedPaths".into(), detail: "the envelope paths do not match its typed leaf payload".into() }.into());
        }
        Ok(application.snapshot)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn command_ids(&self) -> impl Iterator<Item = &'static str> + '_ {
        self.descriptors.keys().copied()
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn gltf_mutation_registry() -> Result<&'static GltfMutationRegistry, GltfMutationRegistryError> {
    static REGISTRY: OnceLock<Result<GltfMutationRegistry, GltfMutationRegistryError>> = OnceLock::new();
    match REGISTRY.get_or_init(|| GltfMutationRegistry::from_descriptors(gltf_mutation_leaf_descriptors().iter().copied())) {
        Ok(registry) => Ok(registry),
        Err(error) => Err(error.clone()),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn registered_gltf_mutation_command_ids() -> Result<Vec<&'static str>, GltfMutationRegistryError> {
    Ok(gltf_mutation_registry()?.command_ids().collect())
}
//#endregion 🔖️Registry

//#region 🔖️MutationAndDiff
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GltfMutation(GltfMutationEnvelope);

impl GltfMutation {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(command_id: impl Into<String>, version: u32, payload: Vec<u8>) -> Result<Self, GltfMutationRegistryError> {
        let envelope = GltfMutationEnvelope { command_id: command_id.into(), version, phase: GltfMutationPhase::Mutation, payload };
        gltf_mutation_registry()?.validate_envelope(&envelope)?;
        Ok(Self(envelope))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn envelope(&self) -> &GltfMutationEnvelope {
        &self.0
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn from_transport(envelope: GltfMutationEnvelope) -> Result<Self, GltfMutationRegistryError> {
        if envelope.phase == GltfMutationPhase::Diff {
            return Err(GltfMutationRegistryError::InvalidPhase { command_id: envelope.command_id, phase: envelope.phase });
        }
        gltf_mutation_registry()?.validate_envelope(&envelope)?;
        Ok(Self(envelope))
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn validate_gltf_mutation_envelope(envelope: &GltfMutationEnvelope) -> Result<(), GltfMutationRegistryError> {
    gltf_mutation_registry()?.validate_envelope(envelope)
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GltfMutationDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub envelopes: Vec<GltfDiffEnvelope>,
}

impl GltfMutationDiff {
    /// 🧭️ Applies only descriptors that validate the entire typed diff envelope.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn try_apply(&self, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfMutationRegistryError> {
        if self.envelopes.len() > GLTF_MUTATION_MAX_DIFF_ENVELOPES {
            return Err(GltfMutationRegistryError::BudgetExceeded("diff envelopes"));
        }
        let registry = gltf_mutation_registry()?;
        self.envelopes.iter().try_fold(base.clone(), |snapshot, envelope| registry.apply(envelope, &snapshot))
    }
}

impl MutationDiff<GltfSnapshot> for GltfMutationDiff {
    fn apply(&self, base: &GltfSnapshot) -> protocol::MutationApplyResult<GltfSnapshot> {
        self.try_apply(base).map_err(|error| {
            let target = self.envelopes.first().map(|envelope| envelope.command_id.clone()).into_iter();
            MutationApplyError::new("gltf.mutation.apply-rejected", error.to_string()).at(target)
        })
    }

    fn absorb(&mut self, other: Self) {
        self.envelopes.extend(other.envelopes);
    }
}

impl Mutation<GltfSnapshot> for GltfMutation {
    type Diff = GltfMutationDiff;

    fn diff(&self, base: &GltfSnapshot) -> MutationOutcome<Self::Diff> {
        match gltf_mutation_registry().and_then(|registry| registry.plan(&self.0, base)) {
            Ok(plan) => {
                let phase = if self.0.phase == GltfMutationPhase::Mutation { GltfMutationPhase::Diff } else { GltfMutationPhase::Inverse };
                MutationOutcome::new(GltfMutationDiff { envelopes: vec![GltfDiffEnvelope { command_id: self.0.command_id.clone(), version: self.0.version, phase, payload: plan.diff_payload, touched_paths: plan.touched_paths }] })
            }
            Err(error) => MutationOutcome::error("mutation.rejected", error.to_string(), [self.0.command_id.clone()]),
        }
    }

    fn inverse(&self, base: &GltfSnapshot) -> Vec<Self> {
        if self.0.phase != GltfMutationPhase::Mutation {
            return Vec::new();
        }
        gltf_mutation_registry()
            .and_then(|registry| registry.plan(&self.0, base))
            .ok()
            .map(|plan| Self(GltfMutationEnvelope { command_id: self.0.command_id.clone(), version: self.0.version, phase: GltfMutationPhase::Inverse, payload: plan.inverse_payload }))
            .into_iter()
            .collect()
    }
}
//#endregion 🔖️MutationAndDiff

//#region 🧪️RegistryLaws
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::gltf::schema::mutations::change_material_alpha_mode::{mutation, DESCRIPTOR};
    use crate::artifacts::gltf::schema::snapshot::GltfAlphaMode;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn base() -> GltfSnapshot {
        let mut snapshot = GltfSnapshot::default();
        snapshot.document.materials.push(Default::default());
        snapshot
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn payload() -> Vec<u8> {
        serde_json::to_vec(&mutation::GltfChangeMaterialAlphaModePayload { material: 0, alpha_mode: GltfAlphaMode::Mask }).expect("canonical alpha-mode payload")
    }

    #[semio_framework_async_macros::async_test]
    async fn registry_rejects_duplicate_unknown_and_stale_descriptors() {
        assert!(matches!(GltfMutationRegistry::from_descriptors([DESCRIPTOR, DESCRIPTOR]), Err(GltfMutationRegistryError::DuplicateCommand(_))));
        let registry = GltfMutationRegistry::from_descriptors([DESCRIPTOR]).expect("one descriptor");
        assert!(matches!(registry.descriptor("s.stdio.gltf.mutation.unknown.v1", 1), Err(GltfMutationRegistryError::UnknownCommand(_))));
        assert!(matches!(registry.descriptor(DESCRIPTOR.command_id, 2), Err(GltfMutationRegistryError::StaleVersion { expected: 1, actual: 2, .. })));
    }

    #[semio_framework_async_macros::async_test]
    async fn descriptor_plan_apply_and_inverse_validate_stale_base_and_paths() {
        let registry = GltfMutationRegistry::from_descriptors([DESCRIPTOR]).expect("one descriptor");
        let before = base();
        let plan = registry.plan(&GltfMutationEnvelope { command_id: DESCRIPTOR.command_id.into(), version: DESCRIPTOR.version, phase: GltfMutationPhase::Mutation, payload: payload() }, &before).expect("forward plan");
        let forward = GltfDiffEnvelope { command_id: DESCRIPTOR.command_id.into(), version: DESCRIPTOR.version, phase: GltfMutationPhase::Diff, payload: plan.diff_payload.clone(), touched_paths: plan.touched_paths.clone() };
        let mut forged = forward.clone();
        forged.touched_paths = vec!["document/materials/99/alphaMode".into()];
        assert!(matches!(registry.apply(&forged, &before), Err(GltfMutationRegistryError::Leaf(GltfMutationLeafError { code, .. })) if code == "gltf.mutation.invalid-touched-paths"));

        let after = registry.apply(&forward, &before).expect("forward apply");
        assert_eq!(after.document.materials[0].alpha_mode, GltfAlphaMode::Mask);
        assert!(matches!(registry.apply(&forward, &after), Err(GltfMutationRegistryError::Leaf(GltfMutationLeafError { code, .. })) if code == "gltf.mutation.stale-diff"));

        let inverse =
            registry.plan(&GltfMutationEnvelope { command_id: DESCRIPTOR.command_id.into(), version: DESCRIPTOR.version, phase: GltfMutationPhase::Inverse, payload: plan.inverse_payload }, &after).expect("inverse plan validates after-state");
        let restored = registry
            .apply(&GltfDiffEnvelope { command_id: DESCRIPTOR.command_id.into(), version: DESCRIPTOR.version, phase: GltfMutationPhase::Inverse, payload: inverse.diff_payload, touched_paths: inverse.touched_paths }, &after)
            .expect("inverse apply");
        assert_eq!(restored, before);
    }

    #[semio_framework_async_macros::async_test]
    async fn mutation_outcome_and_inverse_round_trip() {
        let before = base();
        let forward = GltfMutation::new(DESCRIPTOR.command_id, DESCRIPTOR.version, payload()).expect("registered mutation");
        let outcome = forward.diff(&before);
        assert!(outcome.messages().is_empty());
        let after = outcome.diff().try_apply(&before).expect("planned forward diff");
        let inverse = forward.inverse(&before).pop().expect("inverse mutation");
        let restored = inverse.diff(&after).diff().try_apply(&after).expect("planned inverse diff");
        assert_eq!(restored, before);
    }
}
//#endregion 🧪️RegistryLaws
