//! 🔬️ Immediate-mode reference paint pipeline: the golden oracle the retained paint laws compare against.

use semio_framework_ui_arena::NodeId;
use semio_framework_ui_wgpu_layout::chrome::chrome_item_bg;
use semio_framework_ui_wgpu_layout::chrome::{foreground_on_fill, item_bg, item_text, push_control_border, push_icon, ICON_TINY, SIZE_TINY};
use semio_framework_ui_declaration::ui::{UiButtonNode, UiComponentSceneNode, UiExternalSlotNode, UiGroupNode, UiIconSelectNode, UiImageNode, UiInputNode, UiKeyValueNode, UiNumberStepperNode, UiRingNode, UiSelectItem, UiSelectNode, UiSliderNode, UiTextNode, UiToggleNode};
use semio_framework_ui_declaration::ui::{UiControlNode, UiFieldNode, UiNode, UiSectionNode, UiState, UiStatus, UiTreeItemNode, UiTreeNode, UI_INSPECTOR_MIXED_PLACEHOLDER};
use semio_framework_ui_draw_contract::{DrawList, IconAtlas};
use semio_framework_ui_theme::geometry::Rect;
use semio_framework_ui_wgpu_layout::layout::tree_section_header_height;
use semio_framework_ui_wgpu_layout::layout::TreeRowMetrics;
use semio_framework_ui_text::FontAtlas;
use semio_framework_ui_theme::{Level, Rgba, Theme};
use semio_framework_ui_retained_tree::tree::EditState;
use semio_framework_ui_retained_tree::tree::{NodeFlags, NodeKey, UiTree};
use semio_framework_ui_wgpu_widget_kit::draw_text_on;
use semio_framework_ui_wgpu_widget_kit::wrap_text;
use semio_framework_ui_declaration::IconName;
use semio_framework_ui_locale::Label;
use semio_framework_ui_declaration::ui::UiTreeActionPlacement;
use semio_framework_ui_wgpu_controls::stepper::{push_stepper_chrome, push_stepper_icon};
use semio_framework_ui_wgpu_paint::{TREE_ICON_SIZE, TREE_INDENT_PER_LEVEL, TREE_TOGGLE_WIDTH, celebrate_color, celebrate_stroke, celebrate_turns, chrome_band_rect, color_input_swatch, field_paint_metrics, focus_ring_visible, node_selection_outline, paint_checkbox, paint_loading_border, paint_progress, paint_separator, paint_slider_travel, paint_stack_frame, paint_waiting_border, presence_overlay, section_paint_metrics, select_menu_top_for, select_popup_row_rect};

/// 📐️ Every retained chrome metric below is the generated token's px value, never a hand-typed one
/// — `PANEL_HEADER` is `--size-medium` (`chrome.panelHeaderHeightUiSpacing`, 22.4px, what React's
/// section/group headers get), the tree metrics are `dom.tree*UiSpacing`, and the row icon is the
/// literal `12` the Interpreter passes (`chrome::ICON_TREE_ROW`).
pub const PANEL_HEADER: f32 = (ui_styling::metrics::chrome::UI_SPACING_COMPACT_PX * ui_styling::metrics::chrome::PANEL_HEADER_HEIGHT_UI_SPACING) as f32;

/// 🖼️ Top-level entry point: unconditionally walks and (re)paints every node reachable from `root`,
/// clearing `DIRTY_PAINT` as it visits (mirroring `flex::LayoutEngine::write_back`'s clear-as-you-go
/// pattern) but never touching `DIRTY_LAYOUT`/`SUBTREE_DIRTY` — clearing those is `flex`'s job and
/// `flex::LayoutEngine::compute` already runs (and clears them) before paint each frame, per the
/// intended pipeline. Deliberately has **no internal early-out**: `DrawList` only supports a full
/// clear-and-rebuild (no API to remove/replace a single dirty subtree's prior draw calls while
/// leaving clean siblings' draw calls in place), so a genuinely incremental repaint isn't safe to
/// build yet. Whether to call `paint_tree` at all this frame — i.e. "was anything dirty" — is a
/// decision a later milestone's `engine` facade owns (it already knows from driving `flex::compute`
/// and `reconcile::apply_tree`), not something `paint_tree` decides for itself.
/// 🎬️ `has_scene_host` gates the `ComponentScene`/`Image` leaf arms below (see `paint_node`'s own
/// match): when the caller's `engine::Ui::frame` has a real `scene_slots::SceneHost` for this tick,
/// those leaves paint NOTHING here — the host paints the real content into the same rect right after
/// this call, in `Ui::frame`'s `collect_scene_slots` loop — instead of this pass drawing placeholder
/// chrome that the host would then have to paint over. With no host (`false`), behavior is unchanged
/// from before this parameter existed: `paint_component_scene`/`paint_image`'s own placeholder chrome.
pub fn paint_tree(tree: &mut UiTree, root: NodeId, theme: &Theme, atlas: &mut FontAtlas, icons: Option<&IconAtlas>, has_scene_host: bool, draw: &mut DrawList) {
    sync_interactive_state(tree, root, theme);
    paint_node(tree, root, 0.0, 0.0, theme, atlas, icons, has_scene_host, draw);
    clear_dirty_paint(tree, root);
}

pub fn clear_dirty_paint(tree: &mut UiTree, id: NodeId) {
    if let Some(node) = tree.node_mut(id) {
        node.flags.set(NodeFlags::DIRTY_PAINT, false);
    }
    let children: Vec<NodeId> = tree.children(id).collect();
    for child in children {
        clear_dirty_paint(tree, child);
    }
}

/// 🔎️ Finds `parent`'s retained child keyed `key` — `reconcile`'s synthesized `Select`/`Tree` rows
/// are keyed by stable identity (`item.value`/`section.id`/`item.id`, see `reconcile::children_of`),
/// so this is how this pass re-associates a declarative row (`UiSelectItem`/`UiTreeItemNode`) with
/// its already-existing retained `NodeId`, robust to reconcile's insertion-order quirks (a re-used
/// matched child physically keeps its old sibling-list position — see that module's own doc comment
/// on why key lookup, not positional indexing, is the safe way to do this).
pub fn find_child_by_key(tree: &UiTree, parent: NodeId, key: &NodeKey) -> Option<NodeId> {
    tree.children(parent).find(|&child| tree.node(child).map(|n| &n.key) == Some(key))
}

pub fn sync_interactive_state(tree: &mut UiTree, id: NodeId, theme: &Theme) {
    sync_interactive_state_node(tree, id, theme);
    let children: Vec<NodeId> = tree.children(id).collect();
    for child in children {
        sync_interactive_state(tree, child, theme);
    }
}

