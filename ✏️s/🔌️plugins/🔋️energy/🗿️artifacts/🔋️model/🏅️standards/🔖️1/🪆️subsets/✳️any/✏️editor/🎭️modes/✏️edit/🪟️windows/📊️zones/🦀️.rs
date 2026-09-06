//! 📊️ Energy model editor — `zones` window: a real, directly editable table of every `crate::model::
//! Model` zone, built from the framework `TableWindowKit` (contract §2.6) — the same row/column shape
//! `crate::artifacts::model::energy_zones_table_from_model` already derives for the artifact's own
//! composed `zones` child, kept in lockstep by hand (both read straight off `crate::model::Zone`).

use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_plugin::app::{TableView, TableWindowKit, WindowKit};
use semio_framework_plugin::{ActionArgDef, ActionDefinition, ActionKind, BuiltNode, LocalizedLabel, UiAssemblyResult, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TableWindowKit::KIND_ID;
pub const BODY_KEY: &str = TableWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
fn action(id: &str, en: &str, de: &str, args: Vec<ActionArgDef>) -> ActionDefinition {
    ActionDefinition::bounded_catalog(id, LocalizedLabel::native(en, de), ActionKind::Mutation).with_args(args)
}

/// 📊️ The three verbs the generic `set-cell` kit action structurally cannot express — a table cell
/// can retarget an existing row but can never add or remove one.
pub fn actions() -> Vec<ActionDefinition> {
    vec![
        action(
            "create-zone",
            "Create zone",
            "Zone anlegen",
            vec![
                ActionArgDef::text("name", LocalizedLabel::native("Name", "Name")).required(),
                ActionArgDef::number("volumeM3", LocalizedLabel::native("Volume (m³)", "Volumen (m³)")).required(),
                ActionArgDef::number("multiplier", LocalizedLabel::native("Multiplier", "Multiplikator")),
                ActionArgDef::toggle("conditioned", LocalizedLabel::native("Conditioned", "Konditioniert")),
            ],
        ),
        action(
            "rename-zone",
            "Rename zone",
            "Zone umbenennen",
            vec![ActionArgDef::number("zone", LocalizedLabel::native("Zone id", "Zonen-Id")).required(), ActionArgDef::text("newName", LocalizedLabel::native("New name", "Neuer Name")).required()],
        ),
        action("delete-zone", "Delete zone", "Zone löschen", vec![ActionArgDef::number("zone", LocalizedLabel::native("Zone id", "Zonen-Id")).required()]),
    ]
}

/// 🧱️ Stitched into the editor manifest by `crate::editor::model::create_energy_model_editor` — the
/// kit's own `set-cell` action is kept and this window's three authored verbs appended.
pub fn definition() -> WindowKindDefinition {
    let kit = TableWindowKit::editable_window_kind();
    let mut actions = kit.actions.clone();
    actions.extend(self::actions());
    WindowKindDefinition { label: LocalizedLabel::native("Zones", "Zonen"), icon_id: "table-2".into(), actions, ..kit }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// ✏️ Real `EnergyModelSnapshot -> UiNode`: one row per `crate::model::Zone`, columns `id`/`name`/
/// `volumeM3`/`multiplier`/`conditioned`/`partOfTotalFloorArea` — every column but `id` is a real
/// `set-cell` edit target (`EnergyModelEditorCommand::SetZoneCell`, keyed by row index into
/// `model.zones`).
pub fn render(document: &EnergyModelSnapshot) -> UiAssemblyResult<BuiltNode> {
    let model = crate::artifacts::model::energy_model(document);
    let columns = vec!["id".to_string(), "name".to_string(), "volumeM3".to_string(), "multiplier".to_string(), "conditioned".to_string(), "partOfTotalFloorArea".to_string()];
    let rows = model.zones.iter().map(|zone| vec![zone.id.0.to_string(), zone.name.clone(), format!("{}", zone.volume_m3), zone.multiplier.to_string(), zone.conditioned.to_string(), zone.part_of_total_floor_area.to_string()]).collect();
    TableWindowKit::render(&TableView { columns, rows })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_a_table_window_keeping_the_kit_action() {
        let def = definition();
        assert_eq!(def.id, WINDOW_KIND_ID);
        assert_eq!(def.body_key, BODY_KEY);
        assert!(def.actions.iter().any(|action| action.id == "set-cell"), "the kit's own edit action must survive");
        assert_eq!(def.actions.len(), 1 + actions().len());
    }

    #[semio_framework_async_macros::async_test]
    async fn every_authored_action_is_localized_in_english_and_german() {
        for action in actions() {
            assert_ne!(action.label.native(), action.label.secondary(), "action {} is not really translated", action.id);
            for arg in &action.args {
                assert_ne!(arg.label.native(), arg.label.secondary(), "arg {} of action {} is not really translated", arg.id, action.id);
            }
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn render_lists_one_row_per_zone() {
        let document = EnergyModelSnapshot::default();
        let table = render(&document).expect("the table window assembles");
        assert_eq!(table.key, WINDOW_KIND_ID);
        assert!(table.children.is_empty());
    }
}
//#endregion 🧪️Tests
