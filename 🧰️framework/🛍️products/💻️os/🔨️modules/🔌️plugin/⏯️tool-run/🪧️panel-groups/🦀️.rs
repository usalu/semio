//! 🪧️ Panel groups of the tool run ledger: the ready group and the run panel group of the history panel. See [`super`].

use super::*;

//#region 🔖️Panel
/// ▶️ The ready group of a run-declaring tool with no run yet: its status and a real Start button carrying the tool id.
pub(super) fn tool_run_panel_ready_group(tool_id: &str, controller_id: &str, locale: Locale, label: &str) -> UiAssemblyResult<BuiltNode> {
    let error = ui_assembly_error;
    let scope = format!("{}.ready", semio_framework_tool_run::TOOL_RUN_PANEL_ID);
    let status = text(Label(UiText::clipped(ToolRunLabel::ReadyToStart.text(locale))))
        .live(Liveness::Polite)
        .try_id(format!("{scope}.status"))
        .map_err(|_| error("tool-run-panel.ready-status-id"))?
        .try_build()
        .map_err(|_| error("tool-run-panel.ready-status"))?;
    let mut arguments = UiMapBuilder::try_new().ok_or_else(|| error("tool-run-panel.ready-args"))?;
    arguments.push(TOOL_RUN_ARG_TOOL_ID.to_string(), UiValue::Text(UiText::clipped(tool_id))).map_err(|_| error("tool-run-panel.ready-args"))?;
    let action = ToolRunAction::Start;
    let action_id = ActionId::try_v1(controller_id, action.id()).ok_or_else(|| error("tool-run-panel.action-id"))?;
    let start = button(Label(UiText::clipped(action.label().text(locale))))
        .try_id(format!("{scope}.{}", action.id()))
        .map_err(|_| error("tool-run-panel.button-id"))?
        .try_shortcut(action.chord())
        .map_err(|_| error("tool-run-panel.button-shortcut"))?
        .try_on_with(Trigger::Activate, action_id, UiValue::Map(arguments.finish()))
        .map_err(|_| error("tool-run-panel.button-binding"))?
        .try_build()
        .map_err(|_| error("tool-run-panel.button"))?;
    let mut buttons = BuiltChildren::default();
    buttons.try_push(start).map_err(|_| error("tool-run-panel.buttons"))?;
    let toolbar = row().try_id(format!("{scope}.actions")).map_err(|_| error("tool-run-panel.actions-id"))?.try_children(buttons).map_err(|_| error("tool-run-panel.actions"))?.try_build().map_err(|_| error("tool-run-panel.actions-build"))?;
    let mut children = BuiltChildren::default();
    for child in [status, toolbar] {
        children.try_push(child).map_err(|_| error("tool-run-panel.children"))?;
    }
    column()
        .try_label(label)
        .map_err(|_| error("tool-run-panel.label"))?
        .try_id(scope)
        .map_err(|_| error("tool-run-panel.id"))?
        .try_children(children)
        .map_err(|_| error("tool-run-panel.children"))?
        .try_build()
        .map_err(|_| error("tool-run-panel.build"))
}

