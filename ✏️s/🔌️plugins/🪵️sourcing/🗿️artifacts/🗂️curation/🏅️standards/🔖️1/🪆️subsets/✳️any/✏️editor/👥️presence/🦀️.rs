//! 👥️ Sourcing curation presence — shareable live ephemeral state + mutations.

use protocol::Mutation;
use store::ArtifactPack;

//#region 🔖️Presence
/// 👥️ Shareable live subset of sourcing curation view state (grid camera). Row selection now broadcasts
/// automatically through the framework's typed `PresenceInteraction` field for the "rows" interaction
/// domain (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — no longer mirrored here.
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_os_kernel::DslArtifact, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[artifact(extension = "sourcingcuration.presence")]
#[dsl(layout = "lines")]
pub struct SourcingCurationPresence {
    pub world_camera_position: [f64; 3],
    pub world_camera_target: [f64; 3],
    pub world_camera_fov: f64,
}

impl Default for SourcingCurationPresence {
    fn default() -> Self {
        Self { world_camera_position: [2.5, 2.0, 2.5], world_camera_target: [0.0, 0.0, 0.0], world_camera_fov: 50.0 }
    }
}

impl store::ArtifactPresenceSnapshot for SourcingCurationPresence {}

/// 🔺️ Sparse field delta over [`SourcingCurationPresence`]: every present slot is the new value of exactly that field.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct SourcingCurationPresenceDiff {
    #[value(skip_serializing_if = "Option::is_none")]
    pub world_camera_position: Option<[f64; 3]>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub world_camera_target: Option<[f64; 3]>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub world_camera_fov: Option<f64>,
}

impl protocol::MutationDiff<SourcingCurationPresence> for SourcingCurationPresenceDiff {
    fn apply(&self, base: &SourcingCurationPresence, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<SourcingCurationPresence> {
        let mut next = base.clone();
        if let Some(value) = &self.world_camera_position {
            next.world_camera_position = value.clone();
        }
        if let Some(value) = &self.world_camera_target {
            next.world_camera_target = value.clone();
        }
        if let Some(value) = &self.world_camera_fov {
            next.world_camera_fov = value.clone();
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        if other.world_camera_position.is_some() {
            self.world_camera_position = other.world_camera_position;
        }
        if other.world_camera_target.is_some() {
            self.world_camera_target = other.world_camera_target;
        }
        if other.world_camera_fov.is_some() {
            self.world_camera_fov = other.world_camera_fov;
        }
    }
}

impl protocol::DiffAlgebra<SourcingCurationPresence> for SourcingCurationPresenceDiff {
    fn inverse(&self, base: &SourcingCurationPresence) -> Self {
        Self {
            world_camera_position: self.world_camera_position.as_ref().map(|_| base.world_camera_position.clone()),
            world_camera_target: self.world_camera_target.as_ref().map(|_| base.world_camera_target.clone()),
            world_camera_fov: self.world_camera_fov.as_ref().map(|_| base.world_camera_fov.clone()),
        }
    }
    fn is_empty(&self) -> bool {
        self.world_camera_position.is_none() && self.world_camera_target.is_none() && self.world_camera_fov.is_none()
    }
}

impl store::ArtifactDsl for SourcingCurationPresence {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        if body.trim().is_empty() {
            return Ok(Self::default());
        }
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl ArtifactPack for SourcingCurationPresence {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            return Ok(Self::default());
        }
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️Presence

//#region 🔖️PresenceMutation
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub enum SourcingCurationPresenceMutation {
    #[dsl(key = "world-camera")]
    SetWorldCamera {
        #[dsl(coord)]
        position: [f64; 3],
        #[dsl(coord)]
        target: [f64; 3],
        fov: f64,
    },
}

impl Mutation<SourcingCurationPresence> for SourcingCurationPresenceMutation {
    type Diff = SourcingCurationPresenceDiff;

    /// 🧷️ Hand-written (not `#[derive(dsl::Mutations)]`: a single whole-value snapshot replace, not a
    /// `dsl::Mutations`-eligible semantic-document vocabulary). ⚠️ PROVISIONAL: the `owner` path below
    /// names no directory on disk — this enum has no `🧬️mutations/<slug>` leaf triad of its own, so the
    /// entry is a metadata placeholder to satisfy `protocol::Mutation`, matching the sibling
    /// `🎚️config` enum and puzzle's `🖐️5d` precedent.
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/set-world-camera", semantic_kind: "set-world-camera", display_name: "Set World Camera", emoji: "📄", aggregate_variant: "SetWorldCamera", payload_schema: "🧬️schema/🔣️.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
    ];

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        match self {
            Self::SetWorldCamera { .. } => &Self::DESCRIPTORS[0],
        }
    }

    fn diff(&self, base: &SourcingCurationPresence) -> protocol::MutationOutcome<SourcingCurationPresenceDiff> {
        protocol::MutationOutcome::new(match self {
            Self::SetWorldCamera { position, target, fov } => SourcingCurationPresenceDiff { world_camera_position: (base.world_camera_position != *position).then_some(*position), world_camera_target: (base.world_camera_target != *target).then_some(*target), world_camera_fov: (base.world_camera_fov != *fov).then_some(*fov), ..Default::default() },
        })
    }

    fn inverse(&self, base: &SourcingCurationPresence) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        Ok(vec![match self {
            Self::SetWorldCamera { .. } => Self::SetWorldCamera { position: base.world_camera_position, target: base.world_camera_target, fov: base.world_camera_fov },
        }])
    }
}

impl protocol::OpText for SourcingCurationPresenceMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{keyword} ");
            if line == keyword.as_str() || line.starts_with(&probe) {
                let body = if line.len() > keyword.len() { line[keyword.len()..].trim_start() } else { "" };
                let record = semio_framework_dsl_record::parse(body, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown operation line '{line}'")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        let body = semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline);
        if body.is_empty() {
            keyword
        } else {
            format!("{keyword} {body}")
        }
    }
}

impl protocol::OpBinary for SourcingCurationPresenceMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️PresenceMutation

#[cfg(test)]
mod law_tests {
    use super::*;

    /// ⚖️ The inverse diffs sum to the negative of the forward diff (L3).
    #[semio_framework_async_macros::async_test]
    async fn inverse_diffs_sum_to_the_negative_diff() {
        let base = SourcingCurationPresence::default();
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&SourcingCurationPresenceMutation::SetWorldCamera { position: [1.0, 2.0, 3.0], target: [0.5, 0.0, 0.0], fov: 35.0 }, &base).await;
    }
}
