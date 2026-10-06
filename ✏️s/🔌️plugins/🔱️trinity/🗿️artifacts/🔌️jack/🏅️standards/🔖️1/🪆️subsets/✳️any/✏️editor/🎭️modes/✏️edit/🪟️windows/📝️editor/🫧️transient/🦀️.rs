//! 🔤️ Concrete Jack editor-window transient selection state.

pub const WINDOW_KIND_ID: &str = "trinity-jack-editor";

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct JackEditorSelection {
    pub start: u64,
    pub end: u64,
}

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_os_kernel::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[artifact(extension = "trinity.jackeditorwindowtransient")]
#[dsl(layout = "lines")]
pub struct JackEditorWindowTransient {
    #[dsl(block)]
    pub selection: Option<JackEditorSelection>,
}

impl store::ArtifactDsl for JackEditorWindowTransient {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;

    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }

    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }

    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid Jack editor window transient envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for JackEditorWindowTransient {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::from(error.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::from(error.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }

    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

impl protocol::MutationDiff<JackEditorWindowTransient> for JackEditorWindowTransient {
    fn apply(&self, _base: &JackEditorWindowTransient) -> protocol::MutationApplyResult<JackEditorWindowTransient> {
        Ok(self.clone())
    }

    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::*;

semio_framework_value::artifact_retire_struct!(JackEditorSelection { start, end });
semio_framework_value::artifact_retire_struct!(JackEditorWindowTransient { selection });
semio_framework_value::artifact_retire_struct!(SetEditorSelection { selection });

impl semio_framework_value::retirement::RetireOwned for JackEditorWindowTransientMutation {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        match self {
            Self::SetEditorSelection(value) => semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::leaf(0u8), semio_framework_value::retirement::RetireOwned::retirement(value)]),
        }
    }
}

fn editor_window_transient_footprint(_: &JackEditorWindowTransientMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    Ok(store::ArtifactStoreOneItemFootprint::for_ephemeral_item(std::mem::size_of::<JackEditorWindowTransient>()))
}

fn editor_window_transient_transfer(mutation: JackEditorWindowTransientMutation) -> JackEditorWindowTransient {
    let JackEditorWindowTransientMutation::SetEditorSelection(value) = mutation;
    JackEditorWindowTransient { selection: value.selection }
}

pub struct JackEditorWindowTransientOwner;

impl semio_framework_plugin::WindowTransientOwner for JackEditorWindowTransientOwner {
    const WINDOW_KIND_ID: &'static str = WINDOW_KIND_ID;
    type State = JackEditorWindowTransient;
    type Mutation = JackEditorWindowTransientMutation;

    fn build_owners() -> semio_framework_plugin::WindowTransientOwnerBundle<Self::State, Self::Mutation> {
        let state = std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<Self::State>::default());
        let mutation = std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<Self::Mutation>::default());
        let preparation = std::sync::Arc::new(store::ArtifactEphemeralTransferPreparationFactory::new(editor_window_transient_footprint, editor_window_transient_transfer, state.clone(), mutation.clone()));
        semio_framework_plugin::WindowTransientOwnerBundle::new(preparation, state, mutation)
    }
}

#[path = "🚪️io/🦀️.rs"]
pub mod io;
