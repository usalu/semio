//! 🧰️ Widget kit: the immediate-mode context, interaction maps, widget node model and text drawing every control renders through.

use semio_framework_ui_declaration::ui::UiTreeWindow;
use semio_framework_ui_draw_contract::{DrawList, IconAtlas};
use semio_framework_ui_theme::geometry::Rect;
use semio_framework_ui_wgpu_input::{HitKind, HitTarget, InputState};
use semio_framework_ui_text::{FontAtlas, TextFace};
use semio_framework_ui_theme::{Rgba, Theme};
use semio_framework_ui_declaration::IconName;
use semio_framework_ui_declaration::ui::UiTreeActionPlacement;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct InputMeta<E> {
    pub on_change: E,
    pub commit: Option<String>,
    pub value: String,
    /// 🔢️ `InputProps`' own constraints, carried through to the host that commits this field —
    /// the canvas twin of the `min`/`max`/`step` attributes React hands a `<input type="number">`
    /// (`🗣️Interpreter/🟦️.tsx`'s `InputView`), which the browser enforces for it and an
    /// immediate-mode canvas has to enforce itself. See [`InputMeta::commit_value`].
    pub input_kind: String,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub step: Option<f64>,
    /// 📎️ The `accept` filter a `file` input's picker is opened with — React passes it straight to
    /// `<input type="file" accept>`; a host picker must apply it itself.
    pub accept: Option<String>,
}

impl<E> InputMeta<E> {
    /// 🐙️ `raw` as this field commits it: a `number` kind is parsed and constrained by its own
    /// `min`/`max`/`step` (React's DOM input refuses out-of-range values outright), everything else
    /// commits its text verbatim. `None` when a `number` field's buffer does not parse at all — the
    /// guest sees no commit rather than an invented number.
    pub fn commit_value(&self, raw: &str) -> Option<semio_framework_value::DslValue> {
        if self.input_kind != "number" {
            return Some(semio_framework_value::DslValue::String(raw.to_string()));
        }
        raw.parse::<f64>().ok().map(|value| semio_framework_value::DslValue::float(crate::constrain_number_input(value, self.min, self.max, self.step)))
    }
}

#[derive(Clone, Debug)]
pub struct SliderMeta<E> {
    pub on_change: E,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub value: f64,
    pub bounds_x: f32,
    pub bounds_w: f32,
}

#[derive(Clone, Debug)]
pub struct StepperMeta<E> {
    pub on_absolute: E,
    pub on_delta: E,
    pub step: f64,
    pub value: f64,
}

#[derive(Clone, Debug)]
pub struct RingMeta<E> {
    pub on_change: E,
    pub disabled: bool,
    pub center_x: f32,
    pub center_y: f32,
    pub radius: f32,
}

struct SelectPopupWheelScope {
    owner: String,
    menu: Rect,
    max_scroll: f32,
    end_hit: usize,
}

pub struct WidgetInteractionMaps<E> {
    select_popup_wheel: Option<SelectPopupWheelScope>,
    pub input_metas: HashMap<String, InputMeta<E>>,
    pub select_metas: HashMap<String, E>,
    pub toggle_metas: HashMap<String, (bool, E)>,
    pub slider_metas: HashMap<String, SliderMeta<E>>,
    pub stepper_metas: HashMap<String, StepperMeta<E>>,
    pub ring_metas: HashMap<String, RingMeta<E>>,
    pub slider_live_values: HashMap<String, f64>,
    pub ring_live_values: HashMap<String, f64>,
    pub tree_hover_commands: HashMap<String, E>,
    pub tree_unhover_commands: HashMap<String, E>,
    pub tree_selection_change: Option<E>,
}

impl<E> Default for WidgetInteractionMaps<E> {
    fn default() -> Self {
        Self {
            select_popup_wheel: None,
            input_metas: HashMap::new(),
            select_metas: HashMap::new(),
            toggle_metas: HashMap::new(),
            slider_metas: HashMap::new(),
            stepper_metas: HashMap::new(),
            ring_metas: HashMap::new(),
            slider_live_values: HashMap::new(),
            ring_live_values: HashMap::new(),
            tree_hover_commands: HashMap::new(),
            tree_unhover_commands: HashMap::new(),
            tree_selection_change: None,
        }
    }
}

