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
use semio_framework_plugin::app::{table_row_action, table_window_row, TableWindowKit, TreeWindows, WindowKit};
use semio_framework_plugin::{ActionFactory, IconName, LocalizedLabel, WindowKindDefinition};
use semio_framework_ui_contract::{Buildable, HasBase, HasChildren, HasStackLayout};

//#region 🔖️Constants
pub const S_HOME_WINDOW: &str = "s-home-main";
pub const S_HOME_BODY: &str = TableWindowKit::KIND_ID;
/// 🕳️ The empty-catalog message's own reconciliation key. It exists because this node is a CHILD of
/// the window body's stack: see `render_rows_wrapped`'s doc for why an unkeyed built child is a
/// collision, not a convenience.
const S_HOME_EMPTY: &str = "s-home-empty";
/// 🩹️ The two dead-line spacers' keys — see `window_content_dead_line_spacer`.
const S_HOME_DEAD_LINE: [&str; 2] = ["s-home-dead-line-a", "s-home-dead-line-b"];
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

fn home_space_action(action_id: &str, space_id: &str) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    let mut args = semio_framework_plugin::UiMapBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.table.action-args", "fixed table action argument admission failed"))?;
    args.push("spaceId".to_owned(), semio_framework_plugin::UiValue::Text(fixed_text(space_id, "ui.table.space-id")?))
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.table.action-args.space-id", "fixed table action argument admission failed"))?;
    ActionFactory::new(S_HOME_CONTROLLER_ID).action(action_id, Some(semio_framework_plugin::UiValue::Map(args.finish())))
}

fn home_row_action(icon: IconName, label: semio_framework_plugin::LabelText, action_id: &str, space_id: &str) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::RowAction> {
    table_row_action(icon.as_str(), label.as_str(), home_space_action(action_id, space_id)?)
}

/// 🛂️ `openSpace` is offered to every row; the directory-owned lifecycle affordances
/// (rename/share/delete) and the administration pane (`manageSpace`) are offered ONLY when the
/// caller's own current membership role is `author`. Hub origin alone is not a capability: a
/// spectator reaching a control the server correctly rejects is exactly the role blindness this
/// replaces. The pane it opens still renders solely from the server's own capability flags.
fn row_actions(labels: &SHomeLabels, row: &crate::HomeSpaceRow) -> semio_framework_plugin::UiAssemblyResult<Vec<semio_framework_plugin::RowAction>> {
    let mut actions = vec![home_row_action(IconName::FolderOpen, labels.action_open, "openSpace", &row.id)?];
    if row.data_class == "ephemeralLocalOnly" {
        actions.push(home_row_action(IconName::Cloud, labels.action_promote, "promoteToHubSpace", &row.id)?);
        actions.push(home_row_action(IconName::Save, labels.action_persist, "persistLocally", &row.id)?);
        return Ok(actions);
    }
    if row.origin == "hub" && row.role == Some(crate::DirectorySpaceRole::Author) {
        actions.push(home_row_action(IconName::Pencil, labels.action_rename, "renameSpace", &row.id)?);
        actions.push(home_row_action(IconName::Link, labels.action_share, "shareSpace", &row.id)?);
        actions.push(home_row_action(IconName::Trash2, labels.action_delete, "deleteSpace", &row.id)?);
        actions.push(home_row_action(IconName::Users, labels.action_manage, "manageSpace", &row.id)?);
    }
    Ok(actions)
}

