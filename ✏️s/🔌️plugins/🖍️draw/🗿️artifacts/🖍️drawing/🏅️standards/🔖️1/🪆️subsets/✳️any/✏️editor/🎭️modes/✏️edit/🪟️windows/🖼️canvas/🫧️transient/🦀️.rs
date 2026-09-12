//! 🫧️ Ephemeral interaction state for one exact Drawing Canvas window.

use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DrawingCanvasWindowTransient {
    pub engagement_input: String,
    pub trace_pointer_generation: u64,
    pub trace_pointer_completed_work: u64,
    pub trace_pointer_pending_work: u64,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "kebab-case")]
pub enum DrawingCanvasWindowTransientMutation {
    Snapshot { transient: DrawingCanvasWindowTransient },
}

impl protocol::Mutation<DrawingCanvasWindowTransient> for DrawingCanvasWindowTransientMutation {
    type Diff = DrawingCanvasWindowTransient;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️canvas/🫧️transient",
        semantic_kind: "set-window-transient",
        display_name: "Set Drawing Canvas Window Transient",
        emoji: "🫧️",
        aggregate_variant: "Snapshot",
        payload_schema: "drawing.canvas-window.transient",
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
    fn diff(&self, _base: &DrawingCanvasWindowTransient) -> protocol::MutationOutcome<Self::Diff> {
        match self { Self::Snapshot { transient } => protocol::MutationOutcome::new(transient.clone()) }
    }
    fn inverse(&self, base: &DrawingCanvasWindowTransient) -> Vec<Self> {
        vec![Self::Snapshot { transient: base.clone() }]
    }
}

impl protocol::MutationDiff<DrawingCanvasWindowTransient> for DrawingCanvasWindowTransient {
    fn apply(&self, _base: &DrawingCanvasWindowTransient) -> protocol::MutationApplyResult<DrawingCanvasWindowTransient> { Ok(self.clone()) }
    fn absorb(&mut self, other: Self) { *self = other; }
}

impl store::ArtifactDsl for DrawingCanvasWindowTransient {
    const EXTENSION: &'static str = "drawingcanvaswindowtransient";
    fn envelope_id() -> &'static str { "s.draw.drawing.canvas-window.transient" }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = store::semio_format::split_text_preamble(text).map_or(text, |(_, body)| body);
        let json: serde_json::Value = serde_json::from_str(body).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))?;
        dsl::FromValue::from_value(json.into()).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let value: serde_json::Value = dsl::ToValue::to_value(self).into();
        let body = serde_json::to_string_pretty(&value).expect("Drawing Canvas window transient JSON");
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid Drawing Canvas transient envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for DrawingCanvasWindowTransient {
    fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let value: serde_json::Value = dsl::ToValue::to_value(self).into();
        let body = serde_json::to_vec(&value).map_err(|error| store::PackError::Schema(error.to_string()))?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::Schema(error.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &body))
    }
    fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, body) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::Schema(error.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::Schema("Drawing Canvas window transient pack envelope mismatch".into()));
        }
        let json: serde_json::Value = serde_json::from_slice(&body).map_err(|error| store::PackError::Schema(error.to_string()))?;
        dsl::FromValue::from_value(json.into()).map_err(|error| store::PackError::Schema(error.to_string()))
    }
    fn record_spec() -> Option<dsl::RecordSpec> { None }
}

impl protocol::OpText for DrawingCanvasWindowTransientMutation {
    fn print_op(&self) -> String { dsl::json::to_json_string(self) }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        dsl::json::from_json_str(line).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }
}

impl protocol::OpBinary for DrawingCanvasWindowTransientMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> { Ok(protocol::OpText::print_op(self).into_bytes()) }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))?;
        dsl::json::from_json_str(text).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))
    }
}

