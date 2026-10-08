//! 🪟️ The dashboard's windows: their bodies, the tiling layout they live in, the tab and title each
//! one shows, focus (the engine's focus is the only source), and keeping every terminal exactly as
//! large as the rectangle its widget occupies.
//!
//! @see 🧰️framework/🔨️modules/🖱️ui/⌨️tui/🖥️chrome/🦀️.rs

use super::labels::{labels, DashboardLabels};
use super::launcher::LauncherState;
use super::panes::Pane;
use super::sessions::Link;
use crate::inventory::{Job, Phase};
use crate::ipc::{ClientMsg, SessionInfo, SessionStatus};
use crate::preferences::keymap::Keymap;
use crate::preferences::{Change, Preferences};
use crate::registry::{self, Registry, TaskLabel};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::PathBuf;
use std::sync::Arc;
use ui_locale::Locale;
use ui_styling::appearance::AppearanceName;
use ui_tui::tui::chrome::{shell, ChromeLabels, ChromeState, FooterState, NavItem, NavbarState, Shell, WindowState};
use ui_tui::tui::engine::Tui;
use ui_tui::tui::geometry::Size;
use ui_tui::tui::layout::{activate_stack_tab, create_default_layout, push_window_to_stack, remove_window, split_window, Constraint, Dimension, Direction, WindowLayout, WindowLayoutChild, WindowLayoutRoot, WindowLayoutWindowNode};
use ui_tui::tui::scene::{Node, NodeContent, NodeId};
use ui_tui::tui::theme::{GlyphSet, Status};
use ui_tui::tui::widget::{Align, InputState, LabelState, ListState, TerminalState, WidgetState};

const SCROLLBACK: usize = 8000;

// #region 🔖️Window
/// 🧩️ What a window shows.
pub(super) enum Body {
    Overview,
    Settings,
    Help,
    Launcher(Box<LauncherState>),
    Output { session: Option<Box<SessionInfo>>, sent: Size },
}

/// 🧱️ Which body to build.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Kind { Overview, Settings, Help, Launcher, Output }

