//! 🤝️ Retained UI engine cell: the per-worker retained UI engine, its document close queue and the scene pointer owners, lifted out of the Interpreter for the `contracts` crate of the renderer partition (plan section 5.2). The cell, its guard and the close queue keep their exact layout: they carry the reentrancy law the worker-cell test pins.
use super::*;

//#region 🧩️RetainedUiEngine
/* 🧵️ The wave-3 cutover: `render_ui_node`'s live implementation is now `ui_wgpu::wgpu::engine::Ui`
 * (retained-mode `apply_tree`/`frame`/`dispatch_event`), not `ui_node_to_widget`+`render_widget`.
 * One process-wide `Ui` façade (its own internal `HashMap<window_id, UiWindow>` already partitions
 * per-window retained state — see `report-w0-engine-facade.md`) lives in a `thread_local!`, since neither call site of
 * `render_ui_node` (`shell::ShellChrome`'s `render_window_content`/`render_floating_panel`) is in a
 * region this ticket may touch, so there was no `ShellTypes` struct field available to hang it on
 * (struct field additions there are an Integrator choke point per `region-claims.json`).
 *
 * `window_id` is a genuinely new parameter — the only way to key per-window retained state — added
 * to `render_ui_node`'s public signature. Its two real call sites (`render_window_content`,
 * `render_floating_panel`, both in the off-limits `shell::ShellChrome` region) were touched with the
 * smallest possible diff: appending the one identifier string each already has in scope
 * (`window_id`/`active_tab_id`). See the ticket report for why this was judged unavoidable rather than
 * filed as a pure wiring request: without it the crate does not compile, and no identity for the
 * retained per-window bucket can be derived from anything else already flowing into this function.
 *
 * ✅️ RESOLVED (was a KNOWN, CONFIRMED GAP): `ui_wgpu::wgpu::Ui` no longer owns a private `FontAtlas`/
 * `Option<IconAtlas>` at all — `set_icons`/the old `atlas`/`icons` fields are gone. `Ui::frame` now
 * takes `atlas: &mut FontAtlas, icons: Option<&IconAtlas>` as parameters, mirroring how
 * `flex::LayoutEngine::compute`/`paint::paint_tree` already receive them internally. `render_ui_node`
 * below passes `ctx.atlas`/`ctx.icons` straight through — the SAME `FontAtlas`/`IconAtlas` instances
 * the shell already `GpuContext::upload_font_atlas`/`upload_icon_atlas`s every frame for chrome/dock/
 * panel text — so retained-mode content now shares the one real, GPU-uploaded glyph/icon texture
 * instead of reading from (or clobbering) a second, independent one. See
 * `.🧬semio/🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY/report-w3-interpreter-cutover.md`'s "CRITICAL FINDING"
 * for the original gap and the follow-up ticket work that closed it. */
pub static UI_ENGINE: UiEngineCell = UiEngineCell::new();

/// ⛔️ The named refusal of a retained UI engine access before the shell resolved its locale.
pub const UI_ENGINE_LOCALE_UNRESOLVED: &str = "ui.engine.locale-unresolved: the retained UI engine is used before the shell resolved its locale";

/// 🧫️ The locale a law's engine starts in when the law builds no shell — a test fixture, never a production default.
#[cfg(any(test, feature = "testkit"))]
pub const TEST_UI_ENGINE_LOCALE: semio_framework_ui_locale::Locale = semio_framework_ui_locale::Locale::En;

/// 🌐️ The retained UI engine cell. There is no default language, so the engine exists only once the shell resolved its
/// locale ([`install_ui_engine_locale`]); every access before that is refused by name ([`UI_ENGINE_LOCALE_UNRESOLVED`]).
pub struct UiEngineCell(pub WorkerCell<Option<ui_wgpu::wgpu::Ui>>);

/// 🔐️ The held engine: dereferences to the `Ui`, refusing by name while none was installed.
pub struct UiEngineGuard<'a>(MutexGuard<'a, Option<ui_wgpu::wgpu::Ui>>);

