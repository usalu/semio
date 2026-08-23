//! 🧰️ Draw owned envelope decoder, recursive retirement, and retained store initializer.

use crate::artifacts::draw::op::DrawMutation;
use crate::artifacts::draw::{DrawAttributes, DrawImageAsset, DrawLayerBase, DrawLayerNode, DrawSnapshot, FillStyle, GradientStop, PathSegment, StrokeStyle};

//#region 🔖️OwnedSprCatalog
const DRAW_OWNED_FIELD_BYTES: usize = store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;

enum DrawRetirementOwner {
    Snapshot(DrawSnapshot),
    Mutation(DrawMutation),
    MutationFields(DrawMutationFields),
    Layer(DrawLayerNode),
    Layers(Vec<DrawLayerNode>),
    LayerFields(DrawLayerFields),
    Base(DrawLayerBase),
    Attributes(DrawAttributes),
    Fill(FillStyle),
    Stroke(StrokeStyle),
    Asset(DrawImageAsset),
    AssetEntry { key: String, value: Option<DrawImageAsset> },
    String(String),
    Strings(Vec<String>),
    Segments(Vec<PathSegment>),
    Stops(Vec<GradientStop>),
    Points(Vec<[f64; 2]>),
}

enum DrawMutationFields {
    String(String),
    Strings { first: String, second: Option<String> },
    Fill { id: String, value: Option<FillStyle> },
    Stroke { id: String, value: Option<StrokeStyle> },
    Layer { parent: Option<String>, value: Option<Box<DrawLayerNode>> },
}

enum DrawLayerFields {
    Shape { base: Option<DrawLayerBase>, shape_kind: String, points: Option<Vec<[f64; 2]>> },
    Path { base: Option<DrawLayerBase>, segments: Option<Vec<PathSegment>> },
    Text { base: Option<DrawLayerBase>, content: String },
    Image { base: Option<DrawLayerBase>, image_key: String },
    Group { base: Option<DrawLayerBase>, children: Option<Vec<DrawLayerNode>> },
    Boolean { base: Option<DrawLayerBase>, operation: String, children: Option<Vec<String>> },
    Trace { base: Option<DrawLayerBase>, source_key: String },
}

struct DrawOwnedRetirement {
    owner: std::mem::ManuallyDrop<Option<DrawRetirementOwner>>,
    active: std::mem::ManuallyDrop<Option<Box<DrawOwnedRetirement>>>,
    phase: u8,
}

impl DrawOwnedRetirement {
    fn new(owner: DrawRetirementOwner) -> Self {
        Self { owner: std::mem::ManuallyDrop::new(Some(owner)), active: std::mem::ManuallyDrop::new(None), phase: 0 }
    }