pub fn sync_interactive_state_node(tree: &mut UiTree, id: NodeId, theme: &Theme) {
    let accepted = tree.accepted_layout(id);
    let select_open: Option<(Vec<UiSelectItem>, f32, f32)> = tree.node(id).and_then(|node| match &node.spec.0 {
        UiNode::Select(select) if node.state.open => accepted.map(|layout| (select.items.clone(), layout.width, layout.height)),
        _ => None,
    });
    if let Some((items, select_w, select_h)) = select_open {
        sync_select_popup_rows(tree, id, &items, select_w, select_h, theme);
    }

    let stack_drop_target: Option<bool> = tree.node(id).and_then(|node| match &node.spec.0 {
        UiNode::Stack(stack) => Some(stack.drop_action.is_some()),
        _ => None,
    });
    if let Some(accepts_drop) = stack_drop_target {
        if let Some(node) = tree.node_mut(id) {
            node.flags.set(NodeFlags::DROP_TARGET, accepts_drop);
        }
    }

    if tree.node(id).is_some_and(|node| matches!(node.spec.0, UiNode::Tree(_))) {
        sync_tree_row_drag_sources(tree, id);
    }
}

pub fn sync_select_popup_rows(tree: &mut UiTree, select_id: NodeId, items: &[UiSelectItem], select_w: f32, select_h: f32, theme: &Theme) {
    for (index, item) in items.iter().enumerate() {
        let Some(row_id) = find_child_by_key(tree, select_id, &NodeKey::Explicit(item.value.clone())) else { continue };
        let rect = select_popup_row_rect(select_w, index, select_h + semio_framework_ui_select_behavior::SELECT_SIDE_OFFSET, theme);
        if let Some(node) = tree.node_mut(row_id) {
            node.layout.x = rect.x;
            node.layout.y = rect.y;
            node.layout.width = rect.w;
            node.layout.height = rect.h;
        }
    }
}

/// 🌳️ Keeps every `Tree` row's `NodeFlags::DRAG_SOURCE` synced with its authored `draggable`
/// (see `events::is_plain_stack_container`/`set_drag_payload`, the two consumers of that bit) —
/// the `cfg(test)` twin of `sync_interactive_state_node_step`'s `Tree*` phases. Row GEOMETRY is
/// deliberately absent here: `mounted_layout` is this target's single writer of tree row rects, so
/// that layout and paint can never disagree (`📓️wgpu-tree-row-hit-test-2026-09-12.md`).
pub fn sync_tree_row_drag_sources(tree: &mut UiTree, tree_id: NodeId) {
    let Some(tree_node) = tree.node(tree_id).and_then(|node| match &node.spec.0 {
        UiNode::Tree(tree_node) => Some(tree_node.clone()),
        _ => None,
    }) else {
        return;
    };
    for section in &tree_node.sections {
        let Some(section_id) = find_child_by_key(tree, tree_id, &NodeKey::Explicit(section.id.clone())) else { continue };
        for item in &section.items {
            sync_tree_item_drag_source(tree, section_id, item);
        }
    }
}

pub fn sync_tree_item_drag_source(tree: &mut UiTree, parent: NodeId, item: &UiTreeItemNode) {
    if !item.presence.visible() {
        return;
    }
    let Some(item_id) = find_child_by_key(tree, parent, &NodeKey::Explicit(item.id.clone())) else { return };
    if let Some(node) = tree.node_mut(item_id) {
        node.flags.set(NodeFlags::DRAG_SOURCE, item.draggable.unwrap_or(false));
    }
    for nested in item.items.iter().flatten() {
        sync_tree_item_drag_source(tree, item_id, nested);
    }
}

pub fn paint_node(tree: &UiTree, id: NodeId, origin_x: f32, origin_y: f32, theme: &Theme, atlas: &mut FontAtlas, icons: Option<&IconAtlas>, has_scene_host: bool, draw: &mut DrawList) {
    paint_node_self(tree, id, origin_x, origin_y, theme, atlas, icons, has_scene_host, draw);
    let Some(node) = tree.node(id) else { return };
    let Some(layout) = tree.accepted_layout(id) else { return };
    if !node.spec.0.presence().visible() {
        return;
    }
    let abs_x = origin_x + layout.x;
    let abs_y = origin_y + layout.y;
    match &node.spec.0 {
        UiNode::Stack(_) | UiNode::Field(_) | UiNode::Section(_) | UiNode::Group(_) => paint_stack(tree, id, abs_x, abs_y, theme, atlas, icons, has_scene_host, draw),
        UiNode::Select(select) => paint_select(select, Rect::new(abs_x, abs_y, layout.width, layout.height), node.flags, node.state.open, Some((tree, id)), theme, atlas, icons, draw),
        UiNode::KeyValue(key_value) => paint_key_value(key_value, Rect::new(abs_x, abs_y, layout.width, layout.height), theme, atlas, draw),
        UiNode::IconSelect(select) => paint_icon_select(select, Rect::new(abs_x, abs_y, layout.width, layout.height), node.flags, theme, atlas, icons, draw),
        UiNode::Tree(tree_node) => paint_tree_widget(tree_node, Rect::new(abs_x, abs_y, layout.width, layout.height), theme, atlas, icons, draw),
        _ => {}
    }
}

pub fn paint_node_self(tree: &UiTree, id: NodeId, origin_x: f32, origin_y: f32, theme: &Theme, atlas: &mut FontAtlas, icons: Option<&IconAtlas>, has_scene_host: bool, draw: &mut DrawList) {
    let Some(node) = tree.node(id) else { return };
    let Some(layout) = tree.accepted_layout(id) else { return };
    let presence = node.spec.0.presence();
    if !presence.visible() {
        return;
    }
    let abs_x = origin_x + layout.x;
    let abs_y = origin_y + layout.y;
    let bounds = Rect::new(abs_x, abs_y, layout.width, layout.height);
    if matches!(presence.status, UiStatus::Loading | UiStatus::Waiting) {
        draw.push_solid([bounds.x + theme.padding_standard, bounds.y + theme.padding_standard, (bounds.w - theme.padding_standard * 2.0).max(0.0), (bounds.h - theme.padding_standard * 2.0).max(0.0)], theme.button_hover);
        presence_overlay(draw, bounds, theme, presence, node_selection_outline(&node.spec.0));
        return;
    }
    // 🖱️ Authored `presence.hover` (default false) composes with live pointer hover: every variant's
    // own paint already reads `NodeFlags::HOVERED` for its hover-aware fill, so folding the authored
    // flag in here — suppressed while disabled, matching `events::EventRouter`'s own suppression —
    // makes it effective everywhere for free, with no per-variant paint changes.
    let mut flags = node.flags;
    if presence.state != UiState::Disabled {
        flags.set(NodeFlags::HOVERED, flags.contains(NodeFlags::HOVERED) || presence.hover);
    }
    match &node.spec.0 {
        UiNode::Stack(stack) => {
            paint_stack_frame(stack, bounds, flags, theme, draw);
        }
        UiNode::Text(text) => paint_text(text, bounds, theme, atlas, draw),
        UiNode::Separator(_) => paint_separator(bounds, theme, draw),
        UiNode::Button(button) => paint_button(button, bounds, flags, theme, atlas, icons, draw),
        UiNode::Input(input) => paint_input(input, node.state.edit.as_ref(), bounds, flags, theme, atlas, draw),
        UiNode::Select(_) => {}
        UiNode::Toggle(toggle) => paint_toggle(toggle, bounds, flags, theme, atlas, icons, draw),
        UiNode::KeyValue(_) => {}
        UiNode::Slider(slider) => paint_slider(slider, bounds, theme, atlas, draw),
        UiNode::NumberStepper(stepper) => paint_number_stepper(stepper, bounds, flags, theme, atlas, icons, draw),
        UiNode::Ring(ring) => paint_ring(ring, bounds, theme, draw),
        UiNode::IconSelect(_) => {}
        UiNode::Progress(progress) => paint_progress(progress, bounds, theme, draw),
        UiNode::Field(field) => {
            paint_field(tree, id, field, bounds, theme, atlas, draw);
        }
        UiNode::Section(section) => {
            paint_section(tree, id, section, bounds, theme, atlas, draw);
        }
        UiNode::Group(group) => {
            paint_group(group, bounds, theme, atlas, icons, draw);
        }
        UiNode::Tree(_) => {}
        // 🎬️ With a `SceneHost` registered this tick, leave these two rects untouched here —
        // `engine::Ui::frame`'s `collect_scene_slots` loop paints the real content right after this
        // pass returns. With no host, fall back to the unchanged placeholder chrome.
        UiNode::Image(image) => {
            if !has_scene_host {
                paint_image(image, bounds, theme, atlas, draw);
            }
        }
        UiNode::ComponentScene(scene) => {
            if !has_scene_host {
                paint_component_scene(scene, bounds, theme, draw);
            }
        }
        UiNode::ExternalSlot(slot) => paint_external_slot(slot, bounds, theme, atlas, draw),
    }
    presence_overlay(draw, bounds, theme, presence, node_selection_outline(&node.spec.0));
}

