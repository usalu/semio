//! 📱️ Shell hooks: the domain-neutral seam through which a plugin chooses its layout from a device signal, sees the keyboard before the shell defaults, asks for a bottom sheet and names what a screen reader reads. Schema: `🧬️schema`. See [`super`].

use super::*;
use semio_ui_contract::{AccessibilityProjectionNode, BuiltNode, Buildable, HasBase, HasChildren, Label, UiText};
use std::sync::Arc;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

#[path = "🎯️grip-walk/🦀️.rs"]
mod grip_walk;
pub use grip_walk::*;

/// 📡️ What the host knows about the device: the window width in logical pixels and whether the last input was touch.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShellDeviceSignal {
    pub width_px: u32,
    pub touch: bool,
}

/// 🎛️ The accessibility-relevant preferences of the platform the host runs on (forced colours or high contrast, more contrast, dark scheme, reduced motion): read by the host from the OS (Windows, macOS, Linux) or the web media queries, handed to the plugin through [`PluginShellHooks::platform_prefs_changed`] whenever one changes. Every field is a fact the platform reported; a host that cannot tell reports `false`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShellPlatformPrefs {
    pub forced_colors: bool,
    pub prefers_contrast_more: bool,
    pub prefers_dark: bool,
    pub prefers_reduced_motion: bool,
}

/// 🔔️ Why the shell asks a plugin for its layout.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ShellHookCause {
    Start,
    Resize,
    InputModality,
    Selection,
}

/// 📨️ The question the shell asks: the app, the device, the cause, the plugin's own stored preferences as the text it saved them in, whether something is selected and the language.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShellLayoutRequest {
    pub app_id: String,
    pub signal: ShellDeviceSignal,
    pub cause: ShellHookCause,
    pub prefs: Option<String>,
    pub has_selection: bool,
    pub lang: String,
}

/// 🪟️ How far a bottom sheet is open.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ShellSheetState {
    Collapsed,
    Half,
    Full,
}

/// 🪟️ A bottom sheet a plugin asks for: its announced label, openness, height as a fraction of the screen and the smallest touch target of its handle.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShellBottomSheet {
    pub label: String,
    pub state: ShellSheetState,
    pub height: f64,
    pub min_handle_px: u32,
}

/// 🧭️ The answer: the id of the named layout, the layout itself and the optional bottom sheet.
#[derive(Clone, Debug, PartialEq)]
pub struct ShellLayoutChoice {
    pub layout_id: String,
    pub layout: WindowLayout,
    pub sheet: Option<ShellBottomSheet>,
}

/// ⌨️ A key press as the shell sees it, before any default: the window scope it happens in, the key name (`ArrowLeft`, `Tab`, `z`) and the modifiers.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShellKeyEvent {
    pub scope: String,
    pub key: String,
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub meta: bool,
}

/// ⌨️ What a plugin did with a key: nothing, took it, asks the shell to run a host action by id, replaces an interaction selection, or raises a framework verb (`selectAll`, `clearSelection`, `focusWindow`, ...) with text arguments.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ShellKeyOutcome {
    Ignored,
    Consumed,
    Action { action: String },
    Select {
        #[serde(rename = "domainId")]
        domain_id: String,
        targets: Vec<ShellSelectTarget>,
        merge: String,
    },
    Verb {
        action: String,
        args: BTreeMap<String, String>,
    },
}

/// 🎯️ One address a plugin asks the shell to select in an interaction domain: the granularity and the id inside it.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShellSelectTarget {
    pub granularity: String,
    pub id: String,
}

impl ShellKeyOutcome {
    /// 🎯️ Replaces the selection of `domain_id` with `targets`: a view-state write on the ephemeral local interaction lane, never a document edit and never an undo step.
    pub fn select(domain_id: impl Into<String>, targets: Vec<ShellSelectTarget>) -> Self {
        Self::Select { domain_id: domain_id.into(), targets, merge: "replace".to_owned() }
    }

    /// 🕹️ Raises the framework verb `action` (an `InteractionVerb` id such as `selectAll`, or the shell-intercepted `focusWindow`) with `args`, the same dispatch a palette or touch entry makes, so the chord and the entry cannot differ.
    pub fn verb(action: impl Into<String>, args: impl IntoIterator<Item = (String, String)>) -> Self {
        Self::Verb { action: action.into(), args: args.into_iter().collect() }
    }
}

/// 🖐️ How the touch lane of a surface behaves on a device, asked of the plugin per device signal: whether a one-finger drag orbits the camera (devices where a drag never edits a handle), whether a held finger is an additive press, and the smallest hit edge in logical pixels a target of this device needs (0 keeps the surface's own).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShellTouchProfile {
    pub one_finger_orbit: bool,
    pub long_press: bool,
    pub hit_px: u32,
}

