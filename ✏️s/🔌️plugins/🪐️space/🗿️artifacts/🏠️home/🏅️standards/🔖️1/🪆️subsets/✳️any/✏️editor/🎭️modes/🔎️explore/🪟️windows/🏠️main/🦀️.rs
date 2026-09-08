//! 🪟️ S Home launcher app — main window: definition + render (constitutional: ui/WindowKind + Render).
//!
//! 🔁️ Ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS: replaces the pre-ticket
//! virtual-file-system scene with a real overview TABLE of every space (hub-directory UNIONED with the
//! local-only catalog, `crate::home_space_rows` at plugin root — shared with the read-only viewer,
//! which can never import through `::editor::`). Uses the repo's `TableWindowKit` (the same primitive
//! the sibling `s.space` index editor/viewer render with, lane 1-E) for cross-surface consistency.
//! Column/origin/empty-message strings resolve through the plugin-root `crate::HomeTableLabels` (shared
//! with the viewer, contract: en+de for every visible string); the row-scoped action WORDS
//! (open/rename/share/delete) resolve through this editor's own `SHomeLabels`, since the viewer never
//! renders an actions column with real affordances.
//!
//! 🆔️ Ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS lane 3-F closed the "KNOWN
//! GAP" this file used to document here: `semio_framework_plugin::app::TableWindowKit::render_rows`
//! (additive sibling of `render`, same `TableScene`/`TableCell::Buttons` primitives every hand-built
//! `TableScene` table in this codebase already uses) now stamps a real per-row id — `"space:<id>"`,
//! contract §C0 — that reaches the React DOM as `data-row-id` and the wgpu hit-target's `control_id`,
//! plus real row-scoped action buttons (open always; hub-origin rows additionally rename/share/delete,
//! directory-owned lifecycle, contract §C6) that dispatch a normal `ActionDescriptor` back to this
//! app's controller. Local-only spaces stay open-only until promoted to a hub space.

use crate::editor::home::config::HomeConfig;
use crate::editor::home::terminology::SHomeLabels;
use crate::editor::home::S_HOME_CONTROLLER_ID;
use crate::HomeTableLabels;
use semio_framework_plugin::app::{TableRow, TableRowAction, TableRowsView, TableWindowKit, WindowKit};
use semio_framework_plugin::{ActionFactory, IconName, LocalizedLabel, WindowKindDefinition};
use semio_framework_ui_contract::{Buildable, HasBase, HasChildren};

//#region 🔖️Constants
pub const S_HOME_WINDOW: &str = "s-home-main";
pub const S_HOME_BODY: &str = TableWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Manifest
pub fn definition() -> WindowKindDefinition {
    let mut def = TableWindowKit::editable_window_kind();
    def.id = S_HOME_WINDOW.into();
    def.label = LocalizedLabel::native("Studios", "Studios");
    def
}
//#endregion 🔖️Manifest

//#region 🔖️Render
/// 🕹️ Builds one row's dispatchable action buttons: `openSpace` is always offered; hub-origin rows
/// (directory-owned lifecycle, contract §C6) additionally offer `renameSpace`/`shareSpace`/`deleteSpace`
/// — each dispatched with an empty/absent secondary arg (name/email/confirmed), which their own
/// `handle()` already treats as "open the confirm/staged-form dialog first" (see `🎮️commands/🏷️rename-
/// space`, `🔗️share-space`, `🗑️delete-space`), so a row button never bypasses those dialogs.
fn fixed_text(value: &str, code: &'static str) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiText> {
    semio_framework_plugin::UiText::try_from_str(value).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new(code, "fixed table text admission failed"))
}

/// 🏷️ Resolves an `app_labels!`-checked `LabelText` (locale/terminology already folded) into the
/// contract's own `Label` — the contract crate deliberately has no `From<LabelText>` (see
/// `semio_framework_ui_contract::Label`'s doc), so this is the one bridge point every call site here
/// goes through.
fn fixed_label(value: semio_framework_plugin::LabelText, code: &'static str) -> semio_framework_plugin::UiAssemblyResult<semio_framework_ui_contract::Label> {
    semio_framework_ui_contract::Label::try_from(value.as_str()).map_err(|_| semio_framework_plugin::PluginAssemblyError::new(code, "fixed label admission failed"))
}

