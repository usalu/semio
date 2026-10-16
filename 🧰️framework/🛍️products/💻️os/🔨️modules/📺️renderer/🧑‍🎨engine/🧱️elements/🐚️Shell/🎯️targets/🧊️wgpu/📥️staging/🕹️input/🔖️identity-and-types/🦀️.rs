//! 🕹️ `shell-input` staging of `identity-and-types`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

pub fn decode_window_template_drag(data: &HashMap<String, String>) -> Option<WindowTemplateDragPayload> {
    serde_json::from_str(data.get(COMPOSE_WINDOW_TEMPLATE_MIME)?).ok()
}

//#region ShellTypes
/** 🖱️ The `surface` half of one right-click's context-menu request, as
 * [`ShellState::resolve_context_menu_surface`] resolved it: which surface owns the click, the
 * camelCase menu/kind vocabulary id React sends for it, and that kind's own per-row hits, selection
 * groups and (text editor only) text context. */
#[derive(Clone, Debug, Default)]
pub struct ContextMenuSurfaceResolution {
    pub surface_id: String,
    pub kind: String,
    pub hits: Vec<ui_wgpu::wgpu::ContextMenuHit>,
    pub selection: Vec<ui_wgpu::wgpu::ContextMenuSelectionGroup>,
    pub text: Option<ui_wgpu::wgpu::ContextMenuTextContext>,
}

#[derive(Clone, Debug, Default)]
pub struct ContextMenuItem {
    pub id: String,
    pub label: String,
    pub icon: Option<String>,
    pub shortcut: Option<String>,
    pub destructive: bool,
    pub action: Option<ActionDescriptor>,
    pub children: Vec<ContextMenuItem>,
    pub disabled: bool,
    /// 💬️ Why a disabled row cannot run (`ContextMenuItemSpec.reason`, producer-localized): painted muted beside the label
    /// and announced as the focusable disabled row's description.
    pub reason: Option<String>,
    pub separator: bool,
    pub checked: bool,
}

impl ContextMenuItem {
    /// 🏷️ The row's painted text: its label, and beside it the reason a disabled row cannot run.
    pub fn painted_label(&self) -> String {
        match self.reason.as_deref().filter(|_| self.disabled) {
            Some(reason) => format!("{} \u{b7} {reason}", self.label),
            None => self.label.clone(),
        }
    }

    /// ♿️ Registers the row's hit and its accessible presentation: an enabled row fires its action; a disabled row is still
    /// a focusable `menuitem` (`aria-disabled`, described by its reason) that fires nothing — the `RowAction` contract.
    pub fn register_hit(&self, row: Rect, input: &mut InputState<ActionDescriptor>) {
        input.register_hit(HitTarget { rect: row, event: self.action.clone().filter(|_| !self.disabled), control_id: Some(self.id.clone()), kind: HitKind::ContextMenu, drag_axis: None, drag_data: None });
        note_chrome_control_name(&self.id, Some(self.label.as_str()));
        note_chrome_control_disabled(&self.id, self.disabled);
        note_chrome_control_description(&self.id, self.reason.as_deref().filter(|_| self.disabled));
    }
}

#[derive(Clone, Debug, Default)]
pub struct ContextMenuState {
    pub x: f32,
    pub y: f32,
    pub items: Vec<ContextMenuItem>,
    pub active: Vec<usize>,
    pub submenu_collapsed_at: Option<Vec<usize>>,
    /// 📜️ Vertical scroll offset for whichever rendered level currently exceeds the viewport height —
    /// see `render_context_menu_level`'s clip/scroll handling and `ShellState::handle_pointer_wheel`.
    pub scroll_offset: f32,
}

/** ⌨️ Result of routing a key while the shell context menu is open. */
pub enum ContextMenuKeyOutcome {
    Ignored,
    Consumed,
    Activate(ActionDescriptor),
    CloseMenu,
}

/// ⌨️ The rows arrow keys reach: every row but a separator — a disabled row is focusable, so its reason is heard.
fn context_menu_focusable_indices(items: &[ContextMenuItem]) -> Vec<usize> {
    items.iter().enumerate().filter(|(_, item)| !item.separator).map(|(index, _)| index).collect()
}

fn context_menu_items_at_level<'a>(root: &'a [ContextMenuItem], path_prefix: &[usize]) -> &'a [ContextMenuItem] {
    let mut level = root;
    for &index in path_prefix {
        let Some(row) = level.get(index) else {
            return level;
        };
        if row.children.is_empty() {
            return level;
        }
        level = &row.children;
    }
    level
}

pub fn context_menu_item_at_path<'a>(root: &'a [ContextMenuItem], path: &[usize]) -> Option<&'a ContextMenuItem> {
    if path.is_empty() {
        return None;
    }
    let mut level = root;
    let mut item = None;
    for (depth, &index) in path.iter().enumerate() {
        item = level.get(index);
        let row = item?;
        if depth + 1 < path.len() {
            level = &row.children;
        }
    }
    item
}

