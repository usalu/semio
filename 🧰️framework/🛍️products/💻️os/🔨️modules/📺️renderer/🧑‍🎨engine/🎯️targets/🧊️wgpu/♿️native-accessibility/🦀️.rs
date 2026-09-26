//! ♿️ The native wgpu shell's platform accessibility bridge: the accessibility projection every presented frame publishes —
//! the chrome's `shell.chrome` window and every visible document window, the SAME projection the browser build's DOM mirror
//! renders (`🗣️Interpreter`'s `build_accessibility_dump`) — becomes one platform tree (macOS NSAccessibility, Windows UI
//! Automation, Linux AT-SPI), and every assistive-technology action comes back as the shell's own
//! `DispatchEvent::Accessibility`, admitted by the same generation + key rule a mirror event is
//! (`ShellState::handle_accessibility_event`).
//!
//! 🧱️ AccessKit stays behind this module (AGENTS: external libraries behind an interface): no signature outside it names an
//! `accesskit` type. The projection is a pure function over the publication, replayed by the language-agnostic fixture
//! `🧑‍🎨engine/🧫️fixtures/♿️native-accessibility-tree` with AccessKit's own consumer tree as the third-party oracle
//! (ticket 26/09/23 slice WG10).

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Mutex, OnceLock, PoisonError};

/// 🪟️ The window id the shell chrome announces under (`🗣️Interpreter`'s `SHELL_CHROME_ACCESSIBILITY_WINDOW_ID`).
const CHROME_WINDOW_ID: &str = "shell.chrome";

/// 🌳️ The platform root's node id; every other id is derived from its window and key.
const ROOT_NODE: accesskit::NodeId = accesskit::NodeId(1);

/// 🔑️ The domain every derived node id hashes under, so an id never depends on anything but its window and key.
const NODE_ID_DOMAIN: &[u8] = b"semio.native-accessibility.node.v1\0";

/// ♿️ One window's accessibility projection as the shell publishes it.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct NativeAccessibilityWindow {
    pub window_id: String,
    pub window_generation: u64,
    pub nodes: Vec<ui_contract::AccessibilityProjectionNode>,
}

/// ♿️ What one presented frame publishes: the application's name and every announced window, in announcement order.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct NativeAccessibilityPublication {
    pub title: String,
    pub windows: Vec<NativeAccessibilityWindow>,
}

struct PublishedAccessibility {
    version: u64,
    publication: Arc<NativeAccessibilityPublication>,
}

fn published() -> &'static Mutex<PublishedAccessibility> {
    static PUBLISHED: OnceLock<Mutex<PublishedAccessibility>> = OnceLock::new();
    PUBLISHED.get_or_init(|| Mutex::new(PublishedAccessibility { version: 0, publication: Arc::new(NativeAccessibilityPublication::default()) }))
}

/// 📣️ Publishes one presented frame's projection; the platform tree follows only when it changed.
pub(crate) fn publish_native_accessibility(publication: NativeAccessibilityPublication) {
    let mut published = published().lock().unwrap_or_else(PoisonError::into_inner);
    if *published.publication != publication {
        published.version = published.version.wrapping_add(1);
        published.publication = Arc::new(publication);
    }
}

fn published_snapshot() -> (u64, Arc<NativeAccessibilityPublication>) {
    let published = published().lock().unwrap_or_else(PoisonError::into_inner);
    (published.version, published.publication.clone())
}

/// 🌳️ One publication as a platform tree, with the shell address behind every node an assistive technology can act on.
#[derive(Clone, Debug)]
pub(crate) struct NativeAccessibilityTree {
    update: accesskit::TreeUpdate,
    addresses: HashMap<accesskit::NodeId, ui_render::AccessibilityTarget>,
}

/// 🔑️ The stable platform id of one window (`key: None`) or one node of it: FNV-1a over the domain, the window and the key.
fn derived_node_id(window_id: &str, key: Option<&str>) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    let separator = [key.map_or(0u8, |_| 1u8)];
    for byte in NODE_ID_DOMAIN.iter().chain(window_id.as_bytes()).chain(separator.iter()).chain(key.unwrap_or_default().as_bytes()) {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0100_0000_01b3);
    }
    hash
}

