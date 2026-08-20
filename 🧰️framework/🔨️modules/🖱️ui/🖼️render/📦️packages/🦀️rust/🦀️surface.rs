//! @emoji 🗺️ The embedded product-`Surface` contract: placement, snapshot, intent, deadline.
//!
//! The escape hatch for content the generic widget vocabulary should never try to express — a 3D
//! world, a node graph, a text editor, a map, a paint canvas. UI core knows only [`SurfacePlacement`]
//! (where a surface sits) and never what it contains; that is what stops [`ui_contract::UiNodeRecord`]
//! from growing a `World3d` variant and what lets a surface be replaced or disabled without
//! recompiling core widgets. Replaces the wgpu-old target's `scene_slots.rs` (`SceneHost`/`SceneSlot`/
//! `SlotContent`, borrowed straight out of the retained `UiTree`) with a product-agnostic trait this
//! crate can own without depending on any product.
//!
//! ## Two separate erasure decisions (ruling U3 — zero `dyn` on first-party traits)
//!
//! **[`Surface`] itself carries no erasure at all.** Its two associated types (`Snapshot`, `Intent`)
//! are exactly the shape [`crate::element::Element`]'s `LayoutState`/`PrepaintState` already are in
//! this crate — a product crate that owns a single concrete surface kind implements and drives it
//! directly through generics, no `dyn` in sight.
//!
//! **[`AnySurface`]/[`SurfaceRegistry`] erase multiple *different* surface kinds into one collection**
//! (a document can show a world3d viewport and a node-graph editor at once), which a bare generic
//! cannot do. The packet brief offers two non-`dyn` mechanisms for this: a closed enum over the known
//! kinds, or fn-pointer thunks. **This file picks fn-pointer thunks** — the same "fn-pointer vtable +
//! safe `Box<dyn Any>` storage" technique [`crate::element::AnyElement`] already established in this
//! crate, `dyn Any` being explicitly U3-permitted since it is not a first-party trait. A closed enum is
//! the wrong tool *here* for the same reason [`crate::backend::GraphicsBackend`]'s docstring gives for
//! not aliasing [`crate::backend::ActiveBackend`] to a real backend in this crate: every concrete
//! surface implementation lives in a product crate that *depends on* this one, so this crate can never
//! name them in an enum without inverting that dependency graph. [`SurfaceRegistry::register`] lets
//! each product crate hand this crate a monomorphized `fn(SurfaceKind) -> AnySurface` thunk instead —
//! the enum's closed set exists, just one layer up, exactly where `ActiveBackend`'s real `cfg` aliases
//! belong (`semio-framework-ui-host`, per that file's docstring).

use crate::backend::{DeviceCapabilities, PhysicalSize};
use crate::element::Bounds;
use crate::resource::ResourceRegistry;
use crate::schedule::Deadline;
use crate::scene::SceneBuilder;
use std::any::Any;
use std::collections::HashMap;
use std::rc::Rc;
use ui_contract::{SurfaceId, SurfaceKind};

//#region 🔖️Placement

/// 🆔️ An opaque handle to whichever clip region currently bounds a surface, minted and interpreted by
/// whatever clip system a host layers on top (packet `render-dispatch`'s dispatch tree, ultimately).
/// This crate never resolves a `ClipId` to geometry — it only carries the value faithfully from
/// [`SurfacePlacement`] into [`SurfaceRenderCx`], the same "decision elsewhere, plumbing here" shape
/// [`crate::scene::StencilPolicy`] already has relative to a device-level stencil state.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ClipId(pub u32);

impl ClipId {
    /// 🌳️ The unclipped root region — every surface starts here unless something nests it under a
    /// tighter clip.
    pub const ROOT: Self = Self(0);
}

/// 🔀️ A 2D affine transform: `matrix = [a, b, c, d, tx, ty]`, applied as `x' = a*x + c*y + tx`,
/// `y' = b*x + d*y + ty` — the standard CSS/SVG `matrix()` convention. Reused rather than a full 4x4
/// (this crate's [`crate::scene::SurfacePass::view_proj`] already covers 3D) since a placed 2D surface
/// only ever needs translate/scale/rotate/skew in its own plane.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform2D {
    pub matrix: [f32; 6],
}

