//! 📊️ Energy model editor — `zones` window: a real, directly editable table of every `crate::model::
//! Model` zone, built from the framework `TableWindowKit` (contract §2.6) — the same row/column shape
//! `crate::energy_zones_table_from_model` already derives for the artifact's own
//! composed `zones` child, kept in lockstep by hand (both read straight off `crate::model::Zone`).

use crate::EnergyModelSnapshot;
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
/// 🆕️ `create-zone` — exported for the APP-level roster rather than listed in [`actions`], because
/// the `mod+shift+n` keybinding that reaches it must fire while ANY window is active and an action a
/// window kind declares stops being copied onto the others.
pub fn create_zone_action() -> ActionDefinition {
    action(
        "create-zone",
        "Create zone",
        "Zone anlegen",
        vec![
            ActionArgDef::text("name", LocalizedLabel::native("Name", "Bezeichnung")).required(),
            ActionArgDef::number("volumeM3", LocalizedLabel::native("Volume (m³)", "Volumen (m³)")).required(),
            ActionArgDef::number("multiplier", LocalizedLabel::native("Multiplier", "Multiplikator")),
            ActionArgDef::toggle("conditioned", LocalizedLabel::native("Conditioned", "Konditioniert")),
        ],
    )
}

/// ✏️ `rename-zone` — likewise app-level: the inspector's zone form patches a name through
/// `set-zone-property`, but the palette's own rename verb has to stay reachable everywhere too.
pub fn rename_zone_action() -> ActionDefinition {
    action("rename-zone", "Rename zone", "Zone umbenennen", vec![ActionArgDef::number("zone", LocalizedLabel::native("Zone id", "Zonen-Id")).required(), ActionArgDef::text("newName", LocalizedLabel::native("New name", "Neuer Name")).required()])
}

/// 📊️ Nothing left: both authored verbs are app-level (see above), and the kit's own `set-cell` is
/// this window's by construction. The list stays as the seam a future table-only verb would use.
pub fn actions() -> Vec<ActionDefinition> {
    Vec::new()
}

/// 🧱️ Stitched into the editor manifest by `crate::editor::model::create_energy_model_editor` — the
/// kit's own `set-cell` action is kept and this window's authored verbs appended. `delete-zone` is
/// NOT one of them any more: the inspector panel dispatches it from whatever window is active, and
/// an action declared on a window kind stops being copied onto every other one, so it lives
/// app-level in `crate::editor::model::inspector_action_definitions`.
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
    let model = crate::energy_model(document);
    let columns = vec!["id".to_string(), "name".to_string(), "volumeM3".to_string(), "multiplier".to_string(), "conditioned".to_string(), "partOfTotalFloorArea".to_string()];
    let rows = model.zones.iter().map(|zone| vec![zone.id.0.to_string(), zone.name.clone(), format!("{}", zone.volume_m3), zone.multiplier.to_string(), zone.conditioned.to_string(), zone.part_of_total_floor_area.to_string()]).collect();
    TableWindowKit::render(&TableView { columns, rows })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