impl<E> WidgetInteractionMaps<E> {
    pub fn register_select_popup_wheel(&mut self, owner: &str, menu: Rect, max_scroll: f32, end_hit: usize) {
        self.select_popup_wheel = Some(SelectPopupWheelScope { owner: owner.into(), menu, max_scroll, end_hit });
    }

    /// 🎯️ Uses the popup and hit order from the same presented frame, including empty menu gutters.
    pub fn scroll_select_popup_at(&self, input: &InputState<E>, offsets: &mut HashMap<String, f32>, x: f32, y: f32, delta: f32) -> bool
    where
        E: Clone,
    {
        let Some(scope) = self.select_popup_wheel.as_ref().filter(|scope| scope.menu.contains(x, y)) else { return false };
        if input.hit_index_at(x, y).is_some_and(|index| index >= scope.end_hit) {
            return false;
        }
        if delta.is_finite() && delta != 0.0 {
            let entry = offsets.entry(semio_framework_ui_select_behavior::select_scroll_key(&scope.owner)).or_insert(0.0);
            *entry = (*entry + delta).clamp(0.0, scope.max_scroll);
        }
        true
    }

    pub fn clear_select_popup_wheel(&mut self) {
        self.select_popup_wheel = None;
    }

    pub fn clear_frame(&mut self) {
        self.clear_select_popup_wheel();
        self.input_metas.clear();
        self.select_metas.clear();
        self.toggle_metas.clear();
        self.slider_metas.clear();
        self.stepper_metas.clear();
        self.ring_metas.clear();
        self.slider_live_values.clear();
        self.ring_live_values.clear();
        self.tree_hover_commands.clear();
        self.tree_unhover_commands.clear();
        self.tree_selection_change = None;
    }
}

pub struct WidgetContext<'a, E> {
    pub draw: &'a mut DrawList,
    pub overlay: Option<&'a mut DrawList>,
    pub atlas: &'a mut FontAtlas,
    pub icons: Option<&'a IconAtlas>,
    pub input: &'a mut InputState<E>,
    pub theme: &'a Theme,
    pub scroll_offsets: &'a mut HashMap<String, f32>,
    pub collapsed_sections: &'a mut HashMap<String, bool>,
    pub open_selects: &'a mut HashMap<String, bool>,
    pub interaction_maps: Option<&'a mut WidgetInteractionMaps<E>>,
    pub pick_clip: Option<Rect>,
    /// 📐️ The measured surface height this immediate-mode pass draws into, in logical pixels — the
    /// kit's twin of React's `{ width: window.innerWidth, height: window.innerHeight }` argument to
    /// `resolveSelectPlacement` (`🧱️elements/🔽️Select/🟦️.tsx:514`). `0.0` means "unmeasured": a
    /// popup then places below at its natural height instead of guessing a flip or a clamp.
    pub viewport_height: f32,
}

#[derive(Clone, Debug)]
pub struct SelectItem {
    pub value: String,
    pub label: String,
}

#[derive(Clone, Debug)]
pub struct KeyValueEntry {
    pub label: String,
    pub value: String,
}

#[derive(Clone, Debug)]
pub struct TreeItemAction<E> {
    pub icon_id: IconName,
    pub label: Option<String>,
    pub event: E,
    pub placement: UiTreeActionPlacement,
}

#[derive(Clone, Debug)]
pub struct TreeItem<E> {
    pub id: String,
    pub label: String,
    pub description: Option<String>,
    pub icon_id: Option<IconName>,
    pub selected: bool,
    pub highlighted: bool,
    pub default_open: bool,
    pub dimmed: bool,
    pub event: Option<E>,
    pub hover_event: Option<E>,
    pub unhover_event: Option<E>,
    pub actions: Vec<TreeItemAction<E>>,
    pub draggable: bool,
    pub drag_data: HashMap<String, String>,
    pub control: Option<Box<WidgetNode<E>>>,
    pub children: Vec<TreeItem<E>>,
    /// 🪟️ The materialised slice of this row's logical child list — see
    /// [`crate::wgpu::component::ui::UiTreeWindow`]. `children` are the entries
    /// `[offset, offset + children.len())`; the painter pitches the rest as empty bands. A row with
    /// `window.total > 0` is expandable (shows the fold chevron) even with zero children.
    pub window: Option<UiTreeWindow>,
}