    fn spawn(active: &mut Option<Box<Self>>, owner: DrawRetirementOwner) -> store::SnapshotRetirementStep {
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
            DrawRetirementOwner::Snapshot(value) => match self.phase {
                0 => {
                    if let Some(value) = value.layers.pop() {
                        return Ok(Self::spawn(&mut self.active, DrawRetirementOwner::Layer(value)));
                    }
                    self.phase = 1;
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                }
                1 => {
                    if let Some((key, value)) = value.assets.pop_last() {
                        return Ok(Self::spawn(&mut self.active, DrawRetirementOwner::AssetEntry { key, value: Some(value) }));
                    }
                    self.phase = 2;
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                }
                2 => Ok(Self::release_string(&mut value.schema, &mut self.phase, 3, maximum_items, maximum_bytes)),
                3 => Ok(Self::release_string(&mut value.id, &mut self.phase, 4, maximum_items, maximum_bytes)),
                4 if value.title.is_some() => {
                    let title = value.title.take().expect("Draw title remains retained");
                    self.phase = 5;
                    Ok(Self::spawn(&mut self.active, DrawRetirementOwner::String(title)))
                }
                _ => {
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
            },
            DrawRetirementOwner::Layer(_) => {
                let layer = match self.owner.take() {
                    Some(DrawRetirementOwner::Layer(value)) => value,
                    _ => unreachable!("Draw layer owner variant remains exact"),
                };
                let fields = match layer {
                    DrawLayerNode::Shape(value) => DrawLayerFields::Shape { base: Some(value.base), shape_kind: value.shape_kind, points: value.polygon.map(|polygon| polygon.points) },
                    DrawLayerNode::Path(value) => DrawLayerFields::Path { base: Some(value.base), segments: Some(value.segments) },
                    DrawLayerNode::Text(value) => DrawLayerFields::Text { base: Some(value.base), content: value.content },
                    DrawLayerNode::Image(value) => DrawLayerFields::Image { base: Some(value.base), image_key: value.image_key },
                    DrawLayerNode::Group(value) => DrawLayerFields::Group { base: Some(value.base), children: Some(value.children) },
                    DrawLayerNode::Boolean(value) => DrawLayerFields::Boolean { base: Some(value.base), operation: value.operation, children: Some(value.children) },
                    DrawLayerNode::Trace(value) => DrawLayerFields::Trace { base: Some(value.base), source_key: value.source_key },
                };
                *self.owner = Some(DrawRetirementOwner::LayerFields(fields));
                Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
            }
            DrawRetirementOwner::Layers(values) => {
                if let Some(value) = values.pop() {
                    Ok(Self::spawn(&mut self.active, DrawRetirementOwner::Layer(value)))
                } else {
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
            }
            DrawRetirementOwner::LayerFields(fields) => {
                if self.phase == 0 {
                    let base = match fields {
                        DrawLayerFields::Shape { base, .. }
                        | DrawLayerFields::Path { base, .. }
                        | DrawLayerFields::Text { base, .. }
                        | DrawLayerFields::Image { base, .. }
                        | DrawLayerFields::Group { base, .. }
                        | DrawLayerFields::Boolean { base, .. }
                        | DrawLayerFields::Trace { base, .. } => base.take(),
                    }
                    .ok_or_else(|| "Draw layer base owner missing".to_string())?;
                    self.phase = 1;
                    return Ok(Self::spawn(&mut self.active, DrawRetirementOwner::Base(base)));
                }
                let nested = match fields {
                    DrawLayerFields::Shape { shape_kind, points, .. } if self.phase == 1 => Some(DrawRetirementOwner::String(std::mem::take(shape_kind))),
                    DrawLayerFields::Shape { points, .. } => points.take().map(DrawRetirementOwner::Points),
                    DrawLayerFields::Path { segments, .. } => segments.take().map(DrawRetirementOwner::Segments),
                    DrawLayerFields::Text { content, .. } => (!content.is_empty()).then(|| DrawRetirementOwner::String(std::mem::take(content))),
                    DrawLayerFields::Image { image_key, .. } => (!image_key.is_empty()).then(|| DrawRetirementOwner::String(std::mem::take(image_key))),
                    DrawLayerFields::Group { children, .. } => children.as_mut().and_then(Vec::pop).map(DrawRetirementOwner::Layer),
                    DrawLayerFields::Boolean { operation, .. } if self.phase == 1 => Some(DrawRetirementOwner::String(std::mem::take(operation))),
                    DrawLayerFields::Boolean { children, .. } => children.take().map(DrawRetirementOwner::Strings),
                    DrawLayerFields::Trace { source_key, .. } => (!source_key.is_empty()).then(|| DrawRetirementOwner::String(std::mem::take(source_key))),
                };
                if let Some(nested) = nested {
                    self.phase = self.phase.saturating_add(1);
                    return Ok(Self::spawn(&mut self.active, nested));
                }
                drop(self.owner.take());
                Ok(store::SnapshotRetirementStep::Complete)
            }
            DrawRetirementOwner::Base(value) => match self.phase {
                0 => Ok(Self::release_string(&mut value.id, &mut self.phase, 1, maximum_items, maximum_bytes)),
                1 => Ok(Self::release_string(&mut value.name, &mut self.phase, 2, maximum_items, maximum_bytes)),
                2 => Ok(Self::release_string(&mut value.blend_mode, &mut self.phase, 3, maximum_items, maximum_bytes)),
                3 => {
                    let attributes = std::mem::take(&mut value.attributes);
                    self.phase = 4;
                    Ok(Self::spawn(&mut self.active, DrawRetirementOwner::Attributes(attributes)))
                }
                _ => {
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
            },
            DrawRetirementOwner::Attributes(value) => {
                if let Some(fill) = value.fill.take() {
                    return Ok(Self::spawn(&mut self.active, DrawRetirementOwner::Fill(fill)));
                }
                if let Some(stroke) = value.stroke.take() {
                    return Ok(Self::spawn(&mut self.active, DrawRetirementOwner::Stroke(stroke)));
                }
                drop(self.owner.take());
                Ok(store::SnapshotRetirementStep::Complete)
            }
            DrawRetirementOwner::Fill(value) => match value {
                FillStyle::LinearGradient { stops, .. } | FillStyle::RadialGradient { stops, .. } if !stops.is_empty() => Ok(Self::spawn(&mut self.active, DrawRetirementOwner::Stops(std::mem::take(stops)))),
                _ => {
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
            },
            DrawRetirementOwner::Stroke(value) => match self.phase {
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
            DrawRetirementOwner::Asset(value) => match self.phase {
                0 => Ok(Self::release_string(&mut value.mime, &mut self.phase, 1, maximum_items, maximum_bytes)),
                1 => Ok(Self::release_string(&mut value.data, &mut self.phase, 2, maximum_items, maximum_bytes)),
                _ => {
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
            },
            DrawRetirementOwner::AssetEntry { key, value } => match self.phase {
                0 => Ok(Self::release_string(key, &mut self.phase, 1, maximum_items, maximum_bytes)),
                1 => {
                    self.phase = 2;
                    Ok(Self::spawn(&mut self.active, DrawRetirementOwner::Asset(value.take().ok_or_else(|| "Draw asset owner missing".to_string())?)))
                }
                _ => {
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
            },
            DrawRetirementOwner::String(value) => {
                if self.phase == 0 {
                    return Ok(Self::release_string(value, &mut self.phase, 1, maximum_items, maximum_bytes));
                }
                drop(self.owner.take());
                Ok(store::SnapshotRetirementStep::Complete)
            }
            DrawRetirementOwner::Strings(values) => {
                if let Some(value) = values.pop() {
                    return Ok(Self::spawn(&mut self.active, DrawRetirementOwner::String(value)));
                }
                drop(self.owner.take());
                Ok(store::SnapshotRetirementStep::Complete)
            }
            DrawRetirementOwner::Segments(values) => {
                if values.pop().is_some() {
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                } else {
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
            }
            DrawRetirementOwner::Stops(values) => {
                if values.pop().is_some() {
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                } else {
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
            }
            DrawRetirementOwner::Points(values) => {
                if values.pop().is_some() {
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                } else {
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
            }
            DrawRetirementOwner::Mutation(_) => {
                use DrawMutation::*;
                let mutation = match self.owner.take() {
                    Some(DrawRetirementOwner::Mutation(value)) => value,
                    _ => unreachable!("Draw mutation owner variant remains exact"),
                };
                let fields = match mutation {
                    SetLayerVisible(payload) => DrawMutationFields::String(payload.layer_id),
                    SetLayerLocked(payload) => DrawMutationFields::String(payload.layer_id),
                    SetLayerOpacity(payload) => DrawMutationFields::String(payload.layer_id),
                    SetLayerBlendMode(payload) => DrawMutationFields::Strings { first: payload.layer_id, second: Some(payload.blend_mode) },
                    RenameLayer(payload) => DrawMutationFields::Strings { first: payload.layer_id, second: Some(payload.new_name) },
                    UpdateLayerTransform(payload) => DrawMutationFields::String(payload.layer_id),
                    ReplaceLayerFill(payload) => DrawMutationFields::Fill { id: payload.layer_id, value: payload.fill },
                    ReplaceLayerStroke(payload) => DrawMutationFields::Stroke { id: payload.layer_id, value: payload.stroke },
                    SetLayerBooleanOperation(payload) => DrawMutationFields::Strings { first: payload.layer_id, second: Some(payload.boolean_operation) },
                    UpdateLayerTraceParams(payload) => DrawMutationFields::String(payload.layer_id),
                    CreateLayer(payload) => DrawMutationFields::Layer { parent: payload.parent_id, value: Some(payload.layer) },
                    DuplicateLayer(payload) => DrawMutationFields::String(payload.layer_id),
                    DeleteLayer(payload) => DrawMutationFields::String(payload.layer_id),
                    ReorderLayer(payload) => DrawMutationFields::Strings { first: payload.layer_id, second: payload.parent_id },
                };
                *self.owner = Some(DrawRetirementOwner::MutationFields(fields));
                Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
            }
            DrawRetirementOwner::MutationFields(fields) => match fields {
                DrawMutationFields::String(value) => {
                    if self.phase == 0 {
                        return Ok(Self::release_string(value, &mut self.phase, 1, maximum_items, maximum_bytes));
                    }
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
                DrawMutationFields::Strings { first, second } => match self.phase {
                    0 => Ok(Self::release_string(first, &mut self.phase, 1, maximum_items, maximum_bytes)),
                    1 if second.is_some() => {
                        self.phase = 2;
                        Ok(Self::spawn(&mut self.active, DrawRetirementOwner::String(second.take().expect("Draw second string remains exact"))))
                    }
                    _ => {
                        drop(self.owner.take());
                        Ok(store::SnapshotRetirementStep::Complete)
                    }
                },
                DrawMutationFields::Fill { id, value } => match self.phase {
                    0 => Ok(Self::release_string(id, &mut self.phase, 1, maximum_items, maximum_bytes)),
                    1 if value.is_some() => {
                        self.phase = 2;
                        Ok(Self::spawn(&mut self.active, DrawRetirementOwner::Fill(value.take().expect("Draw fill remains exact"))))
                    }
                    _ => {
                        drop(self.owner.take());
                        Ok(store::SnapshotRetirementStep::Complete)
                    }
                },
                DrawMutationFields::Stroke { id, value } => match self.phase {
                    0 => Ok(Self::release_string(id, &mut self.phase, 1, maximum_items, maximum_bytes)),
                    1 if value.is_some() => {
                        self.phase = 2;
                        Ok(Self::spawn(&mut self.active, DrawRetirementOwner::Stroke(value.take().expect("Draw stroke remains exact"))))
                    }
                    _ => {
                        drop(self.owner.take());
                        Ok(store::SnapshotRetirementStep::Complete)
                    }
                },
                DrawMutationFields::Layer { parent, value } => match self.phase {
                    0 if parent.is_some() => {
                        self.phase = 1;
                        Ok(Self::spawn(&mut self.active, DrawRetirementOwner::String(parent.take().expect("Draw parent remains exact"))))
                    }
                    0 => {
                        self.phase = 1;
                        Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                    }
                    1 if value.is_some() => {
                        self.phase = 2;
                        Ok(Self::spawn(&mut self.active, DrawRetirementOwner::Layer(*value.take().expect("Draw layer remains exact"))))
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

impl store::ErasedSnapshotRetirement for DrawOwnedRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if let Some(active) = self.active.as_mut() {
            return match active.close_step(maximum_items.min(1), maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if active.terminal_is_empty() => {
                    drop(self.active.take());
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                store::SnapshotRetirementStep::Complete => Err("Draw nested retirement reported false terminal".into()),
                step => Ok(step),
            };
        }
        self.advance(maximum_items.min(1), maximum_bytes)
    }

    fn terminal_is_empty(&self) -> bool {
        self.owner.is_none() && self.active.is_none()
    }
}

impl Drop for DrawOwnedRetirement {
    fn drop(&mut self) {
        assert!(store::ErasedSnapshotRetirement::terminal_is_empty(self), "Draw owner reached Drop before cursor retirement reached terminal-empty");
    }
}

pub struct DrawSnapshotRetirementFactory;

impl store::ArtifactOwnedValueRetirementFactory<DrawSnapshot> for DrawSnapshotRetirementFactory {
    fn retire_owned(&self, value: DrawSnapshot) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(DrawOwnedRetirement::new(DrawRetirementOwner::Snapshot(value)))
    }
}

struct DrawSnapshotRootRetirement {
    owner: std::mem::ManuallyDrop<Option<std::sync::Arc<DrawSnapshot>>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
}

impl store::ErasedSnapshotRetirement for DrawSnapshotRootRetirement {
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
                store::SnapshotRetirementStep::Complete => Err("Draw snapshot root retirement reported false terminal".into()),
                step => Ok(step),
            };
        }
        let Some(owner) = self.owner.take() else { return Ok(store::SnapshotRetirementStep::Complete) };
        match std::sync::Arc::try_unwrap(owner) {
            Ok(value) => {
                *self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&DrawSnapshotRetirementFactory, value));
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

impl Drop for DrawSnapshotRootRetirement {
    fn drop(&mut self) {
        assert!(self.owner.is_none() && self.retirement.is_none(), "Draw snapshot root reached Drop before exact Arc handback");
    }
}

impl store::SnapshotRetirementFactory<DrawSnapshot> for DrawSnapshotRetirementFactory {
    fn retire(&self, snapshot: std::sync::Arc<DrawSnapshot>) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(DrawSnapshotRootRetirement { owner: std::mem::ManuallyDrop::new(Some(snapshot)), retirement: std::mem::ManuallyDrop::new(None) })
    }
}

pub struct DrawMutationRetirementFactory;

impl store::ArtifactOwnedValueRetirementFactory<DrawMutation> for DrawMutationRetirementFactory {
    fn retire_owned(&self, value: DrawMutation) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(DrawOwnedRetirement::new(DrawRetirementOwner::Mutation(value)))
    }
}

fn decode_draw_snapshot_pack(bytes: &[u8]) -> Result<DrawSnapshot, ()> {
    <DrawSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|_| ())
}

fn decode_draw_mutation_pack(bytes: &[u8]) -> Result<DrawMutation, ()> {
    DrawMutation::decode_op(bytes).map_err(|_| ())
}

macro_rules! draw_owned_field_authority {
    ($state:ident, $authority:ident, $value:ty, $authority_trait:ident, $target_trait:ident, $publish:ident, $decode:path, $factory:expr, $kind:literal) => {
        enum $state {
            AwaitToken,
            Decode(store::OwnedSchemaHexAuthority<DRAW_OWNED_FIELD_BYTES>),
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
                        return Err(diagnostic(concat!("draw-envelope.", $kind, "-pack-must-be-scalar"), token.start));
                    }
                    self.state = $state::Decode(store::OwnedSchemaHexAuthority::try_new(self.operation, self.generation, token, self.path)?);
                }
                let $state::Decode(authority) = &mut self.state else {
                    return Err(diagnostic(concat!("draw-envelope.", $kind, "-pack-token-replayed"), token.start));
                };
                match authority.step(source, cx) {
                    store::OwnedSchemaHexStep::Pending => Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending),
                    store::OwnedSchemaHexStep::Complete => {
                        let bytes = authority.as_bytes().ok_or_else(|| diagnostic(concat!("draw-envelope.", $kind, "-pack-missing"), token.start))?;
                        let value = $decode(bytes).map_err(|_| diagnostic(concat!("draw-envelope.", $kind, "-pack-malformed"), token.start))?;
                        if !authority.release() {
                            return Err(diagnostic(concat!("draw-envelope.", $kind, "-pack-release-duplicate"), token.start));
                        }
                        *self.value = Some(value);
                        self.state = $state::Ready;
                        Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
                    }
                    store::OwnedSchemaHexStep::Cancelled => Err(diagnostic(concat!("draw-envelope.", $kind, "-pack-cancelled"), token.start)),
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
                    return Err(self.diagnostic(concat!("draw-envelope.", $kind, "-pack-not-ready"), 0));
                }
                let value = self.value.take().ok_or_else(|| self.diagnostic(concat!("draw-envelope.", $kind, "-owner-missing"), 0))?;
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
                let retirement = self.retirement.as_mut().expect("Draw packed field retirement remains retained");
                match retirement.close_step(maximum_items.min(1), maximum_bytes).map_err(|_| store::OwnedSchemaDecodeDiagnostic { code: concat!("draw-envelope.", $kind, "-retirement-fault"), offset: 0, line: 0, column: 0, path })? {
                    store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                        drop(self.retirement.take());
                        self.state = $state::Complete;
                        Ok(store::SnapshotRetirementStep::Complete)
                    }
                    store::SnapshotRetirementStep::Complete => Err(self.diagnostic(concat!("draw-envelope.", $kind, "-retirement-false-terminal"), 0)),
                    step => Ok(step),
                }
            }

            fn terminal_is_empty(&self) -> bool {
                matches!(self.state, $state::Published | $state::Complete) && self.value.is_none() && self.retirement.is_none()
            }
        }

        impl Drop for $authority {
            fn drop(&mut self) {
                assert!(matches!(self.state, $state::Published | $state::Complete) && self.value.is_none() && self.retirement.is_none(), concat!("Draw ", $kind, " decode reached Drop before publication or bounded retirement"));
            }
        }
    };
}

draw_owned_field_authority!(
    DrawSnapshotDecodeState,
    DrawSnapshotDecodeAuthority,
    DrawSnapshot,
    ArtifactEnvelopeSnapshotFieldAuthority,
    ArtifactEnvelopeSnapshotFieldTarget,
    publish_snapshot_reserved,
    decode_draw_snapshot_pack,
    &DrawSnapshotRetirementFactory,
    "snapshot"
);

draw_owned_field_authority!(
    DrawMutationDecodeState,
    DrawMutationDecodeAuthority,
    DrawMutation,
    ArtifactEnvelopeMutationFieldAuthority,
    ArtifactEnvelopeMutationFieldTarget,
    publish_mutation_reserved,
    decode_draw_mutation_pack,
    &DrawMutationRetirementFactory,
    "mutation"
);

struct DrawRejectedConflictAuthority {
    terminal: bool,
}

impl store::ArtifactEnvelopeSprConflictAuthority for DrawRejectedConflictAuthority {
    fn accept_token(
        &mut self,
        token: store::OwnedSchemaToken,
        _terminal: bool,
        _source: &store::OwnedSchemaRecordCursor,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        Err(store::OwnedSchemaDecodeDiagnostic { code: "draw-envelope.fresh-conflict-not-admitted", offset: token.start, line: 0, column: 0, path: store::OwnedSchemaPath::ROOT })
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

pub struct DrawEnvelopeOwnedFieldCatalog;

impl store::ArtifactEnvelopeOwnedFieldCatalog<DrawSnapshot, DrawMutation> for DrawEnvelopeOwnedFieldCatalog {
    fn begin_vcs(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeVcsFieldAuthority<DrawSnapshot, DrawMutation>> {
        Box::new(store::ArtifactEnvelopeFreshVcsAuthority::new(self.begin_snapshot(operation, generation, path), std::sync::Arc::new(DrawSnapshotRetirementFactory), std::sync::Arc::new(DrawMutationRetirementFactory), self.edit_history_decoder()))
    }

    fn begin_snapshot(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeSnapshotFieldAuthority<DrawSnapshot>> {
        Box::new(DrawSnapshotDecodeAuthority::new(operation, generation, path))
    }

    fn begin_mutation(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeMutationFieldAuthority<DrawMutation>> {
        Box::new(DrawMutationDecodeAuthority::new(operation, generation, path))
    }

    fn begin_spr_conflict(&self, _operation: semio_framework_job::OperationId, _generation: semio_framework_job::Generation, _path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeSprConflictAuthority> {
        Box::new(DrawRejectedConflictAuthority { terminal: false })
    }

    fn edit_history_decoder(&self) -> std::sync::Arc<dyn store::ArtifactOwnedHistoryEntryDecoder<protocol::Edit<DrawMutation>>> {
        store::artifact_owned_spr_edit_history_decoder(std::sync::Arc::new(Self), std::sync::Arc::new(DrawMutationRetirementFactory))
    }
}

pub fn draw_envelope_decode_owner_bundle() -> store::ArtifactEnvelopeDecodeOwnerBundle<DrawSnapshot, DrawMutation> {
    store::ArtifactEnvelopeDecodeOwnerBundle::new(std::sync::Arc::new(DrawEnvelopeOwnedFieldCatalog), std::sync::Arc::new(DrawSnapshotRetirementFactory), std::sync::Arc::new(DrawMutationRetirementFactory))
}
//#endregion 🔖️OwnedSprCatalog

//#region 🔖️RetainedStoreInitialization
const DRAW_MAXIMUM_NESTED_ITEMS: usize = 4_096;
const DRAW_MAXIMUM_NESTED_BYTES: usize = store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES;
const DRAW_MAXIMUM_LAYER_DEPTH: usize = 64;

#[derive(Clone, Copy)]
struct DrawTraversalFrame {
    phase: u8,
    child: usize,
    string: usize,
}

impl DrawTraversalFrame {
    const EMPTY: Self = Self { phase: 0, child: 0, string: 0 };
}

struct DrawSnapshotBoundsAuthority {
    root: usize,
    asset: usize,
    depth: usize,
    path: [usize; DRAW_MAXIMUM_LAYER_DEPTH],
    frames: [DrawTraversalFrame; DRAW_MAXIMUM_LAYER_DEPTH],
    items: usize,
    bytes: usize,
    layers_complete: bool,
    terminal: bool,
}

impl DrawSnapshotBoundsAuthority {
    fn new() -> Self {
        Self { root: 0, asset: 0, depth: 0, path: [0; DRAW_MAXIMUM_LAYER_DEPTH], frames: [DrawTraversalFrame::EMPTY; DRAW_MAXIMUM_LAYER_DEPTH], items: 0, bytes: 0, layers_complete: false, terminal: false }
    }

    fn layer_at<'a>(root: &'a DrawLayerNode, path: &[usize]) -> Option<&'a DrawLayerNode> {
        let mut value = root;
        for index in path {
            let DrawLayerNode::Group(group) = value else { return None };
            value = group.children.get(*index)?;
        }
        Some(value)
    }

    fn add(&mut self, items: usize, bytes: usize) -> Result<(), &'static str> {
        self.items = self.items.checked_add(items).ok_or("draw-store.preflight-item-overflow")?;
        self.bytes = self.bytes.checked_add(bytes).ok_or("draw-store.preflight-byte-overflow")?;
        if self.items > DRAW_MAXIMUM_NESTED_ITEMS {
            return Err("draw-store.preflight-item-capacity");
        }
        if self.bytes > DRAW_MAXIMUM_NESTED_BYTES {
            return Err("draw-store.preflight-byte-capacity");
        }
        Ok(())
    }

    fn direct_shape(layer: &DrawLayerNode) -> (usize, usize) {
        let base = match layer {
            DrawLayerNode::Shape(value) => &value.base,
            DrawLayerNode::Path(value) => &value.base,
            DrawLayerNode::Text(value) => &value.base,
            DrawLayerNode::Image(value) => &value.base,
            DrawLayerNode::Group(value) => &value.base,
            DrawLayerNode::Boolean(value) => &value.base,
            DrawLayerNode::Trace(value) => &value.base,
        };
        let mut items = 4;
        let mut bytes = base.id.len() + base.name.len() + base.blend_mode.len();
        if let Some(fill) = &base.attributes.fill {
            items += 1;
            match fill {
                FillStyle::Solid { .. } => {}
                FillStyle::LinearGradient { stops, .. } | FillStyle::RadialGradient { stops, .. } => items += stops.len(),
            }
        }
        if let Some(stroke) = &base.attributes.stroke {
            items += 3 + stroke.dash.as_ref().map_or(0, Vec::len);
            bytes += stroke.cap.len() + stroke.join.len();
        }
        match layer {
            DrawLayerNode::Shape(value) => {
                items += 1 + value.polygon.as_ref().map_or(0, |polygon| polygon.points.len());
                bytes += value.shape_kind.len();
            }
            DrawLayerNode::Path(value) => items += value.segments.len(),
            DrawLayerNode::Text(value) => {
                items += 1;
                bytes += value.content.len();
            }
            DrawLayerNode::Image(value) => {
                items += 1;
                bytes += value.image_key.len();
            }
            DrawLayerNode::Group(_) => {}
            DrawLayerNode::Boolean(value) => {
                items += 1 + value.children.len();
                bytes += value.operation.len();
            }
            DrawLayerNode::Trace(value) => {
                items += 1;
                bytes += value.source_key.len();
            }
        }
        (items, bytes)
    }

    fn step(&mut self, source: &DrawSnapshot, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        if !self.layers_complete {
            let Some(root) = source.layers.get(self.root) else {
                self.layers_complete = true;
                self.add(3 + usize::from(source.title.is_some()), source.schema.len() + source.id.len() + source.title.as_ref().map_or(0, String::len))?;
                cx.consume_fuel(1);
                return Ok(false);
            };
            let layer = Self::layer_at(root, &self.path[..self.depth]).ok_or("draw-store.preflight-path")?;
            let frame = self.frames[self.depth];
            if frame.phase == 0 {
                let (items, bytes) = Self::direct_shape(layer);
                self.add(items + 1, bytes)?;
                self.frames[self.depth].phase = 1;
                cx.consume_fuel(1);
                return Ok(false);
            }
            if let DrawLayerNode::Boolean(value) = layer {
                if let Some(child) = value.children.get(frame.string) {
                    self.add(0, child.len())?;
                    self.frames[self.depth].string += 1;
                    cx.consume_fuel(child.len().max(1) as u64);
                    return Ok(false);
                }
            }
            if let DrawLayerNode::Group(value) = layer {
                if frame.child < value.children.len() {
                    if self.depth + 1 >= DRAW_MAXIMUM_LAYER_DEPTH {
                        return Err("draw-store.preflight-depth-capacity");
                    }
                    self.path[self.depth] = frame.child;
                    self.frames[self.depth].child += 1;
                    self.depth += 1;
                    self.frames[self.depth] = DrawTraversalFrame::EMPTY;
                    cx.consume_fuel(1);
                    return Ok(false);
                }
            }
            if self.depth == 0 {
                self.root += 1;
                self.frames[0] = DrawTraversalFrame::EMPTY;
            } else {
                self.depth -= 1;
            }
            cx.consume_fuel(1);
            return Ok(false);
        }
        if let Some((key, value)) = source.assets.iter().nth(self.asset) {
            self.add(4, key.len() + value.mime.len() + value.data.len())?;
            self.asset += 1;
            cx.consume_fuel(1);
            return Ok(false);
        }
        self.terminal = true;
        Ok(true)
    }
}

struct DrawLayerCloneAuthority {
    value: std::mem::ManuallyDrop<Option<DrawLayerNode>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    depth: usize,
    path: [usize; DRAW_MAXIMUM_LAYER_DEPTH],
    frames: [DrawTraversalFrame; DRAW_MAXIMUM_LAYER_DEPTH],
    terminal: bool,
}

impl DrawLayerCloneAuthority {
    fn clone_string(source: &str) -> Result<String, &'static str> {
        if source.len() > DRAW_OWNED_FIELD_BYTES {
            return Err("draw-store.initializer-field-too-large");
        }
        let mut value = String::new();
        value.try_reserve_exact(source.len()).map_err(|_| "draw-store.initializer-string-admission")?;
        value.push_str(source);
        Ok(value)
    }

    fn base_skeleton(source: &DrawLayerBase) -> DrawLayerBase {
        DrawLayerBase {
            id: String::new(),
            name: String::new(),
            visible: source.visible,
            locked: source.locked,
            opacity: source.opacity,
            blend_mode: String::new(),
            transform: crate::artifacts::draw::DrawTransform { x: source.transform.x, y: source.transform.y, scale_x: source.transform.scale_x, scale_y: source.transform.scale_y, rotation: source.transform.rotation },
            attributes: DrawAttributes::default(),
        }
    }

    fn rect(source: &crate::artifacts::draw::DrawRect) -> crate::artifacts::draw::DrawRect {
        crate::artifacts::draw::DrawRect { x: source.x, y: source.y, width: source.width, height: source.height }
    }

    fn skeleton(source: &DrawLayerNode) -> Result<DrawLayerNode, &'static str> {
        Ok(match source {
            DrawLayerNode::Shape(value) => DrawLayerNode::Shape(crate::artifacts::draw::DrawShapeBody {
                base: Self::base_skeleton(&value.base),
                shape_kind: String::new(),
                rect: value.rect.as_ref().map(Self::rect),
                ellipse: value.ellipse.as_ref().map(|source| crate::artifacts::draw::DrawEllipse { cx: source.cx, cy: source.cy, rx: source.rx, ry: source.ry }),
                circle: value.circle.as_ref().map(|source| crate::artifacts::draw::DrawCircle { cx: source.cx, cy: source.cy, r: source.r }),
                line: value.line.as_ref().map(|source| crate::artifacts::draw::DrawLine { x1: source.x1, y1: source.y1, x2: source.x2, y2: source.y2 }),
                polygon: value.polygon.as_ref().map(|source| crate::artifacts::draw::DrawPolygon { points: Vec::with_capacity(source.points.len()) }),
            }),
            DrawLayerNode::Path(value) => DrawLayerNode::Path(crate::artifacts::draw::DrawPathBody { base: Self::base_skeleton(&value.base), segments: Vec::with_capacity(value.segments.len()) }),
            DrawLayerNode::Text(value) => DrawLayerNode::Text(crate::artifacts::draw::DrawTextBody { base: Self::base_skeleton(&value.base), x: value.x, y: value.y, content: String::new(), size: value.size }),
            DrawLayerNode::Image(value) => DrawLayerNode::Image(crate::artifacts::draw::DrawImageBody { base: Self::base_skeleton(&value.base), image_key: String::new(), width: value.width, height: value.height }),
            DrawLayerNode::Group(value) => DrawLayerNode::Group(crate::artifacts::draw::DrawGroupBody { base: Self::base_skeleton(&value.base), children: Vec::with_capacity(value.children.len()) }),
            DrawLayerNode::Boolean(value) => DrawLayerNode::Boolean(crate::artifacts::draw::DrawBooleanBody { base: Self::base_skeleton(&value.base), operation: String::new(), children: Vec::with_capacity(value.children.len()) }),
            DrawLayerNode::Trace(value) => DrawLayerNode::Trace(crate::artifacts::draw::DrawTraceBody {
                base: Self::base_skeleton(&value.base),
                source_key: String::new(),
                params: crate::artifacts::draw::DrawTraceParams { threshold: value.params.threshold, simplify_epsilon: value.params.simplify_epsilon },
            }),
        })
    }

    fn new(source: &DrawLayerNode) -> Result<Self, &'static str> {
        Ok(Self {
            value: std::mem::ManuallyDrop::new(Some(Self::skeleton(source)?)),
            retirement: std::mem::ManuallyDrop::new(None),
            depth: 0,
            path: [0; DRAW_MAXIMUM_LAYER_DEPTH],
            frames: [DrawTraversalFrame::EMPTY; DRAW_MAXIMUM_LAYER_DEPTH],
            terminal: false,
        })
    }

    fn source_at<'a>(root: &'a DrawLayerNode, path: &[usize]) -> Option<&'a DrawLayerNode> {
        DrawSnapshotBoundsAuthority::layer_at(root, path)
    }

    fn target_at_mut<'a>(root: &'a mut DrawLayerNode, path: &[usize]) -> Option<&'a mut DrawLayerNode> {
        if let Some((head, tail)) = path.split_first() {
            let DrawLayerNode::Group(group) = root else { return None };
            return Self::target_at_mut(group.children.get_mut(*head)?, tail);
        }
        Some(root)
    }

    fn bases<'a>(source: &'a DrawLayerNode, target: &'a mut DrawLayerNode) -> (&'a DrawLayerBase, &'a mut DrawLayerBase) {
        match (source, target) {
            (DrawLayerNode::Shape(source), DrawLayerNode::Shape(target)) => (&source.base, &mut target.base),
            (DrawLayerNode::Path(source), DrawLayerNode::Path(target)) => (&source.base, &mut target.base),
            (DrawLayerNode::Text(source), DrawLayerNode::Text(target)) => (&source.base, &mut target.base),
            (DrawLayerNode::Image(source), DrawLayerNode::Image(target)) => (&source.base, &mut target.base),
            (DrawLayerNode::Group(source), DrawLayerNode::Group(target)) => (&source.base, &mut target.base),
            (DrawLayerNode::Boolean(source), DrawLayerNode::Boolean(target)) => (&source.base, &mut target.base),
            (DrawLayerNode::Trace(source), DrawLayerNode::Trace(target)) => (&source.base, &mut target.base),
            _ => unreachable!("Draw clone source and target variants remain exact"),
        }
    }

