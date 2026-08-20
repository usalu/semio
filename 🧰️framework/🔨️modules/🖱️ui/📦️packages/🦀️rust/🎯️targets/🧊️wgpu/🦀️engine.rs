// #region engine
//! 🧵️ The retained-mode `Ui` façade: the missing keystone tying `arena`/`tree`/`reconcile`/`flex`/
//! `paint`/`events`/`scene_slots`/`shell` into one usable pipeline — each of those regions was built
//! and individually tested to its own milestone but nothing ever assembled them together, and nothing
//! in `framework/renderer/wgpu` calls into any of it (see `.🦑️repo/🎫️tickets/26/07/11/RETAINED-MODE-UI-CRATE`'s
//! plan for the historical intent). This module is purely additive: the immediate-mode `widgets`
//! path stays the only pipeline actually driving pixels until a later workstream proves this façade
//! out (via the golden `tests` module below) and cuts over.

use std::collections::HashMap;

use crate::wgpu::component::layout::WindowLayout;
use crate::wgpu::component::ui::UiNode;
use crate::wgpu::draw::{DrawList, IconAtlas};
use crate::wgpu::events::{EventRouter, UiCommand, UiEvent};
use crate::wgpu::flex::LayoutEngine;
use crate::wgpu::paint::paint_tree;
use crate::wgpu::scene_slots::{collect_scene_slots, SceneHost};
use crate::wgpu::shell::{Shell, ShellEvent};
use crate::wgpu::text::FontAtlas;
use crate::wgpu::theme::Theme;
use crate::wgpu::tree::{NodeFlags, UiTree};
use crate::wgpu::IconName;

//#region 🔖️UiWindow
/// 🪟️ One window's retained pipeline state: its `UiTree` (`reconcile`'s diff target), the taffy
/// `LayoutEngine` that lays it out (`flex`), the `EventRouter` owning its capture/focus/hover state
/// (`events`), and the `DrawList` `paint::paint_tree` last painted into. Mirrors `tree`'s own doc
/// comment ("the engine facade... holds `HashMap<window_id, UiTree>`") by keying the *whole*
/// per-window pipeline the same way, not just the tree.
struct UiWindow {
    tree: UiTree,
    layout: LayoutEngine,
    router: EventRouter,
    draw: DrawList,
    viewport: (f32, f32),
}

impl UiWindow {
    fn new(window_id: &str) -> Self {
        Self { tree: UiTree::new(), layout: LayoutEngine::new(), router: EventRouter::new(window_id), draw: DrawList::default(), viewport: (0.0, 0.0) }
    }

    /// 🚨️ Whether this window's root (and thus, transitively, anything below it per
    /// `UiTree::mark_dirty`'s bubbling) still needs a layout or paint pass.
    fn is_dirty(&self) -> bool {
        self.tree.root.and_then(|root| self.tree.node(root)).is_some_and(|node| node.flags.contains(NodeFlags::DIRTY_LAYOUT) || node.flags.contains(NodeFlags::DIRTY_PAINT) || node.flags.contains(NodeFlags::SUBTREE_DIRTY))
    }
}
//#endregion 🔖️UiWindow

//#region 🔖️Ui
/// 🧵️ Assembles the individually-milestoned retained modules into the one façade a host drives per
/// tick: `apply_tree` runs `reconcile`, `frame` runs `flex` (dirty-gated) then `paint` then hands
/// `scene_slots` to an optional `SceneHost`, `dispatch_event` runs `events::EventRouter`, and
/// `needs_frame` reads the same dirty flags `frame` itself gates on. One `UiWindow` per window id
/// (app-content trees); window-chrome (dock/split/tab) is the separate `Shell` this façade also owns,
/// since `shell`'s own doc comment models it as independent of any single window's content tree.
/// Never submits to the GPU itself — `frame` returns a `&DrawList` for the caller to hand to the
/// existing `gpu::GpuContext::render_frame`, exactly like the immediate-mode `widgets` path's callers
/// already do; wiring that hand-off into a real host event loop is later, renderer-thinning work.
pub struct Ui {
    windows: HashMap<String, UiWindow>,
    shell: Shell,
    theme: Theme,
    pending_commands: Vec<UiCommand>,
}

impl Ui {
    pub fn new() -> Self {
        Self { windows: HashMap::new(), shell: Shell::new(), theme: Theme::default(), pending_commands: Vec::new() }
    }

    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
    }

    fn window_mut(&mut self, window_id: &str) -> &mut UiWindow {
        self.windows.entry(window_id.to_string()).or_insert_with(|| UiWindow::new(window_id))
    }

    /// 📐️ Stores the viewport a later `frame` call lays out against for `window_id`, creating that
    /// window's retained state on first use.
    pub fn set_viewport(&mut self, window_id: &str, width: f32, height: f32) {
        self.window_mut(window_id).viewport = (width, height);
    }

    /// 🔁️ Runs `UiTree::apply_tree` (`reconcile`) to diff `ui_node` into `window_id`'s retained tree,
    /// creating that window's tree/layout-engine/event-router on first use.
    pub fn apply_tree(&mut self, window_id: &str, ui_node: &UiNode) {
        self.window_mut(window_id).tree.apply_tree(ui_node);
    }

    pub fn set_window_kind_icons(&mut self, icons: HashMap<String, IconName>) {
        self.shell.set_window_kind_icons(icons);
    }

    /// 🪟️ Rebuilds the shared `Shell`'s retained dock/split/tab chrome from a declarative
    /// `WindowLayout` (independent of any window's `apply_tree`d content — see `shell`'s doc comment).
    pub fn set_window_layout(&mut self, layout: WindowLayout) {
        self.shell.set_window_layout(layout);
    }

    /// 🧭️ Forwards to `Shell::set_navbar` (stub — see that method's doc comment).
    pub fn set_navbar(&mut self, items: Vec<String>) {
        self.shell.set_navbar(items);
    }

    pub fn shell(&self) -> &Shell {
        &self.shell
    }

    /// 🚦️ True when any window's retained tree still carries `DIRTY_LAYOUT`/`DIRTY_PAINT`/
    /// `SUBTREE_DIRTY` on its root. No animation-clock scaffolding exists anywhere in this crate yet
    /// (nothing under `arena`/`tree`/`reconcile`/`flex`/`paint`/`events`/`scene_slots`/`shell`
    /// schedules a future wake), so this is purely dirty-flag-driven; wiring a real animation deadline
    /// is separate follow-up work, not this façade's job to invent.
    pub fn needs_frame(&self) -> bool {
        self.windows.values().any(UiWindow::is_dirty)
    }

    /// 🖼️ The dirty-gated per-tick pipeline for `window_id`: `flex::LayoutEngine::compute` (itself a
    /// no-operation unless the root carries `DIRTY_LAYOUT`/`SUBTREE_DIRTY`) followed — only if that or the
    /// root's own `DIRTY_PAINT` fired — by `paint::paint_tree`, then handing every
    /// `scene_slots::collect_scene_slots` leaf to `scene_host`, when the caller passed one this tick.
    /// Returns `None` if `window_id` has no tree yet (`apply_tree` never called). A dirty window
    /// always repaints its whole tree — `paint::paint_tree`'s own doc comment: `DrawList` only
    /// supports a full clear-and-rebuild, no incremental dirty-subtree replacement yet.
    ///
    /// 🖋️ `atlas`/`icons` are the CALLER's own `FontAtlas`/`IconAtlas` — `Ui` never owns either (see
    /// this region's top-of-file doc comment): the host must pass the SAME instances it already
    /// `GpuContext::upload_font_atlas`/`upload_icon_atlas`s every frame, exactly like `flex::LayoutEngine::
    /// compute`/`paint::paint_tree` already receive them as parameters rather than fields. This lets
    /// retained-mode content share glyph/icon UVs with the rest of the host's chrome instead of
    /// clobbering (or never populating) a second, independent GPU texture.
    ///
    /// 🎬️ `scene_host` is a PER-FRAME parameter, not a stored field (there used to be a stored
    /// `Option<Box<dyn SceneHost>>` — removed): a caller-owned host typically needs to borrow the
    /// same per-frame state this call site already has in scope (a `GpuContext`, per-surface state
    /// maps, …), which a `Box<dyn SceneHost>` stored on `Ui` itself could never hold, exactly like
    /// `atlas`/`icons` above are parameters rather than fields for the same reason. `paint_tree`
    /// already knows (via `scene_host.is_some()`) whether to paint its own placeholder chrome for
    /// `ComponentScene`/`Image` leaves this tick or leave that rect for the host to fill in below —
    /// see `paint`'s own doc comment on that gate.
    // 🧬️ A former `Option<&mut dyn SceneHost>` — `SceneHost` is a genuine OPEN extension point (its own
    // doc comment: "the only place vello/world3d/raster-decode-specific code may live"; two real
    // implementors already exist, `RecordingSceneHost` here in tests and `FrameworkSceneHost` in
    // `os/renderer/engine/Interpreter`, which is outside this crate). Per R11 this is the trivially-
    // generic argument-position case (R11(a)): each call site already hands `frame` ONE concrete host
    // reference, so `H: SceneHost` loses no expressiveness versus `dyn` and every existing caller's
    // call syntax (`Some(&mut concrete_host)`) is unchanged — `H` is inferred from the argument.
    pub fn frame<H: SceneHost>(&mut self, window_id: &str, viewport_width: f32, viewport_height: f32, atlas: &mut FontAtlas, icons: Option<&IconAtlas>, mut scene_host: Option<&mut H>) -> Option<&DrawList> {
        let window = self.windows.get_mut(window_id)?;
        let root = window.tree.root?;
        window.viewport = (viewport_width, viewport_height);
        let dirty = window.tree.node(root).is_some_and(|node| node.flags.contains(NodeFlags::DIRTY_LAYOUT) || node.flags.contains(NodeFlags::DIRTY_PAINT) || node.flags.contains(NodeFlags::SUBTREE_DIRTY));
        if !dirty {
            return Some(&window.draw);
        }
        window.layout.compute(&mut window.tree, root, atlas, &self.theme, viewport_width, viewport_height);
        window.draw.clear();
        paint_tree(&mut window.tree, root, &self.theme, atlas, icons, scene_host.is_some(), &mut window.draw);
        if let Some(host) = scene_host.as_deref_mut() {
            for slot in collect_scene_slots(&window.tree, root) {
                host.paint_slot(&slot, &mut window.draw, atlas, icons);
            }
        }
        Some(&window.draw)
    }

    /// 📤️ Direct access to `window_id`'s last-painted `DrawList` without re-running the pipeline.
    pub fn draw_list(&self, window_id: &str) -> Option<&DrawList> {
        self.windows.get(window_id).map(|window| &window.draw)
    }

    /// 🕹️ Routes `event` through `window_id`'s `events::EventRouter` (hit-test, capture, focus, hover
    /// updates), returning the `UiCommand`s it produced and also queuing them for a later
    /// `drain_commands` call — callers may use either.
    #[allow(clippy::needless_pass_by_value, reason = "changing to &UiEvent is a breaking public API change across ~30 downstream plugins, out of T1 scope")]
    pub fn dispatch_event(&mut self, window_id: &str, event: UiEvent) -> Vec<UiCommand> {
        let Some(window) = self.windows.get_mut(window_id) else { return Vec::new() };
        let Some(root) = window.tree.root else { return Vec::new() };
        let commands = window.router.dispatch(&mut window.tree, root, &event);
        self.pending_commands.extend(commands.iter().cloned());
        commands
    }

    /// 🪟️ Routes `event` through the shared `Shell`'s own hit-testing, surfacing chrome-level
    /// `ShellEvent`s (tab activation today; drag/drop is `Shell::dispatch`'s own documented gap).
    pub fn dispatch_shell_event(&mut self, event: &UiEvent) -> Vec<ShellEvent> {
        self.shell.dispatch(event)
    }

    /// 📥️ Drains every `UiCommand` queued by `dispatch_event` calls since the last drain.
    pub fn drain_commands(&mut self) -> Vec<UiCommand> {
        std::mem::take(&mut self.pending_commands)
    }
}

