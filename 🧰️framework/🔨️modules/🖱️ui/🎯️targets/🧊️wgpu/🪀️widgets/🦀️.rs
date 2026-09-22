// #region widgets
//! 🧩️ Generic widget tree — layout, measurement, and drawing.

use crate::wgpu::component::ui::UiTreeWindow;
use crate::wgpu::draw::{DrawList, IconAtlas};
use crate::wgpu::geometry::Rect;
use crate::wgpu::input::{HitKind, HitTarget, InputState};
use crate::wgpu::layout::{gap_for_token, layout_horizontal, layout_vertical, padding_for_token};
use crate::wgpu::text::FontAtlas;
use crate::wgpu::theme::{Rgba, Theme};
use crate::wgpu::IconName;
use crate::wgpu::UiTreeActionPlacement;
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
    /// 🔢️ `raw` as this field commits it: a `number` kind is parsed and constrained by its own
    /// `min`/`max`/`step` (React's DOM input refuses out-of-range values outright), everything else
    /// commits its text verbatim. `None` when a `number` field's buffer does not parse at all — the
    /// guest sees no commit rather than an invented number.
    pub fn commit_value(&self, raw: &str) -> Option<dsl::DslValue> {
        if self.input_kind != "number" {
            return Some(dsl::DslValue::String(raw.to_string()));
        }
        raw.parse::<f64>().ok().map(|value| dsl::DslValue::float(crate::wgpu::events::constrain_number_input(value, self.min, self.max, self.step)))
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
    pub(crate) fn register_select_popup_wheel(&mut self, owner: &str, menu: Rect, max_scroll: f32, end_hit: usize) {
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
            let entry = offsets.entry(crate::wgpu::select::select_scroll_key(&scope.owner)).or_insert(0.0);
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
    /// 🪟️ The materialised slice of this section's logical item list — see
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
    // 🧩️ The five `UiNode` kinds this second, smaller paint kit carried NO arm for until ticket
    // 26/09/17 packet W15a — a panel painted through `render_widget` (scene-embedded chrome, the
    // standalone `🌳️Tree` target) could not show a progress bar, an image, a nested group, a scene
    // or an extension slot AT ALL, while the retained `paint_node` ladder painted all five. Each arm
    // below is the same geometry and the same theme tokens as its retained twin
    // (`🖌️paint/🦀️.rs`'s `paint_progress`/`UiNode::Image`/`UiNode::Group`/`UiNode::ComponentScene`/
    // `UiNode::ExternalSlot` arms), so the two kits cannot drift.
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
    /// 🎬️ A scene surface's rect. With no host to fill it this paints the retained arm's placeholder
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
    #[cfg(feature = "wgpu-engine")]
    pub fn component_scene(scene: &crate::wgpu::component::ui::UiComponentSceneNode) -> Self {
        let (hit_kind, hit_control_id) = crate::wgpu::input::retained_scene_hit(scene);
        WidgetNode::ComponentScene { surface_id: scene.surface_id.clone(), hit_kind, hit_control_id }
    }
}

const PANEL_HEADER: f32 = (ui_styling::metrics::chrome::UI_SPACING_COMPACT_PX * ui_styling::metrics::chrome::PANEL_HEADER_HEIGHT_UI_SPACING) as f32;
/// 🌳️ The ONE tree row pitch, straight off `dom.treeRowUiSpacing` — the token React's `Tree` rows
/// carry as `h-workbench`, and the same number `Theme::tree_row_height` hands the retained
/// layout/paint/hit path (`layout::TreeRowMetrics`).
pub(crate) const TREE_ROW_HEIGHT: f32 = (ui_styling::metrics::chrome::UI_SPACING_COMPACT_PX * ui_styling::metrics::dom::TREE_ROW_UI_SPACING) as f32;
pub(crate) const TREE_INDENT_PER_LEVEL: f32 = (ui_styling::metrics::chrome::UI_SPACING_COMPACT_PX * ui_styling::metrics::dom::TREE_INDENT_PER_LEVEL_UI_SPACING) as f32;
pub(crate) const TREE_TOGGLE_WIDTH: f32 = (ui_styling::metrics::chrome::UI_SPACING_COMPACT_PX * ui_styling::metrics::dom::TREE_TOGGLE_UI_SPACING) as f32;
pub(crate) const TREE_ICON_SIZE: f32 = crate::wgpu::chrome::ICON_TREE_ROW;
pub(crate) const TREE_SECTION_GAP: f32 = 8.0;

pub fn measure_widget<E>(atlas: &mut FontAtlas, theme: &Theme, node: &WidgetNode<E>) -> (f32, f32) {
    match node {
        WidgetNode::Stack { direction, gap, padding, children } => {
            let gap = gap_for_token(theme, gap.as_deref());
            let padding = padding_for_token(theme, padding.as_deref()) * 2.0;
            let vertical = direction != "horizontal";
            let mut total_main = 0.0f32;
            let mut max_cross = 0.0f32;
            for (index, child) in children.iter().enumerate() {
                let (w, h) = measure_widget(atlas, theme, child);
                if vertical {
                    total_main += h;
                    max_cross = max_cross.max(w);
                    if index + 1 < children.len() {
                        total_main += gap;
                    }
                } else {
                    total_main += w;
                    max_cross = max_cross.max(h);
                    if index + 1 < children.len() {
                        total_main += gap;
                    }
                }
            }
            if vertical {
                (max_cross + padding, total_main + padding)
            } else {
                (total_main + padding, max_cross + padding)
            }
        }
        WidgetNode::Text { value, emphasize } => {
            let size = if *emphasize { theme.font_size_emphasized } else { theme.font_size_body };
            let (w, _) = atlas.measure_text(value, size);
            let lines = wrap_text(atlas, value, w.max(120.0), size);
            (w.max(120.0), lines.len() as f32 * size * 1.35)
        }
        WidgetNode::Separator => (theme.control_height.max(1.0), 1.0 + theme.gap_standard),
        WidgetNode::Button { .. } | WidgetNode::Input { .. } | WidgetNode::Select { .. } | WidgetNode::Toggle { .. } | WidgetNode::Slider { .. } | WidgetNode::NumberStepper { .. } | WidgetNode::IconSelect { .. } => {
            (theme.control_height, theme.control_height)
        }
        WidgetNode::KeyValue { entries } => {
            let label_w = entries.iter().map(|e| atlas.measure_text(&e.label, theme.font_size_small).0).fold(0.0f32, f32::max);
            (label_w + theme.gap_standard * 2.0 + 80.0, entries.len() as f32 * theme.control_height)
        }
        WidgetNode::Ring { .. } => (80.0, 80.0),
        WidgetNode::Field { label, child, .. } => {
            let label_h = theme.font_size_small;
            let gap = gap_for_token(theme, Some("standard"));
            let (cw, ch) = measure_control(atlas, theme, child);
            (cw.max(atlas.measure_text(label, theme.font_size_small).0), label_h + gap + ch)
        }
        WidgetNode::Section { children, label, .. } => {
            let mut height = PANEL_HEADER;
            let mut max_w = 0.0f32;
            if label.is_some() {
                max_w = max_w.max(160.0);
            }
            for child in children {
                let (w, h) = measure_widget(atlas, theme, child);
                max_w = max_w.max(w);
                height += h + theme.gap_standard;
            }
            (max_w.max(120.0), height)
        }
        WidgetNode::Tree { sections, .. } => (measure_tree_sections_width(sections, atlas, theme), measure_tree_sections(sections)),
        // 📶️ React's bar is `h-tiny w-full` — a full-width track one `--size-tiny` tall, which is
        // what `paint::progress_bar_rects` centres inside whatever box this measure asks for.
        WidgetNode::Progress { .. } => (120.0, crate::wgpu::chrome::SIZE_TINY.max(theme.gap_standard)),
        // 🖼️ React caps the `<img>` at `max-h-64` (`UI_IMAGE_MAX_BOX_HEIGHT`); a source the process
        // cannot decode itself measures as one `alt` line, which is what it paints.
        WidgetNode::Image { src, alt, .. } => match crate::wgpu::paint::ui_image_natural_size(src.as_str()) {
            Some((natural_w, natural_h)) => {
                let height = (natural_h as f32).min(crate::wgpu::paint::UI_IMAGE_MAX_BOX_HEIGHT);
                let scale = if natural_h > 0 { height / natural_h as f32 } else { 1.0 };
                ((natural_w as f32 * scale).max(1.0), height.max(1.0))
            }
            None => {
                let label = alt.as_deref().unwrap_or("");
                (atlas.measure_text(label, theme.font_size_small).0.max(120.0), theme.control_height)
            }
        },
        WidgetNode::Group { children, label, .. } => {
            let mut height = PANEL_HEADER;
            let mut max_w = atlas.measure_text(label, theme.font_size_body).0 + TREE_TOGGLE_WIDTH + theme.gap_standard;
            for child in children {
                let (w, h) = measure_widget(atlas, theme, child);
                max_w = max_w.max(w);
                height += h + theme.gap_standard;
            }
            (max_w.max(120.0), height)
        }
        // 🎬️🧩️ Neither a scene surface nor a plugin slot has an intrinsic size — both fill whatever
        // box their container grants, exactly like the retained ladder's `LayoutNodeKind::EngineSurface`.
        WidgetNode::ComponentScene { .. } | WidgetNode::ExternalSlot { .. } => (120.0, theme.control_height),
    }
}

fn measure_control<E>(atlas: &mut FontAtlas, theme: &Theme, control: &ControlNode<E>) -> (f32, f32) {
    match control {
        ControlNode::Button { .. } | ControlNode::Input { .. } | ControlNode::Select { .. } | ControlNode::Toggle { .. } | ControlNode::Slider { .. } | ControlNode::NumberStepper { .. } | ControlNode::IconSelect { .. } => {
            (theme.control_height, theme.control_height)
        }
        ControlNode::KeyValue { entries } => {
            let label_w = entries.iter().map(|e| atlas.measure_text(&e.label, theme.font_size_small).0).fold(0.0f32, f32::max);
            (label_w + theme.gap_standard * 2.0 + 80.0, entries.len() as f32 * theme.control_height)
        }
        ControlNode::Ring { .. } => (80.0, 80.0),
    }
}

pub fn render_widget<E: Clone>(node: &WidgetNode<E>, bounds: Rect, ctx: &mut WidgetContext<'_, E>) {
    match node {
        WidgetNode::Stack { direction, gap, padding, children } => {
            let gap = gap_for_token(ctx.theme, gap.as_deref());
            let padding = padding_for_token(ctx.theme, padding.as_deref());
            let vertical = direction != "horizontal";
            let sizes: Vec<f32> = children
                .iter()
                .map(|child| {
                    let (w, h) = measure_widget(ctx.atlas, ctx.theme, child);
                    if vertical {
                        h
                    } else {
                        w
                    }
                })
                .collect();
            let rects = if vertical { layout_vertical(bounds, gap, padding, &sizes) } else { layout_horizontal(bounds, gap, padding, &sizes) };
            for (child, rect) in children.iter().zip(rects.iter()) {
                render_widget(child, *rect, ctx);
            }
        }
        WidgetNode::Text { value, emphasize } => {
            let size = if *emphasize { ctx.theme.font_size_emphasized } else { ctx.theme.font_size_body };
            let color = if *emphasize { ctx.theme.text } else { ctx.theme.text_muted };
            draw_text_wrapped(ctx, value, bounds.x, bounds.y, bounds.w.max(1.0), size, color);
        }
        WidgetNode::Separator => {
            let y = bounds.y + bounds.h * 0.5;
            ctx.draw.push_line(bounds.x, y, bounds.x + bounds.w, y, ctx.theme.separator, 1.0);
        }
        WidgetNode::Button { id, icon_id, label, event } => {
            render_button(id.as_ref(), *icon_id, label, event.clone(), bounds, ctx);
        }
        WidgetNode::Input { id, input_kind, value, placeholder, commit, min, max, step, accept, on_change } => {
            register_input_meta(ctx, id, input_kind, value, commit.clone(), (*min, *max, *step, accept.clone()), on_change.clone());
            render_input(id, value, placeholder.as_deref(), bounds, ctx);
        }
        WidgetNode::Select { id, value, items, placeholder, on_change } => {
            register_select_meta(ctx, id, on_change.clone());
            render_select(id, value, items, placeholder.as_deref(), bounds, ctx);
        }
        WidgetNode::Toggle { id, icon_id, pressed, text, on_change } => {
            register_toggle_meta(ctx, id, *pressed, on_change.clone());
            render_toggle(id, *icon_id, *pressed, text.as_deref(), bounds, ctx);
        }
        WidgetNode::KeyValue { entries } => render_key_value(entries, bounds, ctx),
        WidgetNode::Slider { id, value, min, max, step, ready, disabled, on_change } => {
            render_slider(id, *value, *min, *max, *step, *ready, *disabled, on_change.clone(), bounds, ctx);
        }
        WidgetNode::NumberStepper { id, value, step, uniform, on_absolute, on_delta } => {
            render_number_stepper(id, *value, *step, *uniform, on_absolute.clone(), on_delta.clone(), bounds, ctx);
        }
        WidgetNode::Ring { id, t, disabled, on_change } => {
            render_ring(id, *t, *disabled, on_change.clone(), bounds, ctx);
        }
        WidgetNode::IconSelect { id, value, uniform, classifier_kind, on_change } => {
            render_icon_select(id, value, *uniform, classifier_kind, on_change.clone(), bounds, ctx);
        }
        WidgetNode::Field { label, child, .. } => {
            let label_h = ctx.theme.font_size_small;
            let gap = gap_for_token(ctx.theme, Some("standard"));
            draw_text(ctx, label, bounds.x, bounds.y + label_h, ctx.theme.font_size_small, ctx.theme.text_muted);
            let child_bounds = Rect::new(bounds.x, bounds.y + label_h + gap, bounds.w, bounds.h - label_h - gap);
            render_control(child, child_bounds, ctx);
        }
        WidgetNode::Section { label, children, id, default_open } => {
            let section_key = format!("section.{id}");
            if !ctx.collapsed_sections.contains_key(&section_key) {
                ctx.collapsed_sections.insert(section_key.clone(), !default_open);
            }
            let collapsed = tree_row_collapsed(ctx.collapsed_sections, &section_key, *default_open);
            if label.is_some() {
                let header = Rect::new(bounds.x, bounds.y, bounds.w, PANEL_HEADER);
                let chevron_rect = Rect::new(bounds.x, bounds.y, TREE_TOGGLE_WIDTH, PANEL_HEADER);
                let chevron = if collapsed { "chevron-right" } else { "chevron-down" };
                tree_draw_chevron(ctx, chevron, chevron_rect, crate::wgpu::chrome::ICON_TINY);
                if let Some(label) = label {
                    draw_text(ctx, label, bounds.x + TREE_TOGGLE_WIDTH + ctx.theme.gap_standard, bounds.y + (PANEL_HEADER + ctx.theme.font_size_body) * 0.5 - 2.0, ctx.theme.font_size_body, ctx.theme.text);
                }
                ctx.input.register_hit(HitTarget { rect: header, event: None, control_id: Some(format!("section.chevron.{id}")), kind: HitKind::Generic, drag_axis: None, drag_data: None });
            }
            if !collapsed {
                let mut y = bounds.y + PANEL_HEADER;
                for child in children {
                    let (_, h) = measure_widget(ctx.atlas, ctx.theme, child);
                    let child_bounds = Rect::new(bounds.x, y, bounds.w, h);
                    render_widget(child, child_bounds, ctx);
                    y += h + ctx.theme.gap_standard;
                }
            }
        }
        WidgetNode::Tree { sections, selected_ids, highlighted_ids, selection_change } => {
            if let Some(maps) = ctx.interaction_maps.as_deref_mut() {
                maps.tree_selection_change = selection_change.clone();
            }
            let scroll_id = sections.first().map_or_else(|| format!("tree:{:.0}:{:.0}", bounds.x, bounds.y), |section| format!("tree:{}", section.id));
            let content_h = measure_tree_sections_state(sections, ctx.collapsed_sections);
            render_scroll_region(&scroll_id, bounds, content_h.max(bounds.h), ctx, |content, ctx| {
                render_tree(sections, selected_ids, highlighted_ids, content, ctx);
            });
        }
        WidgetNode::Progress { completed, total, .. } => render_progress(*completed, *total, bounds, ctx),
        WidgetNode::Image { id, src, alt } => render_image(id, src, alt.as_deref(), bounds, ctx),
        WidgetNode::Group { id, label, default_open, children } => render_group(id, label, *default_open, children, bounds, ctx),
        WidgetNode::ComponentScene { surface_id, hit_kind, hit_control_id } => {
            render_scene_placeholder(surface_id, bounds, ctx);
            ctx.input.register_hit(HitTarget { rect: bounds, event: None, control_id: Some(hit_control_id.clone()), kind: *hit_kind, drag_axis: None, drag_data: None });
        }
        WidgetNode::ExternalSlot { body_key } => render_external_slot(body_key, bounds, ctx),
    }
}

//#region 🧩️KitCompositeParity
/// 📶️ The kit's progress bar — the SAME track/fill rects `paint::progress_bar_rects` hands the
/// retained ladder, so the two kits cannot drift on geometry, the indeterminate share, or the
/// `muted`/`accent` token pair.
fn render_progress<E>(completed: f64, total: Option<f64>, bounds: Rect, ctx: &mut WidgetContext<'_, E>) {
    let (track, fill) = crate::wgpu::paint::progress_bar_rects_of(completed, total, bounds, ctx.theme);
    ctx.draw.push_rounded(track, ctx.theme.muted, ctx.theme.border_radius);
    if fill[2] > 0.0 {
        ctx.draw.push_rounded(fill, ctx.theme.accent, ctx.theme.border_radius);
    }
}

/// 🖼️ A decodable (`data:`) source draws its real bitmap at React's `object-contain` rect; every
/// other source is a host fetch this crate has no authority to perform, so it shows the placeholder
/// panel plus `alt` — which is also what React's `<img>` shows while it is pending or broken.
fn render_image<E>(id: &str, src: &str, alt: Option<&str>, bounds: Rect, ctx: &mut WidgetContext<'_, E>) {
    let decoded = matches!(crate::wgpu::paint::admit_ui_image(src), crate::wgpu::paint::UiImageAdmission::Ready).then(|| crate::wgpu::paint::ui_image_natural_size(src)).flatten();
    if let Some((natural_w, natural_h)) = decoded {
        let content = crate::wgpu::paint::ui_image_content_rect(bounds, natural_w, natural_h);
        ctx.draw.push_raster_quad(src, [content.x, content.y, content.w, content.h], [0.0, 0.0, 1.0, 1.0], 1.0);
        return;
    }
    ctx.draw.push_rounded([bounds.x, bounds.y, bounds.w, bounds.h], ctx.theme.panel, ctx.theme.border_radius);
    let label = alt.unwrap_or(id);
    draw_text(ctx, label, bounds.x + ctx.theme.padding_standard, bounds.y + (bounds.h + ctx.theme.font_size_small) * 0.5 - 2.0, ctx.theme.font_size_small, ctx.theme.text_muted);
}

/// 🗂️ A `Group` folds through the same `collapsed_sections` slot a `Section` does — one fold state
/// per id, so a panel that mixes both kinds cannot end up with two disagreeing open bits.
fn render_group<E: Clone>(id: &str, label: &str, default_open: bool, children: &[WidgetNode<E>], bounds: Rect, ctx: &mut WidgetContext<'_, E>) {
    let section_key = format!("section.{id}");
    if !ctx.collapsed_sections.contains_key(&section_key) {
        ctx.collapsed_sections.insert(section_key.clone(), !default_open);
    }
    let collapsed = tree_row_collapsed(ctx.collapsed_sections, &section_key, default_open);
    let header = Rect::new(bounds.x, bounds.y, bounds.w, PANEL_HEADER);
    let chevron_rect = Rect::new(bounds.x, bounds.y, TREE_TOGGLE_WIDTH, PANEL_HEADER);
    tree_draw_chevron(ctx, if collapsed { "chevron-right" } else { "chevron-down" }, chevron_rect, crate::wgpu::chrome::ICON_TINY);
    draw_text(ctx, label, bounds.x + TREE_TOGGLE_WIDTH + ctx.theme.gap_standard, bounds.y + (PANEL_HEADER + ctx.theme.font_size_body) * 0.5 - 2.0, ctx.theme.font_size_body, ctx.theme.text);
    ctx.input.register_hit(HitTarget { rect: header, event: None, control_id: Some(format!("section.chevron.{id}")), kind: HitKind::Generic, drag_axis: None, drag_data: None });
    if collapsed {
        return;
    }
    let mut y = bounds.y + PANEL_HEADER;
    for child in children {
        let (_, h) = measure_widget(ctx.atlas, ctx.theme, child);
        render_widget(child, Rect::new(bounds.x, y, bounds.w, h), ctx);
        y += h + ctx.theme.gap_standard;
    }
}

/// 🎬️ The no-host placeholder for a scene rect — "there is something in that box" chrome, identical
/// to the retained ladder's own `ComponentScene` fallback.
fn render_scene_placeholder<E>(surface_id: &str, bounds: Rect, ctx: &mut WidgetContext<'_, E>) {
    let _ = surface_id;
    ctx.draw.push_rounded([bounds.x, bounds.y, bounds.w, bounds.h], ctx.theme.panel, ctx.theme.border_radius);
    crate::wgpu::chrome::push_control_border(ctx.draw, bounds, ctx.theme, ctx.theme.border_normal, ctx.theme.panel);
}

/// 🧩️ A plugin body slot labelled by its `body_key`; the body itself is the host's concern.
fn render_external_slot<E>(body_key: &str, bounds: Rect, ctx: &mut WidgetContext<'_, E>) {
    ctx.draw.push_rounded([bounds.x, bounds.y, bounds.w, bounds.h], ctx.theme.panel, ctx.theme.border_radius);
    crate::wgpu::chrome::push_control_border(ctx.draw, bounds, ctx.theme, ctx.theme.border_normal, ctx.theme.panel);
    draw_text(ctx, body_key, bounds.x + ctx.theme.padding_standard, bounds.y + (bounds.h + ctx.theme.font_size_small) * 0.5 - 2.0, ctx.theme.font_size_small, ctx.theme.text_muted);
}
//#endregion 🧩️KitCompositeParity

fn render_control<E: Clone>(control: &ControlNode<E>, bounds: Rect, ctx: &mut WidgetContext<'_, E>) {
    match control {
        ControlNode::Button { id, icon_id, label, event } => {
            render_button(id.as_ref(), *icon_id, label, event.clone(), bounds, ctx);
        }
        ControlNode::Input { id, input_kind, value, placeholder, commit, min, max, step, accept, on_change } => {
            register_input_meta(ctx, id, input_kind, value, commit.clone(), (*min, *max, *step, accept.clone()), on_change.clone());
            render_input(id, value, placeholder.as_deref(), bounds, ctx);
        }
        ControlNode::Select { id, value, items, placeholder, on_change } => {
            register_select_meta(ctx, id, on_change.clone());
            render_select(id, value, items, placeholder.as_deref(), bounds, ctx);
        }
        ControlNode::Toggle { id, icon_id, pressed, text, on_change } => {
            register_toggle_meta(ctx, id, *pressed, on_change.clone());
            render_toggle(id, *icon_id, *pressed, text.as_deref(), bounds, ctx);
        }
        ControlNode::KeyValue { entries } => render_key_value(entries, bounds, ctx),
        ControlNode::Slider { id, value, min, max, step, ready, disabled, on_change } => {
            render_slider(id, *value, *min, *max, *step, *ready, *disabled, on_change.clone(), bounds, ctx);
        }
        ControlNode::NumberStepper { id, value, step, uniform, on_absolute, on_delta } => {
            render_number_stepper(id, *value, *step, *uniform, on_absolute.clone(), on_delta.clone(), bounds, ctx);
        }
        ControlNode::Ring { id, t, disabled, on_change } => render_ring(id, *t, *disabled, on_change.clone(), bounds, ctx),
        ControlNode::IconSelect { id, value, uniform, classifier_kind, on_change } => {
            render_icon_select(id, value, *uniform, classifier_kind, on_change.clone(), bounds, ctx);
        }
    }
}

/// ✍️ Registers one input's commit contract for the host: the action it fires, its `commit` moment,
/// its current text, and the `(min, max, step, accept)` constraints `InputProps` carries — dropped on
/// the floor here until this pass, so a chrome number field committed whatever was typed while
/// React's own `<input min max step>` refused it.
pub(crate) fn register_input_meta<E: Clone>(ctx: &mut WidgetContext<'_, E>, id: &str, input_kind: &str, value: &str, commit: Option<String>, constraints: (Option<f64>, Option<f64>, Option<f64>, Option<String>), on_change: Option<E>) {
    if let (Some(maps), Some(on_change)) = (ctx.interaction_maps.as_deref_mut(), on_change) {
        let (min, max, step, accept) = constraints;
        maps.input_metas.insert(id.to_string(), InputMeta { on_change, commit, value: value.to_string(), input_kind: input_kind.to_string(), min, max, step, accept });
    }
}

fn register_select_meta<E: Clone>(ctx: &mut WidgetContext<'_, E>, id: &str, on_change: Option<E>) {
    if let (Some(maps), Some(on_change)) = (ctx.interaction_maps.as_deref_mut(), on_change) {
        maps.select_metas.insert(id.to_string(), on_change);
    }
}

fn register_toggle_meta<E: Clone>(ctx: &mut WidgetContext<'_, E>, id: &str, pressed: bool, on_change: Option<E>) {
    if let (Some(maps), Some(on_change)) = (ctx.interaction_maps.as_deref_mut(), on_change) {
        maps.toggle_metas.insert(id.to_string(), (pressed, on_change));
    }
}

use crate::wgpu::button::render_button;
use crate::wgpu::icon_selector::render_icon_select;
use crate::wgpu::input_element::render_input;
use crate::wgpu::key_value::render_key_value;
use crate::wgpu::ring::render_ring;
use crate::wgpu::select::render_select;
use crate::wgpu::slider::render_slider;
use crate::wgpu::stepper::render_number_stepper;
use crate::wgpu::toggle::render_toggle;
use crate::wgpu::tree_element::{measure_tree_sections, measure_tree_sections_state, measure_tree_sections_width, render_tree};

pub(crate) fn tree_gutter_width(depth: u32) -> f32 {
    depth as f32 * TREE_INDENT_PER_LEVEL + TREE_TOGGLE_WIDTH
}

pub(crate) fn tree_icon_id<E>(item: &TreeItem<E>, expandable: bool) -> Option<&str> {
    item.icon_id.map(IconName::as_str).or(if expandable { Some("folder") } else { None })
}

pub(crate) fn tree_row_collapsed(collapsed: &HashMap<String, bool>, key: &str, default_open: bool) -> bool {
    collapsed.get(key).copied().unwrap_or(!default_open)
}

/// 🔽️ One fold chevron centred in `rect`. `size` is the caller's, because React draws a `Tree`
/// ROW's toggle at `size-tiny` — 9.6 px, [`crate::wgpu::chrome::SIZE_TINY`] (`🌳️Tree/🟦️.tsx:4576`) —
/// and a SECTION header's at `size-small`, 16 px, [`crate::wgpu::chrome::ICON_TINY`]
/// (`treeSectionChevronClassName`, `🌳️Tree/🟦️.tsx:251`).
pub(crate) fn tree_draw_chevron<E>(ctx: &mut WidgetContext<'_, E>, icon_id: &str, rect: Rect, size: f32) {
    if let Some(uv) = ctx.icons.and_then(|icons| icons.icon_uv(icon_id)) {
        draw_icon(ctx, uv, rect.x + (rect.w - size) * 0.5, rect.y + (rect.h - size) * 0.5, size, ctx.theme.text_muted);
    }
}

pub(crate) fn tree_draw_guides<E>(ctx: &mut WidgetContext<'_, E>, gutter: Rect, depth: u32, is_last_at_level: &[bool]) {
    let hair = ctx.theme.stroke_hairline.max(1.0);
    let guide_color = ctx.theme.border_normal;
    for level in 0..depth {
        if is_last_at_level.get(level as usize).copied().unwrap_or(false) {
            continue;
        }
        let x = gutter.x + level as f32 * TREE_INDENT_PER_LEVEL + TREE_TOGGLE_WIDTH * 0.5;
        ctx.draw.push_solid([x, gutter.y, hair, gutter.h], guide_color);
    }
    if depth > 0 {
        let x = gutter.x + (depth - 1) as f32 * TREE_INDENT_PER_LEVEL + TREE_TOGGLE_WIDTH * 0.5;
        let mid_y = gutter.y + gutter.h * 0.5;
        ctx.draw.push_solid([x, gutter.y, hair, mid_y - gutter.y], guide_color);
        ctx.draw.push_solid([x, mid_y, TREE_INDENT_PER_LEVEL * 0.5, hair], guide_color);
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

pub(crate) fn measure_text_width<E>(ctx: &mut WidgetContext<'_, E>, text: &str, size: f32) -> f32 {
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
    atlas.wrap_lines(text, max_width, size).into_iter().map(|line| text[line].trim_end_matches(crate::wgpu::text::is_wrap_space).to_string()).collect()
}

/// 🅰️ [`draw_text_on`] at a chosen weight. A [`crate::wgpu::text::TextWeight::Semibold`] run is
/// struck twice, the second time offset by `text::faux_bold_offset`, because no bold Latin face
/// ships. The advances are unchanged, so the run occupies exactly the regular face's line box — which
/// is why this is a weight swap and not the SIZE swap the `Text` node used to make.
pub fn draw_text_weighted(draw: &mut DrawList, atlas: &mut FontAtlas, text: &str, x: f32, y: f32, size: f32, color: Rgba, weight: crate::wgpu::text::TextWeight) {
    draw_text_on(draw, atlas, text, x, y, size, color);
    if matches!(weight, crate::wgpu::text::TextWeight::Semibold) {
        draw_text_on(draw, atlas, text, x + crate::wgpu::text::faux_bold_offset(size), y, size, color);
    }
}

pub fn draw_text_on(draw: &mut DrawList, atlas: &mut FontAtlas, text: &str, x: f32, y: f32, size: f32, color: Rgba) {
    let atlas_w = atlas.width as f32;
    let atlas_h = atlas.height as f32;
    let mut cursor_x = x;
    for ch in text.chars() {
        let glyph = atlas.ensure_glyph(ch, size);
        let gw = glyph.logical_width();
        let gh = glyph.logical_height();
        let gx = cursor_x + glyph.bearing_x;
        let gy = y - gh - glyph.bearing_y;
        let uv_rect = [glyph.atlas_x as f32 / atlas_w, glyph.atlas_y as f32 / atlas_h, (glyph.atlas_x + glyph.width) as f32 / atlas_w, (glyph.atlas_y + glyph.height) as f32 / atlas_h];
        draw.push_glyph([gx, gy, gw.max(1.0), gh.max(1.0)], color, uv_rect);
        cursor_x += glyph.advance;
    }
}

pub fn draw_text_overlay_on(draw: &mut DrawList, atlas: &mut FontAtlas, text: &str, x: f32, y: f32, size: f32, color: Rgba) {
    let atlas_w = atlas.width as f32;
    let atlas_h = atlas.height as f32;
    let mut cursor_x = x;
    for ch in text.chars() {
        let glyph = atlas.ensure_glyph(ch, size);
        let gw = glyph.logical_width();
        let gh = glyph.logical_height();
        let gx = cursor_x + glyph.bearing_x;
        let gy = y - gh - glyph.bearing_y;
        let uv_rect = [glyph.atlas_x as f32 / atlas_w, glyph.atlas_y as f32 / atlas_h, (glyph.atlas_x + glyph.width) as f32 / atlas_w, (glyph.atlas_y + glyph.height) as f32 / atlas_h];
        draw.push_glyph_overlay([gx, gy, gw.max(1.0), gh.max(1.0)], color, uv_rect);
        cursor_x += glyph.advance;
    }
}

pub fn draw_text<E>(ctx: &mut WidgetContext<'_, E>, text: &str, x: f32, y: f32, size: f32, color: Rgba) {
    let atlas_w = ctx.atlas.width as f32;
    let atlas_h = ctx.atlas.height as f32;
    let mut cursor_x = x;
    for ch in text.chars() {
        let glyph = ctx.atlas.ensure_glyph(ch, size);
        let gw = glyph.logical_width();
        let gh = glyph.logical_height();
        let gx = cursor_x + glyph.bearing_x;
        let gy = y - gh - glyph.bearing_y;
        let uv_rect = [glyph.atlas_x as f32 / atlas_w, glyph.atlas_y as f32 / atlas_h, (glyph.atlas_x + glyph.width) as f32 / atlas_w, (glyph.atlas_y + glyph.height) as f32 / atlas_h];
        ctx.draw.push_glyph([gx, gy, gw.max(1.0), gh.max(1.0)], color, uv_rect);
        cursor_x += glyph.advance;
    }
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
/// 🧭️ Retained-mode orbit-view gizmo paint call (ticket
/// 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS): only the actual GPU
/// draw-call issuing stays here, gated with the rest of `widgets.rs` behind `wgpu-engine` — it
/// needs `WidgetContext`, which bundles the font/icon atlases. The placement/tip-geometry/
/// hit-test math it calls into lives target-neutral in `draw_types::gizmo`.
pub mod gizmo {
    use crate::wgpu::draw_types::gizmo::{orbit_view_gizmo_head_radius, orbit_view_gizmo_placement, orbit_view_gizmo_tips};
    use crate::wgpu::widgets::WidgetContext;
    use crate::wgpu::{Camera3d, Rect, Rgba};

    /// 🧭️ Screen-space XYZ orientation gizmo in the lower-right of every world-3d window (wgpu parity with React `WorldOrbitViewGizmo`).
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

#[cfg(test)]
#[path = "../../../🧪️tests/🔬️targets-wgpu-widget-metrics/🦀️.rs"]
mod metrics_tests;
// #endregion widgets
