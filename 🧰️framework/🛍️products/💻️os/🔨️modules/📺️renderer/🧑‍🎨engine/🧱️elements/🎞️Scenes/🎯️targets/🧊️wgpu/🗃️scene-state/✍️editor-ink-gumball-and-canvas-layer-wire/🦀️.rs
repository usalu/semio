//! ✍️ Editor, ink, gumball and canvas layer wire: the text editor ui state, the ink bounds and outline, the vfs focus target, the gumball state and algebra and the canvas layer wire records. See [`super`].

use super::*;

//#region ✍️TextEditorUiKit
#[derive(Clone, Debug, Default)]
pub struct TextEditorUiState {
    pub completions_open: bool,
    pub completion_index: usize,
    pub rename: Option<TextEditorRenameDraft>,
    /// 🖱️ A context-menu row this surface answers itself, parked until the next paint — the shell
    /// resolves and clears its menu without an `InputState` in hand, and every local row commits
    /// through the bounded action queue, which only the paint walk carries.
    pub pending_menu_action: Option<(String, f32, f32)>,
}

/// ✏️ One live rename gesture: the spans the commit rewrites and the name typed so far — React's
/// `renameDraft` (`{ occurrences, text }`).
#[derive(Clone, Debug)]
pub struct TextEditorRenameDraft {
    pub occurrences: Vec<(usize, usize)>,
    pub text: String,
}

pub fn text_editor_ui(host_id: &str) -> TextEditorUiState {
    SCENE_STATE.with(|cell| cell.borrow().get(host_id).map(|state| state.text_editor_ui.clone()).unwrap_or_default())
}
//#endregion ✍️TextEditorUiKit

//#region ✒️InkBounds
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InkBoundsF {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl InkBoundsF {
    pub fn contains_point(&self, x: f64, y: f64) -> bool {
        x >= self.x && x <= self.x + self.w && y >= self.y && y <= self.y + self.h
    }

    pub fn intersects(&self, other: &InkBoundsF) -> bool {
        self.x < other.x + other.w && self.x + self.w > other.x && self.y < other.y + other.h && self.y + self.h > other.y
    }
}
//#endregion ✒️InkBounds

//#region 🖋️InkOutline
pub fn draw_ink_rect_outline(draw: &mut semio_framework_ui_draw_contract::DrawList, x: f32, y: f32, w: f32, h: f32, color: Rgba, width: f32) {
    draw.push_line(x, y, x + w, y, color, width);
    draw.push_line(x + w, y, x + w, y + h, color, width);
    draw.push_line(x + w, y + h, x, y + h, color, width);
    draw.push_line(x, y + h, x, y, color, width);
}
//#endregion 🖋️InkOutline

//#region 🧩️VfsFocusKit
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VfsAccessibilityFocusTarget {
    pub row_id: String,
    pub kind: VfsAccessibilityControlKind,
}
//#endregion 🧩️VfsFocusKit

//#region 📐️GumballState
pub const MOVE_EPSILON: f64 = 1e-9;

pub const TURN_EPSILON: f64 = 1e-6;

/// 🎛️ The handle set a gumball arms.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GumballConfig {
    pub move_axes: bool,
    pub rotate: bool,
    pub scale_axes: bool,
    pub scale_uniform: bool,
}

/// 🗺️ The affine map from model to layer units per axis: `layer = model × scale + offset`.
#[derive(Clone, Copy, Debug, PartialEq, Deserialize)]
pub struct ModelToLayer {
    pub scale: [f64; 2],
    pub offset: [f64; 2],
}

/// 🔭️ The host view the gumball is painted and measured in: the camera and the canvas's logical size, points local to
/// the canvas's top-left corner (`Canvas2dHost`'s `worldToScreenLogical`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GumballView {
    pub camera_x: f64,
    pub camera_y: f64,
    pub zoom: f64,
    pub width: f64,
    pub height: f64,
}

impl GumballView {
    pub(super) fn zoom(&self) -> f64 {
        if self.zoom == 0.0 || self.zoom.is_nan() {
            1.0
        } else {
            self.zoom
        }
    }

    pub(super) fn world_to_screen(&self, x: f64, y: f64) -> (f64, f64) {
        ((x - self.camera_x) * self.zoom() + self.width * 0.5, (y - self.camera_y) * self.zoom() + self.height * 0.5)
    }

    pub(super) fn screen_to_world(&self, x: f64, y: f64) -> (f64, f64) {
        ((x - self.width * 0.5) / self.zoom() + self.camera_x, (y - self.height * 0.5) / self.zoom() + self.camera_y)
    }
}

