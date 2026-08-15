//! 🪟️ framework/products/os/modules/renderer/engine/elements/Dock/component.rs — wgpu layout and
//! render implementation for the Dock element, extracted from lib.rs's inline
//! `pub mod dock { ... }` body (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE). Wired via
//! `#[path = "../../../../🧱️elements/Dock/🧊️component.rs"] pub mod dock;` in lib.rs in place of the
//! former inline block; the module name `dock` is unchanged, so every existing `crate::dock::...`
//! call site elsewhere in the crate keeps resolving with zero other changes.
//! 🪟️ Mode dock — multi-window layout tree with stack chrome and split resize.

use semio_framework::AppDefinition;
use std::collections::HashMap;
use ui_wgpu::wgpu::{
    chrome_item_text, draw_text, even_window_layout, push_chrome_group_border, ActionDescriptor, DragAxis, DrawList, FontAtlas, HitKind, HitTarget, IconAtlas, InputState, Label, Level, Locale, LocalizedLabel, Rect, Rgba,
    Terminology, Theme, UiPresence, WindowLayout, WindowLayoutChild, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode,
};

pub type DockPath = Vec<usize>;

fn empty_path() -> DockPath {
    Vec::new()
}

//#region DockTypes
#[derive(Clone, Debug, PartialEq)]
pub enum DockNode {
    Row(Vec<(DockNode, f32)>),
    Column(Vec<(DockNode, f32)>),
    Stack { windows: Vec<String>, active: String },
}

impl Default for DockState {
    fn default() -> Self {
        Self { root: DockNode::Stack { windows: vec![], active: String::new() }, active_window_id: None, maximized_stack: None, active_stack: None, split_resize_origin: vec![] }
    }
}

#[derive(Clone, Debug)]
pub struct DockState {
    pub root: DockNode,
    pub active_window_id: Option<String>,
    pub maximized_stack: Option<DockPath>,
    pub active_stack: Option<DockPath>,
    pub split_resize_origin: Vec<f32>,
}

pub struct DockRenderContext<'a> {
    pub draw: &'a mut DrawList,
    pub atlas: &'a mut FontAtlas,
    pub icons: &'a IconAtlas,
    pub input: &'a mut InputState<ActionDescriptor>,
    pub theme: &'a Theme,
    pub window_labels: &'a std::collections::HashMap<String, String>,
    pub window_icon_ids: &'a std::collections::HashMap<String, String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DockSide {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DockDropZone {
    Tab { stack_path: DockPath, index: usize },
    Split { stack_path: DockPath, side: DockSide },
    RootSplit { side: DockSide },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DockDragKind {
    Tab,
    Stack,
}

#[derive(Clone, Debug)]
pub struct DockDragPayload {
    pub kind: DockDragKind,
    pub window_id: String,
    pub source_path: DockPath,
    pub tab_index: usize,
    pub ghost_label: String,
}

#[derive(Clone, Debug)]
pub struct DockDragState {
    pub payload: DockDragPayload,
    pub x: f32,
    pub y: f32,
    pub drop_zone: Option<DockDropZone>,
}
//#endregion DockTypes

//#region DockLayout
impl DockState {
    pub fn from_app(app: &AppDefinition, active_window_id: Option<&str>) -> Self {
        let root = app.default_layout.as_ref().map(|layout| dock_from_window_layout(&layout.root)).unwrap_or_else(|| even_layout(&app.window_kinds.iter().map(|k| k.id.clone()).collect::<Vec<_>>()));
        let active = active_window_id.map(str::to_string).or_else(|| first_window_id(&root));
        let active_stack = active.as_ref().and_then(|id| find_stack_path(&root, id, &mut vec![]));
        Self { root, active_window_id: active, maximized_stack: None, active_stack, split_resize_origin: vec![] }
    }

    pub fn sync_active_window(&mut self, window_id: &str) {
        self.active_window_id = Some(window_id.to_string());
        self.active_stack = find_stack_path(&self.root, window_id, &mut vec![]);
    }

    pub fn set_stack_active(&mut self, path: &DockPath, window_id: &str) {
        if let Some(stack) = node_at_mut(&mut self.root, path) {
            if let DockNode::Stack { active, .. } = stack {
                *active = window_id.to_string();
            }
        }
        self.active_window_id = Some(window_id.to_string());
        self.active_stack = Some(path.clone());
    }

    pub fn toggle_maximize(&mut self, path: &DockPath) {
        if self.maximized_stack.as_ref() == Some(path) {
            self.maximized_stack = None;
        } else {
            self.maximized_stack = Some(path.clone());
        }
    }

    pub fn close_active_in_stack(&mut self, path: &DockPath) -> bool {
        let Some(stack) = node_at_mut(&mut self.root, path) else {
            return false;
        };
        let DockNode::Stack { windows, active } = stack else {
            return false;
        };
        if windows.len() <= 1 {
            return false;
        }
        let idx = windows.iter().position(|id| id == active).unwrap_or(0);
        windows.remove(idx);
        let next = windows.get(idx.saturating_sub(1)).or_else(|| windows.first()).cloned();
        if let Some(id) = next {
            *active = id.clone();
            self.active_window_id = Some(id);
        }
        collapse_empty(&mut self.root);
        if self.maximized_stack.as_ref() == Some(path) && !path_exists(&self.root, path) {
            self.maximized_stack = None;
        }
        true
    }

    pub fn apply_split_drag(&mut self, path: &DockPath, split_index: usize, delta_px: f32, axis_total: f32) {
        Self::apply_split_drag_on_node(&mut self.root, path, split_index, delta_px, axis_total, &self.split_resize_origin);
    }

    pub fn apply_split_drag_with_origin(&mut self, path: &DockPath, split_index: usize, delta_px: f32, axis_total: f32, origin: &[f32]) {
        Self::apply_split_drag_on_node(&mut self.root, path, split_index, delta_px, axis_total, origin);
    }

    fn apply_split_drag_on_node(root: &mut DockNode, path: &DockPath, split_index: usize, delta_px: f32, axis_total: f32, origin: &[f32]) {
        let Some(node) = node_at_mut(root, path) else {
            return;
        };
        let children = match node {
            DockNode::Row(children) | DockNode::Column(children) => children,
            DockNode::Stack { .. } => return,
        };
        if split_index + 1 >= children.len() || axis_total <= 0.0 {
            return;
        }
        let delta_frac = delta_px / axis_total;
        let origin_left = origin.get(split_index).copied().unwrap_or(children[split_index].1);
        let origin_right = origin.get(split_index + 1).copied().unwrap_or(children[split_index + 1].1);
        let new_left = (origin_left + delta_frac).clamp(0.08, 0.92);
        let new_right = (origin_right - delta_frac).clamp(0.08, 0.92);
        children[split_index].1 = new_left;
        children[split_index + 1].1 = new_right;
        normalize_pair_sizes(children, split_index);
    }

    pub fn begin_split_drag(&mut self, path: &DockPath) -> Vec<f32> {
        let sizes = match node_at(&self.root, path) {
            Some(DockNode::Row(children) | DockNode::Column(children)) => children.iter().map(|(_, s)| *s).collect(),
            _ => vec![],
        };
        self.split_resize_origin = sizes.clone();
        sizes
    }

    pub fn stack_body_rects(&self, bounds: Rect, theme: &Theme, window_labels: &HashMap<String, String>, atlas: &mut FontAtlas) -> Vec<(DockPath, Rect, String)> {
        self.stack_body_rects_with_silhouettes(bounds, theme, window_labels, atlas).0
    }

    /// 🪟️ Same as {@link Self::stack_body_rects} plus the active window's dock-stack silhouette.
    pub fn stack_body_rects_with_silhouettes(&self, bounds: Rect, theme: &Theme, window_labels: &HashMap<String, String>, atlas: &mut FontAtlas) -> (Vec<(DockPath, Rect, String)>, HashMap<String, WindowSilhouette>) {
        let mut out = Vec::new();
        let mut silhouettes = HashMap::new();
        if let Some(path) = &self.maximized_stack {
            if let Some(node) = node_at(&self.root, path) {
                let rect = bounds;
                if let DockNode::Stack { windows, active } = node {
                    let layout = layout_stack_cap(windows, window_labels, atlas, theme, rect, true);
                    let silhouette = stack_window_silhouette(rect, theme, &layout);
                    out.push((path.clone(), silhouette.safe_body_rect(), active.clone()));
                    silhouettes.insert(active.clone(), silhouette);
                }
            }
            return (out, silhouettes);
        }
        collect_stack_bodies(&self.root, bounds, &empty_path(), theme, window_labels, atlas, self, &mut out, &mut silhouettes);
        (out, silhouettes)
    }

    pub fn stack_tab_bar_rects(&self, bounds: Rect, theme: &Theme) -> Vec<(DockPath, Rect)> {
        let mut out = Vec::new();
        if self.maximized_stack.is_some() {
            if let Some(path) = &self.maximized_stack {
                let rect = bounds;
                out.push((path.clone(), stack_tab_bar_rect(rect, theme)));
            }
            return out;
        }
        collect_stack_tab_bars(&self.root, bounds, &empty_path(), theme, &mut out);
        out
    }

    pub fn tab_index(&self, path: &DockPath, window_id: &str) -> Option<usize> {
        let DockNode::Stack { windows, .. } = node_at(&self.root, path)? else {
            return None;
        };
        windows.iter().position(|id| id == window_id)
    }

    /// 🎯️ Commits a completed dock drag. `drag.window_id` is removed from `self.root` *eagerly* the
    /// moment a drag is promoted (see `ShellState::handle_pointer_move`'s `dock.remove_window` call,
    /// `shell` region) — by the time this runs it is already absent, so every branch below only
    /// *inserts* it at the resolved zone. (Previously this also re-removed it before inserting, which
    /// — given it was already gone — silently no-opped every cross-stack/side drop, or for a same-path
    /// drop, reordered whatever now-shifted window happened to sit at the stale `tab_index`. Real
    /// drops never actually landed; see `DockTests::apply_drop_*` for pinned regressions.)
    pub fn apply_drop(&mut self, drag: &DockDragPayload, zone: &DockDropZone) -> bool {
        match drag.kind {
            DockDragKind::Tab => match zone {
                DockDropZone::Tab { stack_path, index } => self.insert_tab(stack_path, &drag.window_id, Some(*index)),
                DockDropZone::Split { stack_path, side } => self.split_stack_with_window(stack_path, &drag.window_id, *side),
                DockDropZone::RootSplit { side } => self.split_root_with_window(&drag.window_id, *side),
            },
            DockDragKind::Stack => {
                // 🪟️ A stack drag pre-removes only its *active* window (mirrors the tab case) — the
                // rest of the source stack is still sitting at `drag.source_path` and has to travel
                // along with it, matching `ui/js/react/index.tsx`'s `extractStackFromLayout`/
                // `splitWithStack`/`splitRootWithStack`/`mergeStackTabsIntoStack` family.
                let same_source = matches!(
                    zone,
                    DockDropZone::Tab { stack_path, .. } | DockDropZone::Split { stack_path, .. }
                        if *stack_path == drag.source_path
                );
                if same_source {
                    return false;
                }
                // 🔑️ Resolve the target stack's identity (its current active window) *before*
                // extraction disturbs sibling indices — extracting the rest of the source stack can
                // collapse/reindex ancestors, which would otherwise strand a pre-extraction
                // `zone.stack_path` pointing at the wrong node (or nothing). Mirrors React's
                // `targetAnchorId`/`resolveStackPathForWindowId` re-anchoring in `applyModeDrop`.
                let target_anchor = match zone {
                    DockDropZone::Tab { stack_path, .. } | DockDropZone::Split { stack_path, .. } => node_at(&self.root, stack_path).and_then(|node| match node {
                        DockNode::Stack { active, .. } => Some(active.clone()),
                        _ => None,
                    }),
                    DockDropZone::RootSplit { .. } => None,
                };
                let group = self.extract_stack_group(&drag.source_path, &drag.window_id, drag.tab_index);
                if group.is_empty() {
                    return false;
                }
                match zone {
                    DockDropZone::Tab { stack_path, index } => {
                        let target_path = target_anchor.as_deref().and_then(|id| find_stack_path(&self.root, id, &mut vec![])).unwrap_or_else(|| stack_path.clone());
                        self.insert_tabs(&target_path, &group, Some(*index), &drag.window_id)
                    }
                    DockDropZone::Split { stack_path, side } => {
                        let target_path = target_anchor.as_deref().and_then(|id| find_stack_path(&self.root, id, &mut vec![])).unwrap_or_else(|| stack_path.clone());
                        self.split_stack_with_stack(&target_path, group, drag.window_id.clone(), *side)
                    }
                    DockDropZone::RootSplit { side } => self.split_root_with_stack(group, drag.window_id.clone(), *side),
                }
            }
        }
    }

    /// 🪟️ Pulls whatever remains of the source stack (every window besides the already-removed
    /// `primary_id`) out of the tree, then reconstructs the original tab order by reinserting
    /// `primary_id` at `primary_index` — the group a whole-stack drag carries to its drop zone.
    fn extract_stack_group(&mut self, source_path: &DockPath, primary_id: &str, primary_index: usize) -> Vec<String> {
        let remaining = self.stack_windows_at_path(source_path).unwrap_or_default();
        for id in &remaining {
            self.remove_window(id);
        }
        let mut group = remaining;
        let insert_at = primary_index.min(group.len());
        group.insert(insert_at, primary_id.to_string());
        group
    }

