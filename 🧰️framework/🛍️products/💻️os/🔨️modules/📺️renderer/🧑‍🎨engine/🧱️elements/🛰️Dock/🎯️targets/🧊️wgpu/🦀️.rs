//! 🪟️ framework/products/os/modules/renderer/engine/elements/🛰️Dock/component.rs — wgpu layout and
//! render implementation for the Dock element, extracted from lib.rs's inline
//! `pub mod dock { ... }` body (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE). Wired via
//! `#[path = "../../../../🧱️elements/🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs"] pub mod dock;` in lib.rs in place of the
//! former inline block; the module name `dock` is unchanged, so every existing `crate::dock::...`
//! call site elsewhere in the crate keeps resolving with zero other changes.
//! 🪟️ Mode dock — multi-window layout tree with stack chrome and split resize.

use semio_framework::AppDefinition;
use std::collections::HashMap;
use ui_wgpu::wgpu::{
    chrome_item_text, draw_text, even_window_layout, ActionDescriptor, DragAxis, DrawList, FontAtlas, HitKind, HitTarget, IconAtlas, InputState, Level, Rect, Rgba, Theme, WindowLayout, WindowLayoutChild, WindowLayoutRoot, WindowLayoutStackNode,
    WindowLayoutWindowNode, WindowStackCorner,
};

pub type DockPath = Vec<usize>;

fn empty_path() -> DockPath {
    Vec::new()
}

//#region DockTypes
/// 🧭️ Per-tab chrome placement inside a dock stack.
#[derive(Clone, Debug, PartialEq)]
pub struct DockStackTab {
    pub window_id: String,
    pub window_kind_id: String,
    pub corner: WindowStackCorner,
}

impl DockStackTab {
    /// 🧭️ Builds a tab at the default top-left corner.
    pub fn new(window_id: impl Into<String>) -> Self {
        let window_id = window_id.into();
        Self { window_kind_id: window_id.clone(), window_id, corner: WindowStackCorner::TopLeft }
    }

    /// 🧭️ Builds a tab at an explicit chrome corner.
    pub fn at(window_id: impl Into<String>, corner: WindowStackCorner) -> Self {
        let window_id = window_id.into();
        Self { window_kind_id: window_id.clone(), window_id, corner }
    }

    pub fn instance(window_id: impl Into<String>, window_kind_id: impl Into<String>, corner: WindowStackCorner) -> Self {
        Self { window_id: window_id.into(), window_kind_id: window_kind_id.into(), corner }
    }
}

fn dock_tabs_from_ids(ids: impl IntoIterator<Item = impl Into<String>>) -> Vec<DockStackTab> {
    ids.into_iter().map(|id| DockStackTab::new(id)).collect()
}

fn dock_tab_ids(windows: &[DockStackTab]) -> Vec<String> {
    windows.iter().map(|tab| tab.window_id.clone()).collect()
}

fn flat_index_for_corner_insert(windows: &[DockStackTab], corner: WindowStackCorner, corner_index: Option<usize>) -> usize {
    let indices: Vec<usize> = windows.iter().enumerate().filter(|(_, tab)| tab.corner == corner).map(|(i, _)| i).collect();
    if indices.is_empty() {
        return windows.len();
    }
    match corner_index {
        None => indices[indices.len() - 1] + 1,
        Some(i) if i >= indices.len() => indices[indices.len() - 1] + 1,
        Some(i) => indices[i],
    }
}

fn tabs_by_corner(windows: &[DockStackTab]) -> [(WindowStackCorner, Vec<&DockStackTab>); 4] {
    let mut groups: [(WindowStackCorner, Vec<&DockStackTab>); 4] = [(WindowStackCorner::TopLeft, Vec::new()), (WindowStackCorner::TopRight, Vec::new()), (WindowStackCorner::BottomLeft, Vec::new()), (WindowStackCorner::BottomRight, Vec::new())];
    for tab in windows {
        let slot = match tab.corner {
            WindowStackCorner::TopLeft => 0,
            WindowStackCorner::TopRight => 1,
            WindowStackCorner::BottomLeft => 2,
            WindowStackCorner::BottomRight => 3,
        };
        groups[slot].1.push(tab);
    }
    groups
}