#[derive(Clone, Debug)]
pub struct TreeSection<E> {
    pub id: String,
    pub label: Option<String>,
    pub default_open: bool,
    pub items: Vec<TreeItem<E>>,
    /// 🍕️ The materialised slice of this section's logical item list — see
    /// [`crate::wgpu::component::ui::UiTreeWindow`].
    pub window: Option<UiTreeWindow>,
}

#[derive(Clone, Debug)]
pub enum ControlNode<E> {
    Button { id: Option<String>, icon_id: Option<IconName>, label: String, event: Option<E> },
    Input { id: String, input_kind: String, value: String, placeholder: Option<String>, commit: Option<String>, min: Option<f64>, max: Option<f64>, step: Option<f64>, accept: Option<String>, on_change: Option<E> },
    Select { id: String, value: String, items: Vec<SelectItem>, placeholder: Option<String>, on_change: Option<E> },
    Toggle { id: String, icon_id: IconName, pressed: bool, text: Option<String>, on_change: Option<E> },
    KeyValue { entries: Vec<KeyValueEntry> },
    Slider { id: String, value: f64, min: f64, max: f64, step: f64, ready: Option<f64>, disabled: bool, on_change: Option<E> },
    NumberStepper { id: String, value: f64, step: f64, uniform: bool, on_absolute: Option<E>, on_delta: Option<E> },
    Ring { id: String, t: f64, disabled: bool, on_change: Option<E> },
    IconSelect { id: String, value: String, uniform: bool, classifier_kind: String, on_change: Option<E> },
}

#[derive(Clone, Debug)]
pub enum WidgetNode<E> {
    Stack {
        direction: String,
        gap: Option<String>,
        padding: Option<String>,
        children: Vec<WidgetNode<E>>,
    },
    Text {
        value: String,
        emphasize: bool,
    },
    Separator,
    Button {
        id: Option<String>,
        icon_id: Option<IconName>,
        label: String,
        event: Option<E>,
    },
    Input {
        id: String,
        input_kind: String,
        value: String,
        placeholder: Option<String>,
        commit: Option<String>,
        min: Option<f64>,
        max: Option<f64>,
        step: Option<f64>,
        accept: Option<String>,
        on_change: Option<E>,
    },
    Select {
        id: String,
        value: String,
        items: Vec<SelectItem>,
        placeholder: Option<String>,
        on_change: Option<E>,
    },
    Toggle {
        id: String,
        icon_id: IconName,
        pressed: bool,
        text: Option<String>,
        on_change: Option<E>,
    },
    KeyValue {
        entries: Vec<KeyValueEntry>,
    },
    Slider {
        id: String,
        value: f64,
        min: f64,
        max: f64,
        step: f64,
        ready: Option<f64>,
        disabled: bool,
        on_change: Option<E>,
    },
    NumberStepper {
        id: String,
        value: f64,
        step: f64,
        uniform: bool,
        on_absolute: Option<E>,
        on_delta: Option<E>,
    },
    Ring {
        id: String,
        t: f64,
        disabled: bool,
        on_change: Option<E>,
    },
    IconSelect {
        id: String,
        value: String,
        uniform: bool,
        classifier_kind: String,
        on_change: Option<E>,
    },
    Field {
        id: String,
        label: String,
        child: ControlNode<E>,
    },
    Section {
        id: String,
        label: Option<String>,
        default_open: bool,
        children: Vec<WidgetNode<E>>,
    },
    Tree {
        sections: Vec<TreeSection<E>>,
        selected_ids: Vec<String>,
        highlighted_ids: Vec<String>,
        selection_change: Option<E>,
    },
    //#region 🧩️KitCompositeParity
    /// 📶️ React's `ProgressView` (`🗣️Interpreter/🟦️.tsx:2098-2118`): a `bg-muted h-tiny w-full`
    /// track with a `bg-accent` fill at `uiProgressFractionV1(completed, total)`, or the centred
    /// one-third busy band when `total` is absent. `completed`/`total` are `UiProgressNode`'s own
    /// fields, so `paint::progress_bar_rects` prices both kits' geometry.
    Progress {
        id: String,
        completed: f64,
        total: Option<f64>,
    },
    /// 🖼️ React's `ImageView` — a real `<img src>`. A `data:` PNG decodes through the shared
    /// `🖼️UiImageSources` ledger and draws a raster quad at `object-contain`; anything else is a host
    /// fetch and shows the `alt` placeholder, exactly as the retained arm does.
    Image {
        id: String,
        src: String,
        alt: Option<String>,
    },
    /// 🗂️ A `container` with `role="group"` on React's side; a chevron + label header over its
    /// children here, sharing `Section`'s own `collapsed_sections` slot so the two fold the same way.
    Group {
        id: String,
        label: String,
        default_open: bool,
        children: Vec<WidgetNode<E>>,
    },
    /// 🎞️ A scene surface's rect. With no host to fill it this paints the retained arm's placeholder
    /// chrome; the hit target is minted by `scene_hit_kind` so a press resolves the same
    /// kind/control id `input::retained_hit_registration`'s `ComponentScene` arm resolves.
    ComponentScene {
        surface_id: String,
        hit_kind: HitKind,
        hit_control_id: String,
    },
    /// 🧩️ A plugin body slot: placeholder chrome labelled with its `body_key`, like the retained arm.
    ExternalSlot {
        body_key: String,
    },
    //#endregion 🧩️KitCompositeParity
}

