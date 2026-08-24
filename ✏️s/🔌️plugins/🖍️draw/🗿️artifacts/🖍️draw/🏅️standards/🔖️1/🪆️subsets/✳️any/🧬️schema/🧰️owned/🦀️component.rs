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
const DRAW_MUTATION_AGGREGATE_ITEMS: usize = DRAW_MAXIMUM_NESTED_ITEMS;
const DRAW_MUTATION_AGGREGATE_BYTES: usize = DRAW_MAXIMUM_NESTED_BYTES;
const DRAW_MUTATION_RETAINED_PAGE_ITEMS: usize = 1;
const DRAW_MUTATION_RETAINED_PAGE_BYTES: usize = store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;
const DRAW_MUTATION_OVERLAY_PAGE_CAPACITY: usize = 16;
const DRAW_MUTATION_CONTAINER_SLOT_CAPACITY: usize = 64;
const DRAW_MUTATION_ARENA_POOL_CAPACITY: usize = 4;
const DRAW_DUPLICATE_MATERIAL_BYTES: usize = DRAW_OWNED_FIELD_BYTES * 2;
const DRAW_DUPLICATE_ID_BYTES: usize = 80;

struct DrawMutationArenaOwner {
    reverse: Vec<DrawLayerNode>,
    output: Vec<DrawLayerNode>,
    pages: Vec<String>,
    duplicate_id: String,
}

impl DrawMutationArenaOwner {
    fn admitted_totals(&self) -> Result<(usize, usize), &'static str> {
        let items = self.reverse.capacity().checked_add(self.output.capacity()).and_then(|items| items.checked_add(self.pages.capacity())).and_then(|items| items.checked_add(1)).ok_or("draw-store.mutation-arena-item-overflow")?;
        let bytes = std::mem::size_of::<Self>()
            .checked_add(self.reverse.capacity().checked_mul(std::mem::size_of::<DrawLayerNode>()).ok_or("draw-store.mutation-arena-byte-overflow")?)
            .and_then(|bytes| bytes.checked_add(self.output.capacity().checked_mul(std::mem::size_of::<DrawLayerNode>())?))
            .and_then(|bytes| bytes.checked_add(self.pages.capacity().checked_mul(std::mem::size_of::<String>())?))
            .and_then(|bytes| self.pages.iter().try_fold(bytes, |total, page| total.checked_add(page.capacity())))
            .and_then(|bytes| bytes.checked_add(self.duplicate_id.capacity()))
            .ok_or("draw-store.mutation-arena-byte-overflow")?;
        Ok((items, bytes))
    }

    fn terminal_is_empty(&self) -> bool {
        self.reverse.is_empty()
            && self.reverse.capacity() >= DRAW_MUTATION_CONTAINER_SLOT_CAPACITY
            && self.output.is_empty()
            && self.output.capacity() >= DRAW_MUTATION_CONTAINER_SLOT_CAPACITY
            && self.pages.len() == DRAW_MUTATION_OVERLAY_PAGE_CAPACITY
            && self.pages.capacity() >= DRAW_MUTATION_OVERLAY_PAGE_CAPACITY
            && self.pages.iter().all(|page| page.is_empty() && page.capacity() >= DRAW_MUTATION_RETAINED_PAGE_BYTES)
            && self.duplicate_id.is_empty()
            && self.duplicate_id.capacity() >= DRAW_DUPLICATE_ID_BYTES
    }
}

struct DrawMutationArenaOwnerBuilder {
    reverse: std::mem::ManuallyDrop<Option<Vec<DrawLayerNode>>>,
    output: std::mem::ManuallyDrop<Option<Vec<DrawLayerNode>>>,
    pages: std::mem::ManuallyDrop<Option<Vec<String>>>,
    duplicate_id: std::mem::ManuallyDrop<Option<String>>,
    rejected_string: std::mem::ManuallyDrop<Option<String>>,
    phase: usize,
    terminal: bool,
}

impl DrawMutationArenaOwnerBuilder {
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

    fn from_owner(owner: DrawMutationArenaOwner) -> Self {
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
            return Err("draw-store.mutation-arena-bootstrap-injected-allocation");
        }
        match self.phase {
            0 => {
                let mut value = Vec::new();
                if value.try_reserve_exact(DRAW_MUTATION_CONTAINER_SLOT_CAPACITY).is_err() {
                    return Err("draw-store.mutation-reverse-arena-admission");
                }
                *self.reverse = Some(value);
            }
            1 => {
                let mut value = Vec::new();
                if value.try_reserve_exact(DRAW_MUTATION_CONTAINER_SLOT_CAPACITY).is_err() {
                    return Err("draw-store.mutation-output-arena-admission");
                }
                *self.output = Some(value);
            }
            2 => {
                let mut pages = Vec::new();
                if pages.try_reserve_exact(DRAW_MUTATION_OVERLAY_PAGE_CAPACITY).is_err() {
                    return Err("draw-store.mutation-overlay-arena-admission");
                }
                *self.pages = Some(pages);
            }
            phase if phase < 3 + DRAW_MUTATION_OVERLAY_PAGE_CAPACITY => {
                let mut page = String::new();
                if page.try_reserve_exact(DRAW_MUTATION_RETAINED_PAGE_BYTES).is_err() {
                    return Err("draw-store.mutation-overlay-page-admission");
                }
                let Some(pages) = self.pages.as_mut() else {
                    *self.rejected_string = Some(page);
                    return Err("draw-store.mutation-overlay-arena-missing");
                };
                if pages.len() >= pages.capacity() {
                    *self.rejected_string = Some(page);
                    return Err("draw-store.mutation-overlay-arena-saturated");
                }
                pages.push(page);
            }
            19 => {
                let mut value = String::new();
                if value.try_reserve_exact(DRAW_DUPLICATE_ID_BYTES).is_err() {
                    return Err("draw-store.duplicate-id-owner-admission");
                }
                *self.duplicate_id = Some(value);
            }
            _ => return Err("draw-store.mutation-arena-bootstrap-phase"),
        }
        self.phase += 1;
        Ok(self.phase == 20)
    }

    fn take(&mut self) -> Option<DrawMutationArenaOwner> {
        if self.phase != 20 || self.terminal {
            return None;
        }
        if self.rejected_string.is_some() || self.reverse.is_none() || self.output.is_none() || self.pages.is_none() || self.duplicate_id.is_none() {
            return None;
        }
        let owner = DrawMutationArenaOwner {
            reverse: self.reverse.take().expect("validated Draw reverse bootstrap owner remains retained"),
            output: self.output.take().expect("validated Draw output bootstrap owner remains retained"),
            pages: self.pages.take().expect("validated Draw page bootstrap owner remains retained"),
            duplicate_id: self.duplicate_id.take().expect("validated Draw duplicate bootstrap owner remains retained"),
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
            let released_bytes = value.capacity().saturating_mul(std::mem::size_of::<DrawLayerNode>());
            drop(value);
            return store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes };
        }
        if let Some(value) = self.reverse.take() {
            let released_bytes = value.capacity().saturating_mul(std::mem::size_of::<DrawLayerNode>());
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

impl Drop for DrawMutationArenaOwnerBuilder {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Draw mutation arena owner builder reached Drop before exact construction handoff or retirement");
    }
}

struct DrawMutationArenaPoolSlot {
    reverse: Option<Vec<DrawLayerNode>>,
    output: Option<Vec<DrawLayerNode>>,
    pages: Option<Vec<String>>,
    duplicate_id: Option<String>,
    generation: u64,
    leased: bool,
}

impl DrawMutationArenaPoolSlot {
    fn new(owner: DrawMutationArenaOwner) -> Self {
        Self { reverse: Some(owner.reverse), output: Some(owner.output), pages: Some(owner.pages), duplicate_id: Some(owner.duplicate_id), generation: 0, leased: false }
    }

    fn is_available(&self) -> bool {
        !self.leased && self.reverse.is_some() && self.output.is_some() && self.pages.is_some() && self.duplicate_id.is_some()
    }

    fn take(&mut self, generation: u64) -> Option<DrawMutationArenaOwner> {
        if !self.is_available() {
            return None;
        }
        self.generation = generation;
        self.leased = true;
        Some(DrawMutationArenaOwner {
            reverse: self.reverse.take().expect("available Draw pool slot retains reverse owner"),
            output: self.output.take().expect("available Draw pool slot retains output owner"),
            pages: self.pages.take().expect("available Draw pool slot retains page owner"),
            duplicate_id: self.duplicate_id.take().expect("available Draw pool slot retains duplicate owner"),
        })
    }
}

struct DrawMutationArenaPoolState {
    slots: [DrawMutationArenaPoolSlot; DRAW_MUTATION_ARENA_POOL_CAPACITY],
}

struct DrawMutationArenaPool {
    state: std::sync::Mutex<DrawMutationArenaPoolState>,
    admitted_items: usize,
    admitted_bytes: usize,
}

struct DrawMutationArenaPoolBootstrap {
    owners: std::mem::ManuallyDrop<[Option<DrawMutationArenaOwner>; DRAW_MUTATION_ARENA_POOL_CAPACITY]>,
    active: std::mem::ManuallyDrop<Option<DrawMutationArenaOwnerBuilder>>,
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

impl std::fmt::Debug for DrawMutationArenaPoolBootstrap {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("DrawMutationArenaPoolBootstrap").field("owner", &self.owner).field("allocation", &self.allocation).field("fault", &self.fault).finish()
    }
}

impl DrawMutationArenaPoolBootstrap {
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

    fn production(admission: DrawMutationArenaBootstrapAdmission) -> Self {
        Self::new(None, None, admission.maximum_items, admission.maximum_bytes)
    }

    fn fail(&mut self, fault: &'static str) -> Result<bool, &'static str> {
        self.fault = Some(fault);
        Err(fault)
    }

    fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if cx.should_yield() {
            return Err("draw-store.mutation-arena-bootstrap-budget");
        }
        if let Some(fault) = self.fault {
            return Err(fault);
        }
        if self.ready {
            return Ok(true);
        }
        if self.owner == DRAW_MUTATION_ARENA_POOL_CAPACITY {
            if self.admitted_items > self.maximum_items || self.admitted_bytes > self.maximum_bytes {
                return self.fail("draw-store.mutation-arena-pool-capacity");
            }
            self.ready = true;
            return Ok(true);
        }
        if self.active.is_none() {
            *self.active = Some(DrawMutationArenaOwnerBuilder::new());
            return Ok(false);
        }
        let complete = match self.active.as_mut().expect("Draw arena owner builder remains retained").step(&mut self.allocation, self.failure_at) {
            Ok(complete) => complete,
            Err(error) => return self.fail(error),
        };
        if !complete {
            return Ok(false);
        }
        let mut builder = self.active.take().expect("completed Draw arena owner builder remains retained");
        let Some(owner) = builder.take() else {
            *self.active = Some(builder);
            return self.fail("draw-store.mutation-arena-owner-false-terminal");
        };
        drop(builder);
        if !owner.terminal_is_empty() {
            self.owners[self.owner] = Some(owner);
            return self.fail("draw-store.mutation-arena-pool-initial-owner");
        }
        let totals = owner.admitted_totals();
        let owner_index = self.owner;
        self.owners[owner_index] = Some(owner);
        self.owner += 1;
        if self.failure_after_owner == Some(owner_index) {
            return self.fail("draw-store.mutation-arena-bootstrap-injected-owner");
        }
        let (items, bytes) = match totals {
            Ok(totals) => totals,
            Err(error) => return self.fail(error),
        };
        self.admitted_items = match self.admitted_items.checked_add(items) {
            Some(total) => total,
            None => return self.fail("draw-store.mutation-arena-pool-item-overflow"),
        };
        self.admitted_bytes = match self.admitted_bytes.checked_add(bytes) {
            Some(total) => total,
            None => return self.fail("draw-store.mutation-arena-pool-byte-overflow"),
        };
        Ok(false)
    }

    fn take_pool(&mut self) -> Option<std::sync::Arc<DrawMutationArenaPool>> {
        if !self.ready || self.terminal {
            return None;
        }
        let owners = std::mem::replace(&mut *self.owners, std::array::from_fn(|_| None));
        let slots = owners.map(|owner| DrawMutationArenaPoolSlot::new(owner.expect("validated Draw arena bootstrap retains every owner")));
        self.terminal = true;
        Some(std::sync::Arc::new(DrawMutationArenaPool { state: std::sync::Mutex::new(DrawMutationArenaPoolState { slots }), admitted_items: self.admitted_items, admitted_bytes: self.admitted_bytes }))
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
            let owner = self.owners[self.owner].take().expect("Draw bootstrap retirement cursor locates the preceding retained owner");
            *self.active = Some(DrawMutationArenaOwnerBuilder::from_owner(owner));
            return store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 };
        }
        self.terminal = true;
        store::SnapshotRetirementStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal && self.active.is_none() && self.owners.iter().all(Option::is_none)
    }
}

impl Drop for DrawMutationArenaPoolBootstrap {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Draw mutation arena pool bootstrap reached Drop before exact handoff or fault retirement");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct DrawMutationArenaBootstrapAdmission {
    maximum_items: usize,
    maximum_bytes: usize,
}

impl DrawMutationArenaBootstrapAdmission {
    fn fixed() -> Result<Self, &'static str> {
        Ok(Self {
            maximum_items: DRAW_MUTATION_AGGREGATE_ITEMS.checked_mul(DRAW_MUTATION_ARENA_POOL_CAPACITY).ok_or("draw-store.mutation-arena-bootstrap-item-claim")?,
            maximum_bytes: DRAW_MUTATION_AGGREGATE_BYTES.checked_mul(DRAW_MUTATION_ARENA_POOL_CAPACITY).ok_or("draw-store.mutation-arena-bootstrap-byte-claim")?,
        })
    }
}

