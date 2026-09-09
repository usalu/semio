//! 📊️ Ephemeral execution output for one concrete Jack results window.

#[path = "🧬️schema/🦀️.rs"]
mod schema;
pub use schema::JackResultsWindowTransient;

impl store::ArtifactDsl for JackResultsWindowTransient {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str { Self::__DSL_ENVELOPE_ID }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = store::semio_format::split_text_preamble(text).map_or(text, |(_, rest)| rest);
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for JackResultsWindowTransient {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::Schema(error.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::Schema(error.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let (record, _) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> { Some(Self::__dsl_spec()) }
}

impl protocol::MutationDiff<JackResultsWindowTransient> for JackResultsWindowTransient {
    fn apply(&self, _base: &JackResultsWindowTransient) -> protocol::MutationApplyResult<JackResultsWindowTransient> { Ok(self.clone()) }
    fn absorb(&mut self, other: Self) { *self = other; }
}

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::*;

pub struct JackResultsWindowTransientOwner;

impl semio_framework_plugin::WindowTransientOwner for JackResultsWindowTransientOwner {
    const WINDOW_KIND_ID: &'static str = crate::editor::jack::TRINITY_JACK_PLAY_WINDOW_RESULTS;
    type State = JackResultsWindowTransient;
    type Mutation = JackResultsWindowTransientMutation;

    fn build_one_item_preparation_factory() -> std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::State, Self::Mutation>> {
        semio_framework_plugin::bounded_window_transient_preparation_factory::<Self>()
    }
    fn build_root_retirement_factory() -> std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::State>> {
        semio_framework_plugin::bounded_window_transient_root_retirement_factory::<Self>()
    }
    fn build_store_disposer() -> Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::State, Self::Mutation>>> {
        semio_framework_plugin::bounded_window_transient_store_disposer::<Self>()
    }
}

pub fn register(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), semio_framework_plugin::Fault> {
    registry.register::<JackResultsWindowTransientOwner>()
}

pub fn addressed(window_id: &str, view: &semio_framework_plugin::ViewModel, mutation: JackResultsWindowTransientMutation) -> Result<semio_framework_plugin::WindowTransientMutation, semio_framework_plugin::Fault> {
    use semio_framework_plugin::{Fault, FaultCode, FaultOrigin, WindowTransientMutation};
    let kind = view.window_instances.iter().find(|window| window.id == window_id).map(|window| window.window_kind_id.as_str());
    if kind != Some(crate::editor::jack::TRINITY_JACK_PLAY_WINDOW_RESULTS) {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("jack.results-window-required"), "Jack query output requires the explicit attached results window"));
    }
    Ok(WindowTransientMutation::of::<JackResultsWindowTransientOwner>(window_id, mutation))
}
