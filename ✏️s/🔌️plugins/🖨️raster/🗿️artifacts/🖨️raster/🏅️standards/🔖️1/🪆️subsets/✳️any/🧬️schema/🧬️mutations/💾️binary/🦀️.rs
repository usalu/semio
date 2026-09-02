//! ⚖️ Raster artifact — binary command protocol surface + laws (constitutional: spr).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::artifacts::raster::op::RasterMutation;
use crate::artifacts::raster::{RasterAssetChild, RasterImageAsset, RasterLayerNode, RasterOwnedMap, RasterOwnedMapInsert, RasterOwnedMapPageBacking, RasterSnapshot};
use protocol::{Mutation, OpBinary};

/// 📦️ Encodes a `RasterMutation` to its binary command form.
pub async fn encode_op(operation: &RasterMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `RasterMutation` from its binary command form.
pub async fn decode_op(bytes: &[u8]) -> Result<RasterMutation, protocol::ProtocolError> {
    RasterMutation::decode_op(bytes)
}

//#region 🔖️OwnedEnvelopeCatalog
const RASTER_OWNED_FIELD_BYTES: usize = store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;
const RASTER_MAXIMUM_NESTED_DEPTH: usize = 128;
const RASTER_RETIREMENT_LAYER_FRAMES: usize = RASTER_MAXIMUM_NESTED_DEPTH;
const RASTER_RETIREMENT_VALUE_FRAMES: usize = RASTER_MAXIMUM_NESTED_DEPTH * 2;
const RASTER_RETIREMENT_WRAPPER_FRAMES: usize = 16;
const RASTER_RETIREMENT_ADMITTED_FRAME_CAPACITY: usize = RASTER_RETIREMENT_LAYER_FRAMES + RASTER_RETIREMENT_VALUE_FRAMES + RASTER_RETIREMENT_WRAPPER_FRAMES;
const RASTER_RETIREMENT_REJECTED_OWNER_MARGIN: usize = 3;
const RASTER_RETIREMENT_STACK_CAPACITY: usize = RASTER_RETIREMENT_ADMITTED_FRAME_CAPACITY + RASTER_RETIREMENT_REJECTED_OWNER_MARGIN;
const RASTER_RETIREMENT_STACK_PAGE_CAPACITY: usize = 8;
const RASTER_RETIREMENT_STACK_PAGE_COUNT: usize = (RASTER_RETIREMENT_STACK_CAPACITY - 1 + RASTER_RETIREMENT_STACK_PAGE_CAPACITY - 1) / RASTER_RETIREMENT_STACK_PAGE_CAPACITY;
const RASTER_MAXIMUM_NESTED_ITEMS: usize = store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES;
const RASTER_MAXIMUM_NESTED_BYTES: usize = store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES;
const RASTER_CONTROL_BACKING_BYTES: usize = RASTER_OWNED_FIELD_BYTES;
const RASTER_NON_STACK_CONTROL_BACKINGS: usize = 13;
const RASTER_MAXIMUM_CONTROL_BACKINGS: usize = RASTER_RETIREMENT_STACK_PAGE_COUNT + RASTER_NON_STACK_CONTROL_BACKINGS;
const RASTER_MAXIMUM_CONTROL_BYTES: usize = RASTER_MAXIMUM_CONTROL_BACKINGS * RASTER_CONTROL_BACKING_BYTES;
const RASTER_RETIREMENT_PROCESS_OPERATION_CAPACITY: usize = store::ARTIFACT_ENVELOPE_FIELD_DECODER_CAPACITY;
const RASTER_RETIREMENT_PROCESS_PAGE_CAPACITY: usize = RASTER_RETIREMENT_STACK_PAGE_COUNT * RASTER_RETIREMENT_PROCESS_OPERATION_CAPACITY;
static RASTER_RETIREMENT_PROCESS_PAGES: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
const RASTER_INITIALIZATION_PROCESS_CONTROL_CAPACITY: usize = RASTER_NON_STACK_CONTROL_BACKINGS * RASTER_RETIREMENT_PROCESS_OPERATION_CAPACITY;
static RASTER_INITIALIZATION_PROCESS_CONTROLS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
const RASTER_STANDALONE_PROCESS_CONTROL_CAPACITY: usize = RASTER_NON_STACK_CONTROL_BACKINGS * RASTER_RETIREMENT_PROCESS_OPERATION_CAPACITY;
static RASTER_STANDALONE_PROCESS_CONTROLS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

struct RasterStandaloneControlCredit {
    held_items: usize,
    held_bytes: usize,
}

impl RasterStandaloneControlCredit {
    fn try_claim() -> Result<Self, &'static str> {
        let current = RASTER_STANDALONE_PROCESS_CONTROLS.load(std::sync::atomic::Ordering::Acquire);
        let next = current.checked_add(1).ok_or("raster-store.standalone-control-overflow")?;
        if next > RASTER_STANDALONE_PROCESS_CONTROL_CAPACITY {
            return Err("raster-store.standalone-control-capacity");
        }
        if RASTER_STANDALONE_PROCESS_CONTROLS.compare_exchange(current, next, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
            return Err("raster-store.standalone-control-capacity");
        }
        Ok(Self { held_items: 1, held_bytes: RASTER_CONTROL_BACKING_BYTES })
    }

    fn release(&mut self) -> Result<bool, &'static str> {
        if self.held_items != 1 || self.held_bytes != RASTER_CONTROL_BACKING_BYTES {
            return Err("raster-store.standalone-control-duplicate-release");
        }
        let current = RASTER_STANDALONE_PROCESS_CONTROLS.load(std::sync::atomic::Ordering::Acquire);
        let next = current.checked_sub(1).ok_or("raster-store.standalone-control-underflow")?;
        if RASTER_STANDALONE_PROCESS_CONTROLS.compare_exchange(current, next, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
            return Ok(false);
        }
        self.held_items = 0;
        self.held_bytes = 0;
        Ok(true)
    }
}

impl Drop for RasterStandaloneControlCredit {
    fn drop(&mut self) {
        assert!(self.held_items == 0 && self.held_bytes == 0, "Raster standalone control credit reached Drop before exact return");
    }
}

struct RasterInitializationControlReservation {
    remaining: usize,
    remaining_bytes: usize,
}

impl RasterInitializationControlReservation {
    fn try_claim() -> Result<Option<Self>, &'static str> {
        let current = RASTER_INITIALIZATION_PROCESS_CONTROLS.load(std::sync::atomic::Ordering::Acquire);
        let next = current.checked_add(RASTER_NON_STACK_CONTROL_BACKINGS).ok_or("raster-store.control-process-overflow")?;
        if next > RASTER_INITIALIZATION_PROCESS_CONTROL_CAPACITY {
            return Ok(None);
        }
        if RASTER_INITIALIZATION_PROCESS_CONTROLS.compare_exchange(current, next, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
            return Ok(None);
        }
        Ok(Some(Self { remaining: RASTER_NON_STACK_CONTROL_BACKINGS, remaining_bytes: RASTER_NON_STACK_CONTROL_BACKINGS * RASTER_CONTROL_BACKING_BYTES }))
    }

    fn return_one(&mut self) -> Result<bool, &'static str> {
        if self.remaining == 0 && self.remaining_bytes == 0 {
            return Ok(true);
        }
        if self.remaining == 0 || self.remaining_bytes < RASTER_CONTROL_BACKING_BYTES {
            return Err("raster-store.control-process-accounting");
        }
        let previous = RASTER_INITIALIZATION_PROCESS_CONTROLS.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
        if previous == 0 {
            return Err("raster-store.control-process-underflow");
        }
        self.remaining -= 1;
        self.remaining_bytes -= RASTER_CONTROL_BACKING_BYTES;
        Ok(self.remaining == 0)
    }
}

impl Drop for RasterInitializationControlReservation {
    fn drop(&mut self) {
        assert!(self.remaining == 0 && self.remaining_bytes == 0, "Raster initialization control reservation reached Drop before every exact backing credit was returned");
    }
}

enum RasterRetirementOwner {
    Snapshot(RasterSnapshot),
    Layer(RasterLayerNode),
    LayerFields(RasterLayerFields),
    Mutation(RasterMutation),
    MutationFields(RasterMutationFields),
    AssetEntry { key: String, child: Option<RasterAssetChild> },
    Asset(RasterImageAsset),
    AssetMapPage(RasterOwnedMapPageBacking<RasterAssetChild>),
    Value(dsl::DslValue),
    ValueEntry { key: String, value: Option<dsl::DslValue> },
    ValueMapPage(RasterOwnedMapPageBacking<dsl::DslValue>),
    BoxedLayer(Option<Box<RasterLayerNode>>),
    String(String),
    Bytes(Vec<u8>),
}

struct RasterLayerFields {
    strings: [Option<String>; 4],
    children: Option<Vec<RasterLayerNode>>,
    values: Option<RasterOwnedMap<dsl::DslValue>>,
    string_cursor: usize,
}

enum RasterMutationFields {
    String(String),
    Strings { first: String, second: Option<String> },
    Create { parent: Option<String>, layer: Option<Box<RasterLayerNode>> },
    Asset { id: String, asset: Option<RasterImageAsset> },
}

struct RasterRetirementFrame {
    owner: std::mem::ManuallyDrop<Option<RasterRetirementOwner>>,
    phase: u8,
}

struct RasterRetirementFramePage {
    frames: [Option<RasterRetirementFrame>; RASTER_RETIREMENT_STACK_PAGE_CAPACITY],
}

impl RasterRetirementFramePage {
    fn new() -> Self {
        Self { frames: std::array::from_fn(|_| None) }
    }
}

impl Drop for RasterRetirementFramePage {
    fn drop(&mut self) {
        assert!(self.frames.iter().all(Option::is_none), "Raster retirement frame page reached Drop before every admitted owner was returned");
    }
}

impl RasterRetirementFrame {
    fn new(owner: RasterRetirementOwner) -> Self {
        Self { owner: std::mem::ManuallyDrop::new(Some(owner)), phase: 0 }
    }
}

enum RasterRetirementAction {
    Pending { released_items: usize, released_bytes: usize },
    Push(RasterRetirementOwner),
    Pop,
}

struct RasterOwnedRetirement {
    root: std::mem::ManuallyDrop<Option<RasterRetirementFrame>>,
    pages: std::mem::ManuallyDrop<[Option<Box<RasterRetirementFramePage>>; RASTER_RETIREMENT_STACK_PAGE_COUNT]>,
    pending_push: std::mem::ManuallyDrop<Option<RasterRetirementOwner>>,
    pending_empty_page: Option<usize>,
    pending_page_credit: Option<usize>,
    page_credits: [bool; RASTER_RETIREMENT_STACK_PAGE_COUNT],
    control: std::mem::ManuallyDrop<Option<RasterStandaloneControlCredit>>,
    depth: usize,
}

impl RasterOwnedRetirement {
    fn new(owner: RasterRetirementOwner) -> Self {
        let control = RasterStandaloneControlCredit::try_claim().ok();
        Self {
            root: std::mem::ManuallyDrop::new(Some(RasterRetirementFrame::new(owner))),
            pages: std::mem::ManuallyDrop::new(std::array::from_fn(|_| None)),
            pending_push: std::mem::ManuallyDrop::new(None),
            pending_empty_page: None,
            pending_page_credit: None,
            page_credits: [false; RASTER_RETIREMENT_STACK_PAGE_COUNT],
            control: std::mem::ManuallyDrop::new(control),
            depth: 1,
        }
    }

    fn claim_control_if_available(&mut self) -> Result<bool, String> {
        if self.control.is_some() {
            return Ok(true);
        }
        match RasterStandaloneControlCredit::try_claim() {
            Ok(control) => {
                *self.control = Some(control);
                Ok(true)
            }
            Err("raster-store.standalone-control-capacity") => Ok(false),
            Err(code) => Err(code.into()),
        }
    }

    fn reserve_page_credit(&mut self, page_index: usize) -> Result<bool, String> {
        if page_index >= RASTER_RETIREMENT_STACK_PAGE_COUNT || self.page_credits[page_index] {
            return Err("Raster retirement page credit state was not empty".into());
        }
        let current = RASTER_RETIREMENT_PROCESS_PAGES.load(std::sync::atomic::Ordering::Acquire);
        let Some(next) = current.checked_add(1) else { return Err("Raster retirement process page credit overflow".into()) };
        if next > RASTER_RETIREMENT_PROCESS_PAGE_CAPACITY {
            return Ok(false);
        }
        if RASTER_RETIREMENT_PROCESS_PAGES.compare_exchange(current, next, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
            return Ok(false);
        }
        self.page_credits[page_index] = true;
        self.pending_page_credit = Some(page_index);
        Ok(true)
    }

    fn return_page_credit(&mut self, page_index: usize) -> Result<(), String> {
        if page_index >= RASTER_RETIREMENT_STACK_PAGE_COUNT || !self.page_credits[page_index] || self.pending_page_credit == Some(page_index) {
            return Err("Raster retirement page credit was not allocated".into());
        }
        self.page_credits[page_index] = false;
        let previous = RASTER_RETIREMENT_PROCESS_PAGES.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
        if previous == 0 {
            return Err("Raster retirement process page credit underflow".into());
        }
        Ok(())
    }

    fn page_and_slot(index: usize) -> (usize, usize) {
        let offset = index - 1;
        (offset / RASTER_RETIREMENT_STACK_PAGE_CAPACITY, offset % RASTER_RETIREMENT_STACK_PAGE_CAPACITY)
    }

    fn frame_mut(&mut self, index: usize) -> Option<&mut RasterRetirementFrame> {
        if index == 0 {
            return self.root.as_mut();
        }
        let (page, slot) = Self::page_and_slot(index);
        self.pages.get_mut(page)?.as_mut()?.frames.get_mut(slot)?.as_mut()
    }

    fn take_frame(&mut self, index: usize) -> Option<RasterRetirementFrame> {
        if index == 0 {
            return self.root.take();
        }
        let (page, slot) = Self::page_and_slot(index);
        self.pages.get_mut(page)?.as_mut()?.frames.get_mut(slot)?.take()
    }

    fn release_string(value: &mut String, phase: &mut u8, next: u8, maximum_items: usize, maximum_bytes: usize) -> RasterRetirementAction {
        if maximum_items == 0 || value.capacity() > maximum_bytes {
            return RasterRetirementAction::Pending { released_items: 0, released_bytes: 0 };
        }
        let value = std::mem::take(value);
        let released_bytes = value.capacity();
        drop(value);
        *phase = next;
        RasterRetirementAction::Pending { released_items: 1, released_bytes }
    }

    fn child_step<S>(child: &mut store::ArtifactChild<S>, phase: &mut u8, maximum_items: usize, maximum_bytes: usize) -> Option<RasterRetirementAction> {
        let step = match *phase {
            0 => Self::release_string(&mut child.child_id, phase, 1, maximum_items, maximum_bytes),
            1 => Self::release_string(&mut child.target.artifact_id, phase, 2, maximum_items, maximum_bytes),
            2 => Self::release_string(&mut child.target.dialect.artifact_kind, phase, 3, maximum_items, maximum_bytes),
            3 => Self::release_string(&mut child.target.dialect.standard, phase, 4, maximum_items, maximum_bytes),
            4 => Self::release_string(&mut child.target.dialect.subset, phase, 5, maximum_items, maximum_bytes),
            _ => return None,
        };
        Some(step)
    }

    fn layer_fields(layer: RasterLayerNode) -> RasterLayerFields {
        let mut strings: [Option<String>; 4] = Default::default();
        let (children, values) = match layer {
            RasterLayerNode::Pixel { id, name, blend_mode, image_key, .. } => {
                strings = [Some(id), Some(name), Some(blend_mode), image_key];
                (None, None)
            }
            RasterLayerNode::Group { id, name, blend_mode, children, .. } => {
                strings = [Some(id), Some(name), Some(blend_mode), None];
                (Some(children), None)
            }
            RasterLayerNode::Adjustment { id, name, blend_mode, adjustment_kind, params, .. } => {
                strings = [Some(id), Some(name), Some(blend_mode), Some(adjustment_kind)];
                (None, Some(params))
            }
        };
        RasterLayerFields { strings, children, values, string_cursor: 0 }
    }

    fn mutation_fields(mutation: RasterMutation) -> RasterMutationFields {
        use RasterMutation::*;
        match mutation {
            CreateLayer(payload) => RasterMutationFields::Create { parent: payload.parent_id, layer: Some(payload.layer) },
            DeleteLayer(payload) => RasterMutationFields::String(payload.layer_id),
            ReorderLayers(payload) => RasterMutationFields::Strings { first: payload.layer_id, second: payload.parent_id },
            RenameLayer(payload) => RasterMutationFields::Strings { first: payload.layer_id, second: Some(payload.new_name) },
            ChangeLayerVisible(payload) => RasterMutationFields::String(payload.layer_id),
            ChangeLayerOpacity(payload) => RasterMutationFields::String(payload.layer_id),
            ChangeLayerBlendMode(payload) => RasterMutationFields::Strings { first: payload.layer_id, second: Some(payload.new_blend_mode) },
            MoveLayer(payload) => RasterMutationFields::String(payload.layer_id),
            ResizeLayer(payload) => RasterMutationFields::String(payload.layer_id),
            ChangeLayerAdjustmentKind(payload) => RasterMutationFields::Strings { first: payload.layer_id, second: Some(payload.new_adjustment_kind) },
            AddLayerAsset(payload) => RasterMutationFields::Asset { id: payload.asset_id, asset: Some(payload.asset) },
            RemoveLayerAsset(payload) => RasterMutationFields::String(payload.asset_id),
        }
    }

    fn frame_action(frame: &mut RasterRetirementFrame, maximum_items: usize, maximum_bytes: usize) -> Result<RasterRetirementAction, String> {
        if maximum_items == 0 {
            return Ok(RasterRetirementAction::Pending { released_items: 0, released_bytes: 0 });
        }
        let Some(owner) = frame.owner.as_mut() else { return Ok(RasterRetirementAction::Pop) };
        match owner {
            RasterRetirementOwner::Snapshot(value) => match frame.phase {
                0 => {
                    if let Some(layer) = value.layers.pop() {
                        return Ok(RasterRetirementAction::Push(RasterRetirementOwner::Layer(layer)));
                    }
                    let layers = std::mem::take(&mut value.layers);
                    let bytes = layers.capacity().saturating_mul(std::mem::size_of::<RasterLayerNode>());
                    if bytes > maximum_bytes {
                        value.layers = layers;
                        return Ok(RasterRetirementAction::Pending { released_items: 0, released_bytes: 0 });
                    }
                    drop(layers);
                    frame.phase = 1;
                    Ok(RasterRetirementAction::Pending { released_items: 1, released_bytes: bytes })
                }
                1 => {
                    if let Some((key, child)) = value.assets.take_last_entry() {
                        return Ok(RasterRetirementAction::Push(RasterRetirementOwner::AssetEntry { key, child: Some(child) }));
                    }
                    if let Some(page) = value.assets.take_empty_page_backing() {
                        return Ok(RasterRetirementAction::Push(RasterRetirementOwner::AssetMapPage(page)));
                    }
                    drop(std::mem::take(&mut value.assets));
                    frame.phase = 2;
                    Ok(RasterRetirementAction::Pending { released_items: 1, released_bytes: 0 })
                }
                2 => Ok(Self::release_string(&mut value.schema, &mut frame.phase, 3, maximum_items, maximum_bytes)),
                3 => Ok(Self::release_string(&mut value.id, &mut frame.phase, 4, maximum_items, maximum_bytes)),
                4 if value.title.is_some() => {
                    let title = value.title.take().expect("Raster title remains retained");
                    frame.phase = 5;
                    Ok(RasterRetirementAction::Push(RasterRetirementOwner::String(title)))
                }
                _ => {
                    drop(frame.owner.take());
                    Ok(RasterRetirementAction::Pop)
                }
            },
            RasterRetirementOwner::Layer(_) => {
                let layer = match frame.owner.take() {
                    Some(RasterRetirementOwner::Layer(layer)) => layer,
                    _ => unreachable!("Raster layer retirement preserves its exact variant"),
                };
                *frame.owner = Some(RasterRetirementOwner::LayerFields(Self::layer_fields(layer)));
                frame.phase = 0;
                Ok(RasterRetirementAction::Pending { released_items: 0, released_bytes: 0 })
            }
            RasterRetirementOwner::LayerFields(fields) => {
                if fields.string_cursor < fields.strings.len() {
                    let index = fields.string_cursor;
                    fields.string_cursor += 1;
                    if let Some(value) = fields.strings[index].take() {
                        return Ok(RasterRetirementAction::Push(RasterRetirementOwner::String(value)));
                    }
                    return Ok(RasterRetirementAction::Pending { released_items: 1, released_bytes: 0 });
                }
                if let Some(layer) = fields.children.as_mut().and_then(Vec::pop) {
                    return Ok(RasterRetirementAction::Push(RasterRetirementOwner::Layer(layer)));
                }
                if fields.children.as_ref().is_some_and(Vec::is_empty) {
                    let children = fields.children.take().expect("Raster empty child vector remains retained");
                    let bytes = children.capacity().saturating_mul(std::mem::size_of::<RasterLayerNode>());
                    if bytes > maximum_bytes {
                        fields.children = Some(children);
                        return Ok(RasterRetirementAction::Pending { released_items: 0, released_bytes: 0 });
                    }
                    drop(children);
                    return Ok(RasterRetirementAction::Pending { released_items: 1, released_bytes: bytes });
                }
                if let Some((key, value)) = fields.values.as_mut().and_then(RasterOwnedMap::take_last_entry) {
                    return Ok(RasterRetirementAction::Push(RasterRetirementOwner::ValueEntry { key, value: Some(value) }));
                }
                if let Some(page) = fields.values.as_mut().and_then(RasterOwnedMap::take_empty_page_backing) {
                    return Ok(RasterRetirementAction::Push(RasterRetirementOwner::ValueMapPage(page)));
                }
                if fields.values.as_ref().is_some_and(RasterOwnedMap::is_empty) {
                    drop(fields.values.take());
                    return Ok(RasterRetirementAction::Pending { released_items: 1, released_bytes: 0 });
                }
                drop(frame.owner.take());
                Ok(RasterRetirementAction::Pop)
            }
            RasterRetirementOwner::Mutation(_) => {
                let mutation = match frame.owner.take() {
                    Some(RasterRetirementOwner::Mutation(mutation)) => mutation,
                    _ => unreachable!("Raster mutation retirement preserves its exact variant"),
                };
                *frame.owner = Some(RasterRetirementOwner::MutationFields(Self::mutation_fields(mutation)));
                frame.phase = 0;
                Ok(RasterRetirementAction::Pending { released_items: 0, released_bytes: 0 })
            }
            RasterRetirementOwner::MutationFields(fields) => match fields {
                RasterMutationFields::String(value) => {
                    if frame.phase == 0 {
                        return Ok(Self::release_string(value, &mut frame.phase, 1, maximum_items, maximum_bytes));
                    }
                    drop(frame.owner.take());
                    Ok(RasterRetirementAction::Pop)
                }
                RasterMutationFields::Strings { first, second } => match frame.phase {
                    0 => Ok(Self::release_string(first, &mut frame.phase, 1, maximum_items, maximum_bytes)),
                    1 if second.is_some() => {
                        frame.phase = 2;
                        Ok(RasterRetirementAction::Push(RasterRetirementOwner::String(second.take().expect("Raster mutation second string remains retained"))))
                    }
                    _ => {
                        drop(frame.owner.take());
                        Ok(RasterRetirementAction::Pop)
                    }
                },
                RasterMutationFields::Create { parent, layer } => match frame.phase {
                    0 if parent.is_some() => {
                        frame.phase = 1;
                        Ok(RasterRetirementAction::Push(RasterRetirementOwner::String(parent.take().expect("Raster create parent remains retained"))))
                    }
                    0 => {
                        frame.phase = 1;
                        Ok(RasterRetirementAction::Pending { released_items: 0, released_bytes: 0 })
                    }
                    1 if layer.is_some() => {
                        let layer = layer.take().expect("Raster create layer remains retained");
                        frame.phase = 2;
                        Ok(RasterRetirementAction::Push(RasterRetirementOwner::BoxedLayer(Some(layer))))
                    }
                    _ => {
                        drop(frame.owner.take());
                        Ok(RasterRetirementAction::Pop)
                    }
                },
                RasterMutationFields::Asset { id, asset } => match frame.phase {
                    0 => Ok(Self::release_string(id, &mut frame.phase, 1, maximum_items, maximum_bytes)),
                    1 if asset.is_some() => {
                        frame.phase = 2;
                        Ok(RasterRetirementAction::Push(RasterRetirementOwner::Asset(asset.take().expect("Raster mutation asset remains retained"))))
                    }
                    _ => {
                        drop(frame.owner.take());
                        Ok(RasterRetirementAction::Pop)
                    }
                },
            },
            RasterRetirementOwner::AssetEntry { key, child } => match frame.phase {
                0 => Ok(Self::release_string(key, &mut frame.phase, 1, maximum_items, maximum_bytes)),
                1 => {
                    let child = child.as_mut().ok_or_else(|| "Raster child owner missing".to_string())?;
                    let mut child_phase = frame.phase - 1;
                    let step = Self::child_step(child, &mut child_phase, maximum_items, maximum_bytes).ok_or_else(|| "Raster child phase overflow".to_string())?;
                    frame.phase = child_phase + 1;
                    Ok(step)
                }
                2..=5 => {
                    let child = child.as_mut().ok_or_else(|| "Raster child owner missing".to_string())?;
                    let mut child_phase = frame.phase - 1;
                    let step = Self::child_step(child, &mut child_phase, maximum_items, maximum_bytes).ok_or_else(|| "Raster child phase overflow".to_string())?;
                    frame.phase = child_phase + 1;
                    Ok(step)
                }
                _ => {
                    drop(child.take());
                    drop(frame.owner.take());
                    Ok(RasterRetirementAction::Pop)
                }
            },
            RasterRetirementOwner::Asset(value) => match frame.phase {
                0 => Ok(Self::release_string(&mut value.mime, &mut frame.phase, 1, maximum_items, maximum_bytes)),
                1 => {
                    frame.phase = 2;
                    Ok(RasterRetirementAction::Push(RasterRetirementOwner::Bytes(std::mem::take(&mut value.data))))
                }
                _ => {
                    drop(frame.owner.take());
                    Ok(RasterRetirementAction::Pop)
                }
            },
            RasterRetirementOwner::AssetMapPage(page) => {
                let bytes = page.conservative_credit_bytes();
                if bytes > maximum_bytes {
                    return Ok(RasterRetirementAction::Pending { released_items: 0, released_bytes: 0 });
                }
                let page = match frame.owner.take() {
                    Some(RasterRetirementOwner::AssetMapPage(page)) => page,
                    _ => unreachable!("Raster asset map page keeps its exact owner"),
                };
                page.release();
                Ok(RasterRetirementAction::Pending { released_items: 1, released_bytes: bytes })
            }
            RasterRetirementOwner::Value(value) => match value {
                dsl::DslValue::String(value) => {
                    if frame.phase == 0 {
                        return Ok(Self::release_string(value, &mut frame.phase, 1, maximum_items, maximum_bytes));
                    }
                    drop(frame.owner.take());
                    Ok(RasterRetirementAction::Pop)
                }
                dsl::DslValue::Array(values) => {
                    if let Some(value) = values.pop() {
                        Ok(RasterRetirementAction::Push(RasterRetirementOwner::Value(value)))
                    } else {
                        let bytes = values.capacity().saturating_mul(std::mem::size_of::<dsl::DslValue>());
                        if bytes > maximum_bytes {
                            return Ok(RasterRetirementAction::Pending { released_items: 0, released_bytes: 0 });
                        }
                        drop(frame.owner.take());
                        Ok(RasterRetirementAction::Pending { released_items: 1, released_bytes: bytes })
                    }
                }
                dsl::DslValue::Object(values) => {
                    if let Some((key, value)) = values.pop() {
                        Ok(RasterRetirementAction::Push(RasterRetirementOwner::ValueEntry { key, value: Some(value) }))
                    } else {
                        let bytes = values.capacity().saturating_mul(std::mem::size_of::<(String, dsl::DslValue)>());
                        if bytes > maximum_bytes {
                            return Ok(RasterRetirementAction::Pending { released_items: 0, released_bytes: 0 });
                        }
                        drop(frame.owner.take());
                        Ok(RasterRetirementAction::Pending { released_items: 1, released_bytes: bytes })
                    }
                }
                dsl::DslValue::Null | dsl::DslValue::Bool(_) | dsl::DslValue::Number(_) => {
                    drop(frame.owner.take());
                    Ok(RasterRetirementAction::Pop)
                }
            },
            RasterRetirementOwner::ValueEntry { key, value } => match frame.phase {
                0 => Ok(Self::release_string(key, &mut frame.phase, 1, maximum_items, maximum_bytes)),
                1 => {
                    frame.phase = 2;
                    Ok(RasterRetirementAction::Push(RasterRetirementOwner::Value(value.take().ok_or_else(|| "Raster value entry owner missing".to_string())?)))
                }
                _ => {
                    drop(frame.owner.take());
                    Ok(RasterRetirementAction::Pop)
                }
            },
            RasterRetirementOwner::ValueMapPage(page) => {
                let bytes = page.conservative_credit_bytes();
                if bytes > maximum_bytes {
                    return Ok(RasterRetirementAction::Pending { released_items: 0, released_bytes: 0 });
                }
                let page = match frame.owner.take() {
                    Some(RasterRetirementOwner::ValueMapPage(page)) => page,
                    _ => unreachable!("Raster value map page keeps its exact owner"),
                };
                page.release();
                Ok(RasterRetirementAction::Pending { released_items: 1, released_bytes: bytes })
            }
            RasterRetirementOwner::BoxedLayer(layer) => {
                if RASTER_CONTROL_BACKING_BYTES > maximum_bytes {
                    return Ok(RasterRetirementAction::Pending { released_items: 0, released_bytes: 0 });
                }
                let layer = layer.take().ok_or_else(|| "Raster boxed layer owner missing".to_string())?;
                let layer = *layer;
                *frame.owner = Some(RasterRetirementOwner::Layer(layer));
                Ok(RasterRetirementAction::Pending { released_items: 1, released_bytes: RASTER_CONTROL_BACKING_BYTES })
            }
            RasterRetirementOwner::String(value) => {
                if frame.phase == 0 {
                    return Ok(Self::release_string(value, &mut frame.phase, 1, maximum_items, maximum_bytes));
                }
                drop(frame.owner.take());
                Ok(RasterRetirementAction::Pop)
            }
            RasterRetirementOwner::Bytes(value) => {
                if value.capacity() > maximum_bytes {
                    return Ok(RasterRetirementAction::Pending { released_items: 0, released_bytes: 0 });
                }
                let bytes = value.capacity();
                drop(frame.owner.take());
                Ok(RasterRetirementAction::Pending { released_items: 1, released_bytes: bytes })
            }
        }
    }

