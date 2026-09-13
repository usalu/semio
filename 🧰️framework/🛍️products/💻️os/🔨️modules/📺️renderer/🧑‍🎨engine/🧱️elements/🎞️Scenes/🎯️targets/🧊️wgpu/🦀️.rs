//! 🎬️ framework/products/os/modules/renderer/engine/elements/🎞️Scenes/component.rs — wgpu render
//! implementation for the Scenes element, extracted from lib.rs's inline `pub mod scenes { ... }`
//! body (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE). Wired via `#[path =
//! "../../../../🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs"] pub mod scenes;` in lib.rs in place of the former
//! inline block; the module name `scenes` is unchanged, so every existing `crate::scenes::...`
//! call site elsewhere in the crate keeps resolving with zero other changes.
//! 🎬️ Native component scene hosts for canvas-2d, tables, graphs, and 3D views.


use crate::engine_canvas;
use crate::interpreter::FrameworkWidgetContext;
#[cfg(test)]
use base64::Engine;
#[cfg(test)]
use semio_framework::IconName;
use serde::Deserialize;
use serde_json::{json, Value};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::io::Write as _;
#[cfg(test)]
use ui_wgpu::wgpu::input::{DragAxis, KeyAction};
#[cfg(test)]
use ui_wgpu::wgpu::{draw_text, draw_text_wrapped, render_widget, HitKind, HitTarget, Theme, WidgetNode};
use ui_wgpu::wgpu::Rect;
#[cfg(test)]
use ui_wgpu::wgpu::Rgba;
use ui_wgpu::wgpu::{ActionDescriptor, PreparedRasterProducer, PreparedRasterRejected, PreparedRasterReservation, SurfaceKind, UiComponentSceneNode};
#[cfg(test)]
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
    #[cfg(test)]
    fn from_json(raw: &str) -> Self {
        serde_json::from_str::<Value>(raw)
            .ok()
            .map(|value| Self { x: value.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32, y: value.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32, zoom: value.get("zoom").and_then(|v| v.as_f64()).unwrap_or(1.0) as f32 })
            .unwrap_or_default()
    }

    #[cfg(test)]
    fn from_typed(viewport: Option<&semio_framework_os_kernel::Viewport2d>) -> Self {
        viewport.map(|viewport| Self { x: viewport.x as f32, y: viewport.y as f32, zoom: viewport.zoom as f32 }).unwrap_or(Self { x: 0.0, y: 0.0, zoom: 1.0 })
    }

    fn screen_to_world(&self, sx: f32, sy: f32, origin: Rect) -> (f32, f32) {
        let cx = origin.x + origin.w * 0.5;
        let cy = origin.y + origin.h * 0.5;
        ((sx - cx) / self.zoom + self.x, (sy - cy) / self.zoom + self.y)
    }

    #[cfg(test)]
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
    #[cfg(test)]
    last_click_ms: f64,
    #[cfg(test)]
    last_click_target: Option<String>,
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
        debug_assert!(returned, "checked-out raster producer must return to its exact FIFO slot");
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






//#endregion SceneRuntime



//#region SceneInput
const MAP_MARQUEE_THRESHOLD_PX: f32 = 6.0;