/// 📍️ Where a new window goes among the others.
#[derive(Clone, Copy)]
pub(super) enum Place<'a> { Preference, Split(&'a str), Stack(&'a str) }

/// 🖼️ One window: its chrome, the node tree of its body and the state behind it.
pub(super) struct DashboardWindow {
    pub id: String,
    pub chrome: NodeId,
    pub root: NodeId,
    pub list: NodeId,
    pub input: Option<NodeId>,
    pub caption: Option<NodeId>,
    pub tree: Option<NodeId>,
    pub body: Body,
    pub pane: Pane,
}

impl DashboardWindow {
    /// 🎯️ The node that holds keyboard focus while this window does.
    pub fn focus(&self) -> NodeId {
        match (&self.body, self.tree) { (Body::Launcher(launcher), Some(tree)) if launcher.stage() == super::launcher::Stage::Browse => tree, _ => self.list }
    }

    pub fn is_output(&self) -> bool { matches!(self.body, Body::Output { .. }) }

    pub fn session(&self) -> Option<&SessionInfo> { if let Body::Output { session, .. } = &self.body { session.as_deref() } else { None } }
}

/// 🔗️ What the dashboard says about its daemon connection.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum LinkState { Connecting, Connected, Reconnecting, Stopped, Failed(String) }

/// 📋️ The context menu that is open: its overlay, the window it was opened for and what each row does.
pub(super) struct ContextMenu { pub node: NodeId, pub window: usize, pub actions: Vec<Option<super::controls::Action>> }

/// 🌬️ Something the view asks of the terminal it runs in, collected so tests need no terminal.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum Effect { Copy(String) }
// #endregion 🔖️Window

// #region 🔖️Dashboard
/// 🎛️ The whole native dashboard: preferences, keymap, windows, layout and the daemon link.
pub(super) struct Dashboard {
    pub root: PathBuf,
    pub registry: Option<Arc<Registry>>,
    pub inventory: Option<Job>,
    pub phase: Option<Phase>,
    pub last_progress: u64,
    pub preferences: Preferences,
    pub preference_path: PathBuf,
    pub shared_preferences: bool,
    pub saving_preferences: Option<std::sync::mpsc::Receiver<(Change, std::io::Result<u64>)>>,
    pub keymap: Keymap,
    pub locale: Locale,
    pub light: bool,
    pub layout: WindowLayout,
    pub shell: Shell,
    pub windows: Vec<DashboardWindow>,
    pub next_serial: u32,
    pub armed: bool,
    pub armed_at: Option<std::time::Instant>,
    pub input_mode: bool,
    pub link: Link,
    pub state: LinkState,
    pub notice: Option<String>,
    pub sessions: BTreeMap<String, SessionInfo>,
    pub hidden: HashSet<String>,
    pub shutdown_since: Option<std::time::Instant>,
    pub effects: Vec<Effect>,
    pub quit_requested: bool,
    pub last_focus: std::cell::Cell<usize>,
    pub menu: Option<ContextMenu>,
    pub glyphs: GlyphSet,
    pub waker: Option<ui_tui::tui::backend::Waker>,
}

pub(super) fn node_constraint() -> Constraint { Constraint { width: Dimension::Weight(1), height: Dimension::Weight(1), direction: Direction::Column, ..Default::default() } }

impl Dashboard {
    /// 🌱️ A dashboard with its shell and one task overview, ready to be painted.
    pub fn new(tui: &mut Tui, root: PathBuf, preferences: Preferences, preference_path: PathBuf, shared_preferences: bool) -> Self {
        let locale = preferences.locale();
        let text = labels(locale);
        let navbar = NavbarState { left: vec![NavItem { id: "logo".into(), label: text.nav_brand.as_str().into(), active: true }], center: vec![NavItem { id: "mode".into(), label: text.nav_mode.as_str().into(), active: false }], right: vec![] };
        let footer = FooterState { hints: vec![], status: String::new() };
        let layout = create_default_layout(&["w1".into()], "row", None, Some(&[text.title_tasks.as_str().into()]));
        let shell = shell(&mut tui.scene, navbar, footer, &layout);
        let (keymap, _) = preferences.keymap();
        let light = preferences.appearance == "light";
        let mut dashboard = Self {
            root, registry: None, inventory: None, phase: None, last_progress: 0, preferences, preference_path, shared_preferences, saving_preferences: None, keymap, locale, light, layout, shell, windows: Vec::new(), next_serial: 2, armed: false, armed_at: None, input_mode: false,
            link: Link::default(), state: LinkState::Connecting, notice: None, sessions: BTreeMap::new(), hidden: HashSet::new(), shutdown_since: None, effects: Vec::new(), quit_requested: false, last_focus: std::cell::Cell::new(0), menu: None, glyphs: GlyphSet::Unicode, waker: None,
        };
        let (id, chrome) = dashboard.shell.windows[0].clone();
        let window = dashboard.build_window(tui, id, chrome, Kind::Overview, None);
        dashboard.windows.push(window);
        dashboard.remount(tui);
        tui.set_focus(Some(dashboard.windows[0].focus()));
        dashboard
    }

    pub fn text(&self) -> &'static DashboardLabels { labels(self.locale) }

    pub fn window_index(&self, id: &str) -> Option<usize> { self.windows.iter().position(|window| window.id == id) }

    /// 🔎️ The window whose node tree contains `node`.
    pub fn window_of(&self, tui: &Tui, node: NodeId) -> Option<usize> {
        let mut cursor = Some(node);
        while let Some(current) = cursor {
            if let Some(index) = self.windows.iter().position(|window| window.chrome == current) { return Some(index); }
            cursor = tui.scene.try_node(current).and_then(|found| found.parent());
        }
        None
    }

    /// ⌨️ The window that owns the keyboard: the one holding the engine's focus.
    pub fn focused_index(&self, tui: &Tui) -> usize {
        let found = tui.focus().and_then(|node| self.window_of(tui, node));
        if let Some(index) = found { self.last_focus.set(index); }
        found.unwrap_or_else(|| self.last_focus.get()).min(self.windows.len().saturating_sub(1))
    }

    pub fn focused(&self, tui: &Tui) -> Option<&DashboardWindow> { self.windows.get(self.focused_index(tui)) }

    /// 🎙️ Whether the focused window is a terminal that currently forwards keys to its program.
    pub fn terminal_has_keyboard(&self, tui: &Tui) -> bool { self.input_mode && self.focused(tui).is_some_and(DashboardWindow::is_output) }

    /// 🚨️ Names the first keymap customization that had to be ignored, so nothing is dropped silently.
    pub fn report_keymap_problems(&mut self) {
        if let Some(problem) = self.preferences.keymap().1.first() { self.notice = Some(self.text().prefs_keymap_problem.fill(&[("problem", problem)]).into_string()); }
    }

    pub fn running(&self) -> usize { self.sessions.values().filter(|session| session.status.live()).count() }

    pub fn terminal_node(&self, index: usize) -> Option<NodeId> { self.windows.get(index).filter(|window| window.is_output()).map(|window| window.list) }
}
// #endregion 🔖️Dashboard

// #region 🔖️Bodies
fn add_box(tui: &mut Tui, parent: NodeId) -> NodeId {
    let id = tui.scene.add(parent, Node::new(NodeContent::Box));
    tui.scene.node_mut(id).set_constraint(node_constraint());
    id
}

fn add_widget(tui: &mut Tui, parent: NodeId, widget: WidgetState, height: Dimension) -> NodeId {
    let id = tui.scene.add(parent, Node::new(NodeContent::Widget(widget)));
    tui.scene.node_mut(id).set_constraint(Constraint { width: Dimension::Weight(1), height, ..Default::default() });
    id
}

impl Dashboard {
    /// 🏗️ Builds the body widgets of one window under its chrome.
    pub fn build_window(&mut self, tui: &mut Tui, id: String, chrome: NodeId, kind: Kind, size: Option<Size>) -> DashboardWindow {
        let root = add_box(tui, chrome);
        let list_state = || WidgetState::List(ListState::new(Vec::new()));
        let mut tree_node = None;
        let (list, input, caption, body) = match kind {
            Kind::Output => {
                let mut state = TerminalState::new(size.unwrap_or(Size { width: 80, height: 24 }), SCROLLBACK);
                state.apply_theme(&tui.theme);
                let terminal = add_widget(tui, root, WidgetState::Terminal(state), Dimension::Weight(1));
                (terminal, None, None, Body::Output { session: None, sent: Size::default() })
            }
            Kind::Launcher => {
                let input = add_widget(tui, root, WidgetState::Input(InputState { value: String::new(), cursor: 0, placeholder: String::new() }), Dimension::Cells(1));
                let caption = add_widget(tui, root, WidgetState::Label(LabelState { text: String::new(), align: Align::Left, role: ui_tui::tui::theme::Role::MutedForeground }), Dimension::Cells(1));
                let mut launcher = LauncherState::new(self.preferences.clone(), self.locale);
                if let Some(registry) = &self.registry { launcher.set_registry(registry.clone()); }
                let tree = add_widget(tui, root, WidgetState::Tree(launcher.rebuild_tree(None)), Dimension::Weight(1));
                let list = add_widget(tui, root, list_state(), Dimension::Weight(1));
                tui.scene.node_mut(input).set_visible(false);
                tui.scene.node_mut(list).set_visible(false);
                tree_node = Some(tree);
                (list, Some(input), Some(caption), Body::Launcher(Box::new(launcher)))
            }
            Kind::Overview | Kind::Settings | Kind::Help => (add_widget(tui, root, list_state(), Dimension::Weight(1)), None, None, match kind { Kind::Overview => Body::Overview, Kind::Settings => Body::Settings, _ => Body::Help }),
        };
        DashboardWindow { id, chrome, root, list, input, caption, tree: tree_node, body, pane: Pane::default() }
    }

    /// 🔁️ Replaces the body of an existing window, e.g. the launcher by the terminal of the task it started.
    pub fn set_body(&mut self, tui: &mut Tui, index: usize, kind: Kind, size: Option<Size>) {
        let (id, chrome, root) = { let window = &self.windows[index]; (window.id.clone(), window.chrome, window.root) };
        tui.scene.remove(root);
        let window = self.build_window(tui, id, chrome, kind, size);
        let focus = window.focus();
        self.windows[index] = window;
        self.input_mode = kind == Kind::Output;
        if self.focused_index(tui) == index || tui.focus().is_none_or(|node| !tui.scene.contains(node)) { tui.set_focus(Some(focus)); }
        self.remount(tui);
    }

    fn with_window<R>(tui: &mut Tui, chrome: NodeId, change: impl FnOnce(&mut WindowState) -> R) -> Option<R> {
        match tui.scene.node_mut(chrome).chrome() { Some(ChromeState::Window(window)) => Some(change(window)), _ => None }
    }

    /// ➕️ Creates a window, files it into the layout and gives it a body.
    pub fn create_window(&mut self, tui: &mut Tui, kind: Kind, place: Place<'_>) -> usize {
        let id = format!("w{}", self.next_serial);
        self.next_serial += 1;
        let host = self.windows.get(self.focused_index(tui)).map(|window| window.id.clone());
        match (place, host) {
            (Place::Split(direction), Some(host)) => { split_window(&mut self.layout, &host, direction, &id, None); }
            (Place::Stack(host), _) => { push_window_to_stack(&mut self.layout, host, WindowLayoutWindowNode { window_kind_id: id.clone(), title: None, corner: None }); }
            (Place::Preference, Some(host)) => self.place_window(&host, &id),
            _ => { push_window_to_stack(&mut self.layout, "w1", WindowLayoutWindowNode { window_kind_id: id.clone(), title: None, corner: None }); }
        }
        let chrome = tui.scene.add(self.shell.canvas, Node::new(NodeContent::Chrome(ChromeState::Window(Box::new(WindowState::new(id.clone()).with_stack_tabs(vec![id.clone()], 0))))));
        tui.scene.node_mut(chrome).set_constraint(Constraint { width: Dimension::Weight(1), direction: Direction::Column, padding: [2, 1, 1, 1], gap: 1, ..Default::default() });
        let window = self.build_window(tui, id, chrome, kind, None);
        self.windows.push(window);
        self.windows.len() - 1
    }

    fn place_window(&mut self, host: &str, id: &str) {
        match self.preferences.layout.as_str() {
            "columns" => { split_window(&mut self.layout, host, "row", id, None); }
            "rows" => { split_window(&mut self.layout, host, "column", id, None); }
            _ => { push_window_to_stack(&mut self.layout, host, WindowLayoutWindowNode { window_kind_id: id.into(), title: None, corner: None }); }
        }
    }

    /// 🗑️ Removes a window; true when it was the last one.
    pub fn close_window(&mut self, tui: &mut Tui, index: usize) -> bool {
        if index >= self.windows.len() { return false; }
        let window = self.windows.remove(index);
        if let Some(session) = window.session() { self.hidden.insert(session.session_id.clone()); }
        tui.scene.remove(window.chrome);
        remove_window(&mut self.layout, &window.id);
        if self.layout.zoomed.as_deref() == Some(window.id.as_str()) { self.layout.zoomed = None; }
        if self.windows.is_empty() { return true; }
        let next = index.saturating_sub(1).min(self.windows.len() - 1);
        self.focus_window(tui, next);
        false
    }

    /// 🔝️ Brings a window forward in its stack and gives it the keyboard.
    pub fn focus_window(&mut self, tui: &mut Tui, index: usize) {
        let Some(window) = self.windows.get(index) else { return };
        let (id, node, output) = (window.id.clone(), window.focus(), window.is_output());
        activate_stack_tab(&mut self.layout, &id);
        self.remount(tui);
        tui.set_focus(Some(node));
        self.input_mode = output;
    }

    /// 🔀️ Moves the keyboard to the next or previous window in layout order.
    pub fn cycle_window(&mut self, tui: &mut Tui, delta: isize) {
        if self.windows.len() < 2 { return; }
        let count = self.windows.len() as isize;
        let next = (self.focused_index(tui) as isize + delta).rem_euclid(count) as usize;
        self.focus_window(tui, next);
    }
}
// #endregion 🔖️Bodies

// #region 🔖️Tabs
/// 🗂️ The window ids sharing a stack with `id`, in tab order.
pub(super) fn stack_of(layout: &WindowLayout, id: &str) -> Vec<String> {
    fn find(child: &WindowLayoutChild, id: &str) -> Option<Vec<String>> {
        match child {
            WindowLayoutChild::Stack(stack) => stack.children.iter().any(|window| window.window_kind_id == id).then(|| stack.children.iter().map(|window| window.window_kind_id.clone()).collect()),
            WindowLayoutChild::Axis(axis) => axis.children.iter().find_map(|child| find(child, id)),
        }
    }
    match &layout.root {
        WindowLayoutRoot::Stack(stack) => find(&WindowLayoutChild::Stack(stack.clone()), id),
        WindowLayoutRoot::Axis(axis) => axis.children.iter().find_map(|child| find(child, id)),
    }.unwrap_or_default()
}

fn layout_windows(layout: &mut WindowLayout) -> Vec<&mut WindowLayoutWindowNode> {
    fn child<'a>(node: &'a mut WindowLayoutChild, out: &mut Vec<&'a mut WindowLayoutWindowNode>) {
        match node {
            WindowLayoutChild::Stack(stack) => out.extend(stack.children.iter_mut()),
            WindowLayoutChild::Axis(axis) => axis.children.iter_mut().for_each(|node| child(node, out)),
        }
    }
    let mut out = Vec::new();
    match &mut layout.root {
        WindowLayoutRoot::Stack(stack) => out.extend(stack.children.iter_mut()),
        WindowLayoutRoot::Axis(axis) => axis.children.iter_mut().for_each(|node| child(node, &mut out)),
    }
    out
}