    fn advance(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.depth > 0 && self.control.is_none() {
            let _ = self.claim_control_if_available()?;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(page_index) = self.pending_empty_page {
            if maximum_bytes < RASTER_CONTROL_BACKING_BYTES {
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            let page = self.pages.get_mut(page_index).and_then(Option::take).ok_or_else(|| "Raster empty retirement page owner missing".to_string())?;
            drop(page);
            self.return_page_credit(page_index)?;
            self.pending_empty_page = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: RASTER_CONTROL_BACKING_BYTES });
        }
        if self.pending_push.is_some() {
            if self.depth >= RASTER_RETIREMENT_STACK_CAPACITY {
                return Err("Raster retirement exceeded its admitted fixed depth".into());
            }
            let (page_index, slot) = Self::page_and_slot(self.depth);
            if self.pages[page_index].is_none() {
                if std::mem::size_of::<RasterRetirementFramePage>() > RASTER_CONTROL_BACKING_BYTES {
                    return Err("Raster retirement frame page exceeded its conservative control credit".into());
                }
                if self.pending_page_credit.is_none() {
                    if !self.reserve_page_credit(page_index)? {
                        return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                    }
                    return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                }
                if self.pending_page_credit != Some(page_index) || !self.page_credits[page_index] {
                    return Err("Raster retirement frame page allocation lost its exact admitted credit".into());
                }
                if maximum_bytes < RASTER_CONTROL_BACKING_BYTES {
                    return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                }
                self.pages[page_index] = Some(Box::new(RasterRetirementFramePage::new()));
                self.pending_page_credit = None;
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            let occupied = self.pages[page_index].as_ref().and_then(|page| page.frames.get(slot)).ok_or_else(|| "Raster retirement frame page slot missing".to_string())?.is_some();
            if occupied {
                return Err("Raster retirement frame page slot remained occupied".into());
            }
            let owner = self.pending_push.take().ok_or_else(|| "Raster pending retirement owner missing".to_string())?;
            let target = self.pages[page_index].as_mut().and_then(|page| page.frames.get_mut(slot)).ok_or_else(|| "Raster retirement frame page slot missing".to_string())?;
            *target = Some(RasterRetirementFrame::new(owner));
            self.depth += 1;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.depth == 0 {
            if let Some(control) = self.control.as_mut() {
                if maximum_bytes < RASTER_CONTROL_BACKING_BYTES {
                    return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                }
                if !control.release()? {
                    return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                }
                drop(self.control.take());
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: RASTER_CONTROL_BACKING_BYTES });
            }
            return Ok(store::SnapshotRetirementStep::Complete);
        }
        let action = {
            let index = self.depth - 1;
            let frame = self.frame_mut(index).ok_or_else(|| "Raster retirement top frame missing".to_string())?;
            Self::frame_action(frame, maximum_items, maximum_bytes)?
        };
        match action {
            RasterRetirementAction::Pending { released_items, released_bytes } => Ok(store::SnapshotRetirementStep::Pending { released_items, released_bytes }),
            RasterRetirementAction::Push(owner) => {
                if self.depth >= RASTER_RETIREMENT_STACK_CAPACITY {
                    *self.pending_push = Some(owner);
                    return Err("Raster retirement exceeded its admitted fixed depth".into());
                }
                *self.pending_push = Some(owner);
                Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
            }
            RasterRetirementAction::Pop => {
                let index = self.depth - 1;
                let frame = self.take_frame(index).expect("Raster retirement completed frame remains retained");
                if frame.owner.is_some() {
                    return Err("Raster retirement attempted to release a nonempty frame".into());
                }
                drop(frame);
                self.depth -= 1;
                if index > 0 {
                    let (page, slot) = Self::page_and_slot(index);
                    if slot == 0 {
                        self.pending_empty_page = Some(page);
                    }
                }
                Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
            }
        }
    }
}

impl store::ErasedSnapshotRetirement for RasterOwnedRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        self.advance(maximum_items.min(1), maximum_bytes)
    }

    fn terminal_is_empty(&self) -> bool {
        self.depth == 0
            && self.pending_push.is_none()
            && self.pending_empty_page.is_none()
            && self.pending_page_credit.is_none()
            && self.page_credits.iter().all(|credit| !credit)
            && self.control.is_none()
            && self.root.is_none()
            && self.pages.iter().all(Option::is_none)
    }
}

impl Drop for RasterOwnedRetirement {
    fn drop(&mut self) {
        assert!(store::ErasedSnapshotRetirement::terminal_is_empty(self), "Raster owner reached Drop before cursor retirement reached terminal-empty");
    }
}

pub struct RasterSnapshotRetirementFactory;

impl store::ArtifactOwnedValueRetirementFactory<RasterSnapshot> for RasterSnapshotRetirementFactory {
    fn retire_owned(&self, value: RasterSnapshot) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(RasterOwnedRetirement::new(RasterRetirementOwner::Snapshot(value)))
    }
}

struct RasterSnapshotRootRetirement {
    owner: std::mem::ManuallyDrop<Option<std::sync::Arc<RasterSnapshot>>>,
    value: std::mem::ManuallyDrop<Option<RasterSnapshot>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    control: std::mem::ManuallyDrop<Option<RasterStandaloneControlCredit>>,
    control_returned: bool,
}

impl store::ErasedSnapshotRetirement for RasterSnapshotRootRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.control.is_none() && !self.control_returned {
            match RasterStandaloneControlCredit::try_claim() {
                Ok(control) => {
                    *self.control = Some(control);
                    return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                }
                Err("raster-store.standalone-control-capacity") => return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }),
                Err(code) => return Err(code.into()),
            }
        }
        if let Some(retirement) = self.retirement.as_mut() {
            return match retirement.close_step(1, maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                    if maximum_bytes < RASTER_CONTROL_BACKING_BYTES {
                        return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                    }
                    drop(self.retirement.take());
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: RASTER_CONTROL_BACKING_BYTES })
                }
                store::SnapshotRetirementStep::Complete => Err("Raster snapshot root retirement reported false terminal".into()),
                step => Ok(step),
            };
        }
        if self.value.is_some() && maximum_bytes < RASTER_CONTROL_BACKING_BYTES {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(value) = self.value.take() {
            *self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&RasterSnapshotRetirementFactory, value));
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.owner.is_some() && maximum_bytes < RASTER_CONTROL_BACKING_BYTES {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        let Some(owner) = self.owner.take() else {
            if let Some(control) = self.control.as_mut() {
                if maximum_bytes < RASTER_CONTROL_BACKING_BYTES {
                    return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                }
                if !control.release()? {
                    return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                }
                drop(self.control.take());
                self.control_returned = true;
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: RASTER_CONTROL_BACKING_BYTES });
            }
            return Ok(store::SnapshotRetirementStep::Complete);
        };
        match std::sync::Arc::try_unwrap(owner) {
            Ok(value) => {
                *self.value = Some(value);
                Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: RASTER_CONTROL_BACKING_BYTES })
            }
            Err(owner) => {
                *self.owner = Some(owner);
                Ok(store::SnapshotRetirementStep::Blocked)
            }
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.owner.is_none() && self.value.is_none() && self.retirement.is_none() && self.control.is_none() && self.control_returned
    }
}

impl Drop for RasterSnapshotRootRetirement {
    fn drop(&mut self) {
        assert!(self.owner.is_none() && self.value.is_none() && self.retirement.is_none() && self.control.is_none() && self.control_returned, "Raster snapshot root reached Drop before exact Arc handback");
    }
}

impl store::SnapshotRetirementFactory<RasterSnapshot> for RasterSnapshotRetirementFactory {
    fn retire(&self, snapshot: std::sync::Arc<RasterSnapshot>) -> Box<dyn store::ErasedSnapshotRetirement> {
        let control = RasterStandaloneControlCredit::try_claim().ok();
        Box::new(RasterSnapshotRootRetirement {
            owner: std::mem::ManuallyDrop::new(Some(snapshot)),
            value: std::mem::ManuallyDrop::new(None),
            retirement: std::mem::ManuallyDrop::new(None),
            control: std::mem::ManuallyDrop::new(control),
            control_returned: false,
        })
    }
}

pub struct RasterMutationRetirementFactory;

impl store::ArtifactOwnedValueRetirementFactory<RasterMutation> for RasterMutationRetirementFactory {
    fn retire_owned(&self, value: RasterMutation) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(RasterOwnedRetirement::new(RasterRetirementOwner::Mutation(value)))
    }
}

fn decode_raster_snapshot_pack(bytes: &[u8]) -> Result<RasterSnapshot, ()> {
    <RasterSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|_| ())
}

fn decode_raster_mutation_pack(bytes: &[u8]) -> Result<RasterMutation, ()> {
    RasterMutation::decode_op(bytes).map_err(|_| ())
}

macro_rules! raster_owned_field_authority {
    ($state:ident, $authority:ident, $value:ty, $authority_trait:ident, $target_trait:ident, $publish:ident, $decode:path, $factory:expr, $kind:literal) => {
        enum $state {
            AwaitToken,
            Decode(store::OwnedSchemaHexAuthority<RASTER_OWNED_FIELD_BYTES>),
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
            retirement_terminal: bool,
        }

        impl $authority {
            fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Self {
                Self { operation, generation, path, state: $state::AwaitToken, value: std::mem::ManuallyDrop::new(None), retirement: std::mem::ManuallyDrop::new(None), retirement_terminal: false }
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
                if cx.operation() != self.operation || cx.generation() != self.generation {
                    return Err(self.diagnostic(concat!("raster-envelope.", $kind, "-stale-authority"), token.start));
                }
                if cx.is_cancelled() {
                    return Err(self.diagnostic(concat!("raster-envelope.", $kind, "-cancelled"), token.start));
                }
                if cx.should_yield() {
                    return Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending);
                }
                let path = self.path;
                let diagnostic = |code: &'static str, offset| store::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path };
                if matches!(self.state, $state::AwaitToken) {
                    if !terminal {
                        return Err(diagnostic(concat!("raster-envelope.", $kind, "-pack-must-be-scalar"), token.start));
                    }
                    self.state = $state::Decode(store::OwnedSchemaHexAuthority::try_new(self.operation, self.generation, token, self.path)?);
                }
                let $state::Decode(authority) = &mut self.state else {
                    return Err(diagnostic(concat!("raster-envelope.", $kind, "-pack-token-replayed"), token.start));
                };
                match authority.step(source, cx) {
                    store::OwnedSchemaHexStep::Pending => Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending),
                    store::OwnedSchemaHexStep::Complete => {
                        let bytes = authority.as_bytes().ok_or_else(|| diagnostic(concat!("raster-envelope.", $kind, "-pack-missing"), token.start))?;
                        let value = $decode(bytes).map_err(|_| diagnostic(concat!("raster-envelope.", $kind, "-pack-malformed"), token.start))?;
                        if !authority.release() {
                            return Err(diagnostic(concat!("raster-envelope.", $kind, "-pack-release-duplicate"), token.start));
                        }
                        *self.value = Some(value);
                        self.state = $state::Ready;
                        Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
                    }
                    store::OwnedSchemaHexStep::Cancelled => Err(diagnostic(concat!("raster-envelope.", $kind, "-pack-cancelled"), token.start)),
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
                    return Err(self.diagnostic(concat!("raster-envelope.", $kind, "-pack-not-ready"), 0));
                }
                let value = self.value.take().ok_or_else(|| self.diagnostic(concat!("raster-envelope.", $kind, "-owner-missing"), 0))?;
                target.$publish(reservation, value);
                self.state = $state::Published;
                Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
            }

            fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, store::OwnedSchemaDecodeDiagnostic> {
                if maximum_items == 0 {
                    return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                }
                if self.retirement_terminal {
                    if maximum_bytes < RASTER_CONTROL_BACKING_BYTES {
                        return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                    }
                    drop(self.retirement.take());
                    self.retirement_terminal = false;
                    self.state = $state::Complete;
                    return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: RASTER_CONTROL_BACKING_BYTES });
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
                let retirement = self.retirement.as_mut().expect("Raster packed field retirement remains retained");
                match retirement.close_step(maximum_items.min(1), maximum_bytes).map_err(|_| store::OwnedSchemaDecodeDiagnostic { code: concat!("raster-envelope.", $kind, "-retirement-fault"), offset: 0, line: 0, column: 0, path })? {
                    store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                        self.retirement_terminal = true;
                        Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                    }
                    store::SnapshotRetirementStep::Complete => Err(self.diagnostic(concat!("raster-envelope.", $kind, "-retirement-false-terminal"), 0)),
                    step => Ok(step),
                }
            }

            fn terminal_is_empty(&self) -> bool {
                matches!(self.state, $state::Published | $state::Complete) && self.value.is_none() && self.retirement.is_none() && !self.retirement_terminal
            }
        }

        impl Drop for $authority {
            fn drop(&mut self) {
                assert!(
                    matches!(self.state, $state::Published | $state::Complete) && self.value.is_none() && self.retirement.is_none() && !self.retirement_terminal,
                    concat!("Raster ", $kind, " decode reached Drop before publication or bounded retirement"),
                );
            }
        }
    };
}

raster_owned_field_authority!(
    RasterSnapshotDecodeState,
    RasterSnapshotDecodeAuthority,
    RasterSnapshot,
    ArtifactEnvelopeSnapshotFieldAuthority,
    ArtifactEnvelopeSnapshotFieldTarget,
    publish_snapshot_reserved,
    decode_raster_snapshot_pack,
    &RasterSnapshotRetirementFactory,
    "snapshot"
);

raster_owned_field_authority!(
    RasterMutationDecodeState,
    RasterMutationDecodeAuthority,
    RasterMutation,
    ArtifactEnvelopeMutationFieldAuthority,
    ArtifactEnvelopeMutationFieldTarget,
    publish_mutation_reserved,
    decode_raster_mutation_pack,
    &RasterMutationRetirementFactory,
    "mutation"
);

struct RasterRejectedConflictAuthority {
    terminal: bool,
}

impl store::ArtifactEnvelopeSprConflictAuthority for RasterRejectedConflictAuthority {
    fn accept_token(
        &mut self,
        token: store::OwnedSchemaToken,
        _terminal: bool,
        _source: &store::OwnedSchemaRecordCursor,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        Err(store::OwnedSchemaDecodeDiagnostic { code: "raster-envelope.fresh-conflict-not-admitted", offset: token.start, line: 0, column: 0, path: store::OwnedSchemaPath::ROOT })
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

pub struct RasterEnvelopeOwnedFieldCatalog;

impl store::ArtifactEnvelopeOwnedFieldCatalog<RasterSnapshot, RasterMutation> for RasterEnvelopeOwnedFieldCatalog {
    fn begin_vcs(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeVcsFieldAuthority<RasterSnapshot, RasterMutation>> {
        Box::new(store::ArtifactEnvelopeFreshVcsAuthority::new(self.begin_snapshot(operation, generation, path), std::sync::Arc::new(RasterSnapshotRetirementFactory), std::sync::Arc::new(RasterMutationRetirementFactory), self.edit_history_decoder()))
    }

    fn begin_snapshot(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeSnapshotFieldAuthority<RasterSnapshot>> {
        Box::new(RasterSnapshotDecodeAuthority::new(operation, generation, path))
    }

    fn begin_mutation(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeMutationFieldAuthority<RasterMutation>> {
        Box::new(RasterMutationDecodeAuthority::new(operation, generation, path))
    }

    fn begin_spr_conflict(&self, _operation: semio_framework_job::OperationId, _generation: semio_framework_job::Generation, _path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeSprConflictAuthority> {
        Box::new(RasterRejectedConflictAuthority { terminal: false })
    }

    fn edit_history_decoder(&self) -> std::sync::Arc<dyn store::ArtifactOwnedHistoryEntryDecoder<protocol::Edit<RasterMutation>>> {
        store::artifact_owned_spr_edit_history_decoder(std::sync::Arc::new(Self), std::sync::Arc::new(RasterMutationRetirementFactory))
    }
}

pub fn raster_envelope_decode_owner_bundle() -> store::ArtifactEnvelopeDecodeOwnerBundle<RasterSnapshot, RasterMutation> {
    store::ArtifactEnvelopeDecodeOwnerBundle::new(std::sync::Arc::new(RasterEnvelopeOwnedFieldCatalog), std::sync::Arc::new(RasterSnapshotRetirementFactory), std::sync::Arc::new(RasterMutationRetirementFactory))
}
//#endregion 🔖️OwnedEnvelopeCatalog

//#region 🔖️RetainedStoreInitialization
#[derive(Clone, Copy)]
struct RasterTraversalFrame {
    phase: u8,
    child: usize,
}

impl RasterTraversalFrame {
    const EMPTY: Self = Self { phase: 0, child: 0 };
}

fn raster_reserve_unit(cx: &mut semio_framework_job::StepContext<'_>) -> bool {
    if cx.should_yield() || cx.fuel_remaining() == 0 {
        return false;
    }
    cx.consume_fuel(1);
    true
}

fn raster_retirement_frame_requirement(layer_depth: usize, value_depth: usize) -> Result<usize, &'static str> {
    let required =
        layer_depth.checked_add(value_depth.checked_mul(2).ok_or("raster-store.preflight-combined-depth-overflow")?).and_then(|value| value.checked_add(RASTER_RETIREMENT_WRAPPER_FRAMES)).ok_or("raster-store.preflight-combined-depth-overflow")?;
    if required > RASTER_RETIREMENT_ADMITTED_FRAME_CAPACITY {
        return Err("raster-store.preflight-combined-depth");
    }
    Ok(required)
}

struct RasterOwnerTotals {
    source_items: usize,
    source_bytes: usize,
    candidate_items: usize,
    candidate_bytes: usize,
    source_control_items: usize,
    source_control_bytes: usize,
    candidate_control_items: usize,
    candidate_control_bytes: usize,
}

impl RasterOwnerTotals {
    fn new() -> Self {
        Self { source_items: 0, source_bytes: 0, candidate_items: 0, candidate_bytes: 0, source_control_items: 0, source_control_bytes: 0, candidate_control_items: 0, candidate_control_bytes: 0 }
    }

    fn add(&mut self, items: usize, bytes: usize, candidate_items: usize, candidate_bytes: usize) -> Result<(), &'static str> {
        self.source_items = self.source_items.checked_add(items).ok_or("raster-store.preflight-source-item-overflow")?;
        self.source_bytes = self.source_bytes.checked_add(bytes).ok_or("raster-store.preflight-source-byte-overflow")?;
        self.candidate_items = self.candidate_items.checked_add(candidate_items).ok_or("raster-store.preflight-candidate-item-overflow")?;
        self.candidate_bytes = self.candidate_bytes.checked_add(candidate_bytes).ok_or("raster-store.preflight-candidate-byte-overflow")?;
        if self.source_items > RASTER_MAXIMUM_NESTED_ITEMS || self.candidate_items > RASTER_MAXIMUM_NESTED_ITEMS {
            return Err("raster-store.preflight-item-capacity");
        }
        if self.source_bytes > RASTER_MAXIMUM_NESTED_BYTES || self.candidate_bytes > RASTER_MAXIMUM_NESTED_BYTES {
            return Err("raster-store.preflight-byte-capacity");
        }
        Ok(())
    }

    fn string(&mut self, value: &String) -> Result<(), &'static str> {
        if value.capacity() > RASTER_OWNED_FIELD_BYTES {
            return Err("raster-store.preflight-string-allocation-capacity");
        }
        self.add(1, value.capacity(), 1, value.len())
    }

    fn vector<T>(&mut self, value: &Vec<T>) -> Result<(), &'static str> {
        let bytes = value.capacity().checked_mul(std::mem::size_of::<T>()).ok_or("raster-store.preflight-vector-byte-overflow")?;
        if bytes > RASTER_OWNED_FIELD_BYTES {
            return Err("raster-store.preflight-vector-allocation-capacity");
        }
        self.add(1, bytes, 1, bytes)
    }

    fn layer_vector(&mut self, value: &Vec<RasterLayerNode>) -> Result<(), &'static str> {
        let source_bytes = value.capacity().checked_mul(std::mem::size_of::<RasterLayerNode>()).ok_or("raster-store.preflight-layer-vector-byte-overflow")?;
        let candidate_capacity = value.capacity().checked_add(1).ok_or("raster-store.preflight-layer-vector-capacity-overflow")?;
        let candidate_bytes = candidate_capacity.checked_mul(std::mem::size_of::<RasterLayerNode>()).ok_or("raster-store.preflight-layer-candidate-byte-overflow")?;
        if source_bytes > RASTER_OWNED_FIELD_BYTES || candidate_bytes > RASTER_OWNED_FIELD_BYTES {
            return Err("raster-store.preflight-layer-vector-allocation-capacity");
        }
        self.add(1, source_bytes, 1, candidate_bytes)
    }

    fn map<V>(&mut self, value: &RasterOwnedMap<V>, candidate_extra_entries: usize) -> Result<(), &'static str> {
        let source_pages = value.allocated_page_count();
        let candidate_entries = value.len().checked_add(candidate_extra_entries).ok_or("raster-store.preflight-map-entry-overflow")?;
        if candidate_entries > crate::artifacts::raster::RASTER_OWNED_MAP_CAPACITY {
            return Err("raster-store.preflight-map-item-capacity");
        }
        let candidate_pages = candidate_entries.div_ceil(crate::artifacts::raster::RASTER_OWNED_MAP_PAGE_CAPACITY);
        let page_bytes = RasterOwnedMap::<V>::conservative_page_credit_bytes();
        self.add(source_pages, source_pages.checked_mul(page_bytes).ok_or("raster-store.preflight-map-source-byte-overflow")?, candidate_pages, candidate_pages.checked_mul(page_bytes).ok_or("raster-store.preflight-map-candidate-byte-overflow")?)
    }

    fn observe_candidate_capacity(&mut self, requested: usize, observed: usize, element_bytes: usize) -> Result<(), &'static str> {
        if observed < requested {
            return Err("raster-store.candidate-capacity-underflow");
        }
        let extra = observed.checked_sub(requested).and_then(|value| value.checked_mul(element_bytes)).ok_or("raster-store.candidate-capacity-overflow")?;
        self.candidate_bytes = self.candidate_bytes.checked_add(extra).ok_or("raster-store.candidate-byte-overflow")?;
        if self.candidate_bytes > RASTER_MAXIMUM_NESTED_BYTES || extra > RASTER_OWNED_FIELD_BYTES {
            return Err("raster-store.candidate-observed-capacity");
        }
        Ok(())
    }

    fn fixed_control_backings(&mut self) -> Result<(), &'static str> {
        Self::validate_control_backing_count(RASTER_MAXIMUM_CONTROL_BACKINGS)?;
        if self.source_control_items != 0 || self.source_control_bytes != 0 || self.candidate_control_items != 0 || self.candidate_control_bytes != 0 {
            return Err("raster-store.control-backing-double-reservation");
        }
        self.source_control_items = RASTER_MAXIMUM_CONTROL_BACKINGS;
        self.source_control_bytes = RASTER_MAXIMUM_CONTROL_BYTES;
        self.candidate_control_items = RASTER_MAXIMUM_CONTROL_BACKINGS;
        self.candidate_control_bytes = RASTER_MAXIMUM_CONTROL_BYTES;
        if self.source_control_items > RASTER_MAXIMUM_CONTROL_BACKINGS
            || self.candidate_control_items > RASTER_MAXIMUM_CONTROL_BACKINGS
            || self.source_control_bytes > RASTER_MAXIMUM_CONTROL_BYTES
            || self.candidate_control_bytes > RASTER_MAXIMUM_CONTROL_BYTES
        {
            return Err("raster-store.control-backing-capacity");
        }
        Ok(())
    }

    fn validate_control_backing_count(count: usize) -> Result<(), &'static str> {
        if count > RASTER_MAXIMUM_CONTROL_BACKINGS {
            return Err("raster-store.control-backing-capacity");
        }
        Ok(())
    }
}

struct RasterMapKeyCursor {
    index: usize,
}

impl RasterMapKeyCursor {
    fn new() -> Self {
        Self { index: 0 }
    }

    fn next<'a, T>(&self, values: &'a RasterOwnedMap<T>) -> Result<Option<(&'a String, &'a T)>, &'static str> {
        Ok(values.entry_at(self.index))
    }

    fn advance(&mut self, _key: &str) -> Result<(), &'static str> {
        self.index = self.index.checked_add(1).ok_or("raster-store.map-index-overflow")?;
        Ok(())
    }
}

struct RasterDslValueBoundsAuthority {
    layer_depth: usize,
    depth: usize,
    path: [usize; RASTER_MAXIMUM_NESTED_DEPTH],
    frames: [RasterTraversalFrame; RASTER_MAXIMUM_NESTED_DEPTH],
    terminal: bool,
}

impl RasterDslValueBoundsAuthority {
    fn new(layer_depth: usize) -> Self {
        Self { layer_depth, depth: 0, path: [0; RASTER_MAXIMUM_NESTED_DEPTH], frames: [RasterTraversalFrame::EMPTY; RASTER_MAXIMUM_NESTED_DEPTH], terminal: false }
    }

    fn value_at<'a>(root: &'a dsl::DslValue, path: &[usize]) -> Option<&'a dsl::DslValue> {
        let mut value = root;
        for index in path {
            value = match value {
                dsl::DslValue::Array(values) => values.get(*index)?,
                dsl::DslValue::Object(values) => &values.get(*index)?.1,
                _ => return None,
            };
        }
        Some(value)
    }

    fn step(&mut self, root: &dsl::DslValue, totals: &mut RasterOwnerTotals, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        let _required_frames = raster_retirement_frame_requirement(self.layer_depth, self.depth + 1)?;
        let value = Self::value_at(root, &self.path[..self.depth]).ok_or("raster-store.preflight-value-path")?;
        let frame = self.frames[self.depth];
        if frame.phase == 0 {
            if !raster_reserve_unit(cx) {
                return Ok(false);
            }
            totals.add(1, std::mem::size_of::<dsl::DslValue>(), 1, std::mem::size_of::<dsl::DslValue>())?;
            match value {
                dsl::DslValue::String(value) => totals.string(value)?,
                dsl::DslValue::Array(values) => totals.vector(values)?,
                dsl::DslValue::Object(values) => totals.vector(values)?,
                _ => {}
            }
            self.frames[self.depth].phase = 1;
            return Ok(false);
        }
        let child = frame.child;
        match value {
            dsl::DslValue::Object(values) if frame.phase == 1 && child < values.len() => {
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                totals.string(&values[child].0)?;
                self.frames[self.depth].phase = 2;
                return Ok(false);
            }
            _ if match value {
                dsl::DslValue::Array(values) => child < values.len(),
                dsl::DslValue::Object(values) => frame.phase == 2 && child < values.len(),
                _ => false,
            } =>
            {
                if self.depth + 1 >= RASTER_MAXIMUM_NESTED_DEPTH || !raster_reserve_unit(cx) {
                    if self.depth + 1 >= RASTER_MAXIMUM_NESTED_DEPTH {
                        return Err("raster-store.preflight-value-depth");
                    }
                    return Ok(false);
                }
                self.path[self.depth] = child;
                self.frames[self.depth].child += 1;
                self.frames[self.depth].phase = 1;
                self.depth += 1;
                self.frames[self.depth] = RasterTraversalFrame::EMPTY;
                return Ok(false);
            }
            _ => {}
        }
        if !raster_reserve_unit(cx) {
            return Ok(false);
        }
        if self.depth == 0 {
            self.terminal = true;
        } else {
            self.depth -= 1;
        }
        Ok(self.terminal)
    }
}