/// 🧱️ `Stack`'s own paint (beyond `paint_node`'s separate `paint_stack_frame` call for its
/// `activate`/`selected` affordance) is a no-operation — it's pure layout; this just recurses into its
/// retained children, each offset by this node's absolute top-left. Also reused by `Field`/`Section`,
/// whose single/`children` nested `UiNode`s reconcile already expands into retained children (see
/// `reconcile::children_of`) — `paint_stack_frame` doesn't apply to either (neither carries
/// `activate`/`selected`).
#[allow(clippy::too_many_arguments, reason = "one arg per paint context resource; grouping into a struct is a T2 restructure, out of scope")]
pub fn paint_stack(tree: &UiTree, id: NodeId, abs_x: f32, abs_y: f32, theme: &Theme, atlas: &mut FontAtlas, icons: Option<&IconAtlas>, has_scene_host: bool, draw: &mut DrawList) {
    let children: Vec<NodeId> = tree.children(id).collect();
    for child in children {
        paint_node(tree, child, abs_x, abs_y, theme, atlas, icons, has_scene_host, draw);
    }
}

pub fn paint_text(node: &UiTextNode, bounds: Rect, theme: &Theme, atlas: &mut FontAtlas, draw: &mut DrawList) {
    let emphasize = node.emphasize.unwrap_or(false);
    let size = if emphasize { theme.font_size_emphasized } else { theme.font_size_body };
    let color = theme.text;
    let lines = wrap_text(atlas, node.value.as_str(), bounds.w.max(1.0), size);
    let line_h = semio_framework_ui_text::line_height(size);
    for (index, line) in lines.iter().enumerate() {
        draw_text_on(draw, atlas, line, bounds.x, bounds.y + line_h * index as f32 + size, size, color);
    }
}

pub fn paint_button(node: &UiButtonNode, bounds: Rect, flags: NodeFlags, theme: &Theme, atlas: &mut FontAtlas, icons: Option<&IconAtlas>, draw: &mut DrawList) {
    // 🚫️ `disabled:opacity-50` is the shared dimming convention this codebase's React reference
    // (`ui/js/react/index.tsx`'s form controls) uses for every disabled interactive control; ported
    // here via `Rgba::with_alpha` since `paint` has no CSS to lean on. A disabled control also can't
    // be hovered — `widgets::render_button` has no `disabled` concept at all (see this region's own
    // doc comment on why `widgets` is an incomplete reference for this specific fixture), so this is
    // an independent, `UiButtonNode.disabled`-driven fix rather than a widgets port.
    let disabled = node.presence.state == UiState::Disabled;
    let hovered = !disabled && flags.contains(NodeFlags::HOVERED);
    // 🎯️ `formControlFocusBorderClass`'s `focus-visible:border-accent` (`ui/js/react/index.tsx`,
    // applied to every form-control primitive including `Button`) — `widgets::render_button` never
    // implemented a focus ring either (only `render_input` did), so this is another independent
    // React-sourced fix, mirroring `paint_input`'s own established border-swap convention.
    let focused = !disabled && focus_ring_visible(flags);
    let dim = |color: Rgba| if disabled { color.with_alpha(color.a * 0.5) } else { color };
    let bg = dim(item_bg(theme, false, hovered));
    let border = if focused { theme.accent } else { theme.border_normal };
    push_control_border(draw, bounds, theme, dim(border), bg);
    let mut text_x = bounds.x + theme.padding_standard;
    let icon_key = if node.icon_id == IconName::CircleDot { node.label.as_str() } else { node.icon_id.as_str() };
    if let Some(icons) = icons {
        if icons.icon_uv(icon_key).is_some() {
            push_icon(draw, icons, icon_key, text_x, bounds.y + (bounds.h - ICON_TINY) * 0.5, ICON_TINY, dim(item_text(theme, false, hovered)));
            text_x += ICON_TINY + theme.gap_standard;
        }
    }
    draw_text_on(draw, atlas, node.label.as_str(), text_x, bounds.y + (bounds.h + theme.font_size_body) * 0.5 - 2.0, theme.font_size_body, dim(item_text(theme, false, hovered)));
}

/// ↔ Local mirror of `events::selection_bounds` — `anchor..caret` as `(start, end)` regardless of
/// which is smaller (see `tree::EditState`'s own doc comment). Duplicated rather than imported
/// across the `paint`/`events` module boundary for a one-line pure function; keep the two in sync
/// if `EditState`'s selection convention ever changes.
pub fn edit_selection_bounds(anchor: usize, caret: usize) -> (usize, usize) {
    (anchor.min(caret), anchor.max(caret))
}