/// 🧪️ The pure per-row-list core, split out from `render` so the empty-state branch is unit-testable
/// in ISOLATION from `crate::list_all_space_catalog_entries()`'s process-global catalog singleton.
/// Every space is one `TableRow` record (cells and row actions are props) inside the windowed table
/// kit, so any number of spaces stays inside the window's node budget: the host streams the rows its
/// viewport shows (ticket 26/09/18 U5 §6b — 9 author rows used to fault the whole window at
/// `nodes 129 > 128`). A row's own activation (Enter on the focused row) opens the space.
fn render_rows(rows: &[crate::HomeSpaceRow], table: &HomeTableLabels, actions: &SHomeLabels, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    if rows.is_empty() {
        return semio_framework_ui_contract::text(fixed_label(table.empty_message, "ui.table.empty-label")?)
            .try_id(S_HOME_EMPTY)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.table.empty-id", "empty table id admission failed"))?
            .try_build()
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.table.empty", "empty table text admission failed"));
    }
    let columns = [table.column_name.as_str(), table.column_kind.as_str(), table.column_visibility.as_str(), table.column_members.as_str(), table.column_updated.as_str(), table.column_origin.as_str()];
    TableWindowKit::render_rows(windows, table.table_name.as_str(), &columns, Some(table.column_actions.as_str()), rows, |row| {
        let cells = row.cells(table);
        table_window_row(&format!("space:{}", row.id), &cells.each_ref().map(String::as_str), row_actions(actions, row)?, Some(home_space_action("openSpace", &row.id)?))
    })
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
/// 🔑️ Keyed explicitly for the reason `render_rows_wrapped` documents: nothing in this stack may rely
/// on a positional key, because its siblings are already-built nodes that carry keys of their own.
fn window_content_dead_line_spacer(key: &str) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let mut spacer = semio_framework_ui_contract::BuiltNode::empty_separator();
    spacer.key = semio_framework_plugin::UiText::try_from_str(key).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.window.spacer-id", "window spacer id admission failed"))?;
    Ok(spacer)
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

/// 🔑️ **Every child of this stack carries an explicit key, and that is load-bearing.**
/// `Buildable::try_build()` stamps the ROOT-position key `#0` on any node whose author set no id
/// (`🏗️builder/🦀️.rs:849-853`), while `HasChildren::try_child` only fills a key that is still EMPTY.
/// So a `try_build()`-ed node used as a child arrives pre-keyed `#0` and collides with whichever
/// sibling the parent numbered `#0` — the UI document then refuses the whole tree with
/// `DuplicateSiblingKey`, the surface never publishes, and the window shows a fault box.
/// That is exactly what `s-home-main` did for every signed-out visitor: the empty-catalog message was
/// `try_build()`-ed `#0` and the first spacer was positioned `#0` (ticket 26/09/18, S3 — measured live
/// and pinned by `the_signed_out_window_body_survives_the_component_tree_producer`).
fn render_rows_wrapped(rows: &[crate::HomeSpaceRow], table: &HomeTableLabels, actions: &SHomeLabels, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let table_node = render_rows(rows, table, actions, windows)?;
    let mut children: semio_framework_plugin::UiFixedList<semio_framework_plugin::BuiltNode> = semio_framework_plugin::UiFixedList::default();
    for child in [window_content_dead_line_spacer(S_HOME_DEAD_LINE[0])?, window_content_dead_line_spacer(S_HOME_DEAD_LINE[1])?, create_space_button(actions)?, table_node] {
        children.try_push(child).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.window.children", "fixed window child admission failed"))?;
    }
    semio_framework_ui_contract::column()
        .grow(true)
        .try_children(children)
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.window.children", "fixed window child admission failed"))?
        .try_build()
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.window.build", "window admission failed"))
}

pub fn render(cfg: &HomeConfig, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let table = semio_framework_plugin::resolve_labels::<HomeTableLabels>(view_state);
    let actions = semio_framework_plugin::resolve_labels::<SHomeLabels>(view_state);
    let directory = cfg.directory().map_err(|_| semio_framework_plugin::PluginAssemblyError::new("s.home.directory-projection-malformed", "Home directory projection is invalid"))?;
    // 🪪️ SIGNED OUT IS A STATE, NOT A FAULT. Refusing here (`s.home.session-identity-required`) meant
    // the landing window of the whole product never published for anyone who was not already signed in
    // — which is the ordinary first paint of every hub-configured shell. The host then had an app that
    // declined to render, and the human's later sign-in had nothing to re-render (ticket 26/09/18, S2
    // §3.3: measured live, this was the operative blocker of a usable `s`).
    //
    // A signed-out human simply owns no spaces, so the honest answer is this same window with an empty
    // row set — the table kit already states its own empty case in en+de — next to the shell's own
    // "Sign in" affordance, which is chrome and is mounted on every device regardless of this surface.
    // Nothing here is faked: no row is invented, and the moment an identity arrives the host
    // re-establishes and the real rows replace this.
    //
    // 🌉️ `crate::home_space_rows` is a plugin-root async fn (outside this lease); `render` must
    // stay sync (called synchronously by `HomeApp::render`) — bridged via `resolve_ready`.
    let rows = match crate::home_session_identity(view_state) {
        Some(identity) => semio_framework_plugin::resolve_ready(crate::home_space_rows(&directory, &identity.user_id)),
        None => Vec::new(),
    };
    render_rows_wrapped(&rows, table, actions, &TreeWindows::for_body(view_state, S_HOME_BODY))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
