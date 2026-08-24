// #region shell
//! 🪟️ Retained representation of dock/split/tab/window-cap chrome, built from the declarative
//! `WindowLayout` vocabulary (not `UiNode` — window-shell chrome isn't expressed as app-declarative
//! `UiNode`s). `Shell` owns its own `UiTree` so a later `engine` facade milestone can run the same
//! layout/paint/events passes over shell chrome that it runs over app content trees.
//! `set_window_layout` does a full teardown+rebuild each call rather than keyed diffing (window
//! layouts change far less often per-frame than widget content, so a full rebuild is a reasonable v1
//! — incremental shell-tree diffing is a documented gap for a later milestone). Drag-to-reorder/
//! drop-zone computation is stubbed this milestone (see `dispatch`'s doc comment): only hit-testing
//! plus click-to-activate-tab is fully implemented.

use crate::wgpu::IconName;
use crate::wgpu::Label;
use crate::wgpu::Locale;
use crate::wgpu::arena::NodeId;
use crate::wgpu::component::layout::{ActionDescriptor, WindowLayout, WindowLayoutAxisNode, WindowLayoutChild, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};
use crate::wgpu::component::role_chrome::{ChromeRole, role_title_chip_text};
use crate::wgpu::component::ui::{UiButtonNode, UiNode, UiPresence, UiStackNode};
use crate::wgpu::events::{UiEvent, hit_test};
use crate::wgpu::tree::{Node, NodeFlags, NodeKey, UiTree, WidgetSpec};

const SHELL_AXIS: u32 = 200;
const SHELL_STACK: u32 = 201;

/// 📤️ What `Shell::dispatch` surfaces to the host: chrome-level interactions that aren't app
/// `ActionDescriptor`s (those still flow through `events::UiCommand::App`).
#[derive(Clone, Debug, PartialEq)]
pub enum ShellEvent {
    /// 🫳️ A window-cap/tab-header press started a potential drag. `Shell::dispatch` never emits this
    /// yet — see its doc comment for what's stubbed vs. implemented this milestone.
    PanelDragStarted { pane_id: String },
    /// 📥️ A dragged pane was released over a drop zone. `Shell::dispatch` never emits this yet (no
    /// drop-target geometry is computed this milestone); kept in the enum so the host-facing API
    /// shape is settled ahead of the drag-and-drop implementation landing in a later milestone.
    PanelDropped { pane_id: String, target: String },
    /// 🖱️ A tab/window-cap was clicked (pointer down and up over the same window leaf).
    TabActivated { window_id: String },
}

/// 🪟️ Retained dock/split/tab/window-cap/navbar chrome, driven by declarative `WindowLayout`.
pub struct Shell {
    tree: UiTree,
    layout: Option<WindowLayout>,
    navbar: Vec<String>,
    pressed: Option<NodeId>,
    window_kind_icons: std::collections::HashMap<String, IconName>,
    /// 👁️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §5: `window_id ->
    /// ChromeRole` for every window whose session role is known — a window absent here paints with
    /// no role chrome (matches today's zero-role-aware-sessions reality; set via
    /// `set_window_role`/`set_window_roles` once the host resolves a session's `AppDefinition.role`).
    window_roles: std::collections::HashMap<String, ChromeRole>,
    /// 🌐️ The locale used to resolve role-chrome strings (title chip, "Open with…", "Set as
    /// default") — `Locale::En` first per this repo's own "no default language, English declared
    /// first" convention (`Locale`'s own `#[default]`); the host sets this explicitly once it knows
    /// the active locale, same boundary as `set_window_kind_icons`.
    locale: Locale,
}

impl Shell {
    /// 🌱️ An empty shell: no layout applied yet, no navbar items, no role chrome.
    pub fn new() -> Self {
        Self { tree: UiTree::new(), layout: None, navbar: Vec::new(), pressed: None, window_kind_icons: std::collections::HashMap::new(), window_roles: std::collections::HashMap::new(), locale: Locale::default() }
    }

    /// 🪟️ Maps window kind ids to Lucide icon ids for tab-cap painting in `set_window_layout`.
    pub fn set_window_kind_icons(&mut self, icons: std::collections::HashMap<String, IconName>) {
        self.window_kind_icons = icons;
    }