/// ✍️ `edit` is `node.state.edit` (see `tree::WidgetState`'s doc comment: `Some` only while this
/// `Input` is focused and has a live typing buffer). While present, the live `EditState::text`
/// (with any in-progress IME `composition` spliced in at the caret for preview) wins over the
/// declarative `node.value` — the same "focused buffer governs" contract `events::FocusState`
/// already establishes — since caret/selection coordinates are only meaningful against the exact
/// string they were computed from. Neither `widgets::render_input` nor React's native `<input>`
/// (whose caret/selection are rendered by the browser itself, not by application code — there is no
/// CSS/JSX to port for their exact geometry) has anything to port from, so caret/selection styling
/// (`theme.accent`) is this pass's own independent choice, kept consistent with every control's
/// focus border, which is `theme.accent` because React's shared `formControlFocusBorderClass`
/// (`🔨️modules/📝️form-control-presentation/🟦️.ts:14`) is `focus-visible:border-accent`. wgpu has no
/// `focus-visible` distinction of its own, so a pointer-focused control shows the ring React would
/// only show for keyboard focus — a bounded, documented divergence (ticket
/// 26/09/17/WGPU-RENDERER-REACT-PARITY packet W1o).
pub fn paint_input(node: &UiInputNode, edit: Option<&EditState>, bounds: Rect, flags: NodeFlags, theme: &Theme, atlas: &mut FontAtlas, draw: &mut DrawList) {
    let focused = focus_ring_visible(flags);
    let border = if focused { theme.accent } else { theme.border_normal };
    push_control_border(draw, bounds, theme, border, theme.input_bg);
    let swatch = (node.input_kind == "color").then(|| color_input_swatch(bounds, edit.map_or(node.value.as_str(), |edit| edit.text.as_str()))).flatten();
    if let Some((swatch, color, _)) = swatch {
        draw.push_rounded([swatch.x, swatch.y, swatch.w, swatch.h], color, 2.0);
    }
    let text_x = swatch.map_or(bounds.x, |(_, _, text_bounds)| text_bounds.x) + 8.0;
    let text_baseline_y = bounds.y + (bounds.h + theme.font_size_body) * 0.5 - 2.0;
    if let Some(edit) = focused.then_some(edit).flatten() {
        let (start, end) = edit_selection_bounds(edit.anchor, edit.caret);
        if start != end {
            let x0 = atlas.pen_at(&edit.text, start, theme.font_size_body);
            let x1 = atlas.pen_at(&edit.text, end, theme.font_size_body);
            let sel_h = theme.font_size_body * 1.2;
            let sel_y = bounds.y + (bounds.h - sel_h) * 0.5;
            draw.push_solid([text_x + x0, sel_y, (x1 - x0).max(1.0), sel_h], theme.accent.with_alpha(0.3));
        }
        let mut display = edit.text.clone();
        if let Some(composition) = &edit.composition {
            display.insert_str(edit.caret, composition);
        }
        draw_text_on(draw, atlas, &display, text_x, text_baseline_y, theme.font_size_body, theme.text);
        let caret_x = atlas.pen_at(&edit.text, edit.caret, theme.font_size_body);
        let caret_h = theme.font_size_body * 1.2;
        let caret_y = bounds.y + (bounds.h - caret_h) * 0.5;
        draw.push_solid([text_x + caret_x, caret_y, 1.0, caret_h], theme.accent);
        return;
    }
    let (display, muted): (&str, bool) = if node.value.is_empty() { (node.placeholder.as_ref().map(Label::as_str).unwrap_or(""), true) } else { (node.value.as_str(), false) };
    draw_text_on(draw, atlas, display, text_x, text_baseline_y, theme.font_size_body, if muted { theme.text_muted } else { theme.text });
}

/// 🔽️ `retained` is `Some((tree, id))` for a real top-level `Select` node (able to read its
/// synthesized item rows' live `NodeFlags::HOVERED` for the popup's row-hover highlight) and `None`
/// for an inline `Select` painted via `paint_control` (a `TreeItem`'s embedded control — no per-
/// control `NodeId` exists for that yet, same caveat `paint_control`'s own doc comment already
/// makes, so it always paints closed regardless of `open`). W2 wiring: `open` (from
/// `tree::WidgetState::open`, toggled by `events::EventRouter::toggle_select_popup`) now has a real
/// data source, closing the gap this function's own doc comment used to describe — when `true`, the
/// popup paints below the trigger with the exact geometry `select_popup_row_rect` also writes into
/// the rows' `LayoutBucket` (see `sync_select_popup_rows`), so clicking a row actually hit-tests.
#[allow(clippy::too_many_arguments, reason = "one arg per paint context resource; grouping into a struct is a T2 restructure, out of scope")]
pub fn paint_select(node: &UiSelectNode, bounds: Rect, flags: NodeFlags, open: bool, retained: Option<(&UiTree, NodeId)>, theme: &Theme, atlas: &mut FontAtlas, icons: Option<&IconAtlas>, draw: &mut DrawList) {
    let hovered = flags.contains(NodeFlags::HOVERED);
    // 🎯️ `SelectTrigger`'s own `formControlFocusBorderClass` (`ui/js/react/index.tsx`) swaps its
    // border to `border-accent` on `focus-visible` — mirrored via the same border-swap convention
    // `paint_input`/`paint_button` already use, since `widgets::render_select` never implemented one.
    let focused = focus_ring_visible(flags);
    let bg = if hovered { theme.button_hover } else { theme.input_bg };
    let border = if focused { theme.accent } else { theme.border_normal };
    push_control_border(draw, bounds, theme, border, bg);
    let label = node.items.iter().find(|item| item.value == node.value).map_or_else(|| node.placeholder.as_ref().map(Label::as_str).unwrap_or("Select…"), |item| item.label.as_str());
    draw_text_on(draw, atlas, label, bounds.x + theme.padding_standard, bounds.y + (bounds.h + theme.font_size_body) * 0.5 - 2.0, theme.font_size_body, theme.text);
    if let Some(icons) = icons {
        push_icon(draw, icons, "chevron-down", bounds.x + bounds.w - theme.padding_standard - ICON_TINY, bounds.y + (bounds.h - ICON_TINY) * 0.5, ICON_TINY, foreground_on_fill(theme, theme.text_element, false, hovered));
    }
    if !open {
        return;
    }
    let row_children: Vec<NodeId> = retained.map(|(tree, id)| tree.children(id).collect()).unwrap_or_default();
    let menu_h = semio_framework_ui_select_behavior::select_menu_height(node.items.len(), theme);
    let menu_top = retained.map_or(bounds.h + semio_framework_ui_select_behavior::SELECT_SIDE_OFFSET, |(tree, id)| select_menu_top_for(tree, id, bounds.h, node.items.len(), theme));
    let menu = Rect::new(bounds.x, bounds.y + menu_top, bounds.w, menu_h);
    draw.push_glass([menu.x, menu.y, menu.w, menu.h], theme.border_radius, theme.glass(Level::Menu));
    for (index, item) in node.items.iter().enumerate() {
        let relative = select_popup_row_rect(bounds.w, index, menu_top, theme);
        let row = Rect::new(bounds.x + relative.x, bounds.y + relative.y, relative.w, relative.h);
        let row_hovered = retained.zip(row_children.get(index)).is_some_and(|((tree, _), &row_id)| tree.node(row_id).is_some_and(|n| n.flags.contains(NodeFlags::HOVERED)));
        let highlighted = retained.is_some_and(|(tree, id)| tree.node(id).is_some_and(|select| select.state.highlighted == Some(index)));
        if row_hovered || highlighted || item.value == node.value {
            draw.push_rounded([row.x, row.y, row.w, row.h], theme.row_hover, theme.border_radius);
        }
        draw_text_on(draw, atlas, item.label.as_str(), row.x + theme.padding_standard, row.y + (row.h + theme.font_size_body) * 0.5 - 2.0, theme.font_size_body, theme.text);
    }
}

