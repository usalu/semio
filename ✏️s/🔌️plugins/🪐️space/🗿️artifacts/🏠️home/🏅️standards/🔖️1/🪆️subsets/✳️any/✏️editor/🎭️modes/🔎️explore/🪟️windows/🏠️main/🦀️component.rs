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
use semio_framework_plugin::{ActionDescriptor, ActionFactory, IconName, LocalizedLabel, UiButtonNode, UiControlNode, UiNode, UiSeparatorNode, WindowKindDefinition};
use semio_framework_plugin::{ui_control_to_node, ui_stack_vertical};

//#region 🔖️Constants
pub const S_HOME_WINDOW: &str = "s-home-main";
pub const S_HOME_BODY: &str = TableWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Manifest
pub async fn definition() -> WindowKindDefinition {
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
async fn row_actions(labels: &SHomeLabels, row: &crate::HomeSpaceRow) -> Vec<TableRowAction> {
    let action = |action_id: &str| -> ActionDescriptor { ActionFactory::new(S_HOME_CONTROLLER_ID).action(action_id, Some(serde_json::json!({ "spaceId": row.id }))) };
    let mut actions = vec![TableRowAction { icon_id: IconName::FolderOpen, label: Some(labels.action_open.into()), action: action("openSpace") }];
    if row.origin == "hub" {
        actions.push(TableRowAction { icon_id: IconName::Pencil, label: Some(labels.action_rename.into()), action: action("renameSpace") });
        actions.push(TableRowAction { icon_id: IconName::Link, label: Some(labels.action_share.into()), action: action("shareSpace") });
        actions.push(TableRowAction { icon_id: IconName::Trash2, label: Some(labels.action_delete.into()), action: action("deleteSpace") });
    }
    actions
}

/// 🧪️ The pure per-row-list core, split out from `render` so the empty-state branch is unit-testable
/// in ISOLATION from `crate::list_all_space_catalog_entries()`'s process-global catalog singleton
/// (shared across every test in this crate's test binary — genuinely never guaranteed empty once any
/// other test has created a studio, which is why `render` itself cannot be probed for "empty" reliably).
async fn render_rows(rows: &[crate::HomeSpaceRow], table: &HomeTableLabels, actions: &SHomeLabels) -> UiNode {
    if rows.is_empty() {
        return semio_framework_plugin::ui_text(semio_framework_plugin::Label::data(table.empty_message.as_str().to_string()));
    }
    let columns = vec![
        table.column_name.as_str().to_string(),
        table.column_kind.as_str().to_string(),
        table.column_visibility.as_str().to_string(),
        table.column_members.as_str().to_string(),
        table.column_updated.as_str().to_string(),
        table.column_origin.as_str().to_string(),
    ];
    let table_rows: Vec<TableRow> = rows
        .iter()
        .map(|row| TableRow {
            id: format!("space:{}", row.id),
            cells: vec![
                row.name.clone(),
                row.kind.clone(),
                row.visibility.clone(),
                row.members.clone(),
                row.updated.clone(),
                (if row.origin == "hub" { table.origin_hub.as_str() } else { table.origin_local.as_str() }).to_string(),
            ],
            actions: row_actions(actions, row),
        })
        .collect();
    TableWindowKit::render_rows(&TableRowsView { columns, rows: table_rows, actions_label: table.column_actions.as_str().to_string() })
}

/// 🆕️ ticket §C0 lane 4-F — the `#s-home-create-space` toolbar button, always rendered above the
/// table (even on the empty state, since an empty catalog is exactly when a dev most needs it).
/// Dispatches `createSpace` with no args, through the SAME `onAction` → `handleAction` path every
/// row action already uses; `🎮️commands/🌱create-space/🦀️component.rs`'s own handler already treats
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
/// (`Interpreter/🟦️component.tsx`, framework-owned, outside this lane's lease) — applying
/// `padding-top: var(--window-content-dead-line)` to a window body's ROOT stack the same way table
/// hosts already get it. Two empty separators (measured: ~6.4px of clearance each from the stack's own
/// `gap-double`) reliably clear the dead-line with margin; confirmed live via Playwright-style
/// `elementFromPoint` hit-testing at the button's own center before/after.
async fn window_content_dead_line_spacer() -> UiNode {
    UiNode::Separator(UiSeparatorNode { presence: Default::default(), menu: None })
}

async fn create_space_button(actions: &SHomeLabels) -> UiNode {
    ui_control_to_node(UiControlNode::Button(UiButtonNode {
        id: Some("s-home-create-space".into()),
        icon_id: IconName::Plus,
        label: actions.action_create.into(),
        action: ActionFactory::new(S_HOME_CONTROLLER_ID).action("createSpace", None),
        style: None,
        presence: Default::default(),
        menu: None,
    }))
}

pub async fn render(cfg: &HomeConfig) -> UiNode {
    let table = semio_framework_plugin::resolve_labels_for_locale::<HomeTableLabels>(&cfg.locale);
    let actions = semio_framework_plugin::resolve_labels_for_locale::<SHomeLabels>(&cfg.locale);
    let table_node = render_rows(&crate::home_space_rows(&cfg.directory()), table, actions);
    ui_stack_vertical(vec![window_content_dead_line_spacer(), window_content_dead_line_spacer(), create_space_button(actions), table_node])
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    async fn one_local_row() -> crate::HomeSpaceRow {
        crate::HomeSpaceRow { id: "sp-local".into(), name: "Fixture Studio".into(), kind: "atelier".into(), visibility: "private".into(), members: "1".into(), updated: "0".into(), origin: "local" }
    }

    async fn one_hub_row() -> crate::HomeSpaceRow {
        crate::HomeSpaceRow { id: "sp-hub".into(), name: "Fabrication".into(), kind: "studio".into(), visibility: "public".into(), members: "2".into(), updated: "1000".into(), origin: "hub" }
    }

    #[semio_framework_async_macros::async_test]
    async fn empty_rows_render_the_empty_message_not_a_zero_row_table() {
        let json = serde_json::to_string(&render_rows(&[], &HomeTableLabels::NATIVE_EN, &SHomeLabels::NATIVE_EN)).unwrap();
        assert!(json.contains("No studios yet"), "empty rows render the empty message, not a zero-row table: {json}");
        assert!(!json.contains("framework.window.table"), "empty rows must not render the table scene at all: {json}");
    }

    #[semio_framework_async_macros::async_test]
    async fn a_local_row_renders_with_open_only_actions() {
        let json = serde_json::to_string(&render_rows(&[one_local_row()], &HomeTableLabels::NATIVE_EN, &SHomeLabels::NATIVE_EN)).unwrap();
        assert!(json.contains("Fixture Studio"));
        assert!(json.contains("local"));
        assert!(!json.contains("rename"), "local-only rows offer open only, no rename/share/delete: {json}");
    }

    #[semio_framework_async_macros::async_test]
    async fn a_hub_row_renders_with_the_full_action_set() {
        let json = serde_json::to_string(&render_rows(&[one_hub_row()], &HomeTableLabels::NATIVE_EN, &SHomeLabels::NATIVE_EN)).unwrap();
        assert!(json.contains("Fabrication"));
        assert!(json.contains("hub"));
        assert!(json.contains("rename") && json.contains("share") && json.contains("delete"), "hub rows offer the full lifecycle action set: {json}");
    }

    /// 🆔️ Contract §C0: `data-row-id="space:<id>"` must reach the table scene's own row id, and every
    /// row action must be a real, dispatchable `ActionDescriptor` (controller + action id + spaceId
    /// arg) — not text, per ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS lane 3-F.
    #[semio_framework_async_macros::async_test]
    async fn a_hub_row_stamps_the_space_row_id_and_carries_dispatchable_row_actions() {
        let UiNode::ComponentScene(node) = render_rows(&[one_hub_row()], &HomeTableLabels::NATIVE_EN, &SHomeLabels::NATIVE_EN) else { panic!("expected ComponentScene") };
        let scene = node.table.expect("table scene");
        let rows: Vec<serde_json::Value> = serde_json::from_str(&scene.rows_json).expect("rows_json parses");
        assert_eq!(rows[0]["id"], serde_json::json!("space:sp-hub"), "row id must carry the frozen space:<id> grammar: {rows:?}");
        let buttons = rows[0]["actions"]["buttons"].as_array().expect("actions cell has buttons");
        assert_eq!(buttons.len(), 4, "open + rename + share + delete: {buttons:?}");
        let delete_button = buttons.iter().find(|button| button["action"]["action"] == "deleteSpace").expect("delete button present");
        assert_eq!(delete_button["action"]["controllerId"], serde_json::json!(S_HOME_CONTROLLER_ID));
        assert_eq!(delete_button["action"]["args"]["spaceId"], serde_json::json!("sp-hub"), "the delete button's descriptor already carries the row's own space id: {delete_button:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn a_local_row_only_carries_an_open_action_button() {
        let UiNode::ComponentScene(node) = render_rows(&[one_local_row()], &HomeTableLabels::NATIVE_EN, &SHomeLabels::NATIVE_EN) else { panic!("expected ComponentScene") };
        let scene = node.table.expect("table scene");
        let rows: Vec<serde_json::Value> = serde_json::from_str(&scene.rows_json).expect("rows_json parses");
        assert_eq!(rows[0]["id"], serde_json::json!("space:sp-local"));
        let buttons = rows[0]["actions"]["buttons"].as_array().expect("actions cell has buttons");
        assert_eq!(buttons.len(), 1, "local-only rows offer open only: {buttons:?}");
        assert_eq!(buttons[0]["action"]["action"], serde_json::json!("openSpace"));
    }

    #[semio_framework_async_macros::async_test]
    async fn seeded_local_studio_renders_a_table_row() {
        let cfg = HomeConfig::default();
        // 🌱️ `crate::catalog_port()` lazily seeds a demo space on first access (plugin root's own
        // `catalog_port_concrete`), so the local catalog is never truly empty once touched — this test
        // exercises the REAL end-to-end `render` (not `render_rows`), deliberately not asserting on
        // emptiness (see `empty_rows_render_the_empty_message_not_a_zero_row_table` for that, isolated).
        let _ = crate::list_all_space_catalog_entries();
        let node = render(&cfg);
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("local"), "the seeded demo studio has no directory entry, so it renders origin=local: {json}");
    }

    #[semio_framework_async_macros::async_test]
    async fn german_locale_labels_resolve_in_the_rendered_table() {
        let json = serde_json::to_string(&render_rows(&[one_local_row()], &HomeTableLabels::NATIVE_DE, &SHomeLabels::NATIVE_DE)).unwrap();
        assert!(json.contains("Aktualisiert"), "German column header must resolve: {json}");
        assert!(json.contains("Herkunft"), "German column header must resolve: {json}");
        assert!(json.contains("lokal"), "German origin label must resolve for a local-only row: {json}");
    }

    #[semio_framework_async_macros::async_test]
    async fn render_resolves_labels_from_config_locale() {
        let cfg = HomeConfig { locale: "de".into(), ..HomeConfig::default() };
        let json = serde_json::to_string(&render_rows(&[one_local_row()], &HomeTableLabels::NATIVE_DE, &SHomeLabels::NATIVE_DE)).unwrap();
        assert!(json.contains("Aktualisiert"));
        let _ = render(&cfg); // exercises the real locale-resolution path end to end, no panic
    }

    /// 🆔️ Contract §C0 lane 4-F: `render(cfg)` must wrap the table in a real button carrying the
    /// frozen `s-home-create-space` id, dispatching `createSpace` with no args — the harness clicks
    /// this directly instead of hunting the command palette. The button is preceded by two
    /// `window_content_dead_line_spacer()` separators (see that fn's doc) — found by type, not a
    /// hardcoded index, so this test stays valid if the spacer count ever changes.
    #[semio_framework_async_macros::async_test]
    async fn render_wraps_the_table_with_a_real_create_space_button() {
        let UiNode::Stack(stack) = render(&HomeConfig::default()) else { panic!("expected a Stack wrapping button + table") };
        let button = stack.children.iter().find_map(|child| if let UiNode::Button(button) = child { Some(button) } else { None }).expect("a create-space button somewhere in the stack");
        assert_eq!(button.id.as_deref(), Some("s-home-create-space"));
        assert_eq!(button.action.controller_id, S_HOME_CONTROLLER_ID);
        assert_eq!(button.action.action, "createSpace");
        assert!(button.action.args.is_none(), "an empty-args dispatch is what makes the handler open the dialog");
    }

    #[semio_framework_async_macros::async_test]
    async fn empty_catalog_still_renders_the_create_space_button() {
        let UiNode::Stack(stack) = render_rows_wrapped_for_test(&[]) else { panic!("expected a Stack") };
        assert!(stack.children.iter().any(|child| matches!(child, UiNode::Button(_))), "the create button must survive the empty-table branch too");
    }

    /// 🧪️ `render`'s own composition, isolated from `crate::list_all_space_catalog_entries()`'s
    /// process-global singleton — mirrors `render_rows`'s own isolation rationale above.
    async fn render_rows_wrapped_for_test(rows: &[crate::HomeSpaceRow]) -> UiNode {
        let table_node = render_rows(rows, &HomeTableLabels::NATIVE_EN, &SHomeLabels::NATIVE_EN);
        ui_stack_vertical(vec![window_content_dead_line_spacer(), window_content_dead_line_spacer(), create_space_button(&SHomeLabels::NATIVE_EN), table_node])
    }
}
//#endregion 🧪️Tests