/// 🔑️ A derived id no other node of this tree holds yet — the reserved root and a hash collision step to the next free value.
fn unique_node_id(used: &mut HashSet<u64>, derived: u64) -> accesskit::NodeId {
    let mut candidate = derived;
    while candidate <= ROOT_NODE.0 || !used.insert(candidate) {
        candidate = candidate.wrapping_add(1);
    }
    accesskit::NodeId(candidate)
}

/// 🎭️ The platform role of one projected role — the closed vocabulary `ui_contract::accessibility_role` and the shell chrome
/// announce, mapped the way Chromium maps the same ARIA roles; a role outside it is the platform's `Unknown`.
fn platform_role(role: &str, multiline: bool) -> accesskit::Role {
    use accesskit::Role;
    match role {
        "application" => Role::Application,
        "button" => Role::Button,
        "checkbox" => Role::CheckBox,
        "combobox" => Role::ComboBox,
        "form" => Role::Form,
        "grid" => Role::Grid,
        "group" => Role::Group,
        "img" => Role::Image,
        "list" => Role::List,
        "menuitem" => Role::MenuItem,
        "option" => Role::ListBoxOption,
        "paragraph" => Role::Paragraph,
        "progressbar" => Role::ProgressIndicator,
        "radiogroup" => Role::RadioGroup,
        "region" => Role::Region,
        "row" => Role::Row,
        "separator" => Role::Splitter,
        "slider" => Role::Slider,
        "spinbutton" => Role::SpinButton,
        "status" => Role::Status,
        "switch" => Role::Switch,
        "textbox" if multiline => Role::MultilineTextInput,
        "textbox" => Role::TextInput,
        "toolbar" => Role::Toolbar,
        "tree" => Role::Tree,
        "treeitem" => Role::TreeItem,
        _ => Role::Unknown,
    }
}

/// 🧩️ One projected node as a platform node: role, name, state and the actions its flags admit. `author_id` is
/// `<window>/<key>`, the id an assistive-technology tester (UI Automation's AutomationId) reads.
fn platform_node(window_id: &str, projected: &ui_contract::AccessibilityProjectionNode, resolve: &impl Fn(&str) -> accesskit::NodeId) -> accesskit::Node {
    let mut node = accesskit::Node::new(platform_role(&projected.role, projected.multiline));
    node.set_author_id(format!("{window_id}/{}", projected.key));
    if let Some(label) = projected.label.as_deref().filter(|label| !label.is_empty()) {
        node.set_label(label);
    }
    if let Some(description) = projected.description.as_deref().filter(|description| !description.is_empty()) {
        node.set_description(description);
    }
    if let Some(shortcut) = projected.shortcut.as_deref().filter(|shortcut| !shortcut.is_empty()) {
        node.set_keyboard_shortcut(shortcut);
    }
    if projected.hidden {
        node.set_hidden();
    }
    if projected.disabled {
        node.set_disabled();
    }
    if projected.focusable && !projected.disabled {
        node.add_action(accesskit::Action::Focus);
    }
    if projected.actionable && !projected.disabled {
        node.add_action(accesskit::Action::Click);
    }
    if projected.editable && !projected.disabled {
        node.add_action(accesskit::Action::SetValue);
    } else if projected.role == "textbox" {
        node.set_read_only();
    }
    if let Some(toggled) = projected.checked.or(projected.pressed) {
        node.set_toggled(accesskit::Toggled::from(toggled));
    }
    if let Some(selected) = projected.selected {
        node.set_selected(selected);
    }
    if let Some(expanded) = projected.expanded {
        node.set_expanded(expanded);
    }
    if let Some(level) = projected.level {
        node.set_level(level);
    }
    if let Some(value) = projected.value_now {
        node.set_numeric_value(value);
    }
    if let Some(value) = projected.value_min {
        node.set_min_numeric_value(value);
    }
    if let Some(value) = projected.value_max {
        node.set_max_numeric_value(value);
    }
    if let Some(text) = projected.value_text.as_deref() {
        node.set_value(text);
    }
    if projected.busy {
        node.set_busy();
    }
    match projected.live.as_str() {
        "polite" => node.set_live(accesskit::Live::Polite),
        "assertive" => node.set_live(accesskit::Live::Assertive),
        _ => {}
    }
    if let Some([x, y, width, height]) = projected.rect {
        node.set_bounds(accesskit::Rect { x0: f64::from(x), y0: f64::from(y), x1: f64::from(x + width), y1: f64::from(y + height) });
    }
    if let Some(controls) = projected.controls.as_deref() {
        node.set_controls(vec![resolve(controls)]);
    }
    if let Some(descendant) = projected.active_descendant.as_deref() {
        node.set_active_descendant(resolve(descendant));
    }
    node
}