pub fn paint_toggle(node: &UiToggleNode, bounds: Rect, flags: NodeFlags, theme: &Theme, atlas: &mut FontAtlas, icons: Option<&IconAtlas>, draw: &mut DrawList) {
    if node.appearance == ui_contract::ToggleAppearance::Checkbox {
        paint_checkbox(node, bounds, flags, theme, ui_contract::FlowInline::Ltr, draw);
        return;
    }
    let pressed = node.presence.selected;
    let hovered = flags.contains(NodeFlags::HOVERED);
    // 🎯️ Same `formControlFocusBorderClass` border-swap as `paint_button`/`paint_select` — the icon-
    // button variant `Toggle` renders through (`ui/js/react/index.tsx`) carries it too.
    let focused = focus_ring_visible(flags);
    let bg = item_bg(theme, pressed, hovered);
    let border = if focused { theme.accent } else { theme.border_normal };
    push_control_border(draw, bounds, theme, border, bg);
    let mut content_x = bounds.x + theme.padding_standard;
    if let Some(icons) = icons {
        if icons.icon_uv(node.icon_id.as_str()).is_some() {
            push_icon(draw, icons, node.icon_id.as_str(), content_x, bounds.y + (bounds.h - ICON_TINY) * 0.5, ICON_TINY, item_text(theme, pressed, hovered));
            content_x += ICON_TINY + theme.gap_standard;
        }
    }
    if let Some(text) = &node.text {
        draw_text_on(draw, atlas, text.as_str(), content_x, bounds.y + (bounds.h + theme.font_size_body) * 0.5 - 2.0, theme.font_size_body, item_text(theme, pressed, hovered));
    }
}

pub fn paint_key_value(node: &UiKeyValueNode, bounds: Rect, theme: &Theme, atlas: &mut FontAtlas, draw: &mut DrawList) {
    let label_w = node.entries.iter().map(|entry| atlas.measure_text(entry.label.as_str(), theme.font_size_small).0).fold(0.0f32, f32::max);
    let value_x = bounds.x + label_w + theme.gap_standard * 2.0;
    let row_h = theme.control_height;
    for (index, entry) in node.entries.iter().enumerate() {
        let y = bounds.y + index as f32 * row_h;
        draw_text_on(draw, atlas, entry.label.as_str(), bounds.x, y + (row_h + theme.font_size_small) * 0.5 - 1.0, theme.font_size_small, theme.text_muted);
        draw_text_on(draw, atlas, &entry.value, value_x, y + (row_h + theme.font_size_small) * 0.5 - 1.0, theme.font_size_small, theme.text);
    }
}

pub fn paint_slider(node: &UiSliderNode, bounds: Rect, theme: &Theme, atlas: &mut FontAtlas, draw: &mut DrawList) {
    let unit_label = node.unit_label();
    let unit_width = unit_label.as_ref().map(|label| atlas.measure_text(label, theme.font_size_body).0);
    let control = semio_framework_ui_wgpu_layout::layout::slider_control_presentation(bounds, node.axis_position(node.value), 0.0, 1.0, unit_width, theme.gap_standard, ui_contract::FlowInline::Ltr);
    let presentation = control.slider;
    paint_slider_travel(node, node.value, presentation, theme, draw);
    let text = node.readout(node.value);
    let (width, _) = atlas.measure_text(&text, theme.font_size_small);
    draw_text_on(draw, atlas, &text, presentation.value_cell.x + (presentation.value_cell.w - width).max(0.0), presentation.value_cell.y + (presentation.value_cell.h + theme.font_size_small) * 0.5, theme.font_size_small, theme.text_element);
    if let (Some(label), Some(unit_cell)) = (unit_label, control.unit_cell) {
        draw_text_on(draw, atlas, &label, unit_cell.x, unit_cell.y + (unit_cell.h + theme.font_size_body) * 0.5, theme.font_size_body, theme.text_muted);
    }
}

pub fn paint_number_stepper(node: &UiNumberStepperNode, bounds: Rect, flags: NodeFlags, theme: &Theme, atlas: &mut FontAtlas, icons: Option<&IconAtlas>, draw: &mut DrawList) {
    let segments @ [minus, center, plus] = semio_framework_ui_wgpu_layout::layout::number_stepper_segments(bounds, ui_contract::FlowInline::Ltr, theme.stroke_hairline);
    draw.push_scissor(bounds);
    push_stepper_chrome(draw, bounds, segments, ui_contract::FlowInline::Ltr, flags, None, theme);
    push_stepper_icon(draw, icons, "minus", minus, ui_contract::FlowInline::Ltr, theme);
    let (text, color) = if node.uniform { (node.value_text(node.value), theme.text) } else { (UI_INSPECTOR_MIXED_PLACEHOLDER.to_string(), theme.text_muted) };
    let width = atlas.measure_text(&text, theme.font_size_body).0;
    draw_text_on(draw, atlas, &text, center.x + (center.w - width).max(0.0) * 0.5, center.y + (center.h + theme.font_size_body) * 0.5 - 2.0, theme.font_size_body, color);
    push_stepper_icon(draw, icons, "plus", plus, ui_contract::FlowInline::Ltr, theme);
    draw.pop_scissor();
}

pub fn paint_ring(node: &UiRingNode, bounds: Rect, theme: &Theme, draw: &mut DrawList) {
    let cx = bounds.x + bounds.w * 0.5;
    let cy = bounds.y + bounds.h * 0.5;
    let radius = bounds.w.min(bounds.h) * 0.4;
    let segments = 48usize;
    let mut points = Vec::with_capacity(segments + 1);
    for i in 0..=segments {
        let angle = std::f32::consts::TAU * i as f32 / segments as f32;
        points.push([cx + angle.cos() * radius, cy + angle.sin() * radius]);
    }
    for window in points.windows(2) {
        draw.push_line(window[0][0], window[0][1], window[1][0], window[1][1], theme.separator, 2.0);
    }
    let disabled = node.presence.state == UiState::Disabled;
    let knob_angle = std::f32::consts::TAU * node.t as f32;
    let kx = cx + knob_angle.cos() * radius;
    let ky = cy + knob_angle.sin() * radius;
    let accent = if disabled { theme.text_muted } else { theme.accent };
    draw.push_rounded([kx - 6.0, ky - 6.0, 12.0, 12.0], accent, 6.0);
}

