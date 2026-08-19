//! 📊️ Energy model viewer — `zones` window: a real, READ-ONLY table of every `crate::model::Model`
//! zone, built from the framework `TableWindowKit` (contract §2.6). Same row/column shape the sibling
//! mutation-capable surface renders — independent read, no edit affordances.

use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_plugin::app::{TableView, TableWindowKit, WindowKit};
use semio_framework_plugin::{LocalizedLabel, UiNode, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TableWindowKit::KIND_ID;
pub const BODY_KEY: &str = TableWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::model::create_energy_model_viewer`.
pub async fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Zones", "Zonen"), icon_id: "table-2".into(), ..TableWindowKit::window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Pure `EnergyModelSnapshot -> UiNode` read: one row per `crate::model::Zone`, columns `id`/
/// `name`/`volumeM3`/`multiplier`/`conditioned`/`partOfTotalFloorArea` — no run-output row, no
/// command-driven cell edits (a viewer declares none).
pub async fn render(document: &EnergyModelSnapshot) -> UiNode {
    let model = crate::artifacts::model::energy_model(document);
    let columns = vec!["id".to_string(), "name".to_string(), "volumeM3".to_string(), "multiplier".to_string(), "conditioned".to_string(), "partOfTotalFloorArea".to_string()];
    let rows = model
        .zones
        .iter()
        .map(|zone| vec![zone.id.0.to_string(), zone.name.clone(), format!("{}", zone.volume_m3), zone.multiplier.to_string(), zone.conditioned.to_string(), zone.part_of_total_floor_area.to_string()])
        .collect();
    TableWindowKit::render(&TableView { columns, rows })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn definition_declares_a_table_window() {
        let def = definition();
        assert_eq!(def.id, WINDOW_KIND_ID);
        assert_eq!(def.body_key, BODY_KEY);
    }

    #[test]
    async fn render_lists_one_row_per_zone() {
        let document = EnergyModelSnapshot::default();
        let expected = crate::artifacts::model::energy_model(&document).zones.len();
        let UiNode::ComponentScene(node) = render(&document) else { panic!("expected ComponentScene") };
        let scene = node.table.expect("table scene");
        let rows: Vec<Vec<String>> = serde_json::from_str(&scene.rows_json).expect("rows json");
        assert_eq!(rows.len(), expected);
    }
}
//#endregion 🧪️Tests
