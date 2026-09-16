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

/// 🌳️ The authored verbs this window owns beside the kit's generic `set-node`. Deliberately SHORT:
/// an action declared on a window kind becomes "explicitly owned" and `AppBuilder::build_definition`
/// then stops copying it onto every OTHER window, so a panel control that dispatches it while some
/// other window is active is refused with `window kind … does not own action …`. Everything the
/// inspector panel can dispatch therefore lives app-level in
/// `crate::editor::model::inspector_action_definitions`, not here — only the two verbs whose
/// arguments are meaningless outside this window's own tree stay.
/// 🆕️ `create-surface` — declared here but exported for the APP-level roster, because the
/// `mod+shift+s` keybinding that reaches it must fire while ANY window is active. Listing it in
/// [`actions`] would make it this window's alone; `crate::editor::model::window_shared_action_definitions`
/// is its real home.
pub fn create_surface_action() -> ActionDefinition {
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
    )
}

pub fn actions() -> Vec<ActionDefinition> {
    vec![
        action(
            "assign-surface-construction",
            "Assign construction",
            "Konstruktion zuweisen",
            vec![ActionArgDef::number("surface", LocalizedLabel::native("Surface id", "Flächen-Id")).required(), ActionArgDef::number("construction", LocalizedLabel::native("Construction id", "Konstruktions-Id")).required()],
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