    fn clone_fill(source: &FillStyle) -> FillStyle {
        match source {
            FillStyle::Solid { color } => FillStyle::Solid { color: *color },
            FillStyle::LinearGradient { x1, y1, x2, y2, stops } => FillStyle::LinearGradient { x1: *x1, y1: *y1, x2: *x2, y2: *y2, stops: Vec::with_capacity(stops.len()) },
            FillStyle::RadialGradient { cx, cy, r, stops } => FillStyle::RadialGradient { cx: *cx, cy: *cy, r: *r, stops: Vec::with_capacity(stops.len()) },
        }
    }

    fn step(&mut self, source_root: &DrawLayerNode, digest: &mut store::ArtifactStoreInitializationDigest, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        let path = &self.path[..self.depth];
        let source = Self::source_at(source_root, path).ok_or("draw-store.initializer-source-path")?;
        let target = Self::target_at_mut(self.value.as_mut().ok_or("draw-store.initializer-layer-target")?, path).ok_or("draw-store.initializer-target-path")?;
        let frame = self.frames[self.depth];
        let (source_base, target_base) = Self::bases(source, target);
        let observed: &[u8] = match frame.phase {
            0 => {
                target_base.id = Self::clone_string(&source_base.id)?;
                source_base.id.as_bytes()
            }
            1 => {
                target_base.name = Self::clone_string(&source_base.name)?;
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
                let source_stops = match source_base.attributes.fill.as_ref() {
                    Some(FillStyle::LinearGradient { stops, .. }) | Some(FillStyle::RadialGradient { stops, .. }) => stops,
                    _ => &[],
                };
                if let Some(stop) = source_stops.get(frame.string) {
                    let target_stops = match target_base.attributes.fill.as_mut() {
                        Some(FillStyle::LinearGradient { stops, .. }) | Some(FillStyle::RadialGradient { stops, .. }) => stops,
                        _ => return Err("draw-store.initializer-fill-target"),
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
                    target_base.attributes.stroke.as_mut().and_then(|stroke| stroke.dash.as_mut()).ok_or("draw-store.initializer-dash-target")?.push(*value);
                    self.frames[self.depth].string += 1;
                    cx.consume_fuel(1);
                    return Ok(false);
                }
                self.frames[self.depth].string = 0;
                &[]
            }
            9 => match (source, target) {
                (DrawLayerNode::Shape(source), DrawLayerNode::Shape(target)) => {
                    target.shape_kind = Self::clone_string(&source.shape_kind)?;
                    source.shape_kind.as_bytes()
                }
                (DrawLayerNode::Text(source), DrawLayerNode::Text(target)) => {
                    target.content = Self::clone_string(&source.content)?;
                    source.content.as_bytes()
                }
                (DrawLayerNode::Image(source), DrawLayerNode::Image(target)) => {
                    target.image_key = Self::clone_string(&source.image_key)?;
                    source.image_key.as_bytes()
                }
                (DrawLayerNode::Boolean(source), DrawLayerNode::Boolean(target)) => {
                    target.operation = Self::clone_string(&source.operation)?;
                    source.operation.as_bytes()
                }
                (DrawLayerNode::Trace(source), DrawLayerNode::Trace(target)) => {
                    target.source_key = Self::clone_string(&source.source_key)?;
                    source.source_key.as_bytes()
                }
                _ => &[],
            },
            10 => {
                let index = frame.string;
                match (source, target) {
                    (DrawLayerNode::Shape(source), DrawLayerNode::Shape(target)) => {
                        if let Some(point) = source.polygon.as_ref().and_then(|polygon| polygon.points.get(index)) {
                            target.polygon.as_mut().ok_or("draw-store.initializer-polygon-target")?.points.push(*point);
                            self.frames[self.depth].string += 1;
                            cx.consume_fuel(1);
                            return Ok(false);
                        }
                    }
                    (DrawLayerNode::Path(source), DrawLayerNode::Path(target)) => {
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
                    (DrawLayerNode::Boolean(source), DrawLayerNode::Boolean(target)) => {
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
                if let (DrawLayerNode::Group(source), DrawLayerNode::Group(target)) = (source, target) {
                    if let Some(child) = source.children.get(frame.child) {
                        if self.depth + 1 >= DRAW_MAXIMUM_LAYER_DEPTH {
                            return Err("draw-store.initializer-depth-capacity");
                        }
                        target.children.push(Self::skeleton(child)?);
                        self.path[self.depth] = frame.child;
                        self.frames[self.depth].child += 1;
                        self.depth += 1;
                        self.frames[self.depth] = DrawTraversalFrame::EMPTY;
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

    fn take(&mut self) -> Option<DrawLayerNode> {
        self.terminal.then(|| self.value.take()).flatten()
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.retirement.is_none() {
            if let Some(value) = self.value.take() {
                *self.retirement = Some(Box::new(DrawOwnedRetirement::new(DrawRetirementOwner::Layer(value))));
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            return Ok(store::SnapshotRetirementStep::Complete);
        }
        let retirement = self.retirement.as_mut().expect("Draw layer clone retirement remains exact");
        match retirement.close_step(1, maximum_bytes)? {
            store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                drop(self.retirement.take());
                Ok(store::SnapshotRetirementStep::Complete)
            }
            store::SnapshotRetirementStep::Complete => Err("Draw layer clone retirement reported false terminal".into()),
            step => Ok(step),
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.value.is_none() && self.retirement.is_none()
    }
}

impl Drop for DrawLayerCloneAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Draw layer clone reached Drop before exact handoff or cursor retirement");
    }
}

struct DrawSnapshotCloneAuthority {
    value: std::mem::ManuallyDrop<Option<DrawSnapshot>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    layer: std::mem::ManuallyDrop<Option<Box<DrawLayerCloneAuthority>>>,
    pending_asset: std::mem::ManuallyDrop<Option<(String, DrawImageAsset)>>,
    bounds: DrawSnapshotBoundsAuthority,
    phase: u8,
    index: usize,
    field: u8,
    terminal: bool,
}

impl DrawSnapshotCloneAuthority {
    fn new() -> Self {
        Self {
            value: std::mem::ManuallyDrop::new(Some(DrawSnapshot { schema: String::new(), id: String::new(), title: None, layers: Vec::new(), assets: std::collections::BTreeMap::new(), artboard: None })),
            retirement: std::mem::ManuallyDrop::new(None),
            layer: std::mem::ManuallyDrop::new(None),
            pending_asset: std::mem::ManuallyDrop::new(None),
            bounds: DrawSnapshotBoundsAuthority::new(),
            phase: 0,
            index: 0,
            field: 0,
            terminal: false,
        }
    }

    fn clone_string(source: &str) -> Result<String, &'static str> {
        if source.len() > DRAW_OWNED_FIELD_BYTES {
            return Err("draw-store.initializer-field-too-large");
        }
        let mut value = String::new();
        value.try_reserve_exact(source.len()).map_err(|_| "draw-store.initializer-string-admission")?;
        value.push_str(source);
        Ok(value)
    }

    fn step(&mut self, source: &DrawSnapshot, digest: &mut store::ArtifactStoreInitializationDigest, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.phase == 0 {
            if self.bounds.step(source, cx)? {
                self.phase = 1;
            }
            return Ok(false);
        }
        let target = self.value.as_mut().ok_or("draw-store.initializer-clone-target")?;
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
                    if layer.step(source.layers.get(self.index).ok_or("draw-store.initializer-layer-source")?, digest, cx)? {
                        let value = layer.take().ok_or("draw-store.initializer-layer-handoff")?;
                        drop(self.layer.take());
                        target.layers.push(value);
                        self.index += 1;
                    }
                    return Ok(false);
                }
                if let Some(layer_source) = source.layers.get(self.index) {
                    if self.index == 0 {
                        target.layers.try_reserve_exact(source.layers.len()).map_err(|_| "draw-store.initializer-layer-admission")?;
                    }
                    *self.layer = Some(Box::new(DrawLayerCloneAuthority::new(layer_source)?));
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
                        Some((key, _)) => source.assets.range((Excluded(key), Unbounded)).next(),
                        None => source.assets.iter().next(),
                    };
                    let Some((key, value)) = next else {
                        self.phase = 6;
                        self.field = 0;
                        return Ok(false);
                    };
                    *self.pending_asset = Some((Self::clone_string(key)?, DrawImageAsset { mime: String::new(), data: String::new(), width: value.width, height: value.height }));
                    self.field = 0;
                    digest.observe(key.as_bytes());
                    cx.consume_fuel(key.len().max(1) as u64);
                    return Ok(false);
                }
                let (key, pending) = self.pending_asset.as_mut().expect("Draw pending asset remains exact");
                let source = source.assets.get(key).ok_or("draw-store.initializer-asset-source")?;
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
                        let (key, value) = self.pending_asset.take().expect("Draw pending asset handoff remains exact");
                        if target.assets.insert(key, value).is_some() {
                            return Err("draw-store.initializer-duplicate-asset");
                        }
                        self.field = 0;
                        cx.consume_fuel(1);
                        return Ok(false);
                    }
                }
            }
            6 => {
                target.artboard = source.artboard.as_ref().map(|value| crate::artifacts::draw::DrawArtboard { width: value.width, height: value.height });
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

    fn take_value(&mut self) -> Option<DrawSnapshot> {
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
            return match retirement.close_step(1, maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                    drop(self.retirement.take());
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                store::SnapshotRetirementStep::Complete => Err("Draw clone retirement reported false terminal".into()),
                step => Ok(step),
            };
        }
        if let Some(layer) = self.layer.as_mut() {
            return match layer.close_step(1, maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if layer.terminal_is_empty() => {
                    drop(self.layer.take());
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                store::SnapshotRetirementStep::Complete => Err("Draw active layer clone reported false terminal".into()),
                step => Ok(step),
            };
        }
        if let Some((key, value)) = self.pending_asset.take() {
            *self.retirement = Some(Box::new(DrawOwnedRetirement::new(DrawRetirementOwner::AssetEntry { key, value: Some(value) })));
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(value) = self.value.take() {
            *self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&DrawSnapshotRetirementFactory, value));
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.terminal = true;
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal && self.value.is_none() && self.retirement.is_none() && self.layer.is_none() && self.pending_asset.is_none()
    }
}

impl Drop for DrawSnapshotCloneAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Draw snapshot clone reached Drop before exact handoff or cursor retirement");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct DrawLayerAddress {
    length: usize,
    indices: [usize; DRAW_MAXIMUM_LAYER_DEPTH],
}

impl DrawLayerAddress {
    fn parent(self) -> Option<Self> {
        (self.length > 1).then(|| Self { length: self.length - 1, indices: self.indices })
    }

    fn index(self) -> usize {
        self.indices[self.length - 1]
    }
}

struct DrawLayerLocator {
    root: usize,
    depth: usize,
    path: [usize; DRAW_MAXIMUM_LAYER_DEPTH],
    frames: [DrawTraversalFrame; DRAW_MAXIMUM_LAYER_DEPTH],
    found: Option<DrawLayerAddress>,
    terminal: bool,
}

impl DrawLayerLocator {
    fn new() -> Self {
        Self { root: 0, depth: 0, path: [0; DRAW_MAXIMUM_LAYER_DEPTH], frames: [DrawTraversalFrame::EMPTY; DRAW_MAXIMUM_LAYER_DEPTH], found: None, terminal: false }
    }

    fn node_at<'a>(snapshot: &'a DrawSnapshot, address: DrawLayerAddress) -> Option<&'a DrawLayerNode> {
        let mut value = snapshot.layers.get(address.indices[0])?;
        for index in &address.indices[1..address.length] {
            let DrawLayerNode::Group(group) = value else { return None };
            value = group.children.get(*index)?;
        }
        Some(value)
    }

    fn node_at_mut<'a>(snapshot: &'a mut DrawSnapshot, address: DrawLayerAddress) -> Option<&'a mut DrawLayerNode> {
        fn descend<'a>(value: &'a mut DrawLayerNode, path: &[usize]) -> Option<&'a mut DrawLayerNode> {
            let Some((head, tail)) = path.split_first() else { return Some(value) };
            let DrawLayerNode::Group(group) = value else { return None };
            descend(group.children.get_mut(*head)?, tail)
        }
        let value = snapshot.layers.get_mut(address.indices[0])?;
        descend(value, &address.indices[1..address.length])
    }

    fn container_mut<'a>(snapshot: &'a mut DrawSnapshot, parent: Option<DrawLayerAddress>) -> Option<&'a mut Vec<DrawLayerNode>> {
        match parent {
            None => Some(&mut snapshot.layers),
            Some(address) => match Self::node_at_mut(snapshot, address)? {
                DrawLayerNode::Group(group) => Some(&mut group.children),
                _ => None,
            },
        }
    }

    fn step(&mut self, snapshot: &DrawSnapshot, target: &str, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        let Some(root) = snapshot.layers.get(self.root) else {
            self.terminal = true;
            return Ok(true);
        };
        let node = DrawSnapshotBoundsAuthority::layer_at(root, &self.path[..self.depth]).ok_or("draw-store.mutation-locator-path")?;
        if self.frames[self.depth].phase == 0 {
            self.frames[self.depth].phase = 1;
            if crate::artifacts::draw::schema::layer_id(node) == target {
                let mut indices = [0; DRAW_MAXIMUM_LAYER_DEPTH];
                indices[0] = self.root;
                if self.depth > 0 {
                    indices[1..self.depth + 1].copy_from_slice(&self.path[..self.depth]);
                }
                self.found = Some(DrawLayerAddress { length: self.depth + 1, indices });
                self.terminal = true;
            }
            cx.consume_fuel(1);
            return Ok(self.terminal);
        }
        if let DrawLayerNode::Group(group) = node {
            let child = self.frames[self.depth].child;
            if child < group.children.len() {
                if self.depth + 1 >= DRAW_MAXIMUM_LAYER_DEPTH {
                    return Err("draw-store.mutation-locator-depth");
                }
                self.frames[self.depth].child += 1;
                self.path[self.depth] = child;
                self.depth += 1;
                self.frames[self.depth] = DrawTraversalFrame::EMPTY;
                cx.consume_fuel(1);
                return Ok(false);
            }
        }
        if self.depth == 0 {
            self.root += 1;
            self.frames[0] = DrawTraversalFrame::EMPTY;
        } else {
            self.depth -= 1;
        }
        cx.consume_fuel(1);
        Ok(false)
    }

    fn found(&self) -> Option<DrawLayerAddress> {
        self.found
    }
}