impl Default for ShellTouchProfile {
    fn default() -> Self {
        Self { one_finger_orbit: false, long_press: true, hit_px: 0 }
    }
}

/// 🖐️ A touch gesture the plugin answers: a long press (asked for at the deadline while the finger is still down), a tap on the surface or a double tap.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ShellTouchGesture {
    LongPress,
    Tap,
    DoubleTap,
}

/// 🖐️ A recognised touch gesture as the shell hands it to the plugin: the window scope it happened in, the gesture, the point in surface pixels and the window width.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShellTouchEvent {
    pub scope: String,
    pub gesture: ShellTouchGesture,
    pub x: f64,
    pub y: f64,
    pub width_px: u32,
}

/// 🏷️ One thing a screen reader reads, named by the plugin: a role of the shared role vocabulary, a name and the optional state around it.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShellAccessibleNode {
    pub key: String,
    pub role: String,
    pub name: String,
    pub description: Option<String>,
    pub role_description: Option<String>,
    pub value_text: Option<String>,
    pub position: Option<usize>,
    pub set_size: Option<usize>,
    pub selected: Option<bool>,
    pub depth: usize,
}

impl ShellAccessibleNode {
    /// ♿️ The node as the accessibility projection every platform tree is built from; `node_id` must be unique among the nodes of its window.
    pub fn projection(&self, node_id: u64) -> AccessibilityProjectionNode {
        AccessibilityProjectionNode {
            node_id,
            key: self.key.clone(),
            role: self.role.clone(),
            depth: self.depth,
            label: Some(self.name.clone()).filter(|name| !name.is_empty()),
            description: self.description.clone().or_else(|| self.role_description.clone()),
            live: "off".to_owned(),
            focusable: self.role == "option" || self.role == "row" || self.role == "slider",
            selected: self.selected,
            value_text: self.value_text.clone(),
            pos_in_set: self.position,
            set_size: self.set_size,
            ..AccessibilityProjectionNode::default()
        }
    }
}

/// ♿️ The projection of a plugin's nodes in order, numbered from `first_node_id`.
pub fn accessible_projection(nodes: &[ShellAccessibleNode], first_node_id: u64) -> Vec<AccessibilityProjectionNode> {
    nodes.iter().zip(first_node_id..).map(|(node, id)| node.projection(id)).collect()
}

/// 🔌️ What a plugin implements to take part in the shell: every method has a neutral default so a plugin overrides only what it needs.
pub trait PluginShellHooks: Send + Sync {
    /// 📐️ The layout for a device, or `None` to keep the app's authored default.
    fn choose_layout(&self, request: &ShellLayoutRequest) -> Option<ShellLayoutChoice> {
        let _ = request;
        None
    }

    /// 🗄️ The key of the plugin's local-only layout preferences in the shell's field store; the shell reads it once per opened app and hands the text to [`PluginShellHooks::choose_layout`] as [`ShellLayoutRequest::prefs`].
    fn prefs_key(&self) -> Option<String> {
        None
    }

    /// ⌨️ A key before the shell's own defaults.
    fn handle_key(&self, event: &ShellKeyEvent) -> ShellKeyOutcome {
        let _ = event;
        ShellKeyOutcome::Ignored
    }

    /// ♿️ The nodes a screen reader reads for a scope, in reading order.
    fn accessible_nodes(&self, scope: &str, lang: &str) -> Vec<ShellAccessibleNode> {
        let _ = (scope, lang);
        Vec::new()
    }

    /// 🖐️ How the touch lane behaves on this device; the neutral default passes a one-finger drag through and treats a held finger as an additive press.
    fn touch_profile(&self, signal: &ShellDeviceSignal) -> ShellTouchProfile {
        let _ = signal;
        ShellTouchProfile::default()
    }

    /// 🖐️ A recognised touch gesture, answered like a key: nothing, an action of the app, a selection or a framework verb.
    fn handle_touch(&self, event: &ShellTouchEvent) -> ShellKeyOutcome {
        let _ = event;
        ShellKeyOutcome::Ignored
    }

    /// 🎛️ The platform preferences changed (and once at the start): the plugin keeps them for its next paint.
    fn platform_prefs_changed(&self, prefs: &ShellPlatformPrefs) {
        let _ = prefs;
    }
}

static REGISTERED: std::sync::Mutex<BTreeMap<String, Arc<dyn PluginShellHooks>>> = std::sync::Mutex::new(BTreeMap::new());