pub fn paint_icon_select(node: &UiIconSelectNode, bounds: Rect, flags: NodeFlags, theme: &Theme, atlas: &mut FontAtlas, icons: Option<&IconAtlas>, draw: &mut DrawList) {
    let hovered = flags.contains(NodeFlags::HOVERED);
    // 🎯️ Same border-swap-on-focus convention as `paint_button`/`paint_select`/`paint_toggle` — the
    // real `IconSelector` (`ui/js/react/index.tsx`) nests a `Select` for its mode picker, which
    // inherits `formControlFocusBorderClass` the same way.
    let focused = focus_ring_visible(flags);
    let border = if focused { theme.accent } else { theme.border_normal };
    push_control_border(draw, bounds, theme, border, chrome_item_bg(theme, false, hovered));
    let content_x = bounds.x + theme.padding_standard;
    let has_icon = icons.and_then(|icons| icons.icon_uv(&node.value)).is_some();
    if let (true, Some(icons)) = (has_icon, icons) {
        push_icon(draw, icons, &node.value, content_x, bounds.y + (bounds.h - ICON_TINY) * 0.5, ICON_TINY, theme.text_element);
    } else {
        draw_text_on(draw, atlas, &node.value, content_x, bounds.y + (bounds.h + theme.font_size_body) * 0.5 - 2.0, theme.font_size_body, theme.text);
    }
}

/// 📝️ Paints the label, description, and error in the accepted `Field` chrome bands. The retained
/// child paints the control between the description and error bands.
pub fn paint_field(tree: &UiTree, id: NodeId, node: &UiFieldNode, bounds: Rect, theme: &Theme, atlas: &mut FontAtlas, draw: &mut DrawList) {
    let metrics = field_paint_metrics(tree, id, node, bounds, theme, atlas);
    let label = chrome_band_rect(bounds, metrics.label.expect("field label band"));
    semio_framework_ui_wgpu_widget_kit::draw_text_weighted(draw, atlas, node.label.as_str(), label.x, label.y + semio_framework_ui_wgpu_layout::flex::FIELD_LABEL_FONT_SIZE, semio_framework_ui_wgpu_layout::flex::FIELD_LABEL_FONT_SIZE, theme.text, semio_framework_ui_text::TextWeight::Semibold);
    if node.required.unwrap_or(false) {
        let (label_w, _) = atlas.measure_text(node.label.as_str(), semio_framework_ui_wgpu_layout::flex::FIELD_LABEL_FONT_SIZE);
        semio_framework_ui_wgpu_widget_kit::draw_text_weighted(
            draw,
            atlas,
            "*",
            label.x + label_w + theme.gap_standard * 0.5,
            label.y + semio_framework_ui_wgpu_layout::flex::FIELD_LABEL_FONT_SIZE,
            semio_framework_ui_wgpu_layout::flex::FIELD_LABEL_FONT_SIZE,
            theme.error,
            semio_framework_ui_text::TextWeight::Semibold,
        );
    }
    if let Some((description, band)) = node.description.as_ref().zip(metrics.description) {
        let band = chrome_band_rect(bounds, band);
        for (line, value) in wrap_text(atlas, description, band.w.max(1.0), semio_framework_ui_wgpu_layout::flex::FIELD_DETAIL_FONT_SIZE).iter().enumerate() {
            draw_text_on(draw, atlas, value, band.x, band.y + semio_framework_ui_text::line_height(semio_framework_ui_wgpu_layout::flex::FIELD_DETAIL_FONT_SIZE) * (line as f32 + 1.0), semio_framework_ui_wgpu_layout::flex::FIELD_DETAIL_FONT_SIZE, theme.text_muted);
        }
    }
    if let Some((error, band)) = node.error.as_ref().zip(metrics.error) {
        let band = chrome_band_rect(bounds, band);
        for (line, value) in wrap_text(atlas, error, band.w.max(1.0), semio_framework_ui_wgpu_layout::flex::FIELD_DETAIL_FONT_SIZE).iter().enumerate() {
            draw_text_on(draw, atlas, value, band.x, band.y + semio_framework_ui_text::line_height(semio_framework_ui_wgpu_layout::flex::FIELD_DETAIL_FONT_SIZE) * (line as f32 + 1.0), semio_framework_ui_wgpu_layout::flex::FIELD_DETAIL_FONT_SIZE, theme.error);
        }
    }
}

/// 📂️ Paints a wrapped semibold `Section` title in the accepted title band. Retained children paint
/// below the title and body margin.
pub fn paint_section(tree: &UiTree, id: NodeId, node: &UiSectionNode, bounds: Rect, theme: &Theme, atlas: &mut FontAtlas, draw: &mut DrawList) {
    let Some(label) = &node.label else { return };
    let Some(title) = section_paint_metrics(tree, id, node, bounds, atlas).title else { return };
    let title = chrome_band_rect(bounds, title);
    for (line, value) in wrap_text(atlas, label.as_str(), title.w.max(1.0), semio_framework_ui_wgpu_layout::flex::SECTION_TITLE_FONT_SIZE).iter().enumerate() {
        semio_framework_ui_wgpu_widget_kit::draw_text_weighted(
            draw,
            atlas,
            value,
            title.x,
            title.y + semio_framework_ui_wgpu_layout::flex::SECTION_TITLE_FONT_SIZE + semio_framework_ui_wgpu_layout::flex::SECTION_TITLE_LINE_HEIGHT * line as f32,
            semio_framework_ui_wgpu_layout::flex::SECTION_TITLE_FONT_SIZE,
            theme.text,
            semio_framework_ui_text::TextWeight::Semibold,
        );
    }
}

/** 🌿️ Same header chrome as {@link paint_section} (chevron + label), for a `Group`'s always-
 * present `label` — used when a nested subtree (e.g. `Origin`) is painted directly in the native
 * retained tree rather than pre-expanded into `UiTreeItemNode.items`. */
pub fn paint_group(node: &UiGroupNode, bounds: Rect, theme: &Theme, atlas: &mut FontAtlas, icons: Option<&IconAtlas>, draw: &mut DrawList) {
    let collapsed = !node.default_open.unwrap_or(true);
    let chevron = if collapsed { "chevron-right" } else { "chevron-down" };
    if let Some(icons) = icons {
        push_icon(draw, icons, chevron, bounds.x, bounds.y + (PANEL_HEADER - ICON_TINY) * 0.5, ICON_TINY, theme.text_element);
    }
    draw_text_on(draw, atlas, node.label.as_str(), bounds.x + TREE_TOGGLE_WIDTH + theme.gap_standard, bounds.y + (PANEL_HEADER + theme.font_size_body) * 0.5 - 2.0, theme.font_size_body, theme.text);
}

