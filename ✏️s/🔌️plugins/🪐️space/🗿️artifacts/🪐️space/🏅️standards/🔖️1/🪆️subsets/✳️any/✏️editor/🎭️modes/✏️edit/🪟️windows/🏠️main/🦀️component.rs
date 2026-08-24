//! 🏠️ SpaceIndexEditor — the `main` window: a table of the space's artifacts. Ticket
//! 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS lane 3-F closes the row-id/row-action
//! gap this file used to defer here: `TableWindowKit::render_rows` (additive sibling of `render`) now
//! stamps a real `"artifact:<id>"` row id (contract §C0) that reaches the React DOM as `data-row-id`
//! and the wgpu hit-target's `control_id`, plus real row-action buttons dispatching a normal
//! `ActionDescriptor` through the existing, unmodified `space_index_action` relay.

use crate::artifacts::space::standards::v1::subsets::any::schema::snapshot::{space_index_table_row, SSpaceSnapshot, SpaceArtifactRow, SPACE_INDEX_TABLE_COLUMNS};
use crate::editor::space_index::config::SpaceIndexConfig;
use crate::editor::space_index::space_index_action;
use semio_framework_plugin::app::{TableRow, TableRowAction, TableRowsView, TableWindowKit, WindowKit};
use semio_framework_plugin::{IconName, Label, WindowKindDefinition};
use semio_framework_ui_contract::{Buildable, HasBase, HasChildren};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TableWindowKit::KIND_ID;
pub const BODY_KEY: &str = TableWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 📐️ `TableWindowKit::editable_window_kind()`'s `set-cell` action is unused (the table is a read
/// projection of artifact rows/mutations, not a free-form spreadsheet) but kept — matches the
/// primitive's stock editable shape 1-E chose; no sortable flag exists on `TableView`/`TableWindowKit`
/// (framework-owned, `🔌️plugin/🦀️component.rs`, outside this lease) so worker-brief task 1's "sortable
/// if the table primitive supports it" is a documented no-op here.
pub async fn definition() -> WindowKindDefinition {
    TableWindowKit::editable_window_kind()
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🕹️ `open`/`delete` are the only row buttons wired: both are immediately dispatchable from just the
/// row's own id through the EXISTING, unmodified relays (`openArtifact`, `requestDeleteArtifact` — the
/// latter already opens the `deleteArtifact` confirm dialog, never mutates directly). `rename-artifact`
/// mutates unconditionally on any non-empty `newName` (no "empty argument opens a dialog" two-phase
/// safety the way `os.home`'s `renameSpace` has) and `open-artifact-with` needs a role/plugin/app
/// chooser — neither has a dialog registered on this app (2-B's own dialog list: `createArtifact`,
/// `deleteArtifact`, `inviteMember` only) — wiring either straight to a bare row click would either
/// silently blank a name or dispatch an incomplete open-with call, so both stay UNWIRED here pending a
/// `requestRenameArtifact` opener + `renameArtifact` dialog / an open-with chooser (mirrors the
/// `requestDeleteArtifact`/`deleteArtifact` pair already in this app) — see the lane 3-F report's
/// sharedFileRequest. Labels are `Label::data` (English-only), the SAME documented, deferred limitation
/// this app's own `📌️panels/👥️members` render already carries (no `locale` field on `SpaceIndexConfig`
/// yet) — not a new gap.
fn fixed_text(value: &str, code: &'static str) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiText> {
    semio_framework_plugin::UiText::try_from_str(value).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new(code, "fixed table text admission failed"))
}

fn artifact_row_action(icon: IconName, label: &'static str, action: &'static str, row: &SpaceArtifactRow) -> semio_framework_plugin::UiAssemblyResult<TableRowAction> {
    let args = crate::editor::space_index::ui_value_map([("id", crate::editor::space_index::ui_value_text(&row.id)?)])?;
    Ok(TableRowAction::new(fixed_text(icon.as_str(), "ui.table.action-icon")?, Label::data(label), space_index_action(action, Some(args))?))
}

