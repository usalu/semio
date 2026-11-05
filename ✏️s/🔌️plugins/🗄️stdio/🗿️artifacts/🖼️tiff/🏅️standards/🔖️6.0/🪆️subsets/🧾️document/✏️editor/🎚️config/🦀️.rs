//! 🎚️ Local TIFF editor state retained independently from document history.

use protocol::Mutation;

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::DslArtifact, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[artifact(id = "s.stdio.tiff.editor.config", extension = "tiffeditorcfg")]
#[dsl(layout = "lines")]
pub struct TiffEditorConfig {
    pub selected_ifd: usize,
}

impl pack::snapshot::ArtifactDsl for TiffEditorConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str { Self::__DSL_ENVELOPE_ID }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = store::semio_format::split_text_preamble(text).map_or(text, |(_, body)| body);
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid TIFF editor config envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl pack::snapshot::ArtifactPack for TiffEditorConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let body = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::from(error.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &body))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, body) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::from(error.into_value_error()))?;
        if !envelope.matches_identity(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "TIFF editor config pack envelope mismatch")));
        }
        let (record, _) = store::pack_rt::decode_document(&body, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::os_store::serialization::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> { Some(Self::__dsl_spec()) }
}
impl ::semio_os_kernel::os_store::serialization::ArtifactStoreSnapshot for TiffEditorConfig {
    fn reference_snapshot_codec()->Option<::semio_os_kernel::os_store::serialization::ArtifactReferenceSnapshotCodec<Self>>{Some(::semio_os_kernel::os_store::serialization::reference_snapshot_codec::<Self>())}


}

store::config_diff! { record: TiffEditorConfig, diff: TiffEditorConfigDiff, fields: { selected_ifd: usize } }

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_dsl_record_derive::DslReceiving, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum TiffEditorConfigMutation {
    #[dsl(key = "select-ifd")]
    SetSelectedIfd(SetSelectedIfdEdit),
}

/// 🧭️ `SetSelectedIfd` payload record.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_dsl_record_derive::DslReceiving, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct SetSelectedIfdEdit {
    pub selected_ifd: usize,
}

impl TiffEditorConfigMutation {
    /// 👆️ Selects the image-file directory at `selected_ifd`.
    pub fn select_ifd(selected_ifd: usize) -> Self { Self::SetSelectedIfd(SetSelectedIfdEdit { selected_ifd }) }
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
}

impl protocol::ColdMutation<TiffEditorConfig> for TiffEditorConfigMutation {
    type ReferenceDiff = TiffEditorConfigDiff;
    fn diff(&self, base: &TiffEditorConfig) -> protocol::MutationOutcome<Self::ReferenceDiff> {
        let Self::SetSelectedIfd(SetSelectedIfdEdit { selected_ifd }) = self;
        if base.selected_ifd == *selected_ifd {
            return protocol::MutationOutcome::new(TiffEditorConfigDiff::default()).warning("mutation.no-op", "TIFF image page is already selected.");
        }
        protocol::MutationOutcome::new(TiffEditorConfigDiff { selected_ifd: Some(*selected_ifd) })
    }
    fn inverse(&self, base: &TiffEditorConfig) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        Ok(vec![Self::select_ifd(base.selected_ifd)])
    }
}

impl protocol::OpText for TiffEditorConfigMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> { semio_framework_dsl_record::variants_text::parse_op(line) }
    fn print_op(&self) -> String { semio_framework_dsl_record::variants_text::print_op(self) }
}

impl protocol::OpBinary for TiffEditorConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 0, detail: format!("keyword {keyword:?} is not a declared variant") })?;
        let spec = (variants[ordinal].1.ordinary)();
        let body = store::pack_rt::encode_record_body(&spec, &record, &store::PackEncodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        store::pack_rt::write_varint_u64(&mut out, ordinal as u64);
        out.extend_from_slice(&body);
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = store::pack_rt::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64()?;
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) })?;
        let spec = (spec_fn.ordinary)();
        let body = &bytes[reader.position()..];
        let (record, _report) = store::pack_rt::decode_record_body(body, &spec, &store::PackDecodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record).map_err(|error| protocol::ProtocolError::Malformed { what: "op record", offset: reader.position() as u64, detail: error.to_string() })
    }
}
semio_os_kernel::original_ordinal_operation_receiving!(TiffEditorConfigMutation);

/// 🦺️ Resolves a safe image page for the current snapshot; stale local state returns to page zero.
pub fn selected_ifd(config: &TiffEditorConfig, snapshot: &crate::TiffSnapshot) -> Option<usize> {
    (!snapshot.ifds.is_empty()).then_some(config.selected_ifd).map(|index| if index < snapshot.ifds.len() { index } else { 0 })
}

impl store::snapshot_clone_preparation::ConfigApplyMutation<TiffEditorConfig> for TiffEditorConfigMutation {
    fn exchange(self, post: &mut TiffEditorConfig) -> Result<Self, (semio_framework_value::ValueError, Self)> {
        let Self::SetSelectedIfd(SetSelectedIfdEdit { mut selected_ifd }) = self;
        std::mem::swap(&mut selected_ifd, &mut post.selected_ifd);
        Ok(Self::select_ifd(selected_ifd))
    }
}