impl<E> WidgetNode<E> {
    /// 🎬️ The kit's `ComponentScene` arm for one retained scene node, taking its `HitKind`/control id
    /// from `input::retained_scene_hit` — the ONE derivation the retained hit registry uses, so a
    /// scene painted through this kit and the same scene painted through the retained ladder answer
    /// the identical hit contract (`World3d` its own kind, node-graph/board `\u{2026}.pane`, map `\u{2026}.map`).
    pub fn component_scene(scene: &semio_framework_ui_declaration::ui::UiComponentSceneNode) -> Self {
        let (hit_kind, hit_control_id) = semio_framework_ui_wgpu_mounted_layout::hit::retained_scene_hit(scene);
        WidgetNode::ComponentScene { surface_id: scene.surface_id.clone(), hit_kind, hit_control_id }
    }
}

/// ✍️ Registers one input's commit contract for the host: the action it fires, its `commit` moment,
/// its current text, and the `(min, max, step, accept)` constraints `InputProps` carries — dropped on
/// the floor here until this pass, so a chrome number field committed whatever was typed while
/// React's own `<input min max step>` refused it.
pub fn register_input_meta<E: Clone>(ctx: &mut WidgetContext<'_, E>, id: &str, input_kind: &str, value: &str, commit: Option<String>, constraints: (Option<f64>, Option<f64>, Option<f64>, Option<String>), on_change: Option<E>) {
    if let (Some(maps), Some(on_change)) = (ctx.interaction_maps.as_deref_mut(), on_change) {
        let (min, max, step, accept) = constraints;
        maps.input_metas.insert(id.to_string(), InputMeta { on_change, commit, value: value.to_string(), input_kind: input_kind.to_string(), min, max, step, accept });
    }
}

pub fn register_select_meta<E: Clone>(ctx: &mut WidgetContext<'_, E>, id: &str, on_change: Option<E>) {
    if let (Some(maps), Some(on_change)) = (ctx.interaction_maps.as_deref_mut(), on_change) {
        maps.select_metas.insert(id.to_string(), on_change);
    }
}

pub fn register_toggle_meta<E: Clone>(ctx: &mut WidgetContext<'_, E>, id: &str, pressed: bool, on_change: Option<E>) {
    if let (Some(maps), Some(on_change)) = (ctx.interaction_maps.as_deref_mut(), on_change) {
        maps.toggle_metas.insert(id.to_string(), (pressed, on_change));
    }
}

