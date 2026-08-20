// #region widgets
//! 🧩️ Generic widget tree — layout, measurement, and drawing.

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

pub struct WidgetInteractionMaps<E> {
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
    pub fn clear_frame(&mut self) {
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
}

#[derive(Clone, Debug)]
pub struct TreeSection<E> {
    pub id: String,
    pub label: Option<String>,
    pub default_open: bool,
    pub items: Vec<TreeItem<E>>,
}

#[derive(Clone, Debug)]
pub enum ControlNode<E> {
    Button { id: Option<String>, icon_id: Option<IconName>, label: String, event: Option<E> },
    Input { id: String, input_kind: String, value: String, placeholder: Option<String>, commit: Option<String>, on_change: Option<E> },
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
    Stack { direction: String, gap: Option<String>, padding: Option<String>, children: Vec<WidgetNode<E>> },
    Text { value: String, emphasize: bool },
    Separator,
    Button { id: Option<String>, icon_id: Option<IconName>, label: String, event: Option<E> },
    Input { id: String, input_kind: String, value: String, placeholder: Option<String>, commit: Option<String>, on_change: Option<E> },
    Select { id: String, value: String, items: Vec<SelectItem>, placeholder: Option<String>, on_change: Option<E> },
    Toggle { id: String, icon_id: IconName, pressed: bool, text: Option<String>, on_change: Option<E> },
    KeyValue { entries: Vec<KeyValueEntry> },
    Slider { id: String, value: f64, min: f64, max: f64, step: f64, ready: Option<f64>, disabled: bool, on_change: Option<E> },
    NumberStepper { id: String, value: f64, step: f64, uniform: bool, on_absolute: Option<E>, on_delta: Option<E> },
    Ring { id: String, t: f64, disabled: bool, on_change: Option<E> },
    IconSelect { id: String, value: String, uniform: bool, classifier_kind: String, on_change: Option<E> },
    Field { id: String, label: String, child: ControlNode<E> },
    Section { id: String, label: Option<String>, default_open: bool, children: Vec<WidgetNode<E>> },
    Tree { sections: Vec<TreeSection<E>>, selected_ids: Vec<String>, highlighted_ids: Vec<String>, selection_change: Option<E> },
}

const PANEL_HEADER: f32 = 24.0;
pub(crate) const TREE_ROW_HEIGHT: f32 = 24.0;
pub(crate) const TREE_INDENT_PER_LEVEL: f32 = 10.0;
pub(crate) const TREE_TOGGLE_WIDTH: f32 = 14.0;
pub(crate) const TREE_ICON_SIZE: f32 = 14.0;
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
        WidgetNode::Input { id, value, placeholder, commit, on_change, .. } => {
            register_input_meta(ctx, id, value, commit.clone(), on_change.clone());
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
                tree_draw_chevron(ctx, chevron, chevron_rect);
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
            let scroll_id = format!("tree:{:.0}:{:.0}", bounds.x, bounds.y);
            let content_h = measure_tree_sections_state(sections, ctx.collapsed_sections);
            render_scroll_region(&scroll_id, bounds, content_h.max(bounds.h), ctx, |content, ctx| {
                render_tree(sections, selected_ids, highlighted_ids, content, ctx);
            });
        }
    }
}

