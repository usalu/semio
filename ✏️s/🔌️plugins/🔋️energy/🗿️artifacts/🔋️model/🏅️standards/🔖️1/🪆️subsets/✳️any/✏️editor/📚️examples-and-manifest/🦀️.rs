//! 📚️ The registered examples, the app-level action definitions and the editor manifest constructor.

use super::*;
use semio_s_artifact_stdio_semio::editor::modes::edit;

//#region 📚️Examples
/// 📚️ The bundled `📚️examples/🎬️<slug>` fixtures, table-driven — a new example is ONE row here plus
/// its `#[path]` mount in `📦️packages/🦀️rust/🦀️.rs`. Every row names a module that exports the same
/// three items (`ID`, `LABEL_EN`, `model()`), so the table cannot go out of step with a leaf.
/// The ANSI/ASHRAE 140 case ids are proper nouns and read identically in both authored languages;
/// `Demo` likewise. That is a real translation decision, not a missing one.
pub(super) fn example_rows() -> Vec<(&'static str, &'static str, crate::model::Model)> {
    use crate::examples::{bestest_600, bestest_600ff, bestest_610, bestest_620, bestest_630, bestest_640, bestest_650, bestest_900, bestest_900ff, bestest_910, bestest_920, bestest_930, bestest_940, bestest_950, demo};
    vec![
        (demo::ID, demo::LABEL_EN, demo::model()),
        (bestest_600::ID, bestest_600::LABEL_EN, bestest_600::model()),
        (bestest_600ff::ID, bestest_600ff::LABEL_EN, bestest_600ff::model()),
        (bestest_610::ID, bestest_610::LABEL_EN, bestest_610::model()),
        (bestest_620::ID, bestest_620::LABEL_EN, bestest_620::model()),
        (bestest_630::ID, bestest_630::LABEL_EN, bestest_630::model()),
        (bestest_640::ID, bestest_640::LABEL_EN, bestest_640::model()),
        (bestest_650::ID, bestest_650::LABEL_EN, bestest_650::model()),
        (bestest_900::ID, bestest_900::LABEL_EN, bestest_900::model()),
        (bestest_900ff::ID, bestest_900ff::LABEL_EN, bestest_900ff::model()),
        (bestest_910::ID, bestest_910::LABEL_EN, bestest_910::model()),
        (bestest_920::ID, bestest_920::LABEL_EN, bestest_920::model()),
        (bestest_930::ID, bestest_930::LABEL_EN, bestest_930::model()),
        (bestest_940::ID, bestest_940::LABEL_EN, bestest_940::model()),
        (bestest_950::ID, bestest_950::LABEL_EN, bestest_950::model()),
    ]
}

/// 📚️ Catalogue `EnergyModelEditor::examples` returns. `.editor` stamps it onto the manifest,
/// and the navbar dropdown reads that list.
pub fn examples() -> Vec<ExampleSource> {
    example_rows()
        .into_iter()
        .map(|(id, label, model)| {
            let snapshot = crate::snapshot::energy_snapshot_with_state(ENERGY_MODEL_DOCUMENT_SCHEMA, &model, None);
            ExampleSource::new(id, LocalizedLabel::native(label, label), semio_framework_pack_json::to_json_string(&snapshot), "file")
        })
        .collect()
}

/// 📚️ The model behind one bundled example id — the read `setActiveExample` reduces against.
pub(super) fn example_model(example_id: &str) -> Option<crate::model::Model> {
    example_rows().into_iter().find(|(id, _, _)| *id == example_id).map(|(_, _, model)| model)
}

/// 📚️ The picker's option list, in `example_rows` order.
pub(super) fn example_options() -> Vec<semio_framework_plugin::ActionArgOption> {
    example_rows().into_iter().map(|(id, label, _)| semio_framework_plugin::ActionArgOption { value: id.to_string(), label: LocalizedLabel::native(label, label) }).collect()
}
//#endregion 📚️Examples


//#region 🔍️InspectorActions


/// 🎬️ Every action this editor declares APP-level, in one roster: `build_definition` copies each of
/// them onto every window kind, which is what makes a panel control and a keybinding dispatchable
/// whatever window is active. `setActiveExample` is app-level too, through `.mutation(…)`.
pub fn app_level_action_definitions() -> Vec<semio_framework_plugin::ActionDefinition> {
    let mut actions = inspector_action_definitions();
    actions.extend(crate::editor_edit::window_shared_action_definitions());
    actions
}
//#endregion 🔍️InspectorActions

