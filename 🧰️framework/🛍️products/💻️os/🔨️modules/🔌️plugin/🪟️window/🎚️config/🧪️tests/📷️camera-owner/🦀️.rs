//! 📷️ Shared window camera fixtures: one schema-first camera state, its whole-record mutation, the retained edit and the owners that bind them, mounted by the in-crate preview laws and the companion window-config laws alike.

use std::sync::Arc;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use semio_framework_plugin_window_config::{WindowConfigOwner, bounded_window_config_preparation_factory, bounded_window_config_store_disposer, bounded_window_config_store_owners};
use semio_framework_os_kernel as pack;
use semio_framework_os_kernel as protocol;
use semio_framework_os_kernel as store;

/// 📷️ A real schema-first window camera: enveloped Pack, record spec, and a whole-record snapshot mutation.
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Copy, Debug, Default, PartialEq, semio_framework_os_kernel::DslArtifact, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = store)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(layout = "lines")]
#[artifact(id = "test.retainedloadcameraconfig")]
pub struct RetainedLoadCameraConfig {
    #[dsl(block)]
    pub viewport: semio_framework_os_kernel::Viewport2d,
    #[dsl(coord)]
    pub eye: [f64; 3],
}

semio_framework_value::artifact_retire_leaf!(RetainedLoadCameraConfig);

impl pack::snapshot::ArtifactDsl for RetainedLoadCameraConfig {
    const EXTENSION: &'static str = "retainedloadcameracfg";
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = store::semio_format::split_text_preamble(text).map_or(text, |(_, body)| body);
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid retained-load camera envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl pack::snapshot::ArtifactPack for RetainedLoadCameraConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let body = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::from(error.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &body))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, body) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::from(error.into_value_error()))?;
        if !envelope.matches_identity(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "retained-load camera pack envelope mismatch")));
        }
        let (record, _) = store::pack_rt::decode_document(&body, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::os_store::serialization::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
    fn record_spec_producer() -> Option<store::RecordSpecProducer> {
        Some(Self::__artifact_record_spec_producer())
    }
    fn borrowed_record_spec_producer() -> Option<store::BorrowedRecordSpecProducer> {
        Some(Self::__artifact_borrowed_record_spec_producer())
    }
}
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for RetainedLoadCameraConfig {

}

impl store::ConfigRecord for RetainedLoadCameraConfig {}

/// 🔺️ Sparse diff of [`RetainedLoadCameraConfig`]: the absolute new value of each field the mutation moves.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct RetainedLoadCameraConfigDiff {
    #[value(skip_serializing_if = "Option::is_none")]
    pub viewport: Option<semio_framework_os_kernel::Viewport2d>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub eye: Option<[f64; 3]>,
}

impl protocol::MutationDiff<RetainedLoadCameraConfig> for RetainedLoadCameraConfigDiff {


}

impl protocol::ColdMutationDiff<RetainedLoadCameraConfig> for RetainedLoadCameraConfigDiff {
    fn absorb(&mut self, other: Self) {
        self.viewport = other.viewport.or(self.viewport);
        self.eye = other.eye.or(self.eye);
    }

    fn apply(&self, base: &mut RetainedLoadCameraConfig, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
                if let Some(value) = self.viewport { base.viewport = value; }
        if let Some(value) = self.eye { base.eye = value; }
        Ok(())
    }
}

impl protocol::DiffAlgebra<RetainedLoadCameraConfig> for RetainedLoadCameraConfigDiff {

    fn is_empty(&self) -> bool {
        self.viewport.is_none() && self.eye.is_none()
    }
}

impl protocol::ColdDiffAlgebra<RetainedLoadCameraConfig> for RetainedLoadCameraConfigDiff {
    fn inverse(&self, base: &RetainedLoadCameraConfig) -> Self {
        Self { viewport: self.viewport.map(|_| base.viewport), eye: self.eye.map(|_| base.eye) }
    }
}

#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = store)]
#[value(tag = "kind", rename_all = "kebab-case", rename_all_fields = "camelCase", deny_unknown_fields)]
pub(crate) enum RetainedLoadCameraConfigMutation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: Box<RetainedLoadCameraConfig>,
    },
}

impl protocol::OpText for RetainedLoadCameraConfigMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        for (keyword, spec_fn) in &<Self as semio_framework_dsl_record::DslVariants>::variants() {
            if line == keyword || line.starts_with(&format!("{keyword} ")) {
                let record = semio_framework_dsl_record::parse(line, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown retained-load camera mutation '{line}'")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let spec = <Self as semio_framework_dsl_record::DslVariants>::variants().iter().find(|(key, _)| key == &keyword).map(|(_, spec)| (spec.ordinary)()).expect("retained-load camera mutation variant");
        semio_framework_dsl_record::print(&record, &spec, semio_framework_dsl_record::JoinMode::Inline)
    }
}

impl protocol::OpBinary for RetainedLoadCameraConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}

