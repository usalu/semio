//! 🏠️ S Home viewer — the main window: a READ-ONLY render of the SAME overview table the editor's own
//! main window renders (ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS), built from
//! the SAME plugin-root `crate::home_space_rows` helper AND the SAME plugin-root `crate::HomeTableLabels`
//! bilingual label set the editor's own main window uses — this file itself imports nothing from the
//! sibling editor surface (`policyViewerPurityBreaches` forbids it outright); both live at the plugin
//! root precisely so both surfaces can reach them without either importing through the other. Six of
//! the editor's seven columns render here (name/kind/visibility/members/updated/origin) — the trailing
//! "Actions" column is dropped, not left empty: a viewer has no row-scoped affordances to summarize
//! there (contract §2.2, `HomeViewer::handle` is structurally `ViewEmit`-only). No row commands, no
//! create/delete/rename/share affordances: a viewer has no utilities that mutate and emits nothing but
//! `ViewEmit` by construction.

use crate::HomeTableLabels;
use semio_framework_plugin::app::{TableRow, TableRowsView, TableWindowKit, WindowKit};
use semio_framework_plugin::{LocalizedLabel, UiNode, WindowKindDefinition};

//#region 🔖️Constants
pub const S_HOME_VIEW_WINDOW: &str = "s-home-view-main";
pub const S_HOME_VIEW_BODY: &str = TableWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::home::create_home_viewer`.
pub async fn definition() -> WindowKindDefinition {
    let mut def = TableWindowKit::window_kind();
    def.id = S_HOME_VIEW_WINDOW.into();
    def.label = LocalizedLabel::native("Studios", "Studios");
    def
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🧪️ The pure per-row-list core, split out from `render` so the empty-state branch is unit-testable
/// in ISOLATION from `crate::home_space_rows`'s internal union with
/// `crate::list_all_space_catalog_entries()`'s process-global catalog singleton (shared across every
/// test in this crate's test binary — a `DirectoryReadModel::default()` alone is NOT enough to reach an
/// empty row list, since the local catalog half is unconditionally unioned in and never guaranteed
/// empty once any other test has created a studio).
async fn render_rows(rows: &[crate::HomeSpaceRow], labels: &HomeTableLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    if rows.is_empty() {
        return semio_framework_plugin::ui_text(semio_framework_plugin::Label::data(labels.empty_message.as_str().to_string()));
    }
    let empty = semio_framework_plugin::UiText::try_from_str("").ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.table.actions-label", "fixed table label admission failed"))?;
    let mut view = TableRowsView::new(empty);
    for column in [labels.column_name, labels.column_kind, labels.column_visibility, labels.column_members, labels.column_updated, labels.column_origin] {
        let column = semio_framework_plugin::UiText::try_from_str(column.as_str()).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.table.column", "fixed table column admission failed"))?;
        view.try_push_column(column).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.table.columns", "fixed table column admission failed"))?;
    }
    for row in rows {
        let row_id = semio_framework_plugin::UiText::try_format(format_args!("space:{}", row.id))
            .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.table.row-id", "fixed table row id admission failed"))?;
        let mut table_row = TableRow::new(row_id);
        for cell in [&row.name, &row.kind, &row.visibility, &row.members, &row.updated] {
            let cell = semio_framework_plugin::UiText::try_from_str(cell).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.table.cell", "fixed table cell admission failed"))?;
            table_row.try_push_cell(cell).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.table.cells", "fixed table cell admission failed"))?;
        }
        let origin = if row.origin == "hub" { labels.origin_hub.as_str() } else { labels.origin_local.as_str() };
        let origin = semio_framework_plugin::UiText::try_from_str(origin).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.table.origin", "fixed table origin admission failed"))?;
        table_row.try_push_cell(origin).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.table.cells", "fixed table cell admission failed"))?;
        view.try_push_row(table_row).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.table.rows", "fixed table row admission failed"))?;
    }
    // 🆔️ No row has an action (the viewer never renders row affordances), so
    // `TableWindowKit::render_rows` never appends the trailing actions column — `actions_label` is
    // therefore inert here, kept empty rather than reaching for a label nothing displays.
    TableWindowKit::render_rows(view)
}

/// 👁️ No `SHomeSnapshot` argument: exactly like the editor's own main-window render, Home's table rows
/// are derived entirely from `HomeConfig.directory` + the live studio catalog, never from the artifact
/// document itself — see `HomeApp::handle`'s doc comment in the editor for the same observation.
pub async fn render(directory: &store::os_directory::DirectoryReadModel, locale: &str) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let labels = semio_framework_plugin::resolve_labels_for_locale::<HomeTableLabels>(locale);
    render_rows(&crate::home_space_rows(directory), labels).await
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    async fn one_hub_row() -> crate::HomeSpaceRow {
        crate::HomeSpaceRow { id: "sp-1".into(), name: "Fabrication".into(), kind: "studio".into(), visibility: "public".into(), members: "2".into(), updated: "1000".into(), origin: "hub" }
    }

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_the_table_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, S_HOME_VIEW_BODY);
    }

    #[semio_framework_async_macros::async_test]
    async fn empty_rows_render_the_empty_message_not_a_zero_row_table() {
        let json = serde_json::to_string(&render_rows(&[], &HomeTableLabels::NATIVE_EN)).expect("render json");
        assert!(json.contains("No studios yet."));
        assert!(!json.contains("framework.window.table"), "empty rows must not render the table scene at all: {json}");
    }

    #[semio_framework_async_macros::async_test]
    async fn a_row_renders_without_the_actions_column() {
        let json = serde_json::to_string(&render_rows(&[one_hub_row()], &HomeTableLabels::NATIVE_EN)).expect("render json");
        assert!(json.contains("Fabrication"));
        assert!(json.contains("hub"));
        assert!(json.contains("Origin"), "six columns render, the last being Origin: {json}");
        assert!(!json.contains("Actions"), "the viewer never renders an Actions column: {json}");
    }

    /// 🆔️ Contract §C0: even the read-only viewer's rows must carry `data-row-id="space:<id>"` — a
    /// viewer just never attaches row-scoped action buttons to it.
    #[semio_framework_async_macros::async_test]
    async fn a_row_stamps_the_space_row_id() {
        let UiNode::ComponentScene(node) = render_rows(&[one_hub_row()], &HomeTableLabels::NATIVE_EN) else { panic!("expected ComponentScene") };
        let scene = node.table.expect("table scene");
        let rows: Vec<serde_json::Value> = serde_json::from_str(&scene.rows_json).expect("rows_json parses");
        assert_eq!(rows[0]["id"], serde_json::json!("space:sp-1"));
        assert!(rows[0].get("actions").is_none(), "the viewer never carries a row actions cell: {:?}", rows[0]);
    }

    #[semio_framework_async_macros::async_test]
    async fn german_locale_labels_resolve() {
        let json = serde_json::to_string(&render_rows(&[one_hub_row()], &HomeTableLabels::NATIVE_DE)).expect("render json");
        assert!(json.contains("Aktualisiert"));
        assert!(json.contains("Herkunft"));
    }

    #[semio_framework_async_macros::async_test]
    async fn render_with_a_folded_space_renders_a_table_row() {
        let event = store::os_directory::DirectoryEvent {
            seq: 1,
            id: "evt-1".into(),
            hlc: store::os_directory::Hlc { physical_ms: 0, logical: 0 },
            actor: store::os_directory::DirectoryActor { kind: store::os_directory::DirectoryActorKind::User, id: "u".into() },
            space_id: Some("sp-1".into()),
            user_id: None,
            body: store::os_directory::DirectoryEventBody::SpaceCreated {
                space_id: "sp-1".into(),
                name: "Fabrication".into(),
                space_kind: store::os_directory::DirectorySpaceKind::Studio,
                visibility: store::os_directory::DirectorySpaceVisibility::Public,
                owner_user_id: "u1".into(),
            },
            recorded_at_ms: 1000,
        };
        let directory = store::os_directory::fold(store::os_directory::DirectoryReadModel::default(), &event);
        let json = serde_json::to_string(&render(&directory, "en-US")).expect("render json");
        assert!(json.contains("Fabrication"), "the folded space renders: {json}");
        assert!(json.contains("hub"), "hub-folded spaces render origin=hub: {json}");
    }
}
//#endregion 🧪️Tests