    /// 👁️✏️ Contract freeze §5: "a session is `(artifact_ref, AppRef)`; the role is read off the
    /// resolved `AppDefinition.role`, never inferred from the id string at runtime" — the host looks
    /// up the resolved role and tells the shell which window it governs. Applies immediately if a
    /// layout is already set (re-runs `set_window_layout`'s build with the current layout), so a late
    /// role resolution (e.g. the artifact finished opening after the tab already rendered) still
    /// repaints the chrome without a caller having to re-supply the whole layout.
    pub fn set_window_role(&mut self, window_id: impl Into<String>, role: ChromeRole) {
        self.window_roles.insert(window_id.into(), role);
        self.rebuild_if_layout_present();
    }

    /// 👁️✏️ The reverse of `set_window_role` — a window with no known role again, e.g. its
    /// artifact/app session closed.
    pub fn clear_window_role(&mut self, window_id: &str) {
        if self.window_roles.remove(window_id).is_some() {
            self.rebuild_if_layout_present();
        }
    }

    /// 👁️✏️ The role currently associated with `window_id`, if any.
    pub fn window_role(&self, window_id: &str) -> Option<ChromeRole> {
        self.window_roles.get(window_id).copied()
    }

    /// 🌐️ Sets the locale role-chrome strings resolve against; re-paints the current layout's chrome
    /// immediately, same as `set_window_role`.
    pub fn set_locale(&mut self, locale: Locale) {
        if self.locale != locale {
            self.locale = locale;
            self.rebuild_if_layout_present();
        }
    }

    fn rebuild_if_layout_present(&mut self) {
        if let Some(layout) = self.layout.clone() {
            self.set_window_layout(layout);
        }
    }

    /// 🔁️ Rebuilds the shell's retained tree from `layout` (full teardown+rebuild, see module doc).
    /// Axis nodes become row/column `Stack` containers; stack (tab-group) nodes become `Stack`
    /// containers marked `CLIPS_CHILDREN` (a tab group clips its content to its own bounds); each
    /// window leaf becomes a `Button`-shaped hit target keyed by its `instance_id` (falling back to
    /// `window_kind_id`) — a plain `Stack` can never itself be a hit target
    /// (`events::hit_test`'s bare-`Stack`-is-pass-through-only rule), so window caps deliberately use
    /// a non-`Stack` variant instead. A window whose id resolves a `ChromeRole` (`window_roles`) gets
    /// role chrome painted into its window-cap: the frozen title-chip text appended to its label
    /// (contract freeze §5) and, for `ChromeRole::Viewer`, a lock icon standing in for the read-only
    /// badge (this widget vocabulary has one icon slot per window cap, see `build_window`).
    pub fn set_window_layout(&mut self, layout: WindowLayout) {
        let mut tree = UiTree::new();
        let root_id = tree.insert_child(None, Node::new(NodeKey::Explicit("shell.root".into()), WidgetSpec(root_stack())));
        tree.mark_dirty(root_id, NodeFlags::DIRTY_LAYOUT);
        let ctx = ShellPaintContext { window_kind_icons: &self.window_kind_icons, window_roles: &self.window_roles, locale: self.locale };
        build_root(&mut tree, root_id, &layout.root, &ctx);
        self.tree = tree;
        self.layout = Some(layout);
        self.pressed = None;
    }

    /// 🧭️ Minimal stub: stores whatever navbar-relevant labels the host provides. A full navbar data
    /// model and pixel-perfect chrome painting are deferred to a later milestone — getting the tree
    /// integration point right matters more than the visual right now.
    pub fn set_navbar(&mut self, items: Vec<String>) {
        self.navbar = items;
    }

    /// 📖️ The declarative layout last applied via `set_window_layout`, if any.
    pub fn window_layout(&self) -> Option<&WindowLayout> {
        self.layout.as_ref()
    }

    /// 🌳️ Read access to the shell's retained tree, for a later `engine` facade to layout/paint/route.
    pub fn tree(&self) -> &UiTree {
        &self.tree
    }

    /// 🌳️ Mutable access to the shell's retained tree.
    pub fn tree_mut(&mut self) -> &mut UiTree {
        &mut self.tree
    }

    /// 🧭️ The stub navbar item labels currently set via `set_navbar`.
    pub fn navbar(&self) -> &[String] {
        &self.navbar
    }