    pub fn reorder_tab(&mut self, path: &DockPath, from: usize, to: usize) -> bool {
        let Some(stack) = node_at_mut(&mut self.root, path) else {
            return false;
        };
        let DockNode::Stack { windows, .. } = stack else {
            return false;
        };
        if from >= windows.len() || to > windows.len() || from == to {
            return false;
        }
        let window_id = windows.remove(from);
        windows.insert(to.min(windows.len()), window_id);
        true
    }

    pub fn remove_window(&mut self, window_id: &str) -> bool {
        let removed = remove_window_from_node(&mut self.root, window_id);
        if removed {
            collapse_empty(&mut self.root);
            if self.active_window_id.as_deref().is_some_and(|id| id == window_id) {
                self.active_window_id = first_window_id(&self.root);
            }
        }
        removed
    }

    pub fn insert_tab(&mut self, path: &DockPath, window_id: &str, index: Option<usize>) -> bool {
        self.insert_tabs(path, &[window_id.to_string()], index, window_id)
    }

    /// 🪟️ Inserts every window in `windows` (in order) starting at `index`, skipping any already
    /// present at the target (mirrors `insert_tab`'s dedupe guard), then focuses `active_id` — the
    /// multi-window counterpart `apply_drop` uses for whole-stack tab-join drops.
    pub fn insert_tabs(&mut self, path: &DockPath, windows: &[String], index: Option<usize>, active_id: &str) -> bool {
        if windows.is_empty() {
            return false;
        }
        let Some(stack) = node_at_mut(&mut self.root, path) else {
            return false;
        };
        let DockNode::Stack { windows: target, active } = stack else {
            return false;
        };
        let mut insert_at = index.unwrap_or(target.len()).min(target.len());
        let mut inserted_any = false;
        for window_id in windows {
            if target.iter().any(|id| id == window_id) {
                continue;
            }
            target.insert(insert_at, window_id.clone());
            insert_at += 1;
            inserted_any = true;
        }
        if !inserted_any {
            return false;
        }
        *active = active_id.to_string();
        self.active_window_id = Some(active_id.to_string());
        self.active_stack = Some(path.to_vec());
        true
    }

    pub fn split_stack_with_window(&mut self, path: &DockPath, window_id: &str, side: DockSide) -> bool {
        self.split_stack_with_stack(path, vec![window_id.to_string()], window_id.to_string(), side)
    }

    /// 🪟️ Splits the stack at `path` with an already-assembled multi-window stack — the whole-stack
    /// counterpart `apply_drop` uses; single-window splits go through `split_stack_with_window` above.
    pub fn split_stack_with_stack(&mut self, path: &DockPath, windows: Vec<String>, active_id: String, side: DockSide) -> bool {
        let Some(stack_node) = node_at(&self.root, path).cloned() else {
            return false;
        };
        let DockNode::Stack { .. } = stack_node else {
            return false;
        };
        let new_stack = DockNode::Stack { windows, active: active_id.clone() };
        let replacement = match side {
            DockSide::Left | DockSide::Top => axis_pair_from_stacks(&new_stack, &stack_node, side),
            DockSide::Right | DockSide::Bottom => axis_pair_from_stacks(&stack_node, &new_stack, side),
        };
        replace_node_at(&mut self.root, path, replacement);
        self.active_window_id = Some(active_id.clone());
        self.active_stack = find_stack_path(&self.root, &active_id, &mut vec![]);
        true
    }

    pub fn split_root_with_window(&mut self, window_id: &str, side: DockSide) -> bool {
        self.split_root_with_stack(vec![window_id.to_string()], window_id.to_string(), side)
    }

    /// 🪟️ Splits the mode root with an already-assembled multi-window stack — see
    /// `split_stack_with_stack`; single-window splits go through `split_root_with_window` above.
    pub fn split_root_with_stack(&mut self, windows: Vec<String>, active_id: String, side: DockSide) -> bool {
        let current = std::mem::replace(&mut self.root, DockNode::Stack { windows: vec![], active: String::new() });
        if matches!(&current, DockNode::Stack { windows, .. } if windows.is_empty()) {
            self.root = DockNode::Stack { windows, active: active_id.clone() };
        } else {
            let new_stack = DockNode::Stack { windows, active: active_id.clone() };
            self.root = match side {
                DockSide::Left | DockSide::Top => axis_pair_from_stacks(&new_stack, &current, side),
                DockSide::Right | DockSide::Bottom => axis_pair_from_stacks(&current, &new_stack, side),
            };
        }
        self.active_window_id = Some(active_id.clone());
        self.active_stack = find_stack_path(&self.root, &active_id, &mut vec![]);
        true
    }

    /// 🔑️ Applies an incoming `WindowLayout` via keyed diff instead of a full teardown-and-rebuild:
    /// unchanged stacks/axes are reused wholesale (`diff_dock_node`), a stack that's merely reordered
    /// keeps the user's *current* tab focused instead of reverting to whatever a persisted snapshot's
    /// `active` field says, and `active_stack`/`maximized_stack` are re-resolved by window-id key
    /// against the new tree rather than reused as stale positional `DockPath`s a structural change
    /// could silently misdirect (or leave pointing at a now-out-of-bounds index). Any in-flight resize
    /// gesture is abandoned (`split_resize_origin` cleared) since a swapped layout invalidates its
    /// origin indices. See `ShellState::sync_dock`'s `self.dock.root = dock_from_window_layout(...)`
    /// teardown for the call site this is meant to replace — wiring that swap is a follow-up step for
    /// whoever next owns the `shell` region (out of this `dock` claim's bounds).
    pub fn apply_layout_diff(&mut self, layout: &WindowLayout) {
        let next_root = dock_from_window_layout(&layout.root);
        let maximized_key = self.maximized_stack.as_ref().and_then(|path| node_at(&self.root, path)).and_then(|node| match node {
            DockNode::Stack { active, .. } => Some(active.clone()),
            _ => None,
        });
        let active_key = self.active_window_id.clone();
        self.root = diff_dock_node(&self.root, next_root);
        self.active_window_id = active_key.filter(|id| find_stack_path(&self.root, id, &mut vec![]).is_some()).or_else(|| first_window_id(&self.root));
        self.active_stack = self.active_window_id.as_deref().and_then(|id| find_stack_path(&self.root, id, &mut vec![]));
        self.maximized_stack = maximized_key.and_then(|id| find_stack_path(&self.root, &id, &mut vec![]));
        self.split_resize_origin.clear();
    }

    pub fn stack_windows_at_path(&self, path: &DockPath) -> Option<Vec<String>> {
        let DockNode::Stack { windows, .. } = node_at(&self.root, path)? else {
            return None;
        };
        Some(windows.clone())
    }

    pub fn to_window_layout(&self) -> WindowLayout {
        WindowLayout { root: dock_node_to_layout_root(&self.root) }
    }

    pub fn register_hits(&self, ctx: &mut DockRenderContext<'_>, bounds: Rect) {
        if let Some(path) = &self.maximized_stack {
            if let Some(node) = node_at(&self.root, path) {
                let rect = bounds;
                if let DockNode::Stack { .. } = node {
                    render_stack(self, ctx, path, node, rect, true, true, &mut |_, _| {});
                    return;
                }
            }
        }
        render_node(self, ctx, &self.root, bounds, &empty_path(), true, &mut |_, _| {}, None);
    }

    pub fn paint_chrome(&self, ctx: &mut DockRenderContext<'_>, bounds: Rect, body_fill: bool) {
        if let Some(path) = &self.maximized_stack {
            if let Some(node) = node_at(&self.root, path) {
                if let DockNode::Stack { .. } = node {
                    render_stack(self, ctx, path, node, bounds, true, body_fill, &mut |_, _| {});
                    return;
                }
            }
        }
        render_node(self, ctx, &self.root, bounds, &empty_path(), body_fill, &mut |_, _| {}, None);
    }

    pub fn register_resize_hits(&self, ctx: &mut DockRenderContext<'_>, bounds: Rect) {
        if self.maximized_stack.is_some() {
            return;
        }
        walk_resize_hits(self, ctx, &self.root, bounds, &empty_path(), None);
    }

    pub fn split_axis_extent(&self, path: &DockPath, canvas: Rect) -> Option<f32> {
        let bounds = solve_node_bounds(&self.root, canvas, path, &empty_path())?;
        match node_at(&self.root, path)? {
            DockNode::Row(_) => Some(bounds.w.max(1.0)),
            DockNode::Column(_) => Some(bounds.h.max(1.0)),
            DockNode::Stack { .. } => None,
        }
    }
}
//#endregion DockLayout

//#region DockFreeFunctions

pub fn dock_from_window_layout(root: &WindowLayoutRoot) -> DockNode {
    match root {
        WindowLayoutRoot::Axis(axis) => axis_from_children(&axis.kind, &axis.children, axis.size),
        WindowLayoutRoot::Stack(stack) => stack_from_node(stack),
    }
}

fn axis_from_children(kind: &str, children: &[WindowLayoutChild], size: Option<f64>) -> DockNode {
    let parsed: Vec<(DockNode, f32)> = children
        .iter()
        .map(|child| match child {
            WindowLayoutChild::Axis(axis) => (axis_from_children(&axis.kind, &axis.children, axis.size), axis.size.map(|v| v as f32).unwrap_or(1.0)),
            WindowLayoutChild::Stack(stack) => (stack_from_node(stack), stack.size.map(|v| v as f32).unwrap_or(1.0)),
        })
        .collect();
    let normalized = normalize_sizes(parsed, size.map(|v| v as f32));
    if kind == "column" {
        DockNode::Column(normalized)
    } else {
        DockNode::Row(normalized)
    }
}

fn stack_from_node(stack: &WindowLayoutStackNode) -> DockNode {
    let windows: Vec<String> = stack.children.iter().map(|w| w.window_kind_id.clone()).collect();
    let active = stack.active_window_kind_id.clone().filter(|id| windows.iter().any(|w| w == id)).or_else(|| windows.first().cloned()).unwrap_or_default();
    DockNode::Stack { windows, active }
}

pub fn dock_node_to_layout_root(node: &DockNode) -> WindowLayoutRoot {
    match node {
        DockNode::Stack { windows, active } => WindowLayoutRoot::Stack(WindowLayoutStackNode {
            kind: "stack".into(),
            size: None,
            active_window_kind_id: Some(active.clone()),
            children: windows.iter().map(|id| WindowLayoutWindowNode { kind: "window".into(), window_kind_id: id.clone(), title: None, instance_id: None, template_id: None }).collect(),
        }),
        DockNode::Row(children) => WindowLayoutRoot::Axis(ui_wgpu::wgpu::WindowLayoutAxisNode { kind: "row".into(), size: None, children: children.iter().map(|(child, size)| dock_child_from_node(child, *size)).collect() }),
        DockNode::Column(children) => WindowLayoutRoot::Axis(ui_wgpu::wgpu::WindowLayoutAxisNode { kind: "column".into(), size: None, children: children.iter().map(|(child, size)| dock_child_from_node(child, *size)).collect() }),
    }
}

fn dock_child_from_node(node: &DockNode, size: f32) -> WindowLayoutChild {
    match node {
        DockNode::Stack { windows, active } => WindowLayoutChild::Stack(WindowLayoutStackNode {
            kind: "stack".into(),
            size: Some(size as f64),
            active_window_kind_id: Some(active.clone()),
            children: windows.iter().map(|id| WindowLayoutWindowNode { kind: "window".into(), window_kind_id: id.clone(), title: None, instance_id: None, template_id: None }).collect(),
        }),
        DockNode::Row(children) => WindowLayoutChild::Axis(ui_wgpu::wgpu::WindowLayoutAxisNode { kind: "row".into(), size: Some(size as f64), children: children.iter().map(|(child, child_size)| dock_child_from_node(child, *child_size)).collect() }),
        DockNode::Column(children) => {
            WindowLayoutChild::Axis(ui_wgpu::wgpu::WindowLayoutAxisNode { kind: "column".into(), size: Some(size as f64), children: children.iter().map(|(child, child_size)| dock_child_from_node(child, *child_size)).collect() })
        }
    }
}

fn axis_pair_from_stacks(first: &DockNode, second: &DockNode, side: DockSide) -> DockNode {
    let pair = vec![(first.clone(), 0.5), (second.clone(), 0.5)];
    match side {
        DockSide::Left | DockSide::Right => DockNode::Row(pair),
        DockSide::Top | DockSide::Bottom => DockNode::Column(pair),
    }
}

fn replace_node_at(root: &mut DockNode, path: &[usize], replacement: DockNode) {
    if path.is_empty() {
        *root = replacement;
        return;
    }
    if let Some((head, tail)) = path.split_first() {
        let children = match root {
            DockNode::Row(children) | DockNode::Column(children) => children,
            DockNode::Stack { .. } => return,
        };
        if tail.is_empty() {
            if let Some((slot, _)) = children.get_mut(*head) {
                *slot = replacement;
            }
        } else if let Some((slot, _)) = children.get_mut(*head) {
            replace_node_at(slot, tail, replacement);
        }
    }
}

fn remove_window_from_node(node: &mut DockNode, window_id: &str) -> bool {
    match node {
        DockNode::Stack { windows, active } => {
            if let Some(index) = windows.iter().position(|id| id == window_id) {
                windows.remove(index);
                if active == window_id {
                    *active = windows.get(index.saturating_sub(1)).or_else(|| windows.first()).cloned().unwrap_or_default();
                }
                return true;
            }
            false
        }
        DockNode::Row(children) | DockNode::Column(children) => children.iter_mut().any(|(child, _)| remove_window_from_node(child, window_id)),
    }
}