struct RasterLayerBoundsAuthority {
    depth: usize,
    path: [usize; RASTER_MAXIMUM_NESTED_DEPTH],
    frames: [RasterTraversalFrame; RASTER_MAXIMUM_NESTED_DEPTH],
    parameter_key: RasterMapKeyCursor,
    parameter_value: Option<RasterDslValueBoundsAuthority>,
    terminal: bool,
}

impl RasterLayerBoundsAuthority {
    fn new() -> Self {
        Self { depth: 0, path: [0; RASTER_MAXIMUM_NESTED_DEPTH], frames: [RasterTraversalFrame::EMPTY; RASTER_MAXIMUM_NESTED_DEPTH], parameter_key: RasterMapKeyCursor::new(), parameter_value: None, terminal: false }
    }

    fn layer_at<'a>(root: &'a RasterLayerNode, path: &[usize]) -> Option<&'a RasterLayerNode> {
        let mut value = root;
        for index in path {
            let RasterLayerNode::Group { children, .. } = value else { return None };
            value = children.get(*index)?;
        }
        Some(value)
    }

    fn strings(layer: &RasterLayerNode) -> [&String; 3] {
        match layer {
            RasterLayerNode::Pixel { id, name, blend_mode, .. } | RasterLayerNode::Group { id, name, blend_mode, .. } | RasterLayerNode::Adjustment { id, name, blend_mode, .. } => [id, name, blend_mode],
        }
    }

    fn step(&mut self, root: &RasterLayerNode, totals: &mut RasterOwnerTotals, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        let layer = Self::layer_at(root, &self.path[..self.depth]).ok_or("raster-store.preflight-layer-path")?;
        let frame = self.frames[self.depth];
        match frame.phase {
            0 => {
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                totals.add(1, std::mem::size_of::<RasterLayerNode>(), 1, std::mem::size_of::<RasterLayerNode>())?;
                self.frames[self.depth].phase = 1;
            }
            1..=3 => {
                let value = Self::strings(layer)[(frame.phase - 1) as usize];
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                totals.string(value)?;
                self.frames[self.depth].phase += 1;
            }
            4 => {
                let value = match layer {
                    RasterLayerNode::Pixel { image_key, .. } => image_key.as_ref(),
                    RasterLayerNode::Adjustment { adjustment_kind, .. } => Some(adjustment_kind),
                    RasterLayerNode::Group { .. } => None,
                };
                if let Some(value) = value {
                    if !raster_reserve_unit(cx) {
                        return Ok(false);
                    }
                    totals.string(value)?;
                } else if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                self.frames[self.depth].phase = 5;
            }
            5 => {
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                match layer {
                    RasterLayerNode::Group { children, .. } => totals.layer_vector(children)?,
                    RasterLayerNode::Adjustment { params, .. } => totals.map(params, 0)?,
                    RasterLayerNode::Pixel { .. } => {}
                }
                self.frames[self.depth].phase = 6;
            }
            6 => {
                if let RasterLayerNode::Adjustment { params, .. } = layer {
                    if let Some(value) = self.parameter_value.as_mut() {
                        let (_, source) = self.parameter_key.next(params)?.ok_or("raster-store.preflight-parameter-source")?;
                        if value.step(source, totals, cx)? {
                            let (key, _) = self.parameter_key.next(params)?.ok_or("raster-store.preflight-parameter-key")?;
                            if !raster_reserve_unit(cx) {
                                return Ok(false);
                            }
                            self.parameter_value = None;
                            self.parameter_key.advance(key)?;
                        }
                        return Ok(false);
                    }
                    if let Some((key, _)) = self.parameter_key.next(params)? {
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        totals.string(key)?;
                        self.parameter_value = Some(RasterDslValueBoundsAuthority::new(self.depth + 1));
                        return Ok(false);
                    }
                    self.parameter_key = RasterMapKeyCursor::new();
                }
                self.frames[self.depth].phase = 7;
            }
            7 => {
                if let RasterLayerNode::Group { children, .. } = layer {
                    if frame.child < children.len() {
                        if self.depth + 1 >= RASTER_MAXIMUM_NESTED_DEPTH || !raster_reserve_unit(cx) {
                            if self.depth + 1 >= RASTER_MAXIMUM_NESTED_DEPTH {
                                return Err("raster-store.preflight-layer-depth");
                            }
                            return Ok(false);
                        }
                        self.path[self.depth] = frame.child;
                        self.frames[self.depth].child += 1;
                        self.depth += 1;
                        self.frames[self.depth] = RasterTraversalFrame::EMPTY;
                        return Ok(false);
                    }
                }
                self.frames[self.depth].phase = 8;
            }
            _ => {
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                if self.depth == 0 {
                    self.terminal = true;
                } else {
                    self.depth -= 1;
                }
            }
        }
        Ok(self.terminal)
    }
}

struct RasterSnapshotBoundsAuthority {
    totals: RasterOwnerTotals,
    layer: Option<RasterLayerBoundsAuthority>,
    asset_key: RasterMapKeyCursor,
    phase: u8,
    index: usize,
    asset_field: u8,
    terminal: bool,
}

fn raster_exact_string_from_parts(parts: &[&[u8]]) -> Result<String, &'static str> {
    let length = parts.iter().try_fold(0usize, |length, part| length.checked_add(part.len())).ok_or("raster-store.clone-string-length-overflow")?;
    if length > RASTER_OWNED_FIELD_BYTES {
        return Err("raster-store.clone-string-capacity");
    }
    let mut bytes = Box::<[u8]>::new_uninit_slice(length);
    let mut offset = 0;
    for part in parts {
        for byte in *part {
            bytes[offset].write(*byte);
            offset += 1;
        }
    }
    let bytes = unsafe { bytes.assume_init() };
    String::from_utf8(Vec::from(bytes)).map_err(|_| "raster-store.clone-string-utf8")
}

fn raster_clone_owned_string(source: &String) -> Result<String, &'static str> {
    if source.capacity() > RASTER_OWNED_FIELD_BYTES {
        return Err("raster-store.clone-string-capacity");
    }
    raster_exact_string_from_parts(&[source.as_bytes()])
}

fn raster_asset_child_id(hash: u64) -> Result<String, &'static str> {
    const PREFIX: &[u8] = b"raster-asset-";
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut value = [0u8; 29];
    value[..PREFIX.len()].copy_from_slice(PREFIX);
    for index in 0..16 {
        let shift = (15 - index) * 4;
        value[PREFIX.len() + index] = HEX[((hash >> shift) & 0xf) as usize];
    }
    raster_exact_string_from_parts(&[&value])
}

struct RasterDslValueCloneAuthority {
    value: std::mem::ManuallyDrop<Option<dsl::DslValue>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    depth: usize,
    path: [usize; RASTER_MAXIMUM_NESTED_DEPTH],
    frames: [RasterTraversalFrame; RASTER_MAXIMUM_NESTED_DEPTH],
    pending_key: std::mem::ManuallyDrop<Option<String>>,
    root_capacity_observed: bool,
    terminal: bool,
}

impl RasterDslValueCloneAuthority {
    fn skeleton(source: &dsl::DslValue) -> dsl::DslValue {
        match source {
            dsl::DslValue::Null => dsl::DslValue::Null,
            dsl::DslValue::Bool(value) => dsl::DslValue::Bool(*value),
            dsl::DslValue::Number(value) => dsl::DslValue::Number(*value),
            dsl::DslValue::String(_) => dsl::DslValue::String(String::new()),
            dsl::DslValue::Array(values) => dsl::DslValue::Array(Vec::with_capacity(values.capacity())),
            dsl::DslValue::Object(values) => dsl::DslValue::Object(Vec::with_capacity(values.capacity())),
        }
    }

    fn new(source: &dsl::DslValue) -> Self {
        Self {
            value: std::mem::ManuallyDrop::new(Some(Self::skeleton(source))),
            retirement: std::mem::ManuallyDrop::new(None),
            depth: 0,
            path: [0; RASTER_MAXIMUM_NESTED_DEPTH],
            frames: [RasterTraversalFrame::EMPTY; RASTER_MAXIMUM_NESTED_DEPTH],
            pending_key: std::mem::ManuallyDrop::new(None),
            root_capacity_observed: false,
            terminal: false,
        }
    }

    fn target_at_mut<'a>(root: &'a mut dsl::DslValue, path: &[usize]) -> Option<&'a mut dsl::DslValue> {
        let Some((head, tail)) = path.split_first() else { return Some(root) };
        match root {
            dsl::DslValue::Array(values) => Self::target_at_mut(values.get_mut(*head)?, tail),
            dsl::DslValue::Object(values) => Self::target_at_mut(&mut values.get_mut(*head)?.1, tail),
            _ => None,
        }
    }

    fn observe_container_capacity(source: &dsl::DslValue, target: &dsl::DslValue, totals: &mut RasterOwnerTotals) -> Result<(), &'static str> {
        match (source, target) {
            (dsl::DslValue::Array(source), dsl::DslValue::Array(target)) => totals.observe_candidate_capacity(source.capacity(), target.capacity(), std::mem::size_of::<dsl::DslValue>()),
            (dsl::DslValue::Object(source), dsl::DslValue::Object(target)) => totals.observe_candidate_capacity(source.capacity(), target.capacity(), std::mem::size_of::<(String, dsl::DslValue)>()),
            _ => Ok(()),
        }
    }

    fn step(&mut self, source_root: &dsl::DslValue, totals: &mut RasterOwnerTotals, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        if !self.root_capacity_observed {
            if !raster_reserve_unit(cx) {
                return Ok(false);
            }
            Self::observe_container_capacity(source_root, self.value.as_ref().ok_or("raster-store.clone-value-observed-target")?, totals)?;
            self.root_capacity_observed = true;
            return Ok(false);
        }
        let source = RasterDslValueBoundsAuthority::value_at(source_root, &self.path[..self.depth]).ok_or("raster-store.clone-value-source")?;
        let target = Self::target_at_mut(self.value.as_mut().ok_or("raster-store.clone-value-target")?, &self.path[..self.depth]).ok_or("raster-store.clone-value-target-path")?;
        let frame = self.frames[self.depth];
        if frame.phase == 0 {
            match (source, target) {
                (dsl::DslValue::String(source), dsl::DslValue::String(target)) => {
                    if !raster_reserve_unit(cx) {
                        return Ok(false);
                    }
                    *target = raster_clone_owned_string(source)?;
                }
                _ if !raster_reserve_unit(cx) => return Ok(false),
                _ => {}
            }
            self.frames[self.depth].phase = 1;
            return Ok(false);
        }
        match (source, target) {
            (dsl::DslValue::Array(source), dsl::DslValue::Array(target)) if frame.child < source.len() => {
                if self.depth + 1 >= RASTER_MAXIMUM_NESTED_DEPTH {
                    return Err("raster-store.clone-value-depth");
                }
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                target.push(Self::skeleton(&source[frame.child]));
                Self::observe_container_capacity(&source[frame.child], target.last().ok_or("raster-store.clone-value-child-target")?, totals)?;
                self.path[self.depth] = frame.child;
                self.frames[self.depth].child += 1;
                self.depth += 1;
                self.frames[self.depth] = RasterTraversalFrame::EMPTY;
                return Ok(false);
            }
            (dsl::DslValue::Object(source), dsl::DslValue::Object(target)) if frame.child < source.len() => {
                if frame.phase == 1 {
                    if !raster_reserve_unit(cx) {
                        return Ok(false);
                    }
                    *self.pending_key = Some(raster_clone_owned_string(&source[frame.child].0)?);
                    self.frames[self.depth].phase = 2;
                    return Ok(false);
                }
                if self.depth + 1 >= RASTER_MAXIMUM_NESTED_DEPTH {
                    return Err("raster-store.clone-value-depth");
                }
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                target.push((self.pending_key.take().ok_or("raster-store.clone-value-key")?, Self::skeleton(&source[frame.child].1)));
                Self::observe_container_capacity(&source[frame.child].1, &target.last().ok_or("raster-store.clone-value-object-target")?.1, totals)?;
                self.path[self.depth] = frame.child;
                self.frames[self.depth].child += 1;
                self.frames[self.depth].phase = 1;
                self.depth += 1;
                self.frames[self.depth] = RasterTraversalFrame::EMPTY;
                return Ok(false);
            }
            _ => {}
        }
        if !raster_reserve_unit(cx) {
            return Ok(false);
        }
        if self.depth == 0 {
            self.terminal = true;
        } else {
            self.depth -= 1;
        }
        Ok(self.terminal)
    }

    fn take(&mut self) -> Option<dsl::DslValue> {
        self.terminal.then(|| self.value.take()).flatten()
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(key) = self.pending_key.take() {
            *self.retirement = Some(Box::new(RasterOwnedRetirement::new(RasterRetirementOwner::String(key))));
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.retirement.is_none() {
            if let Some(value) = self.value.take() {
                *self.retirement = Some(Box::new(RasterOwnedRetirement::new(RasterRetirementOwner::Value(value))));
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            self.terminal = true;
            return Ok(store::SnapshotRetirementStep::Complete);
        }
        let retirement = self.retirement.as_mut().expect("Raster value clone retirement remains retained");
        match retirement.close_step(1, maximum_bytes)? {
            store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                if maximum_bytes < RASTER_CONTROL_BACKING_BYTES {
                    return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                }
                drop(self.retirement.take());
                Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: RASTER_CONTROL_BACKING_BYTES })
            }
            store::SnapshotRetirementStep::Complete => Err("Raster value clone retirement false terminal".into()),
            step => Ok(step),
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.value.is_none() && self.retirement.is_none() && self.pending_key.is_none()
    }
}

impl Drop for RasterDslValueCloneAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Raster value clone reached Drop before exact handoff or retirement");
    }
}

struct RasterLayerCloneAuthority {
    value: std::mem::ManuallyDrop<Option<RasterLayerNode>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    bounds: RasterLayerBoundsAuthority,
    totals: RasterOwnerTotals,
    admitted: bool,
    root_capacity_observed: bool,
    depth: usize,
    path: [usize; RASTER_MAXIMUM_NESTED_DEPTH],
    frames: [RasterTraversalFrame; RASTER_MAXIMUM_NESTED_DEPTH],
    parameter_key: RasterMapKeyCursor,
    pending_parameter_key: std::mem::ManuallyDrop<Option<String>>,
    parameter_value: std::mem::ManuallyDrop<Option<Box<RasterDslValueCloneAuthority>>>,
    terminal: bool,
}

impl RasterLayerCloneAuthority {
    fn mask(source: &Option<crate::artifacts::raster::RasterLayerMask>) -> Option<crate::artifacts::raster::RasterLayerMask> {
        source.as_ref().map(|value| crate::artifacts::raster::RasterLayerMask { enabled: value.enabled, linked: value.linked, invert: value.invert, width: value.width, height: value.height })
    }

    fn skeleton(source: &RasterLayerNode) -> RasterLayerNode {
        match source {
            RasterLayerNode::Pixel { visible, opacity, transform, mask, width, height, .. } => RasterLayerNode::Pixel {
                id: String::new(),
                name: String::new(),
                visible: *visible,
                opacity: *opacity,
                blend_mode: String::new(),
                transform: crate::artifacts::raster::RasterTransform { x: transform.x, y: transform.y, scale_x: transform.scale_x, scale_y: transform.scale_y, rotation: transform.rotation },
                mask: Self::mask(mask),
                width: *width,
                height: *height,
                image_key: None,
            },
            RasterLayerNode::Group { visible, opacity, transform, mask, children, .. } => RasterLayerNode::Group {
                id: String::new(),
                name: String::new(),
                visible: *visible,
                opacity: *opacity,
                blend_mode: String::new(),
                transform: crate::artifacts::raster::RasterTransform { x: transform.x, y: transform.y, scale_x: transform.scale_x, scale_y: transform.scale_y, rotation: transform.rotation },
                mask: Self::mask(mask),
                children: Vec::with_capacity(children.capacity().saturating_add(1)),
            },
            RasterLayerNode::Adjustment { visible, opacity, transform, .. } => RasterLayerNode::Adjustment {
                id: String::new(),
                name: String::new(),
                visible: *visible,
                opacity: *opacity,
                blend_mode: String::new(),
                transform: crate::artifacts::raster::RasterTransform { x: transform.x, y: transform.y, scale_x: transform.scale_x, scale_y: transform.scale_y, rotation: transform.rotation },
                adjustment_kind: String::new(),
                params: RasterOwnedMap::new(),
            },
        }
    }

    fn new(_source: &RasterLayerNode) -> Self {
        Self {
            value: std::mem::ManuallyDrop::new(None),
            retirement: std::mem::ManuallyDrop::new(None),
            bounds: RasterLayerBoundsAuthority::new(),
            totals: RasterOwnerTotals::new(),
            admitted: false,
            root_capacity_observed: false,
            depth: 0,
            path: [0; RASTER_MAXIMUM_NESTED_DEPTH],
            frames: [RasterTraversalFrame::EMPTY; RASTER_MAXIMUM_NESTED_DEPTH],
            parameter_key: RasterMapKeyCursor::new(),
            pending_parameter_key: std::mem::ManuallyDrop::new(None),
            parameter_value: std::mem::ManuallyDrop::new(None),
            terminal: false,
        }
    }

    fn target_at_mut<'a>(root: &'a mut RasterLayerNode, path: &[usize]) -> Option<&'a mut RasterLayerNode> {
        let Some((head, tail)) = path.split_first() else { return Some(root) };
        let RasterLayerNode::Group { children, .. } = root else { return None };
        Self::target_at_mut(children.get_mut(*head)?, tail)
    }

    fn strings<'a>(source: &'a RasterLayerNode, target: &'a mut RasterLayerNode) -> [(&'a String, &'a mut String); 3] {
        match (source, target) {
            (RasterLayerNode::Pixel { id: source_id, name: source_name, blend_mode: source_blend, .. }, RasterLayerNode::Pixel { id: target_id, name: target_name, blend_mode: target_blend, .. })
            | (RasterLayerNode::Group { id: source_id, name: source_name, blend_mode: source_blend, .. }, RasterLayerNode::Group { id: target_id, name: target_name, blend_mode: target_blend, .. })
            | (RasterLayerNode::Adjustment { id: source_id, name: source_name, blend_mode: source_blend, .. }, RasterLayerNode::Adjustment { id: target_id, name: target_name, blend_mode: target_blend, .. }) => {
                [(source_id, target_id), (source_name, target_name), (source_blend, target_blend)]
            }
            _ => unreachable!("Raster layer clone source and target variants remain exact"),
        }
    }

    fn step(&mut self, source_root: &RasterLayerNode, digest: &mut store::ArtifactStoreInitializationDigest, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        if !self.admitted {
            if self.bounds.step(source_root, &mut self.totals, cx)? {
                self.admitted = true;
            }
            return Ok(false);
        }
        if self.value.is_none() {
            if !raster_reserve_unit(cx) {
                return Ok(false);
            }
            *self.value = Some(Self::skeleton(source_root));
            return Ok(false);
        }
        if !self.root_capacity_observed {
            if !raster_reserve_unit(cx) {
                return Ok(false);
            }
            if let (RasterLayerNode::Group { children: source, .. }, RasterLayerNode::Group { children: target, .. }) = (source_root, self.value.as_ref().ok_or("raster-store.clone-layer-observed-target")?) {
                self.totals.observe_candidate_capacity(source.capacity().saturating_add(1), target.capacity(), std::mem::size_of::<RasterLayerNode>())?;
            }
            self.root_capacity_observed = true;
            return Ok(false);
        }
        let source = RasterLayerBoundsAuthority::layer_at(source_root, &self.path[..self.depth]).ok_or("raster-store.clone-layer-source")?;
        let target = Self::target_at_mut(self.value.as_mut().ok_or("raster-store.clone-layer-target")?, &self.path[..self.depth]).ok_or("raster-store.clone-layer-target-path")?;
        let frame = self.frames[self.depth];
        match frame.phase {
            0..=2 => {
                let (source, target) = &mut Self::strings(source, target)[frame.phase as usize];
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                **target = raster_clone_owned_string(source)?;
                digest.observe(source.as_bytes());
                self.frames[self.depth].phase += 1;
            }
            3 => {
                match (source, target) {
                    (RasterLayerNode::Pixel { image_key: source, .. }, RasterLayerNode::Pixel { image_key: target, .. }) => {
                        if let Some(source) = source {
                            if !raster_reserve_unit(cx) {
                                return Ok(false);
                            }
                            *target = Some(raster_clone_owned_string(source)?);
                            digest.observe(source.as_bytes());
                        } else if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                    }
                    (RasterLayerNode::Adjustment { adjustment_kind: source, .. }, RasterLayerNode::Adjustment { adjustment_kind: target, .. }) => {
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        *target = raster_clone_owned_string(source)?;
                        digest.observe(source.as_bytes());
                    }
                    _ if !raster_reserve_unit(cx) => return Ok(false),
                    _ => {}
                }
                self.frames[self.depth].phase = 4;
            }
            4 => {
                if let (RasterLayerNode::Adjustment { params: source, .. }, RasterLayerNode::Adjustment { params: target, .. }) = (source, target) {
                    if let Some(authority) = self.parameter_value.as_mut() {
                        let (_, source_value) = self.parameter_key.next(source)?.ok_or("raster-store.clone-parameter-source")?;
                        if authority.step(source_value, &mut self.totals, cx)? {
                            let (source_key, _) = self.parameter_key.next(source)?.ok_or("raster-store.clone-parameter-advance")?;
                            let pending_key = self.pending_parameter_key.as_ref().ok_or("raster-store.clone-parameter-key")?;
                            if target.page_required_for_insert(pending_key) {
                                let page_bytes = RasterOwnedMap::<dsl::DslValue>::conservative_page_credit_bytes();
                                if !raster_reserve_unit(cx) {
                                    return Ok(false);
                                }
                                target.admit_one_page()?;
                                return Ok(false);
                            }
                            if !raster_reserve_unit(cx) {
                                return Ok(false);
                            }
                            if !raster_reserve_unit(cx) {
                                return Ok(false);
                            }
                            let key = self.pending_parameter_key.take().ok_or("raster-store.clone-parameter-key")?;
                            let value = authority.take().ok_or("raster-store.clone-parameter-value")?;
                            drop(self.parameter_value.take());
                            match target.insert_pre_admitted(key, value) {
                                Ok(RasterOwnedMapInsert::Inserted) => {}
                                Ok(RasterOwnedMapInsert::Replaced(mut previous)) => {
                                    let (previous_key, previous) = previous.take();
                                    *self.retirement = Some(Box::new(RasterOwnedRetirement::new(RasterRetirementOwner::ValueEntry { key: previous_key, value: Some(previous) })));
                                    return Err("raster-store.clone-duplicate-parameter");
                                }
                                Err(rejected) => {
                                    *self.retirement = Some(Box::new(RasterOwnedRetirement::new(RasterRetirementOwner::ValueEntry { key: rejected.key, value: Some(rejected.value) })));
                                    return Err(rejected.reason);
                                }
                            }
                            self.parameter_key.advance(source_key)?;
                        }
                        return Ok(false);
                    }
                    if let Some((key, value)) = self.parameter_key.next(source)? {
                        if self.pending_parameter_key.is_none() {
                            if !raster_reserve_unit(cx) {
                                return Ok(false);
                            }
                            *self.pending_parameter_key = Some(raster_clone_owned_string(key)?);
                            return Ok(false);
                        }
                        if std::mem::size_of::<RasterDslValueCloneAuthority>() > RASTER_CONTROL_BACKING_BYTES {
                            return Err("raster-store.clone-parameter-control-capacity");
                        }
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        *self.parameter_value = Some(Box::new(RasterDslValueCloneAuthority::new(value)));
                        return Ok(false);
                    }
                    self.parameter_key = RasterMapKeyCursor::new();
                }
                self.frames[self.depth].phase = 5;
            }
            5 => {
                if let (RasterLayerNode::Group { children: source, .. }, RasterLayerNode::Group { children: target, .. }) = (source, target) {
                    if frame.child < source.len() {
                        if self.depth + 1 >= RASTER_MAXIMUM_NESTED_DEPTH {
                            return Err("raster-store.clone-layer-depth");
                        }
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        target.push(Self::skeleton(&source[frame.child]));
                        if let (RasterLayerNode::Group { children: source_child, .. }, RasterLayerNode::Group { children: target_child, .. }) = (&source[frame.child], target.last().ok_or("raster-store.clone-layer-child-target")?) {
                            self.totals.observe_candidate_capacity(source_child.capacity().saturating_add(1), target_child.capacity(), std::mem::size_of::<RasterLayerNode>())?;
                        }
                        self.path[self.depth] = frame.child;
                        self.frames[self.depth].child += 1;
                        self.depth += 1;
                        self.frames[self.depth] = RasterTraversalFrame::EMPTY;
                        return Ok(false);
                    }
                }
                self.frames[self.depth].phase = 6;
            }
            _ => {
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                if self.depth == 0 {
                    self.terminal = true;
                } else {
                    self.depth -= 1;
                }
            }
        }
        Ok(self.terminal)
    }

    fn take(&mut self) -> Option<RasterLayerNode> {
        self.terminal.then(|| self.value.take()).flatten()
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(authority) = self.parameter_value.as_mut() {
            return match authority.close_step(1, maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if authority.terminal_is_empty() => {
                    if maximum_bytes < RASTER_CONTROL_BACKING_BYTES {
                        return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                    }
                    drop(self.parameter_value.take());
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: RASTER_CONTROL_BACKING_BYTES })
                }
                store::SnapshotRetirementStep::Complete => Err("Raster parameter clone false terminal".into()),
                step => Ok(step),
            };
        }
        if let Some(key) = self.pending_parameter_key.take() {
            *self.retirement = Some(Box::new(RasterOwnedRetirement::new(RasterRetirementOwner::String(key))));
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.retirement.is_none() {
            if let Some(value) = self.value.take() {
                *self.retirement = Some(Box::new(RasterOwnedRetirement::new(RasterRetirementOwner::Layer(value))));
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            self.terminal = true;
            return Ok(store::SnapshotRetirementStep::Complete);
        }
        let retirement = self.retirement.as_mut().expect("Raster layer clone retirement remains retained");
        match retirement.close_step(1, maximum_bytes)? {
            store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                if maximum_bytes < RASTER_CONTROL_BACKING_BYTES {
                    return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                }
                drop(self.retirement.take());
                Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: RASTER_CONTROL_BACKING_BYTES })
            }
            store::SnapshotRetirementStep::Complete => Err("Raster layer clone retirement false terminal".into()),
            step => Ok(step),
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.value.is_none() && self.retirement.is_none() && self.pending_parameter_key.is_none() && self.parameter_value.is_none()
    }
}

impl Drop for RasterLayerCloneAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Raster layer clone reached Drop before exact handoff or retirement");
    }
}

impl RasterSnapshotBoundsAuthority {
    fn new() -> Self {
        Self { totals: RasterOwnerTotals::new(), layer: None, asset_key: RasterMapKeyCursor::new(), phase: 0, index: 0, asset_field: 0, terminal: false }
    }

