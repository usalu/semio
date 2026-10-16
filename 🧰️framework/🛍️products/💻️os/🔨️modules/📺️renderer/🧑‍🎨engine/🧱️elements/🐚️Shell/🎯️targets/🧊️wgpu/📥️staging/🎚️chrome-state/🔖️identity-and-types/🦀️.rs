//! 🎚️ `shell-chrome-state` staging of `identity-and-types`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellTypes
//#region 🔖️ChromeThreadBoundary
pub struct RetainedContentFocus {
    pub node: ui_wgpu::wgpu::NodeId,
    pub key: ui_wgpu::wgpu::NodeKey,
    pub kind: RetainedNodeFocusKind,
}

/// 🏗️ Send-capable inputs, scratch state, and outputs consumed by chrome construction.
#[derive(Default)]
pub struct ShellChromeBuildState {
    pub find_items: ShellFindItems,
    /// 🎯️ Which retained node holds focus per window, `None` for a window whose content is blurred —
    /// the wgpu twin of React's active-root registry (`🧱️elements/🌈️Surface/🟦️.tsx:186-197`, whose
    /// `focusin` listener stores `resolveSurfaceActiveRoot(event.target)`, not a boolean). This used to
    /// be a bare `bool`, which is why `UiCommand::FocusChanged`'s node id was thrown away on arrival
    /// (ticket 26/09/17 packet W2k).
    pub content_focus: HashMap<String, Option<RetainedContentFocus>>,
    pub focused_retained_surface: Option<String>,
    pub tooltip_titles: HashMap<String, String>,
    /// 🚗️ The resolved `UiDriver` axes this chrome presents under — React's `UiDriverProvider` value
    /// (`🧱️elements/🚗️UiDriver/🟦️.tsx`), re-resolved from `driver_id` + `custom_drivers` every time
    /// preferences load or change. It is what makes `tooltips: "none"` (the shipped `compact`
    /// driver) actually suppress a hover tooltip on this target; before ticket 26/09/17 packet W15a
    /// the whole `UiDriver` vocabulary had zero readers anywhere in the wgpu tree, so every chrome
    /// control fired its tooltip under every driver.
    pub driver: ui_wgpu::wgpu::UiDriverChrome,
    pub tooltip_hover: Option<ChromeTooltipHover>,
    pub dialog_stack: Vec<ChromeDialogRequest>,
    pub tour_state: Option<ChromeTourState>,
    pub introduction_host_suppressed: bool,
    pub previous_pointer_down: bool,
    pub clicked_this_frame: bool,
    pub tour_reveal_latch: Option<String>,
    pub element_rects: HashMap<String, ChromeElementRectEntry>,
    pub introduction_seen: HashMap<String, bool>,
    pub introduction_seen_writes: Vec<String>,
    pub tutorial_dispatch_internal: bool,
    pub preferences: ChromePrefsState,
    /// 🧯️ The one live non-fatal notice — see `🧯️TransientNoticeAndAgentOverlays`.
    pub transient_notice: Option<ShellTransientNotice>,
    /// 📣️ The structured guest refusal behind the dispatch-fault string the funnel classifies next — see [`RefusedGuestFault`].
    pub refused_guest_fault: Option<RefusedGuestFault>,
    #[cfg(not(target_arch = "wasm32"))]
    pub refused_frame_fault: Option<semio_framework_actor::pack::PackError>,
    #[cfg(not(target_arch="wasm32"))]
    pub refused_kernel_fault:Option<semio_framework_actor::KernelError>,
    #[cfg(not(target_arch="wasm32"))]
    pub refused_actor_call:Option<semio_framework_plugin_host::original_actor_context::OriginalActorFixedCall>,
    /// 🌉️ MCP agent bridge consumer (presence + parked approvals), packet W1j.
    pub agent: semio_framework_os_renderer_wgpu_agent_elements::agent_bridge::AgentBridgeState,
    /// ✅️ The approvals modal's own open/dismiss state, alongside `agent.pending_approvals`.
    pub agent_approvals: semio_framework_os_renderer_wgpu_agent_elements::agent_approvals::AgentApprovalsModel,
}

/// 🖥️ UI-thread persistence and platform integration state surrounding chrome construction.
#[derive(Default)]
pub struct ShellChromePresentState {
    pub last_persisted_dock_ui: Option<DockUiState>,
    pub last_persisted_dock_skeleton: Option<Option<DockSkeleton>>,
    pub preferences_loaded: bool,
    pub last_synced_preferences: Option<UiPrefsSnapshot>,
    pub maintenance: ShellChromeMaintenance,
}

#[derive(Default)]
struct ShellChromeMaintenance {
    pub load_requested: bool,
    pub load_phase: u8,
    pub introduction_read: Option<String>,
    pub introduction_write: Option<String>,
    pub layout_requested: bool,
    pub presence_requested: bool,
    pub persist_requested: bool,
    pub persist_phase: u8,
}

impl ShellChromeMaintenance {
    pub fn pending(&self) -> bool {
        self.load_requested || self.introduction_read.is_some() || self.introduction_write.is_some() || self.layout_requested || self.presence_requested || self.persist_requested
    }
}

//#endregion 🔖️ChromeThreadBoundary
//#endregion ShellTypes

//#region ShellTypes
//#region 🔖️ChromeThreadBoundary
#[cfg(all(any(test, feature = "testkit"), not(target_arch = "wasm32")))]
/// 🧵️ Compile-time proof that chrome construction state can move to a worker.
pub fn assert_shell_chrome_build_state_is_send() {
    fn assert_send<T: Send>() {}
    assert_send::<ShellChromeBuildState>();
}
//#endregion 🔖️ChromeThreadBoundary
//#endregion ShellTypes