pub fn render_scroll_region<E: Clone, F: FnOnce(Rect, &mut WidgetContext<'_, E>)>(scroll_id: &str, bounds: Rect, content_height: f32, ctx: &mut WidgetContext<'_, E>, render_content: F) {
    let max_scroll = (content_height - bounds.h).max(0.0);
    let offset = ctx.scroll_offsets.entry(scroll_id.to_string()).or_insert(0.0);
    *offset = offset.clamp(0.0, max_scroll);
    let scroll = *offset;
    ctx.input.register_hit(HitTarget { rect: bounds, event: None, control_id: Some(scroll_id.to_string()), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None });
    ctx.draw.push_scissor(bounds);
    let content_bounds = Rect::new(bounds.x, bounds.y - scroll, bounds.w, content_height);
    render_content(content_bounds, ctx);
    ctx.draw.pop_scissor();
}

pub fn draw_icon<E>(ctx: &mut WidgetContext<'_, E>, uv: [f32; 4], x: f32, y: f32, size: f32, color: Rgba) {
    ctx.draw.push_textured([x, y, size, size], uv, color);
}

pub fn measure_text_width<E>(ctx: &mut WidgetContext<'_, E>, text: &str, size: f32) -> f32 {
    let (w, _) = ctx.atlas.measure_text(text, size);
    w
}

pub fn draw_text_wrapped<E>(ctx: &mut WidgetContext<'_, E>, text: &str, x: f32, y: f32, max_width: f32, size: f32, color: Rgba) -> f32 {
    let lines = wrap_text(ctx.atlas, text, max_width, size);
    let line_h = size * 1.35;
    for (index, line) in lines.iter().enumerate() {
        draw_text(ctx, line, x, y + line_h * index as f32 + size, size, color);
    }
    lines.len() as f32 * line_h
}

/// ✂️ The owned-line view of [`FontAtlas::wrap_lines`] — CSS greedy word wrap, with each line's
/// hanging trailing spaces trimmed because they paint no ink. It used to `split_whitespace`, which
/// collapsed interior runs of spaces and knew nothing of hyphens or ideographs.
pub fn wrap_text(atlas: &mut FontAtlas, text: &str, max_width: f32, size: f32) -> Vec<String> {
    atlas.wrap_lines(text, max_width, size).into_iter().map(|line| text[line].trim_end_matches(semio_framework_ui_text::is_wrap_space).to_string()).collect()
}

/// 🅰️ [`draw_text_on`] at a chosen weight. A [`crate::wgpu::text::TextWeight::Semibold`] run is
/// struck twice, the second time offset by `text::faux_bold_offset`, because no bold Latin face
/// ships. The advances are unchanged, so the run occupies exactly the regular face's line box — which
/// is why this is a weight swap and not the SIZE swap the `Text` node used to make.
pub fn draw_text_weighted(draw: &mut DrawList, atlas: &mut FontAtlas, text: &str, x: f32, y: f32, size: f32, color: Rgba, weight: semio_framework_ui_text::TextWeight) {
    draw_text_on(draw, atlas, text, x, y, size, color);
    if let Some(offset) = weight.synthetic_offset(size) {
        draw_text_on(draw, atlas, text, x + offset, y, size, color);
    }
}

pub fn draw_text_on(draw: &mut DrawList, atlas: &mut FontAtlas, text: &str, x: f32, y: f32, size: f32, color: Rgba) {
    draw_text_face_on(draw, atlas, TextFace::Sans, text, x, y, size, color);
}

/// 🔤️ Paints one run with a selected authored face into an explicit draw list.
pub fn draw_text_face_on(draw: &mut DrawList, atlas: &mut FontAtlas, face: TextFace, text: &str, x: f32, y: f32, size: f32, color: Rgba) {
    pen_glyph_run(draw, atlas, face, text, (x, y), size, color, false);
}