//#region 🔖️Manifest
/// 🧱️ The editor's `AppDefinition`. Every id in [`ENERGY_MODEL_RETAINED_TOOL_IDS`] is classified
/// `Migrated` here — `set-node`/`set-cell` are already stamped by their kits, the other twelve are
/// classified explicitly, and `EditorBuilder::try_build_definition` panics on any `Unclassified` id.
/// The `energySimulation` tool declares its run, so the framework injects the reserved `toolRun*`
/// actions and binds their chords; the editor binds none of its own for the simulation.
/// Each window owns its own action list (`structure::actions`/`zones::actions`/the simulation
/// window's `definition`), so this function never restates one.
pub fn create_energy_model_editor() -> semio_framework_plugin::AppDefinition {
    let mut builder = Editor::builder(MODEL_DIALECT)
        .document(["semio", "energy", "model"])
        .artifact_kind(crate::snapshot::artifact_kind())
        .terminology("reuse")
        .terminology_document("reuse", ["Entwerfen mit Bestand", "Energie"])
        .icon_id("battery")
        .mode_def(edit::definition())
        .default_mode_id(semio_s_artifact_energy_model_editor_edit::ENERGY_MODEL_EDIT_MODE_ID)
        .window_kind_def(structure::definition())
        .window_kind_def(zones::definition())
        .window_kind_def(simulation::definition())
        .window_kind_def(model_window::definition())
        .interaction(crate::editor_interaction::energy_model_interaction_definition())
        .window_kind_interactions(model_window::WINDOW_KIND_ID, vec![semio_framework_plugin::InteractionRef::new(ENERGY_MODEL_INTERACTION_DOMAIN)])
        .panel_tab_def(artifact_panel::definition())
        .panel_tab_def(inspection_panel::definition())
        .tool(tools::simulation::definition())
        .mutation(SET_ACTIVE_EXAMPLE_ACTION_ID, LocalizedLabel::native("Load example", "Beispiel laden"))
        .action_args(SET_ACTIVE_EXAMPLE_ACTION_ID, vec![semio_framework_plugin::ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), example_options()).required()])
        .keybinding("mod+shift+n", CREATE_ZONE_ACTION_ID)
        .keybinding("mod+shift+s", CREATE_SURFACE_ACTION_ID)
        .keybinding("mod+shift+g", SET_SITE_ACTION_ID)
        .default_layout(edit::layout());
    for action in app_level_action_definitions() {
        builder = builder.action_with(action);
    }
    for tool_id in ENERGY_MODEL_RETAINED_TOOL_IDS {
        builder = builder.action_interactive_job(*tool_id, InteractiveJobClassification::Migrated);
    }
    for destructive_id in ENERGY_MODEL_DESTRUCTIVE_ACTION_IDS {
        builder = builder.action_destructive(*destructive_id);
    }
    builder = builder.action_describe("assign-surface-construction", LocalizedLabel::native("Assigns an existing construction (its layer build-up) to one surface of the energy model by id.", "Weist einer Fläche des Energiemodells anhand ihrer Id eine vorhandene Konstruktion (ihren Schichtaufbau) zu."));
    builder = builder.action_describe("setActiveExample", LocalizedLabel::native("Replaces the whole energy model with a bundled example, the demo or one of the ASHRAE 140 BESTEST cases (600 to 950), by example id.", "Ersetzt das gesamte Energiemodell durch ein mitgeliefertes Beispiel, die Demo oder einen der ASHRAE-140-BESTEST-Fälle (600 bis 950), anhand der Beispiel-Id."));
    builder = builder.action_describe("set-surface-property", LocalizedLabel::native("Sets one property of one surface: name, class, construction, outside boundary (outdoor air, ground, other-side temperature, adiabatic, or interzone with its partner surface), sun or wind exposure, or multiplier.", "Setzt eine Eigenschaft einer Fläche: Name, Klasse, Konstruktion, äußere Randbedingung (Außenluft, Erdreich, Temperatur der Gegenseite, adiabat oder zonenübergreifend mit Partnerfläche), Sonnen- oder Windexposition oder Multiplikator."));
    builder = builder.action_describe("set-fenestration-property", LocalizedLabel::native("Sets one property of one window: name, glazing construction, solar heat gain coefficient, visible transmittance, height, sill height, frame or divider conductance, or overhang and fin dimensions.", "Setzt eine Eigenschaft eines Fensters: Name, Verglasungskonstruktion, Gesamtenergiedurchlassgrad, Lichttransmission, Höhe, Brüstungshöhe, Rahmen- oder Sprossenleitwert oder Maße von Überstand und Seitenblende."));
    builder = builder.action_describe("set-zone-property", LocalizedLabel::native("Sets one property of one thermal zone: its name, multiplier, whether it is conditioned, or whether it counts towards the total floor area.", "Setzt eine Eigenschaft einer thermischen Zone: Name, Multiplikator, ob sie konditioniert ist oder ob sie zur Gesamtnutzfläche zählt."));
    builder = builder.action_describe("set-glazing-material-property", LocalizedLabel::native("Sets one property of one glazing material: name, thickness, conductivity, solar or visible transmittance, or front or back infrared emissivity.", "Setzt eine Eigenschaft eines Verglasungsmaterials: Name, Dicke, Wärmeleitfähigkeit, solaren oder sichtbaren Transmissionsgrad oder vorderen oder hinteren Infrarot-Emissionsgrad."));
    builder = builder.action_describe("set-gas-material-property", LocalizedLabel::native("Sets one property of one gas gap between glazing layers: its name, thickness, or gas (air, argon, krypton or xenon).", "Setzt eine Eigenschaft eines Gaszwischenraums zwischen Verglasungsschichten: Name, Dicke oder Gas (Luft, Argon, Krypton oder Xenon)."));
    builder = builder.action_describe("set-material-property", LocalizedLabel::native("Sets one property of one opaque material: name, roughness, thickness, conductivity, specific heat, or thermal, solar or visible absorptance.", "Setzt eine Eigenschaft eines opaken Materials: Name, Rauigkeit, Dicke, Wärmeleitfähigkeit, spezifische Wärmekapazität oder thermischen, solaren oder sichtbaren Absorptionsgrad."));
    builder = builder.action_describe("set-thermostat-setpoints", LocalizedLabel::native("Sets the heating and cooling setpoint schedules of one thermostat, with optional throttling ranges in kelvin.", "Legt die Heiz- und Kühlsollwert-Zeitpläne eines Thermostats fest, mit optionalen Regelbereichen in Kelvin."));
    builder = builder.action_describe("set-site", LocalizedLabel::native("Sets the building site: latitude, longitude, elevation, time zone and the angle of the building's north axis.", "Legt den Standort des Gebäudes fest: Breite, Länge, Höhe, Zeitzone und den Winkel der Nordachse des Gebäudes."));
    builder = builder.action_describe("delete-zone", LocalizedLabel::native("Deletes one thermal zone by id; refused while any surface, space, gain, HVAC object or airflow node still refers to it.", "Löscht eine thermische Zone anhand ihrer Id; wird abgelehnt, solange noch eine Fläche, ein Raum, eine Last, ein HLK-Objekt oder ein Luftknoten auf sie verweist."));
    builder = builder.action_describe("delete-surface", LocalizedLabel::native("Deletes one surface by id together with every window hosted on it and every adjacency naming it; refused while another surface uses it as its interzone partner.", "Löscht eine Fläche anhand ihrer Id samt aller darin sitzenden Fenster und aller sie nennenden Nachbarschaften; wird abgelehnt, solange eine andere Fläche sie als Partnerfläche nutzt."));
    builder = builder.action_describe("set-construction-property", LocalizedLabel::native("Renames one construction or edits its layer build-up: adds or removes a material layer, or moves a layer up or down.", "Benennt eine Konstruktion um oder bearbeitet ihren Schichtaufbau: fügt eine Materialschicht hinzu, entfernt sie oder verschiebt eine Schicht nach oben oder unten."));
    builder = builder.action_describe("set-simulation-settings", LocalizedLabel::native("Sets the simulation's zone and system timesteps in minutes and the number of warm-up days; a running simulation restarts with them.", "Legt die Zonen- und Systemzeitschritte der Simulation in Minuten und die Zahl der Vorlauftage fest; eine laufende Simulation startet damit neu."));
    builder = builder.action_describe("set-result-field", LocalizedLabel::native("Chooses the simulated per-surface result (conduction loss or gain, transmitted or absorbed solar) the 3D model window is coloured by; a running simulation is not restarted.", "Wählt das simulierte Flächenergebnis (Transmissionsverlust oder -gewinn, transmittierte oder absorbierte Solarstrahlung), nach dem das 3D-Modellfenster eingefärbt wird; eine laufende Simulation startet nicht neu."));
    builder = builder.action_describe("set-run-period", LocalizedLabel::native("Sets the simulated calendar period from a start month and day to an end month and day.", "Legt den simulierten Kalenderzeitraum von einem Startmonat und -tag bis zu einem Endmonat und -tag fest."));
    builder = builder.action_describe("create-surface", LocalizedLabel::native("Adds a new surface with the given name and class to an existing zone, built from an existing construction.", "Fügt einer vorhandenen Zone eine neue Fläche mit dem angegebenen Namen und der Klasse hinzu, aufgebaut aus einer vorhandenen Konstruktion."));
    builder = builder.action_describe("create-zone", LocalizedLabel::native("Adds a new thermal zone with the given name, volume in cubic metres, multiplier and conditioning.", "Fügt eine neue thermische Zone mit dem angegebenen Namen, Volumen in Kubikmetern, Multiplikator und Konditionierung hinzu."));
    builder = builder.action_describe("rename-zone", LocalizedLabel::native("Renames one thermal zone; zone names key every simulation report, so a name already in use is refused.", "Benennt eine thermische Zone um; Zonennamen sind der Schlüssel aller Simulationsberichte, ein bereits vergebener Name wird abgelehnt."));
    builder = builder.action_audience("setCamera", semio_framework_plugin::CapabilityAudience::Chrome);
    builder.build_definition()
}
//#endregion 🔖️Manifest
