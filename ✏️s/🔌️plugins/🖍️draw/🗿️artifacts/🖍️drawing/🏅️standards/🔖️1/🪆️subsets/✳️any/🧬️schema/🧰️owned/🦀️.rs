//! 🧰️ Drawing owned envelope decoder, recursive retirement, and retained store initializer.

use crate::artifacts::drawing::op::DrawingMutation;
use crate::artifacts::drawing::{DrawingAttributes, DrawingImageAsset, DrawingLayerBase, DrawingLayerNode, DrawingSnapshot, FillStyle, GradientStop, PathSegment, StrokeStyle};
use protocol::{Mutation, OpBinary};

//#region 🔖️OwnedSprCatalog
const DRAWING_OWNED_FIELD_BYTES: usize = store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;

enum DrawingRetirementOwner {
    Snapshot(DrawingSnapshot),
    Mutation(DrawingMutation),
    MutationFields(DrawingMutationFields),
    Layer(DrawingLayerNode),
    Layers(Vec<DrawingLayerNode>),
    LayerFields(DrawingLayerFields),
    Base(DrawingLayerBase),
    Attributes(DrawingAttributes),
    Fill(FillStyle),
    Stroke(StrokeStyle),
    Asset(DrawingImageAsset),
    AssetEntry { key: String, value: Option<DrawingImageAsset> },
    String(String),
    Strings(Vec<String>),
    Segments(Vec<PathSegment>),
    Stops(Vec<GradientStop>),
    Points(Vec<[f64; 2]>),
}

enum DrawingMutationFields {
    String(String),
    Strings { first: String, second: Option<String> },
    Fill { id: String, value: Option<FillStyle> },
    Stroke { id: String, value: Option<StrokeStyle> },
    Layer { parent: Option<String>, value: Option<Box<DrawingLayerNode>> },
}

enum DrawingLayerFields {
    Shape { base: Option<DrawingLayerBase>, shape_kind: String, points: Option<Vec<[f64; 2]>> },
    Path { base: Option<DrawingLayerBase>, segments: Option<Vec<PathSegment>> },
    Text { base: Option<DrawingLayerBase>, content: String },
    Image { base: Option<DrawingLayerBase>, image_key: String },
    Group { base: Option<DrawingLayerBase>, children: Option<Vec<DrawingLayerNode>> },
    Boolean { base: Option<DrawingLayerBase>, operation: String, children: Option<Vec<String>> },
    Trace { base: Option<DrawingLayerBase>, source_key: String },
}

struct DrawingOwnedRetirement {
    owner: std::mem::ManuallyDrop<Option<DrawingRetirementOwner>>,
    active: std::mem::ManuallyDrop<Option<Box<DrawingOwnedRetirement>>>,
    phase: u8,
}

impl DrawingOwnedRetirement {
    fn new(owner: DrawingRetirementOwner) -> Self {
        Self { owner: std::mem::ManuallyDrop::new(Some(owner)), active: std::mem::ManuallyDrop::new(None), phase: 0 }
    }

    fn spawn(active: &mut Option<Box<Self>>, owner: DrawingRetirementOwner) -> store::SnapshotRetirementStep {
        *active = Some(Box::new(Self::new(owner)));
        store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }
    }

    fn release_string(value: &mut String, phase: &mut u8, next: u8, maximum_items: usize, maximum_bytes: usize) -> store::SnapshotRetirementStep {
        if maximum_items == 0 || value.len() > maximum_bytes {
            return store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 };
        }
        let value = std::mem::take(value);
        let released_bytes = value.len();
        drop(value);
        *phase = next;
        store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes }
    }

    fn advance(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        let Some(owner) = self.owner.as_mut() else { return Ok(store::SnapshotRetirementStep::Complete) };
        match owner {
            DrawingRetirementOwner::Snapshot(value) => match self.phase {
                0 => {
                    if let Some(value) = value.layers.pop() {
                        return Ok(Self::spawn(&mut self.active, DrawingRetirementOwner::Layer(value)));
                    }
                    self.phase = 1;
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                }
                1 => {
                    if let Some((key, value)) = value.assets.pop_last() {
                        return Ok(Self::spawn(&mut self.active, DrawingRetirementOwner::AssetEntry { key, value: Some(value) }));
                    }
                    self.phase = 2;
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                }
                2 => Ok(Self::release_string(&mut value.schema, &mut self.phase, 3, maximum_items, maximum_bytes)),
                3 => Ok(Self::release_string(&mut value.id, &mut self.phase, 4, maximum_items, maximum_bytes)),
                4 if value.title.is_some() => {
                    let title = value.title.take().expect("Drawing title remains retained");
                    self.phase = 5;
                    Ok(Self::spawn(&mut self.active, DrawingRetirementOwner::String(title)))
                }
                _ => {
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
            },
            DrawingRetirementOwner::Layer(_) => {
                let layer = match self.owner.take() {
                    Some(DrawingRetirementOwner::Layer(value)) => value,
                    _ => unreachable!("Drawing layer owner variant remains exact"),
                };
                let fields = match layer {
                    DrawingLayerNode::Shape(value) => DrawingLayerFields::Shape { base: Some(value.base), shape_kind: value.shape_kind, points: value.polygon.map(|polygon| polygon.points) },
                    DrawingLayerNode::Path(value) => DrawingLayerFields::Path { base: Some(value.base), segments: Some(value.segments) },
                    DrawingLayerNode::Text(value) => DrawingLayerFields::Text { base: Some(value.base), content: value.content },
                    DrawingLayerNode::Image(value) => DrawingLayerFields::Image { base: Some(value.base), image_key: value.image_key },
                    DrawingLayerNode::Group(value) => DrawingLayerFields::Group { base: Some(value.base), children: Some(value.children) },
                    DrawingLayerNode::Boolean(value) => DrawingLayerFields::Boolean { base: Some(value.base), operation: value.operation, children: Some(value.children) },
                    DrawingLayerNode::Trace(value) => DrawingLayerFields::Trace { base: Some(value.base), source_key: value.source_key },
                };
                *self.owner = Some(DrawingRetirementOwner::LayerFields(fields));
                Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
            }
            DrawingRetirementOwner::Layers(values) => {
                if let Some(value) = values.pop() {
                    Ok(Self::spawn(&mut self.active, DrawingRetirementOwner::Layer(value)))
                } else {
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
            }
            DrawingRetirementOwner::LayerFields(fields) => {
                if self.phase == 0 {
                    let base = match fields {
                        DrawingLayerFields::Shape { base, .. }
                        | DrawingLayerFields::Path { base, .. }
                        | DrawingLayerFields::Text { base, .. }
                        | DrawingLayerFields::Image { base, .. }
                        | DrawingLayerFields::Group { base, .. }
                        | DrawingLayerFields::Boolean { base, .. }
                        | DrawingLayerFields::Trace { base, .. } => base.take(),
                    }
                    .ok_or_else(|| "Drawing layer base owner missing".to_string())?;
                    self.phase = 1;
                    return Ok(Self::spawn(&mut self.active, DrawingRetirementOwner::Base(base)));
                }
                let nested = match fields {
                    DrawingLayerFields::Shape { shape_kind, points, .. } if self.phase == 1 => Some(DrawingRetirementOwner::String(std::mem::take(shape_kind))),
                    DrawingLayerFields::Shape { points, .. } => points.take().map(DrawingRetirementOwner::Points),
                    DrawingLayerFields::Path { segments, .. } => segments.take().map(DrawingRetirementOwner::Segments),
                    DrawingLayerFields::Text { content, .. } => (!content.is_empty()).then(|| DrawingRetirementOwner::String(std::mem::take(content))),
                    DrawingLayerFields::Image { image_key, .. } => (!image_key.is_empty()).then(|| DrawingRetirementOwner::String(std::mem::take(image_key))),
                    DrawingLayerFields::Group { children, .. } => children.as_mut().and_then(Vec::pop).map(DrawingRetirementOwner::Layer),
                    DrawingLayerFields::Boolean { operation, .. } if self.phase == 1 => Some(DrawingRetirementOwner::String(std::mem::take(operation))),
                    DrawingLayerFields::Boolean { children, .. } => children.take().map(DrawingRetirementOwner::Strings),
                    DrawingLayerFields::Trace { source_key, .. } => (!source_key.is_empty()).then(|| DrawingRetirementOwner::String(std::mem::take(source_key))),
                };
                if let Some(nested) = nested {
                    self.phase = self.phase.saturating_add(1);
                    return Ok(Self::spawn(&mut self.active, nested));
                }
                drop(self.owner.take());
                Ok(store::SnapshotRetirementStep::Complete)
            }
            DrawingRetirementOwner::Base(value) => match self.phase {
                0 => Ok(Self::release_string(&mut value.id, &mut self.phase, 1, maximum_items, maximum_bytes)),
                1 => Ok(Self::release_string(&mut value.name, &mut self.phase, 2, maximum_items, maximum_bytes)),
                2 => Ok(Self::release_string(&mut value.blend_mode, &mut self.phase, 3, maximum_items, maximum_bytes)),
                3 => {
                    let attributes = std::mem::take(&mut value.attributes);
                    self.phase = 4;
                    Ok(Self::spawn(&mut self.active, DrawingRetirementOwner::Attributes(attributes)))
                }
                _ => {
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
            },
            DrawingRetirementOwner::Attributes(value) => {
                if let Some(fill) = value.fill.take() {
                    return Ok(Self::spawn(&mut self.active, DrawingRetirementOwner::Fill(fill)));
                }
                if let Some(stroke) = value.stroke.take() {
                    return Ok(Self::spawn(&mut self.active, DrawingRetirementOwner::Stroke(stroke)));
                }
                drop(self.owner.take());
                Ok(store::SnapshotRetirementStep::Complete)
            }
            DrawingRetirementOwner::Fill(value) => match value {
                FillStyle::LinearGradient { stops, .. } | FillStyle::RadialGradient { stops, .. } if !stops.is_empty() => Ok(Self::spawn(&mut self.active, DrawingRetirementOwner::Stops(std::mem::take(stops)))),
                _ => {
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
            },
            DrawingRetirementOwner::Stroke(value) => match self.phase {
                0 => Ok(Self::release_string(&mut value.cap, &mut self.phase, 1, maximum_items, maximum_bytes)),
                1 => Ok(Self::release_string(&mut value.join, &mut self.phase, 2, maximum_items, maximum_bytes)),
                _ => {
                    if let Some(dash) = value.dash.as_mut() {
                        if dash.pop().is_some() {
                            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
                        }
                    }
                    value.dash = None;
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
            },
            DrawingRetirementOwner::Asset(value) => match self.phase {
                0 => Ok(Self::release_string(&mut value.mime, &mut self.phase, 1, maximum_items, maximum_bytes)),
                1 => Ok(Self::release_string(&mut value.data, &mut self.phase, 2, maximum_items, maximum_bytes)),
                _ => {
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
            },
            DrawingRetirementOwner::AssetEntry { key, value } => match self.phase {
                0 => Ok(Self::release_string(key, &mut self.phase, 1, maximum_items, maximum_bytes)),
                1 => {
                    self.phase = 2;
                    Ok(Self::spawn(&mut self.active, DrawingRetirementOwner::Asset(value.take().ok_or_else(|| "Drawing asset owner missing".to_string())?)))
                }
                _ => {
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
            },
            DrawingRetirementOwner::String(value) => {
                if self.phase == 0 {
                    return Ok(Self::release_string(value, &mut self.phase, 1, maximum_items, maximum_bytes));
                }
                drop(self.owner.take());
                Ok(store::SnapshotRetirementStep::Complete)
            }
            DrawingRetirementOwner::Strings(values) => {
                if let Some(value) = values.pop() {
                    return Ok(Self::spawn(&mut self.active, DrawingRetirementOwner::String(value)));
                }
                drop(self.owner.take());
                Ok(store::SnapshotRetirementStep::Complete)
            }
            DrawingRetirementOwner::Segments(values) => {
                if values.pop().is_some() {
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                } else {
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
            }
            DrawingRetirementOwner::Stops(values) => {
                if values.pop().is_some() {
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                } else {
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
            }
            DrawingRetirementOwner::Points(values) => {
                if values.pop().is_some() {
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                } else {
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
            }
            DrawingRetirementOwner::Mutation(_) => {
                use DrawingMutation::*;
                let mutation = match self.owner.take() {
                    Some(DrawingRetirementOwner::Mutation(value)) => value,
                    _ => unreachable!("Drawing mutation owner variant remains exact"),
                };
                let fields = match mutation {
                    SetLayerVisible(payload) => DrawingMutationFields::String(payload.layer_id),
                    SetLayerLocked(payload) => DrawingMutationFields::String(payload.layer_id),
                    SetLayerOpacity(payload) => DrawingMutationFields::String(payload.layer_id),
                    SetLayerBlendMode(payload) => DrawingMutationFields::Strings { first: payload.layer_id, second: Some(payload.blend_mode) },
                    RenameLayer(payload) => DrawingMutationFields::Strings { first: payload.layer_id, second: Some(payload.new_name) },
                    UpdateLayerTransform(payload) => DrawingMutationFields::String(payload.layer_id),
                    ReplaceLayerFill(payload) => DrawingMutationFields::Fill { id: payload.layer_id, value: payload.fill },
                    ReplaceLayerStroke(payload) => DrawingMutationFields::Stroke { id: payload.layer_id, value: payload.stroke },
                    SetLayerBooleanOperation(payload) => DrawingMutationFields::Strings { first: payload.layer_id, second: Some(payload.boolean_operation) },
                    UpdateLayerTraceParams(payload) => DrawingMutationFields::String(payload.layer_id),
                    CreateLayer(payload) => DrawingMutationFields::Layer { parent: payload.parent_id, value: Some(payload.layer) },
                    DuplicateLayer(payload) => DrawingMutationFields::String(payload.layer_id),
                    DeleteLayer(payload) => DrawingMutationFields::String(payload.layer_id),
                    ReorderLayer(payload) => DrawingMutationFields::Strings { first: payload.layer_id, second: payload.parent_id },
                };
                *self.owner = Some(DrawingRetirementOwner::MutationFields(fields));
                Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
            }
            DrawingRetirementOwner::MutationFields(fields) => match fields {
                DrawingMutationFields::String(value) => {
                    if self.phase == 0 {
                        return Ok(Self::release_string(value, &mut self.phase, 1, maximum_items, maximum_bytes));
                    }
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
                DrawingMutationFields::Strings { first, second } => match self.phase {
                    0 => Ok(Self::release_string(first, &mut self.phase, 1, maximum_items, maximum_bytes)),
                    1 if second.is_some() => {
                        self.phase = 2;
                        Ok(Self::spawn(&mut self.active, DrawingRetirementOwner::String(second.take().expect("Drawing second string remains exact"))))
                    }
                    _ => {
                        drop(self.owner.take());
                        Ok(store::SnapshotRetirementStep::Complete)
                    }
                },
                DrawingMutationFields::Fill { id, value } => match self.phase {
                    0 => Ok(Self::release_string(id, &mut self.phase, 1, maximum_items, maximum_bytes)),
                    1 if value.is_some() => {
                        self.phase = 2;
                        Ok(Self::spawn(&mut self.active, DrawingRetirementOwner::Fill(value.take().expect("Drawing fill remains exact"))))
                    }
                    _ => {
                        drop(self.owner.take());
                        Ok(store::SnapshotRetirementStep::Complete)
                    }
                },
                DrawingMutationFields::Stroke { id, value } => match self.phase {
                    0 => Ok(Self::release_string(id, &mut self.phase, 1, maximum_items, maximum_bytes)),
                    1 if value.is_some() => {
                        self.phase = 2;
                        Ok(Self::spawn(&mut self.active, DrawingRetirementOwner::Stroke(value.take().expect("Drawing stroke remains exact"))))
                    }
                    _ => {
                        drop(self.owner.take());
                        Ok(store::SnapshotRetirementStep::Complete)
                    }
                },
                DrawingMutationFields::Layer { parent, value } => match self.phase {
                    0 if parent.is_some() => {
                        self.phase = 1;
                        Ok(Self::spawn(&mut self.active, DrawingRetirementOwner::String(parent.take().expect("Drawing parent remains exact"))))
                    }
                    0 => {
                        self.phase = 1;
                        Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                    }
                    1 if value.is_some() => {
                        self.phase = 2;
                        Ok(Self::spawn(&mut self.active, DrawingRetirementOwner::Layer(*value.take().expect("Drawing layer remains exact"))))
                    }
                    _ => {
                        drop(self.owner.take());
                        Ok(store::SnapshotRetirementStep::Complete)
                    }
                },
            },
        }
    }
}

impl store::ErasedSnapshotRetirement for DrawingOwnedRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if let Some(active) = self.active.as_mut() {
            return match active.close_step(maximum_items.min(1), maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if active.terminal_is_empty() => {
                    drop(self.active.take());
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                store::SnapshotRetirementStep::Complete => Err("Drawing nested retirement reported false terminal".into()),
                step => Ok(step),
            };
        }
        self.advance(maximum_items.min(1), maximum_bytes)
    }

    fn terminal_is_empty(&self) -> bool {
        self.owner.is_none() && self.active.is_none()
    }
}

impl Drop for DrawingOwnedRetirement {
    fn drop(&mut self) {
        assert!(store::ErasedSnapshotRetirement::terminal_is_empty(self), "Drawing owner reached Drop before cursor retirement reached terminal-empty");
    }
}

pub struct DrawingSnapshotRetirementFactory;

impl store::ArtifactOwnedValueRetirementFactory<DrawingSnapshot> for DrawingSnapshotRetirementFactory {
    fn retire_owned(&self, value: DrawingSnapshot) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(DrawingOwnedRetirement::new(DrawingRetirementOwner::Snapshot(value)))
    }
}

struct DrawingSnapshotRootRetirement {
    owner: std::mem::ManuallyDrop<Option<std::sync::Arc<DrawingSnapshot>>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
}

impl store::ErasedSnapshotRetirement for DrawingSnapshotRootRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(retirement) = self.retirement.as_mut() {
            return match retirement.close_step(1, maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                    drop(self.retirement.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
                store::SnapshotRetirementStep::Complete => Err("Drawing snapshot root retirement reported false terminal".into()),
                step => Ok(step),
            };
        }
        let Some(owner) = self.owner.take() else { return Ok(store::SnapshotRetirementStep::Complete) };
        match std::sync::Arc::try_unwrap(owner) {
            Ok(value) => {
                *self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&DrawingSnapshotRetirementFactory, value));
                Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
            }
            Err(owner) => {
                *self.owner = Some(owner);
                Ok(store::SnapshotRetirementStep::Blocked)
            }
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.owner.is_none() && self.retirement.is_none()
    }
}

impl Drop for DrawingSnapshotRootRetirement {
    fn drop(&mut self) {
        assert!(self.owner.is_none() && self.retirement.is_none(), "Drawing snapshot root reached Drop before exact Arc handback");
    }
}

impl store::SnapshotRetirementFactory<DrawingSnapshot> for DrawingSnapshotRetirementFactory {
    fn retire(&self, snapshot: std::sync::Arc<DrawingSnapshot>) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(DrawingSnapshotRootRetirement { owner: std::mem::ManuallyDrop::new(Some(snapshot)), retirement: std::mem::ManuallyDrop::new(None) })
    }
}

pub struct DrawingMutationRetirementFactory;

impl store::ArtifactOwnedValueRetirementFactory<DrawingMutation> for DrawingMutationRetirementFactory {
    fn retire_owned(&self, value: DrawingMutation) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(DrawingOwnedRetirement::new(DrawingRetirementOwner::Mutation(value)))
    }
}

fn decode_drawing_snapshot_pack(bytes: &[u8]) -> Result<DrawingSnapshot, ()> {
    <DrawingSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|_| ())
}

fn decode_drawing_mutation_pack(bytes: &[u8]) -> Result<DrawingMutation, ()> {
    DrawingMutation::decode_op(bytes).map_err(|_| ())
}

macro_rules! drawing_owned_field_authority {
    ($state:ident, $authority:ident, $value:ty, $authority_trait:ident, $target_trait:ident, $publish:ident, $decode:path, $factory:expr, $kind:literal) => {
        enum $state {
            AwaitToken,
            Decode(store::OwnedSchemaHexAuthority<DRAWING_OWNED_FIELD_BYTES>),
            Ready,
            Published,
            Closing,
            Complete,
        }

        struct $authority {
            operation: semio_framework_job::OperationId,
            generation: semio_framework_job::Generation,
            path: store::OwnedSchemaPath,
            state: $state,
            value: std::mem::ManuallyDrop<Option<$value>>,
            retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
        }

        impl $authority {
            fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Self {
                Self { operation, generation, path, state: $state::AwaitToken, value: std::mem::ManuallyDrop::new(None), retirement: std::mem::ManuallyDrop::new(None) }
            }

            fn diagnostic(&self, code: &'static str, offset: u64) -> store::OwnedSchemaDecodeDiagnostic {
                store::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path: self.path }
            }
        }

        impl store::$authority_trait<$value> for $authority {
            fn accept_token(
                &mut self,
                token: store::OwnedSchemaToken,
                terminal: bool,
                source: &store::OwnedSchemaRecordCursor,
                cx: &mut semio_framework_job::StepContext<'_>,
            ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
                let path = self.path;
                let diagnostic = |code: &'static str, offset| store::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path };
                if matches!(self.state, $state::AwaitToken) {
                    if !terminal {
                        return Err(diagnostic(concat!("drawing-envelope.", $kind, "-pack-must-be-scalar"), token.start));
                    }
                    self.state = $state::Decode(store::OwnedSchemaHexAuthority::try_new(self.operation, self.generation, token, self.path)?);
                }
                let $state::Decode(authority) = &mut self.state else {
                    return Err(diagnostic(concat!("drawing-envelope.", $kind, "-pack-token-replayed"), token.start));
                };
                match authority.step(source, cx) {
                    store::OwnedSchemaHexStep::Pending => Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending),
                    store::OwnedSchemaHexStep::Complete => {
                        let bytes = authority.as_bytes().ok_or_else(|| diagnostic(concat!("drawing-envelope.", $kind, "-pack-missing"), token.start))?;
                        let value = $decode(bytes).map_err(|_| diagnostic(concat!("drawing-envelope.", $kind, "-pack-malformed"), token.start))?;
                        if !authority.release() {
                            return Err(diagnostic(concat!("drawing-envelope.", $kind, "-pack-release-duplicate"), token.start));
                        }
                        *self.value = Some(value);
                        self.state = $state::Ready;
                        Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
                    }
                    store::OwnedSchemaHexStep::Cancelled => Err(diagnostic(concat!("drawing-envelope.", $kind, "-pack-cancelled"), token.start)),
                    store::OwnedSchemaHexStep::Fault(diagnostic) => Err(diagnostic),
                }
            }

            fn publish_reserved(
                &mut self,
                target: &mut dyn store::$target_trait<$value>,
                reservation: store::ArtifactEnvelopeFieldReservation,
                _cx: &mut semio_framework_job::StepContext<'_>,
            ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
                if !matches!(self.state, $state::Ready) {
                    return Err(self.diagnostic(concat!("drawing-envelope.", $kind, "-pack-not-ready"), 0));
                }
                let value = self.value.take().ok_or_else(|| self.diagnostic(concat!("drawing-envelope.", $kind, "-owner-missing"), 0))?;
                target.$publish(reservation, value);
                self.state = $state::Published;
                Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
            }

            fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, store::OwnedSchemaDecodeDiagnostic> {
                if maximum_items == 0 {
                    return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                }
                if let $state::Decode(authority) = &mut self.state {
                    authority.cancel();
                    self.state = $state::Closing;
                    return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
                }
                if self.retirement.is_none() {
                    if let Some(value) = self.value.take() {
                        *self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned($factory, value));
                        self.state = $state::Closing;
                        return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                    }
                    self.state = $state::Complete;
                    return Ok(store::SnapshotRetirementStep::Complete);
                }
                let path = self.path;
                let retirement = self.retirement.as_mut().expect("Drawing packed field retirement remains retained");
                match retirement.close_step(maximum_items.min(1), maximum_bytes).map_err(|_| store::OwnedSchemaDecodeDiagnostic { code: concat!("drawing-envelope.", $kind, "-retirement-fault"), offset: 0, line: 0, column: 0, path })? {
                    store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                        drop(self.retirement.take());
                        self.state = $state::Complete;
                        Ok(store::SnapshotRetirementStep::Complete)
                    }
                    store::SnapshotRetirementStep::Complete => Err(self.diagnostic(concat!("drawing-envelope.", $kind, "-retirement-false-terminal"), 0)),
                    step => Ok(step),
                }
            }

            fn terminal_is_empty(&self) -> bool {
                matches!(self.state, $state::Published | $state::Complete) && self.value.is_none() && self.retirement.is_none()
            }
        }

        impl Drop for $authority {
            fn drop(&mut self) {
                assert!(matches!(self.state, $state::Published | $state::Complete) && self.value.is_none() && self.retirement.is_none(), concat!("Drawing ", $kind, " decode reached Drop before publication or bounded retirement"));
            }
        }
    };
}

drawing_owned_field_authority!(
    DrawingSnapshotDecodeState,
    DrawingSnapshotDecodeAuthority,
    DrawingSnapshot,
    ArtifactEnvelopeSnapshotFieldAuthority,
    ArtifactEnvelopeSnapshotFieldTarget,
    publish_snapshot_reserved,
    decode_drawing_snapshot_pack,
    &DrawingSnapshotRetirementFactory,
    "snapshot"
);

drawing_owned_field_authority!(
    DrawingMutationDecodeState,
    DrawingMutationDecodeAuthority,
    DrawingMutation,
    ArtifactEnvelopeMutationFieldAuthority,
    ArtifactEnvelopeMutationFieldTarget,
    publish_mutation_reserved,
    decode_drawing_mutation_pack,
    &DrawingMutationRetirementFactory,
    "mutation"
);

struct DrawingRejectedConflictAuthority {
    terminal: bool,
}

impl store::ArtifactEnvelopeSprConflictAuthority for DrawingRejectedConflictAuthority {
    fn accept_token(
        &mut self,
        token: store::OwnedSchemaToken,
        _terminal: bool,
        _source: &store::OwnedSchemaRecordCursor,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        Err(store::OwnedSchemaDecodeDiagnostic { code: "drawing-envelope.fresh-conflict-not-admitted", offset: token.start, line: 0, column: 0, path: store::OwnedSchemaPath::ROOT })
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, store::OwnedSchemaDecodeDiagnostic> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.terminal = true;
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal
    }
}

pub struct DrawingEnvelopeOwnedFieldCatalog;

impl store::ArtifactEnvelopeOwnedFieldCatalog<DrawingSnapshot, DrawingMutation> for DrawingEnvelopeOwnedFieldCatalog {
    fn begin_vcs(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeVcsFieldAuthority<DrawingSnapshot, DrawingMutation>> {
        Box::new(store::ArtifactEnvelopeFreshVcsAuthority::new(self.begin_snapshot(operation, generation, path), std::sync::Arc::new(DrawingSnapshotRetirementFactory), std::sync::Arc::new(DrawingMutationRetirementFactory), self.edit_history_decoder()))
    }

    fn begin_snapshot(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeSnapshotFieldAuthority<DrawingSnapshot>> {
        Box::new(DrawingSnapshotDecodeAuthority::new(operation, generation, path))
    }

    fn begin_mutation(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeMutationFieldAuthority<DrawingMutation>> {
        Box::new(DrawingMutationDecodeAuthority::new(operation, generation, path))
    }

    fn begin_spr_conflict(&self, _operation: semio_framework_job::OperationId, _generation: semio_framework_job::Generation, _path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeSprConflictAuthority> {
        Box::new(DrawingRejectedConflictAuthority { terminal: false })
    }

    fn edit_history_decoder(&self) -> std::sync::Arc<dyn store::ArtifactOwnedHistoryEntryDecoder<protocol::Edit<DrawingMutation>>> {
        store::artifact_owned_spr_edit_history_decoder(std::sync::Arc::new(Self), std::sync::Arc::new(DrawingMutationRetirementFactory))
    }
}

pub fn drawing_envelope_decode_owner_bundle() -> store::ArtifactEnvelopeDecodeOwnerBundle<DrawingSnapshot, DrawingMutation> {
    store::ArtifactEnvelopeDecodeOwnerBundle::new(std::sync::Arc::new(DrawingEnvelopeOwnedFieldCatalog), std::sync::Arc::new(DrawingSnapshotRetirementFactory), std::sync::Arc::new(DrawingMutationRetirementFactory))
}
//#endregion 🔖️OwnedSprCatalog

//#region 🔖️RetainedStoreInitialization
const DRAWING_MAXIMUM_NESTED_ITEMS: usize = 4_096;
const DRAWING_MAXIMUM_NESTED_BYTES: usize = store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES;
const DRAWING_MAXIMUM_LAYER_DEPTH: usize = 64;
const DRAWING_MUTATION_AGGREGATE_ITEMS: usize = DRAWING_MAXIMUM_NESTED_ITEMS;
const DRAWING_MUTATION_AGGREGATE_BYTES: usize = DRAWING_MAXIMUM_NESTED_BYTES;
const DRAWING_MUTATION_RETAINED_PAGE_ITEMS: usize = 1;
const DRAWING_MUTATION_RETAINED_PAGE_BYTES: usize = store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;
const DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY: usize = 16;
const DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY: usize = 64;
const DRAWING_MUTATION_ARENA_POOL_CAPACITY: usize = 4;
const DRAWING_DUPLICATE_MATERIAL_BYTES: usize = DRAWING_OWNED_FIELD_BYTES * 2;
const DRAWING_DUPLICATE_ID_BYTES: usize = 80;

struct DrawingMutationArenaOwner {
    reverse: Vec<DrawingLayerNode>,
    output: Vec<DrawingLayerNode>,
    pages: Vec<String>,
    duplicate_id: String,
}

impl DrawingMutationArenaOwner {
    fn admitted_totals(&self) -> Result<(usize, usize), &'static str> {
        let items = self.reverse.capacity().checked_add(self.output.capacity()).and_then(|items| items.checked_add(self.pages.capacity())).and_then(|items| items.checked_add(1)).ok_or("drawing-store.mutation-arena-item-overflow")?;
        let bytes = std::mem::size_of::<Self>()
            .checked_add(self.reverse.capacity().checked_mul(std::mem::size_of::<DrawingLayerNode>()).ok_or("drawing-store.mutation-arena-byte-overflow")?)
            .and_then(|bytes| bytes.checked_add(self.output.capacity().checked_mul(std::mem::size_of::<DrawingLayerNode>())?))
            .and_then(|bytes| bytes.checked_add(self.pages.capacity().checked_mul(std::mem::size_of::<String>())?))
            .and_then(|bytes| self.pages.iter().try_fold(bytes, |total, page| total.checked_add(page.capacity())))
            .and_then(|bytes| bytes.checked_add(self.duplicate_id.capacity()))
            .ok_or("drawing-store.mutation-arena-byte-overflow")?;
        Ok((items, bytes))
    }

    fn terminal_is_empty(&self) -> bool {
        self.reverse.is_empty()
            && self.reverse.capacity() >= DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY
            && self.output.is_empty()
            && self.output.capacity() >= DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY
            && self.pages.len() == DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY
            && self.pages.capacity() >= DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY
            && self.pages.iter().all(|page| page.is_empty() && page.capacity() >= DRAWING_MUTATION_RETAINED_PAGE_BYTES)
            && self.duplicate_id.is_empty()
            && self.duplicate_id.capacity() >= DRAWING_DUPLICATE_ID_BYTES
    }
}

struct DrawingMutationArenaOwnerBuilder {
    reverse: std::mem::ManuallyDrop<Option<Vec<DrawingLayerNode>>>,
    output: std::mem::ManuallyDrop<Option<Vec<DrawingLayerNode>>>,
    pages: std::mem::ManuallyDrop<Option<Vec<String>>>,
    duplicate_id: std::mem::ManuallyDrop<Option<String>>,
    rejected_string: std::mem::ManuallyDrop<Option<String>>,
    phase: usize,
    terminal: bool,
}

impl DrawingMutationArenaOwnerBuilder {
    fn new() -> Self {
        Self {
            reverse: std::mem::ManuallyDrop::new(None),
            output: std::mem::ManuallyDrop::new(None),
            pages: std::mem::ManuallyDrop::new(None),
            duplicate_id: std::mem::ManuallyDrop::new(None),
            rejected_string: std::mem::ManuallyDrop::new(None),
            phase: 0,
            terminal: false,
        }
    }

    fn from_owner(owner: DrawingMutationArenaOwner) -> Self {
        Self {
            reverse: std::mem::ManuallyDrop::new(Some(owner.reverse)),
            output: std::mem::ManuallyDrop::new(Some(owner.output)),
            pages: std::mem::ManuallyDrop::new(Some(owner.pages)),
            duplicate_id: std::mem::ManuallyDrop::new(Some(owner.duplicate_id)),
            rejected_string: std::mem::ManuallyDrop::new(None),
            phase: 20,
            terminal: false,
        }
    }

    fn inject(allocation: &mut usize, failure_at: Option<usize>) -> bool {
        let current = *allocation;
        *allocation += 1;
        failure_at == Some(current)
    }

    fn step(&mut self, allocation: &mut usize, failure_at: Option<usize>) -> Result<bool, &'static str> {
        if self.phase >= 20 {
            return Ok(true);
        }
        if Self::inject(allocation, failure_at) {
            return Err("drawing-store.mutation-arena-bootstrap-injected-allocation");
        }
        match self.phase {
            0 => {
                let mut value = Vec::new();
                if value.try_reserve_exact(DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY).is_err() {
                    return Err("drawing-store.mutation-reverse-arena-admission");
                }
                *self.reverse = Some(value);
            }
            1 => {
                let mut value = Vec::new();
                if value.try_reserve_exact(DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY).is_err() {
                    return Err("drawing-store.mutation-output-arena-admission");
                }
                *self.output = Some(value);
            }
            2 => {
                let mut pages = Vec::new();
                if pages.try_reserve_exact(DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY).is_err() {
                    return Err("drawing-store.mutation-overlay-arena-admission");
                }
                *self.pages = Some(pages);
            }
            phase if phase < 3 + DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY => {
                let mut page = String::new();
                if page.try_reserve_exact(DRAWING_MUTATION_RETAINED_PAGE_BYTES).is_err() {
                    return Err("drawing-store.mutation-overlay-page-admission");
                }
                let Some(pages) = self.pages.as_mut() else {
                    *self.rejected_string = Some(page);
                    return Err("drawing-store.mutation-overlay-arena-missing");
                };
                if pages.len() >= pages.capacity() {
                    *self.rejected_string = Some(page);
                    return Err("drawing-store.mutation-overlay-arena-saturated");
                }
                pages.push(page);
            }
            19 => {
                let mut value = String::new();
                if value.try_reserve_exact(DRAWING_DUPLICATE_ID_BYTES).is_err() {
                    return Err("drawing-store.duplicate-id-owner-admission");
                }
                *self.duplicate_id = Some(value);
            }
            _ => return Err("drawing-store.mutation-arena-bootstrap-phase"),
        }
        self.phase += 1;
        Ok(self.phase == 20)
    }

    fn take(&mut self) -> Option<DrawingMutationArenaOwner> {
        if self.phase != 20 || self.terminal {
            return None;
        }
        if self.rejected_string.is_some() || self.reverse.is_none() || self.output.is_none() || self.pages.is_none() || self.duplicate_id.is_none() {
            return None;
        }
        let owner = DrawingMutationArenaOwner {
            reverse: self.reverse.take().expect("validated Drawing reverse bootstrap owner remains retained"),
            output: self.output.take().expect("validated Drawing output bootstrap owner remains retained"),
            pages: self.pages.take().expect("validated Drawing page bootstrap owner remains retained"),
            duplicate_id: self.duplicate_id.take().expect("validated Drawing duplicate bootstrap owner remains retained"),
        };
        self.terminal = true;
        Some(owner)
    }

    fn close_step(&mut self) -> store::SnapshotRetirementStep {
        if let Some(value) = self.rejected_string.take() {
            let released_bytes = value.capacity();
            drop(value);
            return store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes };
        }
        if let Some(value) = self.duplicate_id.take() {
            let released_bytes = value.capacity();
            drop(value);
            return store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes };
        }
        if let Some(pages) = self.pages.as_mut() {
            if let Some(value) = pages.pop() {
                let released_bytes = value.capacity();
                drop(value);
                return store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes };
            }
        }
        if let Some(value) = self.pages.take() {
            let released_bytes = value.capacity().saturating_mul(std::mem::size_of::<String>());
            drop(value);
            return store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes };
        }
        if let Some(value) = self.output.take() {
            let released_bytes = value.capacity().saturating_mul(std::mem::size_of::<DrawingLayerNode>());
            drop(value);
            return store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes };
        }
        if let Some(value) = self.reverse.take() {
            let released_bytes = value.capacity().saturating_mul(std::mem::size_of::<DrawingLayerNode>());
            drop(value);
            return store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes };
        }
        self.terminal = true;
        store::SnapshotRetirementStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal && self.reverse.is_none() && self.output.is_none() && self.pages.is_none() && self.duplicate_id.is_none() && self.rejected_string.is_none()
    }
}

impl Drop for DrawingMutationArenaOwnerBuilder {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Drawing mutation arena owner builder reached Drop before exact construction handoff or retirement");
    }
}

struct DrawingMutationArenaPoolSlot {
    reverse: Option<Vec<DrawingLayerNode>>,
    output: Option<Vec<DrawingLayerNode>>,
    pages: Option<Vec<String>>,
    duplicate_id: Option<String>,
    generation: u64,
    leased: bool,
}

impl DrawingMutationArenaPoolSlot {
    fn new(owner: DrawingMutationArenaOwner) -> Self {
        Self { reverse: Some(owner.reverse), output: Some(owner.output), pages: Some(owner.pages), duplicate_id: Some(owner.duplicate_id), generation: 0, leased: false }
    }

    fn is_available(&self) -> bool {
        !self.leased && self.reverse.is_some() && self.output.is_some() && self.pages.is_some() && self.duplicate_id.is_some()
    }

    fn take(&mut self, generation: u64) -> Option<DrawingMutationArenaOwner> {
        if !self.is_available() {
            return None;
        }
        self.generation = generation;
        self.leased = true;
        Some(DrawingMutationArenaOwner {
            reverse: self.reverse.take().expect("available Drawing pool slot retains reverse owner"),
            output: self.output.take().expect("available Drawing pool slot retains output owner"),
            pages: self.pages.take().expect("available Drawing pool slot retains page owner"),
            duplicate_id: self.duplicate_id.take().expect("available Drawing pool slot retains duplicate owner"),
        })
    }
}

struct DrawingMutationArenaPoolState {
    slots: [DrawingMutationArenaPoolSlot; DRAWING_MUTATION_ARENA_POOL_CAPACITY],
}

struct DrawingMutationArenaPool {
    state: std::sync::Mutex<DrawingMutationArenaPoolState>,
    admitted_items: usize,
    admitted_bytes: usize,
}

struct DrawingMutationArenaPoolBootstrap {
    owners: std::mem::ManuallyDrop<[Option<DrawingMutationArenaOwner>; DRAWING_MUTATION_ARENA_POOL_CAPACITY]>,
    active: std::mem::ManuallyDrop<Option<DrawingMutationArenaOwnerBuilder>>,
    owner: usize,
    allocation: usize,
    failure_at: Option<usize>,
    failure_after_owner: Option<usize>,
    maximum_items: usize,
    maximum_bytes: usize,
    admitted_items: usize,
    admitted_bytes: usize,
    ready: bool,
    fault: Option<&'static str>,
    terminal: bool,
}

