//! 🏷️ Action outcome and labels of the tool run ledger: the action outcome, the exact integer argument reader and the step, verdict and state labels. See [`super`].

use super::*;
use semio_framework_ui_locale::Terminology;
use semio_framework_ui_locale::Locale;
use semio_framework_os_kernel::DslValue;
use semio_framework_plugin_interaction::UiAssemblyResult;

//#region 🔖️Driver
/// 🎛️ Outcome of one tool run action dispatch; rejections are silent no-ops (§2.1).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToolRunActionOutcome {
    Applied(ToolRunEffect),
    Rejected(ToolRunRejection),
}

/// 🔢️ One §2.5 action argument read as a `u64`, in every shape the wire can hand it over.
///
/// 🪪️ A JSON number that is an exact non-negative integer IS a `u64` here, and reading it as anything
/// else is not strictness, it is a silent refusal. `toolRunFinalize`'s `generation` crosses as
/// `Number(Float(1.0))` — the shell's own action arguments build it with `UiValue::Number(f64)`, and a
/// guest that mints it with `DslValue::uint` arrives the same way — so accepting only an exact `u64`
/// carrier or a numeric string made `generation` read `None`, and `(_, None)` is
/// `ToolRunRejection::Stale`. EVERY finalize of EVERY app was rejected: runs reached `Complete` and
/// stayed there, and because a start against a live run is `ToolRunRejection::Busy`, no tool run could
/// ever be started a second time. Measured on 6018 as a 3d preview that never re-evaluated after the
/// first evaluation of a session (`📓️preview-rearm-after-inspector-edit-2026-09-14.md`).
#[doc(hidden)]
pub fn tool_run_arg_u64(args: Option<&DslValue>, key: &str) -> Option<u64> {
    let value = args?.get(key)?;
    value
        .as_u64()
        .or_else(|| value.as_str().and_then(|text| text.parse().ok()))
        .or_else(|| value.as_f64().filter(|number| number.is_finite() && *number >= 0.0 && number.fract() == 0.0 && *number < 18_446_744_073_709_551_616.0).map(|number| number as u64))
}
//#endregion 🔖️Driver

//#region 🔖️Panel
pub(super) fn tool_run_step_text(definition: &ToolRunDefinition, step: &ToolRunStep, locale: Locale) -> String {
    let template = match ToolRunLabel::for_reason(step.reason) {
        Some(label) => label.text(locale).to_string(),
        None => definition.reason(step.reason).map_or_else(|| step.reason.to_string(), |reason| reason.template.resolve(Terminology::Native, locale).to_string()),
    };
    let text = tool_run_format(&template, |name| name.parse::<usize>().ok().and_then(|index| step.args.get(index)).map(|arg| arg.to_plain_string()));
    if step.repeat > 1 {
        format!("{text} ×{}", step.repeat)
    } else {
        text
    }
}

pub(super) fn tool_run_step_tone(kind: ToolRunStepKind) -> Tone {
    match kind {
        ToolRunStepKind::Info => Tone::Info,
        ToolRunStepKind::Success => Tone::Success,
        ToolRunStepKind::Warning => Tone::Warning,
        ToolRunStepKind::Danger => Tone::Danger,
    }
}

pub(super) fn tool_run_verdict_tone(verdict: ToolRunVerdict) -> Tone {
    match verdict {
        ToolRunVerdict::Testing => Tone::Neutral,
        ToolRunVerdict::Success => Tone::Success,
        ToolRunVerdict::Warning => Tone::Warning,
        ToolRunVerdict::Danger => Tone::Danger,
    }
}

pub(super) fn tool_run_state_label(state: ToolRunState) -> ToolRunLabel {
    state.label()
}

impl<A: ArtifactApp> ToolRunLedger<A> {
    /// 🪧️ The framework ToolRun panel (§2.6) as a `ComponentTree` root `framework.toolRun`: a ready group
    /// `framework.toolRun.ready` with an enabled Start for `ready_tool` (the active run-declaring tool) while this
    /// instance holds no run of it, then one group `framework.toolRun.<run>` per run, oldest first, labelled by
    /// `tool_label` of its tool.
    pub fn panel(&mut self, controller_id: &str, locale: Locale, ready_tool: Option<&str>, tool_label: impl Fn(&str) -> Option<String>) -> UiAssemblyResult<BuiltNode> {
        let error = ui_assembly_error;
        let mut groups = BuiltChildren::default();
        if let Some(tool_id) = ready_tool.filter(|tool_id| !self.entries.iter().any(|entry| entry.tool_id == *tool_id)) {
            let label = tool_label(tool_id).unwrap_or_else(|| tool_id.to_string());
            groups.try_push(tool_run_panel_ready_group(tool_id, controller_id, locale, &label)?).map_err(|_| error("tool-run-panel.groups"))?;
        }
        for entry in &mut self.entries {
            let label = tool_label(&entry.tool_id).unwrap_or_else(|| entry.tool_id.clone());
            groups.try_push(tool_run_panel_group(entry, controller_id, locale, &label)?).map_err(|_| error("tool-run-panel.groups"))?;
        }
        column()
            .try_label(if locale == Locale::De { "Werkzeugläufe" } else { "Tool runs" })
            .map_err(|_| error("tool-run-panel.label"))?
            .try_id(semio_framework_tool_run::TOOL_RUN_PANEL_ID)
            .map_err(|_| error("tool-run-panel.id"))?
            .try_children(groups)
            .map_err(|_| error("tool-run-panel.groups"))?
            .try_build()
            .map_err(|_| error("tool-run-panel.build"))
    }
}
//#endregion 🔖️Panel