/// ✋️ One gumball handle.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GumballHandle {
    MoveX,
    MoveY,
    Rotate,
    ScaleX,
    ScaleY,
    ScaleUniform,
}

impl GumballHandle {
    /// 🎚️ The verb the handle drags with.
    pub fn verb(self) -> &'static str {
        match self {
            Self::MoveX | Self::MoveY => "translateSelection",
            Self::Rotate => "rotateSelection",
            Self::ScaleX | Self::ScaleY | Self::ScaleUniform => "scaleSelection",
        }
    }

    pub fn enabled(self, config: &GumballConfig) -> bool {
        match self {
            Self::MoveX | Self::MoveY => config.move_axes,
            Self::ScaleX | Self::ScaleY => config.scale_axes,
            Self::ScaleUniform => config.scale_uniform,
            Self::Rotate => config.rotate,
        }
    }
}

/// 📍️ The pivot in local screen pixels under `view`.
pub fn pivot_screen(pivot_layer: [f64; 2], view: &GumballView) -> (f64, f64) {
    view.world_to_screen(pivot_layer[0], pivot_layer[1])
}

//#region 🧮️Algebra
/// 🎬️ A pose delta in model units: an offset, a turn or a pair of factors.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GumballMotion {
    Translate { dx: f64, dy: f64 },
    Rotate { angle: f64 },
    Scale { sx: f64, sy: f64 },
}

/// 🤏️ One gumball gesture in flight: the handle, the local press point, the pivot, the map, the pinned ids, whether it
/// streams, the last cumulative pose delta and whether a stream tick went out.
#[derive(Clone, Debug, PartialEq)]
pub struct GumballGesture {
    pub kind: GumballHandle,
    pub start: (f64, f64),
    pub pivot_layer: [f64; 2],
    pub model_to_layer: ModelToLayer,
    pub ids: Vec<String>,
    pub live: bool,
    pub total: Option<GumballMotion>,
    pub streamed: bool,
}

fn layer_to_model(map: &ModelToLayer, (x, y): (f64, f64)) -> (f64, f64) {
    ((x - map.offset[0]) / map.scale[0], (y - map.offset[1]) / map.scale[1])
}

fn wrap_angle(angle: f64) -> f64 {
    angle - 2.0 * std::f64::consts::PI * (angle / (2.0 * std::f64::consts::PI)).round()
}

/// 🧮️ The cumulative pose delta from the press to local `(x, y)` — a turn unwrapped against the last total; `None` when
/// it moves nothing.
pub fn total(gesture: &GumballGesture, view: &GumballView, x: f64, y: f64) -> Option<GumballMotion> {
    let pivot = pivot_screen(gesture.pivot_layer, view);
    let (start_x, start_y) = gesture.start;
    match gesture.kind {
        GumballHandle::MoveX | GumballHandle::MoveY => {
            let before = layer_to_model(&gesture.model_to_layer, view.screen_to_world(start_x, start_y));
            let after = layer_to_model(&gesture.model_to_layer, view.screen_to_world(x, y));
            let dx = if gesture.kind == GumballHandle::MoveX { after.0 - before.0 } else { 0.0 };
            let dy = if gesture.kind == GumballHandle::MoveY { after.1 - before.1 } else { 0.0 };
            (dx.abs() >= MOVE_EPSILON || dy.abs() >= MOVE_EPSILON).then_some(GumballMotion::Translate { dx, dy })
        }
        GumballHandle::Rotate => {
            let raw = (y - pivot.1).atan2(x - pivot.0) - (start_y - pivot.1).atan2(start_x - pivot.0);
            let previous = match gesture.total {
                Some(GumballMotion::Rotate { angle }) => angle,
                _ => 0.0,
            };
            let angle = previous + wrap_angle(raw - previous);
            (angle.abs() >= TURN_EPSILON).then_some(GumballMotion::Rotate { angle })
        }
        GumballHandle::ScaleX | GumballHandle::ScaleY | GumballHandle::ScaleUniform => {
            let reach0 = (start_x - pivot.0).hypot(start_y - pivot.1);
            let reach1 = (x - pivot.0).hypot(y - pivot.1);
            let ratio = if reach0 > 1e-6 { reach1 / reach0 } else { 1.0 };
            let (sx, sy) = match gesture.kind {
                GumballHandle::ScaleUniform => (ratio, ratio),
                GumballHandle::ScaleX => (ratio, 1.0),
                _ => (1.0, ratio),
            };
            ((ratio - 1.0).abs() >= TURN_EPSILON).then_some(GumballMotion::Scale { sx, sy })
        }
    }
}
//#endregion Algebra