pub fn resolve_split_side(local_x: f32, local_y: f32, width: f32, height: f32) -> DockSide {
    let mid_x = width * 0.5;
    let mid_y = height * 0.5;
    let dx = (local_x - mid_x).abs();
    let dy = (local_y - mid_y).abs();
    if dx >= dy {
        if local_x < mid_x {
            DockSide::Left
        } else {
            DockSide::Right
        }
    } else if local_y < mid_y {
        DockSide::Top
    } else {
        DockSide::Bottom
    }
}

pub fn compute_tab_insert_index(pointer_x: f32, tab_bar: Rect, tab_widths: &[f32], gap: f32) -> usize {
    let mut x = tab_bar.x + gap;
    for (index, width) in tab_widths.iter().enumerate() {
        if pointer_x < x + width * 0.5 {
            return index;
        }
        x += width + gap;
    }
    tab_widths.len()
}

pub fn compute_dock_drop_zone(pointer_x: f32, pointer_y: f32, tab_bars: &[(DockPath, Rect, Vec<f32>)], bodies: &[(DockPath, Rect, String)], canvas: Rect) -> Option<DockDropZone> {
    for (path, rect, widths) in tab_bars {
        if rect.contains(pointer_x, pointer_y) {
            let index = compute_tab_insert_index(pointer_x, *rect, widths, 4.0);
            return Some(DockDropZone::Tab { stack_path: path.clone(), index });
        }
    }
    for (path, rect, _) in bodies {
        if rect.contains(pointer_x, pointer_y) {
            return Some(DockDropZone::Split { stack_path: path.clone(), side: resolve_split_side(pointer_x - rect.x, pointer_y - rect.y, rect.w, rect.h) });
        }
    }
    if canvas.contains(pointer_x, pointer_y) {
        return Some(DockDropZone::RootSplit { side: resolve_split_side(pointer_x - canvas.x, pointer_y - canvas.y, canvas.w, canvas.h) });
    }
    None
}

/// @emoji 📐️ Half-panel rectangle for split drop preview inside a stack body.
pub fn split_drop_preview_in_body(body: Rect, side: DockSide) -> Rect {
    let half_w = body.w * 0.5;
    let half_h = body.h * 0.5;
    match side {
        DockSide::Left => Rect::new(body.x, body.y, half_w, body.h),
        DockSide::Right => Rect::new(body.x + body.w - half_w, body.y, half_w, body.h),
        DockSide::Top => Rect::new(body.x, body.y, body.w, half_h),
        DockSide::Bottom => Rect::new(body.x, body.y + body.h - half_h, body.w, half_h),
    }
}

/// @emoji 🎯️ Resolves the on-canvas indicator rect for an active dock drop zone.
pub fn drop_zone_indicator_rect(zone: &DockDropZone, tab_bars: &[(DockPath, Rect, Vec<f32>)], bodies: &[(DockPath, Rect, String)], canvas: Rect, gap: f32) -> Option<Rect> {
    match zone {
        DockDropZone::Tab { stack_path, index } => {
            let (_, tab_bar, widths) = tab_bars.iter().find(|(path, _, _)| path == stack_path)?;
            let mut x = tab_bar.x + gap;
            for width in widths.iter().take(*index) {
                x += width + gap;
            }
            let preview_w = widths.get(*index).copied().unwrap_or(88.0).clamp(48.0, 120.0);
            Some(Rect::new(x, tab_bar.y + gap * 0.5, preview_w, tab_bar.h - gap))
        }
        DockDropZone::Split { stack_path, side } => {
            let (_, body, _) = bodies.iter().find(|(path, _, _)| path == stack_path)?;
            Some(split_drop_preview_in_body(*body, *side))
        }
        DockDropZone::RootSplit { side } => Some(split_drop_preview_in_body(canvas, *side)),
    }
}

fn stack_tab_bar_rect(bounds: Rect, theme: &Theme) -> Rect {
    Rect::new(bounds.x, bounds.y, bounds.w, theme.control_height)
}

fn collect_stack_tab_bars(node: &DockNode, bounds: Rect, path: &[usize], theme: &Theme, out: &mut Vec<(DockPath, Rect)>) {
    match node {
        DockNode::Row(children) => {
            let total: f32 = children.iter().map(|(_, s)| *s).sum::<f32>().max(0.001);
            let mut x = bounds.x;
            for (index, (child, size)) in children.iter().enumerate() {
                let w = bounds.w * (*size / total);
                let mut child_path = path.to_vec();
                child_path.push(index);
                collect_stack_tab_bars(child, Rect::new(x, bounds.y, w, bounds.h), &child_path, theme, out);
                x += w;
            }
        }
        DockNode::Column(children) => {
            let total: f32 = children.iter().map(|(_, s)| *s).sum::<f32>().max(0.001);
            let mut y = bounds.y;
            for (index, (child, size)) in children.iter().enumerate() {
                let h = bounds.h * (*size / total);
                let mut child_path = path.to_vec();
                child_path.push(index);
                collect_stack_tab_bars(child, Rect::new(bounds.x, y, bounds.w, h), &child_path, theme, out);
                y += h;
            }
        }
        DockNode::Stack { .. } => {
            out.push((path.to_vec(), stack_tab_bar_rect(bounds, theme)));
        }
    }
}

/// 🪟️ Wgpu-local adapter: builds the balanced fallback layout via
/// `ui_wgpu::wgpu::even_window_layout` and converts it to a runtime `DockNode`.
fn even_layout(window_ids: &[String]) -> DockNode {
    dock_from_window_layout(&even_window_layout(window_ids).root)
}

fn normalize_sizes(children: Vec<(DockNode, f32)>, axis_size: Option<f32>) -> Vec<(DockNode, f32)> {
    if children.is_empty() {
        return children;
    }
    let total: f32 = children.iter().map(|(_, s)| *s).sum();
    let scale = axis_size.unwrap_or(total).max(0.001);
    let sum = total.max(0.001);
    children.into_iter().map(|(node, size)| (node, size / sum * scale)).collect()
}

fn normalize_pair_sizes(children: &mut [(DockNode, f32)], index: usize) {
    let pair_sum = children[index].1 + children[index + 1].1;
    let total: f32 = children.iter().map(|(_, s)| *s).sum();
    if total <= 0.0 {
        return;
    }
    let a = children[index].1 / pair_sum;
    let b = children[index + 1].1 / pair_sum;
    children[index].1 = a * pair_sum;
    children[index + 1].1 = b * pair_sum;
}

fn first_window_id(node: &DockNode) -> Option<String> {
    match node {
        DockNode::Stack { active, .. } if !active.is_empty() => Some(active.clone()),
        DockNode::Row(children) | DockNode::Column(children) => children.iter().find_map(|(child, _)| first_window_id(child)),
        DockNode::Stack { .. } => None,
    }
}

fn find_stack_path(node: &DockNode, window_id: &str, path: &mut DockPath) -> Option<DockPath> {
    match node {
        DockNode::Stack { windows, .. } if windows.iter().any(|id| id == window_id) => Some(path.clone()),
        DockNode::Row(children) | DockNode::Column(children) => {
            for (index, (child, _)) in children.iter().enumerate() {
                path.push(index);
                if let Some(found) = find_stack_path(child, window_id, path) {
                    return Some(found);
                }
                path.pop();
            }
            None
        }
        DockNode::Stack { .. } => None,
    }
}

fn path_exists(node: &DockNode, path: &[usize]) -> bool {
    node_at(node, path).is_some()
}

fn node_at<'a>(node: &'a DockNode, path: &[usize]) -> Option<&'a DockNode> {
    let mut current = node;
    for index in path {
        current = match current {
            DockNode::Row(children) | DockNode::Column(children) => children.get(*index).map(|(n, _)| n)?,
            DockNode::Stack { .. } => return None,
        };
    }
    Some(current)
}

fn node_at_mut<'a>(node: &'a mut DockNode, path: &[usize]) -> Option<&'a mut DockNode> {
    if path.is_empty() {
        return Some(node);
    }
    let (head, tail) = path.split_first()?;
    let child = match node {
        DockNode::Row(children) | DockNode::Column(children) => children.get_mut(*head).map(|(n, _)| n)?,
        DockNode::Stack { .. } => return None,
    };
    node_at_mut(child, &tail.to_vec())
}

fn collapse_empty(node: &mut DockNode) {
    match node {
        DockNode::Row(children) | DockNode::Column(children) => {
            children.retain(|(child, _)| !is_empty_stack(child));
            for (child, _) in children.iter_mut() {
                collapse_empty(child);
            }
            children.retain(|(child, _)| !is_empty_node(child));
        }
        DockNode::Stack { .. } => {}
    }
}

fn is_empty_stack(node: &DockNode) -> bool {
    matches!(node, DockNode::Stack { windows, .. } if windows.is_empty())
}

fn is_empty_node(node: &DockNode) -> bool {
    match node {
        DockNode::Stack { windows, .. } => windows.is_empty(),
        DockNode::Row(children) | DockNode::Column(children) => children.is_empty(),
    }
}

/// 🔑️ Diffs an incoming `next` node (freshly parsed from a `WindowLayout`) against the corresponding
/// `old` node by stable window-id key: identical stacks/axes are reused wholesale, a stack that's
/// merely reordered keeps its old `active` tab focused (rather than reverting to whatever `next` says),
/// and a structural-kind change (e.g. `Stack` -> `Row`) just adopts `next` outright since there's no
/// shared identity to preserve. Mirrors the intent of `ui/js/react/index.tsx`'s keyed reconciliation
/// helpers (`mapLayoutStacks` family) for this crate's `DockNode` shape. See `DockState::apply_layout_diff`.
fn diff_dock_node(old: &DockNode, next: DockNode) -> DockNode {
    match (old, &next) {
        (DockNode::Stack { windows: old_windows, active: old_active }, DockNode::Stack { windows: next_windows, active: next_active }) => {
            if old_windows == next_windows {
                return old.clone();
            }
            let same_membership = old_windows.len() == next_windows.len() && old_windows.iter().all(|id| next_windows.contains(id));
            let active = if same_membership && old_windows.iter().any(|id| id == old_active) { old_active.clone() } else { next_active.clone() };
            DockNode::Stack { windows: next_windows.clone(), active }
        }
        (DockNode::Row(old_children), DockNode::Row(next_children)) => DockNode::Row(diff_axis_children(old_children, next_children)),
        (DockNode::Column(old_children), DockNode::Column(next_children)) => DockNode::Column(diff_axis_children(old_children, next_children)),
        _ => next,
    }
}

/// 🔑️ Pairwise-diffs axis children by index (the only stable positional key an `Axis` node offers) —
/// children beyond `old`'s length are new inserts adopted as-is from `next`.
fn diff_axis_children(old_children: &[(DockNode, f32)], next_children: &[(DockNode, f32)]) -> Vec<(DockNode, f32)> {
    next_children
        .iter()
        .enumerate()
        .map(|(index, (next_child, next_size))| {
            let merged = match old_children.get(index) {
                Some((old_child, _)) => diff_dock_node(old_child, next_child.clone()),
                None => next_child.clone(),
            };
            (merged, *next_size)
        })
        .collect()
}

fn solve_node_bounds(node: &DockNode, bounds: Rect, target_path: &[usize], current_path: &[usize]) -> Option<Rect> {
    if current_path == target_path {
        return Some(bounds);
    }
    match node {
        DockNode::Row(children) => {
            let total: f32 = children.iter().map(|(_, s)| *s).sum::<f32>().max(0.001);
            let mut x = bounds.x;
            for (index, (child, size)) in children.iter().enumerate() {
                let w = bounds.w * (*size / total);
                let mut path = current_path.to_vec();
                path.push(index);
                if let Some(found) = solve_node_bounds(child, Rect::new(x, bounds.y, w, bounds.h), target_path, &path) {
                    return Some(found);
                }
                x += w;
            }
            None
        }
        DockNode::Column(children) => {
            let total: f32 = children.iter().map(|(_, s)| *s).sum::<f32>().max(0.001);
            let mut y = bounds.y;
            for (index, (child, size)) in children.iter().enumerate() {
                let h = bounds.h * (*size / total);
                let mut path = current_path.to_vec();
                path.push(index);
                if let Some(found) = solve_node_bounds(child, Rect::new(bounds.x, y, bounds.w, h), target_path, &path) {
                    return Some(found);
                }
                y += h;
            }
            None
        }
        DockNode::Stack { .. } => None,
    }
}

fn render_node(state: &DockState, ctx: &mut DockRenderContext<'_>, node: &DockNode, bounds: Rect, path: &[usize], body_fill: bool, render_body: &mut dyn FnMut(Rect, &str), outer_split: Option<(DockPath, usize, bool)>) {
    match node {
        DockNode::Row(children) => render_axis(state, ctx, children, bounds, path, true, body_fill, render_body, outer_split),
        DockNode::Column(children) => render_axis(state, ctx, children, bounds, path, false, body_fill, render_body, outer_split),
        DockNode::Stack { .. } => {
            let maximized = state.maximized_stack.as_ref().map(|p| p.as_slice()) == Some(path);
            render_stack(state, ctx, path, node, bounds, maximized, body_fill, render_body)
        }
    }
}

const SPLIT_VIS_PX: f32 = 6.0;
const SPLIT_HIT_MIN_PX: f32 = 20.0;

