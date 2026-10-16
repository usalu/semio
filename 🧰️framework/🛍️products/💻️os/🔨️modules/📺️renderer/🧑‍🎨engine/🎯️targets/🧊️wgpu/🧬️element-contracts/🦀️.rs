//! 🧬️ Element-neutral contracts of the wgpu renderer.
//!
//! The shared vocabulary the engine canvas, the interpreter and the scenes elements all speak: the worker-safe retained cell,
//! the widget context of one paint pass, the identity of a retained scene a pointer is captured on and the typed refusal of a
//! raster upload. Each element imports these from here, so no element has to reach into a sibling element for a type.

use std::sync::{Mutex, MutexGuard};
use ui_wgpu::wgpu::{ActionDescriptor, NodeId, WidgetContext};

pub type FrameworkWidgetContext<'a> = WidgetContext<'a, ActionDescriptor>;

/// 🧵️ Worker-safe retained cell used by renderer state that may resume on any shared-pool worker.
/// 🔓️ `pub`: the Shell's native `CHROME_PREFS` static (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:16117`)
/// holds one and named the type bare, which never resolved outside this module.
///
/// 🧪️ Under `cfg(test)` every test thread owns its own state of every cell ([`test_worker_cell`]): libtest runs each law
/// on a thread of its own, so no law meets another law's engine, documents or chrome registry — in parallel or in
/// sequence — and the renderer suite is one usable gate instead of an order-dependent one.
pub struct WorkerCell<T> {
    initialize: fn() -> T,
    #[cfg(not(any(test, feature = "testkit")))]
    inner: std::sync::OnceLock<Mutex<T>>,
    #[cfg(any(test, feature = "testkit"))]
    inner: std::marker::PhantomData<fn() -> T>,
}

impl<T> WorkerCell<T> {
    pub const fn new(initialize: fn() -> T) -> Self {
        #[cfg(not(any(test, feature = "testkit")))]
        return Self { initialize, inner: std::sync::OnceLock::new() };
        #[cfg(any(test, feature = "testkit"))]
        return Self { initialize, inner: std::marker::PhantomData };
    }
}

#[cfg(any(test, feature = "testkit"))]
thread_local! {
    /// 🧪️ This test thread's state of each worker cell, by the cell's address; leaked for the thread's (one law's) life.
    static TEST_WORKER_CELLS: std::cell::RefCell<std::collections::HashMap<usize, &'static dyn std::any::Any>> = std::cell::RefCell::new(std::collections::HashMap::new());
}

/// 🧪️ The calling test thread's own instance of the worker-cell state at `key`, built by the cell's `initialize` on first
/// use. Every renderer `WorkerCell` (this module's, the scenes' and the engine canvas') resolves through it under `cfg(test)`.
#[cfg(any(test, feature = "testkit"))]
pub fn test_worker_cell<T: 'static>(key: usize, initialize: impl FnOnce() -> T) -> &'static Mutex<T> {
    let state = TEST_WORKER_CELLS.with(|cells| *cells.borrow_mut().entry(key).or_insert_with(|| Box::leak(Box::new(Mutex::new(initialize()))) as &'static dyn std::any::Any));
    state.downcast_ref::<Mutex<T>>().expect("one worker cell holds one state type")
}

impl<T: 'static> WorkerCell<T> {
    pub fn state(&self) -> &Mutex<T> {
        #[cfg(not(any(test, feature = "testkit")))]
        return self.inner.get_or_init(|| Mutex::new((self.initialize)()));
        #[cfg(any(test, feature = "testkit"))]
        return test_worker_cell::<T>(std::ptr::from_ref(self).addr(), self.initialize);
    }

    /// 🔒️ Crate-visible like `state`: `🐚️Shell/🎯️targets/🧊️wgpu`'s `with_chrome_prefs` drives this
    /// cell through the same `with`/`borrow_mut` shape as the thread-local twin it replaces.
    pub fn borrow(&self) -> MutexGuard<'_, T> {
        self.state().lock().expect("worker interpreter state")
    }

    pub fn borrow_mut(&self) -> MutexGuard<'_, T> {
        self.borrow()
    }

    /// 🔒️ The state unless another holder has it right now — the engine canvas's non-blocking registry turns.
    pub fn try_borrow_mut(&self) -> Option<MutexGuard<'_, T>> {
        self.state().try_lock().ok()
    }

    pub fn with<R>(&self, apply: impl FnOnce(&Self) -> R) -> R {
        apply(self)
    }
}

pub const SCENE_POINTER_OWNER_CAPACITY: usize = 16;

#[derive(Clone, Debug, PartialEq)]
pub struct ScenePointerTarget {
    pub document_id: ui_contract::UiNodeId,
    pub host_id: String,
    pub window_id: String,
    pub surface_id: String,
    pub kind: ui_wgpu::wgpu::SurfaceKind,
    pub node: NodeId,
    pub key: ui_wgpu::wgpu::NodeKey,
    pub window_generation: u64,
    pub component_generation: u64,
}

impl ScenePointerTarget {
    /// 🪪️ Compares a component resource owner across independently retained tree arenas.
    pub fn same_component_host(&self, other: &Self) -> bool {
        self.document_id == other.document_id
            && self.host_id == other.host_id
            && self.window_id == other.window_id
            && self.window_generation == other.window_generation
            && self.component_generation == other.component_generation
            && self.kind == other.kind
            && self.surface_id == other.surface_id
            && self.key == other.key
    }

    pub fn component_witness(&self) -> ui_wgpu::wgpu::reconcile::UiComponentSceneWitness<'_> {
        ui_wgpu::wgpu::reconcile::UiComponentSceneWitness {
            document_id: self.document_id,
            host_id: &self.host_id,
            window_id: &self.window_id,
            window_generation: self.window_generation,
            component_generation: self.component_generation,
            key: self.key.borrowed(),
            kind: self.kind,
            surface_id: &self.surface_id,
        }
    }
}

impl From<ui_wgpu::wgpu::reconcile::UiRetiredComponentScene> for ScenePointerTarget {
    fn from(retired: ui_wgpu::wgpu::reconcile::UiRetiredComponentScene) -> Self {
        Self {
            document_id: retired.document_id,
            host_id: retired.host_id,
            window_id: retired.window_id,
            surface_id: retired.surface_id,
            kind: retired.kind,
            node: retired.node,
            key: retired.key,
            window_generation: retired.window_generation,
            component_generation: retired.component_generation,
        }
    }
}

#[cfg(any(test, feature = "testkit"))]
pub fn fixture_scene_pointer_target(window_id: &str, surface_id: &str, key: &str) -> ScenePointerTarget {
    let mut arena = ui_wgpu::wgpu::arena::Arena::new();
    ScenePointerTarget {
        document_id: ui_contract::UiNodeId(1),
        host_id: surface_id.into(),
        window_id: window_id.into(),
        surface_id: surface_id.into(),
        kind: ui_wgpu::wgpu::SurfaceKind::TiledMap,
        node: arena.insert(()),
        key: ui_wgpu::wgpu::NodeKey::Explicit(key.into()),
        window_generation: 1,
        component_generation: 1,
    }
}

/// 🖼️ Why one image was not queued for upload ([`crate::scenes::queue_canvas_image_upload_with`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RasterUploadRefusal {
    /// ⏳️ Back-pressure — no free slot, the process ledger at capacity, an owner still retiring; offer the source again later.
    Busy,
    /// 🚫️ The source's own refusal (undecodable, oversized) — a per-image outcome to keep, never to re-offer.
    Invalid(&'static str),
}