impl Default for Ui {
    fn default() -> Self {
        Self::new()
    }
}
//#endregion 🔖️Ui

//#region 🔬️Introspection
/// 🔬️ Read-only accessors for the wgpu↔React parity structural-dump harness (see
/// `.🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY` and `framework/renderer/wgpu`'s own
/// `🔬️Introspection` region, which is the actual JSON-building caller): exposes just enough of
/// `Ui`'s private `windows`/`theme` state for a caller to walk a window's retained `UiTree` (via
/// `UiTree::node`/`UiTree::children`, both already public) and know which theme it last painted
/// with. Purely additive and read-only — no new engine behavior, nothing here is called from
/// `apply_tree`/`frame`/`dispatch_event`'s own pipeline.
impl Ui {
    /// 🪟️ Every window id this façade currently tracks retained state for (`HashMap` iteration
    /// order — not insertion order; a caller needing a deterministic pick must sort/filter itself).
    pub fn window_ids(&self) -> impl Iterator<Item = &str> {
        self.windows.keys().map(String::as_str)
    }

    /// 📐️ `window_id`'s last `set_viewport`/`frame` viewport, if that window has any retained state.
    pub fn viewport(&self, window_id: &str) -> Option<(f32, f32)> {
        self.windows.get(window_id).map(|window| window.viewport)
    }

    /// 🌲️ Read-only access to `window_id`'s retained tree (root + `Node` arena) for a caller to walk.
    pub fn tree(&self, window_id: &str) -> Option<&UiTree> {
        self.windows.get(window_id).map(|window| &window.tree)
    }

    /// 🎨️ The theme this façade last painted every window with (`Theme` is `Copy`).
    pub fn theme(&self) -> Theme {
        self.theme
    }