#[derive(Clone, Debug, PartialEq)]
pub enum DockNode {
    Row(Vec<(DockNode, f32)>),
    Column(Vec<(DockNode, f32)>),
    Stack { windows: Vec<DockStackTab>, active: String },
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
    pub window_labels: &'a HashMap<String, String>,
    pub window_icon_ids: &'a HashMap<String, String>,
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
    Tab { stack_path: DockPath, corner: WindowStackCorner, index: usize },
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
    pub window_kind_id: String,
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
        let DockNode::Stack { active, .. } = stack else {
            return false;
        };
        let window_id = active.clone();
        self.close_window_in_stack(path, &window_id)
    }

    /// 🧭️ Closes a specific tab in a stack (refuses to close the last remaining window).
    pub fn close_window_in_stack(&mut self, path: &DockPath, window_id: &str) -> bool {
        let Some(stack) = node_at_mut(&mut self.root, path) else {
            return false;
        };
        let DockNode::Stack { windows, active } = stack else {
            return false;
        };
        if windows.len() <= 1 {
            return false;
        }
        let Some(idx) = windows.iter().position(|tab| tab.window_id == window_id) else {
            return false;
        };
        windows.remove(idx);
        if active == window_id {
            let next = windows.get(idx.saturating_sub(1)).or_else(|| windows.first()).map(|tab| tab.window_id.clone());
            if let Some(id) = next {
                *active = id.clone();
                self.active_window_id = Some(id);
            }
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
                    let layout = layout_stack_cap(windows, window_labels, &HashMap::new(), atlas, theme, rect);
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
        if let Some(path) = &self.maximized_stack {
            out.push((path.clone(), stack_tab_bar_rect(bounds, theme)));
            return out;
        }
        collect_stack_tab_bars(&self.root, bounds, &empty_path(), theme, &mut out);
        out
    }

    /// 🧭️ Per-corner tab-bar hit regions (and tab widths) for dock drop targeting.
    pub fn stack_corner_tab_bar_rects(&self, bounds: Rect, theme: &Theme, atlas: &mut FontAtlas, window_labels: &HashMap<String, String>) -> Vec<(DockPath, WindowStackCorner, Rect, Vec<f32>)> {
        let mut out = Vec::new();
        if let Some(path) = &self.maximized_stack {
            if let Some(DockNode::Stack { windows, .. }) = node_at(&self.root, path) {
                collect_corner_tab_bars_for_stack(path, windows, bounds, theme, atlas, window_labels, &mut out);
            }
            return out;
        }
        collect_stack_corner_tab_bars(&self.root, bounds, &empty_path(), theme, atlas, window_labels, &mut out);
        out
    }

    pub fn tab_index(&self, path: &DockPath, window_id: &str) -> Option<usize> {
        let DockNode::Stack { windows, .. } = node_at(&self.root, path)? else {
            return None;
        };
        windows.iter().position(|tab| tab.window_id == window_id)
    }

    /// 🎯️ Commits a completed dock drag. `drag.window_id` is removed from `self.root` *eagerly* the
    /// moment a drag is promoted (see `ShellState::handle_pointer_move`'s `dock.remove_window` call,
    /// `shell` region) — by the time this runs it is already absent, so every branch below only
    /// *inserts* it at the resolved zone. (Previously this also re-removed it before inserting, which
    /// — given it was already gone — silently no-opped every cross-stack/side drop, or for a same-path
    /// drop, reordered whatever now-shifted window happened to sit at the stale `tab_index`. Real
    /// drops never actually landed; see `DockTests::apply_drop_*` for pinned regressions.)
    pub fn apply_drop(&mut self, drag: &DockDragPayload, zone: &DockDropZone) -> bool {
        let mut window_kinds: HashMap<String, String> = self.window_instances().into_iter().collect();
        window_kinds.insert(drag.window_id.clone(), drag.window_kind_id.clone());
        match drag.kind {
            DockDragKind::Tab => match zone {
                DockDropZone::Tab { stack_path, corner, index } => self.insert_tabs_at_corner_with_kinds(stack_path, &[drag.window_id.clone()], *corner, Some(*index), &drag.window_id, &window_kinds),
                DockDropZone::Split { stack_path, side } => self.split_stack_with_stack_and_kinds(stack_path, vec![drag.window_id.clone()], drag.window_id.clone(), *side, &window_kinds),
                DockDropZone::RootSplit { side } => self.split_root_with_stack_and_kinds(vec![drag.window_id.clone()], drag.window_id.clone(), *side, &window_kinds),
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
                    DockDropZone::Tab { stack_path, corner, index } => {
                        let target_path = target_anchor.as_deref().and_then(|id| find_stack_path(&self.root, id, &mut vec![])).unwrap_or_else(|| stack_path.clone());
                        self.insert_tabs_at_corner_with_kinds(&target_path, &group, *corner, Some(*index), &drag.window_id, &window_kinds)
                    }
                    DockDropZone::Split { stack_path, side } => {
                        let target_path = target_anchor.as_deref().and_then(|id| find_stack_path(&self.root, id, &mut vec![])).unwrap_or_else(|| stack_path.clone());
                        self.split_stack_with_stack_and_kinds(&target_path, group, drag.window_id.clone(), *side, &window_kinds)
                    }
                    DockDropZone::RootSplit { side } => self.split_root_with_stack_and_kinds(group, drag.window_id.clone(), *side, &window_kinds),
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
        self.insert_tab_at_corner(path, window_id, WindowStackCorner::TopLeft, index)
    }

    /// 🧭️ Inserts a tab into a specific chrome corner at a corner-local index.
    pub fn insert_tab_at_corner(&mut self, path: &DockPath, window_id: &str, corner: WindowStackCorner, index: Option<usize>) -> bool {
        self.insert_tabs_at_corner(path, &[window_id.to_string()], corner, index, window_id)
    }

    /// 🗄️ Inserts every window in `🪟️windows` (in order) starting at `index`, skipping any already
    /// present at the target (mirrors `insert_tab`'s dedupe guard), then focuses `active_id` — the
    /// multi-window counterpart `apply_drop` uses for whole-stack tab-join drops.
    pub fn insert_tabs(&mut self, path: &DockPath, windows: &[String], index: Option<usize>, active_id: &str) -> bool {
        self.insert_tabs_at_corner(path, windows, WindowStackCorner::TopLeft, index, active_id)
    }

    /// 🧭️ Corner-aware multi-tab insert used by tab-join drops.
    pub fn insert_tabs_at_corner(&mut self, path: &DockPath, windows: &[String], corner: WindowStackCorner, index: Option<usize>, active_id: &str) -> bool {
        self.insert_tabs_at_corner_with_kinds(path, windows, corner, index, active_id, &HashMap::new())
    }

    fn insert_tabs_at_corner_with_kinds(&mut self, path: &DockPath, windows: &[String], corner: WindowStackCorner, index: Option<usize>, active_id: &str, window_kinds: &HashMap<String, String>) -> bool {
        if windows.is_empty() {
            return false;
        }
        let Some(stack) = node_at_mut(&mut self.root, path) else {
            return false;
        };
        let DockNode::Stack { windows: target, active } = stack else {
            return false;
        };
        let mut insert_at = flat_index_for_corner_insert(target, corner, index);
        let mut inserted_any = false;
        for window_id in windows {
            if target.iter().any(|tab| tab.window_id == *window_id) {
                continue;
            }
            target.insert(insert_at, DockStackTab::instance(window_id.clone(), window_kinds.get(window_id).cloned().unwrap_or_else(|| window_id.clone()), corner));
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
        self.split_stack_with_stack_and_kinds(path, windows, active_id, side, &HashMap::new())
    }

    fn split_stack_with_stack_and_kinds(&mut self, path: &DockPath, windows: Vec<String>, active_id: String, side: DockSide, window_kinds: &HashMap<String, String>) -> bool {
        let Some(stack_node) = node_at(&self.root, path).cloned() else {
            return false;
        };
        let DockNode::Stack { .. } = stack_node else {
            return false;
        };
        let new_stack = DockNode::Stack {
            windows: windows.into_iter().map(|id| DockStackTab::instance(id.clone(), window_kinds.get(&id).cloned().unwrap_or(id), WindowStackCorner::TopLeft)).collect(),
            active: active_id.clone(),
        };
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
        self.split_root_with_stack_and_kinds(windows, active_id, side, &HashMap::new())
    }

    fn split_root_with_stack_and_kinds(&mut self, windows: Vec<String>, active_id: String, side: DockSide, window_kinds: &HashMap<String, String>) -> bool {
        let tabs = || windows.iter().map(|id| DockStackTab::instance(id.clone(), window_kinds.get(id).cloned().unwrap_or_else(|| id.clone()), WindowStackCorner::TopLeft)).collect();
        let current = std::mem::replace(&mut self.root, DockNode::Stack { windows: vec![], active: String::new() });
        if matches!(&current, DockNode::Stack { windows, .. } if windows.is_empty()) {
            self.root = DockNode::Stack { windows: tabs(), active: active_id.clone() };
        } else {
            let new_stack = DockNode::Stack { windows: tabs(), active: active_id.clone() };
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
        Some(dock_tab_ids(windows))
    }

    /// 🧭️ Returns the full tab records (ids + corners) for a stack path.
    pub fn stack_tabs_at_path(&self, path: &DockPath) -> Option<Vec<DockStackTab>> {
        let DockNode::Stack { windows, .. } = node_at(&self.root, path)? else {
            return None;
        };
        Some(windows.clone())
    }

    pub fn window_instances(&self) -> Vec<(String, String)> {
        fn collect(node: &DockNode, out: &mut Vec<(String, String)>) {
            match node {
                DockNode::Stack { windows, .. } => out.extend(windows.iter().map(|tab| (tab.window_id.clone(), tab.window_kind_id.clone()))),
                DockNode::Row(children) | DockNode::Column(children) => children.iter().for_each(|(child, _)| collect(child, out)),
            }
        }
        let mut instances = Vec::new();
        collect(&self.root, &mut instances);
        instances
    }

    pub fn window_kind_id(&self, window_id: &str) -> Option<&str> {
        fn find<'a>(node: &'a DockNode, window_id: &str) -> Option<&'a str> {
            match node {
                DockNode::Stack { windows, .. } => windows.iter().find(|tab| tab.window_id == window_id).map(|tab| tab.window_kind_id.as_str()),
                DockNode::Row(children) | DockNode::Column(children) => children.iter().find_map(|(child, _)| find(child, window_id)),
            }
        }
        find(&self.root, window_id)
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
    let windows: Vec<DockStackTab> = stack
        .children
        .iter()
        .map(|window| DockStackTab::instance(window.instance_id.clone().unwrap_or_else(|| window.window_kind_id.clone()), window.window_kind_id.clone(), window.corner.unwrap_or(WindowStackCorner::TopLeft)))
        .collect();
    let active = stack
        .active_window_kind_id
        .as_ref()
        .and_then(|id| windows.iter().find(|window| window.window_id == *id).or_else(|| windows.iter().find(|window| window.window_kind_id == *id)))
        .map(|window| window.window_id.clone())
        .or_else(|| windows.first().map(|window| window.window_id.clone()))
        .unwrap_or_default();
    DockNode::Stack { windows, active }
}

fn layout_window_node(tab: &DockStackTab) -> WindowLayoutWindowNode {
    WindowLayoutWindowNode {
        kind: "window".into(),
        window_kind_id: tab.window_kind_id.clone(),
        title: None,
        instance_id: (tab.window_id != tab.window_kind_id).then(|| tab.window_id.clone()),
        template_id: None,
        corner: Some(tab.corner),
    }
}

pub fn dock_node_to_layout_root(node: &DockNode) -> WindowLayoutRoot {
    match node {
        DockNode::Stack { windows, active } => WindowLayoutRoot::Stack(WindowLayoutStackNode { kind: "stack".into(), size: None, active_window_kind_id: Some(active.clone()), children: windows.iter().map(layout_window_node).collect() }),
        DockNode::Row(children) => WindowLayoutRoot::Axis(ui_wgpu::wgpu::WindowLayoutAxisNode { kind: "row".into(), size: None, children: children.iter().map(|(child, size)| dock_child_from_node(child, *size)).collect() }),
        DockNode::Column(children) => WindowLayoutRoot::Axis(ui_wgpu::wgpu::WindowLayoutAxisNode { kind: "column".into(), size: None, children: children.iter().map(|(child, size)| dock_child_from_node(child, *size)).collect() }),
    }
}

fn dock_child_from_node(node: &DockNode, size: f32) -> WindowLayoutChild {
    match node {
        DockNode::Stack { windows, active } => WindowLayoutChild::Stack(WindowLayoutStackNode { kind: "stack".into(), size: Some(size as f64), active_window_kind_id: Some(active.clone()), children: windows.iter().map(layout_window_node).collect() }),
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
            if let Some(index) = windows.iter().position(|tab| tab.window_id == window_id) {
                windows.remove(index);
                if active == window_id {
                    *active = windows.get(index.saturating_sub(1)).or_else(|| windows.first()).map(|tab| tab.window_id.clone()).unwrap_or_default();
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

pub fn compute_dock_drop_zone(pointer_x: f32, pointer_y: f32, tab_bars: &[(DockPath, WindowStackCorner, Rect, Vec<f32>)], bodies: &[(DockPath, Rect, String)], canvas: Rect) -> Option<DockDropZone> {
    for (path, corner, rect, widths) in tab_bars {
        if rect.contains(pointer_x, pointer_y) {
            let index = compute_tab_insert_index(pointer_x, *rect, widths, 4.0);
            return Some(DockDropZone::Tab { stack_path: path.clone(), corner: *corner, index });
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
pub fn drop_zone_indicator_rect(zone: &DockDropZone, tab_bars: &[(DockPath, WindowStackCorner, Rect, Vec<f32>)], bodies: &[(DockPath, Rect, String)], canvas: Rect, gap: f32) -> Option<Rect> {
    match zone {
        DockDropZone::Tab { stack_path, corner, index } => {
            let (_, _, tab_bar, widths) = tab_bars.iter().find(|(path, bar_corner, _, _)| path == stack_path && bar_corner == corner)?;
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

fn dock_tab_chip_width(atlas: &mut FontAtlas, theme: &Theme, label: &str, action_count: usize) -> f32 {
    dock_tab_content_width(atlas, theme, label) + dock_tab_action_width(theme) * action_count as f32
}

fn dock_tab_action_width(theme: &Theme) -> f32 {
    14.0 + theme.padding_standard
}

fn collect_corner_tab_bars_for_stack(path: &DockPath, windows: &[DockStackTab], bounds: Rect, theme: &Theme, atlas: &mut FontAtlas, window_labels: &HashMap<String, String>, out: &mut Vec<(DockPath, WindowStackCorner, Rect, Vec<f32>)>) {
    let tab_h = theme.control_height;
    let pad = 8.0;
    for (corner, tabs) in tabs_by_corner(windows) {
        let widths: Vec<f32> = tabs
            .iter()
            .map(|tab| {
                let label = window_labels.get(&tab.window_id).or_else(|| window_labels.get(&tab.window_kind_id)).map(String::as_str).unwrap_or(tab.window_id.as_str());
                dock_tab_chip_width(atlas, theme, label, 3)
            })
            .collect();
        let total_w = if widths.is_empty() { pad * 3.0 } else { widths.iter().sum::<f32>() };
        let width = total_w.max(pad * 3.0);
        let rect = match corner {
            WindowStackCorner::TopLeft => Rect::new(bounds.x, bounds.y, width, tab_h),
            WindowStackCorner::TopRight => Rect::new(bounds.x + bounds.w - width, bounds.y, width, tab_h),
            WindowStackCorner::BottomLeft => Rect::new(bounds.x, bounds.y + bounds.h - tab_h, width, tab_h),
            WindowStackCorner::BottomRight => Rect::new(bounds.x + bounds.w - width, bounds.y + bounds.h - tab_h, width, tab_h),
        };
        out.push((path.to_vec(), corner, rect, widths));
    }
}

fn collect_stack_corner_tab_bars(node: &DockNode, bounds: Rect, path: &[usize], theme: &Theme, atlas: &mut FontAtlas, window_labels: &HashMap<String, String>, out: &mut Vec<(DockPath, WindowStackCorner, Rect, Vec<f32>)>) {
    match node {
        DockNode::Row(children) => {
            let total: f32 = children.iter().map(|(_, s)| *s).sum::<f32>().max(0.001);
            let mut x = bounds.x;
            for (index, (child, size)) in children.iter().enumerate() {
                let w = bounds.w * (*size / total);
                let mut child_path = path.to_vec();
                child_path.push(index);
                collect_stack_corner_tab_bars(child, Rect::new(x, bounds.y, w, bounds.h), &child_path, theme, atlas, window_labels, out);
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
                collect_stack_corner_tab_bars(child, Rect::new(bounds.x, y, bounds.w, h), &child_path, theme, atlas, window_labels, out);
                y += h;
            }
        }
        DockNode::Stack { windows, .. } => {
            collect_corner_tab_bars_for_stack(&path.to_vec(), windows, bounds, theme, atlas, window_labels, out);
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
        DockNode::Stack { windows, .. } if windows.iter().any(|tab| tab.window_id == window_id) => Some(path.clone()),
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
            let same_membership = old_windows.len() == next_windows.len() && old_windows.iter().all(|tab| next_windows.iter().any(|next| next.window_id == tab.window_id));
            let active = if same_membership && old_windows.iter().any(|tab| tab.window_id == *old_active) { old_active.clone() } else { next_active.clone() };
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

fn render_axis(state: &DockState, ctx: &mut DockRenderContext<'_>, children: &[(DockNode, f32)], bounds: Rect, path: &[usize], horizontal: bool, body_fill: bool, render_body: &mut dyn FnMut(Rect, &str), outer_split: Option<(DockPath, usize, bool)>) {
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
    let _stroke = theme.stroke_hairline;
    let globally_active = state.active_stack.as_ref().map(|p| p.as_slice()) == Some(path);
    let stack_hovered = bounds.contains(ctx.input.pointer_x, ctx.input.pointer_y);
    let _border = if globally_active {
        theme.accent
    } else if stack_hovered {
        theme.border_emphasized
    } else {
        theme.border_normal
    };

    let layout = layout_stack_cap(windows, ctx.window_labels, ctx.window_icon_ids, ctx.atlas, theme, bounds);
    let silhouette = stack_window_silhouette(bounds, theme, &layout);

    if body_fill {
        let content_bounds = silhouette.content_bounds();
        ctx.draw.begin_silhouette_clip(&silhouette.content_clip_rects());
        ctx.draw.push_solid([content_bounds.x, content_bounds.y, content_bounds.w, content_bounds.h], theme.canvas_clear);
        render_body(content_bounds, active);
        ctx.draw.end_silhouette_clip();
    }

    let focus_icon = if maximized { "minimize-2" } else { "maximize-2" };
    for group in &layout.groups {
        if group.tabs.is_empty() {
            continue;
        }
        let group_w = group.tabs.iter().map(|tab| tab.rect.w).sum::<f32>();
        let group_rect = Rect::new(group.tabs[0].rect.x, group.tabs[0].rect.y, group_w, tab_h);
        let group_glass = ctx.draw.push_glass([group_rect.x, group_rect.y, group_rect.w, group_rect.h], 0.0, theme.glass(Level::Window));
        ctx.draw.begin_glass_content(group_glass);
        for tab in &group.tabs {
            let is_active = tab.window_id == *active;
            let stack_active_tab = is_active && globally_active;
            let hovered = tab.rect.contains(ctx.input.pointer_x, ctx.input.pointer_y);
            let tint = if stack_active_tab {
                theme.active_foreground
            } else if hovered {
                theme.border_emphasized
            } else {
                theme.text_element
            };
            let mut content_x = tab.rect.x + theme.padding_standard;
            let icon_w = paint_dock_tab_icon(ctx, &tab.icon_id, content_x, tab.rect, tint);
            content_x += icon_w;
            dock_text(ctx, &tab.label, content_x, tab.rect.y + (tab.rect.h + theme.font_size_small) * 0.5 - 1.0, theme.font_size_small, tint);
            content_x += ctx.atlas.measure_text(&tab.label, theme.font_size_small).0 + theme.gap_standard;
            let action_w = dock_tab_action_width(theme);
            let actions = [("focus", focus_icon), ("new", "app-window"), ("close", "x")];
            for (action, icon_id) in actions {
                let action_rect = Rect::new(content_x, tab.rect.y, action_w, tab.rect.h);
                let action_hovered = action_rect.contains(ctx.input.pointer_x, ctx.input.pointer_y);
                let action_tint = chrome_item_text(theme, false, action_hovered);
                let _ = paint_dock_tab_icon(ctx, icon_id, action_rect.x + theme.padding_standard * 0.5, action_rect, action_tint);
                ctx.input.register_hit(HitTarget { rect: action_rect, event: None, control_id: Some(format!("dock.tab.{}.{}.{}", path_str(path), tab.window_id, action)), kind: HitKind::Button, drag_axis: None, drag_data: None });
                content_x += action_w;
            }
            let select_w = (tab.rect.w - action_w * 3.0).max(theme.padding_standard * 2.0);
            let select_rect = Rect::new(tab.rect.x, tab.rect.y, select_w, tab.rect.h);
            ctx.input.register_hit(HitTarget { rect: select_rect, event: None, control_id: Some(format!("dock.tab.{}.{}", path_str(path), tab.window_id)), kind: HitKind::Window, drag_axis: None, drag_data: None });
        }
        ctx.draw.end_glass_content();
    }

    // 🪟️ push_window_silhouette_border(ctx.draw, &silhouette, stroke, border);
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
        Self { bounds, top: WindowSilhouetteEdge::new(top_depth, Self::normalize_spans(top.spans, width)), bottom: WindowSilhouetteEdge::new(bottom_depth, Self::normalize_spans(bottom.spans, width)) }
    }

    /// 🪟️ Projects measured top/bottom chip spans into the normalized v1 model.
    pub fn from_measured_edges(bounds: Rect, top_spans: Vec<WindowSilhouetteSpan>, bottom_spans: Vec<WindowSilhouetteSpan>, top_depth: f32, bottom_depth: f32) -> Self {
        Self::new(bounds, WindowSilhouetteEdge::new(top_depth, top_spans), WindowSilhouetteEdge::new(bottom_depth, bottom_spans))
    }

    /// 🪟️ Projects the classic top-left tabs + top-right controls pair into the normalized v1 model.
    pub fn from_measured_top(bounds: Rect, tabs_w: f32, controls_w: f32, depth: f32) -> Self {
        let tabs_right = tabs_w.max(0.0).min(bounds.w);
        let controls_left = (bounds.w - controls_w.max(0.0)).max(tabs_right).min(bounds.w);
        Self::from_measured_edges(bounds, vec![WindowSilhouetteSpan::new(0.0, tabs_right), WindowSilhouetteSpan::new(controls_left, bounds.w)], Vec::new(), depth, 0.0)
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
        Rect::new(self.bounds.x, self.bounds.y + self.top.depth, self.bounds.w, (self.bounds.h - self.top.depth - self.bottom.depth).max(0.0))
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

struct StackCapTabLayout {
    window_id: String,
    label: String,
    icon_id: String,
    rect: Rect,
}

struct StackCapGroupLayout {
    tabs: Vec<StackCapTabLayout>,
}

struct StackCapLayout {
    groups: Vec<StackCapGroupLayout>,
    top_spans: Vec<WindowSilhouetteSpan>,
    bottom_spans: Vec<WindowSilhouetteSpan>,
}

fn layout_stack_cap(windows: &[DockStackTab], labels: &HashMap<String, String>, icon_ids: &HashMap<String, String>, atlas: &mut FontAtlas, theme: &Theme, bounds: Rect) -> StackCapLayout {
    let tab_h = theme.control_height;
    let mut groups = Vec::new();
    let mut top_spans = Vec::new();
    let mut bottom_spans = Vec::new();
    for (corner, tabs) in tabs_by_corner(windows) {
        if tabs.is_empty() {
            continue;
        }
        let mut widths = Vec::with_capacity(tabs.len());
        let mut meta = Vec::with_capacity(tabs.len());
        for tab in &tabs {
            let label = labels.get(&tab.window_id).or_else(|| labels.get(&tab.window_kind_id)).cloned().unwrap_or_else(|| tab.window_id.clone());
            let icon_id = icon_ids.get(&tab.window_id).or_else(|| icon_ids.get(&tab.window_kind_id)).cloned().unwrap_or_else(|| "app-window".into());
            let tw = dock_tab_chip_width(atlas, theme, &label, 3);
            widths.push(tw);
            meta.push((tab.window_id.clone(), label, icon_id));
        }
        let total_w = widths.iter().sum::<f32>();
        let (origin_x, origin_y) = match corner {
            WindowStackCorner::TopLeft => (bounds.x, bounds.y),
            WindowStackCorner::TopRight => (bounds.x + bounds.w - total_w, bounds.y),
            WindowStackCorner::BottomLeft => (bounds.x, bounds.y + bounds.h - tab_h),
            WindowStackCorner::BottomRight => (bounds.x + bounds.w - total_w, bounds.y + bounds.h - tab_h),
        };
        let mut x = origin_x;
        let mut painted = Vec::new();
        for ((window_id, label, icon_id), tw) in meta.into_iter().zip(widths.into_iter()) {
            painted.push(StackCapTabLayout { window_id, label, icon_id, rect: Rect::new(x, origin_y, tw, tab_h) });
            x += tw;
        }
        let span = WindowSilhouetteSpan::new((origin_x - bounds.x).max(0.0), (origin_x - bounds.x + total_w).min(bounds.w));
        match corner {
            WindowStackCorner::TopLeft | WindowStackCorner::TopRight => top_spans.push(span),
            WindowStackCorner::BottomLeft | WindowStackCorner::BottomRight => bottom_spans.push(span),
        }
        groups.push(StackCapGroupLayout { tabs: painted });
    }
    StackCapLayout { groups, top_spans, bottom_spans }
}

fn stack_window_silhouette(bounds: Rect, theme: &Theme, layout: &StackCapLayout) -> WindowSilhouette {
    let top_depth = if layout.top_spans.is_empty() { 0.0 } else { theme.control_height };
    let bottom_depth = if layout.bottom_spans.is_empty() { 0.0 } else { theme.control_height };
    WindowSilhouette::from_measured_edges(bounds, layout.top_spans.clone(), layout.bottom_spans.clone(), top_depth, bottom_depth)
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
            let _maximized = state.maximized_stack.as_ref().map(|p| p.as_slice()) == Some(path);
            let layout = layout_stack_cap(windows, window_labels, &HashMap::new(), atlas, theme, bounds);
            let silhouette = stack_window_silhouette(bounds, theme, &layout);
            out.push((path.to_vec(), silhouette.safe_body_rect(), active.clone()));
            silhouettes.insert(active.clone(), silhouette);
        }
    }
}

fn dock_text(ctx: &mut DockRenderContext<'_>, text: &str, x: f32, y: f32, size: f32, color: Rgba) {
    let mut scroll = HashMap::new();
    let mut collapsed = HashMap::new();
    let mut selects = HashMap::new();
    let mut widget_ctx = crate::interpreter::framework_widget_context(ctx.draw, None, ctx.atlas, Some(ctx.icons), ctx.input, ctx.theme, &mut scroll, &mut collapsed, &mut selects, None);
    draw_text(&mut widget_ctx, text, x, y, size, color);
}

//#endregion DockFreeFunctions

//#region DockTests
#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../🧪️tests/🔬️wgpu-unit/🦀️.rs"]
mod tests;
//#endregion DockTests
