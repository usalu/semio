//! 🖱️ Canvas gestures, input kit and chrome: the canvas click and gesture records, the scene input kit and row transfer, the chrome labels, the engine hosts, the table accessibility cells and the tiled map retirement. See [`super`].

use super::*;

//#region SceneRuntime
#[derive(Clone, Copy, Debug)]
pub struct CanvasClick {
    pub x: f32,
    pub y: f32,
    pub button: i16,
    pub at_ms: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CanvasGestureKind {
    Document,
    Pan,
}

#[derive(Clone, Debug)]
pub struct CanvasGesture {
    pub pointer_id: ui_render::PointerId,
    pub kind: CanvasGestureKind,
    pub window_id: String,
    pub document_generation: u64,
    pub host_id: String,
    pub surface_id: String,
    pub controller_id: String,
    pub rect: Rect,
    pub viewport: Viewport,
    pub last_x: f32,
    pub last_y: f32,
    pub button: i16,
    pub cancellation_requested: bool,
}

pub struct CanvasGestureSlots {
    pub slots: Box<[Option<CanvasGesture>; crate::element_contracts::SCENE_POINTER_OWNER_CAPACITY]>,
}

impl Default for CanvasGestureSlots {
    fn default() -> Self {
        Self { slots: Box::new(std::array::from_fn(|_| None)) }
    }
}

impl CanvasGestureSlots {
    pub fn get(&self, pointer_id: ui_render::PointerId) -> Option<&CanvasGesture> {
        self.slots.iter().flatten().find(|gesture| gesture.pointer_id == pointer_id)
    }

    pub fn get_mut(&mut self, pointer_id: ui_render::PointerId) -> Option<&mut CanvasGesture> {
        self.slots.iter_mut().flatten().find(|gesture| gesture.pointer_id == pointer_id)
    }

    pub fn has_capacity_for(&self, pointer_id: ui_render::PointerId) -> bool {
        self.get(pointer_id).is_some() || self.slots.iter().any(Option::is_none)
    }

    pub fn insert(&mut self, gesture: CanvasGesture) -> bool {
        let index = self.slots.iter().position(|slot| slot.as_ref().is_some_and(|active| active.pointer_id == gesture.pointer_id)).or_else(|| self.slots.iter().position(Option::is_none));
        let Some(index) = index else { return false };
        self.slots[index] = Some(gesture);
        true
    }

    pub fn take(&mut self, pointer_id: ui_render::PointerId) -> Option<CanvasGesture> {
        self.slots.iter_mut().find(|slot| slot.as_ref().is_some_and(|gesture| gesture.pointer_id == pointer_id)).and_then(Option::take)
    }
}
//#endregion SceneRuntime

#[derive(Clone, Debug)]
pub struct CanvasCatalogueHover {
    pub window_id: String,
    pub document_generation: u64,
    pub host_id: String,
    pub surface_id: String,
    pub controller_id: String,
    pub last_x: f32,
    pub last_y: f32,
    pub last_dispatch_ms: f64,
    pub cancellation_requested: bool,
}

//#region ⌨️SceneInputKit
//#region SceneInput
pub const MAP_MARQUEE_THRESHOLD_PX: f32 = 6.0;

pub fn canvas_active_utility(scene: &UiComponentSceneNode) -> Option<String> {
    let layers: Vec<CanvasLayer> = serde_json::from_str(scene.canvas_2d.as_ref()?.layers_json.as_str()).ok()?;
    layers.into_iter().find(|layer| layer.role.as_deref() == Some("meta")).and_then(|layer| layer.utility)
}

pub fn request_canvas_pointer_gesture_cancel_for_host(host_id: &str) {
    request_cancel(|_, host, _| host == host_id);
    CANVAS_GESTURE.with(|cell| {
        for active in cell.borrow_mut().slots.iter_mut().flatten().filter(|active| active.host_id == host_id) {
            active.cancellation_requested = true;
        }
    });
    CANVAS_CATALOGUE_HOVER.with(|cell| {
        if let Some(active) = cell.borrow_mut().as_mut().filter(|active| active.host_id == host_id) {
            active.cancellation_requested = true;
        }
    });
}

pub fn write_scene_action_batch(input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>, actions: &[ActionDescriptor], commit: impl FnOnce()) -> Result<(), semio_framework_ui_action::BoundedActionFault> {
    let bytes = actions.iter().try_fold(0usize, |sum, action| sum.checked_add(scene_action_bytes(action)?).ok_or(semio_framework_ui_action::BoundedActionFault::ByteCredits))?;
    let mut batch = input.reserve_actions(actions.len(), bytes)?;
    for action in actions {
        write_scene_action_to_batch(&mut batch, action, scene_action_bytes(action)?)?;
    }
    batch.publish_with(commit)
}
//#endregion SceneInput

//#region SceneRowTransfer
/// 🧩️ `PALETTE_DRAG_MIME` in `🧩️BlockListHost/🟦️.tsx:41` — the block-kind mime its palette rows set
/// and its step cards read back.
pub const BLOCK_LIST_PALETTE_DRAG_MIME: &str = "application/x-semio-block-list-block-kind";

pub struct SceneListTransferStart {
    pub source: SceneListTransferSource,
    pub rect: Rect,
}

pub struct SceneListTransferRelease {
    pub suppress_click: bool,
    pub action: Option<ActionDescriptor>,
}

/// ⌨️🖱️ The modifier set a scene press carries, as `UiEvent`'s pointer variants deliver it. Kept as
/// this module's own tiny `Copy` type rather than threading `ui_wgpu::wgpu::EventModifiers` through
/// every hit signature, so the merge-mode policy below has ONE home.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SceneModifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
}

