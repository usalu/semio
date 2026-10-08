//! 🧮️ Imperative play app — view state (`ImperativeConfig`) and its operation enum
//! (`ImperativeConfigMutation`).
//!
//! This is APP state, not document state: it lives at app level rather than under `🗿️artifacts/` because
//! nothing in it survives into the `.imperative` document. It still round-trips through a real
//! `ArtifactStore` (with a real `backwards`), so run-output edits are VCS'd exactly like document
//! content — absorbing the former app-struct `RefCell` (`ImperativePlayRuntime`'s `run_output_json`).
//! Locale and terminology come from the shared host `ViewModel`. Step selection is no longer here: it is the
//! framework-owned `steps` interaction domain (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).

//#region 🔖️Config
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_os_kernel::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[artifact(extension = "imperative.config")]
#[artifact(id = "imperative.config")]
#[dsl(layout = "lines")]
pub struct ImperativeConfig {
    /// 📤️ Last `run` output, JSON-encoded scope — was `ImperativePlayRuntime::run_output_json`.
    pub run_output_json: String,
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
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted ArtifactPack (P6): envelope-wrapped pack body via `__dsl_*` record lowering.
impl store::ArtifactPack for ImperativeConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
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

//#endregion 🔖️ArtifactCodec

fn default_contributions_json() -> String {
    crate::standards::v1::subsets::any::io::default_imperative_contributions_json()
}

impl Default for ImperativeConfig {
    fn default() -> Self {
        Self { run_output_json: String::new(), contributions_json: default_contributions_json() }
    }
}

impl store::ConfigRecord for ImperativeConfig {}

/// 🔺️ Sparse field delta over [`ImperativeConfig`]: every present slot is the new value of exactly that field.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct ImperativeConfigDiff {
    #[value(skip_serializing_if = "Option::is_none")]
    pub run_output_json: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub contributions_json: Option<String>,
}

impl protocol::MutationDiff<ImperativeConfig> for ImperativeConfigDiff {
    fn apply(&self, base: &ImperativeConfig, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<ImperativeConfig> {
        let mut next = base.clone();
        if let Some(value) = &self.run_output_json {
            next.run_output_json = value.clone();
        }
        if let Some(value) = &self.contributions_json {
            next.contributions_json = value.clone();
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        if other.run_output_json.is_some() {
            self.run_output_json = other.run_output_json;
        }
        if other.contributions_json.is_some() {
            self.contributions_json = other.contributions_json;
        }
    }
}

impl protocol::DiffAlgebra<ImperativeConfig> for ImperativeConfigDiff {
    fn inverse(&self, base: &ImperativeConfig) -> Self {
        Self {
            run_output_json: self.run_output_json.as_ref().map(|_| base.run_output_json.clone()),
            contributions_json: self.contributions_json.as_ref().map(|_| base.contributions_json.clone()),
        }
    }
    fn between(base: &ImperativeConfig, other: &ImperativeConfig) -> Self {
        Self {
            run_output_json: (base.run_output_json != other.run_output_json).then(|| other.run_output_json.clone()),
            contributions_json: (base.contributions_json != other.contributions_json).then(|| other.contributions_json.clone()),
        }
    }
    fn is_empty(&self) -> bool {
        self.run_output_json.is_none() && self.contributions_json.is_none()
    }
}

//#endregion 🔖️Config

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::*;

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️contract-vectors/🦀️.rs"]
mod contract_vectors;

#[path = "🚪️io/🦀️.rs"]
pub mod io;