fn write_canvas_pointer_action(
    batch: &mut ui_wgpu::wgpu::BoundedActionBatchReservation<'_>,
    scene: &UiComponentSceneNode,
    action: &str,
    world_x: f32,
    world_y: f32,
    button: Option<i16>,
    extend: Option<bool>,
) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    let bytes = match (button, extend) {
        (Some(_), Some(_)) => ui_wgpu::wgpu::checked_action_string_bytes(&[&scene.controller_id, action, "surfaceId", &scene.surface_id, "x", "y", "button", "extend"])?,
        (None, None) => ui_wgpu::wgpu::checked_action_string_bytes(&[&scene.controller_id, action, "surfaceId", &scene.surface_id, "x", "y"])?,
        _ => return Err(ui_wgpu::wgpu::BoundedActionFault::Structure),
    };
    batch.action(&scene.controller_id, action, bytes, |builder| {
        builder.begin_object(None)?;
        builder.string(Some("surfaceId"), &scene.surface_id)?;
        builder.number(Some("x"), f64::from(world_x))?;
        builder.number(Some("y"), f64::from(world_y))?;
        if let Some(button) = button {
            builder.number(Some("button"), f64::from(button))?;
        }
        if let Some(extend) = extend {
            builder.boolean(Some("extend"), extend)?;
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
    let (world_x, world_y) = viewport.screen_to_world(x, y, inner);
    let mut batch = action
        .then(|| {
            let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[&scene.controller_id, "canvasPointerMove", "surfaceId", &scene.surface_id, "x", "y"])?;
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
        write_canvas_pointer_action(batch, scene, "canvasPointerMove", world_x, world_y, None, None)?;
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
    let (world_x, world_y) = viewport.screen_to_world(x, y, inner);
    let stroke_action = if down && button == 0 {
        Some("paintStrokeBegin")
    } else if !down && paint_stroke_active {
        Some("paintStrokeEnd")
    } else {
        None
    };
    let pointer_action = if down { "canvasPointerDown" } else { "canvasPointerUp" };
    let pointer_bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[&scene.controller_id, pointer_action, "surfaceId", &scene.surface_id, "x", "y", "button", "extend"])?;
    let stroke_bytes = stroke_action.map(|action| canvas_surface_action_bytes(scene, action)).transpose()?.unwrap_or(0);
    let item_credits = 1 + usize::from(stroke_action.is_some());
    let mut batch = input.reserve_actions(item_credits, pointer_bytes.checked_add(stroke_bytes).ok_or(ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?)?;
    if let Some(action) = stroke_action {
        write_canvas_surface_action(&mut batch, scene, action)?;
    }
    write_canvas_pointer_action(&mut batch, scene, pointer_action, world_x, world_y, Some(button), Some(shift))?;
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
        SurfaceKind::Paint2d => {
            let Some(paint) = scene.paint_2d.as_ref() else {
                return false;
            };
            if paint.document_sync_json.len() > INK_INTERACTION_DOCUMENT_BYTE_CAPACITY {
                return false;
            }
            let document: Paint2dDocSyncJson = serde_json::from_str(&paint.document_sync_json).unwrap_or_default();
            mutate_scene_state(&scene.surface_id, |state| {
                if state.viewport.zoom <= 0.0 {
                    state.viewport = Viewport { x: document.camera.x as f32, y: document.camera.y as f32, zoom: document.camera.zoom as f32 };
                }
                let factor = (1.0 - delta * 0.001).clamp(0.5, 2.0);
                state.viewport.zoom = (state.viewport.zoom * factor).clamp(0.05, 32.0);
                state.camera_dispatch_controller_id = Some(scene.controller_id.clone());
            });
            schedule_scene_camera_dispatch(&scene.surface_id);
        }
        _ => return false,
    }
    true
}

pub(crate) fn passive_scene_pointer_button(scene: &UiComponentSceneNode, bounds: Rect, x: f32, y: f32, down: bool, button: i16) -> bool {
    if scene.component_kind != SurfaceKind::Paint2d || !bounds.contains(x, y) {
        return false;
    }
    if down && (button == 1 || button == 2) {
        mutate_scene_state(&scene.surface_id, |state| {
            state.pointer_was_down = true;
            state.drag = Some(SceneDrag { mode: SceneDragMode::PanViewport });
        });
    } else if !down {
        mutate_scene_state(&scene.surface_id, |state| {
            state.pointer_was_down = false;
            state.drag = None;
        });
    }
    true
}

pub(crate) fn passive_scene_pointer_move(scene: &UiComponentSceneNode, bounds: Rect, x: f32, y: f32, delta_x: f32, delta_y: f32) -> bool {
    if scene.component_kind != SurfaceKind::Paint2d || !bounds.contains(x, y) {
        return false;
    }
    let viewport = SCENE_STATE.with(|cell| cell.borrow().get(&scene.surface_id).and_then(|state| matches!(state.drag.as_ref().map(|drag| &drag.mode), Some(SceneDragMode::PanViewport)).then_some(state.viewport)));
    let Some(viewport) = viewport else {
        return false;
    };
    mutate_scene_state(&scene.surface_id, |state| {
        state.viewport.x -= delta_x / viewport.zoom.max(0.01);
        state.viewport.y -= delta_y / viewport.zoom.max(0.01);
        state.camera_dispatch_controller_id = Some(scene.controller_id.clone());
        state.last_pointer_pos = (x, y);
    });
    schedule_scene_camera_dispatch(&scene.surface_id);
    true
}










//#endregion SceneInput

//#region RenderEntry
/// 🎞️ Advances one retained scene identifier scalar or one pre-admitted chrome output item.
/// 🧩️ The per-frame engine state the retained scene paint needs beyond its own draw list: the shell's
/// `World3dState` map (a `World3d` surface's host, addressed there by every pick/asset/authority
/// ladder) and this frame's `World3dBuildContext` (built in `FrameBuildPhase::WorldResources`, drained
/// again in `FrameBuildPhase::WorldTransfer`). Borrowed for exactly one chrome walk and never stored —
/// the same reason `Ui::frame` takes its `SceneHost` as a parameter rather than owning one.
pub struct SceneEngineHosts<'a> {
    pub world3d_states: &'a mut AdmittedSurfaceMap<infinite_world::world::World3dState>,
    pub world_resources: &'a mut infinite_world::world::World3dBuildContext,
}

pub fn render_component_scene_step(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>, cursor: &mut ui_wgpu::wgpu::ScenePaintCursor, hosts: &mut SceneEngineHosts<'_>) -> ui_wgpu::wgpu::ScenePaintStep {
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
            if scene.component_kind == SurfaceKind::World3d {
                return render_world3d_surface_step(scene, bounds, ctx, cursor, hosts);
            }
            if !engine_canvas::sync_engine_scene(scene, bounds, ctx.theme) {
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
            if scene.component_kind != SurfaceKind::NodeGraph {
                return cursor.finish();
            }
            engine_canvas::paint_node_graph_labels(ctx, scene, bounds);
            if cursor.advance_phase().is_err() {
                return ui_wgpu::wgpu::ScenePaintStep::Fault;
            }
            ui_wgpu::wgpu::ScenePaintStep::Pending
        }
        8 => {
            engine_canvas::paint_node_graph_overlays(ctx, scene, bounds);
            cursor.finish()
        }
        _ => cursor.finish(),
    }
}


