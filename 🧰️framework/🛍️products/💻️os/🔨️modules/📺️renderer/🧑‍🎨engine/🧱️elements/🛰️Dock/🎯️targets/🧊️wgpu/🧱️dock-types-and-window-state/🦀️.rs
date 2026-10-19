//! 🧱️ Dock types and window state: the dock path, the stack tab and node, the dock state, the drag types and the window management, split drag and frame rect methods of `DockState`. See [`super`].

use super::*;

pub type DockPath = Vec<usize>;

/// ↔️ Smallest share of an axis one split child may keep — React's `applyAxisResizeDelta(… , minPct = 8)`
/// (`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎨️Canvas/🟦️.tsx:593-611`) on this crate's `1.0` scale.
pub const SPLIT_MIN_FRACTION: f32 = 0.08;

/// ↔️ React ignores a resize delta under `0.001` percent (`🎨️Canvas/🟦️.tsx:594`) — the same guard on
/// the fraction scale.
const SPLIT_DELTA_EPSILON: f32 = 0.00001;

pub(super) fn empty_path() -> DockPath {
    Vec::new()
}

//#region DockTypes
/// 🧭️ Per-tab chrome placement inside a dock stack.
#[derive(Clone, Debug, PartialEq)]
pub struct DockStackTab {
    pub window_id: String,
    pub window_kind_id: String,
    pub template_id: Option<String>,
    pub corner: WindowStackCorner,
}

impl DockStackTab {
    /// 🧭️ Builds a tab at the default top-left corner.
    pub fn new(window_id: impl Into<String>) -> Self {
        let window_id = window_id.into();
        Self { window_kind_id: window_id.clone(), window_id, template_id: None, corner: WindowStackCorner::TopLeft }
    }

    /// 🧭️ Builds a tab at an explicit chrome corner.
    pub fn at(window_id: impl Into<String>, corner: WindowStackCorner) -> Self {
        let window_id = window_id.into();
        Self { window_kind_id: window_id.clone(), window_id, template_id: None, corner }
    }

    pub fn instance(window_id: impl Into<String>, window_kind_id: impl Into<String>, corner: WindowStackCorner) -> Self {
        Self { window_id: window_id.into(), window_kind_id: window_kind_id.into(), template_id: None, corner }
    }

    /// 🪟️ Builds one concrete window instance carrying its authored projection template.
    pub fn instance_template(window_id: impl Into<String>, window_kind_id: impl Into<String>, template_id: Option<String>, corner: WindowStackCorner) -> Self {
        Self { window_id: window_id.into(), window_kind_id: window_kind_id.into(), template_id, corner }
    }
}

pub fn dock_tab_ids(windows: &[DockStackTab]) -> Vec<String> {
    windows.iter().map(|tab| tab.window_id.clone()).collect()
}

pub(super) fn flat_index_for_corner_insert(windows: &[DockStackTab], corner: WindowStackCorner, corner_index: Option<usize>) -> usize {
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

pub(super) fn tabs_by_corner(windows: &[DockStackTab]) -> [(WindowStackCorner, Vec<&DockStackTab>); 4] {
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
        Self { root: empty_stack(), active_window_id: None, maximized_stack: None, active_stack: None, split_resize_origin: vec![], mobile: false }
    }
}

#[derive(Clone, Debug)]
pub struct DockState {
    pub root: DockNode,
    pub active_window_id: Option<String>,
    pub maximized_stack: Option<DockPath>,
    pub active_stack: Option<DockPath>,
    pub split_resize_origin: Vec<f32>,
    /// 📱️ Viewport narrower than [`MODE_DOCK_MOBILE_MAX_WIDTH_PX`] — React's `Mode`/`ModeDockStack`
    /// `mobile` prop (`🧱️elements/🎨️Canvas/🟦️.tsx:1136-1158`), which hides the Focus/Unfocus tab
    /// action because a mobile window always fills the canvas anyway. Close stays reachable.
    pub mobile: bool,
}

