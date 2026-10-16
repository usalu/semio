//! 🎚️ `shell-chrome-state` staging of `identity-and-types`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellTypes
//#region 🔖️ChromeThreadBoundary
pub(crate) struct RetainedContentFocus {
    pub(crate) node: ui_wgpu::wgpu::NodeId,
    pub(crate) key: ui_wgpu::wgpu::NodeKey,
    pub(crate) kind: RetainedNodeFocusKind,
}

/// 🏗️ Send-capable inputs, scratch state, and outputs consumed by chrome construction.
#[derive(Default)]
pub(crate) struct ShellChromeBuildState {
    pub(crate) find_items: ShellFindItems,
    /// 🎯️ Which retained node holds focus per window, `None` for a window whose content is blurred —
    /// the wgpu twin of React's active-root registry (`🧱️elements/🌈️Surface/🟦️.tsx:186-197`, whose
    /// `focusin` listener stores `resolveSurfaceActiveRoot(event.target)`, not a boolean). This used to
    /// be a bare `bool`, which is why `UiCommand::FocusChanged`'s node id was thrown away on arrival
    /// (ticket 26/09/17 packet W2k).
    pub(crate) content_focus: HashMap<String, Option<RetainedContentFocus>>,
    pub(crate) focused_retained_surface: Option<String>,
    pub(crate) tooltip_titles: HashMap<String, String>,
    /// 🚗️ The resolved `UiDriver` axes this chrome presents under — React's `UiDriverProvider` value
    /// (`🧱️elements/🚗️UiDriver/🟦️.tsx`), re-resolved from `driver_id` + `custom_drivers` every time
    /// preferences load or change. It is what makes `tooltips: "none"` (the shipped `compact`
    /// driver) actually suppress a hover tooltip on this target; before ticket 26/09/17 packet W15a
    /// the whole `UiDriver` vocabulary had zero readers anywhere in the wgpu tree, so every chrome
    /// control fired its tooltip under every driver.
    pub(crate) driver: ui_wgpu::wgpu::UiDriverChrome,
    pub(crate) tooltip_hover: Option<ChromeTooltipHover>,
    pub(crate) dialog_stack: Vec<ChromeDialogRequest>,
    pub(crate) tour_state: Option<ChromeTourState>,
    pub(crate) introduction_host_suppressed: bool,
    pub(crate) previous_pointer_down: bool,
    pub(crate) clicked_this_frame: bool,
    pub(crate) tour_reveal_latch: Option<String>,
    pub(crate) element_rects: HashMap<String, ChromeElementRectEntry>,
    pub(crate) introduction_seen: HashMap<String, bool>,
    pub(crate) introduction_seen_writes: Vec<String>,
    pub(crate) tutorial_dispatch_internal: bool,
    pub(crate) preferences: ChromePrefsState,
    /// 🧯️ The one live non-fatal notice — see `🧯️TransientNoticeAndAgentOverlays`.
    pub(crate) transient_notice: Option<ShellTransientNotice>,
    /// 📣️ The structured guest refusal behind the dispatch-fault string the funnel classifies next — see [`RefusedGuestFault`].
    pub(crate) refused_guest_fault: Option<RefusedGuestFault>,
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) refused_frame_fault: Option<semio_framework_actor::pack::PackError>,
    #[cfg(not(target_arch="wasm32"))]
    pub(crate) refused_kernel_fault:Option<semio_framework_actor::KernelError>,
    #[cfg(not(target_arch="wasm32"))]
    pub(crate) refused_actor_call:Option<semio_framework_plugin_host::original_actor_context::OriginalActorFixedCall>,
    /// 🌉️ MCP agent bridge consumer (presence + parked approvals), packet W1j.
    pub(crate) agent: crate::agent_bridge::AgentBridgeState,
    /// ✅️ The approvals modal's own open/dismiss state, alongside `agent.pending_approvals`.
    pub(crate) agent_approvals: crate::agent_approvals::AgentApprovalsModel,
}

/// 🖥️ UI-thread persistence and platform integration state surrounding chrome construction.
#[derive(Default)]
pub(crate) struct ShellChromePresentState {
    pub(crate) last_persisted_dock_ui: Option<DockUiState>,
    pub(crate) last_persisted_dock_skeleton: Option<Option<DockSkeleton>>,
    pub(crate) preferences_loaded: bool,
    pub(crate) last_synced_preferences: Option<UiPrefsSnapshot>,
    pub(crate) maintenance: ShellChromeMaintenance,
}

#[derive(Default)]
struct ShellChromeMaintenance {
    pub(crate) load_requested: bool,
    pub(crate) load_phase: u8,
    pub(crate) introduction_read: Option<String>,
    pub(crate) introduction_write: Option<String>,
    pub(crate) layout_requested: bool,
    pub(crate) presence_requested: bool,
    pub(crate) persist_requested: bool,
    pub(crate) persist_phase: u8,
}

impl ShellChromeMaintenance {
    pub(crate) fn pending(&self) -> bool {
        self.load_requested || self.introduction_read.is_some() || self.introduction_write.is_some() || self.layout_requested || self.presence_requested || self.persist_requested
    }
}

//#endregion 🔖️ChromeThreadBoundary
//#endregion ShellTypes