impl Transform2D {
    pub const IDENTITY: Self = Self { matrix: [1.0, 0.0, 0.0, 1.0, 0.0, 0.0] };

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub const fn translation(tx: f32, ty: f32) -> Self {
        Self { matrix: [1.0, 0.0, 0.0, 1.0, tx, ty] }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn apply_point(&self, point: [f32; 2]) -> [f32; 2] {
        let [a, b, c, d, tx, ty] = self.matrix;
        [a * point[0] + c * point[1] + tx, b * point[0] + d * point[1] + ty]
    }

    /// 🔁️ `None` for a singular matrix (zero determinant) — a degenerate transform an upstream layout
    /// bug produced, never silently treated as identity.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn invert(&self) -> Option<Self> {
        let [a, b, c, d, tx, ty] = self.matrix;
        let det = a * d - b * c;
        if det.abs() < f32::EPSILON {
            return None;
        }
        let inv_det = 1.0 / det;
        let (ia, ib, ic, id) = (d * inv_det, -b * inv_det, -c * inv_det, a * inv_det);
        let itx = -(ia * tx + ic * ty);
        let ity = -(ib * tx + id * ty);
        Some(Self { matrix: [ia, ib, ic, id, itx, ity] })
    }
}

/// 📍 Where one embedded product surface sits this frame — the entirety of what UI core knows about
/// it. `bounds`/`clip`/`z_index` place it among ordinary elements; `transform` covers a surface that
/// is itself panned/scaled/rotated in its own plane (independent of `bounds`, which stays axis-aligned
/// window space); `id` ties it back to the [`ui_contract::SurfaceProps`] that requested it.
#[derive(Clone, Debug, PartialEq)]
pub struct SurfacePlacement {
    pub id: SurfaceId,
    pub bounds: Bounds,
    pub clip: ClipId,
    pub transform: Transform2D,
    pub z_index: i32,
}

/// 🎯️ `None` when `window_point` falls outside `placement.bounds` — the "input outside the bounds is
/// not routed" half of this file's contract. Otherwise the point in surface-local coordinates: the
/// bounds offset is subtracted first, then `placement.transform`'s inverse is applied — a singular
/// transform (see [`Transform2D::invert`]) falls back to identity rather than dropping the event.
/// **Deliberately bounds-only, not clip-aware:** full transformed-bounds/clip-aware hit testing over a
/// whole tree is [`crate::dispatch`]'s job (packet `render-dispatch`, not landed); this is the minimal
/// per-surface primitive that job composes on top of.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn route_pointer_event(placement: &SurfacePlacement, window_point: [f32; 2]) -> Option<[f32; 2]> {
    let Bounds { x, y, w, h } = placement.bounds;
    if window_point[0] < x || window_point[0] > x + w || window_point[1] < y || window_point[1] > y + h {
        return None;
    }
    let relative = [window_point[0] - x, window_point[1] - y];
    let inverse = placement.transform.invert().unwrap_or(Transform2D::IDENTITY);
    Some(inverse.apply_point(relative))
}

//#endregion 🔖️Placement

//#region 🔖️Surface

/// 🎯️ What [`Surface::render`] wants painted into: `Inline` appends straight into the caller's shared
/// [`SceneBuilder`] (an anchored [`crate::scene::SurfacePass`] for a 3D world, ordinary quads for a
/// 2D canvas); `Offscreen` asks for a dedicated render target of `size` physical pixels a host
/// composites back in afterward (a paint canvas that must stay pixel-stable while the rest of the
/// window repaints around it).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SurfaceRenderTarget {
    Inline,
    Offscreen { size: PhysicalSize },
}

/// 📦️ A coarse, backend-agnostic count of resources [`Surface::render`] expects to touch this frame —
/// enough for a host to size an upload budget or defer a heavy surface without decoding
/// [`crate::resource::ResourceOp`]s it has not been asked to apply yet.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SurfaceResourceNeeds {
    pub textures: u32,
    pub meshes: u32,
}

impl SurfaceResourceNeeds {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub const fn is_empty(&self) -> bool {
        self.textures == 0 && self.meshes == 0
    }
}

/// 📋️ [`Surface::prepare`]'s report, ahead of the [`Surface::render`] call it precedes: whether the
/// surface actually needs repainting this frame, what kind of target it wants, and a rough resource
/// budget.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SurfacePrepare {
    pub dirty: bool,
    pub target: SurfaceRenderTarget,
    pub needs: SurfaceResourceNeeds,
}

/// 🖌️ Everything [`Surface::render`] gets, in this crate's own backend-neutral terms — a
/// [`SceneBuilder`] to append into and a [`ResourceRegistry`] to intern/request uploads through, both
/// already shared with the rest of the frame. No device, no `wgpu`, no `winit`: exactly the same
/// "decisions here, device plumbing in a backend crate" split [`crate::element::PaintCx`] already
/// draws for ordinary elements.
pub struct SurfaceRenderCx<'a> {
    pub scene: &'a mut SceneBuilder,
    pub resources: &'a mut ResourceRegistry,
    pub placement: &'a SurfacePlacement,
    pub time_seconds: f32,
}