fn render_control<E: Clone>(control: &ControlNode<E>, bounds: Rect, ctx: &mut WidgetContext<'_, E>) {
    match control {
        ControlNode::Button { id, icon_id, label, event } => {
            render_button(id.as_ref(), *icon_id, label, event.clone(), bounds, ctx);
        }
        ControlNode::Input { id, value, placeholder, commit, on_change, .. } => {
            register_input_meta(ctx, id, value, commit.clone(), on_change.clone());
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

pub(crate) fn register_input_meta<E: Clone>(ctx: &mut WidgetContext<'_, E>, id: &str, value: &str, commit: Option<String>, on_change: Option<E>) {
    if let (Some(maps), Some(on_change)) = (ctx.interaction_maps.as_deref_mut(), on_change) {
        maps.input_metas.insert(id.to_string(), InputMeta { on_change, commit, value: value.to_string() });
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

pub(crate) fn tree_draw_chevron<E>(ctx: &mut WidgetContext<'_, E>, icon_id: &str, rect: Rect) {
    if let Some(uv) = ctx.icons.and_then(|icons| icons.icon_uv(icon_id)) {
        draw_icon(ctx, uv, rect.x + (rect.w - TREE_ICON_SIZE) * 0.5, rect.y + (rect.h - TREE_ICON_SIZE) * 0.5, TREE_ICON_SIZE, ctx.theme.text_muted);
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

pub fn wrap_text(atlas: &mut FontAtlas, text: &str, max_width: f32, size: f32) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();
    for word in text.split_whitespace() {
        let trial = if current.is_empty() { word.to_string() } else { format!("{current} {word}") };
        let (w, _) = atlas.measure_text(&trial, size);
        if w > max_width && !current.is_empty() {
            lines.push(current);
            current = word.to_string();
        } else {
            current = trial;
        }
    }
    if !current.is_empty() {
        lines.push(current);
    }
    if lines.is_empty() {
        lines.push(String::new());
    }
    lines
}

pub fn draw_text_on(draw: &mut DrawList, atlas: &mut FontAtlas, text: &str, x: f32, y: f32, size: f32, color: Rgba) {
    let atlas_w = atlas.width as f32;
    let atlas_h = atlas.height as f32;
    let mut cursor_x = x;
    for ch in text.chars() {
        let glyph = atlas.ensure_glyph(ch, size);
        let gw = glyph.width as f32;
        let gh = glyph.height as f32;
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
        let gw = glyph.width as f32;
        let gh = glyph.height as f32;
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
        let gw = glyph.width as f32;
        let gh = glyph.height as f32;
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
`.🦑️repo/🎫️tickets/26/08/05/FRAMEWORK-BUILDER-PASSTHROUGHS-APP-COMMANDS-MACRO-WIDGET-EXTRACTION`) so any
plugin's world-3d window can reuse it, not only `♾️infinite`'s own. `World3dState`-specific hover-state
plumbing (`update_world_orbit_view_gizmo_hover`, which owns `&mut World3dState`) stays in `♾️infinite/🌍️world`
— app-specific config plumbing, not paint logic — and now calls through to `orbit_view_gizmo_placement`/
`orbit_view_gizmo_tips`/`orbit_view_gizmo_hit_test` here. */
pub mod gizmo {
    use crate::wgpu::widgets::WidgetContext;
    use crate::wgpu::{Camera3d, Rect, Rgba, Vec3, Vec3Math};

    /// 🧭️ Permanent X/Y/Z paints — primary / secondary / tertiary (semio tokens), not muted chrome.
    pub fn spatial_axis_rgba(axis: u8, alpha: f32) -> Rgba {
        match axis {
            0 => Rgba::new(1.0, 0.204, 0.310, alpha),   // primary #ff344f
            1 => Rgba::new(0.204, 0.820, 0.749, alpha), // secondary #34d1bf
            _ => Rgba::new(0.980, 0.584, 0.0, alpha),   // tertiary #fa9500
        }
    }

    /// 🧭️ Mirrors `resolveSceneGizmoViewportPlacement` — bottom-right corner inset matching pane `--spacing-single` chrome.
    pub fn orbit_view_gizmo_placement(viewport: Rect) -> (f32, f32) {
        let chrome_inset = 4.0_f32;
        let gizmo_half_extent = 28.0_f32;
        let preferred = chrome_inset + gizmo_half_extent;
        let max_fit = (viewport.w.min(viewport.h) / 3.0).floor().max(22.0);
        let margin = preferred.min(max_fit);
        (margin, margin)
    }

    /// 🧭️ Screen-space tip used for orbit-view gizmo hover hit-testing and paint.
    pub struct OrbitViewGizmoTip {
        pub screen_x: f32,
        pub screen_y: f32,
        pub depth: f32,
        pub pick_radius: f32,
        pub color: Rgba,
        pub is_corner: bool,
        pub prominent: bool,
    }

    pub fn orbit_view_gizmo_tips(camera: &Camera3d, viewport: Rect) -> Vec<OrbitViewGizmoTip> {
        let (margin_x, margin_y) = orbit_view_gizmo_placement(viewport);
        let origin_x = viewport.x + viewport.w - margin_x;
        let origin_y = viewport.y + viewport.h - margin_y;
        let axis_len = (viewport.w.min(viewport.h) * 0.04).clamp(14.0, 24.0);
        let forward = camera.position.sub_m(camera.target);
        let forward_len = forward.length_m();
        if forward_len < 1e-5 {
            return Vec::new();
        }
        let forward = forward.scale_m(1.0 / forward_len);
        let right = forward.cross_m(camera.up);
        let right_len = right.length_m();
        if right_len < 1e-5 {
            return Vec::new();
        }
        let right = right.scale_m(1.0 / right_len);
        let up = right.cross_m(forward).normalize_m();
        let neutral = Rgba::new(0.62, 0.62, 0.66, 0.9);
        let axes = [
            (Vec3 { x: 1.0, y: 0.0, z: 0.0 }, spatial_axis_rgba(0, 1.0), true),
            (Vec3 { x: -1.0, y: 0.0, z: 0.0 }, spatial_axis_rgba(0, 0.75), false),
            (Vec3 { x: 0.0, y: 1.0, z: 0.0 }, spatial_axis_rgba(1, 1.0), true),
            (Vec3 { x: 0.0, y: -1.0, z: 0.0 }, spatial_axis_rgba(1, 0.75), false),
            (Vec3 { x: 0.0, y: 0.0, z: 1.0 }, spatial_axis_rgba(2, 1.0), true),
            (Vec3 { x: 0.0, y: 0.0, z: -1.0 }, spatial_axis_rgba(2, 0.75), false),
        ];
        let corners = [
            (Vec3 { x: 0.72, y: 0.72, z: 0.72 }, true),
            (Vec3 { x: -0.72, y: 0.72, z: 0.72 }, true),
            (Vec3 { x: 0.72, y: -0.72, z: 0.72 }, true),
            (Vec3 { x: -0.72, y: -0.72, z: 0.72 }, true),
            (Vec3 { x: 0.72, y: 0.72, z: -0.72 }, false),
            (Vec3 { x: -0.72, y: 0.72, z: -0.72 }, false),
            (Vec3 { x: 0.72, y: -0.72, z: -0.72 }, false),
            (Vec3 { x: -0.72, y: -0.72, z: -0.72 }, false),
        ];
        let mut tips: Vec<OrbitViewGizmoTip> = axes
            .into_iter()
            .map(|(axis, color, prominent)| {
                let sx = axis.dot_m(right);
                let sy = -axis.dot_m(up);
                let depth = axis.dot_m(forward);
                let tip_x = origin_x + sx * axis_len;
                let tip_y = origin_y + sy * axis_len;
                let pick_radius = if prominent { 10.0 } else { 7.0 };
                OrbitViewGizmoTip { screen_x: tip_x, screen_y: tip_y, depth, pick_radius, color, is_corner: false, prominent }
            })
            .chain(corners.into_iter().map(|(axis, prominent)| {
                let sx = axis.dot_m(right);
                let sy = -axis.dot_m(up);
                let depth = axis.dot_m(forward);
                let tip_x = origin_x + sx * axis_len;
                let tip_y = origin_y + sy * axis_len;
                let pick_radius = if prominent { 10.0 } else { 7.0 };
                OrbitViewGizmoTip { screen_x: tip_x, screen_y: tip_y, depth, pick_radius, color: neutral, is_corner: true, prominent }
            }))
            .collect();
        tips.push(OrbitViewGizmoTip { screen_x: origin_x, screen_y: origin_y, depth: 0.0, pick_radius: 9.0, color: neutral, is_corner: false, prominent: true });
        tips
    }

    pub fn orbit_view_gizmo_hit_test(x: f32, y: f32, tips: &[OrbitViewGizmoTip]) -> Option<usize> {
        tips.iter()
            .enumerate()
            .filter_map(|(index, tip)| {
                let distance = ((x - tip.screen_x).powi(2) + (y - tip.screen_y).powi(2)).sqrt();
                if distance <= tip.pick_radius + 3.0 {
                    Some((index, distance))
                } else {
                    None
                }
            })
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(index, _)| index)
    }

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
            let r = if tip.prominent {
                if hovered {
                    3.6
                } else {
                    3.0
                }
            } else if hovered {
                2.4
            } else {
                2.0
            };
            ctx.draw.push_solid_overlay([tip.screen_x - r, tip.screen_y - r, tip.screen_x + r, tip.screen_y + r], stroke);
        }
    }
}
//#endregion 🔖️Gizmo
// #endregion widgets