impl std::fmt::Debug for DrawingMutationArenaPoolBootstrap {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("DrawingMutationArenaPoolBootstrap").field("owner", &self.owner).field("allocation", &self.allocation).field("fault", &self.fault).finish()
    }
}

impl DrawingMutationArenaPoolBootstrap {
    fn new(failure_at: Option<usize>, failure_after_owner: Option<usize>, maximum_items: usize, maximum_bytes: usize) -> Self {
        Self {
            owners: std::mem::ManuallyDrop::new(std::array::from_fn(|_| None)),
            active: std::mem::ManuallyDrop::new(None),
            owner: 0,
            allocation: 0,
            failure_at,
            failure_after_owner,
            maximum_items,
            maximum_bytes,
            admitted_items: 0,
            admitted_bytes: 0,
            ready: false,
            fault: None,
            terminal: false,
        }
    }

    fn production(admission: DrawingMutationArenaBootstrapAdmission) -> Self {
        Self::new(None, None, admission.maximum_items, admission.maximum_bytes)
    }

    fn fail(&mut self, fault: &'static str) -> Result<bool, &'static str> {
        self.fault = Some(fault);
        Err(fault)
    }

    fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if cx.should_yield() {
            return Err("drawing-store.mutation-arena-bootstrap-budget");
        }
        if let Some(fault) = self.fault {
            return Err(fault);
        }
        if self.ready {
            return Ok(true);
        }
        if self.owner == DRAWING_MUTATION_ARENA_POOL_CAPACITY {
            if self.admitted_items > self.maximum_items || self.admitted_bytes > self.maximum_bytes {
                return self.fail("drawing-store.mutation-arena-pool-capacity");
            }
            self.ready = true;
            return Ok(true);
        }
        if self.active.is_none() {
            *self.active = Some(DrawingMutationArenaOwnerBuilder::new());
            return Ok(false);
        }
        let complete = match self.active.as_mut().expect("Drawing arena owner builder remains retained").step(&mut self.allocation, self.failure_at) {
            Ok(complete) => complete,
            Err(error) => return self.fail(error),
        };
        if !complete {
            return Ok(false);
        }
        let mut builder = self.active.take().expect("completed Drawing arena owner builder remains retained");
        let Some(owner) = builder.take() else {
            *self.active = Some(builder);
            return self.fail("drawing-store.mutation-arena-owner-false-terminal");
        };
        drop(builder);
        if !owner.terminal_is_empty() {
            self.owners[self.owner] = Some(owner);
            return self.fail("drawing-store.mutation-arena-pool-initial-owner");
        }
        let totals = owner.admitted_totals();
        let owner_index = self.owner;
        self.owners[owner_index] = Some(owner);
        self.owner += 1;
        if self.failure_after_owner == Some(owner_index) {
            return self.fail("drawing-store.mutation-arena-bootstrap-injected-owner");
        }
        let (items, bytes) = match totals {
            Ok(totals) => totals,
            Err(error) => return self.fail(error),
        };
        self.admitted_items = match self.admitted_items.checked_add(items) {
            Some(total) => total,
            None => return self.fail("drawing-store.mutation-arena-pool-item-overflow"),
        };
        self.admitted_bytes = match self.admitted_bytes.checked_add(bytes) {
            Some(total) => total,
            None => return self.fail("drawing-store.mutation-arena-pool-byte-overflow"),
        };
        Ok(false)
    }

    fn take_pool(&mut self) -> Option<std::sync::Arc<DrawingMutationArenaPool>> {
        if !self.ready || self.terminal {
            return None;
        }
        let owners = std::mem::replace(&mut *self.owners, std::array::from_fn(|_| None));
        let slots = owners.map(|owner| DrawingMutationArenaPoolSlot::new(owner.expect("validated Drawing arena bootstrap retains every owner")));
        self.terminal = true;
        Some(std::sync::Arc::new(DrawingMutationArenaPool { state: std::sync::Mutex::new(DrawingMutationArenaPoolState { slots }), admitted_items: self.admitted_items, admitted_bytes: self.admitted_bytes }))
    }

    fn close_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> store::SnapshotRetirementStep {
        if cx.should_yield() {
            return store::SnapshotRetirementStep::Blocked;
        }
        if let Some(active) = self.active.as_mut() {
            return match active.close_step() {
                store::SnapshotRetirementStep::Complete if active.terminal_is_empty() => {
                    drop(self.active.take());
                    store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }
                }
                step => step,
            };
        }
        if self.owner > 0 {
            self.owner -= 1;
            let owner = self.owners[self.owner].take().expect("Drawing bootstrap retirement cursor locates the preceding retained owner");
            *self.active = Some(DrawingMutationArenaOwnerBuilder::from_owner(owner));
            return store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 };
        }
        self.terminal = true;
        store::SnapshotRetirementStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal && self.active.is_none() && self.owners.iter().all(Option::is_none)
    }
}

impl Drop for DrawingMutationArenaPoolBootstrap {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Drawing mutation arena pool bootstrap reached Drop before exact handoff or fault retirement");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct DrawingMutationArenaBootstrapAdmission {
    maximum_items: usize,
    maximum_bytes: usize,
}

impl DrawingMutationArenaBootstrapAdmission {
    fn fixed() -> Result<Self, &'static str> {
        Ok(Self {
            maximum_items: DRAWING_MUTATION_AGGREGATE_ITEMS.checked_mul(DRAWING_MUTATION_ARENA_POOL_CAPACITY).ok_or("drawing-store.mutation-arena-bootstrap-item-claim")?,
            maximum_bytes: DRAWING_MUTATION_AGGREGATE_BYTES.checked_mul(DRAWING_MUTATION_ARENA_POOL_CAPACITY).ok_or("drawing-store.mutation-arena-bootstrap-byte-claim")?,
        })
    }
}

enum DrawingMutationArenaProcessState {
    Inert,
    Building(DrawingMutationArenaPoolBootstrap),
    Ready(std::sync::Arc<DrawingMutationArenaPool>),
    Retiring(DrawingMutationArenaPoolBootstrap),
    Fault(&'static str),
}

static DRAWING_MUTATION_ARENA_POOL: std::sync::OnceLock<std::sync::Mutex<DrawingMutationArenaProcessState>> = std::sync::OnceLock::new();
static DRAWING_MUTATION_ARENA_BOOTSTRAP_REQUESTED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DrawingMutationArenaPoolAvailability {
    Ready,
    NotReady,
    Contended,
    Fault(&'static str),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DrawingMutationArenaBorrowError {
    NotReady,
    Contended,
    Fault(&'static str),
    Invalid(&'static str),
}

impl DrawingMutationArenaBorrowError {
    fn as_str(self) -> &'static str {
        match self {
            Self::NotReady => "drawing-store.mutation-arena-bootstrap-not-ready",
            Self::Contended => "drawing-store.mutation-arena-pool-contended",
            Self::Fault(fault) | Self::Invalid(fault) => fault,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DrawingMutationArenaBootstrapStep {
    Pending { advanced_items: u64 },
    Blocked,
    Ready,
    Cancelled,
    Fault(&'static str),
}

struct DrawingMutationArenaBootstrapJob {
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    admission: DrawingMutationArenaBootstrapAdmission,
    terminal: bool,
}

impl DrawingMutationArenaBootstrapJob {
    fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Result<Self, &'static str> {
        request_drawing_mutation_arena_pool();
        Ok(Self { operation, generation, admission: DrawingMutationArenaBootstrapAdmission::fixed()?, terminal: false })
    }

    fn inactive(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Self {
        Self { operation, generation, admission: DrawingMutationArenaBootstrapAdmission { maximum_items: 0, maximum_bytes: 0 }, terminal: true }
    }

    fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> DrawingMutationArenaBootstrapStep {
        if self.terminal {
            return DrawingMutationArenaBootstrapStep::Ready;
        }
        if cx.operation() != self.operation || cx.generation() != self.generation {
            self.terminal = true;
            return DrawingMutationArenaBootstrapStep::Fault("drawing-store.mutation-arena-bootstrap-stale-authority");
        }
        if cx.should_yield() {
            return DrawingMutationArenaBootstrapStep::Blocked;
        }
        let state = DRAWING_MUTATION_ARENA_POOL.get_or_init(|| std::sync::Mutex::new(DrawingMutationArenaProcessState::Inert));
        let Ok(mut state) = state.try_lock() else {
            return DrawingMutationArenaBootstrapStep::Blocked;
        };
        self.step_locked(&mut state, cx)
    }

    fn step_locked(&mut self, state: &mut DrawingMutationArenaProcessState, cx: &mut semio_framework_job::StepContext<'_>) -> DrawingMutationArenaBootstrapStep {
        cx.set_stage("drawing-arena-bootstrap");
        if cx.is_cancelled() {
            match &*state {
                DrawingMutationArenaProcessState::Inert | DrawingMutationArenaProcessState::Ready(_) => {
                    self.terminal = true;
                    return DrawingMutationArenaBootstrapStep::Cancelled;
                }
                DrawingMutationArenaProcessState::Building(_) => {
                    let previous = std::mem::replace(&mut *state, DrawingMutationArenaProcessState::Fault("drawing-store.mutation-arena-bootstrap-transition"));
                    let DrawingMutationArenaProcessState::Building(mut bootstrap) = previous else { unreachable!("Drawing bootstrap cancellation preserves its exact building owner") };
                    bootstrap.fault = Some("drawing-store.mutation-arena-bootstrap-cancelled");
                    *state = DrawingMutationArenaProcessState::Retiring(bootstrap);
                    cx.consume_fuel(1);
                    return DrawingMutationArenaBootstrapStep::Pending { advanced_items: 1 };
                }
                DrawingMutationArenaProcessState::Retiring(_) => {}
                DrawingMutationArenaProcessState::Fault(fault) => {
                    self.terminal = true;
                    return DrawingMutationArenaBootstrapStep::Fault(*fault);
                }
            }
        }
        let transition = match &mut *state {
            DrawingMutationArenaProcessState::Inert => {
                if !DRAWING_MUTATION_ARENA_BOOTSTRAP_REQUESTED.swap(false, std::sync::atomic::Ordering::AcqRel) {
                    return DrawingMutationArenaBootstrapStep::Blocked;
                }
                *state = DrawingMutationArenaProcessState::Building(DrawingMutationArenaPoolBootstrap::production(self.admission));
                cx.consume_fuel(1);
                return DrawingMutationArenaBootstrapStep::Pending { advanced_items: 1 };
            }
            DrawingMutationArenaProcessState::Building(bootstrap) => match bootstrap.step(cx) {
                Ok(true) => DrawingMutationArenaProcessTransition::Publish,
                Ok(false) => DrawingMutationArenaProcessTransition::None,
                Err(_) => DrawingMutationArenaProcessTransition::Retire,
            },
            DrawingMutationArenaProcessState::Ready(_) => {
                self.terminal = true;
                return DrawingMutationArenaBootstrapStep::Ready;
            }
            DrawingMutationArenaProcessState::Fault(error) => {
                self.terminal = true;
                return DrawingMutationArenaBootstrapStep::Fault(*error);
            }
            DrawingMutationArenaProcessState::Retiring(bootstrap) => {
                let fault = bootstrap.fault.unwrap_or("drawing-store.mutation-arena-bootstrap-fault");
                if matches!(bootstrap.close_step(cx), store::SnapshotRetirementStep::Complete) && bootstrap.terminal_is_empty() {
                    DrawingMutationArenaProcessTransition::Fault(fault)
                } else {
                    DrawingMutationArenaProcessTransition::None
                }
            }
        };
        cx.consume_fuel(1);
        match transition {
            DrawingMutationArenaProcessTransition::None => DrawingMutationArenaBootstrapStep::Pending { advanced_items: 1 },
            DrawingMutationArenaProcessTransition::Publish => {
                let previous = std::mem::replace(&mut *state, DrawingMutationArenaProcessState::Fault("drawing-store.mutation-arena-bootstrap-transition"));
                let DrawingMutationArenaProcessState::Building(mut bootstrap) = previous else { unreachable!("Drawing arena publish transition preserves the building owner") };
                let Some(pool) = bootstrap.take_pool() else {
                    *state = DrawingMutationArenaProcessState::Retiring(bootstrap);
                    return DrawingMutationArenaBootstrapStep::Pending { advanced_items: 1 };
                };
                drop(bootstrap);
                *state = DrawingMutationArenaProcessState::Ready(pool);
                self.terminal = true;
                DrawingMutationArenaBootstrapStep::Ready
            }
            DrawingMutationArenaProcessTransition::Retire => {
                let previous = std::mem::replace(&mut *state, DrawingMutationArenaProcessState::Fault("drawing-store.mutation-arena-bootstrap-transition"));
                let DrawingMutationArenaProcessState::Building(bootstrap) = previous else { unreachable!("Drawing arena fault transition preserves the building owner") };
                *state = DrawingMutationArenaProcessState::Retiring(bootstrap);
                DrawingMutationArenaBootstrapStep::Pending { advanced_items: 1 }
            }
            DrawingMutationArenaProcessTransition::Fault(fault) => {
                let previous = std::mem::replace(&mut *state, DrawingMutationArenaProcessState::Fault(fault));
                let DrawingMutationArenaProcessState::Retiring(bootstrap) = previous else { unreachable!("Drawing arena terminal fault transition preserves the retirement owner") };
                drop(bootstrap);
                self.terminal = true;
                if fault == "drawing-store.mutation-arena-bootstrap-cancelled" {
                    DrawingMutationArenaBootstrapStep::Cancelled
                } else {
                    DrawingMutationArenaBootstrapStep::Fault(fault)
                }
            }
        }
    }
}

impl DrawingMutationArenaPool {
    #[cfg(test)]
    fn try_new() -> Result<std::sync::Arc<Self>, DrawingMutationArenaPoolBootstrap> {
        let mut bootstrap = DrawingMutationArenaPoolBootstrap::production(DrawingMutationArenaBootstrapAdmission::fixed().expect("fixed Drawing arena bootstrap claim"));
        let cancel = semio_framework_job::root_cancel_token();
        let mut preview_sequence = 0;
        for _ in 0..=DRAWING_MUTATION_ARENA_POOL_CAPACITY * 24 {
            let mut context = semio_framework_job::StepContext::new(
                semio_framework_job::OperationId(7_901),
                semio_framework_job::Generation(79),
                semio_framework_job::StepBudget::new(1, u64::MAX),
                cancel.clone(),
                semio_framework_job::default_now_us,
                &mut preview_sequence,
            );
            match bootstrap.step(&mut context) {
                Ok(true) => return Ok(bootstrap.take_pool().expect("completed Drawing arena bootstrap publishes exact pool")),
                Ok(false) => {}
                Err(_) => return Err(bootstrap),
            }
        }
        bootstrap.fault = Some("drawing-store.mutation-arena-bootstrap-turn-capacity");
        Err(bootstrap)
    }
}

enum DrawingMutationArenaProcessTransition {
    None,
    Publish,
    Retire,
    Fault(&'static str),
}

pub fn request_drawing_mutation_arena_pool() -> DrawingMutationArenaPoolAvailability {
    DRAWING_MUTATION_ARENA_BOOTSTRAP_REQUESTED.store(true, std::sync::atomic::Ordering::Release);
    let state = DRAWING_MUTATION_ARENA_POOL.get_or_init(|| std::sync::Mutex::new(DrawingMutationArenaProcessState::Inert));
    let Ok(state) = state.try_lock() else {
        return DrawingMutationArenaPoolAvailability::Contended;
    };
    match &*state {
        DrawingMutationArenaProcessState::Ready(_) => DrawingMutationArenaPoolAvailability::Ready,
        DrawingMutationArenaProcessState::Fault(fault) => DrawingMutationArenaPoolAvailability::Fault(*fault),
        DrawingMutationArenaProcessState::Inert | DrawingMutationArenaProcessState::Building(_) | DrawingMutationArenaProcessState::Retiring(_) => DrawingMutationArenaPoolAvailability::NotReady,
    }
}

pub fn drawing_mutation_arena_pool_fault() -> Option<&'static str> {
    let state = DRAWING_MUTATION_ARENA_POOL.get()?;
    let state = state.try_lock().ok()?;
    match &*state {
        DrawingMutationArenaProcessState::Fault(fault) => Some(*fault),
        DrawingMutationArenaProcessState::Inert | DrawingMutationArenaProcessState::Building(_) | DrawingMutationArenaProcessState::Ready(_) | DrawingMutationArenaProcessState::Retiring(_) => None,
    }
}

fn borrow_drawing_mutation_arena_from(pool: std::sync::Arc<DrawingMutationArenaPool>) -> Result<(std::sync::Arc<DrawingMutationArenaPool>, usize, u64, DrawingMutationArenaOwner), &'static str> {
    if pool.admitted_items == 0 || pool.admitted_bytes == 0 {
        return Err("drawing-store.mutation-arena-pool-unadmitted");
    }
    let mut state = pool.state.try_lock().map_err(|_| "drawing-store.mutation-arena-pool-contended")?;
    let slot = state.slots.iter().position(DrawingMutationArenaPoolSlot::is_available).ok_or("drawing-store.mutation-arena-pool-saturated")?;
    let generation = state.slots[slot].generation.checked_add(1).ok_or("drawing-store.mutation-arena-generation-exhausted")?;
    let owner = state.slots[slot].take(generation).ok_or("drawing-store.mutation-arena-owner-missing")?;
    drop(state);
    Ok((pool, slot, generation, owner))
}

fn borrow_drawing_mutation_arena() -> Result<(std::sync::Arc<DrawingMutationArenaPool>, usize, u64, DrawingMutationArenaOwner), DrawingMutationArenaBorrowError> {
    match request_drawing_mutation_arena_pool() {
        DrawingMutationArenaPoolAvailability::Ready => {}
        DrawingMutationArenaPoolAvailability::NotReady => return Err(DrawingMutationArenaBorrowError::NotReady),
        DrawingMutationArenaPoolAvailability::Contended => return Err(DrawingMutationArenaBorrowError::Contended),
        DrawingMutationArenaPoolAvailability::Fault(fault) => return Err(DrawingMutationArenaBorrowError::Fault(fault)),
    }
    let state = DRAWING_MUTATION_ARENA_POOL.get().ok_or(DrawingMutationArenaBorrowError::Invalid("drawing-store.mutation-arena-pool-uninitialized"))?;
    let state = state.try_lock().map_err(|_| DrawingMutationArenaBorrowError::Contended)?;
    let DrawingMutationArenaProcessState::Ready(pool) = &*state else { return Err(DrawingMutationArenaBorrowError::NotReady) };
    let pool = pool.clone();
    drop(state);
    borrow_drawing_mutation_arena_from(pool).map_err(DrawingMutationArenaBorrowError::Invalid)
}

#[derive(Clone, Copy)]
struct DrawingTraversalFrame {
    phase: u8,
    child: usize,
    string: usize,
}

impl DrawingTraversalFrame {
    const EMPTY: Self = Self { phase: 0, child: 0, string: 0 };
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct DrawingSnapshotOwnerTotals {
    source_items: usize,
    source_bytes: usize,
    candidate_items: usize,
    candidate_bytes: usize,
    maximum_container: usize,
}

#[derive(Clone, Copy)]
struct DrawingOwnerCreditSlot {
    source_items: u32,
    source_bytes: u32,
    derived_items: u32,
    derived_bytes: u32,
}

impl DrawingOwnerCreditSlot {
    const EMPTY: Self = Self { source_items: 0, source_bytes: 0, derived_items: 0, derived_bytes: 0 };
}

struct DrawingFixedOwnerCensus {
    slots: [DrawingOwnerCreditSlot; DRAWING_MAXIMUM_NESTED_ITEMS],
    length: usize,
}

impl DrawingFixedOwnerCensus {
    fn new() -> Self {
        Self { slots: [DrawingOwnerCreditSlot::EMPTY; DRAWING_MAXIMUM_NESTED_ITEMS], length: 0 }
    }

    fn admit(&mut self, source_items: usize, source_bytes: usize, derived_items: usize, derived_bytes: usize) -> Result<(), &'static str> {
        let target = self.slots.get_mut(self.length).ok_or("drawing-store.owner-census-slot-capacity")?;
        *target = DrawingOwnerCreditSlot {
            source_items: source_items.try_into().map_err(|_| "drawing-store.owner-census-item-width")?,
            source_bytes: source_bytes.try_into().map_err(|_| "drawing-store.owner-census-byte-width")?,
            derived_items: derived_items.try_into().map_err(|_| "drawing-store.owner-census-item-width")?,
            derived_bytes: derived_bytes.try_into().map_err(|_| "drawing-store.owner-census-byte-width")?,
        };
        self.length += 1;
        Ok(())
    }
}

struct DrawingAssetBoundsCursor {
    key: [u8; DRAWING_OWNED_FIELD_BYTES],
    key_len: usize,
    started: bool,
}

impl DrawingAssetBoundsCursor {
    fn new() -> Self {
        Self { key: [0; DRAWING_OWNED_FIELD_BYTES], key_len: 0, started: false }
    }

    fn next<'a>(&self, assets: &'a std::collections::BTreeMap<String, DrawingImageAsset>) -> Result<Option<(&'a String, &'a DrawingImageAsset)>, &'static str> {
        if !self.started {
            return Ok(assets.first_key_value());
        }
        let key = std::str::from_utf8(&self.key[..self.key_len]).map_err(|_| "drawing-store.preflight-asset-key-utf8")?;
        use std::ops::Bound::{Excluded, Unbounded};
        Ok(assets.range::<str, _>((Excluded(key), Unbounded)).next())
    }

    fn advance(&mut self, key: &str) -> Result<(), &'static str> {
        if key.len() > self.key.len() {
            return Err("drawing-store.preflight-asset-key-capacity");
        }
        self.key[..key.len()].copy_from_slice(key.as_bytes());
        self.key_len = key.len();
        self.started = true;
        Ok(())
    }
}

struct DrawingSnapshotBoundsAuthority {
    root: usize,
    asset_cursor: DrawingAssetBoundsCursor,
    depth: usize,
    path: [usize; DRAWING_MAXIMUM_LAYER_DEPTH],
    frames: [DrawingTraversalFrame; DRAWING_MAXIMUM_LAYER_DEPTH],
    items: usize,
    bytes: usize,
    candidate_items: usize,
    candidate_bytes: usize,
    maximum_container: usize,
    owner_census: DrawingFixedOwnerCensus,
    layers_complete: bool,
    terminal: bool,
}

impl DrawingSnapshotBoundsAuthority {
    fn new() -> Self {
        Self {
            root: 0,
            asset_cursor: DrawingAssetBoundsCursor::new(),
            depth: 0,
            path: [0; DRAWING_MAXIMUM_LAYER_DEPTH],
            frames: [DrawingTraversalFrame::EMPTY; DRAWING_MAXIMUM_LAYER_DEPTH],
            items: 0,
            bytes: 0,
            candidate_items: 0,
            candidate_bytes: 0,
            maximum_container: 0,
            owner_census: DrawingFixedOwnerCensus::new(),
            layers_complete: false,
            terminal: false,
        }
    }

    fn layer_at<'a>(root: &'a DrawingLayerNode, path: &[usize]) -> Option<&'a DrawingLayerNode> {
        let mut value = root;
        for index in path {
            let DrawingLayerNode::Group(group) = value else { return None };
            value = group.children.get(*index)?;
        }
        Some(value)
    }

    fn add(&mut self, items: usize, bytes: usize, candidate_items: usize, candidate_bytes: usize) -> Result<(), &'static str> {
        self.owner_census.admit(items, bytes, candidate_items, candidate_bytes)?;
        self.items = self.items.checked_add(items).ok_or("drawing-store.preflight-item-overflow")?;
        self.bytes = self.bytes.checked_add(bytes).ok_or("drawing-store.preflight-byte-overflow")?;
        self.candidate_items = self.candidate_items.checked_add(candidate_items).ok_or("drawing-store.preflight-candidate-item-overflow")?;
        self.candidate_bytes = self.candidate_bytes.checked_add(candidate_bytes).ok_or("drawing-store.preflight-candidate-byte-overflow")?;
        if self.items > DRAWING_MAXIMUM_NESTED_ITEMS || self.candidate_items > DRAWING_MAXIMUM_NESTED_ITEMS {
            return Err("drawing-store.preflight-item-capacity");
        }
        if self.bytes > DRAWING_MAXIMUM_NESTED_BYTES || self.candidate_bytes > DRAWING_MAXIMUM_NESTED_BYTES {
            return Err("drawing-store.preflight-byte-capacity");
        }
        Ok(())
    }

    fn string_owner(value: &String) -> (usize, usize, usize, usize) {
        (1, std::mem::size_of::<String>().saturating_add(value.capacity()), 0, 0)
    }

    fn vec_owner<T>(value: &Vec<T>) -> (usize, usize, usize, usize) {
        (1usize.saturating_add(value.capacity()), std::mem::size_of::<Vec<T>>().saturating_add(value.capacity().saturating_mul(std::mem::size_of::<T>())), 0, 0)
    }

    fn merge(target: &mut (usize, usize, usize, usize), value: (usize, usize, usize, usize)) {
        target.0 = target.0.saturating_add(value.0);
        target.1 = target.1.saturating_add(value.1);
        target.2 = target.2.saturating_add(value.2);
        target.3 = target.3.saturating_add(value.3);
    }

    fn direct_shape(layer: &DrawingLayerNode) -> (usize, usize, usize, usize) {
        let base = match layer {
            DrawingLayerNode::Shape(value) => &value.base,
            DrawingLayerNode::Path(value) => &value.base,
            DrawingLayerNode::Text(value) => &value.base,
            DrawingLayerNode::Image(value) => &value.base,
            DrawingLayerNode::Group(value) => &value.base,
            DrawingLayerNode::Boolean(value) => &value.base,
            DrawingLayerNode::Trace(value) => &value.base,
        };
        let mut total = (1, std::mem::size_of::<DrawingLayerNode>(), 0, 0);
        Self::merge(&mut total, Self::string_owner(&base.id));
        Self::merge(&mut total, Self::string_owner(&base.name));
        Self::merge(&mut total, Self::string_owner(&base.blend_mode));
        if let Some(fill) = &base.attributes.fill {
            total.0 += 1;
            total.1 += std::mem::size_of::<FillStyle>();
            match fill {
                FillStyle::Solid { .. } => {}
                FillStyle::LinearGradient { stops, .. } | FillStyle::RadialGradient { stops, .. } => Self::merge(&mut total, Self::vec_owner(stops)),
            }
        }
        if let Some(stroke) = &base.attributes.stroke {
            total.0 += 1;
            total.1 += std::mem::size_of::<StrokeStyle>();
            Self::merge(&mut total, Self::string_owner(&stroke.cap));
            Self::merge(&mut total, Self::string_owner(&stroke.join));
            if let Some(dash) = &stroke.dash {
                Self::merge(&mut total, Self::vec_owner(dash));
            }
        }
        match layer {
            DrawingLayerNode::Shape(value) => {
                Self::merge(&mut total, Self::string_owner(&value.shape_kind));
                if let Some(polygon) = &value.polygon {
                    Self::merge(&mut total, Self::vec_owner(&polygon.points));
                }
            }
            DrawingLayerNode::Path(value) => Self::merge(&mut total, Self::vec_owner(&value.segments)),
            DrawingLayerNode::Text(value) => Self::merge(&mut total, Self::string_owner(&value.content)),
            DrawingLayerNode::Image(value) => Self::merge(&mut total, Self::string_owner(&value.image_key)),
            DrawingLayerNode::Group(value) => Self::merge(&mut total, Self::vec_owner(&value.children)),
            DrawingLayerNode::Boolean(value) => {
                Self::merge(&mut total, Self::string_owner(&value.operation));
                Self::merge(&mut total, Self::vec_owner(&value.children));
            }
            DrawingLayerNode::Trace(value) => Self::merge(&mut total, Self::string_owner(&value.source_key)),
        }
        total
    }

    fn step(&mut self, source: &DrawingSnapshot, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        if !self.layers_complete {
            self.maximum_container = self.maximum_container.max(source.layers.len());
            let Some(root) = source.layers.get(self.root) else {
                self.layers_complete = true;
                let mut owners = (1, std::mem::size_of::<DrawingSnapshot>(), 0, 0);
                Self::merge(&mut owners, Self::string_owner(&source.schema));
                Self::merge(&mut owners, Self::string_owner(&source.id));
                if let Some(title) = &source.title {
                    Self::merge(&mut owners, Self::string_owner(title));
                }
                Self::merge(&mut owners, Self::vec_owner(&source.layers));
                owners.0 += 1;
                owners.1 += std::mem::size_of_val(&source.assets);
                self.add(owners.0, owners.1, owners.2, owners.3)?;
                cx.consume_fuel(1);
                return Ok(false);
            };
            let layer = Self::layer_at(root, &self.path[..self.depth]).ok_or("drawing-store.preflight-path")?;
            let frame = self.frames[self.depth];
            if frame.phase == 0 {
                let (items, bytes, candidate_items, candidate_bytes) = Self::direct_shape(layer);
                self.add(items, bytes, candidate_items, candidate_bytes)?;
                self.frames[self.depth].phase = 1;
                cx.consume_fuel(1);
                return Ok(false);
            }
            if let DrawingLayerNode::Boolean(value) = layer {
                if let Some(child) = value.children.get(frame.string) {
                    let owners = Self::string_owner(child);
                    self.add(owners.0, owners.1, owners.2, owners.3)?;
                    self.frames[self.depth].string += 1;
                    cx.consume_fuel(child.len().max(1) as u64);
                    return Ok(false);
                }
            }
            if let DrawingLayerNode::Group(value) = layer {
                self.maximum_container = self.maximum_container.max(value.children.len());
                if frame.child < value.children.len() {
                    if self.depth + 1 >= DRAWING_MAXIMUM_LAYER_DEPTH {
                        return Err("drawing-store.preflight-depth-capacity");
                    }
                    self.path[self.depth] = frame.child;
                    self.frames[self.depth].child += 1;
                    self.depth += 1;
                    self.frames[self.depth] = DrawingTraversalFrame::EMPTY;
                    cx.consume_fuel(1);
                    return Ok(false);
                }
            }
            if self.depth == 0 {
                self.root += 1;
                self.frames[0] = DrawingTraversalFrame::EMPTY;
            } else {
                self.depth -= 1;
            }
            cx.consume_fuel(1);
            return Ok(false);
        }
        let Some((key, value)) = self.asset_cursor.next(&source.assets)? else {
            self.terminal = true;
            return Ok(true);
        };
        let mut owners = (1, std::mem::size_of::<(String, DrawingImageAsset)>(), 0, 0);
        Self::merge(&mut owners, Self::string_owner(key));
        Self::merge(&mut owners, Self::string_owner(&value.mime));
        Self::merge(&mut owners, Self::string_owner(&value.data));
        self.add(owners.0, owners.1, owners.2, owners.3)?;
        self.asset_cursor.advance(key)?;
        cx.consume_fuel(1);
        Ok(false)
    }

    fn totals(&self) -> Option<DrawingSnapshotOwnerTotals> {
        self.terminal.then_some(DrawingSnapshotOwnerTotals { source_items: self.items, source_bytes: self.bytes, candidate_items: self.candidate_items, candidate_bytes: self.candidate_bytes, maximum_container: self.maximum_container })
    }
}

struct DrawingLayerCloneAuthority {
    value: std::mem::ManuallyDrop<Option<DrawingLayerNode>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    depth: usize,
    path: [usize; DRAWING_MAXIMUM_LAYER_DEPTH],
    frames: [DrawingTraversalFrame; DRAWING_MAXIMUM_LAYER_DEPTH],
    terminal: bool,
}

