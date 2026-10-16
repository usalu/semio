//! 🪝 `shell-window-actions` staging of `chrome-overlays-and-tour`: RENDERER-WP5, the part of the former chunk that plan Appendix B and C, the WP4 re-homing by the call graph and the WP5 size spill assign to this crate.

use super::*;

//#region ShellChrome
//#region 🔖️ChromeOverlaysAndTour
//#region 🧯️TransientNoticeAndAgentOverlays
/// ⚔️ The notice code a hub refusal naming no history transition is told under — React's `sync.command.rejected`.
pub(super) const SYNC_COMMAND_REJECTED_NOTICE_CODE: &str = "sync.command.rejected";

/// 🔀️ The notice code of a batch the hub applied in adjusted form — React's `sync.command.transformed`.
pub(crate) const SYNC_COMMAND_TRANSFORMED_NOTICE_CODE: &str = "sync.command.transformed";

/// 🚫️ The notice code, chrome line and severity of each local refusal — React's `LOCAL_COMMAND_REJECTION_NOTICES_V1`: one
/// the human can wait out or cannot act on here is a warning, one meaning this device produced a change it cannot send is
/// an error. `None` for the hub's own two codes.
pub(super) fn local_command_rejection_notice(code: store_sync::sync::CommandRejectionCode) -> Option<(&'static str, &'static str, semio_framework::Severity)> {
    use semio_framework::Severity;
    use store_sync::sync::CommandRejectionCode as Code;
    Some(match code {
        Code::HubRefused | Code::HubUnreadable => return None,
        Code::LocalReadOnly => ("local.read-only", "conflict.local.readOnly", Severity::Warning),
        Code::LocalQueueFull => ("local.queue-full", "conflict.local.queueFull", Severity::Warning),
        Code::LocalBackboneCapacity => ("local.backbone-capacity", "conflict.local.queueFull", Severity::Warning),
        Code::LocalBackboneDuplicate => ("local.backbone-duplicate", "conflict.local.duplicate", Severity::Warning),
        Code::LocalBackbonePairUnavailable => ("local.backbone-pair-unavailable", "conflict.local.notReady", Severity::Warning),
        Code::LocalBackboneScopeMismatch => ("local.backbone-scope-mismatch", "conflict.local.foreignDocument", Severity::Error),
        Code::LocalBackboneMalformed => ("local.backbone-malformed", "conflict.local.unreadable", Severity::Error),
        Code::LocalSocketFrameCeiling => ("local.socket-frame-ceiling", "conflict.local.tooLarge", Severity::Error),
    })
}
//#endregion 🧯️TransientNoticeAndAgentOverlays
//#endregion 🔖️ChromeOverlaysAndTour
//#endregion ShellChrome
