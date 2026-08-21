//! 🏊️ Sourcing curate app — the pool window: the full stock catalogue with filter chrome + drag source.

use crate::artifacts::curate::schema::{available_modules, curated_count, typology_flatten, ModuleCatalogue};
use crate::artifacts::curate::{CurateSnapshot, Filters, SortDirection};
use crate::editor::sourcing::config::SourcingCurateConfig;
use crate::editor::sourcing::terminology::SourcingLabels;
use crate::editor::sourcing::{sourcing_action, SOURCING_CONTROLLER_ID, SOURCING_DRAG_MIME};
use semio_framework_plugin::{
    build_table_scene, table_row_json, ui_stack_vertical, Label, LocalizedLabel, SurfaceKind, TableCell, TableScene, UiInputNode, UiNode, UiNumberStepperNode, UiPresence, UiSelectItem, UiSelectNode, UiToggleNode, WindowKindDefinition, WindowOptions,
};
use serde_json::{json, Value};

//#region 🔖️Constants
pub const SOURCING_CURATE_WINDOW_POOL: &str = "sourcing-pool";
pub const SOURCING_CURATE_BODY_POOL: &str = "sourcing.pool";
const SOURCING_CURATE_SURFACE_POOL: &str = "sourcing.pool.table";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: SOURCING_CURATE_WINDOW_POOL.into(),
        label: LocalizedLabel::native("Pool", "Pool"),
        body_key: SOURCING_CURATE_BODY_POOL.into(),
        surface_kind: SurfaceKind::Table,
        icon_id: "library".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        interactions: Vec::new(),
        utilities: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️FilterBar
pub async fn build_filter_bar(filters: &Filters, modules: &[ModuleCatalogue], labels: &SourcingLabels) -> UiNode {
    let mut children = vec![UiNode::Input(UiInputNode {
        presence: UiPresence::default(),
        id: "sourcing-filter-query".into(),
        input_kind: "text".into(),
        value: filters.query.clone(),
        placeholder: Some(labels.search_placeholder.into()),
        commit: None,
        min: None,
        max: None,
        step: None,
        accept: None,
        on_change: sourcing_action("setFilterQuery", None),
        menu: None,
    })];
    for module in modules {
        let pressed = filters.module_ids.iter().any(|id| id == &module.module_id);
        children.push(UiNode::Toggle(UiToggleNode {
            id: format!("sourcing-filter-module-{}", module.module_id),
            icon_id: "layers".into(),
            text: Some(Label::data(module.label.clone())),
            on_change: sourcing_action("setFilterModule", Some(json!({ "moduleId": module.module_id, "enabled": !pressed }))),
            presence: UiPresence::selected(pressed),
            menu: None,
        }));
    }
    let mut typology_items = vec![UiSelectItem { value: String::new(), label: labels.all_typologies.into() }];
    for module in modules {
        for (path, label) in typology_flatten(&module.typology) {
            typology_items.push(UiSelectItem { value: path.join("/"), label: Label::data(label) });
        }
    }
    children.push(UiNode::Select(UiSelectNode {
        presence: UiPresence::default(),
        id: "sourcing-filter-typology".into(),
        value: filters.typology_path.join("/"),
        items: typology_items,
        placeholder: None,
        on_change: sourcing_action("setFilterTypology", None),
        menu: None,
    }));
    children.push(UiNode::NumberStepper(UiNumberStepperNode {
        presence: UiPresence::default(),
        id: "sourcing-filter-min-availability".into(),
        value: filters.min_availability as f64,
        step: 1.0,
        uniform: true,
        on_absolute: sourcing_action("setFilterMinAvailability", None),
        on_delta: sourcing_action("setFilterMinAvailability", None),
        menu: None,
    }));
    ui_stack_vertical(children)
}
//#endregion 🔖️FilterBar

//#region 🔖️Render
async fn pool_columns_json(labels: &SourcingLabels) -> String {
    json!([
        {"id": "name", "label": labels.col_name.as_str()},
        {"id": "module", "label": labels.col_module.as_str(), "sortable": true},
        {"id": "typology", "label": labels.col_typology.as_str()},
        {"id": "availability", "label": labels.col_availability.as_str(), "sortable": true},
        {"id": "curated", "label": labels.col_curated.as_str()},
    ])
    .to_string()
}