/// 🖱️ Which pointer button an input event names. Re-exported from the dispatch layer rather than
/// redefined: a surface receives the *same* button identity generic dispatch resolved, and two
/// structurally identical enums would let them silently drift apart.
pub use crate::PointerButton;

/// 📥️ One input event routed to a surface, already in surface-local coordinates (see
/// [`route_pointer_event`]) — generic UI dispatch never learns surface semantics beyond this shape.
/// `key` is a logical key name (the same open vocabulary `KeyboardEvent.key` uses), never a
/// platform/backend key code, since this crate has no `winit` dependency to borrow one from.
#[derive(Clone, Debug, PartialEq)]
pub enum SurfaceInput {
    PointerMoved { local: [f32; 2] },
    PointerDown { local: [f32; 2], button: PointerButton },
    PointerUp { local: [f32; 2], button: PointerButton },
    Scroll { local: [f32; 2], delta: [f32; 2] },
    KeyDown { key: String },
    KeyUp { key: String },
    TextInput { text: String },
}

/// ⚠️ Why a [`Surface`] could not be resolved or could not paint. `Unregistered` is the explicit
/// degrade path [`SurfaceRegistry::create`] takes for a kind nothing registered — never a silent
/// blank; see this file's top docstring and [`SurfaceRegistry`]'s.
#[derive(Clone, Debug, PartialEq)]
pub enum SurfaceError {
    Unregistered(SurfaceKind),
    ResourceUnavailable(String),
    Internal(String),
}

/// 🗺️ One embedded product surface's lifecycle. Carries zero `dyn` (see this file's top docstring): a
/// product crate implements this directly on its own concrete type and drives it through ordinary
/// generics — [`AnySurface`] only exists for a *host* that must hold several different kinds together.
pub trait Surface {
    type Snapshot: 'static;
    type Intent: 'static;

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn update_snapshot(&mut self, snapshot: Rc<Self::Snapshot>);

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn prepare(&mut self, placement: &SurfacePlacement, caps: &DeviceCapabilities) -> SurfacePrepare;

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn render(&mut self, cx: &mut SurfaceRenderCx<'_>) -> Result<(), SurfaceError>;

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn handle_input(&mut self, event: &SurfaceInput, placement: &SurfacePlacement) -> Vec<Self::Intent>;

    /// ⏰️ The surface's own contribution to the zero-idle-frames guarantee: an animating camera
    /// returns its next due time so [`crate::schedule::FrameScheduler`] wakes exactly then; a still
    /// surface returns `None` and costs nothing between real changes.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn next_deadline(&self) -> Option<Deadline>;
}

/// 🖌️ Applies `placement.bounds` as a scissor around one [`Surface::render`] call, so a surface can
/// never paint outside where UI core placed it and two placements never merge into one scissored
/// batch. Rendering placements in ascending `z_index` order across a frame is the *caller's*
/// discipline — this crate already paints in call order everywhere else ([`SceneBuilder`]'s own
/// docstring), so ordering the calls is what encodes z-order in the emitted
/// [`crate::scene::RenderPacket::batches`]; nothing here maintains a separate z-buffer.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn render_placed_surface<S: Surface>(surface: &mut S, placement: &SurfacePlacement, scene: &mut SceneBuilder, resources: &mut ResourceRegistry, time_seconds: f32) -> Result<(), SurfaceError> {
    scene.push_scissor(placement.bounds);
    let mut cx = SurfaceRenderCx { scene, resources, placement, time_seconds };
    let result = surface.render(&mut cx);
    scene.pop_scissor();
    result
}

/// 🖱️ Routes a pointer-moved `window_point` to `surface` when it lands inside `placement.bounds`,
/// converting to surface-local coordinates first (see [`route_pointer_event`]); an out-of-bounds point
/// never reaches [`Surface::handle_input`] and yields no intents.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn dispatch_pointer_moved<S: Surface>(surface: &mut S, placement: &SurfacePlacement, window_point: [f32; 2]) -> Vec<S::Intent> {
    match route_pointer_event(placement, window_point) {
        Some(local) => surface.handle_input(&SurfaceInput::PointerMoved { local }, placement),
        None => Vec::new(),
    }
}

//#endregion 🔖️Surface

//#region 🔖️Registry