//#region 🖱️Input
/// 🪪️ The one gumball gesture the canvas owns: its pointer, window, host and document generation, the scene's address,
/// the gesture and the last local pointer (the ghost's anchor).
#[derive(Clone, Debug)]
pub struct GumballSlot {
    pub pointer_id: ui_render::PointerId,
    pub window_id: String,
    pub host_id: String,
    pub document_generation: u64,
    pub controller_id: String,
    pub surface_id: String,
    pub view: GumballView,
    pub gesture: GumballGesture,
    pub pointer: (f64, f64),
    pub cancellation_requested: bool,
}

thread_local! {
    pub static GUMBALL: RefCell<Option<GumballSlot>> = const { RefCell::new(None) };
}

/// 🚩️ Asks the next terminal step to cancel the gesture `matches(window_id, host_id, document_generation)` selects.
pub fn request_cancel(mut matches: impl FnMut(&str, &str, u64) -> bool) {
    GUMBALL.with(|cell| {
        if let Some(slot) = cell.borrow_mut().as_mut().filter(|slot| matches(&slot.window_id, &slot.host_id, slot.document_generation)) {
            slot.cancellation_requested = true;
        }
    });
}
//#endregion Input
//#endregion 📐️GumballState

//#region 🖌️CanvasLayerWire
#[derive(Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct CanvasFillJson {
    #[serde(default)]
    pub kind: Option<String>,
    #[serde(default)]
    pub color: Option<Vec<f64>>,
    #[serde(default)]
    pub x1: f64,
    #[serde(default)]
    pub y1: f64,
    #[serde(default)]
    pub x2: f64,
    #[serde(default)]
    pub y2: f64,
    #[serde(default)]
    pub cx: f64,
    #[serde(default)]
    pub cy: f64,
    #[serde(default)]
    pub r: f64,
    #[serde(default)]
    pub stops: Vec<CanvasGradientStopJson>,
}

#[derive(Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct CanvasGradientStopJson {
    #[serde(default)]
    pub offset: f64,
    #[serde(default)]
    pub color: Option<Vec<f64>>,
}

#[derive(Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CanvasStrokeJson {
    #[serde(default)]
    pub color: Option<Vec<f64>>,
    #[serde(default)]
    pub width: Option<f64>,
    #[serde(default)]
    pub dash: Option<Vec<f64>>,
    #[serde(default)]
    pub cap: Option<String>,
    #[serde(default)]
    pub join: Option<String>,
}

#[derive(Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CanvasImageFieldJson {
    #[serde(default)]
    pub src: Option<String>,
    #[serde(default)]
    pub width: Option<f64>,
    #[serde(default)]
    pub height: Option<f64>,
}

#[derive(Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CanvasTextFieldJson {
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub size: Option<f64>,
    pub font_family: TextFace,
}

#[derive(Deserialize)]
pub struct CanvasLayer {
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
    #[serde(default)]
    pub width: f64,
    #[serde(default)]
    pub height: f64,
    #[serde(default)]
    pub x0: Option<f64>,
    #[serde(default)]
    pub y0: Option<f64>,
    #[serde(default)]
    pub x1: Option<f64>,
    #[serde(default)]
    pub y1: Option<f64>,
    #[serde(default, rename = "dataUrl")]
    pub data_url: Option<String>,
    #[serde(default)]
    pub points: Option<Vec<[f64; 2]>>,
    #[serde(default)]
    pub seams: Option<Vec<u8>>,
    /** 🗒️ `role === "meta"` marks a non-visual bookkeeping record (e.g. carries the host's active
     * pointer-tool id) — filtered from rendering, matches `JsonLayersCanvasSession.renderFrame`'s
     * `records.find(role === "meta")` / `layers.filter(role !== "meta")` split. */
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub(crate) utility: Option<String>,
    #[serde(default)]
    pub visible: Option<bool>,
    #[serde(default)]
    pub opacity: Option<f32>,
    #[serde(default, rename = "blendMode")]
    pub blend_mode: Option<String>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub fill: Option<CanvasFillJson>,
    #[serde(default)]
    pub stroke: Option<CanvasStrokeJson>,
    #[serde(default)]
    pub image: Option<CanvasImageFieldJson>,
    #[serde(default)]
    pub text: Option<CanvasTextFieldJson>,
    #[serde(default)]
    pub segments: Option<Vec<crate::canvas2d_paint::ScenePathSegment>>,
    #[serde(default)]
    pub transform: Option<Vec<f64>>,
    #[serde(default, rename = "fillRule")]
    pub fill_rule: Option<String>,
}
//#endregion 🖌️CanvasLayerWire