    /// 🕹️ Hit-tests `event` against the shell's own retained tree and surfaces `ShellEvent`s. Fully
    /// implemented: `PointerDown` over a window-cap captures it; a matching `PointerUp` over the
    /// *same* window-cap emits `TabActivated`. Stubbed: no `PanelDragStarted`/`PanelDropped` are
    /// emitted yet — drag-to-reorder needs drop-zone geometry this milestone doesn't compute
    /// (documented gap, deferred to Phase 4's shell carve-over).
    pub fn dispatch(&mut self, event: &UiEvent) -> Vec<ShellEvent> {
        let mut out = Vec::new();
        let Some(root) = self.tree.root else { return out };
        match event {
            UiEvent::PointerDown { x, y, .. } => {
                self.pressed = hit_test(&self.tree, root, *x, *y);
            }
            UiEvent::PointerUp { x, y, .. } => {
                let released = hit_test(&self.tree, root, *x, *y);
                if let (Some(pressed_id), Some(released_id)) = (self.pressed.take(), released) {
                    if pressed_id == released_id {
                        if let Some(NodeKey::Explicit(window_id)) = self.tree.node(released_id).map(|node| node.key.clone()) {
                            out.push(ShellEvent::TabActivated { window_id });
                        }
                    }
                }
            }
            _ => {}
        }
        out
    }
}

impl Default for Shell {
    fn default() -> Self {
        Self::new()
    }
}

fn root_stack() -> UiNode {
    UiNode::Stack(UiStackNode { direction: "column".into(), gap: None, padding: None, id: Some("shell.root".into()), presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, children: Vec::new(), menu: None })
}

/// 👁️✏️ Everything `build_root`/`build_axis`/`build_stack`/`build_window` need to paint role-aware
/// window-cap chrome, bundled so the recursive builders take one reference instead of growing a new
/// parameter per role-chrome need (contract freeze §5) — mirrors the pre-existing
/// `window_kind_icons`-threading idiom this module already used, generalized to a small context
/// struct now that there are two independent per-window lookups plus a locale.
struct ShellPaintContext<'a> {
    window_kind_icons: &'a std::collections::HashMap<String, IconName>,
    window_roles: &'a std::collections::HashMap<String, ChromeRole>,
    locale: Locale,
}

fn build_root(tree: &mut UiTree, parent: NodeId, root: &WindowLayoutRoot, ctx: &ShellPaintContext<'_>) {
    match root {
        WindowLayoutRoot::Axis(axis) => build_axis(tree, parent, axis, 0, ctx),
        WindowLayoutRoot::Stack(stack) => build_stack(tree, parent, stack, 0, ctx),
    }
}

fn build_axis(tree: &mut UiTree, parent: NodeId, axis: &WindowLayoutAxisNode, ordinal: u32, ctx: &ShellPaintContext<'_>) {
    let spec = UiNode::Stack(UiStackNode { direction: axis.kind.clone(), gap: Some("none".into()), padding: None, id: None, presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, children: Vec::new(), menu: None });
    let id = tree.insert_child(Some(parent), Node::new(NodeKey::Positional(SHELL_AXIS, ordinal), WidgetSpec(spec)));
    tree.mark_dirty(id, NodeFlags::DIRTY_LAYOUT);
    for (index, child) in axis.children.iter().enumerate() {
        match child {
            WindowLayoutChild::Axis(nested) => build_axis(tree, id, nested, index as u32, ctx),
            WindowLayoutChild::Stack(nested) => build_stack(tree, id, nested, index as u32, ctx),
        }
    }
}

/// 🗂️ A tab group: a `Stack` container marked `CLIPS_CHILDREN` (its content clips to its own bounds)
/// whose children are the window-cap `Button` leaves built by `build_window`.
fn build_stack(tree: &mut UiTree, parent: NodeId, stack: &WindowLayoutStackNode, ordinal: u32, ctx: &ShellPaintContext<'_>) {
    let spec = UiNode::Stack(UiStackNode { direction: "column".into(), gap: None, padding: None, id: None, presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, children: Vec::new(), menu: None });
    let id = tree.insert_child(Some(parent), Node::new(NodeKey::Positional(SHELL_STACK, ordinal), WidgetSpec(spec)));
    if let Some(node) = tree.node_mut(id) {
        node.flags.set(NodeFlags::CLIPS_CHILDREN, true);
    }
    tree.mark_dirty(id, NodeFlags::DIRTY_LAYOUT);
    for (index, window) in stack.children.iter().enumerate() {
        build_window(tree, id, window, index as u32, ctx);
    }
}

