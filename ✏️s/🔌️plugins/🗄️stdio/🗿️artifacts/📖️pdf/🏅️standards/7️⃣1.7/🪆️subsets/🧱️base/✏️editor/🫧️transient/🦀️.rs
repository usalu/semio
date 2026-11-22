//! 🫧️ PDF view transient: the ephemeral, local-only state of one editor instance's page view. It is never a snapshot field, never an edit and never undoable; the view tools publish it on the transient lane and every window renders from it.

/// 🔭️ Zoom mode, zoom per mille (for the factor mode), anchored page, fit revision and the search cursor. An empty mode means actual size.
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_os_kernel::DslArtifact, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::SnapshotProjection)]
#[value(rename_all = "camelCase", default, by_value_projection)]
#[artifact(id = "stdio.pdfviewtransient")]
#[artifact(extension = "stdiopdfviewtransient")]
#[dsl(layout = "lines")]
pub struct PdfViewTransient {
    pub zoom_mode: String,
    pub zoom_permille: u32,
    pub page: u32,
    pub revision: u32,
    pub query: String,
    pub match_case: bool,
    pub active: Option<u32>,
}

impl pack::snapshot::ArtifactDsl for PdfViewTransient {
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
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid pdf view transient envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl pack::snapshot::ArtifactPack for PdfViewTransient {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::from(error.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::from(error.into_value_error()))?;
        if !envelope.matches_identity(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as pack::snapshot::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::os_store::serialization::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

impl ::semio_os_kernel::os_store::serialization::ArtifactStoreSnapshot for PdfViewTransient {
    fn reference_snapshot_codec() -> Option<::semio_os_kernel::os_store::serialization::ArtifactReferenceSnapshotCodec<Self>> {
        Some(::semio_os_kernel::os_store::serialization::reference_snapshot_codec::<Self>())
    }
}

/// 🔺️ Field-sparse diff of [`PdfViewTransient`]: each field is an optional absolute value.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
pub struct PdfViewTransientDiff {
    pub zoom_mode: Option<String>,
    pub zoom_permille: Option<u32>,
    pub page: Option<u32>,
    pub revision: Option<u32>,
    pub query: Option<String>,
    pub match_case: Option<bool>,
    pub active: Option<Option<u32>>,
}

protocol::retained_row_patch!(pub retained_fields for PdfViewTransientDiff => PdfViewTransient { zoom_mode: String, zoom_permille: u32, page: u32, revision: u32, query: String, match_case: bool, active: Option<u32> });
protocol::retained_empty_option_patch!(PdfViewTransientDiff { zoom_mode, zoom_permille, page, revision, query, match_case, active });
protocol::retained_patch_diff!(PdfViewTransientDiff => PdfViewTransient);

impl protocol::DiffAlgebra<PdfViewTransient> for PdfViewTransientDiff {
    fn is_empty(&self) -> bool {
        self.zoom_mode.is_none() && self.zoom_permille.is_none() && self.page.is_none() && self.revision.is_none() && self.query.is_none() && self.match_case.is_none() && self.active.is_none()
    }
}

impl protocol::ColdDiffAlgebra<PdfViewTransient> for PdfViewTransientDiff {
    fn inverse(&self, base: &PdfViewTransient) -> Self {
        Self {
            zoom_mode: self.zoom_mode.as_ref().map(|_| base.zoom_mode.clone()),
            zoom_permille: self.zoom_permille.map(|_| base.zoom_permille),
            page: self.page.map(|_| base.page),
            revision: self.revision.map(|_| base.revision),
            query: self.query.as_ref().map(|_| base.query.clone()),
            match_case: self.match_case.map(|_| base.match_case),
            active: self.active.map(|_| base.active),
        }
    }
}

impl protocol::MutationDiff<PdfViewTransient> for PdfViewTransientDiff {}

impl protocol::ColdMutationDiff<PdfViewTransient> for PdfViewTransientDiff {
    fn absorb(&mut self, later: Self) {
        self.zoom_mode = later.zoom_mode.or(self.zoom_mode.take());
        self.zoom_permille = later.zoom_permille.or(self.zoom_permille);
        self.page = later.page.or(self.page);
        self.revision = later.revision.or(self.revision);
        self.query = later.query.or(self.query.take());
        self.match_case = later.match_case.or(self.match_case);
        self.active = later.active.or(self.active);
    }

    fn apply(&self, unpublished: &mut PdfViewTransient, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        let mut next = unpublished.clone();
        if let Some(value) = &self.zoom_mode { next.zoom_mode.clone_from(value); }
        if let Some(value) = self.zoom_permille { next.zoom_permille = value; }
        if let Some(value) = self.page { next.page = value; }
        if let Some(value) = self.revision { next.revision = value; }
        if let Some(value) = &self.query { next.query.clone_from(value); }
        if let Some(value) = self.match_case { next.match_case = value; }
        if let Some(value) = self.active { next.active = value; }
        *unpublished = next;
        Ok(())
    }
}
#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::*;
