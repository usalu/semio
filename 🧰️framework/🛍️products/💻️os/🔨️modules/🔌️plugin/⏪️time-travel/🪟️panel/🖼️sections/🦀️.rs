//! 🖼️ The history panel sections: row refusals of the mutation controls, the band, reprojection and editor sections, control rows, the input row builder and the panel limits.

use semio_framework_plugin_interaction::{UiAssemblyResult,ui_assembly_error};
use semio_framework_plugin_window_transient::{ui_label,ui_text};
use semio_framework_plugin_window_kits_rich::tree_kit::{TreeWindows,tree_window_indexed_item,tree_window_indexed_section};
use semio_framework::{ActionArgControl, ArgPresentation, ArgSchema, FaultFrom, IconName, reference_id_text};
use semio_framework_plugin_decl_app::ArtifactApp;
#[cfg(test)]
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
#[cfg(test)]
use semio_framework_plugin_decl_app::tool_contract::ArtifactOwnedToolJobFactory;
use pack::snapshot::ArtifactPack;
use semio_framework_value::DslValue;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework_value_derive::{FromValue, ToValue};
use semio_framework::history_edit::{HISTORY_EDIT_ACCEPT_ACTION_ID, HISTORY_EDIT_ARG_EDIT, HISTORY_EDIT_ARG_GENERATION, HISTORY_EDIT_ARG_MUTATION_ID, HISTORY_EDIT_ARG_PATH, HISTORY_EDIT_ARG_STORE, HISTORY_EDIT_ARG_VALUE, HISTORY_EDIT_BEGIN_ACTION_ID, HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID, HISTORY_EDIT_DISCARD_ACTION_ID, HISTORY_EDIT_EXIT_ACTION_ID, HISTORY_EDIT_FINALIZE_ACTION_ID, HISTORY_EDIT_INPUT_ACTION_ID, HISTORY_EDIT_INPUT_INSERT, HISTORY_EDIT_INPUT_REMOVE, HISTORY_EDIT_RERUN_ACTION_ID, HISTORY_EDIT_USE_SELECTION_ACTION_ID, HISTORY_EDIT_WITHDRAW_ACTION_ID};
use semio_framework_plugin_command::lifted_command::{HistoryPanelText, MutationRowRefusals, TimeTravelEditorPanel, TimeTravelInputRow, TimeTravelListItem, TimeTravelPanel, history_code_text, history_severity_text, history_severity_tone};
use semio_framework::kernel::{HistoryReprojection, HistoryReprojectionKind, HistoryTimeTravelStage};
use semio_framework_ui_locale::{Locale, LocalizedLabel, Terminology};
use store::{Mutation, MutationDiff};
use protocol::{OpBinary, OpText};
use semio_framework_time_travel::{TimeTravelLabel, TimeTravelRefusal, TimeTravelReview, TimeTravelStage};
#[cfg(test)]
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_window_config::WindowConfigOwner;
use crate::panel_pointer::{time_travel_color_rgba, time_travel_json};
use semio_framework_ui_contract::*;

//#region 🔖️Limits
/// 🔎️ Document values one reference-name lookup visits at most ([`time_travel_entity_names`]).
pub const TIME_TRAVEL_ENTITY_NAME_VISITS: usize = 262_144;
/// ✂️ Characters of an entity id a fallback chip shows before an ellipsis ([`time_travel_reference_fallback_label`]).
pub const TIME_TRAVEL_SHORT_ID_CHARS: usize = 12;
//#endregion 🔖️Limits

//#region 🔖️PanelView
/// 🛃️ The [`MutationRowRefusals`] of the mutation `mutation_id` held by `store` (`None`: the document's own): a session
/// on another store, or a `busy` instance, refuses Edit and Withdraw as busy; the session's own `Begin` law refuses both
/// (`TimeTravelSession::begin_refusal`) — except Withdraw on the mutation being edited, which is the editor's own; an
/// accepted draft is restored while reviewing (`TimeTravelSession::restore_refusal`).
pub fn mutation_row_refusals(panel: Option<&TimeTravelPanel>, busy: bool, mutation_id: &str, store: Option<&str>) -> MutationRowRefusals {
    let session = panel.filter(|panel| panel.store.as_deref() == store);
    let begin = match (panel, session) {
        (Some(_), None) => Some(TimeTravelLabel::RefusalBusy),
        (_, Some(session)) => session.begin_refusal.map(TimeTravelRefusal::label),
        (None, None) => None,
    }
    .or_else(|| busy.then_some(TimeTravelLabel::RefusalBusy));
    let edited = session.is_some_and(|session| session.stage == TimeTravelStage::Editing && session.status.target.as_deref() == Some(mutation_id));
    MutationRowRefusals {
        begin,
        withdraw: if edited { None } else { begin },
        accepted: session.is_some_and(|session| session.accepted.contains(mutation_id)),
        restore: session.and_then(|session| (session.stage != TimeTravelStage::Reviewing).then_some(TimeTravelLabel::RefusalIllegal)),
    }
}
//#endregion 🔖️PanelView

//#region 🔖️Panel
/// 🔘️ A button dispatching `verb` on `controller_id` with `args`.
fn time_travel_button(controller_id: &str, id: &str, label: &str, icon: &str, verb: &str, args: Option<UiValue>, enabled: bool) -> UiAssemblyResult<BuiltNode> {
    let action = ActionId::try_v1(controller_id, verb).ok_or_else(|| ui_assembly_error("time-travel-panel.action-id"))?;
    let builder = button(ui_label(label, "time-travel-panel.button-label")?).icon(ui_text(icon, "time-travel-panel.button-icon")?).disabled(!enabled).try_id(id).map_err(|_| ui_assembly_error("time-travel-panel.button-id"))?;
    let builder = match args {
        Some(args) => builder.try_on_with(Trigger::Activate, action, args),
        None => builder.try_on(Trigger::Activate, action),
    };
    builder.map_err(|_| ui_assembly_error("time-travel-panel.button-binding"))?.try_build().map_err(|_| ui_assembly_error("time-travel-panel.button"))
}

