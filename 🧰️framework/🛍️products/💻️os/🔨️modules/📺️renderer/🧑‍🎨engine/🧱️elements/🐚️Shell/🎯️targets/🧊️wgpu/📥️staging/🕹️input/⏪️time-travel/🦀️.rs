//! 🕹️ `shell-input` staging of `time-travel`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;
use super::time_travel::*;

//#region ⏪️TimeTravelControls
/// 🆔️ The band's live status node: every stage and progress change is announced through it.
pub(super) const TIME_TRAVEL_BAND_STATUS_ID: &str = "shell.time-travel.status";

//#endregion ⏪️TimeTravelControls
//#region 🎯️TimeTravelFocus
/// 🗝️ The history-body nodes the editor target resolves to: the inputs section, and the Accept button standing in when
/// no input takes focus.
pub(crate) const TIME_TRAVEL_EDITOR_INPUTS_KEY: &str = "framework.history.editor.inputs";

pub(crate) const TIME_TRAVEL_EDITOR_ACCEPT_KEY: &str = "framework.history.editor.accept";

/// 🗝️ Whether a projected node key names `key`: the key itself, or a surface-qualified `<surface>/<key>`.
pub(super) fn projected_key_is(projected: &str, key: &str) -> bool {
    projected == key || projected.rsplit_once('/').is_some_and(|(_, tail)| tail == key)
}

/// 🎯️ The node of one retained window's accessibility projection the editor target resolves to — React's
/// `timeTravelFocusElementV1("editor")`: the first enabled control under the draft editor's inputs section (a row is
/// never the target), else its Accept button. `None` when the window does not show the editor.
pub(crate) fn time_travel_editor_focus_node(nodes: &[ui_contract::AccessibilityProjectionNode]) -> Option<usize> {
    let usable = |node: &ui_contract::AccessibilityProjectionNode| node.focusable && !node.disabled && !node.hidden;
    nodes
        .iter()
        .position(|node| projected_key_is(&node.key, TIME_TRAVEL_EDITOR_INPUTS_KEY))
        .and_then(|section| {
            let depth = nodes[section].depth;
            nodes[section + 1..].iter().take_while(|node| node.depth > depth).position(|node| usable(node) && node.role != "treeitem").map(|offset| section + 1 + offset)
        })
        .or_else(|| nodes.iter().position(|node| projected_key_is(&node.key, TIME_TRAVEL_EDITOR_ACCEPT_KEY) && usable(node)))
}

//#endregion 🎯️TimeTravelFocus
impl ShellInputState {
    /// 🎯️ Moves keyboard focus to the waiting time-travel target the moment a presented frame shows it — React's
    /// `scheduleTimeTravelFocusV1` — so the ARIA mirror (which follows the projected `focused` node) lands on it: the
    /// editor's first input receives an accessibility focus in its retained window, the band's live status node becomes
    /// the focused chrome node, and the open finalize prompt already focuses its first stop. A person typing in a field
    /// elsewhere keeps their focus, and a target that has not appeared within [`TIME_TRAVEL_FOCUS_SETTLE_MS`] is given up.
    pub(crate) fn resolve_time_travel_focus(&mut self, input: &mut InputState<ActionDescriptor>, chrome: &mut ChromeState, history: &mut HistoryState) {
        let Some((focus, deadline)) = history.time_travel_focus else { return };
        if chrome.time_travel_focus_is_held(input) || chrome_now_ms() > deadline {
            history.time_travel_focus = None;
            return;
        }
        let moved = match focus {
            TimeTravelFocus::Dialog => chrome.chrome_build.dialog_open(),
            TimeTravelFocus::Band => history.history_time_travel.is_some() && !chrome.chrome_build.dialog_open() && {
                input.blur_input();
                self.accessibility_focused_control_id = Some(TIME_TRAVEL_BAND_STATUS_ID.to_string());
                true
            },
            TimeTravelFocus::Editor => crate::interpreter::visible_retained_accessibility_target(time_travel_editor_focus_node).is_some_and(|target| self.focus_retained_accessibility_target(&target, input, chrome)),
        };
        if moved {
            history.time_travel_focus = None;
        }
    }

    /// 🎯️ Gives one retained node the same accessibility focus a reader's focus would give it, and hands the keyboard
    /// to its window; `false` when the node's window no longer presents it.
    pub(super) fn focus_retained_accessibility_target(&mut self, target: &ui_render::AccessibilityTarget, input: &mut InputState<ActionDescriptor>, chrome: &mut ChromeState) -> bool {
        let Some(commands) = crate::interpreter::dispatch_accessibility_event(&target.window_id, target.window_generation, target.node_id, &target.node_key, ui_wgpu::wgpu::AccessibilityUiEvent::Focus, input) else { return false };
        chrome.chrome_build.note_content_focus_commands(&commands);
        self.accessibility_focused_control_id = None;
        true
    }
}