    fn step(&mut self, source: &RasterSnapshot, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        match self.phase {
            0 => {
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                self.totals.add(1, std::mem::size_of::<RasterSnapshot>(), 1, std::mem::size_of::<RasterSnapshot>())?;
                self.totals.fixed_control_backings()?;
                self.phase = 1;
            }
            1 => {
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                self.totals.string(&source.schema)?;
                self.phase = 2;
            }
            2 => {
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                self.totals.string(&source.id)?;
                self.phase = 3;
            }
            3 => {
                if let Some(title) = &source.title {
                    if !raster_reserve_unit(cx) {
                        return Ok(false);
                    }
                    self.totals.string(title)?;
                } else if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                self.phase = 4;
            }
            4 => {
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                self.totals.layer_vector(&source.layers)?;
                self.phase = 5;
            }
            5 => {
                if let Some(layer) = self.layer.as_mut() {
                    if layer.step(source.layers.get(self.index).ok_or("raster-store.preflight-root-layer")?, &mut self.totals, cx)? {
                        self.layer = None;
                        self.index += 1;
                    }
                    return Ok(false);
                }
                if source.layers.get(self.index).is_some() {
                    if !raster_reserve_unit(cx) {
                        return Ok(false);
                    }
                    self.layer = Some(RasterLayerBoundsAuthority::new());
                    return Ok(false);
                }
                self.phase = 6;
                self.index = 0;
            }
            6 => {
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                self.totals.map(&source.assets, 1)?;
                self.phase = 7;
            }
            7 => {
                let Some((key, child)) = self.asset_key.next(&source.assets)? else {
                    self.terminal = true;
                    return Ok(true);
                };
                let value = match self.asset_field {
                    0 => key,
                    1 => &child.child_id,
                    2 => &child.target.artifact_id,
                    3 => &child.target.dialect.artifact_kind,
                    4 => &child.target.dialect.standard,
                    5 => &child.target.dialect.subset,
                    _ => {
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        self.asset_key.advance(key)?;
                        self.asset_field = 0;
                        return Ok(false);
                    }
                };
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                self.totals.string(value)?;
                self.asset_field += 1;
            }
            _ => self.terminal = true,
        }
        Ok(self.terminal)
    }
}

struct RasterSnapshotCloneAuthority {
    value: std::mem::ManuallyDrop<Option<RasterSnapshot>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    layer: std::mem::ManuallyDrop<Option<Box<RasterLayerCloneAuthority>>>,
    pending_asset: std::mem::ManuallyDrop<Option<(String, RasterAssetChild)>>,
    bounds: RasterSnapshotBoundsAuthority,
    asset_key: RasterMapKeyCursor,
    phase: u8,
    index: usize,
    asset_field: u8,
    terminal: bool,
}

impl RasterSnapshotCloneAuthority {
    fn new() -> Self {
        let value = RasterSnapshot { schema: String::new(), id: String::new(), title: None, layers: Vec::new(), assets: RasterOwnedMap::new() };
        Self {
            value: std::mem::ManuallyDrop::new(Some(value)),
            retirement: std::mem::ManuallyDrop::new(None),
            layer: std::mem::ManuallyDrop::new(None),
            pending_asset: std::mem::ManuallyDrop::new(None),
            bounds: RasterSnapshotBoundsAuthority::new(),
            asset_key: RasterMapKeyCursor::new(),
            phase: 0,
            index: 0,
            asset_field: 0,
            terminal: false,
        }
    }

    fn step(&mut self, source: &RasterSnapshot, digest: &mut store::ArtifactStoreInitializationDigest, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.phase == 0 {
            if self.bounds.step(source, cx)? {
                self.phase = 1;
            }
            return Ok(false);
        }
        let target = self.value.as_mut().ok_or("raster-store.initializer-clone-target")?;
        let observed: &[u8] = match self.phase {
            1 => {
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                target.schema = raster_clone_owned_string(&source.schema)?;
                source.schema.as_bytes()
            }
            2 => {
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                target.id = raster_clone_owned_string(&source.id)?;
                source.id.as_bytes()
            }
            3 => {
                if let Some(title) = source.title.as_ref() {
                    if !raster_reserve_unit(cx) {
                        return Ok(false);
                    }
                    target.title = Some(raster_clone_owned_string(title)?);
                    title.as_bytes()
                } else {
                    if !raster_reserve_unit(cx) {
                        return Ok(false);
                    }
                    &[]
                }
            }
            4 => {
                if self.index == 0 && target.layers.capacity() == 0 {
                    if !raster_reserve_unit(cx) {
                        return Ok(false);
                    }
                    target.layers.try_reserve_exact(source.layers.capacity().saturating_add(1)).map_err(|_| "raster-store.initializer-layer-admission")?;
                    self.bounds.totals.observe_candidate_capacity(source.layers.capacity().saturating_add(1), target.layers.capacity(), std::mem::size_of::<RasterLayerNode>())?;
                    return Ok(false);
                }
                if let Some(layer) = self.layer.as_mut() {
                    if layer.step(source.layers.get(self.index).ok_or("raster-store.initializer-layer-source")?, digest, cx)? {
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        target.layers.push(layer.take().ok_or("raster-store.initializer-layer-handoff")?);
                        drop(self.layer.take());
                        self.index += 1;
                    }
                    return Ok(false);
                }
                if let Some(layer) = source.layers.get(self.index) {
                    if std::mem::size_of::<RasterLayerCloneAuthority>() > RASTER_CONTROL_BACKING_BYTES {
                        return Err("raster-store.initializer-layer-control-capacity");
                    }
                    if !raster_reserve_unit(cx) {
                        return Ok(false);
                    }
                    *self.layer = Some(Box::new(RasterLayerCloneAuthority::new(layer)));
                    return Ok(false);
                }
                self.index = 0;
                &[]
            }
            5 => {
                if self.pending_asset.is_none() {
                    let Some((key, _)) = self.asset_key.next(&source.assets)? else {
                        self.phase = 6;
                        return Ok(false);
                    };
                    if !raster_reserve_unit(cx) {
                        return Ok(false);
                    }
                    let key = raster_clone_owned_string(key)?;
                    let child =
                        store::ArtifactChild::new(String::new(), store::os_io::ArtifactRef { artifact_id: String::new(), dialect: store::os_io::ArtifactDialect { artifact_kind: String::new(), standard: String::new(), subset: String::new() } });
                    *self.pending_asset = Some((key, child));
                    self.asset_field = 0;
                    return Ok(false);
                }
                if self.asset_field >= 5 {
                    let (key, _) = self.pending_asset.as_ref().expect("Raster pending asset remains retained");
                    if target.assets.page_required_for_insert(key) {
                        let page_bytes = RasterOwnedMap::<RasterAssetChild>::conservative_page_credit_bytes();
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        target.assets.admit_one_page()?;
                        return Ok(false);
                    }
                    if !raster_reserve_unit(cx) {
                        return Ok(false);
                    }
                    let (key, child) = self.pending_asset.take().expect("Raster pending asset handoff remains exact");
                    match target.assets.insert_pre_admitted(key, child) {
                        Ok(RasterOwnedMapInsert::Inserted) => {}
                        Ok(RasterOwnedMapInsert::Replaced(mut previous)) => {
                            let (previous_key, previous) = previous.take();
                            *self.retirement = Some(Box::new(RasterOwnedRetirement::new(RasterRetirementOwner::AssetEntry { key: previous_key, child: Some(previous) })));
                            return Err("raster-store.initializer-duplicate-asset");
                        }
                        Err(rejected) => {
                            *self.retirement = Some(Box::new(RasterOwnedRetirement::new(RasterRetirementOwner::AssetEntry { key: rejected.key, child: Some(rejected.value) })));
                            return Err(rejected.reason);
                        }
                    }
                    self.asset_key.advance("")?;
                    self.asset_field = 0;
                    return Ok(false);
                }
                let (key, pending) = self.pending_asset.as_mut().expect("Raster pending asset remains exact");
                let source_child = source.assets.get(key).ok_or("raster-store.initializer-asset-source")?;
                let source_value = match self.asset_field {
                    0 => &source_child.child_id,
                    1 => &source_child.target.artifact_id,
                    2 => &source_child.target.dialect.artifact_kind,
                    3 => &source_child.target.dialect.standard,
                    4 => &source_child.target.dialect.subset,
                    _ => unreachable!("Raster asset field cursor is exact"),
                };
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                let target_value = match self.asset_field {
                    0 => &mut pending.child_id,
                    1 => &mut pending.target.artifact_id,
                    2 => &mut pending.target.dialect.artifact_kind,
                    3 => &mut pending.target.dialect.standard,
                    4 => &mut pending.target.dialect.subset,
                    _ => unreachable!("Raster asset field cursor is exact"),
                };
                *target_value = raster_clone_owned_string(source_value)?;
                digest.observe(source_value.as_bytes());
                self.asset_field += 1;
                return Ok(false);
            }
            _ => {
                self.terminal = true;
                return Ok(true);
            }
        };
        digest.observe(observed);
        self.phase += 1;
        Ok(false)
    }

    fn take_value(&mut self) -> Option<RasterSnapshot> {
        if !self.terminal {
            return None;
        }
        self.value.take()
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(layer) = self.layer.as_mut() {
            return match layer.close_step(1, maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if layer.terminal_is_empty() => {
                    if maximum_bytes < RASTER_CONTROL_BACKING_BYTES {
                        return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                    }
                    drop(self.layer.take());
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: RASTER_CONTROL_BACKING_BYTES })
                }
                store::SnapshotRetirementStep::Complete => Err("Raster active layer clone reported false terminal".into()),
                step => Ok(step),
            };
        }
        if let Some((key, child)) = self.pending_asset.take() {
            *self.retirement = Some(Box::new(RasterOwnedRetirement::new(RasterRetirementOwner::AssetEntry { key, child: Some(child) })));
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.retirement.is_none() {
            if let Some(value) = self.value.take() {
                *self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&RasterSnapshotRetirementFactory, value));
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            self.terminal = true;
            return Ok(store::SnapshotRetirementStep::Complete);
        }
        let retirement = self.retirement.as_mut().expect("Raster clone retirement remains exact");
        match retirement.close_step(1, maximum_bytes)? {
            store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                if maximum_bytes < RASTER_CONTROL_BACKING_BYTES {
                    return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                }
                drop(self.retirement.take());
                Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: RASTER_CONTROL_BACKING_BYTES })
            }
            store::SnapshotRetirementStep::Complete => Err("Raster clone retirement reported false terminal".into()),
            step => Ok(step),
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal && self.value.is_none() && self.retirement.is_none() && self.layer.is_none() && self.pending_asset.is_none()
    }
}

impl Drop for RasterSnapshotCloneAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Raster snapshot clone reached Drop before exact handoff or cursor retirement");
    }
}

struct RasterMutationDigestAuthority {
    layer: std::mem::ManuallyDrop<Option<Box<RasterLayerCloneAuthority>>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    retirement_terminal: bool,
    phase: u8,
    offset: usize,
    terminal: bool,
}

impl RasterMutationDigestAuthority {
    fn new() -> Self {
        Self { layer: std::mem::ManuallyDrop::new(None), retirement: std::mem::ManuallyDrop::new(None), retirement_terminal: false, phase: 0, offset: 0, terminal: false }
    }

    fn variant(operation: &RasterMutation) -> u8 {
        match operation {
            RasterMutation::CreateLayer(_) => 1,
            RasterMutation::DeleteLayer(_) => 2,
            RasterMutation::ReorderLayers(_) => 3,
            RasterMutation::RenameLayer(_) => 4,
            RasterMutation::ChangeLayerVisible(_) => 5,
            RasterMutation::ChangeLayerOpacity(_) => 6,
            RasterMutation::ChangeLayerBlendMode(_) => 7,
            RasterMutation::MoveLayer(_) => 8,
            RasterMutation::ResizeLayer(_) => 9,
            RasterMutation::ChangeLayerAdjustmentKind(_) => 10,
            RasterMutation::AddLayerAsset(_) => 11,
            RasterMutation::RemoveLayerAsset(_) => 12,
        }
    }

    fn observe_string(digest: &mut store::ArtifactStoreInitializationDigest, value: &String, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if value.capacity() > RASTER_OWNED_FIELD_BYTES {
            return Err("raster-store.digest-string-capacity");
        }
        if !raster_reserve_unit(cx) {
            return Ok(false);
        }
        digest.observe(&(value.len() as u64).to_be_bytes());
        digest.observe(value.as_bytes());
        Ok(true)
    }

    fn observe_scalar(digest: &mut store::ArtifactStoreInitializationDigest, bytes: &[u8], cx: &mut semio_framework_job::StepContext<'_>) -> bool {
        if !raster_reserve_unit(cx) {
            return false;
        }
        digest.observe(bytes);
        true
    }

    fn finish(&mut self, digest: &mut store::ArtifactStoreInitializationDigest, cx: &mut semio_framework_job::StepContext<'_>) -> bool {
        if !Self::observe_scalar(digest, &[0xff], cx) {
            return false;
        }
        self.terminal = true;
        true
    }

    fn step(&mut self, operation: &RasterMutation, digest: &mut store::ArtifactStoreInitializationDigest, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        if self.retirement_terminal {
            if !raster_reserve_unit(cx) {
                return Ok(false);
            }
            drop(self.retirement.take());
            self.retirement_terminal = false;
            return Ok(false);
        }
        if let Some(retirement) = self.retirement.as_mut() {
            return match retirement.close_step(1, RASTER_OWNED_FIELD_BYTES).map_err(|_| "raster-store.digest-layer-retirement")? {
                store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                    self.retirement_terminal = true;
                    Ok(false)
                }
                store::SnapshotRetirementStep::Complete => Err("raster-store.digest-layer-false-terminal"),
                _ => Ok(false),
            };
        }
        if self.phase == 0 {
            if !Self::observe_scalar(digest, &[Self::variant(operation)], cx) {
                return Ok(false);
            }
            self.phase = 1;
            return Ok(false);
        }
        macro_rules! string_phase {
            ($value:expr, $next:expr) => {{
                if !Self::observe_string(digest, $value, cx)? {
                    return Ok(false);
                }
                self.phase = $next;
                return Ok(false);
            }};
        }
        macro_rules! scalar_phase {
            ($value:expr, $next:expr) => {{
                if !Self::observe_scalar(digest, $value, cx) {
                    return Ok(false);
                }
                self.phase = $next;
                return Ok(false);
            }};
        }
        match operation {
            RasterMutation::CreateLayer(value) => match self.phase {
                1 => {
                    scalar_phase!(&[u8::from(value.parent_id.is_some())], if value.parent_id.is_some() { 2 } else { 3 });
                }
                2 => string_phase!(value.parent_id.as_ref().ok_or("raster-store.digest-create-parent")?, 3),
                3 => scalar_phase!(&(value.index as u64).to_be_bytes(), 4),
                4 => {
                    if self.layer.is_none() {
                        if std::mem::size_of::<RasterLayerCloneAuthority>() > RASTER_CONTROL_BACKING_BYTES {
                            return Err("raster-store.digest-layer-control-capacity");
                        }
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        *self.layer = Some(Box::new(RasterLayerCloneAuthority::new(&value.layer)));
                        return Ok(false);
                    }
                    let layer = self.layer.as_mut().expect("Raster mutation digest layer remains retained");
                    if !layer.step(&value.layer, digest, cx)? {
                        return Ok(false);
                    }
                    if !raster_reserve_unit(cx) {
                        return Ok(false);
                    }
                    let layer = layer.take().ok_or("raster-store.digest-layer-handoff")?;
                    drop(self.layer.take());
                    *self.retirement = Some(Box::new(RasterOwnedRetirement::new(RasterRetirementOwner::Layer(layer))));
                    Ok(false)
                }
                _ => Ok(self.finish(digest, cx)),
            },
            RasterMutation::DeleteLayer(value) => match self.phase {
                1 => string_phase!(&value.layer_id, 2),
                _ => Ok(self.finish(digest, cx)),
            },
            RasterMutation::ReorderLayers(value) => match self.phase {
                1 => string_phase!(&value.layer_id, 2),
                2 => {
                    scalar_phase!(&[u8::from(value.parent_id.is_some())], if value.parent_id.is_some() { 3 } else { 4 });
                }
                3 => string_phase!(value.parent_id.as_ref().ok_or("raster-store.digest-reorder-parent")?, 4),
                4 => scalar_phase!(&(value.index as u64).to_be_bytes(), 5),
                _ => Ok(self.finish(digest, cx)),
            },
            RasterMutation::RenameLayer(value) => match self.phase {
                1 => string_phase!(&value.layer_id, 2),
                2 => string_phase!(&value.new_name, 3),
                _ => Ok(self.finish(digest, cx)),
            },
            RasterMutation::ChangeLayerVisible(value) => match self.phase {
                1 => string_phase!(&value.layer_id, 2),
                2 => scalar_phase!(&[u8::from(value.new_visible)], 3),
                _ => Ok(self.finish(digest, cx)),
            },
            RasterMutation::ChangeLayerOpacity(value) => match self.phase {
                1 => string_phase!(&value.layer_id, 2),
                2 => scalar_phase!(&value.new_opacity.to_bits().to_be_bytes(), 3),
                _ => Ok(self.finish(digest, cx)),
            },
            RasterMutation::ChangeLayerBlendMode(value) => match self.phase {
                1 => string_phase!(&value.layer_id, 2),
                2 => string_phase!(&value.new_blend_mode, 3),
                _ => Ok(self.finish(digest, cx)),
            },
            RasterMutation::MoveLayer(value) => match self.phase {
                1 => string_phase!(&value.layer_id, 2),
                2 => scalar_phase!(&value.new_x.to_bits().to_be_bytes(), 3),
                3 => scalar_phase!(&value.new_y.to_bits().to_be_bytes(), 4),
                _ => Ok(self.finish(digest, cx)),
            },
            RasterMutation::ResizeLayer(value) => match self.phase {
                1 => string_phase!(&value.layer_id, 2),
                2 => scalar_phase!(&value.new_width.to_be_bytes(), 3),
                3 => scalar_phase!(&value.new_height.to_be_bytes(), 4),
                _ => Ok(self.finish(digest, cx)),
            },
            RasterMutation::ChangeLayerAdjustmentKind(value) => match self.phase {
                1 => string_phase!(&value.layer_id, 2),
                2 => string_phase!(&value.new_adjustment_kind, 3),
                _ => Ok(self.finish(digest, cx)),
            },
            RasterMutation::AddLayerAsset(value) => match self.phase {
                1 => string_phase!(&value.asset_id, 2),
                2 => string_phase!(&value.asset.mime, 3),
                3 => {
                    if value.asset.data.capacity() > RASTER_OWNED_FIELD_BYTES {
                        return Err("raster-store.digest-asset-byte-capacity");
                    }
                    let end = self.offset.saturating_add(256).min(value.asset.data.len());
                    if self.offset < end {
                        if !Self::observe_scalar(digest, &value.asset.data[self.offset..end], cx) {
                            return Ok(false);
                        }
                        self.offset = end;
                        return Ok(false);
                    }
                    self.phase = 4;
                    Ok(false)
                }
                _ => Ok(self.finish(digest, cx)),
            },
            RasterMutation::RemoveLayerAsset(value) => match self.phase {
                1 => string_phase!(&value.asset_id, 2),
                _ => Ok(self.finish(digest, cx)),
            },
        }
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.retirement_terminal {
            if maximum_bytes < RASTER_CONTROL_BACKING_BYTES {
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            drop(self.retirement.take());
            self.retirement_terminal = false;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: RASTER_CONTROL_BACKING_BYTES });
        }
        if let Some(layer) = self.layer.as_mut() {
            return match layer.close_step(1, maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if layer.terminal_is_empty() => {
                    if maximum_bytes < RASTER_CONTROL_BACKING_BYTES {
                        return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                    }
                    drop(self.layer.take());
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: RASTER_CONTROL_BACKING_BYTES })
                }
                store::SnapshotRetirementStep::Complete => Err("Raster digest layer false terminal".into()),
                step => Ok(step),
            };
        }
        if let Some(retirement) = self.retirement.as_mut() {
            return match retirement.close_step(1, maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                    if maximum_bytes < RASTER_CONTROL_BACKING_BYTES {
                        return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                    }
                    drop(self.retirement.take());
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: RASTER_CONTROL_BACKING_BYTES })
                }
                store::SnapshotRetirementStep::Complete => Err("Raster digest retirement false terminal".into()),
                step => Ok(step),
            };
        }
        self.terminal = true;
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.layer.is_none() && self.retirement.is_none() && !self.retirement_terminal
    }
}

impl Drop for RasterMutationDigestAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Raster mutation digest reached Drop with a retained derived owner");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RasterLayerAddress {
    length: usize,
    indices: [usize; RASTER_MAXIMUM_NESTED_DEPTH],
}

impl RasterLayerAddress {
    fn parent(self) -> Option<Self> {
        (self.length > 1).then(|| Self { length: self.length - 1, indices: self.indices })
    }

    fn index(self) -> usize {
        self.indices[self.length - 1]
    }
}

struct RasterLayerLocator {
    root: usize,
    depth: usize,
    path: [usize; RASTER_MAXIMUM_NESTED_DEPTH],
    frames: [RasterTraversalFrame; RASTER_MAXIMUM_NESTED_DEPTH],
    found: Option<RasterLayerAddress>,
    terminal: bool,
}

impl RasterLayerLocator {
    fn new() -> Self {
        Self { root: 0, depth: 0, path: [0; RASTER_MAXIMUM_NESTED_DEPTH], frames: [RasterTraversalFrame::EMPTY; RASTER_MAXIMUM_NESTED_DEPTH], found: None, terminal: false }
    }

    fn node_id(node: &RasterLayerNode) -> &String {
        match node {
            RasterLayerNode::Pixel { id, .. } | RasterLayerNode::Group { id, .. } | RasterLayerNode::Adjustment { id, .. } => id,
        }
    }