/// 🧿️ The `{generation}` argument map every session button carries (a stale press is a silent `timeTravel.stale`).
fn time_travel_generation_args(generation: u32) -> UiAssemblyResult<UiValue> {
    let mut args = UiMapBuilder::try_new().ok_or_else(|| ui_assembly_error("time-travel-panel.args"))?;
    args.push(HISTORY_EDIT_ARG_GENERATION.to_string(), UiValue::Number(f64::from(generation))).map_err(|_| ui_assembly_error("time-travel-panel.args"))?;
    Ok(UiValue::Map(args.finish()))
}

/// 🧭️ The `{generation, path}` argument map of one input control.
fn time_travel_input_args(generation: u32, path: &str) -> UiAssemblyResult<UiValue> {
    let mut args = UiMapBuilder::try_new().ok_or_else(|| ui_assembly_error("time-travel-panel.input-args"))?;
    args.push(HISTORY_EDIT_ARG_GENERATION.to_string(), UiValue::Number(f64::from(generation))).map_err(|_| ui_assembly_error("time-travel-panel.input-args"))?;
    args.push(HISTORY_EDIT_ARG_PATH.to_string(), UiValue::Text(UiText::clipped(path))).map_err(|_| ui_assembly_error("time-travel-panel.input-args"))?;
    Ok(UiValue::Map(args.finish()))
}

/// 🧽️ The `{generation, path, value}` argument map of a button that drafts one fixed value: a nullable input's Clear
/// (`null`), one option of a long choice.
fn time_travel_value_args(generation: u32, path: &str, value: UiValue) -> UiAssemblyResult<UiValue> {
    let mut args = UiMapBuilder::try_new().ok_or_else(|| ui_assembly_error("time-travel-panel.value-args"))?;
    args.push(HISTORY_EDIT_ARG_GENERATION.to_string(), UiValue::Number(f64::from(generation))).map_err(|_| ui_assembly_error("time-travel-panel.value-args"))?;
    args.push(HISTORY_EDIT_ARG_PATH.to_string(), UiValue::Text(UiText::clipped(path))).map_err(|_| ui_assembly_error("time-travel-panel.value-args"))?;
    args.push(HISTORY_EDIT_ARG_VALUE.to_string(), value).map_err(|_| ui_assembly_error("time-travel-panel.value-args"))?;
    Ok(UiValue::Map(args.finish()))
}

/// 🧮️ Why a list edit is disabled, localized: "Maximum 8 items" (`maxItems` reached), "Minimum 1 item" (`minItems` left).
#[doc(hidden)]
pub fn time_travel_item_limit_text(limit: u32, max: bool, locale: Locale) -> String {
    let text = match (max, limit == 1) {
        (true, true) => HistoryPanelText::MaxItem,
        (true, false) => HistoryPanelText::MaxItems,
        (false, true) => HistoryPanelText::MinItem,
        (false, false) => HistoryPanelText::MinItems,
    };
    text.text(locale).replace("{count}", &limit.to_string())
}

/// ✂️ The `{generation, path, edit}` argument map of a list edit button: `insert` (the item's default at `path`, `…/-`
/// appends) or `remove` (the item at `path`).
fn time_travel_list_edit_args(generation: u32, path: &str, edit: &str) -> UiAssemblyResult<UiValue> {
    let mut args = UiMapBuilder::try_new().ok_or_else(|| ui_assembly_error("time-travel-panel.list-args"))?;
    args.push(HISTORY_EDIT_ARG_GENERATION.to_string(), UiValue::Number(f64::from(generation))).map_err(|_| ui_assembly_error("time-travel-panel.list-args"))?;
    args.push(HISTORY_EDIT_ARG_PATH.to_string(), UiValue::Text(UiText::clipped(path))).map_err(|_| ui_assembly_error("time-travel-panel.list-args"))?;
    args.push(HISTORY_EDIT_ARG_EDIT.to_string(), UiValue::Text(UiText::clipped(edit))).map_err(|_| ui_assembly_error("time-travel-panel.list-args"))?;
    Ok(UiValue::Map(args.finish()))
}

/// 🌲️ One tree row holding one inline control — the history body's one vocabulary for a control, the shape both hosts
/// mount: a tree section's `tree_item`s, each with at most one single control child (React mounts a row's non-row
/// children as its controls and activates a row's one activatable control; wgpu mounts one input, select, toggle,
/// button, key-value list, slider, stepper, ring or icon select per row and nests every other child as a row).
fn time_travel_control_row(id: &str, label: &str, icon: &str, control: BuiltNode, enabled: bool, tone: Option<Tone>, description: Option<&str>) -> UiAssemblyResult<BuiltNode> {
    let error = ui_assembly_error;
    let mut row = tree_item(Label(UiText::clipped(label))).icon(ui_text(icon, "time-travel-panel.row-icon")?).disabled(!enabled);
    if let Some(tone) = tone {
        row = row.tone(tone);
    }
    if let Some(description) = description {
        row = row.description(UiText::clipped(description));
    }
    row.try_id(id).map_err(|_| error("time-travel-panel.row-id"))?.try_child(control).map_err(|_| error("time-travel-panel.row-child"))?.try_build().map_err(|_| error("time-travel-panel.row"))
}

/// 🌲️ A button row: the button keeps its own id (`{id}`) and the row wraps it as `{id}.row`.
fn time_travel_button_row(controller_id: &str, id: &str, label: &str, icon: &str, verb: &str, args: Option<UiValue>, enabled: bool) -> UiAssemblyResult<BuiltNode> {
    time_travel_control_row(&format!("{id}.row"), label, icon, time_travel_button(controller_id, id, label, icon, verb, args, enabled)?, enabled, None, None)
}