struct DrawContainerRebuildAuthority {
    source: std::mem::ManuallyDrop<Option<Vec<DrawLayerNode>>>,
    reverse: std::mem::ManuallyDrop<Option<Vec<DrawLayerNode>>>,
    output: std::mem::ManuallyDrop<Option<Vec<DrawLayerNode>>>,
    pending: std::mem::ManuallyDrop<Option<DrawLayerNode>>,
    removed: std::mem::ManuallyDrop<Option<DrawLayerNode>>,
    retirement: std::mem::ManuallyDrop<Option<Box<DrawOwnedRetirement>>>,
    remove_index: Option<usize>,
    insert_index: Option<usize>,
    original_index: usize,
    phase: u8,
    terminal: bool,
}

struct DrawContainerRebuildRejected {
    source: Vec<DrawLayerNode>,
    pending: Option<DrawLayerNode>,
}

impl DrawContainerRebuildAuthority {
    fn new(source: Vec<DrawLayerNode>, remove_index: Option<usize>, insert_index: Option<usize>, pending: Option<DrawLayerNode>) -> Result<Self, DrawContainerRebuildRejected> {
        let extra = usize::from(pending.is_some());
        let Some(output_capacity) = source.len().saturating_sub(usize::from(remove_index.is_some())).checked_add(extra) else {
            return Err(DrawContainerRebuildRejected { source, pending });
        };
        let mut reverse = Vec::new();
        if reverse.try_reserve_exact(source.len()).is_err() {
            return Err(DrawContainerRebuildRejected { source, pending });
        }
        let mut output = Vec::new();
        if output.try_reserve_exact(output_capacity).is_err() {
            return Err(DrawContainerRebuildRejected { source, pending });
        }
        Ok(Self {
            source: std::mem::ManuallyDrop::new(Some(source)),
            reverse: std::mem::ManuallyDrop::new(Some(reverse)),
            output: std::mem::ManuallyDrop::new(Some(output)),
            pending: std::mem::ManuallyDrop::new(pending),
            removed: std::mem::ManuallyDrop::new(None),
            retirement: std::mem::ManuallyDrop::new(None),
            remove_index,
            insert_index,
            original_index: 0,
            phase: 0,
            terminal: false,
        })
    }

    fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        if self.phase == 0 {
            if let Some(value) = self.source.as_mut().ok_or("draw-store.container-source")?.pop() {
                self.reverse.as_mut().ok_or("draw-store.container-reverse")?.push(value);
                cx.consume_fuel(1);
                return Ok(false);
            }
            drop(self.source.take());
            self.phase = 1;
            return Ok(false);
        }
        if self.pending.is_some() && self.insert_index.is_some_and(|index| index.min(self.reverse.as_ref().map_or(0, Vec::len) + self.original_index) == self.output.as_ref().map_or(0, Vec::len)) {
            self.output.as_mut().ok_or("draw-store.container-output")?.push(self.pending.take().expect("Draw insertion owner remains retained"));
            cx.consume_fuel(1);
            return Ok(false);
        }
        if let Some(value) = self.reverse.as_mut().ok_or("draw-store.container-reverse")?.pop() {
            if self.remove_index == Some(self.original_index) {
                if self.removed.replace(value).is_some() {
                    return Err("draw-store.container-duplicate-removal");
                }
            } else {
                self.output.as_mut().ok_or("draw-store.container-output")?.push(value);
            }
            self.original_index += 1;
            cx.consume_fuel(1);
            return Ok(false);
        }
        drop(self.reverse.take());
        if let Some(value) = self.pending.take() {
            self.output.as_mut().ok_or("draw-store.container-output")?.push(value);
            cx.consume_fuel(1);
            return Ok(false);
        }
        self.terminal = true;
        Ok(true)
    }

    fn take(&mut self) -> Option<(Vec<DrawLayerNode>, Option<DrawLayerNode>)> {
        self.terminal.then(|| (self.output.take().expect("Draw rebuilt container remains retained"), self.removed.take()))
    }

    fn close_step(&mut self, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if let Some(retirement) = self.retirement.as_mut() {
            return match retirement.close_step(1, maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                    drop(self.retirement.take());
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                store::SnapshotRetirementStep::Complete => Err("Draw container retirement reported false terminal".into()),
                step => Ok(step),
            };
        }
        let owner = if let Some(values) = self.source.take() {
            Some(DrawRetirementOwner::Layers(values))
        } else if let Some(values) = self.reverse.take() {
            Some(DrawRetirementOwner::Layers(values))
        } else if let Some(values) = self.output.take() {
            Some(DrawRetirementOwner::Layers(values))
        } else if let Some(value) = self.pending.take() {
            Some(DrawRetirementOwner::Layer(value))
        } else {
            self.removed.take().map(DrawRetirementOwner::Layer)
        };
        if let Some(owner) = owner {
            *self.retirement = Some(Box::new(DrawOwnedRetirement::new(owner)));
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.terminal = true;
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal && self.source.is_none() && self.reverse.is_none() && self.output.is_none() && self.pending.is_none() && self.removed.is_none() && self.retirement.is_none()
    }
}

impl Drop for DrawContainerRebuildAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Draw container rebuild reached Drop before exact handoff or cursor retirement");
    }
}

struct DrawFillCloneAuthority {
    value: std::mem::ManuallyDrop<Option<FillStyle>>,
    retirement: std::mem::ManuallyDrop<Option<Box<DrawOwnedRetirement>>>,
    index: usize,
    terminal: bool,
}

impl DrawFillCloneAuthority {
    fn new(source: &FillStyle) -> Result<Self, &'static str> {
        let value = match source {
            FillStyle::Solid { color } => FillStyle::Solid { color: *color },
            FillStyle::LinearGradient { x1, y1, x2, y2, stops } => {
                if stops.len() > DRAW_MAXIMUM_NESTED_ITEMS {
                    return Err("draw-store.fill-stop-capacity");
                }
                let mut target = Vec::new();
                target.try_reserve_exact(stops.len()).map_err(|_| "draw-store.fill-stop-admission")?;
                FillStyle::LinearGradient { x1: *x1, y1: *y1, x2: *x2, y2: *y2, stops: target }
            }
            FillStyle::RadialGradient { cx, cy, r, stops } => {
                if stops.len() > DRAW_MAXIMUM_NESTED_ITEMS {
                    return Err("draw-store.fill-stop-capacity");
                }
                let mut target = Vec::new();
                target.try_reserve_exact(stops.len()).map_err(|_| "draw-store.fill-stop-admission")?;
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
            let target_stops = match self.value.as_mut().ok_or("draw-store.fill-target")? {
                FillStyle::LinearGradient { stops, .. } | FillStyle::RadialGradient { stops, .. } => stops,
                FillStyle::Solid { .. } => return Err("draw-store.fill-variant"),
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
            return match retirement.close_step(1, maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                    drop(self.retirement.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
                store::SnapshotRetirementStep::Complete => Err("Draw fill clone reported false terminal".into()),
                step => Ok(step),
            };
        }
        if let Some(value) = self.value.take() {
            *self.retirement = Some(Box::new(DrawOwnedRetirement::new(DrawRetirementOwner::Fill(value))));
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.terminal = true;
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal && self.value.is_none() && self.retirement.is_none()
    }
}

impl Drop for DrawFillCloneAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Draw fill clone reached Drop before exact handoff or retirement");
    }
}

struct DrawStrokeCloneAuthority {
    value: std::mem::ManuallyDrop<Option<StrokeStyle>>,
    retirement: std::mem::ManuallyDrop<Option<Box<DrawOwnedRetirement>>>,
    phase: u8,
    index: usize,
    terminal: bool,
}

impl DrawStrokeCloneAuthority {
    fn new(source: &StrokeStyle) -> Result<Self, &'static str> {
        let dash = match source.dash.as_ref() {
            Some(values) if values.len() <= DRAW_MAXIMUM_NESTED_ITEMS => {
                let mut target = Vec::new();
                target.try_reserve_exact(values.len()).map_err(|_| "draw-store.stroke-dash-admission")?;
                Some(target)
            }
            Some(_) => return Err("draw-store.stroke-dash-capacity"),
            None => None,
        };
        Ok(Self { value: std::mem::ManuallyDrop::new(Some(StrokeStyle { color: source.color, width: source.width, cap: String::new(), join: String::new(), dash })), retirement: std::mem::ManuallyDrop::new(None), phase: 0, index: 0, terminal: false })
    }

    fn step(&mut self, source: &StrokeStyle, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        let target = self.value.as_mut().ok_or("draw-store.stroke-target")?;
        match self.phase {
            0 => {
                target.cap = DrawSnapshotCloneAuthority::clone_string(&source.cap)?;
                self.phase = 1;
                cx.consume_fuel(source.cap.len().max(1) as u64);
            }
            1 => {
                target.join = DrawSnapshotCloneAuthority::clone_string(&source.join)?;
                self.phase = 2;
                cx.consume_fuel(source.join.len().max(1) as u64);
            }
            2 => {
                if let Some(value) = source.dash.as_ref().and_then(|values| values.get(self.index)) {
                    target.dash.as_mut().ok_or("draw-store.stroke-dash-target")?.push(*value);
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
            return match retirement.close_step(1, maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                    drop(self.retirement.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
                store::SnapshotRetirementStep::Complete => Err("Draw stroke clone reported false terminal".into()),
                step => Ok(step),
            };
        }
        if let Some(value) = self.value.take() {
            *self.retirement = Some(Box::new(DrawOwnedRetirement::new(DrawRetirementOwner::Stroke(value))));
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.terminal = true;
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal && self.value.is_none() && self.retirement.is_none()
    }
}

impl Drop for DrawStrokeCloneAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Draw stroke clone reached Drop before exact handoff or retirement");
    }
}

struct DrawMutationDigestAuthority {
    layer: std::mem::ManuallyDrop<Option<Box<DrawLayerCloneAuthority>>>,
    phase: u8,
    index: usize,
    terminal: bool,
}

impl DrawMutationDigestAuthority {
    fn new() -> Self {
        Self { layer: std::mem::ManuallyDrop::new(None), phase: 0, index: 0, terminal: false }
    }

    fn field<'a>(&self, mutation: &'a DrawMutation) -> (&'static [u8], &'a str, Option<&'a str>) {
        match mutation {
            DrawMutation::SetLayerVisible(value) => (b"set-layer-visible", &value.layer_id, None),
            DrawMutation::SetLayerLocked(value) => (b"set-layer-locked", &value.layer_id, None),
            DrawMutation::SetLayerOpacity(value) => (b"set-layer-opacity", &value.layer_id, None),
            DrawMutation::SetLayerBlendMode(value) => (b"set-layer-blend-mode", &value.layer_id, Some(&value.blend_mode)),
            DrawMutation::RenameLayer(value) => (b"rename-layer", &value.layer_id, Some(&value.new_name)),
            DrawMutation::UpdateLayerTransform(value) => (b"update-layer-transform", &value.layer_id, None),
            DrawMutation::ReplaceLayerFill(value) => (b"replace-layer-fill", &value.layer_id, None),
            DrawMutation::ReplaceLayerStroke(value) => (b"replace-layer-stroke", &value.layer_id, None),
            DrawMutation::SetLayerBooleanOperation(value) => (b"set-layer-boolean-operation", &value.layer_id, Some(&value.boolean_operation)),
            DrawMutation::UpdateLayerTraceParams(value) => (b"update-layer-trace-params", &value.layer_id, None),
            DrawMutation::CreateLayer(value) => (b"create-layer", value.parent_id.as_deref().unwrap_or_default(), None),
            DrawMutation::DuplicateLayer(value) => (b"duplicate-layer", &value.layer_id, None),
            DrawMutation::DeleteLayer(value) => (b"delete-layer", &value.layer_id, None),
            DrawMutation::ReorderLayer(value) => (b"reorder-layer", &value.layer_id, value.parent_id.as_deref()),
        }
    }

    fn observe_field(digest: &mut store::ArtifactStoreInitializationDigest, value: &[u8], cx: &mut semio_framework_job::StepContext<'_>) -> Result<(), &'static str> {
        if value.len() > DRAW_OWNED_FIELD_BYTES {
            return Err("draw-store.mutation-field-capacity");
        }
        digest.observe(value);
        cx.consume_fuel(value.len().max(1) as u64);
        Ok(())
    }

    fn step(&mut self, mutation: &DrawMutation, digest: &mut store::ArtifactStoreInitializationDigest, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        let (tag, first, second) = self.field(mutation);
        match self.phase {
            0 => Self::observe_field(digest, tag, cx)?,
            1 => Self::observe_field(digest, first.as_bytes(), cx)?,
            2 if second.is_some() => Self::observe_field(digest, second.expect("Draw second mutation field remains exact").as_bytes(), cx)?,
            2 => {}
            3 => {
                match mutation {
                    DrawMutation::SetLayerVisible(value) => digest.observe(&[u8::from(value.visible)]),
                    DrawMutation::SetLayerLocked(value) => digest.observe(&[u8::from(value.locked)]),
                    DrawMutation::SetLayerOpacity(value) => digest.observe(&value.opacity.to_be_bytes()),
                    DrawMutation::UpdateLayerTransform(value) => {
                        let fields = [value.transform.x, value.transform.y, value.transform.scale_x, value.transform.scale_y, value.transform.rotation];
                        let Some(field) = fields.get(self.index) else {
                            self.phase = 4;
                            return Ok(false);
                        };
                        self.index += 1;
                        digest.observe(&field.to_be_bytes());
                        cx.consume_fuel(1);
                        return Ok(false);
                    }
                    DrawMutation::UpdateLayerTraceParams(value) => {
                        let fields = [value.params.threshold, value.params.simplify_epsilon];
                        let Some(field) = fields.get(self.index) else {
                            self.phase = 4;
                            return Ok(false);
                        };
                        self.index += 1;
                        digest.observe(&field.to_be_bytes());
                        cx.consume_fuel(1);
                        return Ok(false);
                    }
                    DrawMutation::CreateLayer(value) => digest.observe(&value.index.unwrap_or(usize::MAX).to_be_bytes()),
                    DrawMutation::ReorderLayer(value) => digest.observe(&value.index.to_be_bytes()),
                    _ => digest.observe(&[]),
                }
                cx.consume_fuel(1);
            }
            4 => match mutation {
                DrawMutation::ReplaceLayerFill(value) => {
                    let stops = match value.fill.as_ref() {
                        Some(FillStyle::LinearGradient { stops, .. }) | Some(FillStyle::RadialGradient { stops, .. }) => stops.as_slice(),
                        _ => &[],
                    };
                    if let Some(stop) = stops.get(self.index) {
                        digest.observe(&stop.offset.to_be_bytes());
                        for color in stop.color {
                            digest.observe(&color.to_be_bytes());
                        }
                        self.index += 1;
                        cx.consume_fuel(1);
                        return Ok(false);
                    }
                }
                DrawMutation::ReplaceLayerStroke(value) => {
                    if let Some(stroke) = value.stroke.as_ref() {
                        if self.index == 0 {
                            Self::observe_field(digest, stroke.cap.as_bytes(), cx)?;
                            self.index = 1;
                            return Ok(false);
                        }
                        if self.index == 1 {
                            Self::observe_field(digest, stroke.join.as_bytes(), cx)?;
                            self.index = 2;
                            return Ok(false);
                        }
                        if let Some(value) = stroke.dash.as_ref().and_then(|values| values.get(self.index - 2)) {
                            digest.observe(&value.to_be_bytes());
                            self.index += 1;
                            cx.consume_fuel(1);
                            return Ok(false);
                        }
                    }
                }
                DrawMutation::CreateLayer(value) => {
                    if self.layer.is_none() {
                        *self.layer = Some(Box::new(DrawLayerCloneAuthority::new(&value.layer)?));
                        cx.consume_fuel(1);
                        return Ok(false);
                    }
                    let layer = self.layer.as_mut().expect("Draw mutation layer digest remains retained");
                    if !layer.step(&value.layer, digest, cx)? {
                        return Ok(false);
                    }
                    return match layer.close_step(1, DRAW_OWNED_FIELD_BYTES).map_err(|_| "draw-store.mutation-layer-close")? {
                        store::SnapshotRetirementStep::Complete if layer.terminal_is_empty() => {
                            drop(self.layer.take());
                            self.phase = 5;
                            Ok(false)
                        }
                        store::SnapshotRetirementStep::Complete => Err("draw-store.mutation-layer-false-terminal"),
                        _ => Ok(false),
                    };
                }
                _ => {}
            },
            _ => {
                self.terminal = true;
                return Ok(true);
            }
        }
        self.phase += 1;
        self.index = 0;
        Ok(false)
    }

    fn close_step(&mut self, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if let Some(layer) = self.layer.as_mut() {
            return match layer.close_step(1, maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if layer.terminal_is_empty() => {
                    drop(self.layer.take());
                    self.terminal = true;
                    Ok(store::SnapshotRetirementStep::Complete)
                }
                store::SnapshotRetirementStep::Complete => Err("Draw mutation digest layer reported false terminal".into()),
                step => Ok(step),
            };
        }
        self.terminal = true;
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal && self.layer.is_none()
    }
}

impl Drop for DrawMutationDigestAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Draw mutation digest reached Drop before exact terminal close");
    }
}