impl Dashboard {
    /// 🚦️ The status a task reports: waiting and stopping wait, a started task runs (ready informs), an exit with
    /// code zero succeeds, other exits fail, a failed start is a fault and an interrupted task warns.
    pub fn status_of(session: &SessionInfo) -> Status {
        match session.status {
            SessionStatus::Pending | SessionStatus::Stopping => Status::Waiting,
            SessionStatus::Running if session.ready_url.is_some() => Status::Info,
            SessionStatus::Running => Status::Running,
            SessionStatus::Exited if session.code == Some(0) => Status::Success,
            SessionStatus::Exited => Status::Failure,
            SessionStatus::Failed => Status::Fault,
            SessionStatus::Interrupted => Status::Warning,
        }
    }

    /// 🧰️ Adapts to what the attached terminal renders: without full Unicode every symbol is plain ASCII.
    pub fn apply_capabilities(&mut self, tui: &mut Tui, unicode_full: bool) {
        self.glyphs = if unicode_full { GlyphSet::Unicode } else { GlyphSet::Ascii };
        tui.theme.set_glyphs(self.glyphs);
        tui.set_width_mode(if unicode_full { ui_tui::tui::text::WidthMode::Cluster } else { ui_tui::tui::text::WidthMode::Scalar });
    }

    fn body_title(&self, window: &DashboardWindow) -> String {
        let text = self.text();
        match &window.body {
            Body::Overview => text.title_tasks.as_str().into(),
            Body::Settings => text.title_settings.as_str().into(),
            Body::Help => text.title_keyboard.as_str().into(),
            Body::Launcher(_) => text.title_commands.as_str().into(),
            Body::Output { session: Some(session), .. } => {
                let title = registry::window_title(&session.command.label, self.locale);
                match session.code.filter(|_| !session.status.live()) { Some(code) => format!("{title} · {}", text.exit_code.fill(&[("code", &code.to_string())]).as_str()), None => title }
            }
            Body::Output { session: None, .. } => text.title_tasks.as_str().into(),
        }
    }