/// 🪧️ The session band as the history body's first tree section `framework.history.timeTravel`, one row each: the stage
/// status with the edited mutation and the report's worst severity, replay progress in words, the last fault, then Cancel
/// replay, Next problem, Finalize (disabled with its reason while refused), Replay again and Exit as the stage offers
/// them. Rows carry text in their labels, never as a child, so every host renders them; the shell's own band is the one
/// polite live region that announces the session and draws its progress bar.
pub(crate) fn time_travel_band_section(panel: &TimeTravelPanel, controller_id: &str, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    let error = ui_assembly_error;
    let scope = "framework.history.timeTravel";
    let status = &panel.status;
    let mut rows = BuiltChildren::default();
    let mut stage_line = panel.review.map_or_else(|| panel.stage.label(), TimeTravelReview::label).localized(LocalizedLabel::native).resolve(Terminology::Native, locale).to_string();
    if let Some(label) = status.target_label.as_ref() {
        stage_line = format!("{stage_line}: {}", label.resolve(Terminology::Native, locale));
    }
    if let Some(worst) = status.worst {
        stage_line = format!("{stage_line} \u{b7} {}", history_severity_text(worst, locale));
    }
    let (tone, _) = history_severity_tone(status.worst);
    let tone = if status.worst.is_some() { tone } else { Tone::Neutral };
    let stage_row = tree_item(Label(UiText::clipped(&stage_line)))
        .icon(ui_text("clock", "time-travel-panel.status-icon")?)
        .tone(tone)
        .try_id(format!("{scope}.status"))
        .map_err(|_| error("time-travel-panel.status-id"))?
        .try_build()
        .map_err(|_| error("time-travel-panel.status"))?;
    rows.try_push(stage_row).map_err(|_| error("time-travel-panel.rows"))?;
    if let (Some(done), Some(total)) = (status.done, status.total) {
        let label = if status.stage == HistoryTimeTravelStage::Editing { TimeTravelLabel::PreparationProgressValueText } else { TimeTravelLabel::ReplayProgressValueText };
        let mut value_text = label.localized(LocalizedLabel::native).resolve(Terminology::Native, locale).replace("{done}", &done.to_string()).replace("{total}", &total.to_string());
        if let Some(processed) = status.processed {
            value_text.push_str(" · ");
            value_text.push_str(&TimeTravelLabel::Processed.localized(LocalizedLabel::native).resolve(Terminology::Native, locale).replace("{processed}", &processed.to_string()));
        }
        let progress_row = tree_item(Label(UiText::clipped(&value_text)))
            .icon(ui_text("loader-2", "time-travel-panel.progress-icon")?)
            .try_id(format!("{scope}.progress"))
            .map_err(|_| error("time-travel-panel.progress-id"))?
            .try_build()
            .map_err(|_| error("time-travel-panel.progress"))?;
        rows.try_push(progress_row).map_err(|_| error("time-travel-panel.rows"))?;
    }
    if let Some(fault) = status.fault.as_deref() {
        let label = TimeTravelLabel::for_code(fault).unwrap_or(TimeTravelLabel::ReplayFaulted).localized(LocalizedLabel::native).resolve(Terminology::Native, locale).to_string();
        let node = tree_item(Label(UiText::clipped(&label)))
            .icon(ui_text("triangle-alert", "time-travel-panel.fault-icon")?)
            .tone(Tone::Danger)
            .try_id(format!("{scope}.fault"))
            .map_err(|_| error("time-travel-panel.fault-id"))?
            .try_build()
            .map_err(|_| error("time-travel-panel.fault"))?;
        rows.try_push(node).map_err(|_| error("time-travel-panel.rows"))?;
    }
    let generation = status.generation;
    if panel.stage == TimeTravelStage::Replaying {
        rows.try_push(time_travel_button_row(controller_id, &format!("{scope}.cancelReplay"), HistoryPanelText::CancelReplay.text(locale), "square", HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID, Some(time_travel_generation_args(generation)?), true)?)
            .map_err(|_| error("time-travel-panel.rows"))?;
    }
    if let Some(problem) = panel.next_problem.as_deref() {
        let mut args = UiMapBuilder::try_new().ok_or_else(|| error("time-travel-panel.problem-args"))?;
        args.push(HISTORY_EDIT_ARG_MUTATION_ID.to_string(), UiValue::Text(UiText::clipped(problem))).map_err(|_| error("time-travel-panel.problem-args"))?;
        if let Some(store) = panel.store.as_deref() {
            args.push(HISTORY_EDIT_ARG_STORE.to_string(), UiValue::Text(UiText::clipped(store))).map_err(|_| error("time-travel-panel.problem-args"))?;
        }
        rows.try_push(time_travel_button_row(controller_id, &format!("{scope}.nextProblem"), HistoryPanelText::NextProblem.text(locale), "arrow-right", HISTORY_EDIT_BEGIN_ACTION_ID, Some(UiValue::Map(args.finish())), true)?)
            .map_err(|_| error("time-travel-panel.rows"))?;
    }
    if matches!(panel.stage, TimeTravelStage::Reviewing | TimeTravelStage::Choosing) {
        let refused = panel.finalize_refusal.map(|refusal| refusal.label().localized(LocalizedLabel::native).resolve(Terminology::Native, locale).to_string());
        let label = HistoryPanelText::Finalize.text(locale);
        let mut finalize = button(ui_label(label, "time-travel-panel.finalize-label")?)
            .icon(ui_text("list-checks", "time-travel-panel.finalize-icon")?)
            .disabled(refused.is_some())
            .try_id(format!("{scope}.finalize"))
            .map_err(|_| error("time-travel-panel.finalize-id"))?;
        if let Some(reason) = refused.as_deref() {
            finalize = finalize.try_describe(reason).map_err(|_| error("time-travel-panel.finalize-description"))?;
        }
        let action = ActionId::try_v1(controller_id, HISTORY_EDIT_FINALIZE_ACTION_ID).ok_or_else(|| error("time-travel-panel.action-id"))?;
        let finalize = finalize.try_on_with(Trigger::Activate, action, time_travel_generation_args(generation)?).map_err(|_| error("time-travel-panel.finalize-binding"))?.try_build().map_err(|_| error("time-travel-panel.finalize"))?;
        rows.try_push(time_travel_control_row(&format!("{scope}.finalize.row"), label, "list-checks", finalize, refused.is_none(), None, refused.as_deref())?).map_err(|_| error("time-travel-panel.rows"))?;
    }
    if panel.stage == TimeTravelStage::Reviewing {
        let rerun = TimeTravelLabel::ActionRerun.localized(LocalizedLabel::native).resolve(Terminology::Native, locale).to_string();
        rows.try_push(time_travel_button_row(controller_id, &format!("{scope}.rerun"), &rerun, "skip-forward", HISTORY_EDIT_RERUN_ACTION_ID, Some(time_travel_generation_args(generation)?), panel.rerun_refusal.is_none())?)
            .map_err(|_| error("time-travel-panel.rows"))?;
    }
    rows.try_push(time_travel_button_row(controller_id, &format!("{scope}.exit"), HistoryPanelText::Exit.text(locale), "rotate-ccw", HISTORY_EDIT_EXIT_ACTION_ID, None, panel.stage != TimeTravelStage::Finalizing)?)
        .map_err(|_| error("time-travel-panel.rows"))?;
    tree_section(ui_label(HistoryPanelText::TimeTravel.text(locale), "time-travel-panel.label")?)
        .default_open(true)
        .try_id(scope)
        .map_err(|_| error("time-travel-panel.id"))?
        .try_children(rows)
        .map_err(|_| error("time-travel-panel.rows"))?
        .try_build()
        .map_err(|_| error("time-travel-panel.build"))
}