/// ↗️ Pens a run in local coordinates and transforms every sampled glyph's complete quad.
#[allow(clippy::too_many_arguments,reason="one argument per authored text paint input")]
pub fn draw_text_face_affine_on(draw:&mut DrawList,atlas:&mut FontAtlas,face:TextFace,text:&str,origin:(f32,f32),size:f32,color:Rgba,matrix:[f32;6]) {
    let atlas_w=atlas.width as f32;
    let atlas_h=atlas.height as f32;
    let (mut cursor_x,mut previous)=(origin.0,None);
    for ch in text.chars() {
        cursor_x+=atlas.pen_kerning(face,previous,cursor_x-origin.0,ch,size);
        previous=Some(ch);
        let glyph=*atlas.ensure_glyph_for(face,ch,size);
        let width=glyph.logical_width();
        let height=glyph.logical_height();
        let rect=[cursor_x+glyph.bearing_x,origin.1-height-glyph.bearing_y,width.max(1.0),height.max(1.0)];
        let uv_rect=[glyph.atlas_x as f32/atlas_w,glyph.atlas_y as f32/atlas_h,(glyph.atlas_x+glyph.width) as f32/atlas_w,(glyph.atlas_y+glyph.height) as f32/atlas_h];
        draw.push_glyph_affine(rect,color,uv_rect,matrix);
        cursor_x+=glyph.advance;
    }
}

pub fn draw_text_overlay_on(draw: &mut DrawList, atlas: &mut FontAtlas, text: &str, x: f32, y: f32, size: f32, color: Rgba) {
    pen_glyph_run(draw, atlas, TextFace::Sans, text, (x, y), size, color, true);
}

#[cfg(test)]
#[path="../↗️affine/🧪️tests/🔬️unit/🦀️.rs"]
mod affine_glyph_tests;

/// ✒️ Pens one single-line run from its baseline `origin`, glyph by glyph — each glyph's advance plus its pair kerning with the
/// glyph before it ([`FontAtlas::pen_kerning`], the rule [`FontAtlas::measure_text_face`] sums by) — onto the draw list's
/// regular or overlay instances.
fn pen_glyph_run(draw: &mut DrawList, atlas: &mut FontAtlas, face: TextFace, text: &str, origin: (f32, f32), size: f32, color: Rgba, overlay: bool) {
    let atlas_w = atlas.width as f32;
    let atlas_h = atlas.height as f32;
    let (mut cursor_x, mut previous) = (origin.0, None);
    for ch in text.chars() {
        cursor_x += atlas.pen_kerning(face, previous, cursor_x - origin.0, ch, size);
        previous = Some(ch);
        let glyph = *atlas.ensure_glyph_for(face, ch, size);
        let gw = glyph.logical_width();
        let gh = glyph.logical_height();
        let rect = [cursor_x + glyph.bearing_x, origin.1 - gh - glyph.bearing_y, gw.max(1.0), gh.max(1.0)];
        let uv_rect = [glyph.atlas_x as f32 / atlas_w, glyph.atlas_y as f32 / atlas_h, (glyph.atlas_x + glyph.width) as f32 / atlas_w, (glyph.atlas_y + glyph.height) as f32 / atlas_h];
        if overlay {
            draw.push_glyph_overlay(rect, color, uv_rect);
        } else {
            draw.push_glyph(rect, color, uv_rect);
        }
        cursor_x += glyph.advance;
    }
}

pub fn draw_text<E>(ctx: &mut WidgetContext<'_, E>, text: &str, x: f32, y: f32, size: f32, color: Rgba) {
    draw_text_face(ctx, TextFace::Sans, text, x, y, size, color);
}

/// 📖️ Paints one run with a selected authored face through a widget context.
pub fn draw_text_face<E>(ctx: &mut WidgetContext<'_, E>, face: TextFace, text: &str, x: f32, y: f32, size: f32, color: Rgba) {
    pen_glyph_run(ctx.draw, ctx.atlas, face, text, (x, y), size, color, false);
}

pub fn draw_text_overlay<E>(ctx: &mut WidgetContext<'_, E>, text: &str, x: f32, y: f32, size: f32, color: Rgba) {
    draw_text_overlay_on(ctx.draw, ctx.atlas, text, x, y, size, color);
}