/// 🎨️ The colour a composited engine texture clears to before its host paints — the panel behind a
/// node graph, the canvas ground behind a map or a board, exactly as each React host's own
/// `clear_color` resolves it.
fn engine_surface_clear(kind: SurfaceKind, theme: &ui_wgpu::wgpu::Theme) -> ui_wgpu::wgpu::Rgba {
    match kind {
        SurfaceKind::TiledMap | SurfaceKind::Board2d => theme.canvas_clear,
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
    infinite_world::world::render_world_3d(scene, bounds, ctx, state, hosts.world_resources);
    engine_canvas::register_engine_surface(scene, bounds, engine_canvas::EngineSurfaceKindDetail::World3d { status_json: scene.world_3d.as_ref().and_then(|world| world.status_json.clone()) }, created);
    world3d_surface_debug_log(scene, bounds, ctx, state);
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
    debug_log(&format!("[DEBUG] world3d surface={} pane={:?} bounds={}x{} {geometry} {} {payload}", scene.surface_id, scene.pane_id, bounds.w.round(), bounds.h.round(), state.ingest_census()));
}

fn debug_log(line: &str) {
    #[cfg(target_arch = "wasm32")]
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(line));
    #[cfg(not(target_arch = "wasm32"))]
    eprintln!("{line}");
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
#[derive(Deserialize, Clone, Copy)]
struct Paint2dCameraFields {
    #[serde(default)]
    x: f64,
    #[serde(default)]
    y: f64,
    #[serde(default = "paint2d_default_one")]
    zoom: f64,
}

impl Default for Paint2dCameraFields {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

fn paint2d_default_one() -> f64 {
    1.0
}

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
 * (`framework/surface/paint/rs/lib.rs`'s premigration `rasterNavigatorFitCamera`; that crate is a
 * sibling used by the React `Paint2dHost`'s WASM raster session and is not wired into this wgpu
 * renderer's dependency graph, so the fit math is reimplemented here with this file's own
 * `Viewport`/`Rect`). Falls back to a neutral centered camera when the document has no pixel content. */


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
#[cfg(test)]
#[derive(Deserialize)]
struct TableColumn {
    id: String,
    label: String,
    #[serde(default)]
    sortable: bool,
}

/// 🔀️ Mirrors `sourcing::TableSort`'s wire format (`{columnId, direction}`) — the active sort
/// state for a [`TableScene`] whose columns opt into `sortable`.
#[cfg(test)]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TableSortJson {
    column_id: String,
    direction: String,
}

/// 🧾️ Mirrors `ui_wgpu::wgpu::TableCell` — a typed table cell value parsed out of a row's raw JSON.
#[cfg(test)]
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
enum TableCellPayload {
    Text { value: String },
    Number { value: f64 },
    Stepper { value: f64, min: f64, max: f64, step: f64, action: ActionDescriptor },
    Buttons { buttons: Vec<TableCellButtonPayload> },
}

#[cfg(test)]
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TableCellButtonPayload {
    icon_id: String,
    #[serde(default)]
    label: Option<String>,
    action: ActionDescriptor,
}

/// 🔗️ Merges `patch` into `base`'s existing args (rather than replacing them), so a stepper/button cell keeps its row-identifying args (e.g. `objectId`) alongside the delta/click patch.


/// 🧾️ Renders a table cell's interactive controls (stepper/buttons) directly, or returns the plain text to draw for text/number/legacy-string cells.



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
#[cfg(test)]
#[derive(Deserialize)]
struct BlockListBlockJson {
    id: String,
    label: String,
    kind: String,
}

/// 🧩️ Mirrors `playbook::PlaybookStep`'s renderer-relevant fields.
#[cfg(test)]
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
#[cfg(test)]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct BlockListPaletteEntryJson {
    block_kind: String,
    label: String,
    #[serde(default)]
    icon_id: String,
}