type ErasedUpdateSnapshotFn = fn(&mut dyn Any, Rc<dyn Any>);
type ErasedPrepareFn = fn(&mut dyn Any, &SurfacePlacement, &DeviceCapabilities) -> SurfacePrepare;
type ErasedRenderFn = fn(&mut dyn Any, &mut SurfaceRenderCx<'_>) -> Result<(), SurfaceError>;
type ErasedHandleInputFn = fn(&mut dyn Any, &SurfaceInput, &SurfacePlacement) -> Vec<Box<dyn Any>>;
type ErasedNextDeadlineFn = fn(&dyn Any) -> Option<Deadline>;

/// 🧬️ Five monomorphized `fn` items per concrete `S: Surface`, stored by value — the same
/// fn-pointer-vtable technique [`crate::element::AnyElement`]'s own `ElementVTable` uses, applied to
/// [`Surface`]'s five methods instead of [`crate::element::Element`]'s three.
#[derive(Clone, Copy)]
struct SurfaceVTable {
    update_snapshot: ErasedUpdateSnapshotFn,
    prepare: ErasedPrepareFn,
    render: ErasedRenderFn,
    handle_input: ErasedHandleInputFn,
    next_deadline: ErasedNextDeadlineFn,
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn update_snapshot_erased<S: Surface + 'static>(surface: &mut dyn Any, snapshot: Rc<dyn Any>) {
    let surface = surface.downcast_mut::<S>().expect("AnySurface: vtable/surface type mismatch — see AnySurface::new");
    let snapshot = snapshot.downcast::<S::Snapshot>().unwrap_or_else(|_| panic!("AnySurface: snapshot type mismatch for this surface kind"));
    surface.update_snapshot(snapshot);
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn prepare_erased<S: Surface + 'static>(surface: &mut dyn Any, placement: &SurfacePlacement, caps: &DeviceCapabilities) -> SurfacePrepare {
    let surface = surface.downcast_mut::<S>().expect("AnySurface: vtable/surface type mismatch — see AnySurface::new");
    surface.prepare(placement, caps)
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn render_erased<S: Surface + 'static>(surface: &mut dyn Any, cx: &mut SurfaceRenderCx<'_>) -> Result<(), SurfaceError> {
    let surface = surface.downcast_mut::<S>().expect("AnySurface: vtable/surface type mismatch — see AnySurface::new");
    surface.render(cx)
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn handle_input_erased<S: Surface + 'static>(surface: &mut dyn Any, event: &SurfaceInput, placement: &SurfacePlacement) -> Vec<Box<dyn Any>> {
    let surface = surface.downcast_mut::<S>().expect("AnySurface: vtable/surface type mismatch — see AnySurface::new");
    surface.handle_input(event, placement).into_iter().map(|intent| Box::new(intent) as Box<dyn Any>).collect()
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn next_deadline_erased<S: Surface + 'static>(surface: &dyn Any) -> Option<Deadline> {
    let surface = surface.downcast_ref::<S>().expect("AnySurface: vtable/surface type mismatch — see AnySurface::new");
    surface.next_deadline()
}

/// 🧱️ One type-erased [`Surface`], its kind and its erasure vtable. Never `dyn Surface` (ruling U3) —
/// see this file's top docstring. `handle_input` returns `Vec<Box<dyn Any>>`: a caller downcasts each
/// intent using the `Self::Intent` type it already knows for `kind()`, exactly the same "erase in,
/// downcast by whoever registered the kind" contract [`SurfaceRegistry::register`] establishes for
/// construction.
pub struct AnySurface {
    kind: SurfaceKind,
    inner: Box<dyn Any>,
    vtable: SurfaceVTable,
}

impl AnySurface {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn new<S: Surface + 'static>(kind: SurfaceKind, surface: S) -> Self {
        Self {
            kind,
            inner: Box::new(surface),
            vtable: SurfaceVTable {
                update_snapshot: update_snapshot_erased::<S>,
                prepare: prepare_erased::<S>,
                render: render_erased::<S>,
                handle_input: handle_input_erased::<S>,
                next_deadline: next_deadline_erased::<S>,
            },
        }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn kind(&self) -> SurfaceKind {
        self.kind
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn update_snapshot(&mut self, snapshot: Rc<dyn Any>) {
        (self.vtable.update_snapshot)(self.inner.as_mut(), snapshot);
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn prepare(&mut self, placement: &SurfacePlacement, caps: &DeviceCapabilities) -> SurfacePrepare {
        (self.vtable.prepare)(self.inner.as_mut(), placement, caps)
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn render(&mut self, cx: &mut SurfaceRenderCx<'_>) -> Result<(), SurfaceError> {
        (self.vtable.render)(self.inner.as_mut(), cx)
    }

    /// 🖌️ [`render_placed_surface`]'s equivalent for an already-erased surface.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn render_placed(&mut self, placement: &SurfacePlacement, scene: &mut SceneBuilder, resources: &mut ResourceRegistry, time_seconds: f32) -> Result<(), SurfaceError> {
        scene.push_scissor(placement.bounds);
        let mut cx = SurfaceRenderCx { scene, resources, placement, time_seconds };
        let result = self.render(&mut cx);
        scene.pop_scissor();
        result
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn handle_input(&mut self, event: &SurfaceInput, placement: &SurfacePlacement) -> Vec<Box<dyn Any>> {
        (self.vtable.handle_input)(self.inner.as_mut(), event, placement)
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn next_deadline(&self) -> Option<Deadline> {
        (self.vtable.next_deadline)(self.inner.as_ref())
    }
}

const PLACEHOLDER_FILL: [f32; 4] = [0.85, 0.2, 0.2, 0.35];
const PLACEHOLDER_RADIUS: f32 = 4.0;

/// 🚧️ [`SurfaceRegistry`]'s explicit degrade for a [`SurfaceKind`] nothing registered: a visible tinted
/// rounded rect plus a matchable [`SurfaceError::Unregistered`] from every call, so a missing renderer
/// is loud rather than reading as a broken document (this file's top docstring, and master.md's own
/// framing of the failure mode this replaces).
struct PlaceholderSurface {
    kind: SurfaceKind,
}

impl Surface for PlaceholderSurface {
    type Snapshot = ();
    type Intent = ();

    fn update_snapshot(&mut self, _snapshot: Rc<Self::Snapshot>) {}

    fn prepare(&mut self, _placement: &SurfacePlacement, _caps: &DeviceCapabilities) -> SurfacePrepare {
        SurfacePrepare { dirty: true, target: SurfaceRenderTarget::Inline, needs: SurfaceResourceNeeds::default() }
    }

    fn render(&mut self, cx: &mut SurfaceRenderCx<'_>) -> Result<(), SurfaceError> {
        let Bounds { x, y, w, h } = cx.placement.bounds;
        cx.scene.push_rounded([x, y, w, h], PLACEHOLDER_FILL, PLACEHOLDER_RADIUS);
        Err(SurfaceError::Unregistered(self.kind))
    }

    fn handle_input(&mut self, _event: &SurfaceInput, _placement: &SurfacePlacement) -> Vec<Self::Intent> {
        Vec::new()
    }

    fn next_deadline(&self) -> Option<Deadline> {
        None
    }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn make_surface<S: Surface + Default + 'static>(kind: SurfaceKind) -> AnySurface {
    AnySurface::new(kind, S::default())
}

/// 🗂️ Resolves a [`SurfaceKind`] (read off the contract's [`ui_contract::SurfaceProps::kind`]) to an
/// implementation. Holds one `fn(SurfaceKind) -> AnySurface` thunk per registered kind — see this
/// file's top docstring for why a thunk table stands in for a closed enum here. An unrecognized kind
/// never returns a blank: [`Self::create`] hands back a working [`PlaceholderSurface`] plus the exact
/// [`SurfaceError::Unregistered`] that explains why.
#[derive(Default)]
pub struct SurfaceRegistry {
    factories: HashMap<SurfaceKind, fn(SurfaceKind) -> AnySurface>,
}

impl SurfaceRegistry {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn new() -> Self {
        Self::default()
    }

    /// ➕️ Registers `S` as the implementation for `kind`. `S: Default` is the whole construction
    /// contract: a fresh surface starts empty and receives its real content through
    /// [`Surface::update_snapshot`] on the next frame, the same lifecycle every other retained-per-id
    /// state in this crate follows ([`crate::element::RetainedStore`]'s `get_or_insert_with`).
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn register<S: Surface + Default + 'static>(&mut self, kind: SurfaceKind) {
        self.factories.insert(kind, make_surface::<S>);
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn is_registered(&self, kind: SurfaceKind) -> bool {
        self.factories.contains_key(&kind)
    }

    /// 🏗️ Builds a fresh surface for `kind`. `Some(SurfaceError::Unregistered(kind))` alongside a
    /// working placeholder is this registry's explicit degrade path — never a panic, never a blank.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn create(&self, kind: SurfaceKind) -> (AnySurface, Option<SurfaceError>) {
        match self.factories.get(&kind) {
            Some(factory) => (factory(kind), None),
            None => (AnySurface::new(kind, PlaceholderSurface { kind }), Some(SurfaceError::Unregistered(kind))),
        }
    }
}

//#endregion 🔖️Registry

//#region Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene::{DrawBatch, FinishParams, PipelineKind, Scene, ScissorRect};
    use crate::schedule::InvalidationReason;

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn test_placement(id: &str, bounds: Bounds, clip: ClipId, z_index: i32) -> SurfacePlacement {
        SurfacePlacement { id: SurfaceId(id.into()), bounds, clip, transform: Transform2D::IDENTITY, z_index }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn test_caps() -> DeviceCapabilities {
        DeviceCapabilities {
            max_texture_dimension: 4096,
            max_bind_groups: 4,
            supports_msaa: false,
            supports_timestamp_queries: false,
            supports_storage_buffers: false,
            preferred_surface_format: crate::backend::SurfaceFormat::Rgba8UnormSrgb,
            memory_class: crate::backend::MemoryClass::Standard,
            gpu_tier: crate::backend::GpuTier::Integrated,
        }
    }

    //#region Placement

    #[test]
    fn clip_id_root_is_a_distinct_stable_value() {
        assert_eq!(ClipId::ROOT, ClipId(0));
    }

    #[test]
    fn transform2d_identity_inverts_to_itself() {
        assert_eq!(Transform2D::IDENTITY.invert(), Some(Transform2D::IDENTITY));
    }

    #[test]
    fn transform2d_apply_point_translates() {
        assert_eq!(Transform2D::translation(5.0, -3.0).apply_point([1.0, 1.0]), [6.0, -2.0]);
    }

    #[test]
    fn route_pointer_event_returns_none_outside_bounds_and_local_coordinates_inside() {
        let placement = test_placement("s", Bounds::new(100.0, 50.0, 200.0, 100.0), ClipId::ROOT, 0);
        assert_eq!(route_pointer_event(&placement, [10.0, 10.0]), None, "a point outside the surface's bounds must not route");
        assert_eq!(route_pointer_event(&placement, [150.0, 80.0]), Some([50.0, 30.0]), "a point inside must convert to surface-local coordinates");
    }

    #[test]
    fn route_pointer_event_applies_the_inverse_transform_after_the_bounds_offset() {
        let placement = test_placement("s", Bounds::new(0.0, 0.0, 100.0, 100.0), ClipId::ROOT, 0);
        let placement = SurfacePlacement { transform: Transform2D::translation(10.0, 20.0), ..placement };
        let local = route_pointer_event(&placement, [50.0, 50.0]).expect("inside bounds");
        assert_eq!(local, [40.0, 30.0]);
    }

    //#endregion Placement

    //#region SurfaceLifecycle

    #[derive(Default)]
    struct RevisionSnapshot {
        revision: u64,
    }

    #[derive(Default)]
    struct RevisionSurface {
        snapshot_revision: u64,
        rendered_revision: Option<u64>,
    }

    impl Surface for RevisionSurface {
        type Snapshot = RevisionSnapshot;
        type Intent = ();

        fn update_snapshot(&mut self, snapshot: Rc<Self::Snapshot>) {
            self.snapshot_revision = snapshot.revision;
        }

        fn prepare(&mut self, _placement: &SurfacePlacement, _caps: &DeviceCapabilities) -> SurfacePrepare {
            let dirty = self.rendered_revision != Some(self.snapshot_revision);
            SurfacePrepare { dirty, target: SurfaceRenderTarget::Inline, needs: SurfaceResourceNeeds::default() }
        }

        fn render(&mut self, _cx: &mut SurfaceRenderCx<'_>) -> Result<(), SurfaceError> {
            self.rendered_revision = Some(self.snapshot_revision);
            Ok(())
        }

        fn handle_input(&mut self, _event: &SurfaceInput, _placement: &SurfacePlacement) -> Vec<Self::Intent> {
            Vec::new()
        }

        fn next_deadline(&self) -> Option<Deadline> {
            None
        }
    }

    #[test]
    fn a_snapshot_revision_that_has_not_changed_does_not_mark_the_surface_dirty() {
        let placement = test_placement("s", Bounds::default(), ClipId::ROOT, 0);
        let caps = test_caps();
        let mut surface = RevisionSurface::default();

        surface.update_snapshot(Rc::new(RevisionSnapshot { revision: 1 }));
        assert!(surface.prepare(&placement, &caps).dirty, "a never-rendered surface must report dirty");

        let mut scene = SceneBuilder::default();
        let mut resources = ResourceRegistry::default();
        render_placed_surface(&mut surface, &placement, &mut scene, &mut resources, 0.0).expect("render");
        assert!(!surface.prepare(&placement, &caps).dirty, "an unchanged revision right after a render must not be dirty");

        surface.update_snapshot(Rc::new(RevisionSnapshot { revision: 1 }));
        assert!(!surface.prepare(&placement, &caps).dirty, "re-applying the same revision must not mark dirty");

        surface.update_snapshot(Rc::new(RevisionSnapshot { revision: 2 }));
        assert!(surface.prepare(&placement, &caps).dirty, "a changed revision must mark dirty again");
    }

    #[derive(Default)]
    struct DeadlineSurface {
        deadline: Option<Deadline>,
    }

    impl Surface for DeadlineSurface {
        type Snapshot = ();
        type Intent = ();

        fn update_snapshot(&mut self, _snapshot: Rc<Self::Snapshot>) {}

        fn prepare(&mut self, _placement: &SurfacePlacement, _caps: &DeviceCapabilities) -> SurfacePrepare {
            SurfacePrepare { dirty: false, target: SurfaceRenderTarget::Inline, needs: SurfaceResourceNeeds::default() }
        }

        fn render(&mut self, _cx: &mut SurfaceRenderCx<'_>) -> Result<(), SurfaceError> {
            Ok(())
        }

        fn handle_input(&mut self, _event: &SurfaceInput, _placement: &SurfacePlacement) -> Vec<Self::Intent> {
            Vec::new()
        }

        fn next_deadline(&self) -> Option<Deadline> {
            self.deadline
        }
    }

    #[test]
    fn an_animating_surfaces_next_deadline_reaches_the_scheduler_while_a_still_one_yields_none() {
        let animating = DeadlineSurface { deadline: Some(Deadline { due: 1.5, reason: InvalidationReason::ANIMATION }) };
        let still = DeadlineSurface::default();
        assert_eq!(animating.next_deadline(), Some(Deadline { due: 1.5, reason: InvalidationReason::ANIMATION }));
        assert_eq!(still.next_deadline(), None);
    }

    //#endregion SurfaceLifecycle

    //#region Input

    #[derive(Default)]
    struct RecordingInputSurface {
        received: Vec<[f32; 2]>,
    }

    impl Surface for RecordingInputSurface {
        type Snapshot = ();
        type Intent = [f32; 2];

        fn update_snapshot(&mut self, _snapshot: Rc<Self::Snapshot>) {}

        fn prepare(&mut self, _placement: &SurfacePlacement, _caps: &DeviceCapabilities) -> SurfacePrepare {
            SurfacePrepare { dirty: false, target: SurfaceRenderTarget::Inline, needs: SurfaceResourceNeeds::default() }
        }

        fn render(&mut self, _cx: &mut SurfaceRenderCx<'_>) -> Result<(), SurfaceError> {
            Ok(())
        }

        fn handle_input(&mut self, event: &SurfaceInput, _placement: &SurfacePlacement) -> Vec<Self::Intent> {
            match *event {
                SurfaceInput::PointerMoved { local } => {
                    self.received.push(local);
                    vec![local]
                }
                _ => Vec::new(),
            }
        }

        fn next_deadline(&self) -> Option<Deadline> {
            None
        }
    }

    #[test]
    fn dispatch_pointer_moved_reaches_the_surface_only_when_inside_bounds_and_in_local_coordinates() {
        let placement = test_placement("s", Bounds::new(0.0, 0.0, 100.0, 100.0), ClipId::ROOT, 0);
        let mut surface = RecordingInputSurface::default();

        let outside = dispatch_pointer_moved(&mut surface, &placement, [500.0, 500.0]);
        assert!(outside.is_empty(), "an out-of-bounds pointer event must not reach the surface");
        assert!(surface.received.is_empty());

        let inside = dispatch_pointer_moved(&mut surface, &placement, [10.0, 20.0]);
        assert_eq!(inside, vec![[10.0, 20.0]]);
        assert_eq!(surface.received, vec![[10.0, 20.0]]);
    }

    //#endregion Input

    //#region PlacementAndZOrder

    struct PaintingSurface {
        color: [f32; 4],
        seen_clip: Option<ClipId>,
    }

    impl PaintingSurface {
        // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
        fn with_color(color: [f32; 4]) -> Self {
            Self { color, seen_clip: None }
        }
    }

    impl Surface for PaintingSurface {
        type Snapshot = ();
        type Intent = ();

        fn update_snapshot(&mut self, _snapshot: Rc<Self::Snapshot>) {}

        fn prepare(&mut self, _placement: &SurfacePlacement, _caps: &DeviceCapabilities) -> SurfacePrepare {
            SurfacePrepare { dirty: true, target: SurfaceRenderTarget::Inline, needs: SurfaceResourceNeeds::default() }
        }

        fn render(&mut self, cx: &mut SurfaceRenderCx<'_>) -> Result<(), SurfaceError> {
            self.seen_clip = Some(cx.placement.clip);
            let Bounds { x, y, w, h } = cx.placement.bounds;
            cx.scene.push_solid([x, y, w, h], self.color);
            Ok(())
        }

        fn handle_input(&mut self, _event: &SurfaceInput, _placement: &SurfacePlacement) -> Vec<Self::Intent> {
            Vec::new()
        }

        fn next_deadline(&self) -> Option<Deadline> {
            None
        }
    }

    #[test]
    fn render_placed_surface_scissors_by_bounds_preserves_call_order_for_z_and_passes_clip_through() {
        let mut scene = SceneBuilder::default();
        let mut resources = ResourceRegistry::default();
        let back = test_placement("back", Bounds::new(0.0, 0.0, 50.0, 50.0), ClipId::ROOT, 0);
        let front = test_placement("front", Bounds::new(10.0, 10.0, 20.0, 20.0), ClipId(7), 1);
        let mut back_surface = PaintingSurface::with_color([1.0, 0.0, 0.0, 1.0]);
        let mut front_surface = PaintingSurface::with_color([0.0, 1.0, 0.0, 1.0]);

        render_placed_surface(&mut back_surface, &back, &mut scene, &mut resources, 0.0).expect("back renders");
        render_placed_surface(&mut front_surface, &front, &mut scene, &mut resources, 0.0).expect("front renders");

        assert_eq!(back_surface.seen_clip, Some(ClipId::ROOT), "the placement's clip must reach the render call unchanged");
        assert_eq!(front_surface.seen_clip, Some(ClipId(7)), "each placement's own clip must reach its own render call, not the previous one's");

        let packet = Scene::finish(scene, FinishParams { viewport: [100.0, 100.0], dpr: 1.0, time_seconds_origin: 0.0, resource_ops: Vec::new() }).expect("finish");
        let quad_batches: Vec<&DrawBatch> = packet.batches.iter().filter(|batch| batch.pipeline == PipelineKind::UiQuad).collect();
        assert_eq!(quad_batches.len(), 2, "two differently-scissored surfaces must not merge into one batch");
        assert_eq!(quad_batches[0].layer_state.scissor, Some(ScissorRect::from_rect(back.bounds)), "the first-rendered (lower z) placement's bounds must be the first batch's scissor");
        assert_eq!(quad_batches[1].layer_state.scissor, Some(ScissorRect::from_rect(front.bounds)), "the second-rendered (higher z) placement's bounds must be the second batch's scissor, proving call order encodes z-order");
    }

    #[test]
    fn surface_resource_needs_is_empty_reports_correctly() {
        assert!(SurfaceResourceNeeds::default().is_empty());
        assert!(!SurfaceResourceNeeds { textures: 1, meshes: 0 }.is_empty());
    }

    //#endregion PlacementAndZOrder

    //#region Registry

    #[test]
    fn registering_a_kind_makes_create_resolve_it_without_an_error() {
        let mut registry = SurfaceRegistry::new();
        registry.register::<DeadlineSurface>(SurfaceKind::World3d);
        assert!(registry.is_registered(SurfaceKind::World3d));

        let (surface, error) = registry.create(SurfaceKind::World3d);
        assert!(error.is_none());
        assert_eq!(surface.kind(), SurfaceKind::World3d);
    }

    #[test]
    fn an_unregistered_kind_produces_a_visible_placeholder_plus_an_error_rather_than_silence() {
        let registry = SurfaceRegistry::new();
        let (mut surface, creation_error) = registry.create(SurfaceKind::World3d);
        assert!(matches!(creation_error, Some(SurfaceError::Unregistered(SurfaceKind::World3d))), "an unresolved kind must report a clear error, not silently succeed");

        let placement = test_placement("s", Bounds::new(0.0, 0.0, 10.0, 10.0), ClipId::ROOT, 0);
        let mut scene = SceneBuilder::default();
        let mut resources = ResourceRegistry::default();
        let render_result = surface.render_placed(&placement, &mut scene, &mut resources, 0.0);
        assert!(matches!(render_result, Err(SurfaceError::Unregistered(SurfaceKind::World3d))), "rendering the placeholder must keep reporting the same error");

        let packet = Scene::finish(scene, FinishParams { viewport: [50.0, 50.0], dpr: 1.0, time_seconds_origin: 0.0, resource_ops: Vec::new() }).expect("finish");
        assert!(!packet.batches.is_empty(), "an unregistered surface must still paint a visible placeholder, never a silent blank");
    }

    #[test]
    fn handle_input_through_any_surface_erases_and_boxes_the_concrete_intent_type() {
        let mut registry = SurfaceRegistry::new();
        registry.register::<RecordingInputSurface>(SurfaceKind::NodeGraph);
        let (mut surface, error) = registry.create(SurfaceKind::NodeGraph);
        assert!(error.is_none());

        let placement = test_placement("s", Bounds::new(0.0, 0.0, 100.0, 100.0), ClipId::ROOT, 0);
        let intents = surface.handle_input(&SurfaceInput::PointerMoved { local: [4.0, 5.0] }, &placement);
        assert_eq!(intents.len(), 1);
        let intent = intents.into_iter().next().expect("one intent").downcast::<[f32; 2]>().expect("RecordingInputSurface::Intent is [f32; 2]");
        assert_eq!(*intent, [4.0, 5.0]);
    }

    //#endregion Registry
}

//#endregion Tests
