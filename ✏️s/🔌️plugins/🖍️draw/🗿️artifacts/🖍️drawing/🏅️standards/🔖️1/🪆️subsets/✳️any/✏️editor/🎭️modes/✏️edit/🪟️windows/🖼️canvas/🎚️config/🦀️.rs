//! 🎚️ Persisted local navigation for one exact Drawing Canvas window.

use semio_framework_value_derive::{FromValue, ToValue};

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_os_kernel::DslArtifact)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(layout = "lines")]
#[artifact(id = "s.draw.drawing.canvas-window.config", extension = "drawingcanvaswindowcfg")]
pub struct DrawingCanvasWindowConfig {
    #[dsl(block)]
    pub viewport: store::Viewport2d,
    pub framed: bool,
}

impl Default for DrawingCanvasWindowConfig {
    fn default() -> Self {
        Self { viewport: store::Viewport2d { x: 0.0, y: 0.0, zoom: 1.0 },framed: false }
    }
}

/// 🔺️ Sparse delta of the persisted navigation: only the fields a mutation actually changes.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct DrawingCanvasWindowConfigDiff {
    #[value(skip_serializing_if = "Option::is_none")]
    pub viewport: Option<store::Viewport2d>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub framed: Option<bool>,
}

impl store::ConfigRecord for DrawingCanvasWindowConfig {}

impl protocol::MutationDiff<DrawingCanvasWindowConfig> for DrawingCanvasWindowConfigDiff {
    fn apply(&self, base: &DrawingCanvasWindowConfig, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<DrawingCanvasWindowConfig> {
        Ok(DrawingCanvasWindowConfig { viewport: self.viewport.clone().unwrap_or_else(|| base.viewport.clone()), framed: self.framed.unwrap_or(base.framed) })
    }
    fn absorb(&mut self, other: Self) {
        if other.viewport.is_some() {
            self.viewport = other.viewport;
        }
        if other.framed.is_some() {
            self.framed = other.framed;
        }
    }
}

impl protocol::DiffAlgebra<DrawingCanvasWindowConfig> for DrawingCanvasWindowConfigDiff {
    fn inverse(&self, base: &DrawingCanvasWindowConfig) -> Self {
        Self { viewport: self.viewport.as_ref().map(|_| base.viewport.clone()), framed: self.framed.map(|_| base.framed) }
    }
    fn between(base: &DrawingCanvasWindowConfig, other: &DrawingCanvasWindowConfig) -> Self {
        Self { viewport: (base.viewport != other.viewport).then(|| other.viewport.clone()), framed: (base.framed != other.framed).then_some(other.framed) }
    }
    fn is_empty(&self) -> bool {
        self.viewport.is_none() && self.framed.is_none()
    }
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "kebab-case")]
pub enum DrawingCanvasWindowConfigMutation {
    Set { viewport: store::Viewport2d, framed: bool },
}

impl protocol::Mutation<DrawingCanvasWindowConfig> for DrawingCanvasWindowConfigMutation {
    type Diff = DrawingCanvasWindowConfigDiff;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️canvas/🎚️config",
        semantic_kind: "set-window-config",
        display_name: "Set Drawing Canvas Window Configuration",
        emoji: "🎚️",
        aggregate_variant: "Set",
        payload_schema: "drawing.canvas-window.config",
        text_opcode: None,
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::Detect,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[
            protocol::MutationLanguageSurface::Rust,
            protocol::MutationLanguageSurface::Typescript,
            protocol::MutationLanguageSurface::JsonSchema,
            protocol::MutationLanguageSurface::Graphql,
            protocol::MutationLanguageSurface::Protobuf,
        ],
    }];

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor { &Self::DESCRIPTORS[0] }
    fn diff(&self, base: &DrawingCanvasWindowConfig) -> protocol::MutationOutcome<Self::Diff> {
        match self {
            Self::Set { viewport, framed } => protocol::MutationOutcome::new(DrawingCanvasWindowConfigDiff { viewport: (&base.viewport != viewport).then(|| viewport.clone()), framed: (base.framed != *framed).then_some(*framed) }),
        }
    }
    fn inverse(&self, base: &DrawingCanvasWindowConfig) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        Ok(vec![Self::Set { viewport: base.viewport.clone(), framed: base.framed }])
    }
}