fn render_axis(
    state: &DockState,
    ctx: &mut DockRenderContext<'_>,
    children: &[(DockNode, f32)],
    bounds: Rect,
    path: &[usize],
    horizontal: bool,
    body_fill: bool,
    render_body: &mut dyn FnMut(Rect, &str),
    outer_split: Option<(DockPath, usize, bool)>,
) {
    let total: f32 = children.iter().map(|(_, s)| *s).sum::<f32>().max(0.001);
    if horizontal {
        let mut x = bounds.x;
        for (index, (child, size)) in children.iter().enumerate() {
            let w = bounds.w * (*size / total);
            let child_rect = Rect::new(x, bounds.y, w, bounds.h);
            let mut child_path = path.to_vec();
            child_path.push(index);
            render_node(state, ctx, child, child_rect, &child_path, body_fill, render_body, Some((path.to_vec(), index, true)));
            x += w;
        }
    } else {
        let mut y = bounds.y;
        for (index, (child, size)) in children.iter().enumerate() {
            let h = bounds.h * (*size / total);
            let child_rect = Rect::new(bounds.x, y, bounds.w, h);
            let mut child_path = path.to_vec();
            child_path.push(index);
            render_node(state, ctx, child, child_rect, &child_path, body_fill, render_body, Some((path.to_vec(), index, false)));
            y += h;
        }
    }
    let _ = outer_split;
}

fn walk_resize_hits(state: &DockState, ctx: &mut DockRenderContext<'_>, node: &DockNode, bounds: Rect, path: &[usize], outer_split: Option<(DockPath, usize, bool)>) {
    match node {
        DockNode::Row(children) => walk_resize_axis(state, ctx, children, bounds, path, true, outer_split),
        DockNode::Column(children) => walk_resize_axis(state, ctx, children, bounds, path, false, outer_split),
        DockNode::Stack { .. } => {}
    }
}

fn walk_resize_axis(state: &DockState, ctx: &mut DockRenderContext<'_>, children: &[(DockNode, f32)], bounds: Rect, path: &[usize], horizontal: bool, outer_split: Option<(DockPath, usize, bool)>) {
    let total: f32 = children.iter().map(|(_, s)| *s).sum::<f32>().max(0.001);
    if horizontal {
        let mut x = bounds.x;
        for (index, (child, size)) in children.iter().enumerate() {
            let w = bounds.w * (*size / total);
            let child_rect = Rect::new(x, bounds.y, w, bounds.h);
            let mut child_path = path.to_vec();
            child_path.push(index);
            walk_resize_hits(state, ctx, child, child_rect, &child_path, Some((path.to_vec(), index, true)));
            x += w;
            if index + 1 < children.len() {
                let hit_w = SPLIT_HIT_MIN_PX.max(SPLIT_VIS_PX);
                let handle = Rect::new(x - hit_w * 0.5, bounds.y, hit_w, bounds.h);
                register_split_hit(ctx, path, index, handle, DragAxis::Horizontal);
                if let Some((parent_path, parent_index, parent_horizontal)) = &outer_split {
                    if *parent_horizontal != horizontal {
                        register_join_corner_hits(ctx, path, index, parent_path, *parent_index, handle, horizontal);
                    }
                }
            }
        }
    } else {
        let mut y = bounds.y;
        for (index, (child, size)) in children.iter().enumerate() {
            let h = bounds.h * (*size / total);
            let child_rect = Rect::new(bounds.x, y, bounds.w, h);
            let mut child_path = path.to_vec();
            child_path.push(index);
            walk_resize_hits(state, ctx, child, child_rect, &child_path, Some((path.to_vec(), index, false)));
            y += h;
            if index + 1 < children.len() {
                let hit_h = SPLIT_HIT_MIN_PX.max(SPLIT_VIS_PX);
                let handle = Rect::new(bounds.x, y - hit_h * 0.5, bounds.w, hit_h);
                register_split_hit(ctx, path, index, handle, DragAxis::Vertical);
                if let Some((parent_path, parent_index, parent_horizontal)) = &outer_split {
                    if *parent_horizontal != horizontal {
                        register_join_corner_hits(ctx, path, index, parent_path, *parent_index, handle, horizontal);
                    }
                }
            }
        }
    }
}

fn register_join_corner_hits(ctx: &mut DockRenderContext<'_>, path: &[usize], split_index: usize, parent_path: &DockPath, parent_index: usize, handle: Rect, horizontal: bool) {
    let corner = 10.0;
    let corners = if horizontal {
        [Rect::new(handle.x - corner * 0.5, handle.y, corner, corner), Rect::new(handle.x - corner * 0.5, handle.y + handle.h - corner, corner, corner)]
    } else {
        [Rect::new(handle.x, handle.y - corner * 0.5, corner, corner), Rect::new(handle.x + handle.w - corner, handle.y - corner * 0.5, corner, corner)]
    };
    for (corner_slot, rect) in corners.iter().enumerate() {
        let _ = corner_slot;
        ctx.input.register_hit(HitTarget {
            rect: *rect,
            event: None,
            control_id: Some(format!("dock.corner.r/{}/{}/c/{}/{}", path_str(path), split_index, path_str(parent_path), parent_index)),
            kind: HitKind::DockJoinCorner,
            drag_axis: Some(DragAxis::Both),
            drag_data: None,
        });
    }
}

pub(crate) fn dock_tab_content_width(atlas: &mut FontAtlas, theme: &Theme, label: &str) -> f32 {
    14.0 + theme.gap_standard + atlas.measure_text(label, theme.font_size_small).0 + theme.padding_standard * 2.0
}

fn paint_dock_tab_icon(ctx: &mut DockRenderContext<'_>, icon_id: &str, x: f32, tab_rect: Rect, color: Rgba) -> f32 {
    const ICON_TINY: f32 = 14.0;
    if let Some(uv) = ctx.icons.icon_uv(icon_id) {
        ctx.draw.push_textured([x, tab_rect.y + (tab_rect.h - ICON_TINY) * 0.5, ICON_TINY, ICON_TINY], uv, color);
        ICON_TINY + ctx.theme.gap_standard
    } else {
        0.0
    }
}
fn register_split_hit(ctx: &mut DockRenderContext<'_>, path: &[usize], index: usize, rect: Rect, axis: DragAxis) {
    ctx.input.register_hit(HitTarget { rect, event: None, control_id: Some(format!("dock.split.{}.{index}", path_str(path))), kind: HitKind::DockSplit, drag_axis: Some(axis), drag_data: None });
}

fn render_stack(state: &DockState, ctx: &mut DockRenderContext<'_>, path: &[usize], node: &DockNode, bounds: Rect, maximized: bool, body_fill: bool, render_body: &mut dyn FnMut(Rect, &str)) {
    let DockNode::Stack { windows, active } = node else {
        return;
    };
    if windows.is_empty() {
        return;
    }
    let theme = ctx.theme;
    let tab_h = theme.control_height;
    let stroke = theme.stroke_hairline;
    let globally_active = state.active_stack.as_ref().map(|p| p.as_slice()) == Some(path);
    let stack_hovered = bounds.contains(ctx.input.pointer_x, ctx.input.pointer_y);
    let border = if globally_active {
        theme.accent
    } else if stack_hovered {
        theme.border_emphasized
    } else {
        theme.border_normal
    };
    let cap_y = bounds.y;
    let cap_rect = Rect::new(bounds.x, cap_y, bounds.w, tab_h);

    let focus_label = if maximized { "Unfocus" } else { "Focus" };
    let focus_icon = if maximized { "minimize-2" } else { "maximize-2" };
    let focus_w = measure_cap_button(ctx.atlas, theme, focus_icon, focus_label);
    let close_w = measure_cap_button(ctx.atlas, theme, "x", "Close");
    let controls_w = focus_w + close_w;
    let mut tab_x = cap_rect.x;
    let mut tabs = Vec::with_capacity(windows.len());
    for window_id in windows {
        let label = ctx.window_labels.get(window_id).map(String::as_str).unwrap_or(window_id);
        let icon_id = ctx.window_icon_ids.get(window_id).map(String::as_str).unwrap_or("app-window");
        let tw = dock_tab_content_width(ctx.atlas, theme, label);
        let tab_rect = Rect::new(tab_x, cap_y, tw, tab_h);
        tabs.push((window_id.as_str(), label, icon_id, tab_rect));
        tab_x += tw;
    }
    let gap_x = tab_x;
    let controls_x = cap_rect.x + cap_rect.w - controls_w;
    let silhouette = WindowSilhouette::from_measured_top(bounds, gap_x - bounds.x, controls_w, tab_h);

    if body_fill {
        let content_bounds = silhouette.content_bounds();
        ctx.draw.begin_silhouette_clip(&silhouette.content_clip_rects());
        ctx.draw.push_solid([content_bounds.x, content_bounds.y, content_bounds.w, content_bounds.h], theme.canvas_clear);
        render_body(content_bounds, active);
        ctx.draw.end_silhouette_clip();
    }

    for (window_id, label, icon_id, tab_rect) in &tabs {
        let tab_glass = ctx.draw.push_glass([tab_rect.x, tab_rect.y, tab_rect.w, tab_rect.h], 0.0, theme.glass(Level::Window));
        ctx.draw.begin_glass_content(tab_glass);
        let is_active = *window_id == active;
        let stack_active_tab = is_active && globally_active;
        let hovered = tab_rect.contains(ctx.input.pointer_x, ctx.input.pointer_y);
        let tint = if stack_active_tab {
            theme.active_foreground
        } else if hovered {
            theme.border_emphasized
        } else {
            theme.text_element
        };
        let icon_w = paint_dock_tab_icon(ctx, icon_id, tab_rect.x + theme.padding_standard, tab_rect, tint);
        dock_text(ctx, label, tab_rect.x + theme.padding_standard + icon_w, tab_rect.y + (tab_rect.h + theme.font_size_small) * 0.5 - 1.0, theme.font_size_small, tint);
        ctx.input.register_hit(HitTarget { rect: *tab_rect, event: None, control_id: Some(format!("dock.tab.{}.{}", path_str(path), window_id)), kind: HitKind::Window, drag_axis: None, drag_data: None });
        ctx.draw.end_glass_content();
    }
    let controls_rect = Rect::new(controls_x, cap_y, controls_w, tab_h);
    let controls_glass = ctx.draw.push_glass([controls_rect.x, controls_rect.y, controls_rect.w, controls_rect.h], 0.0, theme.glass(Level::Window));
    ctx.draw.begin_glass_content(controls_glass);
    render_cap_action_group(ctx, controls_rect, &[("dock.focus", focus_icon, focus_label), ("dock.close", "x", "Close")], path, false);
    ctx.draw.end_glass_content();

    // 🪟️ One outline for tabs + glass cutout + controls + body (matches React ModeDockStackSilhouetteBorder).
    push_window_silhouette_border(ctx.draw, &silhouette, stroke, border);
}

struct StackCapLayout {
    gap_x: f32,
    gap_w: f32,
}

//#region WindowSilhouetteGeometry

/// 🪟️ Normalized physical chip span in silhouette-local coordinates.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowSilhouetteSpan {
    pub left: f32,
    pub right: f32,
}

impl WindowSilhouetteSpan {
    pub fn new(left: f32, right: f32) -> Self {
        Self { left, right }
    }
}

/// 🪟️ Normalized top or bottom chrome band.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct WindowSilhouetteEdge {
    pub depth: f32,
    pub spans: Vec<WindowSilhouetteSpan>,
}

impl WindowSilhouetteEdge {
    pub fn new(depth: f32, spans: Vec<WindowSilhouetteSpan>) -> Self {
        Self { depth, spans }
    }
}

/// 🪟️ Rust mirror of `window-silhouette-geometry/v1` using arbitrary merged top and bottom spans.
#[derive(Clone, Debug, PartialEq)]
pub struct WindowSilhouette {
    pub bounds: Rect,
    pub top: WindowSilhouetteEdge,
    pub bottom: WindowSilhouetteEdge,
}

impl WindowSilhouette {
    const CHIP_EPSILON: f32 = 0.5;

    /// 🪟️ Builds and normalizes a v1 silhouette from physical top and bottom chip spans.
    pub fn new(bounds: Rect, top: WindowSilhouetteEdge, bottom: WindowSilhouetteEdge) -> Self {
        let width = bounds.w.max(0.0);
        let height = bounds.h.max(0.0);
        let top_depth = top.depth.max(0.0).min(height);
        let bottom_depth = bottom.depth.max(0.0).min((height - top_depth).max(0.0));
        Self {
            bounds,
            top: WindowSilhouetteEdge::new(top_depth, Self::normalize_spans(top.spans, width)),
            bottom: WindowSilhouetteEdge::new(bottom_depth, Self::normalize_spans(bottom.spans, width)),
        }
    }

    /// 🪟️ Projects the currently measured Dock tab and controls groups into the normalized v1 model.
    pub fn from_measured_top(bounds: Rect, tabs_w: f32, controls_w: f32, depth: f32) -> Self {
        let tabs_right = tabs_w.max(0.0).min(bounds.w);
        let controls_left = (bounds.w - controls_w.max(0.0)).max(tabs_right).min(bounds.w);
        Self::new(
            bounds,
            WindowSilhouetteEdge::new(depth, vec![WindowSilhouetteSpan::new(0.0, tabs_right), WindowSilhouetteSpan::new(controls_left, bounds.w)]),
            WindowSilhouetteEdge::default(),
        )
    }

    fn normalize_spans(spans: Vec<WindowSilhouetteSpan>, width: f32) -> Vec<WindowSilhouetteSpan> {
        let mut spans: Vec<_> = spans
            .into_iter()
            .filter(|span| span.left.is_finite() && span.right.is_finite() && span.right > span.left)
            .map(|span| WindowSilhouetteSpan::new(span.left.clamp(0.0, width), span.right.clamp(0.0, width)))
            .filter(|span| span.right - span.left > Self::CHIP_EPSILON)
            .collect();
        spans.sort_by(|a, b| a.left.total_cmp(&b.left).then(a.right.total_cmp(&b.right)));
        let mut merged: Vec<WindowSilhouetteSpan> = Vec::new();
        for span in spans {
            if let Some(last) = merged.last_mut() {
                if span.left <= last.right + Self::CHIP_EPSILON {
                    last.right = last.right.max(span.right);
                    continue;
                }
            }
            merged.push(span);
        }
        merged
    }

