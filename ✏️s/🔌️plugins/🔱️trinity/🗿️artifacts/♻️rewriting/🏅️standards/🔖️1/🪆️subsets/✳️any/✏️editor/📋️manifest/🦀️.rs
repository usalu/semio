//! 📋️ Rewriting manifest: the app manifest stitch of `create_rewriting_app`.

use super::*;
use crate::editor::rewriting::modes::edit;

//#region 🔖️Manifest
/// 🎯️ `create_rewriting_app` → `Editor::builder(TRINITY_REWRITING_DIALECT)…build_definition()` (contract
/// §2.4). The old `.example("label-core", …)`/`.workflow("trinity-rewriting", …)` calls are DROPPED,
/// not ported — same SDK gap `jack`'s `create_trinity_jack_app` doc comment records.
///
/// 🕸️ `nodeGraphEdit` is the node-graph host's gesture verb: every canvas edit carries the edited
/// graph's `surfaceId` and an `operations` list, neither of which a rail, palette or context-menu press
/// can supply (it was refused `missing operationsJson`), so it stays out of all three. `patchNodes`'
/// `nodeIds` is optional: left empty, the verb patches the selected nodes.
pub fn create_rewriting_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(TRINITY_REWRITING_DIALECT).document(["semio", "trinity", "rewriting"])
            .artifact_kind(crate::standards::v1::subsets::any::schema::model::artifact_kind())
            .icon_id("trinity-rewriting")
            .mode_def(edit::definition())
            .default_mode_id(edit::TRINITY_REWRITING_MODE_EDIT)
            .window_kind(TRINITY_REWRITING_PLAY_WINDOW_BEFORE, LocalizedLabel::native("Before", "Vorher"), TRINITY_REWRITING_PLAY_BODY_BEFORE, SemanticSurfaceKind::NodeGraph, "git-branch")
            .window_kind(TRINITY_REWRITING_PLAY_WINDOW_AFTER, LocalizedLabel::native("After", "Nachher"), TRINITY_REWRITING_PLAY_BODY_AFTER, SemanticSurfaceKind::NodeGraph, "arrow-right")
            .window_kind(TRINITY_REWRITING_PLAY_WINDOW_LHS, LocalizedLabel::native("LHS", "LHS"), TRINITY_REWRITING_PLAY_BODY_LHS, SemanticSurfaceKind::NodeGraph, "trinity-lhs")
            .window_kind(TRINITY_REWRITING_PLAY_WINDOW_RHS, LocalizedLabel::native("RHS", "RHS"), TRINITY_REWRITING_PLAY_BODY_RHS, SemanticSurfaceKind::NodeGraph, "trinity-rhs")
            .window_kind(TRINITY_REWRITING_PLAY_WINDOW_JACK, LocalizedLabel::native("Jack", "Jack"), TRINITY_REWRITING_PLAY_BODY_JACK, SemanticSurfaceKind::TextEditor, "document-jack")
            .window_kind(
                TRINITY_REWRITING_PLAY_WINDOW_PARAMETERS,
                LocalizedLabel::native("Parameters", "Parameter"),
                TRINITY_REWRITING_PLAY_BODY_PARAMETERS,
                SemanticSurfaceKind::Canvas2d,
                "settings-2",
            )
            .default_layout(edit::layout())
            .panel_tab(
                FRAMEWORK_PANEL_TAB_ARTIFACT_ID,
                LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Artefakt"),
                PanelGroup::Workbench,
                TRINITY_REWRITING_PLAY_BODY_ARTIFACT,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
                PanelGroup::Workbench,
                TRINITY_REWRITING_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
                PanelGroup::Details,
                TRINITY_REWRITING_PLAY_BODY_INSPECTION,
            )
            .action_with(semio_framework_plugin::ActionDefinition::bounded_catalog("addRuleClause", LocalizedLabel::native("Add Rule Clause", "Regelklausel hinzufügen"), ActionKind::Mutation).with_category("create"))
            .action_with(semio_framework_plugin::ActionDefinition::bounded_catalog("resetRule", LocalizedLabel::native("Reset Rule", "Regel zurücksetzen"), ActionKind::Mutation).with_category("history"))
            .action_with(semio_framework_plugin::ActionDefinition::new("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), ActionKind::Mutation, "panel-left").with_category("mode"))
            .action_destructive("setActiveExample")
            .action_with(semio_framework_plugin::ActionDefinition::bounded_catalog("setParameter", LocalizedLabel::native("Set Parameter", "Parameter festlegen"), ActionKind::Mutation).with_category("settings"))
            .action_with(semio_framework_plugin::ActionDefinition::bounded_catalog("patchNodes", LocalizedLabel::native("Patch Nodes", "Knoten aktualisieren"), ActionKind::Mutation).with_category("transform"))
            .action_with(semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::bounded_catalog("nodeGraphEdit", LocalizedLabel::native("Edit Graph", "Graph bearbeiten"), ActionKind::Mutation).with_category("transform") })
            .action_with(semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::bounded_catalog("setLhsJson", LocalizedLabel::native("Set LHS Json", "LHS-JSON festlegen"), ActionKind::Mutation).with_category("tools") })
            .action_with(semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::bounded_catalog("setRhsJson", LocalizedLabel::native("Set RHS Json", "RHS-JSON festlegen"), ActionKind::Mutation).with_category("tools") })
            .action_with(semio_framework_plugin::ActionDefinition::bounded_catalog("nodeGraphViewport", LocalizedLabel::native("Set Graph Viewport", "Graph-Ansicht festlegen"), ActionKind::View).with_category("view"))
            .action_with(semio_framework_plugin::ActionDefinition::new("reorganize", LocalizedLabel::native("Reorganize", "Neu anordnen"), ActionKind::Mutation, "rotate-cw").with_category("transform"))
            .action_with(semio_framework_plugin::ActionDefinition::new("setLodMode", LocalizedLabel::native("Set LOD Mode", "LOD-Modus festlegen"), ActionKind::View, "layers").with_category("mode"))
            .action_interactive_job("nodeGraphViewport", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setLodMode", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("addRuleClause", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveExample", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("resetRule", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_destructive("resetRule")
            .action_interactive_job("setParameter", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("patchNodes", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("nodeGraphEdit", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setLhsJson", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setRhsJson", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("reorganize", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(semio_framework_plugin::ActionDefinition::bounded_catalog("addWorkingNode", LocalizedLabel::native("Add Node", "Knoten hinzufügen"), ActionKind::Mutation).with_category("create"))
            .action_interactive_job("addWorkingNode", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .interaction(InteractionDefinition {
                id: "graph".into(),
                label: LocalizedLabel::native("Nodes", "Knoten"),
                granularities: vec![GranularityDefinition { id: "node".into(), label: LocalizedLabel::native("Node", "Knoten"), icon_id: "circle".into() }],
                hierarchy: HierarchyProvider::Topology,
                hover: HoverSpec { transitive: true, ..HoverSpec::default() },
                selection: SelectionSpec { modes: vec![SelectionMode::Multiple, SelectionMode::Single], methods: vec![SelectionMethod::Pick], merges: vec![MergeMode::Replace], transitive: true, broadcast: true },
            })
            .window_kind_interactions(TRINITY_REWRITING_PLAY_WINDOW_BEFORE, vec![InteractionRef::new("graph")])
            .window_kind_interactions(TRINITY_REWRITING_PLAY_WINDOW_AFTER, vec![InteractionRef::new("graph")])
            .window_kind_interactions(TRINITY_REWRITING_PLAY_WINDOW_LHS, vec![InteractionRef::new("graph")])
            .window_kind_interactions(TRINITY_REWRITING_PLAY_WINDOW_RHS, vec![InteractionRef::new("graph")])
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![
                    ActionArgOption::new(crate::examples::demo::ID, crate::examples::demo::label()),
                ]).required(),
            ])
            .action_args("addRuleClause", vec![
                ActionArgDef::select("kind", LocalizedLabel::native("Clause", "Klausel"), vec![
                    ActionArgOption::new("where", LocalizedLabel::native("Where", "Wo")),
                    ActionArgOption::new("create", LocalizedLabel::native("Create", "Erstellen")),
                    ActionArgOption::new("merge", LocalizedLabel::native("Merge", "Zusammenführen")),
                    ActionArgOption::new("set", LocalizedLabel::native("Set", "Setzen")),
                    ActionArgOption::new("delete", LocalizedLabel::native("Delete", "Löschen")),
                    ActionArgOption::new("parameter", LocalizedLabel::native("Parameter", "Parameter")),
                ]).required(),
            ])
            .action_args("patchNodes", vec![
                ActionArgDef::entity_ids("nodeIds", LocalizedLabel::native("Nodes (empty: selection)", "Knoten (leer: Auswahl)"), "graph", "node"),
                ActionArgDef::select("field", LocalizedLabel::native("Field", "Feld"), vec![
                    ActionArgOption::new("name", LocalizedLabel::native("Name", "Name")),
                    ActionArgOption::new("kind", LocalizedLabel::native("Kind", "Art")),
                ]).required(),
                ActionArgDef::text("value", LocalizedLabel::native("Value", "Wert")).required(),
            ])
            .action_args("setParameter", vec![
                ActionArgDef::text("name", LocalizedLabel::native("Parameter", "Parameter")).required(),
                ActionArgDef::text("value", LocalizedLabel::native("Value", "Wert")).required(),
            ])
            .action_args("addWorkingNode", vec![
                ActionArgDef::text("kind", LocalizedLabel::native("Kind (empty: the graph's first node kind)", "Art (leer: erste Knotenart des Graphen)")),
                ActionArgDef::text("name", LocalizedLabel::native("Name (empty: the new id)", "Name (leer: die neue Id)")),
                ActionArgDef::number("x", LocalizedLabel::native("X", "X")).required().default_value(&0.0),
                ActionArgDef::number("y", LocalizedLabel::native("Y", "Y")).required().default_value(&0.0),
            ])
            .action_args("setLhsJson", vec![ActionArgDef::text("value", LocalizedLabel::native("LHS JSON", "LHS-JSON")).required()])
            .action_args("setRhsJson", vec![ActionArgDef::text("value", LocalizedLabel::native("RHS JSON", "RHS-JSON")).required()])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .keybinding("mod+alt+s", "commitCheckpoint")
            .io(rewriting_io())
            .action_describe("setLodMode", LocalizedLabel::native("Sets the level of detail the rule's graph window draws with; only that window's view changes.", "Legt die Detailstufe fest, mit der das Graphfenster der Regel zeichnet; nur die Ansicht dieses Fensters ändert sich."))
            .action_describe("patchNodes", LocalizedLabel::native("Sets the name or kind of the given nodes in the rule's example graph.", "Setzt Name oder Art der angegebenen Knoten im Beispielgraphen der Regel."))
            .action_describe("setActiveExample", LocalizedLabel::native("Replaces the whole rewriting rule with the bundled demo rule, or with the blank default rule, by example id.", "Ersetzt die gesamte Umschreiberegel durch die mitgelieferte Demo-Regel oder die leere Standardregel, anhand der Beispiel-Id."))
            .action_describe("addRuleClause", LocalizedLabel::native("Adds a clause of the given kind (where, create, merge, set, delete or parameter) to the rewriting rule; a where clause is only added once.", "Fügt der Umschreiberegel eine Klausel der angegebenen Art hinzu (where, create, merge, set, delete oder parameter); eine where-Klausel wird nur einmal hinzugefügt."))
            .action_describe("resetRule", LocalizedLabel::native("Resets the rewriting rule to the blank default rule, discarding its pattern, clauses and parameters.", "Setzt die Umschreiberegel auf die leere Standardregel zurück und verwirft Muster, Klauseln und Parameter."))
            .action_describe("setParameter", LocalizedLabel::native("Sets the bound value of one named rule parameter, parsed as a number, boolean or string according to its declared kind.", "Setzt den gebundenen Wert eines benannten Regelparameters, gelesen als Zahl, Wahrheitswert oder Zeichenkette gemäß seiner deklarierten Art."))
            .action_describe("setLhsJson", LocalizedLabel::native("Replaces the rule's left-hand side, the graph pattern it matches, with the given JSON.", "Ersetzt die linke Seite der Regel, das Graphmuster, das sie erkennt, durch das angegebene JSON."))
            .action_describe("setRhsJson", LocalizedLabel::native("Replaces the rule's right-hand side, what it writes for each match, with the given JSON and resets the parameter bindings to their defaults.", "Ersetzt die rechte Seite der Regel, was sie für jeden Treffer schreibt, durch das angegebene JSON und setzt die Parameterbindungen auf ihre Standardwerte zurück."))
            .action_describe("reorganize", LocalizedLabel::native("Drops every manual node position of the rule graph so it is laid out automatically again.", "Verwirft alle manuellen Knotenpositionen des Regelgraphen, sodass er wieder automatisch angeordnet wird."))
            .action_describe("addWorkingNode", LocalizedLabel::native("Adds one node to the rule's example graph at x, y, named with the first free id unless a name is given.", "Fügt dem Beispielgraphen der Regel einen Knoten an x, y hinzu, benannt mit der ersten freien Id, sofern kein Name angegeben ist."))
            .action_audience("nodeGraphEdit", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("nodeGraphViewport", semio_framework_plugin::CapabilityAudience::Chrome)
            .action_destructive("setLhsJson")
            .action_destructive("setRhsJson")
            .action_destructive("reorganize")
            .build_definition()
}
//#endregion 🔖️Manifest