fn row_actions(row: &SpaceArtifactRow) -> semio_framework_plugin::UiAssemblyResult<[TableRowAction; 2]> {
    Ok([
        artifact_row_action(IconName::FolderOpen, "Open", "openArtifact", row)?,
        artifact_row_action(IconName::Trash2, "Delete", "requestDeleteArtifact", row)?,
    ])
}

/// 📊️ `config` supplies the live presence fold (`presence-heartbeat`/`fold-directory-events`); the ID
/// column's own cell still carries the raw artifact id (unchanged), while the row's OWN identity now
/// separately carries the `artifact:<id>` grammar contract §C0 needs. Split out from `render` (lane
/// 4-F) so the pure table structure stays unit-testable in isolation, same rationale as Home's own
/// `render_rows`/`render` split.
async fn render_table(document: &SSpaceSnapshot, config: &SpaceIndexConfig) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let mut view = TableRowsView::new(fixed_text("Actions", "ui.table.actions-label")?);
    for column in SPACE_INDEX_TABLE_COLUMNS {
        let column = fixed_text(column, "ui.table.column")?;
        view.try_push_column(column).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.table.columns", "fixed table column admission failed"))?;
    }
    for row in &document.artifacts {
        let row_id = semio_framework_plugin::UiText::try_format(format_args!("artifact:{}", row.id))
            .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.table.row-id", "fixed table row id admission failed"))?;
        let mut table_row = TableRow::new(row_id);
        for cell in space_index_table_row(row, &config.presence_for(&row.id).join(", ")) {
            let cell = semio_framework_plugin::UiText::try_from_string(cell)
                .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.table.cell", "fixed table cell admission failed"))?;
            table_row.try_push_cell(cell).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.table.cells", "fixed table cell admission failed"))?;
        }
        for action in row_actions(row)? {
            table_row.try_push_action(action).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.table.row-actions", "fixed row action admission failed"))?;
        }
        view.try_push_row(table_row).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.table.rows", "fixed table row admission failed"))?;
    }
    TableWindowKit::render_rows(view)
}

/// 🩹️ **Known framework gap, worked around here** (lane 4-F, out-of-lease root cause — same one
/// documented on Home's own `main/🦀️component.rs` sibling, see that copy for the full
/// `elementFromPoint` evidence): a plugin window's root `UiNode::Stack` renders flush against the
/// window's top edge, but the window's floating tab-strip chrome (z-index 20) overlays that exact
/// strip, so a `Stack`-rooted button there is visually AND interactively covered until the content
/// clears `--window-content-dead-line` (26px) — the same clearance `TableHost` already gets for free.
/// Two empty separators reliably clear it (measured live). Real fix belongs in the interpreter's
/// `UiStackHost` (framework-owned, outside this lane's lease).
async fn window_content_dead_line_spacer() -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    semio_framework_ui_contract::separator()
        .try_build()
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.window.spacer", "window spacer admission failed"))
}

/// 🆕️ ticket §C0 lane 4-F — the `#s-space-create-artifact` toolbar button, always rendered above the
/// table. Dispatches `createArtifact` with no args; `🎮️commands/🌱create-artifact/🦀️component.rs`'s
/// own handler now mirrors Home's `createSpace` "empty args open the dialog" branch (this lane's own
/// addition), so no new dispatch machinery is needed here either — only a real DOM element with the
/// frozen id, reachable directly instead of hunting the command palette.
async fn create_artifact_button() -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let icon = fixed_text(IconName::Plus.as_str(), "ui.window.create-icon")?;
    let action = space_index_action("createArtifact", None)?;
    let builder = semio_framework_ui_contract::button(Label::data("Create Artifact"))
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