    //#region ContentClip

    /// 🪟️ Returns the disjoint body-and-chip union used by paint, content, and hit clipping.
    pub fn content_clip_rects(&self) -> Vec<Rect> {
        let mut regions = Vec::with_capacity(1 + self.top.spans.len() + self.bottom.spans.len());
        let body_h = (self.bounds.h - self.top.depth - self.bottom.depth).max(0.0);
        if self.bounds.w > Self::CHIP_EPSILON && body_h > Self::CHIP_EPSILON {
            regions.push(Rect::new(self.bounds.x, self.bounds.y + self.top.depth, self.bounds.w, body_h));
        }
        regions.extend(self.glass_regions());
        regions
    }

    /// 🪟️ Returns only chip regions, suitable for glass compositing.
    pub fn glass_regions(&self) -> Vec<Rect> {
        let mut regions = Vec::with_capacity(self.top.spans.len() + self.bottom.spans.len());
        if self.top.depth > Self::CHIP_EPSILON {
            regions.extend(self.top.spans.iter().map(|span| Rect::new(self.bounds.x + span.left, self.bounds.y, span.right - span.left, self.top.depth)));
        }
        if self.bottom.depth > Self::CHIP_EPSILON {
            let y = self.bounds.y + self.bounds.h - self.bottom.depth;
            regions.extend(self.bottom.spans.iter().map(|span| Rect::new(self.bounds.x + span.left, y, span.right - span.left, self.bottom.depth)));
        }
        regions
    }

    /// 🪟️ Returns the full content coordinate space before silhouette clipping.
    pub fn content_bounds(&self) -> Rect {
        self.bounds
    }

    /// 🪟️ Returns the chrome-safe center band for dock targets and auxiliary rails.
    pub fn safe_body_rect(&self) -> Rect {
        Rect::new(
            self.bounds.x,
            self.bounds.y + self.top.depth,
            self.bounds.w,
            (self.bounds.h - self.top.depth - self.bottom.depth).max(0.0),
        )
    }

    /// 🪟️ Returns the normalized document-layout clearances `(top, bottom)`.
    pub fn safe_clearances(&self) -> (f32, f32) {
        (self.top.depth, self.bottom.depth)
    }

    /// 🪟️ Tests exact membership in the body-and-chip union, leaving all chrome gaps as cutouts.
    pub fn contains(&self, x: f32, y: f32) -> bool {
        x.is_finite() && y.is_finite() && self.content_clip_rects().iter().any(|region| region.contains(x, y))
    }

    //#endregion ContentClip
}

/// 🪟️ Paints a hairline (or thicker) stroke along the dock-stack silhouette path.
pub fn push_window_silhouette_border(draw: &mut DrawList, silhouette: &WindowSilhouette, stroke: f32, color: Rgba) {
    let b = silhouette.bounds;
    let mut paint_edge = |edge: &WindowSilhouetteEdge, outer: f32, inner: f32| {
        let mut cursor = 0.0;
        for span in &edge.spans {
            if span.left > cursor + WindowSilhouette::CHIP_EPSILON {
                draw.push_solid([b.x + cursor, inner - stroke * 0.5, span.left - cursor, stroke], color);
            }
            draw.push_solid([b.x + span.left, outer - stroke * 0.5, span.right - span.left, stroke], color);
            if span.left > WindowSilhouette::CHIP_EPSILON {
                draw.push_solid([b.x + span.left - stroke * 0.5, outer.min(inner), stroke, (outer - inner).abs()], color);
            }
            if span.right < b.w - WindowSilhouette::CHIP_EPSILON {
                draw.push_solid([b.x + span.right - stroke * 0.5, outer.min(inner), stroke, (outer - inner).abs()], color);
            }
            cursor = span.right;
        }
        if cursor < b.w - WindowSilhouette::CHIP_EPSILON {
            draw.push_solid([b.x + cursor, inner - stroke * 0.5, b.w - cursor, stroke], color);
        }
    };
    paint_edge(&silhouette.top, b.y, b.y + silhouette.top.depth);
    paint_edge(&silhouette.bottom, b.y + b.h, b.y + b.h - silhouette.bottom.depth);
    let left_top = if silhouette.top.spans.first().is_some_and(|span| span.left <= WindowSilhouette::CHIP_EPSILON) { b.y } else { b.y + silhouette.top.depth };
    let left_bottom = if silhouette.bottom.spans.first().is_some_and(|span| span.left <= WindowSilhouette::CHIP_EPSILON) { b.y + b.h } else { b.y + b.h - silhouette.bottom.depth };
    let right_top = if silhouette.top.spans.last().is_some_and(|span| span.right >= b.w - WindowSilhouette::CHIP_EPSILON) { b.y } else { b.y + silhouette.top.depth };
    let right_bottom = if silhouette.bottom.spans.last().is_some_and(|span| span.right >= b.w - WindowSilhouette::CHIP_EPSILON) { b.y + b.h } else { b.y + b.h - silhouette.bottom.depth };
    draw.push_solid([b.x, left_top, stroke, (left_bottom - left_top).max(0.0)], color);
    draw.push_solid([b.x + b.w - stroke, right_top, stroke, (right_bottom - right_top).max(0.0)], color);
}

//#endregion WindowSilhouetteGeometry

fn layout_stack_cap(windows: &[String], labels: &HashMap<String, String>, atlas: &mut FontAtlas, theme: &Theme, bounds: Rect, maximized: bool) -> StackCapLayout {
    let focus_label = if maximized { "Unfocus" } else { "Focus" };
    let focus_icon = if maximized { "minimize-2" } else { "maximize-2" };
    let focus_w = measure_cap_button(atlas, theme, focus_icon, focus_label);
    let close_w = measure_cap_button(atlas, theme, "x", "Close");
    let controls_w = focus_w + close_w;
    let mut tab_x = bounds.x;
    for window_id in windows {
        let label = labels.get(window_id).map(String::as_str).unwrap_or(window_id.as_str());
        let tw = dock_tab_content_width(atlas, theme, label);
        tab_x += tw;
    }
    let gap_x = tab_x;
    let controls_x = bounds.x + bounds.w - controls_w;
    let gap_w = (controls_x - gap_x).max(0.0);
    StackCapLayout { gap_x, gap_w }
}

fn stack_window_silhouette(bounds: Rect, theme: &Theme, layout: &StackCapLayout) -> WindowSilhouette {
    WindowSilhouette::from_measured_top(bounds, layout.gap_x - bounds.x, bounds.x + bounds.w - (layout.gap_x + layout.gap_w), theme.control_height)
}

fn render_cap_action_group(ctx: &mut DockRenderContext<'_>, rect: Rect, buttons: &[(&str, &str, &str)], path: &[usize], draw_outer_border: bool) {
    let theme = ctx.theme;
    let hair = theme.stroke_hairline;
    let inner_y = rect.y + hair;
    let inner_h = (rect.h - hair * 2.0).max(0.0);
    let mut x = rect.x;
    for (index, (prefix, icon_id, label)) in buttons.iter().enumerate() {
        let item_w = measure_cap_button(ctx.atlas, theme, icon_id, label);
        let item_rect = Rect::new(x, inner_y, item_w, inner_h);
        let hovered = item_rect.contains(ctx.input.pointer_x, ctx.input.pointer_y);
        let icon_size = 14.0;
        let mut content_x = item_rect.x + theme.padding_standard;
        let icon_color = chrome_item_text(theme, false, hovered);
        if let Some(uv) = ctx.icons.icon_uv(icon_id) {
            ctx.draw.push_textured([content_x, item_rect.y + (item_rect.h - icon_size) * 0.5, icon_size, icon_size], uv, icon_color);
            content_x += icon_size + theme.gap_standard;
        }
        dock_text(ctx, label, content_x, item_rect.y + (item_rect.h + theme.font_size_small) * 0.5 - 1.0, theme.font_size_small, chrome_item_text(theme, false, hovered));
        ctx.input.register_hit(HitTarget { rect: item_rect, event: None, control_id: Some(format!("{prefix}.{}", path_str(path))), kind: HitKind::Button, drag_axis: None, drag_data: None });
        x += item_w;
        if index + 1 < buttons.len() {
            ctx.draw.push_solid([x, inner_y, hair, inner_h], theme.border_normal);
        }
    }
    if draw_outer_border {
        push_chrome_group_border(ctx.draw, rect, theme);
    }
}

fn measure_cap_button(atlas: &mut FontAtlas, theme: &Theme, _icon_id: &str, label: &str) -> f32 {
    let icon_w = 14.0 + theme.gap_standard;
    let text_w = atlas.measure_text(label, theme.font_size_small).0;
    theme.padding_standard * 2.0 + icon_w + text_w
}

pub fn path_str(path: &[usize]) -> String {
    path.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",")
}

pub fn parse_path(value: &str) -> DockPath {
    if value.is_empty() {
        return vec![];
    }
    value.split(',').filter_map(|part| part.parse().ok()).collect()
}

fn collect_stack_bodies(
    node: &DockNode,
    bounds: Rect,
    path: &[usize],
    theme: &Theme,
    window_labels: &HashMap<String, String>,
    atlas: &mut FontAtlas,
    state: &DockState,
    out: &mut Vec<(DockPath, Rect, String)>,
    silhouettes: &mut HashMap<String, WindowSilhouette>,
) {
    match node {
        DockNode::Row(children) => {
            let total: f32 = children.iter().map(|(_, s)| *s).sum::<f32>().max(0.001);
            let mut x = bounds.x;
            for (index, (child, size)) in children.iter().enumerate() {
                let w = bounds.w * (*size / total);
                let mut child_path = path.to_vec();
                child_path.push(index);
                collect_stack_bodies(child, Rect::new(x, bounds.y, w, bounds.h), &child_path, theme, window_labels, atlas, state, out, silhouettes);
                x += w;
            }
        }
        DockNode::Column(children) => {
            let total: f32 = children.iter().map(|(_, s)| *s).sum::<f32>().max(0.001);
            let mut y = bounds.y;
            for (index, (child, size)) in children.iter().enumerate() {
                let h = bounds.h * (*size / total);
                let mut child_path = path.to_vec();
                child_path.push(index);
                collect_stack_bodies(child, Rect::new(bounds.x, y, bounds.w, h), &child_path, theme, window_labels, atlas, state, out, silhouettes);
                y += h;
            }
        }
        DockNode::Stack { windows, active } => {
            let maximized = state.maximized_stack.as_ref().map(|p| p.as_slice()) == Some(path);
            let layout = layout_stack_cap(windows, window_labels, atlas, theme, bounds, maximized);
            let silhouette = stack_window_silhouette(bounds, theme, &layout);
            out.push((path.to_vec(), silhouette.safe_body_rect(), active.clone()));
            silhouettes.insert(active.clone(), silhouette);
        }
    }
}

fn dock_text(ctx: &mut DockRenderContext<'_>, text: &str, x: f32, y: f32, size: f32, color: Rgba) {
    let mut scroll = std::collections::HashMap::new();
    let mut collapsed = std::collections::HashMap::new();
    let mut selects = std::collections::HashMap::new();
    let mut widget_ctx = crate::interpreter::framework_widget_context(ctx.draw, None, ctx.atlas, Some(ctx.icons), ctx.input, ctx.theme, &mut scroll, &mut collapsed, &mut selects, None);
    draw_text(&mut widget_ctx, text, x, y, size, color);
}

//#endregion DockFreeFunctions