pub fn context_menu_path_for_item_id(root: &[ContextMenuItem], item_id: &str, prefix: &mut Vec<usize>) -> Option<Vec<usize>> {
    for (index, item) in root.iter().enumerate() {
        if item.separator {
            continue;
        }
        prefix.push(index);
        if item.id == item_id {
            return Some(prefix.clone());
        }
        if !item.children.is_empty() {
            if let Some(path) = context_menu_path_for_item_id(&item.children, item_id, prefix) {
                return Some(path);
            }
        }
        prefix.pop();
    }
    None
}

pub fn context_menu_move_active(root: &[ContextMenuItem], path: &[usize], down: bool) -> Vec<usize> {
    let level_prefix = if path.is_empty() { &[][..] } else { &path[..path.len() - 1] };
    let level = context_menu_items_at_level(root, level_prefix);
    let enabled = context_menu_focusable_indices(level);
    if enabled.is_empty() {
        return path.to_vec();
    }
    let current = path.last().copied().unwrap_or(usize::MAX);
    let position = enabled.iter().position(|index| *index == current);
    let next_position = match position {
        None => {
            if down {
                0
            } else {
                enabled.len() - 1
            }
        }
        Some(pos) => {
            if down {
                (pos + 1) % enabled.len()
            } else {
                (pos + enabled.len() - 1) % enabled.len()
            }
        }
    };
    let mut next = level_prefix.to_vec();
    next.push(enabled[next_position]);
    next
}

pub fn context_menu_path_for_ordinal(root: &[ContextMenuItem], path: &[usize], ordinal: usize) -> Option<Vec<usize>> {
    let level_prefix = if path.is_empty() { &[][..] } else { &path[..path.len() - 1] };
    let level = context_menu_items_at_level(root, level_prefix);
    let mut seen = 0usize;
    for (index, item) in level.iter().enumerate() {
        if item.separator || item.disabled {
            continue;
        }
        seen += 1;
        if seen == ordinal {
            let mut next = level_prefix.to_vec();
            next.push(index);
            return Some(next);
        }
    }
    None
}

pub fn context_menu_open_submenu_path(root: &[ContextMenuItem], path: &[usize]) -> Option<Vec<usize>> {
    let item = context_menu_item_at_path(root, path)?;
    if item.children.is_empty() {
        return None;
    }
    let enabled = context_menu_focusable_indices(&item.children);
    if enabled.is_empty() {
        return Some(path.to_vec());
    }
    let mut next = path.to_vec();
    next.push(enabled[0]);
    Some(next)
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum OverlayState {
    #[default]
    None,
    Search,
    Find,
    Dropdown(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShellPaletteKind {
    Search,
    Find,
}

impl ShellPaletteKind {
    pub fn input_id(self) -> &'static str {
        match self {
            Self::Search => "ui.search.input",
            Self::Find => "ui.find.input",
        }
    }

    pub fn dialog_id(self) -> &'static str {
        match self {
            Self::Search => "ui.search.dialog",
            Self::Find => "ui.find.dialog",
        }
    }

    pub fn list_id(self) -> &'static str {
        match self {
            Self::Search => "ui.search.list",
            Self::Find => "ui.find.list",
        }
    }

    pub fn close_id(self) -> &'static str {
        match self {
            Self::Search => "ui.search.close",
            Self::Find => "ui.find.close",
        }
    }

    pub fn control_id(self, index: usize) -> String {
        match self {
            Self::Search => format!("ui.search.item.{index}"),
            Self::Find => format!("ui.find.item.{index}"),
        }
    }

    pub fn from_input_id(id: &str) -> Option<Self> {
        match id {
            "ui.search.input" => Some(Self::Search),
            "ui.find.input" => Some(Self::Find),
            _ => None,
        }
    }

    fn from_dialog_id(id: &str) -> Option<Self> {
        match id {
            "ui.search.dialog" => Some(Self::Search),
            "ui.find.dialog" => Some(Self::Find),
            _ => None,
        }
    }

    pub fn from_close_id(id: &str) -> Option<Self> {
        match id {
            "ui.search.close" => Some(Self::Search),
            "ui.find.close" => Some(Self::Find),
            _ => None,
        }
    }

    pub fn owns_control_id(self, id: &str) -> bool {
        id == self.dialog_id() || id == self.input_id() || id == self.list_id() || id == self.close_id() || Self::row_index(id).is_some_and(|(kind, _)| kind == self)
    }

    pub fn row_index(id: &str) -> Option<(Self, usize)> {
        [(Self::Search, "ui.search.item."), (Self::Find, "ui.find.item.")].into_iter().find_map(|(kind, prefix)| id.strip_prefix(prefix).and_then(|index| index.parse().ok()).map(|index| (kind, index)))
    }
}

#[derive(Clone, Debug, Default)]
pub struct RightClickState {
    pub pending: bool,
    pub x: f32,
    pub y: f32,
}

//#endregion ShellTypes