    /// 🏷️ The tab text of every window, computed from the task labels of its stack and never from strings.
    fn tab_labels(&self) -> HashMap<String, String> {
        let mut tabs = HashMap::new();
        for window in &self.windows {
            if tabs.contains_key(&window.id) { continue; }
            let stack = stack_of(&self.layout, &window.id);
            let members: Vec<&DashboardWindow> = stack.iter().filter_map(|id| self.windows.iter().find(|candidate| &candidate.id == id)).collect();
            let mut tasks: Vec<(&DashboardWindow, &SessionInfo)> = members.iter().filter_map(|member| member.session().map(|session| (*member, session))).collect();
            tasks.sort_by_key(|(_, session)| (session.started_ms, session.session_id.clone()));
            let labels: Vec<&TaskLabel> = tasks.iter().map(|(_, session)| &session.command.label).collect();
            let texts = registry::tab_texts(&labels, self.locale, registry::TAB_CELLS.saturating_sub(2));
            for ((member, _), text) in tasks.iter().zip(texts) { tabs.insert(member.id.clone(), text); }
            for member in members { tabs.entry(member.id.clone()).or_insert_with(|| self.body_title(member)); }
        }
        tabs
    }

    /// 🪪️ Writes the tab text of every window into its layout node, which is where the chrome reads it,
    /// and rebuilds the tiled mount.
    pub fn remount(&mut self, tui: &mut Tui) {
        let tabs = self.tab_labels();
        for node in layout_windows(&mut self.layout) { node.title = tabs.get(&node.window_kind_id).cloned(); }
        self.shell.windows = self.windows.iter().map(|window| (window.id.clone(), window.chrome)).collect();
        self.shell.remount(&mut tui.scene, &self.layout);
        for window in &self.windows { self.shell.set_status(&mut tui.scene, &window.id, window.session().map(Self::status_of)); }
    }

