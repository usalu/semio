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
use semio_framework_plugin::{IconName, Label, UiButtonNode, UiControlNode, UiNode, UiSeparatorNode, WindowKindDefinition};
use semio_framework_plugin::{ui_control_to_node, ui_stack_vertical};

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
async fn row_actions(row: &SpaceArtifactRow) -> Vec<TableRowAction> {
    vec![
        TableRowAction { icon_id: IconName::FolderOpen, label: Some(Label::data("Open")), action: space_index_action("openArtifact", Some(serde_json::json!({ "id": row.id }))) },
        TableRowAction { icon_id: IconName::Trash2, label: Some(Label::data("Delete")), action: space_index_action("requestDeleteArtifact", Some(serde_json::json!({ "id": row.id }))) },
    ]
}

/// 📊️ `config` supplies the live presence fold (`presence-heartbeat`/`fold-directory-events`); the ID
/// column's own cell still carries the raw artifact id (unchanged), while the row's OWN identity now
/// separately carries the `artifact:<id>` grammar contract §C0 needs. Split out from `render` (lane
/// 4-F) so the pure table structure stays unit-testable in isolation, same rationale as Home's own
/// `render_rows`/`render` split.
async fn render_table(document: &SSpaceSnapshot, config: &SpaceIndexConfig) -> UiNode {
    let columns = SPACE_INDEX_TABLE_COLUMNS.iter().map(|s| s.to_string()).collect();
    let rows = document
        .artifacts
        .iter()
        .map(|row| TableRow { id: format!("artifact:{}", row.id), cells: space_index_table_row(row, &config.presence_for(&row.id).join(", ")), actions: row_actions(row) })
        .collect();
    TableWindowKit::render_rows(&TableRowsView { columns, rows, actions_label: "Actions".into() })
}

/// 🩹️ **Known framework gap, worked around here** (lane 4-F, out-of-lease root cause — same one
/// documented on Home's own `main/🦀️component.rs` sibling, see that copy for the full
/// `elementFromPoint` evidence): a plugin window's root `UiNode::Stack` renders flush against the
/// window's top edge, but the window's floating tab-strip chrome (z-index 20) overlays that exact
/// strip, so a `Stack`-rooted button there is visually AND interactively covered until the content
/// clears `--window-content-dead-line` (26px) — the same clearance `TableHost` already gets for free.
/// Two empty separators reliably clear it (measured live). Real fix belongs in the interpreter's
/// `UiStackHost` (framework-owned, outside this lane's lease).
async fn window_content_dead_line_spacer() -> UiNode {
    UiNode::Separator(UiSeparatorNode { presence: Default::default(), menu: None })
}

/// 🆕️ ticket §C0 lane 4-F — the `#s-space-create-artifact` toolbar button, always rendered above the
/// table. Dispatches `createArtifact` with no args; `🎮️commands/🌱create-artifact/🦀️component.rs`'s
/// own handler now mirrors Home's `createSpace` "empty args open the dialog" branch (this lane's own
/// addition), so no new dispatch machinery is needed here either — only a real DOM element with the
/// frozen id, reachable directly instead of hunting the command palette.
async fn create_artifact_button() -> UiNode {
    ui_control_to_node(UiControlNode::Button(UiButtonNode {
        id: Some("s-space-create-artifact".into()),
        icon_id: IconName::Plus,
        label: Label::data("Create Artifact"),
        action: space_index_action("createArtifact", None),
        style: None,
        presence: Default::default(),
        menu: None,
    }))
}

pub async fn render(document: &SSpaceSnapshot, config: &SpaceIndexConfig) -> UiNode {
    ui_stack_vertical(vec![window_content_dead_line_spacer(), window_content_dead_line_spacer(), create_artifact_button(), render_table(document, config)])
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn render_produces_a_node_for_the_default_document() {
        let _node = render(&SSpaceSnapshot::default(), &SpaceIndexConfig::default());
    }

    #[test]
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
    #[test]
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
    #[test]
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