/// 🔌️ Registers the hooks of a plugin process-wide, the registration path of the plugin builder (`PluginBuilder::shell_hooks`); every shell in the process consults it after its own registry.
pub fn register_shell_hooks(plugin_id: impl Into<String>, hooks: Arc<dyn PluginShellHooks>) {
    REGISTERED.lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(plugin_id.into(), hooks);
}

/// 🗂️ The hooks of the loaded plugins by plugin id.
#[derive(Clone, Default)]
pub struct ShellHookRegistry {
    hooks: BTreeMap<String, Arc<dyn PluginShellHooks>>,
}

impl ShellHookRegistry {
    /// ➕️ Installs the hooks of a plugin, replacing earlier ones.
    pub fn install(&mut self, plugin_id: impl Into<String>, hooks: Arc<dyn PluginShellHooks>) {
        self.hooks.insert(plugin_id.into(), hooks);
    }

    fn hooks_of(&self, plugin_id: &str) -> Option<Arc<dyn PluginShellHooks>> {
        self.hooks.get(plugin_id).cloned().or_else(|| REGISTERED.lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(plugin_id).cloned())
    }

    /// 📐️ The layout the plugin chooses; `None` when it has no hooks or keeps the default.
    pub fn choose_layout(&self, plugin_id: &str, request: &ShellLayoutRequest) -> Option<ShellLayoutChoice> {
        self.hooks_of(plugin_id)?.choose_layout(request)
    }

    /// 🗄️ The storage key of the plugin's layout preferences; `None` when it has no hooks or keeps none.
    pub fn prefs_key(&self, plugin_id: &str) -> Option<String> {
        self.hooks_of(plugin_id)?.prefs_key()
    }

    /// ⌨️ What the plugin does with a key; a plugin without hooks ignores it.
    pub fn handle_key(&self, plugin_id: &str, event: &ShellKeyEvent) -> ShellKeyOutcome {
        self.hooks_of(plugin_id).map_or(ShellKeyOutcome::Ignored, |hooks| hooks.handle_key(event))
    }

    /// 🎛️ Tells the plugin the platform preferences; a plugin without hooks ignores them.
    pub fn platform_prefs_changed(&self, plugin_id: &str, prefs: &ShellPlatformPrefs) {
        if let Some(hooks) = self.hooks_of(plugin_id) {
            hooks.platform_prefs_changed(prefs);
        }
    }

    /// ♿️ The nodes the plugin names for a scope.
    pub fn accessible_nodes(&self, plugin_id: &str, scope: &str, lang: &str) -> Vec<ShellAccessibleNode> {
        self.hooks_of(plugin_id).map(|hooks| hooks.accessible_nodes(scope, lang)).unwrap_or_default()
    }

    /// 🖐️ The touch behaviour the plugin asks for on a device; a plugin without hooks gets the neutral default.
    pub fn touch_profile(&self, plugin_id: &str, signal: &ShellDeviceSignal) -> ShellTouchProfile {
        self.hooks_of(plugin_id).map_or_else(ShellTouchProfile::default, |hooks| hooks.touch_profile(signal))
    }

    /// 🖐️ What the plugin does with a recognised touch gesture; a plugin without hooks ignores it.
    pub fn handle_touch(&self, plugin_id: &str, event: &ShellTouchEvent) -> ShellKeyOutcome {
        self.hooks_of(plugin_id).map_or(ShellKeyOutcome::Ignored, |hooks| hooks.handle_touch(event))
    }
}

impl std::fmt::Debug for ShellHookRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.hooks.keys()).finish()
    }
}

/// 👁️ Decides when the shell asks: remembers the last device signal and selection and names what changed.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ShellHookTracker {
    last: Option<(ShellDeviceSignal, bool)>,
}

impl ShellHookTracker {
    /// 🔔️ The cause to ask for, or `None` when nothing changed: the first observation is a start, then a width change is a resize, a touch change an input-modality change and a selection change a selection change; a resize wins when several change at once.
    pub fn observe(&mut self, signal: ShellDeviceSignal, has_selection: bool) -> Option<ShellHookCause> {
        let cause = match self.last {
            None => Some(ShellHookCause::Start),
            Some((before, _)) if before.width_px != signal.width_px => Some(ShellHookCause::Resize),
            Some((before, _)) if before.touch != signal.touch => Some(ShellHookCause::InputModality),
            Some((_, selected)) if selected != has_selection => Some(ShellHookCause::Selection),
            Some(_) => None,
        };
        self.last = Some((signal, has_selection));
        cause
    }

    /// ♻️ Forgets the last observation, so the next one is a start again; the shell does this when another app opens.
    pub fn reset(&mut self) {
        self.last = None;
    }
}

