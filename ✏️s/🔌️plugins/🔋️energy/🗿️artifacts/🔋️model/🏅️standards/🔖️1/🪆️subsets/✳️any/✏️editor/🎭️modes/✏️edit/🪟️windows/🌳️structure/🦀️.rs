//! 🌳️ Energy model editor — `structure` window: a real overview tree of the working `crate::model::
//! Model` behind the artifact's composed `structure` child, built from the framework `TreeWindowKit`
//! (contract §2.6). Two addressable edit-target leaves, `name`/`version` — the collection-size leaves
//! below them are a real read overview, not yet individually addressable (see the surface root's
//! `EnergyModelEditorCommand::SetStructureField` doc comment for the honest scope note).

use crate::EnergyModelSnapshot;
use semio_framework_plugin::app::{TreeWindowKit, WindowKit};
use semio_framework_plugin::{ActionArgDef, ActionDefinition, ActionKind, BuiltNode, LocalizedLabel, UiAssemblyResult, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TreeWindowKit::KIND_ID;
pub const BODY_KEY: &str = TreeWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🎬️ One authored document verb. Execution authority is granted by the surface root's registered
/// factory + proof, and `create_energy_model_editor` stamps `Migrated` on every retained tool id.
fn action(id: &str, en: &str, de: &str, args: Vec<ActionArgDef>) -> ActionDefinition {
    ActionDefinition::bounded_catalog(id, LocalizedLabel::native(en, de), ActionKind::Mutation).with_args(args)
}

/// 🌳️ The authored verbs this window owns beside the kit's generic `set-node` — surface, material,
/// thermostat and site editing, each addressing its target by the model's own `EntityId`.
pub fn actions() -> Vec<ActionDefinition> {
    vec![
        action(
            "create-surface",
            "Create surface",
            "Fläche anlegen",
            vec![
                ActionArgDef::text("name", LocalizedLabel::native("Name", "Bezeichnung")).required(),
                ActionArgDef::number("zone", LocalizedLabel::native("Zone id", "Zonen-Id")).required(),
                ActionArgDef::number("construction", LocalizedLabel::native("Construction id", "Konstruktions-Id")).required(),
                ActionArgDef::text("class", LocalizedLabel::native("Surface class", "Flächenklasse")),
            ],
        ),
        action("delete-surface", "Delete surface", "Fläche löschen", vec![ActionArgDef::number("surface", LocalizedLabel::native("Surface id", "Flächen-Id")).required()]),
        action(
            "assign-surface-construction",
            "Assign construction",
            "Konstruktion zuweisen",
            vec![ActionArgDef::number("surface", LocalizedLabel::native("Surface id", "Flächen-Id")).required(), ActionArgDef::number("construction", LocalizedLabel::native("Construction id", "Konstruktions-Id")).required()],
        ),
        action(
            "set-material-property",
            "Set material property",
            "Materialeigenschaft setzen",
            vec![
                ActionArgDef::number("material", LocalizedLabel::native("Material id", "Material-Id")).required(),
                ActionArgDef::text("property", LocalizedLabel::native("Property", "Eigenschaft")).required(),
                ActionArgDef::number("value", LocalizedLabel::native("Value", "Wert")).required(),
            ],
        ),
        action(
            "set-thermostat-setpoints",
            "Set thermostat setpoints",
            "Thermostat-Sollwerte setzen",
            vec![
                ActionArgDef::number("thermostat", LocalizedLabel::native("Thermostat id", "Thermostat-Id")).required(),
                ActionArgDef::number("heatingSchedule", LocalizedLabel::native("Heating setpoint schedule", "Heiz-Sollwertprofil")).required(),
                ActionArgDef::number("coolingSchedule", LocalizedLabel::native("Cooling setpoint schedule", "Kühl-Sollwertprofil")).required(),
                ActionArgDef::number("heatingThrottleRangeK", LocalizedLabel::native("Heating throttle range (K)", "Heiz-Regelbereich (K)")),
                ActionArgDef::number("coolingThrottleRangeK", LocalizedLabel::native("Cooling throttle range (K)", "Kühl-Regelbereich (K)")),
            ],
        ),
        action(
            "set-site",
            "Set site",
            "Standort setzen",
            vec![
                ActionArgDef::slider("latitudeDeg", LocalizedLabel::native("Latitude (°)", "Breitengrad (°)"), -90.0, 90.0).required(),
                ActionArgDef::slider("longitudeDeg", LocalizedLabel::native("Longitude (°)", "Längengrad (°)"), -180.0, 180.0).required(),
                ActionArgDef::number("elevationM", LocalizedLabel::native("Elevation (m)", "Höhe (m)")),
                ActionArgDef::number("timeZoneHours", LocalizedLabel::native("Time zone (h)", "Zeitzone (h)")),
                ActionArgDef::number("northAxisDeg", LocalizedLabel::native("North axis (°)", "Nordachse (°)")),
            ],
        ),
    ]
}

/// 🧱️ Stitched into the editor manifest by `crate::editor::model::create_energy_model_editor`. The
/// kit's own `set-node` action is KEPT and this window's authored verbs are appended to it — the
/// builder's `window_kind_actions` REPLACES a window's action list, so composing here is the only
/// way both survive.
pub fn definition() -> WindowKindDefinition {
    let kit = TreeWindowKit::editable_window_kind();
    let mut actions = kit.actions.clone();
    actions.extend(self::actions());
    WindowKindDefinition { label: LocalizedLabel::native("Structure", "Struktur"), icon_id: "list-tree".into(), actions, ..kit }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// ✏️ Real `EnergyModelSnapshot -> UiNode`: `name`/`version` (the two `set-node`-editable leaves),
/// the site line, then the grouped collection overview — the one tree `crate::energy_structure_tree`
/// derives for both surfaces, grouped so no node exceeds the kit's 32-sibling ceiling (the 33rd
/// sibling faults `tree-window.siblings` and the window would render empty).
pub fn render(document: &EnergyModelSnapshot) -> UiAssemblyResult<BuiltNode> {
    TreeWindowKit::render(&crate::energy_structure_tree(&crate::energy_model(document)))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