    fn node_at<'a>(snapshot: &'a RasterSnapshot, address: RasterLayerAddress) -> Option<&'a RasterLayerNode> {
        let mut value = snapshot.layers.get(address.indices[0])?;
        for index in &address.indices[1..address.length] {
            let RasterLayerNode::Group { children, .. } = value else { return None };
            value = children.get(*index)?;
        }
        Some(value)
    }

    fn node_at_mut<'a>(snapshot: &'a mut RasterSnapshot, address: RasterLayerAddress) -> Option<&'a mut RasterLayerNode> {
        fn descend<'a>(value: &'a mut RasterLayerNode, path: &[usize]) -> Option<&'a mut RasterLayerNode> {
            let Some((head, tail)) = path.split_first() else { return Some(value) };
            let RasterLayerNode::Group { children, .. } = value else { return None };
            descend(children.get_mut(*head)?, tail)
        }
        descend(snapshot.layers.get_mut(address.indices[0])?, &address.indices[1..address.length])
    }

    fn container_mut<'a>(snapshot: &'a mut RasterSnapshot, parent: Option<RasterLayerAddress>) -> Option<&'a mut Vec<RasterLayerNode>> {
        match parent {
            None => Some(&mut snapshot.layers),
            Some(address) => match Self::node_at_mut(snapshot, address)? {
                RasterLayerNode::Group { children, .. } => Some(children),
                _ => None,
            },
        }
    }

    fn step(&mut self, snapshot: &RasterSnapshot, target: &str, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        let Some(root) = snapshot.layers.get(self.root) else {
            self.terminal = true;
            return Ok(true);
        };
        let node = RasterLayerBoundsAuthority::layer_at(root, &self.path[..self.depth]).ok_or("raster-store.locator-path")?;
        let frame = self.frames[self.depth];
        if frame.phase == 0 {
            if !raster_reserve_unit(cx) {
                return Ok(false);
            }
            self.frames[self.depth].phase = 1;
            if Self::node_id(node) == target {
                let mut indices = [0; RASTER_MAXIMUM_NESTED_DEPTH];
                indices[0] = self.root;
                if self.depth > 0 {
                    indices[1..self.depth + 1].copy_from_slice(&self.path[..self.depth]);
                }
                self.found = Some(RasterLayerAddress { length: self.depth + 1, indices });
                self.terminal = true;
            }
            return Ok(self.terminal);
        }
        if let RasterLayerNode::Group { children, .. } = node {
            if frame.child < children.len() {
                if self.depth + 1 >= RASTER_MAXIMUM_NESTED_DEPTH {
                    return Err("raster-store.locator-depth");
                }
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                self.path[self.depth] = frame.child;
                self.frames[self.depth].child += 1;
                self.depth += 1;
                self.frames[self.depth] = RasterTraversalFrame::EMPTY;
                return Ok(false);
            }
        }
        if !raster_reserve_unit(cx) {
            return Ok(false);
        }
        if self.depth == 0 {
            self.root += 1;
            self.frames[0] = RasterTraversalFrame::EMPTY;
        } else {
            self.depth -= 1;
        }
        Ok(false)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RasterMutationCandidatePhase {
    Clone,
    LocatePrimary,
    LocateSecondary,
    PrepareLayer,
    Apply,
    ShiftRemove,
    LocateDestination,
    BeginInsert,
    ShiftInsert,
    PrepareAsset,
    Drain,
    Complete,
    Closing,
}

struct RasterMutationCandidateAuthority {
    value: std::mem::ManuallyDrop<Option<RasterSnapshot>>,
    clone: std::mem::ManuallyDrop<Option<Box<RasterSnapshotCloneAuthority>>>,
    layer_clone: std::mem::ManuallyDrop<Option<Box<RasterLayerCloneAuthority>>>,
    pending_layer: std::mem::ManuallyDrop<Option<RasterLayerNode>>,
    pending_asset: std::mem::ManuallyDrop<Option<(String, RasterAssetChild)>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    retirement_terminal: bool,
    asset_hasher: Option<std::collections::hash_map::DefaultHasher>,
    asset_hash: u64,
    asset_field: u8,
    locator: Option<RasterLayerLocator>,
    primary: Option<RasterLayerAddress>,
    secondary: Option<RasterLayerAddress>,
    container: Option<RasterLayerAddress>,
    shift_index: usize,
    shift_target: usize,
    phase: RasterMutationCandidatePhase,
    terminal: bool,
}

impl RasterMutationCandidateAuthority {
    fn new() -> Self {
        Self {
            value: std::mem::ManuallyDrop::new(None),
            clone: std::mem::ManuallyDrop::new(None),
            layer_clone: std::mem::ManuallyDrop::new(None),
            pending_layer: std::mem::ManuallyDrop::new(None),
            pending_asset: std::mem::ManuallyDrop::new(None),
            retirement: std::mem::ManuallyDrop::new(None),
            retirement_terminal: false,
            asset_hasher: None,
            asset_hash: 0,
            asset_field: 0,
            locator: None,
            primary: None,
            secondary: None,
            container: None,
            shift_index: 0,
            shift_target: 0,
            phase: RasterMutationCandidatePhase::Clone,
            terminal: false,
        }
    }

    fn target(operation: &RasterMutation) -> Option<&str> {
        match operation {
            RasterMutation::CreateLayer(value) => Some(RasterLayerLocator::node_id(&value.layer)),
            RasterMutation::DeleteLayer(value) => Some(&value.layer_id),
            RasterMutation::ReorderLayers(value) => Some(&value.layer_id),
            RasterMutation::RenameLayer(value) => Some(&value.layer_id),
            RasterMutation::ChangeLayerVisible(value) => Some(&value.layer_id),
            RasterMutation::ChangeLayerOpacity(value) => Some(&value.layer_id),
            RasterMutation::ChangeLayerBlendMode(value) => Some(&value.layer_id),
            RasterMutation::MoveLayer(value) => Some(&value.layer_id),
            RasterMutation::ResizeLayer(value) => Some(&value.layer_id),
            RasterMutation::ChangeLayerAdjustmentKind(value) => Some(&value.layer_id),
            RasterMutation::AddLayerAsset(_) | RasterMutation::RemoveLayerAsset(_) => None,
        }
    }

    fn parent(operation: &RasterMutation) -> Option<&str> {
        match operation {
            RasterMutation::CreateLayer(value) => value.parent_id.as_deref(),
            RasterMutation::ReorderLayers(value) => value.parent_id.as_deref(),
            _ => None,
        }
    }

    fn prepare_insert(&mut self, parent: Option<RasterLayerAddress>, index: usize) {
        self.container = parent;
        self.shift_target = index;
        self.phase = RasterMutationCandidatePhase::BeginInsert;
    }

    fn begin_insert(&mut self) -> Result<(), &'static str> {
        let snapshot = self.value.as_mut().ok_or("raster-store.mutation-insert-snapshot")?;
        let values = RasterLayerLocator::container_mut(snapshot, self.container).ok_or("raster-store.mutation-insert-parent")?;
        if self.shift_target > values.len() || values.len() >= values.capacity() {
            return Err("raster-store.mutation-insert-capacity");
        }
        values.push(self.pending_layer.take().ok_or("raster-store.mutation-insert-owner")?);
        self.shift_index = values.len() - 1;
        self.phase = RasterMutationCandidatePhase::ShiftInsert;
        Ok(())
    }

    fn begin_remove(&mut self, address: RasterLayerAddress) {
        self.container = address.parent();
        self.shift_index = address.index();
        self.phase = RasterMutationCandidatePhase::ShiftRemove;
    }

    fn replace_string(target: &mut String, source: &String) -> Result<Option<Box<dyn store::ErasedSnapshotRetirement>>, &'static str> {
        let replacement = raster_clone_owned_string(source)?;
        let previous = std::mem::replace(target, replacement);
        Ok(Some(Box::new(RasterOwnedRetirement::new(RasterRetirementOwner::String(previous)))))
    }

    fn exact_string(value: &str) -> Result<String, &'static str> {
        raster_exact_string_from_parts(&[value.as_bytes()])
    }

    fn pump_retirement(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        let Some(retirement) = self.retirement.as_mut() else { return Ok(false) };
        if self.retirement_terminal {
            if !raster_reserve_unit(cx) {
                return Ok(true);
            }
            drop(self.retirement.take());
            self.retirement_terminal = false;
            return Ok(true);
        }
        match retirement.close_step(1, RASTER_OWNED_FIELD_BYTES).map_err(|_| "raster-store.mutation-retirement-fault")? {
            store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                self.retirement_terminal = true;
                Ok(true)
            }
            store::SnapshotRetirementStep::Complete => Err("raster-store.mutation-retirement-false-terminal"),
            _ => Ok(true),
        }
    }

    fn step(&mut self, current: &RasterSnapshot, operation: &RasterMutation, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        if self.pump_retirement(cx)? {
            return Ok(false);
        }
        match self.phase {
            RasterMutationCandidatePhase::Clone => {
                if self.clone.is_none() {
                    if std::mem::size_of::<RasterSnapshotCloneAuthority>() > RASTER_CONTROL_BACKING_BYTES {
                        return Err("raster-store.mutation-clone-control-capacity");
                    }
                    if !raster_reserve_unit(cx) {
                        return Ok(false);
                    }
                    *self.clone = Some(Box::new(RasterSnapshotCloneAuthority::new()));
                    return Ok(false);
                }
                let clone = self.clone.as_mut().ok_or("raster-store.mutation-clone")?;
                let mut digest = store::ArtifactStoreInitializationDigest::new(b"raster.mutation-candidate");
                if !clone.step(current, &mut digest, cx)? {
                    return Ok(false);
                }
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                *self.value = clone.take_value();
                drop(self.clone.take());
                if Self::target(operation).is_some() {
                    self.locator = Some(RasterLayerLocator::new());
                    self.phase = RasterMutationCandidatePhase::LocatePrimary;
                } else {
                    self.phase = RasterMutationCandidatePhase::Apply;
                }
                Ok(false)
            }
            RasterMutationCandidatePhase::LocatePrimary => {
                let snapshot = self.value.as_ref().ok_or("raster-store.mutation-locate-snapshot")?;
                let locator = self.locator.as_mut().ok_or("raster-store.mutation-locator")?;
                if !locator.step(snapshot, Self::target(operation).ok_or("raster-store.mutation-target")?, cx)? {
                    return Ok(false);
                }
                self.primary = locator.found;
                self.locator = None;
                if matches!(operation, RasterMutation::CreateLayer(_)) {
                    if self.primary.is_some() {
                        return Err("raster-store.mutation-duplicate-layer");
                    }
                    if Self::parent(operation).is_some() {
                        self.locator = Some(RasterLayerLocator::new());
                        self.phase = RasterMutationCandidatePhase::LocateSecondary;
                    } else {
                        self.phase = RasterMutationCandidatePhase::PrepareLayer;
                    }
                } else if self.primary.is_none() {
                    return Err("raster-store.mutation-target-missing");
                } else {
                    self.phase = RasterMutationCandidatePhase::Apply;
                }
                Ok(false)
            }
            RasterMutationCandidatePhase::LocateSecondary | RasterMutationCandidatePhase::LocateDestination => {
                let snapshot = self.value.as_ref().ok_or("raster-store.mutation-parent-snapshot")?;
                let locator = self.locator.as_mut().ok_or("raster-store.mutation-parent-locator")?;
                if !locator.step(snapshot, Self::parent(operation).ok_or("raster-store.mutation-parent")?, cx)? {
                    return Ok(false);
                }
                self.secondary = locator.found;
                self.locator = None;
                let parent = self.secondary.ok_or("raster-store.mutation-parent-missing")?;
                if !matches!(RasterLayerLocator::node_at(snapshot, parent), Some(RasterLayerNode::Group { .. })) {
                    return Err("raster-store.mutation-parent-not-group");
                }
                if self.phase == RasterMutationCandidatePhase::LocateSecondary {
                    self.phase = RasterMutationCandidatePhase::PrepareLayer;
                } else {
                    let index = match operation {
                        RasterMutation::ReorderLayers(value) => value.index,
                        _ => return Err("raster-store.mutation-destination-variant"),
                    };
                    self.prepare_insert(Some(parent), index);
                }
                Ok(false)
            }
            RasterMutationCandidatePhase::PrepareLayer => {
                let RasterMutation::CreateLayer(value) = operation else { return Err("raster-store.mutation-prepare-variant") };
                if self.layer_clone.is_none() {
                    if std::mem::size_of::<RasterLayerCloneAuthority>() > RASTER_CONTROL_BACKING_BYTES {
                        return Err("raster-store.mutation-layer-control-capacity");
                    }
                    if !raster_reserve_unit(cx) {
                        return Ok(false);
                    }
                    *self.layer_clone = Some(Box::new(RasterLayerCloneAuthority::new(&value.layer)));
                    return Ok(false);
                }
                let clone = self.layer_clone.as_mut().expect("Raster create layer clone remains retained");
                let mut digest = store::ArtifactStoreInitializationDigest::new(b"raster.create-layer");
                if !clone.step(&value.layer, &mut digest, cx)? {
                    return Ok(false);
                }
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                *self.pending_layer = clone.take();
                drop(self.layer_clone.take());
                self.phase = RasterMutationCandidatePhase::Apply;
                Ok(false)
            }
            RasterMutationCandidatePhase::Apply => {
                let snapshot = self.value.as_mut().ok_or("raster-store.mutation-apply-snapshot")?;
                match operation {
                    RasterMutation::CreateLayer(value) => {
                        self.prepare_insert(self.secondary, value.index);
                        return Ok(false);
                    }
                    RasterMutation::DeleteLayer(_) | RasterMutation::ReorderLayers(_) => {
                        self.begin_remove(self.primary.ok_or("raster-store.mutation-remove-address")?);
                        return Ok(false);
                    }
                    RasterMutation::RenameLayer(value) => {
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        let (RasterLayerNode::Pixel { name, .. } | RasterLayerNode::Group { name, .. } | RasterLayerNode::Adjustment { name, .. }) =
                            RasterLayerLocator::node_at_mut(snapshot, self.primary.ok_or("raster-store.mutation-address")?).ok_or("raster-store.mutation-target-lost")?;
                        *self.retirement = Self::replace_string(name, &value.new_name)?;
                    }
                    RasterMutation::ChangeLayerVisible(value) => {
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        let (RasterLayerNode::Pixel { visible, .. } | RasterLayerNode::Group { visible, .. } | RasterLayerNode::Adjustment { visible, .. }) =
                            RasterLayerLocator::node_at_mut(snapshot, self.primary.ok_or("raster-store.mutation-address")?).ok_or("raster-store.mutation-target-lost")?;
                        *visible = value.new_visible;
                    }
                    RasterMutation::ChangeLayerOpacity(value) => {
                        if !value.new_opacity.is_finite() || !raster_reserve_unit(cx) {
                            if !value.new_opacity.is_finite() {
                                return Err("raster-store.mutation-opacity-invalid");
                            }
                            return Ok(false);
                        }
                        let (RasterLayerNode::Pixel { opacity, .. } | RasterLayerNode::Group { opacity, .. } | RasterLayerNode::Adjustment { opacity, .. }) =
                            RasterLayerLocator::node_at_mut(snapshot, self.primary.ok_or("raster-store.mutation-address")?).ok_or("raster-store.mutation-target-lost")?;
                        *opacity = value.new_opacity;
                    }
                    RasterMutation::ChangeLayerBlendMode(value) => {
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        let (RasterLayerNode::Pixel { blend_mode, .. } | RasterLayerNode::Group { blend_mode, .. } | RasterLayerNode::Adjustment { blend_mode, .. }) =
                            RasterLayerLocator::node_at_mut(snapshot, self.primary.ok_or("raster-store.mutation-address")?).ok_or("raster-store.mutation-target-lost")?;
                        *self.retirement = Self::replace_string(blend_mode, &value.new_blend_mode)?;
                    }
                    RasterMutation::MoveLayer(value) => {
                        if !value.new_x.is_finite() || !value.new_y.is_finite() || !raster_reserve_unit(cx) {
                            if !value.new_x.is_finite() || !value.new_y.is_finite() {
                                return Err("raster-store.mutation-transform-invalid");
                            }
                            return Ok(false);
                        }
                        match RasterLayerLocator::node_at_mut(snapshot, self.primary.ok_or("raster-store.mutation-address")?).ok_or("raster-store.mutation-target-lost")? {
                            RasterLayerNode::Pixel { transform, .. } | RasterLayerNode::Group { transform, .. } => {
                                transform.x = value.new_x;
                                transform.y = value.new_y;
                            }
                            RasterLayerNode::Adjustment { .. } => return Err("raster-store.mutation-transform-target"),
                        }
                    }
                    RasterMutation::ResizeLayer(value) => {
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        let RasterLayerNode::Pixel { width, height, .. } = RasterLayerLocator::node_at_mut(snapshot, self.primary.ok_or("raster-store.mutation-address")?).ok_or("raster-store.mutation-target-lost")? else {
                            return Err("raster-store.mutation-resize-target");
                        };
                        *width = Some(value.new_width);
                        *height = Some(value.new_height);
                    }
                    RasterMutation::ChangeLayerAdjustmentKind(value) => {
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        let RasterLayerNode::Adjustment { adjustment_kind, .. } = RasterLayerLocator::node_at_mut(snapshot, self.primary.ok_or("raster-store.mutation-address")?).ok_or("raster-store.mutation-target-lost")? else {
                            return Err("raster-store.mutation-adjustment-target");
                        };
                        *self.retirement = Self::replace_string(adjustment_kind, &value.new_adjustment_kind)?;
                    }
                    RasterMutation::AddLayerAsset(value) => {
                        if value.asset.mime.capacity() > RASTER_OWNED_FIELD_BYTES || value.asset.data.capacity() > RASTER_OWNED_FIELD_BYTES {
                            return Err("raster-store.mutation-asset-capacity");
                        }
                        self.asset_hasher = Some(std::collections::hash_map::DefaultHasher::new());
                        self.asset_field = 0;
                        self.phase = RasterMutationCandidatePhase::PrepareAsset;
                        return Ok(false);
                    }
                    RasterMutation::RemoveLayerAsset(value) => {
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        let mut removed = snapshot.assets.remove_entry(&value.asset_id).ok_or("raster-store.mutation-asset-missing")?;
                        let (key, child) = removed.take();
                        *self.retirement = Some(Box::new(RasterOwnedRetirement::new(RasterRetirementOwner::AssetEntry { key, child: Some(child) })));
                    }
                }
                self.phase = RasterMutationCandidatePhase::Drain;
                Ok(false)
            }
            RasterMutationCandidatePhase::PrepareAsset => {
                use std::hash::{Hash, Hasher};

                let RasterMutation::AddLayerAsset(value) = operation else { return Err("raster-store.mutation-asset-variant") };
                match self.asset_field {
                    0 => {
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        value.asset.mime.hash(self.asset_hasher.as_mut().ok_or("raster-store.mutation-asset-hasher")?);
                    }
                    1 => {
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        value.asset.data.hash(self.asset_hasher.as_mut().ok_or("raster-store.mutation-asset-hasher")?);
                        self.asset_hash = self.asset_hasher.as_ref().ok_or("raster-store.mutation-asset-hasher")?.finish();
                        self.asset_hasher = None;
                    }
                    2 => {
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        let key = raster_clone_owned_string(&value.asset_id)?;
                        let child =
                            store::ArtifactChild::new(String::new(), store::os_io::ArtifactRef { artifact_id: String::new(), dialect: store::os_io::ArtifactDialect { artifact_kind: String::new(), standard: String::new(), subset: String::new() } });
                        *self.pending_asset = Some((key, child));
                    }
                    3 => {
                        let length = "raster-asset-".len() + 16;
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        let child_id = raster_asset_child_id(self.asset_hash)?;
                        self.pending_asset.as_mut().ok_or("raster-store.mutation-asset-owner")?.1.child_id = child_id;
                    }
                    4 => {
                        let length = value.asset_id.len().checked_add(6).ok_or("raster-store.mutation-asset-id-overflow")?;
                        if length > RASTER_OWNED_FIELD_BYTES || !raster_reserve_unit(cx) {
                            if length > RASTER_OWNED_FIELD_BYTES {
                                return Err("raster-store.mutation-asset-id-capacity");
                            }
                            return Ok(false);
                        }
                        let artifact_id = raster_exact_string_from_parts(&[value.asset_id.as_bytes(), b"-image"])?;
                        self.pending_asset.as_mut().ok_or("raster-store.mutation-asset-owner")?.1.target.artifact_id = artifact_id;
                    }
                    5..=7 => {
                        let literal = match self.asset_field {
                            5 => "s.stdio.semio",
                            6 => "v1",
                            _ => "image",
                        };
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        let child = &mut self.pending_asset.as_mut().ok_or("raster-store.mutation-asset-owner")?.1;
                        let target = match self.asset_field {
                            5 => &mut child.target.dialect.artifact_kind,
                            6 => &mut child.target.dialect.standard,
                            _ => &mut child.target.dialect.subset,
                        };
                        *target = Self::exact_string(literal)?;
                    }
                    _ => {
                        let snapshot = self.value.as_mut().ok_or("raster-store.mutation-asset-snapshot")?;
                        let pending_key = &self.pending_asset.as_ref().ok_or("raster-store.mutation-asset-owner")?.0;
                        if snapshot.assets.page_required_for_insert(pending_key) {
                            let page_bytes = RasterOwnedMap::<RasterAssetChild>::conservative_page_credit_bytes();
                            if !raster_reserve_unit(cx) {
                                return Ok(false);
                            }
                            snapshot.assets.admit_one_page()?;
                            return Ok(false);
                        }
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        let (key, child) = self.pending_asset.take().ok_or("raster-store.mutation-asset-owner")?;
                        if let Some(slot) = snapshot.assets.get_mut(&value.asset_id) {
                            let previous = std::mem::replace(slot, child);
                            *self.retirement = Some(Box::new(RasterOwnedRetirement::new(RasterRetirementOwner::AssetEntry { key, child: Some(previous) })));
                        } else {
                            match snapshot.assets.insert_pre_admitted(key, child) {
                                Ok(RasterOwnedMapInsert::Inserted) => {}
                                Ok(RasterOwnedMapInsert::Replaced(mut previous)) => {
                                    let (previous_key, previous) = previous.take();
                                    *self.retirement = Some(Box::new(RasterOwnedRetirement::new(RasterRetirementOwner::AssetEntry { key: previous_key, child: Some(previous) })));
                                    return Err("raster-store.mutation-duplicate-asset");
                                }
                                Err(rejected) => {
                                    *self.retirement = Some(Box::new(RasterOwnedRetirement::new(RasterRetirementOwner::AssetEntry { key: rejected.key, child: Some(rejected.value) })));
                                    return Err(rejected.reason);
                                }
                            }
                        }
                        self.phase = RasterMutationCandidatePhase::Drain;
                        return Ok(false);
                    }
                }
                self.asset_field += 1;
                Ok(false)
            }
            RasterMutationCandidatePhase::ShiftRemove => {
                let snapshot = self.value.as_mut().ok_or("raster-store.mutation-shift-snapshot")?;
                let values = RasterLayerLocator::container_mut(snapshot, self.container).ok_or("raster-store.mutation-shift-parent")?;
                if self.shift_index + 1 < values.len() {
                    if !raster_reserve_unit(cx) {
                        return Ok(false);
                    }
                    values.swap(self.shift_index, self.shift_index + 1);
                    self.shift_index += 1;
                    return Ok(false);
                }
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                *self.pending_layer = values.pop();
                if matches!(operation, RasterMutation::DeleteLayer(_)) {
                    let layer = self.pending_layer.take().ok_or("raster-store.mutation-delete-owner")?;
                    *self.retirement = Some(Box::new(RasterOwnedRetirement::new(RasterRetirementOwner::Layer(layer))));
                    self.phase = RasterMutationCandidatePhase::Drain;
                } else if let RasterMutation::ReorderLayers(value) = operation {
                    self.secondary = None;
                    if value.parent_id.is_some() {
                        self.locator = Some(RasterLayerLocator::new());
                        self.phase = RasterMutationCandidatePhase::LocateDestination;
                    } else {
                        self.prepare_insert(None, value.index);
                    }
                } else {
                    return Err("raster-store.mutation-remove-variant");
                }
                Ok(false)
            }
            RasterMutationCandidatePhase::BeginInsert => {
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                self.begin_insert()?;
                Ok(false)
            }
            RasterMutationCandidatePhase::ShiftInsert => {
                let snapshot = self.value.as_mut().ok_or("raster-store.mutation-shift-snapshot")?;
                let values = RasterLayerLocator::container_mut(snapshot, self.container).ok_or("raster-store.mutation-shift-parent")?;
                if self.shift_index > self.shift_target {
                    if !raster_reserve_unit(cx) {
                        return Ok(false);
                    }
                    values.swap(self.shift_index, self.shift_index - 1);
                    self.shift_index -= 1;
                    return Ok(false);
                }
                self.phase = RasterMutationCandidatePhase::Drain;
                Ok(false)
            }
            RasterMutationCandidatePhase::Drain => {
                if self.retirement.is_none() {
                    self.phase = RasterMutationCandidatePhase::Complete;
                }
                Ok(false)
            }
            RasterMutationCandidatePhase::Complete => {
                self.terminal = true;
                Ok(true)
            }
            RasterMutationCandidatePhase::Closing => Ok(false),
        }
    }

    fn take(&mut self) -> Option<RasterSnapshot> {
        self.terminal.then(|| self.value.take()).flatten()
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.phase = RasterMutationCandidatePhase::Closing;
        if let Some(retirement) = self.retirement.as_mut() {
            return match retirement.close_step(1, maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                    if maximum_bytes < RASTER_CONTROL_BACKING_BYTES {
                        return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                    }
                    drop(self.retirement.take());
                    self.retirement_terminal = false;
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: RASTER_CONTROL_BACKING_BYTES })
                }
                store::SnapshotRetirementStep::Complete => Err("Raster candidate retirement false terminal".into()),
                step => Ok(step),
            };
        }
        if let Some(clone) = self.layer_clone.as_mut() {
            return match clone.close_step(1, maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if clone.terminal_is_empty() => {
                    if maximum_bytes < RASTER_CONTROL_BACKING_BYTES {
                        return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                    }
                    drop(self.layer_clone.take());
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: RASTER_CONTROL_BACKING_BYTES })
                }
                store::SnapshotRetirementStep::Complete => Err("Raster candidate layer clone false terminal".into()),
                step => Ok(step),
            };
        }
        if let Some(clone) = self.clone.as_mut() {
            return match clone.close_step(1, maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if clone.terminal_is_empty() => {
                    if maximum_bytes < RASTER_CONTROL_BACKING_BYTES {
                        return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                    }
                    drop(self.clone.take());
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: RASTER_CONTROL_BACKING_BYTES })
                }
                store::SnapshotRetirementStep::Complete => Err("Raster candidate snapshot clone false terminal".into()),
                step => Ok(step),
            };
        }
        if let Some(layer) = self.pending_layer.take() {
            *self.retirement = Some(Box::new(RasterOwnedRetirement::new(RasterRetirementOwner::Layer(layer))));
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some((key, child)) = self.pending_asset.take() {
            *self.retirement = Some(Box::new(RasterOwnedRetirement::new(RasterRetirementOwner::AssetEntry { key, child: Some(child) })));
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.asset_hasher.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(value) = self.value.take() {
            *self.retirement = Some(Box::new(RasterOwnedRetirement::new(RasterRetirementOwner::Snapshot(value))));
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.terminal = true;
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.value.is_none() && self.clone.is_none() && self.layer_clone.is_none() && self.pending_layer.is_none() && self.pending_asset.is_none() && self.retirement.is_none() && !self.retirement_terminal && self.asset_hasher.is_none()
    }
}

impl Drop for RasterMutationCandidateAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Raster mutation candidate reached Drop before exact handoff or retirement");
    }
}

