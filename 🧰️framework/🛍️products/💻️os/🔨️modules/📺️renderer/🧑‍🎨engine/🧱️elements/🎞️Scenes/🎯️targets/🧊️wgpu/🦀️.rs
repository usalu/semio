//! 🎬️ framework/products/os/modules/renderer/engine/elements/🎞️Scenes/component.rs — wgpu render
//! implementation for the Scenes element, extracted from lib.rs's inline `pub mod scenes { ... }`
//! body (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE). Wired via `#[path =
//! "../../../../🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs"] pub mod scenes;` in lib.rs in place of the former
//! inline block; the module name `scenes` is unchanged, so every existing `crate::scenes::...`
//! call site elsewhere in the crate keeps resolving with zero other changes.
//! 🎬️ Native component scene hosts for canvas-2d, tables, graphs, and 3D views.


use crate::engine_canvas;
use crate::interpreter::FrameworkWidgetContext;
use base64::Engine;
use semio_framework::IconName;
use serde::Deserialize;
use serde_json::{json, Value};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::io::Write as _;
use ui_wgpu::wgpu::input::{DragAxis, KeyAction};
use ui_wgpu::wgpu::{draw_text, draw_text_wrapped, render_widget, HitKind, HitTarget, Theme, WidgetNode};
use ui_wgpu::wgpu::Rect;
use ui_wgpu::wgpu::Rgba;
use ui_wgpu::wgpu::{ActionDescriptor, PreparedRasterProducer, PreparedRasterRejected, PreparedRasterReservation, SurfaceKind, UiComponentSceneNode};
use ui_wgpu::wgpu::UiPresence;

//#region SceneRuntime
pub const SCENE_SURFACE_CAPACITY: usize = 256;
pub const SCENE_SURFACE_ID_BYTE_CAPACITY: usize = 256;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdmittedSurfaceToken {
    slot: u16,
    epoch: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdmittedSurfaceFault {
    IdCapacity,
    ItemCapacity,
    ReplacementPending,
    RejectedPending,
    Closing,
}

#[derive(Debug)]
pub struct AdmittedSurfaceRejected<T> {
    pub fault: AdmittedSurfaceFault,
    pub id: String,
    pub value: T,
}

pub struct AdmittedSurfaceCloseOwner<T> {
    pub id: String,
    pub value: T,
}

struct AdmittedSurfaceEntry<T> {
    id: String,
    epoch: u64,
    value: T,
}

pub struct AdmittedSurfaceMap<T> {
    slots: Box<[Option<AdmittedSurfaceEntry<T>>; SCENE_SURFACE_CAPACITY]>,
    epochs: [u64; SCENE_SURFACE_CAPACITY],
    order: [Option<u16>; SCENE_SURFACE_CAPACITY],
    order_len: usize,
    fault: Option<&'static str>,
    rejected: Option<AdmittedSurfaceRejected<T>>,
    retired: Option<AdmittedSurfaceCloseOwner<T>>,
    closing: bool,
}

impl<T> Default for AdmittedSurfaceMap<T> {
    fn default() -> Self {
        Self { slots: semio_framework_async::boxed_fixed_slots(|| None), epochs: [0; SCENE_SURFACE_CAPACITY], order: [None; SCENE_SURFACE_CAPACITY], order_len: 0, fault: None, rejected: None, retired: None, closing: false }
    }
}

impl<T> AdmittedSurfaceMap<T> {
    fn existing_slot(&self, id: &str) -> Option<usize> {
        self.slots.iter().position(|entry| entry.as_ref().is_some_and(|entry| entry.id == id))
    }

    fn admit_slot(&mut self, id: &str) -> Result<usize, AdmittedSurfaceFault> {
        if self.closing {
            return Err(AdmittedSurfaceFault::Closing);
        }
        if let Some(slot) = self.existing_slot(id) {
            return Ok(slot);
        }
        if id.len() > SCENE_SURFACE_ID_BYTE_CAPACITY {
            self.fault = Some("scene surface identifier exceeded fixed credits");
            return Err(AdmittedSurfaceFault::IdCapacity);
        }
        if self.order_len == SCENE_SURFACE_CAPACITY {
            self.fault = Some("scene surface item credits exceeded");
            return Err(AdmittedSurfaceFault::ItemCapacity);
        }
        let slot = self.slots.iter().position(Option::is_none).expect("surface order credits imply one free fixed slot");
        self.order[self.order_len] = Some(slot as u16);
        self.order_len += 1;
        Ok(slot)
    }

    pub fn try_insert(&mut self, id: String, value: T) -> Result<AdmittedSurfaceToken, AdmittedSurfaceRejected<T>> {
        let slot = match self.admit_slot(&id) {
            Ok(slot) => slot,
            Err(fault) => return Err(AdmittedSurfaceRejected { fault, id, value }),
        };
        if let Some(entry) = self.slots[slot].as_mut() {
            if self.retired.is_some() {
                return Err(AdmittedSurfaceRejected { fault: AdmittedSurfaceFault::ReplacementPending, id, value });
            }
            self.epochs[slot] = self.epochs[slot].wrapping_add(1).max(1);
            entry.epoch = self.epochs[slot];
            let previous = std::mem::replace(&mut entry.value, value);
            self.retired = Some(AdmittedSurfaceCloseOwner { id, value: previous });
            return Ok(AdmittedSurfaceToken { slot: slot as u16, epoch: entry.epoch });
        }
        self.epochs[slot] = self.epochs[slot].wrapping_add(1).max(1);
        self.slots[slot] = Some(AdmittedSurfaceEntry { id, epoch: self.epochs[slot], value });
        Ok(AdmittedSurfaceToken { slot: slot as u16, epoch: self.epochs[slot] })
    }

    pub fn retain_rejected(&mut self, rejected: AdmittedSurfaceRejected<T>) -> Result<(), AdmittedSurfaceRejected<T>> {
        if self.rejected.is_some() {
            return Err(AdmittedSurfaceRejected { fault: AdmittedSurfaceFault::RejectedPending, ..rejected });
        }
        self.rejected = Some(rejected);
        Ok(())
    }

    pub fn retain_first_rejected(&mut self, rejected: AdmittedSurfaceRejected<T>) {
        assert!(self.rejected.is_none(), "surface producer must stop while one exact rejected owner is retained");
        self.rejected = Some(rejected);
    }

    pub fn admission_blocked(&self) -> bool {
        self.closing || self.rejected.is_some() || self.retired.is_some()
    }

    pub fn get_or_insert_with(&mut self, id: String, create: impl FnOnce() -> T) -> Option<&mut T> {
        let slot = self.admit_slot(&id).ok()?;
        if self.slots[slot].is_none() {
            self.epochs[slot] = self.epochs[slot].wrapping_add(1).max(1);
            self.slots[slot] = Some(AdmittedSurfaceEntry { id, epoch: self.epochs[slot], value: create() });
        }
        self.slots[slot].as_mut().map(|entry| &mut entry.value)
    }

    pub fn id_at(&self, index: usize) -> Option<&str> {
        let slot = usize::from(self.order.get(index).copied().flatten()?);
        self.slots.get(slot).and_then(Option::as_ref).map(|entry| entry.id.as_str())
    }

    pub fn len(&self) -> usize {
        self.order_len
    }

    pub fn is_empty(&self) -> bool {
        self.order_len == 0
    }

    pub fn contains_key(&self, id: &str) -> bool {
        self.existing_slot(id).is_some()
    }

    pub fn token(&self, id: &str) -> Option<AdmittedSurfaceToken> {
        let slot = self.existing_slot(id)?;
        Some(AdmittedSurfaceToken { slot: slot as u16, epoch: self.slots[slot].as_ref()?.epoch })
    }

    pub fn get_token(&self, token: AdmittedSurfaceToken) -> Option<&T> {
        self.slots.get(usize::from(token.slot)).and_then(Option::as_ref).filter(|entry| entry.epoch == token.epoch).map(|entry| &entry.value)
    }

    pub fn get_token_mut(&mut self, token: AdmittedSurfaceToken) -> Option<&mut T> {
        self.slots.get_mut(usize::from(token.slot)).and_then(Option::as_mut).filter(|entry| entry.epoch == token.epoch).map(|entry| &mut entry.value)
    }

    pub fn get(&self, id: &str) -> Option<&T> {
        self.existing_slot(id).and_then(|slot| self.slots[slot].as_ref().map(|entry| &entry.value))
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut T> {
        let slot = self.existing_slot(id)?;
        self.slots[slot].as_mut().map(|entry| &mut entry.value)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &T)> {
        self.order[..self.order_len].iter().filter_map(|slot| {
            let entry = self.slots[usize::from((*slot).expect("admitted surface order slot"))].as_ref()?;
            Some((&entry.id, &entry.value))
        })
    }

    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.iter().map(|(id, _)| id)
    }

    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.slots.iter().flatten().map(|entry| &entry.value)
    }

    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.slots.iter_mut().flatten().map(|entry| &mut entry.value)
    }

    pub fn remove(&mut self, id: &str) -> Option<T> {
        let slot = self.existing_slot(id)?;
        let Some(index) = (0..self.order_len).find(|index| self.order[*index] == Some(slot as u16)) else {
            self.fault = Some("scene surface order lost ownership");
            return self.slots[slot].take().map(|entry| entry.value);
        };
        let value = self.slots[slot].take().map(|entry| entry.value);
        for cursor in index..self.order_len - 1 {
            self.order[cursor] = self.order[cursor + 1];
        }
        self.order_len -= 1;
        self.order[self.order_len] = None;
        value
    }

    pub fn clear(&mut self) {
        self.closing = true;
        self.record_fault("scene surface clear requires retained close pumping");
    }

    pub fn take_fault(&mut self) -> Option<&'static str> {
        self.fault.take()
    }

    pub fn record_fault(&mut self, fault: &'static str) {
        if self.fault.is_none() {
            self.fault = Some(fault);
        }
    }

    pub fn begin_close(&mut self) {
        self.closing = true;
    }

    pub fn close_step(&mut self) -> Option<AdmittedSurfaceCloseOwner<T>> {
        if let Some(rejected) = self.rejected.take() {
            return Some(AdmittedSurfaceCloseOwner { id: rejected.id, value: rejected.value });
        }
        if let Some(retired) = self.retired.take() {
            return Some(retired);
        }
        if self.order_len == 0 {
            return None;
        }
        self.order_len -= 1;
        let slot = usize::from(self.order[self.order_len].take().expect("admitted close order slot"));
        self.slots[slot].take().map(|entry| AdmittedSurfaceCloseOwner { id: entry.id, value: entry.value })
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.order_len == 0 && self.slots.iter().all(Option::is_none) && self.rejected.is_none() && self.retired.is_none()
    }
}

impl<'a, T> IntoIterator for &'a AdmittedSurfaceMap<T> {
    type Item = (&'a String, &'a T);
    type IntoIter = Box<dyn Iterator<Item = Self::Item> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(self.iter())
    }
}

#[cfg(test)]
#[path = "../../🧪️tests/🔬️wgpu-admitted-surface-map/🦀️.rs"]
mod admitted_surface_map_tests;

#[derive(Clone, Copy, Debug, Default)]
struct Viewport {
    x: f32,
    y: f32,
    zoom: f32,
}

impl Viewport {
    fn from_json(raw: &str) -> Self {
        serde_json::from_str::<Value>(raw)
            .ok()
            .map(|value| Self { x: value.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32, y: value.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32, zoom: value.get("zoom").and_then(|v| v.as_f64()).unwrap_or(1.0) as f32 })
            .unwrap_or_default()
    }

    fn from_typed(viewport: Option<&semio_framework_os_kernel::Viewport2d>) -> Self {
        viewport.map(|viewport| Self { x: viewport.x as f32, y: viewport.y as f32, zoom: viewport.zoom as f32 }).unwrap_or(Self { x: 0.0, y: 0.0, zoom: 1.0 })
    }

    fn screen_to_world(&self, sx: f32, sy: f32, origin: Rect) -> (f32, f32) {
        let cx = origin.x + origin.w * 0.5;
        let cy = origin.y + origin.h * 0.5;
        ((sx - cx) / self.zoom + self.x, (sy - cy) / self.zoom + self.y)
    }

    fn world_to_screen(&self, wx: f32, wy: f32, origin: Rect) -> (f32, f32) {
        let cx = origin.x + origin.w * 0.5;
        let cy = origin.y + origin.h * 0.5;
        (cx + (wx - self.x) * self.zoom, cy + (wy - self.y) * self.zoom)
    }
}

#[derive(Clone, Debug)]
enum SceneDragMode {
    PanViewport,
    MapMarquee { start_x: f32, start_y: f32, method: String, merge_mode: String },
    MapPan,
    InkPan { start_x: f32, start_y: f32, camera_x: f64, camera_y: f64, zoom: f64 },
    InkMove { origins: HashMap<String, (f64, f64)>, start_x: f32, start_y: f32 },
    InkResize { handle: String, from: InkBoundsF, start_x: f32, start_y: f32, selected_ids: Vec<String> },
    InkStroke { block_id: String },
    InkEraser { mode: String },
    InkMarqueeDrag { start_x: f32, start_y: f32 },
}

#[derive(Clone, Debug)]
struct SceneDrag {
    mode: SceneDragMode,
}

#[derive(Clone, Debug, Default)]
struct SceneSurfaceState {
    scroll_offsets: HashMap<String, f32>,
    viewport: Viewport,
    drag: Option<SceneDrag>,
    pointer_was_down: bool,
    last_click_ms: f64,
    last_click_target: Option<String>,
    /// 🖱️ The list-surface control the pointer last resolved over — this module's own hover
    /// authority, because a `ComponentScene` leaf registers exactly one retained hit target for the
    /// WHOLE surface (`retained_scene_hit`), so `InputState::hovered_id` can never name a row.
    hovered_control_id: Option<String>,
    #[cfg(test)]
    node_positions: HashMap<String, (f32, f32)>,
    selected_ids: HashSet<String>,
    paint_stroke_active: bool,
    vfs_expanded_ids: HashSet<String>,
    vfs_selection_anchor: Option<String>,
    map_marquee_points: Vec<(f32, f32)>,
    map_marquee_active: bool,
    map_last_hover_json: Option<String>,
    ink_camera: Option<(f64, f64, f64)>,
    ink_overrides: HashMap<String, Value>,
    ink_marquee_points: Vec<(f32, f32)>,
    //#region GenericPointerDispatch
    last_pointer_pos: (f32, f32),
    //#endregion GenericPointerDispatch
    /// 🕒️ The controller id a Canvas2d/Paint2d surface's settled `setCamera` dispatch should target —
    /// stashed here (rather than threaded through `SCENE_CAMERA_DISPATCH_DEADLINES_MS`) since that map
    /// only needs a bare surface-id -> deadline shape to stay a drop-in match for
    /// `sweep_expired_camera_dispatch_deadlines`. Set on every wheel/pan mutation, read (never cleared)
    /// by `sweep_expired_scene_camera_dispatches` at expiry.
    camera_dispatch_controller_id: Option<String>,
}

const RASTER_SURFACE_CAPACITY: usize = 256;
const RASTER_UPLOADS_PER_SURFACE_CAPACITY: usize = 16;
const RASTER_UPLOAD_KEY_BYTE_CAPACITY: usize = 256;
const RASTER_UPLOAD_BYTE_CAPACITY: usize = 1024 * 1024;

struct PendingRasterQueue {
    slots: Box<[Option<PreparedRasterProducer>; RASTER_UPLOADS_PER_SURFACE_CAPACITY]>,
    epochs: [u64; RASTER_UPLOADS_PER_SURFACE_CAPACITY],
    head: u8,
    len: u8,
    checked_out: Option<PendingRasterQueueToken>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct PendingRasterQueueToken {
    slot: u8,
    epoch: u64,
}

impl Default for PendingRasterQueue {
    fn default() -> Self {
        Self { slots: semio_framework_async::boxed_fixed_slots(|| None), epochs: [0; RASTER_UPLOADS_PER_SURFACE_CAPACITY], head: 0, len: 0, checked_out: None }
    }
}

impl PendingRasterQueue {
    fn len(&self) -> usize {
        usize::from(self.len)
    }

    fn is_full(&self) -> bool {
        self.len() == RASTER_UPLOADS_PER_SURFACE_CAPACITY
    }

    fn push_back(&mut self, producer: PreparedRasterProducer) -> Result<(), PreparedRasterProducer> {
        if self.is_full() {
            return Err(producer);
        }
        let index = (usize::from(self.head) + self.len()) % RASTER_UPLOADS_PER_SURFACE_CAPACITY;
        self.epochs[index] = self.epochs[index].wrapping_add(1).max(1);
        self.slots[index] = Some(producer);
        self.len += 1;
        Ok(())
    }

    fn checkout_front(&mut self) -> Option<PendingRasterQueueToken> {
        if self.len == 0 || self.checked_out.is_some() {
            return None;
        }
        let index = usize::from(self.head);
        self.slots[index].as_ref()?;
        let token = PendingRasterQueueToken { slot: self.head, epoch: self.epochs[index] };
        self.checked_out = Some(token);
        Some(token)
    }

    fn take_checked_out(&mut self, token: PendingRasterQueueToken) -> Option<PreparedRasterProducer> {
        if self.checked_out != Some(token) || self.head != token.slot || self.epochs[usize::from(token.slot)] != token.epoch {
            return None;
        }
        let producer = self.slots[usize::from(token.slot)].take()?;
        self.checked_out = None;
        let index = usize::from(self.head);
        self.head = ((index + 1) % RASTER_UPLOADS_PER_SURFACE_CAPACITY) as u8;
        self.len -= 1;
        Some(producer)
    }

    fn hand_back(&mut self, token: PendingRasterQueueToken) -> bool {
        if self.checked_out != Some(token) || self.head != token.slot || self.epochs[usize::from(token.slot)] != token.epoch || self.slots[usize::from(token.slot)].is_none() {
            return false;
        }
        self.checked_out = None;
        true
    }

    fn pop_front_for_close(&mut self) -> Option<PreparedRasterProducer> {
        if self.checked_out.is_some() {
            return None;
        }
        let token = self.checkout_front()?;
        self.take_checked_out(token)
    }

    #[cfg(test)]
    fn close_all(&mut self) {
        while let Some(mut producer) = self.pop_front_for_close() {
            producer.begin_close();
            while !producer.close_step() {}
        }
    }
}

#[derive(Default)]
struct PendingRasterSurface {
    queue: PendingRasterQueue,
    admission: Option<PreparedRasterReservation>,
    rejected: Option<PreparedRasterRejected>,
    retiring: Option<PreparedRasterRejected>,
    closing: Option<PreparedRasterProducer>,
}

pub enum PendingRasterUploadStep {
    Pending,
    Upload(PendingRasterCheckedOut),
    Complete,
    Fault(&'static str),
}

pub struct PendingRasterCheckedOut {
    surface: AdmittedSurfaceToken,
    queue: PendingRasterQueueToken,
    active: bool,
}

impl PendingRasterCheckedOut {
    pub fn take(mut self) -> Result<PreparedRasterProducer, Self> {
        let producer = PENDING_RASTER_STATE.with(|cell| cell.borrow_mut().get_token_mut(self.surface).and_then(|surface| surface.queue.take_checked_out(self.queue)));
        let Some(producer) = producer else { return Err(self) };
        self.active = false;
        Ok(producer)
    }
}

impl Drop for PendingRasterCheckedOut {
    fn drop(&mut self) {
        if !self.active {
            return;
        }
        let returned = PENDING_RASTER_STATE.with(|cell| cell.borrow_mut().get_token_mut(self.surface).is_some_and(|surface| surface.queue.hand_back(self.queue)));
        debug_assert!(returned || std::thread::panicking(), "checked-out raster producer must return to its exact FIFO slot");
    }
}

#[derive(Default)]
pub struct PendingRasterUploadCursor {
    surface_index: usize,
    closing: Option<PreparedRasterProducer>,
}

impl PendingRasterUploadCursor {
    pub fn step(&mut self) -> PendingRasterUploadStep {
        if let Some(producer) = self.closing.as_mut() {
            if !producer.close_step() {
                return PendingRasterUploadStep::Pending;
            }
            self.closing = None;
            return PendingRasterUploadStep::Fault("raster producer handoff was rejected");
        }
        PENDING_RASTER_STATE.with(|cell| {
            let mut states = cell.borrow_mut();
            if states.len() > RASTER_SURFACE_CAPACITY {
                return PendingRasterUploadStep::Fault("raster surface credits exceeded");
            }
            if let Some(fault) = states.take_fault() {
                return PendingRasterUploadStep::Fault(fault);
            }
            let Some(surface_id) = states.id_at(self.surface_index).map(str::to_owned) else { return PendingRasterUploadStep::Complete };
            let Some(surface_token) = states.token(&surface_id) else { return PendingRasterUploadStep::Fault("raster surface token lost ownership") };
            let Some(state) = states.get_mut(&surface_id) else { return PendingRasterUploadStep::Fault("raster surface order lost ownership") };
            if let Some(reservation) = state.admission.take() {
                state.rejected = Some(reservation.reject("raster reservation was abandoned before publication", Vec::new()));
                return PendingRasterUploadStep::Pending;
            }
            if let Some(retiring) = state.retiring.as_mut() {
                if !retiring.close_step() {
                    return PendingRasterUploadStep::Pending;
                }
                assert!(retiring.terminal_is_empty(), "retired raster reservation must be terminal-empty");
                state.retiring = None;
                return PendingRasterUploadStep::Pending;
            }
            if let Some(rejected) = state.rejected.as_mut() {
                let fault = rejected.fault();
                if !rejected.close_step() {
                    return PendingRasterUploadStep::Pending;
                }
                assert!(rejected.terminal_is_empty(), "rejected raster reservation must be terminal-empty");
                state.rejected = None;
                return PendingRasterUploadStep::Fault(fault);
            }
            if let Some(producer) = state.closing.as_mut() {
                if !producer.close_step() {
                    return PendingRasterUploadStep::Pending;
                }
                state.closing = None;
                return PendingRasterUploadStep::Fault("raster producer admission changed before FIFO publication");
            }
            if let Some(queue) = state.queue.checkout_front() {
                return PendingRasterUploadStep::Upload(PendingRasterCheckedOut { surface: surface_token, queue, active: true });
            }
            self.surface_index += 1;
            PendingRasterUploadStep::Pending
        })
    }

    pub fn retain_rejected(&mut self, mut producer: PreparedRasterProducer) -> Result<(), PreparedRasterProducer> {
        if self.closing.is_some() {
            return Err(producer);
        }
        producer.begin_close();
        self.closing = Some(producer);
        Ok(())
    }

    pub fn close_step(&mut self) -> bool {
        let Some(producer) = self.closing.as_mut() else { return true };
        if !producer.close_step() {
            return false;
        }
        self.closing = None;
        true
    }
}

struct PendingRasterSurfaceRetirement {
    id: String,
    surface: PendingRasterSurface,
    producer: Option<PreparedRasterProducer>,
    id_released: bool,
    scalars_released: bool,
}

impl PendingRasterSurfaceRetirement {
    fn new(owner: AdmittedSurfaceCloseOwner<PendingRasterSurface>) -> Self {
        Self { id: owner.id, surface: owner.value, producer: None, id_released: false, scalars_released: false }
    }

    fn close_step(&mut self) -> bool {
        if let Some(producer) = self.producer.as_mut() {
            if !producer.close_step() {
                return false;
            }
            assert!(producer.terminal_is_empty(), "realm-retired raster producer must be terminal-empty");
            self.producer = None;
            return false;
        }
        if let Some(retiring) = self.surface.retiring.as_mut() {
            if !retiring.close_step() {
                return false;
            }
            assert!(retiring.terminal_is_empty(), "realm-retired raster reservation must be terminal-empty");
            self.surface.retiring = None;
            return false;
        }
        if let Some(reservation) = self.surface.admission.take() {
            self.surface.retiring = Some(reservation.reject("realm closed pending raster reservation", Vec::new()));
            return false;
        }
        if let Some(rejected) = self.surface.rejected.as_mut() {
            if !rejected.close_step() {
                return false;
            }
            assert!(rejected.terminal_is_empty(), "realm-rejected raster reservation must be terminal-empty");
            self.surface.rejected = None;
            return false;
        }
        if let Some(mut producer) = self.surface.closing.take() {
            producer.begin_close();
            self.producer = Some(producer);
            return false;
        }
        if self.surface.queue.checked_out.is_some() {
            return false;
        }
        if let Some(mut producer) = self.surface.queue.pop_front_for_close() {
            producer.begin_close();
            self.producer = Some(producer);
            return false;
        }
        if self.id.pop().is_some() {
            return false;
        }
        if !self.id_released {
            self.id = String::new();
            self.id_released = true;
            return false;
        }
        if !self.scalars_released {
            self.surface.queue.head = 0;
            self.surface.queue.len = 0;
            self.surface.queue.checked_out = None;
            self.surface.queue.epochs = [0; RASTER_UPLOADS_PER_SURFACE_CAPACITY];
            self.scalars_released = true;
            return false;
        }
        true
    }

    fn terminal_is_empty(&self) -> bool {
        self.id.is_empty()
            && self.id.capacity() == 0
            && self.id_released
            && self.producer.is_none()
            && self.surface.retiring.is_none()
            && self.surface.admission.is_none()
            && self.surface.rejected.is_none()
            && self.surface.closing.is_none()
            && self.surface.queue.len == 0
            && self.surface.queue.checked_out.is_none()
            && self.surface.queue.slots.iter().all(Option::is_none)
            && self.scalars_released
    }
}

pub struct PendingRasterAuthorityClose {
    complete: bool,
}

pub fn begin_pending_raster_authority_close() -> PendingRasterAuthorityClose {
    PENDING_RASTER_STATE.with(|cell| cell.borrow_mut().begin_close());
    PendingRasterAuthorityClose { complete: false }
}

impl PendingRasterAuthorityClose {
    pub fn close_step(&mut self) -> bool {
        if self.complete {
            return true;
        }
        let active = PENDING_RASTER_CLOSE_OWNER.with(|cell| {
            let mut owner = cell.borrow_mut();
            let Some(surface) = owner.as_mut() else { return false };
            if !surface.close_step() {
                return true;
            }
            assert!(surface.terminal_is_empty(), "pending raster surface must be terminal-empty before realm release");
            *owner = None;
            true
        });
        if active {
            return false;
        }
        let owner = PENDING_RASTER_STATE.with(|cell| cell.borrow_mut().close_step());
        if let Some(owner) = owner {
            PENDING_RASTER_CLOSE_OWNER.with(|cell| {
                let mut retained = cell.borrow_mut();
                assert!(retained.is_none(), "one pending raster close owner is admitted at a time");
                *retained = Some(PendingRasterSurfaceRetirement::new(owner));
            });
            return false;
        }
        self.complete = PENDING_RASTER_STATE.with(|cell| cell.borrow().terminal_is_empty()) && PENDING_RASTER_CLOSE_OWNER.with(|cell| cell.borrow().is_none());
        self.complete
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.complete && PENDING_RASTER_STATE.with(|cell| cell.borrow().terminal_is_empty()) && PENDING_RASTER_CLOSE_OWNER.with(|cell| cell.borrow().is_none())
    }
}

#[cfg(not(target_arch = "wasm32"))]
struct WorkerCell<T>(std::sync::OnceLock<std::sync::Mutex<RefCell<T>>>);

#[cfg(not(target_arch = "wasm32"))]
impl<T> WorkerCell<T> {
    const fn new() -> Self {
        Self(std::sync::OnceLock::new())
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<T: Default> WorkerCell<T> {
    fn with<R>(&self, f: impl FnOnce(&RefCell<T>) -> R) -> R {
        let guard = self.0.get_or_init(|| std::sync::Mutex::new(RefCell::new(T::default()))).lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        f(&guard)
    }
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static SCENE_STATE: RefCell<AdmittedSurfaceMap<SceneSurfaceState>> = RefCell::new(AdmittedSurfaceMap::default());
    static PENDING_RASTER_STATE: RefCell<AdmittedSurfaceMap<PendingRasterSurface>> = RefCell::new(AdmittedSurfaceMap::default());
    static PENDING_RASTER_CLOSE_OWNER: RefCell<Option<PendingRasterSurfaceRetirement>> = const { RefCell::new(None) };
    /// 🕒️ Canvas2d/Paint2d's settle-then-dispatch deadline map — surface id -> the timestamp its
    /// debounced `setCamera` should fire at. Same shape/sweep (`sweep_expired_camera_dispatch_deadlines`)
    /// as `AppRuntime`'s `world3d_camera_dispatch_deadlines_ms`; kept thread-local here rather than on
    /// `AppRuntime` because `handle_scene_wheel`/`handle_scene_pointer_move` (this module's wheel/pan
    /// mutators) only ever see a `&UiComponentSceneNode`, never `AppRuntime` itself.
    static SCENE_CAMERA_DISPATCH_DEADLINES_MS: RefCell<HashMap<String, f64>> = RefCell::new(HashMap::new());
    static SCENE_CAMERA_DISPATCH_FAULT: RefCell<Option<&'static str>> = RefCell::new(None);
}

#[cfg(not(target_arch = "wasm32"))]
static SCENE_STATE: WorkerCell<AdmittedSurfaceMap<SceneSurfaceState>> = WorkerCell::new();
#[cfg(not(target_arch = "wasm32"))]
static PENDING_RASTER_STATE: WorkerCell<AdmittedSurfaceMap<PendingRasterSurface>> = WorkerCell::new();
#[cfg(not(target_arch = "wasm32"))]
static PENDING_RASTER_CLOSE_OWNER: WorkerCell<Option<PendingRasterSurfaceRetirement>> = WorkerCell::new();
#[cfg(not(target_arch = "wasm32"))]
#[cfg(not(target_arch = "wasm32"))]
static SCENE_CAMERA_DISPATCH_DEADLINES_MS: WorkerCell<HashMap<String, f64>> = WorkerCell::new();
#[cfg(not(target_arch = "wasm32"))]
static SCENE_CAMERA_DISPATCH_FAULT: WorkerCell<Option<&'static str>> = WorkerCell::new();


/** @emoji 📁️ Toggles VFS row expand/collapse in scene-local state. */
pub fn toggle_vfs_row_expanded(surface_id: &str, row_id: &str) {
    mutate_scene_state(surface_id, |state| {
        if state.vfs_expanded_ids.contains(row_id) {
            state.vfs_expanded_ids.remove(row_id);
        } else {
            state.vfs_expanded_ids.insert(row_id.to_string());
        }
    });
}

/** @emoji 📁️ Seeds default expanded VFS roots on first render. */
pub fn seed_vfs_expanded(surface_id: &str, row_ids: &[String]) {
    mutate_scene_state(surface_id, |state| {
        if state.vfs_expanded_ids.is_empty() {
            for id in row_ids {
                state.vfs_expanded_ids.insert(id.clone());
            }
        }
    });
}

/** @emoji 📁️ Computes VFS multi-select ids for shift/meta click semantics. */
pub fn vfs_selection_for_click(surface_id: &str, row_id: &str, ordered_ids: &[String], shift: bool, additive: bool) -> Vec<String> {
    let mut state = scene_state(surface_id);
    if shift {
        let anchor = state.vfs_selection_anchor.clone().unwrap_or_else(|| row_id.to_string());
        let a = ordered_ids.iter().position(|id| id == &anchor);
        let b = ordered_ids.iter().position(|id| id == row_id);
        if let (Some(a), Some(b)) = (a, b) {
            let (start, end) = if a <= b { (a, b) } else { (b, a) };
            let ids: Vec<String> = ordered_ids[start..=end].to_vec();
            state.vfs_selection_anchor = Some(anchor);
            mutate_scene_state(surface_id, |state| {
                state.vfs_selection_anchor = Some(row_id.to_string());
            });
            return ids;
        }
    }
    mutate_scene_state(surface_id, |state| {
        state.vfs_selection_anchor = Some(row_id.to_string());
    });
    if additive {
        let mut ids: Vec<String> = scene_state(surface_id).selected_ids.into_iter().collect();
        if ids.iter().any(|id| id == row_id) {
            ids.retain(|id| id != row_id);
        } else {
            ids.push(row_id.to_string());
        }
        return ids;
    }
    vec![row_id.to_string()]
}

fn scene_state(surface_id: &str) -> SceneSurfaceState {
    SCENE_STATE.with(|cell| cell.borrow_mut().get_or_insert_with(surface_id.to_string(), SceneSurfaceState::default).cloned().unwrap_or_default())
}

fn mutate_scene_state(surface_id: &str, f: impl FnOnce(&mut SceneSurfaceState)) {
    SCENE_STATE.with(|cell| {
        let mut map = cell.borrow_mut();
        if let Some(entry) = map.get_or_insert_with(surface_id.to_string(), SceneSurfaceState::default) {
            f(entry);
        }
    });
}

//#region SceneCameraDispatch
const SCENE_CAMERA_DISPATCH_CAPACITY: usize = 256;
const SCENE_CAMERA_ID_BYTE_CAPACITY: usize = 256;

/// 🕒️ Pushes a Canvas2d/Paint2d surface's settled `setCamera` deadline ~350ms out — called on every
/// wheel/pan mutation (see `handle_scene_wheel`'s `Canvas2d`/`Paint2d` arms and
/// `handle_scene_pointer_move`'s `PanViewport` arm), same 350ms settle window as
/// `AppRuntime::world3d_camera_dispatch_deadlines_ms`.
fn schedule_scene_camera_dispatch(surface_id: &str) {
    SCENE_CAMERA_DISPATCH_DEADLINES_MS.with(|cell| {
        let mut deadlines = cell.borrow_mut();
        if surface_id.len() > SCENE_CAMERA_ID_BYTE_CAPACITY || (!deadlines.contains_key(surface_id) && deadlines.len() >= SCENE_CAMERA_DISPATCH_CAPACITY) {
            SCENE_CAMERA_DISPATCH_FAULT.with(|fault| *fault.borrow_mut() = Some("scene camera deadline credits exceeded"));
            return;
        }
        deadlines.insert(surface_id.to_string(), crate::app_now_ms() + 350.0);
    });
}

pub enum SceneCameraDispatchStep {
    Pending,
    Action(ActionDescriptor),
    Complete,
    Fault(&'static str),
}

pub struct SceneCameraDispatchCursor {
    entries: std::collections::hash_map::IntoIter<String, f64>,
    now_ms: f64,
    fault: Option<&'static str>,
}

impl SceneCameraDispatchCursor {
    pub fn begin(now_ms: f64) -> Self {
        let entries = SCENE_CAMERA_DISPATCH_DEADLINES_MS.with(|cell| std::mem::take(&mut *cell.borrow_mut()).into_iter());
        let fault = SCENE_CAMERA_DISPATCH_FAULT.with(|cell| cell.borrow_mut().take());
        Self { entries, now_ms, fault }
    }

    fn restore(surface_id: String, deadline: f64) {
        SCENE_CAMERA_DISPATCH_DEADLINES_MS.with(|cell| {
            let mut deadlines = cell.borrow_mut();
            let effective = deadlines.get(surface_id.as_str()).copied().map_or(deadline, |newer| newer.max(deadline));
            deadlines.insert(surface_id, effective);
        });
    }

    pub fn step(&mut self) -> SceneCameraDispatchStep {
        if let Some(fault) = self.fault.take() {
            return SceneCameraDispatchStep::Fault(fault);
        }
        let Some((surface_id, deadline)) = self.entries.next() else { return SceneCameraDispatchStep::Complete };
        if deadline > self.now_ms {
            Self::restore(surface_id, deadline);
            return SceneCameraDispatchStep::Pending;
        }
        let action = SCENE_STATE.with(|cell| -> Result<Option<ActionDescriptor>, &'static str> {
            let states = cell.borrow();
            let Some(state) = states.get(surface_id.as_str()) else { return Ok(None) };
            let Some(controller_id) = state.camera_dispatch_controller_id.as_ref() else { return Ok(None) };
            if controller_id.len() > SCENE_CAMERA_ID_BYTE_CAPACITY {
                return Err("scene camera action identifier exceeded fixed credits");
            }
            Ok(Some(scene_camera_action(&surface_id, controller_id, state.viewport)))
        });
        match action {
            Ok(Some(action)) => SceneCameraDispatchStep::Action(action),
            Ok(None) => SceneCameraDispatchStep::Pending,
            Err(fault) => SceneCameraDispatchStep::Fault(fault),
        }
    }

    pub fn close_step(&mut self) -> bool {
        let Some((surface_id, deadline)) = self.entries.next() else {
            self.fault = None;
            return true;
        };
        Self::restore(surface_id, deadline);
        false
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.entries.len() == 0 && self.fault.is_none()
    }
}

/// 🕒️ A Canvas2d/Paint2d `setCamera` action from a bare surface id/controller id/viewport — same
/// `{surfaceId, camera: {x, y, zoom}}` shape `ink_set_camera_action` already uses for `InkCanvas`'s own
/// camera dispatch (this crate's one other camera-from-viewport builder), reused here since neither
/// `Paint2dHost`'s React source nor this repo's own Ink precedent key it any other way.
fn scene_camera_action(surface_id: &str, controller_id: &str, viewport: Viewport) -> ActionDescriptor {
    ActionDescriptor { controller_id: controller_id.to_string(), action: "setCamera".into(), args: crate::action_args_json!({ "surfaceId": surface_id, "camera": { "x": viewport.x, "y": viewport.y, "zoom": viewport.zoom } }) }
}

/// 🕒️ `AppRuntime::frame`'s per-frame hook into this module's settle-then-dispatch deadlines —
/// sweeps `SCENE_CAMERA_DISPATCH_DEADLINES_MS` via the shared pure
/// `sweep_expired_camera_dispatch_deadlines`, then builds each expired surface's `setCamera` action
/// from its last-known viewport + stashed controller id (`camera_dispatch_controller_id`; `None` only
/// if a deadline outlives its `SCENE_STATE` entry, which never happens in practice since both are
/// written together in `schedule_scene_camera_dispatch`'s call sites).
#[cfg(test)]
include!("../../🧪️tests/🧊️wgpu-standalone/🦀️.rs");
//#endregion SceneCameraDispatch

/** @emoji 🖱️ Cheap read of a surface's pointer edge-detection fields, avoiding a full `SceneSurfaceState` clone. `pub(crate)` so `interpreter::apply_scene_ui_command` (the real per-event `UiCommand::Scene` handler — the sole caller of `handle_scene_pointer_button`/`handle_scene_pointer_move` now, `RenderEntry`'s own once-per-render-frame `apply_scene_wheel`/`apply_scene_pointer` having been deleted once every generic-fallback surface was proven reachable through this path) can read `pointer_was_down`/`last_pointer_pos` to derive `handle_scene_pointer_move`'s `down`/drag-delta parameters. */
pub(crate) fn scene_pointer_edge_state(surface_id: &str) -> (bool, f32, f32) {
    SCENE_STATE.with(|cell| cell.borrow().get(surface_id).map(|state| (state.pointer_was_down, state.last_pointer_pos.0, state.last_pointer_pos.1)).unwrap_or((false, 0.0, 0.0)))
}

/** @emoji 🖱️ Records `surface_id`'s latest known pointer position — the write half of `scene_pointer_edge_state`, `pub(crate)` for the same reason (see that fn's own doc comment). */
pub(crate) fn set_scene_last_pointer_pos(surface_id: &str, x: f32, y: f32) {
    mutate_scene_state(surface_id, |state| {
        state.last_pointer_pos = (x, y);
    });
}

















fn scroll_key(surface_id: &str, suffix: &str) -> String {
    format!("{surface_id}.{suffix}")
}

fn scroll_offset(surface_id: &str, suffix: &str) -> f32 {
    let key = scroll_key(surface_id, suffix);
    SCENE_STATE.with(|cell| cell.borrow().get(surface_id).and_then(|state| state.scroll_offsets.get(&key).copied()).unwrap_or(0.0)).max(0.0)
}

fn set_scroll_offset(surface_id: &str, suffix: &str, value: f32) {
    let key = scroll_key(surface_id, suffix);
    mutate_scene_state(surface_id, |state| {
        state.scroll_offsets.insert(key, value.max(0.0));
    });
}

/// 🎬️ One scene-addressed action descriptor: the surface's own controller id, the action name and its
/// JSON args projected into the DSL. The ONE builder every scene lane shares — `⚙️EngineCanvas`'s
/// standalone test lane calls it too rather than keeping a second copy
/// (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY wave 2–6 integration).
pub(crate) fn scene_action(scene: &UiComponentSceneNode, action: &str, args: Value) -> ActionDescriptor {
    ActionDescriptor { controller_id: scene.controller_id.clone(), action: action.into(), args: semio_framework::optional_json_to_dsl(Some(args)) }
}

#[cfg(target_arch = "wasm32")]
fn now_ms() -> f64 {
    web_sys::window().and_then(|window| window.performance()).map(|perf| perf.now()).unwrap_or(0.0)
}

#[cfg(not(target_arch = "wasm32"))]
fn now_ms() -> f64 {
    0.0
}

/// 🎨️ The theme the most recent scene paint ran under. The pointer/wheel path (`passive_scene_*`)
/// only ever receives a `&UiComponentSceneNode` and a rect — never the frame's `Theme` — yet every
/// list surface's row geometry is theme-derived (`control_height`, `padding_standard`), so a hit
/// resolved against `Theme::default()` would land on a different row than the one painted. Recorded
/// once per scene paint (phase 2 of `render_component_scene_step`) and read back here.
#[cfg(target_arch = "wasm32")]
thread_local! {
    static SCENE_INPUT_THEME: RefCell<Option<Theme>> = const { RefCell::new(None) };
}
#[cfg(not(target_arch = "wasm32"))]
static SCENE_INPUT_THEME: WorkerCell<Option<Theme>> = WorkerCell::new();

fn remember_scene_theme(theme: &Theme) {
    SCENE_INPUT_THEME.with(|cell| {
        *cell.borrow_mut() = Some(*theme);
    });
}

fn scene_input_theme() -> Theme {
    SCENE_INPUT_THEME.with(|cell| *cell.borrow()).unwrap_or_default()
}

/// 📏️ Byte credits one `DslValue` argument payload needs inside a bounded action — every object key
/// and string scalar it carries. Numbers/bools/nulls are inline `FlatValue`s and cost no text bytes.
fn dsl_value_bytes(value: &semio_framework::DslValue) -> usize {
    match value {
        semio_framework::DslValue::String(text) => text.len(),
        semio_framework::DslValue::Array(values) => values.iter().map(dsl_value_bytes).sum(),
        semio_framework::DslValue::Object(entries) => entries.iter().map(|(key, value)| key.len().saturating_add(dsl_value_bytes(value))).sum(),
        _ => 0,
    }
}

/// 📨️ Publishes one already-built `ActionDescriptor` through the bounded action queue — the single
/// ingress every list surface's pointer handler uses, so a row click, a header sort and a palette
/// press all pay the same exact item/byte credits.
fn write_scene_action(input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>, descriptor: &ActionDescriptor) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[descriptor.controller_id.as_str(), descriptor.action.as_str()])?
        .checked_add(descriptor.args.as_ref().map_or(0, dsl_value_bytes))
        .ok_or(ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?;
    let mut batch = input.reserve_actions(1, bytes)?;
    batch.action(&descriptor.controller_id, &descriptor.action, bytes, |builder| match descriptor.args.as_ref() {
        Some(args) => builder.value(None, args),
        None => {
            builder.begin_object(None)?;
            builder.end_container()
        }
    })?;
    batch.publish()
}

/// 🎟️ Pre-admits a list surface's per-row draw backing on the layer the caller most recently opened.
/// Rows are culled to the visible band before they are drawn, so the reservation is bounded by the
/// surface's own height rather than by its payload's row count — a ten-thousand-row table reserves
/// exactly what a screenful costs.
const LIST_ROW_ITEM_BUDGET: usize = 64;
const LIST_SURFACE_ITEM_CEILING: usize = 16 * 1024;

fn reserve_list_rows(ctx: &mut FrameworkWidgetContext<'_>, rows: usize) -> bool {
    ctx.draw.try_reserve_retained_items(rows.saturating_mul(LIST_ROW_ITEM_BUDGET).min(LIST_SURFACE_ITEM_CEILING)).is_ok()
}

/// 📐️ How many rows of `row_h` a surface of `bounds` can show at once, plus the partial row at each
/// edge — the reservation and the paint cull agree on this count.
fn list_visible_row_capacity(bounds: Rect, row_h: f32) -> usize {
    ((bounds.h / row_h.max(1.0)).ceil().max(0.0) as usize).saturating_add(2)
}

/// 🖱️ The control id the pointer last resolved inside this surface — the hover authority for every
/// list kind. The retained hit registry cannot carry it: a `ComponentScene` leaf registers exactly
/// ONE entry (`retained_scene_hit`), minted AFTER this paint stages its own rows, so
/// `InputState::hovered_id` always answers the surface itself and never one of its rows.
fn scene_hovered_control_id(surface_id: &str) -> Option<String> {
    SCENE_STATE.with(|cell| cell.borrow().get(surface_id).and_then(|state| state.hovered_control_id.clone()))
}

//#endregion SceneRuntime



//#region SceneInput
const MAP_MARQUEE_THRESHOLD_PX: f32 = 6.0;

/// 🖱️ ONE canvas-2d pointer sample in the shape `📐️Canvas2dHost`'s gesture lane publishes it
/// (`🧱️elements/📐️Canvas2dHost/🟦️.tsx:426-446`): `x`/`y` are SCREEN-LOGICAL pixels inside the surface
/// rect and `width`/`height` its logical size, because every plugin reader converts them itself —
/// `🖍️draw`'s `canvas_point_to_world(viewport, x, y, width, height)` and `📏️layout`'s
/// `hit_test_at(doc, cfg, x, y, width, height)`. `world_x`/`world_y` ride along as the optional world
/// lane `🖍️draw`'s own payload already declares, and `surface_id`/`extend` stay because `📏️layout`'s
/// `CanvasPointerDown` reads them.
struct CanvasPointerWire {
    screen_x: f32,
    screen_y: f32,
    world_x: f32,
    world_y: f32,
    width: f32,
    height: f32,
    button: Option<i16>,
    shift: bool,
    cancelled: Option<bool>,
    sampled: bool,
}

impl CanvasPointerWire {
    fn new(inner: Rect, viewport: &Viewport, x: f32, y: f32) -> Self {
        let (world_x, world_y) = viewport.screen_to_world(x, y, inner);
        Self { screen_x: x - inner.x, screen_y: y - inner.y, world_x, world_y, width: inner.w, height: inner.h, button: None, shift: false, cancelled: None, sampled: false }
    }

    fn button(mut self, button: i16, shift: bool) -> Self {
        self.button = Some(button);
        self.shift = shift;
        self
    }

    fn release(mut self, shift: bool) -> Self {
        self.shift = shift;
        self.cancelled = Some(false);
        self
    }

    fn sampled(mut self) -> Self {
        self.sampled = true;
        self
    }
}

const CANVAS_POINTER_WIRE_KEYS: &[&str] = &["surfaceId", "x", "y", "width", "height", "shift", "ctrl", "meta", "alt", "extend", "worldX", "worldY", "button", "cancelled", "samples"];

fn canvas_pointer_action_bytes(scene: &UiComponentSceneNode, action: &str) -> Result<usize, ui_wgpu::wgpu::BoundedActionFault> {
    let mut parts: Vec<&str> = vec![&scene.controller_id, action, &scene.surface_id];
    parts.extend_from_slice(CANVAS_POINTER_WIRE_KEYS);
    ui_wgpu::wgpu::checked_action_string_bytes(&parts)
}

fn write_canvas_pointer_action(batch: &mut ui_wgpu::wgpu::BoundedActionBatchReservation<'_>, scene: &UiComponentSceneNode, action: &str, wire: &CanvasPointerWire) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    let bytes = canvas_pointer_action_bytes(scene, action)?;
    batch.action(&scene.controller_id, action, bytes, |builder| {
        builder.begin_object(None)?;
        builder.string(Some("surfaceId"), &scene.surface_id)?;
        builder.number(Some("x"), f64::from(wire.screen_x))?;
        builder.number(Some("y"), f64::from(wire.screen_y))?;
        builder.number(Some("width"), f64::from(wire.width))?;
        builder.number(Some("height"), f64::from(wire.height))?;
        builder.boolean(Some("shift"), wire.shift)?;
        builder.boolean(Some("ctrl"), false)?;
        builder.boolean(Some("meta"), false)?;
        builder.boolean(Some("alt"), false)?;
        builder.boolean(Some("extend"), wire.shift)?;
        builder.number(Some("worldX"), f64::from(wire.world_x))?;
        builder.number(Some("worldY"), f64::from(wire.world_y))?;
        if let Some(button) = wire.button {
            builder.number(Some("button"), f64::from(button))?;
        }
        if let Some(cancelled) = wire.cancelled {
            builder.boolean(Some("cancelled"), cancelled)?;
        }
        if wire.sampled {
            builder.begin_array(Some("samples"))?;
            builder.begin_array(None)?;
            builder.number(None, f64::from(wire.screen_x))?;
            builder.number(None, f64::from(wire.screen_y))?;
            builder.end_container()?;
            builder.end_container()?;
        }
        builder.end_container()
    })
}

fn canvas_surface_action_bytes(scene: &UiComponentSceneNode, action: &str) -> Result<usize, ui_wgpu::wgpu::BoundedActionFault> {
    ui_wgpu::wgpu::checked_action_string_bytes(&[&scene.controller_id, action, "surfaceId", &scene.surface_id])
}

fn write_canvas_surface_action(batch: &mut ui_wgpu::wgpu::BoundedActionBatchReservation<'_>, scene: &UiComponentSceneNode, action: &str) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    let bytes = canvas_surface_action_bytes(scene, action)?;
    batch.action(&scene.controller_id, action, bytes, |builder| {
        builder.begin_object(None)?;
        builder.string(Some("surfaceId"), &scene.surface_id)?;
        builder.end_container()
    })
}

fn canvas_state_snapshot(surface_id: &str) -> (Viewport, bool, bool) {
    SCENE_STATE.with(|cell| cell.borrow().get(surface_id).map(|state| (state.viewport, matches!(state.drag.as_ref().map(|drag| &drag.mode), Some(SceneDragMode::PanViewport)), state.paint_stroke_active)).unwrap_or((Viewport::default(), false, false)))
}

pub fn canvas_pointer_move_into(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32, down: bool, drag_dx: f32, drag_dy: f32, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    if !inner.contains(x, y) {
        return Ok(false);
    }
    let (viewport, is_pan, _) = canvas_state_snapshot(&scene.surface_id);
    let pan = down && is_pan;
    let action = down && !pan;
    let wire = CanvasPointerWire::new(inner, &viewport, x, y).sampled();
    let mut batch = action
        .then(|| {
            let bytes = canvas_pointer_action_bytes(scene, "canvasPointerMove")?;
            input.reserve_actions(1, bytes)
        })
        .transpose()?;
    if pan {
        mutate_scene_state(&scene.surface_id, |state| {
            state.viewport.x -= drag_dx / viewport.zoom.max(0.01);
            state.viewport.y -= drag_dy / viewport.zoom.max(0.01);
            state.camera_dispatch_controller_id = Some(scene.controller_id.clone());
        });
        schedule_scene_camera_dispatch(&scene.surface_id);
    }
    if let Some(batch) = batch.as_mut() {
        write_canvas_pointer_action(batch, scene, "canvasPointerMove", &wire)?;
    }
    if let Some(batch) = batch {
        batch.publish()?;
    }
    Ok(true)
}

pub fn canvas_pointer_button_into(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32, down: bool, button: i16, shift: bool, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    if !inner.contains(x, y) {
        if !down {
            mutate_scene_state(&scene.surface_id, |state| {
                state.drag = None;
                state.pointer_was_down = false;
            });
        }
        return Ok(false);
    }
    let (viewport, _, paint_stroke_active) = canvas_state_snapshot(&scene.surface_id);
    let base = CanvasPointerWire::new(inner, &viewport, x, y);
    let wire = if down { base.button(button, shift) } else { base.release(shift) };
    let stroke_action = if down && button == 0 {
        Some("paintStrokeBegin")
    } else if !down && paint_stroke_active {
        Some("paintStrokeEnd")
    } else {
        None
    };
    let pointer_action = if down { "canvasPointerDown" } else { "canvasPointerUp" };
    let pointer_bytes = canvas_pointer_action_bytes(scene, pointer_action)?;
    let stroke_bytes = stroke_action.map(|action| canvas_surface_action_bytes(scene, action)).transpose()?.unwrap_or(0);
    let item_credits = 1 + usize::from(stroke_action.is_some());
    let mut batch = input.reserve_actions(item_credits, pointer_bytes.checked_add(stroke_bytes).ok_or(ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?)?;
    if let Some(action) = stroke_action {
        write_canvas_surface_action(&mut batch, scene, action)?;
    }
    write_canvas_pointer_action(&mut batch, scene, pointer_action, &wire)?;
    batch.publish_with(|| {
        mutate_scene_state(&scene.surface_id, |state| {
            state.pointer_was_down = down;
            if down && button == 0 {
                state.paint_stroke_active = true;
            }
            if !down {
                state.paint_stroke_active = false;
                state.drag = None;
            } else if button == 1 || button == 2 {
                state.drag = Some(SceneDrag { mode: SceneDragMode::PanViewport });
            }
        });
    })?;
    Ok(true)
}

pub fn canvas_wheel_into(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32, delta: f32) -> bool {
    if !inner.contains(x, y) {
        return false;
    }
    mutate_scene_state(&scene.surface_id, |state| {
        let factor = (1.0 - delta * 0.001).clamp(0.5, 2.0);
        state.viewport.zoom = (state.viewport.zoom * factor).clamp(0.125, 8.0);
        state.camera_dispatch_controller_id = Some(scene.controller_id.clone());
    });
    schedule_scene_camera_dispatch(&scene.surface_id);
    true
}

pub(crate) fn passive_scene_wheel(scene: &UiComponentSceneNode, bounds: Rect, x: f32, y: f32, delta: f32) -> bool {
    if !bounds.contains(x, y) {
        return false;
    }
    match scene.component_kind {
        SurfaceKind::Table => set_scroll_offset(&scene.surface_id, "body", scroll_offset(&scene.surface_id, "body") + delta * 0.5),
        SurfaceKind::VirtualFileSystem => set_scroll_offset(&scene.surface_id, "vfs", scroll_offset(&scene.surface_id, "vfs") + delta * 0.5),
        SurfaceKind::GraphTimeline => set_scroll_offset(&scene.surface_id, "history", scroll_offset(&scene.surface_id, "history") + delta * 0.5),
        SurfaceKind::DiffView => set_scroll_offset(&scene.surface_id, "diff", scroll_offset(&scene.surface_id, "diff") + delta * 0.5),
        SurfaceKind::EventFeed => set_scroll_offset(&scene.surface_id, "feed", scroll_offset(&scene.surface_id, "feed") + delta * 0.5),
        SurfaceKind::BlockList => set_scroll_offset(&scene.surface_id, "blockList", scroll_offset(&scene.surface_id, "blockList") + delta * 0.5),
        _ => return false,
    }
    true
}

/** @emoji 🖱️ The generic (non-bespoke, non-Canvas2d/Ink/TextEditor/Paint2d) per-event press route.
 * Every list kind resolves the press against its own painted row
 * geometry and publishes the action React's matching host dispatches — a row select, a header sort, a
 * stepper delta, a feed activation, a checkpoint checkout, a block add/remove/move, a virtual file
 * system double-click. The action is published on RELEASE, mirroring a DOM `click`. */
pub(crate) fn passive_scene_pointer_button(scene: &UiComponentSceneNode, bounds: Rect, x: f32, y: f32, down: bool, button: i16, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    if !bounds.contains(x, y) {
        if !down {
            mutate_scene_state(&scene.surface_id, |state| {
                state.pointer_was_down = false;
                state.drag = None;
            });
        }
        return Ok(false);
    }
    if !scene_kind_is_list(scene.component_kind) {
        return Ok(false);
    }
    if down {
        mutate_scene_state(&scene.surface_id, |state| {
            state.pointer_was_down = true;
        });
        return Ok(true);
    }
    mutate_scene_state(&scene.surface_id, |state| {
        state.pointer_was_down = false;
        state.drag = None;
    });
    if button != 0 {
        return Ok(true);
    }
    if let Some(action) = scene_double_click_action(scene, bounds, x, y) {
        write_scene_action(input, &action)?;
        return Ok(true);
    }
    let Some(hit) = scene_list_hit(scene, bounds, x, y, &scene_input_theme(), true) else {
        return Ok(true);
    };
    if let Some(row_id) = hit.toggle_expanded {
        toggle_vfs_row_expanded(&scene.surface_id, &row_id);
        return Ok(true);
    }
    if let Some(action) = hit.action {
        write_scene_action(input, &action)?;
    }
    Ok(true)
}

/** @emoji 🖱️ The generic per-event move route: every list kind's own hover authority
 * (`SceneSurfaceState::hovered_control_id`), which the next paint reads back to highlight the row
 * under the pointer. */
pub(crate) fn passive_scene_pointer_move(scene: &UiComponentSceneNode, bounds: Rect, x: f32, y: f32) -> bool {
    if !bounds.contains(x, y) {
        return false;
    }
    if scene_kind_is_list(scene.component_kind) {
        let hovered = scene_list_hit(scene, bounds, x, y, &scene_input_theme(), false).map(|hit| hit.control_id);
        mutate_scene_state(&scene.surface_id, |state| {
            state.hovered_control_id = hovered;
            state.last_pointer_pos = (x, y);
        });
        return true;
    }
    false
}











/** @emoji 🎯️ One resolved pointer target inside a list-like component scene: the control id the paint
 * drew it under (this module's hover authority — see `scene_hovered_control_id`) and the action a
 * primary-button RELEASE on it dispatches, byte-for-byte what the matching React host's own
 * `onClick`/`onSort`/`onSelectionChange` handler sends. A `None` action is a target that only owns
 * hover/selection state: a stepper's read-only centre segment, a disabled reorder button — React
 * stops propagation on exactly those, so the row click underneath must not fire either. */
struct SceneListHit {
    control_id: String,
    action: Option<ActionDescriptor>,
    /// 📁️ Set when the press landed on a virtual-file-system row's own expand/collapse chevron,
    /// which mutates renderer-local expansion state instead of dispatching a program action.
    toggle_expanded: Option<String>,
}

impl SceneListHit {
    fn row(control_id: String, action: Option<ActionDescriptor>) -> Self {
        Self { control_id, action, toggle_expanded: None }
    }
}

/** @emoji 📃️ Surface kinds this module paints as a scrolling list of rows and resolves pointer input
 * for by re-deriving that same row geometry. They register no usable retained hit target of their own
 * (`retained_scene_hit` mints ONE `HitKind::Generic` entry per `ComponentScene` leaf, and the chrome
 * walk stages it after this paint's rows, so it shadows every one of them) — so the press that lands
 * on the surface arrives here through `UiCommand::Scene` and is resolved against the payload. */
pub(crate) fn scene_kind_is_list(kind: SurfaceKind) -> bool {
    matches!(kind, SurfaceKind::Table | SurfaceKind::VirtualFileSystem | SurfaceKind::GraphTimeline | SurfaceKind::DiffView | SurfaceKind::EventFeed | SurfaceKind::BlockList)
}

/// 🎯️ Resolves `(x, y)` inside a list surface. `activate` is `false` for a hover sample: the action
/// is then not built at all, which keeps a pointer move free of both the allocation and — for the
/// virtual file system — the selection-anchor mutation `vfs_selection_for_click` performs.
fn scene_list_hit(scene: &UiComponentSceneNode, bounds: Rect, x: f32, y: f32, theme: &Theme, activate: bool) -> Option<SceneListHit> {
    match scene.component_kind {
        SurfaceKind::Table => table_hit(scene, bounds, x, y, theme),
        SurfaceKind::VirtualFileSystem => vfs_hit(scene, bounds, x, y, theme, activate),
        SurfaceKind::GraphTimeline => graph_timeline_hit(scene, bounds, y, theme),
        SurfaceKind::EventFeed => event_feed_hit(scene, bounds, y, theme),
        SurfaceKind::BlockList => block_list_hit(scene, bounds, x, y, theme),
        _ => None,
    }
}

/// 🖱️ Double-click edge detection for the one list kind that has a double-click verb (the virtual
/// file system's `navigateUri`). Records this press as the surface's last click either way, so the
/// NEXT press within the 400 ms window can answer it.
fn scene_double_click_action(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32) -> Option<ActionDescriptor> {
    let target = hit_double_click_target(scene, inner, x, y)?;
    let now = now_ms();
    let prior = scene_state(&scene.surface_id);
    let repeat = prior.last_click_target.as_deref() == Some(target.as_str()) && now - prior.last_click_ms < 400.0;
    let action = repeat.then(|| double_click_action(scene, &target, inner, x, y)).flatten();
    mutate_scene_state(&scene.surface_id, |state| {
        state.last_click_target = Some(target);
        state.last_click_ms = now;
    });
    action
}

//#endregion SceneInput

//#region 🖱️SurfaceContextMenu
/** @emoji 🖱️ One component scene's answer to a right-click: the `surface` half of the context-menu
 * request its React host sends — `openSurfaceContextMenu({ menu, surface: { kind, hits, selection },
 * windowInstanceId })` in `🗣️Interpreter/🟦️.tsx:759`. Every field here is per-surface-kind by
 * convention, and the convention is the React host's: a kind that tracks no pick state reports
 * `hits: []`, a kind that tracks no selection reports `selection: []`, and only the text editor
 * carries a `text` block.
 *
 * The wgpu shell owns the CHROME (`ShellState::open_context_menu`) and the plugin round trip; this
 * type is the one thing that has to be resolved per kind, from the same paint geometry each kind's
 * pointer resolver already re-derives. */
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SceneContextMenuTarget {
    pub hits: Vec<ui_wgpu::wgpu::ContextMenuHit>,
    pub selection: Vec<ui_wgpu::wgpu::ContextMenuSelectionGroup>,
    pub text: Option<ui_wgpu::wgpu::ContextMenuTextContext>,
}

/** @emoji 🖱️ Resolves `(x, y)` inside `scene` into that kind's own context-menu surface target.
 * `bounds` is the surface's absolute rect, the same rect its paint and pointer resolvers use, so a
 * row hit here is the row the pointer is actually over.
 *
 * Per-kind, against each React host's own `onContextMenu`:
 * - `table` — the body row under the pointer as `{domain:"row"}`, selection `{domain:"row"}`
 *   (`📊️Table/🟦️.tsx:240`).
 * - `virtualFileSystem` — the visible row under the pointer, selection `{domain:"row"}`
 *   (`🗣️Interpreter/🟦️.tsx:820`).
 * - `eventFeed` — the entry under the pointer as `{domain:"entry"}`, no selection
 *   (`📡️EventFeedHost/🟦️.tsx:89`).
 * - `graphTimeline`/`blockList`/`diffView`/`canvas2d` — whole-surface: no hits, no selection
 *   (`🌳️GraphTimelineHost/🟦️.tsx:50`, `🧩️BlockListHost/🟦️.tsx:212`, `🔺️DiffViewHost/🟦️.tsx:183`,
 *   `📐️Canvas2dHost/🟦️.tsx:912`).
 * - `inkCanvas` — every block under the pointer as `{domain:"block"}`, selection `{domain:"block"}`
 *   (`🖋️InkCanvasHost/🟦️.tsx:1367`).
 * - `paint2d`/`board2d`/`nodeGraph`/`textEditor` — the host's own screen pick targets
 *   (see `engine_canvas`'s `🖱️ContextMenuTargets`).
 * - `tiledMap`/`world3d` — resolved by the shell against their own state maps
 *   (`tiled_map_context_menu_target`, `infinite_world::world::world3d_context_menu_surface`). */
pub fn scene_context_menu_target(scene: &UiComponentSceneNode, bounds: Rect, x: f32, y: f32) -> SceneContextMenuTarget {
    let theme = scene_input_theme();
    match scene.component_kind {
        SurfaceKind::Table => table_context_menu_target(scene, bounds, y, &theme),
        SurfaceKind::VirtualFileSystem => vfs_context_menu_target(scene, bounds, y, &theme),
        SurfaceKind::EventFeed => event_feed_context_menu_target(scene, bounds, y, &theme),
        SurfaceKind::InkCanvas => ink_canvas_context_menu_target(scene, bounds, x, y),
        SurfaceKind::Paint2d => {
            let selection_json = scene.paint_2d.as_ref().map(|paint| paint.selection_json.clone()).unwrap_or_else(|| "[]".into());
            let (hits, selection) = engine_canvas::paint2d_context_menu_target(&scene.surface_id, &selection_json, f64::from(x - bounds.x), f64::from(y - bounds.y));
            SceneContextMenuTarget { hits, selection, text: None }
        }
        SurfaceKind::Board2d => {
            let selection_json = scene.board2d.as_ref().map(|board| board.selection_json.clone()).unwrap_or_else(|| "[]".into());
            let (hits, selection) = engine_canvas::board2d_context_menu_target(&scene.surface_id, &selection_json, f64::from(x - bounds.x), f64::from(y - bounds.y));
            SceneContextMenuTarget { hits, selection, text: None }
        }
        SurfaceKind::NodeGraph => {
            let (hits, selection) = engine_canvas::node_graph_context_menu_target(&scene.surface_id, f64::from(x - bounds.x), f64::from(y - bounds.y));
            SceneContextMenuTarget { hits, selection, text: None }
        }
        SurfaceKind::TextEditor => {
            let (hits, text) = engine_canvas::text_editor_context_menu_target(scene, bounds, x, y);
            SceneContextMenuTarget { hits, selection: Vec::new(), text }
        }
        SurfaceKind::TiledMap => {
            let (hits, selection) = tiled_map_context_menu_target(&scene.surface_id, bounds, x, y);
            SceneContextMenuTarget { hits, selection, text: None }
        }
        _ => SceneContextMenuTarget::default(),
    }
}

/// 📊️ `{domain:"row"}` for the body row under `y`, plus the table's own `selectedIds` — the payload
/// `TableHost`'s `onRowContextMenu` sends (`📊️Table/🟦️.tsx:240`). A press in the HEADER band reports
/// no hit: React only binds the menu on rows.
fn table_context_menu_target(scene: &UiComponentSceneNode, inner: Rect, y: f32, theme: &Theme) -> SceneContextMenuTarget {
    let Some(table) = scene.table.as_ref() else {
        return SceneContextMenuTarget::default();
    };
    let columns: Vec<TableColumn> = serde_json::from_str(&table.columns_json).unwrap_or_default();
    let metrics = table_metrics(inner, columns.len(), theme);
    let rows: Vec<Value> = serde_json::from_str(&table.rows_json).unwrap_or_default();
    let mut hits = Vec::new();
    if y >= metrics.body.y {
        let scroll = scroll_offset(&scene.surface_id, "body");
        if let Ok(index) = usize::try_from(((y - metrics.body.y + scroll) / metrics.row_h.max(1.0)).floor() as i64) {
            if let Some(row) = rows.get(index) {
                hits.push(ui_wgpu::wgpu::ContextMenuHit { domain: "row".into(), id: table_row_id(row, index), label: None });
            }
        }
    }
    let ids: Vec<String> = table
        .selection_json
        .as_deref()
        .and_then(|json| serde_json::from_str::<Value>(json).ok())
        .and_then(|value| value.get("selectedIds").cloned())
        .and_then(|value| serde_json::from_value(value).ok())
        .unwrap_or_default();
    let selection = if ids.is_empty() { Vec::new() } else { vec![ui_wgpu::wgpu::ContextMenuSelectionGroup { domain: "row".into(), ids }] };
    SceneContextMenuTarget { hits, selection, text: None }
}

/// 🗂️ `{domain:"row"}` for the VISIBLE row under `y` (expansion-aware, exactly like `vfs_hit`), plus
/// `selectedRowIdsJson` — the payload `VirtualFileSystemHost`'s `onRowContextMenu` sends
/// (`🗣️Interpreter/🟦️.tsx:820`).
fn vfs_context_menu_target(scene: &UiComponentSceneNode, inner: Rect, y: f32, theme: &Theme) -> SceneContextMenuTarget {
    let Some(vfs) = scene.virtual_file_system.as_ref() else {
        return SceneContextMenuTarget::default();
    };
    let metrics = vfs_metrics(inner, theme);
    let mut hits = Vec::new();
    if y >= metrics.body.y {
        let rows: Vec<Value> = serde_json::from_str(&vfs.rows_json).unwrap_or_default();
        let expanded = scene_state(&scene.surface_id).vfs_expanded_ids;
        let visible = build_vfs_visible_rows(&rows, &expanded);
        let scroll = scroll_offset(&scene.surface_id, "vfs");
        if let Ok(index) = usize::try_from(((y - metrics.body.y + scroll) / metrics.row_h.max(1.0)).floor() as i64) {
            if let Some(entry) = visible.get(index) {
                hits.push(ui_wgpu::wgpu::ContextMenuHit { domain: "row".into(), id: vfs_row_id(&entry.row), label: None });
            }
        }
    }
    let ids: Vec<String> = vfs.selected_row_ids_json.as_deref().and_then(|json| serde_json::from_str(json).ok()).unwrap_or_default();
    let selection = if ids.is_empty() { Vec::new() } else { vec![ui_wgpu::wgpu::ContextMenuSelectionGroup { domain: "row".into(), ids }] };
    SceneContextMenuTarget { hits, selection, text: None }
}

/// 📜️ `{domain:"entry"}` for the log row under `y`, no selection — an `EventFeedScene` tracks none
/// (`📡️EventFeedHost/🟦️.tsx:89`).
fn event_feed_context_menu_target(scene: &UiComponentSceneNode, inner: Rect, y: f32, theme: &Theme) -> SceneContextMenuTarget {
    let Some(feed) = scene.event_feed.as_ref() else {
        return SceneContextMenuTarget::default();
    };
    let entries: Vec<EventFeedEntryJson> = serde_json::from_str(&feed.entries_json).unwrap_or_default();
    let row_h = theme.control_height;
    let mut top = inner.y - scroll_offset(&scene.surface_id, "feed");
    for entry in &entries {
        let entry_h = event_feed_row_height(entry, row_h, theme);
        if y >= top && y < top + entry_h {
            return SceneContextMenuTarget { hits: vec![ui_wgpu::wgpu::ContextMenuHit { domain: "entry".into(), id: entry.id.clone(), label: None }], selection: Vec::new(), text: None };
        }
        top += entry_h;
    }
    SceneContextMenuTarget::default()
}

/** @emoji 🖋️ Every ink block under the pointer as `{domain:"block"}`, topmost first, plus the
 * selection the menu applies to — React's `inkItemsAtPoint` + the "right-click outside the selection
 * selects the topmost hit instead" rule (`🖋️InkCanvasHost/🟦️.tsx:1355-1367`). Unlike React this does
 * NOT dispatch `setSelection` as a side effect: the wgpu ink lane commits every selection write
 * through the bounded `InkInteractionJob`, and a right-click resolving a menu is not a commit. */
fn ink_canvas_context_menu_target(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32) -> SceneContextMenuTarget {
    let Some(ink) = scene.ink_canvas.as_ref() else {
        return SceneContextMenuTarget::default();
    };
    let Ok(document) = serde_json::from_str::<Value>(&ink.document_json) else {
        return SceneContextMenuTarget::default();
    };
    let scene_camera = document.get("camera").cloned().and_then(|camera| serde_json::from_value::<InkCameraJson>(camera).ok()).map(InkCameraF::from).unwrap_or_default();
    let camera = scene_state(&scene.surface_id).ink_camera.map(|(x, y, zoom)| InkCameraF { x, y, zoom }).unwrap_or(scene_camera);
    let (world_x, world_y) = ink_screen_to_world(camera, inner, x, y);
    let selected: Vec<String> = serde_json::from_str(&ink.selection_json).unwrap_or_default();
    let blocks = document.get("blocks").and_then(Value::as_array).cloned().unwrap_or_default();
    let mut hits = Vec::new();
    for block in blocks.iter().rev() {
        if hits.len() >= INK_INTERACTION_ITEM_CAPACITY {
            break;
        }
        let Some(id) = block.get("id").and_then(Value::as_str) else { continue };
        if ink_hits_point(block, world_x, world_y, INK_CONTEXT_MENU_HIT_THRESHOLD) {
            hits.push(ui_wgpu::wgpu::ContextMenuHit { domain: "block".into(), id: id.to_string(), label: None });
        }
    }
    let ids = match hits.first() {
        Some(top) if !selected.iter().any(|id| id == &top.id) => vec![top.id.clone()],
        _ => selected,
    };
    let selection = if ids.is_empty() { Vec::new() } else { vec![ui_wgpu::wgpu::ContextMenuSelectionGroup { domain: "block".into(), ids }] };
    SceneContextMenuTarget { hits, selection, text: None }
}

#[cfg(test)]
#[path = "../../🧪️tests/🔬️wgpu-surface-context-menu/🦀️.rs"]
mod surface_context_menu_tests;

/// 🖋️ The world-space slack a right-click gets when picking an ink block — React's own
/// `inkItemsAtPoint` tolerance.
const INK_CONTEXT_MENU_HIT_THRESHOLD: f64 = 6.0;

/** @emoji 🏷️ The context-menu vocabulary id for a surface kind — the string React sends as BOTH
 * `menu.id` and `surface.kind` (`openSurfaceContextMenu({ menu: { id: "virtualFileSystem" },
 * surface: { kind: "virtualFileSystem" } })`). It is deliberately NOT `SurfaceKind`'s serde tag: the
 * scene wire is kebab-case (`virtual-file-system`), while the menu vocabulary a plugin matches on —
 * and `contextMenuSurfaceTitleKeys` keys itself by (`🗣️Interpreter/🟦️.tsx:712`) — is camelCase. */
pub fn context_menu_surface_kind_id(kind: SurfaceKind) -> &'static str {
    match kind {
        SurfaceKind::BlockList => "blockList",
        SurfaceKind::Board2d => "board2d",
        SurfaceKind::Canvas2d => "canvas2d",
        SurfaceKind::DiffView => "diffView",
        SurfaceKind::EventFeed => "eventFeed",
        SurfaceKind::GraphTimeline => "graphTimeline",
        SurfaceKind::IconRender => "iconRender",
        SurfaceKind::InkCanvas => "inkCanvas",
        SurfaceKind::NodeGraph => "nodeGraph",
        SurfaceKind::Paint2d => "paint2d",
        SurfaceKind::Table => "table",
        SurfaceKind::TextEditor => "textEditor",
        SurfaceKind::TiledMap => "tiledMap",
        SurfaceKind::VirtualFileSystem => "virtualFileSystem",
        SurfaceKind::World3d => "world3d",
    }
}
//#endregion 🖱️SurfaceContextMenu

//#region RenderEntry
fn render_placeholder(kind: &str, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let theme = ctx.theme;
    draw_text(ctx, &format!("{kind} host"), bounds.x + 12.0, bounds.y + 24.0, theme.font_size_body, theme.text_muted);
}

/// 🎞️ Advances one retained scene identifier scalar or one pre-admitted chrome output item.
/// 🧩️ The per-frame engine state the retained scene paint needs beyond its own draw list: the shell's
/// `World3dState` map (a `World3d` surface's host, addressed there by every pick/asset/authority
/// ladder) and this frame's `World3dBuildContext` (built in `FrameBuildPhase::WorldResources`, drained
/// again in `FrameBuildPhase::WorldTransfer`). Borrowed for exactly one chrome walk and never stored —
/// the same reason `Ui::frame` takes its `SceneHost` as a parameter rather than owning one.
pub struct SceneEngineHosts<'a> {
    pub world3d_states: &'a mut AdmittedSurfaceMap<infinite_world::world::World3dState>,
    pub world_resources: &'a mut infinite_world::world::World3dBuildContext,
    /// 🪟️ The window instance whose body this walk is painting — the retention authority every
    /// attached engine surface is recorded against, so the shell can keep a surface alive through
    /// frames its window did not repaint. See
    /// `🧑‍🎨engine/🧫️fixtures/🧲️engine-surface-retention/🔣️.json`.
    pub window_id: &'a str,
}

/// 🍿️ The editor's own popup chrome, painted LAST so it lands over the composited
/// `EditorHost` texture phase 6 staged — React's absolutely-positioned overlays inside the
/// host element (`🧱️elements/✏️TextEditor/🟦️.tsx:503-608`).
pub fn render_component_scene_step(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>, cursor: &mut ui_wgpu::wgpu::ScenePaintCursor, hosts: &mut SceneEngineHosts<'_>) -> ui_wgpu::wgpu::ScenePaintStep {
    if cursor.phase() >= ENGINE_PHASE {
        match scene.component_kind {
            SurfaceKind::World3d => return render_world3d_surface_step(scene, bounds, ctx, cursor, hosts),
            SurfaceKind::Canvas2d => return render_canvas_2d_step(scene, bounds, ctx, cursor),
            SurfaceKind::InkCanvas => return render_ink_canvas_step(scene, bounds, ctx, cursor),
            SurfaceKind::IconRender => return render_icon_render_step(scene, bounds, ctx, cursor, hosts),
            kind if scene_kind_is_list(kind) => return render_list_scene_step(scene, bounds, ctx, cursor),
            _ => {}
        }
    }
    match cursor.phase() {
        0 => {
            if scene.surface_id.len() > SCENE_SURFACE_ID_BYTE_CAPACITY {
                return ui_wgpu::wgpu::ScenePaintStep::Fault;
            }
            if cursor.byte() < scene.surface_id.len() {
                if cursor.advance_byte().is_err() {
                    return ui_wgpu::wgpu::ScenePaintStep::Fault;
                }
                return ui_wgpu::wgpu::ScenePaintStep::Pending;
            }
            if cursor.advance_phase().is_err() {
                return ui_wgpu::wgpu::ScenePaintStep::Fault;
            }
            ui_wgpu::wgpu::ScenePaintStep::Pending
        }
        1 => {
            if scene.controller_id.len() > SCENE_SURFACE_ID_BYTE_CAPACITY {
                return ui_wgpu::wgpu::ScenePaintStep::Fault;
            }
            if cursor.byte() < scene.controller_id.len() {
                if cursor.advance_byte().is_err() {
                    return ui_wgpu::wgpu::ScenePaintStep::Fault;
                }
                return ui_wgpu::wgpu::ScenePaintStep::Pending;
            }
            if cursor.advance_phase().is_err() {
                return ui_wgpu::wgpu::ScenePaintStep::Fault;
            }
            ui_wgpu::wgpu::ScenePaintStep::Pending
        }
        2 => {
            ctx.draw.set_screen_height(bounds.y + bounds.h);
            remember_scene_theme(ctx.theme);
            if cursor.advance_phase().is_err() {
                return ui_wgpu::wgpu::ScenePaintStep::Fault;
            }
            ui_wgpu::wgpu::ScenePaintStep::Pending
        }
        3 => {
            if ctx.draw.try_reserve_retained_items(1).is_err() {
                return ui_wgpu::wgpu::ScenePaintStep::Fault;
            }
            ctx.draw.push_rounded([bounds.x, bounds.y, bounds.w, bounds.h], ctx.theme.panel, ctx.theme.border_radius);
            if cursor.advance_item().is_err() || cursor.advance_phase().is_err() {
                return ui_wgpu::wgpu::ScenePaintStep::Fault;
            }
            ui_wgpu::wgpu::ScenePaintStep::Pending
        }
        4 => {
            if !engine_canvas::sync_engine_scene(scene, hosts.window_id, bounds, ctx.theme) {
                return cursor.finish();
            }
            if cursor.advance_phase().is_err() {
                return ui_wgpu::wgpu::ScenePaintStep::Fault;
            }
            ui_wgpu::wgpu::ScenePaintStep::Pending
        }
        5 => {
            if !engine_canvas::stage_engine_scene_paint(scene, bounds, engine_surface_clear(scene.component_kind, ctx.theme)) {
                return cursor.finish();
            }
            if cursor.advance_phase().is_err() {
                return ui_wgpu::wgpu::ScenePaintStep::Fault;
            }
            ui_wgpu::wgpu::ScenePaintStep::Pending
        }
        6 => {
            let Some(key) = engine_canvas::engine_raster_key(&scene.surface_id) else {
                return ui_wgpu::wgpu::ScenePaintStep::Fault;
            };
            if ctx.draw.try_reserve_retained_items(1).is_err() {
                return ui_wgpu::wgpu::ScenePaintStep::Fault;
            }
            ctx.draw.push_raster_quad(&key, [bounds.x, bounds.y, bounds.w, bounds.h], [0.0, 0.0, 1.0, 1.0], 1.0);
            if cursor.advance_item().is_err() || cursor.advance_phase().is_err() {
                return ui_wgpu::wgpu::ScenePaintStep::Fault;
            }
            ui_wgpu::wgpu::ScenePaintStep::Pending
        }
        7 => {
            if scene.component_kind == SurfaceKind::NodeGraph {
                engine_canvas::paint_node_graph_labels(ctx, scene, bounds);
            } else if scene.component_kind != SurfaceKind::TextEditor {
                return cursor.finish();
            }
            if cursor.advance_phase().is_err() {
                return ui_wgpu::wgpu::ScenePaintStep::Fault;
            }
            ui_wgpu::wgpu::ScenePaintStep::Pending
        }
        8 => {
            if scene.component_kind == SurfaceKind::NodeGraph {
                engine_canvas::paint_node_graph_overlays(ctx, scene, bounds);
            }
            if cursor.advance_phase().is_err() {
                return ui_wgpu::wgpu::ScenePaintStep::Fault;
            }
            ui_wgpu::wgpu::ScenePaintStep::Pending
        }
        9 => {
            if scene.component_kind != SurfaceKind::TextEditor {
                return cursor.finish();
            }
            if ctx.draw.try_reserve_retained_items(TEXT_EDITOR_OVERLAY_ITEMS).is_err() {
                return ui_wgpu::wgpu::ScenePaintStep::Fault;
            }
            render_text_editor_overlays(scene, bounds, ctx);
            cursor.finish()
        }
        _ => cursor.finish(),
    }
}


/// 🎞️ The first cursor phase past the identifier/metrics/panel preamble — where a scene kind's own
/// content paint begins, whether that is an engine-texture attach (`sync_engine_scene`) or a direct
/// retained draw (`World3d`, `Canvas2d`, `InkCanvas`, `IconRender`).
const ENGINE_PHASE: u16 = 4;

/** 📃️ Paints one list-like component scene — `Table`, `VirtualFileSystem`, `GraphTimeline`,
 * `BlockList`, `DiffView`, `EventFeed` — straight into the window's retained draw list. Like the
 * `World3d`/`Canvas2d` branches there is no engine texture to attach: the whole surface is chrome,
 * so attach, paint and composite are one step. The per-row backing is pre-admitted first
 * (`reserve_list_rows`), sized from the surface's own visible band rather than its payload's row
 * count, because every one of these renderers culls to that band before it draws. */
fn render_list_scene_step(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>, cursor: &mut ui_wgpu::wgpu::ScenePaintCursor) -> ui_wgpu::wgpu::ScenePaintStep {
    let capacity = list_visible_row_capacity(bounds, ctx.theme.control_height);
    if !reserve_list_rows(ctx, capacity) {
        return ui_wgpu::wgpu::ScenePaintStep::Fault;
    }
    match scene.component_kind {
        SurfaceKind::Table => render_table(scene, bounds, ctx),
        SurfaceKind::VirtualFileSystem => render_vfs(scene, bounds, ctx),
        SurfaceKind::GraphTimeline => render_graph_timeline(scene, bounds, ctx),
        SurfaceKind::BlockList => render_block_list(scene, bounds, ctx),
        SurfaceKind::DiffView => render_diff_view(scene, bounds, ctx),
        SurfaceKind::EventFeed => render_event_feed(scene, bounds, ctx),
        _ => return ui_wgpu::wgpu::ScenePaintStep::Fault,
    }
    cursor.finish()
}

/** 🖼️ Paints one `SurfaceKind::Canvas2d` scene straight into the window's retained draw list — no
 * engine texture, because a canvas-2d document IS a draw list (`drawSceneNode` in React walks the
 * same records into `CanvasRenderingContext2D` calls). Two phases: the content ladder, then the
 * surface hit target the generic pointer dispatch is admitted through.
 *
 * @see `🧱️elements/📐️Canvas2dHost/🟦️.tsx` — `drawSceneNode` */
fn render_canvas_2d_step(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>, cursor: &mut ui_wgpu::wgpu::ScenePaintCursor) -> ui_wgpu::wgpu::ScenePaintStep {
    match cursor.phase() {
        ENGINE_PHASE => {
            if ctx.draw.try_reserve_retained_items(canvas_2d_reserve_items(scene)).is_err() {
                return ui_wgpu::wgpu::ScenePaintStep::Fault;
            }
            render_canvas_2d(scene, bounds, ctx);
            if cursor.advance_phase().is_err() {
                return ui_wgpu::wgpu::ScenePaintStep::Fault;
            }
            ui_wgpu::wgpu::ScenePaintStep::Pending
        }
        _ => cursor.finish(),
    }
}

/// 🧾️ The retained-item grant one canvas-2d paint needs: grid + checkerboard ladder, plus a fixed
/// per-layer worst case (fill, outline, selection glow/ring, label).
fn canvas_2d_reserve_items(scene: &UiComponentSceneNode) -> usize {
    let layers = scene.canvas_2d.as_ref().map_or(0, |canvas| canvas.layers_json.len() / 64);
    CANVAS_2D_CHROME_ITEMS.saturating_add(layers.saturating_mul(CANVAS_2D_ITEMS_PER_LAYER))
}

const CANVAS_2D_CHROME_ITEMS: usize = 4096;
const CANVAS_2D_ITEMS_PER_LAYER: usize = 8;

/** 🖋️ Paints one `SurfaceKind::InkCanvas` scene straight into the window's retained draw list —
 * strokes, grid, block cards and the live marquee rectangle, all under the surface's own scissor.
 *
 * @see `🧱️elements/🖋️InkCanvasHost/🟦️.tsx` */
fn render_ink_canvas_step(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>, cursor: &mut ui_wgpu::wgpu::ScenePaintCursor) -> ui_wgpu::wgpu::ScenePaintStep {
    match cursor.phase() {
        ENGINE_PHASE => {
            if ctx.draw.try_reserve_retained_items(INK_CANVAS_RESERVE_ITEMS).is_err() {
                return ui_wgpu::wgpu::ScenePaintStep::Fault;
            }
            render_ink_canvas(scene, bounds, ctx);
            if cursor.advance_phase().is_err() {
                return ui_wgpu::wgpu::ScenePaintStep::Fault;
            }
            ui_wgpu::wgpu::ScenePaintStep::Pending
        }
        _ => cursor.finish(),
    }
}

/// 🧾️ The retained-item grant one ink paint needs — the grid ladder dominates, and the block count
/// is already capped by `INK_INTERACTION_ITEM_CAPACITY`.
const INK_CANVAS_RESERVE_ITEMS: usize = 8192;

/** 🖼️ Paints one `SurfaceKind::IconRender` scene: the GLB shot goes through the SAME world-3d pass
 * every `World3d` surface uses (React's `iconRenderPort` renders offscreen with the same renderer),
 * and the aspect-fit frame, size/shape badge and footer caption paint on top of it.
 *
 * @see `🧱️elements/🖼️IconRenderHost/🟦️.tsx` — `IconShotFrame` */
fn render_icon_render_step(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>, cursor: &mut ui_wgpu::wgpu::ScenePaintCursor, hosts: &mut SceneEngineHosts<'_>) -> ui_wgpu::wgpu::ScenePaintStep {
    match cursor.phase() {
        ENGINE_PHASE => {
            if ctx.draw.try_reserve_retained_items(ICON_RENDER_RESERVE_ITEMS).is_err() {
                return ui_wgpu::wgpu::ScenePaintStep::Fault;
            }
            render_icon_render(scene, bounds, ctx, hosts);
            if cursor.advance_phase().is_err() {
                return ui_wgpu::wgpu::ScenePaintStep::Fault;
            }
            ui_wgpu::wgpu::ScenePaintStep::Pending
        }
        _ => cursor.finish(),
    }
}

/// 🧾️ Frame border strips, badge chip, badge text and footer caption.
const ICON_RENDER_RESERVE_ITEMS: usize = 256;

/// 🎨️ The colour a composited engine texture clears to before its host paints — the panel behind a
/// node graph, the canvas ground behind a map or a board, exactly as each React host's own
/// `clear_color` resolves it.
fn engine_surface_clear(kind: SurfaceKind, theme: &ui_wgpu::wgpu::Theme) -> ui_wgpu::wgpu::Rgba {
    match kind {
        SurfaceKind::TiledMap | SurfaceKind::Board2d | SurfaceKind::Paint2d => theme.canvas_clear,
        _ => theme.panel,
    }
}

/// 🌍️ Attaches — and paints — the `World3dState` behind one `SurfaceKind::World3d` scene. Unlike the
/// vello-composited kinds this host paints a real 3-D pass straight into the window's own retained
/// draw list (`render_world_3d` → `DrawList::push_scene_pass`), so there is no engine texture and no
/// raster key: attach, paint and composite are the same step. The state lives in the shell's
/// `world3d_states` because every downstream ladder — asset fetch, snapshot apply, bounded pick/hover
/// authority, the OS event loop's orbit/wheel dispatch — addresses it there.
fn render_world3d_surface_step(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>, cursor: &mut ui_wgpu::wgpu::ScenePaintCursor, hosts: &mut SceneEngineHosts<'_>) -> ui_wgpu::wgpu::ScenePaintStep {
    let created = !hosts.world3d_states.contains_key(&scene.surface_id);
    let surface_id = scene.surface_id.clone();
    let controller_id = scene.controller_id.clone();
    let Some(state) = hosts.world3d_states.get_or_insert_with(surface_id.clone(), || infinite_world::world::World3dState::new(surface_id, controller_id)) else {
        return ui_wgpu::wgpu::ScenePaintStep::Fault;
    };
    if state.tool_run_trace_window_id.as_deref() != Some(hosts.window_id) {
        state.tool_run_trace_window_id = Some(hosts.window_id.to_string());
    }
    infinite_world::world::render_world_3d(scene, bounds, ctx, state, hosts.world_resources);
    engine_canvas::register_engine_surface(scene, hosts.window_id, bounds, engine_canvas::EngineSurfaceKindDetail::World3d { status_json: scene.world_3d.as_ref().and_then(|world| world.status_json.clone()) }, created);
    world3d_surface_debug_log(scene, bounds, ctx, state);
    // 🎥️ The one point per frame that holds both the surface id and its LIVE orbit — `dumpMeshStats`
    // reaches only the interpreter's `UI_ENGINE`, never the shell's `world3d_states`.
    crate::interpreter::note_world3d_live_camera(&scene.surface_id, infinite_world::world::world3d_live_camera_json(state));
    cursor.finish()
}

/// 🌍️ `[DEBUG] ` trace of what the World3d pass this step just pushed actually carries — the pass's
/// mesh draws, their instances, its line draws and the surface it belongs to. Temporary: it is the
/// only way to tell "no surface" from "a surface with no meshes" on 6118, where `eprintln!` is a
/// no-op inside the frame Worker.
fn world3d_surface_debug_log(scene: &UiComponentSceneNode, bounds: Rect, ctx: &FrameworkWidgetContext<'_>, state: &infinite_world::world::World3dState) {
    let payload = match scene.world_3d.as_ref() {
        Some(world) => format!(
            "meshes={}b instances={}b delta={}b lanes=[{}] snapshot={} camera={}b",
            world.meshes_json.len(),
            world.instances_json.len(),
            world.instances_delta_json.as_ref().map_or(0, String::len),
            world.lanes.iter().map(|lane| format!("{}:{}", lane.lane, lane.bytes)).collect::<Vec<_>>().join(","),
            world.snapshot.is_some(),
            world.camera_json.len()
        ) + &format!(
            " meshesHead={:?} instancesHead={:?} camera={:?} status={:?}",
            &world.meshes_json[..world.meshes_json.len().min(700)],
            &world.instances_json[..world.instances_json.len().min(700)],
            &world.camera_json[..world.camera_json.len().min(120)],
            world.status_json.as_deref().map(|status| &status[..status.len().min(800)])
        ),
        None => "world3d=none".to_string(),
    };
    let geometry = match ctx.draw.scene_passes.last() {
        Some(pass) => format!(
            "draws={} translucent={} instances={} lines={} textured={} passes={}",
            pass.draws.len(),
            pass.translucent_draws.len(),
            pass.draws.iter().chain(pass.translucent_draws.iter()).map(|draw| draw.instances.len()).sum::<usize>(),
            pass.line_draws.len(),
            pass.textured_draws.len(),
            ctx.draw.scene_passes.len()
        ),
        None => "pass=none".to_string(),
    };
    // 📐️ The ORIGIN belongs in this trace as much as the size: every pointer, wheel and pick the
    // surface receives is admitted by `bounds.contains(x, y)` in PAGE space, so a rect reported only
    // as `WxH` cannot be told apart from the same rect at the wrong origin — which is the shape a
    // silently undispatched hover/select/orbit takes (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
    // `📓️wgpu-input-hit-runtime-2026-09-13.md` §10.5).
    debug_log_diagnostic(&format!(
        "[DEBUG] world3d surface={} pane={:?} bounds={}x{}+{},{} {geometry} {} {} {payload}",
        scene.surface_id,
        scene.pane_id,
        bounds.w.round(),
        bounds.h.round(),
        bounds.x.round(),
        bounds.y.round(),
        state.ingest_census(),
        state.mesh_geometry_census()
    ));
}

fn debug_log(line: &str) {
    #[cfg(target_arch = "wasm32")]
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(line));
    #[cfg(not(target_arch = "wasm32"))]
    eprintln!("{line}");
}

/// 🩺️ A PER-FRAME dump — printed only while `SEMIO_RUNTIME_DIAGNOSTICS` is armed, the same gate
/// `crate::log_debug_diagnostic` applies in the renderer. One `world3d surface=…` census per surface
/// per frame is two lines a frame on this playground alone, which is what buried the asset-decoder
/// boot fault (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w3a-asset-decoder-boot-fault.md`).
fn debug_log_diagnostic(line: &str) {
    if semio_framework_trace::runtime_diagnostics_enabled() {
        debug_log(line);
    }
}

/** @emoji 🧭️ Surface kinds that already receive pointer/wheel input through their own bespoke per-frame host state (`world3d_states`/`node_graph_states`/`tiled_map_states`/`board2d_states`, driven directly by the OS event loop) and must not be double-dispatched through the generic `handle_scene_*` handlers below. `pub(crate)` so `interpreter::apply_scene_ui_command` (the real per-event `UiCommand::Scene` handler, and now the ONLY caller of `handle_scene_wheel`/`handle_scene_pointer_button`/`handle_scene_pointer_move` — see that fn's own doc comment) applies this SAME exclusion list. */
pub(crate) fn scene_has_bespoke_pointer_dispatch(kind: SurfaceKind) -> bool {
    matches!(kind, SurfaceKind::World3d | SurfaceKind::NodeGraph | SurfaceKind::TiledMap | SurfaceKind::Board2d)
}

#[cfg(test)]
#[path = "../../🧪️tests/🔬️wgpu-render-entry/🦀️.rs"]
mod render_entry_tests;
//#endregion RenderEntry



//#region Paint2d
// 🖌️ Production paint-2d rendering and input live on `RasterHost`
// (`🔨️modules/🗺️surface/🎨️paint/🦀️.rs`, attached by `engine_canvas::sync_paint2d_scene`) — the SAME
// host React's wasm `RasterSession` wraps. What remains in this region is the fixture-side document
// projection the standalone tests read.
#[cfg(test)]
#[derive(Deserialize, Clone, Copy)]
struct Paint2dCameraFields {
    #[serde(default)]
    x: f64,
    #[serde(default)]
    y: f64,
    #[serde(default = "paint2d_default_one")]
    zoom: f64,
}

#[cfg(test)]
impl Default for Paint2dCameraFields {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

#[cfg(test)]
fn paint2d_default_one() -> f64 {
    1.0
}

#[cfg(test)]
#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct Paint2dDocSyncJson {
    #[serde(default)]
    camera: Paint2dCameraFields,
}

#[cfg(test)]
struct Paint2dFlatLayer {
    x: f64,
    y: f64,
    scale_x: f64,
    scale_y: f64,
    width: u32,
    height: u32,
}

//#region Paint2dNavigator
#[cfg(test)]
const PAINT2D_NAVIGATOR_PADDING: f32 = 24.0;

/** 🧭️ Fits a camera to the document's pixel-layer bounds so a `viewMode === "navigator"` surface
 * shows the whole composition — a port of `RasterHost::navigator_fit_camera_json`
 * (`🔨️modules/🗺️surface/🎨️paint/🦀️.rs`'s `RasterHost::navigator_fit_camera_json`, which production
 * now calls directly through the attached host). Falls back to a neutral centered camera when the
 * document has no pixel content. */


/** 🧭️ Maps the main (composite) viewport's visible world rect into the navigator's own fitted
 * screen space, producing the "you are here" overlay rectangle — a port of
 * `RasterHost::navigator_viewport_overlay_json`. `content_camera_json` is the main surface's
 * `Paint2dScene.cameraJson` (echoed into the navigator's own scene payload by the owning program) and
 * `content_viewport_json` its reported `compositeViewportJson` (`{width,height}` in CSS px, set via
 * the React reference's `setCompositeViewport` action / this renderer's `ResizeObserver` equivalent
 * — see report for the pointer/resize wiring gap notes). Returns `None` when the main viewport size
 * hasn't been reported yet. */

//#endregion Paint2dNavigator

//#endregion Paint2d

//#region Table
#[derive(Deserialize)]
struct TableColumn {
    id: String,
    label: String,
    #[serde(default)]
    sortable: bool,
}

/// 🔀️ Mirrors `sourcing::TableSort`'s wire format (`{columnId, direction}`) — the active sort
/// state for a [`TableScene`] whose columns opt into `sortable`.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TableSortJson {
    column_id: String,
    direction: String,
}

/// 🧾️ Mirrors `ui_wgpu::wgpu::TableCell` — a typed table cell value parsed out of a row's raw JSON.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
enum TableCellPayload {
    Text { value: String },
    Number { value: f64 },
    Stepper { value: f64, min: f64, max: f64, step: f64, action: ActionDescriptor },
    Buttons { buttons: Vec<TableCellButtonPayload> },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TableCellButtonPayload {
    icon_id: String,
    #[serde(default)]
    label: Option<String>,
    action: ActionDescriptor,
    /// 🔘️ `"row"` (the default) draws the button in the row itself; `"menu"` moves it into the row's
    /// context menu instead — the same split `renderTableCell`/`tableRowMenuPlacementItems` make.
    #[serde(default)]
    placement: Option<String>,
}

/// 🔗️ Merges `patch` into `base`'s existing args (rather than replacing them), so a stepper/button
/// cell keeps its row-identifying args (e.g. `objectId`) alongside the delta/click patch — the port
/// of `dispatchCellAction`'s `{ ...descriptor.args, ...patch }` spread in `📊️Table/🟦️.tsx`.
fn merge_action_args(base: &ActionDescriptor, patch: Value) -> ActionDescriptor {
    let mut args = match &base.args {
        Some(dsl) => match Value::from(dsl) {
            Value::Object(map) => map,
            _ => serde_json::Map::new(),
        },
        None => serde_json::Map::new(),
    };
    if let Value::Object(patch_map) = patch {
        args.extend(patch_map);
    }
    ActionDescriptor { controller_id: base.controller_id.clone(), action: base.action.clone(), args: semio_framework::optional_json_to_dsl(Some(Value::Object(args))) }
}

/// 🪪️ A row's identity, exactly as `TableHost`'s `rowIds` map derives it: `id`, then `pluginId`,
/// then the row's own ordinal. The ordinal fallback matters — a payload whose rows carry neither key
/// used to collapse every row onto the empty id, so selection highlighted all of them at once.
fn table_row_id(row: &Value, index: usize) -> String {
    row.get("id").or_else(|| row.get("pluginId")).and_then(Value::as_str).map(str::to_string).unwrap_or_else(|| index.to_string())
}

/// 🖱️ What a row click dispatches. A table that declares an interaction domain picks into it
/// (`interactionSelect`, with `targets` a JSON STRING of `[{granularity, id}]` — the wire shape the
/// framework's interaction bus reads); a plugin-private table keeps its own `selectRow` verb.
/// Ported from `TableHost`'s `onRowClick`.
fn table_row_action(scene: &UiComponentSceneNode, table: &ui_wgpu::wgpu::TableScene, row: &Value, row_id: &str) -> ActionDescriptor {
    match (table.domain_id.as_deref(), table.domain_granularity_id.as_deref()) {
        (Some(domain_id), Some(granularity)) => scene_action(
            scene,
            "interactionSelect",
            json!({ "domainId": domain_id, "targets": json!([{ "granularity": granularity, "id": row_id }]).to_string(), "merge": "replace", "method": "pick" }),
        ),
        _ => scene_action(scene, "selectRow", json!({ "surfaceId": scene.surface_id, "row": row })),
    }
}

/// 🔘️ The buttons a `buttons` cell draws IN the row — `placement: "menu"` entries belong to the
/// row's context menu instead, exactly as `renderTableCell` filters them.
fn table_row_buttons(buttons: &[TableCellButtonPayload]) -> Vec<&TableCellButtonPayload> {
    buttons.iter().filter(|button| button.placement.as_deref().unwrap_or("row") == "row").collect()
}

/// 🧾️ Renders a table cell's interactive controls (stepper/buttons) directly, or returns the plain
/// text to draw for text/number/legacy-string cells.
fn render_table_cell(cell: &Value, rect: Rect, ctx: &mut FrameworkWidgetContext<'_>) -> Option<String> {
    let Ok(payload) = serde_json::from_value::<TableCellPayload>(cell.clone()) else {
        return Some(match cell {
            Value::String(s) => s.clone(),
            other => other.to_string(),
        });
    };
    match payload {
        TableCellPayload::Text { value } => Some(value),
        TableCellPayload::Number { value } => Some(value.to_string()),
        TableCellPayload::Stepper { value, min, max, step, action } => {
            let seg = rect.w / 3.0;
            let minus = Rect::new(rect.x, rect.y, seg, rect.h);
            let center = Rect::new(rect.x + seg, rect.y, seg, rect.h);
            let plus = Rect::new(rect.x + seg * 2.0, rect.y, seg, rect.h);
            render_widget(&WidgetNode::Button { id: None, icon_id: None, label: "−".into(), event: (value > min).then(|| merge_action_args(&action, json!({ "delta": -step }))) }, minus, ctx);
            render_widget(&WidgetNode::Text { value: format!("{value:.0}"), emphasize: false }, center, ctx);
            render_widget(&WidgetNode::Button { id: None, icon_id: None, label: "+".into(), event: (value < max).then(|| merge_action_args(&action, json!({ "delta": step }))) }, plus, ctx);
            None
        }
        TableCellPayload::Buttons { buttons } => {
            let row_buttons = table_row_buttons(&buttons);
            if row_buttons.is_empty() {
                return None;
            }
            let seg = rect.w / row_buttons.len() as f32;
            for (index, button) in row_buttons.iter().enumerate() {
                let button_rect = Rect::new(rect.x + index as f32 * seg, rect.y, seg, rect.h);
                render_widget(&WidgetNode::Button { id: None, icon_id: IconName::from_str(&button.icon_id), label: button.label.clone().unwrap_or_default(), event: Some(button.action.clone()) }, button_rect, ctx);
            }
            None
        }
    }
}

/// 📐️ A table's fixed metrics — the ONE derivation the paint lays rows out with and the pointer
/// path re-derives hits from, so a header band, a row band and their hit bands cannot drift.
struct TableMetrics {
    header_h: f32,
    row_h: f32,
    pad: f32,
    col_w: f32,
    body: Rect,
}

fn table_metrics(inner: Rect, columns: usize, theme: &Theme) -> TableMetrics {
    let header_h = theme.control_height * 1.33;
    TableMetrics {
        header_h,
        row_h: theme.control_height,
        pad: theme.padding_standard,
        col_w: if columns == 0 { inner.w } else { inner.w / columns as f32 },
        body: Rect::new(inner.x, inner.y + header_h, inner.w, inner.h - header_h),
    }
}

/// 🔀️ The direction a press on `column`'s header asks for next — ascending unless this column is
/// already the ascending sort, matching `Table`'s own `onSort` toggle.
fn table_next_sort_direction(sort: Option<&TableSortJson>, column_id: &str) -> &'static str {
    match sort.filter(|sort| sort.column_id == column_id) {
        Some(sort) if sort.direction == "asc" => "desc",
        _ => "asc",
    }
}

/// 🎯️ The cell control a press at `x` lands on inside `cell_rect`, or `None` when the cell is plain
/// text and the press belongs to the row underneath.
fn table_cell_hit(cell: &Value, cell_rect: Rect, x: f32) -> Option<Option<ActionDescriptor>> {
    let payload = serde_json::from_value::<TableCellPayload>(cell.clone()).ok()?;
    match payload {
        TableCellPayload::Stepper { value, min, max, step, action } => {
            let seg = (cell_rect.w / 3.0).max(1.0);
            let slot = ((x - cell_rect.x) / seg).floor();
            Some(match slot as i64 {
                0 if value > min => Some(merge_action_args(&action, json!({ "delta": -step }))),
                2 if value < max => Some(merge_action_args(&action, json!({ "delta": step }))),
                _ => None,
            })
        }
        TableCellPayload::Buttons { buttons } => {
            let row_buttons = table_row_buttons(&buttons);
            if row_buttons.is_empty() {
                return None;
            }
            let seg = (cell_rect.w / row_buttons.len() as f32).max(1.0);
            let index = ((x - cell_rect.x) / seg).floor();
            Some(usize::try_from(index as i64).ok().and_then(|index| row_buttons.get(index)).map(|button| button.action.clone()))
        }
        TableCellPayload::Text { .. } | TableCellPayload::Number { .. } => None,
    }
}

/// 🎯️ Resolves a pointer point inside a `SurfaceKind::Table`: a sortable header band, a stepper
/// segment, a row action button, or the row itself.
fn table_hit(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32, theme: &Theme) -> Option<SceneListHit> {
    let table = scene.table.as_ref()?;
    let columns: Vec<TableColumn> = serde_json::from_str(&table.columns_json).unwrap_or_default();
    let metrics = table_metrics(inner, columns.len(), theme);
    let column_index = usize::try_from(((x - inner.x) / metrics.col_w.max(1.0)).floor() as i64).ok();
    let column = column_index.and_then(|index| columns.get(index).map(|column| (index, column)));
    if y < metrics.body.y {
        let (_, column) = column?;
        if !column.sortable {
            return None;
        }
        let sort: Option<TableSortJson> = table.sort_json.as_deref().and_then(|json| serde_json::from_str(json).ok());
        let direction = table_next_sort_direction(sort.as_ref(), &column.id);
        let action = scene_action(scene, "sortTable", json!({ "surfaceId": scene.surface_id, "columnId": column.id, "direction": direction }));
        return Some(SceneListHit::row(format!("{}.header.{}", scene.surface_id, column.id), Some(action)));
    }
    let rows: Vec<Value> = serde_json::from_str(&table.rows_json).unwrap_or_default();
    let scroll = scroll_offset(&scene.surface_id, "body");
    let index = usize::try_from(((y - metrics.body.y + scroll) / metrics.row_h.max(1.0)).floor() as i64).ok()?;
    let row = rows.get(index)?;
    let row_id = table_row_id(row, index);
    let control_id = format!("{}.row.{}", scene.surface_id, row_id);
    if let Some((column_index, column)) = column {
        if let Some(cell) = row.get(&column.id) {
            let cell_rect = Rect::new(inner.x + column_index as f32 * metrics.col_w + metrics.pad, 0.0, (metrics.col_w - metrics.pad * 2.0).max(1.0), metrics.row_h);
            if let Some(action) = table_cell_hit(cell, cell_rect, x) {
                return Some(SceneListHit::row(control_id, action));
            }
        }
    }
    Some(SceneListHit::row(control_id, Some(table_row_action(scene, table, row, &row_id))))
}

/// 📊️ Renders `SurfaceKind::Table`: a sortable header band over a scrolling, selectable row body
/// whose cells may themselves be steppers or action buttons — the port of `📊️Table/🟦️.tsx`.
fn render_table(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let theme = ctx.theme;
    let Some(table) = &scene.table else {
        return render_placeholder("table", bounds, ctx);
    };
    let columns: Vec<TableColumn> = serde_json::from_str(&table.columns_json).unwrap_or_default();
    let rows: Vec<Value> = serde_json::from_str(&table.rows_json).unwrap_or_default();
    let selected_ids: Vec<String> =
        table.selection_json.as_deref().and_then(|json| serde_json::from_str::<Value>(json).ok()).and_then(|value| value.get("selectedIds").cloned()).and_then(|value| serde_json::from_value(value).ok()).unwrap_or_default();
    let sort: Option<TableSortJson> = table.sort_json.as_deref().and_then(|json| serde_json::from_str(json).ok());
    let inner = bounds;
    let metrics = table_metrics(inner, columns.len(), theme);
    let (header_h, row_h, pad, col_w) = (metrics.header_h, metrics.row_h, metrics.pad, metrics.col_w);
    ctx.draw.push_solid([inner.x, inner.y, inner.w, header_h], theme.panel);
    for (index, column) in columns.iter().enumerate() {
        let x = inner.x + index as f32 * col_w;
        let sorted_here = sort.as_ref().filter(|s| s.column_id == column.id);
        let label = match sorted_here {
            Some(s) if s.direction == "desc" => format!("{} \u{25BC}", column.label),
            Some(_) => format!("{} \u{25B2}", column.label),
            None => column.label.clone(),
        };
        draw_text(ctx, &label, x + pad, inner.y + header_h * 0.65, theme.font_size_small, if sorted_here.is_some() { theme.text } else { theme.text_muted });
        if column.sortable {
            let direction = table_next_sort_direction(sort.as_ref(), &column.id);
            ctx.input.register_hit(HitTarget {
                rect: Rect::new(x, inner.y, col_w, header_h),
                event: Some(scene_action(scene, "sortTable", json!({ "surfaceId": scene.surface_id, "columnId": column.id, "direction": direction }))),
                control_id: Some(format!("{}.header.{}", scene.surface_id, column.id)),
                kind: HitKind::Generic,
                drag_axis: None,
                drag_data: None,
            });
        }
    }
    ctx.draw.push_line(inner.x, inner.y + header_h, inner.x + inner.w, inner.y + header_h, theme.separator, 1.0);
    let body = metrics.body;
    let scroll = scroll_offset(&scene.surface_id, "body");
    ctx.input.register_hit(HitTarget { rect: body, event: None, control_id: Some(scroll_key(&scene.surface_id, "body")), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None });
    ctx.draw.push_scissor(body);
    reserve_list_rows(ctx, list_visible_row_capacity(body, row_h));
    let hovered_row = scene_hovered_control_id(&scene.surface_id).or_else(|| ctx.input.hovered_id.clone());
    if rows.is_empty() {
        let message = "No rows";
        draw_text(ctx, message, body.x + body.w * 0.5 - 40.0, body.y + body.h * 0.5, theme.font_size_small, theme.text_muted);
    }
    for (row_index, row) in rows.iter().enumerate() {
        let y = body.y + row_index as f32 * row_h - scroll;
        if y + row_h < body.y || y > body.y + body.h {
            continue;
        }
        let row_id = table_row_id(row, row_index);
        let control_id = format!("{}.row.{}", scene.surface_id, row_id);
        let row_rect = Rect::new(body.x, y, body.w, row_h);
        let hovered = hovered_row.as_deref() == Some(control_id.as_str());
        let selected = selected_ids.iter().any(|id| id == &row_id);
        if selected {
            ctx.draw.push_solid([row_rect.x, row_rect.y, row_rect.w, row_rect.h], theme.selected);
        } else if hovered {
            ctx.draw.push_solid([row_rect.x, row_rect.y, row_rect.w, row_rect.h], theme.row_hover);
        }
        ctx.draw.push_line(row_rect.x, row_rect.y + row_rect.h - theme.stroke_hairline, row_rect.x + row_rect.w, row_rect.y + row_rect.h - theme.stroke_hairline, theme.separator, 1.0);
        for (col_index, column) in columns.iter().enumerate() {
            let x = body.x + col_index as f32 * col_w;
            let cell_rect = Rect::new(x + pad, y, col_w - pad * 2.0, row_h);
            let text = match row.get(&column.id) {
                Some(value) => render_table_cell(value, cell_rect, ctx),
                None => Some("—".into()),
            };
            if let Some(text) = text {
                draw_text(ctx, &text, x + pad, y + row_h * 0.65, theme.font_size_small, if selected || hovered { theme.active_foreground } else { theme.text });
            }
        }
        let drag_data = table.row_drag_mime.as_ref().and_then(|mime| row.get("_drag").map(|payload| HashMap::from([(mime.clone(), payload.to_string())])));
        ctx.input.register_hit(HitTarget { rect: row_rect, event: Some(table_row_action(scene, table, row, &row_id)), control_id: Some(control_id), kind: HitKind::Generic, drag_axis: None, drag_data });
    }
    ctx.draw.pop_scissor();
}

//#endregion Table

//#region TableTests
#[cfg(test)]
#[path = "../../🧪️tests/🔬️wgpu-table/🦀️.rs"]
mod table_tests;
//#endregion TableTests

//#region BlockList
/// 🧩️ Mirrors `playbook::PlaybookBlock`'s renderer-relevant fields — a typed block inside a
/// [`BlockListScene`] step. Unknown/extra JSON fields (the block-kind-specific property editor
/// fields owned by the host app) are ignored by `serde` since this crate never edits them.
#[derive(Deserialize)]
struct BlockListBlockJson {
    id: String,
    label: String,
    kind: String,
}

/// 🧩️ Mirrors `playbook::PlaybookStep`'s renderer-relevant fields.
#[derive(Deserialize)]
struct BlockListStepJson {
    id: String,
    title: String,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    blocks: Vec<BlockListBlockJson>,
}

/// 🧩️ Mirrors `ui_wgpu::wgpu::BlockPaletteEntry`'s wire format (`{blockKind, label, iconId}`).
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct BlockListPaletteEntryJson {
    block_kind: String,
    label: String,
    #[serde(default)]
    icon_id: String,
}

/** @emoji 🧩️ What one laid-out block-list target paints as. Keeping the geometry (`BlockListTarget`)
 * and the appearance apart is what lets the paint and the pointer path share ONE layout pass: the
 * paint walks the plan forwards, the hit test walks it backwards so a card's own buttons win over
 * the card band they sit in. */
enum BlockListPaint {
    AddStep,
    StepCard { title: String, description: Option<String>, selected: bool },
    BlockRow { label: String, kind: String, selected: bool },
    IconButton { icon: &'static str },
    PaletteEntry { icon_id: String, label: String },
}

/** @emoji 🧩️ One laid-out `SurfaceKind::BlockList` target: where it sits, the control id it is drawn
 * and hovered under, and the action a press on it dispatches (`None` for a disabled reorder button —
 * React renders those disabled, so the press must fall through to nothing, not to the card). */
struct BlockListTarget {
    rect: Rect,
    control_id: String,
    action: Option<ActionDescriptor>,
    paint: BlockListPaint,
}

/// 🧩️ A block list's whole laid-out surface: the two chrome bands, the scrolling step body, and the
/// flat target list split into "before the body" / "inside the body" / "palette rail".
struct BlockListPlan {
    header: Rect,
    body: Rect,
    palette_rect: Rect,
    targets: Vec<BlockListTarget>,
    body_range: std::ops::Range<usize>,
}

/// 📐️ Lays out one block list. The action payloads are the ones `🧩️BlockListHost/🟦️.tsx` dispatches
/// verbatim (`addStep`/`removeStep`/`moveStep`/`addBlock`/`removeBlock`/`moveBlock`); reordering is
/// driven by explicit move-up/move-down buttons rather than the React host's dnd-kit drag, because
/// this renderer has no cross-frame list-drag primitive.
fn block_list_plan(scene: &UiComponentSceneNode, bounds: Rect, theme: &Theme) -> BlockListPlan {
    let pad = theme.padding_standard;
    let row_h = theme.control_height;
    let btn_w = 26.0;
    let palette_w = (bounds.w * 0.22).clamp(140.0, 220.0);
    let main = Rect::new(bounds.x, bounds.y, (bounds.w - palette_w).max(0.0), bounds.h);
    let palette_rect = Rect::new(main.x + main.w, bounds.y, palette_w, bounds.h);
    let header = Rect::new(main.x, main.y, main.w, row_h);
    let body = Rect::new(main.x, main.y + row_h, main.w, (main.h - row_h).max(0.0));
    let steps: Vec<BlockListStepJson> = scene.block_list.as_ref().map(|list| serde_json::from_str(&list.steps_json).unwrap_or_default()).unwrap_or_default();
    let palette: Vec<BlockListPaletteEntryJson> = scene.block_list.as_ref().map(|list| serde_json::from_str(&list.palette_json).unwrap_or_default()).unwrap_or_default();
    let selected_id = scene.block_list.as_ref().and_then(|list| list.selected_id.clone());
    let selected_id = selected_id.as_deref();

    let mut targets = Vec::new();
    targets.push(BlockListTarget {
        rect: Rect::new(header.x + header.w - 108.0, header.y + 4.0, 104.0, row_h - 8.0),
        control_id: format!("{}.addStep", scene.surface_id),
        action: Some(scene_action(scene, "addStep", json!({}))),
        paint: BlockListPaint::AddStep,
    });
    let body_start = targets.len();

    let scroll = scroll_offset(&scene.surface_id, "blockList");
    let step_count = steps.len();
    let mut y = body.y - scroll;
    for (step_index, step) in steps.iter().enumerate() {
        let block_count = step.blocks.len();
        let description_h = step.description.as_ref().map_or(0.0, |_| theme.font_size_small + pad);
        let step_h = row_h + description_h + block_count as f32 * row_h + pad * 2.0;
        let step_rect = Rect::new(body.x + pad, y, (body.w - pad * 2.0).max(0.0), step_h);
        targets.push(BlockListTarget {
            rect: step_rect,
            control_id: format!("{}.step.{}", scene.surface_id, step.id),
            action: None,
            paint: BlockListPaint::StepCard { title: step.title.clone(), description: step.description.clone(), selected: selected_id == Some(step.id.as_str()) },
        });
        let btn_y = step_rect.y + (row_h - theme.control_height_small) * 0.5;
        let step_buttons = [
            ("moveUp", "chevron-up", (step_index > 0).then(|| scene_action(scene, "moveStep", json!({ "stepId": step.id, "index": step_index.saturating_sub(1) })))),
            ("moveDown", "chevron-down", (step_index + 1 < step_count).then(|| scene_action(scene, "moveStep", json!({ "stepId": step.id, "index": step_index + 1 })))),
            ("remove", "trash-2", Some(scene_action(scene, "removeStep", json!({ "stepId": step.id })))),
        ];
        for (slot, (suffix, icon, action)) in step_buttons.into_iter().enumerate() {
            let x = step_rect.x + step_rect.w - pad - btn_w * (3 - slot) as f32;
            targets.push(BlockListTarget {
                rect: Rect::new(x, btn_y, btn_w, theme.control_height_small),
                control_id: format!("{}.step.{}.{suffix}", scene.surface_id, step.id),
                action,
                paint: BlockListPaint::IconButton { icon },
            });
        }
        let mut inner_y = step_rect.y + row_h + description_h;
        for (block_index, block) in step.blocks.iter().enumerate() {
            let block_rect = Rect::new(step_rect.x + pad, inner_y, (step_rect.w - pad * 2.0).max(0.0), row_h);
            targets.push(BlockListTarget {
                rect: block_rect,
                control_id: format!("{}.block.{}", scene.surface_id, block.id),
                action: None,
                paint: BlockListPaint::BlockRow { label: block.label.clone(), kind: block.kind.clone(), selected: selected_id == Some(block.id.as_str()) },
            });
            let block_btn_y = block_rect.y + (row_h - theme.control_height_small) * 0.5;
            let block_buttons = [
                (
                    "moveUp",
                    "chevron-up",
                    (block_index > 0).then(|| scene_action(scene, "moveBlock", json!({ "blockId": block.id, "fromStepId": step.id, "toStepId": step.id, "index": block_index.saturating_sub(1) }))),
                ),
                (
                    "moveDown",
                    "chevron-down",
                    (block_index + 1 < block_count).then(|| scene_action(scene, "moveBlock", json!({ "blockId": block.id, "fromStepId": step.id, "toStepId": step.id, "index": block_index + 1 }))),
                ),
                ("remove", "trash-2", Some(scene_action(scene, "removeBlock", json!({ "stepId": step.id, "blockId": block.id })))),
            ];
            for (slot, (suffix, icon, action)) in block_buttons.into_iter().enumerate() {
                let x = block_rect.x + block_rect.w - pad - btn_w * (3 - slot) as f32;
                targets.push(BlockListTarget {
                    rect: Rect::new(x, block_btn_y, btn_w, theme.control_height_small),
                    control_id: format!("{}.block.{}.{suffix}", scene.surface_id, block.id),
                    action,
                    paint: BlockListPaint::IconButton { icon },
                });
            }
            inner_y += row_h;
        }
        y += step_h + theme.gap_standard;
    }
    let body_end = targets.len();

    let mut py = palette_rect.y + row_h;
    for entry in &palette {
        targets.push(BlockListTarget {
            rect: Rect::new(palette_rect.x + pad, py, (palette_rect.w - pad * 2.0).max(0.0), row_h),
            control_id: format!("{}.palette.{}", scene.surface_id, entry.block_kind),
            action: Some(scene_action(scene, "addBlock", json!({ "kind": entry.block_kind }))),
            paint: BlockListPaint::PaletteEntry { icon_id: entry.icon_id.clone(), label: entry.label.clone() },
        });
        py += row_h + 2.0;
    }
    BlockListPlan { header, body, palette_rect, targets, body_range: body_start..body_end }
}

/// 🖌️ Draws one planned block-list target.
fn paint_block_list_target(ctx: &mut FrameworkWidgetContext<'_>, target: &BlockListTarget) {
    let theme = ctx.theme;
    let pad = theme.padding_standard;
    let row_h = theme.control_height;
    let rect = target.rect;
    match &target.paint {
        BlockListPaint::AddStep => {
            render_widget(&WidgetNode::Button { id: Some(target.control_id.clone()), icon_id: Some("plus".into()), label: "Add Step".into(), event: target.action.clone() }, rect, ctx);
        }
        BlockListPaint::StepCard { title, description, selected } => {
            ctx.draw.push_rounded([rect.x, rect.y, rect.w, rect.h], if *selected { theme.selected } else { theme.button }, theme.border_radius);
            draw_ink_rect_outline(ctx.draw, rect.x, rect.y, rect.w, rect.h, theme.border_normal, theme.stroke_hairline);
            draw_text(ctx, title, rect.x + pad, rect.y + row_h * 0.65, theme.font_size_body, if *selected { theme.active_foreground } else { theme.text });
            if let Some(description) = description {
                draw_text(ctx, description, rect.x + pad, rect.y + row_h + theme.font_size_small, theme.font_size_small, theme.text_muted);
            }
        }
        BlockListPaint::BlockRow { label, kind, selected } => {
            if *selected {
                ctx.draw.push_rounded([rect.x, rect.y, rect.w, rect.h], theme.selected, theme.border_radius.min(4.0));
            }
            draw_text(ctx, label, rect.x + pad, rect.y + row_h * 0.65, theme.font_size_small, if *selected { theme.active_foreground } else { theme.text });
            draw_text(ctx, kind, rect.x + rect.w * 0.5, rect.y + row_h * 0.65, theme.font_size_small, theme.text_muted);
        }
        BlockListPaint::IconButton { icon } => {
            render_widget(&WidgetNode::Button { id: Some(target.control_id.clone()), icon_id: Some((*icon).into()), label: String::new(), event: target.action.clone() }, rect, ctx);
        }
        BlockListPaint::PaletteEntry { icon_id, label } => {
            render_widget(&WidgetNode::Button { id: Some(target.control_id.clone()), icon_id: IconName::from_str(icon_id), label: label.clone(), event: target.action.clone() }, rect, ctx);
        }
    }
}

/// 🧩️ Renders the strict-list Blockly-like block-list builder (`SurfaceKind::BlockList`): steps
/// stacked vertically (each with its ordered blocks) plus a palette rail for inserting new blocks,
/// mirroring `🧩️BlockListHost/🟦️.tsx`'s layout and action verbs.
fn render_block_list(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let theme = ctx.theme;
    if scene.block_list.is_none() {
        return render_placeholder("block-list", bounds, ctx);
    }
    let plan = block_list_plan(scene, bounds, theme);
    let pad = theme.padding_standard;
    let row_h = theme.control_height;

    draw_text(ctx, "Steps", plan.header.x + pad, plan.header.y + row_h * 0.65, theme.font_size_small, theme.text_muted);
    for target in &plan.targets[..plan.body_range.start] {
        paint_block_list_target(ctx, target);
    }

    ctx.input.register_hit(HitTarget { rect: plan.body, event: None, control_id: Some(scroll_key(&scene.surface_id, "blockList")), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None });
    ctx.draw.push_scissor(plan.body);
    reserve_list_rows(ctx, list_visible_row_capacity(plan.body, row_h));
    if plan.body_range.is_empty() {
        draw_text(ctx, "No steps", plan.body.x + pad, plan.body.y + row_h * 0.65, theme.font_size_small, theme.text_muted);
    }
    for target in &plan.targets[plan.body_range.clone()] {
        if target.rect.y + target.rect.h < plan.body.y || target.rect.y > plan.body.y + plan.body.h {
            continue;
        }
        paint_block_list_target(ctx, target);
    }
    ctx.draw.pop_scissor();

    ctx.draw.push_line(plan.palette_rect.x, plan.palette_rect.y, plan.palette_rect.x, plan.palette_rect.y + plan.palette_rect.h, theme.separator, theme.stroke_hairline);
    draw_text(ctx, "Palette", plan.palette_rect.x + pad, plan.palette_rect.y + row_h * 0.65, theme.font_size_small, theme.text_muted);
    for target in &plan.targets[plan.body_range.end..] {
        paint_block_list_target(ctx, target);
    }
}

/// 🎯️ Resolves a pointer point inside a `SurfaceKind::BlockList` against the SAME plan the paint drew
/// from — scanned back to front so a card's reorder/remove buttons answer before the card itself.
fn block_list_hit(scene: &UiComponentSceneNode, bounds: Rect, x: f32, y: f32, theme: &Theme) -> Option<SceneListHit> {
    scene.block_list.as_ref()?;
    let plan = block_list_plan(scene, bounds, theme);
    let inside_body = plan.body.contains(x, y);
    for (index, target) in plan.targets.iter().enumerate().rev() {
        if plan.body_range.contains(&index) && !inside_body {
            continue;
        }
        if target.rect.contains(x, y) {
            return Some(SceneListHit::row(target.control_id.clone(), target.action.clone()));
        }
    }
    None
}

//#endregion BlockList

//#region BlockListTests
#[cfg(test)]
#[path = "../../🧪️tests/🔬️wgpu-block-list/🦀️.rs"]
mod block_list_tests;
//#endregion BlockListTests

//#region DiffView
#[derive(Clone, Copy, PartialEq, Debug)]
enum DiffLineOperation {
    Equal,
    Removed,
    Added,
}

#[derive(Clone, Copy, Debug)]
struct DiffLine<'a> {
    operation: DiffLineOperation,
    text: &'a str,
}

/// 🧮️ Above this many `before.len() * after.len()` DP cells, [`diff_lines`] skips the LCS table and
/// falls back to a positional compare so a single huge [`SurfaceKind::DiffView`] payload can't blow
/// up per-frame recompute cost (this crate re-derives the diff every render pass, mirroring how
/// `render_graph_timeline` re-parses `columns_json` every frame rather than caching it).
const DIFF_LCS_CELL_BUDGET: usize = 200_000;

fn diff_lines<'a>(before: &[&'a str], after: &[&'a str]) -> Vec<DiffLine<'a>> {
    let (n, m) = (before.len(), after.len());
    if n.saturating_mul(m) > DIFF_LCS_CELL_BUDGET {
        let mut out = Vec::with_capacity(n + m);
        for i in 0..n.max(m) {
            match (before.get(i).copied(), after.get(i).copied()) {
                (Some(b), Some(a)) if b == a => out.push(DiffLine { operation: DiffLineOperation::Equal, text: b }),
                (Some(b), Some(a)) => {
                    out.push(DiffLine { operation: DiffLineOperation::Removed, text: b });
                    out.push(DiffLine { operation: DiffLineOperation::Added, text: a });
                }
                (Some(b), None) => out.push(DiffLine { operation: DiffLineOperation::Removed, text: b }),
                (None, Some(a)) => out.push(DiffLine { operation: DiffLineOperation::Added, text: a }),
                (None, None) => {}
            }
        }
        return out;
    }
    let mut table = vec![vec![0u32; m + 1]; n + 1];
    for i in (0..n).rev() {
        for j in (0..m).rev() {
            table[i][j] = if before[i] == after[j] { table[i + 1][j + 1] + 1 } else { table[i + 1][j].max(table[i][j + 1]) };
        }
    }
    let mut out = Vec::with_capacity(n + m);
    let (mut i, mut j) = (0, 0);
    while i < n && j < m {
        if before[i] == after[j] {
            out.push(DiffLine { operation: DiffLineOperation::Equal, text: before[i] });
            i += 1;
            j += 1;
        } else if table[i + 1][j] >= table[i][j + 1] {
            out.push(DiffLine { operation: DiffLineOperation::Removed, text: before[i] });
            i += 1;
        } else {
            out.push(DiffLine { operation: DiffLineOperation::Added, text: after[j] });
            j += 1;
        }
    }
    while i < n {
        out.push(DiffLine { operation: DiffLineOperation::Removed, text: before[i] });
        i += 1;
    }
    while j < m {
        out.push(DiffLine { operation: DiffLineOperation::Added, text: after[j] });
        j += 1;
    }
    out
}
/// 🪞️ Pairs consecutive remove/add runs into aligned rows for the split-pane layout; equal lines
/// mirror onto both sides. Port of `buildSplitRows` in `🔺️DiffViewHost/🟦️.tsx` — without it a
/// changed line occupied two rows (a left-only one, then a right-only one) instead of one aligned pair.
fn split_diff_rows<'a>(operations: &[DiffLine<'a>]) -> Vec<(Option<DiffLine<'a>>, Option<DiffLine<'a>>)> {
    let mut rows = Vec::with_capacity(operations.len());
    let mut index = 0;
    while index < operations.len() {
        let line = operations[index];
        if line.operation == DiffLineOperation::Equal {
            rows.push((Some(line), Some(line)));
            index += 1;
            continue;
        }
        let mut removed = Vec::new();
        while index < operations.len() && operations[index].operation == DiffLineOperation::Removed {
            removed.push(operations[index]);
            index += 1;
        }
        let mut added = Vec::new();
        while index < operations.len() && operations[index].operation == DiffLineOperation::Added {
            added.push(operations[index]);
            index += 1;
        }
        for pair in 0..removed.len().max(added.len()) {
            rows.push((removed.get(pair).copied(), added.get(pair).copied()));
        }
    }
    rows
}

/// 🩹️ Renders `SurfaceKind::DiffView`: a line-level diff of `before`/`after`, either as a single
/// scrolling column with `+`/`-` markers (default, or `mode: "unified"`) or as two aligned columns
/// (`mode: "split"`). Add/remove TEXT is tinted with the theme's `accent`/`error` tokens and equal
/// text stays full-brightness `theme.text`, matching `DIFF_LINE_CLASS`'s per-line text-color classes
/// in `🔺️DiffViewHost/🟦️.tsx` — never a whole-row background wash.
fn render_diff_view(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let theme = ctx.theme;
    let Some(diff) = &scene.diff_view else {
        return render_placeholder("diff-view", bounds, ctx);
    };
    let before_lines: Vec<&str> = diff.before.split('\n').collect();
    let after_lines: Vec<&str> = diff.after.split('\n').collect();
    let operations = diff_lines(&before_lines, &after_lines);
    let inner = bounds;
    let pad = theme.padding_standard;
    let row_h = theme.font_size_small + pad * 0.5;
    let split = diff.mode.as_deref() == Some("split");

    let scroll = scroll_offset(&scene.surface_id, "diff");
    ctx.input.register_hit(HitTarget { rect: inner, event: None, control_id: Some(scroll_key(&scene.surface_id, "diff")), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None });
    ctx.draw.push_scissor(inner);
    reserve_list_rows(ctx, list_visible_row_capacity(inner, row_h));
    if operations.is_empty() {
        draw_text(ctx, "—", inner.x + pad, inner.y + row_h * 0.65, theme.font_size_small, theme.text_muted);
        ctx.draw.pop_scissor();
        return;
    }

    let col_w = if split { (inner.w * 0.5).max(1.0) } else { inner.w };
    let right_x = inner.x + col_w;
    if split {
        for (row_index, (left, right)) in split_diff_rows(&operations).iter().enumerate() {
            let y = inner.y + row_index as f32 * row_h - scroll;
            if y + row_h < inner.y || y > inner.y + inner.h {
                continue;
            }
            if let Some(line) = left {
                let color = if line.operation == DiffLineOperation::Removed { theme.error } else { theme.text };
                draw_text(ctx, line.text, inner.x + pad, y + row_h * 0.7, theme.font_size_small, color);
            }
            if let Some(line) = right {
                let color = if line.operation == DiffLineOperation::Added { theme.accent } else { theme.text };
                draw_text(ctx, line.text, right_x + pad, y + row_h * 0.7, theme.font_size_small, color);
            }
            ctx.draw.push_line(right_x, y, right_x, y + row_h, theme.separator, theme.stroke_hairline);
        }
        ctx.draw.pop_scissor();
        return;
    }
    for (row_index, line) in operations.iter().enumerate() {
        let y = inner.y + row_index as f32 * row_h - scroll;
        if y + row_h < inner.y || y > inner.y + inner.h {
            continue;
        }
        let (marker, color) = match line.operation {
            DiffLineOperation::Added => ('+', theme.accent),
            DiffLineOperation::Removed => ('-', theme.error),
            DiffLineOperation::Equal => (' ', theme.text),
        };
        draw_text(ctx, &format!("{marker} {}", line.text), inner.x + pad, y + row_h * 0.7, theme.font_size_small, color);
    }
    ctx.draw.pop_scissor();
}

//#endregion DiffView

//#region DiffViewTests
#[cfg(test)]
#[path = "../../🧪️tests/🔬️wgpu-diff-view/🦀️.rs"]
mod diff_view_tests;
//#endregion DiffViewTests

//#region EventFeed
/// 🪶️ Mirrors a `SurfaceKind::EventFeed` entry (`{id, timestampMs, iconId, title, detail?, tone?}`,
/// `ui_wgpu::wgpu::EventFeedScene`'s doc comment / `EventFeedEntry` in `framework/core/js/index.ts`).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EventFeedEntryJson {
    id: String,
    #[serde(default)]
    timestamp_ms: i64,
    #[serde(default)]
    icon_id: String,
    #[serde(default)]
    title: String,
    #[serde(default)]
    detail: Option<String>,
    #[serde(default)]
    tone: Option<String>,
}

fn event_feed_time_of_day_utc(timestamp_ms: i64) -> String {
    let ms_in_day = timestamp_ms.rem_euclid(86_400_000);
    let total_seconds = ms_in_day / 1000;
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    format!("{hours:02}:{minutes:02}:{seconds:02}")
}

fn event_feed_tone_color(tone: Option<&str>, theme: &Theme) -> Rgba {
    match tone {
        Some("error") | Some("danger") => theme.error,
        Some("success") | Some("positive") => theme.accent,
        Some("pending") | Some("warning") => theme.temporary,
        _ => theme.text_muted,
    }
}

fn event_feed_row_height(entry: &EventFeedEntryJson, row_h: f32, theme: &Theme) -> f32 {
    row_h + entry.detail.as_ref().map_or(0.0, |_| theme.font_size_small + theme.padding_standard * 0.25)
}
/// 🎯️ Resolves a pointer point inside a `SurfaceKind::EventFeed`: the entry whose variable-height
/// band contains `y`. Row activation sends `{ surfaceId, id }` — the payload
/// `📡️EventFeedHost/🟦️.tsx`'s own `onClick` sends; this renderer used to send `{ entryId }`, which
/// no host reads.
fn event_feed_hit(scene: &UiComponentSceneNode, inner: Rect, y: f32, theme: &Theme) -> Option<SceneListHit> {
    let feed = scene.event_feed.as_ref()?;
    let entries: Vec<EventFeedEntryJson> = serde_json::from_str(&feed.entries_json).unwrap_or_default();
    let row_h = theme.control_height;
    let scroll = scroll_offset(&scene.surface_id, "feed");
    let mut top = inner.y - scroll;
    for entry in &entries {
        let entry_h = event_feed_row_height(entry, row_h, theme);
        if y >= top && y < top + entry_h {
            let action = feed.activate_action.as_deref().map(|action| scene_action(scene, action, json!({ "surfaceId": scene.surface_id, "id": entry.id })));
            return Some(SceneListHit::row(format!("{}.feed.{}", scene.surface_id, entry.id), action));
        }
        top += entry_h;
    }
    None
}

/// 📜️ Renders `SurfaceKind::EventFeed`: a scrollable list of text rows (`entries_json`), each a tone
/// dot + optional icon + time-of-day + title, with an optional detail line beneath. When `follow` is
/// set the feed snaps to its bottom every frame (log-tail behaviour); a manual wheel-scroll on a
/// following feed is overridden on the next render, the same tradeoff a live log tail makes.
fn render_event_feed(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let theme = ctx.theme;
    let Some(feed) = &scene.event_feed else {
        return render_placeholder("event-feed", bounds, ctx);
    };
    let entries: Vec<EventFeedEntryJson> = serde_json::from_str(&feed.entries_json).unwrap_or_default();
    let inner = bounds;
    let pad = theme.padding_standard;
    let row_h = theme.control_height;
    if entries.is_empty() {
        draw_text(ctx, "—", inner.x + pad, inner.y + row_h * 0.65, theme.font_size_small, theme.text_muted);
        return;
    }

    let heights: Vec<f32> = entries.iter().map(|entry| event_feed_row_height(entry, row_h, theme)).collect();
    let content_h: f32 = heights.iter().sum();
    if feed.follow.unwrap_or(false) {
        set_scroll_offset(&scene.surface_id, "feed", (content_h - inner.h).max(0.0));
    }
    let scroll = scroll_offset(&scene.surface_id, "feed");
    ctx.input.register_hit(HitTarget { rect: inner, event: None, control_id: Some(scroll_key(&scene.surface_id, "feed")), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None });
    ctx.draw.push_scissor(inner);
    reserve_list_rows(ctx, list_visible_row_capacity(inner, row_h));
    let hovered_row = scene_hovered_control_id(&scene.surface_id).or_else(|| ctx.input.hovered_id.clone());
    let mut y = inner.y - scroll;
    for entry in entries.iter() {
        let entry_h = event_feed_row_height(entry, row_h, theme);
        if y + entry_h < inner.y || y > inner.y + inner.h {
            y += entry_h;
            continue;
        }
        let control_id = format!("{}.feed.{}", scene.surface_id, entry.id);
        let hovered = hovered_row.as_deref() == Some(control_id.as_str());
        let row_rect = Rect::new(inner.x, y, inner.w, entry_h);
        if hovered {
            ctx.draw.push_solid([row_rect.x, row_rect.y, row_rect.w, row_rect.h], theme.row_hover);
        }
        ctx.draw.push_line(row_rect.x, row_rect.y + row_rect.h - theme.stroke_hairline, row_rect.x + row_rect.w, row_rect.y + row_rect.h - theme.stroke_hairline, theme.separator, 1.0);

        let tone_color = event_feed_tone_color(entry.tone.as_deref(), theme);
        let title_tone_color = match entry.tone.as_deref() {
            None | Some("info") => theme.text,
            _ => tone_color,
        };
        let dot_y = y + row_h * 0.5;
        ctx.draw.push_rounded([inner.x + pad, dot_y - 3.0, 6.0, 6.0], tone_color, 3.0);
        let mut title_x = inner.x + pad + 6.0 + pad * 0.5;

        if !entry.icon_id.is_empty() {
            if let Some(icons) = ctx.icons {
                if let Some(uv) = icons.icon_uv(&entry.icon_id) {
                    ctx.draw.push_textured([title_x, y + (row_h - 14.0) * 0.5, 14.0, 14.0], uv, theme.text_element);
                    title_x += 14.0 + pad * 0.5;
                }
            }
        }

        if entry.timestamp_ms != 0 {
            let time_label = event_feed_time_of_day_utc(entry.timestamp_ms);
            draw_text(ctx, &time_label, title_x, y + row_h * 0.65, theme.font_size_small, theme.text_muted);
            title_x += 56.0;
        }
        draw_text(ctx, &entry.title, title_x, y + row_h * 0.65, theme.font_size_small, title_tone_color);
        if let Some(detail) = &entry.detail {
            draw_text(ctx, detail, inner.x + pad, y + row_h + theme.font_size_small * 0.9, theme.font_size_small, theme.text_muted);
        }
        if let Some(action) = &feed.activate_action {
            ctx.input.register_hit(HitTarget {
                rect: row_rect,
                event: Some(scene_action(scene, action, json!({ "surfaceId": scene.surface_id, "id": entry.id }))),
                control_id: Some(control_id),
                kind: HitKind::Generic,
                drag_axis: None,
                drag_data: None,
            });
        }
        y += entry_h;
    }
    ctx.draw.pop_scissor();
}

//#endregion EventFeed

//#region EventFeedTests
#[cfg(test)]
#[path = "../../🧪️tests/🔬️wgpu-event-feed/🦀️.rs"]
mod event_feed_tests;
//#endregion EventFeedTests

//#region GraphTimeline
/** @emoji 🗄️ Mirrors `store::HistoryColumn` / React `HistoryColumn` (`ui/js/react/index.tsx:19116`). */
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HistoryColumnAuthorJson {
    #[serde(default)]
    name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HistoryColumnJson {
    checkpoint_id: String,
    #[serde(default)]
    labels: Vec<String>,
    #[serde(default)]
    authors: Vec<HistoryColumnAuthorJson>,
    #[serde(default)]
    parent_checkpoint_id: Option<String>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    lane: usize,
}

const HISTORY_LANE_PITCH: f32 = 16.0;
const HISTORY_LANE_PAD: f32 = 8.0;
const HISTORY_AUTHOR_SLOT: f32 = 40.0;

fn history_lane_count(columns: &[HistoryColumnJson]) -> usize {
    columns.iter().map(|column| column.lane + 1).max().unwrap_or(1).max(1)
}

fn history_graph_width(lane_count: usize) -> f32 {
    (HISTORY_LANE_PAD * 2.0 + lane_count as f32 * HISTORY_LANE_PITCH).max(56.0)
}

fn history_lane_x(lane: usize, lane_count: usize, graph_width: f32) -> f32 {
    if lane_count <= 1 {
        return graph_width * 0.5;
    }
    HISTORY_LANE_PAD + lane as f32 * HISTORY_LANE_PITCH + HISTORY_LANE_PITCH * 0.5
}

fn history_row_lane_guides(columns: &[HistoryColumnJson], lane_count: usize) -> Vec<Vec<bool>> {
    let mut guides = vec![vec![false; lane_count]; columns.len()];
    let row_by_id: HashMap<&str, usize> = columns.iter().enumerate().map(|(index, column)| (column.checkpoint_id.as_str(), index)).collect();
    for (row_index, column) in columns.iter().enumerate() {
        if column.lane < lane_count {
            guides[row_index][column.lane] = true;
        }
        let Some(parent_row) = column.parent_checkpoint_id.as_deref().and_then(|id| row_by_id.get(id).copied()) else {
            continue;
        };
        let parent_lane = columns[parent_row].lane;
        if column.lane == parent_lane {
            for row in (row_index + 1)..parent_row {
                guides[row][column.lane] = true;
            }
            continue;
        }
        let elbow_row = if row_index + 1 < parent_row { row_index + 1 } else { parent_row };
        for row in (row_index + 1)..=elbow_row {
            if column.lane < lane_count {
                guides[row][column.lane] = true;
            }
        }
        for row in elbow_row..parent_row {
            if parent_lane < lane_count {
                guides[row][parent_lane] = true;
            }
        }
    }
    guides
}

fn graph_timeline_avatar_initials(name: &str) -> String {
    let letters: String = name.split_whitespace().filter_map(|word| word.chars().next()).take(2).flat_map(char::to_uppercase).collect();
    if letters.is_empty() {
        "?".to_string()
    } else {
        letters
    }
}

/// 🕰️ Renders `SurfaceKind::GraphTimeline`: the checkpoint history graph — lane guides, parent
/// elbows, a commit dot, author initials and the checkpoint's description per row. Row selection
/// dispatches `checkoutCheckpoint` with `{ checkpointId }`, the payload `🌳️GraphTimelineHost/🟦️.tsx`
/// hands `HistoryTable`'s `onSelectCheckpoint`.
///
/// 🪢️ `color-mix(in oklab, var(--muted-foreground) 40%, transparent)` on the lane guides and
/// parent connectors in `graph-timeline-host.tsx` — a translucent line; opaque `theme.separator`
/// previously read as visibly heavier than React's thin, faded rail.
fn render_graph_timeline(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let theme = ctx.theme;
    let Some(history) = &scene.graph_timeline else {
        return render_placeholder("graph-timeline", bounds, ctx);
    };
    let columns: Vec<HistoryColumnJson> = serde_json::from_str(&history.columns_json).unwrap_or_default();
    let inner = bounds;
    let row_h = theme.control_height * 1.33;
    let pad = theme.padding_standard;
    if columns.is_empty() {
        draw_text(ctx, "—", inner.x + pad, inner.y + row_h * 0.65, theme.font_size_small, theme.text_muted);
        return;
    }
    let lane_count = history_lane_count(&columns);
    let graph_width = history_graph_width(lane_count);
    let graph_col_w = graph_width + HISTORY_AUTHOR_SLOT;
    let labels_col_w = (inner.w * 0.28).max(96.0);
    let guides = history_row_lane_guides(&columns, lane_count);
    let row_by_id: HashMap<&str, usize> = columns.iter().enumerate().map(|(index, column)| (column.checkpoint_id.as_str(), index)).collect();

    let scroll = scroll_offset(&scene.surface_id, "history");
    ctx.input.register_hit(HitTarget { rect: inner, event: None, control_id: Some(scroll_key(&scene.surface_id, "history")), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None });
    ctx.draw.push_scissor(inner);
    reserve_list_rows(ctx, list_visible_row_capacity(inner, row_h));
    let hovered_row = scene_hovered_control_id(&scene.surface_id).or_else(|| ctx.input.hovered_id.clone());
    let graph_x0 = inner.x + labels_col_w;
    let desc_x = inner.x + labels_col_w + graph_col_w;

    for (row_index, column) in columns.iter().enumerate() {
        let y = inner.y + row_index as f32 * row_h - scroll;
        if y + row_h < inner.y || y > inner.y + inner.h {
            continue;
        }
        let control_id = format!("{}.history.{}", scene.surface_id, column.checkpoint_id);
        let hovered = hovered_row.as_deref() == Some(control_id.as_str());
        let row_rect = Rect::new(inner.x, y, inner.w, row_h);
        if hovered {
            ctx.draw.push_solid([row_rect.x, row_rect.y, row_rect.w, row_rect.h], theme.row_hover);
        }
        ctx.draw.push_line(row_rect.x, row_rect.y + row_rect.h - theme.stroke_hairline, row_rect.x + row_rect.w, row_rect.y + row_rect.h - theme.stroke_hairline, theme.separator, 1.0);

        let mut label_x = inner.x + pad;
        if column.labels.is_empty() {
            draw_text(ctx, "checkpoint", label_x, y + row_h * 0.65, theme.font_size_small, theme.text_muted);
        } else {
            for label in &column.labels {
                let chip_w = (label.len() as f32 * 6.0 + pad * 2.0).min((inner.x + labels_col_w - label_x).max(0.0));
                if chip_w <= 0.0 {
                    break;
                }
                ctx.draw.push_rounded([label_x, y + row_h * 0.5 - 9.0, chip_w, 18.0], theme.accent, 4.0);
                draw_text(ctx, label, label_x + 4.0, y + row_h * 0.5 + 4.0, theme.font_size_small, theme.active_foreground);
                label_x += chip_w + 4.0;
            }
        }

        let guide_stroke = theme.separator.with_alpha(theme.separator.a * 0.4);
        for lane in 0..lane_count {
            if guides[row_index][lane] {
                let lx = graph_x0 + history_lane_x(lane, lane_count, graph_width);
                ctx.draw.push_line(lx, y, lx, y + row_h, guide_stroke, 1.0);
            }
        }
        if let Some(parent_id) = column.parent_checkpoint_id.as_deref() {
            if let Some(&parent_row) = row_by_id.get(parent_id) {
                let x0 = graph_x0 + history_lane_x(column.lane, lane_count, graph_width);
                let parent_lane = columns[parent_row].lane;
                let x1 = graph_x0 + history_lane_x(parent_lane, lane_count, graph_width);
                let y0 = y + row_h * 0.5;
                let y1 = inner.y + parent_row as f32 * row_h - scroll + row_h * 0.5;
                if (x0 - x1).abs() < 0.5 {
                    ctx.draw.push_line(x0, y0, x1, y1, guide_stroke, 1.5);
                } else {
                    let elbow_y = y + row_h;
                    ctx.draw.push_line(x0, y0, x0, elbow_y, guide_stroke, 1.5);
                    ctx.draw.push_line(x0, elbow_y, x1, elbow_y, guide_stroke, 1.5);
                    ctx.draw.push_line(x1, elbow_y, x1, y1, guide_stroke, 1.5);
                }
            }
        }
        let dot_x = graph_x0 + history_lane_x(column.lane, lane_count, graph_width);
        let dot_y = y + row_h * 0.5;
        ctx.draw.push_rounded([dot_x - 3.0, dot_y - 3.0, 6.0, 6.0], theme.text, 3.0);

        let avatar_size = 20.0;
        let avatar_x = graph_x0 + graph_width + 4.0;
        let avatar_y = y + row_h * 0.5 - avatar_size * 0.5;
        let initial = column.authors.first().map(|author| graph_timeline_avatar_initials(&author.name)).unwrap_or_else(|| "?".into());
        ctx.draw.push_rounded([avatar_x, avatar_y, avatar_size, avatar_size], theme.button, avatar_size * 0.5);
        let initial_x_frac = if initial.chars().count() >= 2 { 0.18 } else { 0.32 };
        draw_text(ctx, &initial, avatar_x + avatar_size * initial_x_frac, avatar_y + avatar_size * 0.7, theme.font_size_small, theme.text);

        if let Some(description) = &column.description {
            draw_text(ctx, description, desc_x + pad, y + row_h * 0.65, theme.font_size_small, theme.text_muted);
        }

        ctx.input.register_hit(HitTarget {
            rect: row_rect,
            event: Some(scene_action(scene, "checkoutCheckpoint", json!({ "checkpointId": column.checkpoint_id }))),
            control_id: Some(control_id),
            kind: HitKind::Generic,
            drag_axis: None,
            drag_data: None,
        });
    }
    ctx.draw.pop_scissor();
}
/// 🎯️ Resolves a pointer point inside a `SurfaceKind::GraphTimeline` — the checkpoint row band
/// containing `y`. Rows are a uniform `control_height * 1.33`, the same pitch the paint lays out.
fn graph_timeline_hit(scene: &UiComponentSceneNode, inner: Rect, y: f32, theme: &Theme) -> Option<SceneListHit> {
    let history = scene.graph_timeline.as_ref()?;
    let columns: Vec<HistoryColumnJson> = serde_json::from_str(&history.columns_json).unwrap_or_default();
    let row_h = theme.control_height * 1.33;
    let scroll = scroll_offset(&scene.surface_id, "history");
    let index = usize::try_from(((y - inner.y + scroll) / row_h.max(1.0)).floor() as i64).ok()?;
    let column = columns.get(index)?;
    let action = scene_action(scene, "checkoutCheckpoint", json!({ "checkpointId": column.checkpoint_id }));
    Some(SceneListHit::row(format!("{}.history.{}", scene.surface_id, column.checkpoint_id), Some(action)))
}

//#endregion GraphTimeline

//#region GraphTimelineTests
#[cfg(test)]
#[path = "../../🧪️tests/🔬️wgpu-graph-timeline/🦀️.rs"]
mod graph_timeline_tests;
//#endregion GraphTimelineTests

//#region Canvas2d
/** 🪣️ A `CanvasLayerRecord["fill"]` mirror — solid color or linear/radial gradient stops, matches
 * `fillStyleToPaint` in `canvas-2d-host.tsx`. Coordinates (`x1/y1/x2/y2`, `cx/cy/r`) are in the same
 * local space as the owning layer's `x`/`y` (this renderer has no per-layer transform matrix yet, so
 * they are treated as offsets from the layer's own `x`/`y` origin). */
#[derive(Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
struct CanvasFillJson {
    #[serde(default)]
    kind: Option<String>,
    #[serde(default)]
    color: Option<Vec<f64>>,
    #[serde(default)]
    x1: f64,
    #[serde(default)]
    y1: f64,
    #[serde(default)]
    x2: f64,
    #[serde(default)]
    y2: f64,
    #[serde(default)]
    cx: f64,
    #[serde(default)]
    cy: f64,
    #[serde(default)]
    r: f64,
    #[serde(default)]
    stops: Vec<CanvasGradientStopJson>,
}

/** 🎨️ One `CanvasGradientStop` — `offset` in `[0,1]`, `color` an `[r,g,b,a?]` channel array in `0..1`. */
#[derive(Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
struct CanvasGradientStopJson {
    #[serde(default)]
    offset: f64,
    #[serde(default)]
    color: Option<Vec<f64>>,
}

/** 🖊️ A `CanvasLayerRecord["stroke"]` mirror. */
#[derive(Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
struct CanvasStrokeJson {
    #[serde(default)]
    color: Option<Vec<f64>>,
    #[serde(default)]
    width: Option<f64>,
    #[serde(default)]
    dash: Option<Vec<f64>>,
}

/** 🖼️ A `CanvasLayerRecord["image"]` mirror — the nested per-node image field (as opposed to the
 * legacy top-level `dataUrl` used by `kind === "image"` records). */
#[derive(Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
struct CanvasImageFieldJson {
    #[serde(default)]
    src: Option<String>,
    #[serde(default)]
    width: Option<f64>,
    #[serde(default)]
    height: Option<f64>,
}

/** 📝️ A `CanvasLayerRecord["text"]` mirror. */
#[derive(Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
struct CanvasTextFieldJson {
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    size: Option<f64>,
}

#[derive(Deserialize)]
struct CanvasLayer {
    #[serde(default)]
    kind: String,
    #[serde(default)]
    id: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    x: f64,
    #[serde(default)]
    y: f64,
    #[serde(default)]
    width: f64,
    #[serde(default)]
    height: f64,
    #[serde(default)]
    x0: Option<f64>,
    #[serde(default)]
    y0: Option<f64>,
    #[serde(default)]
    x1: Option<f64>,
    #[serde(default)]
    y1: Option<f64>,
    #[serde(default, rename = "dataUrl")]
    data_url: Option<String>,
    #[serde(default)]
    points: Option<Vec<[f64; 2]>>,
    #[serde(default)]
    seams: Option<Vec<u8>>,
    /** 🗒️ `role === "meta"` marks a non-visual bookkeeping record (e.g. carries the host's active
     * pointer-tool id) — filtered from rendering, matches `JsonLayersCanvasSession.renderFrame`'s
     * `records.find(role === "meta")` / `layers.filter(role !== "meta")` split. */
    #[serde(default)]
    role: Option<String>,
    #[serde(default)]
    visible: Option<bool>,
    #[serde(default)]
    opacity: Option<f32>,
    #[serde(default, rename = "blendMode")]
    blend_mode: Option<String>,
    #[serde(default)]
    selected: Option<bool>,
    #[serde(default)]
    fill: Option<CanvasFillJson>,
    #[serde(default)]
    stroke: Option<CanvasStrokeJson>,
    #[serde(default)]
    image: Option<CanvasImageFieldJson>,
    #[serde(default)]
    text: Option<CanvasTextFieldJson>,
}

#[derive(Deserialize)]
struct Canvas2dPacketText<'a> {
    #[serde(borrow)]
    content: &'a str,
    #[serde(default = "canvas2d_packet_text_size")]
    size: f64,
}

#[derive(Deserialize)]
struct Canvas2dPacketItem<'a> {
    #[serde(borrow)]
    kind: &'a str,
    #[serde(borrow)]
    id: &'a str,
    #[serde(default)]
    x: f64,
    #[serde(default)]
    y: f64,
    #[serde(default)]
    width: f64,
    #[serde(default)]
    height: f64,
    #[serde(default)]
    x0: f64,
    #[serde(default)]
    y0: f64,
    #[serde(default)]
    x1: f64,
    #[serde(default)]
    y1: f64,
    #[serde(default, borrow)]
    text: Option<Canvas2dPacketText<'a>>,
}

fn canvas2d_packet_text_size() -> f64 {
    11.0
}

fn canvas_layer_should_render(layer: &CanvasLayer) -> bool {
    layer.role.as_deref() != Some("meta") && layer.visible.unwrap_or(true)
}

fn decode_canvas_image_source(data_url: &str) -> Option<Vec<u8>> {
    let payload = data_url.strip_prefix("data:image/png;base64,").or_else(|| data_url.strip_prefix("data:image/jpeg;base64,")).unwrap_or(data_url);
    base64::engine::general_purpose::STANDARD.decode(payload).ok()
}

fn decode_canvas_image_bytes(bytes: &[u8]) -> Option<(Vec<u8>, u32, u32)> {
    let image = image::load_from_memory(&bytes).ok()?;
    let rgba = image.to_rgba8();
    let (width, height) = rgba.dimensions();
    Some((rgba.into_raw(), width, height))
}

pub(crate) fn queue_canvas_image_upload_sized(surface_id: &str, layer_id: &str, data_url: &str) -> (Option<String>, Option<(u32, u32)>) {
    let dimensions = std::cell::Cell::new(None);
    let key = queue_canvas_image_upload_with(
        surface_id,
        layer_id,
        data_url.as_bytes(),
        || {
            let bytes = decode_canvas_image_source(data_url).ok_or_else(Vec::new)?;
            let measured = match image::ImageReader::new(std::io::Cursor::new(bytes.as_slice())).with_guessed_format().ok().and_then(|reader| reader.into_dimensions().ok()) {
                Some(measured) => measured,
                None => return Err(bytes),
            };
            dimensions.set(Some(measured));
            Ok((measured.0, measured.1, bytes))
        },
        |bytes| decode_canvas_image_bytes(bytes).map(|(pixels, _, _)| pixels),
    );
    (key, dimensions.get())
}

pub(crate) fn queue_canvas_image_upload(surface_id: &str, layer_id: &str, data_url: &str) -> Option<String> {
    queue_canvas_image_upload_sized(surface_id, layer_id, data_url).0
}

fn draw_checkerboard(draw: &mut ui_wgpu::wgpu::DrawList, viewport: &Viewport, inner: Rect, theme: &Theme, extent: f32) {
    let cell = 16.0;
    let half = extent * 0.5;
    let light = theme.checker_light;
    let dark = theme.checker_dark;
    let (mut min_x, mut max_x, mut min_y, mut max_y) = (-half, half, -half, half);
    if viewport.zoom > 0.0 {
        let (wx0, wy0) = viewport.screen_to_world(inner.x, inner.y, inner);
        let (wx1, wy1) = viewport.screen_to_world(inner.x + inner.w, inner.y + inner.h, inner);
        min_x = min_x.max(wx0.min(wx1) - cell);
        max_x = max_x.min(wx0.max(wx1) + cell);
        min_y = min_y.max(wy0.min(wy1) - cell);
        max_y = max_y.min(wy0.max(wy1) + cell);
    }
    let start_row = ((min_y - (-half)) / cell).floor().max(0.0) as i64;
    let start_col = ((min_x - (-half)) / cell).floor().max(0.0) as i64;
    let mut row = start_row;
    let mut wy = -half + start_row as f32 * cell;
    while wy < max_y {
        let mut col = start_col;
        let mut wx = -half + start_col as f32 * cell;
        while wx < max_x {
            let color = if (row + col) % 2 == 0 { light } else { dark };
            let (sx, sy) = viewport.world_to_screen(wx, wy, inner);
            let (sx1, sy1) = viewport.world_to_screen(wx + cell, wy + cell, inner);
            let w = (sx1 - sx).abs().max(1.0);
            let h = (sy1 - sy).abs().max(1.0);
            draw.push_solid([sx.min(sx1), sy.min(sy1), w, h], color);
            wx += cell;
            col += 1;
        }
        wy += cell;
        row += 1;
    }
}

fn draw_canvas_infinite_grid(draw: &mut ui_wgpu::wgpu::DrawList, viewport: &Viewport, inner: Rect, theme: &Theme) {
    if viewport.zoom <= 0.0 {
        return;
    }
    let (wx0, wy0) = viewport.screen_to_world(inner.x, inner.y, inner);
    let (wx1, wy1) = viewport.screen_to_world(inner.x + inner.w, inner.y + inner.h, inner);
    let min_x = wx0.min(wx1);
    let max_x = wx0.max(wx1);
    let min_y = wy0.min(wy1);
    let max_y = wy0.max(wy1);
    let color = theme.separator.with_alpha((theme.separator.a * 0.35).max(0.08));
    let steps: [(f32, f32, f32); 4] = [(10.0, 1.0, 0.0), (2.5, 0.72, 8.0), (0.5, 0.48, 10.0), (0.1, 0.32, 12.0)];
    for (world_step, stroke_px, min_screen) in steps {
        let screen = world_step * viewport.zoom;
        if screen < min_screen {
            continue;
        }
        let width = (stroke_px).max(0.5);
        let start_x = (min_x / world_step).floor() * world_step;
        let start_y = (min_y / world_step).floor() * world_step;
        let mut x = start_x;
        while x <= max_x {
            let (sx0, sy0) = viewport.world_to_screen(x, min_y, inner);
            let (sx1, sy1) = viewport.world_to_screen(x, max_y, inner);
            draw.push_line(sx0, sy0, sx1, sy1, color, width);
            x += world_step;
        }
        let mut y = start_y;
        while y <= max_y {
            let (sx0, sy0) = viewport.world_to_screen(min_x, y, inner);
            let (sx1, sy1) = viewport.world_to_screen(max_x, y, inner);
            draw.push_line(sx0, sy0, sx1, sy1, color, width);
            y += world_step;
        }
    }
}

fn draw_dashed_line(draw: &mut ui_wgpu::wgpu::DrawList, x0: f32, y0: f32, x1: f32, y1: f32, color: Rgba, width: f32) {
    let dx = x1 - x0;
    let dy = y1 - y0;
    let len = (dx * dx + dy * dy).sqrt().max(0.001);
    let ux = dx / len;
    let uy = dy / len;
    let dash = 4.0f32;
    let gap = 4.0f32;
    let mut traveled = 0.0f32;
    let mut drawing = true;
    while traveled < len {
        let segment = if drawing { dash } else { gap };
        let next = (traveled + segment).min(len);
        if drawing {
            let sx0 = x0 + ux * traveled;
            let sy0 = y0 + uy * traveled;
            let sx1 = x0 + ux * next;
            let sy1 = y0 + uy * next;
            draw.push_line(sx0, sy0, sx1, sy1, color, width);
        }
        traveled = next;
        drawing = !drawing;
    }
}

fn canvas_color_channels(v: &[f64], opacity: f32) -> Rgba {
    let r = v.first().copied().unwrap_or(0.58) as f32;
    let g = v.get(1).copied().unwrap_or(0.64) as f32;
    let b = v.get(2).copied().unwrap_or(0.72) as f32;
    let a = v.get(3).copied().unwrap_or(1.0) as f32;
    Rgba::new(r, g, b, (a * opacity).clamp(0.0, 1.0))
}

fn canvas_mix_rgba(a: Rgba, b: Rgba, t: f32) -> Rgba {
    let t = t.clamp(0.0, 1.0);
    Rgba::new(a.r + (b.r - a.r) * t, a.g + (b.g - a.g) * t, a.b + (b.b - a.b) * t, a.a + (b.a - a.a) * t)
}

fn canvas_gradient_color_at(stops: &[CanvasGradientStopJson], t: f32, opacity: f32) -> Rgba {
    if stops.is_empty() {
        return Rgba::new(0.58, 0.64, 0.72, 0.95 * opacity);
    }
    let t = t.clamp(0.0, 1.0);
    if stops.len() == 1 {
        return canvas_color_channels(stops[0].color.as_deref().unwrap_or(&[]), opacity);
    }
    let last = stops.len() - 1;
    for i in 0..last {
        let a_off = stops[i].offset.clamp(0.0, 1.0) as f32;
        let b_off = stops[i + 1].offset.clamp(0.0, 1.0) as f32;
        if t <= b_off || i == last - 1 {
            let span = (b_off - a_off).max(0.0001);
            let local = ((t - a_off) / span).clamp(0.0, 1.0);
            let ca = canvas_color_channels(stops[i].color.as_deref().unwrap_or(&[]), opacity);
            let cb = canvas_color_channels(stops[i + 1].color.as_deref().unwrap_or(&[]), opacity);
            return canvas_mix_rgba(ca, cb, local);
        }
    }
    canvas_color_channels(stops[last].color.as_deref().unwrap_or(&[]), opacity)
}

fn canvas_blend_channel(mode: &str, cb: f32, cs: f32) -> f32 {
    match mode {
        "multiply" => cb * cs,
        "screen" => cb + cs - cb * cs,
        "overlay" => canvas_blend_channel("hardLight", cs, cb),
        "darken" => cb.min(cs),
        "lighten" => cb.max(cs),
        "colorDodge" => {
            if cb <= 0.0 {
                0.0
            } else if cs >= 1.0 {
                1.0
            } else {
                (cb / (1.0 - cs)).min(1.0)
            }
        }
        "colorBurn" => {
            if cb >= 1.0 {
                1.0
            } else if cs <= 0.0 {
                0.0
            } else {
                1.0 - ((1.0 - cb) / cs).min(1.0)
            }
        }
        "hardLight" => {
            if cs <= 0.5 {
                2.0 * cb * cs
            } else {
                1.0 - 2.0 * (1.0 - cb) * (1.0 - cs)
            }
        }
        "softLight" => {
            let d = if cb <= 0.25 { ((16.0 * cb - 12.0) * cb + 4.0) * cb } else { cb.sqrt() };
            if cs <= 0.5 {
                cb - (1.0 - 2.0 * cs) * cb * (1.0 - cb)
            } else {
                cb + (2.0 * cs - 1.0) * (d - cb)
            }
        }
        "difference" => (cb - cs).abs(),
        "exclusion" => cb + cs - 2.0 * cb * cs,
        _ => cs,
    }
}

fn canvas_apply_blend_mode(mode: Option<&str>, backdrop: Rgba, source: Rgba) -> Rgba {
    let mode = match mode {
        None | Some("") | Some("normal") => return source,
        Some(m) => m,
    };
    let cb = [backdrop.r, backdrop.g, backdrop.b];
    let cs = [source.r, source.g, source.b];
    let blended = match mode {
        "hue" => canvas_set_lum(canvas_set_sat(cs, canvas_sat(cb)), canvas_lum(cb)),
        "saturation" => canvas_set_lum(canvas_set_sat(cb, canvas_sat(cs)), canvas_lum(cb)),
        "color" => canvas_set_lum(cs, canvas_lum(cb)),
        "luminosity" => canvas_set_lum(cb, canvas_lum(cs)),
        _ => [canvas_blend_channel(mode, cb[0], cs[0]), canvas_blend_channel(mode, cb[1], cs[1]), canvas_blend_channel(mode, cb[2], cs[2])],
    };
    Rgba::new(blended[0].clamp(0.0, 1.0), blended[1].clamp(0.0, 1.0), blended[2].clamp(0.0, 1.0), source.a)
}

fn push_shape_fill(draw: &mut ui_wgpu::wgpu::DrawList, rect: Rect, color: Rgba, is_circle: bool) {
    if is_circle {
        let cx = rect.x + rect.w * 0.5;
        let cy = rect.y + rect.h * 0.5;
        let radius = rect.w.min(rect.h) * 0.5;
        draw.push_triangle_fan(&canvas_circle_points(cx, cy, radius.max(0.5), CANVAS_CIRCLE_SEGMENTS), color);
    } else {
        draw.push_rounded([rect.x, rect.y, rect.w, rect.h], color, 4.0);
    }
}

fn push_circle_outline(draw: &mut ui_wgpu::wgpu::DrawList, cx: f32, cy: f32, radius: f32, color: Rgba, width: f32) {
    let points = canvas_circle_points(cx, cy, radius.max(0.5), CANVAS_CIRCLE_SEGMENTS);
    for i in 0..points.len() {
        let a = points[i];
        let b = points[(i + 1) % points.len()];
        draw.push_line(a[0], a[1], b[0], b[1], color, width);
    }
}

fn push_shape_outline(draw: &mut ui_wgpu::wgpu::DrawList, rect: Rect, color: Rgba, width: f32, is_circle: bool, dash: Option<&[f64]>) {
    if is_circle {
        let cx = rect.x + rect.w * 0.5;
        let cy = rect.y + rect.h * 0.5;
        let radius = rect.w.min(rect.h) * 0.5;
        push_circle_outline(draw, cx, cy, radius, color, width);
        return;
    }
    if dash.is_some_and(|d| !d.is_empty()) {
        draw_dashed_line(draw, rect.x, rect.y, rect.x + rect.w, rect.y, color, width);
        draw_dashed_line(draw, rect.x + rect.w, rect.y, rect.x + rect.w, rect.y + rect.h, color, width);
        draw_dashed_line(draw, rect.x + rect.w, rect.y + rect.h, rect.x, rect.y + rect.h, color, width);
        draw_dashed_line(draw, rect.x, rect.y + rect.h, rect.x, rect.y, color, width);
    } else {
        draw_ink_rect_outline(draw, rect.x, rect.y, rect.w, rect.h, color, width);
    }
}

fn push_linear_gradient_fill(draw: &mut ui_wgpu::wgpu::DrawList, viewport: &Viewport, inner: Rect, clip: Rect, origin_x: f64, origin_y: f64, fill: &CanvasFillJson, opacity: f32, blend: Option<&str>, backdrop: Rgba) {
    let (sx1, sy1) = viewport.world_to_screen((origin_x + fill.x1) as f32, (origin_y + fill.y1) as f32, inner);
    let (sx2, sy2) = viewport.world_to_screen((origin_x + fill.x2) as f32, (origin_y + fill.y2) as f32, inner);
    let dx = sx2 - sx1;
    let dy = sy2 - sy1;
    let len = (dx * dx + dy * dy).sqrt();
    draw.push_scissor(clip);
    if len < 0.5 {
        let color = canvas_apply_blend_mode(blend, backdrop, canvas_gradient_color_at(&fill.stops, 1.0, opacity));
        draw.push_rounded([clip.x, clip.y, clip.w, clip.h], color, 0.0);
        draw.pop_scissor();
        return;
    }
    let ux = dx / len;
    let uy = dy / len;
    let px = -uy;
    let py = ux;
    let overhang = (clip.w + clip.h).max(32.0);
    for i in 0..CANVAS_GRADIENT_BANDS {
        let t0 = i as f32 / CANVAS_GRADIENT_BANDS as f32;
        let t1 = (i + 1) as f32 / CANVAS_GRADIENT_BANDS as f32;
        let color = canvas_apply_blend_mode(blend, backdrop, canvas_gradient_color_at(&fill.stops, (t0 + t1) * 0.5, opacity));
        let a0 = if i == 0 { -overhang } else { t0 * len };
        let a1 = if i == CANVAS_GRADIENT_BANDS - 1 { len + overhang } else { t1 * len };
        let bx = sx1 + ux * a0;
        let by = sy1 + uy * a0;
        let ex = sx1 + ux * a1;
        let ey = sy1 + uy * a1;
        let points = [[bx + px * overhang, by + py * overhang], [ex + px * overhang, ey + py * overhang], [ex - px * overhang, ey - py * overhang], [bx - px * overhang, by - py * overhang]];
        draw.push_triangle_fan(&points, color);
    }
    draw.pop_scissor();
}

fn push_radial_gradient_fill(draw: &mut ui_wgpu::wgpu::DrawList, viewport: &Viewport, inner: Rect, clip: Rect, origin_x: f64, origin_y: f64, fill: &CanvasFillJson, opacity: f32, blend: Option<&str>, backdrop: Rgba) {
    let (scx, scy) = viewport.world_to_screen((origin_x + fill.cx) as f32, (origin_y + fill.cy) as f32, inner);
    let sr = (fill.r as f32 * viewport.zoom).max(0.5);
    draw.push_scissor(clip);
    let outer_radius = sr.max(clip.w.max(clip.h));
    for i in (0..CANVAS_GRADIENT_BANDS).rev() {
        let t = (i + 1) as f32 / CANVAS_GRADIENT_BANDS as f32;
        let radius = if i == CANVAS_GRADIENT_BANDS - 1 { outer_radius } else { sr * t };
        let color = canvas_apply_blend_mode(blend, backdrop, canvas_gradient_color_at(&fill.stops, t, opacity));
        draw.push_triangle_fan(&canvas_circle_points(scx, scy, radius.max(0.5), CANVAS_CIRCLE_SEGMENTS), color);
    }
    draw.pop_scissor();
}

#[allow(clippy::too_many_arguments, reason = "one arg per resolved paint/geometry input; grouping into a struct is a T2 restructure, out of scope")]
fn render_canvas_shape_fill(draw: &mut ui_wgpu::wgpu::DrawList, viewport: &Viewport, inner: Rect, shape_rect: Rect, layer: &CanvasLayer, opacity: f32, fallback_fill: Rgba, fallback_stroke: Rgba, backdrop: Rgba, is_circle: bool) {
    let blend = layer.blend_mode.as_deref();
    match &layer.fill {
        Some(fill) if fill.kind.as_deref() == Some("linearGradient") && !fill.stops.is_empty() => {
            push_linear_gradient_fill(draw, viewport, inner, shape_rect, layer.x, layer.y, fill, opacity, blend, backdrop);
        }
        Some(fill) if fill.kind.as_deref() == Some("radialGradient") && !fill.stops.is_empty() => {
            push_radial_gradient_fill(draw, viewport, inner, shape_rect, layer.x, layer.y, fill, opacity, blend, backdrop);
        }
        Some(fill) if fill.color.is_some() => {
            let solid = canvas_apply_blend_mode(blend, backdrop, canvas_color_channels(fill.color.as_deref().unwrap_or(&[]), opacity));
            push_shape_fill(draw, shape_rect, solid, is_circle);
        }
        _ => {
            let solid = canvas_apply_blend_mode(blend, backdrop, fallback_fill);
            push_shape_fill(draw, shape_rect, solid, is_circle);
        }
    }
    if let Some(stroke) = &layer.stroke {
        let color = stroke.color.as_deref().map(|c| canvas_apply_blend_mode(blend, backdrop, canvas_color_channels(c, opacity))).unwrap_or_else(|| fallback_stroke.with_alpha(fallback_stroke.a * opacity));
        let width = (stroke.width.unwrap_or(1.0) as f32).max(1.0);
        push_shape_outline(draw, shape_rect, color, width, is_circle, stroke.dash.as_deref());
    }
}

fn render_canvas2d_packet_item(item: &Canvas2dPacketItem<'_>, viewport: &Viewport, inner: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let theme = ctx.theme;
    let color = if item.id.starts_with("residual-field-") {
        theme.diagram_field_residual
    } else if item.id.starts_with("reaction-field-") || item.id.starts_with("load-") {
        theme.diagram_field_reaction
    } else if item.id.starts_with("displacement-field-") || item.id.starts_with("mode-field-") {
        theme.diagram_field_displacement
    } else {
        theme.diagram_accent
    };
    if item.kind == "line" {
        let (x0, y0) = viewport.world_to_screen(item.x0 as f32, item.y0 as f32, inner);
        let (x1, y1) = viewport.world_to_screen(item.x1 as f32, item.y1 as f32, inner);
        ctx.draw.push_line(x0, y0, x1, y1, color, (2.0 * viewport.zoom).max(1.0));
    } else if item.kind == "circle" {
        let (x, y) = viewport.world_to_screen(item.x as f32, item.y as f32, inner);
        let width = (item.width as f32 * viewport.zoom).max(4.0);
        let height = (item.height as f32 * viewport.zoom).max(4.0);
        ctx.draw.push_solid([x, y, width, height], color);
    } else if item.kind == "text" {
        let Some(text) = item.text.as_ref() else { return };
        let (x, y) = viewport.world_to_screen(item.x as f32, item.y as f32, inner);
        draw_text(ctx, text.content, x, y + text.size as f32, (text.size as f32).max(8.0), theme.text);
    }
}

/// 🗒️ `role === "meta"` (activeUtility bookkeeping) and `visible === false` records are
/// non-visual — skip rendering entirely, matches `layers.filter(role !== "meta")` in
/// `canvas-2d-host.tsx`'s `JsonLayersCanvasSession.renderFrame`.
/// 🖼️ Generic bounds-rect (or `kind === "circle"`) draw record — resolves solid/gradient
/// fill, blend-mode approximation, and stroke, matching `drawSceneNode`'s bounds-layer path.
/// 🖊️ Overlay annotation: a two-pass selection highlight (soft outer glow + crisp amber ring)
/// drawn on top of the shape, matches `drawBoundsLayer`'s `isSelected` glow+ring pair in
/// `canvas-2d-host.tsx` (glow at +4px/width 5, ring at +0px/width 2.5, both amber).
/// 🫙️ `layers.length === 0 → ctx.fillText("Empty canvas", -36, 0)` in
/// `🧱️elements/📐️Canvas2dHost/🟦️.tsx`'s `renderFrame` — React counts the same `role !== "meta"`
/// records this loop iterates, so the emptiness verdict matches without a second parse.
fn render_canvas_2d(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let theme = ctx.theme;
    let Some(canvas) = &scene.canvas_2d else {
        return render_placeholder("canvas-2d", bounds, ctx);
    };
    let inner = bounds;
    ctx.draw.push_solid([inner.x, inner.y, inner.w, inner.h], theme.canvas_clear);
    let mut viewport = Viewport { x: canvas.camera_x as f32, y: canvas.camera_y as f32, zoom: canvas.zoom as f32 };
    let local = scene_state(&scene.surface_id);
    if local.viewport.zoom > 0.0 && scene.component_kind == SurfaceKind::Canvas2d {
        viewport = local.viewport;
    }
    draw_canvas_infinite_grid(ctx.draw, &viewport, inner, theme);
    if let Some(snapshot) = canvas.snapshot {
        for page_index in 0..snapshot.page_count {
            let _ = ui_wgpu::wgpu::canvas2d_snapshot_with_page(snapshot, page_index, |page| {
                for item in serde_json::Deserializer::from_slice(page.bytes()).into_iter::<Canvas2dPacketItem<'_>>().flatten() {
                    render_canvas2d_packet_item(&item, &viewport, inner, ctx);
                }
            });
        }
        ctx.input.register_hit(HitTarget { rect: inner, event: None, control_id: Some(scene.surface_id.clone()), kind: HitKind::Generic, drag_axis: Some(DragAxis::Both), drag_data: None });
        return;
    }
    let layers: Vec<CanvasLayer> = serde_json::from_str(&canvas.layers_json).unwrap_or_default();
    let has_polyline = layers.iter().any(|layer| layer.kind == "polyline");
    if has_polyline {
        draw_checkerboard(ctx.draw, &viewport, inner, ctx.theme, 1024.0);
    }
    for (index, layer) in layers.iter().enumerate() {
        if !canvas_layer_should_render(layer) {
            continue;
        }
        let opacity = layer.opacity.unwrap_or(1.0).clamp(0.0, 1.0);
        let blend = layer.blend_mode.as_deref();
        if layer.kind == "image" {
            let source = layer.data_url.clone().or_else(|| layer.image.as_ref().and_then(|image| image.src.clone()));
            if let Some(data_url) = source.filter(|src| src.starts_with("data:")) {
                if let Some(key) = queue_canvas_image_upload(&scene.surface_id, &layer.id, &data_url) {
                    let (sx, sy) = viewport.world_to_screen(layer.x as f32, layer.y as f32, inner);
                    let iw = layer.image.as_ref().and_then(|image| image.width).unwrap_or(layer.width);
                    let ih = layer.image.as_ref().and_then(|image| image.height).unwrap_or(layer.height);
                    let w = iw as f32 * viewport.zoom;
                    let h = ih as f32 * viewport.zoom;
                    ctx.draw.push_raster_quad(&key, [sx, sy, w.max(1.0), h.max(1.0)], [0.0, 0.0, 1.0, 1.0], opacity);
                }
            }
            continue;
        }
        if layer.kind == "polyline" {
            if let Some(points) = &layer.points {
                let stroke = theme.diagram_stroke.with_alpha(theme.diagram_stroke.a * opacity);
                let seam_stroke = theme.diagram_seam.with_alpha(theme.diagram_seam.a * opacity);
                let width = (1.5 * viewport.zoom).max(1.0);
                for (edge_index, chunk) in points.chunks(2).enumerate() {
                    if chunk.len() < 2 {
                        continue;
                    }
                    let (x0, y0) = viewport.world_to_screen(chunk[0][0] as f32, chunk[0][1] as f32, inner);
                    let (x1, y1) = viewport.world_to_screen(chunk[1][0] as f32, chunk[1][1] as f32, inner);
                    let is_seam = layer.seams.as_ref().and_then(|seams| seams.get(edge_index)).copied().unwrap_or(0) != 0;
                    if is_seam {
                        draw_dashed_line(ctx.draw, x0, y0, x1, y1, seam_stroke, width);
                    } else {
                        ctx.draw.push_line(x0, y0, x1, y1, stroke, width);
                    }
                }
            }
            continue;
        }
        let hue = (index * 47 % 360) as f32;
        if layer.kind == "line" || layer.x0.is_some() {
            let x0 = layer.x0.unwrap_or(layer.x) as f32;
            let y0 = layer.y0.unwrap_or(layer.y) as f32;
            let x1 = layer.x1.unwrap_or(layer.x + layer.width) as f32;
            let y1 = layer.y1.unwrap_or(layer.y + layer.height) as f32;
            let (sx0, sy0) = viewport.world_to_screen(x0, y0, inner);
            let (sx1, sy1) = viewport.world_to_screen(x1, y1, inner);
            let base_stroke = layer
                .stroke
                .as_ref()
                .and_then(|stroke| stroke.color.as_deref())
                .map(|c| canvas_color_channels(c, opacity))
                .unwrap_or_else(|| Rgba::new(theme.diagram_accent.r + hue / 720.0, theme.diagram_accent.g, theme.diagram_accent.b, theme.diagram_accent.a * opacity));
            let stroke = canvas_apply_blend_mode(blend, theme.canvas_clear, base_stroke);
            ctx.draw.push_line(sx0, sy0, sx1, sy1, stroke, (2.0 * viewport.zoom).max(1.0));
            continue;
        }
        let (sx, sy) = viewport.world_to_screen(layer.x as f32, layer.y as f32, inner);
        let w = (layer.width as f32 * viewport.zoom).max(8.0);
        let h = (layer.height as f32 * viewport.zoom).max(8.0);
        let shape_rect = Rect::new(sx, sy, w, h);
        let is_circle = layer.kind == "circle";
        let fallback_fill = Rgba::new(theme.diagram_accent_fill.r + hue / 720.0, theme.diagram_accent_fill.g, theme.diagram_accent_fill.b, theme.diagram_accent_fill.a * opacity);
        render_canvas_shape_fill(ctx.draw, &viewport, inner, shape_rect, layer, opacity, fallback_fill, theme.diagram_shape_outline, theme.canvas_clear, is_circle);
        if layer.selected.unwrap_or(false) {
            if is_circle {
                let cx = sx + w * 0.5;
                let cy = sy + h * 0.5;
                let r = w.min(h) * 0.5;
                push_circle_outline(ctx.draw, cx, cy, r + 4.0, CANVAS2D_SELECTION_GLOW, 5.0);
                push_circle_outline(ctx.draw, cx, cy, r, CANVAS2D_SELECTION_RING, 2.5);
            } else {
                draw_ink_rect_outline(ctx.draw, sx - 4.0, sy - 4.0, w + 8.0, h + 8.0, CANVAS2D_SELECTION_GLOW, 5.0);
                draw_ink_rect_outline(ctx.draw, sx, sy, w, h, CANVAS2D_SELECTION_RING, 2.5);
            }
        }
        if let Some(text) = layer.text.as_ref().and_then(|text| text.content.as_deref()) {
            let size = layer.text.as_ref().and_then(|t| t.size).unwrap_or(14.0) as f32;
            draw_text(ctx, text, sx + 2.0, sy + size.max(8.0), size.max(8.0), theme.text);
        } else {
            let label = if layer.name.is_empty() { layer.id.as_str() } else { layer.name.as_str() };
            if !label.is_empty() {
                draw_text(ctx, label, sx + 4.0, sy + 14.0, theme.font_size_small, theme.text);
            }
        }
    }
    if !layers.iter().any(|layer| layer.role.as_deref() != Some("meta")) {
        draw_text(ctx, CANVAS_2D_EMPTY_LABEL, inner.x + inner.w * 0.5 - 36.0, inner.y + inner.h * 0.5, theme.font_size_small, theme.text_muted);
    }
    ctx.input.register_hit(HitTarget { rect: inner, event: None, control_id: Some(scene.surface_id.clone()), kind: HitKind::Generic, drag_axis: Some(DragAxis::Both), drag_data: None });
}

/// 🫙️ The literal `📐️Canvas2dHost` paints in an empty canvas (`🟦️.tsx:634`).
const CANVAS_2D_EMPTY_LABEL: &str = "Empty canvas";










/** 🧾️ Reserves the fixed raster ledger before source preparation or the one-backing decoder. */
/// 🖼️📥️ Queues ONE already-decoded RGBA8 bitmap under its own verbatim `key` — everything
/// [`queue_canvas_image_upload_with`] does for an encoded Canvas layer minus the decode and minus the
/// `canvas-image:<surface>:<layer>` key convention. Its caller is the renderer frame's drain of
/// `ui_wgpu::wgpu::take_ui_image_upload`: `admit_ui_image` already decoded that `data:` PNG in
/// first-party code (`semio-framework-pixels`) and the retained paint keys its `KIND_RASTER` quad by
/// the image `src` itself, so the key must survive unchanged or the quad finds no texture.
pub(crate) fn queue_decoded_raster_upload(surface_id: &str, key: String, width: u32, height: u32, mut pixels: Vec<u8>) -> Option<String> {
    let expected = usize::try_from(width).ok()?.checked_mul(usize::try_from(height).ok()?)?.checked_mul(4)?;
    if expected == 0 || expected > RASTER_UPLOAD_BYTE_CAPACITY || pixels.len() != expected || key.len() > RASTER_UPLOAD_KEY_BYTE_CAPACITY {
        return None;
    }
    pixels.shrink_to_fit();
    let ready = PENDING_RASTER_STATE.with(|cell| {
        let mut surfaces = cell.borrow_mut();
        surfaces
            .get_or_insert_with(surface_id.to_string(), PendingRasterSurface::default)
            .is_some_and(|surface| !surface.queue.is_full() && surface.admission.is_none() && surface.rejected.is_none() && surface.retiring.is_none() && surface.closing.is_none())
    });
    if !ready {
        return None;
    }
    let reserved = PENDING_RASTER_STATE.with(|cell| {
        let mut surfaces = cell.borrow_mut();
        let surface = surfaces.get_mut(surface_id)?;
        match PreparedRasterReservation::try_reserve(key) {
            Ok(reservation) => {
                surface.admission = Some(reservation);
                Some(())
            }
            Err(rejected) => {
                surface.rejected = Some(rejected);
                None
            }
        }
    });
    reserved?;
    let claimed = PENDING_RASTER_STATE.with(|cell| {
        let mut surfaces = cell.borrow_mut();
        let surface = surfaces.get_mut(surface_id)?;
        let reservation = surface.admission.take()?;
        match reservation.claim(width, height) {
            Ok(reservation) => {
                surface.admission = Some(reservation);
                Some(())
            }
            Err(rejected) => {
                surface.rejected = Some(rejected);
                None
            }
        }
    });
    claimed?;
    let admitted = PENDING_RASTER_STATE.with(|cell| cell.borrow_mut().get_mut(surface_id).and_then(|surface| surface.admission.take()).map(|reservation| reservation.finalize(pixels, Vec::new(), width, height)));
    let (producer, published_key) = match admitted {
        Some(Ok(admitted)) => admitted,
        Some(Err(rejected)) => {
            PENDING_RASTER_STATE.with(|cell| {
                if let Some(surface) = cell.borrow_mut().get_mut(surface_id) {
                    surface.rejected = Some(rejected);
                }
            });
            return None;
        }
        None => return None,
    };
    let accepted = PENDING_RASTER_STATE.with(|cell| {
        let mut surfaces = cell.borrow_mut();
        let Some(surface) = surfaces.get_mut(surface_id) else { return false };
        match surface.queue.push_back(producer) {
            Ok(()) => true,
            Err(mut producer) => {
                producer.begin_close();
                surface.closing = Some(producer);
                false
            }
        }
    });
    accepted.then_some(published_key)
}

pub(crate) fn queue_canvas_image_upload_with(surface_id: &str, layer_id: &str, source_identity: &[u8], dimensions: impl FnOnce() -> Result<(u32, u32, Vec<u8>), Vec<u8>>, decode: impl FnOnce(&[u8]) -> Option<Vec<u8>>) -> Option<String> {
    if surface_id.len().saturating_add(layer_id.len()).saturating_add(32) > RASTER_UPLOAD_KEY_BYTE_CAPACITY || source_identity.len() > RASTER_UPLOAD_BYTE_CAPACITY.saturating_mul(2) {
        return None;
    }
    let key = format!("canvas-image:{surface_id}:{layer_id}");
    let ready = PENDING_RASTER_STATE.with(|cell| {
        let mut surfaces = cell.borrow_mut();
        surfaces
            .get_or_insert_with(surface_id.to_string(), PendingRasterSurface::default)
            .is_some_and(|surface| !surface.queue.is_full() && surface.admission.is_none() && surface.rejected.is_none() && surface.retiring.is_none() && surface.closing.is_none())
    });
    if !ready {
        return None;
    }
    let reserved = PENDING_RASTER_STATE.with(|cell| {
        let mut surfaces = cell.borrow_mut();
        let surface = surfaces.get_mut(surface_id)?;
        match PreparedRasterReservation::try_reserve_source(key, source_identity.len()) {
            Ok(reservation) => {
                surface.admission = Some(reservation);
                Some(true)
            }
            Err(rejected) => {
                surface.rejected = Some(rejected);
                None
            }
        }
    });
    if reserved != Some(true) {
        return None;
    }
    let (width, height, retained_source) = match dimensions() {
        Ok(dimensions) => dimensions,
        Err(retained_source) => {
            PENDING_RASTER_STATE.with(|cell| {
                let mut surfaces = cell.borrow_mut();
                let Some(surface) = surfaces.get_mut(surface_id) else { return };
                let Some(reservation) = surface.admission.take() else { return };
                surface.rejected = Some(reservation.reject_with_retained("raster source dimensions failed", Vec::new(), retained_source));
            });
            return None;
        }
    };
    let claimed = PENDING_RASTER_STATE.with(|cell| {
        let mut surfaces = cell.borrow_mut();
        let surface = surfaces.get_mut(surface_id)?;
        let reservation = surface.admission.take()?;
        match reservation.claim_with_retained(width, height, retained_source) {
            Ok((reservation, retained_source)) => {
                surface.admission = Some(reservation);
                Some(retained_source)
            }
            Err(rejected) => {
                surface.rejected = Some(rejected);
                None
            }
        }
    });
    let Some(retained_source) = claimed else { return None };
    let pixels = match decode(&retained_source) {
        Some(decoded) => decoded,
        None => {
            PENDING_RASTER_STATE.with(|cell| {
                let mut surfaces = cell.borrow_mut();
                let Some(surface) = surfaces.get_mut(surface_id) else { return };
                let Some(reservation) = surface.admission.take() else { return };
                surface.rejected = Some(reservation.reject_with_retained("raster source decode failed", Vec::new(), retained_source));
            });
            return None;
        }
    };
    let expected = (width as usize).saturating_mul(height as usize).saturating_mul(4);
    if expected > RASTER_UPLOAD_BYTE_CAPACITY || pixels.len() != expected {
        PENDING_RASTER_STATE.with(|cell| {
            let mut surfaces = cell.borrow_mut();
            let Some(surface) = surfaces.get_mut(surface_id) else { return };
            let Some(reservation) = surface.admission.take() else { return };
            surface.rejected = Some(reservation.reject_with_retained("decoded raster exceeded Canvas upload credits", pixels, retained_source));
        });
        return None;
    }
    let admitted = PENDING_RASTER_STATE.with(|cell| cell.borrow_mut().get_mut(surface_id).and_then(|surface| surface.admission.take()).map(|reservation| reservation.finalize(pixels, retained_source, width, height)));
    let (producer, published_key) = match admitted {
        Some(Ok(admitted)) => admitted,
        Some(Err(rejected)) => {
            PENDING_RASTER_STATE.with(|cell| {
                if let Some(surface) = cell.borrow_mut().get_mut(surface_id) {
                    surface.rejected = Some(rejected);
                }
            });
            return None;
        }
        None => return None,
    };
    let accepted = PENDING_RASTER_STATE.with(|cell| {
        let mut surfaces = cell.borrow_mut();
        let Some(surface) = surfaces.get_mut(surface_id) else { return false };
        match surface.queue.push_back(producer) {
            Ok(()) => true,
            Err(mut producer) => {
                producer.begin_close();
                surface.closing = Some(producer);
                false
            }
        }
    });
    if !accepted {
        return None;
    }
    Some(published_key)
}

/** 🖼️ Reserves first, then decodes one encoded Canvas image backing exactly once. */




/** Clamps checkerboard cell iteration to the world-space rect actually visible through `inner`
 * (intersected with the full `±extent/2` grid) instead of always walking the whole grid — a
 * continuously-rendering surface (paint-2d) was pushing up to `(extent/cell)^2` solid quads every
 * single frame regardless of zoom/pan, which starves headless WebGPU frame pacing. */


/** 📐️ Theme-aware LOD world grid for canvas-2d — same large/medium/small/micro steps as flow and infinite boards. */




//#region Canvas2dFillBlend
/** 🎨️ Reads an `[r,g,b,a?]` channel array (each `0..1`, matches `rgbaToCss` in `canvas-2d-host.tsx`)
 * into an `Rgba`, multiplying alpha by the layer's resolved `opacity`. Missing channels fall back to
 * a neutral slate gray (matches the React reference's `rgba(148, 163, 184, opacity)` default). */




/** 🌈️ Samples a `CanvasGradientStop[]` list at `t ∈ [0,1]`, linearly interpolating between the
 * bracketing stops — mirrors `CanvasGradient.addColorStop` sampling semantics. */


/** 🌗️ A single separable-blend-mode channel formula (W3C Compositing and Blending Level 1 §5.2) —
 * `cb` is the backdrop channel, `cs` the source channel, both `0..1`. */


fn canvas_lum(c: [f32; 3]) -> f32 {
    0.3 * c[0] + 0.59 * c[1] + 0.11 * c[2]
}

fn canvas_clip_color(c: [f32; 3]) -> [f32; 3] {
    let l = canvas_lum(c);
    let n = c[0].min(c[1]).min(c[2]);
    let x = c[0].max(c[1]).max(c[2]);
    let mut out = c;
    if n < 0.0 {
        for ch in out.iter_mut() {
            *ch = l + (*ch - l) * l / (l - n).max(1e-6);
        }
    }
    if x > 1.0 {
        for ch in out.iter_mut() {
            *ch = l + (*ch - l) * (1.0 - l) / (x - l).max(1e-6);
        }
    }
    out
}

fn canvas_set_lum(c: [f32; 3], l: f32) -> [f32; 3] {
    let d = l - canvas_lum(c);
    canvas_clip_color([c[0] + d, c[1] + d, c[2] + d])
}

fn canvas_sat(c: [f32; 3]) -> f32 {
    c[0].max(c[1]).max(c[2]) - c[0].min(c[1]).min(c[2])
}

fn canvas_set_sat(c: [f32; 3], s: f32) -> [f32; 3] {
    let mut idx = [0usize, 1, 2];
    idx.sort_by(|&a, &b| c[a].partial_cmp(&c[b]).unwrap_or(std::cmp::Ordering::Equal));
    let (imin, imid, imax) = (idx[0], idx[1], idx[2]);
    let mut out = [0.0f32; 3];
    if c[imax] > c[imin] {
        out[imid] = (c[imid] - c[imin]) * s / (c[imax] - c[imin]);
        out[imax] = s;
    }
    out[imin] = 0.0;
    out
}

/** 🎨️ Approximates the React `Canvas2dScene`'s 16 CSS blend modes (`BLEND_MODE_TO_COMPOSITE` in
 * `canvas-2d-host.tsx`) by pre-blending the resolved fill/stroke color against `backdrop` — a
 * stand-in for true per-pixel GPU compositing, which would require a `wgpu::BlendState` change per
 * draw call in the shared `ui_wgpu` pipeline (out of scope for this ticket's Canvas2d/Paint2d
 * regions; see ticket 26/07/11/WGPU-RENDERER-FULL-PARITY). The four non-separable modes
 * (hue/saturation/color/luminosity) follow the W3C SetLum/SetSat algorithm exactly. */

//#endregion Canvas2dFillBlend

//#region Canvas2dShapes
/// 🟠️ The band count a linear/radial gradient is approximated with — `ui_wgpu`'s draw list has no
/// gradient primitive, so `fillStyleToPaint`'s `createLinearGradient`/`createRadialGradient` become
/// this many solid quads/rings along the gradient axis.
const CANVAS_GRADIENT_BANDS: usize = 10;
const CANVAS_CIRCLE_SEGMENTS: usize = 28;

fn canvas_circle_points(cx: f32, cy: f32, radius: f32, segments: usize) -> Vec<[f32; 2]> {
    (0..segments)
        .map(|i| {
            let a = (i as f32 / segments as f32) * std::f32::consts::TAU;
            [cx + a.cos() * radius, cy + a.sin() * radius]
        })
        .collect()
}







/** 🌈️ Bands a linear gradient across `clip` (scissor-bounded to the shape's screen bbox) as
 * `CANVAS_GRADIENT_BANDS` solid quads perpendicular to the `(x1,y1)-(x2,y2)` axis — `ui_wgpu::wgpu::
 * DrawList` has no per-vertex gradient primitive, see `canvas_apply_blend_mode` doc comment. */


/** 🌈️ Bands a radial gradient as `CANVAS_GRADIENT_BANDS` concentric circles painted outer-to-inner
 * (painter's algorithm — smaller/later circles overpaint the center), scissor-bounded to `clip`. */


/** 🖌️ Resolves and draws a Canvas2dScene draw record's `fill` (solid / linear / radial gradient) and
 * `stroke`, matching `drawSceneNode`'s fill/stroke resolution in `canvas-2d-host.tsx`. */

//#endregion Canvas2dShapes

/** 🟡️ Selection-ring colors ported verbatim from `drawBoundsLayer`'s literal
 * `"rgba(251, 191, 36, 0.95|0.28)"` strings in `canvas-2d-host.tsx` — an amber that isn't backed by
 * any `Theme` token, so it's kept local to this region rather than mapped onto `theme.accent`
 * (which resolves to the app's red/crimson accent and previously made the ring the wrong hue). */
const CANVAS2D_SELECTION_RING: Rgba = Rgba::new(0.984_314, 0.749_02, 0.141_176, 0.95);
const CANVAS2D_SELECTION_GLOW: Rgba = Rgba::new(0.984_314, 0.749_02, 0.141_176, 0.28);




//#endregion Canvas2d

//#region Canvas2dTests
#[cfg(test)]
#[path = "../../🧪️tests/🔬️wgpu-canvas2d/🦀️.rs"]
mod canvas2d_tests;
//#endregion Canvas2dTests

//#region InkCanvas
// 📝️ Direct DrawList painting for ink-canvas, ported from ink-canvas-host.tsx (framework/renderer/react).

//#region InkCanvasModel
static INK_HOST_ID_COUNTER: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

fn create_ink_host_id(prefix: &str) -> String {
    let next = INK_HOST_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
    format!("{prefix}-host-{next}")
}

#[derive(Clone, Copy, Debug, Default)]
struct InkCameraF {
    x: f64,
    y: f64,
    zoom: f64,
}

impl From<InkCameraJson> for InkCameraF {
    fn from(camera: InkCameraJson) -> Self {
        Self { x: camera.x, y: camera.y, zoom: camera.zoom }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct InkCameraJson {
    #[serde(default)]
    x: f64,
    #[serde(default)]
    y: f64,
    #[serde(default = "ink_default_zoom")]
    zoom: f64,
}

fn ink_default_zoom() -> f64 {
    1.0
}

impl Default for InkCameraJson {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase", default)]
struct InkDocumentJson {
    schema: String,
    id: String,
    camera: InkCameraJson,
    blocks: Vec<Value>,
    active_utility: Option<String>,
    snap_enabled: Option<bool>,
    snap_grid_spacing: Option<f64>,
    eraser_radius: Option<f64>,
    grid_visible: Option<bool>,
    grid_spacing: Option<f64>,
    grid_subdivisions: Option<f64>,
    grid_opacity: Option<f64>,
    assets: HashMap<String, Value>,
}

impl Default for InkDocumentJson {
    fn default() -> Self {
        Self {
            schema: "ink.document".into(),
            id: "empty".into(),
            camera: InkCameraJson::default(),
            blocks: Vec::new(),
            active_utility: Some("selectDirect".into()),
            snap_enabled: None,
            snap_grid_spacing: None,
            eraser_radius: None,
            grid_visible: None,
            grid_spacing: None,
            grid_subdivisions: None,
            grid_opacity: None,
            assets: HashMap::new(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct InkBoundsF {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

impl InkBoundsF {
    fn contains_point(&self, x: f64, y: f64) -> bool {
        x >= self.x && x <= self.x + self.w && y >= self.y && y <= self.y + self.h
    }

    fn intersects(&self, other: &InkBoundsF) -> bool {
        self.x < other.x + other.w && self.x + self.w > other.x && self.y < other.y + other.h && self.y + self.h > other.y
    }
}

fn ink_item_str<'a>(block: &'a Value, key: &str) -> &'a str {
    block.get(key).and_then(Value::as_str).unwrap_or("")
}

fn ink_item_id(block: &Value) -> &str {
    ink_item_str(block, "id")
}

fn ink_item_kind(block: &Value) -> &str {
    ink_item_str(block, "kind")
}


fn ink_item_locked(block: &Value) -> bool {
    block.get("locked").and_then(Value::as_bool).unwrap_or(false)
}

fn ink_item_num(block: &Value, key: &str) -> f64 {
    block.get(key).and_then(Value::as_f64).unwrap_or(0.0)
}

fn ink_item_bounds(block: &Value) -> InkBoundsF {
    let x = ink_item_num(block, "x");
    let y = ink_item_num(block, "y");
    let w = ink_item_num(block, "width");
    let h = ink_item_num(block, "height");
    if ink_item_kind(block) == "stroke" {
        if let Some(points) = block.get("points").and_then(Value::as_array) {
            if !points.is_empty() {
                let mut min_x = f64::INFINITY;
                let mut min_y = f64::INFINITY;
                let mut max_x = f64::NEG_INFINITY;
                let mut max_y = f64::NEG_INFINITY;
                for point in points {
                    let px = point.get(0).and_then(Value::as_f64).unwrap_or(0.0);
                    let py = point.get(1).and_then(Value::as_f64).unwrap_or(0.0);
                    min_x = min_x.min(px);
                    min_y = min_y.min(py);
                    max_x = max_x.max(px);
                    max_y = max_y.max(py);
                }
                return InkBoundsF { x: x + min_x, y: y + min_y, w: (max_x - min_x).max(1.0), h: (max_y - min_y).max(1.0) };
            }
        }
    }
    InkBoundsF { x, y, w, h }
}













fn ink_scale_value(v: f64, from_min: f64, from_size: f64, to_min: f64, to_size: f64) -> f64 {
    if from_size <= 0.0 {
        return to_min;
    }
    to_min + ((v - from_min) / from_size) * to_size
}

fn scale_ink_item(block: &Value, from: InkBoundsF, to: InkBoundsF) -> Value {
    let bounds = ink_item_bounds(block);
    let next_x = ink_scale_value(bounds.x, from.x, from.w, to.x, to.w);
    let next_y = ink_scale_value(bounds.y, from.y, from.h, to.y, to.h);
    let next_w = (ink_scale_value(bounds.x + bounds.w, from.x, from.w, to.x, to.w) - next_x).max(8.0);
    let next_h = (ink_scale_value(bounds.y + bounds.h, from.y, from.h, to.y, to.h) - next_y).max(8.0);
    let mut cloned = block.clone();
    if let Some(obj) = cloned.as_object_mut() {
        obj.insert("x".into(), json!(next_x));
        obj.insert("y".into(), json!(next_y));
        obj.insert("width".into(), json!(next_w));
        obj.insert("height".into(), json!(next_h));
        if ink_item_kind(block) == "stroke" {
            let scale_x = if from.w > 0.0 { to.w / from.w } else { 1.0 };
            let scale_y = if from.h > 0.0 { to.h / from.h } else { 1.0 };
            if let Some(points) = block.get("points").and_then(Value::as_array) {
                let scaled: Vec<Value> = points
                    .iter()
                    .map(|p| {
                        let px = p.get(0).and_then(Value::as_f64).unwrap_or(0.0) * scale_x;
                        let py = p.get(1).and_then(Value::as_f64).unwrap_or(0.0) * scale_y;
                        json!([px, py])
                    })
                    .collect();
                obj.insert("points".into(), Value::Array(scaled));
            }
        }
    }
    cloned
}

fn ink_resize_bounds(from: InkBoundsF, handle: &str, dx: f64, dy: f64, min_size: f64) -> InkBoundsF {
    let mut x = from.x;
    let mut y = from.y;
    let mut w = from.w;
    let mut h = from.h;
    if handle.contains('e') {
        w = (w + dx).max(min_size);
    }
    if handle.contains('w') {
        let next_w = (w - dx).max(min_size);
        x += w - next_w;
        w = next_w;
    }
    if handle.contains('s') {
        h = (h + dy).max(min_size);
    }
    if handle.contains('n') {
        let next_h = (h - dy).max(min_size);
        y += h - next_h;
        h = next_h;
    }
    InkBoundsF { x, y, w, h }
}

fn ink_snap_coordinate(v: f64, spacing: f64) -> f64 {
    if spacing <= 0.0 {
        v
    } else {
        (v / spacing).round() * spacing
    }
}

fn ink_snap_point(x: f64, y: f64, spacing: f64) -> (f64, f64) {
    (ink_snap_coordinate(x, spacing), ink_snap_coordinate(y, spacing))
}



fn ink_maybe_snap_fields(snap_enabled: Option<bool>, snap_grid_spacing: Option<f64>, x: f64, y: f64) -> (f64, f64) {
    if snap_enabled.unwrap_or(false) {
        ink_snap_point(x, y, snap_grid_spacing.unwrap_or(8.0))
    } else {
        (x, y)
    }
}

fn ink_item_with_position(block: &Value, x: f64, y: f64) -> Value {
    let mut cloned = block.clone();
    if let Some(obj) = cloned.as_object_mut() {
        obj.insert("x".into(), json!(x));
        obj.insert("y".into(), json!(y));
    }
    cloned
}

fn create_ink_item(kind: &str, x: f64, y: f64) -> Value {
    let id = create_ink_host_id(kind);
    match kind {
        "image" => json!({
            "id": id, "name": "Image", "kind": "image", "x": x, "y": y, "width": 240.0, "height": 160.0,
            "rotation": 0.0, "visible": true, "locked": false, "imageKey": "placeholder",
        }),
        "table" => json!({
            "id": id, "name": "Table", "kind": "table", "x": x, "y": y, "width": 320.0, "height": 160.0,
            "rotation": 0.0, "visible": true, "locked": false,
            "columns": ["A", "B", "C"],
            "rows": [
                [{"content": ""}, {"content": ""}, {"content": ""}],
                [{"content": ""}, {"content": ""}, {"content": ""}],
            ],
        }),
        "math" => json!({
            "id": id, "name": "Math", "kind": "math", "x": x, "y": y, "width": 200.0, "height": 80.0,
            "rotation": 0.0, "visible": true, "locked": false, "tex": "E = mc^2", "displayMode": true,
        }),
        "stroke" => json!({
            "id": id, "name": "Ink", "kind": "stroke", "x": x, "y": y, "width": 1.0, "height": 1.0,
            "rotation": 0.0, "visible": true, "locked": false, "points": [], "strokeWidth": 3.0, "color": [0.0, 0.0, 0.0, 1.0],
        }),
        "group" => json!({
            "id": id, "name": "Group", "kind": "group", "x": x, "y": y, "width": 280.0, "height": 120.0,
            "rotation": 0.0, "visible": true, "locked": false, "children": [],
        }),
        _ => json!({
            "id": id, "name": "Text", "kind": "text", "x": x, "y": y, "width": 280.0, "height": 120.0,
            "rotation": 0.0, "visible": true, "locked": false,
            "paragraphs": [{"runs": [{"text": ""}]}], "fontSize": 18.0, "fontWeight": "normal", "align": "left",
        }),
    }
}



fn point_segment_distance(px: f64, py: f64, x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let dx = x2 - x1;
    let dy = y2 - y1;
    if dx == 0.0 && dy == 0.0 {
        return ((px - x1).powi(2) + (py - y1).powi(2)).sqrt();
    }
    let t = (((px - x1) * dx + (py - y1) * dy) / (dx * dx + dy * dy)).clamp(0.0, 1.0);
    ((px - (x1 + t * dx)).powi(2) + (py - (y1 + t * dy)).powi(2)).sqrt()
}

fn ink_points(block: &Value) -> Vec<(f64, f64)> {
    let bx = ink_item_num(block, "x");
    let by = ink_item_num(block, "y");
    block
        .get("points")
        .and_then(Value::as_array)
        .map(|points| {
            points
                .iter()
                .map(|p| {
                    let px = p.get(0).and_then(Value::as_f64).unwrap_or(0.0);
                    let py = p.get(1).and_then(Value::as_f64).unwrap_or(0.0);
                    (bx + px, by + py)
                })
                .collect()
        })
        .unwrap_or_default()
}

fn ink_hits_point(block: &Value, x: f64, y: f64, threshold: f64) -> bool {
    let points = ink_points(block);
    let stroke_width = ink_item_num(block, "strokeWidth");
    if points.len() < 2 {
        return points.first().map(|p| ((x - p.0).powi(2) + (y - p.1).powi(2)).sqrt() <= threshold).unwrap_or(false);
    }
    points.windows(2).any(|w| point_segment_distance(x, y, w[0].0, w[0].1, w[1].0, w[1].1) <= threshold + stroke_width / 2.0)
}



fn erase_ink_stroke_points_in_item(block: &Value, x: f64, y: f64, radius: f64) -> Vec<Value> {
    let bx = ink_item_num(block, "x");
    let by = ink_item_num(block, "y");
    let points = block.get("points").and_then(Value::as_array).cloned().unwrap_or_default();
    let mut kept_indices = Vec::new();
    for (index, point) in points.iter().enumerate() {
        let px = bx + point.get(0).and_then(Value::as_f64).unwrap_or(0.0);
        let py = by + point.get(1).and_then(Value::as_f64).unwrap_or(0.0);
        if ((px - x).powi(2) + (py - y).powi(2)).sqrt() > radius {
            kept_indices.push(index);
        }
    }
    if kept_indices.len() == points.len() {
        return vec![block.clone()];
    }
    if kept_indices.is_empty() {
        return Vec::new();
    }
    let mut runs: Vec<Vec<Value>> = Vec::new();
    let mut current: Vec<Value> = vec![points[kept_indices[0]].clone()];
    for window in kept_indices.windows(2) {
        if window[1] - window[0] > 1 {
            if current.len() >= 2 {
                runs.push(current);
            }
            current = vec![points[window[1]].clone()];
        } else {
            current.push(points[window[1]].clone());
        }
    }
    if current.len() >= 2 {
        runs.push(current);
    }
    let name = ink_item_str(block, "name").to_string();
    runs.into_iter()
        .enumerate()
        .map(|(index, pts)| {
            let mut cloned = block.clone();
            if let Some(obj) = cloned.as_object_mut() {
                if index > 0 {
                    obj.insert("id".into(), json!(create_ink_host_id("stroke")));
                    obj.insert("name".into(), json!(format!("{name} fragment")));
                }
                obj.insert("points".into(), Value::Array(pts));
            }
            cloned
        })
        .collect()
}



fn ink_screen_to_world(camera: InkCameraF, inner: Rect, sx: f32, sy: f32) -> (f64, f64) {
    let lx = (sx - inner.x) as f64;
    let ly = (sy - inner.y) as f64;
    ((lx - camera.x) / camera.zoom, (ly - camera.y) / camera.zoom)
}



//#endregion InkCanvasModel

//#region InkCanvasState
const INK_INTERACTION_DOCUMENT_BYTE_CAPACITY: usize = 16 * 1024;
const INK_INTERACTION_ITEM_CAPACITY: usize = 256;
const INK_SELECTION_ITEM_CAPACITY: usize = ui_wgpu::wgpu::action::ACTION_NODE_CAPACITY - 2;
const INK_EVENT_JSON_BYTE_CAPACITY: usize = ui_wgpu::wgpu::action::ACTION_STRING_BYTE_CAPACITY;

struct InkInteractionDocument {
    source: String,
    spans: Box<[Option<(u16, u16)>; INK_INTERACTION_ITEM_CAPACITY]>,
    span_len: usize,
    schema: String,
    id: String,
    camera: InkCameraJson,
    active_utility: Option<String>,
    snap_enabled: Option<bool>,
    snap_grid_spacing: Option<f64>,
    eraser_radius: Option<f64>,
}

impl InkInteractionDocument {
    fn block(&self, index: usize) -> Result<Option<Value>, ui_wgpu::wgpu::BoundedActionFault> {
        let Some((start, end)) = self.spans.get(index).and_then(|span| *span) else {
            return Ok(None);
        };
        serde_json::from_slice(&self.source.as_bytes()[usize::from(start)..usize::from(end)]).map(Some).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::Structure)
    }
}

fn ink_skip_ws(bytes: &[u8], mut index: usize) -> usize {
    while bytes.get(index).is_some_and(u8::is_ascii_whitespace) {
        index += 1;
    }
    index
}

fn ink_scan_string(bytes: &[u8], start: usize) -> Result<usize, ui_wgpu::wgpu::BoundedActionFault> {
    if bytes.get(start) != Some(&b'"') {
        return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
    }
    let mut index = start + 1;
    while let Some(byte) = bytes.get(index) {
        match byte {
            b'"' => return Ok(index + 1),
            b'\\' => index = index.checked_add(2).ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?,
            _ => index += 1,
        }
    }
    Err(ui_wgpu::wgpu::BoundedActionFault::Structure)
}

fn ink_skip_value(bytes: &[u8], start: usize, depth: usize) -> Result<usize, ui_wgpu::wgpu::BoundedActionFault> {
    if depth > ui_wgpu::wgpu::action::ACTION_DEPTH_CAPACITY {
        return Err(ui_wgpu::wgpu::BoundedActionFault::DepthCredits);
    }
    let mut index = ink_skip_ws(bytes, start);
    match bytes.get(index) {
        Some(b'"') => ink_scan_string(bytes, index),
        Some(b'[') => {
            index += 1;
            loop {
                index = ink_skip_ws(bytes, index);
                if bytes.get(index) == Some(&b']') {
                    return Ok(index + 1);
                }
                index = ink_skip_value(bytes, index, depth + 1)?;
                index = ink_skip_ws(bytes, index);
                match bytes.get(index) {
                    Some(b',') => index += 1,
                    Some(b']') => return Ok(index + 1),
                    _ => return Err(ui_wgpu::wgpu::BoundedActionFault::Structure),
                }
            }
        }
        Some(b'{') => {
            index += 1;
            loop {
                index = ink_skip_ws(bytes, index);
                if bytes.get(index) == Some(&b'}') {
                    return Ok(index + 1);
                }
                index = ink_scan_string(bytes, index)?;
                index = ink_skip_ws(bytes, index);
                if bytes.get(index) != Some(&b':') {
                    return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
                }
                index = ink_skip_value(bytes, index + 1, depth + 1)?;
                index = ink_skip_ws(bytes, index);
                match bytes.get(index) {
                    Some(b',') => index += 1,
                    Some(b'}') => return Ok(index + 1),
                    _ => return Err(ui_wgpu::wgpu::BoundedActionFault::Structure),
                }
            }
        }
        Some(_) => {
            while bytes.get(index).is_some_and(|byte| !byte.is_ascii_whitespace() && !matches!(*byte, b',' | b']' | b'}')) {
                index += 1;
            }
            Ok(index)
        }
        None => Err(ui_wgpu::wgpu::BoundedActionFault::Structure),
    }
}

fn ink_collect_block_array(bytes: &[u8], start: usize, spans: &mut [Option<(u16, u16)>; INK_INTERACTION_ITEM_CAPACITY], span_len: &mut usize, depth: usize) -> Result<usize, ui_wgpu::wgpu::BoundedActionFault> {
    if depth > ui_wgpu::wgpu::action::ACTION_DEPTH_CAPACITY || bytes.get(start) != Some(&b'[') {
        return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
    }
    let mut index = start + 1;
    loop {
        index = ink_skip_ws(bytes, index);
        if bytes.get(index) == Some(&b']') {
            return Ok(index + 1);
        }
        if *span_len == spans.len() {
            return Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
        }
        let slot = *span_len;
        *span_len += 1;
        let end = if bytes.get(index) == Some(&b'{') { ink_collect_block_object(bytes, index, spans, span_len, depth + 1)? } else { ink_skip_value(bytes, index, depth + 1)? };
        spans[slot] = Some((u16::try_from(index).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?, u16::try_from(end).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?));
        index = ink_skip_ws(bytes, end);
        match bytes.get(index) {
            Some(b',') => index += 1,
            Some(b']') => return Ok(index + 1),
            _ => return Err(ui_wgpu::wgpu::BoundedActionFault::Structure),
        }
    }
}

fn ink_collect_block_object(bytes: &[u8], start: usize, spans: &mut [Option<(u16, u16)>; INK_INTERACTION_ITEM_CAPACITY], span_len: &mut usize, depth: usize) -> Result<usize, ui_wgpu::wgpu::BoundedActionFault> {
    let mut index = start + 1;
    let mut is_group = false;
    let mut children = None;
    let end = loop {
        index = ink_skip_ws(bytes, index);
        if bytes.get(index) == Some(&b'}') {
            break index + 1;
        }
        let key_start = index + 1;
        let key_end_quote = ink_scan_string(bytes, index)? - 1;
        let key = bytes.get(key_start..key_end_quote).ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
        index = ink_skip_ws(bytes, key_end_quote + 1);
        if bytes.get(index) != Some(&b':') {
            return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
        }
        index = ink_skip_ws(bytes, index + 1);
        let value_start = index;
        index = ink_skip_value(bytes, value_start, depth + 1)?;
        if key == b"kind" {
            is_group = serde_json::from_slice::<String>(&bytes[value_start..index]).is_ok_and(|kind| kind == "group");
        } else if key == b"children" && bytes.get(value_start) == Some(&b'[') {
            children = Some((value_start, index));
        }
        index = ink_skip_ws(bytes, index);
        match bytes.get(index) {
            Some(b',') => index += 1,
            Some(b'}') => break index + 1,
            _ => return Err(ui_wgpu::wgpu::BoundedActionFault::Structure),
        }
    };
    if is_group {
        if let Some((children_start, children_end)) = children {
            let scanned_end = ink_collect_block_array(bytes, children_start, spans, span_len, depth + 1)?;
            if scanned_end != children_end {
                return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
            }
        }
    }
    Ok(end)
}

fn collect_ink_document_block_spans(bytes: &[u8], spans: &mut [Option<(u16, u16)>; INK_INTERACTION_ITEM_CAPACITY], span_len: &mut usize) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    let mut index = ink_skip_ws(bytes, 0);
    if bytes.get(index) != Some(&b'{') {
        return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
    }
    index += 1;
    loop {
        index = ink_skip_ws(bytes, index);
        if bytes.get(index) == Some(&b'}') {
            return Ok(());
        }
        let key_start = index + 1;
        let key_end_quote = ink_scan_string(bytes, index)? - 1;
        let blocks = bytes.get(key_start..key_end_quote) == Some(b"blocks");
        index = ink_skip_ws(bytes, key_end_quote + 1);
        if bytes.get(index) != Some(&b':') {
            return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
        }
        index = ink_skip_ws(bytes, index + 1);
        index = if blocks { ink_collect_block_array(bytes, index, spans, span_len, 0)? } else { ink_skip_value(bytes, index, 0)? };
        index = ink_skip_ws(bytes, index);
        match bytes.get(index) {
            Some(b',') => index += 1,
            Some(b'}') => return Ok(()),
            _ => return Err(ui_wgpu::wgpu::BoundedActionFault::Structure),
        }
    }
}

struct InkEventJsonPages {
    bytes: Box<[u8; INK_EVENT_JSON_BYTE_CAPACITY]>,
    len: usize,
    items: usize,
    sealed: bool,
}

struct InkRawPages {
    bytes: Box<[u8; INK_INTERACTION_DOCUMENT_BYTE_CAPACITY]>,
    spans: Box<[Option<(u16, u16)>; INK_INTERACTION_ITEM_CAPACITY]>,
    byte_len: usize,
    head: usize,
    len: usize,
}

struct InkOwnedRaw {
    bytes: Box<[u8; INK_INTERACTION_DOCUMENT_BYTE_CAPACITY]>,
    len: usize,
}

impl InkOwnedRaw {
    fn new(raw: &str) -> Result<Self, ui_wgpu::wgpu::BoundedActionFault> {
        if raw.len() > INK_INTERACTION_DOCUMENT_BYTE_CAPACITY {
            return Err(ui_wgpu::wgpu::BoundedActionFault::ByteCredits);
        }
        let mut bytes = Box::new([0; INK_INTERACTION_DOCUMENT_BYTE_CAPACITY]);
        bytes[..raw.len()].copy_from_slice(raw.as_bytes());
        Ok(Self { bytes, len: raw.len() })
    }

    fn as_str(&self) -> Result<&str, ui_wgpu::wgpu::BoundedActionFault> {
        std::str::from_utf8(&self.bytes[..self.len]).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::Structure)
    }
}

impl Default for InkRawPages {
    fn default() -> Self {
        Self { bytes: Box::new([0; INK_INTERACTION_DOCUMENT_BYTE_CAPACITY]), spans: Box::new(std::array::from_fn(|_| None)), byte_len: 0, head: 0, len: 0 }
    }
}

impl InkRawPages {
    fn push(&mut self, raw: &str) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
        if self.len == self.spans.len() {
            return Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
        }
        let end = self.byte_len.checked_add(raw.len()).ok_or(ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?;
        if end > self.bytes.len() {
            return Err(ui_wgpu::wgpu::BoundedActionFault::ByteCredits);
        }
        self.bytes[self.byte_len..end].copy_from_slice(raw.as_bytes());
        let index = (self.head + self.len) % self.spans.len();
        self.spans[index] = Some((u16::try_from(self.byte_len).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?, u16::try_from(end).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?));
        self.byte_len = end;
        self.len += 1;
        Ok(())
    }

    fn front(&self) -> Result<Option<&str>, ui_wgpu::wgpu::BoundedActionFault> {
        let Some((start, end)) = self.spans[self.head] else {
            return Ok(None);
        };
        std::str::from_utf8(&self.bytes[usize::from(start)..usize::from(end)]).map(Some).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::Structure)
    }

    fn pop_front(&mut self) -> bool {
        if self.len == 0 {
            return false;
        }
        self.spans[self.head] = None;
        self.head = (self.head + 1) % self.spans.len();
        self.len -= 1;
        if self.len == 0 {
            self.head = 0;
            self.byte_len = 0;
        }
        true
    }
}

impl Default for InkEventJsonPages {
    fn default() -> Self {
        let mut bytes = Box::new([0; INK_EVENT_JSON_BYTE_CAPACITY]);
        bytes[0] = b'[';
        Self { bytes, len: 1, items: 0, sealed: false }
    }
}

impl std::io::Write for InkEventJsonPages {
    fn write(&mut self, source: &[u8]) -> std::io::Result<usize> {
        let end = self.len.checked_add(source.len()).ok_or_else(|| std::io::Error::new(std::io::ErrorKind::WriteZero, "ink event byte credits exhausted"))?;
        if end >= INK_EVENT_JSON_BYTE_CAPACITY {
            return Err(std::io::Error::new(std::io::ErrorKind::WriteZero, "ink event byte credits exhausted"));
        }
        self.bytes[self.len..end].copy_from_slice(source);
        self.len = end;
        Ok(source.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl InkEventJsonPages {
    fn push(&mut self, value: &Value) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
        if self.sealed || self.items == INK_INTERACTION_ITEM_CAPACITY {
            return Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
        }
        let checkpoint = self.len;
        let result = (|| {
            if self.items != 0 {
                self.write_all(b",").map_err(|_| ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?;
            }
            serde_json::to_writer(&mut *self, value).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::ByteCredits)
        })();
        if let Err(fault) = result {
            self.len = checkpoint;
            return Err(fault);
        }
        self.items += 1;
        Ok(())
    }

    fn push_add_block_raw(&mut self, block: &str) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
        if self.sealed || self.items == INK_INTERACTION_ITEM_CAPACITY {
            return Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
        }
        let checkpoint = self.len;
        let result = (|| {
            if self.items != 0 {
                self.write_all(b",").map_err(|_| ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?;
            }
            self.write_all(b"{\"operation\":\"addBlock\",\"block\":").map_err(|_| ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?;
            self.write_all(block.as_bytes()).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?;
            self.write_all(b"}").map_err(|_| ui_wgpu::wgpu::BoundedActionFault::ByteCredits)
        })();
        if let Err(fault) = result {
            self.len = checkpoint;
            return Err(fault);
        }
        self.items += 1;
        Ok(())
    }

    fn seal(&mut self) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
        if self.sealed {
            return Ok(());
        }
        self.write_all(b"]").map_err(|_| ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?;
        self.sealed = true;
        Ok(())
    }

    fn as_str(&self) -> Result<&str, ui_wgpu::wgpu::BoundedActionFault> {
        if !self.sealed {
            return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
        }
        std::str::from_utf8(&self.bytes[..self.len]).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::Structure)
    }
}

fn write_ink_events_action(
    input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>,
    scene: &UiComponentSceneNode,
    events: &InkEventJsonPages,
    phase: &str,
    select_id: Option<&str>,
    mutate: impl FnOnce(),
) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    let events = events.as_str()?;
    let base = ui_wgpu::wgpu::checked_action_string_bytes(&[&scene.controller_id, "inkApplyEvents", "surfaceId", &scene.surface_id, "eventsJson", events, "phase", phase])?;
    let bytes = match select_id {
        Some(id) => base.checked_add(ui_wgpu::wgpu::checked_action_string_bytes(&["selectIds", id])?).filter(|bytes| *bytes <= ui_wgpu::wgpu::action::ACTION_ITEM_BYTE_CAPACITY).ok_or(ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?,
        None => base,
    };
    let mut reservation = input.reserve_action(&scene.controller_id, "inkApplyEvents", bytes)?;
    let builder = reservation.builder();
    builder.begin_object(None)?;
    builder.string(Some("surfaceId"), &scene.surface_id)?;
    builder.string(Some("eventsJson"), events)?;
    builder.string(Some("phase"), phase)?;
    if let Some(id) = select_id {
        builder.begin_array(Some("selectIds"))?;
        builder.string(None, id)?;
        builder.end_container()?;
    }
    builder.end_container()?;
    reservation.publish_with(mutate)
}

fn write_ink_owned_selection_actions(input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>, scene: &UiComponentSceneNode, ids: &[String], mutate: impl FnOnce()) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    if ids.len() > INK_SELECTION_ITEM_CAPACITY {
        return Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
    }
    let mut bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[&scene.controller_id, "setSelection", "surfaceId", &scene.surface_id, "ids"])?;
    for id in ids {
        bytes = bytes.checked_add(ui_wgpu::wgpu::checked_action_string_bytes(&[id])?).filter(|bytes| *bytes <= ui_wgpu::wgpu::action::ACTION_ITEM_BYTE_CAPACITY).ok_or(ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?;
    }
    let mut reservation = input.reserve_action(&scene.controller_id, "setSelection", bytes)?;
    let builder = reservation.builder();
    builder.begin_object(None)?;
    builder.string(Some("surfaceId"), &scene.surface_id)?;
    builder.begin_array(Some("ids"))?;
    for id in ids {
        builder.string(None, id)?;
    }
    builder.end_container()?;
    builder.end_container()?;
    reservation.publish_with(mutate)
}

fn write_ink_hover_action(input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>, scene: &UiComponentSceneNode, id: Option<&str>) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    let base = ui_wgpu::wgpu::checked_action_string_bytes(&[&scene.controller_id, "setHover", "surfaceId", &scene.surface_id, "id"])?;
    let bytes = match id {
        Some(id) => base.checked_add(ui_wgpu::wgpu::checked_action_string_bytes(&[id])?).filter(|bytes| *bytes <= ui_wgpu::wgpu::action::ACTION_ITEM_BYTE_CAPACITY).ok_or(ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?,
        None => base,
    };
    let mut reservation = input.reserve_action(&scene.controller_id, "setHover", bytes)?;
    let builder = reservation.builder();
    builder.begin_object(None)?;
    builder.string(Some("surfaceId"), &scene.surface_id)?;
    match id {
        Some(id) => builder.string(Some("id"), id)?,
        None => builder.null(Some("id"))?,
    }
    builder.end_container()?;
    reservation.publish()
}

fn clear_ink_pointer_state(surface_id: &str) {
    mutate_scene_state(surface_id, |state| {
        state.drag = None;
        state.pointer_was_down = false;
        state.ink_marquee_points.clear();
    });
}

fn checked_ink_document(scene: &UiComponentSceneNode) -> Result<Option<InkInteractionDocument>, ui_wgpu::wgpu::BoundedActionFault> {
    let Some(ink) = scene.ink_canvas.as_ref() else {
        return Ok(None);
    };
    if ink.document_json.len() > INK_INTERACTION_DOCUMENT_BYTE_CAPACITY || ink.selection_json.len() > ui_wgpu::wgpu::action::ACTION_STRING_BYTE_CAPACITY {
        return Err(ui_wgpu::wgpu::BoundedActionFault::ByteCredits);
    }
    let document = serde_json::from_str::<InkDocumentJson>(&ink.document_json).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::Structure)?;
    let mut nodes = 0usize;
    for block in &document.blocks {
        validate_ink_value(block, 0, &mut nodes)?;
    }
    for (key, asset) in &document.assets {
        if key.len() > SCENE_SURFACE_ID_BYTE_CAPACITY {
            return Err(ui_wgpu::wgpu::BoundedActionFault::StringCredits);
        }
        validate_ink_value(asset, 0, &mut nodes)?;
    }
    if document.blocks.len() > INK_INTERACTION_ITEM_CAPACITY || document.assets.len() > INK_INTERACTION_ITEM_CAPACITY {
        return Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
    }
    let source = ink.document_json.clone();
    let mut spans = Box::new(std::array::from_fn(|_| None));
    let mut span_len = 0usize;
    collect_ink_document_block_spans(source.as_bytes(), &mut spans, &mut span_len)?;
    Ok(Some(InkInteractionDocument {
        source,
        spans,
        span_len,
        schema: document.schema,
        id: document.id,
        camera: document.camera,
        active_utility: document.active_utility,
        snap_enabled: document.snap_enabled,
        snap_grid_spacing: document.snap_grid_spacing,
        eraser_radius: document.eraser_radius,
    }))
}

fn validate_ink_value(value: &Value, depth: usize, nodes: &mut usize) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    if depth > ui_wgpu::wgpu::action::ACTION_DEPTH_CAPACITY {
        return Err(ui_wgpu::wgpu::BoundedActionFault::DepthCredits);
    }
    *nodes = nodes.checked_add(1).ok_or(ui_wgpu::wgpu::BoundedActionFault::NodeCredits)?;
    if *nodes > INK_INTERACTION_ITEM_CAPACITY {
        return Err(ui_wgpu::wgpu::BoundedActionFault::NodeCredits);
    }
    match value {
        Value::Array(values) => {
            for value in values {
                validate_ink_value(value, depth + 1, nodes)?;
            }
        }
        Value::Object(entries) => {
            for (key, value) in entries {
                if key.len() > SCENE_SURFACE_ID_BYTE_CAPACITY {
                    return Err(ui_wgpu::wgpu::BoundedActionFault::StringCredits);
                }
                validate_ink_value(value, depth + 1, nodes)?;
            }
        }
        Value::String(value) if value.len() > ui_wgpu::wgpu::action::ACTION_STRING_BYTE_CAPACITY => return Err(ui_wgpu::wgpu::BoundedActionFault::StringCredits),
        _ => {}
    }
    Ok(())
}

fn ink_camera_checked(scene: &UiComponentSceneNode) -> Result<InkCameraF, ui_wgpu::wgpu::BoundedActionFault> {
    if let Some(camera) = SCENE_STATE.with(|cell| cell.borrow().get(&scene.surface_id).and_then(|state| state.ink_camera)) {
        return Ok(InkCameraF { x: camera.0, y: camera.1, zoom: camera.2 });
    }
    Ok(checked_ink_document(scene)?.map(|document| InkCameraF::from(document.camera)).unwrap_or_default())
}

fn write_ink_camera_action(input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>, scene: &UiComponentSceneNode, camera: InkCameraF, mutate: impl FnOnce()) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[&scene.controller_id, "setCamera", "surfaceId", &scene.surface_id, "camera", "x", "y", "zoom"])?;
    let mut reservation = input.reserve_action(&scene.controller_id, "setCamera", bytes)?;
    let builder = reservation.builder();
    builder.begin_object(None)?;
    builder.string(Some("surfaceId"), &scene.surface_id)?;
    builder.begin_object(Some("camera"))?;
    builder.number(Some("x"), camera.x)?;
    builder.number(Some("y"), camera.y)?;
    builder.number(Some("zoom"), camera.zoom)?;
    builder.end_container()?;
    builder.end_container()?;
    reservation.publish_with(mutate)
}

pub(crate) fn ink_wheel_into(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32, delta: f32, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    let Some(ink) = scene.ink_canvas.as_ref() else {
        return Ok(false);
    };
    if ink.view_mode == "navigator" || !inner.contains(x, y) {
        return Ok(false);
    }
    let camera = ink_camera_checked(scene)?;
    let zoom_factor: f64 = if delta < 0.0 { 1.08 } else { 0.92 };
    let next_zoom = (camera.zoom * zoom_factor).clamp(0.1, 8.0);
    let (wx, wy) = ink_screen_to_world(camera, inner, x, y);
    let next = InkCameraF { x: (x - inner.x) as f64 - wx * next_zoom, y: (y - inner.y) as f64 - wy * next_zoom, zoom: next_zoom };
    write_ink_camera_action(input, scene, next, || {
        mutate_scene_state(&scene.surface_id, |state| {
            state.ink_camera = Some((next.x, next.y, next.zoom));
        });
    })?;
    Ok(true)
}

#[derive(Clone, Copy)]
pub(crate) enum InkInteractionEvent {
    PointerDown { x: f32, y: f32, button: i16, shift: bool },
    PointerUp { x: f32, y: f32 },
    PointerMove { x: f32, y: f32 },
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum InkInteractionStage {
    Scan,
    Publish,
    Complete,
}

pub(crate) enum InkInteractionStep {
    Pending,
    Complete,
}

#[derive(Default)]
struct InkBlockCursor {
    index: usize,
}

impl InkBlockCursor {
    fn next(&mut self, document: &InkInteractionDocument) -> Result<Option<Value>, ui_wgpu::wgpu::BoundedActionFault> {
        let block = document.block(self.index)?;
        if block.is_some() {
            self.index += 1;
        }
        Ok(block)
    }
}

pub(crate) struct InkInteractionJob {
    generation: u64,
    event: InkInteractionEvent,
    stage: InkInteractionStage,
    document: Option<InkInteractionDocument>,
    selected_ids: Vec<String>,
    result_ids: Vec<String>,
    result_bytes: usize,
    block_cursor: InkBlockCursor,
    hit_id: Option<String>,
    hit_origin: Option<(f64, f64)>,
    events: InkEventJsonPages,
    utility: String,
    camera: InkCameraF,
    drag: Option<SceneDragMode>,
    stroke_update: Option<(String, InkOwnedRaw)>,
    pending_fragments: InkRawPages,
    pending_remove_id: Option<String>,
}

fn checked_ink_drag(surface_id: &str) -> Result<Option<SceneDragMode>, ui_wgpu::wgpu::BoundedActionFault> {
    SCENE_STATE.with(|cell| {
        let states = cell.borrow();
        let Some(mode) = states.get(surface_id).and_then(|state| state.drag.as_ref()).map(|drag| &drag.mode) else {
            return Ok(None);
        };
        let mut bytes = 0usize;
        match mode {
            SceneDragMode::InkMove { origins, .. } => {
                if origins.len() > INK_INTERACTION_ITEM_CAPACITY {
                    return Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
                }
                for id in origins.keys() {
                    bytes = bytes.checked_add(id.len()).ok_or(ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?;
                }
            }
            SceneDragMode::InkResize { handle, selected_ids, .. } => {
                if selected_ids.len() > INK_INTERACTION_ITEM_CAPACITY {
                    return Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
                }
                bytes = handle.len();
                for id in selected_ids {
                    bytes = bytes.checked_add(id.len()).ok_or(ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?;
                }
            }
            SceneDragMode::InkStroke { block_id } => bytes = block_id.len(),
            SceneDragMode::InkEraser { mode } => bytes = mode.len(),
            _ => {}
        }
        if bytes > ui_wgpu::wgpu::action::ACTION_STRING_BYTE_CAPACITY {
            return Err(ui_wgpu::wgpu::BoundedActionFault::ByteCredits);
        }
        Ok(Some(mode.clone()))
    })
}

impl InkInteractionJob {
    pub(crate) fn new(generation: u64, scene: &UiComponentSceneNode, event: InkInteractionEvent) -> Result<Option<Self>, ui_wgpu::wgpu::BoundedActionFault> {
        if generation == 0 {
            return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
        }
        let Some(ink) = scene.ink_canvas.as_ref() else {
            return Ok(None);
        };
        if ink.view_mode == "navigator" || !ink.interactive {
            return Ok(None);
        }
        let Some(document) = checked_ink_document(scene)? else {
            return Ok(None);
        };
        let selected_ids: Vec<String> = serde_json::from_str(&ink.selection_json).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::Structure)?;
        if selected_ids.len() > INK_INTERACTION_ITEM_CAPACITY {
            return Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
        }
        let mut selected_bytes = 0usize;
        for id in &selected_ids {
            if id.len() > SCENE_SURFACE_ID_BYTE_CAPACITY {
                return Err(ui_wgpu::wgpu::BoundedActionFault::StringCredits);
            }
            selected_bytes = selected_bytes.checked_add(id.len()).ok_or(ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?;
        }
        if selected_bytes > ui_wgpu::wgpu::action::ACTION_STRING_BYTE_CAPACITY {
            return Err(ui_wgpu::wgpu::BoundedActionFault::ByteCredits);
        }
        let camera = SCENE_STATE.with(|cell| cell.borrow().get(&scene.surface_id).and_then(|state| state.ink_camera)).map(|(x, y, zoom)| InkCameraF { x, y, zoom }).unwrap_or_else(|| InkCameraF::from(document.camera.clone()));
        let drag = checked_ink_drag(&scene.surface_id)?;
        let utility = document.active_utility.clone().unwrap_or_else(|| "selectDirect".to_owned());
        if utility.len() > SCENE_SURFACE_ID_BYTE_CAPACITY {
            return Err(ui_wgpu::wgpu::BoundedActionFault::StringCredits);
        }
        Ok(Some(Self {
            generation,
            event,
            stage: InkInteractionStage::Scan,
            document: Some(document),
            selected_ids,
            result_ids: Vec::with_capacity(INK_INTERACTION_ITEM_CAPACITY),
            result_bytes: 0,
            block_cursor: InkBlockCursor::default(),
            hit_id: None,
            hit_origin: None,
            events: InkEventJsonPages::default(),
            utility,
            camera,
            drag,
            stroke_update: None,
            pending_fragments: InkRawPages::default(),
            pending_remove_id: None,
        }))
    }

    pub(crate) fn step(&mut self, generation: u64, scene: &UiComponentSceneNode, inner: Rect, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<InkInteractionStep, ui_wgpu::wgpu::BoundedActionFault> {
        if generation != self.generation {
            return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
        }
        if self.stage == InkInteractionStage::Complete {
            return Ok(InkInteractionStep::Complete);
        }
        if self.stage == InkInteractionStage::Scan {
            if self.scan_one(scene, inner)? {
                self.stage = InkInteractionStage::Publish;
            }
            return Ok(InkInteractionStep::Pending);
        }
        self.publish(scene, inner, input)?;
        self.stage = InkInteractionStage::Complete;
        Ok(InkInteractionStep::Complete)
    }

    fn scan_one(&mut self, scene: &UiComponentSceneNode, inner: Rect) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
        match self.event {
            InkInteractionEvent::PointerDown { x, y, button, .. } if button == 0 && (self.utility == "eraserStroke" || self.utility == "eraserPoint") => {
                if self.push_pending_eraser_event()? {
                    return Ok(false);
                }
                let document = self.document.as_ref().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
                let Some(block) = self.block_cursor.next(document)? else {
                    self.events.seal()?;
                    return Ok(true);
                };
                let (world_x, world_y) = ink_screen_to_world(self.camera, inner, x, y);
                let prepared = Self::prepare_eraser_block(&block, world_x, world_y, document.eraser_radius.unwrap_or(12.0), self.utility == "eraserPoint")?;
                if let Some((remove_id, fragments)) = prepared {
                    self.pending_remove_id = Some(remove_id);
                    self.pending_fragments = fragments;
                }
                Ok(false)
            }
            InkInteractionEvent::PointerDown { x, y, button: 0, .. } if self.utility == "selectDirect" => self.scan_hit(inner, x, y),
            InkInteractionEvent::PointerMove { x, y } => {
                if self.drag.is_none() {
                    self.scan_hit(inner, x, y)
                } else if matches!(self.drag.as_ref(), Some(SceneDragMode::InkMove { .. } | SceneDragMode::InkResize { .. } | SceneDragMode::InkStroke { .. } | SceneDragMode::InkEraser { .. })) {
                    self.scan_drag_event(scene, inner, x, y, false)
                } else {
                    Ok(true)
                }
            }
            InkInteractionEvent::PointerUp { x, y } => {
                if matches!(self.drag.as_ref(), Some(SceneDragMode::InkMove { .. } | SceneDragMode::InkResize { .. } | SceneDragMode::InkStroke { .. })) {
                    self.scan_drag_event(scene, inner, x, y, true)
                } else if let Some((start_x, start_y)) = self.drag.as_ref().and_then(|drag| match drag {
                    SceneDragMode::InkMarqueeDrag { start_x, start_y } => Some((*start_x, *start_y)),
                    _ => None,
                }) {
                    self.scan_marquee(inner, x, y, start_x, start_y)
                } else if matches!(self.drag.as_ref(), Some(SceneDragMode::InkEraser { .. })) {
                    self.events.seal()?;
                    Ok(true)
                } else {
                    Ok(true)
                }
            }
            _ => Ok(true),
        }
    }

    fn push_pending_eraser_event(&mut self) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
        if let Some(remove_id) = self.pending_remove_id.as_deref() {
            self.events.push(&json!({ "operation": "removeBlock", "blockId": remove_id }))?;
            self.pending_remove_id = None;
            return Ok(true);
        }
        if let Some(fragment) = self.pending_fragments.front()? {
            self.events.push_add_block_raw(fragment)?;
            self.pending_fragments.pop_front();
            return Ok(true);
        }
        Ok(false)
    }

    fn prepare_eraser_block(block: &Value, x: f64, y: f64, radius: f64, points_only: bool) -> Result<Option<(String, InkRawPages)>, ui_wgpu::wgpu::BoundedActionFault> {
        if ink_item_kind(block) != "stroke" || !ink_hits_point(block, x, y, radius) {
            return Ok(None);
        }
        let id = ink_item_id(block);
        if id.len() > SCENE_SURFACE_ID_BYTE_CAPACITY {
            return Err(ui_wgpu::wgpu::BoundedActionFault::StringCredits);
        }
        if !points_only {
            return Ok(Some((id.to_owned(), InkRawPages::default())));
        }
        let fragments = erase_ink_stroke_points_in_item(block, x, y, radius);
        if fragments.len() == 1 && fragments[0] == *block {
            return Ok(None);
        }
        if fragments.len() > INK_INTERACTION_ITEM_CAPACITY {
            return Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
        }
        for fragment in &fragments {
            let mut nodes = 0usize;
            validate_ink_value(fragment, 0, &mut nodes)?;
        }
        let mut encoded = InkRawPages::default();
        for fragment in fragments {
            let raw = serde_json::to_string(&fragment).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::Structure)?;
            if raw.len() > INK_INTERACTION_DOCUMENT_BYTE_CAPACITY {
                return Err(ui_wgpu::wgpu::BoundedActionFault::ByteCredits);
            }
            encoded.push(&raw)?;
        }
        Ok(Some((id.to_owned(), encoded)))
    }

    fn scan_drag_event(&mut self, scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32, commit: bool) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
        if matches!(self.drag.as_ref(), Some(SceneDragMode::InkEraser { .. })) {
            if self.push_pending_eraser_event()? {
                return Ok(false);
            }
            let document = self.document.as_ref().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
            let Some(block) = self.block_cursor.next(document)? else {
                self.events.seal()?;
                return Ok(true);
            };
            let (world_x, world_y) = ink_screen_to_world(self.camera, inner, x, y);
            let points_only = matches!(self.drag.as_ref(), Some(SceneDragMode::InkEraser { mode }) if mode == "eraserPoint");
            let prepared = Self::prepare_eraser_block(&block, world_x, world_y, document.eraser_radius.unwrap_or(12.0), points_only)?;
            if let Some((remove_id, fragments)) = prepared {
                self.pending_remove_id = Some(remove_id);
                self.pending_fragments = fragments;
            }
            return Ok(false);
        }
        let document = self.document.as_ref().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
        let Some(block) = self.block_cursor.next(document)? else {
            self.events.seal()?;
            return Ok(true);
        };
        let id = ink_item_id(&block);
        let event = match self.drag.as_ref().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)? {
            SceneDragMode::InkMove { origins, start_x, start_y } => origins.get(id).map(|(origin_x, origin_y)| {
                let dx = (x - *start_x) as f64 / self.camera.zoom.max(0.0001);
                let dy = (y - *start_y) as f64 / self.camera.zoom.max(0.0001);
                let updated = ink_item_with_position(&block, *origin_x + dx, *origin_y + dy);
                json!({ "operation": "updateBlock", "blockId": id, "block": updated })
            }),
            SceneDragMode::InkResize { handle, from, start_x, start_y, selected_ids } if selected_ids.iter().any(|selected| selected == id) => {
                let dx = (x - *start_x) as f64 / self.camera.zoom.max(0.0001);
                let dy = (y - *start_y) as f64 / self.camera.zoom.max(0.0001);
                let to = ink_resize_bounds(*from, handle, dx, dy, 8.0);
                let updated = scale_ink_item(&block, *from, to);
                Some(json!({ "operation": "updateBlock", "blockId": id, "block": updated }))
            }
            SceneDragMode::InkStroke { block_id } if block_id == id => {
                let mut updated = SCENE_STATE
                    .with(|cell| -> Result<Option<Value>, ui_wgpu::wgpu::BoundedActionFault> {
                        let states = cell.borrow();
                        let Some(value) = states.get(&scene.surface_id).and_then(|state| state.ink_overrides.get(id)) else {
                            return Ok(None);
                        };
                        let mut nodes = 0usize;
                        validate_ink_value(value, 0, &mut nodes)?;
                        Ok(Some(value.clone()))
                    })?
                    .unwrap_or_else(|| block.clone());
                if !commit {
                    let (world_x, world_y) = ink_screen_to_world(self.camera, inner, x, y);
                    let block_x = ink_item_num(&updated, "x");
                    let block_y = ink_item_num(&updated, "y");
                    if let Some(points) = updated.get_mut("points").and_then(Value::as_array_mut) {
                        if points.len() == INK_INTERACTION_ITEM_CAPACITY {
                            return Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
                        }
                        points.push(json!([world_x - block_x, world_y - block_y]));
                    }
                }
                let update_json = serde_json::to_string(&updated).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::Structure)?;
                self.stroke_update = Some((id.to_owned(), InkOwnedRaw::new(&update_json)?));
                Some(json!({ "operation": "updateBlock", "blockId": id, "block": updated }))
            }
            _ => None,
        };
        if let Some(event) = event {
            self.events.push(&event)?;
        }
        Ok(false)
    }

    fn scan_marquee(&mut self, inner: Rect, x: f32, y: f32, start_x: f32, start_y: f32) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
        let document = self.document.as_ref().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
        let Some(block) = self.block_cursor.next(document)? else {
            return Ok(true);
        };
        let x0 = start_x.min(x);
        let y0 = start_y.min(y);
        let (world_x0, world_y0) = ink_screen_to_world(self.camera, inner, x0, y0);
        let (world_x1, world_y1) = ink_screen_to_world(self.camera, inner, start_x.max(x), start_y.max(y));
        let rectangle = InkBoundsF { x: world_x0.min(world_x1), y: world_y0.min(world_y1), w: (world_x1 - world_x0).abs(), h: (world_y1 - world_y0).abs() };
        if ink_item_bounds(&block).intersects(&rectangle) {
            let id = ink_item_id(&block);
            let bytes = self.result_bytes.checked_add(id.len()).ok_or(ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?;
            if self.result_ids.len() == INK_SELECTION_ITEM_CAPACITY || bytes > ui_wgpu::wgpu::action::ACTION_STRING_BYTE_CAPACITY {
                return Err(ui_wgpu::wgpu::BoundedActionFault::ByteCredits);
            }
            self.result_ids.push(id.to_owned());
            self.result_bytes = bytes;
        }
        Ok(false)
    }

    fn scan_hit(&mut self, inner: Rect, x: f32, y: f32) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
        let document = self.document.as_ref().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
        let Some(block) = self.block_cursor.next(document)? else {
            return Ok(true);
        };
        let hit = {
            let (world_x, world_y) = ink_screen_to_world(self.camera, inner, x, y);
            if !ink_item_locked(&block) && ink_item_bounds(&block).contains_point(world_x, world_y) {
                let id = ink_item_id(&block);
                if id.len() > SCENE_SURFACE_ID_BYTE_CAPACITY {
                    return Err(ui_wgpu::wgpu::BoundedActionFault::StringCredits);
                }
                Some((id.to_owned(), (ink_item_num(&block, "x"), ink_item_num(&block, "y"))))
            } else {
                None
            }
        };
        if let Some((id, origin)) = hit {
            self.hit_id = Some(id);
            self.hit_origin = Some(origin);
        }
        Ok(false)
    }

    fn publish(&mut self, scene: &UiComponentSceneNode, inner: Rect, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
        match self.event {
            InkInteractionEvent::PointerDown { x, y, button, shift } => self.publish_down(scene, inner, x, y, button, shift, input),
            InkInteractionEvent::PointerUp { .. } => {
                if matches!(self.drag.as_ref(), Some(SceneDragMode::InkMarqueeDrag { .. })) {
                    return write_ink_owned_selection_actions(input, scene, &self.result_ids, || clear_ink_pointer_state(&scene.surface_id));
                }
                if matches!(self.drag.as_ref(), Some(SceneDragMode::InkMove { .. } | SceneDragMode::InkResize { .. } | SceneDragMode::InkStroke { .. } | SceneDragMode::InkEraser { .. })) {
                    return write_ink_events_action(input, scene, &self.events, "commit", None, || clear_ink_pointer_state(&scene.surface_id));
                }
                clear_ink_pointer_state(&scene.surface_id);
                Ok(())
            }
            InkInteractionEvent::PointerMove { x, y } => self.publish_move(scene, inner, x, y, input),
        }
    }

    fn publish_down(&mut self, scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32, button: i16, shift: bool, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
        if self.utility == "pan" || button == 1 {
            let camera = self.camera;
            mutate_scene_state(&scene.surface_id, |state| {
                state.pointer_was_down = true;
                state.drag = Some(SceneDrag { mode: SceneDragMode::InkPan { start_x: x, start_y: y, camera_x: camera.x, camera_y: camera.y, zoom: camera.zoom } });
            });
            return Ok(());
        }
        if button != 0 {
            return Ok(());
        }
        if self.utility == "eraserStroke" || self.utility == "eraserPoint" {
            let utility = self.utility.clone();
            if self.events.items == 0 {
                mutate_scene_state(&scene.surface_id, |state| {
                    state.pointer_was_down = true;
                    state.drag = Some(SceneDrag { mode: SceneDragMode::InkEraser { mode: utility } });
                });
                return Ok(());
            }
            return write_ink_events_action(input, scene, &self.events, "begin", None, || {
                mutate_scene_state(&scene.surface_id, |state| {
                    state.pointer_was_down = true;
                    state.drag = Some(SceneDrag { mode: SceneDragMode::InkEraser { mode: utility } });
                });
            });
        }
        if self.utility == "selectMarquee" {
            mutate_scene_state(&scene.surface_id, |state| {
                state.pointer_was_down = true;
                state.drag = Some(SceneDrag { mode: SceneDragMode::InkMarqueeDrag { start_x: x, start_y: y } });
                state.ink_marquee_points.clear();
                state.ink_marquee_points.push((x, y));
            });
            return Ok(());
        }
        if self.utility == "selectDirect" {
            let hit_id = self.hit_id.as_deref();
            let hit_origin = self.hit_origin;
            let mut next_selection = Vec::with_capacity(INK_SELECTION_ITEM_CAPACITY);
            if shift {
                for id in &self.selected_ids {
                    if next_selection.len() == INK_SELECTION_ITEM_CAPACITY {
                        return Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
                    }
                    next_selection.push(id.clone());
                }
            }
            if let Some(id) = hit_id {
                if !next_selection.iter().any(|selected| selected == id) {
                    if next_selection.len() == INK_SELECTION_ITEM_CAPACITY {
                        return Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
                    }
                    next_selection.push(id.to_owned());
                }
            }
            return write_ink_owned_selection_actions(input, scene, &next_selection, || {
                mutate_scene_state(&scene.surface_id, |state| {
                    state.pointer_was_down = true;
                    if let (Some(id), Some(origin)) = (hit_id, hit_origin) {
                        let mut origins = HashMap::with_capacity(1);
                        origins.insert(id.to_owned(), origin);
                        state.drag = Some(SceneDrag { mode: SceneDragMode::InkMove { origins, start_x: x, start_y: y } });
                    }
                });
            });
        }
        let document = self.document.as_ref().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
        if matches!(self.utility.as_str(), "pencil" | "text" | "image" | "table" | "math") {
            let (world_x, world_y) = ink_screen_to_world(self.camera, inner, x, y);
            let (world_x, world_y) = ink_maybe_snap_fields(document.snap_enabled, document.snap_grid_spacing, world_x, world_y);
            let kind = if self.utility == "pencil" { "stroke" } else { self.utility.as_str() };
            let block = create_ink_item(kind, world_x, world_y);
            let block_id = ink_item_id(&block).to_owned();
            let state_id = block_id.clone();
            let mut events = InkEventJsonPages::default();
            events.push(&json!({ "operation": "addBlock", "block": block.clone() }))?;
            events.seal()?;
            let phase = if self.utility == "pencil" { "begin" } else { "atomic" };
            return write_ink_events_action(input, scene, &events, phase, Some(&block_id), || {
                if kind == "stroke" {
                    mutate_scene_state(&scene.surface_id, |state| {
                        state.pointer_was_down = true;
                        state.ink_overrides.insert(state_id.clone(), block);
                        state.drag = Some(SceneDrag { mode: SceneDragMode::InkStroke { block_id: state_id } });
                    });
                }
            });
        }
        Ok(())
    }

    fn publish_move(&mut self, scene: &UiComponentSceneNode, _inner: Rect, x: f32, y: f32, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
        match self.drag.as_ref() {
            Some(SceneDragMode::InkPan { start_x, start_y, camera_x, camera_y, zoom }) => {
                let next = InkCameraF { x: *camera_x + (x - *start_x) as f64, y: *camera_y + (y - *start_y) as f64, zoom: *zoom };
                write_ink_camera_action(input, scene, next, || {
                    mutate_scene_state(&scene.surface_id, |state| {
                        state.ink_camera = Some((next.x, next.y, next.zoom));
                    });
                })
            }
            Some(SceneDragMode::InkMarqueeDrag { start_x, start_y }) => {
                mutate_scene_state(&scene.surface_id, |state| {
                    state.ink_marquee_points.clear();
                    state.ink_marquee_points.push((*start_x, *start_y));
                    state.ink_marquee_points.push((x, y));
                });
                Ok(())
            }
            Some(SceneDragMode::InkMove { .. } | SceneDragMode::InkResize { .. } | SceneDragMode::InkEraser { .. }) => {
                if self.events.items == 0 {
                    Ok(())
                } else {
                    write_ink_events_action(input, scene, &self.events, "live", None, || {})
                }
            }
            Some(SceneDragMode::InkStroke { .. }) => {
                if let Some((id, _)) = self.stroke_update.as_ref() {
                    let saturated = SCENE_STATE.with(|cell| cell.borrow().get(&scene.surface_id).is_some_and(|state| !state.ink_overrides.contains_key(id) && state.ink_overrides.len() == INK_INTERACTION_ITEM_CAPACITY));
                    if saturated {
                        return Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
                    }
                }
                let update = self.stroke_update.as_ref().map(|(id, raw)| Ok((id.clone(), raw.as_str()?.to_owned()))).transpose()?;
                write_ink_events_action(input, scene, &self.events, "live", None, || {
                    if let Some((id, block_json)) = update {
                        let block = serde_json::from_str(&block_json).expect("validated retained ink block");
                        mutate_scene_state(&scene.surface_id, |state| {
                            state.ink_overrides.insert(id, block);
                        });
                    }
                })
            }
            Some(_) => Err(ui_wgpu::wgpu::BoundedActionFault::Structure),
            None => {
                let hovered = scene.ink_canvas.as_ref().and_then(|ink| ink.hovered_id.as_deref());
                if hovered == self.hit_id.as_deref() {
                    Ok(())
                } else {
                    write_ink_hover_action(input, scene, self.hit_id.as_deref())
                }
            }
        }
    }

    pub(crate) fn close_step(&mut self) -> bool {
        if self.selected_ids.pop().is_some() {
            return false;
        }
        if self.result_ids.pop().is_some() {
            return false;
        }
        if self.pending_remove_id.take().is_some() {
            return false;
        }
        if self.pending_fragments.pop_front() {
            return false;
        }
        if self.stroke_update.take().is_some() {
            return false;
        }
        if let Some(drag) = self.drag.as_mut() {
            match drag {
                SceneDragMode::InkMove { origins, .. } => {
                    if let Some(id) = origins.keys().next().cloned() {
                        origins.remove(&id);
                        return false;
                    }
                }
                SceneDragMode::InkResize { handle, selected_ids, .. } => {
                    if selected_ids.pop().is_some() {
                        return false;
                    }
                    if !handle.is_empty() {
                        handle.clear();
                        return false;
                    }
                }
                SceneDragMode::InkStroke { block_id } => {
                    if !block_id.is_empty() {
                        block_id.clear();
                        return false;
                    }
                }
                SceneDragMode::InkEraser { mode } => {
                    if !mode.is_empty() {
                        mode.clear();
                        return false;
                    }
                }
                _ => {}
            }
            self.drag = None;
            return false;
        }
        let Some(document) = self.document.as_mut() else {
            return true;
        };
        if document.span_len > 0 {
            document.span_len -= 1;
            document.spans[document.span_len] = None;
            return false;
        }
        if !document.source.is_empty() {
            document.source.clear();
            return false;
        }
        if !document.schema.is_empty() {
            document.schema.clear();
            return false;
        }
        if !document.id.is_empty() {
            document.id.clear();
            return false;
        }
        if document.active_utility.take().is_some() {
            return false;
        }
        self.document = None;
        self.hit_id = None;
        true
    }
}













const INK_RESIZE_HANDLES: [&str; 8] = ["nw", "n", "ne", "e", "se", "s", "sw", "w"];





/** @emoji 📝️ Pointer-down entry point for ink-canvas: mirrors handlePointerDown in ink-canvas-host.tsx. */


/** @emoji 📝️ Pointer-up entry point for ink-canvas: commits the active gesture and finalizes marquee selection. */


/** @emoji 📝️ Pointer-move hover entry point for ink-canvas: mirrors the `!dragState` hover branch of handlePointerMove. */


/** @emoji 📝️ Wheel entry point for ink-canvas: zoom-at-cursor, mirrors handleWheel in ink-canvas-host.tsx. */

//#endregion InkCanvasState

//#region InkCanvasRender
fn ink_effective_bounds(block: &Value, overrides: &HashMap<String, Value>) -> InkBoundsF {
    match overrides.get(ink_item_id(block)) {
        Some(over) => ink_item_bounds(over),
        None => ink_item_bounds(block),
    }
}

fn flatten_ink_items(blocks: &[Value]) -> Vec<&Value> {
    let mut out = Vec::new();
    fn visit<'a>(blocks: &'a [Value], out: &mut Vec<&'a Value>) {
        for block in blocks {
            out.push(block);
            if ink_item_kind(block) == "group" {
                if let Some(children) = block.get("children").and_then(Value::as_array) {
                    visit(children, out);
                }
            }
        }
    }
    visit(blocks, &mut out);
    out
}

fn ink_selection_bounds(blocks: &[Value], overrides: &HashMap<String, Value>, ids: &[String]) -> Option<InkBoundsF> {
    let id_set: HashSet<&str> = ids.iter().map(String::as_str).collect();
    let selected: Vec<InkBoundsF> = flatten_ink_items(blocks).into_iter().filter(|block| id_set.contains(ink_item_id(block))).map(|block| ink_effective_bounds(block, overrides)).collect();
    if selected.is_empty() {
        return None;
    }
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    for bounds in &selected {
        min_x = min_x.min(bounds.x);
        min_y = min_y.min(bounds.y);
        max_x = max_x.max(bounds.x + bounds.w);
        max_y = max_y.max(bounds.y + bounds.h);
    }
    Some(InkBoundsF { x: min_x, y: min_y, w: (max_x - min_x).max(1.0), h: (max_y - min_y).max(1.0) })
}

fn ink_text_plain(block: &Value) -> String {
    block
        .get("paragraphs")
        .and_then(Value::as_array)
        .map(|paragraphs| {
            paragraphs.iter().map(|paragraph| paragraph.get("runs").and_then(Value::as_array).map(|runs| runs.iter().filter_map(|run| run.get("text").and_then(Value::as_str)).collect::<String>()).unwrap_or_default()).collect::<Vec<_>>().join("\n")
        })
        .unwrap_or_default()
}

fn ink_world_to_screen(camera: InkCameraF, inner: Rect, wx: f64, wy: f64) -> (f32, f32) {
    (inner.x + (wx * camera.zoom + camera.x) as f32, inner.y + (wy * camera.zoom + camera.y) as f32)
}

fn ink_resize_handle_screen_pos(handle: &str, sx: f32, sy: f32, w: f32, h: f32, size: f32) -> (f32, f32) {
    let half = size * 0.5;
    let x = if handle.contains('w') {
        sx - half
    } else if handle.contains('e') {
        sx + w - half
    } else {
        sx + w * 0.5 - half
    };
    let y = if handle.contains('n') {
        sy - half
    } else if handle.contains('s') {
        sy + h - half
    } else {
        sy + h * 0.5 - half
    };
    (x, y)
}

fn draw_ink_table(ctx: &mut FrameworkWidgetContext<'_>, block: &Value, sx: f32, sy: f32, w: f32, h: f32, theme: &Theme) {
    let columns: Vec<String> = block.get("columns").and_then(Value::as_array).map(|c| c.iter().filter_map(|v| v.as_str().map(str::to_string)).collect()).unwrap_or_default();
    let rows: Vec<Vec<String>> = block
        .get("rows")
        .and_then(Value::as_array)
        .map(|rows| rows.iter().map(|row| row.as_array().map(|cells| cells.iter().map(|cell| cell.get("content").and_then(Value::as_str).unwrap_or("").to_string()).collect()).unwrap_or_default()).collect())
        .unwrap_or_default();
    let col_count = columns.len().max(1);
    let row_count = rows.len() + 1;
    let col_w = w / col_count as f32;
    let row_h = h / row_count as f32;
    let font = theme.font_size_small.min(row_h * 0.6).max(6.0);
    for (index, label) in columns.iter().enumerate() {
        draw_text(ctx, label, sx + index as f32 * col_w + 3.0, sy + row_h * 0.7, font, theme.text_muted);
    }
    for (row_index, row) in rows.iter().enumerate() {
        let ry = sy + (row_index + 1) as f32 * row_h;
        for (col_index, cell) in row.iter().enumerate() {
            draw_text(ctx, cell, sx + col_index as f32 * col_w + 3.0, ry + row_h * 0.7, font, theme.text);
        }
    }
    for index in 0..=col_count {
        let x = sx + index as f32 * col_w;
        ctx.draw.push_line(x, sy, x, sy + h, theme.separator, 0.5);
    }
    for index in 0..=row_count {
        let y = sy + index as f32 * row_h;
        ctx.draw.push_line(sx, y, sx + w, y, theme.separator, 0.5);
    }
}

fn draw_ink_image(ctx: &mut FrameworkWidgetContext<'_>, scene: &UiComponentSceneNode, block: &Value, doc: &InkDocumentJson, sx: f32, sy: f32, w: f32, h: f32) {
    let theme = ctx.theme;
    let image_key = ink_item_str(block, "imageKey");
    if let Some(asset) = doc.assets.get(image_key) {
        let mime = asset.get("mime").and_then(Value::as_str).unwrap_or("image/png");
        let data = asset.get("data").and_then(Value::as_str).unwrap_or("");
        let data_url = if data.starts_with("data:") { data.to_string() } else { format!("data:{mime};base64,{data}") };
        if let Some(key) = queue_canvas_image_upload(&scene.surface_id, ink_item_id(block), &data_url) {
            ctx.draw.push_raster_quad(&key, [sx, sy, w.max(1.0), h.max(1.0)], [0.0, 0.0, 1.0, 1.0], 1.0);
            return;
        }
    }
    draw_text(ctx, image_key, sx + 6.0, sy + h * 0.5, theme.font_size_small, theme.text_muted);
}

/// 🎨️ `bg-background/90` in `ink-canvas-host.tsx` — `theme.panel` (the app-chrome surface token)
/// previously stood in for the canvas-item card token, which is `background`, not `panel`.
fn draw_ink_item(ctx: &mut FrameworkWidgetContext<'_>, scene: &UiComponentSceneNode, block: &Value, camera: InkCameraF, inner: Rect, doc: &InkDocumentJson, selected: bool, hovered: bool) {
    let theme = ctx.theme;
    let kind = ink_item_kind(block);
    let bounds = ink_item_bounds(block);
    let (sx, sy) = ink_world_to_screen(camera, inner, bounds.x, bounds.y);
    let w = (bounds.w * camera.zoom) as f32;
    let h = (bounds.h * camera.zoom) as f32;

    if kind == "stroke" {
        let points = ink_points(block);
        if points.len() >= 2 {
            let color = block
                .get("color")
                .and_then(Value::as_array)
                .map(|c| {
                    let get = |i: usize| c.get(i).and_then(Value::as_f64).unwrap_or(0.0) as f32;
                    Rgba::new(get(0), get(1), get(2), get(3))
                })
                .unwrap_or(theme.text);
            let stroke_width = (ink_item_num(block, "strokeWidth") as f32 * camera.zoom as f32).max(1.0);
            let screen_points: Vec<(f32, f32)> = points.iter().map(|p| ink_world_to_screen(camera, inner, p.0, p.1)).collect();
            for pair in screen_points.windows(2) {
                ctx.draw.push_line(pair[0].0, pair[0].1, pair[1].0, pair[1].1, color, stroke_width);
            }
        }
        return;
    }

    let bg = theme.background;
    ctx.draw.push_rounded([sx, sy, w.max(4.0), h.max(4.0)], bg.with_alpha(0.9), theme.border_radius.min(6.0));

    match kind {
        "text" => {
            let text = ink_text_plain(block);
            let font_size = (ink_item_num(block, "fontSize").max(8.0) as f32 * camera.zoom as f32).max(6.0);
            draw_text_wrapped(ctx, &text, sx + 6.0, sy + 4.0, (w - 12.0).max(1.0), font_size, theme.text);
        }
        "math" => {
            let tex = ink_item_str(block, "tex");
            draw_text(ctx, tex, sx + 8.0, sy + h * 0.5 + 4.0, theme.font_size_body.max(8.0), theme.text);
        }
        "table" => draw_ink_table(ctx, block, sx, sy, w.max(4.0), h.max(4.0), theme),
        "image" => draw_ink_image(ctx, scene, block, doc, sx, sy, w.max(4.0), h.max(4.0)),
        "group" => {
            let children_len = block.get("children").and_then(Value::as_array).map(Vec::len).unwrap_or(0);
            draw_text(ctx, &format!("Group · {children_len} children"), sx + 6.0, sy + 16.0, theme.font_size_small, theme.text_muted);
        }
        _ => {}
    }

    let border = if selected {
        theme.accent
    } else if hovered {
        theme.accent.with_alpha(theme.accent.a * 0.6)
    } else {
        theme.panel_border
    };
    let border_w = if selected { 2.0 } else { 1.0 };
    draw_ink_rect_outline(ctx.draw, sx, sy, w.max(4.0), h.max(4.0), border, border_w);
}

fn ink_item_visible(block: &Value) -> bool {
    block.get("visible").and_then(Value::as_bool).unwrap_or(true)
}

fn positive_mod_f32(v: f32, m: f32) -> f32 {
    if m <= 0.0 {
        0.0
    } else {
        ((v % m) + m) % m
    }
}

fn draw_ink_rect_outline(draw: &mut ui_wgpu::wgpu::DrawList, x: f32, y: f32, w: f32, h: f32, color: Rgba, width: f32) {
    draw.push_line(x, y, x + w, y, color, width);
    draw.push_line(x + w, y, x + w, y + h, color, width);
    draw.push_line(x + w, y + h, x, y + h, color, width);
    draw.push_line(x, y + h, x, y, color, width);
}

fn draw_ink_grid(draw: &mut ui_wgpu::wgpu::DrawList, camera: InkCameraF, inner: Rect, theme: &Theme, spacing: f64, subdivisions: u32, opacity: f64) {
    let major_px = (spacing * camera.zoom) as f32;
    if major_px < 2.0 {
        return;
    }
    let minor_px = major_px / subdivisions.max(1) as f32;
    let offset_x = positive_mod_f32(camera.x as f32, major_px);
    let offset_y = positive_mod_f32(camera.y as f32, major_px);
    let color = theme.separator.with_alpha((theme.separator.a * opacity as f32).max(0.05));
    let minor_color = color.with_alpha(color.a * 0.55);

    let mut wx = inner.x + positive_mod_f32(offset_x, major_px) - major_px;
    while wx < inner.x + inner.w {
        if subdivisions > 1 {
            for s in 1..subdivisions {
                let mx = wx + s as f32 * minor_px;
                if mx >= inner.x && mx <= inner.x + inner.w {
                    draw.push_line(mx, inner.y, mx, inner.y + inner.h, minor_color, 0.5);
                }
            }
        }
        if wx >= inner.x && wx <= inner.x + inner.w {
            draw.push_line(wx, inner.y, wx, inner.y + inner.h, color, 1.0);
        }
        wx += major_px;
    }
    let mut wy = inner.y + positive_mod_f32(offset_y, major_px) - major_px;
    while wy < inner.y + inner.h {
        if subdivisions > 1 {
            for s in 1..subdivisions {
                let my = wy + s as f32 * minor_px;
                if my >= inner.y && my <= inner.y + inner.h {
                    draw.push_line(inner.x, my, inner.x + inner.w, my, minor_color, 0.5);
                }
            }
        }
        if wy >= inner.y && wy <= inner.y + inner.h {
            draw.push_line(inner.x, wy, inner.x + inner.w, wy, color, 1.0);
        }
        wy += major_px;
    }
}

fn draw_ink_selection_chrome(draw: &mut ui_wgpu::wgpu::DrawList, theme: &Theme, camera: InkCameraF, inner: Rect, bounds: InkBoundsF, show_handles: bool) {
    let (sx, sy) = ink_world_to_screen(camera, inner, bounds.x, bounds.y);
    let w = (bounds.w * camera.zoom) as f32;
    let h = (bounds.h * camera.zoom) as f32;
    draw_ink_rect_outline(draw, sx, sy, w, h, theme.accent, 1.5);
    if !show_handles {
        return;
    }
    let handle_size = 8.0;
    for handle in INK_RESIZE_HANDLES {
        let (hx, hy) = ink_resize_handle_screen_pos(handle, sx, sy, w, h, handle_size);
        draw.push_rounded([hx, hy, handle_size, handle_size], theme.background, 1.0);
        draw_ink_rect_outline(draw, hx, hy, handle_size, handle_size, theme.accent, 1.0);
    }
}

fn render_ink_canvas(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let theme = ctx.theme;
    let Some(ink) = &scene.ink_canvas else {
        return render_placeholder("ink-canvas", bounds, ctx);
    };
    let doc: InkDocumentJson = serde_json::from_str(&ink.document_json).unwrap_or_default();
    let selected_ids: Vec<String> = serde_json::from_str(&ink.selection_json).unwrap_or_default();
    let selected_set: HashSet<&str> = selected_ids.iter().map(String::as_str).collect();
    let hovered_id = ink.hovered_id.clone();
    let is_navigator = ink.view_mode == "navigator";
    let inner = bounds;

    let state = scene_state(&scene.surface_id);
    let camera = state.ink_camera.map(|(x, y, zoom)| InkCameraF { x, y, zoom }).unwrap_or_else(|| InkCameraF::from(doc.camera.clone()));

    ctx.draw.push_solid([inner.x, inner.y, inner.w, inner.h], theme.canvas_clear);
    ctx.draw.push_scissor(inner);

    if doc.grid_visible.unwrap_or(true) && !is_navigator {
        draw_ink_grid(ctx.draw, camera, inner, theme, doc.grid_spacing.unwrap_or(32.0), doc.grid_subdivisions.unwrap_or(4.0).max(1.0) as u32, doc.grid_opacity.unwrap_or(0.35));
    }

    let overrides = state.ink_overrides.clone();
    let blocks = flatten_ink_items(&doc.blocks);
    for block in blocks.iter().copied() {
        let effective = overrides.get(ink_item_id(block)).unwrap_or(block);
        if !ink_item_visible(effective) {
            continue;
        }
        let id = ink_item_id(block);
        let selected = selected_set.contains(id);
        let hovered = hovered_id.as_deref() == Some(id);
        draw_ink_item(ctx, scene, effective, camera, inner, &doc, selected, hovered);
    }

    let selection_bounds = ink_selection_bounds(&doc.blocks, &overrides, &selected_ids);
    let utility = doc.active_utility.clone().unwrap_or_else(|| "selectDirect".into());
    let show_handles = !is_navigator && (utility == "selectDirect" || utility == "selectMarquee") && selection_bounds.is_some() && !selected_ids.is_empty();
    if let Some(sel) = selection_bounds {
        draw_ink_selection_chrome(ctx.draw, theme, camera, inner, sel, show_handles);
    }

    if state.ink_marquee_points.len() >= 2 {
        let points: Vec<[f32; 2]> = state.ink_marquee_points.iter().map(|p| [p.0, p.1]).collect();
        ui_wgpu::wgpu::paint_selection_marquee(ctx.draw, theme, false, false, &points, false);
    }

    ctx.draw.pop_scissor();

    ctx.input.register_hit(HitTarget { rect: inner, event: None, control_id: Some(scene.surface_id.clone()), kind: HitKind::Generic, drag_axis: None, drag_data: None });
}












//#endregion InkCanvasRender

//#region RasterFrameCostTests
#[cfg(test)]
#[path = "../../🧪️tests/🔬️wgpu-raster-frame-cost/🦀️.rs"]
mod raster_frame_cost_tests;
//#endregion RasterFrameCostTests

//#region InkCanvasTests
#[cfg(test)]
#[path = "../../🧪️tests/🔬️wgpu-ink-canvas/🦀️.rs"]
mod ink_canvas_tests;
//#endregion InkCanvasTests
//#endregion InkCanvas

//#region NodeGraph
#[derive(Clone, Debug)]
pub struct NodeGraphSurface {
    pub bounds: Rect,
    pub controller_id: String,
    /// 🪟️ The window instance that owns this surface — the ONLY thing that retires it. See
    /// [`NodeGraphSurface`]'s oracle, `🧑‍🎨engine/🧫️fixtures/🧲️engine-surface-retention/🔣️.json`.
    pub window_id: String,
}

//#endregion NodeGraph

//#region TiledMap
#[derive(Clone, Debug)]
pub struct TiledMapSurface {
    pub bounds: Rect,
    pub controller_id: String,
    pub selection_method: String,
    pub window_id: String,
}

fn query_map_feature_hits(host: &framework_surface_tiled_map::tiled_map::MapHost, method: &str, points: &[(f32, f32)], crossing: bool) -> (Vec<String>, Vec<String>) {
    if method == "lasso" && points.len() >= 3 {
        let payload: Vec<[f64; 2]> = points.iter().map(|(x, y)| [*x as f64, *y as f64]).collect();
        let points_json = serde_json::to_string(&payload).unwrap_or_else(|_| "[]".into());
        engine_canvas::parse_map_feature_hit(&host.features_in_polygon_json(&points_json, crossing))
    } else if points.len() >= 2 {
        let (x0, y0) = points[0];
        let (x1, y1) = points[points.len() - 1];
        let (min_x, max_x) = if x0 <= x1 { (x0, x1) } else { (x1, x0) };
        let (min_y, max_y) = if y0 <= y1 { (y0, y1) } else { (y1, y0) };
        engine_canvas::parse_map_feature_hit(&host.features_in_rect_json(min_x as f64, min_y as f64, max_x as f64, max_y as f64, crossing))
    } else {
        (Vec::new(), Vec::new())
    }
}


/** @emoji 🗺️ The GIS map context-menu surface target for a screen-space right-click: the feature
 * under the point as a `ContextMenuHit`, plus the live feature selection split into its
 * `"position"`/`"route"` groups. Port of `onContextMenu` in `🧭️TiledMapHost/🟦️.tsx` — the domains
 * are exactly the ones `gis2d_context_menu_items` matches on (`feature`/`position`/`route`), and the
 * groups are what decides whether its `clearSelection` row renders enabled. */
pub fn tiled_map_context_menu_target(surface_id: &str, inner: Rect, x: f32, y: f32) -> (Vec<ui_wgpu::wgpu::ContextMenuHit>, Vec<ui_wgpu::wgpu::ContextMenuSelectionGroup>) {
    #[derive(Deserialize)]
    struct HitRow {
        kind: String,
        id: String,
    }
    let (sx, sy) = engine_canvas::map_local_pointer(inner, x, y);
    let hit_json = engine_canvas::with_map_host(surface_id, |host| host.hit_test_feature_json(sx, sy)).unwrap_or_else(|| "null".into());
    let hits = serde_json::from_str::<Option<HitRow>>(&hit_json)
        .ok()
        .flatten()
        .map(|row| ui_wgpu::wgpu::ContextMenuHit { domain: if row.kind == "route" { "route".into() } else { "position".into() }, id: row.id, label: None })
        .into_iter()
        .collect();
    let (positions, routes) = engine_canvas::with_map_host(surface_id, |host| (host.selected_position_ids().map(str::to_owned).collect::<Vec<_>>(), host.selected_route_ids().map(str::to_owned).collect::<Vec<_>>())).unwrap_or_default();
    let mut selection = Vec::new();
    if !positions.is_empty() {
        selection.push(ui_wgpu::wgpu::ContextMenuSelectionGroup { domain: "position".into(), ids: positions });
    }
    if !routes.is_empty() {
        selection.push(ui_wgpu::wgpu::ContextMenuSelectionGroup { domain: "route".into(), ids: routes });
    }
    (hits, selection)
}


/// 🕹️ The framework-owned `"features"` interaction domain a GIS map addresses, and its feature
/// granularity — the constants React's `🧭️TiledMapHost` spells `MAP_INTERACTION_DOMAIN`/
/// `MAP_FEATURE_GRANULARITY`/`MAP_HOVER_CHANNEL`, and the domain the GIS app declares
/// (`🌍️gis/…/✏️editor`'s `InteractionDefinition { id: "features", … }`).
const MAP_INTERACTION_DOMAIN: &str = "features";
const MAP_FEATURE_GRANULARITY: &str = "feature";
const MAP_HOVER_CHANNEL: &str = "pointer";

/// 🎯️ `interactionSelect`'s `targets` are JSON TEXT, not a node array — the wire shape
/// `mapFeatureSelectionActionArgs`/`select_feature_action_args` both build.
fn map_feature_targets_json(ids: impl Iterator<Item = impl AsRef<str>>) -> Result<String, ui_wgpu::wgpu::BoundedActionFault> {
    let rows: Vec<Value> = ids.map(|id| serde_json::json!({ "granularity": MAP_FEATURE_GRANULARITY, "id": id.as_ref() })).collect();
    serde_json::to_string(&rows).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::Structure)
}

/// 🕹️ A map pick/marquee release: `interactionSelect` against the `"features"` domain, or
/// `clearSelection` on a miss. Mirrors `emitFeatureSelection` in `🧭️TiledMapHost/🟦️.tsx` — including
/// its hit/miss split, because `next_selection` treats an empty `targets` as a no-op, not a clear.
/// The bespoke `setFeatureSelection` this used to publish was DELETED from the GIS app (ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM): every such dispatch was refused.
fn write_tiled_map_selection(
    input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>,
    controller_id: &str,
    surface_id: &str,
    positions: &[String],
    routes: &[String],
    merge: &str,
    method: &str,
    commit: impl FnOnce(),
) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    if positions.is_empty() && routes.is_empty() {
        let action = ui_wgpu::wgpu::tiled_map_actions::CLEAR_SELECTION;
        let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller_id, action, "surfaceId", surface_id])?;
        let mut reservation = input.reserve_action(controller_id, action, bytes)?;
        let builder = reservation.builder();
        builder.begin_object(None)?;
        builder.string(Some("surfaceId"), surface_id)?;
        builder.end_container()?;
        return reservation.publish_with(commit);
    }
    let targets = map_feature_targets_json(positions.iter().chain(routes))?;
    if targets.len() > ui_wgpu::wgpu::action::ACTION_STRING_BYTE_CAPACITY {
        return Err(ui_wgpu::wgpu::BoundedActionFault::StringCredits);
    }
    let action = "interactionSelect";
    let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller_id, action, "surfaceId", surface_id, "domainId", MAP_INTERACTION_DOMAIN, "targets", targets.as_str(), "merge", merge, "method", method])?;
    let mut reservation = input.reserve_action(controller_id, action, bytes)?;
    let builder = reservation.builder();
    builder.begin_object(None)?;
    builder.string(Some("surfaceId"), surface_id)?;
    builder.string(Some("domainId"), MAP_INTERACTION_DOMAIN)?;
    builder.string(Some("targets"), targets.as_str())?;
    builder.string(Some("merge"), merge)?;
    builder.string(Some("method"), method)?;
    builder.end_container()?;
    reservation.publish_with(commit)
}

/// 🐁️ `interactionHover` on the `"pointer"` channel — `mapFeatureHoverActionArgs`'s twin; a cleared
/// hover publishes an EMPTY `targets` array, which is what clears the guest's hover.
fn write_tiled_map_hover(input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>, controller_id: &str, surface_id: &str, hover: Option<(&str, &str)>, commit: impl FnOnce()) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    let targets = map_feature_targets_json(hover.map(|(_, id)| id).into_iter())?;
    if targets.len() > ui_wgpu::wgpu::action::ACTION_STRING_BYTE_CAPACITY {
        return Err(ui_wgpu::wgpu::BoundedActionFault::StringCredits);
    }
    let action = "interactionHover";
    let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller_id, action, "surfaceId", surface_id, "domainId", MAP_INTERACTION_DOMAIN, "channel", MAP_HOVER_CHANNEL, "targets", targets.as_str()])?;
    let mut reservation = input.reserve_action(controller_id, action, bytes)?;
    let builder = reservation.builder();
    builder.begin_object(None)?;
    builder.string(Some("surfaceId"), surface_id)?;
    builder.string(Some("domainId"), MAP_INTERACTION_DOMAIN)?;
    builder.string(Some("channel"), MAP_HOVER_CHANNEL)?;
    builder.string(Some("targets"), targets.as_str())?;
    builder.end_container()?;
    reservation.publish_with(commit)
}

pub fn tiled_map_pointer_down_into(
    surface_id: &str,
    controller_id: &str,
    inner: Rect,
    x: f32,
    y: f32,
    button: i16,
    shift: bool,
    ctrl_or_meta: bool,
    selection_method: &str,
    input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>,
) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    let (sx, sy) = engine_canvas::map_local_pointer(inner, x, y);
    if button == 0 {
        if selection_method.len() > SCENE_SURFACE_ID_BYTE_CAPACITY {
            return Err(ui_wgpu::wgpu::BoundedActionFault::StringCredits);
        }
        mutate_scene_state(surface_id, |state| {
            state.drag = Some(SceneDrag { mode: SceneDragMode::MapMarquee { start_x: sx as f32, start_y: sy as f32, method: selection_method.to_owned(), merge_mode: engine_canvas::map_marquee_mode(shift, ctrl_or_meta).to_owned() } });
            state.map_marquee_points = vec![(sx as f32, sy as f32)];
            state.map_marquee_active = false;
        });
        return Ok(true);
    }
    if button == 1 {
        let published = engine_canvas::with_map_interaction_into(surface_id, controller_id, input, framework_surface_tiled_map::tiled_map::MapInteractionIntent::PointerDown { sx, sy, button: 1 })?;
        if published {
            mutate_scene_state(surface_id, |state| state.drag = Some(SceneDrag { mode: SceneDragMode::MapPan }));
        }
        return Ok(published);
    }
    Ok(false)
}

pub fn tiled_map_pointer_move_into(surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32, down: bool, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    let (sx, sy) = engine_canvas::map_local_pointer(inner, x, y);
    if down {
        let state = scene_state(surface_id);
        if let Some(drag) = &state.drag {
            match &drag.mode {
                SceneDragMode::MapPan => return engine_canvas::with_map_interaction_into(surface_id, controller_id, input, framework_surface_tiled_map::tiled_map::MapInteractionIntent::PointerMove { sx, sy }),
                SceneDragMode::MapMarquee { start_x, start_y, method, .. } => {
                    let distance = ((sx as f32 - *start_x).powi(2) + (sy as f32 - *start_y).powi(2)).sqrt();
                    mutate_scene_state(surface_id, |state| {
                        if distance >= MAP_MARQUEE_THRESHOLD_PX {
                            state.map_marquee_active = true;
                        }
                        if state.map_marquee_active {
                            if method == "lasso" {
                                if state.map_marquee_points.last().copied() != Some((sx as f32, sy as f32)) {
                                    state.map_marquee_points.push((sx as f32, sy as f32));
                                }
                            } else {
                                state.map_marquee_points = vec![(*start_x, *start_y), (sx as f32, sy as f32)];
                            }
                        }
                    });
                }
                _ => {}
            }
        }
        return Ok(true);
    }
    #[derive(Deserialize)]
    struct HoverRow {
        kind: String,
        id: String,
    }
    let hit_json = engine_canvas::with_map_host(surface_id, |host| host.hit_test_feature_json(sx, sy)).unwrap_or_else(|| "null".into());
    let hover = serde_json::from_str::<Option<HoverRow>>(&hit_json).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::Structure)?;
    let hover_json = hover.as_ref().map(|row| format!("{}:{}", row.kind, row.id)).unwrap_or_else(|| "null".into());
    if scene_state(surface_id).map_last_hover_json.as_deref() == Some(hover_json.as_str()) {
        return Ok(false);
    }
    write_tiled_map_hover(input, controller_id, surface_id, hover.as_ref().map(|row| (row.kind.as_str(), row.id.as_str())), || {
        mutate_scene_state(surface_id, |state| state.map_last_hover_json = Some(hover_json));
    })?;
    Ok(true)
}





/// 🎯️ `method` is `"pick"` for a click and the marquee's own method for a drag — the exact
/// split `emitFeatureSelection`'s two call sites make in `🧭️TiledMapHost/🟦️.tsx`.
pub fn tiled_map_pointer_up_into(surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    let (sx, sy) = engine_canvas::map_local_pointer(inner, x, y);
    let state = scene_state(surface_id);
    let Some(drag) = state.drag.as_ref() else {
        return Ok(false);
    };
    match &drag.mode {
        SceneDragMode::MapPan => {
            let published = engine_canvas::with_map_interaction_into(surface_id, controller_id, input, framework_surface_tiled_map::tiled_map::MapInteractionIntent::PointerUp { sx, sy })?;
            if published {
                mutate_scene_state(surface_id, |state| {
                    state.drag = None;
                    state.map_marquee_points.clear();
                    state.map_marquee_active = false;
                });
            }
            Ok(published)
        }
        SceneDragMode::MapMarquee { start_x, start_y, method, merge_mode } => {
            let distance = ((sx as f32 - *start_x).powi(2) + (sy as f32 - *start_y).powi(2)).sqrt();
            let mut positions = Vec::new();
            let mut routes = Vec::new();
            if state.map_marquee_active && distance >= MAP_MARQUEE_THRESHOLD_PX {
                let mut points = state.map_marquee_points.clone();
                if method == "lasso" {
                    points.push((sx as f32, sy as f32));
                } else {
                    points = vec![(*start_x, *start_y), (sx as f32, sy as f32)];
                }
                let crossing = engine_canvas::map_marquee_crossing(method, *start_x, sx as f32);
                (positions, routes) = engine_canvas::with_map_host(surface_id, |host| query_map_feature_hits(host, method, &points, crossing)).unwrap_or_default();
            } else if distance < MAP_MARQUEE_THRESHOLD_PX {
                #[derive(Deserialize)]
                struct HitRow {
                    kind: String,
                    id: String,
                }
                let hit_json = engine_canvas::with_map_host(surface_id, |host| host.hit_test_feature_json(sx, sy)).unwrap_or_else(|| "null".into());
                if let Some(hit) = serde_json::from_str::<Option<HitRow>>(&hit_json).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::Structure)? {
                    match hit.kind.as_str() {
                        "position" => positions.push(hit.id),
                        "route" => routes.push(hit.id),
                        _ => {}
                    }
                }
            }
            if positions.is_empty() && routes.is_empty() && distance >= MAP_MARQUEE_THRESHOLD_PX && !state.map_marquee_active {
                mutate_scene_state(surface_id, |state| {
                    state.drag = None;
                    state.map_marquee_points.clear();
                    state.map_marquee_active = false;
                });
                return Ok(false);
            }
            let select_method = if state.map_marquee_active && distance >= MAP_MARQUEE_THRESHOLD_PX { method.as_str() } else { "pick" };
            write_tiled_map_selection(input, controller_id, surface_id, &positions, &routes, merge_mode, select_method, || {
                mutate_scene_state(surface_id, |state| {
                    state.drag = None;
                    state.map_marquee_points.clear();
                    state.map_marquee_active = false;
                });
            })?;
            Ok(true)
        }
        _ => Ok(false),
    }
}



pub fn tiled_map_drag_active(surface_id: &str) -> bool {
    scene_state(surface_id).drag.as_ref().is_some_and(|drag| matches!(drag.mode, SceneDragMode::MapMarquee { .. } | SceneDragMode::MapPan))
}

//#endregion TiledMap

//#region IconRender
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct IconRenderCameraFields {
    position: [f64; 3],
    target: [f64; 3],
    #[serde(default = "icon_render_default_zoom")]
    zoom: f64,
    #[serde(default)]
    fov: Option<f64>,
    #[serde(default)]
    up: Option<[f64; 3]>,
}

#[derive(Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
struct IconRenderLightsFields {
    #[serde(default)]
    ambient_intensity: f64,
    #[serde(default)]
    ambient_color: Option<String>,
    #[serde(default)]
    sun_azimuth: f64,
    #[serde(default)]
    sun_elevation: f64,
    #[serde(default)]
    sun_intensity: f64,
    #[serde(default)]
    sun_color: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct IconRenderMaterialFields {
    #[serde(default)]
    color: Option<String>,
    #[serde(default)]
    metalness: Option<f64>,
    #[serde(default)]
    roughness: Option<f64>,
    #[serde(default)]
    emissive: Option<String>,
    #[serde(default)]
    emissive_intensity: Option<f64>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct IconRenderRequestFields {
    asset_url: String,
    camera: IconRenderCameraFields,
    #[serde(default)]
    lights: Option<IconRenderLightsFields>,
    width: f64,
    height: f64,
    #[serde(default)]
    shape: Option<String>,
    #[serde(default)]
    background: Option<String>,
    #[serde(default)]
    shadow_enabled: Option<bool>,
    #[serde(default)]
    material: Option<IconRenderMaterialFields>,
}

fn icon_render_default_zoom() -> f64 {
    1.0
}

fn icon_render_camera_json(camera: &IconRenderCameraFields) -> String {
    let fov = camera.fov.unwrap_or(50.0).max(1.0);
    let zoom = if camera.zoom.abs() > 1e-6 { camera.zoom } else { 1.0 };
    let effective_fov = if (zoom - 1.0).abs() > 1e-6 {
        let half = (fov * 0.5).to_radians();
        (2.0 * (half.tan() / zoom).atan()).to_degrees()
    } else {
        fov
    };
    let up = camera.up.unwrap_or([0.0, 0.0, 1.0]);
    json!({
        "position": camera.position,
        "target": camera.target,
        "up": up,
        "fov": effective_fov,
    })
    .to_string()
}

fn icon_render_environment_json(request: &IconRenderRequestFields) -> String {
    let lights = request.lights.clone().unwrap_or_default();
    let mut value = json!({
        "ambient": { "intensity": lights.ambient_intensity, "color": lights.ambient_color },
        "sun": {
            "azimuth": lights.sun_azimuth,
            "elevation": lights.sun_elevation,
            "intensity": lights.sun_intensity,
            "color": lights.sun_color,
        },
        "shadow": { "enabled": request.shadow_enabled.unwrap_or(false) },
    });
    if let Some(object) = value.as_object_mut() {
        if let Some(material) = &request.material {
            object.insert(
                "material".into(),
                json!({
                    "color": material.color,
                    "metalness": material.metalness,
                    "roughness": material.roughness,
                    "emissive": material.emissive,
                    "emissiveIntensity": material.emissive_intensity,
                }),
            );
        }
        if let Some(background) = &request.background {
            object.insert("background".into(), json!(background));
        }
    }
    value.to_string()
}

fn render_icon_render_empty(bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>, message: &str) {
    let theme = ctx.theme;
    let size = theme.font_size_body;
    let width = ctx.atlas.measure_text(message, size).0;
    draw_text(ctx, message, bounds.x + (bounds.w - width) * 0.5, bounds.y + bounds.h * 0.5, size, theme.text_muted);
}

/// 🖼️ React's shot is an OFFSCREEN render through `iconRenderPort`, so its host has three visible
/// states: the error text, the finished `<img>`, and `ui.host.rendering` while the promise is in
/// flight (`🖼️IconRenderHost/🟦️.tsx:55-61`). This twin draws the GLB straight into the frame, so it
/// used to have exactly one — a silently EMPTY shot frame for the whole time the mesh was being
/// fetched, and forever if the fetch never landed. The residency of the one subject mesh is the
/// same predicate: no lease yet is "rendering", a recorded snapshot fault is the error arm.
/** @emoji 🖼️ Native counterpart of framework/renderer/react/components/icon-render-host.tsx: reframes the request into a synthetic World3dScene and delegates the actual GLB draw to infinite_world::world::render_world_3d, then paints the aspect-fit frame/badge/footer chrome on top. */
fn render_icon_render(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>, hosts: &mut SceneEngineHosts<'_>) {
    let Some(icon_render) = &scene.icon_render else {
        return render_icon_render_empty(bounds, ctx, "No shot");
    };
    let Ok(request) = serde_json::from_str::<IconRenderRequestFields>(&icon_render.request_json) else {
        return render_icon_render_empty(bounds, ctx, "No shot");
    };

    let shape = request.shape.clone().unwrap_or_else(|| "rectangle".into());
    let width = request.width.max(1.0) as f32;
    let height = request.height.max(1.0) as f32;
    let fit_scale = (bounds.w / width).min(bounds.h / height).max(0.01);
    let frame_w = width * fit_scale;
    let frame_h = height * fit_scale;
    let frame = Rect::new(bounds.x + (bounds.w - frame_w) * 0.5, bounds.y + (bounds.h - frame_h) * 0.5, frame_w, frame_h);

    let asset_url = crate::mesh_assets::mesh_asset_transport_url(&request.asset_url);
    let mesh_id = semio_framework_plugin::world3d_mesh_id_from_url(&asset_url);
    let instances_json = json!([{
        "id": "icon-render-subject",
        "meshId": mesh_id,
        "position": [0.0, 0.0, 0.0],
        "rotation": [0.0, 0.0, 0.0, 1.0],
        "scale": [1.0, 1.0, 1.0],
    }])
    .to_string();
    let mut synthetic_world = semio_framework_plugin::world3d_scene(
        icon_render_camera_json(&request.camera),
        semio_framework_plugin::world3d_meshes_json_from_urls(std::slice::from_ref(&asset_url)),
        instances_json,
        semio_framework_plugin::default_world3d_selection(),
        &semio_framework_plugin::WorldSunConfig::default(),
    );
    synthetic_world.environment_json = Some(icon_render_environment_json(&request));

    let synthetic_scene = UiComponentSceneNode {
        presence: UiPresence::default(),
        surface_id: scene.surface_id.clone(),
        controller_id: scene.controller_id.clone(),
        component_kind: SurfaceKind::World3d,
        pane_id: None,
        binding_id: None,
        canvas_2d: None,
        world_3d: Some(synthetic_world),
        node_graph: None,
        text_editor: None,
        table: None,
        paint_2d: None,
        virtual_file_system: None,
        tiled_map: None,
        board2d: None,
        icon_render: None,
        ink_canvas: None,
        graph_timeline: None,
        diff_view: None,
        event_feed: None,
        block_list: None,
        menu: None,
    };

    let surface_id = scene.surface_id.clone();
    let controller_id = scene.controller_id.clone();
    let Some(state) = hosts.world3d_states.get_or_insert_with(surface_id.clone(), || infinite_world::world::World3dState::new(surface_id, controller_id)) else {
        return render_icon_render_empty(bounds, ctx, "No shot");
    };
    infinite_world::world::render_world_3d(&synthetic_scene, frame, ctx, state, hosts.world_resources);
    let status = icon_render_status(state.mesh_lease(&mesh_id).is_some(), state.snapshot_fault().is_some());
    if status != IconRenderStatus::Ready {
        let failed = status == IconRenderStatus::Failed;
        let color = if failed { ctx.theme.error } else { ctx.theme.text_muted };
        let message = if failed { ICON_RENDER_FAILED_MESSAGE } else { ICON_RENDER_RENDERING_MESSAGE };
        render_icon_render_status(ctx, frame, message, color);
    }
    paint_icon_render_chrome(ctx, bounds, frame, &request, &shape, icon_render.footer.as_deref());
}

/// 🖼️ React's `renderingLabel` (`ui.host.rendering`). Hard-coded English like this region's existing
/// `"No shot"`: the wgpu scene painters carry no `useLabel` equivalent yet, a divergence that belongs
/// to whoever wires `LocalizedLabel` into scene chrome, not to this arm.
const ICON_RENDER_RENDERING_MESSAGE: &str = "Rendering…";
/// 🖼️ React shows `iconRenderPort.render`'s own rejection message; this twin has no per-asset error
/// string to show, only the state's snapshot fault, so it names the failure instead of inventing one.
const ICON_RENDER_FAILED_MESSAGE: &str = "Shot failed";

/// 🖼️ The three states React's `IconRenderHost` shows, resolved from the ONE subject mesh's residency.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum IconRenderStatus {
    Ready,
    Rendering,
    Failed,
}

/// 🖼️ `Ready` once the subject's GLB lease is published (the frame now draws real geometry),
/// `Failed` when the state recorded a snapshot/capacity fault while it was still missing, else
/// `Rendering` — the asset lane is still fetching or decoding it. Takes the two predicates rather
/// than the state so the three-way mapping is testable without a live `World3dState`.
fn icon_render_status(mesh_resident: bool, faulted: bool) -> IconRenderStatus {
    if mesh_resident {
        IconRenderStatus::Ready
    } else if faulted {
        IconRenderStatus::Failed
    } else {
        IconRenderStatus::Rendering
    }
}

/// 🖼️ Centres one status line inside the shot frame, over the (empty) world draw — React puts the
/// same text inside `IconShotFrame`, not beside it.
fn render_icon_render_status(ctx: &mut FrameworkWidgetContext<'_>, frame: Rect, message: &str, color: Rgba) {
    let size = ctx.theme.font_size_body;
    let (width, height) = ctx.atlas.measure_text(message, size);
    draw_text(ctx, message, frame.x + (frame.w - width) * 0.5, frame.y + (frame.h + height) * 0.5, size, color);
}

/// 🖼️ 2px, matching `IconShotFrame`'s `border-2 border-accent` in `icon-render-host.tsx` —
/// `theme.stroke_hairline` (1px) previously halved the frame width relative to React.
/// 🏷️ `bg-background/80`, not `panel` — the badge chip sits on the transparent canvas frame in
/// `icon-render-host.tsx`, not on a panel surface.
fn paint_icon_render_chrome(ctx: &mut FrameworkWidgetContext<'_>, bounds: Rect, frame: Rect, request: &IconRenderRequestFields, shape: &str, footer: Option<&str>) {
    let theme = ctx.theme;
    let hair = 2.0_f32;
    ctx.draw.push_solid([frame.x, frame.y, frame.w, hair], theme.accent);
    ctx.draw.push_solid([frame.x, frame.y + frame.h - hair, frame.w, hair], theme.accent);
    ctx.draw.push_solid([frame.x, frame.y, hair, frame.h], theme.accent);
    ctx.draw.push_solid([frame.x + frame.w - hair, frame.y, hair, frame.h], theme.accent);

    let badge = format!("{}×{} · {}", request.width.round() as i64, request.height.round() as i64, shape);
    let badge_size = theme.font_size_small;
    let (badge_text_w, badge_text_h) = ctx.atlas.measure_text(&badge, badge_size);
    let pad = 4.0;
    let badge_w = badge_text_w + pad * 2.0;
    let badge_h = badge_text_h + pad * 2.0;
    let badge_x = frame.x + frame.w - badge_w - 4.0;
    let badge_y = frame.y + frame.h - badge_h - 4.0;
    ctx.draw.push_rounded([badge_x, badge_y, badge_w, badge_h], theme.background.with_alpha(0.8), 2.0);
    draw_text(ctx, &badge, badge_x + pad, badge_y + pad + badge_text_h * 0.8, badge_size, theme.text_muted);

    if let Some(footer) = footer {
        let footer_size = theme.font_size_small;
        let footer_w = ctx.atlas.measure_text(footer, footer_size).0;
        draw_text(ctx, footer, bounds.x + (bounds.w - footer_w) * 0.5, bounds.y + bounds.h - 8.0, footer_size, theme.text_muted);
    }
}


/// 🖼️ The aspect-fit frame border, size/shape badge, and optional footer caption painted on top of
/// the delegated `render_world_3d` GLB draw — split out from `render_icon_render` (which needs a
/// live `GpuContext` and so can't run in a headless unit test) so this chrome-only paint can be
/// exercised directly against a `DrawList`.

//#endregion IconRender

//#region IconRenderTests
#[cfg(test)]
#[path = "../../🧪️tests/🔬️wgpu-icon-render/🦀️.rs"]
mod icon_render_tests;
//#endregion IconRenderTests

//#region Board2d
pub struct Board2dSurface {
    pub bounds: Rect,
    pub controller_id: String,
    pub fixture_json: String,
    pub window_id: String,
}


pub fn puzzle_board_pointer_down(surface_id: &str, inner: Rect, x: f32, y: f32, button: i16, shift: bool, ctrl_or_meta: bool) {
    engine_canvas::puzzle_board_pointer_down(surface_id, inner, x, y, button, shift, ctrl_or_meta);
}

pub fn puzzle_board_pointer_move_into(
    surface_id: &str,
    controller_id: &str,
    inner: Rect,
    x: f32,
    y: f32,
    shift: bool,
    ctrl_or_meta: bool,
    alt: bool,
    input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>,
) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    engine_canvas::puzzle_board_pointer_move_into(surface_id, controller_id, inner, x, y, shift, ctrl_or_meta, alt, input)
}

pub fn puzzle_board_pointer_up_into(
    surface_id: &str,
    controller_id: &str,
    inner: Rect,
    x: f32,
    y: f32,
    shift: bool,
    ctrl_or_meta: bool,
    alt: bool,
    input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>,
) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    engine_canvas::puzzle_board_pointer_up_into(surface_id, controller_id, inner, x, y, shift, ctrl_or_meta, alt, input)
}

pub fn puzzle_board_pointer_leave_into(surface_id: &str, controller_id: &str, alt: bool, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    engine_canvas::puzzle_board_pointer_leave_into(surface_id, controller_id, alt, input)
}







pub fn board2d_drag_active(surface_id: &str) -> bool {
    engine_canvas::board_drag_active(surface_id)
}

pub fn puzzle_board_wheel_into(surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32, delta: f32, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    engine_canvas::puzzle_board_wheel_into(surface_id, controller_id, inner, x, y, delta, input)
}



//#region Puzzle2dSelectionMenu
pub struct Puzzle2dSelectionMenuItem {
    pub id: String,
    pub label: String,
    pub icon: String,
    pub action: String,
    pub args: Option<Value>,
    pub disabled: bool,
    pub destructive: bool,
}

fn puzzle2d_entity_flag(entity: &Value, key: &str) -> bool {
    entity.get(key).and_then(Value::as_bool).unwrap_or(false)
}

/// @emoji 🖱️ Right-click menu for the current selection: Hide/Show, Lock/Unlock, Duplicate, Select same kind, Zoom to selection, Delete — mirrors `buildPuzzle2dSelectionMenuItems` in the React host.
pub fn build_puzzle2d_selection_menu_items(fixture_json: &str, selection_ids: &[String]) -> Vec<Puzzle2dSelectionMenuItem> {
    let fixture: Value = serde_json::from_str(fixture_json).unwrap_or(Value::Null);
    if selection_ids.is_empty() {
        return vec![Puzzle2dSelectionMenuItem { id: "selectAll".into(), label: "Select all".into(), icon: "maximize-2".into(), action: "selectAll".into(), args: None, disabled: false, destructive: false }];
    }
    let selected: HashSet<&str> = selection_ids.iter().map(String::as_str).collect();
    let nodes = fixture.get("nodes").and_then(Value::as_array).cloned().unwrap_or_default();
    let edges = fixture.get("edges").and_then(Value::as_array).cloned().unwrap_or_default();
    let mut selected_entities: Vec<Value> = Vec::new();
    let mut has_selected_node = false;
    for node in &nodes {
        if let Some(id) = node.get("id").and_then(Value::as_str) {
            if selected.contains(id) {
                selected_entities.push(node.clone());
                has_selected_node = true;
            }
        }
        if let Some(handles) = node.get("handles").and_then(Value::as_array) {
            for handle in handles {
                if let Some(id) = handle.get("id").and_then(Value::as_str) {
                    if selected.contains(id) {
                        selected_entities.push(handle.clone());
                    }
                }
            }
        }
    }
    for edge in &edges {
        if let Some(id) = edge.get("id").and_then(Value::as_str) {
            if selected.contains(id) {
                selected_entities.push(edge.clone());
            }
        }
    }
    let any_visible = selected_entities.iter().any(|entity| !puzzle2d_entity_flag(entity, "hidden"));
    let any_unlocked = selected_entities.iter().any(|entity| !puzzle2d_entity_flag(entity, "locked"));
    vec![
        Puzzle2dSelectionMenuItem {
            id: "toggleHidden".into(),
            label: (if any_visible { "Hide" } else { "Show" }).into(),
            icon: (if any_visible { "eye-off" } else { "eye" }).into(),
            action: "setSelectionFlag".into(),
            args: Some(json!({ "flag": "hidden", "value": any_visible })),
            disabled: false,
            destructive: false,
        },
        Puzzle2dSelectionMenuItem {
            id: "toggleLocked".into(),
            label: (if any_unlocked { "Lock" } else { "Unlock" }).into(),
            icon: (if any_unlocked { "lock" } else { "lock-open" }).into(),
            action: "setSelectionFlag".into(),
            args: Some(json!({ "flag": "locked", "value": any_unlocked })),
            disabled: false,
            destructive: false,
        },
        Puzzle2dSelectionMenuItem { id: "duplicate".into(), label: "Duplicate".into(), icon: "copy".into(), action: "duplicateSelection".into(), args: None, disabled: !has_selected_node, destructive: false },
        Puzzle2dSelectionMenuItem { id: "selectSameKind".into(), label: "Select all of same kind".into(), icon: "layers".into(), action: "selectSameKind".into(), args: None, disabled: false, destructive: false },
        Puzzle2dSelectionMenuItem { id: "focusSelection".into(), label: "Zoom to selection".into(), icon: "crosshair".into(), action: "focusSelection".into(), args: None, disabled: false, destructive: false },
        Puzzle2dSelectionMenuItem { id: "deleteSelection".into(), label: "Delete".into(), icon: "trash".into(), action: "deleteSelection".into(), args: None, disabled: false, destructive: true },
    ]
}
//#endregion Puzzle2dSelectionMenu

/// @emoji 🧩️ Pushes board-2d context-menu items for a screen-space hit, eagerly selecting the clicked target if it isn't already selected (mirrors the React host's `onContextMenu`).

//#endregion Board2d

//#region VirtualFileSystem
#[derive(Deserialize)]
struct VfsDescriptorKind {
    #[serde(default)]
    presentation: String,
}

#[derive(Deserialize)]
struct VfsFileNodeKind {
    #[serde(default)]
    icon: Option<String>,
    #[serde(default)]
    descriptors: Vec<VfsDescriptorColumn>,
}

#[derive(Deserialize)]
struct VfsDescriptorColumn {
    id: String,
    #[serde(default)]
    label: String,
    #[serde(rename = "descriptorKindId", default)]
    descriptor_kind_id: String,
}

#[derive(Deserialize)]
struct VfsSchema {
    #[serde(rename = "descriptorColumnIds", default)]
    descriptor_column_ids: Vec<String>,
    #[serde(rename = "descriptorKinds", default)]
    descriptor_kinds: HashMap<String, VfsDescriptorKind>,
    #[serde(rename = "fileNodeKinds", default)]
    file_node_kinds: HashMap<String, VfsFileNodeKind>,
}




fn vfs_glyph_icon<'a>(schema: &'a VfsSchema, row: &Value) -> &'a str {
    let kind_id = row.get("fileNodeKindId").and_then(|v| v.as_str()).unwrap_or("file");
    if let Some(icon) = schema.file_node_kinds.get(kind_id).and_then(|k| k.icon.as_deref()) {
        return icon;
    }
    match kind_id {
        "root" | "studio" | "folder" => "folder",
        "instance" => "box",
        _ => "file-text",
    }
}

fn vfs_double_click_action(scene: &UiComponentSceneNode, row: &Value) -> Option<ActionDescriptor> {
    let uri = row.get("navigateUri").and_then(|v| v.as_str())?;
    if uri.starts_with("os://instance/") {
        return Some(scene_action(
            scene,
            "openInstance",
            json!({
                "surfaceId": scene.surface_id,
                "instanceId": uri.trim_start_matches("os://instance/"),
            }),
        ));
    }
    if uri.starts_with("os://export/") {
        let parts: Vec<&str> = uri.split('/').collect();
        if parts.len() >= 5 {
            return Some(scene_action(
                scene,
                "exportMedia",
                json!({
                    "surfaceId": scene.surface_id,
                    "instanceId": parts[2],
                    "format": parts[4],
                }),
            ));
        }
    }
    if uri.starts_with("/spaces/") {
        let space_id = uri.split('/').nth(2)?;
        return Some(scene_action(scene, "navigateVirtualFileSystemNode", json!({ "surfaceId": scene.surface_id, "spaceId": space_id })));
    }
    if let Some(space_id) = uri.strip_prefix("studio:") {
        return Some(scene_action(scene, "navigateVirtualFileSystemNode", json!({ "surfaceId": scene.surface_id, "spaceId": space_id })));
    }
    None
}
/// 🗂️ One row of the flattened, expansion-aware row list the virtual file system paints.
#[derive(Clone)]
struct VfsVisibleRow {
    row: Value,
    level: u32,
    has_children: bool,
    expanded: bool,
}

fn vfs_children_by_parent(rows: &[Value]) -> HashMap<String, Vec<Value>> {
    let mut map: HashMap<String, Vec<Value>> = HashMap::new();
    for row in rows {
        let parent = row.get("parentId").and_then(|v| v.as_str()).unwrap_or("").to_string();
        map.entry(parent).or_default().push(row.clone());
    }
    map
}

/// 🌲️ Flattens the parent/child row payload into the ordered rows a frame actually shows, honouring
/// this surface's own expansion state — the one ordering the paint, the hit test and the
/// shift-range selection all index into.
/// 🌱️ The one file-node kind React never paints a row for: `buildVirtualFileSystemVisibleRows`
/// synthesises its root as `{ fileNodeKindId: "root" }` and starts the DFS at that root's CHILDREN
/// (`⚙️VirtualFileSystem/🟦️.tsx:417-424`), so the root is elided and everything under it is level 0.
///
/// 🐛️ ticket 26/09/17/WGPU-RENDERER-REACT-PARITY wave 2–6 integration: this port elided EVERY
/// top-level row carrying `hasChildren`, so an ordinary expandable folder at level 0 lost its own row
/// and its chevron — React keeps it (`🧪️owned-locale-detector-retirement/🟦️.tsx:9348`, where `f1`
/// carries `hasChildren: true` and stays a visible row).
const VFS_ROOT_FILE_NODE_KIND: &str = "root";

fn build_vfs_visible_rows(rows: &[Value], expanded_ids: &HashSet<String>) -> Vec<VfsVisibleRow> {
    let children_by_parent = vfs_children_by_parent(rows);
    let mut visible = Vec::new();
    fn visit(node: &Value, level: u32, out: &mut Vec<VfsVisibleRow>, children_by_parent: &HashMap<String, Vec<Value>>, expanded_ids: &HashSet<String>) {
        let id = node.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let has_children = node.get("hasChildren").and_then(|v| v.as_bool()).unwrap_or_else(|| children_by_parent.get(&id).is_some_and(|c| !c.is_empty()));
        let expanded = has_children && expanded_ids.contains(&id);
        out.push(VfsVisibleRow { row: node.clone(), level, has_children, expanded });
        if !expanded {
            return;
        }
        if let Some(children) = children_by_parent.get(&id) {
            for child in children {
                visit(child, level + 1, out, children_by_parent, expanded_ids);
            }
        }
    }
    let roots: Vec<Value> = rows.iter().filter(|row| row.get("parentId").map(|v| v.is_null() || v.as_str() == Some("")).unwrap_or(true)).cloned().collect();
    for root in roots {
        if root.get("fileNodeKindId").and_then(Value::as_str) == Some(VFS_ROOT_FILE_NODE_KIND) {
            let root_id = root.get("id").and_then(Value::as_str).unwrap_or("");
            if let Some(children) = children_by_parent.get(root_id) {
                for child in children {
                    visit(child, 0, &mut visible, &children_by_parent, expanded_ids);
                }
            }
        } else {
            visit(&root, 0, &mut visible, &children_by_parent, expanded_ids);
        }
    }
    visible
}

fn vfs_descriptor_label(schema: &VfsSchema, column_id: &str) -> String {
    for kind in schema.file_node_kinds.values() {
        if let Some(col) = kind.descriptors.iter().find(|c| c.id == column_id) {
            if !col.label.is_empty() {
                return col.label.clone();
            }
        }
    }
    column_id.to_string()
}

fn vfs_descriptor_value(schema: &VfsSchema, row: &Value, column_id: &str) -> String {
    let raw = row
        .get("descriptorValues")
        .and_then(|values| values.get(column_id))
        .map(|v| match v {
            Value::String(s) => s.clone(),
            other => other.to_string(),
        })
        .unwrap_or_default();
    let kind_id = schema.file_node_kinds.values().flat_map(|kind| kind.descriptors.iter()).find(|col| col.id == column_id).map(|col| col.descriptor_kind_id.as_str()).unwrap_or("text");
    let presentation = schema.descriptor_kinds.get(kind_id).map(|k| k.presentation.as_str()).unwrap_or("text");
    if presentation == "time" {
        if let Ok(ms) = raw.parse::<f64>() {
            let secs = (ms / 1000.0) as i64;
            let mins = secs / 60;
            let hours = mins / 60;
            return format!("{:02}:{:02}:{:02}", hours, mins % 60, secs % 60);
        }
    }
    raw
}

/// 📐️ A virtual file system's fixed metrics — the ONE derivation the paint and the pointer path share.
struct VfsMetrics {
    header_h: f32,
    row_h: f32,
    pad: f32,
    body: Rect,
}

fn vfs_metrics(inner: Rect, theme: &Theme) -> VfsMetrics {
    let header_h = theme.control_height * 1.33;
    VfsMetrics { header_h, row_h: theme.control_height, pad: theme.padding_standard, body: Rect::new(inner.x, inner.y + header_h, inner.w, inner.h - header_h) }
}

fn vfs_row_id(row: &Value) -> String {
    row.get("id").and_then(Value::as_str).unwrap_or_default().to_string()
}

/// 🎯️ Resolves a pointer point inside a `SurfaceKind::VirtualFileSystem`: a row's expand chevron, or
/// the row itself. A row press sends `selectRows` with `{ surfaceId, ids }`, the payload
/// `VirtualFileSystemHost`'s `onSelectionChange` sends.
fn vfs_hit(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32, theme: &Theme, activate: bool) -> Option<SceneListHit> {
    let vfs = scene.virtual_file_system.as_ref()?;
    let metrics = vfs_metrics(inner, theme);
    if y < metrics.body.y {
        return None;
    }
    let rows: Vec<Value> = serde_json::from_str(&vfs.rows_json).unwrap_or_default();
    let expanded = scene_state(&scene.surface_id).vfs_expanded_ids;
    let visible = build_vfs_visible_rows(&rows, &expanded);
    let scroll = scroll_offset(&scene.surface_id, "vfs");
    let index = usize::try_from(((y - metrics.body.y + scroll) / metrics.row_h.max(1.0)).floor() as i64).ok()?;
    let entry = visible.get(index)?;
    let row_id = vfs_row_id(&entry.row);
    if entry.has_children {
        let chevron_x = metrics.body.x + metrics.pad + entry.level as f32 * 14.0;
        if x >= chevron_x && x < chevron_x + 14.0 {
            return Some(SceneListHit { control_id: format!("{}.vfs.chevron.{}", scene.surface_id, row_id), action: None, toggle_expanded: Some(row_id) });
        }
    }
    let control_id = format!("{}.vfs.{}", scene.surface_id, row_id);
    if !activate {
        return Some(SceneListHit::row(control_id, None));
    }
    let ordered: Vec<String> = visible.iter().map(|entry| vfs_row_id(&entry.row)).collect();
    let ids = vfs_selection_for_click(&scene.surface_id, &row_id, &ordered, false, false);
    Some(SceneListHit::row(control_id, Some(scene_action(scene, "selectRows", json!({ "surfaceId": scene.surface_id, "ids": ids })))))
}

/// 🖱️ The double-click band a press landed in, keyed by the surface's OWN row ordering so a repeat
/// press resolves the same row the first one did.
fn hit_double_click_target(scene: &UiComponentSceneNode, inner: Rect, _x: f32, y: f32) -> Option<String> {
    if scene.component_kind != SurfaceKind::VirtualFileSystem {
        return None;
    }
    let metrics = vfs_metrics(inner, &scene_input_theme());
    let scroll = scroll_offset(&scene.surface_id, "vfs");
    let index = usize::try_from(((y - metrics.body.y + scroll) / metrics.row_h.max(1.0)).floor() as i64).ok()?;
    Some(format!("{}.vfs.index.{index}", scene.surface_id))
}

/// 🚪️ What a double-click on a virtual file system row opens — `navigateUri`'s own scheme decides
/// between `openInstance`, `exportMedia` and `navigateVirtualFileSystemNode`.
fn double_click_action(scene: &UiComponentSceneNode, _target: &str, inner: Rect, _x: f32, y: f32) -> Option<ActionDescriptor> {
    let vfs = scene.virtual_file_system.as_ref()?;
    let metrics = vfs_metrics(inner, &scene_input_theme());
    let rows: Vec<Value> = serde_json::from_str(&vfs.rows_json).ok()?;
    let expanded = scene_state(&scene.surface_id).vfs_expanded_ids;
    let visible = build_vfs_visible_rows(&rows, &expanded);
    let scroll = scroll_offset(&scene.surface_id, "vfs");
    let index = usize::try_from(((y - metrics.body.y + scroll) / metrics.row_h.max(1.0)).floor() as i64).ok()?;
    visible.get(index).and_then(|entry| vfs_double_click_action(scene, &entry.row))
}

/// 🗂️ Renders `SurfaceKind::VirtualFileSystem`: a header band of descriptor columns over an
/// expansion-aware, multi-selectable row tree — the port of `VirtualFileSystemHost` in
/// `🗣️Interpreter/🟦️.tsx` plus `@semio-tech/ui-react`'s own `VirtualFileSystem`.
fn render_vfs(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let theme = ctx.theme;
    let Some(vfs) = &scene.virtual_file_system else {
        return render_placeholder("virtual-file-system", bounds, ctx);
    };
    let schema: VfsSchema = serde_json::from_str(&vfs.schema_json).unwrap_or(VfsSchema { descriptor_column_ids: vec![], descriptor_kinds: HashMap::new(), file_node_kinds: HashMap::new() });
    let rows: Vec<Value> = serde_json::from_str(&vfs.rows_json).unwrap_or_default();
    let root_expand_ids: Vec<String> = rows.iter().filter(|row| row.get("hasChildren").and_then(|v| v.as_bool()).unwrap_or(false)).filter_map(|row| row.get("id").and_then(|v| v.as_str()).map(str::to_string)).collect();
    seed_vfs_expanded(&scene.surface_id, &root_expand_ids);
    let selected: HashSet<String> = vfs.selected_row_ids_json.as_deref().and_then(|json| serde_json::from_str::<Vec<String>>(json).ok()).unwrap_or_default().into_iter().collect();
    let expanded_ids = scene_state(&scene.surface_id).vfs_expanded_ids;
    let visible_rows = build_vfs_visible_rows(&rows, &expanded_ids);
    let inner = bounds;
    let metrics = vfs_metrics(inner, theme);
    let (header_h, row_h, pad) = (metrics.header_h, metrics.row_h, metrics.pad);
    let name_col_w = inner.w * 0.32;
    let descriptor_ids: Vec<String> = schema.descriptor_column_ids.clone();
    let descriptor_col_w = if descriptor_ids.is_empty() { 0.0 } else { (inner.w - name_col_w) / descriptor_ids.len() as f32 };
    ctx.draw.push_solid([inner.x, inner.y, inner.w, header_h], theme.panel);
    draw_text(ctx, "Name", inner.x + pad, inner.y + header_h * 0.65, theme.font_size_small, theme.text_muted);
    for (index, column_id) in descriptor_ids.iter().enumerate() {
        let x = inner.x + name_col_w + index as f32 * descriptor_col_w;
        draw_text(ctx, &vfs_descriptor_label(&schema, column_id), x + pad, inner.y + header_h * 0.65, theme.font_size_small, theme.text_muted);
    }
    let body = metrics.body;
    let scroll = scroll_offset(&scene.surface_id, "vfs");
    ctx.input.register_hit(HitTarget { rect: body, event: None, control_id: Some(scroll_key(&scene.surface_id, "vfs")), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None });
    ctx.draw.push_scissor(body);
    reserve_list_rows(ctx, list_visible_row_capacity(body, row_h));
    let hovered_row = scene_hovered_control_id(&scene.surface_id).or_else(|| vfs.hovered_row_id.clone()).or_else(|| ctx.input.hovered_id.clone());
    if visible_rows.is_empty() {
        let message = vfs.empty_message.as_deref().unwrap_or("No file system nodes");
        draw_text(ctx, message, body.x + pad, body.y + row_h * 0.65, theme.font_size_small, theme.text_muted);
    }
    for (row_index, entry) in visible_rows.iter().enumerate() {
        let row = &entry.row;
        let y = body.y + row_index as f32 * row_h - scroll;
        if y + row_h < body.y || y > body.y + body.h {
            continue;
        }
        let row_id = vfs_row_id(row);
        let control_id = format!("{}.vfs.{}", scene.surface_id, row_id);
        let row_rect = Rect::new(body.x, y, body.w, row_h);
        let selected_row = selected.contains(&row_id);
        let hovered = hovered_row.as_deref() == Some(control_id.as_str());
        if selected_row {
            ctx.draw.push_solid([row_rect.x, row_rect.y, row_rect.w, row_rect.h], theme.selected);
        } else if hovered {
            ctx.draw.push_solid([row_rect.x, row_rect.y, row_rect.w, row_rect.h], theme.row_hover);
        }
        ctx.draw.push_line(row_rect.x, row_rect.y + row_rect.h - theme.stroke_hairline, row_rect.x + row_rect.w, row_rect.y + row_rect.h - theme.stroke_hairline, theme.separator, 1.0);
        let mut name_x = body.x + pad + entry.level as f32 * 14.0;
        if entry.has_children {
            let chevron = if entry.expanded { "chevron-down" } else { "chevron-right" };
            if let Some(icons) = ctx.icons {
                if let Some(uv) = icons.icon_uv(chevron) {
                    ctx.draw.push_textured([name_x, y + (row_h - 14.0) * 0.5, 14.0, 14.0], uv, theme.text_element);
                }
            }
            ctx.input.register_hit(HitTarget { rect: Rect::new(name_x, y, 14.0, row_h), event: None, control_id: Some(format!("{}.vfs.chevron.{}", scene.surface_id, row_id)), kind: HitKind::Generic, drag_axis: None, drag_data: None });
            name_x += 14.0;
        }
        let icon_id = vfs_glyph_icon(&schema, row);
        if let Some(icons) = ctx.icons {
            if let Some(uv) = icons.icon_uv(icon_id) {
                ctx.draw.push_textured([name_x, y + (row_h - 14.0) * 0.5, 14.0, 14.0], uv, theme.text_element);
            }
        }
        name_x += 18.0;
        let name = row.get("name").and_then(|v| v.as_str()).unwrap_or("—");
        draw_text(ctx, name, name_x, y + row_h * 0.65, theme.font_size_small, if selected_row || hovered { theme.active_foreground } else { theme.text });
        for (col_index, column_id) in descriptor_ids.iter().enumerate() {
            let x = body.x + name_col_w + col_index as f32 * descriptor_col_w;
            let value = vfs_descriptor_value(&schema, row, column_id);
            draw_text(ctx, &value, x + pad, y + row_h * 0.65, theme.font_size_small, if selected_row { theme.active_foreground } else { theme.text_muted });
        }
        let drag_data = vfs.drag_drop_enabled.unwrap_or(false).then(|| HashMap::from([("application/x-semio-vfs-node".to_string(), serde_json::to_string(row).unwrap_or_default())]));
        ctx.input.register_hit(HitTarget { rect: row_rect, event: None, control_id: Some(control_id), kind: HitKind::Generic, drag_axis: None, drag_data });
    }
    ctx.draw.pop_scissor();
}

//#endregion VirtualFileSystem

//#region VirtualFileSystemTests
#[cfg(test)]
#[path = "../../🧪️tests/🔬️wgpu-virtual-file-system/🦀️.rs"]
mod virtual_file_system_tests;
//#endregion VirtualFileSystemTests

//#region TextEditor
//#region State
/// 🗂️ Per-surface interaction state for double-click-to-select-word / completions / context-menu / rename.
///
/// 🔀️ Mid-session reconciliation note (concurrent `w2-scene-wiring` session): this region originally also
/// drove plain single-click-to-caret and drag-to-select itself, reading `ctx.input` pointer state directly
/// during render (mirroring the pre-existing focus-on-click code below), because at the time nothing
/// called `SceneInput::handle_scene_pointer_button`/`handle_scene_pointer_move` for any surface kind.
/// `w2-scene-wiring` landed `apply_scene_pointer` in `RenderEntry` next, calling those for every
/// non-bespoke surface kind including `✏️TextEditor` from a once-per-render-frame `InputState` sample —
/// and `w4-scene-input` (`.🧬semio/🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY/report-w4-scene-input.md`) has
/// since replaced THAT with a real per-event route (`ui_wgpu::wgpu::UiCommand::Scene` ->
/// `interpreter::apply_scene_ui_command`, calling the same two handlers), deleting `apply_scene_pointer`
/// itself. Plain click/drag still reaches `EditorHost` via that generic path today, just per real event
/// now rather than sampled once per frame. That single-click/drag code was removed here to avoid
/// double-dispatching `textSelect`/`textEdit`; what remains below (double-click word-select, right-click
/// context menu, completions, rename) is *not* covered by the generic path and
/// stays. 🐛️➡️✅️ W4 fix (`.🧬semio/🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY/report-w4-scene-input.md`):
/// `EditorHost::pointer_down_screen` used to no-operate entirely for `button != 0`, so both the generic
/// path's raw-button-passthrough call AND this region's own right-click handling had to force `button`
/// to `0` to reposition the caret at all. `pointer_down_screen` now repositions the caret for every
/// button (only a primary press also starts a drag-selection), so this region's `pointer_down`/
/// `pointer_up` pair below passes the real button through instead of forcing it.
/** @emoji ✍️ One text-editor surface's POPUP state — the completions dropdown, the multi-span rename
 * draft and the double-click edge, i.e. exactly the `useState` bucket React's `TextEditor` keeps
 * beside its wasm session (`completionsOpen`/`completionIndex`/`renameDraft`/`renamePosition`,
 * `🧱️elements/✏️TextEditor/🟦️.tsx:267-330`). The BUFFER, selection, carets and tokens are the
 * `EditorHost`'s own state and are never mirrored here.
 *
 * Per surface rather than per window: two editor panes of one window each own their popups, the same
 * way two mounted `TextEditor` hosts each own their own React state. */
#[derive(Clone, Debug, Default)]
struct TextEditorUiState {
    completions_open: bool,
    completion_index: usize,
    rename: Option<TextEditorRenameDraft>,
    /// 🖱️ A context-menu row this surface answers itself, parked until the next paint — the shell
    /// resolves and clears its menu without an `InputState` in hand, and every local row commits
    /// through the bounded action queue, which only the paint walk carries.
    pending_menu_action: Option<(String, f32, f32)>,
}

/// ✏️ One live rename gesture: the spans the commit rewrites and the name typed so far — React's
/// `renameDraft` (`{ occurrences, text }`).
#[derive(Clone, Debug)]
struct TextEditorRenameDraft {
    occurrences: Vec<(usize, usize)>,
    text: String,
}

thread_local! {
    static TEXT_EDITOR_UI: RefCell<HashMap<String, TextEditorUiState>> = RefCell::new(HashMap::new());
}

fn text_editor_ui(surface_id: &str) -> TextEditorUiState {
    TEXT_EDITOR_UI.with(|cell| cell.borrow().get(surface_id).cloned().unwrap_or_default())
}

fn mutate_text_editor_ui<R>(surface_id: &str, apply: impl FnOnce(&mut TextEditorUiState) -> R) -> R {
    TEXT_EDITOR_UI.with(|cell| apply(cell.borrow_mut().entry(surface_id.to_string()).or_default()))
}

/// 📋️ Mirrors `CompletionItem` (`🧱️elements/✏️TextEditor/🟦️.tsx`); `insertText` has no producer yet
/// anywhere in the codebase (`jack_completions_json` only ever emits `label`/`detail`) so
/// `insert_text` falls back to `label`, exactly like `identifierPrefixStart`/`applyCompletion` do on
/// the React side.
#[derive(Clone, Deserialize)]
struct TextEditorCompletionItem {
    label: String,
    #[serde(default)]
    detail: Option<String>,
    #[serde(default, rename = "insertText")]
    insert_text: Option<String>,
}

#[derive(Clone, Copy, Deserialize)]
struct TextEditorSpan {
    start: usize,
    end: usize,
}

/// ✏️ Mirrors `RenameInfo` (`🧱️elements/✏️TextEditor/🟦️.tsx`), parsed from `TextEditorScene::rename_json`.
#[derive(Clone, Deserialize)]
struct TextEditorRenameInfo {
    name: String,
    occurrences: Vec<TextEditorSpan>,
}

fn text_editor_completions(editor: &ui_wgpu::wgpu::TextEditorScene) -> Vec<TextEditorCompletionItem> {
    editor.completions_json.as_deref().and_then(|json| serde_json::from_str(json).ok()).unwrap_or_default()
}

fn text_editor_rename_info(editor: &ui_wgpu::wgpu::TextEditorScene) -> Option<TextEditorRenameInfo> {
    editor.rename_json.as_deref().and_then(|json| serde_json::from_str(json).ok())
}

/// ✂️ Identifier-prefix scan back from `caret` — `identifierPrefixStart`
/// (`🧱️elements/✏️TextEditor/🟦️.tsx:98`) — the replacement range a completion commit overwrites.
fn identifier_prefix_start(text: &str, caret: usize) -> usize {
    let bytes = text.as_bytes();
    let mut start = caret.min(bytes.len());
    while start > 0 && (bytes[start - 1].is_ascii_alphanumeric() || bytes[start - 1] == b'_') {
        start -= 1;
    }
    start
}

/// 📏️ `[start, end)` byte range of the line containing `offset` — `lineRangeAt`
/// (`🧱️elements/✏️TextEditor/🟦️.tsx:91`), the "Select Line" menu row's own range.
fn text_editor_line_range(text: &str, offset: usize) -> (usize, usize) {
    let clamped = offset.min(text.len());
    let start = text[..clamped].rfind('\n').map_or(0, |index| index + 1);
    let end = text[clamped..].find('\n').map_or(text.len(), |index| clamped + index);
    (start, end)
}

/// ✏️ `multiSpanReplace` (`🧱️elements/✏️TextEditor/🟦️.tsx:80`): rewrites every occurrence back-to-front
/// so earlier spans keep their offsets, and answers the post-rewrite spans in document order.
fn multi_span_replace(text: &str, occurrences: &[(usize, usize)], next_name: &str) -> (String, Vec<(usize, usize)>) {
    let mut sorted: Vec<(usize, usize)> = occurrences.to_vec();
    sorted.sort_by(|left, right| right.0.cmp(&left.0));
    let mut out = text.to_string();
    let mut next = Vec::with_capacity(sorted.len());
    for (start, end) in sorted {
        if start > out.len() || end > out.len() || start > end {
            continue;
        }
        out.replace_range(start..end, next_name);
        next.insert(0, (start, start + next_name.len()));
    }
    (out, next)
}

//#region 🍿️Popups
/// 📍️ Anchor (page space) for the completions dropdown: near the caret, falling back to a fixed
/// offset if the host isn't ready yet — mirrors React's `renamePosition ? … : { left: 12, top: 12 }`.
fn text_editor_popup_anchor(scene: &UiComponentSceneNode, inner: Rect) -> (f32, f32) {
    engine_canvas::text_editor_caret_screen(scene, inner).unwrap_or((inner.x + 12.0, inner.y + 12.0))
}

fn text_editor_completion_row_rect(anchor: (f32, f32), theme: &Theme, index: usize) -> Rect {
    Rect::new(anchor.0, anchor.1 + 18.0 + index as f32 * theme.control_height_small, TEXT_EDITOR_COMPLETION_WIDTH, theme.control_height_small)
}

/// 📋️ The completion row under a point, for a click-to-commit press on the dropdown.
fn text_editor_completion_hit(scene: &UiComponentSceneNode, inner: Rect, theme: &Theme, count: usize, x: f32, y: f32) -> Option<usize> {
    let anchor = text_editor_popup_anchor(scene, inner);
    (0..count).find(|&index| text_editor_completion_row_rect(anchor, theme, index).contains(x, y))
}

const TEXT_EDITOR_COMPLETION_WIDTH: f32 = 220.0;
const TEXT_EDITOR_RENAME_WIDTH: f32 = 180.0;
/// 🧾️ The retained-item grant one editor overlay paint needs: the two popup frames plus a bounded
/// completion list (the dropdown is a dropdown, not a document — React never scrolls it either).
const TEXT_EDITOR_OVERLAY_ITEMS: usize = 256;

/** @emoji 🍿️ Paints the editor's popup chrome ABOVE the composited `EditorHost` texture: the
 * completions dropdown and the live rename input. Drawn in the scene's own last cursor phase, so it
 * lands after `push_raster_quad` staged the editor texture and therefore over it — the wgpu
 * equivalent of React's absolutely-positioned `z-50` overlays inside the host element
 * (`🧱️elements/✏️TextEditor/🟦️.tsx:503-608`).
 *
 * The CONTEXT MENU is deliberately not painted here: a right-click on this surface resolves through
 * `scene_context_menu_target` and opens the SHELL's own context-menu chrome
 * (`ShellState::open_context_menu`), which is what carries React's title row, submenu nesting,
 * keyboard navigation and dismissal rules. The seven rows React handles locally rather than
 * dispatching to the guest are executed by [`text_editor_local_menu_action`] from that menu. */
fn render_text_editor_overlays(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let Some(editor) = scene.text_editor.as_ref() else {
        return;
    };
    if let Some((action, x, y)) = mutate_text_editor_ui(&scene.surface_id, |state| state.pending_menu_action.take()) {
        if let Err(fault) = text_editor_local_menu_action(scene, bounds, &action, x, y, ctx.input) {
            ctx.input.record_action_fault(fault);
        }
    }
    let ui = text_editor_ui(&scene.surface_id);
    if let Some(draft) = ui.rename.as_ref() {
        render_text_editor_rename_input(scene, bounds, ctx, &draft.text);
    }
    if !ui.completions_open {
        return;
    }
    let completions = text_editor_completions(editor);
    render_text_editor_completions(scene, bounds, ctx, &completions, ui.completion_index);
}

/// 📋️ The completions dropdown — React's `rounded border border-border bg-popover p-1 shadow-md`
/// list, active row in `bg-accent text-accent-foreground`.
fn render_text_editor_completions(scene: &UiComponentSceneNode, inner: Rect, ctx: &mut FrameworkWidgetContext<'_>, completions: &[TextEditorCompletionItem], active_index: usize) {
    if completions.is_empty() {
        return;
    }
    let theme = ctx.theme;
    let anchor = text_editor_popup_anchor(scene, inner);
    let pad = 4.0;
    let row_h = theme.control_height_small;
    let container = Rect::new(anchor.0 - pad, anchor.1 + 18.0 - pad, TEXT_EDITOR_COMPLETION_WIDTH + pad * 2.0, completions.len() as f32 * row_h + pad * 2.0);
    ctx.draw.push_rounded([container.x, container.y, container.w, container.h], theme.panel, theme.border_radius * 0.5);
    draw_ink_rect_outline(ctx.draw, container.x, container.y, container.w, container.h, theme.panel_border, 1.0);
    for (index, item) in completions.iter().enumerate() {
        let row = text_editor_completion_row_rect(anchor, theme, index);
        let (bg, fg) = if index == active_index { (theme.accent, theme.active_foreground) } else { (theme.panel, theme.text) };
        ctx.draw.push_rounded([row.x, row.y, row.w, row.h], bg, theme.border_radius * 0.5);
        draw_text(ctx, &item.label, row.x + 8.0, row.y + row.h * 0.68, theme.font_size_small, fg);
        if let Some(detail) = &item.detail {
            let label_w = item.label.len() as f32 * theme.font_size_small * 0.6;
            let detail_fg = if index == active_index { fg } else { theme.text_muted };
            draw_text(ctx, detail, row.x + 12.0 + label_w, row.y + row.h * 0.68, theme.font_size_small, detail_fg);
        }
        ctx.input.register_hit(HitTarget { rect: row, event: None, control_id: Some(format!("{}.editor.completion.{index}", scene.surface_id)), kind: HitKind::Generic, drag_axis: None, drag_data: None });
    }
}

/// ✏️ The rename input — React's `border border-border` glass field pinned near the caret, carrying
/// the draft name as the multi-span preview is typed.
fn render_text_editor_rename_input(scene: &UiComponentSceneNode, inner: Rect, ctx: &mut FrameworkWidgetContext<'_>, text: &str) {
    let theme = ctx.theme;
    let (x, y) = text_editor_popup_anchor(scene, inner);
    let rect = Rect::new(x, (y - theme.control_height_small * 0.5).max(inner.y), TEXT_EDITOR_RENAME_WIDTH, theme.control_height_small);
    ctx.draw.push_rounded([rect.x, rect.y, rect.w, rect.h], theme.panel, theme.border_radius * 0.5);
    draw_ink_rect_outline(ctx.draw, rect.x, rect.y, rect.w, rect.h, theme.panel_border, 1.0);
    draw_text(ctx, text, rect.x + 8.0, rect.y + rect.h * 0.68, theme.font_size_small, theme.text);
    ctx.input.register_hit(HitTarget { rect, event: None, control_id: Some(format!("{}.editor.rename", scene.surface_id)), kind: HitKind::Input, drag_axis: None, drag_data: None });
}
//#endregion 🍿️Popups

//#region ✍️PopupDrive
/** @emoji 📋️ Opens the completions dropdown at the first row — React's `openCompletions`, which is a
 * no-operation when the scene carries no completions. `true` when the popup actually opened, so a
 * caller can report the key as consumed the way `event.preventDefault()` does. */
pub fn text_editor_open_completions(scene: &UiComponentSceneNode) -> bool {
    let Some(editor) = scene.text_editor.as_ref() else {
        return false;
    };
    if text_editor_completions(editor).is_empty() {
        return false;
    }
    mutate_text_editor_ui(&scene.surface_id, |state| {
        state.completions_open = true;
        state.completion_index = 0;
    });
    true
}

pub fn text_editor_completions_open(surface_id: &str) -> bool {
    text_editor_ui(surface_id).completions_open
}

pub fn text_editor_close_completions(surface_id: &str) {
    mutate_text_editor_ui(surface_id, |state| {
        state.completions_open = false;
        state.completion_index = 0;
    });
}

/// 📋️ Wraps the dropdown highlight — React's `(index ± 1 + length) % length`.
pub fn text_editor_move_completion(scene: &UiComponentSceneNode, down: bool) -> bool {
    let Some(editor) = scene.text_editor.as_ref() else {
        return false;
    };
    let count = text_editor_completions(editor).len();
    if count == 0 {
        return false;
    }
    mutate_text_editor_ui(&scene.surface_id, |state| {
        state.completion_index = if down { (state.completion_index + 1) % count } else { (state.completion_index + count - 1) % count };
    });
    true
}

/** @emoji ✅️ Commits the highlighted completion — `applyCompletion`: replace
 * `[identifier_prefix_start(text, caret), caret)` with the item's insert text, publish the buffer as
 * `textEdit`, and close the dropdown. */
pub fn text_editor_apply_completion(scene: &UiComponentSceneNode, index: Option<usize>, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    let Some(editor) = scene.text_editor.as_ref() else {
        return Ok(false);
    };
    let completions = text_editor_completions(editor);
    if completions.is_empty() {
        return Ok(false);
    }
    let active = index.unwrap_or_else(|| text_editor_ui(&scene.surface_id).completion_index);
    let Some(item) = completions.get(active).or_else(|| completions.first()) else {
        return Ok(false);
    };
    let (_, caret) = engine_canvas::text_editor_caret(scene);
    let prefix_start = identifier_prefix_start(&editor.buffer, caret);
    let insert = item.insert_text.clone().unwrap_or_else(|| item.label.clone());
    let applied = engine_canvas::text_editor_apply_completion_into(scene, prefix_start, caret, &insert, input)?;
    text_editor_close_completions(&scene.surface_id);
    Ok(applied)
}

/** @emoji ✏️ Arms the multi-span rename — React's `startRename`: the draft carries the scene's own
 * `rename_json` occurrences and current name, and the input takes keyboard focus so the next
 * keystrokes type the new name instead of editing the buffer. */
pub fn text_editor_start_rename(scene: &UiComponentSceneNode, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> bool {
    let Some(info) = scene.text_editor.as_ref().and_then(text_editor_rename_info) else {
        return false;
    };
    let occurrences: Vec<(usize, usize)> = info.occurrences.iter().map(|span| (span.start, span.end)).collect();
    input.focus_input_owned(format!("{}.editor.rename", scene.surface_id), info.name.clone());
    mutate_text_editor_ui(&scene.surface_id, |state| {
        state.rename = Some(TextEditorRenameDraft { occurrences, text: info.name.clone() });
        state.completions_open = false;
    });
    true
}

pub fn text_editor_rename_active(surface_id: &str) -> bool {
    text_editor_ui(surface_id).rename.is_some()
}

/** @emoji ✏️ Retypes the rename draft and republishes the multi-span PREVIEW into the editor host —
 * React's `updateRenamePreview` (`setText` + `setSelectionOccurrencesJson` + `setExtraCaretsJson`).
 * The preview is host-local; nothing is dispatched until the commit. */
pub fn text_editor_update_rename(scene: &UiComponentSceneNode, next_text: &str) -> bool {
    let Some(editor) = scene.text_editor.as_ref() else {
        return false;
    };
    let Some(draft) = text_editor_ui(&scene.surface_id).rename else {
        return false;
    };
    let (preview, occurrences) = multi_span_replace(&editor.buffer, &draft.occurrences, next_text);
    engine_canvas::text_editor_preview_rename(scene, &preview, &occurrences);
    mutate_text_editor_ui(&scene.surface_id, |state| {
        if let Some(draft) = state.rename.as_mut() {
            draft.text = next_text.to_string();
        }
    });
    true
}

/// ✏️ Commits the rename — `commitRename { occurrences, text }`, React's own payload — and disarms
/// the draft.
pub fn text_editor_commit_rename(scene: &UiComponentSceneNode, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    let Some(draft) = text_editor_ui(&scene.surface_id).rename else {
        return Ok(false);
    };
    let action = ui_wgpu::wgpu::text_editor_actions::COMMIT_RENAME;
    let spans = serde_json::to_string(&draft.occurrences.iter().map(|(start, end)| json!({ "start": start, "end": end })).collect::<Vec<_>>()).unwrap_or_else(|_| "[]".into());
    let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[&scene.controller_id, action, "surfaceId", &scene.surface_id, "text", &draft.text, "occurrences", &spans])?;
    let mut reservation = input.reserve_action(&scene.controller_id, action, bytes)?;
    {
        let builder = reservation.builder();
        builder.begin_object(None)?;
        builder.string(Some("surfaceId"), &scene.surface_id)?;
        builder.string(Some("text"), &draft.text)?;
        builder.begin_array(Some("occurrences"))?;
        for (start, end) in &draft.occurrences {
            builder.begin_object(None)?;
            builder.number(Some("start"), *start as f64)?;
            builder.number(Some("end"), *end as f64)?;
            builder.end_container()?;
        }
        builder.end_container()?;
        builder.end_container()?;
    }
    reservation.publish()?;
    mutate_text_editor_ui(&scene.surface_id, |state| state.rename = None);
    Ok(true)
}

/// ✏️ Cancels the rename and restores the committed buffer into the host — React's `cancelRename`.
pub fn text_editor_cancel_rename(scene: &UiComponentSceneNode) -> bool {
    if text_editor_ui(&scene.surface_id).rename.is_none() {
        return false;
    }
    if let Some(editor) = scene.text_editor.as_ref() {
        engine_canvas::text_editor_preview_rename(scene, &editor.buffer, &[]);
    }
    mutate_text_editor_ui(&scene.surface_id, |state| state.rename = None);
    true
}

/** @emoji 🖱️ The context-menu rows a text editor answers ITSELF instead of dispatching to the guest —
 * React's `localActions` map (`🧱️elements/✏️TextEditor/🟦️.tsx:419-470`). `Ok(false)` means the row is
 * not local and the caller must dispatch it as a normal action.
 *
 * `cut`/`copy`/`paste` are absent on purpose: they are `document.execCommand` on the React side and
 * this target has no OS clipboard binding at all (`ui_wgpu::wgpu::events`' `UiCommand::Clipboard*`
 * still says the read/write is an unwired host concern), so pretending to handle them would swallow
 * the row instead of leaving it dispatchable. */
pub fn text_editor_local_menu_action(scene: &UiComponentSceneNode, inner: Rect, action: &str, x: f32, y: f32, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    let Some(editor) = scene.text_editor.as_ref() else {
        return Ok(false);
    };
    match action {
        "requestCompletions" | "suggestCompletions" => {
            text_editor_open_completions(scene);
        }
        "selectToken" => {
            engine_canvas::text_editor_select_span_into(scene, inner, x, y, input)?;
        }
        "selectLine" => {
            let (_, caret) = engine_canvas::text_editor_caret(scene);
            let (start, end) = text_editor_line_range(&editor.buffer, caret);
            engine_canvas::text_editor_set_selection_into(scene, start, end, input)?;
        }
        "selectAll" => {
            engine_canvas::text_editor_set_selection_into(scene, 0, editor.buffer.len(), input)?;
        }
        "commitRename" | "rename" => {
            if !text_editor_start_rename(scene, input) {
                return Ok(false);
            }
        }
        _ => return Ok(false),
    }
    Ok(true)
}

/** @emoji 🖱️ Parks one context-menu row for this surface to answer itself on the next paint, and
 * answers whether the row IS local. The shell calls this before dispatching a menu row: `true` means
 * the row never reaches the guest, exactly as React's `dispatchTextEditorMenu` short-circuits an id
 * present in its own `localActions` map. */
pub fn text_editor_queue_menu_action(surface_id: &str, action: &str, x: f32, y: f32) -> bool {
    if !TEXT_EDITOR_LOCAL_MENU_ACTIONS.contains(&action) {
        return false;
    }
    mutate_text_editor_ui(surface_id, |state| state.pending_menu_action = Some((action.to_string(), x, y)));
    true
}

/// 🖱️ The rows a text editor answers itself — React's `localActions` keys, minus the three clipboard
/// verbs this target has no OS binding for (see [`text_editor_local_menu_action`]).
const TEXT_EDITOR_LOCAL_MENU_ACTIONS: [&str; 6] = ["requestCompletions", "suggestCompletions", "selectToken", "selectLine", "selectAll", "commitRename"];

/// ✏️ An armed rename input has KEYBOARD FOCUS, so it consumes every key: React's `<input>` is a
/// real focused element and `onKeyDown` stops propagation. The draft text is tracked here rather
/// than read back off `InputState::text_view` — that is a PAGED projection that is empty until the
/// text pump has run, so a rename would have previewed the empty string on its first keystroke.
/** @emoji ⌨️ The popup half of one keystroke, offered BEFORE the buffer sees it — React's
 * `onKeyDown` prelude (`🧱️elements/✏️TextEditor/🟦️.tsx:556-608`): `Ctrl/Cmd+Space` opens completions,
 * `F2` starts a rename, and while either popup is open its own arrow/commit/dismiss keys win over
 * editing. `false` leaves the key to `engine_canvas::text_editor_apply_key_into`. */
pub fn text_editor_popup_key(scene: &UiComponentSceneNode, key: &KeyAction, modifiers: &ui_wgpu::wgpu::PointerModifiers, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    let ui = text_editor_ui(&scene.surface_id);
    let accelerator = modifiers.ctrl || modifiers.meta;
    if let Some(draft) = ui.rename.as_ref() {
        match key {
            KeyAction::Escape => return Ok(text_editor_cancel_rename(scene)),
            KeyAction::Enter => return text_editor_commit_rename(scene, input),
            KeyAction::Backspace => {
                let mut next = draft.text.clone();
                next.pop();
                text_editor_update_rename(scene, &next);
            }
            KeyAction::Char(ch) if !accelerator && ch.chars().count() == 1 => {
                text_editor_update_rename(scene, &format!("{}{ch}", draft.text));
            }
            _ => {}
        }
        return Ok(true);
    }
    if ui.completions_open {
        match key {
            KeyAction::ArrowDown => return Ok(text_editor_move_completion(scene, true)),
            KeyAction::ArrowUp => return Ok(text_editor_move_completion(scene, false)),
            KeyAction::Enter | KeyAction::Tab => return text_editor_apply_completion(scene, None, input),
            KeyAction::Escape => {
                text_editor_close_completions(&scene.surface_id);
                return Ok(true);
            }
            _ => {}
        }
    }
    if accelerator && matches!(key, KeyAction::Space(_)) {
        return Ok(text_editor_open_completions(scene));
    }
    if matches!(key, KeyAction::Function(2)) {
        return Ok(text_editor_start_rename(scene, input));
    }
    Ok(false)
}

/** @emoji 🖱️ The popup half of one press. An ALT-click with completions available opens the dropdown
 * (React's `event.altKey && completions.length > 0` branch in `onContextMenu`); a press on an open
 * dropdown row commits that row; a press anywhere else closes it, the way an outside press dismisses
 * any popup. `false` leaves the press to the caret/selection path. */
pub fn text_editor_popup_pointer(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32, alt: bool, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    let Some(editor) = scene.text_editor.as_ref() else {
        return Ok(false);
    };
    if text_editor_ui(&scene.surface_id).completions_open {
        let count = text_editor_completions(editor).len();
        if let Some(index) = text_editor_completion_hit(scene, inner, &scene_input_theme(), count, x, y) {
            return text_editor_apply_completion(scene, Some(index), input);
        }
        text_editor_close_completions(&scene.surface_id);
        return Ok(false);
    }
    if alt {
        return Ok(text_editor_open_completions(scene));
    }
    Ok(false)
}
//#endregion ✍️PopupDrive

#[cfg(test)]
#[path = "../../🧪️tests/🔬️wgpu-text-editor/🦀️.rs"]
mod text_editor_tests;
//#endregion TextEditor