/// 📜️ Record-backed text form — the derived `__dsl_spec` grammar inside this window kind's semio
/// text envelope, the same shape every sibling window config prints.
impl store::ArtifactDsl for DrawingCanvasWindowConfig {
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
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid Drawing canvas window envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 🎒️ Record-backed pack form. `record_spec` is what the retained window-config loader reads to
/// decode a mounted pack field-by-field; returning `None` here would fail every retained load of
/// this window kind with `WindowConfigPackLoadDiagnostic::TypedState`.
impl store::ArtifactPack for DrawingCanvasWindowConfig {
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

impl protocol::OpText for DrawingCanvasWindowConfigMutation {
    fn print_op(&self) -> String { semio_framework_pack_json::to_json_string(self) }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        semio_framework_pack_json::from_json_str(line, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}

impl protocol::OpBinary for DrawingCanvasWindowConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> { Ok(protocol::OpText::print_op(self).into_bytes()) }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(semio_framework_value::ValueError::from(error))))?;
        semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(error)))
    }
}

pub struct DrawingCanvasWindowConfigOwner;

impl semio_framework_plugin::WindowConfigOwner for DrawingCanvasWindowConfigOwner {
    const WINDOW_KIND_ID: &'static str = super::DRAWING_PLAY_WINDOW_CANVAS;
    const SCHEMA: &'static str = "drawing.canvas-window.config";
    const MAXIMUM_PUBLICATION_BYTES: usize = 4_096;
    type State = DrawingCanvasWindowConfig;
    type Mutation = DrawingCanvasWindowConfigMutation;
    fn build_store_owners() -> store::DocumentStoreOwners<Self::State, Self::Mutation> { semio_framework_plugin::bounded_window_config_store_owners::<Self>() }
    fn build_one_item_preparation_factory() -> std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>> { semio_framework_plugin::bounded_window_config_preparation_factory::<Self>() }
    fn build_store_disposer() -> Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::State, Self::Mutation>>> { semio_framework_plugin::bounded_window_config_store_disposer::<Self>() }
}

pub fn register(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), semio_framework_plugin::Fault> {
    registry.register::<DrawingCanvasWindowConfigOwner>()
}

pub fn current<C>(view: &semio_framework_plugin::ConfigView<'_, C>) -> DrawingCanvasWindowConfig {
    view.window::<DrawingCanvasWindowConfigOwner>().cloned().unwrap_or_default()
}

pub fn from_snapshot(snapshot: Option<&semio_framework_plugin::WindowConfigSnapshot>) -> DrawingCanvasWindowConfig {
    snapshot
        .filter(|snapshot| snapshot.window_kind_id() == super::DRAWING_PLAY_WINDOW_CANVAS)
        .and_then(|snapshot| snapshot.get::<DrawingCanvasWindowConfigOwner>())
        .cloned()
        .unwrap_or_default()
}

pub fn addressed(view: &semio_framework_plugin::ViewModel, config: DrawingCanvasWindowConfig) -> Result<semio_framework_plugin::WindowConfigMutation, semio_framework_plugin::Fault> {
    let id = view.window_id.as_deref().ok_or_else(|| semio_framework_plugin::Fault::from("drawing-canvas-window-required"))?;
    let kind = view.window_instances.iter().find(|window| window.id == id).map(|window| window.window_kind_id.as_str()).ok_or_else(|| semio_framework_plugin::Fault::from("drawing-canvas-window-stale"))?;
    if kind != super::DRAWING_PLAY_WINDOW_CANVAS {
        return Err(semio_framework_plugin::Fault::from("drawing-canvas-window-kind-required"));
    }
    Ok(semio_framework_plugin::WindowConfigMutation::of::<DrawingCanvasWindowConfigOwner>(id, DrawingCanvasWindowConfigMutation::Set { viewport: config.viewport, framed: config.framed }))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️window/🦀️.rs"]
mod window_ownership_tests;
