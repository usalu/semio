//! 🛑️ History editing refusal policy shared by native notice and fault projection.

pub(super) fn refusal(code: &str) -> Option<(&'static str, &'static str, semio_framework::Severity, bool)> {
    match code {
        "history.malformed-transition" => Some(("history.malformed-transition", "ui.history.refusal.malformedTransition", semio_framework::Severity::Error, false)),
        "history.unknown-target" => Some(("history.unknown-target", "ui.history.refusal.unknownTarget", semio_framework::Severity::Error, false)),
        "history.transition-refused" => Some(("history.transition-refused", "ui.history.refusal.transitionRefused", semio_framework::Severity::Error, false)),
        "timeTravel.frozen" => Some(("timeTravel.frozen", "ui.timeTravel.refusal.frozen", semio_framework::Severity::Warning, false)),
        "timeTravel.illegal" => Some(("timeTravel.illegal", "ui.timeTravel.refusal.illegal", semio_framework::Severity::Warning, false)),
        "timeTravel.stale" => Some(("timeTravel.stale", "ui.timeTravel.refusal.stale", semio_framework::Severity::Warning, true)),
        "timeTravel.blocked" => Some(("timeTravel.blocked", "ui.timeTravel.refusal.blocked", semio_framework::Severity::Warning, false)),
        "timeTravel.empty" => Some(("timeTravel.empty", "ui.timeTravel.refusal.empty", semio_framework::Severity::Warning, false)),
        "timeTravel.cancelled" => Some(("timeTravel.cancelled", "ui.timeTravel.refusal.cancelled", semio_framework::Severity::Warning, false)),
        "timeTravel.busy" => Some(("timeTravel.busy", "ui.timeTravel.refusal.busy", semio_framework::Severity::Warning, false)),
        "timeTravel.unknown-mutation" => Some(("timeTravel.unknown-mutation", "ui.timeTravel.refusal.unknownMutation", semio_framework::Severity::Warning, false)),
        "timeTravel.not-editable" => Some(("timeTravel.not-editable", "ui.timeTravel.refusal.notEditable", semio_framework::Severity::Warning, false)),
        "timeTravel.unknown-input" => Some(("timeTravel.unknown-input", "ui.timeTravel.refusal.unknownInput", semio_framework::Severity::Warning, false)),
        "timeTravel.invalid-input" => Some(("timeTravel.invalid-input", "ui.timeTravel.refusal.invalidInput", semio_framework::Severity::Warning, false)),
        "timeTravel.no-selection" => Some(("timeTravel.no-selection", "ui.timeTravel.refusal.noSelection", semio_framework::Severity::Warning, false)),
        "timeTravel.name-required" => Some(("timeTravel.name-required", "ui.timeTravel.refusal.nameRequired", semio_framework::Severity::Warning, false)),
        "timeTravel.name-invalid" => Some(("timeTravel.name-invalid", "ui.timeTravel.refusal.nameInvalid", semio_framework::Severity::Warning, false)),
        "timeTravel.schema-unavailable" => Some(("timeTravel.schema-unavailable", "ui.timeTravel.refusal.schemaUnavailable", semio_framework::Severity::Warning, false)),
        "timeTravel.replay-faulted" => Some(("timeTravel.replay-faulted", "ui.timeTravel.refusal.replayFaulted", semio_framework::Severity::Warning, false)),
        "timeTravel.commit-failed" => Some(("timeTravel.commit-failed", "ui.timeTravel.refusal.commitFailed", semio_framework::Severity::Warning, false)),
        "timeTravel.member-gone" => Some(("timeTravel.member-gone", "ui.timeTravel.refusal.memberGone", semio_framework::Severity::Warning, false)),
        "timeTravel.not-withdrawable" => Some(("timeTravel.not-withdrawable", "ui.timeTravel.refusal.notWithdrawable", semio_framework::Severity::Warning, false)),
        "timeTravel.editor-closed" => Some(("timeTravel.editor-closed", "ui.timeTravel.refusal.editorClosed", semio_framework::Severity::Warning, false)),
        _ => None,
    }
}