/// 🧩️ Renders the strict-list Blockly-like block-list builder ([`SurfaceKind::BlockList`]):
/// steps stacked vertically (each with its ordered blocks) plus a palette rail for inserting new
/// blocks, mirroring `block-list-host.tsx`'s layout and action verbs (`addStep`/`removeStep`/
/// `moveStep`/`addBlock`/`removeBlock`/`moveBlock`, see `playbook::builder_kit` and
/// `playbook-plugin`'s `handle_action`). Reordering dispatches `moveStep`/`moveBlock` from
/// move-up/move-down hit targets rather than free-form pointer drag (unlike the React host's
/// dnd-kit drag-and-drop): this renderer's click-dispatch model (see `render_table`'s row/header
/// hits) has no established cross-frame drag-position-tracking primitive for list reordering, and
/// building one is out of this ticket's scope (`w2-scene-wiring` owns generic pointer routing).
/// Selection highlighting reads `selected_id` directly (mirrors `render_table`'s selected-row
/// highlight) even though `block-list-host.tsx` does not yet render it either.

//#endregion BlockList

//#region BlockListTests
#[cfg(test)]
#[path = "../../🧪️tests/🔬️wgpu-block-list/🦀️.rs"]
mod block_list_tests;
//#endregion BlockListTests

//#region DiffView
#[cfg(test)]
#[derive(Clone, Copy, PartialEq, Debug)]
enum DiffLineOperation {
    Equal,
    Removed,
    Added,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug)]
struct DiffLine<'a> {
    operation: DiffLineOperation,
    text: &'a str,
}

/// 🧮️ Above this many `before.len() * after.len()` DP cells, [`diff_lines`] skips the LCS table and
/// falls back to a positional compare so a single huge [`SurfaceKind::DiffView`] payload can't blow
/// up per-frame recompute cost (this crate re-derives the diff every render pass, mirroring how
/// `render_graph_timeline` re-parses `columns_json` every frame rather than caching it).
#[cfg(test)]
const DIFF_LCS_CELL_BUDGET: usize = 200_000;