//#region DockTests
#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use crate::shell::ShellState;
    use semio_framework::{AppDefinition, ModeDefinition, PanelGroup, PanelTabDefinition, PanelTabKind, WindowKindDefinition};
    use ui_wgpu::wgpu::{create_default_layout, WindowOptions};

    fn sample_app(window_ids: &[&str], layout: Option<WindowLayout>) -> AppDefinition {
        AppDefinition {
            id: "test".into(),
            label: LocalizedLabel::data("Test"),
            breadcrumb: vec!["semio".into(), "test".into()],
            icon_id: None,
            controller_id: "test".into(),
            modes: semio_framework::Modes::one(ModeDefinition { id: "default".into(), label: LocalizedLabel::data("Default"), icon_id: "pencil".into(), tools: vec![], layout_id: None, commands: vec![] }),
            default_mode_id: "default".into(),
            window_kinds: semio_framework::WindowKinds::try_from(
                window_ids
                    .iter()
                    .map(|id| WindowKindDefinition {
                        id: (*id).into(),
                        label: LocalizedLabel::data(*id),
                        body_key: format!("{id}.body"),
                        surface_kind: ui_wgpu::wgpu::SurfaceKind::Canvas2d,
                        icon_id: "app-window".into(),
                        options: WindowOptions::default(),
                        actions: vec![],
                        interactions: vec![],
                        utilities: vec![],
                        params_schema: None,
                        artifact_snapshot_schema: None,
                        input_event_schema: None,
                        output_schema: None,
                        capabilities: vec![],
                    })
                    .collect::<Vec<_>>(),
            )
            .expect("sample_app tests always pass at least one window id"),
            panel_tabs: vec![PanelTabDefinition { kind: PanelTabKind::App("tab".into()), label: LocalizedLabel::data("Tab"), group: PanelGroup::Workbench, body_key: Some("tab.body".into()), children: vec![] }],
            keybindings: vec![],
            interactions: vec![],
            utilities: vec![],
            tools: vec![],
            commands: vec![],
            named_layouts: vec![],
            default_layout: layout,
            terminologies: vec![],
            terminology_breadcrumbs: std::collections::HashMap::new(),
            introduction: None,
            tutorials: Vec::new(),
            dialogs: Vec::new(),
            media_inputs: Vec::new(),
            media_outputs: Vec::new(),
            artifact_kinds: Vec::new(),
            config: semio_framework::ConfigSpec::empty(),
            command_grammar: semio_framework::CommandGrammar::empty(),
            io: semio_framework::AppIo::default(),
        }
    }

    #[test]
    fn split_axis_extent_uses_row_width_not_canvas_max() {
        let mut dock = DockState::from_app(&sample_app(&["a", "b"], None), Some("a"));
        dock.root = DockNode::Column(vec![(DockNode::Row(vec![(stack_with("a"), 0.5), (stack_with("b"), 0.5)]), 0.5), (stack_with("c"), 0.5)]);
        let canvas = Rect::new(0.0, 0.0, 1000.0, 800.0);
        let row_extent = dock.split_axis_extent(&vec![0], canvas).unwrap();
        assert!((row_extent - 1000.0).abs() < 0.1);
        let col_extent = dock.split_axis_extent(&vec![], canvas);
        assert!((col_extent.unwrap() - 800.0).abs() < 0.1);
        let nested_extent = dock.split_axis_extent(&vec![0], canvas).unwrap();
        assert!((nested_extent - 1000.0).abs() < 0.1);
    }

    #[test]
    fn panel_scroll_region_blocks_scene_wheel() {
        let panel_scroll = HitTarget { rect: Rect::new(0.0, 0.0, 200.0, 400.0), event: None, control_id: Some("panel.left.lowpoly".into()), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None };
        assert!(!ShellState::wheel_propagates_to_scene_surface(Some(&panel_scroll)));
        let world = HitTarget { rect: Rect::new(0.0, 0.0, 800.0, 600.0), event: None, control_id: Some("world-surface".into()), kind: HitKind::World3d, drag_axis: None, drag_data: None };
        assert!(ShellState::wheel_propagates_to_scene_surface(Some(&world)));
        let graph_pane = HitTarget { rect: Rect::new(0.0, 0.0, 800.0, 600.0), event: None, control_id: Some("graph-surface.pane".into()), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None };
        assert!(ShellState::wheel_propagates_to_scene_surface(Some(&graph_pane)));
    }

    #[test]
    fn row_layout_stack_content_rects_match_per_window() {
        let layout = create_default_layout(&["flow".into(), "preview".into()], "row", Some(&[68.0, 32.0]), Some(&["Flow".into(), "Preview".into()]));
        let app = sample_app(&["flow", "preview"], Some(layout));
        let dock = DockState::from_app(&app, Some("flow"));
        let canvas = Rect::new(0.0, 0.0, 1200.0, 800.0);
        let theme = Theme::default();
        let mut atlas = FontAtlas::builtin();
        let labels = HashMap::from([("flow".into(), "Flow".into()), ("preview".into(), "Preview".into())]);
        let placements = dock.stack_body_rects(canvas, &theme, &labels, &mut atlas);
        assert_eq!(placements.len(), 2);
        let flow_rect = placements.iter().find(|(_, _, id)| id == "flow").map(|(_, rect, _)| *rect);
        let preview_rect = placements.iter().find(|(_, _, id)| id == "preview").map(|(_, rect, _)| *rect);
        let flow_rect = flow_rect.expect("flow body rect");
        let preview_rect = preview_rect.expect("preview body rect");
        assert!(flow_rect.w > preview_rect.w);
        assert!((flow_rect.x + flow_rect.w - preview_rect.x).abs() < 1.0);
        assert!(flow_rect.h > 0.0 && preview_rect.h > 0.0);
    }

    #[test]
    fn dock_tab_content_width_reserves_icon_slot() {
        let theme = Theme::default();
        let mut atlas = FontAtlas::builtin();
        let label = "Main";
        let with_icon = dock_tab_content_width(&mut atlas, &theme, label);
        let text_only = atlas.measure_text(label, theme.font_size_small).0 + theme.padding_standard * 2.0;
        assert!(with_icon > text_only);
    }

    //#region SilhouetteContentTests

    #[test]
    fn window_silhouette_clip_union_excludes_cap_gap() {
        let silhouette = WindowSilhouette::from_measured_top(Rect::new(10.0, 20.0, 300.0, 200.0), 80.0, 60.0, 32.0);
        assert_eq!(
            silhouette.content_clip_rects(),
            vec![Rect::new(10.0, 52.0, 300.0, 168.0), Rect::new(10.0, 20.0, 80.0, 32.0), Rect::new(250.0, 20.0, 60.0, 32.0)]
        );
        assert_eq!(silhouette.content_bounds(), Rect::new(10.0, 20.0, 300.0, 200.0));
        assert_eq!(silhouette.safe_body_rect(), Rect::new(10.0, 52.0, 300.0, 168.0));
        assert!(!silhouette.content_clip_rects().iter().any(|rect| rect.contains(150.0, 30.0)));
    }

    #[test]
    fn window_silhouette_v1_matches_typescript_fixture_for_merging_bottom_and_containment() {
        let normalized = WindowSilhouette::new(
            Rect::new(0.0, 0.0, 200.0, 100.0),
            WindowSilhouetteEdge::new(
                24.0,
                vec![WindowSilhouetteSpan::new(160.0, 220.0), WindowSilhouetteSpan::new(60.25, 90.0), WindowSilhouetteSpan::new(0.0, 60.0), WindowSilhouetteSpan::new(90.25, 120.0)],
            ),
            WindowSilhouetteEdge::new(16.0, vec![WindowSilhouetteSpan::new(80.0, 120.0), WindowSilhouetteSpan::new(0.0, 40.0)]),
        );
        assert_eq!(normalized.top.spans, vec![WindowSilhouetteSpan::new(0.0, 120.0), WindowSilhouetteSpan::new(160.0, 200.0)]);
        let silhouette = WindowSilhouette::new(
            Rect::new(0.0, 0.0, 200.0, 100.0),
            WindowSilhouetteEdge::new(24.0, vec![WindowSilhouetteSpan::new(160.0, 200.0), WindowSilhouetteSpan::new(0.0, 60.0)]),
            WindowSilhouetteEdge::new(16.0, vec![WindowSilhouetteSpan::new(80.0, 120.0), WindowSilhouetteSpan::new(0.0, 40.0)]),
        );
        assert_eq!(
            silhouette.glass_regions(),
            vec![Rect::new(0.0, 0.0, 60.0, 24.0), Rect::new(160.0, 0.0, 40.0, 24.0), Rect::new(0.0, 84.0, 40.0, 16.0), Rect::new(80.0, 84.0, 40.0, 16.0)]
        );
        assert_eq!(silhouette.safe_clearances(), (24.0, 16.0));
        assert!(silhouette.contains(20.0, 12.0));
        assert!(!silhouette.contains(140.0, 12.0));
        assert!(silhouette.contains(100.0, 50.0));
        assert!(!silhouette.contains(60.0, 92.0));
        assert!(silhouette.contains(100.0, 92.0));
    }

    #[test]
    fn dock_stack_glass_and_hits_exist_only_on_owned_chips() {
        let mut dock = DockState::from_app(&sample_app(&["a", "b"], None), Some("a"));
        dock.root = DockNode::Stack { windows: vec!["a".into(), "b".into()], active: "a".into() };
        let bounds = Rect::new(0.0, 0.0, 600.0, 400.0);
        let theme = Theme::default();
        let mut atlas = FontAtlas::builtin();
        let icons = IconAtlas::default();
        let mut input = InputState::<ActionDescriptor>::default();
        let mut draw = DrawList::default();
        let labels = HashMap::from([("a".into(), "A".into()), ("b".into(), "B".into())]);
        let icon_ids = HashMap::new();
        let layout = layout_stack_cap(&["a".into(), "b".into()], &labels, &mut atlas, &theme, bounds, false);
        let gap_point = (layout.gap_x + layout.gap_w * 0.5, bounds.y + theme.control_height * 0.5);
        let mut ctx = DockRenderContext { draw: &mut draw, atlas: &mut atlas, icons: &icons, input: &mut input, theme: &theme, window_labels: &labels, window_icon_ids: &icon_ids };
        dock.paint_chrome(&mut ctx, bounds, false);
        assert_eq!(draw.glass_regions.len(), 3, "two tab chips plus one controls chip");
        assert!(!draw.glass_regions.iter().any(|region| Rect::new(region.rect[0], region.rect[1], region.rect[2], region.rect[3]).contains(gap_point.0, gap_point.1)));
        assert!(!input.hit_targets.iter().any(|hit| hit.control_id.as_deref().is_some_and(|id| id.starts_with("dock.stack."))));
    }

    #[test]
    fn dock_stack_content_fills_full_bounds_through_one_silhouette_clip() {
        let mut dock = DockState::from_app(&sample_app(&["a", "b"], None), Some("a"));
        dock.root = DockNode::Stack { windows: vec!["a".into(), "b".into()], active: "a".into() };
        let bounds = Rect::new(10.0, 20.0, 600.0, 400.0);
        let theme = Theme::default();
        let mut atlas = FontAtlas::builtin();
        let icons = IconAtlas::default();
        let mut input = InputState::<ActionDescriptor>::default();
        let mut draw = DrawList::default();
        let labels = HashMap::from([("a".into(), "A".into()), ("b".into(), "B".into())]);
        let icon_ids = HashMap::new();
        let mut ctx = DockRenderContext { draw: &mut draw, atlas: &mut atlas, icons: &icons, input: &mut input, theme: &theme, window_labels: &labels, window_icon_ids: &icon_ids };
        dock.paint_chrome(&mut ctx, bounds, true);
        let fill = draw
            .layers
            .iter()
            .find(|layer| layer.ui_instances.iter().any(|instance| instance.rect == [bounds.x, bounds.y, bounds.w, bounds.h]))
            .expect("full silhouette content fill");
        assert_eq!(fill.clip.as_ref().map(|clip| clip.scissors.len()), Some(3));
        assert!(!fill.clip.as_ref().is_some_and(|clip| clip.scissors.iter().any(|rect| rect.x <= 300 && 300 < rect.x + rect.w && rect.y <= 30 && 30 < rect.y + rect.h)));
    }

    //#endregion SilhouetteContentTests

    #[test]
    fn resize_hits_win_over_later_scroll_region() {
        let mut dock = DockState::from_app(&sample_app(&["a", "b"], None), Some("a"));
        dock.root = even_layout(&["a".into(), "b".into()]);
        let canvas = Rect::new(0.0, 0.0, 400.0, 300.0);
        let theme = Theme::default();
        let mut atlas = FontAtlas::builtin();
        let mut input = InputState::<ActionDescriptor>::default();
        let mut draw = DrawList::default();
        let labels = HashMap::from([("a".into(), "A".into()), ("b".into(), "B".into())]);
        input.register_hit(HitTarget { rect: canvas, event: None, control_id: Some("content.scroll".into()), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None });
        let mut ctx = DockRenderContext { draw: &mut draw, atlas: &mut atlas, icons: &IconAtlas::default(), input: &mut input, theme: &theme, window_labels: &labels, window_icon_ids: &HashMap::new() };
        dock.register_resize_hits(&mut ctx, canvas);
        let hit = input.hit_at(200.0, 150.0).expect("split hit");
        assert_eq!(hit.kind, HitKind::DockSplit);
        assert_eq!(hit.drag_axis, Some(DragAxis::Horizontal));
        assert!(hit.rect.w >= 20.0);
    }

    fn stack_with(id: &str) -> DockNode {
        DockNode::Stack { windows: vec![id.into()], active: id.into() }
    }

    //#region DragDropAndLayoutDiffTests

    fn tab_payload(window_id: &str, source_path: DockPath, tab_index: usize) -> DockDragPayload {
        DockDragPayload { kind: DockDragKind::Tab, window_id: window_id.into(), source_path, tab_index, ghost_label: window_id.into() }
    }

    fn stack_payload(window_id: &str, source_path: DockPath, tab_index: usize) -> DockDragPayload {
        DockDragPayload { kind: DockDragKind::Stack, window_id: window_id.into(), source_path, tab_index, ghost_label: window_id.into() }
    }

    /// 🎯️ Regression pin for the double-removal bug: `apply_drop` used to call `remove_window` again
    /// on a window the caller (`ShellState::handle_pointer_move`) had *already* removed at drag
    /// promotion, so `remove_window` always failed and every cross-stack tab drop silently no-opped.
    #[test]
    fn apply_drop_tab_moves_window_across_stacks() {
        let mut dock = DockState::default();
        // 🪟️ Three stacks so removing `a` (the sole occupant of stack `[0]`) prunes that slot without
        // also emptying the drop target — `b` shifts from `[1]` down to `[0]`, and `c` (the actual
        // cross-stack drop target) shifts from `[2]` to `[1]`.
        dock.root = DockNode::Row(vec![(stack_with("a"), 0.34), (stack_with("b"), 0.33), (stack_with("c"), 0.33)]);
        // 🎬️ Mirrors `ShellState::handle_pointer_move`'s eager removal at drag-promotion time. In the
        // real runtime `compute_dock_drop_zone` re-derives `stack_path` from the *current* (already
        // shifted) tree on every subsequent pointer move, so `zone` below targets `c`'s post-removal
        // path `[1]`, exactly as a live drag would have resolved it — not `c`'s stale pre-removal `[2]`.
        assert!(dock.remove_window("a"));
        assert_eq!(node_at(&dock.root, &vec![1]), Some(&stack_with("c")), "c shifted to [1] once a's slot was pruned");
        let payload = tab_payload("a", vec![0], 0);
        let zone = DockDropZone::Tab { stack_path: vec![1], index: 0 };
        assert!(dock.apply_drop(&payload, &zone), "cross-stack tab drop must actually land");
        assert_eq!(node_at(&dock.root, &vec![1]), Some(&DockNode::Stack { windows: vec!["a".into(), "c".into()], active: "a".into() }));
        assert!(find_stack_path(&dock.root, "b", &mut vec![]).is_some(), "b (untouched by this drop) still resolvable");
        assert_eq!(dock.active_window_id.as_deref(), Some("a"));
    }

    #[test]
    fn apply_drop_tab_reinserts_into_originating_stack_at_new_index() {
        let mut dock = DockState::default();
        dock.root = DockNode::Row(vec![(DockNode::Stack { windows: vec!["a".into(), "b".into(), "c".into()], active: "a".into() }, 1.0)]);
        assert!(dock.remove_window("a"));
        let payload = tab_payload("a", vec![0], 0);
        let zone = DockDropZone::Tab { stack_path: vec![0], index: 2 };
        assert!(dock.apply_drop(&payload, &zone));
        assert_eq!(node_at(&dock.root, &vec![0]), Some(&DockNode::Stack { windows: vec!["b".into(), "c".into(), "a".into()], active: "a".into() }));
    }

    #[test]
    fn apply_drop_tab_split_targets_the_post_removal_stack() {
        let mut dock = DockState::default();
        dock.root = DockNode::Row(vec![(stack_with("a"), 0.5), (stack_with("b"), 0.5)]);
        assert!(dock.remove_window("a"));
        // 🪟️ Removing the sole occupant of stack `a` prunes it — `b` now sits at path `[0]`.
        let payload = tab_payload("a", vec![0], 0);
        let zone = DockDropZone::Split { stack_path: vec![0], side: DockSide::Right };
        assert!(dock.apply_drop(&payload, &zone));
        assert!(find_stack_path(&dock.root, "a", &mut vec![]).is_some());
        assert!(find_stack_path(&dock.root, "b", &mut vec![]).is_some());
        assert_eq!(dock.active_window_id.as_deref(), Some("a"));
    }

    #[test]
    fn apply_drop_tab_root_split_builds_axis_pair() {
        let mut dock = DockState::default();
        dock.root = DockNode::Stack { windows: vec!["a".into(), "b".into()], active: "a".into() };
        assert!(dock.remove_window("a"));
        let payload = tab_payload("a", vec![], 0);
        let zone = DockDropZone::RootSplit { side: DockSide::Left };
        assert!(dock.apply_drop(&payload, &zone));
        assert_eq!(dock.root, DockNode::Row(vec![(stack_with("a"), 0.5), (DockNode::Stack { windows: vec!["b".into()], active: "b".into() }, 0.5)]));
    }

    #[test]
    fn apply_drop_stack_moves_whole_group_preserving_order_and_target_key() {
        let mut dock = DockState::default();
        dock.root = DockNode::Row(vec![(DockNode::Stack { windows: vec!["a".into(), "b".into(), "c".into()], active: "b".into() }, 0.5), (stack_with("d"), 0.5)]);
        // 🎬️ A stack drag pre-removes only its active window, same as a tab drag.
        assert!(dock.remove_window("b"));
        let payload = stack_payload("b", vec![0], 1);
        let zone = DockDropZone::Tab { stack_path: vec![1], index: 0 };
        assert!(dock.apply_drop(&payload, &zone), "whole-stack tab-join must land");
        // 🔑️ `a`/`c` (the siblings left behind by the eager single-window removal) travel with `b`,
        // reconstructed in their original order around it, and land next to `d` by key — not by the
        // pre-extraction `stack_path`, which the extraction itself would have shifted.
        let target = find_stack_path(&dock.root, "d", &mut vec![]).expect("d still resolvable by key");
        assert_eq!(node_at(&dock.root, &target), Some(&DockNode::Stack { windows: vec!["a".into(), "b".into(), "c".into(), "d".into()], active: "b".into() }));
        assert!(find_stack_path(&dock.root, "a", &mut vec![]).is_some());
        assert_eq!(dock.active_window_id.as_deref(), Some("b"));
    }

    #[test]
    fn apply_drop_stack_split_reanchors_target_after_extraction_shifts_paths() {
        let mut dock = DockState::default();
        // 🔑️ Source stack `[a, x]` keeps two windows, so the eager active-window removal at promotion
        // (`remove_window("a")`) does *not* collapse it — `b` stays at `[1]` right up until
        // `extract_stack_group` later pulls `x` out too, which *does* empty-and-prune slot `[0]`,
        // shifting `b` from `[1]` down to `[0]`. The `stack_path: [1]` captured in `zone` (from
        // hit-testing *before* this drag even started) is therefore stale by the time the drop lands.
        dock.root = DockNode::Row(vec![(DockNode::Stack { windows: vec!["a".into(), "x".into()], active: "a".into() }, 0.5), (stack_with("b"), 0.5)]);
        assert!(dock.remove_window("a"));
        assert_eq!(node_at(&dock.root, &vec![1]), Some(&stack_with("b")), "b starts at [1], pre-shift");
        let payload = stack_payload("a", vec![0], 0);
        let zone = DockDropZone::Split { stack_path: vec![1], side: DockSide::Bottom };
        assert!(dock.apply_drop(&payload, &zone), "split must land even though [1] goes stale mid-drop");
        // `b` shifted to `[0]` once `x` was extracted and slot `[0]` collapsed — proof the naive stale
        // path would have missed (or misdirected onto) the wrong node.
        assert_eq!(node_at(&dock.root, &vec![1]), None, "the pre-extraction path is no longer valid at all");
        let b_path = find_stack_path(&dock.root, "b", &mut vec![]).expect("b still resolvable by key");
        assert_eq!(node_at(&dock.root, &b_path), Some(&stack_with("b")), "b itself must be untouched by the split");
        let a_path = find_stack_path(&dock.root, "a", &mut vec![]).expect("a resolvable by key");
        assert_eq!(node_at(&dock.root, &a_path), Some(&DockNode::Stack { windows: vec!["a".into(), "x".into()], active: "a".into() }), "a's whole group (a + the sibling x it dragged along) landed together");
    }

    #[test]
    fn apply_drop_stack_same_source_is_noop() {
        let mut dock = DockState::default();
        dock.root = DockNode::Row(vec![(DockNode::Stack { windows: vec!["a".into(), "b".into()], active: "a".into() }, 0.5), (stack_with("c"), 0.5)]);
        assert!(dock.remove_window("a"));
        let before = dock.root.clone();
        let payload = stack_payload("a", vec![0], 0);
        let zone = DockDropZone::Tab { stack_path: vec![0], index: 0 };
        assert!(!dock.apply_drop(&payload, &zone), "dropping a stack back onto itself is a no-operation");
        assert_eq!(dock.root, before);
    }

    #[test]
    fn compute_dock_drop_zone_prefers_tab_bar_over_body_over_root() {
        let tab_bars = vec![(vec![0], Rect::new(0.0, 0.0, 200.0, 24.0), vec![80.0, 80.0])];
        let bodies = vec![(vec![0], Rect::new(0.0, 24.0, 200.0, 200.0), "a".to_string())];
        let canvas = Rect::new(0.0, 0.0, 200.0, 224.0);
        // Inside the tab bar rect → `Tab` zone wins even though it's also inside the body's column span.
        assert_eq!(compute_dock_drop_zone(10.0, 10.0, &tab_bars, &bodies, canvas), Some(DockDropZone::Tab { stack_path: vec![0], index: 0 }));
        // Inside the body but below the tab bar → `Split` zone.
        assert!(matches!(compute_dock_drop_zone(100.0, 100.0, &tab_bars, &bodies, canvas), Some(DockDropZone::Split { .. })));
        // Outside every registered stack but inside the canvas → `RootSplit`.
        assert!(matches!(compute_dock_drop_zone(190.0, 300.0, &tab_bars, &bodies, canvas), None));
        let wide_canvas = Rect::new(0.0, 0.0, 400.0, 400.0);
        assert!(matches!(compute_dock_drop_zone(390.0, 390.0, &tab_bars, &bodies, wide_canvas), Some(DockDropZone::RootSplit { .. })));
    }

    #[test]
    fn resolve_split_side_uses_dominant_axis_from_center() {
        // Wide-and-short body: a small vertical offset from center stays dominated by the x-axis.
        assert_eq!(resolve_split_side(10.0, 60.0, 400.0, 120.0), DockSide::Left);
        assert_eq!(resolve_split_side(390.0, 60.0, 400.0, 120.0), DockSide::Right);
        // Tall-and-narrow body: a small horizontal offset stays dominated by the y-axis.
        assert_eq!(resolve_split_side(60.0, 10.0, 120.0, 400.0), DockSide::Top);
        assert_eq!(resolve_split_side(60.0, 390.0, 120.0, 400.0), DockSide::Bottom);
    }

    #[test]
    fn apply_layout_diff_keeps_current_tab_focused_over_stale_persisted_active() {
        let mut dock = DockState::default();
        dock.root = DockNode::Stack { windows: vec!["a".into(), "b".into()], active: "b".into() };
        dock.active_window_id = Some("b".into());
        // 🗄️ A persisted `WindowLayout` snapshot whose `active_window_kind_id` predates the user's
        // later in-session tab switch to `b` — a naive `self.dock.root = dock_from_window_layout(...)`
        // teardown would silently revert focus back to `a`.
        let stale_layout = WindowLayout {
            root: WindowLayoutRoot::Stack(WindowLayoutStackNode {
                kind: "stack".into(),
                size: None,
                active_window_kind_id: Some("a".into()),
                children: vec![
                    WindowLayoutWindowNode { kind: "window".into(), window_kind_id: "a".into(), title: None, instance_id: None, template_id: None },
                    WindowLayoutWindowNode { kind: "window".into(), window_kind_id: "b".into(), title: None, instance_id: None, template_id: None },
                ],
            }),
        };
        dock.apply_layout_diff(&stale_layout);
        assert_eq!(dock.root, DockNode::Stack { windows: vec!["a".into(), "b".into()], active: "b".into() }, "same membership, reordered-or-not — the user's current tab stays focused");
    }

    #[test]
    fn apply_layout_diff_reanchors_active_and_maximized_stack_by_key() {
        let mut dock = DockState::default();
        dock.root = DockNode::Row(vec![(stack_with("a"), 0.5), (stack_with("b"), 0.5)]);
        dock.active_window_id = Some("b".into());
        dock.active_stack = Some(vec![1]);
        dock.maximized_stack = Some(vec![1]);
        dock.split_resize_origin = vec![0.5, 0.5];
        // 🔑️ The incoming layout reverses the two stacks' order — `b`'s *positional* path moves from
        // `[1]` to `[0]`. A stale-path reuse would now silently misdirect `active_stack`/
        // `maximized_stack` at `a` instead of following `b` by key.
        let reversed = WindowLayout {
            root: WindowLayoutRoot::Axis(ui_wgpu::wgpu::WindowLayoutAxisNode {
                kind: "row".into(),
                size: None,
                children: vec![
                    WindowLayoutChild::Stack(WindowLayoutStackNode {
                        kind: "stack".into(),
                        size: Some(0.5),
                        active_window_kind_id: Some("b".into()),
                        children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: "b".into(), title: None, instance_id: None, template_id: None }],
                    }),
                    WindowLayoutChild::Stack(WindowLayoutStackNode {
                        kind: "stack".into(),
                        size: Some(0.5),
                        active_window_kind_id: Some("a".into()),
                        children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: "a".into(), title: None, instance_id: None, template_id: None }],
                    }),
                ],
            }),
        };
        dock.apply_layout_diff(&reversed);
        assert_eq!(dock.active_window_id.as_deref(), Some("b"));
        assert_eq!(dock.active_stack, Some(vec![0]));
        assert_eq!(dock.maximized_stack, Some(vec![0]));
        assert!(dock.split_resize_origin.is_empty(), "an in-flight resize gesture's stale indices must not survive a layout swap");
    }

    #[test]
    fn apply_layout_diff_clears_maximized_stack_when_its_window_is_gone() {
        let mut dock = DockState::default();
        dock.root = DockNode::Row(vec![(stack_with("a"), 0.5), (stack_with("b"), 0.5)]);
        dock.active_window_id = Some("a".into());
        dock.maximized_stack = Some(vec![0]);
        let without_a = WindowLayout {
            root: WindowLayoutRoot::Stack(WindowLayoutStackNode {
                kind: "stack".into(),
                size: None,
                active_window_kind_id: Some("b".into()),
                children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: "b".into(), title: None, instance_id: None, template_id: None }],
            }),
        };
        dock.apply_layout_diff(&without_a);
        assert_eq!(dock.maximized_stack, None);
        assert_eq!(dock.active_window_id.as_deref(), Some("b"));
    }

    #[test]
    fn diff_dock_node_reuses_unchanged_subtree_and_adopts_new_shape_where_changed() {
        let old = DockNode::Row(vec![(stack_with("a"), 0.5), (stack_with("b"), 0.5)]);
        // Identical row → the whole node is byte-for-byte the same value (full reuse).
        let unchanged = diff_dock_node(&old, DockNode::Row(vec![(stack_with("a"), 0.5), (stack_with("b"), 0.5)]));
        assert_eq!(unchanged, old);
        // A structural kind change (Stack -> Row) at index 1 has no shared identity — adopt `next` as-is.
        let next = DockNode::Row(vec![(stack_with("a"), 0.5), (DockNode::Row(vec![(stack_with("b"), 1.0)]), 0.5)]);
        let diffed = diff_dock_node(&old, next.clone());
        assert_eq!(diffed, next);
    }

    //#endregion DragDropAndLayoutDiffTests

    #[test]
    fn even_layout_single_window() {
        let node = even_layout(&["main".into()]);
        assert!(matches!(node, DockNode::Stack { .. }));
        let dock = DockState::from_app(&sample_app(&["main"], None), None);
        assert_eq!(dock.active_window_id.as_deref(), Some("main"));
    }

    #[test]
    fn even_layout_multiple_windows() {
        let node = even_layout(&["a".into(), "b".into(), "c".into()]);
        assert!(matches!(node, DockNode::Row(_)));
        if let DockNode::Row(children) = node {
            assert_eq!(children.len(), 3);
            for (child, size) in children {
                assert!(matches!(child, DockNode::Stack { .. }));
                assert!((size - 1.0 / 3.0).abs() < 0.001);
            }
        }
    }

    #[test]
    fn parses_default_layout_row() {
        let layout = create_default_layout(&["a".into(), "b".into()], "row", None, None);
        let app = sample_app(&["a", "b"], Some(layout));
        let dock = DockState::from_app(&app, None);
        assert!(matches!(dock.root, DockNode::Row(_)));
    }

    #[test]
    fn close_window_collapses_stack_tabs() {
        let mut dock = DockState::from_app(&sample_app(&["a", "b"], None), Some("a"));
        dock.root = DockNode::Stack { windows: vec!["a".into(), "b".into()], active: "a".into() };
        let path = vec![];
        dock.close_active_in_stack(&path);
        if let DockNode::Stack { windows, active } = &dock.root {
            assert_eq!(windows, &vec!["b".to_string()]);
            assert_eq!(active, "b");
        } else {
            panic!("expected stack");
        }
    }

    #[test]
    fn reorder_tab_within_stack() {
        let mut dock = DockState::default();
        dock.root = DockNode::Stack { windows: vec!["a".into(), "b".into(), "c".into()], active: "a".into() };
        assert!(dock.reorder_tab(&vec![], 0, 2));
        if let DockNode::Stack { windows, .. } = &dock.root {
            assert_eq!(windows, &vec!["b".to_string(), "c".to_string(), "a".to_string()]);
        } else {
            panic!("expected stack");
        }
    }

    #[test]
    fn maximized_stack_uses_full_canvas_bounds() {
        let mut dock = DockState::from_app(&sample_app(&["a", "b", "c"], None), Some("a"));
        dock.root = even_layout(&["a".into(), "b".into(), "c".into()]);
        dock.toggle_maximize(&vec![1]);
        let canvas = Rect::new(0.0, 0.0, 900.0, 600.0);
        let theme = Theme::default();
        let mut atlas = FontAtlas::builtin();
        let bodies = dock.stack_body_rects(canvas, &theme, &HashMap::new(), &mut atlas);
        assert_eq!(bodies.len(), 1);
        let (_, body, _) = &bodies[0];
        assert!((body.w - canvas.w).abs() < 1.0);
        assert!((body.h - (canvas.h - theme.control_height)).abs() < 2.0);
    }

    #[test]
    fn split_drop_preview_covers_half_panel() {
        let body = Rect::new(10.0, 20.0, 400.0, 300.0);
        let left = split_drop_preview_in_body(body, DockSide::Left);
        assert_eq!(left.x, 10.0);
        assert_eq!(left.w, 200.0);
        assert_eq!(left.h, 300.0);
        let right = split_drop_preview_in_body(body, DockSide::Right);
        assert_eq!(right.x, 210.0);
        assert_eq!(right.w, 200.0);
    }

    #[test]
    fn map_marquee_mode_matches_ui_react() {
        use crate::engine_canvas::map_marquee_mode;
        assert_eq!(map_marquee_mode(false, false), "default");
        assert_eq!(map_marquee_mode(true, false), "additive");
        assert_eq!(map_marquee_mode(false, true), "subtractive");
        assert_eq!(map_marquee_mode(true, true), "invertive");
    }

    //#region WindowActionsAndUtilitiesTests
    use semio_framework::{ActionArgDef, ActionDefinition, ActionKind, UtilityDefinition, UtilityRef};
    use ui_wgpu::wgpu::{KeyAction, PointerModifiers};

    fn mods(meta: bool, ctrl: bool, shift: bool, alt: bool) -> PointerModifiers {
        PointerModifiers { meta, ctrl, shift, alt }
    }

    /// 🧰️ Builds a two-window app: window `main` scopes `utility.a`, window `aux` scopes nothing; `utility.b`
    /// is an orphan (no window references it). Actions: `zeroArg` (no args) + `withArgs` (required text +
    /// defaulted toggle) scoped to `main`.
    fn actions_utilities_app() -> AppDefinition {
        let mut app = sample_app(&["main", "aux"], None);
        app.controller_id = "ctrl".into();
        app.utilities =
            vec![UtilityDefinition::new("utility.a", LocalizedLabel::data("Utility A"), "circle"), UtilityDefinition { allows_actions_while_active: true, ..UtilityDefinition::new("utility.b", LocalizedLabel::data("Utility B"), "square") }];
        let actions = vec![
            ActionDefinition::new_catalog("zeroArg", LocalizedLabel::data("Zero Arg"), ActionKind::View),
            ActionDefinition {
                args: vec![ActionArgDef::text("name", LocalizedLabel::data("Name")).required(), ActionArgDef { default: Some(semio_framework::to_dsl_value(&serde_json::json!(true)).expect("toggle default")), ..ActionArgDef::toggle("flag", LocalizedLabel::data("Flag")) }],
                keys: Some("mod+e".into()),
                ..ActionDefinition::new_catalog("withArgs", LocalizedLabel::data("With Args"), ActionKind::View)
            },
        ];
        // Scope utility.a + both actions to `main`; leave utility.b an orphan referenced by no window.
        for kind in app.window_kinds.iter_mut() {
            if kind.id == "main" {
                kind.utilities = vec![UtilityRef::new("utility.a")];
                kind.actions = actions.clone();
            }
        }
        app
    }

    fn shell() -> ShellState {
        ShellState::new(vec![], "test".into())
    }

    #[test]
    fn resolve_window_utilities_scopes_explicit_and_orphans() {
        let app = actions_utilities_app();
        let main = app.window_kinds.iter().find(|k| k.id == "main").unwrap();
        let aux = app.window_kinds.iter().find(|k| k.id == "aux").unwrap();
        let main_ids: Vec<&str> = crate::shell::resolve_window_utilities(&app, main).iter().map(|t| t.id.as_str()).collect();
        let aux_ids: Vec<&str> = crate::shell::resolve_window_utilities(&app, aux).iter().map(|t| t.id.as_str()).collect();
        // `main` gets its explicit utility.a first, then the orphan utility.b; `aux` only sees the orphan.
        assert_eq!(main_ids, vec!["utility.a", "utility.b"]);
        assert_eq!(aux_ids, vec!["utility.b"]);
    }

    #[test]
    fn key_event_matches_chord_respects_modifiers() {
        use crate::shell::key_event_matches_chord;
        let z = KeyAction::Char("z".into());
        assert!(key_event_matches_chord(&z, &mods(true, false, false, false), "mod+z"));
        assert!(key_event_matches_chord(&z, &mods(false, true, false, false), "mod+z"));
        // shift held but not declared → no match; declared shift required.
        assert!(!key_event_matches_chord(&z, &mods(true, false, true, false), "mod+z"));
        assert!(key_event_matches_chord(&z, &mods(true, false, true, false), "mod+shift+z"));
        // plain key must not fire while the accelerator is held.
        let k = KeyAction::Char("k".into());
        assert!(key_event_matches_chord(&k, &mods(false, false, false, false), "k"));
        assert!(!key_event_matches_chord(&k, &mods(true, false, false, false), "k"));
        assert!(key_event_matches_chord(&KeyAction::Escape, &mods(false, false, false, false), "escape"));
    }

    #[test]
    fn required_arg_gates_execution_and_merges_defaults() {
        let app = actions_utilities_app();
        let defs = &app.window_kinds.iter().find(|kind| kind.id == "main").unwrap().actions.iter().find(|action| action.id == "withArgs").unwrap().args;
        // Nothing staged → required `name` missing → no executable args (P2 gate).
        assert!(ShellState::resolved_execute_args(defs, &serde_json::Map::new()).is_none());
        // Stage the required arg → executes, merging the defaulted `flag`.
        let mut staged = serde_json::Map::new();
        staged.insert("name".into(), serde_json::json!("hello"));
        let merged = ShellState::resolved_execute_args(defs, &staged).expect("executable");
        assert_eq!(merged.get("name"), Some(&serde_json::json!("hello")));
        assert_eq!(merged.get("flag"), Some(&serde_json::json!(true)));
    }

    #[test]
    fn staging_stage_and_reset_roundtrip() {
        let mut shell = shell();
        shell.stage_arg("main", "withArgs", "name", serde_json::json!("x"));
        shell.stage_arg("main", "withArgs", "flag", serde_json::json!(false));
        let staged = shell.staged_map_for("main", "withArgs");
        assert_eq!(staged.get("name"), Some(&serde_json::json!("x")));
        assert_eq!(staged.get("flag"), Some(&serde_json::json!(false)));
        shell.reset_staged_args("main", "withArgs");
        assert!(shell.staged_map_for("main", "withArgs").is_empty());
    }

    #[test]
    fn utility_activation_toggles_and_switches() {
        let mut shell = shell();
        shell.apply_set_active_utility("main", "utility.a");
        assert_eq!(shell.active_utility_for_window("main"), Some("utility.a"));
        // Re-selecting the active utility deactivates it (the same update a re-click / Escape performs).
        shell.apply_set_active_utility("main", "utility.a");
        assert_eq!(shell.active_utility_for_window("main"), None);
        // Switching to a different utility activates it.
        shell.apply_set_active_utility("main", "utility.a");
        shell.apply_set_active_utility("main", "utility.b");
        assert_eq!(shell.active_utility_for_window("main"), Some("utility.b"));
    }

    #[test]
    fn active_utility_gates_actions_unless_allowed() {
        let app = actions_utilities_app();
        let mut shell = shell();
        // No active utility → actions enabled.
        assert!(shell.actions_enabled_for_window(&app, "main"));
        // utility.a defaults to `allows_actions_while_active = false` → actions gated.
        shell.apply_set_active_utility("main", "utility.a");
        assert!(!shell.actions_enabled_for_window(&app, "main"));
        // utility.b sets the flag true → actions stay enabled.
        shell.apply_set_active_utility("main", "utility.a");
        shell.apply_set_active_utility("main", "utility.b");
        assert!(shell.actions_enabled_for_window(&app, "main"));
    }

    #[test]
    fn action_host_window_id_finds_scoping_window() {
        let app = actions_utilities_app();
        assert_eq!(crate::shell::action_host_window_id(&app, "withArgs").as_deref(), Some("main"));
    }

    /// 🎯️ The Utility Options rail (`render_utility_options_rail`) resolves its content through
    /// `partition_window_measures`: a tagged group surfaces only for its matching active utility, and is
    /// absent from BOTH buckets otherwise — untagged groups always stay in the general Measures rail.
    #[test]
    fn utility_options_partition_gates_tagged_group_by_active_utility() {
        use ui_wgpu::wgpu::{partition_window_measures, ActionDescriptor, WindowMeasure};
        let measures = vec![
            WindowMeasure::Group {
                id: "brush-params".into(),
                label: "Brush".into(),
                default_open: Some(true),
                active_utility_id: Some("utility.a".into()),
                value: None,
                min: None,
                max: None,
                step: None,
                ready: None,
                loading: None,
                waiting: None,
                on_change: None,
                children: vec![WindowMeasure::Toggle {
                    id: "brush-size".into(),
                    icon_id: "paintbrush".into(),
                    label: Some("Size".into()),
                    pressed: false,
                    text: None,
                    on_change: ActionDescriptor { controller_id: "test".into(), action: "noOperation".into(), args: None },
                }],
            },
            WindowMeasure::Group {
                id: "grid".into(),
                label: "Grid".into(),
                default_open: Some(true),
                active_utility_id: None,
                value: None,
                min: None,
                max: None,
                step: None,
                ready: None,
                loading: None,
                waiting: None,
                on_change: None,
                children: vec![],
            },
        ];
        let (general, utility_options) = partition_window_measures(&measures, Some("utility.a"));
        assert_eq!(utility_options.len(), 1, "matching utility surfaces the tagged group in utility options");
        assert!(matches!(&utility_options[0], WindowMeasure::Toggle { id, .. } if id == "brush-size"));
        assert_eq!(general.len(), 1, "untagged group stays in the general measures rail");
        assert!(matches!(&general[0], WindowMeasure::Group { id, .. } if id == "grid"));
        let (general_other, utility_options_other) = partition_window_measures(&measures, Some("utility.b"));
        assert!(utility_options_other.is_empty(), "wrong active utility drops the tagged group");
        assert_eq!(general_other.len(), 1, "untagged group unaffected by active utility");
        let (general_none, utility_options_none) = partition_window_measures(&measures, None);
        assert!(utility_options_none.is_empty(), "no active utility drops the tagged group");
        assert_eq!(general_none.len(), 1);
    }
    //#endregion WindowActionsAndUtilitiesTests
}
//#endregion DockTests