impl From<semio_framework_ui_wgpu_events::EventModifiers> for SceneModifiers {
    fn from(modifiers: semio_framework_ui_wgpu_events::EventModifiers) -> Self {
        Self { shift: modifiers.shift, ctrl: modifiers.ctrl, alt: modifiers.alt, meta: modifiers.meta }
    }
}

impl SceneModifiers {
    /// 🍎️ The platform-neutral "add to the selection" key — `event.metaKey || event.ctrlKey`, exactly
    /// as `VirtualFileSystem`'s own row handler folds it (`⚙️VirtualFileSystem/🟦️.tsx:520`).
    pub fn additive(self) -> bool {
        self.ctrl || self.meta
    }

    /// 🎯️ The wgpu twin of `interactionMergeFromModifiers` (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx:945`) — the ONE
    /// click-select modifier→merge policy every surface routes through. Priority is shift, then
    /// ctrl/meta, then alt, so a chord holding more than one picks the first that matches and shift
    /// always wins a range pick. Distinct from `paint2d_merge_mode`'s marquee-drag policy, which never
    /// produces `range` because a drag has no ordered topology to range over.
    pub fn interaction_merge(self) -> &'static str {
        if self.shift {
            "range"
        } else if self.additive() {
            "invertive"
        } else if self.alt {
            "subtractive"
        } else {
            "replace"
        }
    }
}

pub struct SceneListHit {
    pub control_id: String,
    pub action: Option<ActionDescriptor>,
    /// 📁️ Set when the press landed on a virtual-file-system row's own expand/collapse chevron,
    /// which mutates renderer-local expansion state instead of dispatching a program action.
    pub toggle_expanded: Option<String>,
}

impl SceneListHit {
    pub fn row(control_id: String, action: Option<ActionDescriptor>) -> Self {
        Self { control_id, action, toggle_expanded: None }
    }
}
//#endregion SceneRowTransfer
//#endregion ⌨️SceneInputKit

//#region 🎨️SceneRenderKit
//#region RenderEntry
pub fn render_placeholder(kind: &str, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let theme = ctx.theme;
    draw_text(ctx, &format!("{kind} host"), bounds.x + 12.0, bounds.y + 24.0, theme.font_size_body, theme.text_muted);
}