impl DrawingLayerCloneAuthority {
    fn clone_string(source: &str) -> Result<String, &'static str> {
        if source.len() > DRAWING_OWNED_FIELD_BYTES {
            return Err("drawing-store.initializer-field-too-large");
        }
        let mut value = String::new();
        value.try_reserve_exact(source.len()).map_err(|_| "drawing-store.initializer-string-admission")?;
        value.push_str(source);
        Ok(value)
    }

    fn clone_owned_string(source: &String) -> Result<String, &'static str> {
        if source.len() > DRAWING_OWNED_FIELD_BYTES || source.capacity() > DRAWING_OWNED_FIELD_BYTES {
            return Err("drawing-store.initializer-owned-string-capacity");
        }
        let mut value = String::new();
        value.try_reserve_exact(source.capacity()).map_err(|_| "drawing-store.initializer-owned-string-admission")?;
        value.push_str(source);
        Ok(value)
    }

    fn base_skeleton(source: &DrawingLayerBase) -> DrawingLayerBase {
        DrawingLayerBase {
            id: String::new(),
            name: String::new(),
            visible: source.visible,
            locked: source.locked,
            opacity: source.opacity,
            blend_mode: String::new(),
            transform: crate::artifacts::drawing::DrawingTransform { x: source.transform.x, y: source.transform.y, scale_x: source.transform.scale_x, scale_y: source.transform.scale_y, rotation: source.transform.rotation },
            attributes: DrawingAttributes::default(),
        }
    }

    fn rect(source: &crate::artifacts::drawing::DrawingRect) -> crate::artifacts::drawing::DrawingRect {
        crate::artifacts::drawing::DrawingRect { x: source.x, y: source.y, width: source.width, height: source.height }
    }

    fn skeleton(source: &DrawingLayerNode) -> Result<DrawingLayerNode, &'static str> {
        Ok(match source {
            DrawingLayerNode::Shape(value) => DrawingLayerNode::Shape(crate::artifacts::drawing::DrawingShapeBody {
                base: Self::base_skeleton(&value.base),
                shape_kind: String::new(),
                rect: value.rect.as_ref().map(Self::rect),
                ellipse: value.ellipse.as_ref().map(|source| crate::artifacts::drawing::DrawingEllipse { cx: source.cx, cy: source.cy, rx: source.rx, ry: source.ry }),
                circle: value.circle.as_ref().map(|source| crate::artifacts::drawing::DrawingCircle { cx: source.cx, cy: source.cy, r: source.r }),
                line: value.line.as_ref().map(|source| crate::artifacts::drawing::DrawingLine { x1: source.x1, y1: source.y1, x2: source.x2, y2: source.y2 }),
                polygon: value.polygon.as_ref().map(|source| crate::artifacts::drawing::DrawingPolygon { points: Vec::with_capacity(source.points.len()) }),
            }),
            DrawingLayerNode::Path(value) => DrawingLayerNode::Path(crate::artifacts::drawing::DrawingPathBody { base: Self::base_skeleton(&value.base), segments: Vec::with_capacity(value.segments.len()) }),
            DrawingLayerNode::Text(value) => DrawingLayerNode::Text(crate::artifacts::drawing::DrawingTextBody { base: Self::base_skeleton(&value.base), x: value.x, y: value.y, content: String::new(), size: value.size }),
            DrawingLayerNode::Image(value) => DrawingLayerNode::Image(crate::artifacts::drawing::DrawingImageBody { base: Self::base_skeleton(&value.base), image_key: String::new(), width: value.width, height: value.height }),
            DrawingLayerNode::Group(value) => DrawingLayerNode::Group(crate::artifacts::drawing::DrawingGroupBody { base: Self::base_skeleton(&value.base), children: Vec::with_capacity(value.children.len()) }),
            DrawingLayerNode::Boolean(value) => DrawingLayerNode::Boolean(crate::artifacts::drawing::DrawingBooleanBody { base: Self::base_skeleton(&value.base), operation: String::new(), children: Vec::with_capacity(value.children.len()) }),
            DrawingLayerNode::Trace(value) => DrawingLayerNode::Trace(crate::artifacts::drawing::DrawingTraceBody {
                base: Self::base_skeleton(&value.base),
                source_key: String::new(),
                params: crate::artifacts::drawing::DrawingTraceParams { threshold: value.params.threshold, simplify_epsilon: value.params.simplify_epsilon },
            }),
        })
    }

    fn new(source: &DrawingLayerNode) -> Result<Self, &'static str> {
        Ok(Self {
            value: std::mem::ManuallyDrop::new(Some(Self::skeleton(source)?)),
            retirement: std::mem::ManuallyDrop::new(None),
            depth: 0,
            path: [0; DRAWING_MAXIMUM_LAYER_DEPTH],
            frames: [DrawingTraversalFrame::EMPTY; DRAWING_MAXIMUM_LAYER_DEPTH],
            terminal: false,
        })
    }

    fn source_at<'a>(root: &'a DrawingLayerNode, path: &[usize]) -> Option<&'a DrawingLayerNode> {
        DrawingSnapshotBoundsAuthority::layer_at(root, path)
    }

    fn target_at_mut<'a>(root: &'a mut DrawingLayerNode, path: &[usize]) -> Option<&'a mut DrawingLayerNode> {
        if let Some((head, tail)) = path.split_first() {
            let DrawingLayerNode::Group(group) = root else { return None };
            return Self::target_at_mut(group.children.get_mut(*head)?, tail);
        }
        Some(root)
    }

    fn bases<'a>(source: &'a DrawingLayerNode, target: &'a mut DrawingLayerNode) -> (&'a DrawingLayerBase, &'a mut DrawingLayerBase) {
        match (source, target) {
            (DrawingLayerNode::Shape(source), DrawingLayerNode::Shape(target)) => (&source.base, &mut target.base),
            (DrawingLayerNode::Path(source), DrawingLayerNode::Path(target)) => (&source.base, &mut target.base),
            (DrawingLayerNode::Text(source), DrawingLayerNode::Text(target)) => (&source.base, &mut target.base),
            (DrawingLayerNode::Image(source), DrawingLayerNode::Image(target)) => (&source.base, &mut target.base),
            (DrawingLayerNode::Group(source), DrawingLayerNode::Group(target)) => (&source.base, &mut target.base),
            (DrawingLayerNode::Boolean(source), DrawingLayerNode::Boolean(target)) => (&source.base, &mut target.base),
            (DrawingLayerNode::Trace(source), DrawingLayerNode::Trace(target)) => (&source.base, &mut target.base),
            _ => unreachable!("Drawing clone source and target variants remain exact"),
        }
    }

    fn clone_fill(source: &FillStyle) -> FillStyle {
        match source {
            FillStyle::Solid { color } => FillStyle::Solid { color: *color },
            FillStyle::LinearGradient { x1, y1, x2, y2, stops } => FillStyle::LinearGradient { x1: *x1, y1: *y1, x2: *x2, y2: *y2, stops: Vec::with_capacity(stops.len()) },
            FillStyle::RadialGradient { cx, cy, r, stops } => FillStyle::RadialGradient { cx: *cx, cy: *cy, r: *r, stops: Vec::with_capacity(stops.len()) },
        }
    }

    fn step(&mut self, source_root: &DrawingLayerNode, digest: &mut store::ArtifactStoreInitializationDigest, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        let path = &self.path[..self.depth];
        let source = Self::source_at(source_root, path).ok_or("drawing-store.initializer-source-path")?;
        let target = Self::target_at_mut(self.value.as_mut().ok_or("drawing-store.initializer-layer-target")?, path).ok_or("drawing-store.initializer-target-path")?;
        let frame = self.frames[self.depth];
        let (source_base, target_base) = Self::bases(source, target);
        let observed: &[u8] = match frame.phase {
            0 => {
                target_base.id = Self::clone_owned_string(&source_base.id)?;
                source_base.id.as_bytes()
            }
            1 => {
                target_base.name = Self::clone_owned_string(&source_base.name)?;
                source_base.name.as_bytes()
            }
            2 => {
                target_base.blend_mode = Self::clone_string(&source_base.blend_mode)?;
                source_base.blend_mode.as_bytes()
            }
            3 => {
                target_base.attributes.fill = source_base.attributes.fill.as_ref().map(Self::clone_fill);
                &[]
            }
            4 => {
                let source_stops: &[GradientStop] = match source_base.attributes.fill.as_ref() {
                    Some(FillStyle::LinearGradient { stops, .. }) | Some(FillStyle::RadialGradient { stops, .. }) => stops,
                    _ => &[],
                };
                if let Some(stop) = source_stops.get(frame.string) {
                    let target_stops = match target_base.attributes.fill.as_mut() {
                        Some(FillStyle::LinearGradient { stops, .. }) | Some(FillStyle::RadialGradient { stops, .. }) => stops,
                        _ => return Err("drawing-store.initializer-fill-target"),
                    };
                    target_stops.push(GradientStop { offset: stop.offset, color: stop.color });
                    self.frames[self.depth].string += 1;
                    cx.consume_fuel(1);
                    return Ok(false);
                }
                self.frames[self.depth].string = 0;
                &[]
            }
            5 => {
                target_base.attributes.stroke =
                    source_base.attributes.stroke.as_ref().map(|stroke| StrokeStyle { color: stroke.color, width: stroke.width, cap: String::new(), join: String::new(), dash: stroke.dash.as_ref().map(|dash| Vec::with_capacity(dash.len())) });
                &[]
            }
            6 => {
                if let (Some(source), Some(target)) = (source_base.attributes.stroke.as_ref(), target_base.attributes.stroke.as_mut()) {
                    target.cap = Self::clone_string(&source.cap)?;
                    source.cap.as_bytes()
                } else {
                    &[]
                }
            }
            7 => {
                if let (Some(source), Some(target)) = (source_base.attributes.stroke.as_ref(), target_base.attributes.stroke.as_mut()) {
                    target.join = Self::clone_string(&source.join)?;
                    source.join.as_bytes()
                } else {
                    &[]
                }
            }
            8 => {
                let source_dash = source_base.attributes.stroke.as_ref().and_then(|stroke| stroke.dash.as_ref());
                if let Some(value) = source_dash.and_then(|dash| dash.get(frame.string)) {
                    target_base.attributes.stroke.as_mut().and_then(|stroke| stroke.dash.as_mut()).ok_or("drawing-store.initializer-dash-target")?.push(*value);
                    self.frames[self.depth].string += 1;
                    cx.consume_fuel(1);
                    return Ok(false);
                }
                self.frames[self.depth].string = 0;
                &[]
            }
            9 => match (source, target) {
                (DrawingLayerNode::Shape(source), DrawingLayerNode::Shape(target)) => {
                    target.shape_kind = Self::clone_string(&source.shape_kind)?;
                    source.shape_kind.as_bytes()
                }
                (DrawingLayerNode::Text(source), DrawingLayerNode::Text(target)) => {
                    target.content = Self::clone_string(&source.content)?;
                    source.content.as_bytes()
                }
                (DrawingLayerNode::Image(source), DrawingLayerNode::Image(target)) => {
                    target.image_key = Self::clone_string(&source.image_key)?;
                    source.image_key.as_bytes()
                }
                (DrawingLayerNode::Boolean(source), DrawingLayerNode::Boolean(target)) => {
                    target.operation = Self::clone_string(&source.operation)?;
                    source.operation.as_bytes()
                }
                (DrawingLayerNode::Trace(source), DrawingLayerNode::Trace(target)) => {
                    target.source_key = Self::clone_string(&source.source_key)?;
                    source.source_key.as_bytes()
                }
                _ => &[],
            },
            10 => {
                let index = frame.string;
                match (source, target) {
                    (DrawingLayerNode::Shape(source), DrawingLayerNode::Shape(target)) => {
                        if let Some(point) = source.polygon.as_ref().and_then(|polygon| polygon.points.get(index)) {
                            target.polygon.as_mut().ok_or("drawing-store.initializer-polygon-target")?.points.push(*point);
                            self.frames[self.depth].string += 1;
                            cx.consume_fuel(1);
                            return Ok(false);
                        }
                    }
                    (DrawingLayerNode::Path(source), DrawingLayerNode::Path(target)) => {
                        if let Some(segment) = source.segments.get(index) {
                            target.segments.push(match segment {
                                PathSegment::Move { to } => PathSegment::Move { to: *to },
                                PathSegment::Line { to } => PathSegment::Line { to: *to },
                                PathSegment::Quad { ctrl, to } => PathSegment::Quad { ctrl: *ctrl, to: *to },
                                PathSegment::Cubic { ctrl1, ctrl2, to } => PathSegment::Cubic { ctrl1: *ctrl1, ctrl2: *ctrl2, to: *to },
                                PathSegment::Arc { rx, ry, rotation, large_arc, sweep, to } => PathSegment::Arc { rx: *rx, ry: *ry, rotation: *rotation, large_arc: *large_arc, sweep: *sweep, to: *to },
                                PathSegment::Close => PathSegment::Close,
                            });
                            self.frames[self.depth].string += 1;
                            cx.consume_fuel(1);
                            return Ok(false);
                        }
                    }
                    (DrawingLayerNode::Boolean(source), DrawingLayerNode::Boolean(target)) => {
                        if let Some(value) = source.children.get(index) {
                            target.children.push(Self::clone_string(value)?);
                            self.frames[self.depth].string += 1;
                            digest.observe(value.as_bytes());
                            cx.consume_fuel(value.len().max(1) as u64);
                            return Ok(false);
                        }
                    }
                    _ => {}
                }
                self.frames[self.depth].string = 0;
                &[]
            }
            11 => {
                if let (DrawingLayerNode::Group(source), DrawingLayerNode::Group(target)) = (source, target) {
                    if let Some(child) = source.children.get(frame.child) {
                        if self.depth + 1 >= DRAWING_MAXIMUM_LAYER_DEPTH {
                            return Err("drawing-store.initializer-depth-capacity");
                        }
                        target.children.push(Self::skeleton(child)?);
                        self.path[self.depth] = frame.child;
                        self.frames[self.depth].child += 1;
                        self.depth += 1;
                        self.frames[self.depth] = DrawingTraversalFrame::EMPTY;
                        cx.consume_fuel(1);
                        return Ok(false);
                    }
                }
                &[]
            }
            _ => {
                if self.depth == 0 {
                    self.terminal = true;
                    return Ok(true);
                }
                self.depth -= 1;
                cx.consume_fuel(1);
                return Ok(false);
            }
        };
        digest.observe(observed);
        self.frames[self.depth].phase += 1;
        cx.consume_fuel(observed.len().max(1) as u64);
        Ok(false)
    }

    fn take(&mut self) -> Option<DrawingLayerNode> {
        self.terminal.then(|| self.value.take()).flatten()
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.retirement.is_none() {
            if let Some(value) = self.value.take() {
                *self.retirement = Some(Box::new(DrawingOwnedRetirement::new(DrawingRetirementOwner::Layer(value))));
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            return Ok(store::SnapshotRetirementStep::Complete);
        }
        let retirement = self.retirement.as_mut().expect("Drawing layer clone retirement remains exact");
        match retirement.close_step(1, maximum_bytes)? {
            store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                drop(self.retirement.take());
                Ok(store::SnapshotRetirementStep::Complete)
            }
            store::SnapshotRetirementStep::Complete => Err("Drawing layer clone retirement reported false terminal".into()),
            step => Ok(step),
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.value.is_none() && self.retirement.is_none()
    }
}

impl Drop for DrawingLayerCloneAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Drawing layer clone reached Drop before exact handoff or cursor retirement");
    }
}

struct DrawingSnapshotCloneAuthority {
    value: std::mem::ManuallyDrop<Option<DrawingSnapshot>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    layer: std::mem::ManuallyDrop<Option<Box<DrawingLayerCloneAuthority>>>,
    pending_asset: std::mem::ManuallyDrop<Option<(String, DrawingImageAsset)>>,
    bounds: DrawingSnapshotBoundsAuthority,
    phase: u8,
    index: usize,
    field: u8,
    terminal: bool,
}

impl DrawingSnapshotCloneAuthority {
    fn new() -> Self {
        Self {
            value: std::mem::ManuallyDrop::new(Some(DrawingSnapshot { schema: String::new(), id: String::new(), title: None, layers: Vec::new(), assets: std::collections::BTreeMap::new(), artboard: None })),
            retirement: std::mem::ManuallyDrop::new(None),
            layer: std::mem::ManuallyDrop::new(None),
            pending_asset: std::mem::ManuallyDrop::new(None),
            bounds: DrawingSnapshotBoundsAuthority::new(),
            phase: 0,
            index: 0,
            field: 0,
            terminal: false,
        }
    }

    fn clone_string(source: &str) -> Result<String, &'static str> {
        if source.len() > DRAWING_OWNED_FIELD_BYTES {
            return Err("drawing-store.initializer-field-too-large");
        }
        let mut value = String::new();
        value.try_reserve_exact(source.len()).map_err(|_| "drawing-store.initializer-string-admission")?;
        value.push_str(source);
        Ok(value)
    }

    fn step(&mut self, source: &DrawingSnapshot, digest: &mut store::ArtifactStoreInitializationDigest, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.phase == 0 {
            if self.bounds.step(source, cx)? {
                self.phase = 1;
            }
            return Ok(false);
        }
        let target = self.value.as_mut().ok_or("drawing-store.initializer-clone-target")?;
        let observed = match self.phase {
            1 => {
                target.schema = Self::clone_string(&source.schema)?;
                source.schema.as_bytes()
            }
            2 => {
                target.id = Self::clone_string(&source.id)?;
                source.id.as_bytes()
            }
            3 => {
                target.title = match source.title.as_deref() {
                    Some(value) => Some(Self::clone_string(value)?),
                    None => None,
                };
                source.title.as_deref().unwrap_or_default().as_bytes()
            }
            4 => {
                if let Some(layer) = self.layer.as_mut() {
                    if layer.step(source.layers.get(self.index).ok_or("drawing-store.initializer-layer-source")?, digest, cx)? {
                        let value = layer.take().ok_or("drawing-store.initializer-layer-handoff")?;
                        drop(self.layer.take());
                        target.layers.push(value);
                        self.index += 1;
                    }
                    return Ok(false);
                }
                if let Some(layer_source) = source.layers.get(self.index) {
                    if self.index == 0 {
                        target.layers.try_reserve_exact(source.layers.len()).map_err(|_| "drawing-store.initializer-layer-admission")?;
                    }
                    *self.layer = Some(Box::new(DrawingLayerCloneAuthority::new(layer_source)?));
                    cx.consume_fuel(1);
                    return Ok(false);
                }
                self.index = 0;
                &[]
            }
            5 => {
                if self.pending_asset.is_none() {
                    use std::ops::Bound::{Excluded, Unbounded};
                    let next = match target.assets.last_key_value() {
                        Some((key, _)) => source.assets.range::<str, _>((Excluded(key.as_str()), Unbounded)).next(),
                        None => source.assets.iter().next(),
                    };
                    let Some((key, value)) = next else {
                        self.phase = 6;
                        self.field = 0;
                        return Ok(false);
                    };
                    *self.pending_asset = Some((Self::clone_string(key)?, DrawingImageAsset { mime: String::new(), data: String::new(), width: value.width, height: value.height }));
                    self.field = 0;
                    digest.observe(key.as_bytes());
                    cx.consume_fuel(key.len().max(1) as u64);
                    return Ok(false);
                }
                let (key, pending) = self.pending_asset.as_mut().expect("Drawing pending asset remains exact");
                let source = source.assets.get(key).ok_or("drawing-store.initializer-asset-source")?;
                match self.field {
                    0 => {
                        pending.mime = Self::clone_string(&source.mime)?;
                        self.field = 1;
                        source.mime.as_bytes()
                    }
                    1 => {
                        pending.data = Self::clone_string(&source.data)?;
                        self.field = 2;
                        source.data.as_bytes()
                    }
                    _ => {
                        let (key, value) = self.pending_asset.take().expect("Drawing pending asset handoff remains exact");
                        if target.assets.insert(key, value).is_some() {
                            return Err("drawing-store.initializer-duplicate-asset");
                        }
                        self.field = 0;
                        cx.consume_fuel(1);
                        return Ok(false);
                    }
                }
            }
            6 => {
                target.artboard = source.artboard.as_ref().map(|value| crate::artifacts::drawing::DrawingArtboard { width: value.width, height: value.height });
                &[]
            }
            _ => {
                self.terminal = true;
                return Ok(true);
            }
        };
        digest.observe(observed);
        self.phase += 1;
        cx.consume_fuel(observed.len().max(1) as u64);
        Ok(false)
    }

    fn take_value(&mut self) -> Option<DrawingSnapshot> {
        if !self.terminal {
            return None;
        }
        self.value.take()
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(retirement) = self.retirement.as_mut() {
            return match store::ErasedSnapshotRetirement::close_step(retirement.as_mut(), 1, maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if store::ErasedSnapshotRetirement::terminal_is_empty(retirement.as_ref()) => {
                    drop(self.retirement.take());
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                store::SnapshotRetirementStep::Complete => Err("Drawing clone retirement reported false terminal".into()),
                step => Ok(step),
            };
        }
        if let Some(layer) = self.layer.as_mut() {
            return match layer.close_step(1, maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if layer.terminal_is_empty() => {
                    drop(self.layer.take());
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                store::SnapshotRetirementStep::Complete => Err("Drawing active layer clone reported false terminal".into()),
                step => Ok(step),
            };
        }
        if let Some((key, value)) = self.pending_asset.take() {
            *self.retirement = Some(Box::new(DrawingOwnedRetirement::new(DrawingRetirementOwner::AssetEntry { key, value: Some(value) })));
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(value) = self.value.take() {
            *self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&DrawingSnapshotRetirementFactory, value));
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.terminal = true;
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal && self.value.is_none() && self.retirement.is_none() && self.layer.is_none() && self.pending_asset.is_none()
    }
}

impl Drop for DrawingSnapshotCloneAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Drawing snapshot clone reached Drop before exact handoff or cursor retirement");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct DrawingLayerAddress {
    length: usize,
    indices: [usize; DRAWING_MAXIMUM_LAYER_DEPTH],
}

impl DrawingLayerAddress {
    fn parent(self) -> Option<Self> {
        (self.length > 1).then(|| Self { length: self.length - 1, indices: self.indices })
    }

    fn index(self) -> usize {
        self.indices[self.length - 1]
    }
}

struct DrawingLayerLocator {
    root: usize,
    depth: usize,
    path: [usize; DRAWING_MAXIMUM_LAYER_DEPTH],
    frames: [DrawingTraversalFrame; DRAWING_MAXIMUM_LAYER_DEPTH],
    found: Option<DrawingLayerAddress>,
    terminal: bool,
}

impl DrawingLayerLocator {
    fn new() -> Self {
        Self { root: 0, depth: 0, path: [0; DRAWING_MAXIMUM_LAYER_DEPTH], frames: [DrawingTraversalFrame::EMPTY; DRAWING_MAXIMUM_LAYER_DEPTH], found: None, terminal: false }
    }

    fn node_at<'a>(snapshot: &'a DrawingSnapshot, address: DrawingLayerAddress) -> Option<&'a DrawingLayerNode> {
        let mut value = snapshot.layers.get(address.indices[0])?;
        for index in &address.indices[1..address.length] {
            let DrawingLayerNode::Group(group) = value else { return None };
            value = group.children.get(*index)?;
        }
        Some(value)
    }

    fn node_at_mut<'a>(snapshot: &'a mut DrawingSnapshot, address: DrawingLayerAddress) -> Option<&'a mut DrawingLayerNode> {
        fn descend<'a>(value: &'a mut DrawingLayerNode, path: &[usize]) -> Option<&'a mut DrawingLayerNode> {
            let Some((head, tail)) = path.split_first() else { return Some(value) };
            let DrawingLayerNode::Group(group) = value else { return None };
            descend(group.children.get_mut(*head)?, tail)
        }
        let value = snapshot.layers.get_mut(address.indices[0])?;
        descend(value, &address.indices[1..address.length])
    }

    fn container_mut<'a>(snapshot: &'a mut DrawingSnapshot, parent: Option<DrawingLayerAddress>) -> Option<&'a mut Vec<DrawingLayerNode>> {
        match parent {
            None => Some(&mut snapshot.layers),
            Some(address) => match Self::node_at_mut(snapshot, address)? {
                DrawingLayerNode::Group(group) => Some(&mut group.children),
                _ => None,
            },
        }
    }

    fn step(&mut self, snapshot: &DrawingSnapshot, target: &str, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        let Some(root) = snapshot.layers.get(self.root) else {
            self.terminal = true;
            return Ok(true);
        };
        let node = DrawingSnapshotBoundsAuthority::layer_at(root, &self.path[..self.depth]).ok_or("drawing-store.mutation-locator-path")?;
        if self.frames[self.depth].phase == 0 {
            self.frames[self.depth].phase = 1;
            if crate::artifacts::drawing::schema::layer_id(node) == target {
                let mut indices = [0; DRAWING_MAXIMUM_LAYER_DEPTH];
                indices[0] = self.root;
                if self.depth > 0 {
                    indices[1..self.depth + 1].copy_from_slice(&self.path[..self.depth]);
                }
                self.found = Some(DrawingLayerAddress { length: self.depth + 1, indices });
                self.terminal = true;
            }
            cx.consume_fuel(1);
            return Ok(self.terminal);
        }
        if let DrawingLayerNode::Group(group) = node {
            let child = self.frames[self.depth].child;
            if child < group.children.len() {
                if self.depth + 1 >= DRAWING_MAXIMUM_LAYER_DEPTH {
                    return Err("drawing-store.mutation-locator-depth");
                }
                self.frames[self.depth].child += 1;
                self.path[self.depth] = child;
                self.depth += 1;
                self.frames[self.depth] = DrawingTraversalFrame::EMPTY;
                cx.consume_fuel(1);
                return Ok(false);
            }
        }
        if self.depth == 0 {
            self.root += 1;
            self.frames[0] = DrawingTraversalFrame::EMPTY;
        } else {
            self.depth -= 1;
        }
        cx.consume_fuel(1);
        Ok(false)
    }

    fn found(&self) -> Option<DrawingLayerAddress> {
        self.found
    }
}

const DRAWING_CONTAINER_REBUILD_MOVE_CAPACITY: usize = DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY * 4 + 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DrawingContainerRebuildMove {
    Empty,
    SourceToReverse,
    PendingToOutput,
    ReverseToOutput,
    ReverseToRemoved,
    OutputToReverse,
    ReverseToSource,
}

struct DrawingContainerRebuildAuthority {
    source: std::mem::ManuallyDrop<Option<Vec<DrawingLayerNode>>>,
    reverse: std::mem::ManuallyDrop<Option<Vec<DrawingLayerNode>>>,
    output: std::mem::ManuallyDrop<Option<Vec<DrawingLayerNode>>>,
    pending: std::mem::ManuallyDrop<Option<DrawingLayerNode>>,
    removed: std::mem::ManuallyDrop<Option<DrawingLayerNode>>,
    moves: [DrawingContainerRebuildMove; DRAWING_CONTAINER_REBUILD_MOVE_CAPACITY],
    move_count: usize,
    rollback_cursor: Option<usize>,
    remove_index: Option<usize>,
    insert_index: Option<usize>,
    original_index: usize,
    phase: u8,
    terminal: bool,
}

struct DrawingContainerRebuildRejected {
    source: Vec<DrawingLayerNode>,
    pending: Option<DrawingLayerNode>,
    reverse: Vec<DrawingLayerNode>,
    output: Vec<DrawingLayerNode>,
}

impl DrawingContainerRebuildAuthority {
    fn new(
        source: Vec<DrawingLayerNode>,
        remove_index: Option<usize>,
        insert_index: Option<usize>,
        pending: Option<DrawingLayerNode>,
        reverse: Vec<DrawingLayerNode>,
        output: Vec<DrawingLayerNode>,
        reservation: DrawingMutationAggregateReservation,
    ) -> Result<Self, DrawingContainerRebuildRejected> {
        let extra = usize::from(pending.is_some());
        let Some(output_capacity) = source.len().saturating_sub(usize::from(remove_index.is_some())).checked_add(extra) else {
            return Err(DrawingContainerRebuildRejected { source, pending, reverse, output });
        };
        if output_capacity > DRAWING_MAXIMUM_NESTED_ITEMS
            || source.len().saturating_add(output_capacity) > reservation.container_slots
            || source.len() > reservation.maximum_container.saturating_add(1)
            || output_capacity > reservation.maximum_container.saturating_add(1)
            || source.capacity() < output_capacity
            || reverse.capacity() < source.len().max(output_capacity)
            || output.capacity() < output_capacity
            || !reverse.is_empty()
            || !output.is_empty()
        {
            return Err(DrawingContainerRebuildRejected { source, pending, reverse, output });
        }
        Ok(Self {
            source: std::mem::ManuallyDrop::new(Some(source)),
            reverse: std::mem::ManuallyDrop::new(Some(reverse)),
            output: std::mem::ManuallyDrop::new(Some(output)),
            pending: std::mem::ManuallyDrop::new(pending),
            removed: std::mem::ManuallyDrop::new(None),
            moves: [DrawingContainerRebuildMove::Empty; DRAWING_CONTAINER_REBUILD_MOVE_CAPACITY],
            move_count: 0,
            rollback_cursor: None,
            remove_index,
            insert_index,
            original_index: 0,
            phase: 0,
            terminal: false,
        })
    }

    fn reserve_move(&self) -> Result<(), &'static str> {
        self.moves.get(self.move_count).map(|_| ()).ok_or("drawing-store.container-move-capacity")
    }

    fn record_reserved_move(&mut self, value: DrawingContainerRebuildMove) {
        self.moves[self.move_count] = value;
        self.move_count += 1;
    }

    fn advance(&mut self) -> Result<(bool, u64), &'static str> {
        if self.terminal {
            return Ok((true, 0));
        }
        if self.rollback_cursor.is_some() {
            return Err("drawing-store.container-advance-after-rollback");
        }
        if self.source.is_none() || self.reverse.is_none() || self.output.is_none() {
            return Err("drawing-store.container-owner-missing");
        }
        if self.phase == 0 {
            if !self.source.as_ref().expect("validated Drawing source remains retained").is_empty() {
                self.reserve_move()?;
                let value = self.source.as_mut().expect("validated Drawing source remains retained").pop().expect("nonempty Drawing source yields one owner");
                self.reverse.as_mut().expect("validated Drawing reverse remains retained").push(value);
                self.record_reserved_move(DrawingContainerRebuildMove::SourceToReverse);
                return Ok((false, 1));
            }
            self.phase = 1;
            return Ok((false, 0));
        }
        if self.phase == 1 {
            if self.pending.is_some() && self.insert_index.is_some_and(|index| index.min(self.reverse.as_ref().map_or(0, Vec::len) + self.original_index) == self.output.as_ref().map_or(0, Vec::len)) {
                self.reserve_move()?;
                self.output.as_mut().expect("validated Drawing output remains retained").push(self.pending.take().expect("Drawing insertion owner remains retained"));
                self.record_reserved_move(DrawingContainerRebuildMove::PendingToOutput);
                return Ok((false, 1));
            }
            if !self.reverse.as_ref().expect("validated Drawing reverse remains retained").is_empty() {
                if self.remove_index == Some(self.original_index) && self.removed.is_some() {
                    return Err("drawing-store.container-duplicate-removal");
                }
                self.reserve_move()?;
                let value = self.reverse.as_mut().expect("validated Drawing reverse remains retained").pop().expect("nonempty Drawing reverse yields one owner");
                if self.remove_index == Some(self.original_index) {
                    *self.removed = Some(value);
                    self.record_reserved_move(DrawingContainerRebuildMove::ReverseToRemoved);
                } else {
                    self.output.as_mut().expect("validated Drawing output remains retained").push(value);
                    self.record_reserved_move(DrawingContainerRebuildMove::ReverseToOutput);
                }
                self.original_index += 1;
                return Ok((false, 1));
            }
            if self.pending.is_some() {
                self.reserve_move()?;
                let value = self.pending.take().expect("validated Drawing pending owner remains retained");
                self.output.as_mut().expect("validated Drawing output remains retained").push(value);
                self.record_reserved_move(DrawingContainerRebuildMove::PendingToOutput);
                return Ok((false, 1));
            }
            self.phase = 2;
            return Ok((false, 0));
        }
        if self.phase == 2 {
            if !self.output.as_ref().expect("validated Drawing output remains retained").is_empty() {
                self.reserve_move()?;
                let value = self.output.as_mut().expect("validated Drawing output remains retained").pop().expect("nonempty Drawing output yields one owner");
                self.reverse.as_mut().expect("validated Drawing reverse remains retained").push(value);
                self.record_reserved_move(DrawingContainerRebuildMove::OutputToReverse);
                return Ok((false, 1));
            }
            self.phase = 3;
            return Ok((false, 0));
        }
        if !self.reverse.as_ref().expect("validated Drawing reverse remains retained").is_empty() {
            self.reserve_move()?;
            let value = self.reverse.as_mut().expect("validated Drawing reverse remains retained").pop().expect("nonempty Drawing reverse yields one owner");
            self.source.as_mut().expect("validated Drawing source remains retained").push(value);
            self.record_reserved_move(DrawingContainerRebuildMove::ReverseToSource);
            return Ok((false, 1));
        }
        self.terminal = true;
        Ok((true, 0))
    }

    fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        let (complete, fuel) = self.advance()?;
        if fuel > 0 {
            cx.consume_fuel(fuel);
        }
        Ok(complete)
    }

    fn close_forward_step(&mut self) -> Result<bool, &'static str> {
        self.advance().map(|(complete, _)| complete)
    }

    fn take(&mut self) -> Option<(Vec<DrawingLayerNode>, Option<DrawingLayerNode>, Vec<DrawingLayerNode>, Vec<DrawingLayerNode>)> {
        self.terminal.then(|| {
            (
                self.source.take().expect("Drawing rebuilt source container remains retained"),
                self.removed.take(),
                self.reverse.take().expect("Drawing emptied reverse arena remains retained"),
                self.output.take().expect("Drawing emptied output arena remains retained"),
            )
        })
    }

    fn rollback_step(&mut self) -> Result<bool, &'static str> {
        if self.source.is_none() || self.reverse.is_none() || self.output.is_none() {
            return Err("drawing-store.container-rollback-owner-missing");
        }
        let cursor = *self.rollback_cursor.get_or_insert(self.move_count);
        if cursor == 0 {
            self.phase = 0;
            self.original_index = 0;
            return Ok(true);
        }
        let index = cursor - 1;
        match self.moves[index] {
            DrawingContainerRebuildMove::SourceToReverse => {
                let value = self.reverse.as_mut().ok_or("drawing-store.container-reverse")?.pop().ok_or("drawing-store.container-rollback-reverse")?;
                self.source.as_mut().ok_or("drawing-store.container-source")?.push(value);
            }
            DrawingContainerRebuildMove::PendingToOutput => {
                if self.pending.is_some() {
                    return Err("drawing-store.container-rollback-pending");
                }
                let value = self.output.as_mut().ok_or("drawing-store.container-output")?.pop().ok_or("drawing-store.container-rollback-output")?;
                *self.pending = Some(value);
            }
            DrawingContainerRebuildMove::ReverseToOutput => {
                let value = self.output.as_mut().ok_or("drawing-store.container-output")?.pop().ok_or("drawing-store.container-rollback-output")?;
                self.reverse.as_mut().ok_or("drawing-store.container-reverse")?.push(value);
            }
            DrawingContainerRebuildMove::ReverseToRemoved => {
                let value = self.removed.take().ok_or("drawing-store.container-rollback-removed")?;
                self.reverse.as_mut().ok_or("drawing-store.container-reverse")?.push(value);
            }
            DrawingContainerRebuildMove::OutputToReverse => {
                let value = self.reverse.as_mut().ok_or("drawing-store.container-reverse")?.pop().ok_or("drawing-store.container-rollback-reverse")?;
                self.output.as_mut().ok_or("drawing-store.container-output")?.push(value);
            }
            DrawingContainerRebuildMove::ReverseToSource => {
                let value = self.source.as_mut().ok_or("drawing-store.container-source")?.pop().ok_or("drawing-store.container-rollback-source")?;
                self.reverse.as_mut().ok_or("drawing-store.container-reverse")?.push(value);
            }
            DrawingContainerRebuildMove::Empty => return Err("drawing-store.container-rollback-empty-move"),
        }
        self.moves[index] = DrawingContainerRebuildMove::Empty;
        self.rollback_cursor = Some(index);
        Ok(false)
    }

    fn rollback_complete(&self) -> bool {
        self.rollback_cursor == Some(0)
    }

    #[cfg(test)]
    fn recorded_move_in_phase(&self, phase: u8) -> bool {
        let Some(index) = self.move_count.checked_sub(1) else { return false };
        self.phase == phase
            && match phase {
                0 => self.moves[index] == DrawingContainerRebuildMove::SourceToReverse,
                1 => matches!(self.moves[index], DrawingContainerRebuildMove::PendingToOutput | DrawingContainerRebuildMove::ReverseToOutput | DrawingContainerRebuildMove::ReverseToRemoved),
                2 => self.moves[index] == DrawingContainerRebuildMove::OutputToReverse,
                3 => self.moves[index] == DrawingContainerRebuildMove::ReverseToSource,
                _ => false,
            }
    }

    fn finish_handoff(&mut self) -> Result<(), &'static str> {
        if !(self.terminal || self.rollback_complete()) || self.source.is_some() || self.reverse.is_some() || self.output.is_some() || self.pending.is_some() || self.removed.is_some() {
            return Err("drawing-store.container-rollback-handoff-incomplete");
        }
        self.terminal = true;
        Ok(())
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal && self.source.is_none() && self.reverse.is_none() && self.output.is_none() && self.pending.is_none() && self.removed.is_none()
    }
}

impl Drop for DrawingContainerRebuildAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Drawing container rebuild reached Drop before exact handoff or cursor retirement");
    }
}

struct DrawingFillCloneAuthority {
    value: std::mem::ManuallyDrop<Option<FillStyle>>,
    retirement: std::mem::ManuallyDrop<Option<Box<DrawingOwnedRetirement>>>,
    index: usize,
    terminal: bool,
}

impl DrawingFillCloneAuthority {
    fn new(source: &FillStyle) -> Result<Self, &'static str> {
        let value = match source {
            FillStyle::Solid { color } => FillStyle::Solid { color: *color },
            FillStyle::LinearGradient { x1, y1, x2, y2, stops } => {
                if stops.len() > DRAWING_MAXIMUM_NESTED_ITEMS {
                    return Err("drawing-store.fill-stop-capacity");
                }
                let mut target = Vec::new();
                target.try_reserve_exact(stops.len()).map_err(|_| "drawing-store.fill-stop-admission")?;
                FillStyle::LinearGradient { x1: *x1, y1: *y1, x2: *x2, y2: *y2, stops: target }
            }
            FillStyle::RadialGradient { cx, cy, r, stops } => {
                if stops.len() > DRAWING_MAXIMUM_NESTED_ITEMS {
                    return Err("drawing-store.fill-stop-capacity");
                }
                let mut target = Vec::new();
                target.try_reserve_exact(stops.len()).map_err(|_| "drawing-store.fill-stop-admission")?;
                FillStyle::RadialGradient { cx: *cx, cy: *cy, r: *r, stops: target }
            }
        };
        Ok(Self { value: std::mem::ManuallyDrop::new(Some(value)), retirement: std::mem::ManuallyDrop::new(None), index: 0, terminal: false })
    }

    fn step(&mut self, source: &FillStyle, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        let source_stops = match source {
            FillStyle::LinearGradient { stops, .. } | FillStyle::RadialGradient { stops, .. } => stops.as_slice(),
            FillStyle::Solid { .. } => &[],
        };
        if let Some(stop) = source_stops.get(self.index) {
            let target_stops = match self.value.as_mut().ok_or("drawing-store.fill-target")? {
                FillStyle::LinearGradient { stops, .. } | FillStyle::RadialGradient { stops, .. } => stops,
                FillStyle::Solid { .. } => return Err("drawing-store.fill-variant"),
            };
            target_stops.push(GradientStop { offset: stop.offset, color: stop.color });
            self.index += 1;
            cx.consume_fuel(1);
            return Ok(false);
        }
        self.terminal = true;
        Ok(true)
    }

    fn take(&mut self) -> Option<FillStyle> {
        self.terminal.then(|| self.value.take()).flatten()
    }

    fn close_step(&mut self, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if let Some(retirement) = self.retirement.as_mut() {
            return match store::ErasedSnapshotRetirement::close_step(retirement.as_mut(), 1, maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if store::ErasedSnapshotRetirement::terminal_is_empty(retirement.as_ref()) => {
                    drop(self.retirement.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
                store::SnapshotRetirementStep::Complete => Err("Drawing fill clone reported false terminal".into()),
                step => Ok(step),
            };
        }
        if let Some(value) = self.value.take() {
            *self.retirement = Some(Box::new(DrawingOwnedRetirement::new(DrawingRetirementOwner::Fill(value))));
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.terminal = true;
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal && self.value.is_none() && self.retirement.is_none()
    }
}

impl Drop for DrawingFillCloneAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Drawing fill clone reached Drop before exact handoff or retirement");
    }
}

struct DrawingStrokeCloneAuthority {
    value: std::mem::ManuallyDrop<Option<StrokeStyle>>,
    retirement: std::mem::ManuallyDrop<Option<Box<DrawingOwnedRetirement>>>,
    phase: u8,
    index: usize,
    terminal: bool,
}

