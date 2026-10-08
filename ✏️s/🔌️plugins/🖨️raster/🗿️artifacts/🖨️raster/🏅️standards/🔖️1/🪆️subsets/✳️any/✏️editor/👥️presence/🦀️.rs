//! 👥️ Raster presence — shareable live ephemeral state + mutations.

use crate::RasterCamera;
use protocol::Mutation;
use store::ArtifactPack;

//#region 🔖️Presence
/// 👥️ Shareable live raster brush and camera state. Layer selection/hover
/// deleted (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM): the `"layers"` interaction
/// domain broadcasts automatically via the framework's typed `PresenceInteraction` field now.
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_os_kernel::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[artifact(extension = "raster.presence")]
#[dsl(layout = "lines")]
pub struct RasterPresence {
    pub brush_size: f64,
    pub brush_opacity: f64,
    pub camera: RasterCamera,
}

impl Default for RasterPresence {
    fn default() -> Self {
        Self { brush_size: 24.0, brush_opacity: 1.0, camera: RasterCamera::default() }
    }
}

/// 🔺️ Sparse delta of the shareable presence: only the fields a mutation actually changes.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct RasterPresenceDiff {
    #[value(skip_serializing_if = "Option::is_none")]
    pub brush_size: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub brush_opacity: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub camera: Option<RasterCamera>,
}

impl protocol::MutationDiff<RasterPresence> for RasterPresenceDiff {
    fn apply(&self, base: &RasterPresence, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<RasterPresence> {
        Ok(RasterPresence { brush_size: self.brush_size.unwrap_or(base.brush_size), brush_opacity: self.brush_opacity.unwrap_or(base.brush_opacity), camera: self.camera.clone().unwrap_or_else(|| base.camera.clone()) })
    }
    fn absorb(&mut self, other: Self) {
        if other.brush_size.is_some() {
            self.brush_size = other.brush_size;
        }
        if other.brush_opacity.is_some() {
            self.brush_opacity = other.brush_opacity;
        }
        if other.camera.is_some() {
            self.camera = other.camera;
        }
    }
}

impl protocol::DiffAlgebra<RasterPresence> for RasterPresenceDiff {
    fn inverse(&self, base: &RasterPresence) -> Self {
        Self { brush_size: self.brush_size.map(|_| base.brush_size), brush_opacity: self.brush_opacity.map(|_| base.brush_opacity), camera: self.camera.as_ref().map(|_| base.camera.clone()) }
    }
    fn between(base: &RasterPresence, other: &RasterPresence) -> Self {
        Self { brush_size: (base.brush_size != other.brush_size).then_some(other.brush_size), brush_opacity: (base.brush_opacity != other.brush_opacity).then_some(other.brush_opacity), camera: (base.camera != other.camera).then(|| other.camera.clone()) }
    }
    fn is_empty(&self) -> bool {
        *self == Self::default()
    }
}

impl store::ArtifactDsl for RasterPresence {
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

impl ArtifactPack for RasterPresence {
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
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum)]
#[value(rename_all = "camelCase")]
pub enum RasterPresenceMutation {
    #[dsl(key = "set")]
    Set {
        #[dsl(block)]
        presence: RasterPresence,
    },
}

impl Mutation<RasterPresence> for RasterPresenceMutation {
    type Diff = RasterPresenceDiff;

    /// 🧷️ Hand-written (no `dsl::Mutations` derive on this enum) — presence is ephemeral shared
    /// state, so the `owner` path is registry metadata only.
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence",
        semantic_kind: "set-presence",
        display_name: "Set Presence",
        emoji: "👥️",
        aggregate_variant: "Set",
        payload_schema: "🧬️schema/🔣️.json",
        text_opcode: None,
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::Detect,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
    }];

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        match self {
            Self::Set { .. } => &Self::DESCRIPTORS[0],
        }
    }

    fn diff(&self, base: &RasterPresence) -> protocol::MutationOutcome<RasterPresenceDiff> {
        let Self::Set { presence } = self;
        let diff = RasterPresenceDiff {
            brush_size: (base.brush_size != presence.brush_size).then_some(presence.brush_size),
            brush_opacity: (base.brush_opacity != presence.brush_opacity).then_some(presence.brush_opacity),
            camera: (base.camera != presence.camera).then(|| presence.camera.clone()),
        };
        if diff == RasterPresenceDiff::default() {
            return protocol::MutationOutcome::new(diff).warning("mutation.no-op", "Presence is already up to date.");
        }
        protocol::MutationOutcome::new(diff)
    }

    fn inverse(&self, base: &RasterPresence) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        Ok(vec![Self::Set { presence: base.clone() }])
    }
}

impl protocol::OpText for RasterPresenceMutation {
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

impl protocol::OpBinary for RasterPresenceMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️PresenceMutation

//#region 🧹️Retirement
/// 📏️ A `RasterPresence` root owns nothing beyond its inline bytes (two `f64`s and a camera of three
/// `f64`s), so every root is terminal-empty and one bounded turn returns it.
pub fn raster_presence_is_terminal_empty(_presence: &RasterPresence) -> bool {
    true
}

/// 👥️ Exact local and peer root ownership for raster presence (process3d precedent): without it and
/// the disposer below, every close of a registry-backed app faulted
/// `interactive-job.close-owned-disposer-missing … presence-store` (mounted boot test of ticket
/// 26/09/05/RASTER-PLUGIN-END-TO-END, 2026-09-16).
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct RasterPresenceRetirementFactory;

impl store::SnapshotRetirementFactory<RasterPresence> for RasterPresenceRetirementFactory {
    fn retirement_birth_bytes(&self, _snapshot: &std::sync::Arc<RasterPresence>) -> usize { std::mem::size_of::<RasterPresenceRetirement>() }

    fn retire(&self, root: std::sync::Arc<RasterPresence>) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(RasterPresenceRetirement { root: std::mem::ManuallyDrop::new(Some(root)) })
    }
}

struct RasterPresenceRetirement {
    root: std::mem::ManuallyDrop<Option<std::sync::Arc<RasterPresence>>>,
}

impl store::ErasedSnapshotRetirement for RasterPresenceRetirement {
    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, semio_framework_value::ValueError> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Blocked);
        }
        if let Some(root) = self.root.take() {
            drop(root);
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.root.is_none()
    }
}

impl Drop for RasterPresenceRetirement {
    fn drop(&mut self) {
        assert!((self.root.is_none()) || std::thread::panicking(), "raster presence retirement reached Drop before its root was returned");
        unsafe { std::mem::ManuallyDrop::drop(&mut self.root) };
    }
}

pub fn raster_presence_store_disposer() -> Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<RasterPresence, RasterPresenceMutation>>> {
    Box::new(semio_framework_plugin::PresenceStoreOwnedDisposer::new(std::sync::Arc::new(RasterPresence::default()), raster_presence_is_terminal_empty).expect("the default raster presence root owns no heap"))
}
//#endregion 🧹️Retirement

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