struct DrawDuplicateRewriteAuthority {
    depth: usize,
    path: [usize; DRAW_MAXIMUM_LAYER_DEPTH],
    frames: [DrawTraversalFrame; DRAW_MAXIMUM_LAYER_DEPTH],
    terminal: bool,
}

impl DrawDuplicateRewriteAuthority {
    fn new() -> Self {
        Self { depth: 0, path: [0; DRAW_MAXIMUM_LAYER_DEPTH], frames: [DrawTraversalFrame::EMPTY; DRAW_MAXIMUM_LAYER_DEPTH], terminal: false }
    }

    fn step(&mut self, root: &mut DrawLayerNode, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        let node = DrawLayerCloneAuthority::target_at_mut(root, &self.path[..self.depth]).ok_or("draw-store.duplicate-path")?;
        if self.frames[self.depth].phase == 0 {
            let prefix = match node {
                DrawLayerNode::Shape(_) => "shape",
                DrawLayerNode::Path(_) => "path",
                DrawLayerNode::Text(_) => "text",
                DrawLayerNode::Image(_) => "image",
                DrawLayerNode::Group(_) => "group",
                DrawLayerNode::Boolean(_) => "boolean",
                DrawLayerNode::Trace(_) => "trace",
            };
            let base = crate::artifacts::draw::schema::layer_base_mut(node);
            let suffix = if self.depth == 0 { " copy" } else { "" };
            let bytes = base.id.len().checked_add(base.name.len()).ok_or("draw-store.duplicate-byte-overflow")?;
            if bytes > DRAW_OWNED_FIELD_BYTES || base.name.len().checked_add(suffix.len()).is_none_or(|length| length > DRAW_OWNED_FIELD_BYTES) {
                return Err("draw-store.duplicate-field-capacity");
            }
            let mut material = Vec::new();
            material.try_reserve_exact(bytes).map_err(|_| "draw-store.duplicate-material-admission")?;
            material.extend_from_slice(base.id.as_bytes());
            material.extend_from_slice(base.name.as_bytes());
            base.id = crate::artifacts::draw::schema::create_draw_id(prefix, &material);
            base.name.try_reserve_exact(suffix.len()).map_err(|_| "draw-store.duplicate-name-admission")?;
            base.name.push_str(suffix);
            self.frames[self.depth].phase = 1;
            cx.consume_fuel(bytes.max(1) as u64);
            return Ok(false);
        }
        if let DrawLayerNode::Group(group) = node {
            let child = self.frames[self.depth].child;
            if child < group.children.len() {
                if self.depth + 1 >= DRAW_MAXIMUM_LAYER_DEPTH {
                    return Err("draw-store.duplicate-depth-capacity");
                }
                self.frames[self.depth].child += 1;
                self.path[self.depth] = child;
                self.depth += 1;
                self.frames[self.depth] = DrawTraversalFrame::EMPTY;
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
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DrawMutationCandidatePhase {
    Clone,
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

struct DrawMutationCandidateAuthority {
    candidate: std::mem::ManuallyDrop<Option<DrawSnapshot>>,
    clone: std::mem::ManuallyDrop<Option<DrawSnapshotCloneAuthority>>,
    clone_digest: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationDigest>>,
    locator: Option<DrawLayerLocator>,
    primary: Option<DrawLayerAddress>,
    secondary: Option<DrawLayerAddress>,
    layer_clone: std::mem::ManuallyDrop<Option<Box<DrawLayerCloneAuthority>>>,
    fill_clone: std::mem::ManuallyDrop<Option<DrawFillCloneAuthority>>,
    stroke_clone: std::mem::ManuallyDrop<Option<DrawStrokeCloneAuthority>>,
    duplicate_rewrite: Option<DrawDuplicateRewriteAuthority>,
    rebuild: std::mem::ManuallyDrop<Option<DrawContainerRebuildAuthority>>,
    pending_layer: std::mem::ManuallyDrop<Option<DrawLayerNode>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    phase: DrawMutationCandidatePhase,
    terminal: bool,
    fault: Option<&'static str>,
}

impl DrawMutationCandidateAuthority {
    fn new() -> Self {
        Self {
            candidate: std::mem::ManuallyDrop::new(None),
            clone: std::mem::ManuallyDrop::new(Some(DrawSnapshotCloneAuthority::new())),
            clone_digest: std::mem::ManuallyDrop::new(Some(store::ArtifactStoreInitializationDigest::new(b"draw.mutation-candidate"))),
            locator: None,
            primary: None,
            secondary: None,
            layer_clone: std::mem::ManuallyDrop::new(None),
            fill_clone: std::mem::ManuallyDrop::new(None),
            stroke_clone: std::mem::ManuallyDrop::new(None),
            duplicate_rewrite: None,
            rebuild: std::mem::ManuallyDrop::new(None),
            pending_layer: std::mem::ManuallyDrop::new(None),
            retirement: std::mem::ManuallyDrop::new(None),
            phase: DrawMutationCandidatePhase::Clone,
            terminal: false,
            fault: None,
        }
    }

    fn target(mutation: &DrawMutation) -> &str {
        match mutation {
            DrawMutation::SetLayerVisible(value) => &value.layer_id,
            DrawMutation::SetLayerLocked(value) => &value.layer_id,
            DrawMutation::SetLayerOpacity(value) => &value.layer_id,
            DrawMutation::SetLayerBlendMode(value) => &value.layer_id,
            DrawMutation::RenameLayer(value) => &value.layer_id,
            DrawMutation::UpdateLayerTransform(value) => &value.layer_id,
            DrawMutation::ReplaceLayerFill(value) => &value.layer_id,
            DrawMutation::ReplaceLayerStroke(value) => &value.layer_id,
            DrawMutation::SetLayerBooleanOperation(value) => &value.layer_id,
            DrawMutation::UpdateLayerTraceParams(value) => &value.layer_id,
            DrawMutation::CreateLayer(value) => crate::artifacts::draw::schema::layer_id(&value.layer),
            DrawMutation::DuplicateLayer(value) => &value.layer_id,
            DrawMutation::DeleteLayer(value) => &value.layer_id,
            DrawMutation::ReorderLayer(value) => &value.layer_id,
        }
    }

    fn parent(mutation: &DrawMutation) -> Option<&str> {
        match mutation {
            DrawMutation::CreateLayer(value) => value.parent_id.as_deref(),
            DrawMutation::ReorderLayer(value) => value.parent_id.as_deref(),
            _ => None,
        }
    }

    fn fail(&mut self, fault: &'static str) -> Result<bool, &'static str> {
        self.fault = Some(fault);
        self.phase = DrawMutationCandidatePhase::Fault;
        Err(fault)
    }

    fn start_rebuild(&mut self, parent: Option<DrawLayerAddress>, remove_index: Option<usize>, insert_index: Option<usize>) -> Result<(), &'static str> {
        let candidate = self.candidate.as_mut().ok_or("draw-store.mutation-candidate-missing")?;
        let container = DrawLayerLocator::container_mut(candidate, parent).ok_or("draw-store.mutation-container-missing")?;
        let source = std::mem::take(container);
        let pending = self.pending_layer.take();
        match DrawContainerRebuildAuthority::new(source, remove_index, insert_index, pending) {
            Ok(rebuild) => {
                *self.rebuild = Some(rebuild);
                Ok(())
            }
            Err(rejected) => {
                *container = rejected.source;
                *self.pending_layer = rejected.pending;
                Err("draw-store.mutation-container-admission")
            }
        }
    }

    fn finish_rebuild(&mut self, parent: Option<DrawLayerAddress>) -> Result<Option<DrawLayerNode>, &'static str> {
        let (output, removed) = self.rebuild.as_mut().ok_or("draw-store.mutation-rebuild-missing")?.take().ok_or("draw-store.mutation-rebuild-false-terminal")?;
        let candidate = self.candidate.as_mut().ok_or("draw-store.mutation-candidate-missing")?;
        *DrawLayerLocator::container_mut(candidate, parent).ok_or("draw-store.mutation-container-lost")? = output;
        let mut rebuild = self.rebuild.take().expect("Draw completed rebuild remains exact");
        rebuild.terminal = true;
        drop(rebuild);
        Ok(removed)
    }

    fn step(&mut self, source: &DrawSnapshot, mutation: &DrawMutation, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        match self.phase {
            DrawMutationCandidatePhase::Clone => {
                let clone = self.clone.as_mut().ok_or("draw-store.mutation-clone-missing")?;
                if clone.step(source, self.clone_digest.as_mut().ok_or("draw-store.mutation-clone-digest")?, cx)? {
                    *self.candidate = clone.take_value();
                    drop(self.clone.take());
                    drop(self.clone_digest.take());
                    self.locator = Some(DrawLayerLocator::new());
                    self.phase = DrawMutationCandidatePhase::LocatePrimary;
                }
                Ok(false)
            }
            DrawMutationCandidatePhase::LocatePrimary => {
                let candidate = self.candidate.as_ref().ok_or("draw-store.mutation-candidate-missing")?;
                let locator = self.locator.as_mut().ok_or("draw-store.mutation-locator-missing")?;
                if !locator.step(candidate, Self::target(mutation), cx)? {
                    return Ok(false);
                }
                self.primary = locator.found();
                self.locator = None;
                if matches!(mutation, DrawMutation::CreateLayer(_)) {
                    if self.primary.is_some() {
                        return Err("draw-store.mutation-duplicate-layer");
                    }
                    if Self::parent(mutation).is_some() {
                        self.locator = Some(DrawLayerLocator::new());
                        self.phase = DrawMutationCandidatePhase::LocateSecondary;
                    } else {
                        self.phase = DrawMutationCandidatePhase::PrepareOwnedValue;
                    }
                } else if self.primary.is_none() {
                    return Err("draw-store.mutation-target-missing");
                } else {
                    self.phase = DrawMutationCandidatePhase::PrepareOwnedValue;
                }
                Ok(false)
            }
            DrawMutationCandidatePhase::LocateSecondary => {
                let target = Self::parent(mutation).ok_or("draw-store.mutation-parent-missing")?;
                let candidate = self.candidate.as_ref().ok_or("draw-store.mutation-candidate-missing")?;
                let locator = self.locator.as_mut().ok_or("draw-store.mutation-parent-locator")?;
                if !locator.step(candidate, target, cx)? {
                    return Ok(false);
                }
                self.secondary = locator.found();
                self.locator = None;
                let Some(address) = self.secondary else { return Err("draw-store.mutation-parent-not-found") };
                if !matches!(DrawLayerLocator::node_at(candidate, address), Some(DrawLayerNode::Group(_))) {
                    return Err("draw-store.mutation-parent-not-group");
                }
                self.phase = DrawMutationCandidatePhase::PrepareOwnedValue;
                Ok(false)
            }
            DrawMutationCandidatePhase::PrepareOwnedValue => {
                match mutation {
                    DrawMutation::CreateLayer(value) => {
                        if self.layer_clone.is_none() {
                            *self.layer_clone = Some(Box::new(DrawLayerCloneAuthority::new(&value.layer)?));
                            cx.consume_fuel(1);
                            return Ok(false);
                        }
                        let clone = self.layer_clone.as_mut().expect("Draw create layer clone remains retained");
                        if !clone.step(&value.layer, self.clone_digest.get_or_insert_with(|| store::ArtifactStoreInitializationDigest::new(b"draw.create-layer")), cx)? {
                            return Ok(false);
                        }
                        *self.pending_layer = clone.take();
                        drop(self.layer_clone.take());
                    }
                    DrawMutation::DuplicateLayer(_) => {
                        let source = DrawLayerLocator::node_at(self.candidate.as_ref().ok_or("draw-store.mutation-candidate-missing")?, self.primary.ok_or("draw-store.mutation-primary-missing")?).ok_or("draw-store.mutation-duplicate-source")?;
                        if self.pending_layer.is_none() {
                            if self.layer_clone.is_none() {
                                *self.layer_clone = Some(Box::new(DrawLayerCloneAuthority::new(source)?));
                                cx.consume_fuel(1);
                                return Ok(false);
                            }
                            let clone = self.layer_clone.as_mut().expect("Draw duplicate layer clone remains retained");
                            if !clone.step(source, self.clone_digest.get_or_insert_with(|| store::ArtifactStoreInitializationDigest::new(b"draw.duplicate-layer")), cx)? {
                                return Ok(false);
                            }
                            *self.pending_layer = clone.take();
                            drop(self.layer_clone.take());
                            self.duplicate_rewrite = Some(DrawDuplicateRewriteAuthority::new());
                            return Ok(false);
                        }
                        if !self.duplicate_rewrite.as_mut().ok_or("draw-store.duplicate-rewrite-missing")?.step(self.pending_layer.as_mut().ok_or("draw-store.duplicate-owner-missing")?, cx)? {
                            return Ok(false);
                        }
                        self.duplicate_rewrite = None;
                    }
                    DrawMutation::ReplaceLayerFill(value) => {
                        if let Some(source) = value.fill.as_ref() {
                            if self.fill_clone.is_none() {
                                *self.fill_clone = Some(DrawFillCloneAuthority::new(source)?);
                                cx.consume_fuel(1);
                                return Ok(false);
                            }
                            if !self.fill_clone.as_mut().expect("Draw fill clone remains retained").step(source, cx)? {
                                return Ok(false);
                            }
                        }
                    }
                    DrawMutation::ReplaceLayerStroke(value) => {
                        if let Some(source) = value.stroke.as_ref() {
                            if self.stroke_clone.is_none() {
                                *self.stroke_clone = Some(DrawStrokeCloneAuthority::new(source)?);
                                cx.consume_fuel(1);
                                return Ok(false);
                            }
                            if !self.stroke_clone.as_mut().expect("Draw stroke clone remains retained").step(source, cx)? {
                                return Ok(false);
                            }
                        }
                    }
                    _ => {}
                }
                self.phase = DrawMutationCandidatePhase::Apply;
                Ok(false)
            }
            DrawMutationCandidatePhase::Apply => {
                match mutation {
                    DrawMutation::CreateLayer(value) => {
                        let parent = self.secondary;
                        let index = match value.index {
                            Some(index) => index,
                            None => DrawLayerLocator::container_mut(self.candidate.as_mut().ok_or("draw-store.mutation-candidate-missing")?, parent).map_or(0, |values| values.len()),
                        };
                        self.start_rebuild(parent, None, Some(index))?;
                        self.phase = DrawMutationCandidatePhase::RebuildDestination;
                        return Ok(false);
                    }
                    DrawMutation::DuplicateLayer(_) => {
                        let address = self.primary.ok_or("draw-store.mutation-primary-missing")?;
                        self.start_rebuild(address.parent(), None, Some(address.index() + 1))?;
                        self.phase = DrawMutationCandidatePhase::RebuildDestination;
                        return Ok(false);
                    }
                    DrawMutation::DeleteLayer(_) | DrawMutation::ReorderLayer(_) => {
                        let address = self.primary.ok_or("draw-store.mutation-primary-missing")?;
                        self.start_rebuild(address.parent(), Some(address.index()), None)?;
                        self.phase = DrawMutationCandidatePhase::RebuildSource;
                        return Ok(false);
                    }
                    _ => {}
                }
                let candidate = self.candidate.as_mut().ok_or("draw-store.mutation-candidate-missing")?;
                let address = self.primary;
                match mutation {
                    DrawMutation::SetLayerVisible(value) => {
                        crate::artifacts::draw::schema::layer_base_mut(DrawLayerLocator::node_at_mut(candidate, address.ok_or("draw-store.mutation-primary-missing")?).ok_or("draw-store.mutation-target-lost")?).visible = value.visible
                    }
                    DrawMutation::SetLayerLocked(value) => {
                        crate::artifacts::draw::schema::layer_base_mut(DrawLayerLocator::node_at_mut(candidate, address.ok_or("draw-store.mutation-primary-missing")?).ok_or("draw-store.mutation-target-lost")?).locked = value.locked
                    }
                    DrawMutation::SetLayerOpacity(value) if value.opacity.is_finite() => {
                        crate::artifacts::draw::schema::layer_base_mut(DrawLayerLocator::node_at_mut(candidate, address.ok_or("draw-store.mutation-primary-missing")?).ok_or("draw-store.mutation-target-lost")?).opacity = value.opacity
                    }
                    DrawMutation::SetLayerOpacity(_) => return Err("draw-store.mutation-opacity-invalid"),
                    DrawMutation::SetLayerBlendMode(value) => {
                        let replacement = DrawSnapshotCloneAuthority::clone_string(&value.blend_mode)?;
                        let old = std::mem::replace(
                            &mut crate::artifacts::draw::schema::layer_base_mut(DrawLayerLocator::node_at_mut(candidate, address.ok_or("draw-store.mutation-primary-missing")?).ok_or("draw-store.mutation-target-lost")?).blend_mode,
                            replacement,
                        );
                        *self.retirement = Some(Box::new(DrawOwnedRetirement::new(DrawRetirementOwner::String(old))));
                    }
                    DrawMutation::RenameLayer(value) => {
                        let replacement = DrawSnapshotCloneAuthority::clone_string(&value.new_name)?;
                        let old = std::mem::replace(
                            &mut crate::artifacts::draw::schema::layer_base_mut(DrawLayerLocator::node_at_mut(candidate, address.ok_or("draw-store.mutation-primary-missing")?).ok_or("draw-store.mutation-target-lost")?).name,
                            replacement,
                        );
                        *self.retirement = Some(Box::new(DrawOwnedRetirement::new(DrawRetirementOwner::String(old))));
                    }
                    DrawMutation::UpdateLayerTransform(value)
                        if [value.transform.x, value.transform.y, value.transform.scale_x, value.transform.scale_y, value.transform.rotation].iter().all(|field| field.is_finite()) && value.transform.scale_x > 0.0 && value.transform.scale_y > 0.0 =>
                    {
                        crate::artifacts::draw::schema::layer_base_mut(DrawLayerLocator::node_at_mut(candidate, address.ok_or("draw-store.mutation-primary-missing")?).ok_or("draw-store.mutation-target-lost")?).transform =
                            crate::artifacts::draw::DrawTransform { x: value.transform.x, y: value.transform.y, scale_x: value.transform.scale_x, scale_y: value.transform.scale_y, rotation: value.transform.rotation };
                    }
                    DrawMutation::UpdateLayerTransform(_) => return Err("draw-store.mutation-transform-invalid"),
                    DrawMutation::ReplaceLayerFill(value) => {
                        let replacement = match value.fill.as_ref() {
                            Some(_) => Some(self.fill_clone.as_mut().ok_or("draw-store.fill-clone-missing")?.take().ok_or("draw-store.fill-false-terminal")?),
                            None => None,
                        };
                        let old = std::mem::replace(
                            &mut crate::artifacts::draw::schema::layer_base_mut(DrawLayerLocator::node_at_mut(candidate, address.ok_or("draw-store.mutation-primary-missing")?).ok_or("draw-store.mutation-target-lost")?).attributes.fill,
                            replacement,
                        );
                        if let Some(old) = old {
                            *self.retirement = Some(Box::new(DrawOwnedRetirement::new(DrawRetirementOwner::Fill(old))));
                        }
                        if let Some(mut clone) = self.fill_clone.take() {
                            clone.terminal = true;
                            drop(clone);
                        }
                    }
                    DrawMutation::ReplaceLayerStroke(value) => {
                        let replacement = match value.stroke.as_ref() {
                            Some(_) => Some(self.stroke_clone.as_mut().ok_or("draw-store.stroke-clone-missing")?.take().ok_or("draw-store.stroke-false-terminal")?),
                            None => None,
                        };
                        let old = std::mem::replace(
                            &mut crate::artifacts::draw::schema::layer_base_mut(DrawLayerLocator::node_at_mut(candidate, address.ok_or("draw-store.mutation-primary-missing")?).ok_or("draw-store.mutation-target-lost")?).attributes.stroke,
                            replacement,
                        );
                        if let Some(old) = old {
                            *self.retirement = Some(Box::new(DrawOwnedRetirement::new(DrawRetirementOwner::Stroke(old))));
                        }
                        if let Some(mut clone) = self.stroke_clone.take() {
                            clone.terminal = true;
                            drop(clone);
                        }
                    }
                    DrawMutation::SetLayerBooleanOperation(value) => {
                        let DrawLayerNode::Boolean(target) = DrawLayerLocator::node_at_mut(candidate, address.ok_or("draw-store.mutation-primary-missing")?).ok_or("draw-store.mutation-target-lost")? else {
                            return Err("draw-store.mutation-boolean-target");
                        };
                        let old = std::mem::replace(&mut target.operation, DrawSnapshotCloneAuthority::clone_string(&value.boolean_operation)?);
                        *self.retirement = Some(Box::new(DrawOwnedRetirement::new(DrawRetirementOwner::String(old))));
                    }
                    DrawMutation::UpdateLayerTraceParams(value) if value.params.threshold.is_finite() && value.params.simplify_epsilon.is_finite() => {
                        let DrawLayerNode::Trace(target) = DrawLayerLocator::node_at_mut(candidate, address.ok_or("draw-store.mutation-primary-missing")?).ok_or("draw-store.mutation-target-lost")? else {
                            return Err("draw-store.mutation-trace-target");
                        };
                        target.params = crate::artifacts::draw::DrawTraceParams { threshold: value.params.threshold, simplify_epsilon: value.params.simplify_epsilon };
                    }
                    DrawMutation::UpdateLayerTraceParams(_) => return Err("draw-store.mutation-trace-invalid"),
                    DrawMutation::CreateLayer(_) | DrawMutation::DuplicateLayer(_) | DrawMutation::DeleteLayer(_) | DrawMutation::ReorderLayer(_) => unreachable!("structural Draw mutations start retained rebuild before scalar mutation"),
                }
                self.phase = DrawMutationCandidatePhase::Complete;
                cx.consume_fuel(1);
                Ok(false)
            }
            DrawMutationCandidatePhase::RebuildSource => {
                if !self.rebuild.as_mut().ok_or("draw-store.mutation-rebuild-missing")?.step(cx)? {
                    return Ok(false);
                }
                let parent = self.primary.ok_or("draw-store.mutation-primary-missing")?.parent();
                let removed = self.finish_rebuild(parent)?.ok_or("draw-store.mutation-removal-missing")?;
                if matches!(mutation, DrawMutation::DeleteLayer(_)) {
                    *self.retirement = Some(Box::new(DrawOwnedRetirement::new(DrawRetirementOwner::Layer(removed))));
                    self.phase = DrawMutationCandidatePhase::Complete;
                } else {
                    *self.pending_layer = Some(removed);
                    self.secondary = None;
                    if Self::parent(mutation).is_some() {
                        self.locator = Some(DrawLayerLocator::new());
                        self.phase = DrawMutationCandidatePhase::LocateDestination;
                    } else {
                        self.phase = DrawMutationCandidatePhase::RebuildDestination;
                    }
                }
                Ok(false)
            }
            DrawMutationCandidatePhase::LocateDestination => {
                let candidate = self.candidate.as_ref().ok_or("draw-store.mutation-candidate-missing")?;
                let locator = self.locator.as_mut().ok_or("draw-store.mutation-destination-locator")?;
                if !locator.step(candidate, Self::parent(mutation).ok_or("draw-store.mutation-parent-missing")?, cx)? {
                    return Ok(false);
                }
                self.secondary = locator.found();
                self.locator = None;
                let Some(address) = self.secondary else { return Err("draw-store.mutation-parent-not-found") };
                if !matches!(DrawLayerLocator::node_at(candidate, address), Some(DrawLayerNode::Group(_))) {
                    return Err("draw-store.mutation-parent-not-group");
                }
                self.phase = DrawMutationCandidatePhase::RebuildDestination;
                Ok(false)
            }
            DrawMutationCandidatePhase::RebuildDestination => {
                if self.rebuild.is_none() {
                    let parent = match mutation {
                        DrawMutation::CreateLayer(_) => self.secondary,
                        DrawMutation::DuplicateLayer(_) => self.primary.ok_or("draw-store.mutation-primary-missing")?.parent(),
                        DrawMutation::ReorderLayer(_) => self.secondary,
                        _ => return Err("draw-store.mutation-destination-variant"),
                    };
                    let index = match mutation {
                        DrawMutation::CreateLayer(value) => value.index.unwrap_or_else(|| DrawLayerLocator::container_mut(self.candidate.as_mut().expect("Draw mutation candidate remains exact"), parent).map_or(0, |values| values.len())),
                        DrawMutation::DuplicateLayer(_) => self.primary.ok_or("draw-store.mutation-primary-missing")?.index() + 1,
                        DrawMutation::ReorderLayer(value) => value.index,
                        _ => 0,
                    };
                    self.start_rebuild(parent, None, Some(index))?;
                }
                if !self.rebuild.as_mut().ok_or("draw-store.mutation-rebuild-missing")?.step(cx)? {
                    return Ok(false);
                }
                let parent = match mutation {
                    DrawMutation::CreateLayer(_) | DrawMutation::ReorderLayer(_) => self.secondary,
                    DrawMutation::DuplicateLayer(_) => self.primary.ok_or("draw-store.mutation-primary-missing")?.parent(),
                    _ => None,
                };
                if self.finish_rebuild(parent)?.is_some() {
                    return Err("draw-store.mutation-unexpected-removal");
                }
                self.phase = DrawMutationCandidatePhase::Complete;
                Ok(false)
            }
            DrawMutationCandidatePhase::Complete => {
                if let Some(retirement) = self.retirement.as_mut() {
                    return match retirement.close_step(1, DRAW_OWNED_FIELD_BYTES).map_err(|_| "draw-store.mutation-retirement")? {
                        store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                            drop(self.retirement.take());
                            self.terminal = true;
                            Ok(true)
                        }
                        store::SnapshotRetirementStep::Complete => self.fail("draw-store.mutation-retirement-false-terminal"),
                        _ => Ok(false),
                    };
                }
                self.terminal = true;
                Ok(true)
            }
            DrawMutationCandidatePhase::Retire | DrawMutationCandidatePhase::Fault => Err(self.fault.unwrap_or("draw-store.mutation-candidate-fault")),
        }
    }

    fn take(&mut self) -> Option<DrawSnapshot> {
        if !self.terminal {
            return None;
        }
        drop(self.clone_digest.take());
        self.candidate.take()
    }

    fn close_step(&mut self, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if let Some(retirement) = self.retirement.as_mut() {
            return match retirement.close_step(1, maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                    drop(self.retirement.take());
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                store::SnapshotRetirementStep::Complete => Err("Draw mutation candidate retirement reported false terminal".into()),
                step => Ok(step),
            };
        }
        if let Some(rebuild) = self.rebuild.as_mut() {
            return match rebuild.close_step(maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if rebuild.terminal_is_empty() => {
                    drop(self.rebuild.take());
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                store::SnapshotRetirementStep::Complete => Err("Draw mutation rebuild reported false terminal".into()),
                step => Ok(step),
            };
        }
        if let Some(layer) = self.layer_clone.as_mut() {
            return match layer.close_step(1, maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if layer.terminal_is_empty() => {
                    drop(self.layer_clone.take());
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                store::SnapshotRetirementStep::Complete => Err("Draw mutation layer clone reported false terminal".into()),
                step => Ok(step),
            };
        }
        if let Some(fill) = self.fill_clone.as_mut() {
            return match fill.close_step(maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if fill.terminal_is_empty() => {
                    drop(self.fill_clone.take());
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                store::SnapshotRetirementStep::Complete => Err("Draw mutation fill clone reported false terminal".into()),
                step => Ok(step),
            };
        }
        if let Some(stroke) = self.stroke_clone.as_mut() {
            return match stroke.close_step(maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if stroke.terminal_is_empty() => {
                    drop(self.stroke_clone.take());
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                store::SnapshotRetirementStep::Complete => Err("Draw mutation stroke clone reported false terminal".into()),
                step => Ok(step),
            };
        }
        if let Some(value) = self.pending_layer.take() {
            *self.retirement = Some(Box::new(DrawOwnedRetirement::new(DrawRetirementOwner::Layer(value))));
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(clone) = self.clone.as_mut() {
            return match clone.close_step(1, maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if clone.terminal_is_empty() => {
                    drop(self.clone.take());
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                store::SnapshotRetirementStep::Complete => Err("Draw mutation snapshot clone reported false terminal".into()),
                step => Ok(step),
            };
        }
        if let Some(value) = self.candidate.take() {
            *self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&DrawSnapshotRetirementFactory, value));
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        drop(self.clone_digest.take());
        self.terminal = true;
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal
            && self.candidate.is_none()
            && self.clone.is_none()
            && self.clone_digest.is_none()
            && self.layer_clone.is_none()
            && self.fill_clone.is_none()
            && self.stroke_clone.is_none()
            && self.rebuild.is_none()
            && self.pending_layer.is_none()
            && self.retirement.is_none()
    }
}

impl Drop for DrawMutationCandidateAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Draw mutation candidate reached Drop before atomic handoff or cursor retirement");
    }
}

pub fn draw_document_store_owners() -> store::MemberStoreOwners<DrawSnapshot, DrawMutation> {
    store::MemberStoreOwners::new(
        std::sync::Arc::new(DrawSnapshotRetirementFactory),
        std::sync::Arc::new(DrawSnapshotRetirementFactory),
        std::sync::Arc::new(DrawMutationRetirementFactory),
        Box::new(store::ArtifactStoreCursorDisposer::<DrawSnapshot, DrawMutation>::new()),
    )
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DrawStoreInitializationPhase {
    ValidateEnvelope,
    ValidateEditId { edit: usize },
    ValidateEditPair { left: usize, right: usize },
    CloneInitial,
    SeedHistory { edit: usize, lane: u8, index: usize },
    FindApplied { position: usize, scan: usize },
    ApplyForward { position: usize, edit: usize, mutation: usize },
    HashInverse { position: usize, edit: usize, mutation: usize },
    CommitApplied { position: usize, edit: usize },
    FindRedo { position: usize, scan: usize },
    HashRedoForward { position: usize, edit: usize, mutation: usize },
    HashRedoInverse { position: usize, edit: usize, mutation: usize },
    CommitRedo { position: usize, edit: usize },
    BuildCandidate,
    RetireCancelled,
    RetireFault,
    Complete,
    Cancelled,
    Fault,
}

struct DrawStoreInitializationAuthority {
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    envelope: std::mem::ManuallyDrop<Option<store::ArtifactEnvelope<DrawSnapshot, DrawMutation>>>,
    runtime: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationRuntime<DrawSnapshot>>>,
    candidate: std::mem::ManuallyDrop<Option<store::ArtifactStore<DrawSnapshot, DrawMutation>>>,
    active: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    envelope_retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    clone: std::mem::ManuallyDrop<Option<DrawSnapshotCloneAuthority>>,
    mutation_digest: std::mem::ManuallyDrop<Option<DrawMutationDigestAuthority>>,
    mutation_candidate: std::mem::ManuallyDrop<Option<DrawMutationCandidateAuthority>>,
    initial_digest: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationDigest>>,
    edit_digest: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationDigest>>,
    phase: DrawStoreInitializationPhase,
    cancel_requested: bool,
    fault: Option<Vec<u8>>,
    terminal_handoff: bool,
}

impl DrawStoreInitializationAuthority {
    fn new(envelope: store::ArtifactEnvelope<DrawSnapshot, DrawMutation>, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Self {
        Self {
            operation,
            generation,
            envelope: std::mem::ManuallyDrop::new(Some(envelope)),
            runtime: std::mem::ManuallyDrop::new(None),
            candidate: std::mem::ManuallyDrop::new(None),
            active: std::mem::ManuallyDrop::new(None),
            envelope_retirement: std::mem::ManuallyDrop::new(None),
            clone: std::mem::ManuallyDrop::new(Some(DrawSnapshotCloneAuthority::new())),
            mutation_digest: std::mem::ManuallyDrop::new(None),
            mutation_candidate: std::mem::ManuallyDrop::new(None),
            initial_digest: std::mem::ManuallyDrop::new(Some(store::ArtifactStoreInitializationDigest::new(b"draw.initial"))),
            edit_digest: std::mem::ManuallyDrop::new(None),
            phase: DrawStoreInitializationPhase::ValidateEnvelope,
            cancel_requested: false,
            fault: None,
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
        self.phase = DrawStoreInitializationPhase::RetireFault;
    }

    fn pump_active(&mut self) -> Result<bool, String> {
        let Some(active) = self.active.as_mut() else { return Ok(false) };
        match active.close_step(1, DRAW_OWNED_FIELD_BYTES)? {
            store::SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items <= 1 && released_bytes <= DRAW_OWNED_FIELD_BYTES => Ok(true),
            store::SnapshotRetirementStep::Pending { .. } => Err("Draw store initializer retirement exceeded its exact grant".into()),
            store::SnapshotRetirementStep::Blocked => Ok(true),
            store::SnapshotRetirementStep::Complete if active.terminal_is_empty() => {
                drop(self.active.take());
                Ok(true)
            }
            store::SnapshotRetirementStep::Complete => Err("Draw store initializer retirement reported a false terminal".into()),
        }
    }

    fn pump_terminal_retirement(&mut self) -> Result<bool, String> {
        if self.pump_active()? {
            return Ok(false);
        }
        if let Some(runtime) = self.runtime.as_mut() {
            match runtime.close_step(&DrawSnapshotRetirementFactory, 1, DRAW_OWNED_FIELD_BYTES)? {
                store::SnapshotRetirementStep::Complete if runtime.terminal_is_empty() => {
                    drop(self.runtime.take());
                    return Ok(false);
                }
                store::SnapshotRetirementStep::Complete => return Err("Draw initialization runtime reported a false terminal".into()),
                _ => return Ok(false),
            }
        }
        if let Some(clone) = self.clone.as_mut() {
            match clone.close_step(1, DRAW_OWNED_FIELD_BYTES)? {
                store::SnapshotRetirementStep::Complete if clone.terminal_is_empty() => {
                    drop(self.clone.take());
                    return Ok(false);
                }
                store::SnapshotRetirementStep::Complete => return Err("Draw snapshot clone reported a false terminal".into()),
                _ => return Ok(false),
            }
        }
        if let Some(digest) = self.mutation_digest.as_mut() {
            return match digest.close_step(DRAW_OWNED_FIELD_BYTES)? {
                store::SnapshotRetirementStep::Complete if digest.terminal_is_empty() => {
                    drop(self.mutation_digest.take());
                    Ok(false)
                }
                store::SnapshotRetirementStep::Complete => Err("Draw mutation digest reported a false terminal".into()),
                _ => Ok(false),
            };
        }
        if let Some(candidate) = self.mutation_candidate.as_mut() {
            return match candidate.close_step(DRAW_OWNED_FIELD_BYTES)? {
                store::SnapshotRetirementStep::Complete if candidate.terminal_is_empty() => {
                    drop(self.mutation_candidate.take());
                    Ok(false)
                }
                store::SnapshotRetirementStep::Complete => Err("Draw mutation candidate reported a false terminal".into()),
                _ => Ok(false),
            };
        }
        if self.envelope_retirement.is_none() {
            if let Some(envelope) = self.envelope.take() {
                *self.envelope_retirement = Some(draw_envelope_decode_owner_bundle().retire_envelope(envelope));
                return Ok(false);
            }
        }
        if let Some(retirement) = self.envelope_retirement.as_mut() {
            return match retirement.close_step(1, DRAW_OWNED_FIELD_BYTES)? {
                store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                    drop(self.envelope_retirement.take());
                    Ok(true)
                }
                store::SnapshotRetirementStep::Complete => Err("Draw initialization envelope retirement reported a false terminal".into()),
                _ => Ok(false),
            };
        }
        Ok(true)
    }

    fn terminal_is_empty_inner(&self) -> bool {
        self.terminal_handoff
            && self.envelope.is_none()
            && self.runtime.is_none()
            && self.candidate.is_none()
            && self.active.is_none()
            && self.envelope_retirement.is_none()
            && self.clone.is_none()
            && self.mutation_digest.is_none()
            && self.mutation_candidate.is_none()
            && self.initial_digest.is_none()
            && self.edit_digest.is_none()
    }
}

impl semio_framework_plugin::ArtifactStoreInitializationAuthority<DrawSnapshot, DrawMutation> for DrawStoreInitializationAuthority {
    fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
        if cx.operation() != self.operation || cx.generation() != self.generation {
            self.fail(b"draw-store.initializer-stale-authority");
        }
        if self.cancel_requested && !matches!(self.phase, DrawStoreInitializationPhase::RetireCancelled | DrawStoreInitializationPhase::Cancelled) {
            self.phase = DrawStoreInitializationPhase::RetireCancelled;
        }
        if let Err(error) = self.pump_active() {
            self.fault = Some(error.into_bytes());
            self.phase = DrawStoreInitializationPhase::RetireFault;
        } else if self.active.is_some() {
            return semio_framework_job::StepOutcome::Yield;
        }
        match self.phase {
            DrawStoreInitializationPhase::ValidateEnvelope => {
                let Some(envelope) = self.envelope.as_ref() else {
                    self.fail(b"draw-store.initializer-envelope-missing");
                    return semio_framework_job::StepOutcome::Yield;
                };
                if envelope.schema != crate::artifacts::draw::DRAW_DOCUMENT_SCHEMA || envelope.id.is_empty() || envelope.id.len() > DRAW_OWNED_FIELD_BYTES {
                    self.fail(b"draw-store.initializer-envelope-invalid");
                } else {
                    self.phase = DrawStoreInitializationPhase::ValidateEditId { edit: 0 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            DrawStoreInitializationPhase::ValidateEditId { edit } => {
                let envelope = self.envelope.as_ref().expect("validated Draw envelope remains retained");
                let Some(entry) = envelope.vcs.edits.get(edit) else {
                    self.phase = DrawStoreInitializationPhase::ValidateEditPair { left: 0, right: 1 };
                    return semio_framework_job::StepOutcome::Yield;
                };
                if entry.id.is_empty()
                    || entry.id.len() > DRAW_OWNED_FIELD_BYTES
                    || entry.actor.as_ref().is_some_and(|actor| actor.len() > DRAW_OWNED_FIELD_BYTES)
                    || entry.started_at.len() > DRAW_OWNED_FIELD_BYTES
                    || entry.mutation_meta.iter().any(|meta| meta.timestamp.len() > DRAW_OWNED_FIELD_BYTES || meta.mutation_id.as_ref().is_some_and(|id| id.0.len() > DRAW_OWNED_FIELD_BYTES))
                {
                    self.fail(b"draw-store.initializer-hostile-edit-field");
                } else {
                    self.phase = DrawStoreInitializationPhase::ValidateEditId { edit: edit + 1 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            DrawStoreInitializationPhase::ValidateEditPair { left, right } => {
                let envelope = self.envelope.as_ref().expect("validated Draw envelope remains retained");
                if left >= envelope.vcs.edits.len() {
                    self.phase = DrawStoreInitializationPhase::CloneInitial;
                } else if right >= envelope.vcs.edits.len() {
                    self.phase = DrawStoreInitializationPhase::ValidateEditPair { left: left + 1, right: left + 2 };
                } else if envelope.vcs.edits[left].id == envelope.vcs.edits[right].id {
                    self.fail(b"draw-store.initializer-duplicate-edit");
                } else {
                    self.phase = DrawStoreInitializationPhase::ValidateEditPair { left, right: right + 1 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            DrawStoreInitializationPhase::CloneInitial => {
                let source = &self.envelope.as_ref().expect("Draw envelope remains retained during initial clone").vcs.initial_snapshot;
                let clone = self.clone.as_mut().expect("Draw initial clone authority remains retained");
                let complete = match clone.step(source, self.initial_digest.as_mut().expect("Draw initial digest remains retained"), cx) {
                    Ok(complete) => complete,
                    Err(code) => {
                        self.fail(code.as_bytes());
                        return semio_framework_job::StepOutcome::Yield;
                    }
                };
                if complete {
                    let initial = clone.take_value().expect("Draw initial snapshot was built one semantic item at a time");
                    drop(self.clone.take());
                    let initial_digest = self.initial_digest.take().expect("Draw initial digest remains retained").finish();
                    let envelope = self.envelope.as_ref().expect("Draw envelope remains retained during runtime construction");
                    *self.runtime = Some(store::ArtifactStoreInitializationRuntime::new(&envelope.id, &envelope.schema, initial, initial_digest));
                    self.phase = DrawStoreInitializationPhase::SeedHistory { edit: 0, lane: 0, index: 0 };
                }
                semio_framework_job::StepOutcome::Yield
            }
            DrawStoreInitializationPhase::SeedHistory { edit, lane, index } => {
                let envelope = self.envelope.as_ref().expect("Draw envelope remains retained while causal history is seeded");
                let Some(entry) = envelope.vcs.edits.get(edit) else {
                    self.phase = DrawStoreInitializationPhase::FindApplied { position: 0, scan: 0 };
                    return semio_framework_job::StepOutcome::Yield;
                };
                let runtime = self.runtime.as_mut().expect("Draw runtime remains retained while history is seeded");
                match lane {
                    0 => {
                        if let Err(error) = runtime.seed_mutation(protocol::MutationId(entry.id.clone())) {
                            self.fault = Some(error.into_bytes());
                            self.phase = DrawStoreInitializationPhase::RetireFault;
                        } else {
                            runtime.observe_sequence(entry.sequence_number);
                            self.phase = DrawStoreInitializationPhase::SeedHistory { edit, lane: 1, index: 0 };
                        }
                    }
                    1 if index < entry.forwards.len() => {
                        let id = entry.mutation_meta.get(index).and_then(|meta| meta.mutation_id.clone()).or_else(|| entry.forwards[index].mutation_id()).unwrap_or_else(|| protocol::MutationId(format!("{}#{index}", entry.id)));
                        if let Err(error) = runtime.seed_mutation(id) {
                            self.fault = Some(error.into_bytes());
                            self.phase = DrawStoreInitializationPhase::RetireFault;
                        } else {
                            self.phase = DrawStoreInitializationPhase::SeedHistory { edit, lane, index: index + 1 };
                        }
                    }
                    1 => self.phase = DrawStoreInitializationPhase::SeedHistory { edit, lane: 2, index: 0 },
                    2 if index < entry.mutation_meta.len() => {
                        runtime.observe_timestamp(entry.mutation_meta[index].timestamp.clone());
                        self.phase = DrawStoreInitializationPhase::SeedHistory { edit, lane, index: index + 1 };
                    }
                    _ => self.phase = DrawStoreInitializationPhase::SeedHistory { edit: edit + 1, lane: 0, index: 0 },
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            DrawStoreInitializationPhase::FindApplied { position, scan } => {
                let Some(id) = self.applied_id(position) else {
                    let checkpoint = self.envelope.as_ref().and_then(|envelope| envelope.cursor.as_ref().and_then(|cursor| cursor.checkpoint_id.clone()).or_else(|| envelope.vcs.checkpoints.last().map(|checkpoint| checkpoint.id.clone())));
                    self.runtime.as_mut().expect("Draw runtime remains retained").set_current_checkpoint_id(checkpoint);
                    self.phase = DrawStoreInitializationPhase::FindRedo { position: 0, scan: 0 };
                    return semio_framework_job::StepOutcome::Yield;
                };
                let envelope = self.envelope.as_ref().expect("Draw envelope remains retained");
                let Some(edit) = envelope.vcs.edits.get(scan) else {
                    self.fail(b"draw-store.initializer-applied-edit-missing");
                    return semio_framework_job::StepOutcome::Yield;
                };
                if edit.id == id {
                    let mut digest = store::ArtifactStoreInitializationDigest::new(b"draw.edit");
                    digest.observe(edit.id.as_bytes());
                    digest.observe(&edit.sequence_number.to_be_bytes());
                    digest.observe(edit.started_at.as_bytes());
                    *self.edit_digest = Some(digest);
                    self.phase = DrawStoreInitializationPhase::ApplyForward { position, edit: scan, mutation: 0 };
                } else {
                    self.phase = DrawStoreInitializationPhase::FindApplied { position, scan: scan + 1 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            DrawStoreInitializationPhase::ApplyForward { position, edit, mutation } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Draw applied edit remains retained");
                let Some(operation) = entry.forwards.get(mutation) else {
                    self.phase = DrawStoreInitializationPhase::HashInverse { position, edit, mutation: 0 };
                    return semio_framework_job::StepOutcome::Yield;
                };
                if self.mutation_digest.is_none() {
                    *self.mutation_digest = Some(DrawMutationDigestAuthority::new());
                    cx.consume_fuel(1);
                    return semio_framework_job::StepOutcome::Yield;
                }
                let digest_complete = match self.mutation_digest.as_mut().expect("Draw mutation digest remains retained").step(operation, self.edit_digest.as_mut().expect("Draw edit digest remains retained"), cx) {
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
                    *self.mutation_candidate = Some(DrawMutationCandidateAuthority::new());
                    cx.consume_fuel(1);
                    return semio_framework_job::StepOutcome::Yield;
                }
                let current = self.runtime.as_mut().and_then(store::ArtifactStoreInitializationRuntime::current_mut).expect("Draw runtime current snapshot remains retained");
                let candidate_complete = match self.mutation_candidate.as_mut().expect("Draw mutation candidate remains retained").step(current, operation, cx) {
                    Ok(complete) => complete,
                    Err(error) => {
                        self.fail(error.as_bytes());
                        return semio_framework_job::StepOutcome::Yield;
                    }
                };
                if candidate_complete {
                    let next = self.mutation_candidate.as_mut().expect("Draw completed mutation candidate remains retained").take().expect("Draw mutation candidate terminal handoff remains exact");
                    drop(self.mutation_candidate.take());
                    let current = self.runtime.as_mut().and_then(store::ArtifactStoreInitializationRuntime::current_mut).expect("Draw runtime current snapshot remains retained");
                    let previous = std::mem::replace(current, next);
                    *self.active = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&DrawSnapshotRetirementFactory, previous));
                    self.phase = DrawStoreInitializationPhase::ApplyForward { position, edit, mutation: mutation + 1 };
                }
                semio_framework_job::StepOutcome::Yield
            }
            DrawStoreInitializationPhase::HashInverse { position, edit, mutation } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Draw applied edit remains retained");
                let Some(operation) = entry.inverse.get(mutation) else {
                    self.phase = DrawStoreInitializationPhase::CommitApplied { position, edit };
                    return semio_framework_job::StepOutcome::Yield;
                };
                if self.mutation_digest.is_none() {
                    *self.mutation_digest = Some(DrawMutationDigestAuthority::new());
                    cx.consume_fuel(1);
                    return semio_framework_job::StepOutcome::Yield;
                }
                match self.mutation_digest.as_mut().expect("Draw inverse digest remains retained").step(operation, self.edit_digest.as_mut().expect("Draw edit digest remains retained"), cx) {
                    Ok(true) => {
                        drop(self.mutation_digest.take());
                        self.phase = DrawStoreInitializationPhase::HashInverse { position, edit, mutation: mutation + 1 };
                    }
                    Ok(false) => {}
                    Err(error) => self.fail(error.as_bytes()),
                }
                semio_framework_job::StepOutcome::Yield
            }
            DrawStoreInitializationPhase::CommitApplied { position, edit } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Draw applied edit remains retained");
                let id = entry.id.clone();
                let actor = entry.actor.clone();
                let digest = self.edit_digest.take().expect("Draw applied edit digest remains retained").finish();
                let runtime = self.runtime.as_mut().expect("Draw runtime remains retained");
                if let Err(error) = runtime.push_applied(id, digest) {
                    self.fault = Some(error.into_bytes());
                    self.phase = DrawStoreInitializationPhase::RetireFault;
                } else {
                    runtime.set_local_actor_id(actor);
                    self.phase = DrawStoreInitializationPhase::FindApplied { position: position + 1, scan: 0 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            DrawStoreInitializationPhase::FindRedo { position, scan } => {
                let Some(id) = self.redo_id(position) else {
                    self.phase = DrawStoreInitializationPhase::BuildCandidate;
                    return semio_framework_job::StepOutcome::Yield;
                };
                let envelope = self.envelope.as_ref().expect("Draw envelope remains retained");
                let Some(edit) = envelope.vcs.edits.get(scan) else {
                    self.fail(b"draw-store.initializer-redo-edit-missing");
                    return semio_framework_job::StepOutcome::Yield;
                };
                if edit.id == id {
                    let mut digest = store::ArtifactStoreInitializationDigest::new(b"draw.edit");
                    digest.observe(edit.id.as_bytes());
                    digest.observe(&edit.sequence_number.to_be_bytes());
                    digest.observe(edit.started_at.as_bytes());
                    *self.edit_digest = Some(digest);
                    self.phase = DrawStoreInitializationPhase::HashRedoForward { position, edit: scan, mutation: 0 };
                } else {
                    self.phase = DrawStoreInitializationPhase::FindRedo { position, scan: scan + 1 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            DrawStoreInitializationPhase::HashRedoForward { position, edit, mutation } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Draw redo edit remains retained");
                let Some(operation) = entry.forwards.get(mutation) else {
                    self.phase = DrawStoreInitializationPhase::HashRedoInverse { position, edit, mutation: 0 };
                    return semio_framework_job::StepOutcome::Yield;
                };
                if self.mutation_digest.is_none() {
                    *self.mutation_digest = Some(DrawMutationDigestAuthority::new());
                    cx.consume_fuel(1);
                    return semio_framework_job::StepOutcome::Yield;
                }
                match self.mutation_digest.as_mut().expect("Draw redo forward digest remains retained").step(operation, self.edit_digest.as_mut().expect("Draw redo digest remains retained"), cx) {
                    Ok(true) => {
                        drop(self.mutation_digest.take());
                        self.phase = DrawStoreInitializationPhase::HashRedoForward { position, edit, mutation: mutation + 1 };
                    }
                    Ok(false) => {}
                    Err(error) => self.fail(error.as_bytes()),
                }
                semio_framework_job::StepOutcome::Yield
            }
            DrawStoreInitializationPhase::HashRedoInverse { position, edit, mutation } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Draw redo edit remains retained");
                let Some(operation) = entry.inverse.get(mutation) else {
                    self.phase = DrawStoreInitializationPhase::CommitRedo { position, edit };
                    return semio_framework_job::StepOutcome::Yield;
                };
                if self.mutation_digest.is_none() {
                    *self.mutation_digest = Some(DrawMutationDigestAuthority::new());
                    cx.consume_fuel(1);
                    return semio_framework_job::StepOutcome::Yield;
                }
                match self.mutation_digest.as_mut().expect("Draw redo inverse digest remains retained").step(operation, self.edit_digest.as_mut().expect("Draw redo digest remains retained"), cx) {
                    Ok(true) => {
                        drop(self.mutation_digest.take());
                        self.phase = DrawStoreInitializationPhase::HashRedoInverse { position, edit, mutation: mutation + 1 };
                    }
                    Ok(false) => {}
                    Err(error) => self.fail(error.as_bytes()),
                }
                semio_framework_job::StepOutcome::Yield
            }
            DrawStoreInitializationPhase::CommitRedo { position, edit } => {
                let id = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Draw redo edit remains retained").id.clone();
                let digest = self.edit_digest.take().expect("Draw redo digest remains retained").finish();
                if let Err(error) = self.runtime.as_mut().expect("Draw runtime remains retained").push_redo(id, digest) {
                    self.fault = Some(error.into_bytes());
                    self.phase = DrawStoreInitializationPhase::RetireFault;
                } else {
                    self.phase = DrawStoreInitializationPhase::FindRedo { position: position + 1, scan: 0 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            DrawStoreInitializationPhase::BuildCandidate => {
                let Some(candidate_generation) = self.generation.0.checked_add(1) else {
                    self.fail(b"draw-store.initializer-generation-exhausted");
                    return semio_framework_job::StepOutcome::Yield;
                };
                let envelope = self.envelope.take().expect("Draw envelope remains retained until atomic store construction");
                let runtime = self.runtime.take().expect("Draw runtime remains retained until atomic store construction");
                let candidate = store::ArtifactStore::from_initialized_runtime_with_owners(envelope, runtime, candidate_generation, draw_document_store_owners());
                *self.candidate = Some(candidate);
                self.phase = DrawStoreInitializationPhase::Complete;
                semio_framework_job::StepOutcome::Complete(semio_framework_job::CommitCandidate { state: Vec::new(), output: Vec::new() })
            }
            DrawStoreInitializationPhase::RetireCancelled | DrawStoreInitializationPhase::RetireFault => match self.pump_terminal_retirement() {
                Ok(false) => semio_framework_job::StepOutcome::Yield,
                Ok(true) => {
                    drop(self.initial_digest.take());
                    drop(self.edit_digest.take());
                    self.terminal_handoff = true;
                    if self.phase == DrawStoreInitializationPhase::RetireCancelled {
                        self.phase = DrawStoreInitializationPhase::Cancelled;
                        semio_framework_job::StepOutcome::Cancelled
                    } else {
                        self.phase = DrawStoreInitializationPhase::Fault;
                        semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail: self.fault.take().unwrap_or_else(|| b"draw-store.initializer-fault".to_vec()) })
                    }
                }
                Err(error) => {
                    self.fault = Some(error.into_bytes());
                    semio_framework_job::StepOutcome::Yield
                }
            },
            DrawStoreInitializationPhase::Complete => semio_framework_job::StepOutcome::Complete(semio_framework_job::CommitCandidate { state: Vec::new(), output: Vec::new() }),
            DrawStoreInitializationPhase::Cancelled => semio_framework_job::StepOutcome::Cancelled,
            DrawStoreInitializationPhase::Fault => semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail: self.fault.clone().unwrap_or_else(|| b"draw-store.initializer-fault".to_vec()) }),
        }
    }

    fn request_cancel(&mut self) {
        self.cancel_requested = true;
    }

    fn take_candidate(&mut self) -> Option<store::ArtifactStore<DrawSnapshot, DrawMutation>> {
        if self.phase != DrawStoreInitializationPhase::Complete || self.terminal_handoff {
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

impl Drop for DrawStoreInitializationAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty_inner(), "Draw store initialization authority reached Drop before exact candidate handoff or retained rejection close");
    }
}

pub fn draw_document_store_initialization_job(
    envelope: store::ArtifactEnvelope<DrawSnapshot, DrawMutation>,
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
) -> semio_framework_plugin::ArtifactStoreInitializationJob<DrawSnapshot, DrawMutation> {
    semio_framework_plugin::ArtifactStoreInitializationJob::new(Box::new(DrawStoreInitializationAuthority::new(envelope, operation, generation)))
}
//#endregion 🔖️RetainedStoreInitialization

//#region 🧪️RetainedMutationAuthorityTests
#[cfg(test)]
mod retained_mutation_authority_tests {
    use super::*;
    use crate::artifacts::draw::mutations::{
        CreateLayer, DeleteLayer, DuplicateLayer, RenameLayer, ReorderLayer, ReplaceLayerFill, ReplaceLayerStroke, SetLayerBlendMode, SetLayerBooleanOperation, SetLayerLocked, SetLayerOpacity, SetLayerVisible, UpdateLayerTraceParams,
        UpdateLayerTransform,
    };

    fn nested_snapshot() -> DrawSnapshot {
        let mut snapshot = crate::artifacts::draw::schema::default_draw_document("draw-retained-mutation", None);
        let shape = crate::artifacts::draw::schema::create_draw_shape_layer_rect("Shape");
        let boolean = crate::artifacts::draw::schema::create_draw_boolean_layer("Boolean", "union", vec![crate::artifacts::draw::schema::layer_id(&shape).into()]);
        let trace = crate::artifacts::draw::schema::create_draw_trace_layer("Trace", "asset-a");
        let mut group = crate::artifacts::draw::schema::create_draw_group_layer("Group");
        if let DrawLayerNode::Group(value) = &mut group {
            value.children.push(shape);
            value.children.push(boolean);
            value.children.push(trace);
        }
        snapshot.layers.push(group);
        snapshot.assets.insert("asset-a".into(), DrawImageAsset { mime: "image/png".into(), data: "AA==".into(), width: Some(1), height: Some(1) });
        snapshot
    }

    fn drain_snapshot(value: DrawSnapshot) {
        let mut retirement = store::ArtifactOwnedValueRetirementFactory::retire_owned(&DrawSnapshotRetirementFactory, value);
        for _ in 0..100_000 {
            match retirement.close_step(1, DRAW_OWNED_FIELD_BYTES).expect("Draw snapshot retirement") {
                store::SnapshotRetirementStep::Complete => {
                    assert!(retirement.terminal_is_empty());
                    drop(retirement);
                    return;
                }
                store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                    assert!(released_items <= 1);
                    assert!(released_bytes <= DRAW_OWNED_FIELD_BYTES);
                }
                store::SnapshotRetirementStep::Blocked => panic!("owned Draw snapshot retirement cannot block"),
            }
        }
        panic!("Draw snapshot retirement did not terminate")
    }

    fn close_candidate(authority: &mut DrawMutationCandidateAuthority) {
        for _ in 0..100_000 {
            match authority.close_step(DRAW_OWNED_FIELD_BYTES).expect("Draw candidate close") {
                store::SnapshotRetirementStep::Complete => {
                    assert!(authority.terminal_is_empty());
                    return;
                }
                store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                    assert!(released_items <= 1);
                    assert!(released_bytes <= DRAW_OWNED_FIELD_BYTES);
                }
                store::SnapshotRetirementStep::Blocked => panic!("owned Draw candidate close cannot block"),
            }
        }
        panic!("Draw candidate close did not terminate")
    }

    fn apply(source: &DrawSnapshot, mutation: &DrawMutation) -> Result<DrawSnapshot, &'static str> {
        let mut authority = DrawMutationCandidateAuthority::new();
        let cancel = semio_framework_job::root_cancel_token();
        let mut preview_sequence = 0;
        for _ in 0..200_000 {
            let mut context = semio_framework_job::StepContext::new(
                semio_framework_job::OperationId(8_001),
                semio_framework_job::Generation(81),
                semio_framework_job::StepBudget::new(1, u64::MAX),
                cancel.clone(),
                semio_framework_job::default_now_ms,
                &mut preview_sequence,
            );
            match authority.step(source, mutation, &mut context) {
                Ok(true) => {
                    let value = authority.take().expect("Draw mutation candidate exact terminal handoff");
                    assert!(authority.terminal_is_empty());
                    drop(authority);
                    return Ok(value);
                }
                Ok(false) => {}
                Err(error) => {
                    close_candidate(&mut authority);
                    drop(authority);
                    return Err(error);
                }
            }
        }
        close_candidate(&mut authority);
        drop(authority);
        panic!("Draw mutation candidate did not terminate")
    }

    #[test]
    fn retained_draw_mutation_candidate_covers_all_fourteen_variants_and_returns_exact_owners() {
        let source = nested_snapshot();
        let group = crate::artifacts::draw::schema::layer_id(source.layers.last().expect("group")).to_string();
        let (shape, boolean, trace) = match source.layers.last().expect("group") {
            DrawLayerNode::Group(value) => {
                (crate::artifacts::draw::schema::layer_id(&value.children[0]).to_string(), crate::artifacts::draw::schema::layer_id(&value.children[1]).to_string(), crate::artifacts::draw::schema::layer_id(&value.children[2]).to_string())
            }
            _ => unreachable!("Draw fixture group remains exact"),
        };
        let mutations = vec![
            DrawMutation::SetLayerVisible(SetLayerVisible { layer_id: shape.clone(), visible: false }),
            DrawMutation::SetLayerLocked(SetLayerLocked { layer_id: shape.clone(), locked: true }),
            DrawMutation::SetLayerOpacity(SetLayerOpacity { layer_id: shape.clone(), opacity: 0.5 }),
            DrawMutation::SetLayerBlendMode(SetLayerBlendMode { layer_id: shape.clone(), blend_mode: "multiply".into() }),
            DrawMutation::RenameLayer(RenameLayer { layer_id: shape.clone(), new_name: "Renamed".into() }),
            DrawMutation::UpdateLayerTransform(UpdateLayerTransform { layer_id: shape.clone(), transform: crate::artifacts::draw::DrawTransform { x: 1.0, y: 2.0, scale_x: 3.0, scale_y: 4.0, rotation: 0.5 } }),
            DrawMutation::ReplaceLayerFill(ReplaceLayerFill {
                layer_id: shape.clone(),
                fill: Some(FillStyle::LinearGradient { x1: 0.0, y1: 0.0, x2: 1.0, y2: 1.0, stops: vec![GradientStop { offset: 0.0, color: [1.0, 0.0, 0.0, 1.0] }, GradientStop { offset: 1.0, color: [0.0, 0.0, 1.0, 1.0] }] }),
            }),
            DrawMutation::ReplaceLayerStroke(ReplaceLayerStroke { layer_id: shape.clone(), stroke: Some(StrokeStyle { color: [0.0, 0.0, 0.0, 1.0], width: 2.0, cap: "round".into(), join: "bevel".into(), dash: Some(vec![1.0, 2.0]) }) }),
            DrawMutation::SetLayerBooleanOperation(SetLayerBooleanOperation { layer_id: boolean, boolean_operation: "subtract".into() }),
            DrawMutation::UpdateLayerTraceParams(UpdateLayerTraceParams { layer_id: trace, params: crate::artifacts::draw::DrawTraceParams { threshold: 0.4, simplify_epsilon: 1.2 } }),
            DrawMutation::CreateLayer(CreateLayer { parent_id: Some(group.clone()), index: Some(1), layer: Box::new(crate::artifacts::draw::schema::create_draw_path_layer("Created", vec![PathSegment::Move { to: [0.0, 0.0] }])) }),
            DrawMutation::DuplicateLayer(DuplicateLayer { layer_id: shape.clone() }),
            DrawMutation::DeleteLayer(DeleteLayer { layer_id: shape.clone() }),
            DrawMutation::ReorderLayer(ReorderLayer { layer_id: shape, parent_id: None, index: 0 }),
        ];
        for mutation in mutations {
            let value = apply(&source, &mutation).expect("retained Draw mutation applies");
            drain_snapshot(value);
        }
        drain_snapshot(source);
    }

    #[test]
    fn retained_draw_depth_plus_one_and_hostile_fields_fault_then_close_terminal_empty() {
        let mut layer = crate::artifacts::draw::schema::create_draw_path_layer("leaf", Vec::new());
        for depth in 0..=DRAW_MAXIMUM_LAYER_DEPTH {
            let mut parent = crate::artifacts::draw::schema::create_draw_group_layer(&format!("depth-{depth}"));
            if let DrawLayerNode::Group(value) = &mut parent {
                value.children.push(layer);
            }
            layer = parent;
        }
        let mut source = crate::artifacts::draw::schema::default_draw_document("draw-depth-plus-one", None);
        source.layers = vec![layer];
        let mutation = DrawMutation::SetLayerVisible(SetLayerVisible { layer_id: "missing".into(), visible: false });
        assert_eq!(apply(&source, &mutation), Err("draw-store.preflight-depth-capacity"));
        drain_snapshot(source);

        let source = nested_snapshot();
        let mutation = DrawMutation::RenameLayer(RenameLayer { layer_id: "x".repeat(DRAW_OWNED_FIELD_BYTES + 1), new_name: "hostile".into() });
        assert!(apply(&source, &mutation).is_err());
        drain_snapshot(source);
    }

    #[test]
    fn retained_draw_container_false_terminal_saturation_and_interrupted_close_preserve_exact_owner() {
        let source = vec![crate::artifacts::draw::schema::create_draw_path_layer("first", Vec::new()), crate::artifacts::draw::schema::create_draw_path_layer("second", Vec::new())];
        let pending = crate::artifacts::draw::schema::create_draw_path_layer("pending", Vec::new());
        let mut authority = DrawContainerRebuildAuthority::new(source, Some(0), Some(1), Some(pending)).expect("fixed Draw rebuild admitted");
        assert!(authority.take().is_none(), "false terminal cannot expose a partially rebuilt owner");
        match authority.close_step(0).expect("interrupted Draw rebuild close") {
            store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert_eq!(released_items, 0);
                assert_eq!(released_bytes, 0);
            }
            _ => panic!("zero-grant close must retain every exact owner"),
        }
        for _ in 0..100_000 {
            if matches!(authority.close_step(DRAW_OWNED_FIELD_BYTES).expect("Draw rebuild close"), store::SnapshotRetirementStep::Complete) {
                assert!(authority.terminal_is_empty());
                drop(authority);
                return;
            }
        }
        panic!("Draw rebuild exact owner close did not terminate")
    }
}
//#endregion 🧪️RetainedMutationAuthorityTests
