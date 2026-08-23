//! ⚖️ Draw artifact — state-patch-representation wire codec + laws (was: constitutional
//! `protocol`; no `📡️protocol` path segment may survive under plugins).
//!
//! 🧷️ `DrawMutation` derives `dsl::DslEnum` directly (no foreign `CollectionMutation` in its
//! shape — every variant wraps a local `dsl::DslRecord` payload declared in its own triad leaf), so
//! this component is a pure pass-through over the derived codec, matching `🏔️gisterrain`'s sibling
//! facet's identical shape.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::artifacts::draw::op::DrawMutation;
use crate::artifacts::draw::{
    DrawAttributes, DrawImageAsset, DrawLayerBase, DrawLayerNode, DrawSnapshot, FillStyle,
    GradientStop, PathSegment, StrokeStyle,
};
use protocol::{Mutation, MutationDiff, OpBinary};

//#region 🔖️Codec
/// 📦️ Encodes a `DrawMutation` to its binary command form.
pub async fn encode_op(operation: &DrawMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `DrawMutation` from its binary command form.
pub async fn decode_op(bytes: &[u8]) -> Result<DrawMutation, protocol::ProtocolError> {
    DrawMutation::decode_op(bytes)
}
//#endregion 🔖️Codec

//#region 🔖️OwnedSprCatalog
const DRAW_OWNED_FIELD_BYTES: usize = store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;

enum DrawRetirementOwner {
    Snapshot(DrawSnapshot),
    Mutation(DrawMutation),
    MutationFields(DrawMutationFields),
    Layer(DrawLayerNode),
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
                    if let Some(value) = value.positions.pop() {
                        return Ok(Self::spawn(&mut self.active, DrawRetirementOwner::Feature(value)));
                    }
                    self.phase = 1;
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                }
                1 => {
                    if let Some(value) = value.routes.pop() {
                        return Ok(Self::spawn(&mut self.active, DrawRetirementOwner::Feature(value)));
                    }
                    self.phase = 2;
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                }
                2 => {
                    if let Some(value) = value.regions.pop() {
                        return Ok(Self::spawn(&mut self.active, DrawRetirementOwner::Feature(value)));
                    }
                    self.phase = 3;
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                }
                3..=7 => Ok(Self::child_step(&mut value.drawing, &mut self.phase, 3, maximum_items, maximum_bytes).expect("Draw drawing child phase is exact")),
                8..=12 => {
                    if let Some(image) = value.image.as_mut() {
                        Ok(Self::child_step(image, &mut self.phase, 8, maximum_items, maximum_bytes).expect("Draw image child phase is exact"))
                    } else {
                        self.phase = 13;
                        Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                    }
                }
                13..=17 => Ok(Self::child_step(&mut value.value, &mut self.phase, 13, maximum_items, maximum_bytes).expect("Draw value child phase is exact")),
                _ => {
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
            },
            DrawRetirementOwner::Feature(value) => match self.phase {
                0 => Ok(Self::release_string(&mut value.id, &mut self.phase, 1, maximum_items, maximum_bytes)),
                1 => {
                    let data = std::mem::replace(&mut value.data, dsl::DslValue::Null);
                    self.phase = 2;
                    Ok(Self::spawn(&mut self.active, DrawRetirementOwner::Value(data)))
                }
                _ => {
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
            },
            DrawRetirementOwner::Value(value) => match value {
                dsl::DslValue::String(value) => {
                    if self.phase == 0 {
                        return Ok(Self::release_string(value, &mut self.phase, 1, maximum_items, maximum_bytes));
                    }
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
                dsl::DslValue::Array(values) => {
                    if let Some(value) = values.pop() {
                        Ok(Self::spawn(&mut self.active, DrawRetirementOwner::Value(value)))
                    } else {
                        drop(self.owner.take());
                        Ok(store::SnapshotRetirementStep::Complete)
                    }
                }
                dsl::DslValue::Object(values) => {
                    if let Some((key, value)) = values.pop() {
                        Ok(Self::spawn(&mut self.active, DrawRetirementOwner::ValueEntry { key, value: Some(value) }))
                    } else {
                        drop(self.owner.take());
                        Ok(store::SnapshotRetirementStep::Complete)
                    }
                }
                dsl::DslValue::Null | dsl::DslValue::Bool(_) | dsl::DslValue::Number(_) => {
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
            },
            DrawRetirementOwner::ValueEntry { key, value } => match self.phase {
                0 => Ok(Self::release_string(key, &mut self.phase, 1, maximum_items, maximum_bytes)),
                1 => {
                    let value = value.take().ok_or_else(|| "Draw value entry lost its retained value".to_string())?;
                    self.phase = 2;
                    Ok(Self::spawn(&mut self.active, DrawRetirementOwner::Value(value)))
                }
                _ => {
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
            },
            DrawRetirementOwner::Mutation(_) => {
                use DrawMutation::*;
                let mutation = match self.owner.take() {
                    Some(DrawRetirementOwner::Mutation(value)) => value,
                    _ => unreachable!("Draw mutation owner variant remains exact"),
                };
                let fields = match mutation {
                    CreatePosition(payload) => DrawMutationFields::Feature(Some(payload.item)),
                    CreateRoute(payload) => DrawMutationFields::Feature(Some(payload.item)),
                    CreateRegion(payload) => DrawMutationFields::Feature(Some(payload.item)),
                    DeletePosition(payload) => DrawMutationFields::String(payload.id),
                    DeleteRoute(payload) => DrawMutationFields::String(payload.id),
                    DeleteRegion(payload) => DrawMutationFields::String(payload.id),
                    ReorderPositions(payload) => DrawMutationFields::String(payload.id),
                    ReorderRoutes(payload) => DrawMutationFields::String(payload.id),
                    ReorderRegions(payload) => DrawMutationFields::String(payload.id),
                    ReplacePositionData(payload) => DrawMutationFields::Value { id: payload.id, value: Some(payload.new_data) },
                    ReplaceRouteData(payload) => DrawMutationFields::Value { id: payload.id, value: Some(payload.new_data) },
                    ReplaceRegionData(payload) => DrawMutationFields::Value { id: payload.id, value: Some(payload.new_data) },
                };
                *self.owner = Some(DrawRetirementOwner::MutationFields(fields));
                Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
            }
            DrawRetirementOwner::MutationFields(fields) => match fields {
                DrawMutationFields::Feature(value) => {
                    if let Some(value) = value.take() {
                        return Ok(Self::spawn(&mut self.active, DrawRetirementOwner::Feature(value)));
                    }
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
                DrawMutationFields::String(value) => {
                    if self.phase == 0 {
                        return Ok(Self::release_string(value, &mut self.phase, 1, maximum_items, maximum_bytes));
                    }
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
                DrawMutationFields::Value { id, value } => match self.phase {
                    0 => Ok(Self::release_string(id, &mut self.phase, 1, maximum_items, maximum_bytes)),
                    1 => {
                        let value = value.take().ok_or_else(|| "Draw mutation value owner was lost".to_string())?;
                        self.phase = 2;
                        Ok(Self::spawn(&mut self.active, DrawRetirementOwner::Value(value)))
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
struct DrawSnapshotCloneAuthority {
    value: std::mem::ManuallyDrop<Option<DrawSnapshot>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    phase: u8,
    index: usize,
    terminal: bool,
}

impl DrawSnapshotCloneAuthority {
    fn empty_child<S>() -> store::ArtifactChild<S> {
        store::ArtifactChild::new(String::new(), store::os_io::ArtifactRef { artifact_id: String::new(), dialect: store::os_io::ArtifactDialect { artifact_kind: String::new(), standard: String::new(), subset: String::new() } })
    }

    fn new() -> Self {
        Self {
            value: std::mem::ManuallyDrop::new(Some(DrawSnapshot { positions: Vec::new(), routes: Vec::new(), regions: Vec::new(), drawing: Self::empty_child(), image: None, value: Self::empty_child() })),
            retirement: std::mem::ManuallyDrop::new(None),
            phase: 0,
            index: 0,
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

    fn clone_feature(source: &MapFeature) -> Result<MapFeature, &'static str> {
        let encoded = serde_json::to_vec(source).map_err(|_| "draw-store.initializer-feature-encoding")?;
        if encoded.len() > DRAW_OWNED_FIELD_BYTES {
            return Err("draw-store.initializer-feature-too-large");
        }
        Ok(source.clone())
    }

    fn copy_child_field<'a, S>(target: &mut store::ArtifactChild<S>, source: &'a store::ArtifactChild<S>, phase: u8, base: u8) -> Result<&'a [u8], &'static str> {
        let observed = match phase - base {
            0 => {
                target.child_id = Self::clone_string(&source.child_id)?;
                source.child_id.as_bytes()
            }
            1 => {
                target.target.artifact_id = Self::clone_string(&source.target.artifact_id)?;
                source.target.artifact_id.as_bytes()
            }
            2 => {
                target.target.dialect.artifact_kind = Self::clone_string(&source.target.dialect.artifact_kind)?;
                source.target.dialect.artifact_kind.as_bytes()
            }
            3 => {
                target.target.dialect.standard = Self::clone_string(&source.target.dialect.standard)?;
                source.target.dialect.standard.as_bytes()
            }
            4 => {
                target.target.dialect.subset = Self::clone_string(&source.target.dialect.subset)?;
                source.target.dialect.subset.as_bytes()
            }
            _ => return Err("draw-store.initializer-child-phase"),
        };
        Ok(observed)
    }

    fn step(&mut self, source: &DrawSnapshot, digest: &mut store::ArtifactStoreInitializationDigest, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        let target = self.value.as_mut().ok_or("draw-store.initializer-clone-target")?;
        let observed = match self.phase {
            0..=2 => {
                let (source, target, admission) = match self.phase {
                    0 => (&source.positions, &mut target.positions, "draw-store.initializer-position-admission"),
                    1 => (&source.routes, &mut target.routes, "draw-store.initializer-route-admission"),
                    _ => (&source.regions, &mut target.regions, "draw-store.initializer-region-admission"),
                };
                if self.index == 0 && target.capacity() == 0 {
                    target.try_reserve_exact(source.len()).map_err(|_| admission)?;
                }
                if let Some(feature) = source.get(self.index) {
                    let encoded = serde_json::to_vec(feature).map_err(|_| "draw-store.initializer-feature-encoding")?;
                    if encoded.len() > DRAW_OWNED_FIELD_BYTES {
                        return Err("draw-store.initializer-feature-too-large");
                    }
                    target.push(Self::clone_feature(feature)?);
                    self.index += 1;
                    digest.observe(&encoded);
                    cx.consume_fuel(encoded.len().max(1) as u64);
                    return Ok(false);
                }
                self.phase += 1;
                self.index = 0;
                &[]
            }
            3..=7 => Self::copy_child_field(&mut target.drawing, &source.drawing, self.phase, 3)?,
            8 => {
                if source.image.is_some() {
                    target.image = Some(Self::empty_child());
                }
                &[]
            }
            9..=13 => {
                let source = source.image.as_ref().ok_or("draw-store.initializer-image-source")?;
                let target = target.image.as_mut().ok_or("draw-store.initializer-image-target")?;
                Self::copy_child_field(target, source, self.phase, 9)?
            }
            14..=18 => Self::copy_child_field(&mut target.value, &source.value, self.phase, 14)?,
            _ => {
                self.terminal = true;
                return Ok(true);
            }
        };
        digest.observe(observed);
        self.phase = match self.phase {
            8 if source.image.is_none() => 14,
            value => value + 1,
        };
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
        if self.retirement.is_none() {
            if let Some(value) = self.value.take() {
                *self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&DrawSnapshotRetirementFactory, value));
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            self.terminal = true;
            return Ok(store::SnapshotRetirementStep::Complete);
        }
        let retirement = self.retirement.as_mut().expect("Draw clone retirement remains exact");
        match retirement.close_step(1, maximum_bytes)? {
            store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                drop(self.retirement.take());
                Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
            }
            store::SnapshotRetirementStep::Complete => Err("Draw clone retirement reported false terminal".into()),
            step => Ok(step),
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal && self.value.is_none() && self.retirement.is_none()
    }
}

impl Drop for DrawSnapshotCloneAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Draw snapshot clone reached Drop before exact handoff or cursor retirement");
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
                if envelope.schema != crate::artifacts::draw::DRAW_SCHEMA || envelope.id.is_empty() || envelope.id.len() > DRAW_OWNED_FIELD_BYTES {
                    self.fail(b"draw-store.initializer-envelope-invalid");
                } else {
                    self.phase = DrawStoreInitializationPhase::ValidateEditPair { left: 0, right: 1 };
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
                } else if envelope.vcs.edits[left].id == envelope.vcs.edits[right].id || envelope.vcs.edits[left].id.len() > DRAW_OWNED_FIELD_BYTES {
                    self.fail(b"draw-store.initializer-duplicate-or-hostile-edit");
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
                let encoded = match operation.encode_op() {
                    Ok(encoded) if encoded.len() <= DRAW_OWNED_FIELD_BYTES => encoded,
                    _ => {
                        self.fail(b"draw-store.initializer-forward-encoding");
                        return semio_framework_job::StepOutcome::Yield;
                    }
                };
                self.edit_digest.as_mut().expect("Draw edit digest remains retained").observe(&encoded);
                let current = self.runtime.as_mut().and_then(store::ArtifactStoreInitializationRuntime::current_mut).expect("Draw runtime current snapshot remains retained");
                let (diff, messages) = operation.diff(current).into_parts();
                if messages.iter().any(|message| message.level == protocol::Severity::Fatal) {
                    self.fail(b"draw-store.initializer-fatal-mutation");
                    return semio_framework_job::StepOutcome::Yield;
                }
                match diff.apply(current) {
                    Ok(next) => {
                        let previous = std::mem::replace(current, next);
                        *self.active = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&DrawSnapshotRetirementFactory, previous));
                        self.phase = DrawStoreInitializationPhase::ApplyForward { position, edit, mutation: mutation + 1 };
                        cx.consume_fuel(encoded.len().max(1) as u64);
                    }
                    Err(error) => {
                        self.fault = Some(error.to_string().into_bytes());
                        self.phase = DrawStoreInitializationPhase::RetireFault;
                    }
                }
                semio_framework_job::StepOutcome::Yield
            }
            DrawStoreInitializationPhase::HashInverse { position, edit, mutation } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Draw applied edit remains retained");
                let Some(operation) = entry.inverse.get(mutation) else {
                    self.phase = DrawStoreInitializationPhase::CommitApplied { position, edit };
                    return semio_framework_job::StepOutcome::Yield;
                };
                match operation.encode_op() {
                    Ok(encoded) if encoded.len() <= DRAW_OWNED_FIELD_BYTES => {
                        self.edit_digest.as_mut().expect("Draw edit digest remains retained").observe(&encoded);
                        self.phase = DrawStoreInitializationPhase::HashInverse { position, edit, mutation: mutation + 1 };
                        cx.consume_fuel(encoded.len().max(1) as u64);
                    }
                    _ => self.fail(b"draw-store.initializer-inverse-encoding"),
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
                match operation.encode_op() {
                    Ok(encoded) if encoded.len() <= DRAW_OWNED_FIELD_BYTES => {
                        self.edit_digest.as_mut().expect("Draw redo digest remains retained").observe(&encoded);
                        self.phase = DrawStoreInitializationPhase::HashRedoForward { position, edit, mutation: mutation + 1 };
                        cx.consume_fuel(encoded.len().max(1) as u64);
                    }
                    _ => self.fail(b"draw-store.initializer-redo-forward-encoding"),
                }
                semio_framework_job::StepOutcome::Yield
            }
            DrawStoreInitializationPhase::HashRedoInverse { position, edit, mutation } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Draw redo edit remains retained");
                let Some(operation) = entry.inverse.get(mutation) else {
                    self.phase = DrawStoreInitializationPhase::CommitRedo { position, edit };
                    return semio_framework_job::StepOutcome::Yield;
                };
                match operation.encode_op() {
                    Ok(encoded) if encoded.len() <= DRAW_OWNED_FIELD_BYTES => {
                        self.edit_digest.as_mut().expect("Draw redo digest remains retained").observe(&encoded);
                        self.phase = DrawStoreInitializationPhase::HashRedoInverse { position, edit, mutation: mutation + 1 };
                        cx.consume_fuel(encoded.len().max(1) as u64);
                    }
                    _ => self.fail(b"draw-store.initializer-redo-inverse-encoding"),
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