/// 📡️ The history change waiting for its replay as the history body's tree section `framework.history.reprojection`
/// (design §16.6, gap N17) — a remote change, this replica's own history step or a whole-document load: one status row whose
/// title and line are the kernel's one copy ([`semio_framework::kernel::history_reprojection_status`], the line both shells
/// announce; a refused adoption reads its named notice, never a raw code — audit W2A-8), its tone and icon by state and kind,
/// then, when `controls`, Replay again while a remote change is paused (its replay was dropped, so nothing counts yet) or
/// Cancel replay while one runs (a local step is dropped with zero trace) — no controls while a session is open, whose own
/// Cancel replay and Replay again address the session, nor for a viewer.
pub(crate) fn time_travel_reprojection_section(remote: &HistoryReprojection, controls: bool, controller_id: &str, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    let error = ui_assembly_error;
    let scope = "framework.history.reprojection";
    let status = semio_framework::kernel::history_reprojection_status(remote, Terminology::Native, locale);
    let tone = match (&status.fault, status.paused) {
        (Some(_), _) if status.total == 0 => Tone::Danger,
        (_, true) => Tone::Warning,
        _ => Tone::Info,
    };
    let icon = match remote.kind {
        HistoryReprojectionKind::Remote => "cloud-download",
        HistoryReprojectionKind::Step => IconName::Undo.as_str(),
        HistoryReprojectionKind::Load => IconName::Download.as_str(),
    };
    let mut rows = BuiltChildren::default();
    let line = tree_item(Label(UiText::clipped(&status.text)))
        .icon(ui_text(icon, "time-travel-panel.remote-icon")?)
        .tone(tone)
        .try_id(format!("{scope}.status"))
        .map_err(|_| error("time-travel-panel.remote-status-id"))?
        .try_build()
        .map_err(|_| error("time-travel-panel.remote-status"))?;
    rows.try_push(line).map_err(|_| error("time-travel-panel.remote-rows"))?;
    let control = match (controls, status.paused, status.total > 0) {
        (true, true, _) => Some(time_travel_button_row(controller_id, &format!("{scope}.rerun"), TimeTravelLabel::ActionRerun.localized(LocalizedLabel::native).resolve(Terminology::Native, locale), "skip-forward", HISTORY_EDIT_RERUN_ACTION_ID, None, true)?),
        (true, false, true) => Some(time_travel_button_row(controller_id, &format!("{scope}.cancelReplay"), HistoryPanelText::CancelReplay.text(locale), "square", HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID, None, true)?),
        _ => None,
    };
    if let Some(control) = control {
        rows.try_push(control).map_err(|_| error("time-travel-panel.remote-rows"))?;
    }
    tree_section(ui_label(status.title.as_str(), "time-travel-panel.remote-label")?)
        .default_open(true)
        .try_id(scope)
        .map_err(|_| error("time-travel-panel.remote-id"))?
        .try_children(rows)
        .map_err(|_| error("time-travel-panel.remote-rows"))?
        .try_build()
        .map_err(|_| error("time-travel-panel.remote"))
}