impl protocol::Mutation<RetainedLoadCameraConfig> for RetainedLoadCameraConfigMutation {
    type Diff = RetainedLoadCameraConfigDiff;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🧪️tests/📥️retained-pack-load",
        semantic_kind: "set-window-config",
        display_name: "Set Retained Load Camera Window Configuration",
        emoji: "📷️",
        aggregate_variant: "Snapshot",
        payload_schema: "test.retainedloadcameraconfig",
        text_opcode: None,
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::Detect,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust],
    }];
    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        &Self::DESCRIPTORS[0]
    }


}

impl protocol::ColdMutation<RetainedLoadCameraConfig> for RetainedLoadCameraConfigMutation {
    type ReferenceDiff = RetainedLoadCameraConfigDiff;
    fn diff(&self, base: &RetainedLoadCameraConfig) -> protocol::MutationOutcome<Self::ReferenceDiff> {
        match self {
            Self::Snapshot { config } => protocol::MutationOutcome::new(RetainedLoadCameraConfigDiff { viewport: (config.viewport != base.viewport).then_some(config.viewport), eye: (config.eye != base.eye).then_some(config.eye) }),
        }
    }
    fn inverse(&self, base: &RetainedLoadCameraConfig) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        Ok(vec![Self::Snapshot { config: Box::new(RetainedLoadCameraConfig { viewport: base.viewport, eye: base.eye }) }])
    }
}

#[doc(hidden)]
#[path="🧬️preparation/🦀️.rs"]
pub mod preparation;
pub use preparation::{CameraRetainedEdit, CameraRetainedEditCursor};

pub struct RetainedLoadOwnerA;
pub struct RetainedLoadOwnerB;

macro_rules! retained_load_owner {
    ($owner:ty, $kind:literal, $schema:literal) => {
        impl WindowConfigOwner for $owner {
            const WINDOW_KIND_ID: &'static str = $kind;
            const SCHEMA: &'static str = $schema;
            const MAXIMUM_PUBLICATION_BYTES: usize = 4_096;
            type State = RetainedLoadCameraConfig;
            type Mutation = RetainedLoadCameraConfigMutation;
            type Edit=CameraRetainedEdit;
            const MAXIMUM_PREPARATION_DEPTH:usize=64;
            fn build_retained_edit()->Arc<Self::Edit>{Arc::new(CameraRetainedEdit)}

            fn preparation_source_birth()->semio_framework_value::retained_clone::RetainedCloneBirthDemand{semio_framework_plugin_window_config::bounded_window_config_preparation_source_birth::<Self>()}
    fn build_store_owners(grant:semio_framework_value::RetainedCloneGrant) -> Result<(store::os_store::document_store_owners::DocumentStoreOwners<Self::State, Self::Mutation>,semio_framework_value::RetainedCloneProgress),store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::State,Self::Mutation>> {
                bounded_window_config_store_owners::<Self>(grant)
            }

            fn build_one_item_preparation_factory() -> Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>> {
                bounded_window_config_preparation_factory::<Self>()
            }

            fn build_store_disposer() -> Box<dyn ArtifactOwnedDisposer<store::ConfigStore<Self::State, Self::Mutation>>> {
                bounded_window_config_store_disposer::<Self>()
            }
        }
    };
}

retained_load_owner!(RetainedLoadOwnerA, "retained-load-a", "test.window.retained-load.a");
retained_load_owner!(RetainedLoadOwnerB, "retained-load-b", "test.window.retained-load.b");

pub struct IdentityWindowOwner;

impl WindowConfigOwner for IdentityWindowOwner {
    const WINDOW_KIND_ID: &'static str = "identity-window";
    const SCHEMA: &'static str = "test.window.identity";
    const MAXIMUM_PUBLICATION_BYTES: usize = 16_384;
    type State = RetainedLoadCameraConfig;
    type Mutation = RetainedLoadCameraConfigMutation;
    type Edit=CameraRetainedEdit;
    const MAXIMUM_PREPARATION_DEPTH:usize=64;
    fn build_retained_edit()->Arc<Self::Edit>{Arc::new(CameraRetainedEdit)}

    fn preparation_source_birth()->semio_framework_value::retained_clone::RetainedCloneBirthDemand{semio_framework_plugin_window_config::bounded_window_config_preparation_source_birth::<Self>()}
    fn build_store_owners(grant:semio_framework_value::RetainedCloneGrant) -> Result<(store::os_store::document_store_owners::DocumentStoreOwners<Self::State, Self::Mutation>,semio_framework_value::RetainedCloneProgress),store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::State,Self::Mutation>> {
        bounded_window_config_store_owners::<Self>(grant)
    }

    fn build_one_item_preparation_factory() -> Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>> {
        bounded_window_config_preparation_factory::<Self>()
    }

    fn build_store_disposer() -> Box<dyn ArtifactOwnedDisposer<store::ConfigStore<Self::State, Self::Mutation>>> {
        bounded_window_config_store_disposer::<Self>()
    }
}