//#region 🔖️Gizmo
/** 🧭️ Screen-space XYZ orientation gizmo (wgpu parity with React `WorldOrbitViewGizmo`) — placement,
hit-testing, and paint. Relocated verbatim from `♾️infinite/🌍️world` (see
`.🧬semio/🦑️repo/🎫️tickets/26/08/05/FRAMEWORK-BUILDER-PASSTHROUGHS-APP-COMMANDS-MACRO-WIDGET-EXTRACTION`) so any
plugin's world-3d window can reuse it, not only `♾️infinite`'s own. `World3dState`-specific hover-state
plumbing (`update_world_orbit_view_gizmo_hover`, which owns `&mut World3dState`) stays in `♾️infinite/🌍️world`
— app-specific config plumbing, not paint logic — and now calls through to `orbit_view_gizmo_placement`/
`orbit_view_gizmo_tips`/`orbit_view_gizmo_hit_test` here. */
/// 🛰️ Retained-mode orbit-view gizmo paint call (ticket
/// 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS): only the actual GPU
/// draw-call issuing stays here, gated with the rest of `widgets.rs` behind `wgpu-engine` — it
/// needs `WidgetContext`, which bundles the font/icon atlases. The placement/tip-geometry/
/// hit-test math it calls into lives target-neutral in `draw_types::gizmo`.
pub mod gizmo {
    use semio_framework_ui_draw_contract::gizmo::{orbit_view_gizmo_head_radius, orbit_view_gizmo_placement, orbit_view_gizmo_tips};
    use crate::WidgetContext;
    use ui_scene::math::Camera3d;
use semio_framework_ui_theme::geometry::Rect;
use semio_framework_ui_theme::Rgba;

    /// 🪐️ Screen-space XYZ orientation gizmo in the lower-right of every world-3d window (wgpu parity with React `WorldOrbitViewGizmo`).
    pub fn paint_orbit_view_gizmo<E>(ctx: &mut WidgetContext<'_, E>, camera: &Camera3d, viewport: Rect, hovered_tip: Option<usize>) {
        let (margin_x, margin_y) = orbit_view_gizmo_placement(viewport);
        let origin_x = viewport.x + viewport.w - margin_x;
        let origin_y = viewport.y + viewport.h - margin_y;
        let tips = orbit_view_gizmo_tips(camera, viewport);
        let mut ordered: Vec<(f32, usize)> = tips.iter().enumerate().map(|(index, tip)| (tip.depth, index)).collect();
        ordered.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        let has_hover = hovered_tip.is_some();
        for (_, index) in ordered {
            let tip = &tips[index];
            let hovered = hovered_tip == Some(index);
            let depth_fade = if tip.depth > 0.05 { 0.45 } else { 1.0 };
            let hover_fade = if has_hover && !hovered { 0.42 } else { 1.0 };
            let alpha = (tip.color.a * depth_fade * hover_fade).min(1.0);
            let stroke = Rgba::new(tip.color.r, tip.color.g, tip.color.b, if hovered { tip.color.a.min(1.0) } else { alpha });
            ctx.draw.push_line_overlay(origin_x, origin_y, tip.screen_x, tip.screen_y, stroke, if tip.is_corner { 1.5 } else { 2.0 });
            let r = orbit_view_gizmo_head_radius(tip.prominent, hovered);
            ctx.draw.push_rounded_overlay([tip.screen_x - r, tip.screen_y - r, r * 2.0, r * 2.0], stroke, r);
        }
    }
}

//#endregion 🔖️Gizmo

/// 🪜️ Clamps to `[min, max]` and snaps to the nearest `step` off `min` — the three `InputProps`
/// constraints the wire has always carried and this target used to drop on the floor, letting a
/// number field commit `999` into a `max: 10` parameter that React's own DOM input refuses. A `NaN`
/// (an unparseable buffer) passes through untouched so the guest still sees the refusal rather than a
/// silently invented number.
pub fn constrain_number_input(value: f64, min: Option<f64>, max: Option<f64>, step: Option<f64>) -> f64 {
    if value.is_nan() {
        return value;
    }
    let stepped = match step.filter(|step| *step > 0.0) {
        Some(step) => {
            let origin = min.unwrap_or(0.0);
            origin + ((value - origin) / step).round() * step
        }
        None => value,
    };
    stepped.max(min.unwrap_or(f64::NEG_INFINITY)).min(max.unwrap_or(f64::INFINITY))
}