enum DrawMutationArenaProcessState {
    Inert,
    Building(DrawMutationArenaPoolBootstrap),
    Ready(std::sync::Arc<DrawMutationArenaPool>),
    Retiring(DrawMutationArenaPoolBootstrap),
    Fault(&'static str),
}

static DRAW_MUTATION_ARENA_POOL: std::sync::OnceLock<std::sync::Mutex<DrawMutationArenaProcessState>> = std::sync::OnceLock::new();
static DRAW_MUTATION_ARENA_BOOTSTRAP_REQUESTED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DrawMutationArenaPoolAvailability {
    Ready,
    NotReady,
    Contended,
    Fault(&'static str),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DrawMutationArenaBorrowError {
    NotReady,
    Contended,
    Fault(&'static str),
    Invalid(&'static str),
}

impl DrawMutationArenaBorrowError {
    fn as_str(self) -> &'static str {
        match self {
            Self::NotReady => "draw-store.mutation-arena-bootstrap-not-ready",
            Self::Contended => "draw-store.mutation-arena-pool-contended",
            Self::Fault(fault) | Self::Invalid(fault) => fault,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DrawMutationArenaBootstrapStep {
    Pending { advanced_items: u64 },
    Blocked,
    Ready,
    Cancelled,
    Fault(&'static str),
}

struct DrawMutationArenaBootstrapJob {
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    admission: DrawMutationArenaBootstrapAdmission,
    terminal: bool,
}

impl DrawMutationArenaBootstrapJob {
    fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Result<Self, &'static str> {
        request_draw_mutation_arena_pool();
        Ok(Self { operation, generation, admission: DrawMutationArenaBootstrapAdmission::fixed()?, terminal: false })
    }

    fn inactive(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Self {
        Self { operation, generation, admission: DrawMutationArenaBootstrapAdmission { maximum_items: 0, maximum_bytes: 0 }, terminal: true }
    }

    fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> DrawMutationArenaBootstrapStep {
        if self.terminal {
            return DrawMutationArenaBootstrapStep::Ready;
        }
        if cx.operation() != self.operation || cx.generation() != self.generation {
            self.terminal = true;
            return DrawMutationArenaBootstrapStep::Fault("draw-store.mutation-arena-bootstrap-stale-authority");
        }
        if cx.should_yield() {
            return DrawMutationArenaBootstrapStep::Blocked;
        }
        let state = DRAW_MUTATION_ARENA_POOL.get_or_init(|| std::sync::Mutex::new(DrawMutationArenaProcessState::Inert));
        let Ok(mut state) = state.try_lock() else {
            return DrawMutationArenaBootstrapStep::Blocked;
        };
        self.step_locked(&mut state, cx)
    }

    fn step_locked(&mut self, state: &mut DrawMutationArenaProcessState, cx: &mut semio_framework_job::StepContext<'_>) -> DrawMutationArenaBootstrapStep {
        cx.set_stage("draw-arena-bootstrap");
        if cx.is_cancelled() {
            match &*state {
                DrawMutationArenaProcessState::Inert | DrawMutationArenaProcessState::Ready(_) => {
                    self.terminal = true;
                    return DrawMutationArenaBootstrapStep::Cancelled;
                }
                DrawMutationArenaProcessState::Building(_) => {
                    let previous = std::mem::replace(&mut *state, DrawMutationArenaProcessState::Fault("draw-store.mutation-arena-bootstrap-transition"));
                    let DrawMutationArenaProcessState::Building(mut bootstrap) = previous else { unreachable!("Draw bootstrap cancellation preserves its exact building owner") };
                    bootstrap.fault = Some("draw-store.mutation-arena-bootstrap-cancelled");
                    *state = DrawMutationArenaProcessState::Retiring(bootstrap);
                    cx.consume_fuel(1);
                    return DrawMutationArenaBootstrapStep::Pending { advanced_items: 1 };
                }
                DrawMutationArenaProcessState::Retiring(_) => {}
                DrawMutationArenaProcessState::Fault(fault) => {
                    self.terminal = true;
                    return DrawMutationArenaBootstrapStep::Fault(*fault);
                }
            }
        }
        let transition = match &mut *state {
            DrawMutationArenaProcessState::Inert => {
                if !DRAW_MUTATION_ARENA_BOOTSTRAP_REQUESTED.swap(false, std::sync::atomic::Ordering::AcqRel) {
                    return DrawMutationArenaBootstrapStep::Blocked;
                }
                *state = DrawMutationArenaProcessState::Building(DrawMutationArenaPoolBootstrap::production(self.admission));
                cx.consume_fuel(1);
                return DrawMutationArenaBootstrapStep::Pending { advanced_items: 1 };
            }
            DrawMutationArenaProcessState::Building(bootstrap) => match bootstrap.step(cx) {
                Ok(true) => DrawMutationArenaProcessTransition::Publish,
                Ok(false) => DrawMutationArenaProcessTransition::None,
                Err(_) => DrawMutationArenaProcessTransition::Retire,
            },
            DrawMutationArenaProcessState::Ready(_) => {
                self.terminal = true;
                return DrawMutationArenaBootstrapStep::Ready;
            }
            DrawMutationArenaProcessState::Fault(error) => {
                self.terminal = true;
                return DrawMutationArenaBootstrapStep::Fault(*error);
            }
            DrawMutationArenaProcessState::Retiring(bootstrap) => {
                let fault = bootstrap.fault.unwrap_or("draw-store.mutation-arena-bootstrap-fault");
                if matches!(bootstrap.close_step(cx), store::SnapshotRetirementStep::Complete) && bootstrap.terminal_is_empty() {
                    DrawMutationArenaProcessTransition::Fault(fault)
                } else {
                    DrawMutationArenaProcessTransition::None
                }
            }
        };
        cx.consume_fuel(1);
        match transition {
            DrawMutationArenaProcessTransition::None => DrawMutationArenaBootstrapStep::Pending { advanced_items: 1 },
            DrawMutationArenaProcessTransition::Publish => {
                let previous = std::mem::replace(&mut *state, DrawMutationArenaProcessState::Fault("draw-store.mutation-arena-bootstrap-transition"));
                let DrawMutationArenaProcessState::Building(mut bootstrap) = previous else { unreachable!("Draw arena publish transition preserves the building owner") };
                let Some(pool) = bootstrap.take_pool() else {
                    *state = DrawMutationArenaProcessState::Retiring(bootstrap);
                    return DrawMutationArenaBootstrapStep::Pending { advanced_items: 1 };
                };
                drop(bootstrap);
                *state = DrawMutationArenaProcessState::Ready(pool);
                self.terminal = true;
                DrawMutationArenaBootstrapStep::Ready
            }
            DrawMutationArenaProcessTransition::Retire => {
                let previous = std::mem::replace(&mut *state, DrawMutationArenaProcessState::Fault("draw-store.mutation-arena-bootstrap-transition"));
                let DrawMutationArenaProcessState::Building(bootstrap) = previous else { unreachable!("Draw arena fault transition preserves the building owner") };
                *state = DrawMutationArenaProcessState::Retiring(bootstrap);
                DrawMutationArenaBootstrapStep::Pending { advanced_items: 1 }
            }
            DrawMutationArenaProcessTransition::Fault(fault) => {
                let previous = std::mem::replace(&mut *state, DrawMutationArenaProcessState::Fault(fault));
                let DrawMutationArenaProcessState::Retiring(bootstrap) = previous else { unreachable!("Draw arena terminal fault transition preserves the retirement owner") };
                drop(bootstrap);
                self.terminal = true;
                if fault == "draw-store.mutation-arena-bootstrap-cancelled" {
                    DrawMutationArenaBootstrapStep::Cancelled
                } else {
                    DrawMutationArenaBootstrapStep::Fault(fault)
                }
            }
        }
    }
}

impl DrawMutationArenaPool {
    #[cfg(test)]
    fn try_new() -> Result<std::sync::Arc<Self>, DrawMutationArenaPoolBootstrap> {
        let mut bootstrap = DrawMutationArenaPoolBootstrap::production(DrawMutationArenaBootstrapAdmission::fixed().expect("fixed Draw arena bootstrap claim"));
        let cancel = semio_framework_job::root_cancel_token();
        let mut preview_sequence = 0;
        for _ in 0..=DRAW_MUTATION_ARENA_POOL_CAPACITY * 24 {
            let mut context = semio_framework_job::StepContext::new(
                semio_framework_job::OperationId(7_901),
                semio_framework_job::Generation(79),
                semio_framework_job::StepBudget::new(1, u64::MAX),
                cancel.clone(),
                semio_framework_job::default_now_ms,
                &mut preview_sequence,
            );
            match bootstrap.step(&mut context) {
                Ok(true) => return Ok(bootstrap.take_pool().expect("completed Draw arena bootstrap publishes exact pool")),
                Ok(false) => {}
                Err(_) => return Err(bootstrap),
            }
        }
        bootstrap.fault = Some("draw-store.mutation-arena-bootstrap-turn-capacity");
        Err(bootstrap)
    }
}

enum DrawMutationArenaProcessTransition {
    None,
    Publish,
    Retire,
    Fault(&'static str),
}

pub fn request_draw_mutation_arena_pool() -> DrawMutationArenaPoolAvailability {
    DRAW_MUTATION_ARENA_BOOTSTRAP_REQUESTED.store(true, std::sync::atomic::Ordering::Release);
    let state = DRAW_MUTATION_ARENA_POOL.get_or_init(|| std::sync::Mutex::new(DrawMutationArenaProcessState::Inert));
    let Ok(state) = state.try_lock() else {
        return DrawMutationArenaPoolAvailability::Contended;
    };
    match &*state {
        DrawMutationArenaProcessState::Ready(_) => DrawMutationArenaPoolAvailability::Ready,
        DrawMutationArenaProcessState::Fault(fault) => DrawMutationArenaPoolAvailability::Fault(*fault),
        DrawMutationArenaProcessState::Inert | DrawMutationArenaProcessState::Building(_) | DrawMutationArenaProcessState::Retiring(_) => DrawMutationArenaPoolAvailability::NotReady,
    }
}

pub fn draw_mutation_arena_pool_fault() -> Option<&'static str> {
    let state = DRAW_MUTATION_ARENA_POOL.get()?;
    let state = state.try_lock().ok()?;
    match &*state {
        DrawMutationArenaProcessState::Fault(fault) => Some(*fault),
        DrawMutationArenaProcessState::Inert | DrawMutationArenaProcessState::Building(_) | DrawMutationArenaProcessState::Ready(_) | DrawMutationArenaProcessState::Retiring(_) => None,
    }
}

fn borrow_draw_mutation_arena_from(pool: std::sync::Arc<DrawMutationArenaPool>) -> Result<(std::sync::Arc<DrawMutationArenaPool>, usize, u64, DrawMutationArenaOwner), &'static str> {
    if pool.admitted_items == 0 || pool.admitted_bytes == 0 {
        return Err("draw-store.mutation-arena-pool-unadmitted");
    }
    let mut state = pool.state.try_lock().map_err(|_| "draw-store.mutation-arena-pool-contended")?;
    let slot = state.slots.iter().position(DrawMutationArenaPoolSlot::is_available).ok_or("draw-store.mutation-arena-pool-saturated")?;
    let generation = state.slots[slot].generation.checked_add(1).ok_or("draw-store.mutation-arena-generation-exhausted")?;
    let owner = state.slots[slot].take(generation).ok_or("draw-store.mutation-arena-owner-missing")?;
    drop(state);
    Ok((pool, slot, generation, owner))
}

fn borrow_draw_mutation_arena() -> Result<(std::sync::Arc<DrawMutationArenaPool>, usize, u64, DrawMutationArenaOwner), DrawMutationArenaBorrowError> {
    match request_draw_mutation_arena_pool() {
        DrawMutationArenaPoolAvailability::Ready => {}
        DrawMutationArenaPoolAvailability::NotReady => return Err(DrawMutationArenaBorrowError::NotReady),
        DrawMutationArenaPoolAvailability::Contended => return Err(DrawMutationArenaBorrowError::Contended),
        DrawMutationArenaPoolAvailability::Fault(fault) => return Err(DrawMutationArenaBorrowError::Fault(fault)),
    }
    let state = DRAW_MUTATION_ARENA_POOL.get().ok_or(DrawMutationArenaBorrowError::Invalid("draw-store.mutation-arena-pool-uninitialized"))?;
    let state = state.try_lock().map_err(|_| DrawMutationArenaBorrowError::Contended)?;
    let DrawMutationArenaProcessState::Ready(pool) = &*state else { return Err(DrawMutationArenaBorrowError::NotReady) };
    let pool = pool.clone();
    drop(state);
    borrow_draw_mutation_arena_from(pool).map_err(DrawMutationArenaBorrowError::Invalid)
}

#[derive(Clone, Copy)]
struct DrawTraversalFrame {
    phase: u8,
    child: usize,
    string: usize,
}

impl DrawTraversalFrame {
    const EMPTY: Self = Self { phase: 0, child: 0, string: 0 };
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct DrawSnapshotOwnerTotals {
    source_items: usize,
    source_bytes: usize,
    candidate_items: usize,
    candidate_bytes: usize,
    maximum_container: usize,
}

#[derive(Clone, Copy)]
struct DrawOwnerCreditSlot {
    source_items: u32,
    source_bytes: u32,
    derived_items: u32,
    derived_bytes: u32,
}

impl DrawOwnerCreditSlot {
    const EMPTY: Self = Self { source_items: 0, source_bytes: 0, derived_items: 0, derived_bytes: 0 };
}

struct DrawFixedOwnerCensus {
    slots: [DrawOwnerCreditSlot; DRAW_MAXIMUM_NESTED_ITEMS],
    length: usize,
}

impl DrawFixedOwnerCensus {
    fn new() -> Self {
        Self { slots: [DrawOwnerCreditSlot::EMPTY; DRAW_MAXIMUM_NESTED_ITEMS], length: 0 }
    }

    fn admit(&mut self, source_items: usize, source_bytes: usize, derived_items: usize, derived_bytes: usize) -> Result<(), &'static str> {
        let target = self.slots.get_mut(self.length).ok_or("draw-store.owner-census-slot-capacity")?;
        *target = DrawOwnerCreditSlot {
            source_items: source_items.try_into().map_err(|_| "draw-store.owner-census-item-width")?,
            source_bytes: source_bytes.try_into().map_err(|_| "draw-store.owner-census-byte-width")?,
            derived_items: derived_items.try_into().map_err(|_| "draw-store.owner-census-item-width")?,
            derived_bytes: derived_bytes.try_into().map_err(|_| "draw-store.owner-census-byte-width")?,
        };
        self.length += 1;
        Ok(())
    }
}

struct DrawAssetBoundsCursor {
    key: [u8; DRAW_OWNED_FIELD_BYTES],
    key_len: usize,
    started: bool,
}

impl DrawAssetBoundsCursor {
    fn new() -> Self {
        Self { key: [0; DRAW_OWNED_FIELD_BYTES], key_len: 0, started: false }
    }

    fn next<'a>(&self, assets: &'a std::collections::BTreeMap<String, DrawImageAsset>) -> Result<Option<(&'a String, &'a DrawImageAsset)>, &'static str> {
        if !self.started {
            return Ok(assets.first_key_value());
        }
        let key = std::str::from_utf8(&self.key[..self.key_len]).map_err(|_| "draw-store.preflight-asset-key-utf8")?;
        use std::ops::Bound::{Excluded, Unbounded};
        Ok(assets.range::<str, _>((Excluded(key), Unbounded)).next())
    }

    fn advance(&mut self, key: &str) -> Result<(), &'static str> {
        if key.len() > self.key.len() {
            return Err("draw-store.preflight-asset-key-capacity");
        }
        self.key[..key.len()].copy_from_slice(key.as_bytes());
        self.key_len = key.len();
        self.started = true;
        Ok(())
    }
}

struct DrawSnapshotBoundsAuthority {
    root: usize,
    asset_cursor: DrawAssetBoundsCursor,
    depth: usize,
    path: [usize; DRAW_MAXIMUM_LAYER_DEPTH],
    frames: [DrawTraversalFrame; DRAW_MAXIMUM_LAYER_DEPTH],
    items: usize,
    bytes: usize,
    candidate_items: usize,
    candidate_bytes: usize,
    maximum_container: usize,
    owner_census: DrawFixedOwnerCensus,
    layers_complete: bool,
    terminal: bool,
}

impl DrawSnapshotBoundsAuthority {
    fn new() -> Self {
        Self {
            root: 0,
            asset_cursor: DrawAssetBoundsCursor::new(),
            depth: 0,
            path: [0; DRAW_MAXIMUM_LAYER_DEPTH],
            frames: [DrawTraversalFrame::EMPTY; DRAW_MAXIMUM_LAYER_DEPTH],
            items: 0,
            bytes: 0,
            candidate_items: 0,
            candidate_bytes: 0,
            maximum_container: 0,
            owner_census: DrawFixedOwnerCensus::new(),
            layers_complete: false,
            terminal: false,
        }
    }

    fn layer_at<'a>(root: &'a DrawLayerNode, path: &[usize]) -> Option<&'a DrawLayerNode> {
        let mut value = root;
        for index in path {
            let DrawLayerNode::Group(group) = value else { return None };
            value = group.children.get(*index)?;
        }
        Some(value)
    }

    fn add(&mut self, items: usize, bytes: usize, candidate_items: usize, candidate_bytes: usize) -> Result<(), &'static str> {
        self.owner_census.admit(items, bytes, candidate_items, candidate_bytes)?;
        self.items = self.items.checked_add(items).ok_or("draw-store.preflight-item-overflow")?;
        self.bytes = self.bytes.checked_add(bytes).ok_or("draw-store.preflight-byte-overflow")?;
        self.candidate_items = self.candidate_items.checked_add(candidate_items).ok_or("draw-store.preflight-candidate-item-overflow")?;
        self.candidate_bytes = self.candidate_bytes.checked_add(candidate_bytes).ok_or("draw-store.preflight-candidate-byte-overflow")?;
        if self.items > DRAW_MAXIMUM_NESTED_ITEMS || self.candidate_items > DRAW_MAXIMUM_NESTED_ITEMS {
            return Err("draw-store.preflight-item-capacity");
        }
        if self.bytes > DRAW_MAXIMUM_NESTED_BYTES || self.candidate_bytes > DRAW_MAXIMUM_NESTED_BYTES {
            return Err("draw-store.preflight-byte-capacity");
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

    fn direct_shape(layer: &DrawLayerNode) -> (usize, usize, usize, usize) {
        let base = match layer {
            DrawLayerNode::Shape(value) => &value.base,
            DrawLayerNode::Path(value) => &value.base,
            DrawLayerNode::Text(value) => &value.base,
            DrawLayerNode::Image(value) => &value.base,
            DrawLayerNode::Group(value) => &value.base,
            DrawLayerNode::Boolean(value) => &value.base,
            DrawLayerNode::Trace(value) => &value.base,
        };
        let mut total = (1, std::mem::size_of::<DrawLayerNode>(), 0, 0);
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
            DrawLayerNode::Shape(value) => {
                Self::merge(&mut total, Self::string_owner(&value.shape_kind));
                if let Some(polygon) = &value.polygon {
                    Self::merge(&mut total, Self::vec_owner(&polygon.points));
                }
            }
            DrawLayerNode::Path(value) => Self::merge(&mut total, Self::vec_owner(&value.segments)),
            DrawLayerNode::Text(value) => Self::merge(&mut total, Self::string_owner(&value.content)),
            DrawLayerNode::Image(value) => Self::merge(&mut total, Self::string_owner(&value.image_key)),
            DrawLayerNode::Group(value) => Self::merge(&mut total, Self::vec_owner(&value.children)),
            DrawLayerNode::Boolean(value) => {
                Self::merge(&mut total, Self::string_owner(&value.operation));
                Self::merge(&mut total, Self::vec_owner(&value.children));
            }
            DrawLayerNode::Trace(value) => Self::merge(&mut total, Self::string_owner(&value.source_key)),
        }
        total
    }

    fn step(&mut self, source: &DrawSnapshot, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        if !self.layers_complete {
            self.maximum_container = self.maximum_container.max(source.layers.len());
            let Some(root) = source.layers.get(self.root) else {
                self.layers_complete = true;
                let mut owners = (1, std::mem::size_of::<DrawSnapshot>(), 0, 0);
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
            let layer = Self::layer_at(root, &self.path[..self.depth]).ok_or("draw-store.preflight-path")?;
            let frame = self.frames[self.depth];
            if frame.phase == 0 {
                let (items, bytes, candidate_items, candidate_bytes) = Self::direct_shape(layer);
                self.add(items, bytes, candidate_items, candidate_bytes)?;
                self.frames[self.depth].phase = 1;
                cx.consume_fuel(1);
                return Ok(false);
            }
            if let DrawLayerNode::Boolean(value) = layer {
                if let Some(child) = value.children.get(frame.string) {
                    let owners = Self::string_owner(child);
                    self.add(owners.0, owners.1, owners.2, owners.3)?;
                    self.frames[self.depth].string += 1;
                    cx.consume_fuel(child.len().max(1) as u64);
                    return Ok(false);
                }
            }
            if let DrawLayerNode::Group(value) = layer {
                self.maximum_container = self.maximum_container.max(value.children.len());
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
        let Some((key, value)) = self.asset_cursor.next(&source.assets)? else {
            self.terminal = true;
            return Ok(true);
        };
        let mut owners = (1, std::mem::size_of::<(String, DrawImageAsset)>(), 0, 0);
        Self::merge(&mut owners, Self::string_owner(key));
        Self::merge(&mut owners, Self::string_owner(&value.mime));
        Self::merge(&mut owners, Self::string_owner(&value.data));
        self.add(owners.0, owners.1, owners.2, owners.3)?;
        self.asset_cursor.advance(key)?;
        cx.consume_fuel(1);
        Ok(false)
    }

    fn totals(&self) -> Option<DrawSnapshotOwnerTotals> {
        self.terminal.then_some(DrawSnapshotOwnerTotals { source_items: self.items, source_bytes: self.bytes, candidate_items: self.candidate_items, candidate_bytes: self.candidate_bytes, maximum_container: self.maximum_container })
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

    fn clone_owned_string(source: &String) -> Result<String, &'static str> {
        if source.len() > DRAW_OWNED_FIELD_BYTES || source.capacity() > DRAW_OWNED_FIELD_BYTES {
            return Err("draw-store.initializer-owned-string-capacity");
        }
        let mut value = String::new();
        value.try_reserve_exact(source.capacity()).map_err(|_| "draw-store.initializer-owned-string-admission")?;
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

const DRAW_CONTAINER_REBUILD_MOVE_CAPACITY: usize = DRAW_MUTATION_CONTAINER_SLOT_CAPACITY * 4 + 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DrawContainerRebuildMove {
    Empty,
    SourceToReverse,
    PendingToOutput,
    ReverseToOutput,
    ReverseToRemoved,
    OutputToReverse,
    ReverseToSource,
}

struct DrawContainerRebuildAuthority {
    source: std::mem::ManuallyDrop<Option<Vec<DrawLayerNode>>>,
    reverse: std::mem::ManuallyDrop<Option<Vec<DrawLayerNode>>>,
    output: std::mem::ManuallyDrop<Option<Vec<DrawLayerNode>>>,
    pending: std::mem::ManuallyDrop<Option<DrawLayerNode>>,
    removed: std::mem::ManuallyDrop<Option<DrawLayerNode>>,
    moves: [DrawContainerRebuildMove; DRAW_CONTAINER_REBUILD_MOVE_CAPACITY],
    move_count: usize,
    rollback_cursor: Option<usize>,
    remove_index: Option<usize>,
    insert_index: Option<usize>,
    original_index: usize,
    phase: u8,
    terminal: bool,
}

struct DrawContainerRebuildRejected {
    source: Vec<DrawLayerNode>,
    pending: Option<DrawLayerNode>,
    reverse: Vec<DrawLayerNode>,
    output: Vec<DrawLayerNode>,
}

impl DrawContainerRebuildAuthority {
    fn new(
        source: Vec<DrawLayerNode>,
        remove_index: Option<usize>,
        insert_index: Option<usize>,
        pending: Option<DrawLayerNode>,
        reverse: Vec<DrawLayerNode>,
        output: Vec<DrawLayerNode>,
        reservation: DrawMutationAggregateReservation,
    ) -> Result<Self, DrawContainerRebuildRejected> {
        let extra = usize::from(pending.is_some());
        let Some(output_capacity) = source.len().saturating_sub(usize::from(remove_index.is_some())).checked_add(extra) else {
            return Err(DrawContainerRebuildRejected { source, pending, reverse, output });
        };
        if output_capacity > DRAW_MAXIMUM_NESTED_ITEMS
            || source.len().saturating_add(output_capacity) > reservation.container_slots
            || source.len() > reservation.maximum_container.saturating_add(1)
            || output_capacity > reservation.maximum_container.saturating_add(1)
            || source.capacity() < output_capacity
            || reverse.capacity() < source.len().max(output_capacity)
            || output.capacity() < output_capacity
            || !reverse.is_empty()
            || !output.is_empty()
        {
            return Err(DrawContainerRebuildRejected { source, pending, reverse, output });
        }
        Ok(Self {
            source: std::mem::ManuallyDrop::new(Some(source)),
            reverse: std::mem::ManuallyDrop::new(Some(reverse)),
            output: std::mem::ManuallyDrop::new(Some(output)),
            pending: std::mem::ManuallyDrop::new(pending),
            removed: std::mem::ManuallyDrop::new(None),
            moves: [DrawContainerRebuildMove::Empty; DRAW_CONTAINER_REBUILD_MOVE_CAPACITY],
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
        self.moves.get(self.move_count).map(|_| ()).ok_or("draw-store.container-move-capacity")
    }

    fn record_reserved_move(&mut self, value: DrawContainerRebuildMove) {
        self.moves[self.move_count] = value;
        self.move_count += 1;
    }

    fn advance(&mut self) -> Result<(bool, u64), &'static str> {
        if self.terminal {
            return Ok((true, 0));
        }
        if self.rollback_cursor.is_some() {
            return Err("draw-store.container-advance-after-rollback");
        }
        if self.source.is_none() || self.reverse.is_none() || self.output.is_none() {
            return Err("draw-store.container-owner-missing");
        }
        if self.phase == 0 {
            if !self.source.as_ref().expect("validated Draw source remains retained").is_empty() {
                self.reserve_move()?;
                let value = self.source.as_mut().expect("validated Draw source remains retained").pop().expect("nonempty Draw source yields one owner");
                self.reverse.as_mut().expect("validated Draw reverse remains retained").push(value);
                self.record_reserved_move(DrawContainerRebuildMove::SourceToReverse);
                return Ok((false, 1));
            }
            self.phase = 1;
            return Ok((false, 0));
        }
        if self.phase == 1 {
            if self.pending.is_some() && self.insert_index.is_some_and(|index| index.min(self.reverse.as_ref().map_or(0, Vec::len) + self.original_index) == self.output.as_ref().map_or(0, Vec::len)) {
                self.reserve_move()?;
                self.output.as_mut().expect("validated Draw output remains retained").push(self.pending.take().expect("Draw insertion owner remains retained"));
                self.record_reserved_move(DrawContainerRebuildMove::PendingToOutput);
                return Ok((false, 1));
            }
            if !self.reverse.as_ref().expect("validated Draw reverse remains retained").is_empty() {
                if self.remove_index == Some(self.original_index) && self.removed.is_some() {
                    return Err("draw-store.container-duplicate-removal");
                }
                self.reserve_move()?;
                let value = self.reverse.as_mut().expect("validated Draw reverse remains retained").pop().expect("nonempty Draw reverse yields one owner");
                if self.remove_index == Some(self.original_index) {
                    *self.removed = Some(value);
                    self.record_reserved_move(DrawContainerRebuildMove::ReverseToRemoved);
                } else {
                    self.output.as_mut().expect("validated Draw output remains retained").push(value);
                    self.record_reserved_move(DrawContainerRebuildMove::ReverseToOutput);
                }
                self.original_index += 1;
                return Ok((false, 1));
            }
            if self.pending.is_some() {
                self.reserve_move()?;
                let value = self.pending.take().expect("validated Draw pending owner remains retained");
                self.output.as_mut().expect("validated Draw output remains retained").push(value);
                self.record_reserved_move(DrawContainerRebuildMove::PendingToOutput);
                return Ok((false, 1));
            }
            self.phase = 2;
            return Ok((false, 0));
        }
        if self.phase == 2 {
            if !self.output.as_ref().expect("validated Draw output remains retained").is_empty() {
                self.reserve_move()?;
                let value = self.output.as_mut().expect("validated Draw output remains retained").pop().expect("nonempty Draw output yields one owner");
                self.reverse.as_mut().expect("validated Draw reverse remains retained").push(value);
                self.record_reserved_move(DrawContainerRebuildMove::OutputToReverse);
                return Ok((false, 1));
            }
            self.phase = 3;
            return Ok((false, 0));
        }
        if !self.reverse.as_ref().expect("validated Draw reverse remains retained").is_empty() {
            self.reserve_move()?;
            let value = self.reverse.as_mut().expect("validated Draw reverse remains retained").pop().expect("nonempty Draw reverse yields one owner");
            self.source.as_mut().expect("validated Draw source remains retained").push(value);
            self.record_reserved_move(DrawContainerRebuildMove::ReverseToSource);
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

    fn take(&mut self) -> Option<(Vec<DrawLayerNode>, Option<DrawLayerNode>, Vec<DrawLayerNode>, Vec<DrawLayerNode>)> {
        self.terminal.then(|| {
            (
                self.source.take().expect("Draw rebuilt source container remains retained"),
                self.removed.take(),
                self.reverse.take().expect("Draw emptied reverse arena remains retained"),
                self.output.take().expect("Draw emptied output arena remains retained"),
            )
        })
    }

    fn rollback_step(&mut self) -> Result<bool, &'static str> {
        if self.source.is_none() || self.reverse.is_none() || self.output.is_none() {
            return Err("draw-store.container-rollback-owner-missing");
        }
        let cursor = *self.rollback_cursor.get_or_insert(self.move_count);
        if cursor == 0 {
            self.phase = 0;
            self.original_index = 0;
            return Ok(true);
        }
        let index = cursor - 1;
        match self.moves[index] {
            DrawContainerRebuildMove::SourceToReverse => {
                let value = self.reverse.as_mut().ok_or("draw-store.container-reverse")?.pop().ok_or("draw-store.container-rollback-reverse")?;
                self.source.as_mut().ok_or("draw-store.container-source")?.push(value);
            }
            DrawContainerRebuildMove::PendingToOutput => {
                if self.pending.is_some() {
                    return Err("draw-store.container-rollback-pending");
                }
                let value = self.output.as_mut().ok_or("draw-store.container-output")?.pop().ok_or("draw-store.container-rollback-output")?;
                *self.pending = Some(value);
            }
            DrawContainerRebuildMove::ReverseToOutput => {
                let value = self.output.as_mut().ok_or("draw-store.container-output")?.pop().ok_or("draw-store.container-rollback-output")?;
                self.reverse.as_mut().ok_or("draw-store.container-reverse")?.push(value);
            }
            DrawContainerRebuildMove::ReverseToRemoved => {
                let value = self.removed.take().ok_or("draw-store.container-rollback-removed")?;
                self.reverse.as_mut().ok_or("draw-store.container-reverse")?.push(value);
            }
            DrawContainerRebuildMove::OutputToReverse => {
                let value = self.reverse.as_mut().ok_or("draw-store.container-reverse")?.pop().ok_or("draw-store.container-rollback-reverse")?;
                self.output.as_mut().ok_or("draw-store.container-output")?.push(value);
            }
            DrawContainerRebuildMove::ReverseToSource => {
                let value = self.source.as_mut().ok_or("draw-store.container-source")?.pop().ok_or("draw-store.container-rollback-source")?;
                self.reverse.as_mut().ok_or("draw-store.container-reverse")?.push(value);
            }
            DrawContainerRebuildMove::Empty => return Err("draw-store.container-rollback-empty-move"),
        }
        self.moves[index] = DrawContainerRebuildMove::Empty;
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
                0 => self.moves[index] == DrawContainerRebuildMove::SourceToReverse,
                1 => matches!(self.moves[index], DrawContainerRebuildMove::PendingToOutput | DrawContainerRebuildMove::ReverseToOutput | DrawContainerRebuildMove::ReverseToRemoved),
                2 => self.moves[index] == DrawContainerRebuildMove::OutputToReverse,
                3 => self.moves[index] == DrawContainerRebuildMove::ReverseToSource,
                _ => false,
            }
    }

    fn finish_handoff(&mut self) -> Result<(), &'static str> {
        if !(self.terminal || self.rollback_complete()) || self.source.is_some() || self.reverse.is_some() || self.output.is_some() || self.pending.is_some() || self.removed.is_some() {
            return Err("draw-store.container-rollback-handoff-incomplete");
        }
        self.terminal = true;
        Ok(())
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal && self.source.is_none() && self.reverse.is_none() && self.output.is_none() && self.pending.is_none() && self.removed.is_none()
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct DrawSemanticDigestTotals {
    semantic_items: usize,
    semantic_bytes: usize,
    source_owner_items: usize,
    source_owner_bytes: usize,
    derived_owner_items: usize,
    derived_owner_bytes: usize,
}

struct DrawSemanticDigestCredit {
    items: usize,
    bytes: usize,
    source_owner_items: usize,
    source_owner_bytes: usize,
    derived_owner_items: usize,
    derived_owner_bytes: usize,
    owner_census: DrawFixedOwnerCensus,
    semantic: Option<semio_framework_hash::Sha256>,
}

impl Default for DrawSemanticDigestCredit {
    fn default() -> Self {
        Self {
            items: 0,
            bytes: 0,
            source_owner_items: 1,
            source_owner_bytes: std::mem::size_of::<DrawMutation>(),
            derived_owner_items: 0,
            derived_owner_bytes: 0,
            owner_census: DrawFixedOwnerCensus::new(),
            semantic: Some(semio_framework_hash::Sha256::new()),
        }
    }
}

impl DrawSemanticDigestCredit {
    fn add_source_owner(&mut self, items: usize, bytes: usize) -> Result<(), &'static str> {
        self.owner_census.admit(items, bytes, 0, 0)?;
        self.source_owner_items = self.source_owner_items.checked_add(items).ok_or("draw-store.mutation-source-owner-item-overflow")?;
        self.source_owner_bytes = self.source_owner_bytes.checked_add(bytes).ok_or("draw-store.mutation-source-owner-byte-overflow")?;
        if self.source_owner_items > DRAW_MAXIMUM_NESTED_ITEMS {
            return Err("draw-store.mutation-source-owner-item-capacity");
        }
        if self.source_owner_bytes > DRAW_MAXIMUM_NESTED_BYTES {
            return Err("draw-store.mutation-source-owner-byte-capacity");
        }
        Ok(())
    }

    fn add_derived_owner(&mut self, items: usize, bytes: usize) -> Result<(), &'static str> {
        self.owner_census.admit(0, 0, items, bytes)?;
        self.derived_owner_items = self.derived_owner_items.checked_add(items).ok_or("draw-store.mutation-derived-owner-item-overflow")?;
        self.derived_owner_bytes = self.derived_owner_bytes.checked_add(bytes).ok_or("draw-store.mutation-derived-owner-byte-overflow")?;
        if self.derived_owner_items > DRAW_MAXIMUM_NESTED_ITEMS {
            return Err("draw-store.mutation-derived-owner-item-capacity");
        }
        if self.derived_owner_bytes > DRAW_MAXIMUM_NESTED_BYTES {
            return Err("draw-store.mutation-derived-owner-byte-capacity");
        }
        Ok(())
    }

    fn source_string(&mut self, value: &String) -> Result<(), &'static str> {
        self.add_source_owner(1, std::mem::size_of::<String>() + value.capacity())
    }

    fn derived_string(&mut self, value: &String) -> Result<(), &'static str> {
        if value.len() > DRAW_OWNED_FIELD_BYTES {
            return Err("draw-store.mutation-derived-string-page-capacity");
        }
        self.add_derived_owner(1, 0)
    }

    fn source_vec<T>(&mut self, value: &Vec<T>) -> Result<(), &'static str> {
        let items = 1usize.checked_add(value.capacity()).ok_or("draw-store.mutation-source-owner-item-overflow")?;
        let bytes = value.capacity().checked_mul(std::mem::size_of::<T>()).and_then(|bytes| bytes.checked_add(std::mem::size_of::<Vec<T>>())).ok_or("draw-store.mutation-source-owner-byte-overflow")?;
        self.add_source_owner(items, bytes)
    }

    fn derived_vec<T>(&mut self, value: &Vec<T>) -> Result<(), &'static str> {
        let bytes = value.len().checked_mul(std::mem::size_of::<T>()).ok_or("draw-store.mutation-derived-owner-byte-overflow")?;
        let pages = bytes.checked_add(DRAW_MUTATION_RETAINED_PAGE_BYTES - 1).ok_or("draw-store.mutation-derived-owner-byte-overflow")? / DRAW_MUTATION_RETAINED_PAGE_BYTES;
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
        if value.len() > DRAW_OWNED_FIELD_BYTES {
            return Err("draw-store.mutation-field-capacity");
        }
        self.items = self.items.checked_add(1).ok_or("draw-store.mutation-item-overflow")?;
        self.bytes = self.bytes.checked_add(11).and_then(|bytes| bytes.checked_add(value.len())).ok_or("draw-store.mutation-byte-overflow")?;
        if self.items > DRAW_MAXIMUM_NESTED_ITEMS {
            return Err("draw-store.mutation-item-capacity");
        }
        if self.bytes > DRAW_MAXIMUM_NESTED_BYTES {
            return Err("draw-store.mutation-byte-capacity");
        }
        let prefix = [0xd8];
        let tag = tag.to_be_bytes();
        let length = (value.len() as u64).to_be_bytes();
        let semantic = self.semantic.as_mut().ok_or("draw-store.mutation-digest-sealed")?;
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
        let semantic = self.semantic.take().ok_or("draw-store.mutation-digest-sealed")?.finalize();
        digest.observe(b"draw.semantic.sha256");
        digest.observe(&semantic);
        Ok(())
    }

    fn totals(&self) -> Option<DrawSemanticDigestTotals> {
        self.semantic.is_none().then_some(DrawSemanticDigestTotals {
            semantic_items: self.items,
            semantic_bytes: self.bytes,
            source_owner_items: self.source_owner_items,
            source_owner_bytes: self.source_owner_bytes,
            derived_owner_items: self.derived_owner_items,
            derived_owner_bytes: self.derived_owner_bytes,
        })
    }
}

struct DrawFillDigestAuthority {
    phase: u8,
    index: usize,
    field: u8,
    terminal: bool,
}

impl DrawFillDigestAuthority {
    fn new() -> Self {
        Self { phase: 0, index: 0, field: 0, terminal: false }
    }

    fn step(&mut self, value: Option<&FillStyle>, digest: &mut store::ArtifactStoreInitializationDigest, credit: &mut DrawSemanticDigestCredit, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
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
        let value = value.ok_or("draw-store.digest-fill-missing")?;
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
                    let stop = stops.get(self.index).ok_or("draw-store.digest-linear-stop")?;
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
                    let stop = stops.get(self.index).ok_or("draw-store.digest-radial-stop")?;
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

struct DrawStrokeDigestAuthority {
    phase: u8,
    index: usize,
    terminal: bool,
}

impl DrawStrokeDigestAuthority {
    fn new() -> Self {
        Self { phase: 0, index: 0, terminal: false }
    }

    fn step(&mut self, value: Option<&StrokeStyle>, digest: &mut store::ArtifactStoreInitializationDigest, credit: &mut DrawSemanticDigestCredit, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
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
        let value = value.ok_or("draw-store.digest-stroke-missing")?;
        match self.phase {
            1..=4 => credit.scalar_f64(digest, 240 + u16::from(self.phase), value.color[(self.phase - 1) as usize], cx)?,
            5 => credit.scalar_f64(digest, 245, value.width, cx)?,
            6 => credit.observe_owned_string(digest, 246, &value.cap, true, cx)?,
            7 => credit.observe_owned_string(digest, 247, &value.join, true, cx)?,
            8 => credit.observe(digest, 248, &[u8::from(value.dash.is_some())], cx)?,
            9 => {
                let dash = value.dash.as_ref().ok_or("draw-store.digest-dash-missing")?;
                credit.source_vec(dash)?;
                credit.derived_vec(dash)?;
                credit.scalar_usize(digest, 249, dash.len(), cx)?;
                self.terminal = dash.is_empty();
            }
            _ => {
                let dash = value.dash.as_ref().ok_or("draw-store.digest-dash-missing")?;
                let item = *dash.get(self.index).ok_or("draw-store.digest-dash-index")?;
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

struct DrawPathSegmentDigestAuthority {
    phase: u8,
    terminal: bool,
}

impl DrawPathSegmentDigestAuthority {
    fn new() -> Self {
        Self { phase: 0, terminal: false }
    }

    fn step(&mut self, value: &PathSegment, digest: &mut store::ArtifactStoreInitializationDigest, credit: &mut DrawSemanticDigestCredit, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
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

struct DrawLayerVariantDigestAuthority {
    phase: u8,
    index: usize,
    field: u8,
    segment: Option<DrawPathSegmentDigestAuthority>,
    terminal: bool,
}

impl DrawLayerVariantDigestAuthority {
    fn new() -> Self {
        Self { phase: 1, index: 0, field: 0, segment: None, terminal: false }
    }

    fn option(&mut self, digest: &mut store::ArtifactStoreInitializationDigest, credit: &mut DrawSemanticDigestCredit, tag: u16, present: bool, next: u8, absent: u8, cx: &mut semio_framework_job::StepContext<'_>) -> Result<(), &'static str> {
        credit.observe(digest, tag, &[u8::from(present)], cx)?;
        self.phase = if present { next } else { absent };
        Ok(())
    }

    fn step(&mut self, layer: &DrawLayerNode, digest: &mut store::ArtifactStoreInitializationDigest, credit: &mut DrawSemanticDigestCredit, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        match layer {
            DrawLayerNode::Shape(value) => match self.phase {
                1 => {
                    credit.observe_owned_string(digest, 340, &value.shape_kind, true, cx)?;
                    self.phase = 2;
                }
                2 => self.option(digest, credit, 341, value.rect.is_some(), 3, 7, cx)?,
                3..=6 => {
                    let rect = value.rect.as_ref().ok_or("draw-store.digest-rect-missing")?;
                    credit.scalar_f64(digest, 342 + u16::from(self.phase - 3), [rect.x, rect.y, rect.width, rect.height][(self.phase - 3) as usize], cx)?;
                    self.phase += 1;
                }
                7 => self.option(digest, credit, 346, value.ellipse.is_some(), 8, 12, cx)?,
                8..=11 => {
                    let ellipse = value.ellipse.as_ref().ok_or("draw-store.digest-ellipse-missing")?;
                    credit.scalar_f64(digest, 347 + u16::from(self.phase - 8), [ellipse.cx, ellipse.cy, ellipse.rx, ellipse.ry][(self.phase - 8) as usize], cx)?;
                    self.phase += 1;
                }
                12 => self.option(digest, credit, 351, value.circle.is_some(), 13, 16, cx)?,
                13..=15 => {
                    let circle = value.circle.as_ref().ok_or("draw-store.digest-circle-missing")?;
                    credit.scalar_f64(digest, 352 + u16::from(self.phase - 13), [circle.cx, circle.cy, circle.r][(self.phase - 13) as usize], cx)?;
                    self.phase += 1;
                }
                16 => self.option(digest, credit, 355, value.line.is_some(), 17, 21, cx)?,
                17..=20 => {
                    let line = value.line.as_ref().ok_or("draw-store.digest-line-missing")?;
                    credit.scalar_f64(digest, 356 + u16::from(self.phase - 17), [line.x1, line.y1, line.x2, line.y2][(self.phase - 17) as usize], cx)?;
                    self.phase += 1;
                }
                21 => self.option(digest, credit, 360, value.polygon.is_some(), 22, 24, cx)?,
                22 => {
                    let points = &value.polygon.as_ref().ok_or("draw-store.digest-polygon-missing")?.points;
                    credit.source_vec(points)?;
                    credit.derived_vec(points)?;
                    credit.scalar_usize(digest, 361, points.len(), cx)?;
                    self.phase = 23;
                    self.terminal = points.is_empty();
                }
                23 => {
                    let points = &value.polygon.as_ref().ok_or("draw-store.digest-polygon-missing")?.points;
                    let point = points.get(self.index).ok_or("draw-store.digest-point-index")?;
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
            DrawLayerNode::Path(value) => match self.phase {
                1 => {
                    credit.source_vec(&value.segments)?;
                    credit.derived_vec(&value.segments)?;
                    credit.scalar_usize(digest, 370, value.segments.len(), cx)?;
                    self.phase = 2;
                    self.terminal = value.segments.is_empty();
                }
                _ => {
                    let segment = value.segments.get(self.index).ok_or("draw-store.digest-segment-index")?;
                    let cursor = self.segment.get_or_insert_with(DrawPathSegmentDigestAuthority::new);
                    if cursor.step(segment, digest, credit, cx)? {
                        self.segment = None;
                        self.index += 1;
                        self.terminal = self.index == value.segments.len();
                    }
                }
            },
            DrawLayerNode::Text(value) => {
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
            DrawLayerNode::Image(value) => {
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
            DrawLayerNode::Group(value) => {
                credit.source_vec(&value.children)?;
                credit.derived_vec(&value.children)?;
                credit.scalar_usize(digest, 400, value.children.len(), cx)?;
                self.terminal = true;
            }
            DrawLayerNode::Boolean(value) => match self.phase {
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
                    let item = value.children.get(self.index).ok_or("draw-store.digest-boolean-child")?;
                    credit.observe_owned_string(digest, 412, item, true, cx)?;
                    self.index += 1;
                    self.terminal = self.index == value.children.len();
                }
            },
            DrawLayerNode::Trace(value) => {
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

struct DrawLayerDigestAuthority {
    depth: usize,
    path: [usize; DRAW_MAXIMUM_LAYER_DEPTH],
    frames: [DrawTraversalFrame; DRAW_MAXIMUM_LAYER_DEPTH],
    fill: Option<DrawFillDigestAuthority>,
    stroke: Option<DrawStrokeDigestAuthority>,
    variant: Option<DrawLayerVariantDigestAuthority>,
    terminal: bool,
}

impl DrawLayerDigestAuthority {
    fn new() -> Self {
        Self { depth: 0, path: [0; DRAW_MAXIMUM_LAYER_DEPTH], frames: [DrawTraversalFrame::EMPTY; DRAW_MAXIMUM_LAYER_DEPTH], fill: None, stroke: None, variant: None, terminal: false }
    }

    fn step(&mut self, root: &DrawLayerNode, digest: &mut store::ArtifactStoreInitializationDigest, credit: &mut DrawSemanticDigestCredit, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        let node = DrawSnapshotBoundsAuthority::layer_at(root, &self.path[..self.depth]).ok_or("draw-store.digest-layer-path")?;
        let base = crate::artifacts::draw::schema::layer_base(node);
        let phase = self.frames[self.depth].phase;
        match phase {
            0 => {
                credit.add_source_owner(1, std::mem::size_of::<DrawLayerNode>())?;
                credit.add_derived_owner(1, std::mem::size_of::<DrawLayerNode>())?;
                let variant = match node {
                    DrawLayerNode::Shape(_) => 1,
                    DrawLayerNode::Path(_) => 2,
                    DrawLayerNode::Text(_) => 3,
                    DrawLayerNode::Image(_) => 4,
                    DrawLayerNode::Group(_) => 5,
                    DrawLayerNode::Boolean(_) => 6,
                    DrawLayerNode::Trace(_) => 7,
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
                let fill = self.fill.get_or_insert_with(DrawFillDigestAuthority::new);
                if fill.step(base.attributes.fill.as_ref(), digest, credit, cx)? {
                    self.fill = None;
                    self.frames[self.depth].phase = 13;
                }
            }
            13 => {
                let stroke = self.stroke.get_or_insert_with(DrawStrokeDigestAuthority::new);
                if stroke.step(base.attributes.stroke.as_ref(), digest, credit, cx)? {
                    self.stroke = None;
                    self.frames[self.depth].phase = 14;
                }
            }
            14 => {
                let variant = self.variant.get_or_insert_with(DrawLayerVariantDigestAuthority::new);
                if variant.step(node, digest, credit, cx)? {
                    self.variant = None;
                    self.frames[self.depth].phase = 15;
                }
            }
            15 => {
                if let DrawLayerNode::Group(group) = node {
                    let child = self.frames[self.depth].child;
                    if child < group.children.len() {
                        if self.depth + 1 >= DRAW_MAXIMUM_LAYER_DEPTH {
                            return Err("draw-store.digest-layer-depth");
                        }
                        self.frames[self.depth].child += 1;
                        self.path[self.depth] = child;
                        self.depth += 1;
                        self.frames[self.depth] = DrawTraversalFrame::EMPTY;
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

struct DrawMutationDigestAuthority {
    layer: Option<DrawLayerDigestAuthority>,
    fill: Option<DrawFillDigestAuthority>,
    stroke: Option<DrawStrokeDigestAuthority>,
    credit: DrawSemanticDigestCredit,
    phase: u8,
    terminal: bool,
}

impl DrawMutationDigestAuthority {
    fn new() -> Self {
        Self { layer: None, fill: None, stroke: None, credit: DrawSemanticDigestCredit::default(), phase: 0, terminal: false }
    }

    fn variant(mutation: &DrawMutation) -> u8 {
        match mutation {
            DrawMutation::SetLayerVisible(_) => 1,
            DrawMutation::SetLayerLocked(_) => 2,
            DrawMutation::SetLayerOpacity(_) => 3,
            DrawMutation::SetLayerBlendMode(_) => 4,
            DrawMutation::RenameLayer(_) => 5,
            DrawMutation::UpdateLayerTransform(_) => 6,
            DrawMutation::ReplaceLayerFill(_) => 7,
            DrawMutation::ReplaceLayerStroke(_) => 8,
            DrawMutation::SetLayerBooleanOperation(_) => 9,
            DrawMutation::UpdateLayerTraceParams(_) => 10,
            DrawMutation::CreateLayer(_) => 11,
            DrawMutation::DuplicateLayer(_) => 12,
            DrawMutation::DeleteLayer(_) => 13,
            DrawMutation::ReorderLayer(_) => 14,
        }
    }

    fn finish(&mut self, digest: &mut store::ArtifactStoreInitializationDigest, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        self.credit.seal(digest, cx)?;
        self.terminal = true;
        Ok(true)
    }

    fn step(&mut self, mutation: &DrawMutation, digest: &mut store::ArtifactStoreInitializationDigest, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
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
                DrawMutation::CreateLayer(value) => {
                    self.credit.add_source_owner(1, std::mem::size_of::<Box<DrawLayerNode>>())?;
                    self.credit.add_derived_owner(1, std::mem::size_of::<Box<DrawLayerNode>>())?;
                    self.credit.observe(digest, 2, &[u8::from(value.parent_id.is_some())], cx)?;
                    self.phase = if value.parent_id.is_some() { 2 } else { 3 };
                }
                _ => {
                    let target = DrawMutationCandidateAuthority::target_owner(mutation).ok_or("draw-store.mutation-target-owner")?;
                    self.credit.observe_owned_string(digest, 2, target, false, cx)?;
                    self.phase = 2;
                }
            }
            return Ok(false);
        }
        match mutation {
            DrawMutation::SetLayerVisible(value) => {
                if self.phase == 2 {
                    self.credit.observe(digest, 3, &[u8::from(value.visible)], cx)?;
                    self.phase = 3;
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawMutation::SetLayerLocked(value) => {
                if self.phase == 2 {
                    self.credit.observe(digest, 3, &[u8::from(value.locked)], cx)?;
                    self.phase = 3;
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawMutation::SetLayerOpacity(value) => {
                if self.phase == 2 {
                    self.credit.scalar_f64(digest, 3, value.opacity, cx)?;
                    self.phase = 3;
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawMutation::SetLayerBlendMode(value) => {
                if self.phase == 2 {
                    self.credit.observe_owned_string(digest, 3, &value.blend_mode, true, cx)?;
                    self.phase = 3;
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawMutation::RenameLayer(value) => {
                if self.phase == 2 {
                    self.credit.observe_owned_string(digest, 3, &value.new_name, true, cx)?;
                    self.phase = 3;
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawMutation::UpdateLayerTransform(value) => {
                if self.phase <= 6 {
                    let fields = [value.transform.x, value.transform.y, value.transform.scale_x, value.transform.scale_y, value.transform.rotation];
                    self.credit.scalar_f64(digest, 3 + u16::from(self.phase - 2), fields[(self.phase - 2) as usize], cx)?;
                    self.phase += 1;
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawMutation::ReplaceLayerFill(value) => {
                if self.phase == 2 {
                    let fill = self.fill.get_or_insert_with(DrawFillDigestAuthority::new);
                    if fill.step(value.fill.as_ref(), digest, &mut self.credit, cx)? {
                        self.fill = None;
                        self.phase = 3;
                    }
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawMutation::ReplaceLayerStroke(value) => {
                if self.phase == 2 {
                    let stroke = self.stroke.get_or_insert_with(DrawStrokeDigestAuthority::new);
                    if stroke.step(value.stroke.as_ref(), digest, &mut self.credit, cx)? {
                        self.stroke = None;
                        self.phase = 3;
                    }
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawMutation::SetLayerBooleanOperation(value) => {
                if self.phase == 2 {
                    self.credit.observe_owned_string(digest, 3, &value.boolean_operation, true, cx)?;
                    self.phase = 3;
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawMutation::UpdateLayerTraceParams(value) => {
                if self.phase <= 3 {
                    self.credit.scalar_f64(digest, 3 + u16::from(self.phase - 2), [value.params.threshold, value.params.simplify_epsilon][(self.phase - 2) as usize], cx)?;
                    self.phase += 1;
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawMutation::CreateLayer(value) => match self.phase {
                2 => {
                    self.credit.observe_owned_string(digest, 3, value.parent_id.as_ref().ok_or("draw-store.digest-parent-missing")?, false, cx)?;
                    self.phase = 3;
                    Ok(false)
                }
                3 => {
                    self.credit.observe(digest, 4, &[u8::from(value.index.is_some())], cx)?;
                    self.phase = if value.index.is_some() { 4 } else { 5 };
                    Ok(false)
                }
                4 => {
                    self.credit.scalar_usize(digest, 5, value.index.ok_or("draw-store.digest-index-missing")?, cx)?;
                    self.phase = 5;
                    Ok(false)
                }
                5 => {
                    let layer = self.layer.get_or_insert_with(DrawLayerDigestAuthority::new);
                    if layer.step(&value.layer, digest, &mut self.credit, cx)? {
                        self.layer = None;
                        self.phase = 6;
                    }
                    Ok(false)
                }
                _ => self.finish(digest, cx),
            },
            DrawMutation::DuplicateLayer(_) | DrawMutation::DeleteLayer(_) => self.finish(digest, cx),
            DrawMutation::ReorderLayer(value) => match self.phase {
                2 => {
                    self.credit.observe(digest, 3, &[u8::from(value.parent_id.is_some())], cx)?;
                    self.phase = if value.parent_id.is_some() { 3 } else { 4 };
                    Ok(false)
                }
                3 => {
                    self.credit.observe_owned_string(digest, 4, value.parent_id.as_ref().ok_or("draw-store.digest-parent-missing")?, false, cx)?;
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

    fn totals(&self) -> Option<DrawSemanticDigestTotals> {
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

impl Drop for DrawMutationDigestAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Draw mutation digest reached Drop before exact terminal close");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct DrawMutationAggregateReservation {
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

impl DrawMutationAggregateReservation {
    fn checked_total(values: &[usize], fault: &'static str) -> Result<usize, &'static str> {
        values.iter().try_fold(0usize, |total, value| total.checked_add(*value).ok_or(fault))
    }

    fn admit(
        source: DrawSnapshotOwnerTotals,
        mutation: DrawSemanticDigestTotals,
        operation: &DrawMutation,
        reverse_slots: usize,
        output_slots: usize,
        overlay_slots: usize,
        overlay_bytes: usize,
        duplicate_id_bytes: usize,
    ) -> Result<Self, &'static str> {
        let container_slots = reverse_slots.checked_add(output_slots).ok_or("draw-store.mutation-container-credit-overflow")?;
        let container_items = 2;
        let container_bytes = container_slots.checked_mul(std::mem::size_of::<DrawLayerNode>()).ok_or("draw-store.mutation-container-credit-overflow")?;
        let (duplicate_candidate_items, duplicate_candidate_bytes) = if matches!(operation, DrawMutation::DuplicateLayer(_)) { (1, duplicate_id_bytes) } else { (0, 0) };
        let authority_items = 1;
        let authority_bytes = std::mem::size_of::<DrawMutationCandidateAuthority>();
        let reservation = Self {
            source_items: source.source_items,
            candidate_items: source.candidate_items,
            mutation_source_items: mutation.source_owner_items,
            mutation_derived_items: mutation.derived_owner_items,
            duplicate_candidate_items,
            authority_items,
            container_items,
            page_items: DRAW_MUTATION_RETAINED_PAGE_ITEMS.checked_add(overlay_slots).ok_or("draw-store.mutation-overlay-item-overflow")?,
            source_bytes: source.source_bytes,
            candidate_bytes: source.candidate_bytes,
            mutation_source_bytes: mutation.source_owner_bytes,
            mutation_derived_bytes: mutation.derived_owner_bytes,
            duplicate_candidate_bytes,
            authority_bytes,
            container_bytes,
            page_bytes: DRAW_MUTATION_RETAINED_PAGE_BYTES.checked_add(overlay_bytes).ok_or("draw-store.mutation-overlay-byte-overflow")?,
            maximum_container: source.maximum_container,
            container_slots,
        };
        if reservation.total_items()? > DRAW_MUTATION_AGGREGATE_ITEMS {
            return Err("draw-store.mutation-aggregate-item-capacity");
        }
        if reservation.total_bytes()? > DRAW_MUTATION_AGGREGATE_BYTES {
            return Err("draw-store.mutation-aggregate-byte-capacity");
        }
        Ok(reservation)
    }

    fn total_items(&self) -> Result<usize, &'static str> {
        Self::checked_total(
            &[self.source_items, self.candidate_items, self.mutation_source_items, self.mutation_derived_items, self.duplicate_candidate_items, self.authority_items, self.container_items, self.page_items],
            "draw-store.mutation-item-overflow",
        )
    }

    fn total_bytes(&self) -> Result<usize, &'static str> {
        Self::checked_total(
            &[self.source_bytes, self.candidate_bytes, self.mutation_source_bytes, self.mutation_derived_bytes, self.duplicate_candidate_bytes, self.authority_bytes, self.container_bytes, self.page_bytes],
            "draw-store.mutation-byte-overflow",
        )
    }
}

struct DrawDuplicateRewriteAuthority {
    depth: usize,
    path: [usize; DRAW_MAXIMUM_LAYER_DEPTH],
    frames: [DrawTraversalFrame; DRAW_MAXIMUM_LAYER_DEPTH],
    material: [u8; DRAW_DUPLICATE_MATERIAL_BYTES],
    material_len: usize,
    id_len: usize,
    name_len: usize,
    hash_cursor: usize,
    hasher: Option<semio_framework_hash::Sha256>,
    pending_id: std::mem::ManuallyDrop<Option<String>>,
    pending_name: std::mem::ManuallyDrop<Option<String>>,
    terminal: bool,
}

impl DrawDuplicateRewriteAuthority {
    fn new(mut pending_id: String, mut pending_name: String) -> Self {
        pending_id.clear();
        pending_name.clear();
        Self {
            depth: 0,
            path: [0; DRAW_MAXIMUM_LAYER_DEPTH],
            frames: [DrawTraversalFrame::EMPTY; DRAW_MAXIMUM_LAYER_DEPTH],
            material: [0; DRAW_DUPLICATE_MATERIAL_BYTES],
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

    fn step(&mut self, root: &mut DrawLayerNode, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        let node = DrawLayerCloneAuthority::target_at_mut(root, &self.path[..self.depth]).ok_or("draw-store.duplicate-path")?;
        let prefix = match node {
            DrawLayerNode::Shape(_) => "shape",
            DrawLayerNode::Path(_) => "path",
            DrawLayerNode::Text(_) => "text",
            DrawLayerNode::Image(_) => "image",
            DrawLayerNode::Group(_) => "group",
            DrawLayerNode::Boolean(_) => "boolean",
            DrawLayerNode::Trace(_) => "trace",
        };
        let phase = self.frames[self.depth].phase;
        if phase <= 13 {
            let base = crate::artifacts::draw::schema::layer_base_mut(node);
            let suffix = if self.depth == 0 { " copy" } else { "" };
            match phase {
                0 => {
                    if base.id.len() > DRAW_OWNED_FIELD_BYTES {
                        return Err("draw-store.duplicate-id-capacity");
                    }
                    self.material[..base.id.len()].copy_from_slice(base.id.as_bytes());
                    self.id_len = base.id.len();
                    self.material_len = base.id.len();
                    self.frames[self.depth].phase = 1;
                    cx.consume_fuel(base.id.len().max(1) as u64);
                }
                1 => {
                    let total = self.material_len.checked_add(base.name.len()).ok_or("draw-store.duplicate-byte-overflow")?;
                    if base.name.len() > DRAW_OWNED_FIELD_BYTES || total > self.material.len() {
                        return Err("draw-store.duplicate-name-capacity");
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
                    self.hasher.as_mut().ok_or("draw-store.duplicate-hasher-missing")?.update(b"semio.draw.duplicate-id.v1");
                    self.frames[self.depth].phase = 4;
                    cx.consume_fuel(1);
                }
                4 => {
                    self.hasher.as_mut().ok_or("draw-store.duplicate-hasher-missing")?.update(&(self.id_len as u64).to_be_bytes());
                    self.hash_cursor = 0;
                    self.frames[self.depth].phase = 5;
                    cx.consume_fuel(1);
                }
                5 => {
                    let end = self.id_len.min(self.hash_cursor + DRAW_OWNED_FIELD_BYTES);
                    self.hasher.as_mut().ok_or("draw-store.duplicate-hasher-missing")?.update(&self.material[self.hash_cursor..end]);
                    let consumed = end.saturating_sub(self.hash_cursor);
                    self.hash_cursor = end;
                    if self.hash_cursor == self.id_len {
                        self.frames[self.depth].phase = 6;
                    }
                    cx.consume_fuel(consumed.max(1) as u64);
                }
                6 => {
                    self.hasher.as_mut().ok_or("draw-store.duplicate-hasher-missing")?.update(&(self.name_len as u64).to_be_bytes());
                    self.hash_cursor = self.id_len;
                    self.frames[self.depth].phase = 7;
                    cx.consume_fuel(1);
                }
                7 => {
                    let end = self.material_len.min(self.hash_cursor + DRAW_OWNED_FIELD_BYTES);
                    self.hasher.as_mut().ok_or("draw-store.duplicate-hasher-missing")?.update(&self.material[self.hash_cursor..end]);
                    let consumed = end.saturating_sub(self.hash_cursor);
                    self.hash_cursor = end;
                    if self.hash_cursor == self.material_len {
                        self.frames[self.depth].phase = 8;
                    }
                    cx.consume_fuel(consumed.max(1) as u64);
                }
                8 => {
                    let hash = self.hasher.take().ok_or("draw-store.duplicate-hasher-missing")?.finalize();
                    let capacity = prefix.len().checked_add(65).ok_or("draw-store.duplicate-id-overflow")?;
                    let mut id = self.pending_id.take().ok_or("draw-store.duplicate-id-owner-missing")?;
                    if id.capacity() < capacity {
                        *self.pending_id = Some(id);
                        return Err("draw-store.duplicate-id-owner-capacity");
                    }
                    id.push_str(prefix);
                    id.push('-');
                    const HEX: &[u8; 16] = b"0123456789abcdef";
                    for byte in hash {
                        id.push(HEX[(byte >> 4) as usize] as char);
                        id.push(HEX[(byte & 0x0f) as usize] as char);
                    }
                    let name_capacity = base.name.len().checked_add(suffix.len()).ok_or("draw-store.duplicate-name-overflow")?;
                    let name_owner = self.pending_name.as_ref().ok_or("draw-store.duplicate-name-owner-missing")?;
                    if base.id.capacity() < id.len() || base.name.capacity() < name_capacity || name_owner.capacity() < name_capacity {
                        *self.pending_id = Some(id);
                        return Err("draw-store.duplicate-destination-capacity");
                    }
                    *self.pending_id = Some(id);
                    self.frames[self.depth].phase = 9;
                    cx.consume_fuel(1);
                }
                9 => {
                    let pending = self.pending_id.as_mut().ok_or("draw-store.duplicate-id-missing")?;
                    base.id.clear();
                    base.id.push_str(pending);
                    pending.clear();
                    self.frames[self.depth].phase = 10;
                    cx.consume_fuel(1);
                }
                10 => {
                    let pending = self.pending_name.as_mut().ok_or("draw-store.duplicate-name-owner-missing")?;
                    pending.clear();
                    pending.push_str(&base.name);
                    self.frames[self.depth].phase = 11;
                    cx.consume_fuel(base.name.len().max(1) as u64);
                }
                11 => {
                    self.pending_name.as_mut().ok_or("draw-store.duplicate-name-owner-missing")?.push_str(suffix);
                    self.frames[self.depth].phase = 12;
                    cx.consume_fuel(suffix.len().max(1) as u64);
                }
                12 => {
                    let pending = self.pending_name.as_mut().ok_or("draw-store.duplicate-name-owner-missing")?;
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

impl Drop for DrawDuplicateRewriteAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Draw duplicate rewrite reached Drop before staged id/name retirement");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DrawMutationCandidatePhase {
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
enum DrawContainerRebuildRole {
    Source,
    Destination,
    CloseSourceUndo,
}

#[derive(Clone, Copy)]
struct DrawContainerSourceUndo {
    parent: Option<DrawLayerAddress>,
    index: usize,
}

struct DrawMutationOverlayPatch {
    source_owner: usize,
    committed: bool,
}

impl DrawMutationOverlayPatch {
    fn bind(source: &DrawSnapshot) -> Self {
        Self { source_owner: std::ptr::from_ref(source) as usize, committed: false }
    }

    fn validate(&self, source: &DrawSnapshot) -> Result<(), &'static str> {
        if self.source_owner != std::ptr::from_ref(source) as usize {
            return Err("draw-store.mutation-overlay-owner-changed");
        }
        Ok(())
    }

    fn commit(&mut self, source: &DrawSnapshot) -> Result<(), &'static str> {
        self.validate(source)?;
        self.committed = true;
        Ok(())
    }
}

struct DrawMutationCandidateAuthority {
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    arena_pool: Option<std::sync::Arc<DrawMutationArenaPool>>,
    arena_slot: usize,
    arena_generation: u64,
    arena_return_phase: u8,
    preflight_source: Option<DrawSnapshotBoundsAuthority>,
    preflight_mutation: std::mem::ManuallyDrop<Option<DrawMutationDigestAuthority>>,
    preflight_digest: Option<store::ArtifactStoreInitializationDigest>,
    reservation: Option<DrawMutationAggregateReservation>,
    overlay: Option<DrawMutationOverlayPatch>,
    locator: Option<DrawLayerLocator>,
    primary: Option<DrawLayerAddress>,
    secondary: Option<DrawLayerAddress>,
    layer_clone: std::mem::ManuallyDrop<Option<Box<DrawLayerCloneAuthority>>>,
    fill_clone: std::mem::ManuallyDrop<Option<DrawFillCloneAuthority>>,
    stroke_clone: std::mem::ManuallyDrop<Option<DrawStrokeCloneAuthority>>,
    duplicate_rewrite: Option<DrawDuplicateRewriteAuthority>,
    duplicate_id_owner: std::mem::ManuallyDrop<Option<String>>,
    rebuild: std::mem::ManuallyDrop<Option<DrawContainerRebuildAuthority>>,
    rebuild_target: Option<Option<DrawLayerAddress>>,
    rebuild_role: Option<DrawContainerRebuildRole>,
    rebuild_close_phase: u8,
    source_undo: Option<DrawContainerSourceUndo>,
    container_reverse: std::mem::ManuallyDrop<Option<Vec<DrawLayerNode>>>,
    container_output: std::mem::ManuallyDrop<Option<Vec<DrawLayerNode>>>,
    overlay_pages: std::mem::ManuallyDrop<Option<Vec<String>>>,
    pending_layer: std::mem::ManuallyDrop<Option<DrawLayerNode>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    phase: DrawMutationCandidatePhase,
    terminal: bool,
    fault: Option<&'static str>,
}

impl DrawMutationCandidateAuthority {
    fn try_new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Result<Self, &'static str> {
        let (arena_pool, arena_slot, arena_generation, owner) = borrow_draw_mutation_arena().map_err(DrawMutationArenaBorrowError::as_str)?;
        Ok(Self::from_arena(operation, generation, arena_pool, arena_slot, arena_generation, owner))
    }

    fn try_new_from_pool(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, pool: std::sync::Arc<DrawMutationArenaPool>) -> Result<Self, &'static str> {
        let (arena_pool, arena_slot, arena_generation, owner) = borrow_draw_mutation_arena_from(pool)?;
        Ok(Self::from_arena(operation, generation, arena_pool, arena_slot, arena_generation, owner))
    }

    fn from_arena(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, arena_pool: std::sync::Arc<DrawMutationArenaPool>, arena_slot: usize, arena_generation: u64, owner: DrawMutationArenaOwner) -> Self {
        Self {
            operation,
            generation,
            arena_pool: Some(arena_pool),
            arena_slot,
            arena_generation,
            arena_return_phase: 0,
            preflight_source: Some(DrawSnapshotBoundsAuthority::new()),
            preflight_mutation: std::mem::ManuallyDrop::new(Some(DrawMutationDigestAuthority::new())),
            preflight_digest: Some(store::ArtifactStoreInitializationDigest::new(b"draw.mutation-preflight")),
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
            phase: DrawMutationCandidatePhase::PreflightSource,
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

    fn target_owner(mutation: &DrawMutation) -> Option<&String> {
        match mutation {
            DrawMutation::SetLayerVisible(value) => Some(&value.layer_id),
            DrawMutation::SetLayerLocked(value) => Some(&value.layer_id),
            DrawMutation::SetLayerOpacity(value) => Some(&value.layer_id),
            DrawMutation::SetLayerBlendMode(value) => Some(&value.layer_id),
            DrawMutation::RenameLayer(value) => Some(&value.layer_id),
            DrawMutation::UpdateLayerTransform(value) => Some(&value.layer_id),
            DrawMutation::ReplaceLayerFill(value) => Some(&value.layer_id),
            DrawMutation::ReplaceLayerStroke(value) => Some(&value.layer_id),
            DrawMutation::SetLayerBooleanOperation(value) => Some(&value.layer_id),
            DrawMutation::UpdateLayerTraceParams(value) => Some(&value.layer_id),
            DrawMutation::CreateLayer(_) => None,
            DrawMutation::DuplicateLayer(value) => Some(&value.layer_id),
            DrawMutation::DeleteLayer(value) => Some(&value.layer_id),
            DrawMutation::ReorderLayer(value) => Some(&value.layer_id),
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

    fn return_arena_owner(&mut self) -> Result<Option<bool>, &'static str> {
        let Some(pool) = self.arena_pool.as_ref() else {
            return Ok(Some(true));
        };
        let Ok(mut state) = pool.state.try_lock() else {
            return Ok(None);
        };
        let slot = state.slots.get_mut(self.arena_slot).ok_or("draw-store.mutation-arena-slot")?;
        if !slot.leased || slot.generation != self.arena_generation {
            return Err("draw-store.mutation-arena-stale-generation");
        }
        match self.arena_return_phase {
            0 => {
                let reverse = self.container_reverse.as_ref().ok_or("draw-store.mutation-reverse-arena-missing")?;
                if !reverse.is_empty() || reverse.capacity() < DRAW_MUTATION_CONTAINER_SLOT_CAPACITY || slot.reverse.is_some() {
                    return Err("draw-store.mutation-reverse-arena-not-terminal");
                }
                slot.reverse = Some(self.container_reverse.take().expect("validated Draw reverse owner remains retained"));
            }
            1 => {
                let output = self.container_output.as_ref().ok_or("draw-store.mutation-output-arena-missing")?;
                if !output.is_empty() || output.capacity() < DRAW_MUTATION_CONTAINER_SLOT_CAPACITY || slot.output.is_some() {
                    return Err("draw-store.mutation-output-arena-not-terminal");
                }
                slot.output = Some(self.container_output.take().expect("validated Draw output owner remains retained"));
            }
            2 => {
                let pages = self.overlay_pages.as_ref().ok_or("draw-store.mutation-overlay-arena-missing")?;
                if pages.len() != DRAW_MUTATION_OVERLAY_PAGE_CAPACITY
                    || pages.capacity() < DRAW_MUTATION_OVERLAY_PAGE_CAPACITY
                    || pages.iter().any(|page| !page.is_empty() || page.capacity() < DRAW_MUTATION_RETAINED_PAGE_BYTES)
                    || slot.pages.is_some()
                {
                    return Err("draw-store.mutation-overlay-arena-not-terminal");
                }
                slot.pages = Some(self.overlay_pages.take().expect("validated Draw page owner remains retained"));
            }
            3 => {
                let duplicate_id = self.duplicate_id_owner.as_ref().ok_or("draw-store.mutation-duplicate-id-owner-missing")?;
                if !duplicate_id.is_empty() || duplicate_id.capacity() < DRAW_DUPLICATE_ID_BYTES || slot.duplicate_id.is_some() {
                    return Err("draw-store.mutation-duplicate-id-owner-not-terminal");
                }
                slot.duplicate_id = Some(self.duplicate_id_owner.take().expect("validated Draw duplicate owner remains retained"));
                slot.leased = false;
            }
            _ => return Err("draw-store.mutation-arena-return-phase"),
        }
        self.arena_return_phase += 1;
        let complete = self.arena_return_phase == 4;
        if complete && !slot.is_available() {
            return Err("draw-store.mutation-arena-return-false-terminal");
        }
        drop(state);
        if complete {
            self.arena_pool = None;
        }
        Ok(Some(complete))
    }

    fn write_overlay_string(&mut self, target: &mut String, source: &str) -> Result<(), &'static str> {
        if source.len() > DRAW_MUTATION_RETAINED_PAGE_BYTES {
            return Err("draw-store.mutation-overlay-string-capacity");
        }
        let pages = self.overlay_pages.as_mut().ok_or("draw-store.mutation-overlay-arena-missing")?;
        let mut page = pages.pop().ok_or("draw-store.mutation-overlay-slot-capacity")?;
        if page.capacity() < source.len() || target.capacity() < source.len() {
            pages.push(page);
            return Err("draw-store.mutation-overlay-destination-capacity");
        }
        page.push_str(source);
        target.clear();
        target.push_str(&page);
        page.clear();
        pages.push(page);
        Ok(())
    }

    fn start_rebuild(&mut self, source: &mut DrawSnapshot, parent: Option<DrawLayerAddress>, remove_index: Option<usize>, insert_index: Option<usize>, role: DrawContainerRebuildRole) -> Result<(), &'static str> {
        let reservation = self.reservation.ok_or("draw-store.mutation-reservation-missing")?;
        if self.container_reverse.is_none() {
            return Err("draw-store.mutation-reverse-arena-missing");
        }
        if self.container_output.is_none() {
            return Err("draw-store.mutation-output-arena-missing");
        }
        let container = DrawLayerLocator::container_mut(source, parent).ok_or("draw-store.mutation-container-missing")?;
        let source = std::mem::take(container);
        let pending = self.pending_layer.take();
        let reverse = self.container_reverse.take().expect("validated Draw reverse arena remains retained");
        let output = self.container_output.take().expect("validated Draw output arena remains retained");
        match DrawContainerRebuildAuthority::new(source, remove_index, insert_index, pending, reverse, output, reservation) {
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
                Err("draw-store.mutation-container-admission")
            }
        }
    }

    fn finish_rebuild(&mut self, source: &mut DrawSnapshot, parent: Option<DrawLayerAddress>) -> Result<Option<DrawLayerNode>, &'static str> {
        if DrawLayerLocator::container_mut(source, parent).is_none() {
            return Err("draw-store.mutation-container-lost");
        }
        let (rebuilt_source, removed, reverse_arena, output_arena) = self.rebuild.as_mut().ok_or("draw-store.mutation-rebuild-missing")?.take().ok_or("draw-store.mutation-rebuild-false-terminal")?;
        *DrawLayerLocator::container_mut(source, parent).expect("validated Draw mutation container remains available") = rebuilt_source;
        *self.container_reverse = Some(reverse_arena);
        *self.container_output = Some(output_arena);
        let mut rebuild = self.rebuild.take().expect("Draw completed rebuild remains exact");
        rebuild.terminal = true;
        drop(rebuild);
        self.rebuild_target = None;
        self.rebuild_role = None;
        self.rebuild_close_phase = 0;
        Ok(removed)
    }

    fn step(&mut self, source: &mut DrawSnapshot, mutation: &DrawMutation, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if (cx.operation() != self.operation || cx.generation() != self.generation) && !self.overlay.as_ref().is_some_and(|overlay| overlay.committed) {
            return self.fail("draw-store.mutation-candidate-stale-authority");
        }
        if cx.is_cancelled() && !self.overlay.as_ref().is_some_and(|overlay| overlay.committed) {
            self.fault = Some("draw-store.mutation-candidate-cancelled");
            self.phase = DrawMutationCandidatePhase::Retire;
            return Err("draw-store.mutation-candidate-cancelled");
        }
        if let Some(overlay) = self.overlay.as_ref() {
            overlay.validate(source)?;
        }
        match self.phase {
            DrawMutationCandidatePhase::PreflightSource => {
                let bounds = self.preflight_source.as_mut().ok_or("draw-store.mutation-source-preflight-missing")?;
                if bounds.step(source, cx)? {
                    self.phase = DrawMutationCandidatePhase::PreflightMutation;
                }
                Ok(false)
            }
            DrawMutationCandidatePhase::PreflightMutation => {
                let digest = self.preflight_mutation.as_mut().ok_or("draw-store.mutation-preflight-missing")?;
                if !digest.step(mutation, self.preflight_digest.as_mut().ok_or("draw-store.mutation-preflight-digest")?, cx)? {
                    return Ok(false);
                }
                let source_credit = self.preflight_source.as_ref().and_then(DrawSnapshotBoundsAuthority::totals).ok_or("draw-store.mutation-source-preflight-incomplete")?;
                let mutation_credit = digest.totals().ok_or("draw-store.mutation-preflight-incomplete")?;
                let reverse_slots = self.container_reverse.as_ref().ok_or("draw-store.mutation-reverse-arena-missing")?.capacity();
                let output_slots = self.container_output.as_ref().ok_or("draw-store.mutation-output-arena-missing")?.capacity();
                let overlay_pages = self.overlay_pages.as_ref().ok_or("draw-store.mutation-overlay-arena-missing")?;
                let overlay_slots = overlay_pages.capacity();
                if mutation_credit.derived_owner_items > overlay_slots {
                    return Err("draw-store.mutation-overlay-slot-capacity");
                }
                let overlay_bytes = overlay_pages
                    .iter()
                    .try_fold(overlay_slots.checked_mul(std::mem::size_of::<String>()).ok_or("draw-store.mutation-overlay-byte-overflow")?, |total, page| total.checked_add(page.capacity()).ok_or("draw-store.mutation-overlay-byte-overflow"))?;
                let duplicate_id_bytes = self.duplicate_id_owner.as_ref().map_or(0, |value| std::mem::size_of::<String>().saturating_add(value.capacity()));
                self.reservation = Some(DrawMutationAggregateReservation::admit(source_credit, mutation_credit, mutation, reverse_slots, output_slots, overlay_slots, overlay_bytes, duplicate_id_bytes)?);
                drop(self.preflight_mutation.take());
                self.preflight_source = None;
                self.preflight_digest = None;
                self.phase = DrawMutationCandidatePhase::BindOverlay;
                Ok(false)
            }
            DrawMutationCandidatePhase::BindOverlay => {
                self.overlay = Some(DrawMutationOverlayPatch::bind(source));
                self.locator = Some(DrawLayerLocator::new());
                self.phase = DrawMutationCandidatePhase::LocatePrimary;
                cx.consume_fuel(1);
                Ok(false)
            }
            DrawMutationCandidatePhase::LocatePrimary => {
                let locator = self.locator.as_mut().ok_or("draw-store.mutation-locator-missing")?;
                if !locator.step(source, Self::target(mutation), cx)? {
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
                let locator = self.locator.as_mut().ok_or("draw-store.mutation-parent-locator")?;
                if !locator.step(source, target, cx)? {
                    return Ok(false);
                }
                self.secondary = locator.found();
                self.locator = None;
                let Some(address) = self.secondary else { return Err("draw-store.mutation-parent-not-found") };
                if !matches!(DrawLayerLocator::node_at(source, address), Some(DrawLayerNode::Group(_))) {
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
                        if !clone.step(&value.layer, self.preflight_digest.get_or_insert_with(|| store::ArtifactStoreInitializationDigest::new(b"draw.create-layer")), cx)? {
                            return Ok(false);
                        }
                        *self.pending_layer = clone.take();
                        drop(self.layer_clone.take());
                    }
                    DrawMutation::DuplicateLayer(_) => {
                        let duplicate_source = DrawLayerLocator::node_at(source, self.primary.ok_or("draw-store.mutation-primary-missing")?).ok_or("draw-store.mutation-duplicate-source")?;
                        if self.pending_layer.is_none() {
                            if self.layer_clone.is_none() {
                                *self.layer_clone = Some(Box::new(DrawLayerCloneAuthority::new(duplicate_source)?));
                                cx.consume_fuel(1);
                                return Ok(false);
                            }
                            let clone = self.layer_clone.as_mut().expect("Draw duplicate layer clone remains retained");
                            if !clone.step(duplicate_source, self.preflight_digest.get_or_insert_with(|| store::ArtifactStoreInitializationDigest::new(b"draw.duplicate-layer")), cx)? {
                                return Ok(false);
                            }
                            *self.pending_layer = clone.take();
                            drop(self.layer_clone.take());
                            let name_owner = self.overlay_pages.as_mut().ok_or("draw-store.mutation-overlay-arena-missing")?.pop().ok_or("draw-store.duplicate-name-owner-missing")?;
                            self.duplicate_rewrite = Some(DrawDuplicateRewriteAuthority::new(self.duplicate_id_owner.take().ok_or("draw-store.duplicate-id-owner-missing")?, name_owner));
                            return Ok(false);
                        }
                        if !self.duplicate_rewrite.as_mut().ok_or("draw-store.duplicate-rewrite-missing")?.step(self.pending_layer.as_mut().ok_or("draw-store.duplicate-owner-missing")?, cx)? {
                            return Ok(false);
                        }
                        let pages = self.overlay_pages.as_mut().ok_or("draw-store.mutation-overlay-arena-missing")?;
                        if pages.len() >= pages.capacity() {
                            return Err("draw-store.duplicate-name-owner-return-saturated");
                        }
                        let (id_owner, name_owner) = self.duplicate_rewrite.as_mut().and_then(DrawDuplicateRewriteAuthority::take_owners).ok_or("draw-store.duplicate-owner-false-terminal")?;
                        *self.duplicate_id_owner = Some(id_owner);
                        pages.push(name_owner);
                        drop(self.duplicate_rewrite.take());
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
                            None => DrawLayerLocator::container_mut(source, parent).map_or(0, |values| values.len()),
                        };
                        self.start_rebuild(source, parent, None, Some(index), DrawContainerRebuildRole::Destination)?;
                        self.phase = DrawMutationCandidatePhase::RebuildDestination;
                        return Ok(false);
                    }
                    DrawMutation::DuplicateLayer(_) => {
                        let address = self.primary.ok_or("draw-store.mutation-primary-missing")?;
                        self.start_rebuild(source, address.parent(), None, Some(address.index() + 1), DrawContainerRebuildRole::Destination)?;
                        self.phase = DrawMutationCandidatePhase::RebuildDestination;
                        return Ok(false);
                    }
                    DrawMutation::DeleteLayer(_) | DrawMutation::ReorderLayer(_) => {
                        let address = self.primary.ok_or("draw-store.mutation-primary-missing")?;
                        self.start_rebuild(source, address.parent(), Some(address.index()), None, DrawContainerRebuildRole::Source)?;
                        self.source_undo = Some(DrawContainerSourceUndo { parent: address.parent(), index: address.index() });
                        self.phase = DrawMutationCandidatePhase::RebuildSource;
                        return Ok(false);
                    }
                    _ => {}
                }
                let address = self.primary;
                match mutation {
                    DrawMutation::SetLayerVisible(value) => {
                        crate::artifacts::draw::schema::layer_base_mut(DrawLayerLocator::node_at_mut(source, address.ok_or("draw-store.mutation-primary-missing")?).ok_or("draw-store.mutation-target-lost")?).visible = value.visible
                    }
                    DrawMutation::SetLayerLocked(value) => {
                        crate::artifacts::draw::schema::layer_base_mut(DrawLayerLocator::node_at_mut(source, address.ok_or("draw-store.mutation-primary-missing")?).ok_or("draw-store.mutation-target-lost")?).locked = value.locked
                    }
                    DrawMutation::SetLayerOpacity(value) if value.opacity.is_finite() => {
                        crate::artifacts::draw::schema::layer_base_mut(DrawLayerLocator::node_at_mut(source, address.ok_or("draw-store.mutation-primary-missing")?).ok_or("draw-store.mutation-target-lost")?).opacity = value.opacity
                    }
                    DrawMutation::SetLayerOpacity(_) => return Err("draw-store.mutation-opacity-invalid"),
                    DrawMutation::SetLayerBlendMode(value) => {
                        self.write_overlay_string(
                            &mut crate::artifacts::draw::schema::layer_base_mut(DrawLayerLocator::node_at_mut(source, address.ok_or("draw-store.mutation-primary-missing")?).ok_or("draw-store.mutation-target-lost")?).blend_mode,
                            &value.blend_mode,
                        )?;
                    }
                    DrawMutation::RenameLayer(value) => {
                        self.write_overlay_string(
                            &mut crate::artifacts::draw::schema::layer_base_mut(DrawLayerLocator::node_at_mut(source, address.ok_or("draw-store.mutation-primary-missing")?).ok_or("draw-store.mutation-target-lost")?).name,
                            &value.new_name,
                        )?;
                    }
                    DrawMutation::UpdateLayerTransform(value)
                        if [value.transform.x, value.transform.y, value.transform.scale_x, value.transform.scale_y, value.transform.rotation].iter().all(|field| field.is_finite()) && value.transform.scale_x > 0.0 && value.transform.scale_y > 0.0 =>
                    {
                        crate::artifacts::draw::schema::layer_base_mut(DrawLayerLocator::node_at_mut(source, address.ok_or("draw-store.mutation-primary-missing")?).ok_or("draw-store.mutation-target-lost")?).transform =
                            crate::artifacts::draw::DrawTransform { x: value.transform.x, y: value.transform.y, scale_x: value.transform.scale_x, scale_y: value.transform.scale_y, rotation: value.transform.rotation };
                    }
                    DrawMutation::UpdateLayerTransform(_) => return Err("draw-store.mutation-transform-invalid"),
                    DrawMutation::ReplaceLayerFill(value) => {
                        let replacement = match value.fill.as_ref() {
                            Some(_) => Some(self.fill_clone.as_mut().ok_or("draw-store.fill-clone-missing")?.take().ok_or("draw-store.fill-false-terminal")?),
                            None => None,
                        };
                        let old = std::mem::replace(
                            &mut crate::artifacts::draw::schema::layer_base_mut(DrawLayerLocator::node_at_mut(source, address.ok_or("draw-store.mutation-primary-missing")?).ok_or("draw-store.mutation-target-lost")?).attributes.fill,
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
                            &mut crate::artifacts::draw::schema::layer_base_mut(DrawLayerLocator::node_at_mut(source, address.ok_or("draw-store.mutation-primary-missing")?).ok_or("draw-store.mutation-target-lost")?).attributes.stroke,
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
                        let DrawLayerNode::Boolean(target) = DrawLayerLocator::node_at_mut(source, address.ok_or("draw-store.mutation-primary-missing")?).ok_or("draw-store.mutation-target-lost")? else {
                            return Err("draw-store.mutation-boolean-target");
                        };
                        self.write_overlay_string(&mut target.operation, &value.boolean_operation)?;
                    }
                    DrawMutation::UpdateLayerTraceParams(value) if value.params.threshold.is_finite() && value.params.simplify_epsilon.is_finite() => {
                        let DrawLayerNode::Trace(target) = DrawLayerLocator::node_at_mut(source, address.ok_or("draw-store.mutation-primary-missing")?).ok_or("draw-store.mutation-target-lost")? else {
                            return Err("draw-store.mutation-trace-target");
                        };
                        target.params = crate::artifacts::draw::DrawTraceParams { threshold: value.params.threshold, simplify_epsilon: value.params.simplify_epsilon };
                    }
                    DrawMutation::UpdateLayerTraceParams(_) => return Err("draw-store.mutation-trace-invalid"),
                    DrawMutation::CreateLayer(_) | DrawMutation::DuplicateLayer(_) | DrawMutation::DeleteLayer(_) | DrawMutation::ReorderLayer(_) => unreachable!("structural Draw mutations start retained rebuild before scalar mutation"),
                }
                self.overlay.as_mut().ok_or("draw-store.mutation-overlay-missing")?.commit(source)?;
                self.phase = DrawMutationCandidatePhase::Complete;
                cx.consume_fuel(1);
                Ok(false)
            }
            DrawMutationCandidatePhase::RebuildSource => {
                if !self.rebuild.as_mut().ok_or("draw-store.mutation-rebuild-missing")?.step(cx)? {
                    return Ok(false);
                }
                let parent = self.primary.ok_or("draw-store.mutation-primary-missing")?.parent();
                let removed = self.finish_rebuild(source, parent)?.ok_or("draw-store.mutation-removal-missing")?;
                *self.pending_layer = Some(removed);
                if matches!(mutation, DrawMutation::DeleteLayer(_)) {
                    self.overlay.as_mut().ok_or("draw-store.mutation-overlay-missing")?.commit(source)?;
                    self.source_undo = None;
                    self.phase = DrawMutationCandidatePhase::Complete;
                } else {
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
                let locator = self.locator.as_mut().ok_or("draw-store.mutation-destination-locator")?;
                if !locator.step(source, Self::parent(mutation).ok_or("draw-store.mutation-parent-missing")?, cx)? {
                    return Ok(false);
                }
                self.secondary = locator.found();
                self.locator = None;
                let Some(address) = self.secondary else { return Err("draw-store.mutation-parent-not-found") };
                if !matches!(DrawLayerLocator::node_at(source, address), Some(DrawLayerNode::Group(_))) {
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
                        DrawMutation::CreateLayer(value) => value.index.unwrap_or_else(|| DrawLayerLocator::container_mut(source, parent).map_or(0, |values| values.len())),
                        DrawMutation::DuplicateLayer(_) => self.primary.ok_or("draw-store.mutation-primary-missing")?.index() + 1,
                        DrawMutation::ReorderLayer(value) => value.index,
                        _ => 0,
                    };
                    self.start_rebuild(source, parent, None, Some(index), DrawContainerRebuildRole::Destination)?;
                }
                if !self.rebuild.as_mut().ok_or("draw-store.mutation-rebuild-missing")?.step(cx)? {
                    return Ok(false);
                }
                let parent = match mutation {
                    DrawMutation::CreateLayer(_) | DrawMutation::ReorderLayer(_) => self.secondary,
                    DrawMutation::DuplicateLayer(_) => self.primary.ok_or("draw-store.mutation-primary-missing")?.parent(),
                    _ => None,
                };
                if self.finish_rebuild(source, parent)?.is_some() {
                    return Err("draw-store.mutation-unexpected-removal");
                }
                self.overlay.as_mut().ok_or("draw-store.mutation-overlay-missing")?.commit(source)?;
                self.source_undo = None;
                self.phase = DrawMutationCandidatePhase::Complete;
                Ok(false)
            }
            DrawMutationCandidatePhase::Complete => {
                if let Some(value) = self.pending_layer.take() {
                    *self.retirement = Some(Box::new(DrawOwnedRetirement::new(DrawRetirementOwner::Layer(value))));
                    return Ok(false);
                }
                if let Some(retirement) = self.retirement.as_mut() {
                    return match retirement.close_step(1, DRAW_OWNED_FIELD_BYTES).map_err(|_| "draw-store.mutation-retirement")? {
                        store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                            drop(self.retirement.take());
                            Ok(false)
                        }
                        store::SnapshotRetirementStep::Complete => self.fail("draw-store.mutation-retirement-false-terminal"),
                        _ => Ok(false),
                    };
                }
                if self.return_arena_owner()? != Some(true) {
                    return Ok(false);
                }
                if !self.overlay.as_ref().is_some_and(|overlay| overlay.committed) {
                    self.overlay.as_mut().ok_or("draw-store.mutation-overlay-missing")?.commit(source)?;
                }
                self.terminal = true;
                Ok(true)
            }
            DrawMutationCandidatePhase::Retire | DrawMutationCandidatePhase::Fault => Err(self.fault.unwrap_or("draw-store.mutation-candidate-fault")),
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

    fn pump_rebuild_close(&mut self, source: &mut DrawSnapshot) -> Result<store::SnapshotRetirementStep, String> {
        let role = self.rebuild_role.ok_or("Draw mutation rebuild role missing")?;
        let target = self.rebuild_target.ok_or("Draw mutation rebuild target missing")?;
        let rebuild = self.rebuild.as_mut().ok_or("Draw mutation rebuild missing")?;
        let ready = if role == DrawContainerRebuildRole::CloseSourceUndo { rebuild.close_forward_step()? } else { rebuild.rollback_step()? };
        if !ready {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        match self.rebuild_close_phase {
            0 => {
                let container = DrawLayerLocator::container_mut(source, target).ok_or("Draw mutation rollback container missing")?;
                if !container.is_empty() {
                    return Err("Draw mutation rollback destination was not empty".into());
                }
                *container = rebuild.source.take().ok_or("Draw mutation rollback source missing")?;
            }
            1 => {
                if rebuild.pending.is_some() && self.pending_layer.is_some() {
                    return Err("Draw mutation rollback pending owner collision".into());
                }
                *self.pending_layer = rebuild.pending.take();
            }
            2 => {
                if self.container_reverse.is_some() {
                    return Err("Draw mutation rollback reverse owner collision".into());
                }
                *self.container_reverse = Some(rebuild.reverse.take().ok_or("Draw mutation rollback reverse owner missing")?);
            }
            3 => {
                if self.container_output.is_some() {
                    return Err("Draw mutation rollback output owner collision".into());
                }
                *self.container_output = Some(rebuild.output.take().ok_or("Draw mutation rollback output owner missing")?);
            }
            4 => {
                if rebuild.removed.is_some() {
                    return Err("Draw mutation rollback retained an unexpected removed owner".into());
                }
                rebuild.finish_handoff()?;
                drop(self.rebuild.take());
                self.rebuild_target = None;
                self.rebuild_role = None;
                self.rebuild_close_phase = 0;
                if role != DrawContainerRebuildRole::Destination {
                    self.source_undo = None;
                }
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            _ => return Err("Draw mutation rollback handoff phase invalid".into()),
        }
        self.rebuild_close_phase += 1;
        Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
    }

    fn close_step(&mut self, mut source: Option<&mut DrawSnapshot>, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if let Some(preflight) = self.preflight_mutation.as_mut() {
            return match preflight.close_step(maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if preflight.terminal_is_empty() => {
                    drop(self.preflight_mutation.take());
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                store::SnapshotRetirementStep::Complete => Err("Draw mutation preflight reported false terminal".into()),
                step => Ok(step),
            };
        }
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
        if let Some(rewrite) = self.duplicate_rewrite.as_mut() {
            return match rewrite.close_step(maximum_bytes)? {
                store::SnapshotRetirementStep::Complete => {
                    let pages = self.overlay_pages.as_mut().ok_or("Draw duplicate name arena missing")?;
                    if pages.len() >= pages.capacity() {
                        return Err("Draw duplicate name arena return saturated".into());
                    }
                    let (id_owner, name_owner) = rewrite.take_owners().ok_or("Draw duplicate rewrite reported false terminal")?;
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
                self.start_rebuild(source, undo.parent, None, Some(undo.index), DrawContainerRebuildRole::CloseSourceUndo)?;
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
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

struct DrawStoreInitializationAuthority {
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    arena_bootstrap_job: DrawMutationArenaBootstrapJob,
    envelope: std::mem::ManuallyDrop<Option<store::ArtifactEnvelope<DrawSnapshot, DrawMutation>>>,
    runtime: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationRuntime<DrawSnapshot>>>,
    candidate: std::mem::ManuallyDrop<Option<store::ArtifactStore<DrawSnapshot, DrawMutation>>>,
    active: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    envelope_retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    owner_catalog: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationOwnerCatalog>>,
    mutation_digest: std::mem::ManuallyDrop<Option<DrawMutationDigestAuthority>>,
    mutation_candidate: std::mem::ManuallyDrop<Option<DrawMutationCandidateAuthority>>,
    prepared_history_id: std::mem::ManuallyDrop<Option<String>>,
    prepared_actor: std::mem::ManuallyDrop<Option<String>>,
    initial_digest: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationDigest>>,
    edit_digest: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationDigest>>,
    phase: DrawStoreInitializationPhase,
    cancel_requested: bool,
    fault: Option<Vec<u8>>,
    terminal_handoff: bool,
}

impl DrawStoreInitializationAuthority {
    fn new(
        envelope: store::ArtifactEnvelope<DrawSnapshot, DrawMutation>,
        owner_catalog: Result<store::ArtifactStoreInitializationOwnerCatalog, &'static str>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
    ) -> Self {
        let bootstrap_job = DrawMutationArenaBootstrapJob::new(operation, generation);
        let (owner_catalog, arena_bootstrap_job, phase, fault) = match (owner_catalog, bootstrap_job) {
            (Ok(owner_catalog), Ok(arena_bootstrap_job)) => (Some(owner_catalog), arena_bootstrap_job, DrawStoreInitializationPhase::InitializeArena, None),
            (Err(error), Ok(mut arena_bootstrap_job)) => {
                arena_bootstrap_job.terminal = true;
                (None, arena_bootstrap_job, DrawStoreInitializationPhase::RetireFault, Some(error.as_bytes().to_vec()))
            }
            (_, Err(error)) => (None, DrawMutationArenaBootstrapJob::inactive(operation, generation), DrawStoreInitializationPhase::RetireFault, Some(error.as_bytes().to_vec())),
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
            initial_digest: std::mem::ManuallyDrop::new(Some(store::ArtifactStoreInitializationDigest::new(b"draw.initial"))),
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
        if let Some(value) = self.prepared_history_id.take() {
            *self.active = Some(Box::new(DrawOwnedRetirement::new(DrawRetirementOwner::String(value))));
            return Ok(false);
        }
        if let Some(value) = self.prepared_actor.take() {
            *self.active = Some(Box::new(DrawOwnedRetirement::new(DrawRetirementOwner::String(value))));
            return Ok(false);
        }
        if self.mutation_candidate.is_some() {
            let current = self.runtime.as_mut().and_then(store::ArtifactStoreInitializationRuntime::current_mut);
            let candidate = self.mutation_candidate.as_mut().expect("Draw mutation candidate remains retained during close");
            return match candidate.close_step(current, DRAW_OWNED_FIELD_BYTES)? {
                store::SnapshotRetirementStep::Complete if candidate.terminal_is_empty() => {
                    drop(self.mutation_candidate.take());
                    Ok(false)
                }
                store::SnapshotRetirementStep::Complete => Err("Draw mutation candidate reported a false terminal".into()),
                _ => Ok(false),
            };
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
        drop(self.owner_catalog.take());
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

impl semio_framework_plugin::ArtifactStoreInitializationAuthority<DrawSnapshot, DrawMutation> for DrawStoreInitializationAuthority {
    fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
        if cx.operation() != self.operation || cx.generation() != self.generation {
            self.arena_bootstrap_job.terminal = true;
            self.fail(b"draw-store.initializer-stale-authority");
        }
        if (self.cancel_requested || cx.is_cancelled()) && !matches!(self.phase, DrawStoreInitializationPhase::InitializeArena | DrawStoreInitializationPhase::RetireCancelled | DrawStoreInitializationPhase::Cancelled) {
            self.phase = DrawStoreInitializationPhase::RetireCancelled;
        }
        if let Err(error) = self.pump_active() {
            self.fault = Some(error.into_bytes());
            self.phase = DrawStoreInitializationPhase::RetireFault;
        } else if self.active.is_some() {
            return semio_framework_job::StepOutcome::Yield;
        }
        match self.phase {
            DrawStoreInitializationPhase::InitializeArena => {
                match self.arena_bootstrap_job.step(cx) {
                    DrawMutationArenaBootstrapStep::Ready => self.phase = DrawStoreInitializationPhase::ValidateEnvelope,
                    DrawMutationArenaBootstrapStep::Pending { .. } | DrawMutationArenaBootstrapStep::Blocked => {}
                    DrawMutationArenaBootstrapStep::Cancelled => self.phase = DrawStoreInitializationPhase::RetireCancelled,
                    DrawMutationArenaBootstrapStep::Fault(error) => self.fail(error.as_bytes()),
                }
                semio_framework_job::StepOutcome::Yield
            }
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
                if entry.id.is_empty() || entry.id.len() > DRAW_OWNED_FIELD_BYTES || entry.actor.as_ref().is_some_and(|actor| actor.len() > DRAW_OWNED_FIELD_BYTES) || entry.started_at.len() > DRAW_OWNED_FIELD_BYTES {
                    self.fail(b"draw-store.initializer-hostile-edit-field");
                } else {
                    self.phase = DrawStoreInitializationPhase::ValidateEditMeta { edit, meta: 0 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            DrawStoreInitializationPhase::ValidateEditMeta { edit, meta } => {
                let envelope = self.envelope.as_ref().expect("validated Draw envelope remains retained");
                let entry = envelope.vcs.edits.get(edit).expect("Draw edit remains retained during metadata validation");
                let Some(value) = entry.mutation_meta.get(meta) else {
                    self.phase = DrawStoreInitializationPhase::ValidateEditId { edit: edit + 1 };
                    return semio_framework_job::StepOutcome::Yield;
                };
                if value.timestamp.len() > DRAW_OWNED_FIELD_BYTES || value.mutation_id.as_ref().is_some_and(|id| id.0.len() > DRAW_OWNED_FIELD_BYTES) {
                    self.fail(b"draw-store.initializer-hostile-edit-field");
                } else {
                    self.phase = DrawStoreInitializationPhase::ValidateEditMeta { edit, meta: meta + 1 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            DrawStoreInitializationPhase::ValidateEditPair { left, right } => {
                let envelope = self.envelope.as_ref().expect("validated Draw envelope remains retained");
                if left >= envelope.vcs.edits.len() {
                    self.phase = DrawStoreInitializationPhase::HashInitialSchema;
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
            DrawStoreInitializationPhase::HashInitialSchema => {
                let source = &self.envelope.as_ref().expect("Draw envelope remains retained during initial digest").vcs.initial_snapshot;
                self.initial_digest.as_mut().expect("Draw initial digest remains retained").observe(source.schema.as_bytes());
                self.phase = DrawStoreInitializationPhase::HashInitialId;
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            DrawStoreInitializationPhase::HashInitialId => {
                let source = &self.envelope.as_ref().expect("Draw envelope remains retained during initial digest").vcs.initial_snapshot;
                self.initial_digest.as_mut().expect("Draw initial digest remains retained").observe(source.id.as_bytes());
                self.phase = DrawStoreInitializationPhase::MoveInitialOwner;
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            DrawStoreInitializationPhase::MoveInitialOwner => {
                let envelope = self.envelope.as_mut().expect("Draw envelope remains retained during initial owner move");
                let initial = std::mem::replace(&mut envelope.vcs.initial_snapshot, DrawSnapshot { schema: String::new(), id: String::new(), title: None, layers: Vec::new(), assets: std::collections::BTreeMap::new(), artboard: None });
                let initial_digest = self.initial_digest.take().expect("Draw initial digest remains retained").finish();
                let owner_catalog = self.owner_catalog.take().expect("Draw owner catalog was pre-admitted before initialization");
                *self.runtime = Some(store::ArtifactStoreInitializationRuntime::new_with_owner_catalog(&envelope.id, &envelope.schema, initial, initial_digest, owner_catalog));
                self.phase = DrawStoreInitializationPhase::SeedHistory { edit: 0, lane: 0, index: 0 };
                cx.consume_fuel(1);
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
                    match DrawMutationCandidateAuthority::try_new(self.operation, self.generation) {
                        Ok(candidate) => *self.mutation_candidate = Some(candidate),
                        Err(error) => {
                            self.fail(error.as_bytes());
                            return semio_framework_job::StepOutcome::Yield;
                        }
                    }
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
                    self.mutation_candidate.as_mut().expect("Draw completed mutation candidate remains retained").take().expect("Draw mutation overlay terminal commit witness remains exact");
                    drop(self.mutation_candidate.take());
                    self.phase = DrawStoreInitializationPhase::ApplyForward { position, edit, mutation: mutation + 1 };
                }
                semio_framework_job::StepOutcome::Yield
            }
            DrawStoreInitializationPhase::HashInverse { position, edit, mutation } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Draw applied edit remains retained");
                let Some(operation) = entry.inverse.get(mutation) else {
                    self.phase = DrawStoreInitializationPhase::PrepareApplied { position, edit, field: 0 };
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
            DrawStoreInitializationPhase::PrepareApplied { position, edit, field } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Draw applied edit remains retained");
                match field {
                    0 => {
                        *self.prepared_history_id = Some(DrawSnapshotCloneAuthority::clone_string(&entry.id).expect("validated Draw applied id remains admitted"));
                        self.phase = DrawStoreInitializationPhase::PrepareApplied { position, edit, field: 1 };
                        cx.consume_fuel(entry.id.len().max(1) as u64);
                    }
                    1 => {
                        if let Some(actor) = entry.actor.as_deref() {
                            *self.prepared_actor = Some(DrawSnapshotCloneAuthority::clone_string(actor).expect("validated Draw actor remains admitted"));
                            cx.consume_fuel(actor.len().max(1) as u64);
                        } else {
                            cx.consume_fuel(1);
                        }
                        self.phase = DrawStoreInitializationPhase::CommitApplied { position, edit };
                    }
                    _ => self.fail(b"draw-store.initializer-applied-preparation"),
                }
                semio_framework_job::StepOutcome::Yield
            }
            DrawStoreInitializationPhase::CommitApplied { position, edit } => {
                let id = self.prepared_history_id.take().expect("Draw applied id was retained in its own preparation grant");
                let actor = self.prepared_actor.take();
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
                    self.phase = DrawStoreInitializationPhase::PrepareRedo { position, edit };
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
            DrawStoreInitializationPhase::PrepareRedo { position, edit } => {
                let id = &self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Draw redo edit remains retained").id;
                *self.prepared_history_id = Some(DrawSnapshotCloneAuthority::clone_string(id).expect("validated Draw redo id remains admitted"));
                self.phase = DrawStoreInitializationPhase::CommitRedo { position, edit };
                cx.consume_fuel(id.len().max(1) as u64);
                semio_framework_job::StepOutcome::Yield
            }
            DrawStoreInitializationPhase::CommitRedo { position, edit } => {
                let id = self.prepared_history_id.take().expect("Draw redo id was retained in its own preparation grant");
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
                semio_framework_job::StepOutcome::Complete(semio_framework_job::CommitCandidate {
                    state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
                    output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
                })
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
            DrawStoreInitializationPhase::Complete => semio_framework_job::StepOutcome::Complete(semio_framework_job::CommitCandidate {
                state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
                output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
            }),
            DrawStoreInitializationPhase::Cancelled => semio_framework_job::StepOutcome::Cancelled,
            DrawStoreInitializationPhase::Fault => semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail: self.fault.clone().unwrap_or_else(|| b"draw-store.initializer-fault".to_vec()) }),
        }
    }

    fn request_cancel(&mut self) {
        self.cancel_requested = true;
    }

    fn begin_close(&mut self) {
        self.cancel_requested = true;
        if !matches!(self.phase, DrawStoreInitializationPhase::Cancelled | DrawStoreInitializationPhase::Fault) {
            self.phase = DrawStoreInitializationPhase::RetireCancelled;
        }
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<semio_framework_plugin::PluginCloseStep, semio_framework::Fault> {
        self.begin_close();
        if maximum_items == 0 || maximum_bytes < DRAW_OWNED_FIELD_BYTES {
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
            Err(error) => Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("artifact-store.initializer-close"), format!("Draw initializer close failed: {error}"))),
        }
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
    let owner_catalog = store::ArtifactStoreInitializationOwnerCatalog::try_new();
    semio_framework_plugin::ArtifactStoreInitializationJob::new(Box::new(DrawStoreInitializationAuthority::new(envelope, owner_catalog, operation, generation)))
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

    fn admit_string_destination(value: &mut String) {
        if value.capacity() < DRAW_OWNED_FIELD_BYTES {
            value.try_reserve_exact(DRAW_OWNED_FIELD_BYTES.saturating_sub(value.len())).expect("Draw fixture string destination is pre-admitted");
        }
        assert!(value.capacity() >= DRAW_OWNED_FIELD_BYTES);
    }

    fn admit_layer_string_destinations(layer: &mut DrawLayerNode) {
        let base = crate::artifacts::draw::schema::layer_base_mut(layer);
        admit_string_destination(&mut base.id);
        admit_string_destination(&mut base.name);
        admit_string_destination(&mut base.blend_mode);
        match layer {
            DrawLayerNode::Group(group) => {
                for child in &mut group.children {
                    admit_layer_string_destinations(child);
                }
            }
            DrawLayerNode::Boolean(boolean) => admit_string_destination(&mut boolean.operation),
            _ => {}
        }
    }

    fn initialize_draw_mutation_arena_pool_for_test() {
        let operation = semio_framework_job::OperationId(7_900);
        let generation = semio_framework_job::Generation(79);
        let mut job = DrawMutationArenaBootstrapJob::new(operation, generation).expect("fixed Draw arena bootstrap job admission");
        let cancel = semio_framework_job::root_cancel_token();
        let mut preview_sequence = 0;
        for _ in 0..1_000 {
            let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_ms, &mut preview_sequence);
            match job.step(&mut context) {
                DrawMutationArenaBootstrapStep::Ready => return,
                DrawMutationArenaBootstrapStep::Pending { advanced_items } => assert_eq!(advanced_items, 1),
                DrawMutationArenaBootstrapStep::Blocked => {}
                DrawMutationArenaBootstrapStep::Cancelled => panic!("Draw mutation arena bootstrap fixture was unexpectedly cancelled"),
                DrawMutationArenaBootstrapStep::Fault(error) => panic!("Draw mutation arena pool initialization faulted: {error}"),
            }
        }
        panic!("Draw mutation arena pool initialization did not terminate")
    }

    fn nested_snapshot() -> DrawSnapshot {
        initialize_draw_mutation_arena_pool_for_test();
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
        admit_layer_string_destinations(&mut group);
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

    fn drain_mutation(value: DrawMutation) {
        let mut retirement = store::ArtifactOwnedValueRetirementFactory::retire_owned(&DrawMutationRetirementFactory, value);
        for _ in 0..100_000 {
            match retirement.close_step(1, DRAW_OWNED_FIELD_BYTES).expect("Draw mutation retirement") {
                store::SnapshotRetirementStep::Complete => {
                    assert!(retirement.terminal_is_empty());
                    drop(retirement);
                    return;
                }
                store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                    assert!(released_items <= 1);
                    assert!(released_bytes <= DRAW_OWNED_FIELD_BYTES);
                }
                store::SnapshotRetirementStep::Blocked => panic!("owned Draw mutation retirement cannot block"),
            }
        }
        panic!("Draw mutation retirement did not terminate")
    }

    fn close_candidate(authority: &mut DrawMutationCandidateAuthority, mut source: Option<&mut DrawSnapshot>) {
        for _ in 0..100_000 {
            match authority.close_step(source.as_deref_mut(), DRAW_OWNED_FIELD_BYTES).expect("Draw candidate close") {
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

    fn apply(mut source: DrawSnapshot, mutation: &DrawMutation) -> Result<DrawSnapshot, (DrawSnapshot, &'static str)> {
        initialize_draw_mutation_arena_pool_for_test();
        let operation = semio_framework_job::OperationId(8_001);
        let generation = semio_framework_job::Generation(81);
        let mut authority = DrawMutationCandidateAuthority::try_new(operation, generation).expect("Draw candidate fixed owner arenas admit");
        let cancel = semio_framework_job::root_cancel_token();
        let mut preview_sequence = 0;
        for _ in 0..200_000 {
            let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_ms, &mut preview_sequence);
            match authority.step(&mut source, mutation, &mut context) {
                Ok(true) => {
                    authority.take().expect("Draw mutation overlay exact terminal commit witness");
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
        panic!("Draw mutation candidate did not terminate")
    }

    fn live_reservation(source: &mut DrawSnapshot, mutation: &DrawMutation) -> Result<DrawMutationAggregateReservation, &'static str> {
        initialize_draw_mutation_arena_pool_for_test();
        let operation = semio_framework_job::OperationId(8_004);
        let generation = semio_framework_job::Generation(84);
        let cancel = semio_framework_job::root_cancel_token();
        let mut preview_sequence = 0;
        let mut authority = DrawMutationCandidateAuthority::try_new(operation, generation)?;
        for _ in 0..100_000 {
            let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_ms, &mut preview_sequence);
            authority.step(source, mutation, &mut context)?;
            if let Some(reservation) = authority.reservation {
                close_candidate(&mut authority, Some(source));
                drop(authority);
                return Ok(reservation);
            }
        }
        close_candidate(&mut authority, Some(source));
        drop(authority);
        Err("draw-store.test-mutation-preflight-incomplete")
    }

    fn digest(mutation: &DrawMutation) -> Result<[u8; 32], &'static str> {
        let mut authority = DrawMutationDigestAuthority::new();
        let mut output = store::ArtifactStoreInitializationDigest::new(b"draw.test.mutation");
        let cancel = semio_framework_job::root_cancel_token();
        let mut preview_sequence = 0;
        for _ in 0..100_000 {
            let mut context = semio_framework_job::StepContext::new(
                semio_framework_job::OperationId(8_002),
                semio_framework_job::Generation(82),
                semio_framework_job::StepBudget::new(1, u64::MAX),
                cancel.clone(),
                semio_framework_job::default_now_ms,
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
                        authority.close_step(DRAW_OWNED_FIELD_BYTES).expect("Draw digest rejection closes exactly");
                    }
                    drop(authority);
                    return Err(error);
                }
            }
        }
        panic!("Draw mutation digest did not terminate")
    }

    fn rich_layer() -> DrawLayerNode {
        let mut group = crate::artifacts::draw::schema::create_draw_group_layer("Digest Group");
        let base = crate::artifacts::draw::schema::layer_base_mut(&mut group);
        base.visible = false;
        base.locked = true;
        base.opacity = 0.75;
        base.blend_mode = "multiply".into();
        base.transform = crate::artifacts::draw::DrawTransform { x: 1.0, y: 2.0, scale_x: 3.0, scale_y: 4.0, rotation: 0.5 };
        base.attributes.fill = Some(FillStyle::RadialGradient { cx: 1.0, cy: 2.0, r: 3.0, stops: vec![GradientStop { offset: 0.25, color: [0.1, 0.2, 0.3, 0.4] }] });
        base.attributes.stroke = Some(StrokeStyle { color: [0.5, 0.6, 0.7, 0.8], width: 2.0, cap: "round".into(), join: "bevel".into(), dash: Some(vec![1.0, 2.0]) });
        if let DrawLayerNode::Group(value) = &mut group {
            value.children.push(crate::artifacts::draw::schema::create_draw_shape_layer_rect("Shape"));
            value.children.push(crate::artifacts::draw::schema::create_draw_path_layer(
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
            value.children.push(crate::artifacts::draw::schema::create_draw_text_layer("Text"));
            value.children.push(crate::artifacts::draw::schema::create_draw_image_layer("Image", "asset-reference"));
            value.children.push(crate::artifacts::draw::schema::create_draw_boolean_layer("Boolean", "union", vec!["a".into(), "b".into()]));
            value.children.push(crate::artifacts::draw::schema::create_draw_trace_layer("Trace", "trace-source"));
        }
        group
    }

    fn create_digest(layer: DrawLayerNode) -> [u8; 32] {
        let mutation = DrawMutation::CreateLayer(CreateLayer { parent_id: Some("parent".into()), index: Some(3), layer: Box::new(layer) });
        let output = digest(&mutation).expect("rich Draw create mutation hashes");
        drain_mutation(mutation);
        output
    }

    fn assert_mutation_digest_distinct(left: DrawMutation, right: DrawMutation) {
        let left_digest = digest(&left).expect("left Draw mutation hashes");
        let right_digest = digest(&right).expect("right Draw mutation hashes");
        assert_ne!(left_digest, right_digest, "changing one Draw mutation semantic field changes the retained SHA-256 authority");
        drain_mutation(left);
        drain_mutation(right);
    }

    fn rich_child(layer: &mut DrawLayerNode, index: usize) -> &mut DrawLayerNode {
        let DrawLayerNode::Group(group) = layer else { panic!("rich Draw fixture root remains a Group") };
        group.children.get_mut(index).expect("rich Draw fixture child")
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
            let value = apply(nested_snapshot(), &mutation).expect("retained Draw mutation applies");
            drain_snapshot(value);
        }
        drain_snapshot(source);
    }

    #[test]
    fn retained_draw_process_arena_pool_cap_plus_one_returns_exact_slots_and_rejects_stale_aba() {
        let pool = DrawMutationArenaPool::try_new().expect("isolated Draw process arena pool claims exact bytes and items before operation admission");
        let mut first = Vec::new();
        for index in 0..DRAW_MUTATION_ARENA_POOL_CAPACITY {
            let candidate = DrawMutationCandidateAuthority::try_new_from_pool(semio_framework_job::OperationId(8_100 + index as u64), semio_framework_job::Generation(100 + index as u64), pool.clone())
                .expect("each fixed Draw process arena slot admits exactly once");
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
        match DrawMutationCandidateAuthority::try_new_from_pool(semio_framework_job::OperationId(8_999), semio_framework_job::Generation(999), pool.clone()) {
            Err(error) => assert_eq!(error, "draw-store.mutation-arena-pool-saturated"),
            Ok(_) => panic!("fixed Draw arena pool must reject capacity +1"),
        }
        for entry in &mut first {
            for phase in 0..4 {
                assert_eq!(entry.6.return_arena_owner().expect("one fixed Draw root returns per opportunity"), Some(phase == 3));
                let state = pool.state.try_lock().expect("isolated Draw pool is uncontended");
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
        for index in 0..DRAW_MUTATION_ARENA_POOL_CAPACITY {
            let candidate = DrawMutationCandidateAuthority::try_new_from_pool(semio_framework_job::OperationId(8_200 + index as u64), semio_framework_job::Generation(200 + index as u64), pool.clone()).expect("returned Draw arena slot re-admits");
            second.push(candidate);
        }
        for candidate in &second {
            let witness = first_witnesses.iter().find(|entry| entry.0 == candidate.arena_slot).expect("same fixed Draw arena slot returns");
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

    fn step_arena_bootstrap(bootstrap: &mut DrawMutationArenaPoolBootstrap) -> Result<bool, &'static str> {
        let cancel = semio_framework_job::root_cancel_token();
        let mut preview_sequence = 0;
        let mut context =
            semio_framework_job::StepContext::new(semio_framework_job::OperationId(7_902), semio_framework_job::Generation(79), semio_framework_job::StepBudget::new(1, u64::MAX), cancel, semio_framework_job::default_now_ms, &mut preview_sequence);
        bootstrap.step(&mut context)
    }

    fn close_arena_bootstrap_step(bootstrap: &mut DrawMutationArenaPoolBootstrap) -> store::SnapshotRetirementStep {
        let cancel = semio_framework_job::root_cancel_token();
        let mut preview_sequence = 0;
        let mut context =
            semio_framework_job::StepContext::new(semio_framework_job::OperationId(7_903), semio_framework_job::Generation(79), semio_framework_job::StepBudget::new(1, u64::MAX), cancel, semio_framework_job::default_now_ms, &mut preview_sequence);
        bootstrap.close_step(&mut context)
    }

    fn close_arena_bootstrap(bootstrap: &mut DrawMutationArenaPoolBootstrap) -> usize {
        let mut released_roots = 0;
        for _ in 0..10_000 {
            match close_arena_bootstrap_step(bootstrap) {
                store::SnapshotRetirementStep::Pending { released_items, .. } => {
                    assert!(released_items <= 1, "Draw arena bootstrap releases at most one exact root per grant");
                    released_roots += released_items;
                }
                store::SnapshotRetirementStep::Complete => {
                    assert!(bootstrap.terminal_is_empty());
                    return released_roots;
                }
                store::SnapshotRetirementStep::Blocked => panic!("isolated Draw arena bootstrap never blocks"),
            }
        }
        panic!("Draw arena bootstrap retirement did not terminate")
    }

    fn build_arena_bootstrap_owners(bootstrap: &mut DrawMutationArenaPoolBootstrap) {
        for _ in 0..10_000 {
            if bootstrap.owner == DRAW_MUTATION_ARENA_POOL_CAPACITY {
                return;
            }
            assert_eq!(step_arena_bootstrap(bootstrap), Ok(false));
        }
        panic!("Draw arena bootstrap did not construct its fixed owner catalog")
    }

    #[test]
    fn retained_draw_arena_bootstrap_failure_at_each_allocation_retires_one_exact_root_per_grant() {
        let allocations = DRAW_MUTATION_ARENA_POOL_CAPACITY * 20;
        for failure_at in 0..allocations {
            let mut bootstrap = DrawMutationArenaPoolBootstrap::new(Some(failure_at), None, usize::MAX, usize::MAX);
            let fault = loop {
                match step_arena_bootstrap(&mut bootstrap) {
                    Ok(false) => {}
                    Ok(true) => panic!("injected Draw arena allocation failure was not observed"),
                    Err(error) => break error,
                }
            };
            assert_eq!(fault, "draw-store.mutation-arena-bootstrap-injected-allocation");
            assert_eq!(bootstrap.allocation, failure_at + 1);
            assert_eq!(close_arena_bootstrap(&mut bootstrap), failure_at, "every successfully constructed Vec/String/page root is handed to the retained fault cursor");
            drop(bootstrap);
        }
    }

    #[test]
    fn retained_draw_arena_bootstrap_failure_after_each_bundle_keeps_every_root_until_terminal_close() {
        for owner in 0..DRAW_MUTATION_ARENA_POOL_CAPACITY {
            let mut bootstrap = DrawMutationArenaPoolBootstrap::new(None, Some(owner), usize::MAX, usize::MAX);
            let fault = loop {
                match step_arena_bootstrap(&mut bootstrap) {
                    Ok(false) => {}
                    Ok(true) => panic!("injected Draw arena bundle failure was not observed"),
                    Err(error) => break error,
                }
            };
            assert_eq!(fault, "draw-store.mutation-arena-bootstrap-injected-owner");
            assert_eq!(bootstrap.owner, owner + 1);
            assert_eq!(close_arena_bootstrap(&mut bootstrap), (owner + 1) * 20, "every completed bundle remains in the retained construction-fault owner");
            drop(bootstrap);
        }
    }

    #[test]
    fn retained_draw_arena_bootstrap_advances_one_allocation_per_turn_and_withholds_incomplete_pool() {
        let mut bootstrap = DrawMutationArenaPoolBootstrap::production(DrawMutationArenaBootstrapAdmission::fixed().expect("fixed Draw arena bootstrap claim"));
        let mut turns = 0;
        while !bootstrap.ready {
            let allocation = bootstrap.allocation;
            assert!(bootstrap.take_pool().is_none(), "an incomplete Draw arena bootstrap cannot publish an operation-admission pool");
            assert!(matches!(step_arena_bootstrap(&mut bootstrap), Ok(false) | Ok(true)));
            assert!(bootstrap.allocation.saturating_sub(allocation) <= 1, "one governed bootstrap turn allocates at most one retained root");
            turns += 1;
            assert!(turns < 1_000);
        }
        let pool = bootstrap.take_pool().expect("terminal Draw arena bootstrap publishes the fixed process pool");
        assert_eq!(pool.state.try_lock().expect("isolated Draw arena pool is uncontended").slots.len(), DRAW_MUTATION_ARENA_POOL_CAPACITY);
        drop(bootstrap);
        drop(pool);
    }

    #[test]
    fn retained_draw_arena_bootstrap_exact_cap_and_plus_one_rejection_preserve_every_owner_until_close() {
        let mut exact = DrawMutationArenaPoolBootstrap::new(None, None, usize::MAX, usize::MAX);
        build_arena_bootstrap_owners(&mut exact);
        let admitted_items = exact.admitted_items;
        let admitted_bytes = exact.admitted_bytes;
        exact.maximum_items = admitted_items;
        exact.maximum_bytes = admitted_bytes;
        assert_eq!(step_arena_bootstrap(&mut exact), Ok(true), "allocator-returned Draw arena capacities admit at the exact boundary");
        assert_eq!(close_arena_bootstrap(&mut exact), DRAW_MUTATION_ARENA_POOL_CAPACITY * 20);
        drop(exact);

        for (maximum_items, maximum_bytes) in [(admitted_items - 1, admitted_bytes), (admitted_items, admitted_bytes - 1)] {
            let mut rejected = DrawMutationArenaPoolBootstrap::new(None, None, usize::MAX, usize::MAX);
            build_arena_bootstrap_owners(&mut rejected);
            rejected.maximum_items = maximum_items;
            rejected.maximum_bytes = maximum_bytes;
            assert_eq!(step_arena_bootstrap(&mut rejected), Err("draw-store.mutation-arena-pool-capacity"));
            assert_eq!(close_arena_bootstrap(&mut rejected), DRAW_MUTATION_ARENA_POOL_CAPACITY * 20, "aggregate +1 rejection retains all eighty exact roots until cursorized close");
            drop(rejected);
        }
    }

    #[test]
    fn retained_draw_arena_default_second_app_and_borrow_only_request_without_allocation() {
        let state = DRAW_MUTATION_ARENA_POOL.get_or_init(|| std::sync::Mutex::new(DrawMutationArenaProcessState::Inert));
        let guard = state.try_lock().expect("isolated Draw request fixture owns the inert process metadata");
        let witness = match &*guard {
            DrawMutationArenaProcessState::Inert => (0, 0),
            DrawMutationArenaProcessState::Building(bootstrap) => (1, bootstrap.allocation),
            DrawMutationArenaProcessState::Ready(_) => (2, 0),
            DrawMutationArenaProcessState::Retiring(bootstrap) => (3, bootstrap.allocation),
            DrawMutationArenaProcessState::Fault(_) => (4, 0),
        };
        assert_eq!(request_draw_mutation_arena_pool(), DrawMutationArenaPoolAvailability::Contended);
        assert_eq!(request_draw_mutation_arena_pool(), DrawMutationArenaPoolAvailability::Contended, "a second app request coalesces fixed metadata without allocation");
        match borrow_draw_mutation_arena() {
            Err(error) => assert_eq!(error, DrawMutationArenaBorrowError::Contended),
            Ok(_) => panic!("borrow under process contention cannot expose an arena owner"),
        }
        let after = match &*guard {
            DrawMutationArenaProcessState::Inert => (0, 0),
            DrawMutationArenaProcessState::Building(bootstrap) => (1, bootstrap.allocation),
            DrawMutationArenaProcessState::Ready(_) => (2, 0),
            DrawMutationArenaProcessState::Retiring(bootstrap) => (3, bootstrap.allocation),
            DrawMutationArenaProcessState::Fault(_) => (4, 0),
        };
        assert_eq!(after, witness, "default/request/borrow cannot advance a bootstrap allocation while no governed job owns the process turn");
        drop(guard);
    }

    #[test]
    fn retained_draw_arena_bootstrap_job_cancel_budget_contention_and_saturation_are_governed() {
        let operation = semio_framework_job::OperationId(7_904);
        let generation = semio_framework_job::Generation(79);
        let mut job = DrawMutationArenaBootstrapJob::new(operation, generation).expect("fixed Draw bootstrap admission claim");
        let cancel = semio_framework_job::root_cancel_token();
        let mut preview_sequence = 0;
        let mut exhausted = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(0, u64::MAX), cancel.clone(), semio_framework_job::default_now_ms, &mut preview_sequence);
        assert_eq!(job.step(&mut exhausted), DrawMutationArenaBootstrapStep::Blocked, "zero-budget bootstrap cannot allocate or retire");

        let state = DRAW_MUTATION_ARENA_POOL.get_or_init(|| std::sync::Mutex::new(DrawMutationArenaProcessState::Inert));
        let guard = state.try_lock().expect("isolated Draw bootstrap fixture owns process contention");
        let mut contended = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_ms, &mut preview_sequence);
        assert_eq!(job.step(&mut contended), DrawMutationArenaBootstrapStep::Blocked, "process contention leaves the exact bootstrap owner untouched");
        drop(guard);

        let local_operation = semio_framework_job::OperationId(7_905);
        let local_generation = semio_framework_job::Generation(79);
        let mut local_job = DrawMutationArenaBootstrapJob::new(local_operation, local_generation).expect("local Draw bootstrap job claims its fixed admission");
        let local_cancel = semio_framework_job::root_cancel_token();
        let mut local_preview_sequence = 0;
        let mut local_state = DrawMutationArenaProcessState::Inert;
        for _ in 0..3 {
            let mut admitted = semio_framework_job::StepContext::new(local_operation, local_generation, semio_framework_job::StepBudget::new(1, u64::MAX), local_cancel.clone(), semio_framework_job::default_now_ms, &mut local_preview_sequence);
            assert_eq!(local_job.step_locked(&mut local_state, &mut admitted), DrawMutationArenaBootstrapStep::Pending { advanced_items: 1 });
        }
        let allocated = match &local_state {
            DrawMutationArenaProcessState::Building(bootstrap) => bootstrap.allocation,
            _ => panic!("three governed Draw bootstrap turns retain one partially allocated bundle"),
        };
        assert_eq!(allocated, 1, "only admitted worker turns may advance allocation boundaries");
        local_cancel.cancel_now();
        for _ in 0..100 {
            let mut cancelled = semio_framework_job::StepContext::new(local_operation, local_generation, semio_framework_job::StepBudget::new(1, u64::MAX), local_cancel.clone(), semio_framework_job::default_now_ms, &mut local_preview_sequence);
            match local_job.step_locked(&mut local_state, &mut cancelled) {
                DrawMutationArenaBootstrapStep::Pending { advanced_items } => assert!(advanced_items <= 1),
                DrawMutationArenaBootstrapStep::Cancelled => break,
                DrawMutationArenaBootstrapStep::Blocked => {}
                DrawMutationArenaBootstrapStep::Ready | DrawMutationArenaBootstrapStep::Fault(_) => panic!("cancelled partial Draw bootstrap must retire to exact Cancelled"),
            }
        }
        assert!(local_job.terminal);

        let pool = DrawMutationArenaPool::try_new().expect("isolated fixed Draw pool admits exact saturation fixture");
        let mut candidates = Vec::new();
        for index in 0..DRAW_MUTATION_ARENA_POOL_CAPACITY {
            candidates.push(DrawMutationCandidateAuthority::try_new_from_pool(semio_framework_job::OperationId(7_910 + index as u64), semio_framework_job::Generation(80 + index as u64), pool.clone()).expect("each fixed Draw pool slot admits once"));
        }
        assert!(matches!(DrawMutationCandidateAuthority::try_new_from_pool(semio_framework_job::OperationId(7_999), semio_framework_job::Generation(99), pool), Err("draw-store.mutation-arena-pool-saturated")));
        for candidate in &mut candidates {
            close_candidate(candidate, None);
        }
        for candidate in candidates {
            drop(candidate);
        }
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
        let (source, error) = apply(source, &mutation).expect_err("retained Draw depth +1 authority rejects");
        assert_eq!(error, "draw-store.preflight-depth-capacity");
        drain_snapshot(source);

        let source = nested_snapshot();
        let mutation = DrawMutation::RenameLayer(RenameLayer { layer_id: "x".repeat(DRAW_OWNED_FIELD_BYTES + 1), new_name: "hostile".into() });
        let (source, _) = apply(source, &mutation).expect_err("hostile Draw field rejects");
        drain_snapshot(source);
    }

    #[test]
    fn retained_draw_container_false_terminal_saturation_and_interrupted_close_preserve_exact_owner() {
        let mut snapshot = crate::artifacts::draw::schema::default_draw_document("rebuild-reservation", None);
        snapshot.layers = vec![crate::artifacts::draw::schema::create_draw_path_layer("first", Vec::new()), crate::artifacts::draw::schema::create_draw_path_layer("second", Vec::new())];
        let mutation = DrawMutation::CreateLayer(CreateLayer { parent_id: None, index: Some(1), layer: Box::new(crate::artifacts::draw::schema::create_draw_path_layer("pending", Vec::new())) });
        let reservation = live_reservation(&mut snapshot, &mutation).expect("live Draw rebuild reservation admitted");
        let source = std::mem::take(&mut snapshot.layers);
        let DrawMutation::CreateLayer(mut create) = mutation else { unreachable!() };
        let pending = *std::mem::replace(&mut create.layer, Box::new(crate::artifacts::draw::schema::create_draw_path_layer("retired-placeholder", Vec::new())));
        drain_mutation(DrawMutation::CreateLayer(create));
        drain_snapshot(snapshot);
        let mut reverse = Vec::new();
        let mut output = Vec::new();
        reverse.try_reserve_exact(DRAW_MUTATION_CONTAINER_SLOT_CAPACITY).expect("fixed Draw reverse arena");
        output.try_reserve_exact(DRAW_MUTATION_CONTAINER_SLOT_CAPACITY).expect("fixed Draw output arena");
        let source_owner = source.as_ptr();
        let reverse_owner = reverse.as_ptr();
        let output_owner = output.as_ptr();
        let mut authority = DrawContainerRebuildAuthority::new(source, Some(0), Some(1), Some(pending), reverse, output, reservation).expect("fixed Draw rebuild admitted");
        assert!(authority.take().is_none(), "false terminal cannot expose a partially rebuilt owner");
        let cancel = semio_framework_job::root_cancel_token();
        let mut preview_sequence = 0;
        for _ in 0..3 {
            let mut context = semio_framework_job::StepContext::new(
                semio_framework_job::OperationId(8_005),
                semio_framework_job::Generation(85),
                semio_framework_job::StepBudget::new(1, u64::MAX),
                cancel.clone(),
                semio_framework_job::default_now_ms,
                &mut preview_sequence,
            );
            assert!(!authority.step(&mut context).expect("Draw rebuild advances before interruption"));
        }
        let mut rollback_turns = 0;
        while !authority.rollback_step().expect("Draw rebuild rollback advances one exact owner") {
            rollback_turns += 1;
        }
        assert_eq!(rollback_turns, authority.move_count, "one recorded owner move rolls back per close grant");
        let restored = authority.source.take().expect("original Draw source owner returns");
        let pending = authority.pending.take().expect("pending Draw owner returns");
        let reverse = authority.reverse.take().expect("reverse Draw scratch owner returns");
        let output = authority.output.take().expect("output Draw scratch owner returns");
        assert_eq!(restored.as_ptr(), source_owner);
        assert_eq!(reverse.as_ptr(), reverse_owner);
        assert_eq!(output.as_ptr(), output_owner);
        assert!(authority.removed.is_none());
        authority.finish_handoff().expect("Draw rebuild rollback reaches exact terminal handoff");
        assert!(authority.terminal_is_empty());
        drop(authority);
        drop(reverse);
        drop(output);
        drain_mutation(DrawMutation::CreateLayer(CreateLayer { parent_id: None, index: None, layer: Box::new(pending) }));
        let mut restored_snapshot = crate::artifacts::draw::schema::default_draw_document("restored-rebuild", None);
        restored_snapshot.layers = restored;
        drain_snapshot(restored_snapshot);
    }

    #[test]
    fn retained_draw_rebuild_fault_after_every_phase_rolls_back_exact_container_and_reuses_pool_slot() {
        for phase in 0..=3 {
            for stale in [false, true] {
                let pool = DrawMutationArenaPool::try_new().expect("isolated Draw rollback pool admits exact owners");
                let mut source = crate::artifacts::draw::schema::default_draw_document("rebuild-rollback", None);
                source.layers.try_reserve_exact(DRAW_MUTATION_CONTAINER_SLOT_CAPACITY).expect("Draw rollback fixture pre-admits original live container backing");
                for index in 0..3 {
                    source.layers.push(crate::artifacts::draw::schema::create_draw_path_layer(&format!("source-{index}"), Vec::new()));
                }
                let source_owner = source.layers.as_ptr();
                let source_ids: Vec<_> = source.layers.iter().map(|layer| crate::artifacts::draw::schema::layer_id(layer).to_string()).collect();
                let mutation = DrawMutation::CreateLayer(CreateLayer { parent_id: None, index: Some(1), layer: Box::new(crate::artifacts::draw::schema::create_draw_path_layer("pending", Vec::new())) });
                let operation = semio_framework_job::OperationId(8_500 + phase as u64);
                let generation = semio_framework_job::Generation(850 + phase as u64);
                let mut authority = DrawMutationCandidateAuthority::try_new_from_pool(operation, generation, pool.clone()).expect("Draw rollback candidate borrows one exact pool slot");
                let slot = authority.arena_slot;
                let arena_generation = authority.arena_generation;
                let reverse_owner = authority.container_reverse.as_ref().expect("Draw rollback reverse owner").as_ptr();
                let output_owner = authority.container_output.as_ref().expect("Draw rollback output owner").as_ptr();
                let page_catalog_owner = authority.overlay_pages.as_ref().expect("Draw rollback page catalog owner").as_ptr();
                let page_owners: [usize; DRAW_MUTATION_OVERLAY_PAGE_CAPACITY] = std::array::from_fn(|index| authority.overlay_pages.as_ref().expect("Draw rollback page owner")[index].as_ptr() as usize);
                let duplicate_owner = authority.duplicate_id_owner.as_ref().expect("Draw rollback duplicate owner").as_ptr();
                let cancel = semio_framework_job::root_cancel_token();
                let mut preview_sequence = 0;
                for _ in 0..100_000 {
                    if authority.rebuild.as_ref().is_some_and(|rebuild| rebuild.recorded_move_in_phase(phase)) {
                        break;
                    }
                    let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_ms, &mut preview_sequence);
                    assert!(!authority.step(&mut source, &mutation, &mut context).expect("Draw rollback fixture reaches every internal rebuild phase"));
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
                    semio_framework_job::default_now_ms,
                    &mut preview_sequence,
                );
                assert_eq!(authority.step(&mut source, &mutation, &mut rejected), Err(if stale { "draw-store.mutation-candidate-stale-authority" } else { "draw-store.mutation-candidate-cancelled" }));
                close_candidate(&mut authority, Some(&mut source));
                assert_eq!(source.layers.as_ptr(), source_owner, "rollback restores the exact original live Vec backing");
                assert_eq!(source.layers.iter().map(|layer| crate::artifacts::draw::schema::layer_id(layer)).collect::<Vec<_>>(), source_ids.iter().map(String::as_str).collect::<Vec<_>>(), "rollback restores exact FIFO layer order");
                drop(authority);

                let mut reused =
                    DrawMutationCandidateAuthority::try_new_from_pool(semio_framework_job::OperationId(operation.0 + 100), semio_framework_job::Generation(generation.0 + 100), pool.clone()).expect("rolled-back Draw pool slot re-admits immediately");
                assert_eq!(reused.arena_slot, slot);
                assert!(reused.arena_generation > arena_generation);
                assert_eq!(reused.container_reverse.as_ref().expect("returned reverse owner").as_ptr(), reverse_owner);
                assert_eq!(reused.container_output.as_ref().expect("returned output owner").as_ptr(), output_owner);
                assert_eq!(reused.overlay_pages.as_ref().expect("returned page catalog owner").as_ptr(), page_catalog_owner);
                assert_eq!(std::array::from_fn::<_, DRAW_MUTATION_OVERLAY_PAGE_CAPACITY, _>(|index| reused.overlay_pages.as_ref().expect("returned page owner")[index].as_ptr() as usize), page_owners);
                assert_eq!(reused.duplicate_id_owner.as_ref().expect("returned duplicate owner").as_ptr(), duplicate_owner);
                close_candidate(&mut reused, None);
                drop(reused);
                drain_mutation(mutation);
                drain_snapshot(source);
            }
        }
    }

    #[test]
    fn retained_draw_reorder_fault_after_source_handoff_restores_exact_nested_fifo_and_pool_roots() {
        for stale in [false, true] {
            let pool = DrawMutationArenaPool::try_new().expect("isolated Draw reorder rollback pool admits exact owners");
            let mut source = nested_snapshot();
            let (group_id, target, source_owner, source_ids) = match source.layers.last_mut().expect("Draw reorder rollback group") {
                DrawLayerNode::Group(group) => {
                    group.children.try_reserve_exact(DRAW_MUTATION_CONTAINER_SLOT_CAPACITY.saturating_sub(group.children.len())).expect("Draw reorder rollback fixture pre-admits the nested live container");
                    (
                        group.base.id.clone(),
                        crate::artifacts::draw::schema::layer_id(&group.children[0]).to_string(),
                        group.children.as_ptr(),
                        group.children.iter().map(|layer| crate::artifacts::draw::schema::layer_id(layer).to_string()).collect::<Vec<_>>(),
                    )
                }
                _ => unreachable!("Draw reorder rollback fixture remains a group"),
            };
            let mutation = DrawMutation::ReorderLayer(ReorderLayer { layer_id: target, parent_id: Some(group_id), index: 2 });
            let operation = semio_framework_job::OperationId(8_700);
            let generation = semio_framework_job::Generation(870);
            let mut authority = DrawMutationCandidateAuthority::try_new_from_pool(operation, generation, pool.clone()).expect("Draw reorder rollback candidate borrows one exact pool slot");
            let slot = authority.arena_slot;
            let reverse_owner = authority.container_reverse.as_ref().expect("Draw reorder reverse owner").as_ptr();
            let output_owner = authority.container_output.as_ref().expect("Draw reorder output owner").as_ptr();
            let cancel = semio_framework_job::root_cancel_token();
            let mut preview_sequence = 0;
            for _ in 0..100_000 {
                if authority.rebuild_role == Some(DrawContainerRebuildRole::Destination) && authority.rebuild.as_ref().is_some_and(|rebuild| rebuild.recorded_move_in_phase(2)) {
                    break;
                }
                let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_ms, &mut preview_sequence);
                assert!(!authority.step(&mut source, &mutation, &mut context).expect("Draw reorder rollback fixture reaches destination rebuild after source handoff"));
            }
            assert_eq!(authority.rebuild_role, Some(DrawContainerRebuildRole::Destination));
            assert!(authority.source_undo.is_some(), "source handoff keeps the exact insertion undo authority until destination publication");
            if !stale {
                cancel.cancel_now();
            }
            let mut rejected = semio_framework_job::StepContext::new(
                operation,
                if stale { semio_framework_job::Generation(generation.0 + 1) } else { generation },
                semio_framework_job::StepBudget::new(1, u64::MAX),
                cancel,
                semio_framework_job::default_now_ms,
                &mut preview_sequence,
            );
            assert_eq!(authority.step(&mut source, &mutation, &mut rejected), Err(if stale { "draw-store.mutation-candidate-stale-authority" } else { "draw-store.mutation-candidate-cancelled" }));
            close_candidate(&mut authority, Some(&mut source));
            drop(authority);
            let DrawLayerNode::Group(group) = source.layers.last().expect("Draw reorder rollback group remains retained") else { unreachable!("Draw reorder rollback group remains a group") };
            assert_eq!(group.children.as_ptr(), source_owner, "source undo restores the exact nested live Vec backing");
            assert_eq!(group.children.iter().map(|layer| crate::artifacts::draw::schema::layer_id(layer)).collect::<Vec<_>>(), source_ids.iter().map(String::as_str).collect::<Vec<_>>(), "source undo restores the exact nested FIFO order");

            let mut reused = DrawMutationCandidateAuthority::try_new_from_pool(semio_framework_job::OperationId(operation.0 + 1), semio_framework_job::Generation(generation.0 + 1), pool).expect("reorder rollback returns the exact pool slot");
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
    fn retained_draw_schema_digest_distinguishes_every_nested_semantic_field() {
        let baseline = rich_layer();
        let baseline_digest = create_digest(baseline.clone());
        let mut variants = Vec::new();

        let modifiers: &[fn(&mut DrawLayerNode)] = &[
            |value| crate::artifacts::draw::schema::layer_base_mut(value).id = "different-id".into(),
            |value| crate::artifacts::draw::schema::layer_base_mut(value).name = "different-name".into(),
            |value| crate::artifacts::draw::schema::layer_base_mut(value).transform.x = 9.0,
            |value| crate::artifacts::draw::schema::layer_base_mut(value).transform.y = 9.0,
            |value| crate::artifacts::draw::schema::layer_base_mut(value).transform.scale_x = 9.0,
            |value| crate::artifacts::draw::schema::layer_base_mut(value).transform.scale_y = 9.0,
            |value| crate::artifacts::draw::schema::layer_base_mut(value).attributes.fill = None,
            |value| {
                if let Some(FillStyle::RadialGradient { cx, .. }) = &mut crate::artifacts::draw::schema::layer_base_mut(value).attributes.fill {
                    *cx = 9.0;
                }
            },
            |value| {
                if let Some(FillStyle::RadialGradient { cy, .. }) = &mut crate::artifacts::draw::schema::layer_base_mut(value).attributes.fill {
                    *cy = 9.0;
                }
            },
            |value| {
                if let Some(FillStyle::RadialGradient { r, .. }) = &mut crate::artifacts::draw::schema::layer_base_mut(value).attributes.fill {
                    *r = 9.0;
                }
            },
            |value| {
                if let Some(FillStyle::RadialGradient { stops, .. }) = &mut crate::artifacts::draw::schema::layer_base_mut(value).attributes.fill {
                    stops[0].offset = 0.75;
                }
            },
            |value| {
                if let Some(FillStyle::RadialGradient { stops, .. }) = &mut crate::artifacts::draw::schema::layer_base_mut(value).attributes.fill {
                    stops[0].color[2] = 0.9;
                }
            },
            |value| crate::artifacts::draw::schema::layer_base_mut(value).attributes.stroke = None,
            |value| crate::artifacts::draw::schema::layer_base_mut(value).attributes.stroke.as_mut().expect("stroke").color[0] = 0.9,
            |value| crate::artifacts::draw::schema::layer_base_mut(value).attributes.stroke.as_mut().expect("stroke").width = 9.0,
            |value| crate::artifacts::draw::schema::layer_base_mut(value).attributes.stroke.as_mut().expect("stroke").cap = "square".into(),
            |value| crate::artifacts::draw::schema::layer_base_mut(value).attributes.stroke.as_mut().expect("stroke").join = "round".into(),
            |value| crate::artifacts::draw::schema::layer_base_mut(value).attributes.stroke.as_mut().expect("stroke").dash.as_mut().expect("dash")[0] = 9.0,
            |value| {
                if let DrawLayerNode::Shape(shape) = rich_child(value, 0) {
                    shape.rect.as_mut().expect("rect").width = 9.0;
                }
            },
            |value| {
                if let DrawLayerNode::Shape(shape) = rich_child(value, 0) {
                    shape.ellipse = Some(crate::artifacts::draw::DrawEllipse { cx: 1.0, cy: 2.0, rx: 3.0, ry: 4.0 });
                }
            },
            |value| {
                if let DrawLayerNode::Shape(shape) = rich_child(value, 0) {
                    shape.circle = Some(crate::artifacts::draw::DrawCircle { cx: 1.0, cy: 2.0, r: 3.0 });
                }
            },
            |value| {
                if let DrawLayerNode::Shape(shape) = rich_child(value, 0) {
                    shape.line = Some(crate::artifacts::draw::DrawLine { x1: 1.0, y1: 2.0, x2: 3.0, y2: 4.0 });
                }
            },
            |value| {
                if let DrawLayerNode::Shape(shape) = rich_child(value, 0) {
                    shape.polygon = Some(crate::artifacts::draw::DrawPolygon { points: vec![[1.0, 2.0], [3.0, 4.0]] });
                }
            },
            |value| {
                if let DrawLayerNode::Path(path) = rich_child(value, 1) {
                    path.segments[0] = PathSegment::Line { to: [1.0, 2.0] };
                }
            },
            |value| {
                if let DrawLayerNode::Path(path) = rich_child(value, 1) {
                    path.segments[1] = PathSegment::Line { to: [9.0, 4.0] };
                }
            },
            |value| {
                if let DrawLayerNode::Path(path) = rich_child(value, 1) {
                    path.segments[2] = PathSegment::Quad { ctrl: [9.0, 6.0], to: [7.0, 8.0] };
                }
            },
            |value| {
                if let DrawLayerNode::Path(path) = rich_child(value, 1) {
                    path.segments[3] = PathSegment::Cubic { ctrl1: [9.0, 10.0], ctrl2: [11.0, 12.0], to: [13.0, 20.0] };
                }
            },
            |value| {
                if let DrawLayerNode::Text(text) = rich_child(value, 2) {
                    text.x = 9.0;
                }
            },
            |value| {
                if let DrawLayerNode::Text(text) = rich_child(value, 2) {
                    text.y = 9.0;
                }
            },
            |value| {
                if let DrawLayerNode::Text(text) = rich_child(value, 2) {
                    text.content = "different text".into();
                }
            },
            |value| {
                if let DrawLayerNode::Text(text) = rich_child(value, 2) {
                    text.size = 9.0;
                }
            },
            |value| {
                if let DrawLayerNode::Image(image) = rich_child(value, 3) {
                    image.height = 9.0;
                }
            },
            |value| {
                if let DrawLayerNode::Trace(trace) = rich_child(value, 5) {
                    trace.params.simplify_epsilon = 9.0;
                }
            },
            |value| {
                let DrawLayerNode::Group(group) = value else { unreachable!() };
                group.children.swap(0, 1);
            },
        ];
        for modifier in modifiers {
            let mut value = baseline.clone();
            modifier(&mut value);
            variants.push(value);
        }

        let mut value = baseline.clone();
        crate::artifacts::draw::schema::layer_base_mut(&mut value).visible = true;
        variants.push(value);
        let mut value = baseline.clone();
        crate::artifacts::draw::schema::layer_base_mut(&mut value).locked = false;
        variants.push(value);
        let mut value = baseline.clone();
        crate::artifacts::draw::schema::layer_base_mut(&mut value).opacity = 0.5;
        variants.push(value);
        let mut value = baseline.clone();
        crate::artifacts::draw::schema::layer_base_mut(&mut value).blend_mode = "screen".into();
        variants.push(value);
        let mut value = baseline.clone();
        crate::artifacts::draw::schema::layer_base_mut(&mut value).transform.rotation = 0.75;
        variants.push(value);
        let mut value = baseline.clone();
        crate::artifacts::draw::schema::layer_base_mut(&mut value).attributes.fill = Some(FillStyle::Solid { color: [0.9, 0.2, 0.3, 0.4] });
        variants.push(value);
        let mut value = baseline.clone();
        crate::artifacts::draw::schema::layer_base_mut(&mut value).attributes.fill = Some(FillStyle::LinearGradient { x1: 1.0, y1: 2.0, x2: 3.0, y2: 4.0, stops: vec![GradientStop { offset: 0.5, color: [0.1, 0.2, 0.8, 0.4] }] });
        variants.push(value);
        let mut value = baseline.clone();
        crate::artifacts::draw::schema::layer_base_mut(&mut value).attributes.stroke = Some(StrokeStyle { color: [0.9, 0.6, 0.7, 0.8], width: 3.0, cap: "square".into(), join: "round".into(), dash: Some(vec![2.0, 3.0]) });
        variants.push(value);
        let mut value = baseline.clone();
        if let DrawLayerNode::Group(group) = &mut value {
            if let DrawLayerNode::Path(path) = &mut group.children[1] {
                path.segments[4] = PathSegment::Arc { rx: 15.0, ry: 16.0, rotation: 18.0, large_arc: false, sweep: true, to: [20.0, 19.0] };
            }
        }
        variants.push(value);
        let mut value = baseline.clone();
        if let DrawLayerNode::Group(group) = &mut value {
            if let DrawLayerNode::Image(image) = &mut group.children[3] {
                image.image_key = "other-asset-reference".into();
                image.width = 2.0;
            }
        }
        variants.push(value);
        let mut value = baseline.clone();
        if let DrawLayerNode::Group(group) = &mut value {
            if let DrawLayerNode::Boolean(boolean) = &mut group.children[4] {
                boolean.operation = "subtract".into();
                boolean.children.swap(0, 1);
            }
        }
        variants.push(value);
        let mut value = baseline.clone();
        if let DrawLayerNode::Group(group) = &mut value {
            if let DrawLayerNode::Trace(trace) = &mut group.children[5] {
                trace.source_key = "other-trace-source".into();
                trace.params.threshold = 0.25;
            }
        }
        variants.push(value);

        for variant in variants {
            assert_ne!(create_digest(variant), baseline_digest, "every Draw layer scalar, style, geometry, order, and asset reference changes the SHA-256 semantic authority");
        }

        let id = "layer".to_string();
        let all_payloads = [
            DrawMutation::SetLayerVisible(SetLayerVisible { layer_id: id.clone(), visible: false }),
            DrawMutation::SetLayerLocked(SetLayerLocked { layer_id: id.clone(), locked: true }),
            DrawMutation::SetLayerOpacity(SetLayerOpacity { layer_id: id.clone(), opacity: 0.25 }),
            DrawMutation::SetLayerBlendMode(SetLayerBlendMode { layer_id: id.clone(), blend_mode: "screen".into() }),
            DrawMutation::RenameLayer(RenameLayer { layer_id: id.clone(), new_name: "renamed".into() }),
            DrawMutation::UpdateLayerTransform(UpdateLayerTransform { layer_id: id.clone(), transform: crate::artifacts::draw::DrawTransform { x: 1.0, y: 2.0, scale_x: 3.0, scale_y: 4.0, rotation: 5.0 } }),
            DrawMutation::ReplaceLayerFill(ReplaceLayerFill { layer_id: id.clone(), fill: Some(FillStyle::Solid { color: [0.1, 0.2, 0.3, 0.4] }) }),
            DrawMutation::ReplaceLayerStroke(ReplaceLayerStroke { layer_id: id.clone(), stroke: Some(StrokeStyle { color: [0.1, 0.2, 0.3, 0.4], width: 2.0, cap: "round".into(), join: "bevel".into(), dash: Some(vec![1.0]) }) }),
            DrawMutation::SetLayerBooleanOperation(SetLayerBooleanOperation { layer_id: id.clone(), boolean_operation: "intersect".into() }),
            DrawMutation::UpdateLayerTraceParams(UpdateLayerTraceParams { layer_id: id.clone(), params: crate::artifacts::draw::DrawTraceParams { threshold: 0.25, simplify_epsilon: 0.5 } }),
            DrawMutation::CreateLayer(CreateLayer { parent_id: Some("parent".into()), index: Some(2), layer: Box::new(baseline.clone()) }),
            DrawMutation::DuplicateLayer(DuplicateLayer { layer_id: id.clone() }),
            DrawMutation::DeleteLayer(DeleteLayer { layer_id: id.clone() }),
            DrawMutation::ReorderLayer(ReorderLayer { layer_id: id, parent_id: Some("parent".into()), index: 3 }),
        ];
        let mut digests = std::collections::HashSet::new();
        for payload in all_payloads {
            assert!(digests.insert(digest(&payload).expect("all fourteen Draw mutation payloads hash distinctly")));
            drain_mutation(payload);
        }
        assert_mutation_digest_distinct(DrawMutation::SetLayerVisible(SetLayerVisible { layer_id: "layer".into(), visible: false }), DrawMutation::SetLayerVisible(SetLayerVisible { layer_id: "layer".into(), visible: true }));
        assert_mutation_digest_distinct(DrawMutation::SetLayerLocked(SetLayerLocked { layer_id: "layer".into(), locked: false }), DrawMutation::SetLayerLocked(SetLayerLocked { layer_id: "layer".into(), locked: true }));
        assert_mutation_digest_distinct(DrawMutation::SetLayerOpacity(SetLayerOpacity { layer_id: "layer".into(), opacity: 0.25 }), DrawMutation::SetLayerOpacity(SetLayerOpacity { layer_id: "layer".into(), opacity: 0.5 }));
        assert_mutation_digest_distinct(
            DrawMutation::SetLayerBlendMode(SetLayerBlendMode { layer_id: "layer".into(), blend_mode: "multiply".into() }),
            DrawMutation::SetLayerBlendMode(SetLayerBlendMode { layer_id: "layer".into(), blend_mode: "screen".into() }),
        );
        assert_mutation_digest_distinct(DrawMutation::RenameLayer(RenameLayer { layer_id: "layer".into(), new_name: "left".into() }), DrawMutation::RenameLayer(RenameLayer { layer_id: "layer".into(), new_name: "right".into() }));
        assert_mutation_digest_distinct(
            DrawMutation::UpdateLayerTransform(UpdateLayerTransform { layer_id: "layer".into(), transform: crate::artifacts::draw::DrawTransform { x: 1.0, y: 2.0, scale_x: 3.0, scale_y: 4.0, rotation: 5.0 } }),
            DrawMutation::UpdateLayerTransform(UpdateLayerTransform { layer_id: "layer".into(), transform: crate::artifacts::draw::DrawTransform { x: 6.0, y: 2.0, scale_x: 3.0, scale_y: 4.0, rotation: 5.0 } }),
        );
        assert_mutation_digest_distinct(
            DrawMutation::ReplaceLayerFill(ReplaceLayerFill { layer_id: "layer".into(), fill: None }),
            DrawMutation::ReplaceLayerFill(ReplaceLayerFill { layer_id: "layer".into(), fill: Some(FillStyle::Solid { color: [0.1, 0.2, 0.3, 0.4] }) }),
        );
        assert_mutation_digest_distinct(
            DrawMutation::ReplaceLayerFill(ReplaceLayerFill { layer_id: "layer".into(), fill: Some(FillStyle::LinearGradient { x1: 0.0, y1: 1.0, x2: 2.0, y2: 3.0, stops: vec![GradientStop { offset: 0.5, color: [0.1, 0.2, 0.3, 0.4] }] }) }),
            DrawMutation::ReplaceLayerFill(ReplaceLayerFill { layer_id: "layer".into(), fill: Some(FillStyle::LinearGradient { x1: 9.0, y1: 1.0, x2: 2.0, y2: 3.0, stops: vec![GradientStop { offset: 0.75, color: [0.1, 0.2, 0.8, 0.4] }] }) }),
        );
        assert_mutation_digest_distinct(
            DrawMutation::ReplaceLayerStroke(ReplaceLayerStroke { layer_id: "layer".into(), stroke: None }),
            DrawMutation::ReplaceLayerStroke(ReplaceLayerStroke { layer_id: "layer".into(), stroke: Some(StrokeStyle { color: [0.1, 0.2, 0.3, 0.4], width: 1.0, cap: "round".into(), join: "bevel".into(), dash: Some(vec![1.0]) }) }),
        );
        assert_mutation_digest_distinct(
            DrawMutation::ReplaceLayerStroke(ReplaceLayerStroke { layer_id: "layer".into(), stroke: Some(StrokeStyle { color: [0.1, 0.2, 0.3, 0.4], width: 1.0, cap: "round".into(), join: "bevel".into(), dash: Some(vec![1.0]) }) }),
            DrawMutation::ReplaceLayerStroke(ReplaceLayerStroke { layer_id: "layer".into(), stroke: Some(StrokeStyle { color: [0.9, 0.2, 0.3, 0.4], width: 2.0, cap: "square".into(), join: "round".into(), dash: Some(vec![2.0]) }) }),
        );
        assert_mutation_digest_distinct(
            DrawMutation::SetLayerBooleanOperation(SetLayerBooleanOperation { layer_id: "layer".into(), boolean_operation: "union".into() }),
            DrawMutation::SetLayerBooleanOperation(SetLayerBooleanOperation { layer_id: "layer".into(), boolean_operation: "subtract".into() }),
        );
        assert_mutation_digest_distinct(
            DrawMutation::UpdateLayerTraceParams(UpdateLayerTraceParams { layer_id: "layer".into(), params: crate::artifacts::draw::DrawTraceParams { threshold: 0.25, simplify_epsilon: 0.5 } }),
            DrawMutation::UpdateLayerTraceParams(UpdateLayerTraceParams { layer_id: "layer".into(), params: crate::artifacts::draw::DrawTraceParams { threshold: 0.75, simplify_epsilon: 1.5 } }),
        );
        assert_mutation_digest_distinct(
            DrawMutation::CreateLayer(CreateLayer { parent_id: None, index: None, layer: Box::new(baseline.clone()) }),
            DrawMutation::CreateLayer(CreateLayer { parent_id: Some("parent".into()), index: Some(1), layer: Box::new(baseline.clone()) }),
        );
        assert_mutation_digest_distinct(DrawMutation::DuplicateLayer(DuplicateLayer { layer_id: "left".into() }), DrawMutation::DuplicateLayer(DuplicateLayer { layer_id: "right".into() }));
        assert_mutation_digest_distinct(DrawMutation::DeleteLayer(DeleteLayer { layer_id: "left".into() }), DrawMutation::DeleteLayer(DeleteLayer { layer_id: "right".into() }));
        assert_mutation_digest_distinct(
            DrawMutation::ReorderLayer(ReorderLayer { layer_id: "layer".into(), parent_id: None, index: 0 }),
            DrawMutation::ReorderLayer(ReorderLayer { layer_id: "layer".into(), parent_id: Some("parent".into()), index: 1 }),
        );
        drain_snapshot(DrawSnapshot { layers: vec![baseline], ..crate::artifacts::draw::schema::default_draw_document("digest-owner", None) });
    }

    #[test]
    fn retained_draw_aggregate_credit_admits_exact_4096_rejects_plus_one_with_owner_handback() {
        let exact_source = nested_snapshot();
        let exact_owner = exact_source.layers.as_ptr();
        let exact_target = match exact_source.layers.last().expect("Draw exact-boundary group") {
            DrawLayerNode::Group(group) => crate::artifacts::draw::schema::layer_id(&group.children[0]).to_string(),
            _ => unreachable!("Draw exact-boundary group remains exact"),
        };
        let exact = DrawMutation::RenameLayer(RenameLayer { layer_id: exact_target, new_name: "x".repeat(DRAW_OWNED_FIELD_BYTES) });
        let exact_source = apply(exact_source, &exact).expect("an exact 4096-byte retained overlay page is admitted");
        assert_eq!(exact_source.layers.as_ptr(), exact_owner, "exact boundary publication retains the source container owner");
        drain_mutation(exact);
        drain_snapshot(exact_source);

        let plus_source = nested_snapshot();
        let plus_owner = plus_source.layers.as_ptr();
        let plus_target = match plus_source.layers.last().expect("Draw +1 group") {
            DrawLayerNode::Group(group) => crate::artifacts::draw::schema::layer_id(&group.children[0]).to_string(),
            _ => unreachable!("Draw +1 group remains exact"),
        };
        let plus_one = DrawMutation::RenameLayer(RenameLayer { layer_id: plus_target, new_name: "x".repeat(DRAW_OWNED_FIELD_BYTES + 1) });
        let (plus_source, error) = apply(plus_source, &plus_one).expect_err("4096 +1 retained overlay page rejects");
        assert_eq!(error, "draw-store.mutation-field-capacity");
        assert_eq!(plus_source.layers.as_ptr(), plus_owner, "+1 rejection returns the exact source owner without partial publication");
        drain_mutation(plus_one);
        drain_snapshot(plus_source);

        let mut source = crate::artifacts::draw::schema::default_draw_document("aggregate-owner", None);
        let mutation = DrawMutation::SetLayerVisible(SetLayerVisible { layer_id: crate::artifacts::draw::schema::layer_id(&source.layers[0]).into(), visible: false });
        let mut last_admitted = None;
        for index in 0..DRAW_MUTATION_AGGREGATE_ITEMS {
            source.layers.push(crate::artifacts::draw::schema::create_draw_path_layer(&format!("layer-{index}"), Vec::new()));
            let owner = source.layers.as_ptr();
            match live_reservation(&mut source, &mutation) {
                Ok(reservation) => {
                    assert_eq!(source.layers.as_ptr(), owner, "live aggregate census never replaces the exact source owner");
                    last_admitted = Some((source.layers.len(), reservation.total_items().expect("live item total"), reservation.total_bytes().expect("live byte total")));
                }
                Err("draw-store.mutation-aggregate-item-capacity" | "draw-store.mutation-aggregate-byte-capacity") => {
                    assert_eq!(source.layers.as_ptr(), owner, "aggregate +1 rejection returns the exact source backing");
                    break;
                }
                Err(error) => panic!("unexpected live Draw aggregate rejection: {error}"),
            }
        }
        let (admitted_layers, admitted_items, admitted_bytes) = last_admitted.expect("at least one live aggregate owner is admitted");
        assert_eq!(source.layers.len(), admitted_layers + 1, "the first additional real layer owner is the +1 rejection");
        assert!(admitted_items <= DRAW_MUTATION_AGGREGATE_ITEMS && admitted_bytes <= DRAW_MUTATION_AGGREGATE_BYTES);
        let last_valid_id = source.id.clone();
        let (source, error) = apply(source, &mutation).expect_err("aggregate +1 rejects");
        assert_eq!(error, "draw-store.mutation-aggregate-item-capacity");
        assert_eq!(source.id, last_valid_id, "aggregate rejection returns the exact source authority without partial publication");
        drain_mutation(mutation);
        drain_snapshot(source);
    }

    #[test]
    fn retained_draw_duplicate_hash_frames_domain_id_and_name_lengths_without_concatenation_collision() {
        fn duplicate_id(id: &str, name: &str) -> String {
            let mut source = crate::artifacts::draw::schema::default_draw_document("duplicate-framing", None);
            let mut layer = crate::artifacts::draw::schema::create_draw_path_layer(name, Vec::new());
            let base = crate::artifacts::draw::schema::layer_base_mut(&mut layer);
            base.id.clear();
            base.id.push_str(id);
            admit_layer_string_destinations(&mut layer);
            source.layers = vec![layer];
            let mutation = DrawMutation::DuplicateLayer(DuplicateLayer { layer_id: id.into() });
            let source = apply(source, &mutation).expect("framed duplicate mutation applies");
            let duplicate = source.layers.get(1).map(crate::artifacts::draw::schema::layer_id).expect("duplicated layer remains retained").to_string();
            drain_mutation(mutation);
            drain_snapshot(source);
            duplicate
        }

        assert_ne!(duplicate_id("ab", "c"), duplicate_id("a", "bc"), "separate id/name length frames prevent concatenation collisions");
    }

    #[test]
    fn retained_draw_duplicate_name_uses_preadmitted_page_and_returns_exact_rejection_owner() {
        let mut source = crate::artifacts::draw::schema::default_draw_document("duplicate-name-owner", None);
        let mut layer = crate::artifacts::draw::schema::create_draw_path_layer("Layer", Vec::new());
        admit_layer_string_destinations(&mut layer);
        let target = crate::artifacts::draw::schema::layer_id(&layer).to_string();
        let original_name_owner = crate::artifacts::draw::schema::layer_base_mut(&mut layer).name.as_ptr();
        source.layers = vec![layer];
        let mutation = DrawMutation::DuplicateLayer(DuplicateLayer { layer_id: target });
        let source = apply(source, &mutation).expect("duplicate name suffix uses only pre-admitted destination and fixed scratch page");
        assert_eq!(crate::artifacts::draw::schema::layer_base(&source.layers[0]).name.as_ptr(), original_name_owner, "last-valid name backing remains exact");
        assert_eq!(crate::artifacts::draw::schema::layer_base(&source.layers[1]).name, "Layer copy");
        drain_mutation(mutation);
        drain_snapshot(source);

        let mut rejected = crate::artifacts::draw::schema::default_draw_document("duplicate-name-rejected", None);
        let layer = crate::artifacts::draw::schema::create_draw_path_layer("Layer", Vec::new());
        let target = crate::artifacts::draw::schema::layer_id(&layer).to_string();
        rejected.layers = vec![layer];
        let exact_owner = rejected.layers.as_ptr();
        let mutation = DrawMutation::DuplicateLayer(DuplicateLayer { layer_id: target });
        let (rejected, error) = apply(rejected, &mutation).expect_err("unadmitted duplicate destination rejects without allocating after operation admission");
        assert_eq!(error, "draw-store.duplicate-destination-capacity");
        assert_eq!(rejected.layers.as_ptr(), exact_owner, "duplicate rejection returns the exact source container owner");
        drain_mutation(mutation);
        drain_snapshot(rejected);
    }

    #[test]
    fn retained_draw_cancel_stale_each_replay_candidate_container_stage_preserves_last_valid() {
        let stages = [
            DrawMutationCandidatePhase::PreflightSource,
            DrawMutationCandidatePhase::PreflightMutation,
            DrawMutationCandidatePhase::BindOverlay,
            DrawMutationCandidatePhase::LocatePrimary,
            DrawMutationCandidatePhase::LocateSecondary,
            DrawMutationCandidatePhase::PrepareOwnedValue,
            DrawMutationCandidatePhase::Apply,
            DrawMutationCandidatePhase::RebuildSource,
            DrawMutationCandidatePhase::LocateDestination,
            DrawMutationCandidatePhase::RebuildDestination,
            DrawMutationCandidatePhase::Complete,
        ];
        for stage in stages {
            for stale in [false, true] {
                let mut source = nested_snapshot();
                let (group_id, target) = match source.layers.last().expect("Draw group") {
                    DrawLayerNode::Group(group) => (group.base.id.clone(), crate::artifacts::draw::schema::layer_id(&group.children[0]).to_string()),
                    _ => unreachable!("Draw fixture group remains exact"),
                };
                let last_valid_id = source.id.clone();
                let mutation = match stage {
                    DrawMutationCandidatePhase::LocateSecondary => {
                        DrawMutation::CreateLayer(CreateLayer { parent_id: Some(group_id), index: Some(0), layer: Box::new(crate::artifacts::draw::schema::create_draw_path_layer("cancel-create", Vec::new())) })
                    }
                    DrawMutationCandidatePhase::RebuildSource | DrawMutationCandidatePhase::LocateDestination => DrawMutation::ReorderLayer(ReorderLayer { layer_id: target, parent_id: Some(group_id), index: 2 }),
                    _ => DrawMutation::DuplicateLayer(DuplicateLayer { layer_id: target }),
                };
                let operation = semio_framework_job::OperationId(8_003);
                let generation = semio_framework_job::Generation(83);
                let mut authority = DrawMutationCandidateAuthority::try_new(operation, generation).expect("Draw candidate fixed owner arenas admit");
                let cancel = semio_framework_job::root_cancel_token();
                let mut preview_sequence = 0;
                for _ in 0..100_000 {
                    if authority.phase == stage {
                        break;
                    }
                    let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_ms, &mut preview_sequence);
                    assert!(!authority.step(&mut source, &mutation, &mut context).expect("Draw stage fixture advances"));
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
                    semio_framework_job::default_now_ms,
                    &mut preview_sequence,
                );
                assert_eq!(authority.step(&mut source, &mutation, &mut rejected), Err(if stale { "draw-store.mutation-candidate-stale-authority" } else { "draw-store.mutation-candidate-cancelled" }),);
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