/// 🔀️ Line-level LCS diff (classic DP backtrace). Falls back to a naive positional compare above
/// [`DIFF_LCS_CELL_BUDGET`] cells.


/// 🩹️ Renders [`SurfaceKind::DiffView`]: a line-level diff of `before`/`after` text, either as a
/// single scrolling column with `+`/`-` markers (default, or `mode: "unified"`) or as two aligned
/// columns (`mode: "split"`). Text-only — no syntax highlighting, gutter line numbers, or monospace
/// font for `language` yet (the latter two need capabilities this crate's `draw_text`/layout don't
/// have). Add/remove **text** is tinted with the theme's `accent`/`error` tokens (equal text stays
/// full-brightness `theme.text`), matching `DIFF_LINE_CLASS`'s per-line text-color classes in
/// `diff-view-host.tsx` — this used to instead wash the whole row background and dim the *unchanged*
/// majority of lines, the opposite of what the React source of truth does.

//#endregion DiffView

//#region DiffViewTests
#[cfg(test)]
#[path = "../../🧪️tests/🔬️wgpu-diff-view/🦀️.rs"]
mod diff_view_tests;
//#endregion DiffViewTests

//#region EventFeed
/// 🪶️ Mirrors a `SurfaceKind::EventFeed` entry (`{id, timestampMs, iconId, title, detail?, tone?}`,
/// `ui_wgpu::wgpu::EventFeedScene`'s doc comment / `EventFeedEntry` in `framework/core/js/index.ts`).
#[cfg(test)]
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

/// 🕒️ Renders a ms-since-epoch timestamp as a bare `HH:MM:SS` UTC time-of-day — no calendar/timezone
/// library in this crate, so this deliberately doesn't attempt a full date.


/// 🎨️ Maps an entry's free-form `tone` to an existing theme token — no new color literals. Only the
/// tones this crate already has a token for get a distinct color; anything else (including no tone)
/// stays neutral. Widen this as more tones prove common once the host apps start emitting them.




/// 📜️ Renders [`SurfaceKind::EventFeed`]: a scrollable list of text rows (`entries_json`), each a
/// tone dot + optional icon + time-of-day + title, with an optional detail line beneath. When
/// `follow` is set the feed snaps to its bottom every frame (log-tail behavior); a manual
/// wheel-scroll on a following feed is overridden on the next render, same tradeoff a live log tail
/// makes. Rows dispatch `activate_action` (when set) with `{ "entryId": ... }`, mirroring
/// `render_graph_timeline`'s per-row `checkoutCheckpoint` hit.

//#endregion EventFeed

//#region EventFeedTests
#[cfg(test)]
#[path = "../../🧪️tests/🔬️wgpu-event-feed/🦀️.rs"]
mod event_feed_tests;
//#endregion EventFeedTests

//#region GraphTimeline
/** @emoji 🗄️ Mirrors `store::HistoryColumn` / React `HistoryColumn` (`ui/js/react/index.tsx:19116`). */
#[cfg(test)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HistoryColumnAuthorJson {
    #[serde(default)]
    name: String,
}

#[cfg(test)]
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

#[cfg(test)]
const HISTORY_LANE_PITCH: f32 = 16.0;
#[cfg(test)]
const HISTORY_LANE_PAD: f32 = 8.0;
#[cfg(test)]
const HISTORY_AUTHOR_SLOT: f32 = 40.0;

/** Ports `historyLaneCount` (`ui/js/react/index.tsx:19141`). */


/** Ports `historyGraphWidth` (`ui/js/react/index.tsx:19145`). */


/** Ports `historyLaneX` (`ui/js/react/index.tsx:19153`). */


/** Ports `historyRowLaneGuides` (`ui/js/react/index.tsx:19162`): per-row, per-lane guide-line
 * visibility, including the elbow-row propagation when a checkpoint's parent sits on another lane. */