pub fn paint_tree_widget(node: &UiTreeNode, bounds: Rect, theme: &Theme, atlas: &mut FontAtlas, icons: Option<&IconAtlas>, draw: &mut DrawList) {
    draw.push_scissor(bounds);
    let metrics = TreeRowMetrics::from_theme(theme);
    let mut y = bounds.y;
    for section in &node.sections {
        if let Some(label) = &section.label {
            // 🗂️ `widgets::render_tree_section_header` draws a folder icon before the label and
            // dims the label to `text_muted` only while collapsed (`text_element` otherwise) —
            // ported here; previously this always used `text_muted` regardless of collapsed state.
            let collapsed = !semio_framework_ui_wgpu_layout::layout::tree_section_default_open(section);
            let text_color = if collapsed { theme.text_muted } else { theme.text_element };
            let label_x = bounds.x + TREE_TOGGLE_WIDTH + theme.gap_standard;
            if let Some(icons) = icons {
                push_icon(draw, icons, "folder", label_x, y + (metrics.header_height - TREE_ICON_SIZE) * 0.5, TREE_ICON_SIZE, text_color);
            }
            draw_text_on(draw, atlas, label.as_str(), label_x + TREE_ICON_SIZE + theme.gap_standard, y + (metrics.header_height + theme.font_size_small) * 0.5 - 2.0, theme.font_size_small, text_color);
            y += tree_section_header_height(section, &metrics);
        }
        for item in &section.items {
            y = paint_tree_item(item, bounds.x, bounds.w, y, 1, node, theme, atlas, icons, draw, &[]);
        }
    }
    draw.pop_scissor();
    // 🧭️ Status/selected/introducing rings for the whole `Tree` are drawn once, centrally, by
    // `paint_node`'s shared `presence_overlay` — not duplicated here.
}

/// 🌳️ Recursive row painter for one `Tree` item (and, if expanded, its nested `items`). Ports every
/// piece of `widgets::render_tree_item`'s visual structure that depends only on static retained data
/// (ancestor guide lines, selected/highlighted text color, description text, always-visible actions,
/// an inline `control`) — anything that depends on *live* hover/drag/focus state (row hover fill,
/// hover-revealed actions, hover-highlighted action icons, drag guides) stays out of scope: there is
/// no per-tree-row `NodeId`/`NodeFlags` yet (`reconcile::children_of` doesn't expand `Tree` into
/// retained item children — see `paint_select`'s neighboring doc comment for the same root cause), so
/// there is nowhere to read a live per-row hover/drag flag from until that reconcile expansion lands.
#[allow(clippy::too_many_arguments, reason = "one arg per paint context resource; grouping into a struct is a T2 restructure, out of scope")]
pub fn paint_tree_item(item: &UiTreeItemNode, x: f32, width: f32, y: f32, depth: u32, tree_node: &UiTreeNode, theme: &Theme, atlas: &mut FontAtlas, icons: Option<&IconAtlas>, draw: &mut DrawList, is_last_at_level: &[bool]) -> f32 {
    if !item.presence.visible() {
        return y;
    }
    let metrics = TreeRowMetrics::from_theme(theme);
    let row = Rect::new(x, y, width, metrics.row_height);
    let selected = item.presence.selected;
    let previewed = item.presence.state == UiState::Previewed;
    let on_hover_fill = previewed || item.presence.hover;
    let dimmed = item.dimmed.unwrap_or(false) || item.presence.state == UiState::Disabled;
    if selected {
        draw.push_rounded([row.x, row.y, row.w, row.h], theme.selected, theme.border_radius);
    } else if on_hover_fill {
        draw.push_rounded([row.x, row.y, row.w, row.h], theme.row_hover, theme.border_radius);
    }
    let ring_color = if selected { theme.selected } else { theme.border_normal };
    match item.presence.status {
        UiStatus::Loading => paint_loading_border(draw, row, ring_color, theme),
        UiStatus::Waiting => paint_waiting_border(draw, row, ring_color, theme),
        UiStatus::Finished => draw.push_finished_border([row.x, row.y, row.w, row.h], ring_color, theme.border_radius, theme.stroke_hairline),
        UiStatus::Idle => {}
    }
    if item.presence.state == UiState::Introducing || item.presence.state == UiState::Celebrating {
        let celebrating = item.presence.state == UiState::Celebrating;
        let turns = if celebrating { celebrate_turns(draw.clock_seconds(), theme.celebrate_duration_seconds) } else { 0.0 };
        let ring = if celebrating { celebrate_color(theme, turns) } else { theme.accent };
        let stroke = if celebrating { celebrate_stroke(theme, turns) } else { theme.stroke_hairline };
        draw.push_introducing_border([row.x, row.y, row.w, row.h], ring, theme.border_radius, stroke);
    }
    paint_tree_guides(draw, x, row.y, row.h, depth, is_last_at_level, theme);
    let indent = x + (depth - 1) as f32 * TREE_INDENT_PER_LEVEL + TREE_TOGGLE_WIDTH;
    let expandable = item.items.as_ref().is_some_and(|items| !items.is_empty());
    if expandable {
        if let Some(icons) = icons {
            let chevron = if item.default_open.unwrap_or(false) { "chevron-down" } else { "chevron-right" };
            push_icon(draw, icons, chevron, indent - TREE_TOGGLE_WIDTH, row.y + (metrics.row_height - SIZE_TINY) * 0.5, SIZE_TINY, foreground_on_fill(theme, theme.text_element, selected, on_hover_fill));
        }
    }
    // 🎨️ `widgets::render_tree_item`'s `text_color`: selected/previewed rows use `active_foreground`
    // for both icon tint and label (previously this always used `text_element`/`theme.text`);
    // `dimmed` (the eye-toggle "hidden in scene" domain flag, or `presence.state == Disabled`) halves
    // its alpha without skipping the row — it stays visible and clickable to un-hide/re-enable.
    let text_color = foreground_on_fill(theme, item.tone.and_then(|tone| theme.tone_ink(tone)).unwrap_or(theme.text_element), selected, on_hover_fill);
    let text_color = if dimmed { text_color.with_alpha(text_color.a * 0.5) } else { text_color };
    if let (Some(icons), Some(icon_id)) = (icons, item.icon_id) {
        push_icon(draw, icons, icon_id.as_str(), indent, row.y + (metrics.row_height - TREE_ICON_SIZE) * 0.5, TREE_ICON_SIZE, text_color);
    }
    let label_x = indent + if item.icon_id.is_some() { TREE_ICON_SIZE + theme.gap_standard } else { 0.0 };
    draw_text_on(draw, atlas, item.label.as_str(), label_x, row.y + (metrics.row_height + theme.font_size_body) * 0.5 - 2.0, theme.font_size_body, text_color);
    if let Some(description) = &item.description {
        let (label_w, _) = atlas.measure_text(item.label.as_str(), theme.font_size_body);
        draw_text_on(draw, atlas, description, label_x + label_w + theme.gap_standard, row.y + (metrics.row_height + theme.font_size_small) * 0.5 - 1.0, theme.font_size_small, foreground_on_fill(theme, theme.text_muted, selected, on_hover_fill));
    }
    let mut actions_x = row.x + row.w - theme.gap_standard;
    if let Some(icons) = icons {
        for action in item.actions.iter().flatten().rev() {
            if action.placement() == UiTreeActionPlacement::Menu {
                continue;
            }
            actions_x -= TREE_ICON_SIZE + theme.padding_standard;
            let color = foreground_on_fill(theme, theme.text_element, selected, on_hover_fill);
            let color = if action.disabled { color.with_alpha(color.a * 0.5) } else { color };
            push_icon(draw, icons, action.icon_id.as_str(), actions_x, row.y + (metrics.row_height - TREE_ICON_SIZE) * 0.5, TREE_ICON_SIZE, color);
        }
    }
    // 🎛️ An inline per-row control (e.g. a small toggle/select embedded in a tree row), static data
    // already present on `UiTreeItemNode` that the old paint pass never rendered at all.
    if let Some(control) = &item.control {
        let control_height = semio_framework_ui_wgpu_layout::layout::tree_inline_control_height(control, theme);
        let local = semio_framework_ui_wgpu_layout::layout::tree_row_control_rect_with_height(row.w, control_height, &metrics);
        let control_rect = Rect::new(row.x + local.x, row.y + local.y, local.w, local.h);
        paint_control(control, control_rect, theme, atlas, icons, draw);
    }
    let mut next_y = y + metrics.row_height;
    if expandable && item.default_open.unwrap_or(false) {
        for (index, child) in item.items.as_ref().unwrap().iter().enumerate() {
            let mut child_is_last = is_last_at_level.to_vec();
            child_is_last.push(index + 1 == item.items.as_ref().unwrap().len());
            next_y = paint_tree_item(child, x, width, next_y, depth + 1, tree_node, theme, atlas, icons, draw, &child_is_last);
        }
    }
    next_y
}