/// ✏️ The draft editor as two tree sections: `framework.history.editor` — the edited mutation with its draft's own
/// outcome, why no input is editable (a mutation without editable inputs reads the framework's own words, a payload
/// schema that describes none its reason), then Accept, Discard and Withdraw — and `framework.history.editor.inputs`,
/// one row per input row of [`time_travel_input_rows`] ([`time_travel_input_row`]). The inputs section is a tree window
/// over every row, so a payload of any size stays reachable: the host streams the slice it scrolls to.
#[doc(hidden)]
pub fn time_travel_editor_sections(panel: &TimeTravelPanel, editor: &TimeTravelEditorPanel, windows: &TreeWindows<'_>, controller_id: &str, locale: Locale) -> UiAssemblyResult<[BuiltNode; 2]> {
    let error = ui_assembly_error;
    let scope = "framework.history.editor";
    let generation = panel.status.generation;
    let mut rows = BuiltChildren::default();
    let mut heading = format!("{}: {}", HistoryPanelText::Draft.text(locale), editor.label.resolve(Terminology::Native, locale));
    if editor.withdrawn {
        heading = format!("{heading} \u{b7} {}", HistoryPanelText::Withdrawn.text(locale));
    }
    let worst = editor.outcome.iter().map(|message| message.level).max();
    if let Some(worst) = worst {
        let detail = editor.outcome.iter().find(|message| message.level == worst).map(|message| history_code_text(&message.code.0, locale)).unwrap_or_default();
        heading = format!("{heading} \u{b7} {}: {detail}", history_severity_text(worst, locale));
    }
    let (tone, icon) = history_severity_tone(worst);
    let head = tree_item(Label(UiText::clipped(&heading)))
        .icon(ui_text(icon, "time-travel-panel.editor-icon")?)
        .tone(tone)
        .try_id(format!("{scope}.target"))
        .map_err(|_| error("time-travel-panel.editor-target-id"))?
        .try_build()
        .map_err(|_| error("time-travel-panel.editor-target"))?;
    rows.try_push(head).map_err(|_| error("time-travel-panel.editor-rows"))?;
    let uneditable = (!editor.editable).then(|| TimeTravelLabel::RefusalNotEditable.localized(LocalizedLabel::native).resolve(Terminology::Native, locale).to_string());
    if let Some(reason) = uneditable.as_deref().or(editor.inputs_refused.as_deref()) {
        let node = tree_item(Label(UiText::clipped(HistoryPanelText::NoInputs.text(locale))))
            .description(UiText::clipped(reason))
            .tone(Tone::Warning)
            .try_id(format!("{scope}.refused"))
            .map_err(|_| error("time-travel-panel.editor-refused-id"))?
            .try_build()
            .map_err(|_| error("time-travel-panel.editor-refused"))?;
        rows.try_push(node).map_err(|_| error("time-travel-panel.editor-rows"))?;
    }
    let accept_reason = (!editor.changed).then(|| TimeTravelLabel::RefusalUnchanged.localized(LocalizedLabel::native).resolve(Terminology::Native, locale).to_string());
    let accept = time_travel_button(controller_id, &format!("{scope}.accept"), HistoryPanelText::Accept.text(locale), "check", HISTORY_EDIT_ACCEPT_ACTION_ID, Some(time_travel_generation_args(generation)?), editor.changed)?;
    rows.try_push(time_travel_control_row(&format!("{scope}.accept.row"), HistoryPanelText::Accept.text(locale), "check", accept, editor.changed, None, accept_reason.as_deref())?)
        .map_err(|_| error("time-travel-panel.editor-rows"))?;
    rows.try_push(time_travel_button_row(controller_id, &format!("{scope}.discard"), HistoryPanelText::Discard.text(locale), "x", HISTORY_EDIT_DISCARD_ACTION_ID, Some(time_travel_generation_args(generation)?), true)?)
        .map_err(|_| error("time-travel-panel.editor-rows"))?;
    rows.try_push(time_travel_button_row(controller_id, &format!("{scope}.withdraw"), HistoryPanelText::Withdraw.text(locale), "eye-off", HISTORY_EDIT_WITHDRAW_ACTION_ID, Some(time_travel_generation_args(generation)?), !editor.withdrawn)?)
        .map_err(|_| error("time-travel-panel.editor-rows"))?;
    let section = tree_section(ui_label(editor.label.resolve(Terminology::Native, locale), "time-travel-panel.editor-label")?)
        .default_open(true)
        .try_id(scope)
        .map_err(|_| error("time-travel-panel.editor-id"))?
        .try_children(rows)
        .map_err(|_| error("time-travel-panel.editor-rows"))?
        .try_build()
        .map_err(|_| error("time-travel-panel.editor-build"))?;
    let inputs = tree_window_indexed_section(windows, "framework.history.editor.inputs", ui_label(HistoryPanelText::Inputs.text(locale), "time-travel-panel.inputs-label")?, true, editor.rows.len(), |index| {
        time_travel_input_row(&editor.rows[index], editor, windows, controller_id, generation, locale)
    })?;
    Ok([section, inputs])
}

/// 🎨️ The colour control of an sRGB vector input — the one place that picks its widget: the UI contract's
/// `color_input` recipe. Its swatch and hex field commit the colour text to `pointer` (a text without alpha keeps the
/// stored alpha); with `alpha`, its opacity slider commits the fourth component to `pointer/3`.
fn time_travel_color_control(value: &DslValue, alpha: bool, id: &str, label: &str, locale: Locale, action: &ActionId, generation: u32, pointer: &str) -> UiAssemblyResult<BuiltNode> {
    let error = ui_assembly_error;
    let labels = ColorInputLabels { hex: Label(UiText::clipped(HistoryPanelText::Hex.text(locale))), alpha: Label(UiText::clipped(HistoryPanelText::Opacity.text(locale))) };
    let alpha_args = time_travel_input_args(generation, &format!("{pointer}/3"))?;
    let bind_hex = |field: InputBuilder| match time_travel_input_args(generation, pointer) {
        Ok(args) => field.try_on_with(Trigger::Commit, action.clone(), args).map_err(|(field, _)| field),
        Err(_) => Err(field),
    };
    let bind_alpha = |slider: SliderBuilder| slider.try_on_with(Trigger::Change, action.clone(), alpha_args).map_err(|(slider, _)| slider);
    let recipe = color_input(Label(UiText::clipped(label)), labels).try_color(&time_travel_color_rgba(value), alpha, bind_hex, bind_alpha).map_err(|_| error("time-travel-panel.color-input"))?;
    Ok(BuiltNode::from(recipe.try_id(id).map_err(|_| error("time-travel-panel.input-id"))?))
}

/// 🛝️ The slider (or dial) of a number input — its [`semio_framework::ActionArgNumberFacets`]: travel, step, look, axis,
/// stored and shown unit, display factor, precision, detents, and the hard limits that make the travel soft.
fn time_travel_slider(value: f64, facets: &semio_framework::ActionArgNumberFacets) -> SliderBuilder {
    let mut builder = slider(value).min(facets.min.unwrap_or(0.0)).max(facets.max.unwrap_or(0.0)).appearance(facets.appearance).scale(facets.scale).limits(facets.limits.clone());
    if let Some(step) = facets.step {
        builder = builder.step(step);
    }
    if let Some(unit) = &facets.unit {
        builder = builder.unit(UiText::clipped(unit));
    }
    if let Some(display_unit) = &facets.display_unit {
        builder = builder.display_unit(UiText::clipped(display_unit));
    }
    if let Some(factor) = facets.display_factor {
        builder = builder.display_factor(factor);
    }
    if let Some(precision) = facets.precision {
        builder = builder.precision(precision);
    }
    for &snap in &facets.snaps {
        builder = builder.try_snap(snap).unwrap_or_else(|builder| builder);
    }
    builder
}

