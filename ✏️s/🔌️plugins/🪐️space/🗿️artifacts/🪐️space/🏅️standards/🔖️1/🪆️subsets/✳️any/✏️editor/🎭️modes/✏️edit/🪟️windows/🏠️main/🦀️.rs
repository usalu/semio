//! 🏠️ SpaceIndexEditor — the `main` window: a table of the space's artifacts. Ticket
//! 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS lane 3-F closes the row-id/row-action
//! gap this file used to defer here: `TableWindowKit::render_rows` (additive sibling of `render`) now
//! stamps a real `"artifact:<id>"` row id (contract §C0) that reaches the React DOM as `data-row-id`
//! and the wgpu hit-target's `control_id`, plus real row-action buttons dispatching a normal
//! `ActionDescriptor` through the existing, unmodified `space_index_action` relay.

use crate::standards::v1::subsets::any::schema::snapshot::{SSpaceSnapshot, SpaceArtifactRow, SpaceIndexTableLabels};
use crate::editor::space_index::config::SpaceIndexConfig;
use crate::editor::space_index::space_index_action;
use semio_framework_plugin::app::{table_row_action, table_window_row, TableWindowKit, TreeWindows, WindowKit};
use semio_framework_plugin::plugin_app_close_prelude::Label;
use semio_framework_plugin::{IconName, WindowKindDefinition};
use semio_framework_ui_contract::{Buildable, HasBase, HasChildren, HasStackLayout};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TableWindowKit::KIND_ID;
pub const BODY_KEY: &str = TableWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 📐️ `TableWindowKit::editable_window_kind()`'s `set-cell` action is unused (the table is a read
/// projection of artifact rows/mutations, not a free-form spreadsheet) but kept — matches the
/// primitive's stock editable shape 1-E chose; no sortable flag exists on `TableView`/`TableWindowKit`
/// (framework-owned, `🔌️plugin/🦀️.rs`, outside this lease) so worker-brief task 1's "sortable
/// if the table primitive supports it" is a documented no-op here.
pub fn definition() -> WindowKindDefinition {
    TableWindowKit::editable_window_kind()
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🕹️ `open` is the only row button wired: it is immediately dispatchable from the Directory-owned
/// row id through the existing `openArtifact` relay. The former delete action targeted the retired
/// whole-vector `SSpaceSnapshot.artifacts` lane and must not be exposed for Directory-owned rows. `rename-artifact`
/// mutates unconditionally on any non-empty `newName` (no "empty argument opens a dialog" two-phase
/// safety the way `os.home`'s `renameSpace` has) and `open-artifact-with` needs a role/plugin/app
/// chooser — neither has a dialog registered on this app (2-B's own dialog list: `createArtifact`,
/// `deleteArtifact`, `inviteMember` only) — wiring either straight to a bare row click would either
/// silently blank a name or dispatch an incomplete open-with call, so both stay UNWIRED here pending a
/// `requestRenameArtifact` opener + `renameArtifact` dialog / an open-with chooser (mirrors the
/// `requestDeleteArtifact`/`deleteArtifact` pair already in this app) — see the lane 3-F report's
/// sharedFileRequest. Every visible string resolves through [`SpaceIndexTableLabels`] in the viewer's
/// language.
fn fixed_text(value: &str, code: &'static str) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiText> {
    semio_framework_plugin::UiText::try_from_str(value).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new(code, "fixed table text admission failed"))
}

fn fixed_label(value: &str, code: &'static str) -> semio_framework_plugin::UiAssemblyResult<Label> {
    Label::try_from(value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new(code, "fixed table label admission failed"))
}

fn open_artifact_action(row: &SpaceArtifactRow) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    let args = crate::editor::space_index::ui_value_map([("id", crate::editor::space_index::ui_value_text(&row.id)?)])?;
    space_index_action("openArtifact", Some(args))
}

