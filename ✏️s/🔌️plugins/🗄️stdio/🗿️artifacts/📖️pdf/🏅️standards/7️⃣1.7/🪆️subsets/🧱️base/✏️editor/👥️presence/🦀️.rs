//! 👥️ PDF presence lane: the ephemeral shared state of one collaborator in one PDF editor instance (current page, pointer, visible page rectangle and text caret). It is never a snapshot field, never an edit and never undoable; selection and typing previews travel on the framework's typed interaction and typing channels and are not repeated here.

/// 👥️ Every coordinate is a per mille of the page's displayed size, so peers with different zoom and window sizes agree. An all-zero view rectangle means unknown; an empty caret object means no caret.
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_os_kernel::DslArtifact, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::SnapshotProjection, semio_framework_value::CanonicalJsonTree, semio_framework_value_derive::NativeReceive)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default, by_value_projection)]
#[artifact(id = "stdio.pdfpresence")]
#[artifact(extension = "stdiopdfpresence")]
#[dsl(layout = "lines")]
pub struct PdfPresence {
    pub page: u32,
    pub pointer_active: bool,
    pub pointer_x: u32,
    pub pointer_y: u32,
    pub view_left: u32,
    pub view_top: u32,
    pub view_right: u32,
    pub view_bottom: u32,
    pub caret_object: String,
    pub caret_offset: u32,
    pub caret_anchor: u32,
}

impl pack::snapshot::ArtifactDsl for PdfPresence {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        if body.trim().is_empty() {
            return Ok(Self::default());
        }
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid pdf presence envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl pack::snapshot::ArtifactPack for PdfPresence {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::from(error.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            return Ok(Self::default());
        }
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

impl ::semio_os_kernel::os_store::serialization::ArtifactStoreSnapshot for PdfPresence {
    fn reference_snapshot_codec() -> Option<::semio_os_kernel::os_store::serialization::ArtifactReferenceSnapshotCodec<Self>> {
        Some(::semio_os_kernel::os_store::serialization::reference_snapshot_codec::<Self>())
    }
}

impl store::ArtifactPresenceSnapshot for PdfPresence {
    fn decode_presence_native(bytes: &[u8], owner: &mut store::io::control::NativeSnapshotDecodeOwner<'_, '_>) -> semio_framework_value::RetainedWorkOutcome<Self, store::io::control::NativeSnapshotFailure> {
        store::decode_presence_json(bytes, owner)
    }
}

/// 🔺️ Field-sparse diff of [`PdfPresence`]: each field is an optional absolute value.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
pub struct PdfPresenceDiff {
    pub page: Option<u32>,
    pub pointer_active: Option<bool>,
    pub pointer_x: Option<u32>,
    pub pointer_y: Option<u32>,
    pub view_left: Option<u32>,
    pub view_top: Option<u32>,
    pub view_right: Option<u32>,
    pub view_bottom: Option<u32>,
    pub caret_object: Option<String>,
    pub caret_offset: Option<u32>,
    pub caret_anchor: Option<u32>,
}

protocol::retained_row_patch!(pub retained_fields for PdfPresenceDiff => PdfPresence { page: u32, pointer_active: bool, pointer_x: u32, pointer_y: u32, view_left: u32, view_top: u32, view_right: u32, view_bottom: u32, caret_object: String, caret_offset: u32, caret_anchor: u32 });
protocol::retained_empty_option_patch!(PdfPresenceDiff { page, pointer_active, pointer_x, pointer_y, view_left, view_top, view_right, view_bottom, caret_object, caret_offset, caret_anchor });
protocol::retained_patch_diff!(PdfPresenceDiff => PdfPresence);

impl protocol::DiffAlgebra<PdfPresence> for PdfPresenceDiff {
    fn is_empty(&self) -> bool {
        self.page.is_none() && self.pointer_active.is_none() && self.pointer_x.is_none() && self.pointer_y.is_none() && self.view_left.is_none() && self.view_top.is_none() && self.view_right.is_none() && self.view_bottom.is_none() && self.caret_object.is_none() && self.caret_offset.is_none() && self.caret_anchor.is_none()
    }
}

impl protocol::ColdDiffAlgebra<PdfPresence> for PdfPresenceDiff {
    fn inverse(&self, base: &PdfPresence) -> Self {
        Self {
            page: self.page.map(|_| base.page),
            pointer_active: self.pointer_active.map(|_| base.pointer_active),
            pointer_x: self.pointer_x.map(|_| base.pointer_x),
            pointer_y: self.pointer_y.map(|_| base.pointer_y),
            view_left: self.view_left.map(|_| base.view_left),
            view_top: self.view_top.map(|_| base.view_top),
            view_right: self.view_right.map(|_| base.view_right),
            view_bottom: self.view_bottom.map(|_| base.view_bottom),
            caret_object: self.caret_object.as_ref().map(|_| base.caret_object.clone()),
            caret_offset: self.caret_offset.map(|_| base.caret_offset),
            caret_anchor: self.caret_anchor.map(|_| base.caret_anchor),
        }
    }
}

impl protocol::MutationDiff<PdfPresence> for PdfPresenceDiff {}

impl protocol::ColdMutationDiff<PdfPresence> for PdfPresenceDiff {
    fn absorb(&mut self, later: Self) {
        self.page = later.page.or(self.page);
        self.pointer_active = later.pointer_active.or(self.pointer_active);
        self.pointer_x = later.pointer_x.or(self.pointer_x);
        self.pointer_y = later.pointer_y.or(self.pointer_y);
        self.view_left = later.view_left.or(self.view_left);
        self.view_top = later.view_top.or(self.view_top);
        self.view_right = later.view_right.or(self.view_right);
        self.view_bottom = later.view_bottom.or(self.view_bottom);
        self.caret_object = later.caret_object.or(self.caret_object.take());
        self.caret_offset = later.caret_offset.or(self.caret_offset);
        self.caret_anchor = later.caret_anchor.or(self.caret_anchor);
    }

    fn apply(&self, unpublished: &mut PdfPresence, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        let mut next = unpublished.clone();
        if let Some(value) = self.page { next.page = value; }
        if let Some(value) = self.pointer_active { next.pointer_active = value; }
        if let Some(value) = self.pointer_x { next.pointer_x = value; }
        if let Some(value) = self.pointer_y { next.pointer_y = value; }
        if let Some(value) = self.view_left { next.view_left = value; }
        if let Some(value) = self.view_top { next.view_top = value; }
        if let Some(value) = self.view_right { next.view_right = value; }
        if let Some(value) = self.view_bottom { next.view_bottom = value; }
        if let Some(value) = &self.caret_object { next.caret_object.clone_from(value); }
        if let Some(value) = self.caret_offset { next.caret_offset = value; }
        if let Some(value) = self.caret_anchor { next.caret_anchor = value; }
        *unpublished = next;
        Ok(())
    }
}

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::*;