fn home_row_action(icon: IconName, label: semio_framework_plugin::LabelText, action_id: &str, space_id: &str) -> semio_framework_plugin::UiAssemblyResult<TableRowAction> {
    let mut args = semio_framework_plugin::UiMapBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.table.action-args", "fixed table action argument admission failed"))?;
    args.push("spaceId".to_owned(), semio_framework_plugin::UiValue::Text(fixed_text(space_id, "ui.table.space-id")?))
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.table.action-args.space-id", "fixed table action argument admission failed"))?;
    let action = ActionFactory::new(S_HOME_CONTROLLER_ID).action(action_id, Some(semio_framework_plugin::UiValue::Map(args.finish())))?;
    Ok(TableRowAction::new(fixed_text(icon.as_str(), "ui.table.action-icon")?, fixed_label(label, "ui.table.action-label")?, action))
}

/// 🛂️ `openSpace` is offered to every row; the directory-owned lifecycle affordances
/// (rename/share/delete) and the administration pane (`manageSpace`) are offered ONLY when the
/// caller's own current membership role is `author`. Hub origin alone is not a capability: a
/// spectator reaching a control the server correctly rejects is exactly the role blindness this
/// replaces. The pane it opens still renders solely from the server's own capability flags.
fn row_actions(labels: &SHomeLabels, row: &crate::HomeSpaceRow) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiFixedList<TableRowAction>> {
    let mut actions = semio_framework_plugin::UiFixedList::default();
    actions.try_push(home_row_action(IconName::FolderOpen, labels.action_open, "openSpace", &row.id)?).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.table.row-actions", "fixed row action admission failed"))?;
    if row.origin == "hub" && row.role == Some(crate::DirectorySpaceRole::Author) {
        for action in [
            home_row_action(IconName::Pencil, labels.action_rename, "renameSpace", &row.id)?,
            home_row_action(IconName::Link, labels.action_share, "shareSpace", &row.id)?,
            home_row_action(IconName::Trash2, labels.action_delete, "deleteSpace", &row.id)?,
            home_row_action(IconName::Users, labels.action_manage, "manageSpace", &row.id)?,
        ] {
            actions.try_push(action).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.table.row-actions", "fixed row action admission failed"))?;
        }
    }
    Ok(actions)
}

/// 🧪️ The pure per-row-list core, split out from `render` so the empty-state branch is unit-testable
/// in ISOLATION from `crate::list_all_space_catalog_entries()`'s process-global catalog singleton
/// (shared across every test in this crate's test binary — genuinely never guaranteed empty once any
/// other test has created a studio, which is why `render` itself cannot be probed for "empty" reliably).
fn render_rows(rows: &[crate::HomeSpaceRow], table: &HomeTableLabels, actions: &SHomeLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    if rows.is_empty() {
        return semio_framework_plugin::built_text_node(semio_framework_plugin::Label::data(table.empty_message.as_str().to_string()))
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.table.empty", "empty table text admission failed"));
    }
    let mut view = TableRowsView::new(fixed_text(table.column_actions.as_str(), "ui.table.actions-label")?);
    for column in [table.column_name, table.column_kind, table.column_visibility, table.column_members, table.column_updated, table.column_origin] {
        let column = fixed_text(column.as_str(), "ui.table.column")?;
        view.try_push_column(column).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.table.columns", "fixed table column admission failed"))?;
    }
    for row in rows {
        let row_id = semio_framework_plugin::UiText::try_format(format_args!("space:{}", row.id)).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.table.row-id", "fixed table row id admission failed"))?;
        let mut table_row = TableRow::new(row_id);
        for cell in [&row.name, &row.kind, &row.visibility, &row.members, &row.updated] {
            let cell = fixed_text(cell, "ui.table.cell")?;
            table_row.try_push_cell(cell).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.table.cells", "fixed table cell admission failed"))?;
        }
        let origin = if row.origin == "hub" { table.origin_hub.as_str() } else { table.origin_local.as_str() };
        let origin = fixed_text(origin, "ui.table.origin")?;
        table_row.try_push_cell(origin).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.table.cells", "fixed table cell admission failed"))?;
        for action in row_actions(actions, row)? {
            table_row.try_push_action(action).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.table.row-actions", "fixed row action admission failed"))?;
        }
        view.try_push_row(table_row).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.table.rows", "fixed table row admission failed"))?;
    }
    TableWindowKit::render_rows(view)
}