/// 🔤️ Two-letter initials from the first two words of an author name (e.g. "Jane Doe" → "JD"),
/// matching the avatar-initials helper in `index.tsx` — previously the caller only took the very
/// first character of the whole string (e.g. "Jane Doe" → "J").



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
#[cfg(test)]
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
#[cfg(test)]
#[derive(Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
struct CanvasGradientStopJson {
    #[serde(default)]
    offset: f64,
    #[serde(default)]
    color: Option<Vec<f64>>,
}

/** 🖊️ A `CanvasLayerRecord["stroke"]` mirror. */
#[cfg(test)]
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
#[cfg(test)]
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
#[cfg(test)]
#[derive(Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
struct CanvasTextFieldJson {
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    size: Option<f64>,
}

#[cfg(test)]
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

#[cfg(test)]
#[derive(Deserialize)]
struct Canvas2dPacketText<'a> {
    #[serde(borrow)]
    content: &'a str,
    #[serde(default = "canvas2d_packet_text_size")]
    size: f64,
}

#[cfg(test)]
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










/** 🧾️ Reserves the fixed raster ledger before source preparation or the one-backing decoder. */
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


#[cfg(test)]
fn canvas_lum(c: [f32; 3]) -> f32 {
    0.3 * c[0] + 0.59 * c[1] + 0.11 * c[2]
}

#[cfg(test)]
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

#[cfg(test)]
fn canvas_set_lum(c: [f32; 3], l: f32) -> [f32; 3] {
    let d = l - canvas_lum(c);
    canvas_clip_color([c[0] + d, c[1] + d, c[2] + d])
}

#[cfg(test)]
fn canvas_sat(c: [f32; 3]) -> f32 {
    c[0].max(c[1]).max(c[2]) - c[0].min(c[1]).min(c[2])
}

#[cfg(test)]
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
#[cfg(test)]
const CANVAS_GRADIENT_BANDS: usize = 10;
#[cfg(test)]
const CANVAS_CIRCLE_SEGMENTS: usize = 28;

#[cfg(test)]
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
#[cfg(test)]
const CANVAS2D_SELECTION_RING: Rgba = Rgba::new(0.984_314, 0.749_02, 0.141_176, 0.95);
#[cfg(test)]
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













#[cfg(test)]
const INK_RESIZE_HANDLES: [&str; 8] = ["nw", "n", "ne", "e", "se", "s", "sw", "w"];





/** @emoji 📝️ Pointer-down entry point for ink-canvas: mirrors handlePointerDown in ink-canvas-host.tsx. */


/** @emoji 📝️ Pointer-up entry point for ink-canvas: commits the active gesture and finalizes marquee selection. */


/** @emoji 📝️ Pointer-move hover entry point for ink-canvas: mirrors the `!dragState` hover branch of handlePointerMove. */


/** @emoji 📝️ Wheel entry point for ink-canvas: zoom-at-cursor, mirrors handleWheel in ink-canvas-host.tsx. */

//#endregion InkCanvasState

//#region InkCanvasRender










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
}

//#endregion NodeGraph

//#region TiledMap
#[derive(Clone, Debug)]
pub struct TiledMapSurface {
    pub bounds: Rect,
    pub controller_id: String,
    pub selection_method: String,
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


/** @emoji 🗺️ Pushes GIS map context-menu items for a screen-space hit. */

fn write_tiled_map_selection(
    input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>,
    controller_id: &str,
    surface_id: &str,
    positions: &[String],
    routes: &[String],
    mode: &str,
    commit: impl FnOnce(),
) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    if positions.len().checked_add(routes.len()).ok_or(ui_wgpu::wgpu::BoundedActionFault::ItemCredits)? > ui_wgpu::wgpu::action::ACTION_NODE_CAPACITY - 5 {
        return Err(ui_wgpu::wgpu::BoundedActionFault::NodeCredits);
    }
    let action = ui_wgpu::wgpu::tiled_map_actions::SET_FEATURE_SELECTION;
    let mut bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller_id, action, "surfaceId", surface_id, "positions", "routes", "mode", mode])?;
    for id in positions.iter().chain(routes) {
        bytes = bytes.checked_add(id.len()).ok_or(ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?;
    }
    let mut reservation = input.reserve_action(controller_id, action, bytes)?;
    let builder = reservation.builder();
    builder.begin_object(None)?;
    builder.string(Some("surfaceId"), surface_id)?;
    builder.begin_array(Some("positions"))?;
    for id in positions {
        builder.string(None, id)?;
    }
    builder.end_container()?;
    builder.begin_array(Some("routes"))?;
    for id in routes {
        builder.string(None, id)?;
    }
    builder.end_container()?;
    builder.string(Some("mode"), mode)?;
    builder.end_container()?;
    reservation.publish_with(commit)
}

