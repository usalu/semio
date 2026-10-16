//! 🎮️ Lift file: 1 item(s), about 218 lines, of `🎮️command/🦀️.rs` that belong to crate `time-travel-panel` (WP5).
//!
//! WP3 owns the source file and cuts the items here (ui_history_panel); WP5 wires this file with one `mod` line in the crate root. Until `time-travel-panel` exists, the source module re-exports the items with `pub use`.
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_ui_contract::{BuiltNode, *};
use semio_framework_plugin_decl_views::history_view::{CommandView, HistoryCommandFilter, HistoryView};
use semio_framework_plugin_command::{FRAMEWORK_CHECKIN_CONTROLLER_ID, history_panel_alternative_row, history_panel_icon_id, history_panel_mutation_row, history_panel_unavailable_row, uncommitted_command_count};
use semio_framework_ui_layout::layout::FRAMEWORK_HISTORY_BODY_KEY;
use semio_framework_plugin_command::lifted_command::{HistoryMutationPages, TimeTravelPanel, history_mutation_entry};
use semio_framework::{IconName, REVERT_TO_COMMAND_ACTION_ID, SET_HISTORY_COMMAND_FILTER_ACTION_ID, ViewModel};
use semio_framework_ui_locale::{Locale, Terminology};
use semio_framework_plugin_window_kits_rich::tree_kit::{TreeWindows, tree_window_indexed_item, tree_window_section};
use semio_framework_plugin_interaction::{UiAssemblyResult, ui_assembly_error};
use semio_framework_plugin_window_kits_rich::table_kit::{row_action, row_target};
use crate::tool_jobs_time_travel as time_travel;
use crate::command_history::lifted_panel::ui_history_panel;
use semio_framework_plugin_window_transient::{ui_label, ui_text};

