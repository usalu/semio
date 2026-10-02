//! 🎚️ Reusable configuration contract for Rewriting graph windows.

#[path = "🧬️schema/🦀️.rs"]
mod schema;
pub use schema::RewritingWindowConfig;

impl store::ArtifactDsl for RewritingWindowConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted ArtifactPack (P6): envelope-wrapped pack body via `__dsl_*` record lowering.
impl store::ArtifactPack for RewritingWindowConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

store::impl_whole_record_config!(RewritingWindowConfig);

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::*;

macro_rules! window_owner {
    ($name:ident, $kind:ident) => {
        pub struct $name;
        impl semio_framework_plugin::WindowConfigOwner for $name {
            const WINDOW_KIND_ID: &'static str = super::$kind;
            const SCHEMA: &'static str = "trinity.rewritingwindowcfg";
            const MAXIMUM_PUBLICATION_BYTES: usize = 4096;
            type State = RewritingWindowConfig;
            type Mutation = RewritingWindowConfigMutation;

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
    };
}

window_owner!(BeforeWindowConfigOwner, TRINITY_REWRITING_PLAY_WINDOW_BEFORE);
window_owner!(AfterWindowConfigOwner, TRINITY_REWRITING_PLAY_WINDOW_AFTER);
window_owner!(LhsWindowConfigOwner, TRINITY_REWRITING_PLAY_WINDOW_LHS);
window_owner!(RhsWindowConfigOwner, TRINITY_REWRITING_PLAY_WINDOW_RHS);

pub fn register(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), semio_framework_plugin::Fault> {
    registry.register::<BeforeWindowConfigOwner>()?;
    registry.register::<AfterWindowConfigOwner>()?;
    registry.register::<LhsWindowConfigOwner>()?;
    registry.register::<RhsWindowConfigOwner>()
}

pub fn current<'a>(view: &'a semio_framework_plugin::ConfigView<'_, semio_framework_plugin::NoConfig>) -> Option<&'a RewritingWindowConfig> {
    view.window::<BeforeWindowConfigOwner>().or_else(|| view.window::<AfterWindowConfigOwner>()).or_else(|| view.window::<LhsWindowConfigOwner>()).or_else(|| view.window::<RhsWindowConfigOwner>())
}

pub fn addressed(view: &semio_framework_plugin::ViewModel, mutation: RewritingWindowConfigMutation) -> Result<semio_framework_plugin::WindowConfigMutation, semio_framework_plugin::Fault> {
    use semio_framework_plugin::{Fault, FaultCode, FaultOrigin, WindowConfigMutation};
    let window_id = view.window_id.as_deref().ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("rewriting.window-required"), "Rewriting view changes require a concrete window"))?;
    let kind = view.window_instances.iter().find(|window| window.id == window_id).map(|window| window.window_kind_id.as_str());
    match kind {
        Some(super::TRINITY_REWRITING_PLAY_WINDOW_BEFORE) => Ok(WindowConfigMutation::of::<BeforeWindowConfigOwner>(window_id, mutation)),
        Some(super::TRINITY_REWRITING_PLAY_WINDOW_AFTER) => Ok(WindowConfigMutation::of::<AfterWindowConfigOwner>(window_id, mutation)),
        Some(super::TRINITY_REWRITING_PLAY_WINDOW_LHS) => Ok(WindowConfigMutation::of::<LhsWindowConfigOwner>(window_id, mutation)),
        Some(super::TRINITY_REWRITING_PLAY_WINDOW_RHS) => Ok(WindowConfigMutation::of::<RhsWindowConfigOwner>(window_id, mutation)),
        _ => Err(Fault::new(FaultOrigin::App, FaultCode::new("rewriting.graph-window-required"), "Rewriting view changes require a graph window")),
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️window-config-ownership/🦀️.rs"]
mod tests;

#[path = "🧵️job/🦀️.rs"]
pub mod job;