/// 📊️ `config` supplies the live presence fold (`presence-heartbeat`/`fold-directory-events`); the ID
/// column's own cell still carries the raw artifact id, while the row's OWN identity carries the
/// `artifact:<id>` grammar contract §C0 needs. One `TableRow` record per artifact inside the windowed
/// table kit, so a space of any size stays inside the window's node budget. Split out from `render`
/// (lane 4-F) so the pure table structure stays unit-testable in isolation.
fn render_table(config: &SpaceIndexConfig, labels: &SpaceIndexTableLabels, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    TableWindowKit::render_rows(windows, labels.table_name.as_str(), &labels.columns(), Some(labels.column_actions.as_str()), &config.indexed_artifacts, |row| {
        let cells = labels.row(row, &config.presence_for(&row.id).join(", "));
        table_window_row(&format!("artifact:{}", row.id), &cells.each_ref().map(String::as_str), [table_row_action(IconName::FolderOpen.as_str(), labels.action_open.as_str(), open_artifact_action(row)?)?], Some(open_artifact_action(row)?))
    })
}

/// 🩹️ **Known framework gap, worked around here** (lane 4-F, out-of-lease root cause — same one
/// documented on Home's own `main/🦀️.rs` sibling, see that copy for the full
/// `elementFromPoint` evidence): a plugin window's root `UiNode::Stack` renders flush against the
/// window's top edge, but the window's floating tab-strip chrome (z-index 20) overlays that exact
/// strip, so a `Stack`-rooted button there is visually AND interactively covered until the content
/// clears `--window-content-dead-line` (26px) — the same clearance `TableHost` already gets for free.
/// Two empty separators reliably clear it (measured live). Real fix belongs in the interpreter's
/// `UiStackHost` (framework-owned, outside this lane's lease).
fn window_content_dead_line_spacer() -> semio_framework_plugin::BuiltNode {
    semio_framework_ui_contract::BuiltNode::empty_separator()
}

/// 🆕️ ticket §C0 lane 4-F — the `#s-space-create-artifact` toolbar button, always rendered above the
/// table. Dispatches `createArtifact` with no args; `🎮️commands/🌱create-artifact/🦀️.rs`'s
/// own handler now mirrors Home's `createSpace` "empty args open the dialog" branch (this lane's own
/// addition), so no new dispatch machinery is needed here either — only a real DOM element with the
/// frozen id, reachable directly instead of hunting the command palette.
fn create_artifact_button(labels: &SpaceIndexTableLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let icon = fixed_text(IconName::Plus.as_str(), "ui.window.create-icon")?;
    let action = space_index_action("createArtifact", None)?;
    let builder = semio_framework_ui_contract::button(fixed_label(labels.create_artifact.as_str(), "ui.window.create-label")?)
        .icon(icon)
        .try_id("s-space-create-artifact")
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.window.create-id", "create button id admission failed"))?;
    let builder = match action.1 {
        Some(args) => builder.try_on_with(semio_framework_plugin::Trigger::Activate, action.0, args),
        None => builder.try_on(semio_framework_plugin::Trigger::Activate, action.0),
    }
    .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.window.create-action", "create button action admission failed"))?;
    builder.try_build().map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.window.create", "create button admission failed"))
}

pub fn render(_document: &SSpaceSnapshot, config: &SpaceIndexConfig, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let labels = semio_framework_plugin::resolve_labels::<SpaceIndexTableLabels>(view_state);
    let mut children = semio_framework_plugin::UiFixedList::<semio_framework_plugin::BuiltNode>::default();
    for child in [window_content_dead_line_spacer(), window_content_dead_line_spacer(), create_artifact_button(labels)?, render_table(config, labels, &TreeWindows::for_body(view_state, BODY_KEY))?] {
        children.try_push(child).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.window.children", "fixed window child admission failed"))?;
    }
    semio_framework_ui_contract::column()
        .grow(true)
        .try_children(children)
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.window.children", "fixed window child admission failed"))?
        .try_build()
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.window.build", "window admission failed"))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