/// 🧭️ What the shell keeps per window: the hooks, the tracker, the input modality and the bottom sheet of the current choice.
#[derive(Clone, Debug, Default)]
pub struct ShellHookDriver {
    pub registry: ShellHookRegistry,
    pub tracker: ShellHookTracker,
    pub touch: bool,
    pub width_px: u32,
    pub has_selection: bool,
    pub sheet: Option<ShellBottomSheet>,
    pub prefs_text: Option<String>,
    platform: Option<ShellPlatformPrefs>,
    app_id: Option<String>,
    prefs_for: Option<String>,
}

impl ShellHookDriver {
    /// 👆️ Records that the last input was touch (`true`) or pointer and keyboard (`false`).
    pub fn note_touch(&mut self, touch: bool) {
        self.touch = touch;
    }

    /// 🎛️ Records the platform preferences the host reports and tells the plugin when they differ from the last report (the first report always counts); `true` when the plugin was told, so the caller repaints.
    pub fn note_platform(&mut self, plugin_id: &str, prefs: ShellPlatformPrefs) -> bool {
        if self.platform == Some(prefs) {
            return false;
        }
        self.platform = Some(prefs);
        self.registry.platform_prefs_changed(plugin_id, &prefs);
        true
    }

    /// 🎯️ Records whether something is selected, the shell's feed for the sheet of a small screen.
    pub fn note_selection(&mut self, has_selection: bool) {
        self.has_selection = has_selection;
    }

    /// 🗄️ The storage key to read the plugin's layout preferences from, once per opened app: `Some(key)` the first time it is asked for an app, then `None` until another app opens; the shell answers with [`ShellHookDriver::supply_prefs`].
    pub fn prefs_to_load(&mut self, plugin_id: &str, app_id: &str) -> Option<String> {
        if self.prefs_for.as_deref() == Some(app_id) {
            return None;
        }
        self.prefs_for = Some(app_id.to_owned());
        self.prefs_text = None;
        self.registry.prefs_key(plugin_id)
    }

    /// 💾 The stored text of the plugin's layout preferences (or none when nothing was stored); the next layout request carries it.
    pub fn supply_prefs(&mut self, text: Option<String>) {
        self.prefs_text = text;
        self.tracker.reset();
    }

    /// 📐️ Called by the shell on start, on every frame's size and on input and selection changes; asks the plugin only when something changed and keeps the sheet of its answer; another app starts again with a start.
    pub fn observe(&mut self, plugin_id: &str, app_id: &str, width_px: u32, prefs: Option<&str>, lang: &str) -> Option<ShellLayoutChoice> {
        if self.app_id.as_deref() != Some(app_id) {
            self.app_id = Some(app_id.to_owned());
            self.tracker.reset();
            self.platform = None;
        }
        self.width_px = width_px;
        let signal = ShellDeviceSignal { width_px, touch: self.touch };
        let cause = self.tracker.observe(signal, self.has_selection)?;
        let request = ShellLayoutRequest { app_id: app_id.to_owned(), signal, cause, prefs: prefs.map(str::to_owned).or_else(|| self.prefs_text.clone()), has_selection: self.has_selection, lang: lang.to_owned() };
        let choice = self.registry.choose_layout(plugin_id, &request);
        self.sheet = choice.as_ref().and_then(|choice| choice.sheet.clone());
        choice
    }

    /// ⌨️ Offers a key to the plugin before the shell's defaults.
    pub fn handle_key(&self, plugin_id: &str, event: &ShellKeyEvent) -> ShellKeyOutcome {
        self.registry.handle_key(plugin_id, event)
    }

    /// 🖐️ The touch behaviour the plugin asks for on the device the shell last observed (the window width of the last frame, touch as the modality).
    pub fn touch_profile(&self, plugin_id: &str) -> ShellTouchProfile {
        self.registry.touch_profile(plugin_id, &ShellDeviceSignal { width_px: self.width_px, touch: true })
    }

    /// 🖐️ Offers a recognised touch gesture to the plugin, stamping the window width the shell last observed.
    pub fn handle_touch(&self, plugin_id: &str, scope: &str, gesture: ShellTouchGesture, x: f64, y: f64) -> ShellKeyOutcome {
        self.registry.handle_touch(plugin_id, &ShellTouchEvent { scope: scope.to_owned(), gesture, x, y, width_px: self.width_px })
    }
}

/// 🪟️ The bottom sheet as a node of the UI tree: a container of role `sheet`, announced as a complementary landmark, around the given children; `None` when `id` or the label is refused by the tree's limits.
pub fn bottom_sheet_node(sheet: &ShellBottomSheet, id: &str, children: Vec<BuiltNode>) -> Option<BuiltNode> {
    let label = Label(UiText::try_from_str(&sheet.label)?);
    semio_ui_contract::sheet(label).try_id(id).ok()?.try_children(children).ok()?.try_build().ok()
}
