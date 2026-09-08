//! 🧮️ Imperative play app — view state (`ImperativeConfig`) and its operation enum
//! (`ImperativeConfigMutation`).
//!
//! This is APP state, not document state: it lives at app level rather than under `🗿️artifacts/` because
//! nothing in it survives into the `.imperative` document. It still round-trips through a real
//! `ArtifactStore` (with a real `backwards`), so run-output/locale edits are VCS'd exactly like document
//! content — absorbing the former app-struct `RefCell` (`ImperativePlayRuntime`'s `run_output_json`) plus
//! the locale the UI used to read off the deleted `ViewModel`. Step selection is no longer here: it is the
//! framework-owned `steps` interaction domain (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).

//#region 🔖️Config
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[dsl(extension = "imperative.config")]
#[dsl(id = "imperative.config")]
#[dsl(layout = "lines")]
pub struct ImperativeConfig {
    /// 📤️ Last `run` output, JSON-encoded scope — was `ImperativePlayRuntime::run_output_json`.
    pub run_output_json: String,
    /// 🗣️ BCP-47 locale tag — was read off `view_state.locale`.
    pub locale: String,
    /// 🧩️ Host-pushed `ProgramContributionEntry[]` JSON for `imperative.module` hot-swap installs.
    #[value(default = "default_contributions_json")]
    pub contributions_json: String,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for ImperativeConfig {
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
impl store::ArtifactPack for ImperativeConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

//#endregion 🔖️ArtifactCodec

fn default_contributions_json() -> String {
    crate::io::default_imperative_contributions_json()
}

impl Default for ImperativeConfig {
    fn default() -> Self {
        Self { run_output_json: String::new(), locale: "en-US".into(), contributions_json: default_contributions_json() }
    }
}

store::impl_whole_record_config!(ImperativeConfig);
//#endregion 🔖️Config

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::*;

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn imperative_config_default_is_empty_english() {
        let config = ImperativeConfig::default();
        assert!(config.run_output_json.is_empty());
        assert_eq!(config.locale, "en-US");
    }

    #[semio_framework_async_macros::async_test]
    async fn imperative_config_dsl_round_trips() {
        let config = ImperativeConfig { run_output_json: r#"{"counter":1}"#.into(), locale: "de-DE".into(), contributions_json: "[]".into() };
        store::os_store::test_support::assert_dsl_round_trip(&config);
        store::os_store::test_support::assert_dsl_pack_equivalence(&config);
    }

    #[semio_framework_async_macros::async_test]
    async fn config_operation_snapshot_diff_ignores_base() {
        let base = ImperativeConfig::default();
        let mut snapshot = base.clone();
        snapshot.run_output_json = r#"{"counter":1}"#.into();
        let operation = ImperativeConfigMutation::ReplaceConfig(ReplaceConfig { config: snapshot.clone() });
        assert_eq!(protocol::Mutation::diff(&operation, &base).diff(), &snapshot);
    }

    #[semio_framework_async_macros::async_test]
    async fn config_operation_set_run_output_and_locale_round_trip() {
        store::os_store::test_support::assert_op_line_round_trip(&ImperativeConfigMutation::SetRunOutput(SetRunOutput { json: r#"{"counter":1}"#.into() }));
        store::os_store::test_support::assert_op_line_round_trip(&ImperativeConfigMutation::SetLocale(SetLocale { value: "de-DE".into() }));
    }
}
//#endregion 🧪️Tests

#[cfg(test)]
mod contract_vectors {
    use super::*;
    use protocol::{Mutation, MutationDiff, OpBinary, OpText};
    use dsl::os_pack as pack;

    #[test]
    fn imperative_configuration_contract_vectors_match_the_json_oracle() {
        let vectors: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🔁️mutation-contracts.json")).expect("neutral contract vectors");
        let base: ImperativeConfig = pack::from_json_str(&vectors["base"].to_string()).expect("owned base decoder");
        assert_eq!(<ImperativeConfigMutation as Mutation<ImperativeConfig>>::DESCRIPTORS.len(), vectors["cases"].as_array().expect("cases").len());
        for vector in vectors["cases"].as_array().expect("cases") {
            let mutation: ImperativeConfigMutation = pack::from_json_str(&vector["mutation"].to_string()).expect("owned operation decoder");
            assert_eq!(serde_json::from_str::<serde_json::Value>(&pack::to_json_string(&mutation)).expect("independent operation oracle"), vector["mutation"]);
            assert_eq!(mutation.descriptor().semantic_kind, vector["kind"].as_str().expect("semantic kind"));
            assert_eq!(ImperativeConfigMutation::parse_op(&mutation.print_op()).expect("operation text"), mutation);
            assert_eq!(ImperativeConfigMutation::decode_op(&mutation.encode_op().expect("operation binary")).expect("binary decode"), mutation);
            let outcome = mutation.diff(&base);
            assert!(outcome.messages().is_empty());
            let next = outcome.diff().apply(&base).expect("apply diff");
            assert_eq!(serde_json::from_str::<serde_json::Value>(&pack::to_json_string(&next)).expect("independent state oracle"), vector["expected"]);
            let restored = mutation.inverse(&base).into_iter().fold(next, |state, inverse| inverse.diff(&state).diff().apply(&state).expect("apply inverse"));
            assert_eq!(restored, base);
        }
    }
}