/// 🆕️ ticket §C0 lane 4-F — the `#s-home-create-space` toolbar button, always rendered above the
/// table (even on the empty state, since an empty catalog is exactly when a dev most needs it).
/// Dispatches `createSpace` with no args, through the SAME `onAction` → `handleAction` path every
/// row action already uses; `🎮️commands/🌱create-space/🦀️.rs`'s own handler already treats
/// an empty `name` as "open the dialog first" (`empty_name_opens_the_dialog_instead_of_relaying`),
/// so this button needs no new dispatch machinery, only a real DOM element with the frozen id.
/// 🩹️ **Known framework gap, worked around here** (lane 4-F, out-of-lease root cause): a plugin
/// window's root `UiNode::Stack` renders flush against the window's top edge, but the window's OWN
/// floating tab-strip chrome (z-index 20) occupies that same top strip as an overlay — confirmed via
/// live `elementFromPoint` probing: a `Stack`-rooted button at the very top is visually and
/// interactively covered by the tab strip until the content clears `26px`
/// (`getComputedStyle(...).getPropertyValue("--window-content-dead-line")`), the SAME clearance
/// `ComponentSceneHost`/`TableHost` already gets for free (its own top-level wrapper applies it) but a
/// bare `UiNode::Stack` root does not. The real fix belongs in the interpreter's `UiStackHost`
/// (`🟦️Interpreter/🟦️.tsx`, framework-owned, outside this lane's lease) — applying
/// `padding-top: var(--window-content-dead-line)` to a window body's ROOT stack the same way table
/// hosts already get it. Two empty separators (measured: ~6.4px of clearance each from the stack's own
/// `gap-double`) reliably clear the dead-line with margin; confirmed live via Playwright-style
/// `elementFromPoint` hit-testing at the button's own center before/after.
fn window_content_dead_line_spacer() -> semio_framework_plugin::BuiltNode {
    semio_framework_ui_contract::BuiltNode::empty_separator()
}

fn create_space_button(actions: &SHomeLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let icon = fixed_text(IconName::Plus.as_str(), "ui.window.create-icon")?;
    let action = ActionFactory::new(S_HOME_CONTROLLER_ID).action("createSpace", None)?;
    let builder = semio_framework_ui_contract::button(fixed_label(actions.action_create, "ui.window.create-label")?)
        .icon(icon)
        .try_id("s-home-create-space")
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.window.create-id", "create button id admission failed"))?;
    let builder = match action.1 {
        Some(args) => builder.try_on_with(semio_framework_plugin::Trigger::Activate, action.0, args),
        None => builder.try_on(semio_framework_plugin::Trigger::Activate, action.0),
    }
    .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.window.create-action", "create button action admission failed"))?;
    builder.try_build().map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.window.create", "create button admission failed"))
}

fn render_rows_wrapped(rows: &[crate::HomeSpaceRow], table: &HomeTableLabels, actions: &SHomeLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let table_node = render_rows(rows, table, actions)?;
    let mut children: semio_framework_plugin::UiFixedList<semio_framework_plugin::BuiltNode> = semio_framework_plugin::UiFixedList::default();
    for child in [window_content_dead_line_spacer(), window_content_dead_line_spacer(), create_space_button(actions)?, table_node] {
        children.try_push(child).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.window.children", "fixed window child admission failed"))?;
    }
    semio_framework_ui_contract::column()
        .try_children(children)
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.window.children", "fixed window child admission failed"))?
        .try_build()
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.window.build", "window admission failed"))
}

pub fn render(cfg: &HomeConfig, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let table = semio_framework_plugin::resolve_labels::<HomeTableLabels>(view_state);
    let actions = semio_framework_plugin::resolve_labels::<SHomeLabels>(view_state);
    let directory = cfg.directory().map_err(|_| semio_framework_plugin::PluginAssemblyError::new("s.home.directory-projection-malformed", "Home directory projection is invalid"))?;
    // 🌉️ `crate::home_space_rows` is a plugin-root async fn (outside this lease); `render` must
    // stay sync (called synchronously by `HomeApp::render`) — bridged via `resolve_ready`.
    let rows = semio_framework_plugin::resolve_ready(crate::home_space_rows(&directory, &cfg.client_id));
    render_rows_wrapped(&rows, table, actions)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
