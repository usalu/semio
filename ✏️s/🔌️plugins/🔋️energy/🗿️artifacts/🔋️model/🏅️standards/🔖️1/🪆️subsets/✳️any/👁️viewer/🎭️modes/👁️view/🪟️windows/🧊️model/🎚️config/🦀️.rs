//! 🎚️ Persisted local state for ONE concrete `energy.model.3d` VIEWER window: its orbit pose.
//!
//! 👁️ An independently authored twin of the editor window's `🎚️config` — a viewer may never import
//! through `✏️editor` (`policyViewerPurityBreaches`), so the record, its `set-camera` verb and its
//! owner are spelled again here under their own dsl id (`energy.model3dviewerwindowconfig`) and their
//! own store. Two surfaces, two window-config lanes, one shape.
//!
//! 🎥️ Why a WINDOW config and not an app config: two open 3d windows (a split pane, a second tab)
//! orbit independently, and a read-only surface may not write the artifact lane at all.

#[path = "🧬️schema/🦀️.rs"]
mod schema;
pub use schema::*;

impl store::ArtifactDsl for EnergyModelViewerWindowConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = store::semio_format::split_text_preamble(text).map_or(text, |(_, rest)| rest);
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid energy 3d window-config envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for EnergyModelViewerWindowConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::Schema(error.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::Schema(error.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token())));
        }
        let (record, _) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

store::impl_whole_record_config!(EnergyModelViewerWindowConfig);

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::*;

pub struct EnergyModelViewerWindowConfigOwner;

impl semio_framework_plugin::WindowConfigOwner for EnergyModelViewerWindowConfigOwner {
    const WINDOW_KIND_ID: &'static str = super::WINDOW_KIND_ID;
    const SCHEMA: &'static str = "energy.model3dviewerwindowconfig";
    /// 📏️ A pose is seven doubles; 1 KiB is two orders of magnitude of headroom over that.
    const MAXIMUM_PUBLICATION_BYTES: usize = 1024;
    type State = EnergyModelViewerWindowConfig;
    type Mutation = EnergyModelViewerWindowConfigMutation;

    fn build_store_owners() -> store::DocumentStoreOwners<Self::State, Self::Mutation> {
        semio_framework_plugin::bounded_window_config_store_owners::<Self>()
    }
    fn build_one_item_preparation_factory() -> std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>> {
        semio_framework_plugin::bounded_window_config_preparation_factory::<Self>()
    }
    fn build_store_disposer() -> Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::State, Self::Mutation>>> {
        semio_framework_plugin::bounded_window_config_store_disposer::<Self>()
    }
}

/// 🎥️ The addressed window's retained pose, or `None` while it has never been orbited — a render
/// that reads `None` falls back to the model-derived camera plus `fit_json`'s one-shot framing.
pub fn current<'a, C>(view: &'a semio_framework_plugin::ConfigView<'_, C>) -> Option<&'a EnergyModelViewerWindowConfig> {
    view.window::<EnergyModelViewerWindowConfigOwner>()
}

/// 🪟️ Binds one config mutation to the EXACT window instance the gesture came from — a camera is
/// per instance, so a dispatch with no addressed window, or one addressed at another window kind, is
/// a refusal rather than a write to the wrong pane.
pub fn addressed(view: &semio_framework_plugin::ViewModel, mutation: EnergyModelViewerWindowConfigMutation) -> Result<semio_framework_plugin::WindowConfigMutation, semio_framework_plugin::Fault> {
    let window_id = view
        .window_id
        .as_deref()
        .ok_or_else(|| semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("energy.model.3d.viewer.window-required"), "a camera change requires a concrete 3d model window"))?;
    let kind = view.window_instances.iter().find(|window| window.id == window_id).map(|window| window.window_kind_id.as_str());
    if kind != Some(super::WINDOW_KIND_ID) {
        return Err(semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("energy.model.3d.viewer.window-kind"), "a camera change requires a 3d model window"));
    }
    Ok(semio_framework_plugin::WindowConfigMutation::of::<EnergyModelViewerWindowConfigOwner>(window_id, mutation))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