/// 🌳️ Projects one publication into one full platform tree: a `Window` root named after the application, one pane per
/// announced window (the chrome is a `Group`), and each window's flat, depth-ordered projection rebuilt into nesting. The
/// focused node is the platform focus; a publication with none focuses the root.
pub(crate) fn native_accessibility_tree(publication: &NativeAccessibilityPublication) -> NativeAccessibilityTree {
    let mut used = HashSet::from([ROOT_NODE.0]);
    let mut nodes = Vec::new();
    let mut addresses = HashMap::new();
    let mut root_children = Vec::new();
    let mut focus = ROOT_NODE;
    for window in &publication.windows {
        let window_node = unique_node_id(&mut used, derived_node_id(&window.window_id, None));
        let ids: Vec<accesskit::NodeId> = window.nodes.iter().map(|projected| unique_node_id(&mut used, derived_node_id(&window.window_id, Some(&projected.key)))).collect();
        let by_key: HashMap<&str, accesskit::NodeId> = window.nodes.iter().zip(&ids).map(|(projected, id)| (projected.key.as_str(), *id)).collect();
        let resolve = |key: &str| by_key.get(key).copied().unwrap_or_else(|| accesskit::NodeId(derived_node_id(&window.window_id, Some(key))));
        let mut built: Vec<(accesskit::NodeId, accesskit::Node)> = Vec::with_capacity(window.nodes.len());
        let mut window_children = Vec::new();
        let mut ancestors: Vec<(usize, usize)> = Vec::new();
        for (projected, id) in window.nodes.iter().zip(&ids) {
            while ancestors.last().is_some_and(|(depth, _)| *depth >= projected.depth) {
                ancestors.pop();
            }
            match ancestors.last() {
                Some((_, parent)) => built[*parent].1.push_child(*id),
                None => window_children.push(*id),
            }
            if projected.focused {
                focus = *id;
            }
            addresses.insert(*id, ui_render::AccessibilityTarget { window_id: window.window_id.clone(), window_generation: window.window_generation, node_id: projected.node_id, node_key: projected.key.clone() });
            built.push((*id, platform_node(&window.window_id, projected, &resolve)));
            ancestors.push((projected.depth, built.len() - 1));
        }
        let mut pane = accesskit::Node::new(if window.window_id == CHROME_WINDOW_ID { accesskit::Role::Group } else { accesskit::Role::Pane });
        pane.set_author_id(window.window_id.clone());
        pane.set_children(window_children);
        nodes.push((window_node, pane));
        nodes.extend(built);
        root_children.push(window_node);
    }
    let mut root = accesskit::Node::new(accesskit::Role::Window);
    root.set_label(publication.title.as_str());
    root.set_children(root_children);
    nodes.push((ROOT_NODE, root));
    let tree = accesskit::TreeInfo { root: ROOT_NODE, toolkit_name: Some("semio".into()), toolkit_version: Some(env!("CARGO_PKG_VERSION").into()) };
    NativeAccessibilityTree { update: accesskit::TreeUpdate { nodes, tree: Some(tree), tree_id: accesskit::TreeId::ROOT, focus }, addresses }
}

/// 🎯️ The shell event one assistive-technology action request is: `Focus`, `Blur`, `Click` (activate) and `SetValue`
/// on a node this tree addresses. Anything else — another action, a node the tree does not know — answers nothing.
fn action_dispatch(tree: &NativeAccessibilityTree, request: &accesskit::ActionRequest) -> Option<ui_render::DispatchEvent> {
    let target = tree.addresses.get(&request.target_node)?.clone();
    let event = match (request.action, request.data.as_ref()) {
        (accesskit::Action::Focus, _) => ui_render::AccessibilityEvent::Focus,
        (accesskit::Action::Blur, _) => ui_render::AccessibilityEvent::Blur,
        (accesskit::Action::Click, _) => ui_render::AccessibilityEvent::Activate,
        (accesskit::Action::SetValue, Some(accesskit::ActionData::Value(value))) => ui_render::AccessibilityEvent::Value(value.to_string()),
        _ => return None,
    };
    Some(ui_render::DispatchEvent::Accessibility { target, event })
}

