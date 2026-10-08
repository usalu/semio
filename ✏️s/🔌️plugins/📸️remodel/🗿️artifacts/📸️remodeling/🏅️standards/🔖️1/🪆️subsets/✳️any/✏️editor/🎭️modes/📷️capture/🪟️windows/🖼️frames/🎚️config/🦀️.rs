//! 🎚️ Persisted local configuration for one exact Remodeling Frames window.

use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct RemodelingFrameCursor { pub stream_id: Option<String>, pub frame_index: u32 }

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_os_kernel::DslArtifact)]
#[value(rename_all = "camelCase")]
#[dsl(layout = "lines")]
#[artifact(id = "s.remodel.remodeling.frameswindowconfig", extension = "remodelingframeswindowcfg")]
pub struct RemodelingFramesWindowConfig {
    #[dsl(block)]
    pub frame_cursor: RemodelingFrameCursor,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "kebab-case")]
pub enum RemodelingFramesWindowConfigMutation { SetFrameCursor { frame_cursor: RemodelingFrameCursor } }

impl protocol::Mutation<RemodelingFramesWindowConfig> for RemodelingFramesWindowConfigMutation {
    type Diff = RemodelingFramesWindowConfigDiff;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1, owner: "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📷️capture/🪟️windows/🖼️frames/🎚️config", semantic_kind: "set-frame-cursor", display_name: "Set Remodeling Frames Window Frame Cursor", emoji: "🎚️", aggregate_variant: "SetFrameCursor", payload_schema: "remodeling.frameswindowconfig", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
    }];
    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor { &Self::DESCRIPTORS[0] }
    fn diff(&self, base: &RemodelingFramesWindowConfig) -> protocol::MutationOutcome<Self::Diff> {
        match self {
            Self::SetFrameCursor { frame_cursor } => {
                let diff = RemodelingFramesWindowConfigDiff { frame_cursor: (&base.frame_cursor != frame_cursor).then(|| frame_cursor.clone()), ..Default::default() };
                match protocol::DiffAlgebra::<RemodelingFramesWindowConfig>::is_empty(&diff) {
                    true => protocol::MutationOutcome::empty().warning("mutation.no-op", "Frame cursor is unchanged."),
                    false => protocol::MutationOutcome::new(diff),
                }
            }
        }
    }
    fn inverse(&self, base: &RemodelingFramesWindowConfig) -> Result<Vec<Self>, semio_framework_value::ValueError> {
    Ok(vec![Self::SetFrameCursor { frame_cursor: base.frame_cursor.clone() }])
}
}

macro_rules! impl_codecs { () => {
    /// 📜️ Record-backed text form — the derived `__dsl_spec` grammar inside this window kind's semio
/// text envelope, the same shape every sibling window config prints.
impl store::ArtifactDsl for RemodelingFramesWindowConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = store::semio_format::split_text_preamble(text).map_or(text, |(_, body)| body);
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid Remodeling frames window envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
    /// 🎒️ Record-backed pack form. `record_spec` is what the retained window-config loader reads to
/// decode a mounted pack field-by-field; returning `None` here would fail every retained load of
/// this window kind with `WindowConfigPackLoadDiagnostic::TypedState`.
impl store::ArtifactPack for RemodelingFramesWindowConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let body = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::from(error.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &body))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, body) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::from(error.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _) = store::pack_rt::decode_document(&body, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
} }
impl_codecs!();
impl store::ConfigRecord for RemodelingFramesWindowConfig {}
impl protocol::OpText for RemodelingFramesWindowConfigMutation { fn print_op(&self) -> String { semio_framework_pack_json::to_json_string(self) } fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> { semio_framework_pack_json::from_json_str(line, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1))) } }
impl protocol::OpBinary for RemodelingFramesWindowConfigMutation { fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> { Ok(protocol::OpText::print_op(self).into_bytes()) } fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> { let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(semio_framework_value::ValueError::from(error))))?; semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(error))) } }

pub struct RemodelingFramesWindowConfigOwner;
impl semio_framework_plugin::WindowConfigOwner for RemodelingFramesWindowConfigOwner {
    const WINDOW_KIND_ID: &'static str = super::REMODELING_PLAY_WINDOW_FRAMES; const SCHEMA: &'static str = "remodeling.frameswindowconfig"; const MAXIMUM_PUBLICATION_BYTES: usize = 65_536;
    type State = RemodelingFramesWindowConfig; type Mutation = RemodelingFramesWindowConfigMutation;
    fn build_store_owners() -> store::DocumentStoreOwners<Self::State, Self::Mutation> { semio_framework_plugin::bounded_window_config_store_owners::<Self>() }
    fn build_one_item_preparation_factory() -> std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>> { semio_framework_plugin::bounded_window_config_preparation_factory::<Self>() }
    fn build_store_disposer() -> Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::State, Self::Mutation>>> { semio_framework_plugin::bounded_window_config_store_disposer::<Self>() }
}
pub fn current<C>(view: &semio_framework_plugin::ConfigView<'_, C>) -> RemodelingFramesWindowConfig { view.window::<RemodelingFramesWindowConfigOwner>().cloned().unwrap_or_default() }
pub fn from_snapshot(snapshot: Option<&semio_framework_plugin::WindowConfigSnapshot>) -> RemodelingFramesWindowConfig { snapshot.and_then(|snapshot| snapshot.get::<RemodelingFramesWindowConfigOwner>()).cloned().unwrap_or_default() }
pub fn addressed(view: &semio_framework_plugin::ViewModel, config: RemodelingFramesWindowConfig) -> Result<semio_framework_plugin::WindowConfigMutation, semio_framework_plugin::Fault> { let id = view.window_id.as_deref().ok_or_else(|| semio_framework_plugin::Fault::from("remodeling-frames-window-required"))?; let kind = view.window_instances.iter().find(|window| window.id == id).map(|window| window.window_kind_id.as_str()).ok_or_else(|| semio_framework_plugin::Fault::from("remodeling-window-stale"))?; if kind != super::REMODELING_PLAY_WINDOW_FRAMES { return Err(semio_framework_plugin::Fault::from("remodeling-frames-window-kind-required")); } Ok(semio_framework_plugin::WindowConfigMutation::of::<RemodelingFramesWindowConfigOwner>(id, RemodelingFramesWindowConfigMutation::SetFrameCursor { frame_cursor: config.frame_cursor })) }

//#region 🔺️Diff
/// 🔺️ Sparse field delta for the window configuration; an absent field is untouched.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct RemodelingFramesWindowConfigDiff {
    pub frame_cursor: Option<RemodelingFrameCursor>,
}

impl protocol::MutationDiff<RemodelingFramesWindowConfig> for RemodelingFramesWindowConfigDiff {
    fn apply(&self, base: &RemodelingFramesWindowConfig, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<RemodelingFramesWindowConfig> {
        let mut next = base.clone();
        if let Some(frame_cursor) = &self.frame_cursor {
            next.frame_cursor = frame_cursor.clone();
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.frame_cursor.is_some() {
            self.frame_cursor = other.frame_cursor;
        }
    }
}

impl protocol::DiffAlgebra<RemodelingFramesWindowConfig> for RemodelingFramesWindowConfigDiff {
    fn inverse(&self, base: &RemodelingFramesWindowConfig) -> Self {
        Self {
            frame_cursor: self.frame_cursor.as_ref().map(|_| base.frame_cursor.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}
//#endregion 🔺️Diff