pub async fn render(document: &SSpaceSnapshot, config: &SpaceIndexConfig) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let mut children = semio_framework_plugin::UiFixedList::default();
    for child in [window_content_dead_line_spacer().await?, window_content_dead_line_spacer().await?, create_artifact_button().await?, render_table(document, config).await?] {
        children.try_push(child).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.window.children", "fixed window child admission failed"))?;
    }
    semio_framework_ui_contract::column()
        .try_children(children)
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.window.children", "fixed window child admission failed"))?
        .try_build()
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.window.build", "window admission failed"))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn render_produces_a_node_for_the_default_document() {
        let _node = render(&SSpaceSnapshot::default(), &SpaceIndexConfig::default());
    }

    #[semio_framework_async_macros::async_test]
    async fn render_reflects_live_presence_for_a_row() {
        use crate::artifacts::space::standards::v1::subsets::any::schema::snapshot::{SpaceArtifactDialect, SpaceArtifactRow};
        use crate::editor::space_index::config::SpaceIndexArtifactPresence;
        let mut document = SSpaceSnapshot::default();
        document.artifacts.push(SpaceArtifactRow { id: "artifact-1".into(), name: "First".into(), dialect: SpaceArtifactDialect { artifact_kind: "s.draw.draw".into(), standard: "1".into(), subset: "*".into() }, ..Default::default() });
        let config = SpaceIndexConfig { presence: vec![SpaceIndexArtifactPresence { artifact_id: "artifact-1".into(), actors_csv: "user:1,user:2".into() }], ..Default::default() };
        let node = render(&document, &config);
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("user:1, user:2"), "presence must reach the table cell: {json}");
    }

    /// 🆔️ Contract §C0: `data-row-id="artifact:<id>"` must reach the table scene's own row id, and the
    /// row's open/delete buttons must be real, dispatchable `ActionDescriptor`s carrying the row's own
    /// id — per ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS lane 3-F.
    #[semio_framework_async_macros::async_test]
    async fn a_row_stamps_the_artifact_row_id_and_carries_dispatchable_open_and_delete_buttons() {
        use crate::artifacts::space::standards::v1::subsets::any::schema::snapshot::{SpaceArtifactDialect, SpaceArtifactRow};
        let mut document = SSpaceSnapshot::default();
        document.artifacts.push(SpaceArtifactRow { id: "artifact-1".into(), name: "First".into(), dialect: SpaceArtifactDialect { artifact_kind: "s.draw.draw".into(), standard: "1".into(), subset: "*".into() }, ..Default::default() });
        let UiNode::ComponentScene(node) = render_table(&document, &SpaceIndexConfig::default()) else { panic!("expected ComponentScene") };
        let scene = node.table.expect("table scene");
        let rows: Vec<serde_json::Value> = serde_json::from_str(&scene.rows_json).expect("rows_json parses");
        assert_eq!(rows[0]["id"], serde_json::json!("artifact:artifact-1"), "row id must carry the frozen artifact:<id> grammar: {rows:?}");
        let buttons = rows[0]["actions"]["buttons"].as_array().expect("actions cell has buttons");
        assert_eq!(buttons.len(), 2, "open + delete: {buttons:?}");
        let open_button = buttons.iter().find(|button| button["action"]["action"] == "openArtifact").expect("open button present");
        assert_eq!(open_button["action"]["args"]["id"], serde_json::json!("artifact-1"));
        let delete_button = buttons.iter().find(|button| button["action"]["action"] == "requestDeleteArtifact").expect("delete button present");
        assert_eq!(delete_button["action"]["args"]["id"], serde_json::json!("artifact-1"));
    }

    /// 🆔️ Contract §C0 lane 4-F: `render(...)` must wrap the table in a real button carrying the
    /// frozen `s-space-create-artifact` id, dispatching `createArtifact` with no args — the harness
    /// clicks this directly instead of hunting the command palette.
    #[semio_framework_async_macros::async_test]
    async fn render_wraps_the_table_with_a_real_create_artifact_button() {
        use crate::editor::space_index::SPACE_INDEX_CONTROLLER_ID;
        let UiNode::Stack(stack) = render(&SSpaceSnapshot::default(), &SpaceIndexConfig::default()) else { panic!("expected a Stack wrapping button + table") };
        let button = stack.children.iter().find_map(|child| if let UiNode::Button(button) = child { Some(button) } else { None }).expect("a create-artifact button somewhere in the stack");
        assert_eq!(button.id.as_deref(), Some("s-space-create-artifact"));
        assert_eq!(button.action.controller_id, SPACE_INDEX_CONTROLLER_ID);
        assert_eq!(button.action.action, "createArtifact");
        assert!(button.action.args.is_none(), "an empty-args dispatch is what makes the handler open the dialog");
    }
}
//#endregion 🧪️Tests