fn write_tiled_map_hover(input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>, controller_id: &str, surface_id: &str, hover: Option<(&str, &str)>, commit: impl FnOnce()) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    let action = ui_wgpu::wgpu::tiled_map_actions::SET_HOVER;
    let (kind, id) = hover.unwrap_or(("", ""));
    let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller_id, action, "surfaceId", surface_id, "hover", "kind", "id", kind, id])?;
    let mut reservation = input.reserve_action(controller_id, action, bytes)?;
    let builder = reservation.builder();
    builder.begin_object(None)?;
    builder.string(Some("surfaceId"), surface_id)?;
    if hover.is_some() {
        builder.begin_object(Some("hover"))?;
        builder.string(Some("kind"), kind)?;
        builder.string(Some("id"), id)?;
        builder.end_container()?;
    } else {
        builder.null(Some("hover"))?;
    }
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
            write_tiled_map_selection(input, controller_id, surface_id, &positions, &routes, merge_mode, || {
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
#[cfg(test)]
#[derive(Deserialize)]
struct IconRenderRequestFields {
    width: f64,
    height: f64,
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
#[cfg(test)]
#[derive(Deserialize)]
struct VfsDescriptorKind {
    #[serde(default)]
    presentation: String,
}

#[cfg(test)]
#[derive(Deserialize)]
struct VfsFileNodeKind {
    #[serde(default)]
    icon: Option<String>,
    #[serde(default)]
    descriptors: Vec<VfsDescriptorColumn>,
}

#[cfg(test)]
#[derive(Deserialize)]
struct VfsDescriptorColumn {
    id: String,
    #[serde(default)]
    label: String,
    #[serde(rename = "descriptorKindId", default)]
    descriptor_kind_id: String,
}

#[cfg(test)]
#[derive(Deserialize)]
struct VfsSchema {
    #[serde(rename = "descriptorColumnIds", default)]
    descriptor_column_ids: Vec<String>,
    #[serde(rename = "descriptorKinds", default)]
    descriptor_kinds: HashMap<String, VfsDescriptorKind>,
    #[serde(rename = "fileNodeKinds", default)]
    file_node_kinds: HashMap<String, VfsFileNodeKind>,
}




/// 🗂️ Resolves the row glyph, matching `VirtualFileSystemNodeGlyph`'s kind→icon lookup in
/// `index.tsx`. Previously a configured `fileNodeKinds[kindId].icon` only gated an `.is_some()`
/// check and the *actual* configured icon id was discarded in favor of a hardcoded `"folder"` —
/// any non-folder kind with its own icon (e.g. a custom "asset" kind) rendered the wrong glyph.
/// Extension-based file-type glyphs (React's ~40-entry `zip`→file-archive table) are not ported
/// here: the native icon atlas's available id set overlaps an in-flight `IconName` migration in
/// another session, so guessing unverified ids risks silently blank icons — left as a known gap.






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
#[derive(Clone, Debug, Default)]
#[cfg(test)]
struct TextEditorUiState {
    was_pointer_down: bool,
    /// 🖱️ `now_ms()`-based double-click detection — same convention (and the same always-`0.0`-on-native
    /// limitation) as `hit_double_click_target` elsewhere in this file.
    last_click_ms: f64,
    last_click_offset: Option<usize>,
    completions_open: bool,
    completion_index: usize,
    context_menu: Option<TextEditorContextMenu>,
    pending_context_click: Option<(f32, f32, i16)>,
    rename_active: bool,
    rename_occurrences: Vec<(usize, usize)>,
}

#[derive(Clone, Debug)]
#[cfg(test)]
struct TextEditorContextMenu {
    x: f32,
    y: f32,
    items: Vec<TextEditorMenuItem>,
}

#[derive(Clone, Copy, Debug)]
#[cfg(test)]
struct TextEditorMenuItem {
    id: &'static str,
    label: &'static str,
}

/// 📋️ Mirrors `CompletionItem` (`text-editor-host.tsx`); `insertText` has no producer yet anywhere in the
/// codebase (`jack_completions_json` only ever emits `label`/`detail`) so `insert_text` falls back to
/// `label`, exactly like `identifierPrefixStart`/`applyCompletion` do on the React side.
#[cfg(test)]
#[derive(Clone, Deserialize)]
struct TextEditorCompletionItem {
    label: String,
    #[serde(default)]
    detail: Option<String>,
    #[serde(default, rename = "insertText")]
    insert_text: Option<String>,
}

#[cfg(test)]
#[derive(Clone, Copy, Deserialize)]
struct TextEditorSpan {
    start: usize,
    end: usize,
}

/// ✏️ Mirrors `RenameInfo` (`text-editor-host.tsx`), parsed from `TextEditorScene::rename_json`.
#[cfg(test)]
#[derive(Clone, Deserialize)]
struct TextEditorRenameInfo {
    name: String,
    occurrences: Vec<TextEditorSpan>,
}









/// ✂️ Identifier-prefix scan back from `caret`, mirroring `identifierPrefixStart`
/// (`framework/renderer/react/components/text-editor-host.tsx`) for the completion-commit replacement range.


/// 📏️ `[start, end)` byte range of the buffer line containing `cursor`, via the (previously unwired)
/// `line_col_at` helper below — mirrors `lineRangeAt` (`text-editor-host.tsx`)'s "Select Line" semantics.


/// 🧭️ Right-click menu rows, mirroring `buildTextEditorContextMenuItems` (`text-editor-host.tsx`) minus
/// clipboard (no OS clipboard binding exists anywhere in this crate yet — see `ui_wgpu::wgpu::events`'
/// `UiCommand::ClipboardCopy/Cut/PasteRequested`, which even there says the OS read/write is a
/// "host-region concern" still unwired) and the domain-specific "pick target" rows (those need a new
/// `EditorHost::pick_targets_at_screen_json` wrapper; deferred, noted in the ticket report).


/// ▶️ Executes one context-menu row. `inner` re-derives the click point in surface-local screen space for
/// "Select Token"/"Select Line"; "Select All" reuses `engine_canvas::text_editor_apply_key`'s existing
/// Ctrl/Cmd+A path instead of adding a sixth wrapper.

//#endregion State

//#region Popups
/// 📍️ Anchor (surface-local screen space) for the completions dropdown: near the caret, falling back to a
/// fixed offset if the host isn't ready yet — mirrors `WasmEditorSurface`'s `position ? ... : { left: 12, top: 12 }`.









/// 🍿️ Local fallback popup: `ui_wgpu::wgpu::events::{OverlayKind, open_overlay}` (the w1d-events-overlay
/// workstream) is `pub(crate)` inside `ui_wgpu` — not reachable from this crate yet ("None of the new
/// `EventRouter` API or new public types are called/re-exported from `engine`/crate-root yet", per that
/// workstream's own report) — so this draws directly via `ctx.draw`, same convention as row-list surfaces
/// elsewhere in this module (`render_vfs`'s `theme.selected`/`theme.row_hover` rows). Known limitation:
/// unlike `shell`'s own `render_context_menu` (drawn into a dedicated top-level overlay `DrawList`), this
/// draws into the regular in-flow layer, so it can't guarantee being on top of *other* panels — only of
/// this surface's own content and anything already drawn earlier in the frame.





//#endregion Popups

//#region Geometry



//#endregion Geometry

//#region Render
//#endregion Render

#[cfg(test)]
#[path = "../../🧪️tests/🔬️wgpu-text-editor/🦀️.rs"]
mod text_editor_tests;
//#endregion TextEditor