/// 🏷️ The semantic name of a control painted by this Dock walk.
pub enum DockControlName<'a> {
    Body(&'a str),
    Tab { title: &'a str, selected: bool, panel: &'a str },
    Focus,
    Unfocus,
    Close,
    Drag(&'a str),
}

pub struct DockRenderContext<'a> {
    pub draw: &'a mut DrawList,
    pub atlas: &'a mut FontAtlas,
    pub icons: &'a IconAtlas,
    pub input: &'a mut InputState<ActionDescriptor>,
    pub theme: &'a Theme,
    pub window_labels: &'a HashMap<String, String>,
    pub window_icon_ids: &'a HashMap<String, String>,
    pub control_names: Option<&'a mut dyn FnMut(&str, DockControlName<'_>)>,
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
    NewWindow,
}

#[derive(Clone, Debug)]
pub struct DockDragPayload {
    pub kind: DockDragKind,
    pub window_id: String,
    pub window_kind_id: String,
    pub template_id: Option<String>,
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
        let active = active_window_id.and_then(|id| find_stack_path(&root, id, &mut vec![]).map(|_| id.to_string())).or_else(|| first_window_id(&root));
        let active_stack = active.as_ref().and_then(|id| find_stack_path(&root, id, &mut vec![]));
        Self { root, active_window_id: active, maximized_stack: None, active_stack, split_resize_origin: vec![], mobile: false }
    }

    /// 🪟️ Every window id the layout tree holds, in layout order — twin of React's
    /// `modeCollectWindowIds` (`🧱️elements/🎨️Canvas/🟦️.tsx:211-215`).
    pub fn collect_window_ids(&self) -> Vec<String> {
        fn walk(node: &DockNode, out: &mut Vec<String>) {
            match node {
                DockNode::Stack { windows, .. } => out.extend(windows.iter().map(|tab| tab.window_id.clone())),
                DockNode::Row(children) | DockNode::Column(children) => children.iter().for_each(|(child, _)| walk(child, out)),
            }
        }
        let mut ids = Vec::new();
        walk(&self.root, &mut ids);
        ids
    }

    /// 🧹️ Reconciles declared base windows and known extra instances before they enter the live dock.
    pub fn reconcile_app_windows(&mut self, app: &AppDefinition) {
        fn walk(node: &mut DockNode, declared: &std::collections::HashSet<&str>, instances: &HashMap<String, String>) {
            match node {
                DockNode::Stack { windows, active } => {
                    windows.retain_mut(|tab| {
                        if declared.contains(tab.window_kind_id.as_str()) {
                            return true;
                        }
                        let Some(kind) = instances.get(&tab.window_id) else {
                            return false;
                        };
                        tab.window_kind_id = kind.clone();
                        tab.template_id = None;
                        true
                    });
                    if !windows.iter().any(|tab| &tab.window_id == active) {
                        *active = windows.first().map(|tab| tab.window_id.clone()).unwrap_or_default();
                    }
                }
                DockNode::Row(children) | DockNode::Column(children) => {
                    for (child, _) in children {
                        walk(child, declared, instances);
                    }
                }
            }
        }
        let declared: std::collections::HashSet<&str> = app.window_kinds.iter().map(|kind| kind.id.as_str()).collect();
        let mut instances: HashMap<String, String> = app.window_kinds.iter().map(|kind| (kind.id.clone(), kind.id.clone())).collect();
        for (id, kind) in self.window_instances() {
            if declared.contains(kind.as_str()) {
                instances.entry(id).or_insert(kind);
            }
        }
        let maximized = self.maximized_stack.as_ref().and_then(|path| node_at(&self.root, path)).and_then(|node| match node {
            DockNode::Stack { active, .. } => Some(active.clone()),
            _ => None,
        });
        walk(&mut self.root, &declared, &instances);
        self.root = collapse_layout_node(&self.root, 1.0).map(|(node, _)| node).unwrap_or_else(empty_stack);
        self.active_window_id = self.active_window_id.take().filter(|id| find_stack_path(&self.root, id, &mut vec![]).is_some()).or_else(|| first_window_id(&self.root));
        self.active_stack = self.active_window_id.as_deref().and_then(|id| find_stack_path(&self.root, id, &mut vec![]));
        self.maximized_stack = maximized.and_then(|id| find_stack_path(&self.root, &id, &mut vec![]));
    }

    /// 🔲️ Whether maximize/restore is offered at all — React's
    /// `canMaximize = modeCollectWindowIds(layoutState).length > 1` (`🎨️Canvas/🟦️.tsx:1800`): a lone
    /// window already fills the canvas, so focusing it would be a no-operation.
    pub fn can_maximize(&self) -> bool {
        self.collect_window_ids().len() > 1
    }

    /// 📑️ Whether a tab paints the Focus/Unfocus action — React's
    /// `showMaximize = !mobile && Boolean(dock?.canMaximize)` (`🎨️Canvas/🟦️.tsx:1158`).
    pub fn show_maximize(&self) -> bool {
        !self.mobile && self.can_maximize()
    }

    /// 📑️ How many action chips each tab reserves width for — the one [`dock_tab_actions`] answer, so
    /// a measured tab width can never disagree with the painted chip run.
    pub fn tab_action_count(&self) -> usize {
        dock_tab_actions(self.show_maximize(), false).len()
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

    /// 🔲️ Maximize/restore one stack — React's `toggleMaximize` (`🎨️Canvas/🟦️.tsx:1488-1490`) plus the
    /// effect that drops a stale maximized path the moment the canvas falls back to a single window
    /// (`:1802-1805`), which is the same guard that makes the `mod+shift+enter` chord inert there.
    pub fn toggle_maximize(&mut self, path: &DockPath) {
        if !self.can_maximize() {
            self.maximized_stack = None;
            return;
        }
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

    /// 🧭️ Closes one tab of a stack — the twin of React's `closeWindow`
    /// (`🧱️elements/🎨️Canvas/🟦️.tsx:1475-1485`): `collapseLayout(removeWindowFromLayout(prev, id))`,
    /// falling back to an empty root stack, then `onActiveWindowChange(remaining[0] ?? null)`.
    ///
    /// 🩸️ It used to `return false` on `windows.len() <= 1`, so closing an app's ONLY window in a
    /// stack was a silent no-operation while React collapsed the surrounding split — and the tab's
    /// stack-local successor was picked backwards (`idx - 1`) where React takes `children[0]`.
    pub fn close_window_in_stack(&mut self, path: &DockPath, window_id: &str) -> bool {
        let Some(DockNode::Stack { windows, .. }) = node_at(&self.root, path) else {
            return false;
        };
        if !windows.iter().any(|tab| tab.window_id == window_id) {
            return false;
        }
        self.close_window(window_id)
    }

    /// 🧭️ Closes a window wherever it sits — React's `closeWindow` addresses windows by id, not by
    /// stack path, so the tab chip and the `mod+shift+w` chord share one collapse rule.
    pub fn close_window(&mut self, window_id: &str) -> bool {
        let maximized_key = self.maximized_stack.as_ref().and_then(|path| node_at(&self.root, path)).and_then(|node| match node {
            DockNode::Stack { active, .. } => Some(active.clone()),
            _ => None,
        });
        if !self.collect_window_ids().iter().any(|id| id == window_id) {
            return false;
        }
        let Some(next) = remove_window_from_layout(&self.root, window_id) else {
            self.root = empty_stack();
            self.active_window_id = None;
            self.active_stack = None;
            self.maximized_stack = None;
            self.split_resize_origin.clear();
            return true;
        };
        self.root = next;
        if self.active_window_id.as_deref() == Some(window_id) {
            self.active_window_id = self.collect_window_ids().into_iter().next();
        }
        self.active_stack = self.active_window_id.as_deref().and_then(|id| find_stack_path(&self.root, id, &mut vec![]));
        self.maximized_stack = maximized_key.filter(|id| id != window_id).and_then(|id| find_stack_path(&self.root, &id, &mut vec![])).filter(|_| self.can_maximize());
        self.split_resize_origin.clear();
        true
    }

    /// ↔️ React's `applyAxisResizeDelta` `minPct = 8` (`🧱️elements/🎨️Canvas/🟦️.tsx:593`),
    /// converted onto the captured axis's authored weight scale.
    ///
    /// 🩸️ The pair used to be clamped side-by-side to `[0.08, 0.92]` and then fed to a
    /// `normalize_pair_sizes` that was arithmetically a no-op, so a drag that hit the clamp changed
    /// the PAIR'S total and silently restretched every other child of the axis. React pins
    /// `nextRight = pairSize - nextLeft`, which conserves it.
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
        let delta_ratio = delta_px / axis_total;
        if delta_ratio.abs() < SPLIT_DELTA_EPSILON {
            return;
        }
        let origin_left = origin.get(split_index).copied().unwrap_or(children[split_index].1);
        let origin_right = origin.get(split_index + 1).copied().unwrap_or(children[split_index + 1].1);
        let axis_weight: f32 = (0..children.len()).map(|index| origin.get(index).copied().unwrap_or(children[index].1)).map(|weight| if weight.is_finite() { weight.max(0.0) } else { 0.0 }).sum();
        if axis_weight <= 0.0 {
            return;
        }
        let pair_sum = origin_left + origin_right;
        let minimum_weight = axis_weight * SPLIT_MIN_FRACTION;
        let new_left = (origin_left + delta_ratio * axis_weight).clamp(minimum_weight, (pair_sum - minimum_weight).max(minimum_weight));
        children[split_index].1 = new_left;
        children[split_index + 1].1 = pair_sum - new_left;
    }

    pub fn begin_split_drag(&mut self, path: &DockPath) -> Vec<f32> {
        let sizes = match node_at(&self.root, path) {
            Some(DockNode::Row(children) | DockNode::Column(children)) => children.iter().map(|(_, s)| *s).collect(),
            _ => vec![],
        };
        self.split_resize_origin = sizes.clone();
        sizes
    }

    /// 📐️ The raw stack FRAMES this layout solves for `bounds` — the pure axis arithmetic, before any
    /// theme, tab cap or silhouette touches it. `stack_body_rects` is this minus each stack's cap;
    /// separating them is what lets one language-neutral fixture state a layout's geometry without
    /// also pinning a theme metric (`🧫️fixtures/🪟️app-mode-layouts/🔣️.json`).
    pub fn stack_frame_rects(&self, bounds: Rect) -> Vec<(DockPath, Rect, String)> {
        let mut out = Vec::new();
        if let Some(path) = &self.maximized_stack {
            if let Some(DockNode::Stack { active, .. }) = node_at(&self.root, path) {
                out.push((path.clone(), bounds, active.clone()));
            }
            return out;
        }
        collect_stack_frames(&self.root, bounds, &empty_path(), 0.0, &mut out);
        out
    }

    /// 📐️ The physical stack frames after reserving one themed separator between axis siblings.
    pub fn stack_frame_rects_with_separator(&self, bounds: Rect, separator: f32) -> Vec<(DockPath, Rect, String)> {
        let mut out = Vec::new();
        if let Some(path) = &self.maximized_stack {
            if let Some(DockNode::Stack { active, .. }) = node_at(&self.root, path) {
                out.push((path.clone(), bounds, active.clone()));
            }
            return out;
        }
        collect_stack_frames(&self.root, bounds, &empty_path(), separator, &mut out);
        out
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
                    let layout = layout_stack_cap(windows, window_labels, &HashMap::new(), atlas, theme, rect, self.tab_action_count());
                    let silhouette = stack_window_silhouette(rect, theme, &layout);
                    out.push((path.clone(), dock_stack_body_rect(silhouette.safe_body_rect(), theme), active.clone()));
                    silhouettes.insert(active.clone(), silhouette);
                }
            }
            return (out, silhouettes);
        }
        collect_stack_bodies(&self.root, bounds, &empty_path(), theme, window_labels, atlas, self, self.tab_action_count(), &mut out, &mut silhouettes);
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
                collect_corner_tab_bars_for_stack(path, windows, bounds, theme, atlas, window_labels, self.tab_action_count(), &mut out);
            }
            return out;
        }
        collect_stack_corner_tab_bars(&self.root, bounds, &empty_path(), theme, atlas, window_labels, self.tab_action_count(), &mut out);
        out
    }

    pub fn tab_index(&self, path: &DockPath, window_id: &str) -> Option<usize> {
        let DockNode::Stack { windows, .. } = node_at(&self.root, path)? else {
            return None;
        };
        windows.iter().position(|tab| tab.window_id == window_id)
    }

}
//#endregion DockLayout