/// 🎞️ Advances one retained scene identifier scalar or one pre-admitted chrome output item.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VirtualFileSystemChromeLabels {
    pub name: &'static str,
    pub no_file_system_nodes: &'static str,
    pub expand: &'static str,
    pub collapse: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BlockListChromeLabels {
    pub steps: &'static str,
    pub add_step: &'static str,
    pub delete: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IconRenderChromeLabels {
    pub empty_scene: &'static str,
    pub rendering: &'static str,
    pub failed: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SceneChromeLabels {
    pub virtual_file_system: VirtualFileSystemChromeLabels,
    pub block_list: BlockListChromeLabels,
    pub icon_render: IconRenderChromeLabels,
    /// 🌍️ Whether the labels are the German bundle; the world surface writes its annotation and legend text in the same language.
    pub german: bool,
}

impl SceneChromeLabels {
    pub const fn english() -> Self {
        Self {
            german: false,
            virtual_file_system: VirtualFileSystemChromeLabels { name: "Name", no_file_system_nodes: "No file system nodes", expand: "Expand", collapse: "Collapse" },
            block_list: BlockListChromeLabels { steps: "Steps", add_step: "Add Step", delete: "Delete" },
            icon_render: IconRenderChromeLabels { empty_scene: "No scene", rendering: "Rendering…", failed: "Icon rendering failed" },
        }
    }
}

/// 🧩️ The per-frame engine state the retained scene paint needs beyond its own draw list: the shell's
/// `World3dState` map (a `World3d` surface's host, addressed there by every pick/asset/authority
/// ladder) and this frame's `World3dBuildContext` (built in `FrameBuildPhase::WorldResources`, drained
/// again in `FrameBuildPhase::WorldTransfer`). Borrowed for exactly one chrome walk and never stored —
/// the same reason `Ui::frame` takes its `SceneHost` as a parameter rather than owning one.
pub struct SceneEngineHosts<'a> {
    pub chrome_labels: SceneChromeLabels,
    pub world3d_states: &'a mut AdmittedSurfaceMap<infinite_world::world::World3dState>,
    pub world_resources: &'a mut infinite_world::world::World3dBuildContext,
    /// 🪟️ The window instance whose body this walk is painting — the retention authority every
    /// attached engine surface is recorded against, so the shell can keep a surface alive through
    /// frames its window did not repaint. See
    /// `🧑‍🎨engine/🧫️fixtures/🧲️engine-surface-retention/🔣️.json`.
    pub window_id: &'a str,
}
//#endregion RenderEntry
//#endregion 🎨️SceneRenderKit

//#region 🧮️TableAccessibilityKit
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TableStepperFocusTarget {
    pub row_id: String,
    pub column_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TableEditableTextFocusTarget {
    pub row_id: String,
    pub column_id: String,
    pub value: String,
}

impl TableEditableTextFocusTarget {
    pub fn control_id(&self, host_id: &str) -> String {
        format!("{host_id}.row.{}.{}.editable", self.row_id, self.column_id)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TableStepperAccessibilityCell {
    pub key: String,
    pub label: String,
    pub target: TableStepperFocusTarget,
    pub rect: Rect,
    pub min: f64,
    pub max: f64,
    pub value: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TableEditableTextAccessibilityCell {
    pub key: String,
    pub label: String,
    pub target: TableEditableTextFocusTarget,
    pub rect: Rect,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TableButtonAccessibilityCell {
    pub key: String,
    pub label: String,
    pub row_id: String,
    pub column_id: String,
    pub button_index: usize,
    pub rect: Rect,
    pub action: ActionDescriptor,
}
//#endregion 🧮️TableAccessibilityKit

//#region 🧭️TiledMapRetirement
pub fn retire_tiled_map_scene_identity(owner: &crate::element_contracts::ScenePointerTarget) -> bool {
    if owner.kind != semio_framework_ui_declaration::ui::SurfaceKind::TiledMap {
        return false;
    }
    let (retired_interaction, retired_hover) = SCENE_STATE.with(|cell| {
        let mut states = cell.borrow_mut();
        let Some(state) = states.get_mut(&owner.host_id) else { return (false, false) };
        let interaction = state.map_interaction_owner.as_ref().is_some_and(|current| current.same_component_host(owner));
        let hover = state.map_hover_owner.as_ref().is_some_and(|current| current.same_component_host(owner));
        if interaction {
            state.drag = None;
            state.map_marquee_points.clear();
            state.map_marquee_active = false;
            state.map_interaction_owner = None;
        }
        if hover {
            state.map_last_hover_json = None;
            state.map_hover_owner = None;
        }
        (interaction, hover)
    });
    if retired_interaction {
        semio_framework_os_renderer_wgpu_engine_board::tiled_map::retire_map_interaction_owner(owner);
    }
    retired_interaction || retired_hover
}
//#endregion 🧭️TiledMapRetirement