/// 🤝️ What the platform handlers and the event loop share: the latest tree and the action requests not yet dispatched.
#[derive(Default)]
struct BridgeShared {
    tree: Option<NativeAccessibilityTree>,
    actions: VecDeque<accesskit::ActionRequest>,
}

/// 🌱️ Answers the platform's first request for the tree synchronously with the latest publication.
struct BridgeActivation {
    shared: Arc<Mutex<BridgeShared>>,
}

impl accesskit::ActivationHandler for BridgeActivation {
    fn request_initial_tree(&mut self) -> Option<accesskit::TreeUpdate> {
        let (_, publication) = published_snapshot();
        let tree = native_accessibility_tree(&publication);
        let update = tree.update.clone();
        self.shared.lock().unwrap_or_else(PoisonError::into_inner).tree = Some(tree);
        Some(update)
    }
}

/// 📨️ Queues one action request and wakes the event loop, which dispatches it on the UI thread.
struct BridgeActions {
    shared: Arc<Mutex<BridgeShared>>,
    wake: Arc<dyn Fn() + Send + Sync>,
}

impl accesskit::ActionHandler for BridgeActions {
    fn do_action(&mut self, request: accesskit::ActionRequest) {
        self.shared.lock().unwrap_or_else(PoisonError::into_inner).actions.push_back(request);
        (self.wake)();
    }
}

struct BridgeDeactivation;

impl accesskit::DeactivationHandler for BridgeDeactivation {
    fn deactivate_accessibility(&mut self) {}
}

/// ♿️ The one platform accessibility adapter of the native shell's window. Created before the window is first shown.
pub(crate) struct NativeAccessibilityBridge {
    adapter: accesskit_winit::Adapter,
    shared: Arc<Mutex<BridgeShared>>,
    version: u64,
}

impl NativeAccessibilityBridge {
    /// ♿️ Attaches the platform adapter to `window`, which must not have been shown yet; `wake` asks the event loop for a
    /// turn when an assistive technology acts.
    pub(crate) fn new(event_loop: &winit::event_loop::ActiveEventLoop, window: &winit::window::Window, wake: Arc<dyn Fn() + Send + Sync>) -> Self {
        let shared = Arc::new(Mutex::new(BridgeShared::default()));
        let adapter = accesskit_winit::Adapter::with_direct_handlers(event_loop, window, BridgeActivation { shared: shared.clone() }, BridgeActions { shared: shared.clone(), wake }, BridgeDeactivation);
        Self { adapter, shared, version: 0 }
    }

    /// 🪟️ Lets the adapter see one window event before the shell handles it (focus, resize, activation).
    pub(crate) fn process_window_event(&mut self, window: &winit::window::Window, event: &winit::event::WindowEvent) {
        self.adapter.process_event(window, event);
    }

    /// 🔄️ Pushes the latest publication to the platform when it changed since the last push.
    pub(crate) fn refresh(&mut self) {
        let (version, publication) = published_snapshot();
        if version == self.version {
            return;
        }
        self.version = version;
        let tree = native_accessibility_tree(&publication);
        let update = tree.update.clone();
        self.shared.lock().unwrap_or_else(PoisonError::into_inner).tree = Some(tree);
        self.adapter.update_if_active(|| update);
    }

    /// 📨️ Every queued assistive-technology action as the shell event it is, oldest first.
    pub(crate) fn take_dispatch_events(&mut self) -> Vec<ui_render::DispatchEvent> {
        let mut shared = self.shared.lock().unwrap_or_else(PoisonError::into_inner);
        let BridgeShared { tree, actions } = &mut *shared;
        let Some(tree) = tree.as_ref() else {
            actions.clear();
            return Vec::new();
        };
        actions.drain(..).filter_map(|request| action_dispatch(tree, &request)).collect()
    }
}

#[cfg(test)]
#[path = "../../../🧪️tests/♿️native-accessibility/🦀️.rs"]
mod tests;