/// 🪟️ One window-cap/tab-header hit target, keyed by `instance_id` (falling back to
/// `window_kind_id`). Modeled as a `Button` rather than a `Stack` specifically so `events::hit_test`
/// treats it as a matchable leaf, not a pass-through container (see `set_window_layout`'s doc
/// comment). 👁️✏️ Contract freeze §5: a window whose id resolves a `ChromeRole` in
/// `ctx.window_roles` gets the frozen title-chip text (`role_title_chip_text`) appended to its
/// label, and — for `ChromeRole::Viewer` only — its icon swaps to a lock, standing in for the
/// read-only badge (this leaf has exactly one icon slot, see `UiButtonNode`).
fn build_window(tree: &mut UiTree, parent: NodeId, window: &WindowLayoutWindowNode, ordinal: u32, ctx: &ShellPaintContext<'_>) {
    let _ = ordinal;
    let window_id = window.instance_id.clone().unwrap_or_else(|| window.window_kind_id.clone());
    let title = window.title.clone().unwrap_or_else(|| window.window_kind_id.clone());
    let role = ctx.window_roles.get(&window_id).copied();
    let label = match role {
        Some(role) => format!("{title} · {}", role_title_chip_text(role, ctx.locale == Locale::De)),
        None => title,
    };
    let icon_id = if role.is_some_and(ChromeRole::is_read_only) { IconName::Lock } else { ctx.window_kind_icons.get(&window.window_kind_id).copied().unwrap_or(IconName::AppWindow) };
    let spec = UiNode::Button(UiButtonNode {
        id: Some(window_id.clone()),
        icon_id,
        label: Label::data(label),
        action: ActionDescriptor { controller_id: "shell.window".into(), action: "activate".into(), args: None },
        style: None,
        presence: UiPresence::default(),
        menu: None,
    });
    let id = tree.insert_child(Some(parent), Node::new(NodeKey::Explicit(window_id), WidgetSpec(spec)));
    tree.mark_dirty(id, NodeFlags::DIRTY_LAYOUT);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wgpu::flex::LayoutEngine;
    use crate::wgpu::text::FontAtlas;
    use crate::wgpu::theme::Theme;

    fn single_window_layout(window_kind_id: &str) -> WindowLayout {
        crate::wgpu::even_window_layout(&[window_kind_id.to_string()])
    }

    fn run_layout(shell: &mut Shell) {
        let root = shell.tree().root.expect("set_window_layout must produce a root");
        let mut engine = LayoutEngine::new();
        let mut atlas = FontAtlas::builtin();
        let theme = Theme::default();
        engine.compute(shell.tree_mut(), root, &mut atlas, &theme, 400.0, 400.0);
    }

    fn count_nodes(tree: &UiTree, id: NodeId) -> usize {
        1 + tree.children(id).map(|child| count_nodes(tree, child)).sum::<usize>()
    }

    #[test]
    fn set_window_layout_with_one_window_produces_the_expected_retained_tree_shape() {
        let mut shell = Shell::new();
        shell.set_window_layout(single_window_layout("app.viewport"));

        let root = shell.tree().root.expect("expected a root node");
        // shell.root -> tab-group Stack -> one Button window leaf = 3 nodes.
        assert_eq!(count_nodes(shell.tree(), root), 3);
        let tab_group = shell.tree().children(root).next().expect("expected a tab-group child");
        assert!(shell.tree().node(tab_group).unwrap().flags.contains(NodeFlags::CLIPS_CHILDREN));
        let window_leaf = shell.tree().children(tab_group).next().expect("expected a window leaf");
        assert!(matches!(shell.tree().node(window_leaf).unwrap().spec.0, UiNode::Button(_)));
    }

    #[test]
    fn set_window_layout_called_twice_with_the_same_layout_is_idempotent_and_does_not_panic() {
        let mut shell = Shell::new();
        shell.set_window_layout(single_window_layout("app.viewport"));
        let first_count = count_nodes(shell.tree(), shell.tree().root.unwrap());

        shell.set_window_layout(single_window_layout("app.viewport"));
        let second_count = count_nodes(shell.tree(), shell.tree().root.unwrap());

        assert_eq!(first_count, second_count);
        assert_eq!(shell.window_layout(), Some(&single_window_layout("app.viewport")));
    }

    #[test]
    fn pointer_down_and_up_on_the_same_window_cap_activates_its_tab() {
        let mut shell = Shell::new();
        shell.set_window_layout(single_window_layout("app.viewport"));
        run_layout(&mut shell);

        let down = shell.dispatch(&UiEvent::PointerDown { x: 10.0, y: 10.0, button: crate::wgpu::events::PointerButton::Primary });
        assert!(down.is_empty(), "press alone must not activate a tab");

        let up = shell.dispatch(&UiEvent::PointerUp { x: 10.0, y: 10.0, button: crate::wgpu::events::PointerButton::Primary });
        assert_eq!(up, vec![ShellEvent::TabActivated { window_id: "app.viewport".into() }]);
    }

    #[test]
    fn pointer_down_then_up_outside_the_pressed_window_cap_does_not_activate_a_tab() {
        let mut shell = Shell::new();
        shell.set_window_layout(single_window_layout("app.viewport"));
        run_layout(&mut shell);

        shell.dispatch(&UiEvent::PointerDown { x: 10.0, y: 10.0, button: crate::wgpu::events::PointerButton::Primary });
        let up = shell.dispatch(&UiEvent::PointerUp { x: -50.0, y: -50.0, button: crate::wgpu::events::PointerButton::Primary });
        assert!(up.is_empty(), "releasing outside every hit target must not activate a tab");
    }

    fn only_window_button(shell: &Shell) -> UiButtonNode {
        let root = shell.tree().root.expect("expected a root node");
        let tab_group = shell.tree().children(root).next().expect("expected a tab-group child");
        let window_leaf = shell.tree().children(tab_group).next().expect("expected a window leaf");
        match &shell.tree().node(window_leaf).unwrap().spec.0 {
            UiNode::Button(button) => button.clone(),
            other => panic!("expected a Button window leaf, got {other:?}"),
        }
    }

    #[test]
    fn a_window_with_no_role_paints_no_role_chrome() {
        let mut shell = Shell::new();
        shell.set_window_layout(single_window_layout("app.viewport"));
        let button = only_window_button(&shell);
        assert_eq!(button.label.as_str(), "app.viewport");
        assert_eq!(button.icon_id, IconName::AppWindow);
    }

    #[test]
    fn set_window_role_viewer_appends_the_title_chip_and_swaps_to_the_lock_icon() {
        let mut shell = Shell::new();
        shell.set_window_role("app.viewport", ChromeRole::Viewer);
        shell.set_window_layout(single_window_layout("app.viewport"));
        let button = only_window_button(&shell);
        assert_eq!(button.label.as_str(), "app.viewport · Viewer");
        assert_eq!(button.icon_id, IconName::Lock, "the read-only badge stands in for the window-kind icon");
    }

    #[test]
    fn set_window_role_editor_appends_the_title_chip_but_keeps_the_window_kind_icon() {
        let mut shell = Shell::new();
        shell.set_window_kind_icons(std::collections::HashMap::from([("app.viewport".to_string(), IconName::Folder)]));
        shell.set_window_role("app.viewport", ChromeRole::Editor);
        shell.set_window_layout(single_window_layout("app.viewport"));
        let button = only_window_button(&shell);
        assert_eq!(button.label.as_str(), "app.viewport · Editor");
        assert_eq!(button.icon_id, IconName::Folder, "an editor session is not read-only, so its own window-kind icon survives");
    }

    #[test]
    fn set_window_role_after_the_layout_is_already_set_repaints_immediately() {
        let mut shell = Shell::new();
        shell.set_window_layout(single_window_layout("app.viewport"));
        assert_eq!(only_window_button(&shell).label.as_str(), "app.viewport", "no role yet");

        shell.set_window_role("app.viewport", ChromeRole::Viewer);
        assert_eq!(only_window_button(&shell).label.as_str(), "app.viewport · Viewer", "role chrome appears without a caller re-supplying the layout");

        shell.clear_window_role("app.viewport");
        assert_eq!(only_window_button(&shell).label.as_str(), "app.viewport", "clearing the role reverts the chrome");
    }

    #[test]
    fn set_locale_de_resolves_the_german_title_chip() {
        let mut shell = Shell::new();
        shell.set_window_role("app.viewport", ChromeRole::Viewer);
        shell.set_locale(Locale::De);
        shell.set_window_layout(single_window_layout("app.viewport"));
        assert_eq!(only_window_button(&shell).label.as_str(), "app.viewport · Betrachter");
    }
}
// #endregion shell