/// 📏️ Ancestor connector lines for one tree row, ported from `widgets::tree_draw_guides` — adjusted
/// for `paint_tree_item`'s `depth` starting at `1` for top-level items (`widgets`' `render_tree_item`
/// starts its own `depth` at `0`), so every `widgets_depth` reference there is this function's
/// `depth - 1`.
pub fn paint_tree_guides(draw: &mut DrawList, row_x: f32, row_y: f32, row_h: f32, depth: u32, is_last_at_level: &[bool], theme: &Theme) {
    let hair = theme.stroke_hairline.max(1.0);
    let guide_color = theme.border_normal;
    for level in 0..depth.saturating_sub(1) {
        if is_last_at_level.get(level as usize).copied().unwrap_or(false) {
            continue;
        }
        let x = row_x + level as f32 * TREE_INDENT_PER_LEVEL + TREE_TOGGLE_WIDTH * 0.5;
        draw.push_solid([x, row_y, hair, row_h], guide_color);
    }
    if depth > 1 {
        let x = row_x + (depth - 2) as f32 * TREE_INDENT_PER_LEVEL + TREE_TOGGLE_WIDTH * 0.5;
        let mid_y = row_y + row_h * 0.5;
        draw.push_solid([x, row_y, hair, mid_y - row_y], guide_color);
        draw.push_solid([x, mid_y, TREE_INDENT_PER_LEVEL * 0.5, hair], guide_color);
    }
}

/// 🎛️ Adapter from a `TreeItem`'s inline `UiControlNode` payload (a narrower enum than `UiNode` —
/// see `component::ui::UiControlNode`'s own doc comment) to the matching `paint_*` function; mirrors
/// `paint_node`'s `UiNode` dispatch table one level down. No per-control `NodeId` exists for an inline
/// tree-row control yet, so it always paints at rest (`NodeFlags::empty()`) — same interactive-state
/// caveat as the rest of this function's caller.
pub fn paint_control(control: &UiControlNode, bounds: Rect, theme: &Theme, atlas: &mut FontAtlas, icons: Option<&IconAtlas>, draw: &mut DrawList) {
    let flags = NodeFlags::empty();
    match control {
        UiControlNode::Button(node) => paint_button(node, bounds, flags, theme, atlas, icons, draw),
        UiControlNode::Input(node) => paint_input(node, None, bounds, flags, theme, atlas, draw),
        UiControlNode::Select(node) => paint_select(node, bounds, flags, false, None, theme, atlas, icons, draw),
        UiControlNode::Toggle(node) => paint_toggle(node, bounds, flags, theme, atlas, icons, draw),
        UiControlNode::KeyValue(node) => paint_key_value(node, bounds, theme, atlas, draw),
        UiControlNode::Slider(node) => paint_slider(node, bounds, theme, atlas, draw),
        UiControlNode::NumberStepper(node) => paint_number_stepper(node, bounds, flags, theme, atlas, icons, draw),
        UiControlNode::Ring(node) => paint_ring(node, bounds, theme, draw),
        UiControlNode::IconSelect(node) => paint_icon_select(node, bounds, flags, theme, atlas, icons, draw),
    }
}

/// 🖼️ `paint_node`'s caller (`paint_tree`) only reaches this when `has_scene_host` is `false` this
/// tick — a real `scene_slots::SceneHost` paints the actual image content instead (see `paint_node`'s
/// `UiNode::Image` arm). No host-side texture-upload queue exists in `ui_wgpu` itself even so (that
/// lives in the renderer's `program_bridge`/`engine_canvas`, outside this crate's scope); paints a
/// raster quad keyed by `src` on the chance a caller-owned `RasterTextureTable` already has that key
/// uploaded, falling back to `alt` text when there's nothing to show yet.
pub fn paint_image(node: &UiImageNode, bounds: Rect, theme: &Theme, atlas: &mut FontAtlas, draw: &mut DrawList) {
    if node.src.is_empty() {
        if let Some(alt) = &node.alt {
            draw_text_on(draw, atlas, alt.as_str(), bounds.x + 4.0, bounds.y + 16.0, theme.font_size_small, theme.text_muted);
        }
        return;
    }
    draw.push_raster_quad(&node.src, [bounds.x, bounds.y, bounds.w, bounds.h], [0.0, 0.0, 1.0, 1.0], 1.0);
}

/// 🎬️ `paint_node`'s caller only reaches this when `has_scene_host` is `false` this tick — with a
/// real `scene_slots::SceneHost` registered, `engine::Ui::frame`'s `collect_scene_slots` loop paints
/// the actual scene surface (canvas2d/world3d/node-graph/…) into this same rect right after this
/// pass returns (see `paint_node`'s `UiNode::ComponentScene` arm), so this placeholder chrome is
/// purely the no-host fallback — "there's something visible in that rect" rather than nothing.
pub fn paint_component_scene(node: &UiComponentSceneNode, bounds: Rect, theme: &Theme, draw: &mut DrawList) {
    let _ = &node.surface_id;
    push_control_border(draw, bounds, theme, theme.border_normal, theme.panel);
}

/// 🧩️ Same placeholder-chrome treatment as `paint_component_scene`: the plugin body itself is a host
/// concern (`program_bridge`), out of scope here; label the slot with its `body_key` for now.
pub fn paint_external_slot(node: &UiExternalSlotNode, bounds: Rect, theme: &Theme, atlas: &mut FontAtlas, draw: &mut DrawList) {
    push_control_border(draw, bounds, theme, theme.border_normal, theme.panel);
    draw_text_on(draw, atlas, node.host_status.as_deref().unwrap_or(node.body_key.as_str()), bounds.x + theme.padding_standard, bounds.y + (bounds.h + theme.font_size_small) * 0.5 - 2.0, theme.font_size_small, theme.text_muted);
}