/// 🪧️ One run's panel group (§2.6): polite status, progressbar, real buttons with `aria-keyshortcuts` addressing the
/// run by id, step log (live off) and a keyboard-navigable trace list; every id is scoped `framework.toolRun.<run>.`.
///
/// 👁️ The throttle belongs to the ANNOUNCEMENT, never to the text. Publishing the last ANNOUNCED
/// string froze the visible pill at whatever stage the run was in when it was first announced:
/// a `Finalized` run still read `Evaluating nodes (1/2)` on 6021 at 18:28 while its own progress
/// bar had moved on (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). A sighted reader is watching the
/// same string a reader hears, so the string is always current and only `live` is rationed.
pub(super) fn tool_run_panel_group<A: ArtifactApp>(entry: &mut ToolRunEntry<A>, controller_id: &str, locale: Locale, label: &str) -> UiAssemblyResult<BuiltNode> {
    let error = ui_assembly_error;
    let scope = semio_framework_tool_run::tool_run_panel_group_id(entry.slot.run);
    let state = entry.slot.state;
    let stage_count = entry.definition.stages.len();
    let stage_label = entry.definition.stage(entry.stage).map_or_else(String::new, |stage| stage.label.resolve(Terminology::Native, locale).to_string());
    let status = format!("{} · {stage_label} ({}/{stage_count})", tool_run_state_label(state).text(locale), usize::from(entry.stage) + 1);
    let now_ms = semio_framework_job::default_now_ms().unwrap_or(0);
    let announce = match entry.announced.as_ref() {
        Some((announced_state, at, announced)) => *announced_state != state || (*announced != status && now_ms.saturating_sub(*at) >= TOOL_RUN_STATUS_ANNOUNCE_INTERVAL_MS),
        None => true,
    };
    if announce {
        entry.announced = Some((state, now_ms, status.clone()));
    }
    let live = if !announce {
        Liveness::Off
    } else if state == ToolRunState::Faulted || entry.conflicts > 0 {
        Liveness::Assertive
    } else {
        Liveness::Polite
    };
    let status_node = text(Label(UiText::clipped(&status))).live(live).try_id(format!("{scope}.status")).map_err(|_| error("tool-run-panel.status-id"))?.try_build().map_err(|_| error("tool-run-panel.status"))?;
    let unit = entry.definition.unit.resolve(Terminology::Native, locale).to_string();
    let value_text = match entry.total {
        Some(total) => {
            let percent = if total == 0 { 100 } else { entry.completed.saturating_mul(100) / total };
            tool_run_format(ToolRunLabel::ProgressValueText.text(locale), |name| match name {
                "stage" => Some(stage_label.clone()),
                "i" => Some((usize::from(entry.stage) + 1).to_string()),
                "n" => Some(stage_count.to_string()),
                "completed" => Some(entry.completed.to_string()),
                "total" => Some(total.to_string()),
                "unit" => Some(unit.clone()),
                "pct" => Some(percent.to_string()),
                _ => None,
            })
        }
        None => format!("{stage_label} ({}/{stage_count}): {} {unit}", usize::from(entry.stage) + 1, entry.completed),
    };
    let mut bar = progress(entry.completed as f64, Label(UiText::clipped(&value_text)));
    if let Some(total) = entry.total {
        bar = bar.total(total as f64);
    }
    let bar = bar.try_id(format!("{scope}.progress")).map_err(|_| error("tool-run-panel.progress-id"))?.try_build().map_err(|_| error("tool-run-panel.progress"))?;
    let (run, generation) = (entry.slot.run, entry.slot.generation);
    let mut buttons = BuiltChildren::default();
    let toggle = if state == ToolRunState::Paused { ToolRunAction::Resume } else { ToolRunAction::Pause };
    let actions = if state.is_terminal() { vec![ToolRunAction::Start, ToolRunAction::Dismiss] } else { vec![toggle, ToolRunAction::Step, ToolRunAction::Abort, ToolRunAction::Finalize] };
    for action in actions {
        let mut arguments = UiMapBuilder::try_new().ok_or_else(|| error("tool-run-panel.args"))?;
        let pushed = match action {
            ToolRunAction::Start => arguments.push(TOOL_RUN_ARG_TOOL_ID.to_string(), UiValue::Text(UiText::clipped(&entry.tool_id))).is_ok(),
            _ => arguments.push(TOOL_RUN_ARG_RUN_ID.to_string(), UiValue::Text(UiText::clipped(&run.to_string()))).is_ok() && arguments.push(TOOL_RUN_ARG_GENERATION.to_string(), UiValue::Number(f64::from(generation))).is_ok(),
        };
        if !pushed {
            return Err(error("tool-run-panel.args"));
        }
        let action_id = ActionId::try_v1(controller_id, action.id()).ok_or_else(|| error("tool-run-panel.action-id"))?;
        let legal = action.is_legal_in(Some(state));
        let mut builder = button(Label(UiText::clipped(action.label().text(locale)))).disabled(!legal);
        builder = builder.try_id(format!("{scope}.{}", action.id())).map_err(|_| error("tool-run-panel.button-id"))?;
        builder = builder.try_shortcut(action.chord()).map_err(|_| error("tool-run-panel.button-shortcut"))?;
        if action == ToolRunAction::Finalize && !legal {
            builder = builder.try_describe(ToolRunLabel::FinalizeDisabled.text(locale)).map_err(|_| error("tool-run-panel.finalize-description"))?;
        }
        let node = builder.try_on_with(Trigger::Activate, action_id, UiValue::Map(arguments.finish())).map_err(|_| error("tool-run-panel.button-binding"))?.try_build().map_err(|_| error("tool-run-panel.button"))?;
        buttons.try_push(node).map_err(|_| error("tool-run-panel.buttons"))?;
    }
    let toolbar = row().try_id(format!("{scope}.actions")).map_err(|_| error("tool-run-panel.actions-id"))?.try_children(buttons).map_err(|_| error("tool-run-panel.actions"))?.try_build().map_err(|_| error("tool-run-panel.actions-build"))?;
    let mut step_rows = BuiltChildren::default();
    let newest_steps: Vec<&ToolRunStep> = entry.steps.iter().collect();
    for step in newest_steps.into_iter().rev().take(TOOL_RUN_PANEL_STEP_ROWS) {
        let row = text(Label(UiText::clipped(&tool_run_step_text(&entry.definition, step, locale))))
            .tone(tool_run_step_tone(step.kind))
            .try_id(format!("{scope}.step.{}", step.sequence))
            .map_err(|_| error("tool-run-panel.step-id"))?
            .try_build()
            .map_err(|_| error("tool-run-panel.step"))?;
        step_rows.try_push(row).map_err(|_| error("tool-run-panel.steps"))?;
    }
    let steps = column()
        .live(Liveness::Off)
        .try_label(if locale == Locale::De { "Schritte" } else { "Steps" })
        .map_err(|_| error("tool-run-panel.steps-label"))?
        .try_id(format!("{scope}.steps"))
        .map_err(|_| error("tool-run-panel.steps-id"))?
        .try_children(step_rows)
        .map_err(|_| error("tool-run-panel.steps-children"))?
        .try_build()
        .map_err(|_| error("tool-run-panel.steps-build"))?;
    let mut trace_rows = BuiltChildren::default();
    for key in entry.recent_trace.iter().rev() {
        let Some(record) = entry.trace.record(*key) else { continue };
        let reason = ToolRunStep { sequence: 0, kind: ToolRunStepKind::Info, stage: entry.stage, reason: record.reason, subject: Some(*key), repeat: 1, args: Vec::new() };
        let item = tree_item(Label(UiText::clipped(&tool_run_step_text(&entry.definition, &reason, locale))))
            .tone(tool_run_verdict_tone(record.verdict))
            .try_id(format!("{scope}.trace.{key}"))
            .map_err(|_| error("tool-run-panel.trace-id"))?
            .try_build()
            .map_err(|_| error("tool-run-panel.trace-item"))?;
        trace_rows.try_push(item).map_err(|_| error("tool-run-panel.trace-rows"))?;
    }
    let trace = tree()
        .try_label(if locale == Locale::De { "Versuche" } else { "Attempts" })
        .map_err(|_| error("tool-run-panel.trace-label"))?
        .try_id(format!("{scope}.trace"))
        .map_err(|_| error("tool-run-panel.trace-id"))?
        .try_children(trace_rows)
        .map_err(|_| error("tool-run-panel.trace"))?
        .try_build()
        .map_err(|_| error("tool-run-panel.trace-build"))?;
    let mut children = BuiltChildren::default();
    for child in [status_node, bar, toolbar, steps, trace] {
        children.try_push(child).map_err(|_| error("tool-run-panel.children"))?;
    }
    column()
        .try_label(label)
        .map_err(|_| error("tool-run-panel.label"))?
        .try_id(scope)
        .map_err(|_| error("tool-run-panel.id"))?
        .try_children(children)
        .map_err(|_| error("tool-run-panel.children"))?
        .try_build()
        .map_err(|_| error("tool-run-panel.build"))
}
//#endregion 🔖️Panel
