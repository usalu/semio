//! 🧮️ Block 5D play app — the view-state config artifact and its operation enum.
//!
//! Session-only but real, undoable config: it round-trips through the config `ArtifactStore` exactly
//! like document content, with a true `backwards` per operation. Nothing here is document state — the
//! part kind's identity/presentations/grips live in `crate`.


//#region 🔖️Config
/// 🧮️ `Block5dPlayApp`'s empty artifact config; selection and locale live in the shared view model.
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_os_kernel::DslArtifact)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[artifact(extension = "block5dcfg")]
#[artifact(id = "block5d.config")]
#[dsl(layout = "lines")]
pub struct Block5dConfig {}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for Block5dConfig {
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
impl store::ArtifactPack for Block5dConfig {
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

impl store::ConfigRecord for Block5dConfig {}

/// 🔺️ The diff of an empty config: there is nothing to change.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Block5dConfigDiff {}

impl protocol::DiffAlgebra<Block5dConfig> for Block5dConfigDiff {
    fn inverse(&self, _base: &Block5dConfig) -> Self {
        Self {}
    }
    fn between(_base: &Block5dConfig, _other: &Block5dConfig) -> Self {
        Self {}
    }
    fn is_empty(&self) -> bool {
        true
    }
}

impl protocol::MutationDiff<Block5dConfig> for Block5dConfigDiff {
    fn apply(&self, base: &Block5dConfig, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Block5dConfig> {
        Ok(base.clone())
    }
    fn absorb(&mut self, _other: Self) {}
}

//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// 🧮️ An empty config has no mutation: the enum is uninhabited, so no diff can be raised against it.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
pub enum Block5dConfigMutation {}

impl protocol::Mutation<Block5dConfig> for Block5dConfigMutation {
    type Diff = Block5dConfigDiff;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[];

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        match *self {}
    }

    fn diff(&self, _base: &Block5dConfig) -> protocol::MutationOutcome<Block5dConfigDiff> {
        match *self {}
    }

    fn inverse(&self, _base: &Block5dConfig) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        match *self {}
    }
}

impl protocol::OpText for Block5dConfigMutation {
    fn parse_op(_line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "no config mutations exist", semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
    fn print_op(&self) -> String {
        match *self {}
    }
}

impl protocol::OpBinary for Block5dConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        match *self {}
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Err(protocol::ProtocolError::Malformed { what: "no-config-mutation", offset: 0, detail: "no config mutations exist".into() })
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