impl std::ops::Deref for UiEngineGuard<'_> {
    type Target = ui_wgpu::wgpu::Ui;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref().expect(UI_ENGINE_LOCALE_UNRESOLVED)
    }
}

impl std::ops::DerefMut for UiEngineGuard<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut().expect(UI_ENGINE_LOCALE_UNRESOLVED)
    }
}

/// 🌱️ The engine before any locale: none in production; a law's fixture locale under `cfg(test)`.
fn initial_ui_engine() -> Option<ui_wgpu::wgpu::Ui> {
    #[cfg(any(test, feature = "testkit"))]
    return Some(ui_wgpu::wgpu::Ui::new(TEST_UI_ENGINE_LOCALE));
    #[cfg(not(any(test, feature = "testkit")))]
    None
}

impl UiEngineCell {
    pub const fn new() -> Self {
        Self(WorkerCell::new(initial_ui_engine))
    }

    pub fn borrow(&self) -> UiEngineGuard<'_> {
        UiEngineGuard(self.0.borrow())
    }

    pub fn borrow_mut(&self) -> UiEngineGuard<'_> {
        self.borrow()
    }

    pub fn with<R>(&self, apply: impl FnOnce(&Self) -> R) -> R {
        apply(self)
    }
}

/// 🏁️ Installs the retained UI engine in the shell's resolved locale — the first resolution builds it, every later one
/// (a `setLocale`, a preferences load) re-sets the language of its own chrome.
pub fn install_ui_engine_locale(locale: semio_framework_ui_locale::Locale) {
    let mut engine = UI_ENGINE.0.borrow();
    match engine.as_mut() {
        Some(ui) => ui.set_locale(locale),
        None => *engine = Some(ui_wgpu::wgpu::Ui::new(locale)),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UiDocumentCloseOwner {
    pub window_id: SurfaceId,
    pub token: ui_wgpu::wgpu::engine::UiSurfaceToken,
    pub generation: u64,
}

#[derive(Default)]
pub struct UiDocumentCloseQueue {
    pub owners: std::collections::VecDeque<UiDocumentCloseOwner>,
}

impl UiDocumentCloseQueue {
    pub fn try_upsert(&mut self, owner: UiDocumentCloseOwner) -> bool {
        if let Some(existing) = self.owners.iter_mut().find(|existing| existing.window_id == owner.window_id) {
            *existing = owner;
            return true;
        }
        if self.owners.len() == ui_wgpu::wgpu::engine::UI_LAYOUT_SURFACE_SLOTS {
            return false;
        }
        self.owners.push_back(owner);
        true
    }

    fn owns_window(&self, window_id: &str) -> bool {
        self.owners.iter().any(|owner| owner.window_id.as_ref() == window_id)
    }
}

pub static UI_DOCUMENT_CLOSE_QUEUE: WorkerCell<UiDocumentCloseQueue> = WorkerCell::new(Default::default);

pub fn ui_document_close_pending() -> bool {
    UI_DOCUMENT_CLOSE_QUEUE.with(|cell| !cell.borrow().owners.is_empty())
}

pub fn ui_document_close_pending_for(window_id: &str) -> bool {
    UI_DOCUMENT_CLOSE_QUEUE.with(|cell| cell.borrow().owns_window(window_id))
}

pub fn component_scene_is_presented(witness: &ui_wgpu::wgpu::reconcile::UiComponentSceneWitness<'_>) -> bool {
    UI_ENGINE.with(|cell| cell.borrow().presented_component_scene_matches(witness))
}

#[derive(Clone)]
pub struct ScenePointerOwner {
    pub pointer_id: ui_render::PointerId,
    pub target: ScenePointerTarget,
}

#[derive(Default)]
pub struct ScenePointerOwners {
    pub slots: Vec<ScenePointerOwner>,
}

pub static SCENE_POINTER_OWNERS: WorkerCell<ScenePointerOwners> = WorkerCell::new(Default::default);
//#endregion 🧩️RetainedUiEngine