pub fn raster_document_store_owners() -> store::MemberStoreOwners<RasterSnapshot, RasterMutation> {
    store::MemberStoreOwners::new(
        std::sync::Arc::new(RasterSnapshotRetirementFactory),
        std::sync::Arc::new(RasterSnapshotRetirementFactory),
        std::sync::Arc::new(RasterMutationRetirementFactory),
        Box::new(store::ArtifactStoreCursorDisposer::<RasterSnapshot, RasterMutation>::new()),
    )
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RasterStoreInitializationPhase {
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
    ReleaseControlSuccess,
    RetireCancelled,
    RetireFault,
    Complete,
    Cancelled,
    Fault,
}

struct RasterStoreInitializationAuthority {
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    envelope: std::mem::ManuallyDrop<Option<store::ArtifactEnvelope<RasterSnapshot, RasterMutation>>>,
    runtime: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationRuntime<RasterSnapshot>>>,
    candidate: std::mem::ManuallyDrop<Option<store::ArtifactStore<RasterSnapshot, RasterMutation>>>,
    active: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    active_terminal: bool,
    envelope_retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    envelope_retirement_terminal: bool,
    clone: std::mem::ManuallyDrop<Option<RasterSnapshotCloneAuthority>>,
    mutation_digest: std::mem::ManuallyDrop<Option<RasterMutationDigestAuthority>>,
    mutation_candidate: std::mem::ManuallyDrop<Option<RasterMutationCandidateAuthority>>,
    candidate_disposer: std::mem::ManuallyDrop<Option<semio_framework_plugin::ArtifactDocumentStoreDisposer<RasterSnapshot, RasterMutation>>>,
    initial_digest: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationDigest>>,
    edit_digest: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationDigest>>,
    control_reservation: std::mem::ManuallyDrop<Option<RasterInitializationControlReservation>>,
    phase: RasterStoreInitializationPhase,
    cancel_requested: bool,
    fault: Option<Vec<u8>>,
    terminal_handoff: bool,
}

impl RasterStoreInitializationAuthority {
    fn new(envelope: store::ArtifactEnvelope<RasterSnapshot, RasterMutation>, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Self {
        Self {
            operation,
            generation,
            envelope: std::mem::ManuallyDrop::new(Some(envelope)),
            runtime: std::mem::ManuallyDrop::new(None),
            candidate: std::mem::ManuallyDrop::new(None),
            active: std::mem::ManuallyDrop::new(None),
            active_terminal: false,
            envelope_retirement: std::mem::ManuallyDrop::new(None),
            envelope_retirement_terminal: false,
            clone: std::mem::ManuallyDrop::new(Some(RasterSnapshotCloneAuthority::new())),
            mutation_digest: std::mem::ManuallyDrop::new(None),
            mutation_candidate: std::mem::ManuallyDrop::new(None),
            candidate_disposer: std::mem::ManuallyDrop::new(None),
            initial_digest: std::mem::ManuallyDrop::new(Some(store::ArtifactStoreInitializationDigest::new(b"raster.initial"))),
            edit_digest: std::mem::ManuallyDrop::new(None),
            control_reservation: std::mem::ManuallyDrop::new(None),
            phase: RasterStoreInitializationPhase::ValidateEnvelope,
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
        self.phase = RasterStoreInitializationPhase::RetireFault;
    }

    fn pump_active(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, String> {
        if self.active_terminal {
            if !raster_reserve_unit(cx) {
                return Ok(true);
            }
            drop(self.active.take());
            self.active_terminal = false;
            return Ok(true);
        }
        let Some(active) = self.active.as_mut() else { return Ok(false) };
        if !raster_reserve_unit(cx) {
            return Ok(true);
        }
        match active.close_step(1, RASTER_OWNED_FIELD_BYTES)? {
            store::SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items <= 1 && released_bytes <= RASTER_OWNED_FIELD_BYTES => Ok(true),
            store::SnapshotRetirementStep::Pending { .. } => Err("Raster store initializer retirement exceeded its exact grant".into()),
            store::SnapshotRetirementStep::Blocked => Ok(true),
            store::SnapshotRetirementStep::Complete if active.terminal_is_empty() => {
                self.active_terminal = true;
                Ok(true)
            }
            store::SnapshotRetirementStep::Complete => Err("Raster store initializer retirement reported a false terminal".into()),
        }
    }

    fn pump_terminal_retirement(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, String> {
        if self.pump_active(cx)? {
            return Ok(false);
        }
        if let Some(candidate) = self.candidate.as_mut() {
            use semio_framework_plugin::ArtifactOwnedDisposer;
            if self.candidate_disposer.is_none() {
                *self.candidate_disposer = Some(semio_framework_plugin::ArtifactDocumentStoreDisposer::new());
                return Ok(false);
            }
            let disposer = self.candidate_disposer.as_mut().expect("Raster candidate disposer remains retained");
            return match disposer.close_step(candidate, 1, RASTER_OWNED_FIELD_BYTES)? {
                semio_framework_plugin::PluginCloseStep::Complete if disposer.terminal_is_empty(candidate) => {
                    drop(self.candidate_disposer.take());
                    drop(self.candidate.take());
                    Ok(false)
                }
                semio_framework_plugin::PluginCloseStep::Complete => Err("Raster completed candidate disposer reported false terminal".into()),
                _ => Ok(false),
            };
        }
        if let Some(candidate) = self.mutation_candidate.as_mut() {
            return match candidate.close_step(1, RASTER_OWNED_FIELD_BYTES)? {
                store::SnapshotRetirementStep::Complete if candidate.terminal_is_empty() => {
                    drop(self.mutation_candidate.take());
                    Ok(false)
                }
                store::SnapshotRetirementStep::Complete => Err("Raster mutation candidate reported false terminal".into()),
                _ => Ok(false),
            };
        }
        if let Some(digest) = self.mutation_digest.as_mut() {
            return match digest.close_step(1, RASTER_OWNED_FIELD_BYTES)? {
                store::SnapshotRetirementStep::Complete if digest.terminal_is_empty() => {
                    drop(self.mutation_digest.take());
                    Ok(false)
                }
                store::SnapshotRetirementStep::Complete => Err("Raster mutation digest reported false terminal".into()),
                _ => Ok(false),
            };
        }
        if let Some(runtime) = self.runtime.as_mut() {
            match runtime.close_step(&RasterSnapshotRetirementFactory, 1, RASTER_OWNED_FIELD_BYTES)? {
                store::SnapshotRetirementStep::Complete if runtime.terminal_is_empty() => {
                    drop(self.runtime.take());
                    return Ok(false);
                }
                store::SnapshotRetirementStep::Complete => return Err("Raster initialization runtime reported a false terminal".into()),
                _ => return Ok(false),
            }
        }
        if let Some(clone) = self.clone.as_mut() {
            match clone.close_step(1, RASTER_OWNED_FIELD_BYTES)? {
                store::SnapshotRetirementStep::Complete if clone.terminal_is_empty() => {
                    drop(self.clone.take());
                    return Ok(false);
                }
                store::SnapshotRetirementStep::Complete => return Err("Raster snapshot clone reported a false terminal".into()),
                _ => return Ok(false),
            }
        }
        if self.envelope_retirement.is_none() {
            if let Some(envelope) = self.envelope.take() {
                *self.envelope_retirement = Some(raster_envelope_decode_owner_bundle().retire_envelope(envelope));
                return Ok(false);
            }
        }
        if self.envelope_retirement_terminal {
            if !raster_reserve_unit(cx) {
                return Ok(false);
            }
            drop(self.envelope_retirement.take());
            self.envelope_retirement_terminal = false;
            return Ok(true);
        }
        if let Some(retirement) = self.envelope_retirement.as_mut() {
            if !raster_reserve_unit(cx) {
                return Ok(false);
            }
            return match retirement.close_step(1, RASTER_OWNED_FIELD_BYTES)? {
                store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                    self.envelope_retirement_terminal = true;
                    Ok(false)
                }
                store::SnapshotRetirementStep::Complete => Err("Raster initialization envelope retirement reported a false terminal".into()),
                _ => Ok(false),
            };
        }
        if let Some(control) = self.control_reservation.as_mut() {
            if !raster_reserve_unit(cx) {
                return Ok(false);
            }
            if control.return_one()? {
                drop(self.control_reservation.take());
            }
            return Ok(false);
        }
        Ok(true)
    }

    fn terminal_is_empty_inner(&self) -> bool {
        self.terminal_handoff
            && self.envelope.is_none()
            && self.runtime.is_none()
            && self.candidate.is_none()
            && self.active.is_none()
            && !self.active_terminal
            && self.envelope_retirement.is_none()
            && !self.envelope_retirement_terminal
            && self.clone.is_none()
            && self.mutation_digest.is_none()
            && self.mutation_candidate.is_none()
            && self.candidate_disposer.is_none()
            && self.initial_digest.is_none()
            && self.edit_digest.is_none()
            && self.control_reservation.is_none()
    }
}

impl semio_framework_plugin::ArtifactStoreInitializationAuthority<RasterSnapshot, RasterMutation> for RasterStoreInitializationAuthority {
    fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
        if cx.operation() != self.operation || cx.generation() != self.generation {
            self.fail(b"raster-store.initializer-stale-authority");
        }
        if (self.cancel_requested || cx.is_cancelled()) && !matches!(self.phase, RasterStoreInitializationPhase::RetireCancelled | RasterStoreInitializationPhase::Cancelled) {
            self.phase = RasterStoreInitializationPhase::RetireCancelled;
        }
        if cx.should_yield() {
            return semio_framework_job::StepOutcome::Yield;
        }
        match self.pump_active(cx) {
            Ok(true) => return semio_framework_job::StepOutcome::Yield,
            Ok(false) => {}
            Err(error) => {
                self.fault = Some(error.into_bytes());
                self.phase = RasterStoreInitializationPhase::RetireFault;
            }
        }
        match self.phase {
            RasterStoreInitializationPhase::ValidateEnvelope => {
                let Some(envelope) = self.envelope.as_ref() else {
                    self.fail(b"raster-store.initializer-envelope-missing");
                    return semio_framework_job::StepOutcome::Yield;
                };
                if envelope.schema != crate::artifacts::raster::RASTER_DOCUMENT_SCHEMA || envelope.id.is_empty() || envelope.id.len() > RASTER_OWNED_FIELD_BYTES {
                    self.fail(b"raster-store.initializer-envelope-invalid");
                } else {
                    self.phase = RasterStoreInitializationPhase::ValidateEditPair { left: 0, right: 1 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            RasterStoreInitializationPhase::ValidateEditPair { left, right } => {
                let envelope = self.envelope.as_ref().expect("validated Raster envelope remains retained");
                if left >= envelope.vcs.edits.len() {
                    self.phase = RasterStoreInitializationPhase::CloneInitial;
                } else if envelope.vcs.edits[left].id.len() > RASTER_OWNED_FIELD_BYTES {
                    self.fail(b"raster-store.initializer-hostile-edit-id");
                } else if right >= envelope.vcs.edits.len() {
                    self.phase = RasterStoreInitializationPhase::ValidateEditPair { left: left + 1, right: left + 2 };
                } else if envelope.vcs.edits[left].id == envelope.vcs.edits[right].id {
                    self.fail(b"raster-store.initializer-duplicate-edit");
                } else {
                    self.phase = RasterStoreInitializationPhase::ValidateEditPair { left, right: right + 1 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            RasterStoreInitializationPhase::CloneInitial => {
                let source = &self.envelope.as_ref().expect("Raster envelope remains retained during initial clone").vcs.initial_snapshot;
                let clone = self.clone.as_mut().expect("Raster initial clone authority remains retained");
                if clone.bounds.terminal && self.control_reservation.is_none() {
                    if !raster_reserve_unit(cx) {
                        return semio_framework_job::StepOutcome::Yield;
                    }
                    match RasterInitializationControlReservation::try_claim() {
                        Ok(Some(control)) => *self.control_reservation = Some(control),
                        Ok(None) => return semio_framework_job::StepOutcome::Yield,
                        Err(code) => {
                            self.fail(code.as_bytes());
                            return semio_framework_job::StepOutcome::Yield;
                        }
                    }
                    return semio_framework_job::StepOutcome::Yield;
                }
                let complete = match clone.step(source, self.initial_digest.as_mut().expect("Raster initial digest remains retained"), cx) {
                    Ok(complete) => complete,
                    Err(code) => {
                        self.fail(code.as_bytes());
                        return semio_framework_job::StepOutcome::Yield;
                    }
                };
                if complete {
                    let initial = clone.take_value().expect("Raster initial snapshot was built one semantic item at a time");
                    drop(self.clone.take());
                    let initial_digest = self.initial_digest.take().expect("Raster initial digest remains retained").finish();
                    let envelope = self.envelope.as_ref().expect("Raster envelope remains retained during runtime construction");
                    *self.runtime = Some(store::ArtifactStoreInitializationRuntime::new(&envelope.id, &envelope.schema, initial, initial_digest));
                    self.phase = RasterStoreInitializationPhase::SeedHistory { edit: 0, lane: 0, index: 0 };
                }
                semio_framework_job::StepOutcome::Yield
            }
            RasterStoreInitializationPhase::SeedHistory { edit, lane, index } => {
                let envelope = self.envelope.as_ref().expect("Raster envelope remains retained while causal history is seeded");
                let Some(entry) = envelope.vcs.edits.get(edit) else {
                    self.phase = RasterStoreInitializationPhase::FindApplied { position: 0, scan: 0 };
                    return semio_framework_job::StepOutcome::Yield;
                };
                let runtime = self.runtime.as_mut().expect("Raster runtime remains retained while history is seeded");
                match lane {
                    0 => {
                        if let Err(error) = runtime.seed_mutation(protocol::MutationId(entry.id.clone())) {
                            self.fault = Some(error.into_bytes());
                            self.phase = RasterStoreInitializationPhase::RetireFault;
                        } else {
                            runtime.observe_sequence(entry.sequence_number);
                            self.phase = RasterStoreInitializationPhase::SeedHistory { edit, lane: 1, index: 0 };
                        }
                    }
                    1 if index < entry.forwards.len() => {
                        let id = entry.mutation_meta.get(index).and_then(|meta| meta.mutation_id.clone()).or_else(|| entry.forwards[index].mutation_id()).unwrap_or_else(|| protocol::MutationId(format!("{}#{index}", entry.id)));
                        if let Err(error) = runtime.seed_mutation(id) {
                            self.fault = Some(error.into_bytes());
                            self.phase = RasterStoreInitializationPhase::RetireFault;
                        } else {
                            self.phase = RasterStoreInitializationPhase::SeedHistory { edit, lane, index: index + 1 };
                        }
                    }
                    1 => self.phase = RasterStoreInitializationPhase::SeedHistory { edit, lane: 2, index: 0 },
                    2 if index < entry.mutation_meta.len() => {
                        runtime.observe_timestamp(entry.mutation_meta[index].timestamp.clone());
                        self.phase = RasterStoreInitializationPhase::SeedHistory { edit, lane, index: index + 1 };
                    }
                    _ => self.phase = RasterStoreInitializationPhase::SeedHistory { edit: edit + 1, lane: 0, index: 0 },
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            RasterStoreInitializationPhase::FindApplied { position, scan } => {
                let Some(id) = self.applied_id(position) else {
                    let checkpoint = self.envelope.as_ref().and_then(|envelope| envelope.cursor.as_ref().and_then(|cursor| cursor.checkpoint_id.clone()).or_else(|| envelope.vcs.checkpoints.last().map(|checkpoint| checkpoint.id.clone())));
                    self.runtime.as_mut().expect("Raster runtime remains retained").set_current_checkpoint_id(checkpoint);
                    self.phase = RasterStoreInitializationPhase::FindRedo { position: 0, scan: 0 };
                    return semio_framework_job::StepOutcome::Yield;
                };
                let envelope = self.envelope.as_ref().expect("Raster envelope remains retained");
                let Some(edit) = envelope.vcs.edits.get(scan) else {
                    self.fail(b"raster-store.initializer-applied-edit-missing");
                    return semio_framework_job::StepOutcome::Yield;
                };
                if edit.id == id {
                    let mut digest = store::ArtifactStoreInitializationDigest::new(b"raster.edit");
                    digest.observe(edit.id.as_bytes());
                    digest.observe(&edit.sequence_number.to_be_bytes());
                    digest.observe(edit.started_at.as_bytes());
                    *self.edit_digest = Some(digest);
                    self.phase = RasterStoreInitializationPhase::ApplyForward { position, edit: scan, mutation: 0 };
                } else {
                    self.phase = RasterStoreInitializationPhase::FindApplied { position, scan: scan + 1 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            RasterStoreInitializationPhase::ApplyForward { position, edit, mutation } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Raster applied edit remains retained");
                let Some(operation) = entry.forwards.get(mutation) else {
                    self.phase = RasterStoreInitializationPhase::HashInverse { position, edit, mutation: 0 };
                    return semio_framework_job::StepOutcome::Yield;
                };
                if self.mutation_digest.is_none() {
                    if !raster_reserve_unit(cx) {
                        return semio_framework_job::StepOutcome::Yield;
                    }
                    *self.mutation_digest = Some(RasterMutationDigestAuthority::new());
                    return semio_framework_job::StepOutcome::Yield;
                }
                let digest_complete = match self.mutation_digest.as_mut().expect("Raster forward digest remains retained").step(operation, self.edit_digest.as_mut().expect("Raster edit digest remains retained"), cx) {
                    Ok(value) => value,
                    Err(code) => {
                        self.fail(code.as_bytes());
                        return semio_framework_job::StepOutcome::Yield;
                    }
                };
                if !digest_complete {
                    return semio_framework_job::StepOutcome::Yield;
                }
                drop(self.mutation_digest.take());
                if self.mutation_candidate.is_none() {
                    if !raster_reserve_unit(cx) {
                        return semio_framework_job::StepOutcome::Yield;
                    }
                    *self.mutation_candidate = Some(RasterMutationCandidateAuthority::new());
                    return semio_framework_job::StepOutcome::Yield;
                }
                let current = self.runtime.as_mut().and_then(store::ArtifactStoreInitializationRuntime::current_mut).expect("Raster runtime current snapshot remains retained");
                let candidate_complete = match self.mutation_candidate.as_mut().expect("Raster mutation candidate remains retained").step(current, operation, cx) {
                    Ok(value) => value,
                    Err(code) => {
                        self.fail(code.as_bytes());
                        return semio_framework_job::StepOutcome::Yield;
                    }
                };
                if candidate_complete {
                    let next = self.mutation_candidate.as_mut().expect("Raster completed mutation candidate remains retained").take().expect("Raster mutation candidate handoff");
                    drop(self.mutation_candidate.take());
                    let current = self.runtime.as_mut().and_then(store::ArtifactStoreInitializationRuntime::current_mut).expect("Raster runtime current snapshot remains retained");
                    let previous = std::mem::replace(current, next);
                    *self.active = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&RasterSnapshotRetirementFactory, previous));
                    self.phase = RasterStoreInitializationPhase::ApplyForward { position, edit, mutation: mutation + 1 };
                }
                semio_framework_job::StepOutcome::Yield
            }
            RasterStoreInitializationPhase::HashInverse { position, edit, mutation } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Raster applied edit remains retained");
                let Some(operation) = entry.inverse.get(mutation) else {
                    self.phase = RasterStoreInitializationPhase::CommitApplied { position, edit };
                    return semio_framework_job::StepOutcome::Yield;
                };
                if self.mutation_digest.is_none() {
                    if raster_reserve_unit(cx) {
                        *self.mutation_digest = Some(RasterMutationDigestAuthority::new());
                    }
                    return semio_framework_job::StepOutcome::Yield;
                }
                match self.mutation_digest.as_mut().expect("Raster inverse digest remains retained").step(operation, self.edit_digest.as_mut().expect("Raster edit digest remains retained"), cx) {
                    Ok(true) => {
                        drop(self.mutation_digest.take());
                        self.phase = RasterStoreInitializationPhase::HashInverse { position, edit, mutation: mutation + 1 };
                    }
                    Ok(false) => {}
                    Err(code) => self.fail(code.as_bytes()),
                }
                semio_framework_job::StepOutcome::Yield
            }
            RasterStoreInitializationPhase::CommitApplied { position, edit } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Raster applied edit remains retained");
                let id = entry.id.clone();
                let actor = entry.actor.clone();
                let digest = self.edit_digest.take().expect("Raster applied edit digest remains retained").finish();
                let runtime = self.runtime.as_mut().expect("Raster runtime remains retained");
                if let Err(error) = runtime.push_applied(id, digest) {
                    self.fault = Some(error.into_bytes());
                    self.phase = RasterStoreInitializationPhase::RetireFault;
                } else {
                    runtime.set_local_actor_id(actor);
                    self.phase = RasterStoreInitializationPhase::FindApplied { position: position + 1, scan: 0 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            RasterStoreInitializationPhase::FindRedo { position, scan } => {
                let Some(id) = self.redo_id(position) else {
                    self.phase = RasterStoreInitializationPhase::BuildCandidate;
                    return semio_framework_job::StepOutcome::Yield;
                };
                let envelope = self.envelope.as_ref().expect("Raster envelope remains retained");
                let Some(edit) = envelope.vcs.edits.get(scan) else {
                    self.fail(b"raster-store.initializer-redo-edit-missing");
                    return semio_framework_job::StepOutcome::Yield;
                };
                if edit.id == id {
                    let mut digest = store::ArtifactStoreInitializationDigest::new(b"raster.edit");
                    digest.observe(edit.id.as_bytes());
                    digest.observe(&edit.sequence_number.to_be_bytes());
                    digest.observe(edit.started_at.as_bytes());
                    *self.edit_digest = Some(digest);
                    self.phase = RasterStoreInitializationPhase::HashRedoForward { position, edit: scan, mutation: 0 };
                } else {
                    self.phase = RasterStoreInitializationPhase::FindRedo { position, scan: scan + 1 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            RasterStoreInitializationPhase::HashRedoForward { position, edit, mutation } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Raster redo edit remains retained");
                let Some(operation) = entry.forwards.get(mutation) else {
                    self.phase = RasterStoreInitializationPhase::HashRedoInverse { position, edit, mutation: 0 };
                    return semio_framework_job::StepOutcome::Yield;
                };
                if self.mutation_digest.is_none() {
                    if raster_reserve_unit(cx) {
                        *self.mutation_digest = Some(RasterMutationDigestAuthority::new());
                    }
                    return semio_framework_job::StepOutcome::Yield;
                }
                match self.mutation_digest.as_mut().expect("Raster redo forward digest remains retained").step(operation, self.edit_digest.as_mut().expect("Raster redo digest remains retained"), cx) {
                    Ok(true) => {
                        drop(self.mutation_digest.take());
                        self.phase = RasterStoreInitializationPhase::HashRedoForward { position, edit, mutation: mutation + 1 };
                    }
                    Ok(false) => {}
                    Err(code) => self.fail(code.as_bytes()),
                }
                semio_framework_job::StepOutcome::Yield
            }
            RasterStoreInitializationPhase::HashRedoInverse { position, edit, mutation } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Raster redo edit remains retained");
                let Some(operation) = entry.inverse.get(mutation) else {
                    self.phase = RasterStoreInitializationPhase::CommitRedo { position, edit };
                    return semio_framework_job::StepOutcome::Yield;
                };
                if self.mutation_digest.is_none() {
                    if raster_reserve_unit(cx) {
                        *self.mutation_digest = Some(RasterMutationDigestAuthority::new());
                    }
                    return semio_framework_job::StepOutcome::Yield;
                }
                match self.mutation_digest.as_mut().expect("Raster redo inverse digest remains retained").step(operation, self.edit_digest.as_mut().expect("Raster redo digest remains retained"), cx) {
                    Ok(true) => {
                        drop(self.mutation_digest.take());
                        self.phase = RasterStoreInitializationPhase::HashRedoInverse { position, edit, mutation: mutation + 1 };
                    }
                    Ok(false) => {}
                    Err(code) => self.fail(code.as_bytes()),
                }
                semio_framework_job::StepOutcome::Yield
            }
            RasterStoreInitializationPhase::CommitRedo { position, edit } => {
                let id = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Raster redo edit remains retained").id.clone();
                let digest = self.edit_digest.take().expect("Raster redo digest remains retained").finish();
                if let Err(error) = self.runtime.as_mut().expect("Raster runtime remains retained").push_redo(id, digest) {
                    self.fault = Some(error.into_bytes());
                    self.phase = RasterStoreInitializationPhase::RetireFault;
                } else {
                    self.phase = RasterStoreInitializationPhase::FindRedo { position: position + 1, scan: 0 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            RasterStoreInitializationPhase::BuildCandidate => {
                if !raster_reserve_unit(cx) {
                    return semio_framework_job::StepOutcome::Yield;
                }
                let Some(candidate_generation) = self.generation.0.checked_add(1) else {
                    self.fail(b"raster-store.initializer-generation-exhausted");
                    return semio_framework_job::StepOutcome::Yield;
                };
                let envelope = self.envelope.take().expect("Raster envelope remains retained until atomic store construction");
                let runtime = self.runtime.take().expect("Raster runtime remains retained until atomic store construction");
                let candidate = store::ArtifactStore::from_initialized_runtime_with_owners(envelope, runtime, candidate_generation, raster_document_store_owners());
                *self.candidate = Some(candidate);
                self.phase = RasterStoreInitializationPhase::ReleaseControlSuccess;
                semio_framework_job::StepOutcome::Yield
            }
            RasterStoreInitializationPhase::ReleaseControlSuccess => {
                if !raster_reserve_unit(cx) {
                    return semio_framework_job::StepOutcome::Yield;
                }
                let control = self.control_reservation.as_mut().expect("Raster completed candidate retains its exact control reservation");
                match control.return_one() {
                    Ok(true) => {
                        drop(self.control_reservation.take());
                        self.phase = RasterStoreInitializationPhase::Complete;
                        semio_framework_job::StepOutcome::Complete(semio_framework_job::CommitCandidate {
                            state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
                            output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
                        })
                    }
                    Ok(false) => semio_framework_job::StepOutcome::Yield,
                    Err(code) => {
                        self.fail(code.as_bytes());
                        semio_framework_job::StepOutcome::Yield
                    }
                }
            }
            RasterStoreInitializationPhase::RetireCancelled | RasterStoreInitializationPhase::RetireFault => match self.pump_terminal_retirement(cx) {
                Ok(false) => semio_framework_job::StepOutcome::Yield,
                Ok(true) => {
                    drop(self.initial_digest.take());
                    drop(self.edit_digest.take());
                    self.terminal_handoff = true;
                    if self.phase == RasterStoreInitializationPhase::RetireCancelled {
                        self.phase = RasterStoreInitializationPhase::Cancelled;
                        semio_framework_job::StepOutcome::Cancelled
                    } else {
                        self.phase = RasterStoreInitializationPhase::Fault;
                        let source = self.fault.take().unwrap_or_else(|| b"raster-store.initializer-fault".to_vec());
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
            RasterStoreInitializationPhase::Complete => semio_framework_job::StepOutcome::Complete(semio_framework_job::CommitCandidate {
                state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
                output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
            }),
            RasterStoreInitializationPhase::Cancelled => semio_framework_job::StepOutcome::Cancelled,
            RasterStoreInitializationPhase::Fault => {
                let source = self.fault.as_deref().unwrap_or(b"raster-store.initializer-fault");
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
        if !matches!(self.phase, RasterStoreInitializationPhase::Cancelled | RasterStoreInitializationPhase::Fault) {
            self.phase = RasterStoreInitializationPhase::RetireCancelled;
        }
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<semio_framework_plugin::PluginCloseStep, semio_framework::Fault> {
        self.begin_close();
        if maximum_items == 0 || maximum_bytes < RASTER_OWNED_FIELD_BYTES {
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        match self.pump_terminal_retirement() {
            Ok(false) => Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }),
            Ok(true) => {
                drop(self.initial_digest.take());
                drop(self.edit_digest.take());
                self.terminal_handoff = true;
                Ok(semio_framework_plugin::PluginCloseStep::Complete)
            }
            Err(error) => Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("artifact-store.initializer-close"), format!("Raster initializer close failed: {error}"))),
        }
    }

    fn take_candidate(&mut self) -> Option<store::ArtifactStore<RasterSnapshot, RasterMutation>> {
        if self.phase != RasterStoreInitializationPhase::Complete || self.terminal_handoff {
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

impl Drop for RasterStoreInitializationAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty_inner(), "Raster store initialization authority reached Drop before exact candidate handoff or retained rejection close");
    }
}

pub fn raster_document_store_initialization_job(
    envelope: store::ArtifactEnvelope<RasterSnapshot, RasterMutation>,
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
) -> semio_framework_plugin::ArtifactStoreInitializationJob<RasterSnapshot, RasterMutation> {
    semio_framework_plugin::ArtifactStoreInitializationJob::new(Box::new(RasterStoreInitializationAuthority::new(envelope, operation, generation)))
}
//#endregion 🔖️RetainedStoreInitialization

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::raster::mutations::{
        add_layer_asset, change_layer_adjustment_kind, change_layer_blend_mode, change_layer_opacity, change_layer_visible, create_layer, delete_layer, move_layer, remove_layer_asset, rename_layer, reorder_layers, resize_layer,
    };
    use crate::artifacts::raster::schema::empty_raster_document;
    use crate::artifacts::raster::{RasterLayerNode, RasterTransform, RASTER_DOCUMENT_SCHEMA};

    static RASTER_INITIALIZER_TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    static RASTER_STANDALONE_RETIREMENT_TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    #[semio_framework_async_macros::async_test]
    async fn op_binary_round_trips_and_agrees_with_text() {
        let document = empty_raster_document();
        let operation = RasterMutation::CreateLayer(create_layer::mutation::CreateLayer {
            parent_id: None,
            index: document.layers.len(),
            layer: Box::new(RasterLayerNode::Pixel {
                id: "op-binary-test".into(),
                name: "Op Binary Test".into(),
                visible: true,
                opacity: 1.0,
                blend_mode: "normal".into(),
                transform: RasterTransform::default(),
                mask: None,
                width: Some(64),
                height: Some(64),
                image_key: None,
            }),
        });
        store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[semio_framework_async_macros::async_test]
    async fn raster_document_text_round_trips_store_with_applied_operation() {
        use crate::artifacts::raster::RasterSnapshot;

        let envelope = store::create_document_envelope::<RasterSnapshot, RasterMutation>(RASTER_DOCUMENT_SCHEMA, "doc-text-test", empty_raster_document(), None);
        let mut store = store::ArtifactStore::new(envelope).expect("valid artifact store fixture");
        store
            .dispatch(store::ArtifactCommand::Apply {
                mutations: vec![RasterMutation::CreateLayer(create_layer::mutation::CreateLayer {
                    parent_id: None,
                    index: 1,
                    layer: Box::new(RasterLayerNode::Adjustment {
                        id: "adjust-text".into(),
                        name: "Levels".into(),
                        visible: true,
                        opacity: 1.0,
                        blend_mode: "normal".into(),
                        transform: RasterTransform::default(),
                        adjustment_kind: "levels".into(),
                        params: RasterOwnedMap::new(),
                    }),
                })],
                description: None,
            })
            .expect("apply");
        store::os_store::test_support::assert_document_text_round_trip(&store);
        store::os_store::test_support::assert_document_pack_round_trip(&store);
    }

    fn empty_raster_initializer(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> RasterStoreInitializationAuthority {
        let envelope = store::create_document_envelope(RASTER_DOCUMENT_SCHEMA, "raster-retained-load", empty_raster_document(), None);
        RasterStoreInitializationAuthority::new(envelope, operation, generation)
    }

    fn drive_raster_initializer(authority: &mut RasterStoreInitializationAuthority, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> semio_framework_job::StepOutcome {
        let _guard = RASTER_INITIALIZER_TEST_LOCK.lock().expect("Raster initializer test lock");
        let cancel = semio_framework_job::root_cancel_token();
        let mut preview_sequence = 0;
        for _ in 0..100_000 {
            let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(4_096, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
            let outcome = semio_framework_plugin::ArtifactStoreInitializationAuthority::step(authority, &mut context);
            if outcome.is_terminal() {
                return outcome;
            }
        }
        panic!("Raster retained initializer did not reach a bounded terminal")
    }

    fn close_raster_candidate(mut candidate: store::ArtifactStore<RasterSnapshot, RasterMutation>) {
        use semio_framework_plugin::ArtifactOwnedDisposer;

        let mut disposer = semio_framework_plugin::ArtifactDocumentStoreDisposer::<RasterSnapshot, RasterMutation>::new();
        for _ in 0..100_000 {
            match disposer.close_step(&mut candidate, 1, RASTER_OWNED_FIELD_BYTES).expect("Raster candidate close step") {
                semio_framework_plugin::PluginCloseStep::Pending { released_items, released_bytes } => {
                    assert!(released_items <= 1);
                    assert!(released_bytes <= RASTER_OWNED_FIELD_BYTES);
                }
                semio_framework_plugin::PluginCloseStep::Blocked { reason } => panic!("fresh Raster candidate close unexpectedly blocked: {reason}"),
                semio_framework_plugin::PluginCloseStep::Complete => {
                    assert!(disposer.terminal_is_empty(&candidate));
                    drop(disposer);
                    drop(candidate);
                    return;
                }
            }
        }
        panic!("Raster candidate did not reach terminal-empty close")
    }

    fn close_raster_retirement(retirement: &mut dyn store::ErasedSnapshotRetirement) {
        for _ in 0..200_000 {
            if retirement.terminal_is_empty() {
                return;
            }
            let step = retirement.close_step(1, RASTER_OWNED_FIELD_BYTES).expect("Raster retained owner closes after admitted saturation resumes");
            if let store::SnapshotRetirementStep::Pending { released_items, released_bytes } = step {
                assert!(released_items <= 1);
                assert!(released_bytes <= RASTER_OWNED_FIELD_BYTES);
            }
        }
        panic!("Raster retained owner did not reach terminal-empty");
    }

    #[test]
    fn raster_store_initializer_publishes_next_generation_and_candidate_closes_incrementally() {
        let operation = semio_framework_job::OperationId(701);
        let generation = semio_framework_job::Generation(31);
        let mut authority = empty_raster_initializer(operation, generation);
        assert!(matches!(drive_raster_initializer(&mut authority, operation, generation), semio_framework_job::StepOutcome::Complete(_)));
        let candidate = semio_framework_plugin::ArtifactStoreInitializationAuthority::take_candidate(&mut authority).expect("exact Raster candidate");
        assert_eq!(candidate.generation_now(), 32);
        assert!(semio_framework_plugin::ArtifactStoreInitializationAuthority::terminal_is_empty(&authority));
        drop(authority);
        close_raster_candidate(candidate);
    }

    #[test]
    fn raster_store_initializer_cancel_and_stale_generation_return_every_owner_terminal_empty() {
        let operation = semio_framework_job::OperationId(702);
        let generation = semio_framework_job::Generation(33);
        let mut cancelled = empty_raster_initializer(operation, generation);
        semio_framework_plugin::ArtifactStoreInitializationAuthority::request_cancel(&mut cancelled);
        assert!(matches!(drive_raster_initializer(&mut cancelled, operation, generation), semio_framework_job::StepOutcome::Cancelled));
        assert!(semio_framework_plugin::ArtifactStoreInitializationAuthority::terminal_is_empty(&cancelled));
        drop(cancelled);

        let mut stale = empty_raster_initializer(operation, generation);
        assert!(matches!(drive_raster_initializer(&mut stale, operation, semio_framework_job::Generation(generation.0 + 1)), semio_framework_job::StepOutcome::Fault(_)));
        assert!(semio_framework_plugin::ArtifactStoreInitializationAuthority::terminal_is_empty(&stale));
        drop(stale);
    }

    #[test]
    fn raster_store_initializer_zero_budget_advances_no_owner_or_phase() {
        let operation = semio_framework_job::OperationId(703);
        let generation = semio_framework_job::Generation(35);
        let mut authority = empty_raster_initializer(operation, generation);
        let cancel = semio_framework_job::root_cancel_token();
        let mut preview_sequence = 0;
        let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(0, u64::MAX), cancel, semio_framework_job::default_now_us, &mut preview_sequence);
        assert!(matches!(semio_framework_plugin::ArtifactStoreInitializationAuthority::step(&mut authority, &mut context), semio_framework_job::StepOutcome::Yield));
        assert_eq!(authority.phase, RasterStoreInitializationPhase::ValidateEnvelope);
        assert!(authority.envelope.is_some());
        semio_framework_plugin::ArtifactStoreInitializationAuthority::request_cancel(&mut authority);
        assert!(matches!(drive_raster_initializer(&mut authority, operation, generation), semio_framework_job::StepOutcome::Cancelled));
        assert!(semio_framework_plugin::ArtifactStoreInitializationAuthority::terminal_is_empty(&authority));
        drop(authority);
    }

    fn deeply_nested_raster_snapshot(depth: usize) -> RasterSnapshot {
        let mut layer = RasterLayerNode::Pixel { id: "leaf".into(), name: "Leaf".into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, width: Some(1), height: Some(1), image_key: None };
        for index in 0..depth {
            layer = RasterLayerNode::Group { id: format!("group-{index}"), name: format!("Group {index}"), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, children: vec![layer] };
        }
        let mut snapshot = empty_raster_document();
        snapshot.layers.clear();
        snapshot.layers.push(layer);
        snapshot
    }

    #[test]
    fn raster_snapshot_bounds_and_clone_advance_one_pre_admitted_unit_with_low_nonzero_fuel() {
        let source = deeply_nested_raster_snapshot(48);
        let operation = semio_framework_job::OperationId(704);
        let generation = semio_framework_job::Generation(36);
        let cancel = semio_framework_job::root_cancel_token();
        let mut preview_sequence = 0;
        let mut clone = RasterSnapshotCloneAuthority::new();
        let mut digest = store::ArtifactStoreInitializationDigest::new(b"raster.low-fuel");
        let mut turns = 0;
        while !clone.terminal {
            let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
            assert!(!clone.step(&source, &mut digest, &mut context).expect("bounded Raster clone") || clone.terminal);
            turns += 1;
            assert!(turns < 20_000);
        }
        assert!(turns > 48, "nested clone must resume across the recursive layer depth");
        let candidate = clone.take_value().expect("bounded clone candidate");
        drop(clone);
        assert_eq!(candidate, source);
        let mut retirement = RasterOwnedRetirement::new(RasterRetirementOwner::Snapshot(candidate));
        while !store::ErasedSnapshotRetirement::terminal_is_empty(&retirement) {
            let step = store::ErasedSnapshotRetirement::close_step(&mut retirement, 1, RASTER_OWNED_FIELD_BYTES).expect("bounded candidate retirement");
            if let store::SnapshotRetirementStep::Pending { released_items, released_bytes } = step {
                assert!(released_items <= 1);
                assert!(released_bytes <= RASTER_OWNED_FIELD_BYTES);
            }
        }
        drop(retirement);
    }

    #[test]
    fn raster_empty_bounds_and_mounted_sixty_four_fuel_progress_across_second_map_page() {
        let _guard = RASTER_INITIALIZER_TEST_LOCK.lock().expect("mounted Raster initializer test lock");
        assert_eq!(RASTER_INITIALIZATION_PROCESS_CONTROLS.load(std::sync::atomic::Ordering::Acquire), 0);
        let operation = semio_framework_job::OperationId(7_041);
        let generation = semio_framework_job::Generation(361);
        let cancel = semio_framework_job::root_cancel_token();
        let mut preview_sequence = 0;
        let empty = empty_raster_document();
        let mut bounds = RasterSnapshotBoundsAuthority::new();
        for _ in 0..256 {
            let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(64, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
            if bounds.step(&empty, &mut context).expect("empty Raster bounds remain admissible") {
                break;
            }
        }
        assert!(bounds.terminal);
        assert!(bounds.totals.source_bytes < RASTER_MAXIMUM_NESTED_BYTES);
        assert_eq!(bounds.totals.source_control_items, RASTER_MAXIMUM_CONTROL_BACKINGS);
        assert_eq!(bounds.totals.source_control_bytes, RASTER_MAXIMUM_CONTROL_BYTES);

        let mut source = empty_raster_document();
        source.layers.push(RasterLayerNode::Pixel {
            id: "mounted-layer".into(),
            name: "Mounted".into(),
            visible: true,
            opacity: 1.0,
            blend_mode: "normal".into(),
            transform: RasterTransform::default(),
            mask: None,
            width: Some(1),
            height: Some(1),
            image_key: None,
        });
        for index in 0..(crate::artifacts::raster::RASTER_OWNED_MAP_PAGE_CAPACITY + 1) {
            source
                .assets
                .insert(
                    format!("mounted-{index}"),
                    store::ArtifactChild::new(
                        format!("child-{index}"),
                        store::os_io::ArtifactRef { artifact_id: format!("artifact-{index}"), dialect: store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "image".into() } },
                    ),
                )
                .expect("mounted-shaped source admits its second fixed map page");
        }
        let envelope = store::create_document_envelope(RASTER_DOCUMENT_SCHEMA, "raster-mounted-64-fuel", source, None);
        let mut authority = RasterStoreInitializationAuthority::new(envelope, operation, generation);
        let mut terminal = None;
        for _ in 0..100_000 {
            let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(64, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
            let outcome = semio_framework_plugin::ArtifactStoreInitializationAuthority::step(&mut authority, &mut context);
            if outcome.is_terminal() {
                terminal = Some(outcome);
                break;
            }
        }
        assert!(matches!(terminal, Some(semio_framework_job::StepOutcome::Complete(_))));
        assert_eq!(RASTER_INITIALIZATION_PROCESS_CONTROLS.load(std::sync::atomic::Ordering::Acquire), 0, "normal completion returns every non-stack process control credit");
        let candidate = semio_framework_plugin::ArtifactStoreInitializationAuthority::take_candidate(&mut authority).expect("mounted-shaped candidate");
        assert!(semio_framework_plugin::ArtifactStoreInitializationAuthority::terminal_is_empty(&authority));
        drop(authority);
        close_raster_candidate(candidate);
    }

    #[test]
    fn raster_expired_deadline_advances_no_bounds_clone_or_mutation_owner() {
        fn expired_now() -> Option<u64> {
            Some(10)
        }
        let source = deeply_nested_raster_snapshot(8);
        let operation = semio_framework_job::OperationId(705);
        let generation = semio_framework_job::Generation(37);
        let cancel = semio_framework_job::root_cancel_token();
        let mut preview_sequence = 0;
        let mut clone = RasterSnapshotCloneAuthority::new();
        let mut digest = store::ArtifactStoreInitializationDigest::new(b"raster.expired");
        let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, 10), cancel, expired_now, &mut preview_sequence);
        assert!(!clone.step(&source, &mut digest, &mut context).expect("expired clone yields"));
        assert_eq!(clone.phase, 0);
        assert_eq!(clone.bounds.phase, 0);
        assert!(clone.value.as_ref().expect("clone shell").layers.is_empty());
        while !clone.terminal_is_empty() {
            let _ = clone.close_step(1, RASTER_OWNED_FIELD_BYTES).expect("expired clone closes through retained owner");
        }
        drop(clone);
    }

    #[test]
    fn raster_small_mutation_against_deep_snapshot_is_cursorized_and_atomic() {
        let source = deeply_nested_raster_snapshot(40);
        let mutation = RasterMutation::RenameLayer(rename_layer::mutation::RenameLayer { layer_id: "leaf".into(), new_name: "Renamed leaf".into() });
        let operation = semio_framework_job::OperationId(706);
        let generation = semio_framework_job::Generation(38);
        let cancel = semio_framework_job::root_cancel_token();
        let mut preview_sequence = 0;
        let mut authority = RasterMutationCandidateAuthority::new();
        let mut turns = 0;
        loop {
            let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
            if authority.step(&source, &mutation, &mut context).expect("cursorized Raster mutation") {
                break;
            }
            turns += 1;
            assert!(turns < 30_000);
            let source_leaf = RasterLayerLocator::node_at(&source, RasterLayerAddress { length: 41, indices: [0; RASTER_MAXIMUM_NESTED_DEPTH] }).expect("source leaf remains reachable");
            let RasterLayerNode::Pixel { name, .. } = source_leaf else { panic!("source leaf remains a pixel") };
            assert_eq!(name, "Leaf", "the published source remains unchanged while the candidate is pending");
        }
        assert!(turns > 40);
        let candidate = authority.take().expect("mutation candidate publishes atomically");
        drop(authority);
        let mut locator = RasterLayerLocator::new();
        loop {
            let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
            if locator.step(&candidate, "leaf", &mut context).expect("candidate leaf locator") {
                break;
            }
        }
        let leaf = RasterLayerLocator::node_at(&candidate, locator.found.expect("candidate leaf")).expect("candidate leaf node");
        let RasterLayerNode::Pixel { name, .. } = leaf else { panic!("leaf remains a pixel") };
        assert_eq!(name, "Renamed leaf");
        let mut retirement = RasterOwnedRetirement::new(RasterRetirementOwner::Snapshot(candidate));
        while !store::ErasedSnapshotRetirement::terminal_is_empty(&retirement) {
            let _ = store::ErasedSnapshotRetirement::close_step(&mut retirement, 1, RASTER_OWNED_FIELD_BYTES).expect("candidate closes one owner per grant");
        }
        drop(retirement);
    }

    #[test]
    fn raster_cancel_after_complete_retires_the_unclaimed_candidate_before_terminal() {
        let operation = semio_framework_job::OperationId(707);
        let generation = semio_framework_job::Generation(39);
        let envelope = store::create_document_envelope(RASTER_DOCUMENT_SCHEMA, "raster-cancel-complete", deeply_nested_raster_snapshot(24), None);
        let mut authority = RasterStoreInitializationAuthority::new(envelope, operation, generation);
        assert!(matches!(drive_raster_initializer(&mut authority, operation, generation), semio_framework_job::StepOutcome::Complete(_)));
        assert!(authority.candidate.is_some());
        semio_framework_plugin::ArtifactStoreInitializationAuthority::request_cancel(&mut authority);
        assert!(matches!(drive_raster_initializer(&mut authority, operation, generation), semio_framework_job::StepOutcome::Cancelled));
        assert!(authority.candidate.is_none());
        assert!(authority.candidate_disposer.is_none());
        assert!(semio_framework_plugin::ArtifactStoreInitializationAuthority::terminal_is_empty(&authority));
        drop(authority);
    }

    #[test]
    fn raster_retirement_uses_allocation_capacity_and_fixed_iterative_depth() {
        let mut spare = String::with_capacity(RASTER_OWNED_FIELD_BYTES);
        spare.push('x');
        let mut retirement = RasterOwnedRetirement::new(RasterRetirementOwner::String(spare));
        assert!(matches!(store::ErasedSnapshotRetirement::close_step(&mut retirement, 1, 1).expect("insufficient byte grant retains exact string"), store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }));
        assert!(!store::ErasedSnapshotRetirement::terminal_is_empty(&retirement));
        for _ in 0..16 {
            let _ = store::ErasedSnapshotRetirement::close_step(&mut retirement, 1, RASTER_OWNED_FIELD_BYTES).expect("capacity-exact string and fixed frame retire");
            if store::ErasedSnapshotRetirement::terminal_is_empty(&retirement) {
                break;
            }
        }
        assert!(store::ErasedSnapshotRetirement::terminal_is_empty(&retirement));
        drop(retirement);

        let snapshot = deeply_nested_raster_snapshot(RASTER_MAXIMUM_NESTED_DEPTH - 8);
        let mut retirement = RasterOwnedRetirement::new(RasterRetirementOwner::Snapshot(snapshot));
        let mut turns = 0;
        while !store::ErasedSnapshotRetirement::terminal_is_empty(&retirement) {
            let step = store::ErasedSnapshotRetirement::close_step(&mut retirement, 1, RASTER_OWNED_FIELD_BYTES).expect("fixed iterative Raster depth retires");
            if let store::SnapshotRetirementStep::Pending { released_items, released_bytes } = step {
                assert!(released_items <= 1);
                assert!(released_bytes <= RASTER_OWNED_FIELD_BYTES);
            }
            turns += 1;
            assert!(turns < 100_000);
        }
        drop(retirement);

        let mut value = dsl::DslValue::String(String::with_capacity(RASTER_OWNED_FIELD_BYTES));
        for _ in 0..(RASTER_MAXIMUM_NESTED_DEPTH - 8) {
            value = dsl::DslValue::Array(vec![value]);
        }
        let mut retirement = RasterOwnedRetirement::new(RasterRetirementOwner::Value(value));
        let mut turns = 0;
        while !store::ErasedSnapshotRetirement::terminal_is_empty(&retirement) {
            let step = store::ErasedSnapshotRetirement::close_step(&mut retirement, 1, RASTER_OWNED_FIELD_BYTES).expect("deep fixed value stack retires");
            if let store::SnapshotRetirementStep::Pending { released_items, released_bytes } = step {
                assert!(released_items <= 1);
                assert!(released_bytes <= RASTER_OWNED_FIELD_BYTES);
            }
            turns += 1;
            assert!(turns < 100_000);
        }
        drop(retirement);
    }

    #[test]
    fn raster_nested_owner_item_and_byte_capacity_plus_one_reject_before_clone() {
        let mut totals = RasterOwnerTotals::new();
        assert!(totals.add(RASTER_MAXIMUM_NESTED_ITEMS, 0, RASTER_MAXIMUM_NESTED_ITEMS, 0).is_ok());
        assert_eq!(totals.add(1, 0, 0, 0), Err("raster-store.preflight-item-capacity"));

        let mut totals = RasterOwnerTotals::new();
        assert!(totals.add(0, RASTER_MAXIMUM_NESTED_BYTES, 0, RASTER_MAXIMUM_NESTED_BYTES).is_ok());
        assert_eq!(totals.add(0, 1, 0, 0), Err("raster-store.preflight-byte-capacity"));
    }

    #[test]
    fn raster_retirement_page_credit_is_claimed_before_allocation_and_returned_with_backing() {
        let baseline = RASTER_RETIREMENT_PROCESS_PAGES.load(std::sync::atomic::Ordering::Acquire);
        let mut retirement = RasterOwnedRetirement::new(RasterRetirementOwner::Value(dsl::DslValue::Array(vec![dsl::DslValue::String("owned".into())])));
        assert!(matches!(store::ErasedSnapshotRetirement::close_step(&mut retirement, 1, RASTER_CONTROL_BACKING_BYTES).expect("nested owner stages one push"), store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }));
        assert!(matches!(
            store::ErasedSnapshotRetirement::close_step(&mut retirement, 1, RASTER_CONTROL_BACKING_BYTES).expect("page credit is claimed before allocation"),
            store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }
        ));
        assert_eq!(RASTER_RETIREMENT_PROCESS_PAGES.load(std::sync::atomic::Ordering::Acquire), baseline + 1);
        assert_eq!(retirement.pending_page_credit, Some(0));
        assert!(retirement.pages[0].is_none());
        assert!(matches!(
            store::ErasedSnapshotRetirement::close_step(&mut retirement, 1, RASTER_CONTROL_BACKING_BYTES).expect("credited page allocation is a distinct transition"),
            store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }
        ));
        assert!(retirement.pending_page_credit.is_none());
        assert!(retirement.page_credits[0]);
        assert!(retirement.pages[0].is_some());
        while !store::ErasedSnapshotRetirement::terminal_is_empty(&retirement) {
            let step = store::ErasedSnapshotRetirement::close_step(&mut retirement, 1, RASTER_CONTROL_BACKING_BYTES).expect("credited page retires incrementally");
            if let store::SnapshotRetirementStep::Pending { released_items, released_bytes } = step {
                assert!(released_items <= 1);
                assert!(released_bytes <= RASTER_CONTROL_BACKING_BYTES);
            }
        }
        assert_eq!(RASTER_RETIREMENT_PROCESS_PAGES.load(std::sync::atomic::Ordering::Acquire), baseline);
        drop(retirement);
    }

    #[test]
    fn raster_owned_map_removal_returns_exact_pair_and_populated_drop_refuses() {
        let mut map = RasterOwnedMap::new();
        let key = String::from("exact-key");
        let key_pointer = key.as_ptr();
        map.insert(key, 7_u8).expect("one exact map entry");
        let mut removed = map.remove_entry("exact-key").expect("pair-returning removal");
        let (removed_key, removed_value) = removed.take();
        assert_eq!(removed_key.as_ptr(), key_pointer);
        assert_eq!(removed_value, 7);
        drop(removed_key);
        while let Some(page) = map.take_empty_page_backing() {
            page.release();
        }
        drop(map);

        let result = std::panic::catch_unwind(|| {
            let mut populated = RasterOwnedMap::new();
            populated.insert(String::from("must-retire"), 9_u8).expect("one populated page");
            drop(populated);
        });
        assert!(result.is_err(), "populated Raster map ordinary Drop must fail closed");
    }

    #[test]
    fn raster_empty_asset_map_retirement_has_no_hidden_allocation_release() {
        let snapshot = RasterSnapshot { schema: String::new(), id: String::new(), title: None, layers: Vec::new(), assets: RasterOwnedMap::new() };
        let mut retirement = RasterOwnedRetirement::new(RasterRetirementOwner::Snapshot(snapshot));
        let layers = store::ErasedSnapshotRetirement::close_step(&mut retirement, 1, RASTER_OWNED_FIELD_BYTES).expect("empty layer vector closes");
        assert!(matches!(layers, store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }));
        let assets = store::ErasedSnapshotRetirement::close_step(&mut retirement, 1, RASTER_OWNED_FIELD_BYTES).expect("empty fixed-page map shell closes allocation-free");
        assert!(matches!(assets, store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }));
        while !store::ErasedSnapshotRetirement::terminal_is_empty(&retirement) {
            let _ = store::ErasedSnapshotRetirement::close_step(&mut retirement, 1, RASTER_OWNED_FIELD_BYTES).expect("empty snapshot reaches terminal");
        }
        drop(retirement);
    }

    #[test]
    fn raster_owned_map_cap_plus_one_returns_exact_owner_and_populated_pages_retire_explicitly() {
        let mut assets = RasterOwnedMap::new();
        for index in 0..crate::artifacts::raster::RASTER_OWNED_MAP_CAPACITY {
            assets
                .insert(
                    format!("asset-{index:02}"),
                    store::ArtifactChild::new(
                        format!("child-{index:02}"),
                        store::os_io::ArtifactRef { artifact_id: format!("artifact-{index:02}"), dialect: store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "image".into() } },
                    ),
                )
                .expect("fixed Raster map admits its exact item capacity");
        }
        let rejected_key = String::from("asset-overflow");
        let rejected_child_id = String::from("child-overflow");
        let key_pointer = rejected_key.as_ptr();
        let child_pointer = rejected_child_id.as_ptr();
        let rejected = assets
            .insert(
                rejected_key,
                store::ArtifactChild::new(
                    rejected_child_id,
                    store::os_io::ArtifactRef { artifact_id: "overflow".into(), dialect: store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "image".into() } },
                ),
            )
            .expect_err("fixed Raster map rejects capacity plus one");
        assert_eq!(rejected.key.as_ptr(), key_pointer);
        assert_eq!(rejected.value.child_id.as_ptr(), child_pointer);

        let (old_key_pointer, old_child_pointer) = {
            let (key, child) = assets.entry_at(0).expect("first admitted Raster map entry");
            (key.as_ptr(), child.child_id.as_ptr())
        };
        let mut replacement_key = String::with_capacity(64);
        replacement_key.push_str("asset-00");
        let replacement_key_pointer = replacement_key.as_ptr();
        let replacement_child_id = String::from("replacement-child");
        let replacement_child_pointer = replacement_child_id.as_ptr();
        let mut replaced = match assets
            .insert_pre_admitted(
                replacement_key,
                store::ArtifactChild::new(
                    replacement_child_id,
                    store::os_io::ArtifactRef { artifact_id: "replacement-artifact".into(), dialect: store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "image".into() } },
                ),
            )
            .expect("replacement preserves fixed capacity")
        {
            RasterOwnedMapInsert::Replaced(previous) => previous,
            RasterOwnedMapInsert::Inserted => panic!("replacement returns the exact displaced pair"),
        };
        let (previous_key, previous_child) = replaced.take();
        assert_eq!(previous_key.as_ptr(), old_key_pointer);
        assert_eq!(previous_child.child_id.as_ptr(), old_child_pointer);
        let (installed_key, installed_child) = assets.entry_at(0).expect("replacement remains in stable order");
        assert_eq!(installed_key.as_ptr(), replacement_key_pointer);
        assert_eq!(installed_child.child_id.as_ptr(), replacement_child_pointer);
        let mut displaced = RasterOwnedRetirement::new(RasterRetirementOwner::AssetEntry { key: previous_key, child: Some(previous_child) });
        while !store::ErasedSnapshotRetirement::terminal_is_empty(&displaced) {
            let step = store::ErasedSnapshotRetirement::close_step(&mut displaced, 1, RASTER_OWNED_FIELD_BYTES).expect("displaced replacement owner closes exactly");
            if let store::SnapshotRetirementStep::Pending { released_items, released_bytes } = step {
                assert!(released_items <= 1);
                assert!(released_bytes <= RASTER_OWNED_FIELD_BYTES);
            }
        }
        drop(displaced);

        let snapshot = RasterSnapshot { schema: String::new(), id: String::new(), title: None, layers: Vec::new(), assets };
        let mut retirement = RasterOwnedRetirement::new(RasterRetirementOwner::Snapshot(snapshot));
        let mut page_backings = 0;
        while !store::ErasedSnapshotRetirement::terminal_is_empty(&retirement) {
            if let store::SnapshotRetirementStep::Pending { released_items, released_bytes } = store::ErasedSnapshotRetirement::close_step(&mut retirement, 1, RASTER_OWNED_FIELD_BYTES).expect("populated owned map retires one exact owner") {
                assert!(released_items <= 1);
                assert!(released_bytes <= RASTER_OWNED_FIELD_BYTES);
                if released_bytes == RasterOwnedMap::<RasterAssetChild>::conservative_page_credit_bytes() {
                    page_backings += 1;
                }
            }
        }
        assert_eq!(page_backings, crate::artifacts::raster::RASTER_OWNED_MAP_CAPACITY / crate::artifacts::raster::RASTER_OWNED_MAP_PAGE_CAPACITY);
        drop(retirement);
    }

    #[test]
    fn raster_observed_capacity_and_combined_retirement_depth_are_exact() {
        let mut source = String::with_capacity(64);
        source.push_str("observed");
        let candidate = raster_clone_owned_string(&source).expect("fixed-slice String construction is exact");
        assert_eq!(candidate.capacity(), candidate.len());

        let mut totals = RasterOwnerTotals::new();
        totals.add(0, 0, 0, 8).expect("base candidate credit");
        totals.observe_candidate_capacity(4, 7, 2).expect("allocator over-capacity is observed and admitted");
        assert_eq!(totals.candidate_bytes, 14);
        assert_eq!(RasterOwnerTotals::validate_control_backing_count(RASTER_MAXIMUM_CONTROL_BACKINGS), Ok(()));
        assert_eq!(RasterOwnerTotals::validate_control_backing_count(RASTER_MAXIMUM_CONTROL_BACKINGS + 1), Err("raster-store.control-backing-capacity"));
        assert_eq!(RASTER_MAXIMUM_CONTROL_BACKINGS, 64);
        assert_eq!(raster_retirement_frame_requirement(RASTER_MAXIMUM_NESTED_DEPTH, RASTER_MAXIMUM_NESTED_DEPTH), Ok(RASTER_RETIREMENT_ADMITTED_FRAME_CAPACITY));
        assert_eq!(raster_retirement_frame_requirement(RASTER_MAXIMUM_NESTED_DEPTH, RASTER_MAXIMUM_NESTED_DEPTH + 1), Err("raster-store.preflight-combined-depth"));
    }

    #[test]
    fn raster_box_and_arc_control_backings_require_and_report_fixed_credit() {
        assert!(std::mem::size_of::<RasterOwnedRetirement>() <= RASTER_CONTROL_BACKING_BYTES);
        assert!(std::mem::size_of::<RasterRetirementFramePage>() <= RASTER_CONTROL_BACKING_BYTES);
        assert!(std::mem::size_of::<RasterSnapshotCloneAuthority>() <= RASTER_CONTROL_BACKING_BYTES);
        assert!(std::mem::size_of::<RasterLayerCloneAuthority>() <= RASTER_CONTROL_BACKING_BYTES);
        assert!(std::mem::size_of::<RasterDslValueCloneAuthority>() <= RASTER_CONTROL_BACKING_BYTES);
        let layer =
            Box::new(RasterLayerNode::Pixel { id: String::new(), name: String::new(), visible: true, opacity: 1.0, blend_mode: String::new(), transform: RasterTransform::default(), mask: None, width: Some(1), height: Some(1), image_key: None });
        let mut boxed = RasterOwnedRetirement::new(RasterRetirementOwner::BoxedLayer(Some(layer)));
        assert_eq!(boxed.control.as_ref().map(|credit| (credit.held_items, credit.held_bytes)), Some((1, RASTER_CONTROL_BACKING_BYTES)));
        assert!(matches!(
            store::ErasedSnapshotRetirement::close_step(&mut boxed, 1, RASTER_CONTROL_BACKING_BYTES - 1).expect("insufficient Box backing credit retains exact owner"),
            store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }
        ));
        assert!(matches!(
            store::ErasedSnapshotRetirement::close_step(&mut boxed, 1, RASTER_CONTROL_BACKING_BYTES).expect("fixed Box backing credit releases exact control owner"),
            store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: RASTER_CONTROL_BACKING_BYTES }
        ));
        while !store::ErasedSnapshotRetirement::terminal_is_empty(&boxed) {
            let _ = store::ErasedSnapshotRetirement::close_step(&mut boxed, 1, RASTER_OWNED_FIELD_BYTES).expect("boxed layer payload retires after its control backing");
        }
        assert!(boxed.control.is_none(), "standalone Box control credit is returned before terminal-empty");
        drop(boxed);

        let snapshot = std::sync::Arc::new(RasterSnapshot { schema: String::new(), id: String::new(), title: None, layers: Vec::new(), assets: RasterOwnedMap::new() });
        let mut root = store::SnapshotRetirementFactory::retire(&RasterSnapshotRetirementFactory, snapshot);
        assert!(matches!(root.close_step(1, RASTER_CONTROL_BACKING_BYTES - 1).expect("insufficient Arc credit retains root"), store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }));
        assert!(matches!(root.close_step(1, RASTER_CONTROL_BACKING_BYTES).expect("Arc control backing is reported before root payload"), store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: RASTER_CONTROL_BACKING_BYTES }));
        while !root.terminal_is_empty() {
            let _ = root.close_step(1, RASTER_OWNED_FIELD_BYTES).expect("Arc root retires through exact retained owner");
        }
        assert!(root.terminal_is_empty(), "standalone Arc and inner Box control credits return before terminal-empty");
        drop(root);
    }

    #[test]
    fn raster_standalone_control_max_plus_one_returns_exact_owner_and_resumes_after_full_saturation() {
        let _guard = RASTER_STANDALONE_RETIREMENT_TEST_LOCK.lock().expect("Raster standalone retirement test lock");
        assert_eq!(RASTER_STANDALONE_PROCESS_CONTROLS.load(std::sync::atomic::Ordering::Acquire), 0);
        let mut saturated = Vec::with_capacity(RASTER_STANDALONE_PROCESS_CONTROL_CAPACITY);
        for index in 0..RASTER_STANDALONE_PROCESS_CONTROL_CAPACITY {
            let retirement = RasterOwnedRetirement::new(RasterRetirementOwner::String(format!("held-{index}")));
            assert_eq!(retirement.control.as_ref().map(|credit| (credit.held_items, credit.held_bytes)), Some((1, RASTER_CONTROL_BACKING_BYTES)));
            saturated.push(retirement);
        }
        assert_eq!(RASTER_STANDALONE_PROCESS_CONTROLS.load(std::sync::atomic::Ordering::Acquire), RASTER_STANDALONE_PROCESS_CONTROL_CAPACITY);

        let plus_one_owner = String::from("exact-plus-one-owner");
        let plus_one_pointer = plus_one_owner.as_ptr();
        let construction = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| RasterOwnedRetirement::new(RasterRetirementOwner::String(plus_one_owner))));
        let mut plus_one = construction.expect("saturated standalone construction retains rather than panics");
        assert!(plus_one.control.is_none());
        let retained_pointer = match plus_one.root.as_ref().and_then(|frame| frame.owner.as_ref()) {
            Some(RasterRetirementOwner::String(value)) => value.as_ptr(),
            _ => panic!("saturated standalone retirement retains the exact producer owner"),
        };
        assert_eq!(retained_pointer, plus_one_pointer);
        assert!(matches!(
            store::ErasedSnapshotRetirement::close_step(&mut plus_one, 1, RASTER_OWNED_FIELD_BYTES).expect("full saturation is a retained pending result"),
            store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }
        ));
        assert!(plus_one.control.is_none());

        let mut returned = saturated.pop().expect("one saturated control is returned");
        close_raster_retirement(&mut returned);
        assert!(store::ErasedSnapshotRetirement::terminal_is_empty(&returned));
        drop(returned);
        assert_eq!(RASTER_STANDALONE_PROCESS_CONTROLS.load(std::sync::atomic::Ordering::Acquire), RASTER_STANDALONE_PROCESS_CONTROL_CAPACITY - 1);

        assert!(matches!(store::ErasedSnapshotRetirement::close_step(&mut plus_one, 1, 0).expect("plus-one owner resumes into the returned exact control"), store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }));
        assert_eq!(plus_one.control.as_ref().map(|credit| (credit.held_items, credit.held_bytes)), Some((1, RASTER_CONTROL_BACKING_BYTES)));
        let resumed_pointer = match plus_one.root.as_ref().and_then(|frame| frame.owner.as_ref()) {
            Some(RasterRetirementOwner::String(value)) => value.as_ptr(),
            _ => panic!("resumed standalone retirement still retains the exact producer owner"),
        };
        assert_eq!(resumed_pointer, plus_one_pointer);
        close_raster_retirement(&mut plus_one);
        assert!(store::ErasedSnapshotRetirement::terminal_is_empty(&plus_one));
        drop(plus_one);
        for retirement in &mut saturated {
            close_raster_retirement(retirement);
            assert!(store::ErasedSnapshotRetirement::terminal_is_empty(retirement));
        }
        drop(saturated);
        assert_eq!(RASTER_STANDALONE_PROCESS_CONTROLS.load(std::sync::atomic::Ordering::Acquire), 0, "held standalone control credits equal returned credits");
        assert_eq!(RASTER_RETIREMENT_PROCESS_PAGES.load(std::sync::atomic::Ordering::Acquire), 0, "terminal standalone saturation leaves no page credit");
    }

    #[test]
    fn raster_arc_factory_full_saturation_preserves_exact_producer_through_every_control_phase() {
        let _guard = RASTER_STANDALONE_RETIREMENT_TEST_LOCK.lock().expect("Raster standalone retirement test lock");
        assert_eq!(RASTER_STANDALONE_PROCESS_CONTROLS.load(std::sync::atomic::Ordering::Acquire), 0);
        let mut saturated = Vec::with_capacity(RASTER_STANDALONE_PROCESS_CONTROL_CAPACITY);
        for index in 0..RASTER_STANDALONE_PROCESS_CONTROL_CAPACITY {
            saturated.push(RasterOwnedRetirement::new(RasterRetirementOwner::String(format!("arc-held-{index}"))));
        }
        let producer = std::sync::Arc::new(RasterSnapshot { schema: "arc-owner".into(), id: String::new(), title: None, layers: Vec::new(), assets: RasterOwnedMap::new() });
        let producer_pointer = std::sync::Arc::as_ptr(&producer);
        let producer_witness = std::sync::Arc::downgrade(&producer);
        let construction = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| store::SnapshotRetirementFactory::retire(&RasterSnapshotRetirementFactory, producer)));
        let mut root = construction.expect("saturated Arc factory retains rather than panics");
        assert_eq!(std::sync::Arc::as_ptr(&producer_witness.upgrade().expect("saturated Arc owner remains alive")), producer_pointer);
        assert!(matches!(root.close_step(1, RASTER_OWNED_FIELD_BYTES).expect("full Arc control saturation retains exact owner"), store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }));

        let mut first_return = saturated.pop().expect("one root control is returned");
        close_raster_retirement(&mut first_return);
        drop(first_return);
        assert!(matches!(root.close_step(1, 0).expect("Arc root claims the returned control without consuming its producer"), store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }));
        assert_eq!(std::sync::Arc::as_ptr(&producer_witness.upgrade().expect("admitted Arc owner remains exact before unwrap")), producer_pointer);
        assert!(matches!(root.close_step(1, RASTER_OWNED_FIELD_BYTES).expect("Arc allocation transfers into the retained value phase"), store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: RASTER_CONTROL_BACKING_BYTES }));
        assert!(matches!(root.close_step(1, RASTER_OWNED_FIELD_BYTES).expect("inner Box retirement construction remains retained at saturation"), store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }));
        assert!(matches!(root.close_step(1, RASTER_OWNED_FIELD_BYTES).expect("saturated inner Box retirement yields without owner loss"), store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }));

        let mut second_return = saturated.pop().expect("one inner Box control is returned");
        close_raster_retirement(&mut second_return);
        drop(second_return);
        close_raster_retirement(root.as_mut());
        assert!(root.terminal_is_empty(), "Arc owner, inner Box, and root control all reach terminal-empty");
        drop(root);
        assert!(producer_witness.upgrade().is_none(), "the exact producer allocation is retired once after every control phase");
        for retirement in &mut saturated {
            close_raster_retirement(retirement);
        }
        drop(saturated);
        assert_eq!(RASTER_STANDALONE_PROCESS_CONTROLS.load(std::sync::atomic::Ordering::Acquire), 0, "Arc and Box held controls equal returned controls");
        assert_eq!(RASTER_RETIREMENT_PROCESS_PAGES.load(std::sync::atomic::Ordering::Acquire), 0, "Arc saturation leaves the retirement page process empty");
    }

    #[test]
    fn raster_populated_dsl_materialization_max_plus_one_nested_cancel_fault_panic_and_close_are_exact() {
        let _guard = RASTER_STANDALONE_RETIREMENT_TEST_LOCK.lock().expect("Raster standalone retirement test lock");
        let mut params = RasterOwnedMap::new();
        let mut first_key_pointer = std::ptr::null();
        for index in 0..crate::artifacts::raster::RASTER_OWNED_MAP_CAPACITY {
            let key = format!("key-{index:02}");
            if index == 0 {
                first_key_pointer = key.as_ptr();
            }
            let value = dsl::DslValue::Object(vec![("nested".into(), dsl::DslValue::Array(vec![dsl::DslValue::String(format!("value-{index}")), dsl::DslValue::Object(vec![("leaf".into(), dsl::DslValue::uint(index as u64))])]))]);
            params.insert(key, value).expect("maximum populated DSL map remains exactly page admitted");
        }
        assert_eq!(params.len(), crate::artifacts::raster::RASTER_OWNED_MAP_CAPACITY);
        assert_eq!(params.entry_at(0).expect("first exact map owner remains installed").0.as_ptr(), first_key_pointer);

        let plus_one_key = String::from("key-plus-one");
        let plus_one_key_pointer = plus_one_key.as_ptr();
        let plus_one_value = dsl::DslValue::String("plus-one-value".into());
        let rejected = params.insert(plus_one_key, plus_one_value).expect_err("capacity plus one returns both exact owners");
        assert_eq!(rejected.key.as_ptr(), plus_one_key_pointer);
        assert_eq!(rejected.reason, "raster-map.item-capacity");
        let mut rejected_retirement = RasterOwnedRetirement::new(RasterRetirementOwner::ValueEntry { key: rejected.key, value: Some(rejected.value) });

        let output = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| dsl::DslField::to_value(&params)));
        assert!(output.is_err(), "populated ordinary DSL output is rejected before any whole-map result exists");
        assert_eq!(params.len(), crate::artifacts::raster::RASTER_OWNED_MAP_CAPACITY);
        assert_eq!(params.entry_at(0).expect("panic keeps the first exact key/value/page owner installed").0.as_ptr(), first_key_pointer);
        assert!(matches!(
            store::ErasedSnapshotRetirement::close_step(&mut rejected_retirement, 0, 0).expect("cancellation-shaped zero grant preserves the rejected pair"),
            store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }
        ));
        close_raster_retirement(&mut rejected_retirement);
        drop(rejected_retirement);

        let populated_input = dsl::FieldValue::Map(vec![("forbidden".into(), dsl::FieldValue::Text("owner".into()))]);
        assert!(<RasterOwnedMap<dsl::DslValue> as dsl::DslField>::from_value(&populated_input).is_err(), "populated DSL input faults before page or semantic owner admission");
        let layer = RasterLayerNode::Adjustment { id: "dsl-output".into(), name: "DSL Output".into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), adjustment_kind: "nested".into(), params };
        let snapshot = RasterSnapshot { schema: String::new(), id: String::new(), title: None, layers: vec![layer], assets: RasterOwnedMap::new() };
        let mut retirement = RasterOwnedRetirement::new(RasterRetirementOwner::Snapshot(snapshot));
        assert!(matches!(store::ErasedSnapshotRetirement::close_step(&mut retirement, 0, 0).expect("cancelled populated output keeps every exact owner"), store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }));
        close_raster_retirement(&mut retirement);
        assert!(store::ErasedSnapshotRetirement::terminal_is_empty(&retirement));
        drop(retirement);
        assert_eq!(RASTER_STANDALONE_PROCESS_CONTROLS.load(std::sync::atomic::Ordering::Acquire), 0, "populated DSL rejection and close return every standalone control");
        assert_eq!(RASTER_RETIREMENT_PROCESS_PAGES.load(std::sync::atomic::Ordering::Acquire), 0, "populated DSL rejection and close return every stack page credit");
    }

    #[test]
    fn raster_populated_serde_output_max_plus_one_nested_cancel_fault_panic_and_close_are_exact() {
        let _guard = RASTER_STANDALONE_RETIREMENT_TEST_LOCK.lock().expect("Raster standalone retirement test lock");
        let mut params = RasterOwnedMap::new();
        let mut first_key_pointer = std::ptr::null();
        for index in 0..crate::artifacts::raster::RASTER_OWNED_MAP_CAPACITY {
            let key = format!("serde-key-{index:02}");
            if index == 0 {
                first_key_pointer = key.as_ptr();
            }
            let value = dsl::DslValue::Object(vec![("nested".into(), dsl::DslValue::Array(vec![dsl::DslValue::String(format!("serde-value-{index}")), dsl::DslValue::Object(vec![("leaf".into(), dsl::DslValue::uint(index as u64))])]))]);
            params.insert(key, value).expect("maximum populated serde map remains exactly page admitted");
        }
        assert_eq!(params.len(), crate::artifacts::raster::RASTER_OWNED_MAP_CAPACITY);
        assert_eq!(params.entry_at(0).expect("first serde owner remains installed").0.as_ptr(), first_key_pointer);

        let plus_one_key = String::from("serde-key-plus-one");
        let plus_one_key_pointer = plus_one_key.as_ptr();
        let rejected = params.insert(plus_one_key, dsl::DslValue::String("serde-plus-one-value".into())).expect_err("serde capacity plus one returns both exact owners");
        assert_eq!(rejected.key.as_ptr(), plus_one_key_pointer);
        assert_eq!(rejected.reason, "raster-map.item-capacity");
        let mut rejected_retirement = RasterOwnedRetirement::new(RasterRetirementOwner::ValueEntry { key: rejected.key, value: Some(rejected.value) });
        assert!(matches!(store::ErasedSnapshotRetirement::close_step(&mut rejected_retirement, 0, 0).expect("cancelled serde output preserves the rejected pair"), store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }));
        close_raster_retirement(&mut rejected_retirement);
        drop(rejected_retirement);

        let layer = RasterLayerNode::Adjustment { id: "serde-output".into(), name: "Serde Output".into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), adjustment_kind: "nested".into(), params };
        let output = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| serde_json::to_vec(&layer)));
        assert!(matches!(output, Ok(Err(_))), "public populated serde output faults before any whole-map result and contains panic");
        let params = match &layer {
            RasterLayerNode::Adjustment { params, .. } => params,
            _ => unreachable!("serde fixture remains an adjustment"),
        };
        assert_eq!(params.len(), crate::artifacts::raster::RASTER_OWNED_MAP_CAPACITY);
        assert_eq!(params.entry_at(0).expect("serde fault keeps the first exact owner installed").0.as_ptr(), first_key_pointer);

        let snapshot = RasterSnapshot { schema: String::new(), id: String::new(), title: None, layers: vec![layer], assets: RasterOwnedMap::new() };
        let mut retirement = RasterOwnedRetirement::new(RasterRetirementOwner::Snapshot(snapshot));
        assert!(matches!(store::ErasedSnapshotRetirement::close_step(&mut retirement, 0, 0).expect("zero-grant serde cancellation keeps every exact owner"), store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }));
        close_raster_retirement(&mut retirement);
        assert!(store::ErasedSnapshotRetirement::terminal_is_empty(&retirement));
        drop(retirement);
        assert_eq!(RASTER_STANDALONE_PROCESS_CONTROLS.load(std::sync::atomic::Ordering::Acquire), 0, "populated serde fault and close return every standalone control");
        assert_eq!(RASTER_RETIREMENT_PROCESS_PAGES.load(std::sync::atomic::Ordering::Acquire), 0, "populated serde fault and close return every stack page credit");
        assert_eq!(RASTER_INITIALIZATION_PROCESS_CONTROLS.load(std::sync::atomic::Ordering::Acquire), 0, "populated serde fault never claims or leaks an initialization process control");
    }

    #[test]
    fn raster_populated_snapshot_output_max_plus_one_nested_cancel_fault_panic_and_close_are_exact() {
        let _guard = RASTER_STANDALONE_RETIREMENT_TEST_LOCK.lock().expect("Raster standalone retirement test lock");
        let mut deepest = dsl::DslValue::String("deep-output-owner".into());
        for _ in 1..RASTER_MAXIMUM_NESTED_DEPTH {
            deepest = dsl::DslValue::Array(vec![deepest]);
        }
        let mut params = RasterOwnedMap::new();
        let mut first_param_pointer = std::ptr::null();
        for index in 0..crate::artifacts::raster::RASTER_OWNED_MAP_CAPACITY {
            let key = format!("output-param-{index:02}");
            if index == 0 {
                first_param_pointer = key.as_ptr();
            }
            let value = if index == 0 { std::mem::replace(&mut deepest, dsl::DslValue::Null) } else { dsl::DslValue::String(format!("output-value-{index}")) };
            params.insert(key, value).expect("maximum populated output parameter map remains exactly admitted");
        }
        let plus_one_param_key = String::from("output-param-plus-one");
        let plus_one_param_pointer = plus_one_param_key.as_ptr();
        let plus_one_param_value = String::from("rejected-output-value");
        let plus_one_param_value_pointer = plus_one_param_value.as_ptr();
        let rejected_param = params.insert(plus_one_param_key, dsl::DslValue::String(plus_one_param_value)).expect_err("output parameter capacity plus one returns both exact owners");
        assert_eq!(rejected_param.key.as_ptr(), plus_one_param_pointer);
        let rejected_param_value = match &rejected_param.value {
            dsl::DslValue::String(value) => value,
            _ => unreachable!("rejected output parameter remains the exact string variant"),
        };
        assert_eq!(rejected_param_value.as_ptr(), plus_one_param_value_pointer, "rejected output parameter returns the exact value allocation");
        assert_eq!(rejected_param.reason, "raster-map.item-capacity");
        let mut rejected_param_retirement = RasterOwnedRetirement::new(RasterRetirementOwner::ValueEntry { key: rejected_param.key, value: Some(rejected_param.value) });
        assert!(matches!(
            store::ErasedSnapshotRetirement::close_step(&mut rejected_param_retirement, 0, 0).expect("zero-grant output parameter close preserves the rejected pair"),
            store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }
        ));
        close_raster_retirement(&mut rejected_param_retirement);
        drop(rejected_param_retirement);

        let mut assets = RasterOwnedMap::new();
        let mut first_asset_pointer = std::ptr::null();
        for index in 0..crate::artifacts::raster::RASTER_OWNED_MAP_CAPACITY {
            let key = format!("output-asset-{index:02}");
            if index == 0 {
                first_asset_pointer = key.as_ptr();
            }
            assets
                .insert(
                    key,
                    store::ArtifactChild::new(
                        format!("output-child-{index:02}"),
                        store::os_io::ArtifactRef { artifact_id: format!("output-artifact-{index:02}"), dialect: store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "image".into() } },
                    ),
                )
                .expect("maximum populated output asset map remains exactly admitted");
        }
        let plus_one_asset_key = String::from("output-asset-plus-one");
        let plus_one_asset_pointer = plus_one_asset_key.as_ptr();
        let plus_one_asset_child = store::ArtifactChild::new(
            "output-child-plus-one".into(),
            store::os_io::ArtifactRef { artifact_id: "output-artifact-plus-one".into(), dialect: store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "image".into() } },
        );
        let plus_one_asset_child_pointer = plus_one_asset_child.child_id.as_ptr();
        let rejected_asset = assets.insert(plus_one_asset_key, plus_one_asset_child).expect_err("output asset capacity plus one returns both exact owners");
        assert_eq!(rejected_asset.key.as_ptr(), plus_one_asset_pointer);
        assert_eq!(rejected_asset.value.child_id.as_ptr(), plus_one_asset_child_pointer, "rejected output asset returns the exact child allocation");
        assert_eq!(rejected_asset.reason, "raster-map.item-capacity");
        let mut rejected_asset_retirement = RasterOwnedRetirement::new(RasterRetirementOwner::AssetEntry { key: rejected_asset.key, child: Some(rejected_asset.value) });
        assert!(matches!(
            store::ErasedSnapshotRetirement::close_step(&mut rejected_asset_retirement, 0, 0).expect("zero-grant mounted output close preserves the rejected child pair"),
            store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }
        ));
        close_raster_retirement(&mut rejected_asset_retirement);
        drop(rejected_asset_retirement);

        let layer = RasterLayerNode::Adjustment { id: "retained-output".into(), name: "Retained Output".into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), adjustment_kind: "deep".into(), params };
        let snapshot = RasterSnapshot { schema: String::new(), id: String::new(), title: None, layers: vec![layer], assets };
        assert_eq!(snapshot.require_empty_output_shell(), Err(crate::artifacts::raster::schema::snapshot::RASTER_POPULATED_OUTPUT_ERROR));
        let panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| snapshot.require_empty_output_shell().expect(crate::artifacts::raster::schema::snapshot::RASTER_POPULATED_OUTPUT_ERROR)));
        assert!(panic.is_err(), "public DSL panic path contains the fail-closed populated output before allocation");
        let params = match &snapshot.layers[0] {
            RasterLayerNode::Adjustment { params, .. } => params,
            _ => unreachable!("output fixture remains an adjustment"),
        };
        assert_eq!(params.entry_at(0).expect("fault and panic retain the first parameter owner").0.as_ptr(), first_param_pointer);
        assert_eq!(snapshot.assets.entry_at(0).expect("all mounted exporters retain the first asset owner").0.as_ptr(), first_asset_pointer);

        let mut retirement = RasterOwnedRetirement::new(RasterRetirementOwner::Snapshot(snapshot));
        assert!(matches!(store::ErasedSnapshotRetirement::close_step(&mut retirement, 0, 0).expect("cancelled output preserves every populated snapshot owner"), store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }));
        close_raster_retirement(&mut retirement);
        assert!(store::ErasedSnapshotRetirement::terminal_is_empty(&retirement));
        drop(retirement);
        assert_eq!(RASTER_STANDALONE_PROCESS_CONTROLS.load(std::sync::atomic::Ordering::Acquire), 0, "populated output rejection and close return every standalone control");
        assert_eq!(RASTER_RETIREMENT_PROCESS_PAGES.load(std::sync::atomic::Ordering::Acquire), 0, "populated output rejection and close return every stack page credit");
        assert_eq!(RASTER_INITIALIZATION_PROCESS_CONTROLS.load(std::sync::atomic::Ordering::Acquire), 0, "populated output rejection never claims or leaks an initialization control");
    }

    #[test]
    fn raster_maximum_combined_layer_and_value_depth_retires_to_terminal() {
        let mut value = dsl::DslValue::String("terminal".into());
        for _ in 1..RASTER_MAXIMUM_NESTED_DEPTH {
            value = dsl::DslValue::Array(vec![value]);
        }
        let mut params = RasterOwnedMap::new();
        params.insert("deep".into(), value).expect("one fixed parameter page");
        let mut layer = RasterLayerNode::Adjustment { id: "adjustment".into(), name: "Adjustment".into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), adjustment_kind: "levels".into(), params };
        for index in 1..RASTER_MAXIMUM_NESTED_DEPTH {
            layer = RasterLayerNode::Group { id: format!("group-{index}"), name: "Group".into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, children: vec![layer] };
        }
        let snapshot = RasterSnapshot { schema: String::new(), id: String::new(), title: None, layers: vec![layer], assets: RasterOwnedMap::new() };
        let mut retirement = RasterOwnedRetirement::new(RasterRetirementOwner::Snapshot(snapshot));
        for _ in 0..200_000 {
            if store::ErasedSnapshotRetirement::terminal_is_empty(&retirement) {
                drop(retirement);
                return;
            }
            let step = store::ErasedSnapshotRetirement::close_step(&mut retirement, 1, RASTER_OWNED_FIELD_BYTES).expect("maximum combined fixed retirement stack remains sufficient");
            if let store::SnapshotRetirementStep::Pending { released_items, released_bytes } = step {
                assert!(released_items <= 1);
                assert!(released_bytes <= RASTER_OWNED_FIELD_BYTES);
            }
        }
        panic!("maximum combined Raster owner did not reach terminal-empty");
    }

    #[test]
    fn raster_nested_snapshot_and_child_handles_retire_one_owner_per_grant() {
        let mut params = RasterOwnedMap::new();
        params.insert("nested".repeat(16), dsl::DslValue::Object(vec![("array".repeat(16), dsl::DslValue::Array(vec![dsl::DslValue::String("payload".repeat(64)), dsl::DslValue::String("tail".into())]))]));
        let adjustment = RasterLayerNode::Adjustment { id: "adjustment".into(), name: "Adjustment".into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), adjustment_kind: "levels".into(), params };
        let mut snapshot = empty_raster_document();
        snapshot.title = Some("Nested raster".into());
        snapshot.layers.push(RasterLayerNode::Group { id: "group".into(), name: "Group".into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, children: vec![adjustment] });
        snapshot.assets.insert(
            "asset".into(),
            store::ArtifactChild::new("child".into(), store::os_io::ArtifactRef { artifact_id: "artifact".into(), dialect: store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "image".into() } }),
        );
        let mut retirement = store::ArtifactOwnedValueRetirementFactory::retire_owned(&RasterSnapshotRetirementFactory, snapshot);
        for _ in 0..10_000 {
            match retirement.close_step(1, RASTER_OWNED_FIELD_BYTES).expect("one nested Raster owner retires") {
                store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                    assert!(released_items <= 1);
                    assert!(released_bytes <= RASTER_OWNED_FIELD_BYTES);
                }
                store::SnapshotRetirementStep::Complete => {
                    assert!(retirement.terminal_is_empty());
                    drop(retirement);
                    return;
                }
                store::SnapshotRetirementStep::Blocked => panic!("unshared Raster snapshot retirement cannot block"),
            }
        }
        panic!("nested Raster retirement did not reach terminal")
    }

    #[test]
    fn raster_owner_caps_and_all_mutation_variants_retire_one_owner_per_grant() {
        assert!(raster_clone_owned_string(&"x".repeat(RASTER_OWNED_FIELD_BYTES)).is_ok());
        assert!(raster_clone_owned_string(&"x".repeat(RASTER_OWNED_FIELD_BYTES + 1)).is_err());
        let pixel = || {
            Box::new(RasterLayerNode::Pixel { id: "pixel".into(), name: "Pixel".into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, width: Some(1), height: Some(1), image_key: None })
        };
        let mutations = vec![
            RasterMutation::CreateLayer(create_layer::mutation::CreateLayer { parent_id: Some("root".into()), index: 0, layer: pixel() }),
            RasterMutation::DeleteLayer(delete_layer::mutation::DeleteLayer { layer_id: "pixel".into() }),
            RasterMutation::ReorderLayers(reorder_layers::mutation::ReorderLayers { layer_id: "pixel".into(), parent_id: Some("root".into()), index: 1 }),
            RasterMutation::RenameLayer(rename_layer::mutation::RenameLayer { layer_id: "pixel".into(), new_name: "Renamed".into() }),
            RasterMutation::ChangeLayerVisible(change_layer_visible::mutation::ChangeLayerVisible { layer_id: "pixel".into(), new_visible: false }),
            RasterMutation::ChangeLayerOpacity(change_layer_opacity::mutation::ChangeLayerOpacity { layer_id: "pixel".into(), new_opacity: 0.5 }),
            RasterMutation::ChangeLayerBlendMode(change_layer_blend_mode::mutation::ChangeLayerBlendMode { layer_id: "pixel".into(), new_blend_mode: "multiply".into() }),
            RasterMutation::MoveLayer(move_layer::mutation::MoveLayer { layer_id: "pixel".into(), new_x: 1.0, new_y: 2.0 }),
            RasterMutation::ResizeLayer(resize_layer::mutation::ResizeLayer { layer_id: "pixel".into(), new_width: 2, new_height: 3 }),
            RasterMutation::ChangeLayerAdjustmentKind(change_layer_adjustment_kind::mutation::ChangeLayerAdjustmentKind { layer_id: "pixel".into(), new_adjustment_kind: "levels".into() }),
            RasterMutation::AddLayerAsset(add_layer_asset::mutation::AddLayerAsset { asset_id: "asset".into(), asset: RasterImageAsset { mime: "image/png".into(), data: vec![1, 2, 3] } }),
            RasterMutation::RemoveLayerAsset(remove_layer_asset::mutation::RemoveLayerAsset { asset_id: "asset".into() }),
        ];
        for mutation in mutations {
            let mut retirement = store::ArtifactOwnedValueRetirementFactory::retire_owned(&RasterMutationRetirementFactory, mutation);
            assert!(matches!(retirement.close_step(0, RASTER_OWNED_FIELD_BYTES).expect("zero-grant Raster retirement"), store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }));
            for _ in 0..100 {
                match retirement.close_step(1, RASTER_OWNED_FIELD_BYTES).expect("one Raster catalog owner retires") {
                    store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                        assert!(released_items <= 1);
                        assert!(released_bytes <= RASTER_OWNED_FIELD_BYTES);
                    }
                    store::SnapshotRetirementStep::Complete => {
                        assert!(retirement.terminal_is_empty());
                        break;
                    }
                    store::SnapshotRetirementStep::Blocked => panic!("unshared Raster mutation owner cannot block"),
                }
            }
            assert!(retirement.terminal_is_empty());
            drop(retirement);
        }
    }

    #[test]
    fn raster_envelope_caps_and_plus_one_page_return_the_exact_fixed_owner() {
        assert_eq!(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES, 4_096);
        assert_eq!(store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_PAGES, 64);
        assert_eq!(store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES, 262_144);
        let mut exact = [0; store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
        exact[0] = 0x52;
        exact[store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES - 1] = 0x7f;
        let rejected = store::ArtifactEnvelopeDecodePage::try_from_array(exact, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES + 1).expect_err("page cap plus one returns the exact caller owner");
        assert_eq!(rejected[0], 0x52);
        assert_eq!(rejected[store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES - 1], 0x7f);
    }

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
    /// `RasterMutation`'s `Edit` round-trips through `protocol::MutationEnvelope`s beside this file's
    /// existing pack round-trip law (same pattern as `mathematical_protocol`'s own
    /// `command_envelope_round_trip_holds_for_an_applied_operation`).
    #[semio_framework_async_macros::async_test]
    async fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use crate::artifacts::raster::RasterSnapshot;
        use protocol::{ArtifactId, Edit, SchemaId};

        let envelope = store::create_document_envelope::<RasterSnapshot, RasterMutation>(RASTER_DOCUMENT_SCHEMA, "command-envelope-demo", empty_raster_document(), None);
        let mut store = store::ArtifactStore::new(envelope).expect("valid artifact store fixture");
        store
            .dispatch(store::ArtifactCommand::Apply {
                mutations: vec![RasterMutation::CreateLayer(create_layer::mutation::CreateLayer {
                    parent_id: None,
                    index: 0,
                    layer: Box::new(RasterLayerNode::Pixel {
                        id: "command-envelope-pixel".into(),
                        name: "Command Envelope Pixel".into(),
                        visible: true,
                        opacity: 1.0,
                        blend_mode: "normal".into(),
                        transform: RasterTransform::default(),
                        mask: None,
                        width: Some(32),
                        height: Some(32),
                        image_key: None,
                    }),
                })],
                description: None,
            })
            .expect("apply");
        let edit: &Edit<RasterMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        store::os_store::test_support::assert_command_envelope_round_trip::<RasterSnapshot, RasterMutation>(edit, &ArtifactId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
    }
    //#endregion 🔖️CommandEnvelopeTests
}
//#endregion 🧪️Tests