/// 📜️ The text field of a text input — one line, or `LongText` for a multi-line one (design §22.7) — committing on blur.
/// A value longer than one UI text holds is shown clipped and read-only, so a commit never writes the clipped text back.
fn time_travel_text_control(kind: InputKind, shown: &str, id: &str, label: &str, action: ActionId, args: UiValue) -> UiAssemblyResult<BuiltNode> {
    let error = ui_assembly_error;
    input(kind)
        .value(UiText::clipped(shown))
        .commit(UiText::clipped("blur"))
        .disabled(shown.len() > UI_TEXT_MAX_BYTES)
        .try_id(id)
        .map_err(|_| error("time-travel-panel.input-id"))?
        .try_label(label)
        .map_err(|_| error("time-travel-panel.input-label"))?
        .try_on_with(Trigger::Commit, action, args)
        .map_err(|_| error("time-travel-panel.input-binding"))?
        .try_build()
        .map_err(|_| error("time-travel-panel.input"))
}

/// 🎛️ One input row, its id the pointer with `.` for `/` under `framework.history.editor.input` (`….row`): a tree item
/// reading the row's label (a refused value names its reason as the description) holding the control its descriptor
/// derives (`ActionArgDef::control`) as itself (design §22.7: a segmented choice is a segmented select, an icon choice an
/// icon picker, a multi-line text a multi-line field), bound to the row's pointer. A list's row holds Add item (`historyEditInput{edit:
/// insert}` at `<pointer>/-`, disabled at `maxItems`) and reads its item count; a list item's first row holds — or, under
/// its value control, nests — Remove item (`edit: remove` at its pointer, disabled at `minItems`). A reference row and a
/// choice of more options than one select holds ([`UI_FIXED_LIST_ITEMS`]) are tree windows: "Use selection", then one
/// chip per referenced id named by `editor.reference_labels` (removable by index from a reference list), or one row per
/// option. A `nullable` input nests a Clear row ("Clear <label>") that drafts `null` — disabled, and the row described as
/// cleared, while the input already is.
fn time_travel_input_row(row: &TimeTravelInputRow, editor: &TimeTravelEditorPanel, windows: &TreeWindows<'_>, controller_id: &str, generation: u32, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    let error = ui_assembly_error;
    let (arg, value, pointer) = (&row.input, &row.value, row.pointer.as_str());
    let mut shown_unit: Option<String> = None;
    let label = row.label.resolve(Terminology::Native, locale);
    let id = format!("framework.history.editor.input{}", pointer.replace('/', "."));
    let cleared = arg.nullable && value.is_null();
    let refused = editor.refused.as_ref().filter(|(path, _)| path == pointer).map(|(_, reason)| reason.as_str());
    let described = |item: TreeItemBuilder, shown_unit: Option<String>| {
        if let Some(reason) = refused {
            return item.description(UiText::clipped(reason)).tone(Tone::Danger);
        }
        if cleared {
            return item.description(UiText::clipped(HistoryPanelText::Cleared.text(locale)));
        }
        let description = arg.description.as_ref().map(|description| description.resolve(Terminology::Native, locale).to_string());
        match (description, shown_unit) {
            (Some(description), Some(unit)) => item.description(UiText::clipped(&format!("{description} ({unit})"))),
            (Some(description), None) | (None, Some(description)) => item.description(UiText::clipped(&description)),
            (None, None) => item,
        }
    };
    let floor = |item: TimeTravelListItem| item.min.filter(|_| !item.removable).map(|min| time_travel_item_limit_text(min, false, locale));
    let remove_item = |item: TimeTravelListItem| {
        let remove = time_travel_button(
            controller_id,
            &format!("{id}.remove"),
            HistoryPanelText::RemoveItem.text(locale),
            "trash-2",
            HISTORY_EDIT_INPUT_ACTION_ID,
            Some(time_travel_list_edit_args(generation, pointer, HISTORY_EDIT_INPUT_REMOVE)?),
            item.removable,
        )?;
        time_travel_control_row(&format!("{id}.remove.row"), HistoryPanelText::RemoveItem.text(locale), "trash-2", remove, true, None, floor(item).as_deref())
    };
    let clear_row = || {
        let clear_label = HistoryPanelText::ClearInput.text(locale).replace("{label}", label);
        time_travel_button_row(controller_id, &format!("{id}.clear"), &clear_label, "eraser", HISTORY_EDIT_INPUT_ACTION_ID, Some(time_travel_value_args(generation, pointer, UiValue::Null)?), !cleared)
    };
    let item = tree_item(Label(UiText::clipped(label))).try_id(format!("{id}.row")).map_err(|_| error("time-travel-panel.input-row-id"))?;
    if let Some(list) = row.list {
        let add = time_travel_button(
            controller_id,
            &format!("{id}.add"),
            HistoryPanelText::AddItem.text(locale),
            "plus",
            HISTORY_EDIT_INPUT_ACTION_ID,
            Some(time_travel_list_edit_args(generation, &format!("{pointer}/-"), HISTORY_EDIT_INPUT_INSERT)?),
            list.addable,
        )?;
        let mut count = HistoryPanelText::ItemCount.text(locale).replace("{count}", &list.len.to_string());
        if let Some(max) = list.max.filter(|_| !list.addable) {
            count = format!("{count} \u{b7} {}", time_travel_item_limit_text(max, true, locale));
        }
        let mut item = described(item, Some(count)).try_child(add).map_err(|_| error("time-travel-panel.list-row-child"))?;
        if let Some(entry) = row.item {
            item = item.default_open(true).try_child(remove_item(entry)?).map_err(|_| error("time-travel-panel.list-row-remove"))?;
        }
        return item.try_build().map_err(|_| error("time-travel-panel.list-row"));
    }
    if let (Some(entry), ArgSchema::Object { .. }) = (row.item, &arg.schema) {
        let remove = time_travel_button(
            controller_id,
            &format!("{id}.remove"),
            HistoryPanelText::RemoveItem.text(locale),
            "trash-2",
            HISTORY_EDIT_INPUT_ACTION_ID,
            Some(time_travel_list_edit_args(generation, pointer, HISTORY_EDIT_INPUT_REMOVE)?),
            entry.removable,
        )?;
        return described(item, floor(entry)).try_child(remove).map_err(|_| error("time-travel-panel.item-row-child"))?.try_build().map_err(|_| error("time-travel-panel.item-row"));
    }
    match arg.control() {
        ActionArgControl::Reference { domain, many, min_items, .. } => {
            let ids: Vec<String> = match value {
                DslValue::Array(items) => items.iter().filter_map(reference_id_text).collect(),
                other => reference_id_text(other).into_iter().collect(),
            };
            let removable = many && min_items.is_none_or(|min| ids.len() > min as usize);
            let kept = min_items.filter(|_| many && !removable).map(|min| time_travel_item_limit_text(min, false, locale));
            let chips = ids.len().max(1);
            let total = 1 + chips + usize::from(arg.nullable);
            return tree_window_indexed_item(windows, described(item, None), &format!("{id}.row"), true, total, |index| match index {
                0 => time_travel_button_row(
                    controller_id,
                    &format!("{id}.useSelection"),
                    HistoryPanelText::UseSelection.text(locale),
                    "crosshair",
                    HISTORY_EDIT_USE_SELECTION_ACTION_ID,
                    Some(time_travel_input_args(generation, pointer)?),
                    domain.is_some(),
                ),
                index if index > chips => clear_row(),
                _ if ids.is_empty() => tree_item(Label(UiText::clipped(HistoryPanelText::NothingSelected.text(locale))))
                    .dimmed(true)
                    .try_id(format!("{id}.empty"))
                    .map_err(|_| error("time-travel-panel.reference-empty-id"))?
                    .try_build()
                    .map_err(|_| error("time-travel-panel.reference-empty")),
                index => {
                    let reference = &ids[index - 1];
                    let name = editor.reference_labels.get(reference).map_or_else(|| reference.clone(), |label| label.resolve(Terminology::Native, locale).to_string());
                    let chip = format!("{id}.chip.{}", index - 1);
                    if !many {
                        return tree_item(Label(UiText::clipped(&name)))
                            .icon(ui_text("link", "time-travel-panel.chip-icon")?)
                            .try_id(format!("{chip}.row"))
                            .map_err(|_| error("time-travel-panel.chip-id"))?
                            .try_build()
                            .map_err(|_| error("time-travel-panel.chip"));
                    }
                    let remove = time_travel_button(
                        controller_id,
                        &chip,
                        &format!("{} {name}", HistoryPanelText::Remove.text(locale)),
                        "x",
                        HISTORY_EDIT_INPUT_ACTION_ID,
                        Some(time_travel_list_edit_args(generation, &format!("{pointer}/{}", index - 1), HISTORY_EDIT_INPUT_REMOVE)?),
                        removable,
                    )?;
                    time_travel_control_row(&format!("{chip}.row"), &name, "link", remove, true, None, kept.as_deref())
                }
            });
        }
        ActionArgControl::Select { options } | ActionArgControl::Segmented { options } if options.len() > UI_FIXED_LIST_ITEMS => {
            let selected = value.as_str().unwrap_or_default();
            let current = options.iter().find(|option| option.value == selected).map(|option| option.label.resolve(Terminology::Native, locale).to_string());
            let total = options.len() + usize::from(arg.nullable);
            return tree_window_indexed_item(windows, described(item, current), &format!("{id}.row"), true, total, |index| match options.get(index) {
                Some(option) => {
                    let chosen = option.value == selected;
                    let label = option.label.resolve(Terminology::Native, locale);
                    let icon = if chosen { "check" } else { "circle" };
                    let choose = time_travel_button(controller_id, &format!("{id}.option.{index}"), label, icon, HISTORY_EDIT_INPUT_ACTION_ID, Some(time_travel_value_args(generation, pointer, UiValue::Text(UiText::clipped(&option.value)))?), true)?;
                    tree_item(Label(UiText::clipped(label)))
                        .icon(ui_text(icon, "time-travel-panel.option-icon")?)
                        .selected(chosen)
                        .try_id(format!("{id}.option.{index}.row"))
                        .map_err(|_| error("time-travel-panel.option-id"))?
                        .try_child(choose)
                        .map_err(|_| error("time-travel-panel.option-child"))?
                        .try_build()
                        .map_err(|_| error("time-travel-panel.option"))
                }
                None => clear_row(),
            });
        }
        _ => {}
    }
    let action = ActionId::try_v1(controller_id, HISTORY_EDIT_INPUT_ACTION_ID).ok_or_else(|| error("time-travel-panel.input-action"))?;
    let args = time_travel_input_args(generation, pointer)?;
    let number = value.as_f64().unwrap_or(0.0);
    let facets = arg.number_facets(locale).unwrap_or_default();
    let control: BuiltNode = match arg.control() {
        ActionArgControl::Slider { .. } | ActionArgControl::Dial { .. } => time_travel_slider(number, &facets)
            .try_id(&id)
            .map_err(|_| error("time-travel-panel.input-id"))?
            .try_label(label)
            .map_err(|_| error("time-travel-panel.input-label"))?
            .try_on_with(Trigger::Change, action, args)
            .map_err(|_| error("time-travel-panel.input-binding"))?
            .try_build()
            .map_err(|_| error("time-travel-panel.input"))?,
        ActionArgControl::Stepper { .. } => {
            let mut builder = number_stepper(number).step(facets.step.unwrap_or(1.0)).limits(facets.limits.clone());
            if let Some(min) = facets.min {
                builder = builder.min(min);
            }
            if let Some(max) = facets.max {
                builder = builder.max(max);
            }
            if let Some(precision) = facets.precision {
                builder = builder.precision(precision);
            }
            if let Some(unit) = &facets.unit {
                builder = builder.unit(UiText::clipped(unit));
            }
            if let Some(display_unit) = &facets.display_unit {
                builder = builder.display_unit(UiText::clipped(display_unit));
            }
            if let Some(factor) = facets.display_factor {
                builder = builder.display_factor(factor);
            }
            for &snap in &facets.snaps {
                builder = builder.try_snap(snap).unwrap_or_else(|builder| builder);
            }
            builder
                .try_id(&id)
                .map_err(|_| error("time-travel-panel.input-id"))?
                .try_label(label)
                .map_err(|_| error("time-travel-panel.input-label"))?
                .try_on_with(Trigger::Change, action, args)
                .map_err(|_| error("time-travel-panel.input-binding"))?
                .try_build()
                .map_err(|_| error("time-travel-panel.input"))?
        }
        ActionArgControl::Number { .. } => {
            let mut builder = input(InputKind::Number).commit(UiText::clipped("blur")).limits(facets.limits.clone());
            if let Some(min) = facets.min {
                builder = builder.min(min);
            }
            if let Some(max) = facets.max {
                builder = builder.max(max);
            }
            if let Some(step) = facets.step {
                builder = builder.step(step);
            }
            if let Some(precision) = facets.precision {
                builder = builder.precision(precision);
            }
            if let Some(factor) = facets.display_factor {
                builder = builder.display_factor(factor);
            }
            shown_unit = facets.shown_unit().map(ToOwned::to_owned);
            builder
                .number(number)
                .try_id(&id)
                .map_err(|_| error("time-travel-panel.input-id"))?
                .try_label(label)
                .map_err(|_| error("time-travel-panel.input-label"))?
                .try_on_with(Trigger::Commit, action, args)
                .map_err(|_| error("time-travel-panel.input-binding"))?
                .try_build()
                .map_err(|_| error("time-travel-panel.input"))?
        }
        ActionArgControl::Toggle => toggle(value.as_bool().unwrap_or(false))
            .try_id(&id)
            .map_err(|_| error("time-travel-panel.input-id"))?
            .try_label(label)
            .map_err(|_| error("time-travel-panel.input-label"))?
            .try_on_with(Trigger::Change, action, args)
            .map_err(|_| error("time-travel-panel.input-binding"))?
            .try_build()
            .map_err(|_| error("time-travel-panel.input"))?,
        ActionArgControl::Select { options } | ActionArgControl::Segmented { options } => {
            let appearance = if arg.presentation == Some(ArgPresentation::Segmented) { SelectAppearance::Segmented } else { SelectAppearance::Menu };
            let mut builder = select(UiText::clipped(value.as_str().unwrap_or_default())).appearance(appearance);
            for option in options.iter().take(UI_FIXED_LIST_ITEMS) {
                builder = builder.try_item(UiText::clipped(&option.value), Label(UiText::clipped(option.label.resolve(Terminology::Native, locale)))).map_err(|_| error("time-travel-panel.input-options"))?;
            }
            builder
                .try_id(&id)
                .map_err(|_| error("time-travel-panel.input-id"))?
                .try_label(label)
                .map_err(|_| error("time-travel-panel.input-label"))?
                .try_on_with(Trigger::Change, action, args)
                .map_err(|_| error("time-travel-panel.input-binding"))?
                .try_build()
                .map_err(|_| error("time-travel-panel.input"))?
        }
        ActionArgControl::Vector { dims, .. } => {
            let mut builder = vector_input(Label(UiText::clipped(label))).limits(facets.limits.clone());
            if let Some(factor) = facets.display_factor {
                builder = builder.display_factor(factor);
            }
            if let Some(step) = facets.step {
                builder = builder.step(step);
            }
            if let Some(min) = facets.min {
                builder = builder.min(min);
            }
            if let Some(max) = facets.max {
                builder = builder.max(max);
            }
            for &snap in &facets.snaps {
                builder = builder.try_snap(snap).unwrap_or_else(|builder| builder);
            }
            if let Some(precision) = facets.precision {
                builder = builder.precision(precision);
            }
            if let Some(unit) = facets.shown_unit() {
                builder = builder.unit(UiText::clipped(unit));
            }
            for axis in 0..dims {
                let axis_value = value.as_array().and_then(|items| items.get(axis as usize)).and_then(DslValue::as_f64).unwrap_or(0.0);
                let axis_args = time_travel_input_args(generation, &format!("{pointer}/{axis}"))?;
                let axis_action = action.clone();
                builder = builder
                    .try_axis(&format!("{id}.{axis}"), Label(UiText::clipped(&["x", "y", "z", "w"].get(axis as usize).map_or_else(|| axis.to_string(), |name| (*name).to_string()))), axis_value, |field| {
                        field.commit(UiText::clipped("blur")).try_on_with(Trigger::Commit, axis_action, axis_args).map_err(|(field, _)| field)
                    })
                    .map_err(|_| error("time-travel-panel.vector-axis"))?;
            }
            BuiltNode::from(builder.try_id(&id).map_err(|_| error("time-travel-panel.input-id"))?)
        }
        ActionArgControl::Color { alpha } => time_travel_color_control(value, alpha, &id, label, locale, &action, generation, pointer)?,
        ActionArgControl::IconSelect { classifier_kind } => icon_select(UiText::clipped(value.as_str().unwrap_or_default()), UiText::clipped(&classifier_kind))
            .try_id(&id)
            .map_err(|_| error("time-travel-panel.input-id"))?
            .try_label(label)
            .map_err(|_| error("time-travel-panel.input-label"))?
            .try_on_with(Trigger::Change, action, args)
            .map_err(|_| error("time-travel-panel.input-binding"))?
            .try_build()
            .map_err(|_| error("time-travel-panel.input"))?,
        control => {
            let shown = match value {
                DslValue::String(text) => text.clone(),
                other => time_travel_json(other),
            };
            if shown.len() > UI_TEXT_MAX_BYTES {
                shown_unit = Some(HistoryPanelText::TooLong.text(locale).to_string());
            }
            let kind = if matches!(control, ActionArgControl::Multiline) { InputKind::LongText } else { InputKind::Text };
            time_travel_text_control(kind, &shown, &id, label, action, args)?
        }
    };
    let mut item = described(item, shown_unit).try_child(control).map_err(|_| error("time-travel-panel.input-row-child"))?;
    if let Some(entry) = row.item {
        item = item.default_open(true).try_child(remove_item(entry)?).map_err(|_| error("time-travel-panel.input-row-remove"))?;
    }
    if arg.nullable {
        item = item.default_open(true).try_child(clear_row()?).map_err(|_| error("time-travel-panel.input-row-clear"))?;
    }
    item.try_build().map_err(|_| error("time-travel-panel.input-row"))
}
//#endregion 🔖️Panel