    /// 🎯️ Whether `window_id`'s retained content currently has a focused node — `false` if that
    /// window has no retained state at all. Lets a host (`w2-input-wiring`,
    /// `.🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY/report-w2-input-wiring.md`) decide whether real
    /// keyboard/IME events belong to this window's content (route via `dispatch_event`) or should
    /// fall back to chrome-level shortcuts. Forwards to `EventRouter::is_focused`, itself added this
    /// same pass — both purely additive reads, no change to `dispatch_event`'s own focus logic.
    pub fn window_has_focus(&self, window_id: &str) -> bool {
        self.windows.get(window_id).is_some_and(|window| window.router.is_focused())
    }
}
//#endregion 🔬️Introspection

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wgpu::component::layout::ActionDescriptor;
    use crate::wgpu::component::ui::{
        ui_node_to_control, SurfaceKind, UiButtonNode, UiComponentSceneNode, UiControlNode, UiExternalSlotNode, UiFieldNode, UiGroupNode, UiIconSelectNode, UiImageNode, UiInputNode, UiKeyValueEntry, UiKeyValueNode, UiNumberStepperNode,
        UiPresence, UiRingNode, UiSectionNode, UiSelectItem, UiSelectNode, UiSeparatorNode, UiSliderNode, UiStackNode, UiState, UiTextNode, UiToggleNode, UiTreeActionPlacement, UiTreeItemAction, UiTreeItemNode, UiTreeNode, UiTreeSectionNode,
    };
    use crate::wgpu::Label;
    use crate::wgpu::events::PointerButton;
    use crate::wgpu::geometry::Rect;
    use crate::wgpu::input::InputState;
    use crate::wgpu::scene_slots::SceneSlot;
    use crate::wgpu::widgets::{
        draw_text_on, draw_text_overlay_on, measure_widget, render_scroll_region, render_widget, wrap_text, ControlNode, InputMeta, KeyValueEntry, RingMeta, SelectItem, SliderMeta, StepperMeta, TreeItem, TreeItemAction, TreeSection,
        WidgetContext, WidgetInteractionMaps, WidgetNode,
    };
    use std::collections::HashMap as StdHashMap;

    //#region 🔖️FacadeTests
    fn stack_ui(children: Vec<UiNode>) -> UiNode {
        UiNode::Stack(UiStackNode { direction: "vertical".into(), gap: None, padding: None, id: Some("root".into()), presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, children, menu: None })
    }

    fn action() -> ActionDescriptor {
        ActionDescriptor { controller_id: "ctrl".into(), action: "go".into(), args: None }
    }

    fn button_ui(id: &str, label: &str) -> UiNode {
        UiNode::Button(UiButtonNode { id: Some(id.into()), icon_id: IconName::CircleDot, label: Label::data(label), action: action(), style: None, presence: UiPresence::default(), menu: None })
    }

    #[test]
    fn apply_tree_then_frame_produces_a_non_empty_draw_list() {
        let mut ui = Ui::new();
        let mut atlas = FontAtlas::builtin();
        ui.apply_tree("main", &stack_ui(vec![UiNode::Text(UiTextNode { value: Label::data("hi"), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None })]));

        assert!(ui.needs_frame(), "a freshly applied tree must report needing a frame");
        let draw = ui.frame::<RecordingSceneHost>("main", 400.0, 400.0, &mut atlas, None, None).expect("frame must produce a draw list once a tree was applied");
        let total: usize = draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();
        assert!(total > 0, "expected the text node to emit at least one glyph instance");
    }

    #[test]
    fn frame_before_any_apply_tree_returns_none() {
        let mut ui = Ui::new();
        let mut atlas = FontAtlas::builtin();
        assert!(ui.frame::<RecordingSceneHost>("nonexistent", 400.0, 400.0, &mut atlas, None, None).is_none());
    }

    #[test]
    fn needs_frame_is_false_once_a_stable_tree_has_been_framed() {
        let mut ui = Ui::new();
        let mut atlas = FontAtlas::builtin();
        let ui_node = stack_ui(vec![UiNode::Text(UiTextNode { value: Label::data("hi"), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None })]);
        ui.apply_tree("main", &ui_node);
        ui.frame::<RecordingSceneHost>("main", 400.0, 400.0, &mut atlas, None, None);
        assert!(!ui.needs_frame(), "nothing changed since the last frame, so no frame should be needed");

        ui.apply_tree("main", &ui_node);
        assert!(!ui.needs_frame(), "re-applying an identical tree must set zero dirty flags (reconcile's own golden rule)");
    }

    #[test]
    fn dispatch_event_emits_a_button_click_command_and_it_is_also_drainable() {
        let mut ui = Ui::new();
        let mut atlas = FontAtlas::builtin();
        ui.apply_tree("main", &stack_ui(vec![button_ui("go", "Go")]));
        ui.frame::<RecordingSceneHost>("main", 400.0, 400.0, &mut atlas, None, None);

        ui.dispatch_event("main", UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary });
        let commands = ui.dispatch_event("main", UiEvent::PointerUp { x: 10.0, y: 10.0, button: PointerButton::Primary });

        assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::App { action: fired_action, .. } if *fired_action == action())));
        let drained = ui.drain_commands();
        assert!(!drained.is_empty(), "commands dispatched should also be queryable via drain_commands");
        assert!(ui.drain_commands().is_empty(), "a second drain with nothing new dispatched must be empty");
    }

    #[test]
    fn set_window_layout_wires_into_the_facades_shell() {
        let mut ui = Ui::new();
        ui.set_window_layout(crate::wgpu::even_window_layout(&["app.viewport".to_string()]));
        assert!(ui.shell().window_layout().is_some());
    }
    //#endregion 🔖️FacadeTests

    //#region 🔖️SceneHostTests
    fn component_scene_ui(surface_id: &str) -> UiNode {
        UiNode::ComponentScene(UiComponentSceneNode {
            surface_id: surface_id.into(),
            controller_id: "ctrl".into(),
            component_kind: SurfaceKind::World3d,
            pane_id: None,
            binding_id: None,
            presence: UiPresence::default(),
            canvas_2d: None,
            world_3d: None,
            node_graph: None,
            text_editor: None,
            table: None,
            paint_2d: None,
            virtual_file_system: None,
            tiled_map: None,
            board2d: None,
            icon_render: None,
            ink_canvas: None,
            graph_timeline: None,
            block_list: None,
            diff_view: None,
            event_feed: None,
            menu: None,
        })
    }

    /// 🎬️ A bare-bones `SceneHost` recording every call it receives, so tests can assert `Ui::frame`
    /// actually reaches the host (once per slot, with the right payload) instead of just trusting the
    /// wiring compiles. Paints a single filled rect per slot so a hosted frame's `DrawList` is
    /// distinguishable from an unpainted one.
    struct RecordingSceneHost {
        paint_calls: usize,
        last_surface_id: Option<String>,
    }

    impl SceneHost for RecordingSceneHost {
        fn paint_slot(&mut self, slot: &SceneSlot<'_>, draw: &mut DrawList, _atlas: &mut FontAtlas, _icons: Option<&IconAtlas>) {
            self.paint_calls += 1;
            self.last_surface_id = slot.surface().map(|(surface_id, _)| surface_id.to_string());
            draw.push_rounded([slot.rect.x, slot.rect.y, slot.rect.w, slot.rect.h], Theme::default().accent, 0.0);
        }
    }

    #[test]
    fn frame_with_no_scene_host_falls_back_to_the_placeholder_chrome() {
        let mut ui = Ui::new();
        let mut atlas = FontAtlas::builtin();
        ui.apply_tree("w", &stack_ui(vec![component_scene_ui("surface.no-host")]));
        let draw = ui.frame::<RecordingSceneHost>("w", 400.0, 400.0, &mut atlas, None, None).expect("frame must produce a draw list");
        let instances: usize = draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();
        assert!(instances > 0, "with no scene host registered, paint_component_scene's placeholder chrome should still paint");
    }

    #[test]
    fn frame_with_a_scene_host_routes_the_component_scene_leaf_through_it() {
        let mut ui = Ui::new();
        let mut atlas = FontAtlas::builtin();
        ui.apply_tree("w", &stack_ui(vec![component_scene_ui("surface.host-test")]));

        let mut host = RecordingSceneHost { paint_calls: 0, last_surface_id: None };
        let draw = ui.frame("w", 400.0, 400.0, &mut atlas, None, Some(&mut host)).expect("frame must produce a draw list even with a scene host registered");
        let instances: usize = draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();

        assert_eq!(host.paint_calls, 1, "the host should be invoked exactly once for the single ComponentScene leaf");
        assert_eq!(host.last_surface_id.as_deref(), Some("surface.host-test"));
        assert!(instances > 0, "the host's own draw call should still land in the frame's DrawList");
    }

    #[test]
    fn frame_with_a_scene_host_still_paints_ancestor_chrome_around_the_hosted_slot() {
        // 🌳️ Nests the ComponentScene under a Group (not just a bare Stack) — regression for the
        // shadow-walk gap this bridge replaces: `collect_scene_slots` must still find it, and the
        // Group's own header/frame chrome (unrelated to the scene leaf) must still paint normally.
        let mut ui = Ui::new();
        let mut atlas = FontAtlas::builtin();
        let group_node = UiNode::Group(UiGroupNode {
            id: "group".into(),
            label: Label::data("Group"),
            default_open: None,
            presence: UiPresence::default(),
            children: vec![UiNode::Text(UiTextNode { value: Label::data("label"), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None }), component_scene_ui("surface.nested")],
            menu: None,
        });
        ui.apply_tree("w", &group_node);

        let mut host = RecordingSceneHost { paint_calls: 0, last_surface_id: None };
        ui.frame("w", 400.0, 400.0, &mut atlas, None, Some(&mut host)).expect("frame must produce a draw list");

        assert_eq!(host.paint_calls, 1);
        assert_eq!(host.last_surface_id.as_deref(), Some("surface.nested"));
    }
    //#endregion 🔖️SceneHostTests

    //#region 🔖️GoldenHarness
    /// 🏆️ Acceptance gate for this workstream: for a curated fixture of every `UiNode` variant, runs
    /// the retained façade (`apply_tree` + `frame`) and the immediate-mode path (`render_widget` over
    /// a hand-converted `WidgetNode`) and asserts they emit structurally equivalent `DrawList`s
    /// (same instance/vector/raster counts — not bit-identical geometry, per this ticket's brief).
    /// `to_widget_node`/`control_to_widget`/`tree_*_to_widget` below mirror
    /// `framework/renderer/wgpu/rs/lib.rs`'s private `ui_node_to_widget` conversion; they're
    /// duplicated here (test-only) rather than shared because that crate depends on `ui_wgpu`, never
    /// the reverse — keeping the two in sync is this harness's job.
    fn to_widget_node(node: &UiNode) -> WidgetNode<ActionDescriptor> {
        match node {
            UiNode::Stack(stack) => WidgetNode::Stack { direction: stack.direction.clone(), gap: stack.gap.clone(), padding: stack.padding.clone(), children: stack.children.iter().map(to_widget_node).collect() },
            UiNode::Text(text) => WidgetNode::Text { value: text.value.to_string(), emphasize: text.emphasize.unwrap_or(false) },
            UiNode::Separator(_) => WidgetNode::Separator,
            UiNode::Button(button) => WidgetNode::Button { id: button.id.clone(), icon_id: Some(button.icon_id.clone()), label: button.label.to_string(), event: Some(button.action.clone()) },
            UiNode::Input(input) => {
                WidgetNode::Input { id: input.id.clone(), input_kind: input.input_kind.clone(), value: input.value.clone(), placeholder: input.placeholder.clone().map(|l| l.to_string()), commit: input.commit.clone(), on_change: Some(input.on_change.clone()) }
            }
            UiNode::Select(select) => WidgetNode::Select {
                id: select.id.clone(),
                value: select.value.clone(),
                items: select.items.iter().map(|item| SelectItem { value: item.value.clone(), label: item.label.to_string() }).collect(),
                placeholder: select.placeholder.clone().map(|l| l.to_string()),
                on_change: Some(select.on_change.clone()),
            },
            UiNode::Toggle(toggle) => WidgetNode::Toggle { id: toggle.id.clone(), icon_id: toggle.icon_id.clone(), pressed: toggle.presence.selected, text: toggle.text.clone().map(|l| l.to_string()), on_change: Some(toggle.on_change.clone()) },
            UiNode::KeyValue(kv) => WidgetNode::KeyValue { entries: kv.entries.iter().map(|entry| KeyValueEntry { label: entry.label.to_string(), value: entry.value.clone() }).collect() },
            UiNode::Slider(slider) => WidgetNode::Slider { id: slider.id.clone(), value: slider.value, min: slider.min, max: slider.max, step: slider.step, ready: None, disabled: false, on_change: Some(slider.on_change.clone()) },
            UiNode::NumberStepper(stepper) => {
                WidgetNode::NumberStepper { id: stepper.id.clone(), value: stepper.value, step: stepper.step, uniform: stepper.uniform, on_absolute: Some(stepper.on_absolute.clone()), on_delta: Some(stepper.on_delta.clone()) }
            }
            UiNode::Ring(ring) => WidgetNode::Ring { id: ring.id.clone(), t: ring.t, disabled: ring.presence.state == UiState::Disabled, on_change: Some(ring.on_change.clone()) },
            UiNode::IconSelect(select) => WidgetNode::IconSelect { id: select.id.clone(), value: select.value.clone(), uniform: select.uniform, classifier_kind: select.classifier_kind.clone(), on_change: Some(select.on_change.clone()) },
            UiNode::Field(field) => match ui_node_to_control(&field.child) {
                Some(control) => WidgetNode::Field { id: field.id.clone(), label: field.label.to_string(), child: control_to_widget(&control) },
                None => WidgetNode::Section { id: field.id.clone(), label: Some(field.label.to_string()), default_open: true, children: vec![to_widget_node(&field.child)] },
            },
            UiNode::Section(section) => WidgetNode::Section { id: section.id.clone(), label: section.label.clone().map(|l| l.to_string()), default_open: section.default_open.unwrap_or(true), children: section.children.iter().map(to_widget_node).collect() },
            UiNode::Group(group) => WidgetNode::Section { id: group.id.clone(), label: Some(group.label.to_string()), default_open: group.default_open.unwrap_or(true), children: group.children.iter().map(to_widget_node).collect() },
            UiNode::Tree(tree) => WidgetNode::Tree {
                // 🧭️ Per-item `selected`/`highlighted` (see `tree_item_to_widget`) already carry the
                // full signal from `item.presence` — the tree-level id lists are gone, not re-derived.
                sections: tree.sections.iter().map(tree_section_to_widget).collect(),
                selected_ids: Vec::new(),
                highlighted_ids: Vec::new(),
                // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM W3a: `UiTreeNode.selectionChange`
                // is deleted — selection now flows through `interactionSelect`/`interaction_domain`, not yet
                // wired into this retained-mode engine.
                selection_change: None,
            },
            // KNOWN GAP: `WidgetNode<E>` (the immediate-mode `widgets` region's tree type) has no
            // Image/ComponentScene/ExternalSlot variant at all — the renderer's own
            // `ui_node_to_widget` collapses all three to an empty placeholder `Text` node, which
            // isn't a like-for-like rendering of the same node. There is no immediate-mode output to
            // compare the retained `paint::paint_image`/`paint_component_scene`/`paint_external_slot`
            // against; see the golden tests below for these three, which verify the retained side
            // alone produces sane output and skip the two-pipeline equivalence assertion.
            UiNode::Image(_) | UiNode::ComponentScene(_) | UiNode::ExternalSlot(_) => WidgetNode::Text { value: String::new(), emphasize: false },
        }
    }

    fn control_to_widget(control: &UiControlNode) -> ControlNode<ActionDescriptor> {
        match control {
            UiControlNode::Button(n) => ControlNode::Button { id: n.id.clone(), icon_id: Some(n.icon_id.clone()), label: n.label.to_string(), event: Some(n.action.clone()) },
            UiControlNode::Input(n) => ControlNode::Input { id: n.id.clone(), input_kind: n.input_kind.clone(), value: n.value.clone(), placeholder: n.placeholder.clone().map(|l| l.to_string()), commit: n.commit.clone(), on_change: Some(n.on_change.clone()) },
            UiControlNode::Select(n) => ControlNode::Select {
                id: n.id.clone(),
                value: n.value.clone(),
                items: n.items.iter().map(|item| SelectItem { value: item.value.clone(), label: item.label.to_string() }).collect(),
                placeholder: n.placeholder.clone().map(|l| l.to_string()),
                on_change: Some(n.on_change.clone()),
            },
            UiControlNode::Toggle(n) => ControlNode::Toggle { id: n.id.clone(), icon_id: n.icon_id.clone(), pressed: n.presence.selected, text: n.text.clone().map(|l| l.to_string()), on_change: Some(n.on_change.clone()) },
            UiControlNode::KeyValue(n) => ControlNode::KeyValue { entries: n.entries.iter().map(|entry| KeyValueEntry { label: entry.label.to_string(), value: entry.value.clone() }).collect() },
            UiControlNode::Slider(n) => ControlNode::Slider { id: n.id.clone(), value: n.value, min: n.min, max: n.max, step: n.step, ready: None, disabled: false, on_change: Some(n.on_change.clone()) },
            UiControlNode::NumberStepper(n) => ControlNode::NumberStepper { id: n.id.clone(), value: n.value, step: n.step, uniform: n.uniform, on_absolute: Some(n.on_absolute.clone()), on_delta: Some(n.on_delta.clone()) },
            UiControlNode::Ring(n) => ControlNode::Ring { id: n.id.clone(), t: n.t, disabled: n.presence.state == UiState::Disabled, on_change: Some(n.on_change.clone()) },
            UiControlNode::IconSelect(n) => ControlNode::IconSelect { id: n.id.clone(), value: n.value.clone(), uniform: n.uniform, classifier_kind: n.classifier_kind.clone(), on_change: Some(n.on_change.clone()) },
        }
    }

    /// 🎛️ Same per-variant field mapping as `control_to_widget`, but into a `WidgetNode` instead of a
    /// `ControlNode` — needed for `TreeItem::control: Option<Box<WidgetNode<E>>>`, which (unlike
    /// `Field`'s `child: ControlNode<E>`) embeds a full widget, not a bare control payload.
    fn control_to_widget_node(control: &UiControlNode) -> WidgetNode<ActionDescriptor> {
        match control {
            UiControlNode::Button(n) => WidgetNode::Button { id: n.id.clone(), icon_id: Some(n.icon_id.clone()), label: n.label.to_string(), event: Some(n.action.clone()) },
            UiControlNode::Input(n) => WidgetNode::Input { id: n.id.clone(), input_kind: n.input_kind.clone(), value: n.value.clone(), placeholder: n.placeholder.clone().map(|l| l.to_string()), commit: n.commit.clone(), on_change: Some(n.on_change.clone()) },
            UiControlNode::Select(n) => WidgetNode::Select {
                id: n.id.clone(),
                value: n.value.clone(),
                items: n.items.iter().map(|item| SelectItem { value: item.value.clone(), label: item.label.to_string() }).collect(),
                placeholder: n.placeholder.clone().map(|l| l.to_string()),
                on_change: Some(n.on_change.clone()),
            },
            UiControlNode::Toggle(n) => WidgetNode::Toggle { id: n.id.clone(), icon_id: n.icon_id.clone(), pressed: n.presence.selected, text: n.text.clone().map(|l| l.to_string()), on_change: Some(n.on_change.clone()) },
            UiControlNode::KeyValue(n) => WidgetNode::KeyValue { entries: n.entries.iter().map(|entry| KeyValueEntry { label: entry.label.to_string(), value: entry.value.clone() }).collect() },
            UiControlNode::Slider(n) => WidgetNode::Slider { id: n.id.clone(), value: n.value, min: n.min, max: n.max, step: n.step, ready: None, disabled: false, on_change: Some(n.on_change.clone()) },
            UiControlNode::NumberStepper(n) => WidgetNode::NumberStepper { id: n.id.clone(), value: n.value, step: n.step, uniform: n.uniform, on_absolute: Some(n.on_absolute.clone()), on_delta: Some(n.on_delta.clone()) },
            UiControlNode::Ring(n) => WidgetNode::Ring { id: n.id.clone(), t: n.t, disabled: n.presence.state == UiState::Disabled, on_change: Some(n.on_change.clone()) },
            UiControlNode::IconSelect(n) => WidgetNode::IconSelect { id: n.id.clone(), value: n.value.clone(), uniform: n.uniform, classifier_kind: n.classifier_kind.clone(), on_change: Some(n.on_change.clone()) },
        }
    }

    fn tree_action_to_widget(action: &UiTreeItemAction) -> TreeItemAction<ActionDescriptor> {
        TreeItemAction { icon_id: action.icon_id.clone(), label: action.label.clone().map(|l| l.to_string()), event: action.action.clone(), placement: action.placement() }
    }

    fn tree_item_to_widget(item: &UiTreeItemNode) -> TreeItem<ActionDescriptor> {
        TreeItem {
            id: item.id.clone(),
            label: item.label.to_string(),
            description: item.description.clone(),
            icon_id: item.icon_id.clone(),
            selected: item.presence.selected,
            highlighted: item.presence.state == UiState::Previewed,
            default_open: item.default_open.unwrap_or(false),
            dimmed: item.dimmed.unwrap_or(false),
            event: item.action.clone(),
            // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM W3a: `UiTreeItemNode.hoverAction`/
            // `unhoverAction` are deleted — hover is now framework-owned per `UiTreeNode.interactionDomain`.
            hover_event: None,
            unhover_event: None,
            actions: item.actions.as_ref().map(|actions| actions.iter().map(tree_action_to_widget).collect()).unwrap_or_default(),
            draggable: item.draggable.unwrap_or(false),
            drag_data: item.drag_data.clone().unwrap_or_default(),
            control: item.control.as_ref().map(|control| Box::new(control_to_widget_node(control))),
            children: item.items.as_ref().map(|items| items.iter().map(tree_item_to_widget).collect()).unwrap_or_default(),
        }
    }

    fn tree_section_to_widget(section: &UiTreeSectionNode) -> TreeSection<ActionDescriptor> {
        TreeSection { id: section.id.clone(), label: section.label.clone().map(|l| l.to_string()), default_open: section.default_open.unwrap_or(true), items: section.items.iter().map(tree_item_to_widget).collect() }
    }

    /// 📊️ Total (ui_instances incl. overlay, vector_vertices incl. overlay, raster_instances) across
    /// every layer of a `DrawList` — the "structurally equivalent" signal this harness compares,
    /// deliberately coarser than exact geometry per this ticket's tolerance allowance.
    fn stats(draw: &DrawList) -> (usize, usize, usize) {
        let instances = draw.layers.iter().map(|layer| layer.ui_instances.len() + layer.overlay_ui_instances.len()).sum();
        let vectors = draw.layers.iter().map(|layer| layer.vector_vertices.len() + layer.overlay_vector_vertices.len()).sum();
        let raster = draw.layers.iter().map(|layer| layer.raster_instances.len()).sum();
        (instances, vectors, raster)
    }

    fn retained_stats(node: &UiNode) -> (usize, usize, usize) {
        let mut ui = Ui::new();
        let mut atlas = FontAtlas::builtin();
        ui.apply_tree("golden", node);
        let draw = ui.frame::<RecordingSceneHost>("golden", 400.0, 400.0, &mut atlas, None, None).expect("apply_tree then frame must produce a draw list");
        stats(draw)
    }

    fn immediate_stats(node: &UiNode, bounds: Rect) -> (usize, usize, usize) {
        let widget = to_widget_node(node);
        let mut draw = DrawList::default();
        let mut atlas = FontAtlas::builtin();
        let theme = Theme::default();
        let mut input = InputState::<ActionDescriptor>::default();
        let mut scroll_offsets: StdHashMap<String, f32> = StdHashMap::new();
        let mut collapsed_sections: StdHashMap<String, bool> = StdHashMap::new();
        let mut open_selects: StdHashMap<String, bool> = StdHashMap::new();
        let mut ctx = WidgetContext {
            draw: &mut draw,
            overlay: None,
            atlas: &mut atlas,
            icons: None,
            input: &mut input,
            theme: &theme,
            scroll_offsets: &mut scroll_offsets,
            collapsed_sections: &mut collapsed_sections,
            open_selects: &mut open_selects,
            interaction_maps: None,
            pick_clip: None,
        };
        render_widget(&widget, bounds, &mut ctx);
        stats(&draw)
    }

    const VIEWPORT: Rect = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };

    /// 🧱️ Wraps a leaf `UiNode` as the sole child of a gap-less/padding-less vertical `Stack`: on the
    /// retained side, `flex::LayoutEngine` always forces the *root* to the full viewport size
    /// (`compute`'s `root_style.size` override) and gives a `Stack`'s only child `flex_grow: 1.0`, so
    /// the child's resolved `LayoutBucket` is exactly the full viewport. On the immediate side,
    /// `layout::layout_vertical`/`layout_horizontal`'s `extra_per_child` gives a lone child the same
    /// full bounds. Wrapping every leaf fixture this way guarantees both pipelines paint it at
    /// identical bounds, which is what makes an exact instance/vector-count comparison meaningful
    /// instead of an artifact of divergent layout math.
    fn leaf(child: UiNode) -> UiNode {
        UiNode::Stack(UiStackNode {
            direction: "vertical".into(),
            gap: Some("none".into()),
            padding: Some("none".into()),
            id: None,
            presence: UiPresence::default(),
            activate: None,
            drop_action: None,
            drop_overlay: None,
            children: vec![child],
            menu: None,
        })
    }

    fn assert_equivalent(kind: &str, node: &UiNode) {
        let retained = retained_stats(node);
        let immediate = immediate_stats(node, VIEWPORT);
        assert_eq!(retained, immediate, "{kind}: retained (instances, vectors, raster) {retained:?} != immediate {immediate:?}");
    }

    // `action()` is shared with 🔖️FacadeTests above — both sub-regions live in the same `mod tests`.

    #[test]
    fn golden_stack() {
        let node = UiNode::Stack(UiStackNode {
            direction: "vertical".into(),
            gap: Some("none".into()),
            padding: Some("none".into()),
            id: None,
            presence: UiPresence::default(),
            activate: None,
            drop_action: None,
            drop_overlay: None,
            children: vec![
                UiNode::Text(UiTextNode { value: Label::data("hello"), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None }),
                UiNode::Separator(UiSeparatorNode { presence: UiPresence::default(), menu: None }),
            ],
            menu: None,
        });
        assert_equivalent("Stack", &node);
    }

    #[test]
    fn golden_text() {
        assert_equivalent("Text", &leaf(UiNode::Text(UiTextNode { value: Label::data("hello world"), emphasize: Some(true), data_attributes: None, presence: UiPresence::default(), menu: None })));
    }

    #[test]
    fn golden_button() {
        assert_equivalent("Button", &leaf(UiNode::Button(UiButtonNode { id: Some("btn".into()), icon_id: IconName::CircleDot, label: Label::data("Go"), action: action(), style: None, presence: UiPresence::default(), menu: None })));
    }

    #[test]
    fn golden_separator() {
        assert_equivalent("Separator", &leaf(UiNode::Separator(UiSeparatorNode { presence: UiPresence::default(), menu: None })));
    }

    #[test]
    fn golden_input() {
        assert_equivalent(
            "Input",
            &leaf(UiNode::Input(UiInputNode {
                id: "in".into(),
                input_kind: "text".into(),
                value: "abc".into(),
                placeholder: None,
                commit: None,
                min: None,
                max: None,
                step: None,
                accept: None,
                on_change: action(),
                presence: UiPresence::default(),
                menu: None,
            })),
        );
    }

    #[test]
    fn golden_select() {
        assert_equivalent(
            "Select",
            &leaf(UiNode::Select(UiSelectNode {
                id: "sel".into(),
                value: "a".into(),
                items: vec![UiSelectItem { value: "a".into(), label: Label::data("Alpha") }, UiSelectItem { value: "b".into(), label: Label::data("Beta") }],
                placeholder: None,
                on_change: action(),
                presence: UiPresence::default(),
                menu: None,
            })),
        );
    }

    #[test]
    fn golden_toggle() {
        // 🚫️ `presence.selected` is intentionally NOT exercised here: the shared `presence_overlay`
        // now draws an outset accent ring for ANY selected element (see
        // `selected_presence_draws_an_outset_ring_on_any_element`, below) — a deliberate new
        // capability `widgets::render_toggle` (the immediate-mode reference this harness compares
        // against) never had, so a selected fixture would fail this equivalence check for the wrong
        // reason. This test stays scoped to the base (unselected) toggle's fill/label parity.
        assert_equivalent("Toggle", &leaf(UiNode::Toggle(UiToggleNode { id: "tog".into(), icon_id: IconName::CircleDot, text: Some(Label::data("On")), on_change: action(), presence: UiPresence::default(), menu: None })));
    }

    /// ✨️ `presence.selected` draws its outset accent ring universally — proven here on `Toggle`, a
    /// non-`Stack` variant, since `selected` used to be a `UiStackNode`-only field. Instance count
    /// grows vs. the unselected fixture (the extra `push_chrome_border` edges), confirming the ring
    /// is now a shared channel every element gets for free from `presence_overlay`.
    #[test]
    fn selected_presence_draws_an_outset_ring_on_any_element() {
        let unselected = UiNode::Toggle(UiToggleNode { id: "tog".into(), icon_id: IconName::CircleDot, text: Some(Label::data("On")), on_change: action(), presence: UiPresence::default(), menu: None });
        let selected = UiNode::Toggle(UiToggleNode { id: "tog".into(), icon_id: IconName::CircleDot, text: Some(Label::data("On")), on_change: action(), presence: UiPresence::selected(true), menu: None });
        let (unselected_instances, _, _) = retained_stats(&leaf(unselected));
        let (selected_instances, _, _) = retained_stats(&leaf(selected));
        assert!(selected_instances > unselected_instances, "a selected element should paint more instances than an unselected one (the outset accent ring)");
    }

    #[test]
    fn golden_key_value() {
        assert_equivalent("KeyValue", &leaf(UiNode::KeyValue(UiKeyValueNode { entries: vec![UiKeyValueEntry { label: Label::data("Name"), value: Label::data("Semio").to_string() }], presence: UiPresence::default(), menu: None })));
    }

    #[test]
    fn golden_slider() {
        assert_equivalent("Slider", &leaf(UiNode::Slider(UiSliderNode { id: "sl".into(), value: 0.5, min: 0.0, max: 1.0, step: 0.01, unit: None, on_change: action(), presence: UiPresence::default(), menu: None })));
    }

    /// KNOWN GAP: `widgets::render_number_stepper` renders its center value segment via a full
    /// `render_input` call (which itself calls `push_control_border` — a background fill plus 4
    /// border-edge quads, 5 instances), giving the center value its own nested input-style border box.
    /// `paint::paint_number_stepper` instead just `draw_text_on`s the formatted value directly with no
    /// surrounding border. Confirmed by running this fixture: retained emits 14 instances (one
    /// `push_control_border` for the whole control + 2 divider lines + 3 text runs), immediate emits
    /// 19 (the same 14 plus the center value's own nested 5-instance border box) — a real, reproducible
    /// paint-logic difference, not a fixture/harness artifact. This is real follow-up work for `paint`
    /// (either add the nested border to `paint_number_stepper`, or confirm the immediate path's nested
    /// border is unintentional and should be dropped there instead — a product decision outside this
    /// façade's scope), not something to paper over here.
    #[test]
    fn golden_number_stepper_known_gap() {
        let (instances, _, _) = retained_stats(&leaf(UiNode::NumberStepper(UiNumberStepperNode { id: "ns".into(), value: 2.0, step: 1.0, uniform: false, on_absolute: action(), on_delta: action(), presence: UiPresence::default(), menu: None })));
        assert!(instances > 0, "NumberStepper should paint its minus/value/plus segments");
    }

    /// 🔒️ Added by `w1c-paint-parity` (see `.🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY/report-w1c-paint-parity.md`):
    /// `paint::paint_number_stepper` now ports `widgets::render_number_stepper`'s nested
    /// center-value border box (the exact gap `golden_number_stepper_known_gap`'s doc comment
    /// above documents), closing the 14-vs-19-instance divergence for the `uniform: true` case.
    /// Left `golden_number_stepper_known_gap` itself untouched (still valid, still a `uniform: false`
    /// fixture) and added this as a new, additive `assert_equivalent` case for `uniform: true`
    /// instead, per this workstream's "don't modify existing tests" rule.
    #[test]
    fn golden_number_stepper() {
        assert_equivalent("NumberStepper", &leaf(UiNode::NumberStepper(UiNumberStepperNode { id: "ns".into(), value: 2.0, step: 1.0, uniform: true, on_absolute: action(), on_delta: action(), presence: UiPresence::default(), menu: None })));
    }

    #[test]
    fn golden_ring() {
        assert_equivalent("Ring", &leaf(UiNode::Ring(UiRingNode { id: "ring".into(), orb_id: "orb".into(), t: 0.25, on_change: action(), presence: UiPresence::default(), menu: None })));
    }

    #[test]
    fn golden_icon_select() {
        assert_equivalent(
            "IconSelect",
            &leaf(UiNode::IconSelect(UiIconSelectNode { id: "ic".into(), value: IconName::Sparkles.to_string(), uniform: false, classifier_kind: "kind".into(), on_change: action(), presence: UiPresence::default(), menu: None })),
        );
    }

    #[test]
    fn golden_tree() {
        let item = |id: &str, label: &str| UiTreeItemNode {
            id: id.into(),
            label: Label::data(label),
            description: None,
            icon_id: None,
            presence: UiPresence::default(),
            default_open: None,
            action: None,
            actions: None,
            draggable: None,
            drag_data: None,
            items: None,
            control: None,
            dimmed: None,
            menu: None,
        };
        let node = UiNode::Tree(UiTreeNode {
            sections: vec![UiTreeSectionNode { id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![item("i1", "Item One"), item("i2", "Item Two")] }],
            presence: UiPresence::default(),
            drop_action: None,
            menu: None,
            interaction_domain: None,
        });
        assert_equivalent("Tree", &node);
    }

    /// KNOWN GAP: `reconcile` only expands `Field`/`Section` into a real retained child for their
    /// `child`/`children` payload (per `reconcile`'s own module doc comment — M2 recurses into
    /// `Stack`/`Section`/`Field` only), but `flex::LayoutEngine` only grants `flex_grow: 1.0` to a
    /// `Stack`'s children (see `style_with_grow`'s `flex_grow_child` param, gated on
    /// `matches!(node.spec.0, UiNode::Stack(_))`). A `Field`/`Section`'s synthetic retained child is
    /// therefore laid out at its own intrinsic content size instead of filling the label-adjusted
    /// remainder the way `widgets::render_widget`'s hand-rolled `Field`/`Section` branches
    /// (`Rect::new(bounds.x, bounds.y + label_h + gap, bounds.w, bounds.h - label_h - gap)` for
    /// `Field`, per-child accumulated `y` for `Section`) explicitly carve out. The two pipelines'
    /// geometry — and therefore instance counts for size-dependent content like wrapped `Text` — can
    /// genuinely diverge here. This is real follow-up work for `flex`, not something this façade can
    /// paper over; these two tests verify the retained side alone produces sane, non-empty output.
    #[test]
    fn golden_field_known_gap() {
        let node = UiNode::Field(UiFieldNode {
            id: "f".into(),
            label: Label::data("Label"),
            description: None,
            required: None,
            error: None,
            child: Box::new(UiNode::Input(UiInputNode {
                id: "in".into(),
                input_kind: "text".into(),
                value: "abc".into(),
                placeholder: None,
                commit: None,
                min: None,
                max: None,
                step: None,
                accept: None,
                on_change: action(),
                presence: UiPresence::default(),
                menu: None,
            })),
            presence: UiPresence::default(),
            menu: None,
        });
        let (instances, _, _) = retained_stats(&node);
        assert!(instances > 0, "Field should paint its label plus its child control");
    }

    #[test]
    fn golden_section_known_gap() {
        let node = UiNode::Section(UiSectionNode {
            id: "sec".into(),
            label: Some(Label::data("Section")),
            default_open: Some(true),
            presence: UiPresence::default(),
            children: vec![UiNode::Text(UiTextNode { value: Label::data("child"), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None })],
            menu: None,
        });
        let (instances, _, _) = retained_stats(&node);
        assert!(instances > 0, "Section should paint its header label plus its children");
    }

    /// KNOWN GAP: see `to_widget_node`'s own `UiNode::Image | UiNode::ComponentScene | UiNode::ExternalSlot`
    /// match arm doc comment — `WidgetNode<E>` has no variant for any of these three, so there is no
    /// immediate-mode equivalent to compare against at all. `paint::paint_image`/
    /// `paint_component_scene`/`paint_external_slot` are themselves documented placeholders (no host
    /// texture-upload queue / scene-host / plugin-body wiring exists in `ui_wgpu` yet either) — these
    /// tests only verify the retained side produces the placeholder chrome its own doc comments
    /// promise, not equivalence with anything immediate-mode.
    #[test]
    fn golden_image_known_gap() {
        let node = UiNode::Image(UiImageNode { id: "img".into(), src: String::new(), alt: Some(Label::data("alt text")), presence: UiPresence::default(), menu: None });
        let (instances, _, _) = retained_stats(&node);
        assert!(instances > 0, "an empty-src Image should still paint its alt text");
    }

    #[test]
    fn golden_component_scene_known_gap() {
        let node = UiNode::ComponentScene(UiComponentSceneNode {
            surface_id: "surf".into(),
            controller_id: "ctrl".into(),
            component_kind: SurfaceKind::World3d,
            pane_id: None,
            binding_id: None,
            presence: UiPresence::default(),
            canvas_2d: None,
            world_3d: None,
            node_graph: None,
            text_editor: None,
            table: None,
            paint_2d: None,
            virtual_file_system: None,
            tiled_map: None,
            board2d: None,
            icon_render: None,
            ink_canvas: None,
            graph_timeline: None,
            block_list: None,
            diff_view: None,
            event_feed: None,
            menu: None,
        });
        let (instances, _, _) = retained_stats(&node);
        assert!(instances > 0, "ComponentScene should paint its placeholder border chrome");
    }

    #[test]
    fn golden_external_slot_known_gap() {
        let node = UiNode::ExternalSlot(UiExternalSlotNode { plugin_id: "plug".into(), app_id: "app".into(), body_key: "body".into(), params_json: "{}".into(), presence: UiPresence::default(), menu: None });
        let (instances, _, _) = retained_stats(&node);
        assert!(instances > 0, "ExternalSlot should paint its placeholder chrome plus its body_key label");
    }
    //#endregion 🔖️GoldenHarness

    //#region 🔬️IntrospectionTests
    #[test]
    fn window_ids_viewport_tree_and_theme_expose_private_window_state() {
        let mut ui = Ui::new();
        assert_eq!(ui.window_ids().count(), 0);
        assert_eq!(ui.viewport("win"), None);
        assert!(ui.tree("win").is_none());

        let node = UiNode::Text(UiTextNode { value: Label::data("hi"), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None });
        ui.apply_tree("win", &node);
        ui.set_viewport("win", 800.0, 600.0);

        let ids: Vec<&str> = ui.window_ids().collect();
        assert_eq!(ids, vec!["win"]);
        assert_eq!(ui.viewport("win"), Some((800.0, 600.0)));
        let tree = ui.tree("win").expect("tree exists after apply_tree");
        assert!(tree.root.is_some());
        assert_eq!(ui.theme().text.a, Theme::default().text.a);
    }
    //#endregion 🔬️IntrospectionTests

    //#region 🧩️WidgetsInternalsTests
    /// 🧰️ Owns every piece `widgets::WidgetContext<'_, ActionDescriptor>` borrows, so each test can
    /// build one without fighting lifetimes; `ctx()` re-borrows fresh each call (a `WidgetContext`
    /// isn't `Clone`/reusable once passed to `render_widget`, which can mutate through it).
    struct WidgetHarness {
        draw: DrawList,
        atlas: FontAtlas,
        theme: Theme,
        input: InputState<ActionDescriptor>,
        scroll_offsets: StdHashMap<String, f32>,
        collapsed_sections: StdHashMap<String, bool>,
        open_selects: StdHashMap<String, bool>,
        maps: WidgetInteractionMaps<ActionDescriptor>,
    }

    impl WidgetHarness {
        fn new() -> Self {
            Self {
                draw: DrawList::default(),
                atlas: FontAtlas::builtin(),
                theme: Theme::default(),
                input: InputState::default(),
                scroll_offsets: StdHashMap::new(),
                collapsed_sections: StdHashMap::new(),
                open_selects: StdHashMap::new(),
                maps: WidgetInteractionMaps::default(),
            }
        }

        fn ctx(&mut self) -> WidgetContext<'_, ActionDescriptor> {
            WidgetContext {
                draw: &mut self.draw,
                overlay: None,
                atlas: &mut self.atlas,
                icons: None,
                input: &mut self.input,
                theme: &self.theme,
                scroll_offsets: &mut self.scroll_offsets,
                collapsed_sections: &mut self.collapsed_sections,
                open_selects: &mut self.open_selects,
                interaction_maps: Some(&mut self.maps),
                pick_clip: None,
            }
        }
    }

    #[test]
    fn wrap_text_wraps_long_text_across_multiple_lines() {
        let mut atlas = FontAtlas::builtin();
        let long = "word ".repeat(40);
        let lines = wrap_text(&mut atlas, &long, 100.0, 16.0);
        assert!(lines.len() > 1, "text far wider than max_width must wrap into multiple lines");
        for line in &lines {
            assert!(!line.is_empty());
        }
    }

    #[test]
    fn wrap_text_of_empty_string_yields_one_empty_line() {
        let mut atlas = FontAtlas::builtin();
        let lines = wrap_text(&mut atlas, "", 200.0, 16.0);
        assert_eq!(lines, vec![String::new()], "an empty input must still produce a single (empty) line, never zero lines");
    }

    #[test]
    fn measure_widget_stack_vertical_sums_child_heights_and_maxes_width() {
        let mut atlas = FontAtlas::builtin();
        let theme = Theme::default();
        let node = WidgetNode::<ActionDescriptor>::Stack { direction: "vertical".into(), gap: Some("none".into()), padding: Some("none".into()), children: vec![WidgetNode::Separator, WidgetNode::Separator] };
        let (_, h) = measure_widget(&mut atlas, &theme, &node);
        let (_, single_h) = measure_widget(&mut atlas, &theme, &WidgetNode::<ActionDescriptor>::Separator);
        assert!((h - single_h * 2.0).abs() < 0.001, "two stacked separators with no gap/padding must measure to exactly twice one separator's height, got {h} vs {single_h}");
    }

    #[test]
    fn measure_widget_stack_horizontal_sums_child_widths() {
        let mut atlas = FontAtlas::builtin();
        let theme = Theme::default();
        let button = || WidgetNode::<ActionDescriptor>::Button { id: Some("b".into()), icon_id: None, label: Label::data("Go").to_string(), event: None };
        let node = WidgetNode::Stack { direction: "horizontal".into(), gap: Some("none".into()), padding: Some("none".into()), children: vec![button(), button()] };
        let (w, _) = measure_widget(&mut atlas, &theme, &node);
        assert!((w - theme.control_height * 2.0).abs() < 0.001, "two gap-less horizontal buttons must measure to exactly twice one control's width");
    }

    #[test]
    fn measure_widget_separator_uses_theme_control_height_floor() {
        let mut atlas = FontAtlas::builtin();
        let theme = Theme::default();
        let (w, h) = measure_widget(&mut atlas, &theme, &WidgetNode::<ActionDescriptor>::Separator);
        assert_eq!(w, theme.control_height.max(1.0));
        assert_eq!(h, 1.0 + theme.gap_standard);
    }

    #[test]
    fn measure_widget_key_value_grows_with_entry_count() {
        let mut atlas = FontAtlas::builtin();
        let theme = Theme::default();
        let one = WidgetNode::<ActionDescriptor>::KeyValue { entries: vec![KeyValueEntry { label: Label::data("A").to_string(), value: "1".into() }] };
        let two = WidgetNode::<ActionDescriptor>::KeyValue { entries: vec![KeyValueEntry { label: Label::data("A").to_string(), value: "1".into() }, KeyValueEntry { label: Label::data("B").to_string(), value: "2".into() }] };
        let (_, h1) = measure_widget(&mut atlas, &theme, &one);
        let (_, h2) = measure_widget(&mut atlas, &theme, &two);
        assert!((h2 - h1 * 2.0).abs() < 0.001, "KeyValue height must scale linearly with entry count");
    }

    #[test]
    fn measure_widget_ring_is_fixed_size() {
        let mut atlas = FontAtlas::builtin();
        let theme = Theme::default();
        let (w, h) = measure_widget(&mut atlas, &theme, &WidgetNode::<ActionDescriptor>::Ring { id: "r".into(), t: 0.5, disabled: false, on_change: None });
        assert_eq!((w, h), (80.0, 80.0));
    }

    #[test]
    fn measure_widget_field_combines_label_and_child_height() {
        let mut atlas = FontAtlas::builtin();
        let theme = Theme::default();
        let node = WidgetNode::<ActionDescriptor>::Field { id: "f".into(), label: Label::data("Label").to_string(), child: ControlNode::Slider { id: "s".into(), value: 0.5, min: 0.0, max: 1.0, step: 0.1, ready: None, disabled: false, on_change: None } };
        let (_, h) = measure_widget(&mut atlas, &theme, &node);
        assert!(h > theme.control_height, "a Field's total height must be its label plus its child control, so it must exceed the control's own height alone");
    }

    #[test]
    fn measure_widget_section_sums_header_and_children_plus_gap() {
        let mut atlas = FontAtlas::builtin();
        let theme = Theme::default();
        let empty = WidgetNode::<ActionDescriptor>::Section { id: "s".into(), label: None, default_open: true, children: vec![] };
        let with_child = WidgetNode::<ActionDescriptor>::Section { id: "s".into(), label: None, default_open: true, children: vec![WidgetNode::Separator] };
        let (_, empty_h) = measure_widget(&mut atlas, &theme, &empty);
        let (_, child_h) = measure_widget(&mut atlas, &theme, &with_child);
        assert!(child_h > empty_h, "adding a child must grow a Section's measured height beyond its bare header height");
    }

    #[test]
    fn measure_widget_tree_skips_dimmed_items_in_height() {
        let mut atlas = FontAtlas::builtin();
        let theme = Theme::default();
        let item = |id: &str, dimmed: bool| TreeItem {
            id: id.into(),
            label: Label::data(id).to_string(),
            description: None,
            icon_id: None,
            selected: false,
            highlighted: false,
            default_open: false,
            dimmed,
            event: None,
            hover_event: None,
            unhover_event: None,
            actions: vec![],
            draggable: false,
            drag_data: StdHashMap::new(),
            control: None,
            children: vec![],
        };
        let visible =
            WidgetNode::<ActionDescriptor>::Tree { sections: vec![TreeSection { id: "sec".into(), label: None, default_open: true, items: vec![item("a", false)] }], selected_ids: vec![], highlighted_ids: vec![], selection_change: None };
        let dimmed = WidgetNode::<ActionDescriptor>::Tree { sections: vec![TreeSection { id: "sec".into(), label: None, default_open: true, items: vec![item("a", true)] }], selected_ids: vec![], highlighted_ids: vec![], selection_change: None };
        let (_, visible_h) = measure_widget(&mut atlas, &theme, &visible);
        let (_, dimmed_h) = measure_widget(&mut atlas, &theme, &dimmed);
        assert!(dimmed_h < visible_h, "a dimmed tree item must contribute zero height, so the dimmed tree must measure shorter than the visible one");
    }

    #[test]
    fn widget_interaction_maps_clear_frame_empties_every_map() {
        let mut maps = WidgetInteractionMaps::<ActionDescriptor>::default();
        maps.input_metas.insert("i".into(), InputMeta { on_change: action(), commit: None, value: "v".into() });
        maps.select_metas.insert("s".into(), action());
        maps.toggle_metas.insert("t".into(), (true, action()));
        maps.slider_metas.insert("sl".into(), SliderMeta { on_change: action(), min: 0.0, max: 1.0, step: 0.1, value: 0.5, bounds_x: 0.0, bounds_w: 10.0 });
        maps.stepper_metas.insert("st".into(), StepperMeta { on_absolute: action(), on_delta: action(), step: 1.0, value: 1.0 });
        maps.ring_metas.insert("r".into(), RingMeta { on_change: action(), disabled: false, center_x: 0.0, center_y: 0.0, radius: 10.0 });
        maps.slider_live_values.insert("sl".into(), 0.5);
        maps.ring_live_values.insert("r".into(), 0.5);
        maps.tree_hover_commands.insert("h".into(), action());
        maps.tree_unhover_commands.insert("u".into(), action());
        maps.tree_selection_change = Some(action());

        maps.clear_frame();

        assert!(maps.input_metas.is_empty());
        assert!(maps.select_metas.is_empty());
        assert!(maps.toggle_metas.is_empty());
        assert!(maps.slider_metas.is_empty());
        assert!(maps.stepper_metas.is_empty());
        assert!(maps.ring_metas.is_empty());
        assert!(maps.slider_live_values.is_empty());
        assert!(maps.ring_live_values.is_empty());
        assert!(maps.tree_hover_commands.is_empty());
        assert!(maps.tree_unhover_commands.is_empty());
        assert!(maps.tree_selection_change.is_none());
    }

    #[test]
    fn render_widget_input_registers_interaction_meta_when_maps_present() {
        let mut h = WidgetHarness::new();
        let node = WidgetNode::Input { id: "in".into(), input_kind: "text".into(), value: "hello".into(), placeholder: None, commit: Some("blur".into()), on_change: Some(action()) };
        render_widget(&node, VIEWPORT, &mut h.ctx());
        let meta = h.maps.input_metas.get("in").expect("register_input_meta must populate the map when interaction_maps is Some and on_change is Some");
        assert_eq!(meta.value, "hello");
        assert_eq!(meta.commit.as_deref(), Some("blur"));
    }

    #[test]
    fn render_widget_input_with_no_on_change_does_not_register_meta() {
        let mut h = WidgetHarness::new();
        let node = WidgetNode::Input { id: "in".into(), input_kind: "text".into(), value: "hello".into(), placeholder: None, commit: None, on_change: None };
        render_widget(&node, VIEWPORT, &mut h.ctx());
        assert!(h.maps.input_metas.is_empty(), "no on_change means nothing should be wired for the host to fire");
    }

    #[test]
    fn render_widget_select_and_toggle_register_interaction_metas() {
        let mut h = WidgetHarness::new();
        let select = WidgetNode::Select { id: "sel".into(), value: "a".into(), items: vec![SelectItem { value: "a".into(), label: Label::data("Alpha").to_string() }], placeholder: None, on_change: Some(action()) };
        render_widget(&select, VIEWPORT, &mut h.ctx());
        assert!(h.maps.select_metas.contains_key("sel"));

        let toggle = WidgetNode::Toggle { id: "tog".into(), icon_id: IconName::CircleDot, pressed: true, text: Some("On".into()), on_change: Some(action()) };
        render_widget(&toggle, VIEWPORT, &mut h.ctx());
        let (pressed, _) = h.maps.toggle_metas.get("tog").expect("toggle meta must be registered");
        assert!(*pressed);
    }

    #[test]
    fn render_widget_slider_registers_meta_and_live_value_unless_disabled() {
        let mut h = WidgetHarness::new();
        let enabled = WidgetNode::Slider { id: "sl".into(), value: 0.5, min: 0.0, max: 1.0, step: 0.01, ready: None, disabled: false, on_change: Some(action()) };
        render_widget(&enabled, VIEWPORT, &mut h.ctx());
        assert!(h.maps.slider_metas.contains_key("sl"));
        assert!(h.maps.slider_live_values.contains_key("sl"));

        let mut h2 = WidgetHarness::new();
        let disabled = WidgetNode::Slider { id: "sl".into(), value: 0.5, min: 0.0, max: 1.0, step: 0.01, ready: None, disabled: true, on_change: Some(action()) };
        render_widget(&disabled, VIEWPORT, &mut h2.ctx());
        assert!(h2.maps.slider_metas.is_empty(), "a disabled slider must not register interaction metadata");
        assert!(h2.maps.slider_live_values.is_empty());
    }

    #[test]
    fn render_widget_number_stepper_registers_stepper_meta() {
        let mut h = WidgetHarness::new();
        let node = WidgetNode::NumberStepper { id: "ns".into(), value: 3.0, step: 1.0, uniform: false, on_absolute: Some(action()), on_delta: Some(action()) };
        render_widget(&node, VIEWPORT, &mut h.ctx());
        let meta = h.maps.stepper_metas.get("ns").expect("stepper meta must be registered when both on_absolute and on_delta are Some");
        assert_eq!(meta.value, 3.0);
        assert!(h.maps.input_metas.contains_key("ns.input"), "the stepper's embedded value segment renders through render_input and must also register an input meta");
    }

    #[test]
    fn render_widget_ring_registers_meta_and_live_value() {
        let mut h = WidgetHarness::new();
        let node = WidgetNode::Ring { id: "r".into(), t: 0.25, disabled: false, on_change: Some(action()) };
        render_widget(&node, VIEWPORT, &mut h.ctx());
        assert!(h.maps.ring_metas.contains_key("r"));
        assert_eq!(h.maps.ring_live_values.get("r"), Some(&0.25));
    }

    #[test]
    fn render_widget_field_draws_label_and_delegates_to_control() {
        let mut h = WidgetHarness::new();
        let node = WidgetNode::Field { id: "f".into(), label: Label::data("Name").to_string(), child: ControlNode::Input { id: "in".into(), input_kind: "text".into(), value: "x".into(), placeholder: None, commit: None, on_change: Some(action()) } };
        render_widget(&node, VIEWPORT, &mut h.ctx());
        assert!(h.maps.input_metas.contains_key("in"), "Field must render its child control (an Input here), which registers its own interaction meta");
        let total: usize = h.draw.layers.iter().map(|l| l.ui_instances.len()).sum();
        assert!(total > 0, "Field must paint its label plus its child control");
    }

    #[test]
    fn render_widget_section_toggles_collapsed_state_from_default_open() {
        let child = || WidgetNode::<ActionDescriptor>::Text { value: "child text".into(), emphasize: false };
        let mut h = WidgetHarness::new();
        let closed = WidgetNode::<ActionDescriptor>::Section { id: "sec".into(), label: Some(Label::data("Sec").to_string()), default_open: false, children: vec![child()] };
        render_widget(&closed, VIEWPORT, &mut h.ctx());
        assert_eq!(h.collapsed_sections.get("section.sec"), Some(&true), "a Section with default_open: false must seed its collapsed_sections entry as collapsed");

        let mut h2 = WidgetHarness::new();
        let open = WidgetNode::<ActionDescriptor>::Section { id: "sec".into(), label: Some(Label::data("Sec").to_string()), default_open: true, children: vec![child()] };
        render_widget(&open, VIEWPORT, &mut h2.ctx());
        assert_eq!(h2.collapsed_sections.get("section.sec"), Some(&false));
        let closed_instances: usize = h.draw.layers.iter().map(|l| l.ui_instances.len()).sum();
        let open_instances: usize = h2.draw.layers.iter().map(|l| l.ui_instances.len()).sum();
        assert!(open_instances > closed_instances, "an open section must also paint its (visible) child's glyphs, a collapsed one must not");
    }

    #[test]
    fn render_widget_tree_populates_hover_and_unhover_commands() {
        let mut h = WidgetHarness::new();
        let item = TreeItem {
            id: "i1".into(),
            label: Label::data("Item").to_string(),
            description: None,
            icon_id: None,
            selected: false,
            highlighted: false,
            default_open: false,
            dimmed: false,
            event: None,
            hover_event: Some(action()),
            unhover_event: Some(action()),
            actions: vec![],
            draggable: false,
            drag_data: StdHashMap::new(),
            control: None,
            children: vec![],
        };
        let node = WidgetNode::<ActionDescriptor>::Tree {
            sections: vec![TreeSection { id: "s".into(), label: Some(Label::data("Section").to_string()), default_open: true, items: vec![item] }],
            selected_ids: vec![],
            highlighted_ids: vec![],
            selection_change: Some(action()),
        };
        render_widget(&node, VIEWPORT, &mut h.ctx());
        assert!(h.maps.tree_hover_commands.contains_key("i1"));
        assert!(h.maps.tree_unhover_commands.contains_key("i1"));
        assert_eq!(h.maps.tree_selection_change, Some(action()));
    }

    #[test]
    fn render_widget_tree_row_actions_register_hits_without_hover() {
        let mut h = WidgetHarness::new();
        let item = TreeItem {
            id: "i1".into(),
            label: Label::data("Item").to_string(),
            description: None,
            icon_id: None,
            selected: false,
            highlighted: false,
            default_open: false,
            dimmed: false,
            event: None,
            hover_event: None,
            unhover_event: None,
            actions: vec![TreeItemAction { icon_id: IconName::CircleDot, label: Some(Label::data("Del").to_string()), event: action(), placement: UiTreeActionPlacement::Row }],
            draggable: false,
            drag_data: StdHashMap::new(),
            control: None,
            children: vec![],
        };
        let node = WidgetNode::<ActionDescriptor>::Tree { sections: vec![TreeSection { id: "s".into(), label: None, default_open: true, items: vec![item] }], selected_ids: vec![], highlighted_ids: vec![], selection_change: None };
        render_widget(&node, VIEWPORT, &mut h.ctx());
        let action_hits = h.input.hit_targets.iter().filter(|t| t.control_id.as_deref() == Some("tree.action.i1.0")).count();
        assert_eq!(action_hits, 1, "row-placement actions must register a hit target even when the row is unhovered");
    }

    #[test]
    fn render_widget_tree_menu_placement_skips_row_action_hits() {
        let mut h = WidgetHarness::new();
        let item = TreeItem {
            id: "i1".into(),
            label: Label::data("Item").to_string(),
            description: None,
            icon_id: None,
            selected: false,
            highlighted: false,
            default_open: false,
            dimmed: false,
            event: None,
            hover_event: None,
            unhover_event: None,
            actions: vec![TreeItemAction { icon_id: IconName::CircleDot, label: Some(Label::data("Del").to_string()), event: action(), placement: UiTreeActionPlacement::Menu }],
            draggable: false,
            drag_data: StdHashMap::new(),
            control: None,
            children: vec![],
        };
        let node = WidgetNode::<ActionDescriptor>::Tree { sections: vec![TreeSection { id: "s".into(), label: None, default_open: true, items: vec![item] }], selected_ids: vec![], highlighted_ids: vec![], selection_change: None };
        render_widget(&node, VIEWPORT, &mut h.ctx());
        let action_hits = h.input.hit_targets.iter().filter(|t| t.control_id.as_deref() == Some("tree.action.i1.0")).count();
        assert_eq!(action_hits, 0, "menu-placement actions must not register row hit targets");
    }

    #[test]
    fn render_widget_tree_marks_selected_and_highlighted_ids_via_ids_list() {
        let mut h = WidgetHarness::new();
        let item = TreeItem {
            id: "i1".into(),
            label: Label::data("Item").to_string(),
            description: None,
            icon_id: None,
            selected: false,
            highlighted: false,
            default_open: false,
            dimmed: false,
            event: Some(action()),
            hover_event: None,
            unhover_event: None,
            actions: vec![],
            draggable: false,
            drag_data: StdHashMap::new(),
            control: None,
            children: vec![],
        };
        let node = WidgetNode::<ActionDescriptor>::Tree { sections: vec![TreeSection { id: "s".into(), label: None, default_open: true, items: vec![item] }], selected_ids: vec!["i1".into()], highlighted_ids: vec![], selection_change: None };
        render_widget(&node, VIEWPORT, &mut h.ctx());
        let hit = h.input.hit_targets.iter().find(|t| t.control_id.as_deref() == Some("tree.label.i1")).expect("tree item label must register a hit target");
        assert_eq!(hit.event, Some(action()));
    }

    #[test]
    fn render_scroll_region_clamps_stale_offset_to_new_max_scroll() {
        let mut h = WidgetHarness::new();
        h.scroll_offsets.insert("scroll".into(), 500.0);
        let bounds = Rect::new(0.0, 0.0, 100.0, 100.0);
        {
            let mut ctx = h.ctx();
            render_scroll_region("scroll", bounds, 150.0, &mut ctx, |_content, _ctx| {});
        }
        assert_eq!(h.scroll_offsets.get("scroll"), Some(&50.0), "offset must clamp to max_scroll (content_height - bounds.h) even if a stale value was larger");
    }

    #[test]
    fn render_scroll_region_registers_a_scroll_region_hit_target() {
        let mut h = WidgetHarness::new();
        let bounds = Rect::new(0.0, 0.0, 100.0, 100.0);
        {
            let mut ctx = h.ctx();
            render_scroll_region("myscroll", bounds, 400.0, &mut ctx, |_content, _ctx| {});
        }
        assert!(h.input.hit_targets.iter().any(|t| t.control_id.as_deref() == Some("myscroll")));
    }

    #[test]
    fn draw_text_on_emits_one_glyph_instance_per_character() {
        let mut draw = DrawList::default();
        let mut atlas = FontAtlas::builtin();
        draw_text_on(&mut draw, &mut atlas, "abc", 0.0, 0.0, 16.0, Theme::default().text);
        let total: usize = draw.layers.iter().map(|l| l.ui_instances.len()).sum();
        assert_eq!(total, 3);
    }

    #[test]
    fn draw_text_overlay_on_writes_to_the_overlay_channel_not_the_main_one() {
        let mut draw = DrawList::default();
        let mut atlas = FontAtlas::builtin();
        draw_text_overlay_on(&mut draw, &mut atlas, "hi", 0.0, 0.0, 16.0, Theme::default().text);
        let main: usize = draw.layers.iter().map(|l| l.ui_instances.len()).sum();
        let overlay: usize = draw.layers.iter().map(|l| l.overlay_ui_instances.len()).sum();
        assert_eq!(main, 0, "overlay glyphs must not land in the main ui_instances channel");
        assert_eq!(overlay, 2, "one overlay glyph instance per character");
    }
    //#endregion 🧩️WidgetsInternalsTests
}
// #endregion engine
