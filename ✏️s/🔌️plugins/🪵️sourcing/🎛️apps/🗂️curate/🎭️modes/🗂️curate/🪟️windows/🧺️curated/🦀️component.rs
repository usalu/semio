//! 🧺️ Sourcing curate app — the curated window: the currently-picked objects and their counts.

use crate::apps::curate::config::{selection_json_for, SourcingCurateConfig};
use crate::apps::curate::terminology::SourcingLabels;
use crate::apps::curate::{sourcing_action, SOURCING_CONTROLLER_ID, SOURCING_DRAG_MIME};
use crate::artifacts::curate::CurateSnapshot;
use semio_framework_plugin::{build_table_scene, table_row_json, LocalizedLabel, SurfaceKind, TableCell, TableScene, UiNode, UiTreeActionPlacement, UiTreeItemAction, WindowKindDefinition, WindowOptions};
use serde_json::{json, Value};

//#region 🔖️Constants
pub const SOURCING_CURATE_WINDOW_CURATED: &str = "sourcing-curated";
pub const SOURCING_CURATE_BODY_CURATED: &str = "sourcing.curated";
const SOURCING_CURATE_SURFACE_CURATED: &str = "sourcing.curated.table";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: SOURCING_CURATE_WINDOW_CURATED.into(),
        label: LocalizedLabel::native("Curated", "Kuratiert"),
        body_key: SOURCING_CURATE_BODY_CURATED.into(),
        surface_kind: SurfaceKind::Table,
        icon_id: "tags".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(document: &CurateSnapshot, cfg: &SourcingCurateConfig, labels: &SourcingLabels) -> UiNode {
    let columns = json!([
        {"id": "name", "label": labels.col_name.as_str()},
        {"id": "availability", "label": labels.col_availability.as_str()},
        {"id": "count", "label": labels.col_count.as_str()},
        {"id": "actions", "label": ""},
    ])
    .to_string();
    let stock = crate::artifacts::curate::stock_of(document);
    let rows: Vec<Value> = document
        .curated
        .iter()
        .filter_map(|item| {
            let kind = stock.iter().find(|kind| kind.id == item.object_id)?;
            Some(table_row_json(
                &kind.id,
                Some(&json!({ "objectId": kind.id })),
                &[
                    ("name", TableCell::Text { value: kind.name.clone() }),
                    ("availability", TableCell::Number { value: kind.availability as f64 }),
                    ("count", TableCell::Stepper { value: item.count as f64, min: 0.0, max: kind.availability as f64, step: 1.0, action: sourcing_action("curateSetCount", Some(json!({ "objectId": kind.id }))) }),
                    (
                        "actions",
                        TableCell::Buttons {
                            buttons: vec![UiTreeItemAction { icon_id: "trash-2".into(), label: Some(labels.remove.into()), action: sourcing_action("curateRemove", Some(json!({ "objectId": kind.id }))), placement: Some(UiTreeActionPlacement::Row) }],
                        },
                    ),
                ],
            ))
        })
        .collect();
    let mut scene = TableScene::base(columns, serde_json::to_string(&rows).unwrap_or_else(|_| "[]".into()));
    scene.selection_json = Some(selection_json_for(cfg));
    scene.row_drag_mime = Some(SOURCING_DRAG_MIME.into());
    scene.drop_action = Some(sourcing_action("dropOnCurated", None));
    build_table_scene(SOURCING_CURATE_SURFACE_CURATED, SOURCING_CONTROLLER_ID, scene)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::curate::testkit::{new_app, render as render_body};

    #[test]
    fn definition_declares_the_table_surface_and_body_key() {
        let def = definition();
        assert_eq!(def.body_key, SOURCING_CURATE_BODY_CURATED);
        assert!(matches!(def.surface_kind, SurfaceKind::Table));
    }

    #[test]
    fn renders_curated_table_scene() {
        let mut app = new_app();
        assert!(render_body(&mut app, SOURCING_CURATE_BODY_CURATED).contains("table"));
    }
}
//#endregion 🧪️Tests