/// 🕰️ Builds the framework's history panel body from a `HistoryView` as a pure side-panel
/// `Tree` (same shape as Document/Catalogue): an Actions section (undo/redo/checkpoint/alternative +
/// filter control) and a Commands section of newest-first rows with optional "inverse" revert.
/// Shared by both renderers — `VcsArtifactApp::render` returns this verbatim for
/// `FRAMEWORK_HISTORY_BODY_KEY`. `read_only` is contract §2.3 clause 3 (`VcsArtifactApp::render`
/// passes `A::ROLE == AppRole::Viewer`): every mutating action row renders disabled and the
/// per-command "Backwards" revert action is omitted entirely, regardless of `can_undo`/`can_redo`
/// or `entry.revertible` — the filter control and Commands list themselves stay fully live, since
/// browsing history is not a mutation.
///
/// 🪟️ The Commands section is a windowed container ([`tree_window_section`]) keyed
/// `framework.history.commands` under body [`FRAMEWORK_HISTORY_BODY_KEY`]: it stamps the full live
/// filtered row count as its `TreeWindow::total` and materialises only the slice the host asked
/// for through `ViewModel::tree_windows` (one viewport on a cold paint). A session log of any
/// length therefore assembles, the scrollbar spans it all, and there is no `+N` row and no guest
/// page cursor.
///
/// 📌️ `#s-checkin` — React's EXPLICIT check-in row (`🏛️ShellHost/🟦️.tsx`'s
/// `framework.history.checkin`), a separate affordance from the no-message quick Checkpoint
/// above and absent outright for a viewer (never disabled — React's own `canCheckIn` gate,
/// whose wgpu twin is `can_check_in`, and which `read_only` carries here). The button keeps
/// React's bare `s-checkin` id: it is the id every battery and probe looks for, and it had no
/// trigger anywhere on the wgpu target at all (`📓️audit-w14-shell-residual.md` C2).
///
/// ⚠️ React's press OPENS a message dialog whose typed text rides the checkpoint; the message
/// is shell-local state this assembler cannot see, so the row dispatches `submit` directly and
/// the shell falls back to its own `"check-in"` message (React's own blank-draft fallback).
///
/// ⏪️ While a history edit is open (`time_travel`), the body leads with the session band
/// (`framework.history.timeTravel`) and, while a mutation is being edited, its draft editor
/// (`framework.history.editor` and its unwindowed inputs), then the alternatives, the actions and the commands;
/// otherwise the actions lead, then the alternatives and the commands. The history-lane actions and the per-row
/// revert are frozen during the edit, and every row's mutations carry the session overlay (replay outcome,
/// pending, edited).
///
/// ↩️ Undo is enabled exactly where its chord and palette entry are: for every editor outside a history edit. The
/// host routes an `undo` to a remote (inference) history it owns, which this guest cannot see, and an undo with
/// nothing to undo is a benign no-op; `can_undo` therefore never disables it here.
///
/// ✏️ A row with applied mutations is a tree window over ALL of them (gap N1): its projected ones first — the ones whose
/// outcome blocks finalizing lead (so the review's next problem is inside the first slice a host materialises, design
/// §22.2), then the other flagged ones — then every other operation of its edit in op order, read from `pages` for the
/// slice a host window opened past the projection ([`time_travel::history_row_mutation_extent`]). Each child carries its
/// label, a description naming its state and severity in words (colour is never the only carrier), tone and icon by
/// severity, and its row actions ([`history_panel_mutation_row`]): Edit, then Withdraw (design §22.1) — or Restore, on a
/// mutation holding an accepted draft — each disabled, naming why, wherever a viewer, the mutation, the store, the
/// session or the instance refuses it ([`time_travel::mutation_row_refusals`]: gap N15, audit W2A-9, design §22.20).
#[allow(clippy::too_many_arguments)]
pub async fn ui_history_panel(
    history: &HistoryView,
    time_travel: Option<&TimeTravelPanel>,
    reprojection: Option<&semio_framework::kernel::HistoryReprojection>,
    pages: &HistoryMutationPages,
    controller_id: &str,
    locale: Locale,
    read_only: bool,
    busy: bool,
    view: &ViewModel,
) -> UiAssemblyResult<BuiltNode> {
    let frozen = time_travel.is_some();
    let text_of = |text: time_travel::HistoryPanelText| text.text(locale);
    let action_item = |id: &str, icon_id: IconName, label: &str, action: &str, enabled: bool| -> UiAssemblyResult<BuiltNode> {
        let action = ActionId::try_v1(controller_id, action).ok_or_else(|| ui_assembly_error("history-panel.action-id"))?;
        let button_id = UiText::try_format(format_args!("{id}.run")).ok_or_else(|| ui_assembly_error("history-panel.button-id"))?;
        let icon = ui_text(icon_id.as_str(), "history-panel.icon")?;
        let builder = button(ui_label(label, "history-panel.button-label")?).icon(icon.clone()).disabled(!enabled);
        let builder = builder.try_id(&button_id).map_err(|_| ui_assembly_error("history-panel.button-id"))?;
        let button = builder.try_on(Trigger::Activate, action).map_err(|_| ui_assembly_error("history-panel.button-binding"))?.try_build().map_err(|_| ui_assembly_error("history-panel.button-build"))?;
        let builder = tree_item(ui_label(label, "history-panel.item-label")?).icon(icon).disabled(!enabled);
        let builder = builder.try_id(id).map_err(|_| ui_assembly_error("history-panel.item-id"))?;
        let builder = builder.try_child(button).map_err(|_| ui_assembly_error("history-panel.item-child"))?;
        builder.try_build().map_err(|_| ui_assembly_error("history-panel.item-build"))
    };

    let mut filter_builder = select(ui_text(history.command_filter.value(), "history-panel.filter-value")?);
    for filter in HistoryCommandFilter::ALL {
        let label = match filter {
            HistoryCommandFilter::All => time_travel::HistoryPanelText::FilterAll,
            HistoryCommandFilter::WithoutMutations => time_travel::HistoryPanelText::FilterWithoutMutations,
            HistoryCommandFilter::OnlyMutations => time_travel::HistoryPanelText::FilterOnlyMutations,
        };
        filter_builder = filter_builder.try_item(ui_text(filter.value(), "history-panel.filter-item-value")?, ui_label(text_of(label), "history-panel.filter-item-label")?).map_err(|_| ui_assembly_error("history-panel.filter-items"))?;
    }
    let filter_builder = filter_builder.try_id("framework.history.filter.control").map_err(|_| ui_assembly_error("history-panel.filter-id"))?;
    let filter_builder = filter_builder.try_label(text_of(time_travel::HistoryPanelText::Filter)).map_err(|_| ui_assembly_error("history-panel.filter-label"))?;
    let filter_action = ActionId::try_v1(controller_id, SET_HISTORY_COMMAND_FILTER_ACTION_ID).ok_or_else(|| ui_assembly_error("history-panel.filter-action"))?;
    let filter_select = filter_builder.try_on(Trigger::Change, filter_action).map_err(|_| ui_assembly_error("history-panel.filter-binding"))?.try_build().map_err(|_| ui_assembly_error("history-panel.filter-build"))?;
    let filter_builder = tree_item(ui_label(text_of(time_travel::HistoryPanelText::Filter), "history-panel.filter-label")?).icon(ui_text(IconName::Filter.as_str(), "history-panel.filter-icon")?);
    let filter_builder = filter_builder.try_id("framework.history.filter").map_err(|_| ui_assembly_error("history-panel.filter-item-id"))?;
    let filter_item = filter_builder.try_child(filter_select).map_err(|_| ui_assembly_error("history-panel.filter-child"))?.try_build().map_err(|_| ui_assembly_error("history-panel.filter-item-build"))?;

    let command_rows: Vec<&CommandView> = history
        .commands
        .iter()
        .filter(|entry| match history.command_filter {
            HistoryCommandFilter::All => true,
            HistoryCommandFilter::WithoutMutations => entry.edit_id.is_none(),
            HistoryCommandFilter::OnlyMutations => entry.edit_id.is_some(),
        })
        .collect();
    let windows = TreeWindows::for_body(view, FRAMEWORK_HISTORY_BODY_KEY);
    let commands_section = tree_window_section(&windows, time_travel::HISTORY_COMMANDS_SECTION_KEY, ui_label(text_of(time_travel::HistoryPanelText::Commands), "history-panel.commands-label")?, true, &command_rows, |entry| {
        let entry_label = entry.label.resolve(Terminology::Native, locale);
        let label = if entry.count > 1 { UiText::clipped(&format!("{entry_label} x{}", entry.count)) } else { UiText::clipped(entry_label) };
        let id = UiText::try_format(format_args!("{}{}", time_travel::HISTORY_ROW_KEY_PREFIX, entry.seq)).ok_or_else(|| ui_assembly_error("history-panel.command-id"))?;
        let mutations: Vec<semio_framework::kernel::HistoryMutationEntry> = entry.mutations.iter().map(|mutation| history_mutation_entry(mutation, time_travel)).collect();
        let worst = mutations.iter().filter_map(|mutation| mutation.worst).max();
        let mut builder = tree_item(Label(label)).icon(ui_text(history_panel_icon_id(entry.kind).as_str(), "history-panel.command-icon")?);
        builder = builder.try_id(&id).map_err(|_| ui_assembly_error("history-panel.command-id"))?;
        let mut description = if entry.mutations.is_empty() { entry.op_lines.join(" \u{b7} ") } else { String::new() };
        if let Some(worst) = worst {
            let severity = time_travel::history_severity_text(worst, locale);
            description = if description.is_empty() { severity.to_string() } else { format!("{severity} \u{b7} {description}") };
            builder = builder.tone(time_travel::history_severity_tone(Some(worst)).0);
        }
        if let Some(author) = entry.author.as_deref().filter(|_| entry.transition_id.is_some()) {
            description = if description.is_empty() { author.to_string() } else { format!("{author} \u{b7} {description}") };
        }
        if !description.is_empty() {
            builder = builder.description(UiText::clipped(&description));
        }
        if (entry.edit_id.is_some() || entry.transition_id.is_some()) && !entry.applied {
            builder = builder.dimmed(true);
        }
        if entry.revertible && !read_only && !frozen {
            let mut args = UiMapBuilder::try_new().ok_or_else(|| ui_assembly_error("history-panel.row-action-args"))?;
            args.push("entrySeq".to_owned(), UiValue::Number(entry.seq as f64)).map_err(|_| ui_assembly_error("history-panel.row-action-args"))?;
            let revert = row_action(IconName::RotateCcw.as_str(), text_of(time_travel::HistoryPanelText::Backwards), REVERT_TO_COMMAND_ACTION_ID, RowActionPlacement::Menu)?;
            builder = builder.try_row_action(revert).map_err(|_| ui_assembly_error("history-panel.row-actions"))?.target(row_target(controller_id, Some(UiValue::Map(args.finish())), None)?);
        }
        let blocking = |mutation: &semio_framework::kernel::HistoryMutationEntry| mutation.worst.is_some_and(|worst| protocol::MergePolicy::Normal.rejects(worst));
        let flagged = |mutation: &semio_framework::kernel::HistoryMutationEntry| mutation.worst.is_some() || mutation.edited || mutation.superseded;
        let mut shown: Vec<&semio_framework::kernel::HistoryMutationEntry> = mutations.iter().filter(|mutation| blocking(*mutation)).collect();
        shown.extend(mutations.iter().filter(|mutation| flagged(*mutation) && !blocking(*mutation)));
        shown.extend(mutations.iter().filter(|mutation| !flagged(*mutation)));
        let (projected, total) = time_travel::history_row_mutation_extent(entry);
        if total == 0 {
            return builder.try_build().map_err(|_| ui_assembly_error("history-panel.command-build"));
        }
        let page = pages.get(&entry.seq);
        tree_window_indexed_item(&windows, builder, id.as_str(), time_travel::history_row_opens_by_default(&mutations), total, |index| {
            let paged;
            let mutation = match shown.get(index) {
                Some(mutation) => *mutation,
                None => match page.and_then(|page| page.rows.get(index.checked_sub(projected + page.from)?)) {
                    Some(view) => {
                        paged = history_mutation_entry(view, time_travel);
                        &paged
                    }
                    None => return history_panel_unavailable_row(entry.seq, index, locale),
                },
            };
            history_panel_mutation_row(mutation, controller_id, locale, read_only, time_travel::mutation_row_refusals(time_travel, busy, &mutation.mutation_id, mutation.store.as_deref()))
        })
    })?;

    let checkin_item = if read_only {
        None
    } else {
        let pending = uncommitted_command_count(history);
        let text = text_of(time_travel::HistoryPanelText::CheckIn);
        let text = if pending > 0 { format!("{text} ({pending})") } else { text.to_string() };
        let icon = ui_text(IconName::GitCommit.as_str(), "history-panel.checkin-icon")?;
        let submit = ActionId::try_v1(FRAMEWORK_CHECKIN_CONTROLLER_ID, "submit").ok_or_else(|| ui_assembly_error("history-panel.checkin-action"))?;
        let builder = button(ui_label(text.as_str(), "history-panel.checkin-label")?).icon(icon.clone()).disabled(frozen);
        let builder = builder.try_id("s-checkin").map_err(|_| ui_assembly_error("history-panel.checkin-button-id"))?;
        let trigger = builder.try_on(Trigger::Activate, submit).map_err(|_| ui_assembly_error("history-panel.checkin-binding"))?.try_build().map_err(|_| ui_assembly_error("history-panel.checkin-button-build"))?;
        let builder = tree_item(ui_label(text.as_str(), "history-panel.checkin-item-label")?).icon(icon).disabled(frozen);
        let builder = builder.try_id("framework.history.checkin").map_err(|_| ui_assembly_error("history-panel.checkin-item-id"))?;
        Some(builder.try_child(trigger).map_err(|_| ui_assembly_error("history-panel.checkin-item-child"))?.try_build().map_err(|_| ui_assembly_error("history-panel.checkin-item-build"))?)
    };
    let mutable = !read_only && !frozen;
    let mut action_items = BuiltChildren::default();
    for item in [
        Some(action_item("framework.history.undo", IconName::Undo, text_of(time_travel::HistoryPanelText::Undo), "undo", mutable)?),
        Some(action_item("framework.history.redo", IconName::Redo, text_of(time_travel::HistoryPanelText::Redo), "redo", history.can_redo && mutable)?),
        Some(action_item("framework.history.commitCheckpoint", IconName::GitCommit, text_of(time_travel::HistoryPanelText::CommitCheckpoint), "commitCheckpoint", mutable)?),
        checkin_item,
        Some(action_item("framework.history.createAlternative", IconName::GitBranch, text_of(time_travel::HistoryPanelText::CreateAlternative), "createAlternative", mutable)?),
        Some(filter_item),
    ]
    .into_iter()
    .flatten()
    {
        action_items.try_push(item).map_err(|_| ui_assembly_error("history-panel.actions"))?;
    }
    let actions_builder = tree_section(ui_label(text_of(time_travel::HistoryPanelText::Actions), "history-panel.actions-label")?).default_open(true);
    let actions_builder = actions_builder.try_id("framework.history.actions").map_err(|_| ui_assembly_error("history-panel.actions-id"))?;
    let actions_section = actions_builder.try_children(action_items).map_err(|_| ui_assembly_error("history-panel.actions"))?.try_build().map_err(|_| ui_assembly_error("history-panel.actions-build"))?;
    let alternatives_section = if history.alternatives.is_empty() {
        None
    } else {
        Some(tree_window_section(&windows, "framework.history.alternatives", ui_label(text_of(time_travel::HistoryPanelText::Alternatives), "history-panel.alternatives-label")?, true, &history.alternatives, |alternative| {
            history_panel_alternative_row(alternative, controller_id, locale, mutable)
        })?)
    };
    let mut sections = BuiltChildren::default();
    let remote_section = match reprojection {
        Some(remote) => Some(time_travel::time_travel_reprojection_section(remote, time_travel.is_none() && !read_only, controller_id, locale)?),
        None => None,
    };
    let ordered: Vec<BuiltNode> = match time_travel {
        Some(panel) => {
            let mut session = vec![time_travel::time_travel_band_section(panel, controller_id, locale)?];
            if let Some(editor) = panel.editor.as_ref().filter(|_| !read_only) {
                session.extend(time_travel::time_travel_editor_sections(panel, editor, &windows, controller_id, locale)?);
            }
            session.into_iter().chain(remote_section).chain(alternatives_section).chain([actions_section, commands_section]).collect()
        }
        None => remote_section.into_iter().chain([actions_section]).chain(alternatives_section).chain([commands_section]).collect(),
    };
    for section in ordered {
        sections.try_push(section).map_err(|_| ui_assembly_error("history-panel.sections"))?;
    }
    let mut panel = tree().try_children(sections).map_err(|_| ui_assembly_error("history-panel.sections"))?.try_build().map_err(|_| ui_assembly_error("history-panel.build"))?;
    panel.layout = LayoutSpec::Scroll(ScrollLayout { axes: ScrollAxes::Vertical, sizing: Sizing::Fill, ..Default::default() });
    Ok(panel)
}