store::artifact_retire_struct!(DrawingCanvasWindowTransient { engagement_input, trace_pointer_generation, trace_pointer_completed_work, trace_pointer_pending_work });
impl store::retirement::RetireOwned for DrawingCanvasWindowTransientMutation {
    fn retirement(self) -> Box<dyn store::retirement::RetirementCursor> {
        match self {
            Self::Snapshot { transient } => store::retirement::sequence(vec![store::retirement::leaf(0u8), store::retirement::RetireOwned::retirement(transient)]),
        }
    }
}

fn preflight(mutation: &DrawingCanvasWindowTransientMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let DrawingCanvasWindowTransientMutation::Snapshot { transient } = mutation;
    let retained_bytes = transient.engagement_input.len().checked_add(std::mem::size_of::<u64>() * 3).ok_or_else(|| "Drawing Canvas transient footprint overflowed".to_string())?;
    let footprint = store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes };
    footprint.is_admissible().then_some(footprint).ok_or_else(|| "Drawing Canvas transient exceeds its retained publication envelope".into())
}

fn transfer(mutation: DrawingCanvasWindowTransientMutation) -> DrawingCanvasWindowTransient {
    match mutation { DrawingCanvasWindowTransientMutation::Snapshot { transient } => transient }
}

pub struct DrawingCanvasWindowTransientOwner;

impl semio_framework_plugin::WindowTransientOwner for DrawingCanvasWindowTransientOwner {
    const WINDOW_KIND_ID: &'static str = super::DRAWING_PLAY_WINDOW_CANVAS;
    type State = DrawingCanvasWindowTransient;
    type Mutation = DrawingCanvasWindowTransientMutation;
    fn build_owners() -> semio_framework_plugin::WindowTransientOwnerBundle<Self::State, Self::Mutation> {
        let state = std::sync::Arc::new(store::retirement::OwnedValueRetirementFactory::<Self::State>::default());
        let mutation = std::sync::Arc::new(store::retirement::OwnedValueRetirementFactory::<Self::Mutation>::default());
        let preparation = std::sync::Arc::new(store::ArtifactEphemeralTransferPreparationFactory::new(preflight, transfer, state.clone(), mutation.clone()));
        semio_framework_plugin::WindowTransientOwnerBundle::new(preparation, state, mutation)
    }
}

pub fn register(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), semio_framework_plugin::Fault> {
    registry.register::<DrawingCanvasWindowTransientOwner>()
}

pub fn current(view: &semio_framework_plugin::TransientView<'_, semio_framework_plugin::NoTransient>) -> DrawingCanvasWindowTransient {
    view.window::<DrawingCanvasWindowTransientOwner>().cloned().unwrap_or_default()
}

pub fn from_snapshot(snapshot: Option<&semio_framework_plugin::WindowTransientSnapshot>) -> DrawingCanvasWindowTransient {
    snapshot
        .filter(|snapshot| snapshot.window_kind_id() == super::DRAWING_PLAY_WINDOW_CANVAS)
        .and_then(|snapshot| snapshot.get::<DrawingCanvasWindowTransientOwner>())
        .cloned()
        .unwrap_or_default()
}

pub fn addressed(view: &semio_framework_plugin::ViewModel, transient: DrawingCanvasWindowTransient) -> Result<semio_framework_plugin::WindowTransientMutation, semio_framework_plugin::Fault> {
    let id = view.window_id.as_deref().ok_or_else(|| semio_framework_plugin::Fault::from("drawing-canvas-window-required"))?;
    let kind = view.window_instances.iter().find(|window| window.id == id).map(|window| window.window_kind_id.as_str()).ok_or_else(|| semio_framework_plugin::Fault::from("drawing-canvas-window-stale"))?;
    if kind != super::DRAWING_PLAY_WINDOW_CANVAS {
        return Err(semio_framework_plugin::Fault::from("drawing-canvas-window-kind-required"));
    }
    Ok(semio_framework_plugin::WindowTransientMutation::of::<DrawingCanvasWindowTransientOwner>(id, DrawingCanvasWindowTransientMutation::Snapshot { transient }))
}