impl DrawingStrokeCloneAuthority {
    fn new(source: &StrokeStyle) -> Result<Self, &'static str> {
        let dash = match source.dash.as_ref() {
            Some(values) if values.len() <= DRAWING_MAXIMUM_NESTED_ITEMS => {
                let mut target = Vec::new();
                target.try_reserve_exact(values.len()).map_err(|_| "drawing-store.stroke-dash-admission")?;
                Some(target)
            }
            Some(_) => return Err("drawing-store.stroke-dash-capacity"),
            None => None,
        };
        Ok(Self { value: std::mem::ManuallyDrop::new(Some(StrokeStyle { color: source.color, width: source.width, cap: String::new(), join: String::new(), dash })), retirement: std::mem::ManuallyDrop::new(None), phase: 0, index: 0, terminal: false })
    }

    fn step(&mut self, source: &StrokeStyle, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        let target = self.value.as_mut().ok_or("drawing-store.stroke-target")?;
        match self.phase {
            0 => {
                target.cap = DrawingSnapshotCloneAuthority::clone_string(&source.cap)?;
                self.phase = 1;
                cx.consume_fuel(source.cap.len().max(1) as u64);
            }
            1 => {
                target.join = DrawingSnapshotCloneAuthority::clone_string(&source.join)?;
                self.phase = 2;
                cx.consume_fuel(source.join.len().max(1) as u64);
            }
            2 => {
                if let Some(value) = source.dash.as_ref().and_then(|values| values.get(self.index)) {
                    target.dash.as_mut().ok_or("drawing-store.stroke-dash-target")?.push(*value);
                    self.index += 1;
                    cx.consume_fuel(1);
                } else {
                    self.phase = 3;
                }
            }
            _ => self.terminal = true,
        }
        Ok(self.terminal)
    }

    fn take(&mut self) -> Option<StrokeStyle> {
        self.terminal.then(|| self.value.take()).flatten()
    }

    fn close_step(&mut self, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if let Some(retirement) = self.retirement.as_mut() {
            return match store::ErasedSnapshotRetirement::close_step(retirement.as_mut(), 1, maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if store::ErasedSnapshotRetirement::terminal_is_empty(retirement.as_ref()) => {
                    drop(self.retirement.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
                store::SnapshotRetirementStep::Complete => Err("Drawing stroke clone reported false terminal".into()),
                step => Ok(step),
            };
        }
        if let Some(value) = self.value.take() {
            *self.retirement = Some(Box::new(DrawingOwnedRetirement::new(DrawingRetirementOwner::Stroke(value))));
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.terminal = true;
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal && self.value.is_none() && self.retirement.is_none()
    }
}

impl Drop for DrawingStrokeCloneAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Drawing stroke clone reached Drop before exact handoff or retirement");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct DrawingSemanticDigestTotals {
    semantic_items: usize,
    semantic_bytes: usize,
    source_owner_items: usize,
    source_owner_bytes: usize,
    derived_owner_items: usize,
    derived_owner_bytes: usize,
}

struct DrawingSemanticDigestCredit {
    items: usize,
    bytes: usize,
    source_owner_items: usize,
    source_owner_bytes: usize,
    derived_owner_items: usize,
    derived_owner_bytes: usize,
    owner_census: DrawingFixedOwnerCensus,
    semantic: Option<semio_framework_hash::Sha256>,
}

impl Default for DrawingSemanticDigestCredit {
    fn default() -> Self {
        Self {
            items: 0,
            bytes: 0,
            source_owner_items: 1,
            source_owner_bytes: std::mem::size_of::<DrawingMutation>(),
            derived_owner_items: 0,
            derived_owner_bytes: 0,
            owner_census: DrawingFixedOwnerCensus::new(),
            semantic: Some(semio_framework_hash::Sha256::new()),
        }
    }
}

impl DrawingSemanticDigestCredit {
    fn add_source_owner(&mut self, items: usize, bytes: usize) -> Result<(), &'static str> {
        self.owner_census.admit(items, bytes, 0, 0)?;
        self.source_owner_items = self.source_owner_items.checked_add(items).ok_or("drawing-store.mutation-source-owner-item-overflow")?;
        self.source_owner_bytes = self.source_owner_bytes.checked_add(bytes).ok_or("drawing-store.mutation-source-owner-byte-overflow")?;
        if self.source_owner_items > DRAWING_MAXIMUM_NESTED_ITEMS {
            return Err("drawing-store.mutation-source-owner-item-capacity");
        }
        if self.source_owner_bytes > DRAWING_MAXIMUM_NESTED_BYTES {
            return Err("drawing-store.mutation-source-owner-byte-capacity");
        }
        Ok(())
    }

    fn add_derived_owner(&mut self, items: usize, bytes: usize) -> Result<(), &'static str> {
        self.owner_census.admit(0, 0, items, bytes)?;
        self.derived_owner_items = self.derived_owner_items.checked_add(items).ok_or("drawing-store.mutation-derived-owner-item-overflow")?;
        self.derived_owner_bytes = self.derived_owner_bytes.checked_add(bytes).ok_or("drawing-store.mutation-derived-owner-byte-overflow")?;
        if self.derived_owner_items > DRAWING_MAXIMUM_NESTED_ITEMS {
            return Err("drawing-store.mutation-derived-owner-item-capacity");
        }
        if self.derived_owner_bytes > DRAWING_MAXIMUM_NESTED_BYTES {
            return Err("drawing-store.mutation-derived-owner-byte-capacity");
        }
        Ok(())
    }

    fn source_string(&mut self, value: &String) -> Result<(), &'static str> {
        self.add_source_owner(1, std::mem::size_of::<String>() + value.capacity())
    }

    fn derived_string(&mut self, value: &String) -> Result<(), &'static str> {
        if value.len() > DRAWING_OWNED_FIELD_BYTES {
            return Err("drawing-store.mutation-derived-string-page-capacity");
        }
        self.add_derived_owner(1, 0)
    }

    fn source_vec<T>(&mut self, value: &Vec<T>) -> Result<(), &'static str> {
        let items = 1usize.checked_add(value.capacity()).ok_or("drawing-store.mutation-source-owner-item-overflow")?;
        let bytes = value.capacity().checked_mul(std::mem::size_of::<T>()).and_then(|bytes| bytes.checked_add(std::mem::size_of::<Vec<T>>())).ok_or("drawing-store.mutation-source-owner-byte-overflow")?;
        self.add_source_owner(items, bytes)
    }

    fn derived_vec<T>(&mut self, value: &Vec<T>) -> Result<(), &'static str> {
        let bytes = value.len().checked_mul(std::mem::size_of::<T>()).ok_or("drawing-store.mutation-derived-owner-byte-overflow")?;
        let pages = bytes.checked_add(DRAWING_MUTATION_RETAINED_PAGE_BYTES - 1).ok_or("drawing-store.mutation-derived-owner-byte-overflow")? / DRAWING_MUTATION_RETAINED_PAGE_BYTES;
        self.add_derived_owner(pages.max(1), 0)
    }

    fn observe_owned_string(&mut self, digest: &mut store::ArtifactStoreInitializationDigest, tag: u16, value: &String, cloned: bool, cx: &mut semio_framework_job::StepContext<'_>) -> Result<(), &'static str> {
        self.source_string(value)?;
        if cloned {
            self.derived_string(value)?;
        }
        self.observe(digest, tag, value.as_bytes(), cx)
    }

    fn observe(&mut self, digest: &mut store::ArtifactStoreInitializationDigest, tag: u16, value: &[u8], cx: &mut semio_framework_job::StepContext<'_>) -> Result<(), &'static str> {
        if value.len() > DRAWING_OWNED_FIELD_BYTES {
            return Err("drawing-store.mutation-field-capacity");
        }
        self.items = self.items.checked_add(1).ok_or("drawing-store.mutation-item-overflow")?;
        self.bytes = self.bytes.checked_add(11).and_then(|bytes| bytes.checked_add(value.len())).ok_or("drawing-store.mutation-byte-overflow")?;
        if self.items > DRAWING_MAXIMUM_NESTED_ITEMS {
            return Err("drawing-store.mutation-item-capacity");
        }
        if self.bytes > DRAWING_MAXIMUM_NESTED_BYTES {
            return Err("drawing-store.mutation-byte-capacity");
        }
        let prefix = [0xd8];
        let tag = tag.to_be_bytes();
        let length = (value.len() as u64).to_be_bytes();
        let semantic = self.semantic.as_mut().ok_or("drawing-store.mutation-digest-sealed")?;
        semantic.update(&prefix);
        semantic.update(&tag);
        semantic.update(&length);
        semantic.update(value);
        digest.observe(&prefix);
        digest.observe(&tag);
        digest.observe(&length);
        digest.observe(value);
        cx.consume_fuel(1);
        Ok(())
    }

    fn scalar_f64(&mut self, digest: &mut store::ArtifactStoreInitializationDigest, tag: u16, value: f64, cx: &mut semio_framework_job::StepContext<'_>) -> Result<(), &'static str> {
        self.observe(digest, tag, &value.to_bits().to_be_bytes(), cx)
    }

    fn scalar_usize(&mut self, digest: &mut store::ArtifactStoreInitializationDigest, tag: u16, value: usize, cx: &mut semio_framework_job::StepContext<'_>) -> Result<(), &'static str> {
        self.observe(digest, tag, &(value as u64).to_be_bytes(), cx)
    }

    fn seal(&mut self, digest: &mut store::ArtifactStoreInitializationDigest, cx: &mut semio_framework_job::StepContext<'_>) -> Result<(), &'static str> {
        self.observe(digest, 99, &[], cx)?;
        let semantic = self.semantic.take().ok_or("drawing-store.mutation-digest-sealed")?.finalize();
        digest.observe(b"drawing.semantic.sha256");
        digest.observe(&semantic);
        Ok(())
    }

    fn totals(&self) -> Option<DrawingSemanticDigestTotals> {
        self.semantic.is_none().then_some(DrawingSemanticDigestTotals {
            semantic_items: self.items,
            semantic_bytes: self.bytes,
            source_owner_items: self.source_owner_items,
            source_owner_bytes: self.source_owner_bytes,
            derived_owner_items: self.derived_owner_items,
            derived_owner_bytes: self.derived_owner_bytes,
        })
    }
}

struct DrawingFillDigestAuthority {
    phase: u8,
    index: usize,
    field: u8,
    terminal: bool,
}

impl DrawingFillDigestAuthority {
    fn new() -> Self {
        Self { phase: 0, index: 0, field: 0, terminal: false }
    }

    fn step(&mut self, value: Option<&FillStyle>, digest: &mut store::ArtifactStoreInitializationDigest, credit: &mut DrawingSemanticDigestCredit, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        if self.phase == 0 {
            credit.observe(digest, 200, &[u8::from(value.is_some())], cx)?;
            self.phase = 1;
            if value.is_none() {
                self.terminal = true;
            }
            return Ok(self.terminal);
        }
        let value = value.ok_or("drawing-store.digest-fill-missing")?;
        if self.phase == 1 {
            let variant = match value {
                FillStyle::Solid { .. } => 1,
                FillStyle::LinearGradient { .. } => 2,
                FillStyle::RadialGradient { .. } => 3,
            };
            credit.observe(digest, 201, &[variant], cx)?;
            self.phase = 2;
            return Ok(false);
        }
        match value {
            FillStyle::Solid { color } => {
                let Some(value) = color.get((self.phase - 2) as usize) else {
                    self.terminal = true;
                    return Ok(true);
                };
                credit.scalar_f64(digest, 202 + u16::from(self.phase - 2), *value, cx)?;
                self.phase += 1;
                self.terminal = self.phase == 6;
            }
            FillStyle::LinearGradient { x1, y1, x2, y2, stops } => {
                if self.phase <= 5 {
                    let fields = [*x1, *y1, *x2, *y2];
                    credit.scalar_f64(digest, 210 + u16::from(self.phase - 2), fields[(self.phase - 2) as usize], cx)?;
                    self.phase += 1;
                } else if self.phase == 6 {
                    credit.source_vec(stops)?;
                    credit.derived_vec(stops)?;
                    credit.scalar_usize(digest, 214, stops.len(), cx)?;
                    self.phase = 7;
                    self.terminal = stops.is_empty();
                } else {
                    let stop = stops.get(self.index).ok_or("drawing-store.digest-linear-stop")?;
                    if self.field == 0 {
                        credit.scalar_f64(digest, 215, stop.offset, cx)?;
                    } else {
                        credit.scalar_f64(digest, 215 + u16::from(self.field), stop.color[(self.field - 1) as usize], cx)?;
                    }
                    self.field += 1;
                    if self.field == 5 {
                        self.field = 0;
                        self.index += 1;
                        self.terminal = self.index == stops.len();
                    }
                }
            }
            FillStyle::RadialGradient { cx: center_x, cy: center_y, r, stops } => {
                if self.phase <= 4 {
                    let fields = [*center_x, *center_y, *r];
                    credit.scalar_f64(digest, 220 + u16::from(self.phase - 2), fields[(self.phase - 2) as usize], cx)?;
                    self.phase += 1;
                } else if self.phase == 5 {
                    credit.source_vec(stops)?;
                    credit.derived_vec(stops)?;
                    credit.scalar_usize(digest, 223, stops.len(), cx)?;
                    self.phase = 6;
                    self.terminal = stops.is_empty();
                } else {
                    let stop = stops.get(self.index).ok_or("drawing-store.digest-radial-stop")?;
                    if self.field == 0 {
                        credit.scalar_f64(digest, 224, stop.offset, cx)?;
                    } else {
                        credit.scalar_f64(digest, 224 + u16::from(self.field), stop.color[(self.field - 1) as usize], cx)?;
                    }
                    self.field += 1;
                    if self.field == 5 {
                        self.field = 0;
                        self.index += 1;
                        self.terminal = self.index == stops.len();
                    }
                }
            }
        }
        Ok(self.terminal)
    }
}

struct DrawingStrokeDigestAuthority {
    phase: u8,
    index: usize,
    terminal: bool,
}

impl DrawingStrokeDigestAuthority {
    fn new() -> Self {
        Self { phase: 0, index: 0, terminal: false }
    }

    fn step(&mut self, value: Option<&StrokeStyle>, digest: &mut store::ArtifactStoreInitializationDigest, credit: &mut DrawingSemanticDigestCredit, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        if self.phase == 0 {
            credit.observe(digest, 240, &[u8::from(value.is_some())], cx)?;
            self.phase = 1;
            if value.is_none() {
                self.terminal = true;
            }
            return Ok(self.terminal);
        }
        let value = value.ok_or("drawing-store.digest-stroke-missing")?;
        match self.phase {
            1..=4 => credit.scalar_f64(digest, 240 + u16::from(self.phase), value.color[(self.phase - 1) as usize], cx)?,
            5 => credit.scalar_f64(digest, 245, value.width, cx)?,
            6 => credit.observe_owned_string(digest, 246, &value.cap, true, cx)?,
            7 => credit.observe_owned_string(digest, 247, &value.join, true, cx)?,
            8 => credit.observe(digest, 248, &[u8::from(value.dash.is_some())], cx)?,
            9 => {
                let dash = value.dash.as_ref().ok_or("drawing-store.digest-dash-missing")?;
                credit.source_vec(dash)?;
                credit.derived_vec(dash)?;
                credit.scalar_usize(digest, 249, dash.len(), cx)?;
                self.terminal = dash.is_empty();
            }
            _ => {
                let dash = value.dash.as_ref().ok_or("drawing-store.digest-dash-missing")?;
                let item = *dash.get(self.index).ok_or("drawing-store.digest-dash-index")?;
                credit.scalar_f64(digest, 250, item, cx)?;
                self.index += 1;
                self.terminal = self.index == dash.len();
            }
        }
        if !self.terminal {
            self.phase = match self.phase {
                8 if value.dash.is_none() => {
                    self.terminal = true;
                    8
                }
                phase => phase + 1,
            };
        }
        Ok(self.terminal)
    }
}

struct DrawingPathSegmentDigestAuthority {
    phase: u8,
    terminal: bool,
}

impl DrawingPathSegmentDigestAuthority {
    fn new() -> Self {
        Self { phase: 0, terminal: false }
    }

    fn step(&mut self, value: &PathSegment, digest: &mut store::ArtifactStoreInitializationDigest, credit: &mut DrawingSemanticDigestCredit, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        if self.phase == 0 {
            let variant = match value {
                PathSegment::Move { .. } => 1,
                PathSegment::Line { .. } => 2,
                PathSegment::Quad { .. } => 3,
                PathSegment::Cubic { .. } => 4,
                PathSegment::Arc { .. } => 5,
                PathSegment::Close => 6,
            };
            credit.observe(digest, 300, &[variant], cx)?;
            self.phase = 1;
            self.terminal = matches!(value, PathSegment::Close);
            return Ok(self.terminal);
        }
        let index = (self.phase - 1) as usize;
        match value {
            PathSegment::Move { to } | PathSegment::Line { to } => {
                let Some(field) = to.get(index) else {
                    self.terminal = true;
                    return Ok(true);
                };
                credit.scalar_f64(digest, 301 + index as u16, *field, cx)?;
                self.phase += 1;
                self.terminal = self.phase == 3;
            }
            PathSegment::Quad { ctrl, to } => {
                let fields = [ctrl[0], ctrl[1], to[0], to[1]];
                credit.scalar_f64(digest, 303 + index as u16, fields[index], cx)?;
                self.phase += 1;
                self.terminal = self.phase == 5;
            }
            PathSegment::Cubic { ctrl1, ctrl2, to } => {
                let fields = [ctrl1[0], ctrl1[1], ctrl2[0], ctrl2[1], to[0], to[1]];
                credit.scalar_f64(digest, 307 + index as u16, fields[index], cx)?;
                self.phase += 1;
                self.terminal = self.phase == 7;
            }
            PathSegment::Arc { rx, ry, rotation, large_arc, sweep, to } => {
                if index < 3 {
                    credit.scalar_f64(digest, 313 + index as u16, [*rx, *ry, *rotation][index], cx)?;
                } else if index < 5 {
                    credit.observe(digest, 313 + index as u16, &[u8::from([*large_arc, *sweep][index - 3])], cx)?;
                } else {
                    credit.scalar_f64(digest, 313 + index as u16, to[index - 5], cx)?;
                }
                self.phase += 1;
                self.terminal = self.phase == 8;
            }
            PathSegment::Close => self.terminal = true,
        }
        Ok(self.terminal)
    }
}

struct DrawingLayerVariantDigestAuthority {
    phase: u8,
    index: usize,
    field: u8,
    segment: Option<DrawingPathSegmentDigestAuthority>,
    terminal: bool,
}

impl DrawingLayerVariantDigestAuthority {
    fn new() -> Self {
        Self { phase: 1, index: 0, field: 0, segment: None, terminal: false }
    }

    fn option(&mut self, digest: &mut store::ArtifactStoreInitializationDigest, credit: &mut DrawingSemanticDigestCredit, tag: u16, present: bool, next: u8, absent: u8, cx: &mut semio_framework_job::StepContext<'_>) -> Result<(), &'static str> {
        credit.observe(digest, tag, &[u8::from(present)], cx)?;
        self.phase = if present { next } else { absent };
        Ok(())
    }

    fn step(&mut self, layer: &DrawingLayerNode, digest: &mut store::ArtifactStoreInitializationDigest, credit: &mut DrawingSemanticDigestCredit, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        match layer {
            DrawingLayerNode::Shape(value) => match self.phase {
                1 => {
                    credit.observe_owned_string(digest, 340, &value.shape_kind, true, cx)?;
                    self.phase = 2;
                }
                2 => self.option(digest, credit, 341, value.rect.is_some(), 3, 7, cx)?,
                3..=6 => {
                    let rect = value.rect.as_ref().ok_or("drawing-store.digest-rect-missing")?;
                    credit.scalar_f64(digest, 342 + u16::from(self.phase - 3), [rect.x, rect.y, rect.width, rect.height][(self.phase - 3) as usize], cx)?;
                    self.phase += 1;
                }
                7 => self.option(digest, credit, 346, value.ellipse.is_some(), 8, 12, cx)?,
                8..=11 => {
                    let ellipse = value.ellipse.as_ref().ok_or("drawing-store.digest-ellipse-missing")?;
                    credit.scalar_f64(digest, 347 + u16::from(self.phase - 8), [ellipse.cx, ellipse.cy, ellipse.rx, ellipse.ry][(self.phase - 8) as usize], cx)?;
                    self.phase += 1;
                }
                12 => self.option(digest, credit, 351, value.circle.is_some(), 13, 16, cx)?,
                13..=15 => {
                    let circle = value.circle.as_ref().ok_or("drawing-store.digest-circle-missing")?;
                    credit.scalar_f64(digest, 352 + u16::from(self.phase - 13), [circle.cx, circle.cy, circle.r][(self.phase - 13) as usize], cx)?;
                    self.phase += 1;
                }
                16 => self.option(digest, credit, 355, value.line.is_some(), 17, 21, cx)?,
                17..=20 => {
                    let line = value.line.as_ref().ok_or("drawing-store.digest-line-missing")?;
                    credit.scalar_f64(digest, 356 + u16::from(self.phase - 17), [line.x1, line.y1, line.x2, line.y2][(self.phase - 17) as usize], cx)?;
                    self.phase += 1;
                }
                21 => self.option(digest, credit, 360, value.polygon.is_some(), 22, 24, cx)?,
                22 => {
                    let points = &value.polygon.as_ref().ok_or("drawing-store.digest-polygon-missing")?.points;
                    credit.source_vec(points)?;
                    credit.derived_vec(points)?;
                    credit.scalar_usize(digest, 361, points.len(), cx)?;
                    self.phase = 23;
                    self.terminal = points.is_empty();
                }
                23 => {
                    let points = &value.polygon.as_ref().ok_or("drawing-store.digest-polygon-missing")?.points;
                    let point = points.get(self.index).ok_or("drawing-store.digest-point-index")?;
                    credit.scalar_f64(digest, 362 + u16::from(self.field), point[self.field as usize], cx)?;
                    self.field += 1;
                    if self.field == 2 {
                        self.field = 0;
                        self.index += 1;
                        self.terminal = self.index == points.len();
                    }
                }
                _ => self.terminal = true,
            },
            DrawingLayerNode::Path(value) => match self.phase {
                1 => {
                    credit.source_vec(&value.segments)?;
                    credit.derived_vec(&value.segments)?;
                    credit.scalar_usize(digest, 370, value.segments.len(), cx)?;
                    self.phase = 2;
                    self.terminal = value.segments.is_empty();
                }
                _ => {
                    let segment = value.segments.get(self.index).ok_or("drawing-store.digest-segment-index")?;
                    let cursor = self.segment.get_or_insert_with(DrawingPathSegmentDigestAuthority::new);
                    if cursor.step(segment, digest, credit, cx)? {
                        self.segment = None;
                        self.index += 1;
                        self.terminal = self.index == value.segments.len();
                    }
                }
            },
            DrawingLayerNode::Text(value) => {
                match self.phase {
                    1 => credit.scalar_f64(digest, 380, value.x, cx)?,
                    2 => credit.scalar_f64(digest, 381, value.y, cx)?,
                    3 => credit.observe_owned_string(digest, 382, &value.content, true, cx)?,
                    4 => credit.scalar_f64(digest, 383, value.size, cx)?,
                    _ => {
                        self.terminal = true;
                        return Ok(true);
                    }
                }
                self.phase += 1;
                self.terminal = self.phase == 5;
            }
            DrawingLayerNode::Image(value) => {
                match self.phase {
                    1 => credit.observe_owned_string(digest, 390, &value.image_key, true, cx)?,
                    2 => credit.scalar_f64(digest, 391, value.width, cx)?,
                    3 => credit.scalar_f64(digest, 392, value.height, cx)?,
                    _ => {
                        self.terminal = true;
                        return Ok(true);
                    }
                }
                self.phase += 1;
                self.terminal = self.phase == 4;
            }
            DrawingLayerNode::Group(value) => {
                credit.source_vec(&value.children)?;
                credit.derived_vec(&value.children)?;
                credit.scalar_usize(digest, 400, value.children.len(), cx)?;
                self.terminal = true;
            }
            DrawingLayerNode::Boolean(value) => match self.phase {
                1 => {
                    credit.observe_owned_string(digest, 410, &value.operation, true, cx)?;
                    self.phase = 2;
                }
                2 => {
                    credit.source_vec(&value.children)?;
                    credit.derived_vec(&value.children)?;
                    credit.scalar_usize(digest, 411, value.children.len(), cx)?;
                    self.phase = 3;
                    self.terminal = value.children.is_empty();
                }
                _ => {
                    let item = value.children.get(self.index).ok_or("drawing-store.digest-boolean-child")?;
                    credit.observe_owned_string(digest, 412, item, true, cx)?;
                    self.index += 1;
                    self.terminal = self.index == value.children.len();
                }
            },
            DrawingLayerNode::Trace(value) => {
                match self.phase {
                    1 => credit.observe_owned_string(digest, 420, &value.source_key, true, cx)?,
                    2 => credit.scalar_f64(digest, 421, value.params.threshold, cx)?,
                    3 => credit.scalar_f64(digest, 422, value.params.simplify_epsilon, cx)?,
                    _ => {
                        self.terminal = true;
                        return Ok(true);
                    }
                }
                self.phase += 1;
                self.terminal = self.phase == 4;
            }
        }
        Ok(self.terminal)
    }
}

struct DrawingLayerDigestAuthority {
    depth: usize,
    path: [usize; DRAWING_MAXIMUM_LAYER_DEPTH],
    frames: [DrawingTraversalFrame; DRAWING_MAXIMUM_LAYER_DEPTH],
    fill: Option<DrawingFillDigestAuthority>,
    stroke: Option<DrawingStrokeDigestAuthority>,
    variant: Option<DrawingLayerVariantDigestAuthority>,
    terminal: bool,
}

impl DrawingLayerDigestAuthority {
    fn new() -> Self {
        Self { depth: 0, path: [0; DRAWING_MAXIMUM_LAYER_DEPTH], frames: [DrawingTraversalFrame::EMPTY; DRAWING_MAXIMUM_LAYER_DEPTH], fill: None, stroke: None, variant: None, terminal: false }
    }

    fn step(&mut self, root: &DrawingLayerNode, digest: &mut store::ArtifactStoreInitializationDigest, credit: &mut DrawingSemanticDigestCredit, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        let node = DrawingSnapshotBoundsAuthority::layer_at(root, &self.path[..self.depth]).ok_or("drawing-store.digest-layer-path")?;
        let base = crate::artifacts::drawing::schema::layer_base(node);
        let phase = self.frames[self.depth].phase;
        match phase {
            0 => {
                credit.add_source_owner(1, std::mem::size_of::<DrawingLayerNode>())?;
                credit.add_derived_owner(1, std::mem::size_of::<DrawingLayerNode>())?;
                let variant = match node {
                    DrawingLayerNode::Shape(_) => 1,
                    DrawingLayerNode::Path(_) => 2,
                    DrawingLayerNode::Text(_) => 3,
                    DrawingLayerNode::Image(_) => 4,
                    DrawingLayerNode::Group(_) => 5,
                    DrawingLayerNode::Boolean(_) => 6,
                    DrawingLayerNode::Trace(_) => 7,
                };
                credit.observe(digest, 100, &[variant], cx)?;
                self.frames[self.depth].phase = 1;
            }
            1 => {
                credit.observe_owned_string(digest, 101, &base.id, true, cx)?;
                self.frames[self.depth].phase = 2;
            }
            2 => {
                credit.observe_owned_string(digest, 102, &base.name, true, cx)?;
                self.frames[self.depth].phase = 3;
            }
            3 => {
                credit.observe(digest, 103, &[u8::from(base.visible)], cx)?;
                self.frames[self.depth].phase = 4;
            }
            4 => {
                credit.observe(digest, 104, &[u8::from(base.locked)], cx)?;
                self.frames[self.depth].phase = 5;
            }
            5 => {
                credit.scalar_f64(digest, 105, base.opacity, cx)?;
                self.frames[self.depth].phase = 6;
            }
            6 => {
                credit.observe_owned_string(digest, 106, &base.blend_mode, true, cx)?;
                self.frames[self.depth].phase = 7;
            }
            7..=11 => {
                let fields = [base.transform.x, base.transform.y, base.transform.scale_x, base.transform.scale_y, base.transform.rotation];
                credit.scalar_f64(digest, 107 + u16::from(phase - 7), fields[(phase - 7) as usize], cx)?;
                self.frames[self.depth].phase += 1;
            }
            12 => {
                let fill = self.fill.get_or_insert_with(DrawingFillDigestAuthority::new);
                if fill.step(base.attributes.fill.as_ref(), digest, credit, cx)? {
                    self.fill = None;
                    self.frames[self.depth].phase = 13;
                }
            }
            13 => {
                let stroke = self.stroke.get_or_insert_with(DrawingStrokeDigestAuthority::new);
                if stroke.step(base.attributes.stroke.as_ref(), digest, credit, cx)? {
                    self.stroke = None;
                    self.frames[self.depth].phase = 14;
                }
            }
            14 => {
                let variant = self.variant.get_or_insert_with(DrawingLayerVariantDigestAuthority::new);
                if variant.step(node, digest, credit, cx)? {
                    self.variant = None;
                    self.frames[self.depth].phase = 15;
                }
            }
            15 => {
                if let DrawingLayerNode::Group(group) = node {
                    let child = self.frames[self.depth].child;
                    if child < group.children.len() {
                        if self.depth + 1 >= DRAWING_MAXIMUM_LAYER_DEPTH {
                            return Err("drawing-store.digest-layer-depth");
                        }
                        self.frames[self.depth].child += 1;
                        self.path[self.depth] = child;
                        self.depth += 1;
                        self.frames[self.depth] = DrawingTraversalFrame::EMPTY;
                        cx.consume_fuel(1);
                        return Ok(false);
                    }
                }
                self.frames[self.depth].phase = 16;
                return Ok(false);
            }
            _ => {
                credit.observe(digest, 199, &[], cx)?;
                if self.depth == 0 {
                    self.terminal = true;
                } else {
                    self.depth -= 1;
                }
            }
        }
        Ok(self.terminal)
    }

    fn close(&mut self) {
        self.fill = None;
        self.stroke = None;
        self.variant = None;
        self.terminal = true;
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal && self.fill.is_none() && self.stroke.is_none() && self.variant.is_none()
    }
}

struct DrawingMutationDigestAuthority {
    layer: Option<DrawingLayerDigestAuthority>,
    fill: Option<DrawingFillDigestAuthority>,
    stroke: Option<DrawingStrokeDigestAuthority>,
    credit: DrawingSemanticDigestCredit,
    phase: u8,
    terminal: bool,
}

impl DrawingMutationDigestAuthority {
    fn new() -> Self {
        Self { layer: None, fill: None, stroke: None, credit: DrawingSemanticDigestCredit::default(), phase: 0, terminal: false }
    }

    fn variant(mutation: &DrawingMutation) -> u8 {
        match mutation {
            DrawingMutation::SetLayerVisible(_) => 1,
            DrawingMutation::SetLayerLocked(_) => 2,
            DrawingMutation::SetLayerOpacity(_) => 3,
            DrawingMutation::SetLayerBlendMode(_) => 4,
            DrawingMutation::RenameLayer(_) => 5,
            DrawingMutation::UpdateLayerTransform(_) => 6,
            DrawingMutation::ReplaceLayerFill(_) => 7,
            DrawingMutation::ReplaceLayerStroke(_) => 8,
            DrawingMutation::SetLayerBooleanOperation(_) => 9,
            DrawingMutation::UpdateLayerTraceParams(_) => 10,
            DrawingMutation::CreateLayer(_) => 11,
            DrawingMutation::DuplicateLayer(_) => 12,
            DrawingMutation::DeleteLayer(_) => 13,
            DrawingMutation::ReorderLayer(_) => 14,
        }
    }

    fn finish(&mut self, digest: &mut store::ArtifactStoreInitializationDigest, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        self.credit.seal(digest, cx)?;
        self.terminal = true;
        Ok(true)
    }

    fn step(&mut self, mutation: &DrawingMutation, digest: &mut store::ArtifactStoreInitializationDigest, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        if self.phase == 0 {
            self.credit.observe(digest, 1, &[Self::variant(mutation)], cx)?;
            self.phase = 1;
            return Ok(false);
        }
        if self.phase == 1 {
            match mutation {
                DrawingMutation::CreateLayer(value) => {
                    self.credit.add_source_owner(1, std::mem::size_of::<Box<DrawingLayerNode>>())?;
                    self.credit.add_derived_owner(1, std::mem::size_of::<Box<DrawingLayerNode>>())?;
                    self.credit.observe(digest, 2, &[u8::from(value.parent_id.is_some())], cx)?;
                    self.phase = if value.parent_id.is_some() { 2 } else { 3 };
                }
                _ => {
                    let target = DrawingMutationCandidateAuthority::target_owner(mutation).ok_or("drawing-store.mutation-target-owner")?;
                    self.credit.observe_owned_string(digest, 2, target, false, cx)?;
                    self.phase = 2;
                }
            }
            return Ok(false);
        }
        match mutation {
            DrawingMutation::SetLayerVisible(value) => {
                if self.phase == 2 {
                    self.credit.observe(digest, 3, &[u8::from(value.visible)], cx)?;
                    self.phase = 3;
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawingMutation::SetLayerLocked(value) => {
                if self.phase == 2 {
                    self.credit.observe(digest, 3, &[u8::from(value.locked)], cx)?;
                    self.phase = 3;
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawingMutation::SetLayerOpacity(value) => {
                if self.phase == 2 {
                    self.credit.scalar_f64(digest, 3, value.opacity, cx)?;
                    self.phase = 3;
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawingMutation::SetLayerBlendMode(value) => {
                if self.phase == 2 {
                    self.credit.observe_owned_string(digest, 3, &value.blend_mode, true, cx)?;
                    self.phase = 3;
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawingMutation::RenameLayer(value) => {
                if self.phase == 2 {
                    self.credit.observe_owned_string(digest, 3, &value.new_name, true, cx)?;
                    self.phase = 3;
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawingMutation::UpdateLayerTransform(value) => {
                if self.phase <= 6 {
                    let fields = [value.transform.x, value.transform.y, value.transform.scale_x, value.transform.scale_y, value.transform.rotation];
                    self.credit.scalar_f64(digest, 3 + u16::from(self.phase - 2), fields[(self.phase - 2) as usize], cx)?;
                    self.phase += 1;
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawingMutation::ReplaceLayerFill(value) => {
                if self.phase == 2 {
                    let fill = self.fill.get_or_insert_with(DrawingFillDigestAuthority::new);
                    if fill.step(value.fill.as_ref(), digest, &mut self.credit, cx)? {
                        self.fill = None;
                        self.phase = 3;
                    }
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawingMutation::ReplaceLayerStroke(value) => {
                if self.phase == 2 {
                    let stroke = self.stroke.get_or_insert_with(DrawingStrokeDigestAuthority::new);
                    if stroke.step(value.stroke.as_ref(), digest, &mut self.credit, cx)? {
                        self.stroke = None;
                        self.phase = 3;
                    }
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawingMutation::SetLayerBooleanOperation(value) => {
                if self.phase == 2 {
                    self.credit.observe_owned_string(digest, 3, &value.boolean_operation, true, cx)?;
                    self.phase = 3;
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawingMutation::UpdateLayerTraceParams(value) => {
                if self.phase <= 3 {
                    self.credit.scalar_f64(digest, 3 + u16::from(self.phase - 2), [value.params.threshold, value.params.simplify_epsilon][(self.phase - 2) as usize], cx)?;
                    self.phase += 1;
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawingMutation::CreateLayer(value) => match self.phase {
                2 => {
                    self.credit.observe_owned_string(digest, 3, value.parent_id.as_ref().ok_or("drawing-store.digest-parent-missing")?, false, cx)?;
                    self.phase = 3;
                    Ok(false)
                }
                3 => {
                    self.credit.observe(digest, 4, &[u8::from(value.index.is_some())], cx)?;
                    self.phase = if value.index.is_some() { 4 } else { 5 };
                    Ok(false)
                }
                4 => {
                    self.credit.scalar_usize(digest, 5, value.index.ok_or("drawing-store.digest-index-missing")?, cx)?;
                    self.phase = 5;
                    Ok(false)
                }
                5 => {
                    let layer = self.layer.get_or_insert_with(DrawingLayerDigestAuthority::new);
                    if layer.step(&value.layer, digest, &mut self.credit, cx)? {
                        self.layer = None;
                        self.phase = 6;
                    }
                    Ok(false)
                }
                _ => self.finish(digest, cx),
            },
            DrawingMutation::DuplicateLayer(_) | DrawingMutation::DeleteLayer(_) => self.finish(digest, cx),
            DrawingMutation::ReorderLayer(value) => match self.phase {
                2 => {
                    self.credit.observe(digest, 3, &[u8::from(value.parent_id.is_some())], cx)?;
                    self.phase = if value.parent_id.is_some() { 3 } else { 4 };
                    Ok(false)
                }
                3 => {
                    self.credit.observe_owned_string(digest, 4, value.parent_id.as_ref().ok_or("drawing-store.digest-parent-missing")?, false, cx)?;
                    self.phase = 4;
                    Ok(false)
                }
                4 => {
                    self.credit.scalar_usize(digest, 5, value.index, cx)?;
                    self.phase = 5;
                    Ok(false)
                }
                _ => self.finish(digest, cx),
            },
        }
    }

    fn totals(&self) -> Option<DrawingSemanticDigestTotals> {
        self.terminal.then(|| self.credit.totals()).flatten()
    }

    fn close_step(&mut self, _maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if let Some(layer) = self.layer.as_mut() {
            layer.close();
        }
        self.layer = None;
        self.fill = None;
        self.stroke = None;
        self.terminal = true;
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal && self.layer.is_none() && self.fill.is_none() && self.stroke.is_none()
    }
}

impl Drop for DrawingMutationDigestAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Drawing mutation digest reached Drop before exact terminal close");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct DrawingMutationAggregateReservation {
    source_items: usize,
    candidate_items: usize,
    mutation_source_items: usize,
    mutation_derived_items: usize,
    duplicate_candidate_items: usize,
    authority_items: usize,
    container_items: usize,
    page_items: usize,
    source_bytes: usize,
    candidate_bytes: usize,
    mutation_source_bytes: usize,
    mutation_derived_bytes: usize,
    duplicate_candidate_bytes: usize,
    authority_bytes: usize,
    container_bytes: usize,
    page_bytes: usize,
    maximum_container: usize,
    container_slots: usize,
}

impl DrawingMutationAggregateReservation {
    fn checked_total(values: &[usize], fault: &'static str) -> Result<usize, &'static str> {
        values.iter().try_fold(0usize, |total, value| total.checked_add(*value).ok_or(fault))
    }

    fn admit(
        source: DrawingSnapshotOwnerTotals,
        mutation: DrawingSemanticDigestTotals,
        operation: &DrawingMutation,
        reverse_slots: usize,
        output_slots: usize,
        overlay_slots: usize,
        overlay_bytes: usize,
        duplicate_id_bytes: usize,
    ) -> Result<Self, &'static str> {
        let container_slots = reverse_slots.checked_add(output_slots).ok_or("drawing-store.mutation-container-credit-overflow")?;
        let container_items = 2;
        let container_bytes = container_slots.checked_mul(std::mem::size_of::<DrawingLayerNode>()).ok_or("drawing-store.mutation-container-credit-overflow")?;
        let (duplicate_candidate_items, duplicate_candidate_bytes) = if matches!(operation, DrawingMutation::DuplicateLayer(_)) { (1, duplicate_id_bytes) } else { (0, 0) };
        let authority_items = 1;
        let authority_bytes = std::mem::size_of::<DrawingMutationCandidateAuthority>();
        let reservation = Self {
            source_items: source.source_items,
            candidate_items: source.candidate_items,
            mutation_source_items: mutation.source_owner_items,
            mutation_derived_items: mutation.derived_owner_items,
            duplicate_candidate_items,
            authority_items,
            container_items,
            page_items: DRAWING_MUTATION_RETAINED_PAGE_ITEMS.checked_add(overlay_slots).ok_or("drawing-store.mutation-overlay-item-overflow")?,
            source_bytes: source.source_bytes,
            candidate_bytes: source.candidate_bytes,
            mutation_source_bytes: mutation.source_owner_bytes,
            mutation_derived_bytes: mutation.derived_owner_bytes,
            duplicate_candidate_bytes,
            authority_bytes,
            container_bytes,
            page_bytes: DRAWING_MUTATION_RETAINED_PAGE_BYTES.checked_add(overlay_bytes).ok_or("drawing-store.mutation-overlay-byte-overflow")?,
            maximum_container: source.maximum_container,
            container_slots,
        };
        if reservation.total_items()? > DRAWING_MUTATION_AGGREGATE_ITEMS {
            return Err("drawing-store.mutation-aggregate-item-capacity");
        }
        if reservation.total_bytes()? > DRAWING_MUTATION_AGGREGATE_BYTES {
            return Err("drawing-store.mutation-aggregate-byte-capacity");
        }
        Ok(reservation)
    }

    fn total_items(&self) -> Result<usize, &'static str> {
        Self::checked_total(
            &[self.source_items, self.candidate_items, self.mutation_source_items, self.mutation_derived_items, self.duplicate_candidate_items, self.authority_items, self.container_items, self.page_items],
            "drawing-store.mutation-item-overflow",
        )
    }

    fn total_bytes(&self) -> Result<usize, &'static str> {
        Self::checked_total(
            &[self.source_bytes, self.candidate_bytes, self.mutation_source_bytes, self.mutation_derived_bytes, self.duplicate_candidate_bytes, self.authority_bytes, self.container_bytes, self.page_bytes],
            "drawing-store.mutation-byte-overflow",
        )
    }
}

struct DrawingDuplicateRewriteAuthority {
    depth: usize,
    path: [usize; DRAWING_MAXIMUM_LAYER_DEPTH],
    frames: [DrawingTraversalFrame; DRAWING_MAXIMUM_LAYER_DEPTH],
    material: [u8; DRAWING_DUPLICATE_MATERIAL_BYTES],
    material_len: usize,
    id_len: usize,
    name_len: usize,
    hash_cursor: usize,
    hasher: Option<semio_framework_hash::Sha256>,
    pending_id: std::mem::ManuallyDrop<Option<String>>,
    pending_name: std::mem::ManuallyDrop<Option<String>>,
    terminal: bool,
}

impl DrawingDuplicateRewriteAuthority {
    fn new(mut pending_id: String, mut pending_name: String) -> Self {
        pending_id.clear();
        pending_name.clear();
        Self {
            depth: 0,
            path: [0; DRAWING_MAXIMUM_LAYER_DEPTH],
            frames: [DrawingTraversalFrame::EMPTY; DRAWING_MAXIMUM_LAYER_DEPTH],
            material: [0; DRAWING_DUPLICATE_MATERIAL_BYTES],
            material_len: 0,
            id_len: 0,
            name_len: 0,
            hash_cursor: 0,
            hasher: None,
            pending_id: std::mem::ManuallyDrop::new(Some(pending_id)),
            pending_name: std::mem::ManuallyDrop::new(Some(pending_name)),
            terminal: false,
        }
    }

    fn step(&mut self, root: &mut DrawingLayerNode, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        let node = DrawingLayerCloneAuthority::target_at_mut(root, &self.path[..self.depth]).ok_or("drawing-store.duplicate-path")?;
        let prefix = match node {
            DrawingLayerNode::Shape(_) => "shape",
            DrawingLayerNode::Path(_) => "path",
            DrawingLayerNode::Text(_) => "text",
            DrawingLayerNode::Image(_) => "image",
            DrawingLayerNode::Group(_) => "group",
            DrawingLayerNode::Boolean(_) => "boolean",
            DrawingLayerNode::Trace(_) => "trace",
        };
        let phase = self.frames[self.depth].phase;
        if phase <= 13 {
            let base = crate::artifacts::drawing::schema::layer_base_mut(node);
            let suffix = if self.depth == 0 { " copy" } else { "" };
            match phase {
                0 => {
                    if base.id.len() > DRAWING_OWNED_FIELD_BYTES {
                        return Err("drawing-store.duplicate-id-capacity");
                    }
                    self.material[..base.id.len()].copy_from_slice(base.id.as_bytes());
                    self.id_len = base.id.len();
                    self.material_len = base.id.len();
                    self.frames[self.depth].phase = 1;
                    cx.consume_fuel(base.id.len().max(1) as u64);
                }
                1 => {
                    let total = self.material_len.checked_add(base.name.len()).ok_or("drawing-store.duplicate-byte-overflow")?;
                    if base.name.len() > DRAWING_OWNED_FIELD_BYTES || total > self.material.len() {
                        return Err("drawing-store.duplicate-name-capacity");
                    }
                    self.material[self.material_len..total].copy_from_slice(base.name.as_bytes());
                    self.name_len = base.name.len();
                    self.material_len = total;
                    self.frames[self.depth].phase = 2;
                    cx.consume_fuel(base.name.len().max(1) as u64);
                }
                2 => {
                    self.hasher = Some(semio_framework_hash::Sha256::new());
                    self.hash_cursor = 0;
                    self.frames[self.depth].phase = 3;
                    cx.consume_fuel(1);
                }
                3 => {
                    self.hasher.as_mut().ok_or("drawing-store.duplicate-hasher-missing")?.update(b"semio.drawing.duplicate-id.v1");
                    self.frames[self.depth].phase = 4;
                    cx.consume_fuel(1);
                }
                4 => {
                    self.hasher.as_mut().ok_or("drawing-store.duplicate-hasher-missing")?.update(&(self.id_len as u64).to_be_bytes());
                    self.hash_cursor = 0;
                    self.frames[self.depth].phase = 5;
                    cx.consume_fuel(1);
                }
                5 => {
                    let end = self.id_len.min(self.hash_cursor + DRAWING_OWNED_FIELD_BYTES);
                    self.hasher.as_mut().ok_or("drawing-store.duplicate-hasher-missing")?.update(&self.material[self.hash_cursor..end]);
                    let consumed = end.saturating_sub(self.hash_cursor);
                    self.hash_cursor = end;
                    if self.hash_cursor == self.id_len {
                        self.frames[self.depth].phase = 6;
                    }
                    cx.consume_fuel(consumed.max(1) as u64);
                }
                6 => {
                    self.hasher.as_mut().ok_or("drawing-store.duplicate-hasher-missing")?.update(&(self.name_len as u64).to_be_bytes());
                    self.hash_cursor = self.id_len;
                    self.frames[self.depth].phase = 7;
                    cx.consume_fuel(1);
                }
                7 => {
                    let end = self.material_len.min(self.hash_cursor + DRAWING_OWNED_FIELD_BYTES);
                    self.hasher.as_mut().ok_or("drawing-store.duplicate-hasher-missing")?.update(&self.material[self.hash_cursor..end]);
                    let consumed = end.saturating_sub(self.hash_cursor);
                    self.hash_cursor = end;
                    if self.hash_cursor == self.material_len {
                        self.frames[self.depth].phase = 8;
                    }
                    cx.consume_fuel(consumed.max(1) as u64);
                }
                8 => {
                    let hash = self.hasher.take().ok_or("drawing-store.duplicate-hasher-missing")?.finalize();
                    let capacity = prefix.len().checked_add(65).ok_or("drawing-store.duplicate-id-overflow")?;
                    let mut id = self.pending_id.take().ok_or("drawing-store.duplicate-id-owner-missing")?;
                    if id.capacity() < capacity {
                        *self.pending_id = Some(id);
                        return Err("drawing-store.duplicate-id-owner-capacity");
                    }
                    id.push_str(prefix);
                    id.push('-');
                    const HEX: &[u8; 16] = b"0123456789abcdef";
                    for byte in hash {
                        id.push(HEX[(byte >> 4) as usize] as char);
                        id.push(HEX[(byte & 0x0f) as usize] as char);
                    }
                    let name_capacity = base.name.len().checked_add(suffix.len()).ok_or("drawing-store.duplicate-name-overflow")?;
                    let name_owner = self.pending_name.as_ref().ok_or("drawing-store.duplicate-name-owner-missing")?;
                    if base.id.capacity() < id.len() || base.name.capacity() < name_capacity || name_owner.capacity() < name_capacity {
                        *self.pending_id = Some(id);
                        return Err("drawing-store.duplicate-destination-capacity");
                    }
                    *self.pending_id = Some(id);
                    self.frames[self.depth].phase = 9;
                    cx.consume_fuel(1);
                }
                9 => {
                    let pending = self.pending_id.as_mut().ok_or("drawing-store.duplicate-id-missing")?;
                    base.id.clear();
                    base.id.push_str(pending);
                    pending.clear();
                    self.frames[self.depth].phase = 10;
                    cx.consume_fuel(1);
                }
                10 => {
                    let pending = self.pending_name.as_mut().ok_or("drawing-store.duplicate-name-owner-missing")?;
                    pending.clear();
                    pending.push_str(&base.name);
                    self.frames[self.depth].phase = 11;
                    cx.consume_fuel(base.name.len().max(1) as u64);
                }
                11 => {
                    self.pending_name.as_mut().ok_or("drawing-store.duplicate-name-owner-missing")?.push_str(suffix);
                    self.frames[self.depth].phase = 12;
                    cx.consume_fuel(suffix.len().max(1) as u64);
                }
                12 => {
                    let pending = self.pending_name.as_mut().ok_or("drawing-store.duplicate-name-owner-missing")?;
                    base.name.clear();
                    base.name.push_str(pending);
                    pending.clear();
                    self.frames[self.depth].phase = 13;
                    cx.consume_fuel(1);
                }
                13 => {
                    self.material_len = 0;
                    self.id_len = 0;
                    self.name_len = 0;
                    self.hash_cursor = 0;
                    cx.consume_fuel(1);
                }
                _ => unreachable!(),
            }
            return Ok(false);
        }
        if let DrawingLayerNode::Group(group) = node {
            let child = self.frames[self.depth].child;
            if child < group.children.len() {
                if self.depth + 1 >= DRAWING_MAXIMUM_LAYER_DEPTH {
                    return Err("drawing-store.duplicate-depth-capacity");
                }
                self.frames[self.depth].child += 1;
                self.path[self.depth] = child;
                self.depth += 1;
                self.frames[self.depth] = DrawingTraversalFrame::EMPTY;
                cx.consume_fuel(1);
                return Ok(false);
            }
        }
        if self.depth == 0 {
            self.terminal = true;
            Ok(true)
        } else {
            self.depth -= 1;
            cx.consume_fuel(1);
            Ok(false)
        }
    }

    fn close_step(&mut self, _maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        self.hasher = None;
        self.material_len = 0;
        self.terminal = true;
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn take_owners(&mut self) -> Option<(String, String)> {
        if !self.terminal || self.hasher.is_some() {
            return None;
        }
        let mut id = self.pending_id.take()?;
        let mut name = self.pending_name.take()?;
        id.clear();
        name.clear();
        Some((id, name))
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal && self.pending_id.is_none() && self.pending_name.is_none() && self.hasher.is_none()
    }
}

impl Drop for DrawingDuplicateRewriteAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Drawing duplicate rewrite reached Drop before staged id/name retirement");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DrawingMutationCandidatePhase {
    PreflightSource,
    PreflightMutation,
    BindOverlay,
    LocatePrimary,
    LocateSecondary,
    PrepareOwnedValue,
    Apply,
    RebuildSource,
    LocateDestination,
    RebuildDestination,
    Complete,
    Retire,
    Fault,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DrawingContainerRebuildRole {
    Source,
    Destination,
    CloseSourceUndo,
}

#[derive(Clone, Copy)]
struct DrawingContainerSourceUndo {
    parent: Option<DrawingLayerAddress>,
    index: usize,
}

struct DrawingMutationOverlayPatch {
    source_owner: usize,
    committed: bool,
}

impl DrawingMutationOverlayPatch {
    fn bind(source: &DrawingSnapshot) -> Self {
        Self { source_owner: std::ptr::from_ref(source) as usize, committed: false }
    }

    fn validate(&self, source: &DrawingSnapshot) -> Result<(), &'static str> {
        if self.source_owner != std::ptr::from_ref(source) as usize {
            return Err("drawing-store.mutation-overlay-owner-changed");
        }
        Ok(())
    }

    fn commit(&mut self, source: &DrawingSnapshot) -> Result<(), &'static str> {
        self.validate(source)?;
        self.committed = true;
        Ok(())
    }
}

struct DrawingMutationCandidateAuthority {
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    arena_pool: Option<std::sync::Arc<DrawingMutationArenaPool>>,
    arena_slot: usize,
    arena_generation: u64,
    arena_return_phase: u8,
    preflight_source: Option<DrawingSnapshotBoundsAuthority>,
    preflight_mutation: std::mem::ManuallyDrop<Option<DrawingMutationDigestAuthority>>,
    preflight_digest: Option<store::ArtifactStoreInitializationDigest>,
    reservation: Option<DrawingMutationAggregateReservation>,
    overlay: Option<DrawingMutationOverlayPatch>,
    locator: Option<DrawingLayerLocator>,
    primary: Option<DrawingLayerAddress>,
    secondary: Option<DrawingLayerAddress>,
    layer_clone: std::mem::ManuallyDrop<Option<Box<DrawingLayerCloneAuthority>>>,
    fill_clone: std::mem::ManuallyDrop<Option<DrawingFillCloneAuthority>>,
    stroke_clone: std::mem::ManuallyDrop<Option<DrawingStrokeCloneAuthority>>,
    duplicate_rewrite: Option<DrawingDuplicateRewriteAuthority>,
    duplicate_id_owner: std::mem::ManuallyDrop<Option<String>>,
    rebuild: std::mem::ManuallyDrop<Option<DrawingContainerRebuildAuthority>>,
    rebuild_target: Option<Option<DrawingLayerAddress>>,
    rebuild_role: Option<DrawingContainerRebuildRole>,
    rebuild_close_phase: u8,
    source_undo: Option<DrawingContainerSourceUndo>,
    container_reverse: std::mem::ManuallyDrop<Option<Vec<DrawingLayerNode>>>,
    container_output: std::mem::ManuallyDrop<Option<Vec<DrawingLayerNode>>>,
    overlay_pages: std::mem::ManuallyDrop<Option<Vec<String>>>,
    pending_layer: std::mem::ManuallyDrop<Option<DrawingLayerNode>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    phase: DrawingMutationCandidatePhase,
    terminal: bool,
    fault: Option<&'static str>,
}

impl DrawingMutationCandidateAuthority {
    fn try_new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Result<Self, &'static str> {
        let (arena_pool, arena_slot, arena_generation, owner) = borrow_drawing_mutation_arena().map_err(DrawingMutationArenaBorrowError::as_str)?;
        Ok(Self::from_arena(operation, generation, arena_pool, arena_slot, arena_generation, owner))
    }

    fn try_new_from_pool(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, pool: std::sync::Arc<DrawingMutationArenaPool>) -> Result<Self, &'static str> {
        let (arena_pool, arena_slot, arena_generation, owner) = borrow_drawing_mutation_arena_from(pool)?;
        Ok(Self::from_arena(operation, generation, arena_pool, arena_slot, arena_generation, owner))
    }

    fn from_arena(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, arena_pool: std::sync::Arc<DrawingMutationArenaPool>, arena_slot: usize, arena_generation: u64, owner: DrawingMutationArenaOwner) -> Self {
        Self {
            operation,
            generation,
            arena_pool: Some(arena_pool),
            arena_slot,
            arena_generation,
            arena_return_phase: 0,
            preflight_source: Some(DrawingSnapshotBoundsAuthority::new()),
            preflight_mutation: std::mem::ManuallyDrop::new(Some(DrawingMutationDigestAuthority::new())),
            preflight_digest: Some(store::ArtifactStoreInitializationDigest::new(b"drawing.mutation-preflight")),
            reservation: None,
            overlay: None,
            locator: None,
            primary: None,
            secondary: None,
            layer_clone: std::mem::ManuallyDrop::new(None),
            fill_clone: std::mem::ManuallyDrop::new(None),
            stroke_clone: std::mem::ManuallyDrop::new(None),
            duplicate_rewrite: None,
            duplicate_id_owner: std::mem::ManuallyDrop::new(Some(owner.duplicate_id)),
            rebuild: std::mem::ManuallyDrop::new(None),
            rebuild_target: None,
            rebuild_role: None,
            rebuild_close_phase: 0,
            source_undo: None,
            container_reverse: std::mem::ManuallyDrop::new(Some(owner.reverse)),
            container_output: std::mem::ManuallyDrop::new(Some(owner.output)),
            overlay_pages: std::mem::ManuallyDrop::new(Some(owner.pages)),
            pending_layer: std::mem::ManuallyDrop::new(None),
            retirement: std::mem::ManuallyDrop::new(None),
            phase: DrawingMutationCandidatePhase::PreflightSource,
            terminal: false,
            fault: None,
        }
    }

    fn target(mutation: &DrawingMutation) -> &str {
        match mutation {
            DrawingMutation::SetLayerVisible(value) => &value.layer_id,
            DrawingMutation::SetLayerLocked(value) => &value.layer_id,
            DrawingMutation::SetLayerOpacity(value) => &value.layer_id,
            DrawingMutation::SetLayerBlendMode(value) => &value.layer_id,
            DrawingMutation::RenameLayer(value) => &value.layer_id,
            DrawingMutation::UpdateLayerTransform(value) => &value.layer_id,
            DrawingMutation::ReplaceLayerFill(value) => &value.layer_id,
            DrawingMutation::ReplaceLayerStroke(value) => &value.layer_id,
            DrawingMutation::SetLayerBooleanOperation(value) => &value.layer_id,
            DrawingMutation::UpdateLayerTraceParams(value) => &value.layer_id,
            DrawingMutation::CreateLayer(value) => crate::artifacts::drawing::schema::layer_id(&value.layer),
            DrawingMutation::DuplicateLayer(value) => &value.layer_id,
            DrawingMutation::DeleteLayer(value) => &value.layer_id,
            DrawingMutation::ReorderLayer(value) => &value.layer_id,
        }
    }

    fn target_owner(mutation: &DrawingMutation) -> Option<&String> {
        match mutation {
            DrawingMutation::SetLayerVisible(value) => Some(&value.layer_id),
            DrawingMutation::SetLayerLocked(value) => Some(&value.layer_id),
            DrawingMutation::SetLayerOpacity(value) => Some(&value.layer_id),
            DrawingMutation::SetLayerBlendMode(value) => Some(&value.layer_id),
            DrawingMutation::RenameLayer(value) => Some(&value.layer_id),
            DrawingMutation::UpdateLayerTransform(value) => Some(&value.layer_id),
            DrawingMutation::ReplaceLayerFill(value) => Some(&value.layer_id),
            DrawingMutation::ReplaceLayerStroke(value) => Some(&value.layer_id),
            DrawingMutation::SetLayerBooleanOperation(value) => Some(&value.layer_id),
            DrawingMutation::UpdateLayerTraceParams(value) => Some(&value.layer_id),
            DrawingMutation::CreateLayer(_) => None,
            DrawingMutation::DuplicateLayer(value) => Some(&value.layer_id),
            DrawingMutation::DeleteLayer(value) => Some(&value.layer_id),
            DrawingMutation::ReorderLayer(value) => Some(&value.layer_id),
        }
    }

    fn parent(mutation: &DrawingMutation) -> Option<&str> {
        match mutation {
            DrawingMutation::CreateLayer(value) => value.parent_id.as_deref(),
            DrawingMutation::ReorderLayer(value) => value.parent_id.as_deref(),
            _ => None,
        }
    }

    fn fail(&mut self, fault: &'static str) -> Result<bool, &'static str> {
        self.fault = Some(fault);
        self.phase = DrawingMutationCandidatePhase::Fault;
        Err(fault)
    }

    fn return_arena_owner(&mut self) -> Result<Option<bool>, &'static str> {
        let Some(pool) = self.arena_pool.as_ref() else {
            return Ok(Some(true));
        };
        let Ok(mut state) = pool.state.try_lock() else {
            return Ok(None);
        };
        let slot = state.slots.get_mut(self.arena_slot).ok_or("drawing-store.mutation-arena-slot")?;
        if !slot.leased || slot.generation != self.arena_generation {
            return Err("drawing-store.mutation-arena-stale-generation");
        }
        match self.arena_return_phase {
            0 => {
                let reverse = self.container_reverse.as_ref().ok_or("drawing-store.mutation-reverse-arena-missing")?;
                if !reverse.is_empty() || reverse.capacity() < DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY || slot.reverse.is_some() {
                    return Err("drawing-store.mutation-reverse-arena-not-terminal");
                }
                slot.reverse = Some(self.container_reverse.take().expect("validated Drawing reverse owner remains retained"));
            }
            1 => {
                let output = self.container_output.as_ref().ok_or("drawing-store.mutation-output-arena-missing")?;
                if !output.is_empty() || output.capacity() < DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY || slot.output.is_some() {
                    return Err("drawing-store.mutation-output-arena-not-terminal");
                }
                slot.output = Some(self.container_output.take().expect("validated Drawing output owner remains retained"));
            }
            2 => {
                let pages = self.overlay_pages.as_ref().ok_or("drawing-store.mutation-overlay-arena-missing")?;
                if pages.len() != DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY
                    || pages.capacity() < DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY
                    || pages.iter().any(|page| !page.is_empty() || page.capacity() < DRAWING_MUTATION_RETAINED_PAGE_BYTES)
                    || slot.pages.is_some()
                {
                    return Err("drawing-store.mutation-overlay-arena-not-terminal");
                }
                slot.pages = Some(self.overlay_pages.take().expect("validated Drawing page owner remains retained"));
            }
            3 => {
                let duplicate_id = self.duplicate_id_owner.as_ref().ok_or("drawing-store.mutation-duplicate-id-owner-missing")?;
                if !duplicate_id.is_empty() || duplicate_id.capacity() < DRAWING_DUPLICATE_ID_BYTES || slot.duplicate_id.is_some() {
                    return Err("drawing-store.mutation-duplicate-id-owner-not-terminal");
                }
                slot.duplicate_id = Some(self.duplicate_id_owner.take().expect("validated Drawing duplicate owner remains retained"));
                slot.leased = false;
            }
            _ => return Err("drawing-store.mutation-arena-return-phase"),
        }
        self.arena_return_phase += 1;
        let complete = self.arena_return_phase == 4;
        if complete && !slot.is_available() {
            return Err("drawing-store.mutation-arena-return-false-terminal");
        }
        drop(state);
        if complete {
            self.arena_pool = None;
        }
        Ok(Some(complete))
    }

    fn write_overlay_string(&mut self, target: &mut String, source: &str) -> Result<(), &'static str> {
        if source.len() > DRAWING_MUTATION_RETAINED_PAGE_BYTES {
            return Err("drawing-store.mutation-overlay-string-capacity");
        }
        let pages = self.overlay_pages.as_mut().ok_or("drawing-store.mutation-overlay-arena-missing")?;
        let mut page = pages.pop().ok_or("drawing-store.mutation-overlay-slot-capacity")?;
        if page.capacity() < source.len() || target.capacity() < source.len() {
            pages.push(page);
            return Err("drawing-store.mutation-overlay-destination-capacity");
        }
        page.push_str(source);
        target.clear();
        target.push_str(&page);
        page.clear();
        pages.push(page);
        Ok(())
    }

    fn start_rebuild(&mut self, source: &mut DrawingSnapshot, parent: Option<DrawingLayerAddress>, remove_index: Option<usize>, insert_index: Option<usize>, role: DrawingContainerRebuildRole) -> Result<(), &'static str> {
        let reservation = self.reservation.ok_or("drawing-store.mutation-reservation-missing")?;
        if self.container_reverse.is_none() {
            return Err("drawing-store.mutation-reverse-arena-missing");
        }
        if self.container_output.is_none() {
            return Err("drawing-store.mutation-output-arena-missing");
        }
        let container = DrawingLayerLocator::container_mut(source, parent).ok_or("drawing-store.mutation-container-missing")?;
        let source = std::mem::take(container);
        let pending = self.pending_layer.take();
        let reverse = self.container_reverse.take().expect("validated Drawing reverse arena remains retained");
        let output = self.container_output.take().expect("validated Drawing output arena remains retained");
        match DrawingContainerRebuildAuthority::new(source, remove_index, insert_index, pending, reverse, output, reservation) {
            Ok(rebuild) => {
                *self.rebuild = Some(rebuild);
                self.rebuild_target = Some(parent);
                self.rebuild_role = Some(role);
                self.rebuild_close_phase = 0;
                Ok(())
            }
            Err(rejected) => {
                *container = rejected.source;
                *self.pending_layer = rejected.pending;
                *self.container_reverse = Some(rejected.reverse);
                *self.container_output = Some(rejected.output);
                Err("drawing-store.mutation-container-admission")
            }
        }
    }

    fn finish_rebuild(&mut self, source: &mut DrawingSnapshot, parent: Option<DrawingLayerAddress>) -> Result<Option<DrawingLayerNode>, &'static str> {
        if DrawingLayerLocator::container_mut(source, parent).is_none() {
            return Err("drawing-store.mutation-container-lost");
        }
        let (rebuilt_source, removed, reverse_arena, output_arena) = self.rebuild.as_mut().ok_or("drawing-store.mutation-rebuild-missing")?.take().ok_or("drawing-store.mutation-rebuild-false-terminal")?;
        *DrawingLayerLocator::container_mut(source, parent).expect("validated Drawing mutation container remains available") = rebuilt_source;
        *self.container_reverse = Some(reverse_arena);
        *self.container_output = Some(output_arena);
        let mut rebuild = self.rebuild.take().expect("Drawing completed rebuild remains exact");
        rebuild.terminal = true;
        drop(rebuild);
        self.rebuild_target = None;
        self.rebuild_role = None;
        self.rebuild_close_phase = 0;
        Ok(removed)
    }

    fn step(&mut self, source: &mut DrawingSnapshot, mutation: &DrawingMutation, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if (cx.operation() != self.operation || cx.generation() != self.generation) && !self.overlay.as_ref().is_some_and(|overlay| overlay.committed) {
            return self.fail("drawing-store.mutation-candidate-stale-authority");
        }
        if cx.is_cancelled() && !self.overlay.as_ref().is_some_and(|overlay| overlay.committed) {
            self.fault = Some("drawing-store.mutation-candidate-cancelled");
            self.phase = DrawingMutationCandidatePhase::Retire;
            return Err("drawing-store.mutation-candidate-cancelled");
        }
        if let Some(overlay) = self.overlay.as_ref() {
            overlay.validate(source)?;
        }
        match self.phase {
            DrawingMutationCandidatePhase::PreflightSource => {
                let bounds = self.preflight_source.as_mut().ok_or("drawing-store.mutation-source-preflight-missing")?;
                if bounds.step(source, cx)? {
                    self.phase = DrawingMutationCandidatePhase::PreflightMutation;
                }
                Ok(false)
            }
            DrawingMutationCandidatePhase::PreflightMutation => {
                let digest = self.preflight_mutation.as_mut().ok_or("drawing-store.mutation-preflight-missing")?;
                if !digest.step(mutation, self.preflight_digest.as_mut().ok_or("drawing-store.mutation-preflight-digest")?, cx)? {
                    return Ok(false);
                }
                let source_credit = self.preflight_source.as_ref().and_then(DrawingSnapshotBoundsAuthority::totals).ok_or("drawing-store.mutation-source-preflight-incomplete")?;
                let mutation_credit = digest.totals().ok_or("drawing-store.mutation-preflight-incomplete")?;
                let reverse_slots = self.container_reverse.as_ref().ok_or("drawing-store.mutation-reverse-arena-missing")?.capacity();
                let output_slots = self.container_output.as_ref().ok_or("drawing-store.mutation-output-arena-missing")?.capacity();
                let overlay_pages = self.overlay_pages.as_ref().ok_or("drawing-store.mutation-overlay-arena-missing")?;
                let overlay_slots = overlay_pages.capacity();
                if mutation_credit.derived_owner_items > overlay_slots {
                    return Err("drawing-store.mutation-overlay-slot-capacity");
                }
                let overlay_bytes = overlay_pages
                    .iter()
                    .try_fold(overlay_slots.checked_mul(std::mem::size_of::<String>()).ok_or("drawing-store.mutation-overlay-byte-overflow")?, |total, page| total.checked_add(page.capacity()).ok_or("drawing-store.mutation-overlay-byte-overflow"))?;
                let duplicate_id_bytes = self.duplicate_id_owner.as_ref().map_or(0, |value| std::mem::size_of::<String>().saturating_add(value.capacity()));
                self.reservation = Some(DrawingMutationAggregateReservation::admit(source_credit, mutation_credit, mutation, reverse_slots, output_slots, overlay_slots, overlay_bytes, duplicate_id_bytes)?);
                drop(self.preflight_mutation.take());
                self.preflight_source = None;
                self.preflight_digest = None;
                self.phase = DrawingMutationCandidatePhase::BindOverlay;
                Ok(false)
            }
            DrawingMutationCandidatePhase::BindOverlay => {
                self.overlay = Some(DrawingMutationOverlayPatch::bind(source));
                self.locator = Some(DrawingLayerLocator::new());
                self.phase = DrawingMutationCandidatePhase::LocatePrimary;
                cx.consume_fuel(1);
                Ok(false)
            }
            DrawingMutationCandidatePhase::LocatePrimary => {
                let locator = self.locator.as_mut().ok_or("drawing-store.mutation-locator-missing")?;
                if !locator.step(source, Self::target(mutation), cx)? {
                    return Ok(false);
                }
                self.primary = locator.found();
                self.locator = None;
                if matches!(mutation, DrawingMutation::CreateLayer(_)) {
                    if self.primary.is_some() {
                        return Err("drawing-store.mutation-duplicate-layer");
                    }
                    if Self::parent(mutation).is_some() {
                        self.locator = Some(DrawingLayerLocator::new());
                        self.phase = DrawingMutationCandidatePhase::LocateSecondary;
                    } else {
                        self.phase = DrawingMutationCandidatePhase::PrepareOwnedValue;
                    }
                } else if self.primary.is_none() {
                    return Err("drawing-store.mutation-target-missing");
                } else {
                    self.phase = DrawingMutationCandidatePhase::PrepareOwnedValue;
                }
                Ok(false)
            }
            DrawingMutationCandidatePhase::LocateSecondary => {
                let target = Self::parent(mutation).ok_or("drawing-store.mutation-parent-missing")?;
                let locator = self.locator.as_mut().ok_or("drawing-store.mutation-parent-locator")?;
                if !locator.step(source, target, cx)? {
                    return Ok(false);
                }
                self.secondary = locator.found();
                self.locator = None;
                let Some(address) = self.secondary else { return Err("drawing-store.mutation-parent-not-found") };
                if !matches!(DrawingLayerLocator::node_at(source, address), Some(DrawingLayerNode::Group(_))) {
                    return Err("drawing-store.mutation-parent-not-group");
                }
                self.phase = DrawingMutationCandidatePhase::PrepareOwnedValue;
                Ok(false)
            }
            DrawingMutationCandidatePhase::PrepareOwnedValue => {
                match mutation {
                    DrawingMutation::CreateLayer(value) => {
                        if self.layer_clone.is_none() {
                            *self.layer_clone = Some(Box::new(DrawingLayerCloneAuthority::new(&value.layer)?));
                            cx.consume_fuel(1);
                            return Ok(false);
                        }
                        let clone = self.layer_clone.as_mut().expect("Drawing create layer clone remains retained");
                        if !clone.step(&value.layer, self.preflight_digest.get_or_insert_with(|| store::ArtifactStoreInitializationDigest::new(b"drawing.create-layer")), cx)? {
                            return Ok(false);
                        }
                        *self.pending_layer = clone.take();
                        drop(self.layer_clone.take());
                    }
                    DrawingMutation::DuplicateLayer(_) => {
                        let duplicate_source = DrawingLayerLocator::node_at(source, self.primary.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-duplicate-source")?;
                        if self.pending_layer.is_none() {
                            if self.layer_clone.is_none() {
                                *self.layer_clone = Some(Box::new(DrawingLayerCloneAuthority::new(duplicate_source)?));
                                cx.consume_fuel(1);
                                return Ok(false);
                            }
                            let clone = self.layer_clone.as_mut().expect("Drawing duplicate layer clone remains retained");
                            if !clone.step(duplicate_source, self.preflight_digest.get_or_insert_with(|| store::ArtifactStoreInitializationDigest::new(b"drawing.duplicate-layer")), cx)? {
                                return Ok(false);
                            }
                            *self.pending_layer = clone.take();
                            drop(self.layer_clone.take());
                            let name_owner = self.overlay_pages.as_mut().ok_or("drawing-store.mutation-overlay-arena-missing")?.pop().ok_or("drawing-store.duplicate-name-owner-missing")?;
                            self.duplicate_rewrite = Some(DrawingDuplicateRewriteAuthority::new(self.duplicate_id_owner.take().ok_or("drawing-store.duplicate-id-owner-missing")?, name_owner));
                            return Ok(false);
                        }
                        if !self.duplicate_rewrite.as_mut().ok_or("drawing-store.duplicate-rewrite-missing")?.step(self.pending_layer.as_mut().ok_or("drawing-store.duplicate-owner-missing")?, cx)? {
                            return Ok(false);
                        }
                        let pages = self.overlay_pages.as_mut().ok_or("drawing-store.mutation-overlay-arena-missing")?;
                        if pages.len() >= pages.capacity() {
                            return Err("drawing-store.duplicate-name-owner-return-saturated");
                        }
                        let (id_owner, name_owner) = self.duplicate_rewrite.as_mut().and_then(DrawingDuplicateRewriteAuthority::take_owners).ok_or("drawing-store.duplicate-owner-false-terminal")?;
                        *self.duplicate_id_owner = Some(id_owner);
                        pages.push(name_owner);
                        drop(self.duplicate_rewrite.take());
                    }
                    DrawingMutation::ReplaceLayerFill(value) => {
                        if let Some(source) = value.fill.as_ref() {
                            if self.fill_clone.is_none() {
                                *self.fill_clone = Some(DrawingFillCloneAuthority::new(source)?);
                                cx.consume_fuel(1);
                                return Ok(false);
                            }
                            if !self.fill_clone.as_mut().expect("Drawing fill clone remains retained").step(source, cx)? {
                                return Ok(false);
                            }
                        }
                    }
                    DrawingMutation::ReplaceLayerStroke(value) => {
                        if let Some(source) = value.stroke.as_ref() {
                            if self.stroke_clone.is_none() {
                                *self.stroke_clone = Some(DrawingStrokeCloneAuthority::new(source)?);
                                cx.consume_fuel(1);
                                return Ok(false);
                            }
                            if !self.stroke_clone.as_mut().expect("Drawing stroke clone remains retained").step(source, cx)? {
                                return Ok(false);
                            }
                        }
                    }
                    _ => {}
                }
                self.phase = DrawingMutationCandidatePhase::Apply;
                Ok(false)
            }
            DrawingMutationCandidatePhase::Apply => {
                match mutation {
                    DrawingMutation::CreateLayer(value) => {
                        let parent = self.secondary;
                        let index = match value.index {
                            Some(index) => index,
                            None => DrawingLayerLocator::container_mut(source, parent).map_or(0, |values| values.len()),
                        };
                        self.start_rebuild(source, parent, None, Some(index), DrawingContainerRebuildRole::Destination)?;
                        self.phase = DrawingMutationCandidatePhase::RebuildDestination;
                        return Ok(false);
                    }
                    DrawingMutation::DuplicateLayer(_) => {
                        let address = self.primary.ok_or("drawing-store.mutation-primary-missing")?;
                        self.start_rebuild(source, address.parent(), None, Some(address.index() + 1), DrawingContainerRebuildRole::Destination)?;
                        self.phase = DrawingMutationCandidatePhase::RebuildDestination;
                        return Ok(false);
                    }
                    DrawingMutation::DeleteLayer(_) | DrawingMutation::ReorderLayer(_) => {
                        let address = self.primary.ok_or("drawing-store.mutation-primary-missing")?;
                        self.start_rebuild(source, address.parent(), Some(address.index()), None, DrawingContainerRebuildRole::Source)?;
                        self.source_undo = Some(DrawingContainerSourceUndo { parent: address.parent(), index: address.index() });
                        self.phase = DrawingMutationCandidatePhase::RebuildSource;
                        return Ok(false);
                    }
                    _ => {}
                }
                let address = self.primary;
                match mutation {
                    DrawingMutation::SetLayerVisible(value) => {
                        crate::artifacts::drawing::schema::layer_base_mut(DrawingLayerLocator::node_at_mut(source, address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")?).visible = value.visible
                    }
                    DrawingMutation::SetLayerLocked(value) => {
                        crate::artifacts::drawing::schema::layer_base_mut(DrawingLayerLocator::node_at_mut(source, address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")?).locked = value.locked
                    }
                    DrawingMutation::SetLayerOpacity(value) if value.opacity.is_finite() => {
                        crate::artifacts::drawing::schema::layer_base_mut(DrawingLayerLocator::node_at_mut(source, address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")?).opacity = value.opacity
                    }
                    DrawingMutation::SetLayerOpacity(_) => return Err("drawing-store.mutation-opacity-invalid"),
                    DrawingMutation::SetLayerBlendMode(value) => {
                        self.write_overlay_string(
                            &mut crate::artifacts::drawing::schema::layer_base_mut(DrawingLayerLocator::node_at_mut(source, address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")?).blend_mode,
                            &value.blend_mode,
                        )?;
                    }
                    DrawingMutation::RenameLayer(value) => {
                        self.write_overlay_string(
                            &mut crate::artifacts::drawing::schema::layer_base_mut(DrawingLayerLocator::node_at_mut(source, address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")?).name,
                            &value.new_name,
                        )?;
                    }
                    DrawingMutation::UpdateLayerTransform(value)
                        if [value.transform.x, value.transform.y, value.transform.scale_x, value.transform.scale_y, value.transform.rotation].iter().all(|field| field.is_finite()) && value.transform.scale_x > 0.0 && value.transform.scale_y > 0.0 =>
                    {
                        crate::artifacts::drawing::schema::layer_base_mut(DrawingLayerLocator::node_at_mut(source, address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")?).transform =
                            crate::artifacts::drawing::DrawingTransform { x: value.transform.x, y: value.transform.y, scale_x: value.transform.scale_x, scale_y: value.transform.scale_y, rotation: value.transform.rotation };
                    }
                    DrawingMutation::UpdateLayerTransform(_) => return Err("drawing-store.mutation-transform-invalid"),
                    DrawingMutation::ReplaceLayerFill(value) => {
                        let replacement = match value.fill.as_ref() {
                            Some(_) => Some(self.fill_clone.as_mut().ok_or("drawing-store.fill-clone-missing")?.take().ok_or("drawing-store.fill-false-terminal")?),
                            None => None,
                        };
                        let old = std::mem::replace(
                            &mut crate::artifacts::drawing::schema::layer_base_mut(DrawingLayerLocator::node_at_mut(source, address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")?).attributes.fill,
                            replacement,
                        );
                        if let Some(old) = old {
                            *self.retirement = Some(Box::new(DrawingOwnedRetirement::new(DrawingRetirementOwner::Fill(old))));
                        }
                        if let Some(mut clone) = self.fill_clone.take() {
                            clone.terminal = true;
                            drop(clone);
                        }
                    }
                    DrawingMutation::ReplaceLayerStroke(value) => {
                        let replacement = match value.stroke.as_ref() {
                            Some(_) => Some(self.stroke_clone.as_mut().ok_or("drawing-store.stroke-clone-missing")?.take().ok_or("drawing-store.stroke-false-terminal")?),
                            None => None,
                        };
                        let old = std::mem::replace(
                            &mut crate::artifacts::drawing::schema::layer_base_mut(DrawingLayerLocator::node_at_mut(source, address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")?).attributes.stroke,
                            replacement,
                        );
                        if let Some(old) = old {
                            *self.retirement = Some(Box::new(DrawingOwnedRetirement::new(DrawingRetirementOwner::Stroke(old))));
                        }
                        if let Some(mut clone) = self.stroke_clone.take() {
                            clone.terminal = true;
                            drop(clone);
                        }
                    }
                    DrawingMutation::SetLayerBooleanOperation(value) => {
                        let DrawingLayerNode::Boolean(target) = DrawingLayerLocator::node_at_mut(source, address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")? else {
                            return Err("drawing-store.mutation-boolean-target");
                        };
                        self.write_overlay_string(&mut target.operation, &value.boolean_operation)?;
                    }
                    DrawingMutation::UpdateLayerTraceParams(value) if value.params.threshold.is_finite() && value.params.simplify_epsilon.is_finite() => {
                        let DrawingLayerNode::Trace(target) = DrawingLayerLocator::node_at_mut(source, address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")? else {
                            return Err("drawing-store.mutation-trace-target");
                        };
                        target.params = crate::artifacts::drawing::DrawingTraceParams { threshold: value.params.threshold, simplify_epsilon: value.params.simplify_epsilon };
                    }
                    DrawingMutation::UpdateLayerTraceParams(_) => return Err("drawing-store.mutation-trace-invalid"),
                    DrawingMutation::CreateLayer(_) | DrawingMutation::DuplicateLayer(_) | DrawingMutation::DeleteLayer(_) | DrawingMutation::ReorderLayer(_) => unreachable!("structural Drawing mutations start retained rebuild before scalar mutation"),
                }
                self.overlay.as_mut().ok_or("drawing-store.mutation-overlay-missing")?.commit(source)?;
                self.phase = DrawingMutationCandidatePhase::Complete;
                cx.consume_fuel(1);
                Ok(false)
            }
            DrawingMutationCandidatePhase::RebuildSource => {
                if !self.rebuild.as_mut().ok_or("drawing-store.mutation-rebuild-missing")?.step(cx)? {
                    return Ok(false);
                }
                let parent = self.primary.ok_or("drawing-store.mutation-primary-missing")?.parent();
                let removed = self.finish_rebuild(source, parent)?.ok_or("drawing-store.mutation-removal-missing")?;
                *self.pending_layer = Some(removed);
                if matches!(mutation, DrawingMutation::DeleteLayer(_)) {
                    self.overlay.as_mut().ok_or("drawing-store.mutation-overlay-missing")?.commit(source)?;
                    self.source_undo = None;
                    self.phase = DrawingMutationCandidatePhase::Complete;
                } else {
                    self.secondary = None;
                    if Self::parent(mutation).is_some() {
                        self.locator = Some(DrawingLayerLocator::new());
                        self.phase = DrawingMutationCandidatePhase::LocateDestination;
                    } else {
                        self.phase = DrawingMutationCandidatePhase::RebuildDestination;
                    }
                }
                Ok(false)
            }
            DrawingMutationCandidatePhase::LocateDestination => {
                let locator = self.locator.as_mut().ok_or("drawing-store.mutation-destination-locator")?;
                if !locator.step(source, Self::parent(mutation).ok_or("drawing-store.mutation-parent-missing")?, cx)? {
                    return Ok(false);
                }
                self.secondary = locator.found();
                self.locator = None;
                let Some(address) = self.secondary else { return Err("drawing-store.mutation-parent-not-found") };
                if !matches!(DrawingLayerLocator::node_at(source, address), Some(DrawingLayerNode::Group(_))) {
                    return Err("drawing-store.mutation-parent-not-group");
                }
                self.phase = DrawingMutationCandidatePhase::RebuildDestination;
                Ok(false)
            }
            DrawingMutationCandidatePhase::RebuildDestination => {
                if self.rebuild.is_none() {
                    let parent = match mutation {
                        DrawingMutation::CreateLayer(_) => self.secondary,
                        DrawingMutation::DuplicateLayer(_) => self.primary.ok_or("drawing-store.mutation-primary-missing")?.parent(),
                        DrawingMutation::ReorderLayer(_) => self.secondary,
                        _ => return Err("drawing-store.mutation-destination-variant"),
                    };
                    let index = match mutation {
                        DrawingMutation::CreateLayer(value) => value.index.unwrap_or_else(|| DrawingLayerLocator::container_mut(source, parent).map_or(0, |values| values.len())),
                        DrawingMutation::DuplicateLayer(_) => self.primary.ok_or("drawing-store.mutation-primary-missing")?.index() + 1,
                        DrawingMutation::ReorderLayer(value) => value.index,
                        _ => 0,
                    };
                    self.start_rebuild(source, parent, None, Some(index), DrawingContainerRebuildRole::Destination)?;
                }
                if !self.rebuild.as_mut().ok_or("drawing-store.mutation-rebuild-missing")?.step(cx)? {
                    return Ok(false);
                }
                let parent = match mutation {
                    DrawingMutation::CreateLayer(_) | DrawingMutation::ReorderLayer(_) => self.secondary,
                    DrawingMutation::DuplicateLayer(_) => self.primary.ok_or("drawing-store.mutation-primary-missing")?.parent(),
                    _ => None,
                };
                if self.finish_rebuild(source, parent)?.is_some() {
                    return Err("drawing-store.mutation-unexpected-removal");
                }
                self.overlay.as_mut().ok_or("drawing-store.mutation-overlay-missing")?.commit(source)?;
                self.source_undo = None;
                self.phase = DrawingMutationCandidatePhase::Complete;
                Ok(false)
            }
            DrawingMutationCandidatePhase::Complete => {
                if let Some(value) = self.pending_layer.take() {
                    *self.retirement = Some(Box::new(DrawingOwnedRetirement::new(DrawingRetirementOwner::Layer(value))));
                    return Ok(false);
                }
                if let Some(retirement) = self.retirement.as_mut() {
                    return match retirement.close_step(1, DRAWING_OWNED_FIELD_BYTES).map_err(|_| "drawing-store.mutation-retirement")? {
                        store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                            drop(self.retirement.take());
                            Ok(false)
                        }
                        store::SnapshotRetirementStep::Complete => self.fail("drawing-store.mutation-retirement-false-terminal"),
                        _ => Ok(false),
                    };
                }
                if self.return_arena_owner()? != Some(true) {
                    return Ok(false);
                }
                if !self.overlay.as_ref().is_some_and(|overlay| overlay.committed) {
                    self.overlay.as_mut().ok_or("drawing-store.mutation-overlay-missing")?.commit(source)?;
                }
                self.terminal = true;
                Ok(true)
            }
            DrawingMutationCandidatePhase::Retire | DrawingMutationCandidatePhase::Fault => Err(self.fault.unwrap_or("drawing-store.mutation-candidate-fault")),
        }
    }

    fn take(&mut self) -> Option<()> {
        if !self.terminal {
            return None;
        }
        self.preflight_source = None;
        self.preflight_digest = None;
        self.reservation = None;
        if !self.overlay.as_ref().is_some_and(|overlay| overlay.committed) {
            return None;
        }
        self.overlay = None;
        Some(())
    }

    fn pump_rebuild_close(&mut self, source: &mut DrawingSnapshot) -> Result<store::SnapshotRetirementStep, String> {
        let role = self.rebuild_role.ok_or("Drawing mutation rebuild role missing")?;
        let target = self.rebuild_target.ok_or("Drawing mutation rebuild target missing")?;
        let rebuild = self.rebuild.as_mut().ok_or("Drawing mutation rebuild missing")?;
        let ready = if role == DrawingContainerRebuildRole::CloseSourceUndo { rebuild.close_forward_step()? } else { rebuild.rollback_step()? };
        if !ready {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        match self.rebuild_close_phase {
            0 => {
                let container = DrawingLayerLocator::container_mut(source, target).ok_or("Drawing mutation rollback container missing")?;
                if !container.is_empty() {
                    return Err("Drawing mutation rollback destination was not empty".into());
                }
                *container = rebuild.source.take().ok_or("Drawing mutation rollback source missing")?;
            }
            1 => {
                if rebuild.pending.is_some() && self.pending_layer.is_some() {
                    return Err("Drawing mutation rollback pending owner collision".into());
                }
                *self.pending_layer = rebuild.pending.take();
            }
            2 => {
                if self.container_reverse.is_some() {
                    return Err("Drawing mutation rollback reverse owner collision".into());
                }
                *self.container_reverse = Some(rebuild.reverse.take().ok_or("Drawing mutation rollback reverse owner missing")?);
            }
            3 => {
                if self.container_output.is_some() {
                    return Err("Drawing mutation rollback output owner collision".into());
                }
                *self.container_output = Some(rebuild.output.take().ok_or("Drawing mutation rollback output owner missing")?);
            }
            4 => {
                if rebuild.removed.is_some() {
                    return Err("Drawing mutation rollback retained an unexpected removed owner".into());
                }
                rebuild.finish_handoff()?;
                drop(self.rebuild.take());
                self.rebuild_target = None;
                self.rebuild_role = None;
                self.rebuild_close_phase = 0;
                if role != DrawingContainerRebuildRole::Destination {
                    self.source_undo = None;
                }
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            _ => return Err("Drawing mutation rollback handoff phase invalid".into()),
        }
        self.rebuild_close_phase += 1;
        Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
    }

    fn close_step(&mut self, mut source: Option<&mut DrawingSnapshot>, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if let Some(preflight) = self.preflight_mutation.as_mut() {
            return match preflight.close_step(maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if preflight.terminal_is_empty() => {
                    drop(self.preflight_mutation.take());
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                store::SnapshotRetirementStep::Complete => Err("Drawing mutation preflight reported false terminal".into()),
                step => Ok(step),
            };
        }
        if let Some(retirement) = self.retirement.as_mut() {
            return match retirement.close_step(1, maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                    drop(self.retirement.take());
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                store::SnapshotRetirementStep::Complete => Err("Drawing mutation candidate retirement reported false terminal".into()),
                step => Ok(step),
            };
        }
        if let Some(rewrite) = self.duplicate_rewrite.as_mut() {
            return match rewrite.close_step(maximum_bytes)? {
                store::SnapshotRetirementStep::Complete => {
                    let pages = self.overlay_pages.as_mut().ok_or("Drawing duplicate name arena missing")?;
                    if pages.len() >= pages.capacity() {
                        return Err("Drawing duplicate name arena return saturated".into());
                    }
                    let (id_owner, name_owner) = rewrite.take_owners().ok_or("Drawing duplicate rewrite reported false terminal")?;
                    *self.duplicate_id_owner = Some(id_owner);
                    pages.push(name_owner);
                    drop(self.duplicate_rewrite.take());
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                step => Ok(step),
            };
        }
        if self.rebuild.is_some() {
            let Some(source) = source.as_deref_mut() else { return Ok(store::SnapshotRetirementStep::Blocked) };
            return self.pump_rebuild_close(source);
        }
        if !self.overlay.as_ref().is_some_and(|overlay| overlay.committed) {
            if let Some(undo) = self.source_undo {
                let Some(source) = source.as_deref_mut() else { return Ok(store::SnapshotRetirementStep::Blocked) };
                self.start_rebuild(source, undo.parent, None, Some(undo.index), DrawingContainerRebuildRole::CloseSourceUndo)?;
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
        }
        if let Some(layer) = self.layer_clone.as_mut() {
            return match layer.close_step(1, maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if layer.terminal_is_empty() => {
                    drop(self.layer_clone.take());
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                store::SnapshotRetirementStep::Complete => Err("Drawing mutation layer clone reported false terminal".into()),
                step => Ok(step),
            };
        }
        if let Some(fill) = self.fill_clone.as_mut() {
            return match fill.close_step(maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if fill.terminal_is_empty() => {
                    drop(self.fill_clone.take());
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                store::SnapshotRetirementStep::Complete => Err("Drawing mutation fill clone reported false terminal".into()),
                step => Ok(step),
            };
        }
        if let Some(stroke) = self.stroke_clone.as_mut() {
            return match stroke.close_step(maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if stroke.terminal_is_empty() => {
                    drop(self.stroke_clone.take());
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                store::SnapshotRetirementStep::Complete => Err("Drawing mutation stroke clone reported false terminal".into()),
                step => Ok(step),
            };
        }
        if let Some(value) = self.pending_layer.take() {
            *self.retirement = Some(Box::new(DrawingOwnedRetirement::new(DrawingRetirementOwner::Layer(value))));
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        match self.return_arena_owner() {
            Ok(Some(true)) => {}
            Ok(Some(false)) => return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }),
            Ok(None) => return Ok(store::SnapshotRetirementStep::Blocked),
            Err(error) => return Err(error.into()),
        }
        self.preflight_source = None;
        self.preflight_digest = None;
        self.reservation = None;
        self.overlay = None;
        self.terminal = true;
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal
            && self.preflight_source.is_none()
            && self.preflight_mutation.is_none()
            && self.preflight_digest.is_none()
            && self.reservation.is_none()
            && self.arena_pool.is_none()
            && self.arena_return_phase == 4
            && self.overlay.is_none()
            && self.layer_clone.is_none()
            && self.fill_clone.is_none()
            && self.stroke_clone.is_none()
            && self.duplicate_rewrite.is_none()
            && self.duplicate_id_owner.is_none()
            && self.rebuild.is_none()
            && self.rebuild_target.is_none()
            && self.rebuild_role.is_none()
            && self.rebuild_close_phase == 0
            && self.source_undo.is_none()
            && self.container_reverse.is_none()
            && self.container_output.is_none()
            && self.overlay_pages.is_none()
            && self.pending_layer.is_none()
            && self.retirement.is_none()
    }
}

impl Drop for DrawingMutationCandidateAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Drawing mutation candidate reached Drop before atomic handoff or cursor retirement");
    }
}

pub fn drawing_document_store_owners() -> store::MemberStoreOwners<DrawingSnapshot, DrawingMutation> {
    store::MemberStoreOwners::new(
        std::sync::Arc::new(DrawingSnapshotRetirementFactory),
        std::sync::Arc::new(DrawingSnapshotRetirementFactory),
        std::sync::Arc::new(DrawingMutationRetirementFactory),
        Box::new(store::ArtifactStoreCursorDisposer::<DrawingSnapshot, DrawingMutation>::new()),
    )
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DrawingStoreInitializationPhase {
    InitializeArena,
    ValidateEnvelope,
    ValidateEditId { edit: usize },
    ValidateEditMeta { edit: usize, meta: usize },
    ValidateEditPair { left: usize, right: usize },
    HashInitialSchema,
    HashInitialId,
    MoveInitialOwner,
    SeedHistory { edit: usize, lane: u8, index: usize },
    FindApplied { position: usize, scan: usize },
    ApplyForward { position: usize, edit: usize, mutation: usize },
    HashInverse { position: usize, edit: usize, mutation: usize },
    PrepareApplied { position: usize, edit: usize, field: u8 },
    CommitApplied { position: usize, edit: usize },
    FindRedo { position: usize, scan: usize },
    HashRedoForward { position: usize, edit: usize, mutation: usize },
    HashRedoInverse { position: usize, edit: usize, mutation: usize },
    PrepareRedo { position: usize, edit: usize },
    CommitRedo { position: usize, edit: usize },
    BuildCandidate,
    RetireCancelled,
    RetireFault,
    Complete,
    Cancelled,
    Fault,
}

struct DrawingStoreInitializationAuthority {
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    arena_bootstrap_job: DrawingMutationArenaBootstrapJob,
    envelope: std::mem::ManuallyDrop<Option<store::ArtifactEnvelope<DrawingSnapshot, DrawingMutation>>>,
    runtime: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationRuntime<DrawingSnapshot>>>,
    candidate: std::mem::ManuallyDrop<Option<store::ArtifactStore<DrawingSnapshot, DrawingMutation>>>,
    active: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    envelope_retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    owner_catalog: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationOwnerCatalog>>,
    mutation_digest: std::mem::ManuallyDrop<Option<DrawingMutationDigestAuthority>>,
    mutation_candidate: std::mem::ManuallyDrop<Option<DrawingMutationCandidateAuthority>>,
    prepared_history_id: std::mem::ManuallyDrop<Option<String>>,
    prepared_actor: std::mem::ManuallyDrop<Option<String>>,
    initial_digest: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationDigest>>,
    edit_digest: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationDigest>>,
    phase: DrawingStoreInitializationPhase,
    cancel_requested: bool,
    fault: Option<Vec<u8>>,
    terminal_handoff: bool,
}

impl DrawingStoreInitializationAuthority {
    fn new(
        envelope: store::ArtifactEnvelope<DrawingSnapshot, DrawingMutation>,
        owner_catalog: Result<store::ArtifactStoreInitializationOwnerCatalog, &'static str>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
    ) -> Self {
        let bootstrap_job = DrawingMutationArenaBootstrapJob::new(operation, generation);
        let (owner_catalog, arena_bootstrap_job, phase, fault) = match (owner_catalog, bootstrap_job) {
            (Ok(owner_catalog), Ok(arena_bootstrap_job)) => (Some(owner_catalog), arena_bootstrap_job, DrawingStoreInitializationPhase::InitializeArena, None),
            (Err(error), Ok(mut arena_bootstrap_job)) => {
                arena_bootstrap_job.terminal = true;
                (None, arena_bootstrap_job, DrawingStoreInitializationPhase::RetireFault, Some(error.as_bytes().to_vec()))
            }
            (_, Err(error)) => (None, DrawingMutationArenaBootstrapJob::inactive(operation, generation), DrawingStoreInitializationPhase::RetireFault, Some(error.as_bytes().to_vec())),
        };
        Self {
            operation,
            generation,
            arena_bootstrap_job,
            envelope: std::mem::ManuallyDrop::new(Some(envelope)),
            runtime: std::mem::ManuallyDrop::new(None),
            candidate: std::mem::ManuallyDrop::new(None),
            active: std::mem::ManuallyDrop::new(None),
            envelope_retirement: std::mem::ManuallyDrop::new(None),
            owner_catalog: std::mem::ManuallyDrop::new(owner_catalog),
            mutation_digest: std::mem::ManuallyDrop::new(None),
            mutation_candidate: std::mem::ManuallyDrop::new(None),
            prepared_history_id: std::mem::ManuallyDrop::new(None),
            prepared_actor: std::mem::ManuallyDrop::new(None),
            initial_digest: std::mem::ManuallyDrop::new(Some(store::ArtifactStoreInitializationDigest::new(b"drawing.initial"))),
            edit_digest: std::mem::ManuallyDrop::new(None),
            phase,
            cancel_requested: false,
            fault,
            terminal_handoff: false,
        }
    }

    fn applied_id(&self, position: usize) -> Option<&str> {
        let envelope = self.envelope.as_ref()?;
        match &envelope.cursor {
            Some(cursor) => cursor.applied_edit_ids.get(position).map(String::as_str),
            None => envelope.vcs.edits.get(position).map(|edit| edit.id.as_str()),
        }
    }

    fn redo_id(&self, position: usize) -> Option<&str> {
        self.envelope.as_ref()?.cursor.as_ref()?.redo_edit_ids.get(position).map(String::as_str)
    }

    fn fail(&mut self, code: &'static [u8]) {
        self.fault = Some(code.to_vec());
        self.phase = DrawingStoreInitializationPhase::RetireFault;
    }

    fn pump_active(&mut self) -> Result<bool, String> {
        let Some(active) = self.active.as_mut() else { return Ok(false) };
        match active.close_step(1, DRAWING_OWNED_FIELD_BYTES)? {
            store::SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items <= 1 && released_bytes <= DRAWING_OWNED_FIELD_BYTES => Ok(true),
            store::SnapshotRetirementStep::Pending { .. } => Err("Drawing store initializer retirement exceeded its exact grant".into()),
            store::SnapshotRetirementStep::Blocked => Ok(true),
            store::SnapshotRetirementStep::Complete if active.terminal_is_empty() => {
                drop(self.active.take());
                Ok(true)
            }
            store::SnapshotRetirementStep::Complete => Err("Drawing store initializer retirement reported a false terminal".into()),
        }
    }

    fn pump_terminal_retirement(&mut self) -> Result<bool, String> {
        if self.pump_active()? {
            return Ok(false);
        }
        if let Some(value) = self.prepared_history_id.take() {
            *self.active = Some(Box::new(DrawingOwnedRetirement::new(DrawingRetirementOwner::String(value))));
            return Ok(false);
        }
        if let Some(value) = self.prepared_actor.take() {
            *self.active = Some(Box::new(DrawingOwnedRetirement::new(DrawingRetirementOwner::String(value))));
            return Ok(false);
        }
        if self.mutation_candidate.is_some() {
            let current = self.runtime.as_mut().and_then(store::ArtifactStoreInitializationRuntime::current_mut);
            let candidate = self.mutation_candidate.as_mut().expect("Drawing mutation candidate remains retained during close");
            return match candidate.close_step(current, DRAWING_OWNED_FIELD_BYTES)? {
                store::SnapshotRetirementStep::Complete if candidate.terminal_is_empty() => {
                    drop(self.mutation_candidate.take());
                    Ok(false)
                }
                store::SnapshotRetirementStep::Complete => Err("Drawing mutation candidate reported a false terminal".into()),
                _ => Ok(false),
            };
        }
        if let Some(runtime) = self.runtime.as_mut() {
            match runtime.close_step(&DrawingSnapshotRetirementFactory, 1, DRAWING_OWNED_FIELD_BYTES)? {
                store::SnapshotRetirementStep::Complete if runtime.terminal_is_empty() => {
                    drop(self.runtime.take());
                    return Ok(false);
                }
                store::SnapshotRetirementStep::Complete => return Err("Drawing initialization runtime reported a false terminal".into()),
                _ => return Ok(false),
            }
        }
        drop(self.owner_catalog.take());
        if let Some(digest) = self.mutation_digest.as_mut() {
            return match digest.close_step(DRAWING_OWNED_FIELD_BYTES)? {
                store::SnapshotRetirementStep::Complete if digest.terminal_is_empty() => {
                    drop(self.mutation_digest.take());
                    Ok(false)
                }
                store::SnapshotRetirementStep::Complete => Err("Drawing mutation digest reported a false terminal".into()),
                _ => Ok(false),
            };
        }
        if self.envelope_retirement.is_none() {
            if let Some(envelope) = self.envelope.take() {
                *self.envelope_retirement = Some(drawing_envelope_decode_owner_bundle().retire_envelope(envelope));
                return Ok(false);
            }
        }
        if let Some(retirement) = self.envelope_retirement.as_mut() {
            return match retirement.close_step(1, DRAWING_OWNED_FIELD_BYTES)? {
                store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                    drop(self.envelope_retirement.take());
                    Ok(true)
                }
                store::SnapshotRetirementStep::Complete => Err("Drawing initialization envelope retirement reported a false terminal".into()),
                _ => Ok(false),
            };
        }
        Ok(true)
    }

    fn terminal_is_empty_inner(&self) -> bool {
        self.terminal_handoff
            && self.arena_bootstrap_job.terminal
            && self.envelope.is_none()
            && self.runtime.is_none()
            && self.candidate.is_none()
            && self.active.is_none()
            && self.envelope_retirement.is_none()
            && self.owner_catalog.is_none()
            && self.mutation_digest.is_none()
            && self.mutation_candidate.is_none()
            && self.prepared_history_id.is_none()
            && self.prepared_actor.is_none()
            && self.initial_digest.is_none()
            && self.edit_digest.is_none()
    }
}

impl semio_framework_plugin::ArtifactStoreInitializationAuthority<DrawingSnapshot, DrawingMutation> for DrawingStoreInitializationAuthority {
    fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
        if cx.operation() != self.operation || cx.generation() != self.generation {
            self.arena_bootstrap_job.terminal = true;
            self.fail(b"drawing-store.initializer-stale-authority");
        }
        if (self.cancel_requested || cx.is_cancelled()) && !matches!(self.phase, DrawingStoreInitializationPhase::InitializeArena | DrawingStoreInitializationPhase::RetireCancelled | DrawingStoreInitializationPhase::Cancelled) {
            self.phase = DrawingStoreInitializationPhase::RetireCancelled;
        }
        if let Err(error) = self.pump_active() {
            self.fault = Some(error.into_bytes());
            self.phase = DrawingStoreInitializationPhase::RetireFault;
        } else if self.active.is_some() {
            return semio_framework_job::StepOutcome::Yield;
        }
        match self.phase {
            DrawingStoreInitializationPhase::InitializeArena => {
                match self.arena_bootstrap_job.step(cx) {
                    DrawingMutationArenaBootstrapStep::Ready => self.phase = DrawingStoreInitializationPhase::ValidateEnvelope,
                    DrawingMutationArenaBootstrapStep::Pending { .. } | DrawingMutationArenaBootstrapStep::Blocked => {}
                    DrawingMutationArenaBootstrapStep::Cancelled => self.phase = DrawingStoreInitializationPhase::RetireCancelled,
                    DrawingMutationArenaBootstrapStep::Fault(error) => self.fail(error.as_bytes()),
                }
                semio_framework_job::StepOutcome::Yield
            }
            DrawingStoreInitializationPhase::ValidateEnvelope => {
                let Some(envelope) = self.envelope.as_ref() else {
                    self.fail(b"drawing-store.initializer-envelope-missing");
                    return semio_framework_job::StepOutcome::Yield;
                };
                if envelope.schema != crate::artifacts::drawing::DRAWING_DOCUMENT_SCHEMA || envelope.id.is_empty() || envelope.id.len() > DRAWING_OWNED_FIELD_BYTES {
                    self.fail(b"drawing-store.initializer-envelope-invalid");
                } else {
                    self.phase = DrawingStoreInitializationPhase::ValidateEditId { edit: 0 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            DrawingStoreInitializationPhase::ValidateEditId { edit } => {
                let envelope = self.envelope.as_ref().expect("validated Drawing envelope remains retained");
                let Some(entry) = envelope.vcs.edits.get(edit) else {
                    self.phase = DrawingStoreInitializationPhase::ValidateEditPair { left: 0, right: 1 };
                    return semio_framework_job::StepOutcome::Yield;
                };
                if entry.id.is_empty() || entry.id.len() > DRAWING_OWNED_FIELD_BYTES || entry.actor.as_ref().is_some_and(|actor| actor.len() > DRAWING_OWNED_FIELD_BYTES) || entry.started_at.len() > DRAWING_OWNED_FIELD_BYTES {
                    self.fail(b"drawing-store.initializer-hostile-edit-field");
                } else {
                    self.phase = DrawingStoreInitializationPhase::ValidateEditMeta { edit, meta: 0 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            DrawingStoreInitializationPhase::ValidateEditMeta { edit, meta } => {
                let envelope = self.envelope.as_ref().expect("validated Drawing envelope remains retained");
                let entry = envelope.vcs.edits.get(edit).expect("Drawing edit remains retained during metadata validation");
                let Some(value) = entry.mutation_meta.get(meta) else {
                    self.phase = DrawingStoreInitializationPhase::ValidateEditId { edit: edit + 1 };
                    return semio_framework_job::StepOutcome::Yield;
                };
                if value.mutation_id.as_ref().is_some_and(|id| id.0.len() > DRAWING_OWNED_FIELD_BYTES) {
                    self.fail(b"drawing-store.initializer-hostile-edit-field");
                } else {
                    self.phase = DrawingStoreInitializationPhase::ValidateEditMeta { edit, meta: meta + 1 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            DrawingStoreInitializationPhase::ValidateEditPair { left, right } => {
                let envelope = self.envelope.as_ref().expect("validated Drawing envelope remains retained");
                if left >= envelope.vcs.edits.len() {
                    self.phase = DrawingStoreInitializationPhase::HashInitialSchema;
                } else if right >= envelope.vcs.edits.len() {
                    self.phase = DrawingStoreInitializationPhase::ValidateEditPair { left: left + 1, right: left + 2 };
                } else if envelope.vcs.edits[left].id == envelope.vcs.edits[right].id {
                    self.fail(b"drawing-store.initializer-duplicate-edit");
                } else {
                    self.phase = DrawingStoreInitializationPhase::ValidateEditPair { left, right: right + 1 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            DrawingStoreInitializationPhase::HashInitialSchema => {
                let source = &self.envelope.as_ref().expect("Drawing envelope remains retained during initial digest").vcs.initial_snapshot;
                self.initial_digest.as_mut().expect("Drawing initial digest remains retained").observe(source.schema.as_bytes());
                self.phase = DrawingStoreInitializationPhase::HashInitialId;
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            DrawingStoreInitializationPhase::HashInitialId => {
                let source = &self.envelope.as_ref().expect("Drawing envelope remains retained during initial digest").vcs.initial_snapshot;
                self.initial_digest.as_mut().expect("Drawing initial digest remains retained").observe(source.id.as_bytes());
                self.phase = DrawingStoreInitializationPhase::MoveInitialOwner;
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            DrawingStoreInitializationPhase::MoveInitialOwner => {
                let envelope = self.envelope.as_mut().expect("Drawing envelope remains retained during initial owner move");
                let initial = std::mem::replace(&mut envelope.vcs.initial_snapshot, DrawingSnapshot { schema: String::new(), id: String::new(), title: None, layers: Vec::new(), assets: std::collections::BTreeMap::new(), artboard: None });
                let initial_digest = self.initial_digest.take().expect("Drawing initial digest remains retained").finish();
                let owner_catalog = self.owner_catalog.take().expect("Drawing owner catalog was pre-admitted before initialization");
                *self.runtime = Some(store::ArtifactStoreInitializationRuntime::new_with_owner_catalog(&envelope.id, &envelope.schema, initial, initial_digest, owner_catalog));
                self.phase = DrawingStoreInitializationPhase::SeedHistory { edit: 0, lane: 0, index: 0 };
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            DrawingStoreInitializationPhase::SeedHistory { edit, lane, index } => {
                let envelope = self.envelope.as_ref().expect("Drawing envelope remains retained while causal history is seeded");
                let Some(entry) = envelope.vcs.edits.get(edit) else {
                    self.phase = DrawingStoreInitializationPhase::FindApplied { position: 0, scan: 0 };
                    return semio_framework_job::StepOutcome::Yield;
                };
                let runtime = self.runtime.as_mut().expect("Drawing runtime remains retained while history is seeded");
                match lane {
                    0 => {
                        if let Err(error) = runtime.seed_mutation(protocol::MutationId(entry.id.clone())) {
                            self.fault = Some(error.into_bytes());
                            self.phase = DrawingStoreInitializationPhase::RetireFault;
                        } else {
                            runtime.observe_sequence(entry.sequence_number);
                            self.phase = DrawingStoreInitializationPhase::SeedHistory { edit, lane: 1, index: 0 };
                        }
                    }
                    1 if index < entry.forwards.len() => {
                        let id = entry.mutation_meta.get(index).and_then(|meta| meta.mutation_id.clone()).or_else(|| entry.forwards[index].mutation_id()).unwrap_or_else(|| protocol::MutationId(format!("{}#{index}", entry.id)));
                        if let Err(error) = runtime.seed_mutation(id) {
                            self.fault = Some(error.into_bytes());
                            self.phase = DrawingStoreInitializationPhase::RetireFault;
                        } else {
                            self.phase = DrawingStoreInitializationPhase::SeedHistory { edit, lane, index: index + 1 };
                        }
                    }
                    1 => self.phase = DrawingStoreInitializationPhase::SeedHistory { edit, lane: 2, index: 0 },
                    2 if index < entry.mutation_meta.len() => {
                        runtime.observe_timestamp(entry.mutation_meta[index].timestamp.clone());
                        self.phase = DrawingStoreInitializationPhase::SeedHistory { edit, lane, index: index + 1 };
                    }
                    _ => self.phase = DrawingStoreInitializationPhase::SeedHistory { edit: edit + 1, lane: 0, index: 0 },
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            DrawingStoreInitializationPhase::FindApplied { position, scan } => {
                let Some(id) = self.applied_id(position) else {
                    let checkpoint = self.envelope.as_ref().and_then(|envelope| envelope.cursor.as_ref().and_then(|cursor| cursor.checkpoint_id.clone()).or_else(|| envelope.vcs.checkpoints.last().map(|checkpoint| checkpoint.id.clone())));
                    self.runtime.as_mut().expect("Drawing runtime remains retained").set_current_checkpoint_id(checkpoint);
                    self.phase = DrawingStoreInitializationPhase::FindRedo { position: 0, scan: 0 };
                    return semio_framework_job::StepOutcome::Yield;
                };
                let envelope = self.envelope.as_ref().expect("Drawing envelope remains retained");
                let Some(edit) = envelope.vcs.edits.get(scan) else {
                    self.fail(b"drawing-store.initializer-applied-edit-missing");
                    return semio_framework_job::StepOutcome::Yield;
                };
                if edit.id == id {
                    let mut digest = store::ArtifactStoreInitializationDigest::new(b"drawing.edit");
                    digest.observe(edit.id.as_bytes());
                    digest.observe(&edit.sequence_number.to_be_bytes());
                    digest.observe(edit.started_at.as_bytes());
                    *self.edit_digest = Some(digest);
                    self.phase = DrawingStoreInitializationPhase::ApplyForward { position, edit: scan, mutation: 0 };
                } else {
                    self.phase = DrawingStoreInitializationPhase::FindApplied { position, scan: scan + 1 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            DrawingStoreInitializationPhase::ApplyForward { position, edit, mutation } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Drawing applied edit remains retained");
                let Some(operation) = entry.forwards.get(mutation) else {
                    self.phase = DrawingStoreInitializationPhase::HashInverse { position, edit, mutation: 0 };
                    return semio_framework_job::StepOutcome::Yield;
                };
                if self.mutation_digest.is_none() {
                    *self.mutation_digest = Some(DrawingMutationDigestAuthority::new());
                    cx.consume_fuel(1);
                    return semio_framework_job::StepOutcome::Yield;
                }
                let digest_complete = match self.mutation_digest.as_mut().expect("Drawing mutation digest remains retained").step(operation, self.edit_digest.as_mut().expect("Drawing edit digest remains retained"), cx) {
                    Ok(complete) => complete,
                    Err(error) => {
                        self.fail(error.as_bytes());
                        return semio_framework_job::StepOutcome::Yield;
                    }
                };
                if !digest_complete {
                    return semio_framework_job::StepOutcome::Yield;
                }
                drop(self.mutation_digest.take());
                if self.mutation_candidate.is_none() {
                    match DrawingMutationCandidateAuthority::try_new(self.operation, self.generation) {
                        Ok(candidate) => *self.mutation_candidate = Some(candidate),
                        Err(error) => {
                            self.fail(error.as_bytes());
                            return semio_framework_job::StepOutcome::Yield;
                        }
                    }
                    cx.consume_fuel(1);
                    return semio_framework_job::StepOutcome::Yield;
                }
                let current = self.runtime.as_mut().and_then(store::ArtifactStoreInitializationRuntime::current_mut).expect("Drawing runtime current snapshot remains retained");
                let candidate_complete = match self.mutation_candidate.as_mut().expect("Drawing mutation candidate remains retained").step(current, operation, cx) {
                    Ok(complete) => complete,
                    Err(error) => {
                        self.fail(error.as_bytes());
                        return semio_framework_job::StepOutcome::Yield;
                    }
                };
                if candidate_complete {
                    self.mutation_candidate.as_mut().expect("Drawing completed mutation candidate remains retained").take().expect("Drawing mutation overlay terminal commit witness remains exact");
                    drop(self.mutation_candidate.take());
                    self.phase = DrawingStoreInitializationPhase::ApplyForward { position, edit, mutation: mutation + 1 };
                }
                semio_framework_job::StepOutcome::Yield
            }
            DrawingStoreInitializationPhase::HashInverse { position, edit, mutation } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Drawing applied edit remains retained");
                let Some(operation) = entry.inverse.get(mutation) else {
                    self.phase = DrawingStoreInitializationPhase::PrepareApplied { position, edit, field: 0 };
                    return semio_framework_job::StepOutcome::Yield;
                };
                if self.mutation_digest.is_none() {
                    *self.mutation_digest = Some(DrawingMutationDigestAuthority::new());
                    cx.consume_fuel(1);
                    return semio_framework_job::StepOutcome::Yield;
                }
                match self.mutation_digest.as_mut().expect("Drawing inverse digest remains retained").step(operation, self.edit_digest.as_mut().expect("Drawing edit digest remains retained"), cx) {
                    Ok(true) => {
                        drop(self.mutation_digest.take());
                        self.phase = DrawingStoreInitializationPhase::HashInverse { position, edit, mutation: mutation + 1 };
                    }
                    Ok(false) => {}
                    Err(error) => self.fail(error.as_bytes()),
                }
                semio_framework_job::StepOutcome::Yield
            }
            DrawingStoreInitializationPhase::PrepareApplied { position, edit, field } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Drawing applied edit remains retained");
                match field {
                    0 => {
                        *self.prepared_history_id = Some(DrawingSnapshotCloneAuthority::clone_string(&entry.id).expect("validated Drawing applied id remains admitted"));
                        self.phase = DrawingStoreInitializationPhase::PrepareApplied { position, edit, field: 1 };
                        cx.consume_fuel(entry.id.len().max(1) as u64);
                    }
                    1 => {
                        if let Some(actor) = entry.actor.as_deref() {
                            *self.prepared_actor = Some(DrawingSnapshotCloneAuthority::clone_string(actor).expect("validated Drawing actor remains admitted"));
                            cx.consume_fuel(actor.len().max(1) as u64);
                        } else {
                            cx.consume_fuel(1);
                        }
                        self.phase = DrawingStoreInitializationPhase::CommitApplied { position, edit };
                    }
                    _ => self.fail(b"drawing-store.initializer-applied-preparation"),
                }
                semio_framework_job::StepOutcome::Yield
            }
            DrawingStoreInitializationPhase::CommitApplied { position, edit } => {
                let id = self.prepared_history_id.take().expect("Drawing applied id was retained in its own preparation grant");
                let actor = self.prepared_actor.take();
                let digest = self.edit_digest.take().expect("Drawing applied edit digest remains retained").finish();
                let runtime = self.runtime.as_mut().expect("Drawing runtime remains retained");
                if let Err(error) = runtime.push_applied(id, digest) {
                    self.fault = Some(error.into_bytes());
                    self.phase = DrawingStoreInitializationPhase::RetireFault;
                } else {
                    runtime.set_local_actor_id(actor);
                    self.phase = DrawingStoreInitializationPhase::FindApplied { position: position + 1, scan: 0 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            DrawingStoreInitializationPhase::FindRedo { position, scan } => {
                let Some(id) = self.redo_id(position) else {
                    self.phase = DrawingStoreInitializationPhase::BuildCandidate;
                    return semio_framework_job::StepOutcome::Yield;
                };
                let envelope = self.envelope.as_ref().expect("Drawing envelope remains retained");
                let Some(edit) = envelope.vcs.edits.get(scan) else {
                    self.fail(b"drawing-store.initializer-redo-edit-missing");
                    return semio_framework_job::StepOutcome::Yield;
                };
                if edit.id == id {
                    let mut digest = store::ArtifactStoreInitializationDigest::new(b"drawing.edit");
                    digest.observe(edit.id.as_bytes());
                    digest.observe(&edit.sequence_number.to_be_bytes());
                    digest.observe(edit.started_at.as_bytes());
                    *self.edit_digest = Some(digest);
                    self.phase = DrawingStoreInitializationPhase::HashRedoForward { position, edit: scan, mutation: 0 };
                } else {
                    self.phase = DrawingStoreInitializationPhase::FindRedo { position, scan: scan + 1 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            DrawingStoreInitializationPhase::HashRedoForward { position, edit, mutation } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Drawing redo edit remains retained");
                let Some(operation) = entry.forwards.get(mutation) else {
                    self.phase = DrawingStoreInitializationPhase::HashRedoInverse { position, edit, mutation: 0 };
                    return semio_framework_job::StepOutcome::Yield;
                };
                if self.mutation_digest.is_none() {
                    *self.mutation_digest = Some(DrawingMutationDigestAuthority::new());
                    cx.consume_fuel(1);
                    return semio_framework_job::StepOutcome::Yield;
                }
                match self.mutation_digest.as_mut().expect("Drawing redo forward digest remains retained").step(operation, self.edit_digest.as_mut().expect("Drawing redo digest remains retained"), cx) {
                    Ok(true) => {
                        drop(self.mutation_digest.take());
                        self.phase = DrawingStoreInitializationPhase::HashRedoForward { position, edit, mutation: mutation + 1 };
                    }
                    Ok(false) => {}
                    Err(error) => self.fail(error.as_bytes()),
                }
                semio_framework_job::StepOutcome::Yield
            }
            DrawingStoreInitializationPhase::HashRedoInverse { position, edit, mutation } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Drawing redo edit remains retained");
                let Some(operation) = entry.inverse.get(mutation) else {
                    self.phase = DrawingStoreInitializationPhase::PrepareRedo { position, edit };
                    return semio_framework_job::StepOutcome::Yield;
                };
                if self.mutation_digest.is_none() {
                    *self.mutation_digest = Some(DrawingMutationDigestAuthority::new());
                    cx.consume_fuel(1);
                    return semio_framework_job::StepOutcome::Yield;
                }
                match self.mutation_digest.as_mut().expect("Drawing redo inverse digest remains retained").step(operation, self.edit_digest.as_mut().expect("Drawing redo digest remains retained"), cx) {
                    Ok(true) => {
                        drop(self.mutation_digest.take());
                        self.phase = DrawingStoreInitializationPhase::HashRedoInverse { position, edit, mutation: mutation + 1 };
                    }
                    Ok(false) => {}
                    Err(error) => self.fail(error.as_bytes()),
                }
                semio_framework_job::StepOutcome::Yield
            }
            DrawingStoreInitializationPhase::PrepareRedo { position, edit } => {
                let id = &self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Drawing redo edit remains retained").id;
                *self.prepared_history_id = Some(DrawingSnapshotCloneAuthority::clone_string(id).expect("validated Drawing redo id remains admitted"));
                self.phase = DrawingStoreInitializationPhase::CommitRedo { position, edit };
                cx.consume_fuel(id.len().max(1) as u64);
                semio_framework_job::StepOutcome::Yield
            }
            DrawingStoreInitializationPhase::CommitRedo { position, edit } => {
                let id = self.prepared_history_id.take().expect("Drawing redo id was retained in its own preparation grant");
                let digest = self.edit_digest.take().expect("Drawing redo digest remains retained").finish();
                if let Err(error) = self.runtime.as_mut().expect("Drawing runtime remains retained").push_redo(id, digest) {
                    self.fault = Some(error.into_bytes());
                    self.phase = DrawingStoreInitializationPhase::RetireFault;
                } else {
                    self.phase = DrawingStoreInitializationPhase::FindRedo { position: position + 1, scan: 0 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            DrawingStoreInitializationPhase::BuildCandidate => {
                let Some(candidate_generation) = self.generation.0.checked_add(1) else {
                    self.fail(b"drawing-store.initializer-generation-exhausted");
                    return semio_framework_job::StepOutcome::Yield;
                };
                let envelope = self.envelope.take().expect("Drawing envelope remains retained until atomic store construction");
                let runtime = self.runtime.take().expect("Drawing runtime remains retained until atomic store construction");
                let candidate = store::ArtifactStore::from_initialized_runtime_with_owners(envelope, runtime, candidate_generation, drawing_document_store_owners());
                *self.candidate = Some(candidate);
                self.phase = DrawingStoreInitializationPhase::Complete;
                semio_framework_job::StepOutcome::Complete(semio_framework_job::CommitCandidate {
                    state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
                    output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
                })
            }
            DrawingStoreInitializationPhase::RetireCancelled | DrawingStoreInitializationPhase::RetireFault => match self.pump_terminal_retirement() {
                Ok(false) => semio_framework_job::StepOutcome::Yield,
                Ok(true) => {
                    drop(self.initial_digest.take());
                    drop(self.edit_digest.take());
                    self.terminal_handoff = true;
                    if self.phase == DrawingStoreInitializationPhase::RetireCancelled {
                        self.phase = DrawingStoreInitializationPhase::Cancelled;
                        semio_framework_job::StepOutcome::Cancelled
                    } else {
                        self.phase = DrawingStoreInitializationPhase::Fault;
                        let source = self.fault.take().unwrap_or_else(|| b"drawing-store.initializer-fault".to_vec());
                        let detail = cx
                            .payload_from_bytes(semio_framework_job::JobPayloadStream::Fault, &source)
                            .unwrap_or_else(|_| semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault));
                        semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail })
                    }
                }
                Err(error) => {
                    self.fault = Some(error.into_bytes());
                    semio_framework_job::StepOutcome::Yield
                }
            },
            DrawingStoreInitializationPhase::Complete => semio_framework_job::StepOutcome::Complete(semio_framework_job::CommitCandidate {
                state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
                output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
            }),
            DrawingStoreInitializationPhase::Cancelled => semio_framework_job::StepOutcome::Cancelled,
            DrawingStoreInitializationPhase::Fault => {
                let source = self.fault.as_deref().unwrap_or(b"drawing-store.initializer-fault");
                let detail = cx
                    .payload_from_bytes(semio_framework_job::JobPayloadStream::Fault, source)
                    .unwrap_or_else(|_| semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault));
                semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail })
            }
        }
    }

    fn request_cancel(&mut self) {
        self.cancel_requested = true;
    }

    fn begin_close(&mut self) {
        self.cancel_requested = true;
        if !matches!(self.phase, DrawingStoreInitializationPhase::Cancelled | DrawingStoreInitializationPhase::Fault) {
            self.phase = DrawingStoreInitializationPhase::RetireCancelled;
        }
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<semio_framework_plugin::PluginCloseStep, semio_framework::Fault> {
        self.begin_close();
        if maximum_items == 0 || maximum_bytes < DRAWING_OWNED_FIELD_BYTES {
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        match self.pump_terminal_retirement() {
            Ok(false) => Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }),
            Ok(true) => {
                drop(self.initial_digest.take());
                drop(self.edit_digest.take());
                drop(self.mutation_digest.take());
                drop(self.mutation_candidate.take());
                self.terminal_handoff = true;
                Ok(semio_framework_plugin::PluginCloseStep::Complete)
            }
            Err(error) => Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("artifact-store.initializer-close"), format!("Drawing initializer close failed: {error}"))),
        }
    }

    fn take_candidate(&mut self) -> Option<store::ArtifactStore<DrawingSnapshot, DrawingMutation>> {
        if self.phase != DrawingStoreInitializationPhase::Complete || self.terminal_handoff {
            return None;
        }
        let candidate = self.candidate.take()?;
        drop(self.initial_digest.take());
        drop(self.edit_digest.take());
        drop(self.mutation_digest.take());
        drop(self.mutation_candidate.take());
        self.terminal_handoff = true;
        Some(candidate)
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal_is_empty_inner()
    }
}

impl Drop for DrawingStoreInitializationAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty_inner(), "Drawing store initialization authority reached Drop before exact candidate handoff or retained rejection close");
    }
}

pub fn drawing_document_store_initialization_job(
    envelope: store::ArtifactEnvelope<DrawingSnapshot, DrawingMutation>,
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
) -> semio_framework_plugin::ArtifactStoreInitializationJob<DrawingSnapshot, DrawingMutation> {
    let owner_catalog = store::ArtifactStoreInitializationOwnerCatalog::try_new();
    semio_framework_plugin::ArtifactStoreInitializationJob::new(Box::new(DrawingStoreInitializationAuthority::new(envelope, owner_catalog, operation, generation)))
}
//#endregion 🔖️RetainedStoreInitialization

//#region 🧪️RetainedMutationAuthorityTests
#[cfg(test)]
mod retained_mutation_authority_tests {
    use super::*;
    use crate::artifacts::drawing::mutations::{
        CreateLayer, DeleteLayer, DuplicateLayer, RenameLayer, ReorderLayer, ReplaceLayerFill, ReplaceLayerStroke, SetLayerBlendMode, SetLayerBooleanOperation, SetLayerLocked, SetLayerOpacity, SetLayerVisible, UpdateLayerTraceParams,
        UpdateLayerTransform,
    };

    fn admit_string_destination(value: &mut String) {
        if value.capacity() < DRAWING_OWNED_FIELD_BYTES {
            value.try_reserve_exact(DRAWING_OWNED_FIELD_BYTES.saturating_sub(value.len())).expect("Drawing fixture string destination is pre-admitted");
        }
        assert!(value.capacity() >= DRAWING_OWNED_FIELD_BYTES);
    }

    fn admit_layer_string_destinations(layer: &mut DrawingLayerNode) {
        let base = crate::artifacts::drawing::schema::layer_base_mut(layer);
        admit_string_destination(&mut base.id);
        admit_string_destination(&mut base.name);
        admit_string_destination(&mut base.blend_mode);
        match layer {
            DrawingLayerNode::Group(group) => {
                for child in &mut group.children {
                    admit_layer_string_destinations(child);
                }
            }
            DrawingLayerNode::Boolean(boolean) => admit_string_destination(&mut boolean.operation),
            _ => {}
        }
    }

    fn initialize_drawing_mutation_arena_pool_for_test() {
        let operation = semio_framework_job::OperationId(7_900);
        let generation = semio_framework_job::Generation(79);
        let mut job = DrawingMutationArenaBootstrapJob::new(operation, generation).expect("fixed Drawing arena bootstrap job admission");
        let cancel = semio_framework_job::root_cancel_token();
        let mut preview_sequence = 0;
        for _ in 0..1_000 {
            let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
            match job.step(&mut context) {
                DrawingMutationArenaBootstrapStep::Ready => return,
                DrawingMutationArenaBootstrapStep::Pending { advanced_items } => assert_eq!(advanced_items, 1),
                DrawingMutationArenaBootstrapStep::Blocked => {}
                DrawingMutationArenaBootstrapStep::Cancelled => panic!("Drawing mutation arena bootstrap fixture was unexpectedly cancelled"),
                DrawingMutationArenaBootstrapStep::Fault(error) => panic!("Drawing mutation arena pool initialization faulted: {error}"),
            }
        }
        panic!("Drawing mutation arena pool initialization did not terminate")
    }

    fn nested_snapshot() -> DrawingSnapshot {
        initialize_drawing_mutation_arena_pool_for_test();
        let mut snapshot = crate::artifacts::drawing::schema::default_drawing_document("drawing-retained-mutation", None);
        let shape = crate::artifacts::drawing::schema::create_drawing_shape_layer_rect("Shape");
        let boolean = crate::artifacts::drawing::schema::create_drawing_boolean_layer("Boolean", "union", vec![crate::artifacts::drawing::schema::layer_id(&shape).into()]);
        let trace = crate::artifacts::drawing::schema::create_drawing_trace_layer("Trace", "asset-a");
        let mut group = crate::artifacts::drawing::schema::create_drawing_group_layer("Group");
        if let DrawingLayerNode::Group(value) = &mut group {
            value.children.push(shape);
            value.children.push(boolean);
            value.children.push(trace);
        }
        admit_layer_string_destinations(&mut group);
        snapshot.layers.push(group);
        snapshot.assets.insert("asset-a".into(), DrawingImageAsset { mime: "image/png".into(), data: "AA==".into(), width: Some(1), height: Some(1) });
        snapshot
    }

    fn drain_snapshot(value: DrawingSnapshot) {
        let mut retirement = store::ArtifactOwnedValueRetirementFactory::retire_owned(&DrawingSnapshotRetirementFactory, value);
        for _ in 0..100_000 {
            match retirement.close_step(1, DRAWING_OWNED_FIELD_BYTES).expect("Drawing snapshot retirement") {
                store::SnapshotRetirementStep::Complete => {
                    assert!(retirement.terminal_is_empty());
                    drop(retirement);
                    return;
                }
                store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                    assert!(released_items <= 1);
                    assert!(released_bytes <= DRAWING_OWNED_FIELD_BYTES);
                }
                store::SnapshotRetirementStep::Blocked => panic!("owned Drawing snapshot retirement cannot block"),
            }
        }
        panic!("Drawing snapshot retirement did not terminate")
    }

    fn drain_mutation(value: DrawingMutation) {
        let mut retirement = store::ArtifactOwnedValueRetirementFactory::retire_owned(&DrawingMutationRetirementFactory, value);
        for _ in 0..100_000 {
            match retirement.close_step(1, DRAWING_OWNED_FIELD_BYTES).expect("Drawing mutation retirement") {
                store::SnapshotRetirementStep::Complete => {
                    assert!(retirement.terminal_is_empty());
                    drop(retirement);
                    return;
                }
                store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                    assert!(released_items <= 1);
                    assert!(released_bytes <= DRAWING_OWNED_FIELD_BYTES);
                }
                store::SnapshotRetirementStep::Blocked => panic!("owned Drawing mutation retirement cannot block"),
            }
        }
        panic!("Drawing mutation retirement did not terminate")
    }

    fn close_candidate(authority: &mut DrawingMutationCandidateAuthority, mut source: Option<&mut DrawingSnapshot>) {
        for _ in 0..100_000 {
            match authority.close_step(source.as_deref_mut(), DRAWING_OWNED_FIELD_BYTES).expect("Drawing candidate close") {
                store::SnapshotRetirementStep::Complete => {
                    assert!(authority.terminal_is_empty());
                    return;
                }
                store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                    assert!(released_items <= 1);
                    assert!(released_bytes <= DRAWING_OWNED_FIELD_BYTES);
                }
                store::SnapshotRetirementStep::Blocked => panic!("owned Drawing candidate close cannot block"),
            }
        }
        panic!("Drawing candidate close did not terminate")
    }

    fn apply(mut source: DrawingSnapshot, mutation: &DrawingMutation) -> Result<DrawingSnapshot, (DrawingSnapshot, &'static str)> {
        initialize_drawing_mutation_arena_pool_for_test();
        let operation = semio_framework_job::OperationId(8_001);
        let generation = semio_framework_job::Generation(81);
        let mut authority = DrawingMutationCandidateAuthority::try_new(operation, generation).expect("Drawing candidate fixed owner arenas admit");
        let cancel = semio_framework_job::root_cancel_token();
        let mut preview_sequence = 0;
        for _ in 0..200_000 {
            let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
            match authority.step(&mut source, mutation, &mut context) {
                Ok(true) => {
                    authority.take().expect("Drawing mutation overlay exact terminal commit witness");
                    assert!(authority.terminal_is_empty());
                    drop(authority);
                    return Ok(source);
                }
                Ok(false) => {}
                Err(error) => {
                    close_candidate(&mut authority, Some(&mut source));
                    drop(authority);
                    return Err((source, error));
                }
            }
        }
        close_candidate(&mut authority, Some(&mut source));
        drop(authority);
        drain_snapshot(source);
        panic!("Drawing mutation candidate did not terminate")
    }

    fn live_reservation(source: &mut DrawingSnapshot, mutation: &DrawingMutation) -> Result<DrawingMutationAggregateReservation, &'static str> {
        initialize_drawing_mutation_arena_pool_for_test();
        let operation = semio_framework_job::OperationId(8_004);
        let generation = semio_framework_job::Generation(84);
        let cancel = semio_framework_job::root_cancel_token();
        let mut preview_sequence = 0;
        let mut authority = DrawingMutationCandidateAuthority::try_new(operation, generation)?;
        for _ in 0..100_000 {
            let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
            authority.step(source, mutation, &mut context)?;
            if let Some(reservation) = authority.reservation {
                close_candidate(&mut authority, Some(source));
                drop(authority);
                return Ok(reservation);
            }
        }
        close_candidate(&mut authority, Some(source));
        drop(authority);
        Err("drawing-store.test-mutation-preflight-incomplete")
    }

    fn digest(mutation: &DrawingMutation) -> Result<[u8; 32], &'static str> {
        let mut authority = DrawingMutationDigestAuthority::new();
        let mut output = store::ArtifactStoreInitializationDigest::new(b"drawing.test.mutation");
        let cancel = semio_framework_job::root_cancel_token();
        let mut preview_sequence = 0;
        for _ in 0..100_000 {
            let mut context = semio_framework_job::StepContext::new(
                semio_framework_job::OperationId(8_002),
                semio_framework_job::Generation(82),
                semio_framework_job::StepBudget::new(1, u64::MAX),
                cancel.clone(),
                semio_framework_job::default_now_us,
                &mut preview_sequence,
            );
            match authority.step(mutation, &mut output, &mut context) {
                Ok(true) => {
                    assert!(authority.terminal_is_empty());
                    drop(authority);
                    return Ok(output.finish());
                }
                Ok(false) => {}
                Err(error) => {
                    while !authority.terminal_is_empty() {
                        authority.close_step(DRAWING_OWNED_FIELD_BYTES).expect("Drawing digest rejection closes exactly");
                    }
                    drop(authority);
                    return Err(error);
                }
            }
        }
        panic!("Drawing mutation digest did not terminate")
    }

    fn rich_layer() -> DrawingLayerNode {
        let mut group = crate::artifacts::drawing::schema::create_drawing_group_layer("Digest Group");
        let base = crate::artifacts::drawing::schema::layer_base_mut(&mut group);
        base.visible = false;
        base.locked = true;
        base.opacity = 0.75;
        base.blend_mode = "multiply".into();
        base.transform = crate::artifacts::drawing::DrawingTransform { x: 1.0, y: 2.0, scale_x: 3.0, scale_y: 4.0, rotation: 0.5 };
        base.attributes.fill = Some(FillStyle::RadialGradient { cx: 1.0, cy: 2.0, r: 3.0, stops: vec![GradientStop { offset: 0.25, color: [0.1, 0.2, 0.3, 0.4] }] });
        base.attributes.stroke = Some(StrokeStyle { color: [0.5, 0.6, 0.7, 0.8], width: 2.0, cap: "round".into(), join: "bevel".into(), dash: Some(vec![1.0, 2.0]) });
        if let DrawingLayerNode::Group(value) = &mut group {
            value.children.push(crate::artifacts::drawing::schema::create_drawing_shape_layer_rect("Shape"));
            value.children.push(crate::artifacts::drawing::schema::create_drawing_path_layer(
                "Path",
                vec![
                    PathSegment::Move { to: [1.0, 2.0] },
                    PathSegment::Line { to: [3.0, 4.0] },
                    PathSegment::Quad { ctrl: [5.0, 6.0], to: [7.0, 8.0] },
                    PathSegment::Cubic { ctrl1: [9.0, 10.0], ctrl2: [11.0, 12.0], to: [13.0, 14.0] },
                    PathSegment::Arc { rx: 15.0, ry: 16.0, rotation: 17.0, large_arc: true, sweep: false, to: [18.0, 19.0] },
                    PathSegment::Close,
                ],
            ));
            value.children.push(crate::artifacts::drawing::schema::create_drawing_text_layer("Text"));
            value.children.push(crate::artifacts::drawing::schema::create_drawing_image_layer("Image", "asset-reference"));
            value.children.push(crate::artifacts::drawing::schema::create_drawing_boolean_layer("Boolean", "union", vec!["a".into(), "b".into()]));
            value.children.push(crate::artifacts::drawing::schema::create_drawing_trace_layer("Trace", "trace-source"));
        }
        group
    }

    fn create_digest(layer: DrawingLayerNode) -> [u8; 32] {
        let mutation = DrawingMutation::CreateLayer(CreateLayer { parent_id: Some("parent".into()), index: Some(3), layer: Box::new(layer) });
        let output = digest(&mutation).expect("rich Drawing create mutation hashes");
        drain_mutation(mutation);
        output
    }

    fn assert_mutation_digest_distinct(left: DrawingMutation, right: DrawingMutation) {
        let left_digest = digest(&left).expect("left Drawing mutation hashes");
        let right_digest = digest(&right).expect("right Drawing mutation hashes");
        assert_ne!(left_digest, right_digest, "changing one Drawing mutation semantic field changes the retained SHA-256 authority");
        drain_mutation(left);
        drain_mutation(right);
    }

    fn rich_child(layer: &mut DrawingLayerNode, index: usize) -> &mut DrawingLayerNode {
        let DrawingLayerNode::Group(group) = layer else { panic!("rich Drawing fixture root remains a Group") };
        group.children.get_mut(index).expect("rich Drawing fixture child")
    }

    #[test]
    fn retained_drawing_mutation_candidate_covers_all_fourteen_variants_and_returns_exact_owners() {
        let source = nested_snapshot();
        let group = crate::artifacts::drawing::schema::layer_id(source.layers.last().expect("group")).to_string();
        let (shape, boolean, trace) = match source.layers.last().expect("group") {
            DrawingLayerNode::Group(value) => {
                (crate::artifacts::drawing::schema::layer_id(&value.children[0]).to_string(), crate::artifacts::drawing::schema::layer_id(&value.children[1]).to_string(), crate::artifacts::drawing::schema::layer_id(&value.children[2]).to_string())
            }
            _ => unreachable!("Drawing fixture group remains exact"),
        };
        let mutations = vec![
            DrawingMutation::SetLayerVisible(SetLayerVisible { layer_id: shape.clone(), visible: false }),
            DrawingMutation::SetLayerLocked(SetLayerLocked { layer_id: shape.clone(), locked: true }),
            DrawingMutation::SetLayerOpacity(SetLayerOpacity { layer_id: shape.clone(), opacity: 0.5 }),
            DrawingMutation::SetLayerBlendMode(SetLayerBlendMode { layer_id: shape.clone(), blend_mode: "multiply".into() }),
            DrawingMutation::RenameLayer(RenameLayer { layer_id: shape.clone(), new_name: "Renamed".into() }),
            DrawingMutation::UpdateLayerTransform(UpdateLayerTransform { layer_id: shape.clone(), transform: crate::artifacts::drawing::DrawingTransform { x: 1.0, y: 2.0, scale_x: 3.0, scale_y: 4.0, rotation: 0.5 } }),
            DrawingMutation::ReplaceLayerFill(ReplaceLayerFill {
                layer_id: shape.clone(),
                fill: Some(FillStyle::LinearGradient { x1: 0.0, y1: 0.0, x2: 1.0, y2: 1.0, stops: vec![GradientStop { offset: 0.0, color: [1.0, 0.0, 0.0, 1.0] }, GradientStop { offset: 1.0, color: [0.0, 0.0, 1.0, 1.0] }] }),
            }),
            DrawingMutation::ReplaceLayerStroke(ReplaceLayerStroke { layer_id: shape.clone(), stroke: Some(StrokeStyle { color: [0.0, 0.0, 0.0, 1.0], width: 2.0, cap: "round".into(), join: "bevel".into(), dash: Some(vec![1.0, 2.0]) }) }),
            DrawingMutation::SetLayerBooleanOperation(SetLayerBooleanOperation { layer_id: boolean, boolean_operation: "subtract".into() }),
            DrawingMutation::UpdateLayerTraceParams(UpdateLayerTraceParams { layer_id: trace, params: crate::artifacts::drawing::DrawingTraceParams { threshold: 0.4, simplify_epsilon: 1.2 } }),
            DrawingMutation::CreateLayer(CreateLayer { parent_id: Some(group.clone()), index: Some(1), layer: Box::new(crate::artifacts::drawing::schema::create_drawing_path_layer("Created", vec![PathSegment::Move { to: [0.0, 0.0] }])) }),
            DrawingMutation::DuplicateLayer(DuplicateLayer { layer_id: shape.clone() }),
            DrawingMutation::DeleteLayer(DeleteLayer { layer_id: shape.clone() }),
            DrawingMutation::ReorderLayer(ReorderLayer { layer_id: shape, parent_id: None, index: 0 }),
        ];
        for mutation in mutations {
            let value = apply(nested_snapshot(), &mutation).expect("retained Drawing mutation applies");
            drain_snapshot(value);
        }
        drain_snapshot(source);
    }

    #[test]
    fn retained_drawing_process_arena_pool_cap_plus_one_returns_exact_slots_and_rejects_stale_aba() {
        let pool = DrawingMutationArenaPool::try_new().expect("isolated Drawing process arena pool claims exact bytes and items before operation admission");
        let mut first = Vec::new();
        for index in 0..DRAWING_MUTATION_ARENA_POOL_CAPACITY {
            let candidate = DrawingMutationCandidateAuthority::try_new_from_pool(semio_framework_job::OperationId(8_100 + index as u64), semio_framework_job::Generation(100 + index as u64), pool.clone())
                .expect("each fixed Drawing process arena slot admits exactly once");
            first.push((
                candidate.arena_slot,
                candidate.arena_generation,
                candidate.container_reverse.as_ref().expect("reverse arena owner").as_ptr(),
                candidate.container_output.as_ref().expect("output arena owner").as_ptr(),
                candidate.overlay_pages.as_ref().expect("page arena owner")[0].as_ptr(),
                candidate.duplicate_id_owner.as_ref().expect("duplicate id owner").as_ptr(),
                candidate,
            ));
        }
        match DrawingMutationCandidateAuthority::try_new_from_pool(semio_framework_job::OperationId(8_999), semio_framework_job::Generation(999), pool.clone()) {
            Err(error) => assert_eq!(error, "drawing-store.mutation-arena-pool-saturated"),
            Ok(_) => panic!("fixed Drawing arena pool must reject capacity +1"),
        }
        for entry in &mut first {
            for phase in 0..4 {
                assert_eq!(entry.6.return_arena_owner().expect("one fixed Drawing root returns per opportunity"), Some(phase == 3));
                let state = pool.state.try_lock().expect("isolated Drawing pool is uncontended");
                let slot = &state.slots[entry.0];
                let returned = usize::from(slot.reverse.is_some()) + usize::from(slot.output.is_some()) + usize::from(slot.pages.is_some()) + usize::from(slot.duplicate_id.is_some());
                assert_eq!(returned, phase + 1, "exactly one fixed arena root returns per grant");
                assert_eq!(slot.leased, phase < 3, "slot becomes available only after the fourth exact owner returns");
            }
            close_candidate(&mut entry.6, None);
        }
        let first_witnesses: Vec<_> = first.iter().map(|entry| (entry.0, entry.1, entry.2, entry.3, entry.4, entry.5)).collect();
        for entry in first {
            drop(entry.6);
        }

        let mut second = Vec::new();
        for index in 0..DRAWING_MUTATION_ARENA_POOL_CAPACITY {
            let candidate = DrawingMutationCandidateAuthority::try_new_from_pool(semio_framework_job::OperationId(8_200 + index as u64), semio_framework_job::Generation(200 + index as u64), pool.clone()).expect("returned Drawing arena slot re-admits");
            second.push(candidate);
        }
        for candidate in &second {
            let witness = first_witnesses.iter().find(|entry| entry.0 == candidate.arena_slot).expect("same fixed Drawing arena slot returns");
            assert!(candidate.arena_generation > witness.1, "slot generation advances to reject stale ABA returns");
            assert_eq!(candidate.container_reverse.as_ref().expect("reverse arena returned").as_ptr(), witness.2);
            assert_eq!(candidate.container_output.as_ref().expect("output arena returned").as_ptr(), witness.3);
            assert_eq!(candidate.overlay_pages.as_ref().expect("page arena returned")[0].as_ptr(), witness.4);
            assert_eq!(candidate.duplicate_id_owner.as_ref().expect("duplicate owner returned").as_ptr(), witness.5);
        }
        for candidate in &mut second {
            close_candidate(candidate, None);
        }
        for candidate in second {
            drop(candidate);
        }
    }

    fn step_arena_bootstrap(bootstrap: &mut DrawingMutationArenaPoolBootstrap) -> Result<bool, &'static str> {
        let cancel = semio_framework_job::root_cancel_token();
        let mut preview_sequence = 0;
        let mut context =
            semio_framework_job::StepContext::new(semio_framework_job::OperationId(7_902), semio_framework_job::Generation(79), semio_framework_job::StepBudget::new(1, u64::MAX), cancel, semio_framework_job::default_now_us, &mut preview_sequence);
        bootstrap.step(&mut context)
    }

    fn close_arena_bootstrap_step(bootstrap: &mut DrawingMutationArenaPoolBootstrap) -> store::SnapshotRetirementStep {
        let cancel = semio_framework_job::root_cancel_token();
        let mut preview_sequence = 0;
        let mut context =
            semio_framework_job::StepContext::new(semio_framework_job::OperationId(7_903), semio_framework_job::Generation(79), semio_framework_job::StepBudget::new(1, u64::MAX), cancel, semio_framework_job::default_now_us, &mut preview_sequence);
        bootstrap.close_step(&mut context)
    }

    fn close_arena_bootstrap(bootstrap: &mut DrawingMutationArenaPoolBootstrap) -> usize {
        let mut released_roots = 0;
        for _ in 0..10_000 {
            match close_arena_bootstrap_step(bootstrap) {
                store::SnapshotRetirementStep::Pending { released_items, .. } => {
                    assert!(released_items <= 1, "Drawing arena bootstrap releases at most one exact root per grant");
                    released_roots += released_items;
                }
                store::SnapshotRetirementStep::Complete => {
                    assert!(bootstrap.terminal_is_empty());
                    return released_roots;
                }
                store::SnapshotRetirementStep::Blocked => panic!("isolated Drawing arena bootstrap never blocks"),
            }
        }
        panic!("Drawing arena bootstrap retirement did not terminate")
    }

    fn build_arena_bootstrap_owners(bootstrap: &mut DrawingMutationArenaPoolBootstrap) {
        for _ in 0..10_000 {
            if bootstrap.owner == DRAWING_MUTATION_ARENA_POOL_CAPACITY {
                return;
            }
            assert_eq!(step_arena_bootstrap(bootstrap), Ok(false));
        }
        panic!("Drawing arena bootstrap did not construct its fixed owner catalog")
    }

    #[test]
    fn retained_drawing_arena_bootstrap_failure_at_each_allocation_retires_one_exact_root_per_grant() {
        let allocations = DRAWING_MUTATION_ARENA_POOL_CAPACITY * 20;
        for failure_at in 0..allocations {
            let mut bootstrap = DrawingMutationArenaPoolBootstrap::new(Some(failure_at), None, usize::MAX, usize::MAX);
            let fault = loop {
                match step_arena_bootstrap(&mut bootstrap) {
                    Ok(false) => {}
                    Ok(true) => panic!("injected Drawing arena allocation failure was not observed"),
                    Err(error) => break error,
                }
            };
            assert_eq!(fault, "drawing-store.mutation-arena-bootstrap-injected-allocation");
            assert_eq!(bootstrap.allocation, failure_at + 1);
            assert_eq!(close_arena_bootstrap(&mut bootstrap), failure_at, "every successfully constructed Vec/String/page root is handed to the retained fault cursor");
            drop(bootstrap);
        }
    }

    #[test]
    fn retained_drawing_arena_bootstrap_failure_after_each_bundle_keeps_every_root_until_terminal_close() {
        for owner in 0..DRAWING_MUTATION_ARENA_POOL_CAPACITY {
            let mut bootstrap = DrawingMutationArenaPoolBootstrap::new(None, Some(owner), usize::MAX, usize::MAX);
            let fault = loop {
                match step_arena_bootstrap(&mut bootstrap) {
                    Ok(false) => {}
                    Ok(true) => panic!("injected Drawing arena bundle failure was not observed"),
                    Err(error) => break error,
                }
            };
            assert_eq!(fault, "drawing-store.mutation-arena-bootstrap-injected-owner");
            assert_eq!(bootstrap.owner, owner + 1);
            assert_eq!(close_arena_bootstrap(&mut bootstrap), (owner + 1) * 20, "every completed bundle remains in the retained construction-fault owner");
            drop(bootstrap);
        }
    }

    #[test]
    fn retained_drawing_arena_bootstrap_advances_one_allocation_per_turn_and_withholds_incomplete_pool() {
        let mut bootstrap = DrawingMutationArenaPoolBootstrap::production(DrawingMutationArenaBootstrapAdmission::fixed().expect("fixed Drawing arena bootstrap claim"));
        let mut turns = 0;
        while !bootstrap.ready {
            let allocation = bootstrap.allocation;
            assert!(bootstrap.take_pool().is_none(), "an incomplete Drawing arena bootstrap cannot publish an operation-admission pool");
            assert!(matches!(step_arena_bootstrap(&mut bootstrap), Ok(false) | Ok(true)));
            assert!(bootstrap.allocation.saturating_sub(allocation) <= 1, "one governed bootstrap turn allocates at most one retained root");
            turns += 1;
            assert!(turns < 1_000);
        }
        let pool = bootstrap.take_pool().expect("terminal Drawing arena bootstrap publishes the fixed process pool");
        assert_eq!(pool.state.try_lock().expect("isolated Drawing arena pool is uncontended").slots.len(), DRAWING_MUTATION_ARENA_POOL_CAPACITY);
        drop(bootstrap);
        drop(pool);
    }

    #[test]
    fn retained_drawing_arena_bootstrap_exact_cap_and_plus_one_rejection_preserve_every_owner_until_close() {
        let mut exact = DrawingMutationArenaPoolBootstrap::new(None, None, usize::MAX, usize::MAX);
        build_arena_bootstrap_owners(&mut exact);
        let admitted_items = exact.admitted_items;
        let admitted_bytes = exact.admitted_bytes;
        exact.maximum_items = admitted_items;
        exact.maximum_bytes = admitted_bytes;
        assert_eq!(step_arena_bootstrap(&mut exact), Ok(true), "allocator-returned Drawing arena capacities admit at the exact boundary");
        assert_eq!(close_arena_bootstrap(&mut exact), DRAWING_MUTATION_ARENA_POOL_CAPACITY * 20);
        drop(exact);

        for (maximum_items, maximum_bytes) in [(admitted_items - 1, admitted_bytes), (admitted_items, admitted_bytes - 1)] {
            let mut rejected = DrawingMutationArenaPoolBootstrap::new(None, None, usize::MAX, usize::MAX);
            build_arena_bootstrap_owners(&mut rejected);
            rejected.maximum_items = maximum_items;
            rejected.maximum_bytes = maximum_bytes;
            assert_eq!(step_arena_bootstrap(&mut rejected), Err("drawing-store.mutation-arena-pool-capacity"));
            assert_eq!(close_arena_bootstrap(&mut rejected), DRAWING_MUTATION_ARENA_POOL_CAPACITY * 20, "aggregate +1 rejection retains all eighty exact roots until cursorized close");
            drop(rejected);
        }
    }

    #[test]
    fn retained_drawing_arena_default_second_app_and_borrow_only_request_without_allocation() {
        let state = DRAWING_MUTATION_ARENA_POOL.get_or_init(|| std::sync::Mutex::new(DrawingMutationArenaProcessState::Inert));
        let guard = state.try_lock().expect("isolated Drawing request fixture owns the inert process metadata");
        let witness = match &*guard {
            DrawingMutationArenaProcessState::Inert => (0, 0),
            DrawingMutationArenaProcessState::Building(bootstrap) => (1, bootstrap.allocation),
            DrawingMutationArenaProcessState::Ready(_) => (2, 0),
            DrawingMutationArenaProcessState::Retiring(bootstrap) => (3, bootstrap.allocation),
            DrawingMutationArenaProcessState::Fault(_) => (4, 0),
        };
        assert_eq!(request_drawing_mutation_arena_pool(), DrawingMutationArenaPoolAvailability::Contended);
        assert_eq!(request_drawing_mutation_arena_pool(), DrawingMutationArenaPoolAvailability::Contended, "a second app request coalesces fixed metadata without allocation");
        match borrow_drawing_mutation_arena() {
            Err(error) => assert_eq!(error, DrawingMutationArenaBorrowError::Contended),
            Ok(_) => panic!("borrow under process contention cannot expose an arena owner"),
        }
        let after = match &*guard {
            DrawingMutationArenaProcessState::Inert => (0, 0),
            DrawingMutationArenaProcessState::Building(bootstrap) => (1, bootstrap.allocation),
            DrawingMutationArenaProcessState::Ready(_) => (2, 0),
            DrawingMutationArenaProcessState::Retiring(bootstrap) => (3, bootstrap.allocation),
            DrawingMutationArenaProcessState::Fault(_) => (4, 0),
        };
        assert_eq!(after, witness, "default/request/borrow cannot advance a bootstrap allocation while no governed job owns the process turn");
        drop(guard);
    }

    #[test]
    fn retained_drawing_arena_bootstrap_job_cancel_budget_contention_and_saturation_are_governed() {
        let operation = semio_framework_job::OperationId(7_904);
        let generation = semio_framework_job::Generation(79);
        let mut job = DrawingMutationArenaBootstrapJob::new(operation, generation).expect("fixed Drawing bootstrap admission claim");
        let cancel = semio_framework_job::root_cancel_token();
        let mut preview_sequence = 0;
        let mut exhausted = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(0, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
        assert_eq!(job.step(&mut exhausted), DrawingMutationArenaBootstrapStep::Blocked, "zero-budget bootstrap cannot allocate or retire");

        let state = DRAWING_MUTATION_ARENA_POOL.get_or_init(|| std::sync::Mutex::new(DrawingMutationArenaProcessState::Inert));
        let guard = state.try_lock().expect("isolated Drawing bootstrap fixture owns process contention");
        let mut contended = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
        assert_eq!(job.step(&mut contended), DrawingMutationArenaBootstrapStep::Blocked, "process contention leaves the exact bootstrap owner untouched");
        drop(guard);

        let local_operation = semio_framework_job::OperationId(7_905);
        let local_generation = semio_framework_job::Generation(79);
        let mut local_job = DrawingMutationArenaBootstrapJob::new(local_operation, local_generation).expect("local Drawing bootstrap job claims its fixed admission");
        let local_cancel = semio_framework_job::root_cancel_token();
        let mut local_preview_sequence = 0;
        let mut local_state = DrawingMutationArenaProcessState::Inert;
        for _ in 0..3 {
            let mut admitted = semio_framework_job::StepContext::new(local_operation, local_generation, semio_framework_job::StepBudget::new(1, u64::MAX), local_cancel.clone(), semio_framework_job::default_now_us, &mut local_preview_sequence);
            assert_eq!(local_job.step_locked(&mut local_state, &mut admitted), DrawingMutationArenaBootstrapStep::Pending { advanced_items: 1 });
        }
        let allocated = match &local_state {
            DrawingMutationArenaProcessState::Building(bootstrap) => bootstrap.allocation,
            _ => panic!("three governed Drawing bootstrap turns retain one partially allocated bundle"),
        };
        assert_eq!(allocated, 1, "only admitted worker turns may advance allocation boundaries");
        local_cancel.cancel_now();
        for _ in 0..100 {
            let mut cancelled = semio_framework_job::StepContext::new(local_operation, local_generation, semio_framework_job::StepBudget::new(1, u64::MAX), local_cancel.clone(), semio_framework_job::default_now_us, &mut local_preview_sequence);
            match local_job.step_locked(&mut local_state, &mut cancelled) {
                DrawingMutationArenaBootstrapStep::Pending { advanced_items } => assert!(advanced_items <= 1),
                DrawingMutationArenaBootstrapStep::Cancelled => break,
                DrawingMutationArenaBootstrapStep::Blocked => {}
                DrawingMutationArenaBootstrapStep::Ready | DrawingMutationArenaBootstrapStep::Fault(_) => panic!("cancelled partial Drawing bootstrap must retire to exact Cancelled"),
            }
        }
        assert!(local_job.terminal);

        let pool = DrawingMutationArenaPool::try_new().expect("isolated fixed Drawing pool admits exact saturation fixture");
        let mut candidates = Vec::new();
        for index in 0..DRAWING_MUTATION_ARENA_POOL_CAPACITY {
            candidates.push(DrawingMutationCandidateAuthority::try_new_from_pool(semio_framework_job::OperationId(7_910 + index as u64), semio_framework_job::Generation(80 + index as u64), pool.clone()).expect("each fixed Drawing pool slot admits once"));
        }
        assert!(matches!(DrawingMutationCandidateAuthority::try_new_from_pool(semio_framework_job::OperationId(7_999), semio_framework_job::Generation(99), pool), Err("drawing-store.mutation-arena-pool-saturated")));
        for candidate in &mut candidates {
            close_candidate(candidate, None);
        }
        for candidate in candidates {
            drop(candidate);
        }
    }

    #[test]
    fn retained_drawing_depth_plus_one_and_hostile_fields_fault_then_close_terminal_empty() {
        let mut layer = crate::artifacts::drawing::schema::create_drawing_path_layer("leaf", Vec::new());
        for depth in 0..=DRAWING_MAXIMUM_LAYER_DEPTH {
            let mut parent = crate::artifacts::drawing::schema::create_drawing_group_layer(&format!("depth-{depth}"));
            if let DrawingLayerNode::Group(value) = &mut parent {
                value.children.push(layer);
            }
            layer = parent;
        }
        let mut source = crate::artifacts::drawing::schema::default_drawing_document("drawing-depth-plus-one", None);
        source.layers = vec![layer];
        let mutation = DrawingMutation::SetLayerVisible(SetLayerVisible { layer_id: "missing".into(), visible: false });
        let (source, error) = apply(source, &mutation).expect_err("retained Drawing depth +1 authority rejects");
        assert_eq!(error, "drawing-store.preflight-depth-capacity");
        drain_snapshot(source);

        let source = nested_snapshot();
        let mutation = DrawingMutation::RenameLayer(RenameLayer { layer_id: "x".repeat(DRAWING_OWNED_FIELD_BYTES + 1), new_name: "hostile".into() });
        let (source, _) = apply(source, &mutation).expect_err("hostile Drawing field rejects");
        drain_snapshot(source);
    }

    #[test]
    fn retained_drawing_container_false_terminal_saturation_and_interrupted_close_preserve_exact_owner() {
        let mut snapshot = crate::artifacts::drawing::schema::default_drawing_document("rebuild-reservation", None);
        snapshot.layers = vec![crate::artifacts::drawing::schema::create_drawing_path_layer("first", Vec::new()), crate::artifacts::drawing::schema::create_drawing_path_layer("second", Vec::new())];
        let mutation = DrawingMutation::CreateLayer(CreateLayer { parent_id: None, index: Some(1), layer: Box::new(crate::artifacts::drawing::schema::create_drawing_path_layer("pending", Vec::new())) });
        let reservation = live_reservation(&mut snapshot, &mutation).expect("live Drawing rebuild reservation admitted");
        let source = std::mem::take(&mut snapshot.layers);
        let DrawingMutation::CreateLayer(mut create) = mutation else { unreachable!() };
        let pending = *std::mem::replace(&mut create.layer, Box::new(crate::artifacts::drawing::schema::create_drawing_path_layer("retired-placeholder", Vec::new())));
        drain_mutation(DrawingMutation::CreateLayer(create));
        drain_snapshot(snapshot);
        let mut reverse = Vec::new();
        let mut output = Vec::new();
        reverse.try_reserve_exact(DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY).expect("fixed Drawing reverse arena");
        output.try_reserve_exact(DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY).expect("fixed Drawing output arena");
        let source_owner = source.as_ptr();
        let reverse_owner = reverse.as_ptr();
        let output_owner = output.as_ptr();
        let mut authority = DrawingContainerRebuildAuthority::new(source, Some(0), Some(1), Some(pending), reverse, output, reservation).expect("fixed Drawing rebuild admitted");
        assert!(authority.take().is_none(), "false terminal cannot expose a partially rebuilt owner");
        let cancel = semio_framework_job::root_cancel_token();
        let mut preview_sequence = 0;
        for _ in 0..3 {
            let mut context = semio_framework_job::StepContext::new(
                semio_framework_job::OperationId(8_005),
                semio_framework_job::Generation(85),
                semio_framework_job::StepBudget::new(1, u64::MAX),
                cancel.clone(),
                semio_framework_job::default_now_us,
                &mut preview_sequence,
            );
            assert!(!authority.step(&mut context).expect("Drawing rebuild advances before interruption"));
        }
        let mut rollback_turns = 0;
        while !authority.rollback_step().expect("Drawing rebuild rollback advances one exact owner") {
            rollback_turns += 1;
        }
        assert_eq!(rollback_turns, authority.move_count, "one recorded owner move rolls back per close grant");
        let restored = authority.source.take().expect("original Drawing source owner returns");
        let pending = authority.pending.take().expect("pending Drawing owner returns");
        let reverse = authority.reverse.take().expect("reverse Drawing scratch owner returns");
        let output = authority.output.take().expect("output Drawing scratch owner returns");
        assert_eq!(restored.as_ptr(), source_owner);
        assert_eq!(reverse.as_ptr(), reverse_owner);
        assert_eq!(output.as_ptr(), output_owner);
        assert!(authority.removed.is_none());
        authority.finish_handoff().expect("Drawing rebuild rollback reaches exact terminal handoff");
        assert!(authority.terminal_is_empty());
        drop(authority);
        drop(reverse);
        drop(output);
        drain_mutation(DrawingMutation::CreateLayer(CreateLayer { parent_id: None, index: None, layer: Box::new(pending) }));
        let mut restored_snapshot = crate::artifacts::drawing::schema::default_drawing_document("restored-rebuild", None);
        restored_snapshot.layers = restored;
        drain_snapshot(restored_snapshot);
    }

    #[test]
    fn retained_drawing_rebuild_fault_after_every_phase_rolls_back_exact_container_and_reuses_pool_slot() {
        for phase in 0..=3 {
            for stale in [false, true] {
                let pool = DrawingMutationArenaPool::try_new().expect("isolated Drawing rollback pool admits exact owners");
                let mut source = crate::artifacts::drawing::schema::default_drawing_document("rebuild-rollback", None);
                source.layers.try_reserve_exact(DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY).expect("Drawing rollback fixture pre-admits original live container backing");
                for index in 0..3 {
                    source.layers.push(crate::artifacts::drawing::schema::create_drawing_path_layer(&format!("source-{index}"), Vec::new()));
                }
                let source_owner = source.layers.as_ptr();
                let source_ids: Vec<_> = source.layers.iter().map(|layer| crate::artifacts::drawing::schema::layer_id(layer).to_string()).collect();
                let mutation = DrawingMutation::CreateLayer(CreateLayer { parent_id: None, index: Some(1), layer: Box::new(crate::artifacts::drawing::schema::create_drawing_path_layer("pending", Vec::new())) });
                let operation = semio_framework_job::OperationId(8_500 + phase as u64);
                let generation = semio_framework_job::Generation(850 + phase as u64);
                let mut authority = DrawingMutationCandidateAuthority::try_new_from_pool(operation, generation, pool.clone()).expect("Drawing rollback candidate borrows one exact pool slot");
                let slot = authority.arena_slot;
                let arena_generation = authority.arena_generation;
                let reverse_owner = authority.container_reverse.as_ref().expect("Drawing rollback reverse owner").as_ptr();
                let output_owner = authority.container_output.as_ref().expect("Drawing rollback output owner").as_ptr();
                let page_catalog_owner = authority.overlay_pages.as_ref().expect("Drawing rollback page catalog owner").as_ptr();
                let page_owners: [usize; DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY] = std::array::from_fn(|index| authority.overlay_pages.as_ref().expect("Drawing rollback page owner")[index].as_ptr() as usize);
                let duplicate_owner = authority.duplicate_id_owner.as_ref().expect("Drawing rollback duplicate owner").as_ptr();
                let cancel = semio_framework_job::root_cancel_token();
                let mut preview_sequence = 0;
                for _ in 0..100_000 {
                    if authority.rebuild.as_ref().is_some_and(|rebuild| rebuild.recorded_move_in_phase(phase)) {
                        break;
                    }
                    let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
                    assert!(!authority.step(&mut source, &mutation, &mut context).expect("Drawing rollback fixture reaches every internal rebuild phase"));
                }
                assert!(authority.rebuild.as_ref().is_some_and(|rebuild| rebuild.recorded_move_in_phase(phase)));
                if !stale {
                    cancel.cancel_now();
                }
                let mut rejected = semio_framework_job::StepContext::new(
                    operation,
                    if stale { semio_framework_job::Generation(generation.0 + 1) } else { generation },
                    semio_framework_job::StepBudget::new(1, u64::MAX),
                    cancel,
                    semio_framework_job::default_now_us,
                    &mut preview_sequence,
                );
                assert_eq!(authority.step(&mut source, &mutation, &mut rejected), Err(if stale { "drawing-store.mutation-candidate-stale-authority" } else { "drawing-store.mutation-candidate-cancelled" }));
                close_candidate(&mut authority, Some(&mut source));
                assert_eq!(source.layers.as_ptr(), source_owner, "rollback restores the exact original live Vec backing");
                assert_eq!(source.layers.iter().map(|layer| crate::artifacts::drawing::schema::layer_id(layer)).collect::<Vec<_>>(), source_ids.iter().map(String::as_str).collect::<Vec<_>>(), "rollback restores exact FIFO layer order");
                drop(authority);

                let mut reused =
                    DrawingMutationCandidateAuthority::try_new_from_pool(semio_framework_job::OperationId(operation.0 + 100), semio_framework_job::Generation(generation.0 + 100), pool.clone()).expect("rolled-back Drawing pool slot re-admits immediately");
                assert_eq!(reused.arena_slot, slot);
                assert!(reused.arena_generation > arena_generation);
                assert_eq!(reused.container_reverse.as_ref().expect("returned reverse owner").as_ptr(), reverse_owner);
                assert_eq!(reused.container_output.as_ref().expect("returned output owner").as_ptr(), output_owner);
                assert_eq!(reused.overlay_pages.as_ref().expect("returned page catalog owner").as_ptr(), page_catalog_owner);
                assert_eq!(std::array::from_fn::<_, DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY, _>(|index| reused.overlay_pages.as_ref().expect("returned page owner")[index].as_ptr() as usize), page_owners);
                assert_eq!(reused.duplicate_id_owner.as_ref().expect("returned duplicate owner").as_ptr(), duplicate_owner);
                close_candidate(&mut reused, None);
                drop(reused);
                drain_mutation(mutation);
                drain_snapshot(source);
            }
        }
    }

    #[test]
    fn retained_drawing_reorder_fault_after_source_handoff_restores_exact_nested_fifo_and_pool_roots() {
        for stale in [false, true] {
            let pool = DrawingMutationArenaPool::try_new().expect("isolated Drawing reorder rollback pool admits exact owners");
            let mut source = nested_snapshot();
            let (group_id, target, source_owner, source_ids) = match source.layers.last_mut().expect("Drawing reorder rollback group") {
                DrawingLayerNode::Group(group) => {
                    group.children.try_reserve_exact(DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY.saturating_sub(group.children.len())).expect("Drawing reorder rollback fixture pre-admits the nested live container");
                    (
                        group.base.id.clone(),
                        crate::artifacts::drawing::schema::layer_id(&group.children[0]).to_string(),
                        group.children.as_ptr(),
                        group.children.iter().map(|layer| crate::artifacts::drawing::schema::layer_id(layer).to_string()).collect::<Vec<_>>(),
                    )
                }
                _ => unreachable!("Drawing reorder rollback fixture remains a group"),
            };
            let mutation = DrawingMutation::ReorderLayer(ReorderLayer { layer_id: target, parent_id: Some(group_id), index: 2 });
            let operation = semio_framework_job::OperationId(8_700);
            let generation = semio_framework_job::Generation(870);
            let mut authority = DrawingMutationCandidateAuthority::try_new_from_pool(operation, generation, pool.clone()).expect("Drawing reorder rollback candidate borrows one exact pool slot");
            let slot = authority.arena_slot;
            let reverse_owner = authority.container_reverse.as_ref().expect("Drawing reorder reverse owner").as_ptr();
            let output_owner = authority.container_output.as_ref().expect("Drawing reorder output owner").as_ptr();
            let cancel = semio_framework_job::root_cancel_token();
            let mut preview_sequence = 0;
            for _ in 0..100_000 {
                if authority.rebuild_role == Some(DrawingContainerRebuildRole::Destination) && authority.rebuild.as_ref().is_some_and(|rebuild| rebuild.recorded_move_in_phase(2)) {
                    break;
                }
                let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
                assert!(!authority.step(&mut source, &mutation, &mut context).expect("Drawing reorder rollback fixture reaches destination rebuild after source handoff"));
            }
            assert_eq!(authority.rebuild_role, Some(DrawingContainerRebuildRole::Destination));
            assert!(authority.source_undo.is_some(), "source handoff keeps the exact insertion undo authority until destination publication");
            if !stale {
                cancel.cancel_now();
            }
            let mut rejected = semio_framework_job::StepContext::new(
                operation,
                if stale { semio_framework_job::Generation(generation.0 + 1) } else { generation },
                semio_framework_job::StepBudget::new(1, u64::MAX),
                cancel,
                semio_framework_job::default_now_us,
                &mut preview_sequence,
            );
            assert_eq!(authority.step(&mut source, &mutation, &mut rejected), Err(if stale { "drawing-store.mutation-candidate-stale-authority" } else { "drawing-store.mutation-candidate-cancelled" }));
            close_candidate(&mut authority, Some(&mut source));
            drop(authority);
            let DrawingLayerNode::Group(group) = source.layers.last().expect("Drawing reorder rollback group remains retained") else { unreachable!("Drawing reorder rollback group remains a group") };
            assert_eq!(group.children.as_ptr(), source_owner, "source undo restores the exact nested live Vec backing");
            assert_eq!(group.children.iter().map(|layer| crate::artifacts::drawing::schema::layer_id(layer)).collect::<Vec<_>>(), source_ids.iter().map(String::as_str).collect::<Vec<_>>(), "source undo restores the exact nested FIFO order");

            let mut reused = DrawingMutationCandidateAuthority::try_new_from_pool(semio_framework_job::OperationId(operation.0 + 1), semio_framework_job::Generation(generation.0 + 1), pool).expect("reorder rollback returns the exact pool slot");
            assert_eq!(reused.arena_slot, slot);
            assert_eq!(reused.container_reverse.as_ref().expect("returned reorder reverse owner").as_ptr(), reverse_owner);
            assert_eq!(reused.container_output.as_ref().expect("returned reorder output owner").as_ptr(), output_owner);
            close_candidate(&mut reused, None);
            drop(reused);
            drain_mutation(mutation);
            drain_snapshot(source);
        }
    }

    #[test]
    fn retained_drawing_schema_digest_distinguishes_every_nested_semantic_field() {
        let baseline = rich_layer();
        let baseline_digest = create_digest(baseline.clone());
        let mut variants = Vec::new();

        let modifiers: &[fn(&mut DrawingLayerNode)] = &[
            |value| crate::artifacts::drawing::schema::layer_base_mut(value).id = "different-id".into(),
            |value| crate::artifacts::drawing::schema::layer_base_mut(value).name = "different-name".into(),
            |value| crate::artifacts::drawing::schema::layer_base_mut(value).transform.x = 9.0,
            |value| crate::artifacts::drawing::schema::layer_base_mut(value).transform.y = 9.0,
            |value| crate::artifacts::drawing::schema::layer_base_mut(value).transform.scale_x = 9.0,
            |value| crate::artifacts::drawing::schema::layer_base_mut(value).transform.scale_y = 9.0,
            |value| crate::artifacts::drawing::schema::layer_base_mut(value).attributes.fill = None,
            |value| {
                if let Some(FillStyle::RadialGradient { cx, .. }) = &mut crate::artifacts::drawing::schema::layer_base_mut(value).attributes.fill {
                    *cx = 9.0;
                }
            },
            |value| {
                if let Some(FillStyle::RadialGradient { cy, .. }) = &mut crate::artifacts::drawing::schema::layer_base_mut(value).attributes.fill {
                    *cy = 9.0;
                }
            },
            |value| {
                if let Some(FillStyle::RadialGradient { r, .. }) = &mut crate::artifacts::drawing::schema::layer_base_mut(value).attributes.fill {
                    *r = 9.0;
                }
            },
            |value| {
                if let Some(FillStyle::RadialGradient { stops, .. }) = &mut crate::artifacts::drawing::schema::layer_base_mut(value).attributes.fill {
                    stops[0].offset = 0.75;
                }
            },
            |value| {
                if let Some(FillStyle::RadialGradient { stops, .. }) = &mut crate::artifacts::drawing::schema::layer_base_mut(value).attributes.fill {
                    stops[0].color[2] = 0.9;
                }
            },
            |value| crate::artifacts::drawing::schema::layer_base_mut(value).attributes.stroke = None,
            |value| crate::artifacts::drawing::schema::layer_base_mut(value).attributes.stroke.as_mut().expect("stroke").color[0] = 0.9,
            |value| crate::artifacts::drawing::schema::layer_base_mut(value).attributes.stroke.as_mut().expect("stroke").width = 9.0,
            |value| crate::artifacts::drawing::schema::layer_base_mut(value).attributes.stroke.as_mut().expect("stroke").cap = "square".into(),
            |value| crate::artifacts::drawing::schema::layer_base_mut(value).attributes.stroke.as_mut().expect("stroke").join = "round".into(),
            |value| crate::artifacts::drawing::schema::layer_base_mut(value).attributes.stroke.as_mut().expect("stroke").dash.as_mut().expect("dash")[0] = 9.0,
            |value| {
                if let DrawingLayerNode::Shape(shape) = rich_child(value, 0) {
                    shape.rect.as_mut().expect("rect").width = 9.0;
                }
            },
            |value| {
                if let DrawingLayerNode::Shape(shape) = rich_child(value, 0) {
                    shape.ellipse = Some(crate::artifacts::drawing::DrawingEllipse { cx: 1.0, cy: 2.0, rx: 3.0, ry: 4.0 });
                }
            },
            |value| {
                if let DrawingLayerNode::Shape(shape) = rich_child(value, 0) {
                    shape.circle = Some(crate::artifacts::drawing::DrawingCircle { cx: 1.0, cy: 2.0, r: 3.0 });
                }
            },
            |value| {
                if let DrawingLayerNode::Shape(shape) = rich_child(value, 0) {
                    shape.line = Some(crate::artifacts::drawing::DrawingLine { x1: 1.0, y1: 2.0, x2: 3.0, y2: 4.0 });
                }
            },
            |value| {
                if let DrawingLayerNode::Shape(shape) = rich_child(value, 0) {
                    shape.polygon = Some(crate::artifacts::drawing::DrawingPolygon { points: vec![[1.0, 2.0], [3.0, 4.0]] });
                }
            },
            |value| {
                if let DrawingLayerNode::Path(path) = rich_child(value, 1) {
                    path.segments[0] = PathSegment::Line { to: [1.0, 2.0] };
                }
            },
            |value| {
                if let DrawingLayerNode::Path(path) = rich_child(value, 1) {
                    path.segments[1] = PathSegment::Line { to: [9.0, 4.0] };
                }
            },
            |value| {
                if let DrawingLayerNode::Path(path) = rich_child(value, 1) {
                    path.segments[2] = PathSegment::Quad { ctrl: [9.0, 6.0], to: [7.0, 8.0] };
                }
            },
            |value| {
                if let DrawingLayerNode::Path(path) = rich_child(value, 1) {
                    path.segments[3] = PathSegment::Cubic { ctrl1: [9.0, 10.0], ctrl2: [11.0, 12.0], to: [13.0, 20.0] };
                }
            },
            |value| {
                if let DrawingLayerNode::Text(text) = rich_child(value, 2) {
                    text.x = 9.0;
                }
            },
            |value| {
                if let DrawingLayerNode::Text(text) = rich_child(value, 2) {
                    text.y = 9.0;
                }
            },
            |value| {
                if let DrawingLayerNode::Text(text) = rich_child(value, 2) {
                    text.content = "different text".into();
                }
            },
            |value| {
                if let DrawingLayerNode::Text(text) = rich_child(value, 2) {
                    text.size = 9.0;
                }
            },
            |value| {
                if let DrawingLayerNode::Image(image) = rich_child(value, 3) {
                    image.height = 9.0;
                }
            },
            |value| {
                if let DrawingLayerNode::Trace(trace) = rich_child(value, 5) {
                    trace.params.simplify_epsilon = 9.0;
                }
            },
            |value| {
                let DrawingLayerNode::Group(group) = value else { unreachable!() };
                group.children.swap(0, 1);
            },
        ];
        for modifier in modifiers {
            let mut value = baseline.clone();
            modifier(&mut value);
            variants.push(value);
        }

        let mut value = baseline.clone();
        crate::artifacts::drawing::schema::layer_base_mut(&mut value).visible = true;
        variants.push(value);
        let mut value = baseline.clone();
        crate::artifacts::drawing::schema::layer_base_mut(&mut value).locked = false;
        variants.push(value);
        let mut value = baseline.clone();
        crate::artifacts::drawing::schema::layer_base_mut(&mut value).opacity = 0.5;
        variants.push(value);
        let mut value = baseline.clone();
        crate::artifacts::drawing::schema::layer_base_mut(&mut value).blend_mode = "screen".into();
        variants.push(value);
        let mut value = baseline.clone();
        crate::artifacts::drawing::schema::layer_base_mut(&mut value).transform.rotation = 0.75;
        variants.push(value);
        let mut value = baseline.clone();
        crate::artifacts::drawing::schema::layer_base_mut(&mut value).attributes.fill = Some(FillStyle::Solid { color: [0.9, 0.2, 0.3, 0.4] });
        variants.push(value);
        let mut value = baseline.clone();
        crate::artifacts::drawing::schema::layer_base_mut(&mut value).attributes.fill = Some(FillStyle::LinearGradient { x1: 1.0, y1: 2.0, x2: 3.0, y2: 4.0, stops: vec![GradientStop { offset: 0.5, color: [0.1, 0.2, 0.8, 0.4] }] });
        variants.push(value);
        let mut value = baseline.clone();
        crate::artifacts::drawing::schema::layer_base_mut(&mut value).attributes.stroke = Some(StrokeStyle { color: [0.9, 0.6, 0.7, 0.8], width: 3.0, cap: "square".into(), join: "round".into(), dash: Some(vec![2.0, 3.0]) });
        variants.push(value);
        let mut value = baseline.clone();
        if let DrawingLayerNode::Group(group) = &mut value {
            if let DrawingLayerNode::Path(path) = &mut group.children[1] {
                path.segments[4] = PathSegment::Arc { rx: 15.0, ry: 16.0, rotation: 18.0, large_arc: false, sweep: true, to: [20.0, 19.0] };
            }
        }
        variants.push(value);
        let mut value = baseline.clone();
        if let DrawingLayerNode::Group(group) = &mut value {
            if let DrawingLayerNode::Image(image) = &mut group.children[3] {
                image.image_key = "other-asset-reference".into();
                image.width = 2.0;
            }
        }
        variants.push(value);
        let mut value = baseline.clone();
        if let DrawingLayerNode::Group(group) = &mut value {
            if let DrawingLayerNode::Boolean(boolean) = &mut group.children[4] {
                boolean.operation = "subtract".into();
                boolean.children.swap(0, 1);
            }
        }
        variants.push(value);
        let mut value = baseline.clone();
        if let DrawingLayerNode::Group(group) = &mut value {
            if let DrawingLayerNode::Trace(trace) = &mut group.children[5] {
                trace.source_key = "other-trace-source".into();
                trace.params.threshold = 0.25;
            }
        }
        variants.push(value);

        for variant in variants {
            assert_ne!(create_digest(variant), baseline_digest, "every Drawing layer scalar, style, geometry, order, and asset reference changes the SHA-256 semantic authority");
        }

        let id = "layer".to_string();
        let all_payloads = [
            DrawingMutation::SetLayerVisible(SetLayerVisible { layer_id: id.clone(), visible: false }),
            DrawingMutation::SetLayerLocked(SetLayerLocked { layer_id: id.clone(), locked: true }),
            DrawingMutation::SetLayerOpacity(SetLayerOpacity { layer_id: id.clone(), opacity: 0.25 }),
            DrawingMutation::SetLayerBlendMode(SetLayerBlendMode { layer_id: id.clone(), blend_mode: "screen".into() }),
            DrawingMutation::RenameLayer(RenameLayer { layer_id: id.clone(), new_name: "renamed".into() }),
            DrawingMutation::UpdateLayerTransform(UpdateLayerTransform { layer_id: id.clone(), transform: crate::artifacts::drawing::DrawingTransform { x: 1.0, y: 2.0, scale_x: 3.0, scale_y: 4.0, rotation: 5.0 } }),
            DrawingMutation::ReplaceLayerFill(ReplaceLayerFill { layer_id: id.clone(), fill: Some(FillStyle::Solid { color: [0.1, 0.2, 0.3, 0.4] }) }),
            DrawingMutation::ReplaceLayerStroke(ReplaceLayerStroke { layer_id: id.clone(), stroke: Some(StrokeStyle { color: [0.1, 0.2, 0.3, 0.4], width: 2.0, cap: "round".into(), join: "bevel".into(), dash: Some(vec![1.0]) }) }),
            DrawingMutation::SetLayerBooleanOperation(SetLayerBooleanOperation { layer_id: id.clone(), boolean_operation: "intersect".into() }),
            DrawingMutation::UpdateLayerTraceParams(UpdateLayerTraceParams { layer_id: id.clone(), params: crate::artifacts::drawing::DrawingTraceParams { threshold: 0.25, simplify_epsilon: 0.5 } }),
            DrawingMutation::CreateLayer(CreateLayer { parent_id: Some("parent".into()), index: Some(2), layer: Box::new(baseline.clone()) }),
            DrawingMutation::DuplicateLayer(DuplicateLayer { layer_id: id.clone() }),
            DrawingMutation::DeleteLayer(DeleteLayer { layer_id: id.clone() }),
            DrawingMutation::ReorderLayer(ReorderLayer { layer_id: id, parent_id: Some("parent".into()), index: 3 }),
        ];
        let mut digests = std::collections::HashSet::new();
        for payload in all_payloads {
            assert!(digests.insert(digest(&payload).expect("all fourteen Drawing mutation payloads hash distinctly")));
            drain_mutation(payload);
        }
        assert_mutation_digest_distinct(DrawingMutation::SetLayerVisible(SetLayerVisible { layer_id: "layer".into(), visible: false }), DrawingMutation::SetLayerVisible(SetLayerVisible { layer_id: "layer".into(), visible: true }));
        assert_mutation_digest_distinct(DrawingMutation::SetLayerLocked(SetLayerLocked { layer_id: "layer".into(), locked: false }), DrawingMutation::SetLayerLocked(SetLayerLocked { layer_id: "layer".into(), locked: true }));
        assert_mutation_digest_distinct(DrawingMutation::SetLayerOpacity(SetLayerOpacity { layer_id: "layer".into(), opacity: 0.25 }), DrawingMutation::SetLayerOpacity(SetLayerOpacity { layer_id: "layer".into(), opacity: 0.5 }));
        assert_mutation_digest_distinct(
            DrawingMutation::SetLayerBlendMode(SetLayerBlendMode { layer_id: "layer".into(), blend_mode: "multiply".into() }),
            DrawingMutation::SetLayerBlendMode(SetLayerBlendMode { layer_id: "layer".into(), blend_mode: "screen".into() }),
        );
        assert_mutation_digest_distinct(DrawingMutation::RenameLayer(RenameLayer { layer_id: "layer".into(), new_name: "left".into() }), DrawingMutation::RenameLayer(RenameLayer { layer_id: "layer".into(), new_name: "right".into() }));
        assert_mutation_digest_distinct(
            DrawingMutation::UpdateLayerTransform(UpdateLayerTransform { layer_id: "layer".into(), transform: crate::artifacts::drawing::DrawingTransform { x: 1.0, y: 2.0, scale_x: 3.0, scale_y: 4.0, rotation: 5.0 } }),
            DrawingMutation::UpdateLayerTransform(UpdateLayerTransform { layer_id: "layer".into(), transform: crate::artifacts::drawing::DrawingTransform { x: 6.0, y: 2.0, scale_x: 3.0, scale_y: 4.0, rotation: 5.0 } }),
        );
        assert_mutation_digest_distinct(
            DrawingMutation::ReplaceLayerFill(ReplaceLayerFill { layer_id: "layer".into(), fill: None }),
            DrawingMutation::ReplaceLayerFill(ReplaceLayerFill { layer_id: "layer".into(), fill: Some(FillStyle::Solid { color: [0.1, 0.2, 0.3, 0.4] }) }),
        );
        assert_mutation_digest_distinct(
            DrawingMutation::ReplaceLayerFill(ReplaceLayerFill { layer_id: "layer".into(), fill: Some(FillStyle::LinearGradient { x1: 0.0, y1: 1.0, x2: 2.0, y2: 3.0, stops: vec![GradientStop { offset: 0.5, color: [0.1, 0.2, 0.3, 0.4] }] }) }),
            DrawingMutation::ReplaceLayerFill(ReplaceLayerFill { layer_id: "layer".into(), fill: Some(FillStyle::LinearGradient { x1: 9.0, y1: 1.0, x2: 2.0, y2: 3.0, stops: vec![GradientStop { offset: 0.75, color: [0.1, 0.2, 0.8, 0.4] }] }) }),
        );
        assert_mutation_digest_distinct(
            DrawingMutation::ReplaceLayerStroke(ReplaceLayerStroke { layer_id: "layer".into(), stroke: None }),
            DrawingMutation::ReplaceLayerStroke(ReplaceLayerStroke { layer_id: "layer".into(), stroke: Some(StrokeStyle { color: [0.1, 0.2, 0.3, 0.4], width: 1.0, cap: "round".into(), join: "bevel".into(), dash: Some(vec![1.0]) }) }),
        );
        assert_mutation_digest_distinct(
            DrawingMutation::ReplaceLayerStroke(ReplaceLayerStroke { layer_id: "layer".into(), stroke: Some(StrokeStyle { color: [0.1, 0.2, 0.3, 0.4], width: 1.0, cap: "round".into(), join: "bevel".into(), dash: Some(vec![1.0]) }) }),
            DrawingMutation::ReplaceLayerStroke(ReplaceLayerStroke { layer_id: "layer".into(), stroke: Some(StrokeStyle { color: [0.9, 0.2, 0.3, 0.4], width: 2.0, cap: "square".into(), join: "round".into(), dash: Some(vec![2.0]) }) }),
        );
        assert_mutation_digest_distinct(
            DrawingMutation::SetLayerBooleanOperation(SetLayerBooleanOperation { layer_id: "layer".into(), boolean_operation: "union".into() }),
            DrawingMutation::SetLayerBooleanOperation(SetLayerBooleanOperation { layer_id: "layer".into(), boolean_operation: "subtract".into() }),
        );
        assert_mutation_digest_distinct(
            DrawingMutation::UpdateLayerTraceParams(UpdateLayerTraceParams { layer_id: "layer".into(), params: crate::artifacts::drawing::DrawingTraceParams { threshold: 0.25, simplify_epsilon: 0.5 } }),
            DrawingMutation::UpdateLayerTraceParams(UpdateLayerTraceParams { layer_id: "layer".into(), params: crate::artifacts::drawing::DrawingTraceParams { threshold: 0.75, simplify_epsilon: 1.5 } }),
        );
        assert_mutation_digest_distinct(
            DrawingMutation::CreateLayer(CreateLayer { parent_id: None, index: None, layer: Box::new(baseline.clone()) }),
            DrawingMutation::CreateLayer(CreateLayer { parent_id: Some("parent".into()), index: Some(1), layer: Box::new(baseline.clone()) }),
        );
        assert_mutation_digest_distinct(DrawingMutation::DuplicateLayer(DuplicateLayer { layer_id: "left".into() }), DrawingMutation::DuplicateLayer(DuplicateLayer { layer_id: "right".into() }));
        assert_mutation_digest_distinct(DrawingMutation::DeleteLayer(DeleteLayer { layer_id: "left".into() }), DrawingMutation::DeleteLayer(DeleteLayer { layer_id: "right".into() }));
        assert_mutation_digest_distinct(
            DrawingMutation::ReorderLayer(ReorderLayer { layer_id: "layer".into(), parent_id: None, index: 0 }),
            DrawingMutation::ReorderLayer(ReorderLayer { layer_id: "layer".into(), parent_id: Some("parent".into()), index: 1 }),
        );
        drain_snapshot(DrawingSnapshot { layers: vec![baseline], ..crate::artifacts::drawing::schema::default_drawing_document("digest-owner", None) });
    }

    #[test]
    fn retained_drawing_aggregate_credit_admits_exact_4096_rejects_plus_one_with_owner_handback() {
        let exact_source = nested_snapshot();
        let exact_owner = exact_source.layers.as_ptr();
        let exact_target = match exact_source.layers.last().expect("Drawing exact-boundary group") {
            DrawingLayerNode::Group(group) => crate::artifacts::drawing::schema::layer_id(&group.children[0]).to_string(),
            _ => unreachable!("Drawing exact-boundary group remains exact"),
        };
        let exact = DrawingMutation::RenameLayer(RenameLayer { layer_id: exact_target, new_name: "x".repeat(DRAWING_OWNED_FIELD_BYTES) });
        let exact_source = apply(exact_source, &exact).expect("an exact 4096-byte retained overlay page is admitted");
        assert_eq!(exact_source.layers.as_ptr(), exact_owner, "exact boundary publication retains the source container owner");
        drain_mutation(exact);
        drain_snapshot(exact_source);

        let plus_source = nested_snapshot();
        let plus_owner = plus_source.layers.as_ptr();
        let plus_target = match plus_source.layers.last().expect("Drawing +1 group") {
            DrawingLayerNode::Group(group) => crate::artifacts::drawing::schema::layer_id(&group.children[0]).to_string(),
            _ => unreachable!("Drawing +1 group remains exact"),
        };
        let plus_one = DrawingMutation::RenameLayer(RenameLayer { layer_id: plus_target, new_name: "x".repeat(DRAWING_OWNED_FIELD_BYTES + 1) });
        let (plus_source, error) = apply(plus_source, &plus_one).expect_err("4096 +1 retained overlay page rejects");
        assert_eq!(error, "drawing-store.mutation-field-capacity");
        assert_eq!(plus_source.layers.as_ptr(), plus_owner, "+1 rejection returns the exact source owner without partial publication");
        drain_mutation(plus_one);
        drain_snapshot(plus_source);

        let mut source = crate::artifacts::drawing::schema::default_drawing_document("aggregate-owner", None);
        let mutation = DrawingMutation::SetLayerVisible(SetLayerVisible { layer_id: crate::artifacts::drawing::schema::layer_id(&source.layers[0]).into(), visible: false });
        let mut last_admitted = None;
        for index in 0..DRAWING_MUTATION_AGGREGATE_ITEMS {
            source.layers.push(crate::artifacts::drawing::schema::create_drawing_path_layer(&format!("layer-{index}"), Vec::new()));
            let owner = source.layers.as_ptr();
            match live_reservation(&mut source, &mutation) {
                Ok(reservation) => {
                    assert_eq!(source.layers.as_ptr(), owner, "live aggregate census never replaces the exact source owner");
                    last_admitted = Some((source.layers.len(), reservation.total_items().expect("live item total"), reservation.total_bytes().expect("live byte total")));
                }
                Err("drawing-store.mutation-aggregate-item-capacity" | "drawing-store.mutation-aggregate-byte-capacity") => {
                    assert_eq!(source.layers.as_ptr(), owner, "aggregate +1 rejection returns the exact source backing");
                    break;
                }
                Err(error) => panic!("unexpected live Drawing aggregate rejection: {error}"),
            }
        }
        let (admitted_layers, admitted_items, admitted_bytes) = last_admitted.expect("at least one live aggregate owner is admitted");
        assert_eq!(source.layers.len(), admitted_layers + 1, "the first additional real layer owner is the +1 rejection");
        assert!(admitted_items <= DRAWING_MUTATION_AGGREGATE_ITEMS && admitted_bytes <= DRAWING_MUTATION_AGGREGATE_BYTES);
        let last_valid_id = source.id.clone();
        let (source, error) = apply(source, &mutation).expect_err("aggregate +1 rejects");
        assert_eq!(error, "drawing-store.mutation-aggregate-item-capacity");
        assert_eq!(source.id, last_valid_id, "aggregate rejection returns the exact source authority without partial publication");
        drain_mutation(mutation);
        drain_snapshot(source);
    }

    #[test]
    fn retained_drawing_duplicate_hash_frames_domain_id_and_name_lengths_without_concatenation_collision() {
        fn duplicate_id(id: &str, name: &str) -> String {
            let mut source = crate::artifacts::drawing::schema::default_drawing_document("duplicate-framing", None);
            let mut layer = crate::artifacts::drawing::schema::create_drawing_path_layer(name, Vec::new());
            let base = crate::artifacts::drawing::schema::layer_base_mut(&mut layer);
            base.id.clear();
            base.id.push_str(id);
            admit_layer_string_destinations(&mut layer);
            source.layers = vec![layer];
            let mutation = DrawingMutation::DuplicateLayer(DuplicateLayer { layer_id: id.into() });
            let source = apply(source, &mutation).expect("framed duplicate mutation applies");
            let duplicate = source.layers.get(1).map(crate::artifacts::drawing::schema::layer_id).expect("duplicated layer remains retained").to_string();
            drain_mutation(mutation);
            drain_snapshot(source);
            duplicate
        }

        assert_ne!(duplicate_id("ab", "c"), duplicate_id("a", "bc"), "separate id/name length frames prevent concatenation collisions");
    }

    #[test]
    fn retained_drawing_duplicate_name_uses_preadmitted_page_and_returns_exact_rejection_owner() {
        let mut source = crate::artifacts::drawing::schema::default_drawing_document("duplicate-name-owner", None);
        let mut layer = crate::artifacts::drawing::schema::create_drawing_path_layer("Layer", Vec::new());
        admit_layer_string_destinations(&mut layer);
        let target = crate::artifacts::drawing::schema::layer_id(&layer).to_string();
        let original_name_owner = crate::artifacts::drawing::schema::layer_base_mut(&mut layer).name.as_ptr();
        source.layers = vec![layer];
        let mutation = DrawingMutation::DuplicateLayer(DuplicateLayer { layer_id: target });
        let source = apply(source, &mutation).expect("duplicate name suffix uses only pre-admitted destination and fixed scratch page");
        assert_eq!(crate::artifacts::drawing::schema::layer_base(&source.layers[0]).name.as_ptr(), original_name_owner, "last-valid name backing remains exact");
        assert_eq!(crate::artifacts::drawing::schema::layer_base(&source.layers[1]).name, "Layer copy");
        drain_mutation(mutation);
        drain_snapshot(source);

        let mut rejected = crate::artifacts::drawing::schema::default_drawing_document("duplicate-name-rejected", None);
        let layer = crate::artifacts::drawing::schema::create_drawing_path_layer("Layer", Vec::new());
        let target = crate::artifacts::drawing::schema::layer_id(&layer).to_string();
        rejected.layers = vec![layer];
        let exact_owner = rejected.layers.as_ptr();
        let mutation = DrawingMutation::DuplicateLayer(DuplicateLayer { layer_id: target });
        let (rejected, error) = apply(rejected, &mutation).expect_err("unadmitted duplicate destination rejects without allocating after operation admission");
        assert_eq!(error, "drawing-store.duplicate-destination-capacity");
        assert_eq!(rejected.layers.as_ptr(), exact_owner, "duplicate rejection returns the exact source container owner");
        drain_mutation(mutation);
        drain_snapshot(rejected);
    }

    #[test]
    fn retained_drawing_cancel_stale_each_replay_candidate_container_stage_preserves_last_valid() {
        let stages = [
            DrawingMutationCandidatePhase::PreflightSource,
            DrawingMutationCandidatePhase::PreflightMutation,
            DrawingMutationCandidatePhase::BindOverlay,
            DrawingMutationCandidatePhase::LocatePrimary,
            DrawingMutationCandidatePhase::LocateSecondary,
            DrawingMutationCandidatePhase::PrepareOwnedValue,
            DrawingMutationCandidatePhase::Apply,
            DrawingMutationCandidatePhase::RebuildSource,
            DrawingMutationCandidatePhase::LocateDestination,
            DrawingMutationCandidatePhase::RebuildDestination,
            DrawingMutationCandidatePhase::Complete,
        ];
        for stage in stages {
            for stale in [false, true] {
                let mut source = nested_snapshot();
                let (group_id, target) = match source.layers.last().expect("Drawing group") {
                    DrawingLayerNode::Group(group) => (group.base.id.clone(), crate::artifacts::drawing::schema::layer_id(&group.children[0]).to_string()),
                    _ => unreachable!("Drawing fixture group remains exact"),
                };
                let last_valid_id = source.id.clone();
                let mutation = match stage {
                    DrawingMutationCandidatePhase::LocateSecondary => {
                        DrawingMutation::CreateLayer(CreateLayer { parent_id: Some(group_id), index: Some(0), layer: Box::new(crate::artifacts::drawing::schema::create_drawing_path_layer("cancel-create", Vec::new())) })
                    }
                    DrawingMutationCandidatePhase::RebuildSource | DrawingMutationCandidatePhase::LocateDestination => DrawingMutation::ReorderLayer(ReorderLayer { layer_id: target, parent_id: Some(group_id), index: 2 }),
                    _ => DrawingMutation::DuplicateLayer(DuplicateLayer { layer_id: target }),
                };
                let operation = semio_framework_job::OperationId(8_003);
                let generation = semio_framework_job::Generation(83);
                let mut authority = DrawingMutationCandidateAuthority::try_new(operation, generation).expect("Drawing candidate fixed owner arenas admit");
                let cancel = semio_framework_job::root_cancel_token();
                let mut preview_sequence = 0;
                for _ in 0..100_000 {
                    if authority.phase == stage {
                        break;
                    }
                    let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
                    assert!(!authority.step(&mut source, &mutation, &mut context).expect("Drawing stage fixture advances"));
                }
                assert_eq!(authority.phase, stage, "fixture reaches each replay/candidate/container stage exactly");
                if !stale {
                    cancel.cancel_now();
                }
                let mut rejected = semio_framework_job::StepContext::new(
                    operation,
                    if stale { semio_framework_job::Generation(84) } else { generation },
                    semio_framework_job::StepBudget::new(1, u64::MAX),
                    cancel,
                    semio_framework_job::default_now_us,
                    &mut preview_sequence,
                );
                assert_eq!(authority.step(&mut source, &mutation, &mut rejected), Err(if stale { "drawing-store.mutation-candidate-stale-authority" } else { "drawing-store.mutation-candidate-cancelled" }),);
                close_candidate(&mut authority, Some(&mut source));
                drop(authority);
                assert_eq!(source.id, last_valid_id, "cancel/stale close never publishes a partial candidate");
                drain_mutation(mutation);
                drain_snapshot(source);
            }
        }
    }
}
//#endregion 🧪️RetainedMutationAuthorityTests