    /// 🪧️ The long title of the focused task: verb, owner, subject, qualifier, chosen parameters and exit code.
    pub fn focus_title(&self, tui: &Tui) -> Option<String> {
        let window = self.focused(tui)?;
        window.session()?;
        Some(self.body_title(window))
    }
}
// #endregion 🔖️Tabs

// #region 🔖️Geometry
impl Dashboard {
    /// 📐️ Lays out the scene and makes every terminal exactly as large as the rectangle of its
    /// widget; the daemon is told the sizes that changed. Hidden tabs keep their last size.
    pub fn sync_sizes(&mut self, tui: &mut Tui) -> Vec<ClientMsg> {
        tui.layout();
        let mut messages = Vec::new();
        for window in &mut self.windows {
            let Body::Output { session, sent } = &mut window.body else { continue };
            let rect = tui.scene.rect(window.list);
            if rect.width == 0 || rect.height == 0 { continue; }
            let size = Size { width: rect.width, height: rect.height };
            if let Some(WidgetState::Terminal(terminal)) = tui.scene.node_mut(window.list).widget() { terminal.fit(rect); }
            if *sent != size {
                *sent = size;
                if let Some(session) = session { messages.push(ClientMsg::Resize { session_id: session.session_id.clone(), cols: size.width, rows: size.height }); }
            }
        }
        messages
    }

    /// 🔤️ Gives every window chrome the control names of the current language and the new-tab control; the
    /// engine itself marks the focused window, and nothing is written when nothing differs.
    pub fn sync_chrome(&mut self, tui: &mut Tui) {
        let text = self.text();
        let labels = ChromeLabels { close: text.chrome_close.as_str().into(), maximize: text.chrome_maximize.as_str().into(), restore: text.chrome_restore.as_str().into(), new_tab: text.chrome_new_tab.as_str().into(), previous_tabs: text.chrome_previous_tabs.as_str().into(), next_tabs: text.chrome_next_tabs.as_str().into() };
        for window in &self.windows {
            let current = match &tui.scene.node(window.chrome).content { NodeContent::Chrome(ChromeState::Window(chrome)) => Some(chrome.labels == labels && chrome.new_tab), _ => None };
            if current == Some(false) { Self::with_window(tui, window.chrome, |chrome| { chrome.labels = labels.clone(); chrome.new_tab = true; }); }
        }
    }

    pub fn set_appearance(&mut self, tui: &mut Tui, light: bool) {
        self.light = light;
        tui.set_appearance(if light { AppearanceName::Light } else { AppearanceName::Dark });
    }
}
// #endregion 🔖️Geometry