async fn build_pool_table(document: &CurateSnapshot, cfg: &SourcingCurateConfig, labels: &SourcingLabels) -> UiNode {
    let mut filtered = crate::artifacts::curate::schema::filtered_stock(document, &cfg.filters);
    if let Some(sort) = &cfg.filters.sort {
        filtered.sort_by(|a, b| {
            let ordering = match sort.column_id.as_str() {
                "availability" => a.availability.cmp(&b.availability),
                _ => a.name.cmp(&b.name),
            };
            if sort.direction == SortDirection::Desc {
                ordering.reverse()
            } else {
                ordering
            }
        });
    }
    let rows: Vec<Value> = filtered
        .iter()
        .map(|kind| {
            let curated = curated_count(document, &kind.id) as f64;
            table_row_json(
                &kind.id,
                Some(&json!({ "objectId": kind.id })),
                &[
                    ("name", TableCell::Text { value: kind.name.clone() }),
                    ("module", TableCell::Text { value: kind.module_id.clone() }),
                    ("typology", TableCell::Text { value: kind.typology_path.join(" / ") }),
                    ("availability", TableCell::Number { value: kind.availability as f64 }),
                    ("curated", TableCell::Stepper { value: curated, min: 0.0, max: kind.availability as f64, step: 1.0, action: sourcing_action("curateSetCount", Some(json!({ "objectId": kind.id }))) }),
                ],
            )
        })
        .collect();
    let mut scene = TableScene::base(pool_columns_json(labels), serde_json::to_string(&rows).unwrap_or_else(|_| "[]".into()));
    // 🕹️ Row selection is the framework-owned "rows" interaction domain now (ticket
    // 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — `ArtifactApp::render` carries no
    // `InteractionView`, so `scene.selection_json` stays at its default (unset) here.
    scene.row_drag_mime = Some(SOURCING_DRAG_MIME.into());
    scene.drop_action = Some(sourcing_action("dropOnPool", None));
    scene.sort_json = cfg.filters.sort.as_ref().and_then(|sort| serde_json::to_string(sort).ok());
    build_table_scene(SOURCING_CURATE_SURFACE_POOL, SOURCING_CONTROLLER_ID, scene)
}

pub async fn render(document: &CurateSnapshot, cfg: &SourcingCurateConfig, labels: &SourcingLabels) -> UiNode {
    let modules = available_modules();
    ui_stack_vertical(vec![build_filter_bar(&cfg.filters, &modules, labels), build_pool_table(document, cfg, labels)])
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::sourcing::testkit::{new_app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn pool_render_respects_query_filter() {
        let document = crate::artifacts::curate::schema::default_document();
        let cfg = SourcingCurateConfig { filters: Filters { query: "glulam".into(), ..Default::default() }, ..Default::default() };
        let node = build_pool_table(&document, &cfg, crate::editor::sourcing::terminology::sourcing_curate_labels(&SourcingCurateConfig::default()));
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Glulam"));
        assert!(!json.contains("Hollow Core"));
    }

    #[semio_framework_async_macros::async_test]
    async fn pool_stepper_cell_max_equals_availability() {
        let document = crate::artifacts::curate::schema::default_document();
        let cfg = SourcingCurateConfig::default();
        let stock = crate::artifacts::curate::stock_of(&document);
        let kind = &stock[0];
        let node = build_pool_table(&document, &cfg, crate::editor::sourcing::terminology::sourcing_curate_labels(&SourcingCurateConfig::default()));
        let json = serde_json::to_value(&node).unwrap();
        let rows_json = json.pointer("/table/rowsJson").and_then(|value| value.as_str()).unwrap();
        let rows: Vec<Value> = serde_json::from_str(rows_json).unwrap();
        let row = rows.iter().find(|row| row.get("id").and_then(|id| id.as_str()) == Some(kind.id.as_str())).unwrap();
        assert_eq!(row["curated"]["max"].as_f64().unwrap(), kind.availability as f64);
    }

    #[semio_framework_async_macros::async_test]
    async fn filter_bar_module_toggles_encode_pressed_state_as_presence_selected() {
        let filters = Filters { module_ids: vec!["beams".into()], ..Default::default() };
        let modules = available_modules();
        let node = build_filter_bar(&filters, &modules, crate::editor::sourcing::terminology::sourcing_curate_labels(&SourcingCurateConfig::default()));
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("\"id\":\"sourcing-filter-module-beams\""), "beams toggle present: {json}");
        assert!(json.contains("\"id\":\"sourcing-filter-module-windows\""), "windows toggle present: {json}");
        // Selected module uses presence.selected=true; skip_serializing_if drops the default/false case.
        assert!(json.contains("\"selected\":true"), "pressed module encodes selected presence: {json}");
        let beams_idx = json.find("\"id\":\"sourcing-filter-module-beams\"").expect("beams id");
        let windows_idx = json.find("\"id\":\"sourcing-filter-module-windows\"").expect("windows id");
        let beams_slice = &json[beams_idx..beams_idx + 220.min(json.len() - beams_idx)];
        let windows_slice = &json[windows_idx..windows_idx + 220.min(json.len() - windows_idx)];
        assert!(beams_slice.contains("\"selected\":true"), "beams toggle selected: {beams_slice}");
        assert!(!windows_slice.contains("\"selected\":true"), "windows toggle not selected: {windows_slice}");
    }

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_the_table_surface_and_body_key() {
        let def = definition();
        assert_eq!(def.body_key, SOURCING_CURATE_BODY_POOL);
        assert!(matches!(def.surface_kind, SurfaceKind::Table));
    }

    #[semio_framework_async_macros::async_test]
    async fn renders_pool_table_scene() {
        let mut app = new_app();
        assert!(render_body(&mut app, SOURCING_CURATE_BODY_POOL).contains("table"));
    }
}
//#endregion 🧪️Tests
