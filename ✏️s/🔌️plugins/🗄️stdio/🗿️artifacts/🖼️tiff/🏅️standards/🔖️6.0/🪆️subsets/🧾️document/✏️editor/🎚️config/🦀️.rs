//! 🎚️ Local TIFF editor state retained independently from document history.

use protocol::Mutation;

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::DslArtifact)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[artifact(id = "s.stdio.tiff.editor.config", extension = "tiffeditorcfg")]
#[dsl(layout = "lines")]
pub struct TiffEditorConfig {
    pub selected_ifd: usize,
}

impl store::ArtifactDsl for TiffEditorConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str { Self::__DSL_ENVELOPE_ID }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = store::semio_format::split_text_preamble(text).map_or(text, |(_, body)| body);
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid TIFF editor config envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for TiffEditorConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let body = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::from(error.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &body))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, body) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::from(error.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "TIFF editor config pack envelope mismatch")));
        }
        let (record, _) = store::pack_rt::decode_document(&body, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> { Some(Self::__dsl_spec()) }
}

store::config_diff! { record: TiffEditorConfig, diff: TiffEditorConfigDiff, fields: { selected_ifd: usize } }

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum TiffEditorConfigMutation {
    SetSelectedIfd { selected_ifd: usize },
}

impl Mutation<TiffEditorConfig> for TiffEditorConfigMutation {
    type Diff = TiffEditorConfigDiff;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/✏️editor/🎚️config/🧭️select-ifd",
        semantic_kind: "select-ifd",
        display_name: "Select TIFF Image Page",
        emoji: "🧭️",
        aggregate_variant: "SetSelectedIfd",
        payload_schema: "s.stdio.tiff.editor.config.select-ifd.v1",
        text_opcode: None,
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::Detect,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied, protocol::MutationOutcomeClass::NoOp],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
    }];
    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor { &Self::DESCRIPTORS[0] }
    fn diff(&self, base: &TiffEditorConfig) -> protocol::MutationOutcome<Self::Diff> {
        let Self::SetSelectedIfd { selected_ifd } = self;
        if base.selected_ifd == *selected_ifd {
            return protocol::MutationOutcome::new(TiffEditorConfigDiff::default()).warning("mutation.no-op", "TIFF image page is already selected.");
        }
        protocol::MutationOutcome::new(TiffEditorConfigDiff { selected_ifd: Some(*selected_ifd) })
    }
    fn inverse(&self, base: &TiffEditorConfig) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        Ok(vec![Self::SetSelectedIfd { selected_ifd: base.selected_ifd }])
    }
}

impl protocol::OpText for TiffEditorConfigMutation {
    fn print_op(&self) -> String { semio_framework_pack_json::to_json_string(self) }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        semio_framework_pack_json::from_json_str(line, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}

impl protocol::OpBinary for TiffEditorConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> { Ok(protocol::OpText::print_op(self).into_bytes()) }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(semio_framework_value::ValueError::from(error))))?;
        semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(error)))
    }
}

/// 🧭️ Resolves a safe image page for the current snapshot; stale local state returns to page zero.
pub fn selected_ifd(config: &TiffEditorConfig, snapshot: &crate::TiffSnapshot) -> Option<usize> {
    (!snapshot.ifds.is_empty()).then_some(config.selected_ifd).map(|index| if index < snapshot.ifds.len() { index } else { 0 })
}
