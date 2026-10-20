//! 🚀️ App assembly: the internal action, fault notice and flag value helpers and the app creation with its manifest stitch. See [`super`].

use super::*;
use crate::mirror::editor::puzzle2d::modes::edit::windows::overview;
use crate::mirror::editor::puzzle2d::modes::edit::windows::detail;
use crate::mirror::editor::puzzle2d::modes::edit::windows::selection;
use crate::mirror::editor::puzzle2d::panels::catalogue;
use crate::mirror::editor::puzzle2d::panels::inspection;
use semio_s_artifact_puzzle_2d_retained_panels::editor::puzzle2d::panels::settings;
use crate::mirror::editor::puzzle2d::panels::artifact;
use crate::mirror::editor::puzzle2d::modes::edit::windows::overview::utilities::area_brush as area_brush_utility;
use crate::mirror::editor::puzzle2d::modes::edit::windows::overview::utilities::brush as brush_utility;
use crate::mirror::editor::puzzle2d::modes::edit::windows::overview::utilities::select as select_utility;

//#region 🔖️Manifest
/// 🛠️ An internal (non-palette) action declaration — the pointer/gesture/inspector/engagement-bound
/// vocabulary dispatched by the canvas/panels, never surfaced as a standalone command palette entry.
fn puzzle2d_internal_action(id: &str, label: impl Into<LocalizedLabel>, kind: ActionKind) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog(id, label, kind) }
}

/// 📢️ The localized notice of every refusal code the puzzle2d guest names (design §20.12): a set-verb's missing flag value.
pub(super) fn puzzle2d_fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
    static NOTICES: std::sync::LazyLock<[(&str, LocalizedLabel); 1]> = std::sync::LazyLock::new(|| {
        [
            ("puzzle2d.action.flag-value-required", LocalizedLabel::native("Choose on or off for this setting.", "Für diese Einstellung Ein oder Aus wählen.")),
        ]
    });
    &*NOTICES
}

/// 🙈️ The flag a set-verb sets to exactly the boolean its arguments carry (`setSelectionHidden{hidden}`,
/// `setTargetRegionLocked{locked}`, …) — the row target's explicit next state, so a stale view sets a value and never flips one.
pub(super) fn puzzle2d_flag_value_argument(action: &str) -> Option<&'static str> {
    match action {
        "setSelectionHidden" | "setTargetRegionHidden" => Some("hidden"),
        "setSelectionLocked" | "setTargetRegionLocked" => Some("locked"),
        _ => None,
    }
}

/// 🙈️ A set-verb's definition: the explicit identity a row names (or, left empty, the live selection) and the REQUIRED
/// boolean `flag` it sets — a missing value is refused, never defaulted.
fn puzzle2d_flag_value_action(id: &str, category: &str, identity: ActionArgDef, flag: &str, label: LocalizedLabel, value: LocalizedLabel) -> ActionDefinition {
    puzzle2d_internal_action(id, label, ActionKind::Mutation).with_category(category).with_args([identity, ActionArgDef::toggle(flag, value).required()])
}

/// 🎭️✏️ `.example_source(...)` (×2, concrete-forest + nakagin) and `.workflow("puzzle2d", …)` were
/// dropped, not ported — `EditorBuilder` has neither method (contract §2.4: `.editor::<E>(def:
/// AppDefinition)` only takes the definition, `App.examples` has no seam on this builder). Flagged to
/// the coordinator, not silently lost; see `📚️examples/🎬️demo-session` for this subset's own example
/// facet, the likely intended replacement mechanism.
pub fn create_puzzle2d_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(Puzzle2dPlayApp::DIALECT)
            .document(["semio", "puzzle", "2d"])
            .artifact_kind(crate::standards::v1::subsets::any::schema::records::artifact_kind())
            .icon_id("puzzle")
            .terminology("reuse")
            .terminology_document("reuse", ["Entwerfen mit Bestand", "puzzle", "2d"])
            .mode_def(edit::definition())
            .default_mode_id(edit::PUZZLE2D_PLAY_MODE_EDIT)
            .window_kind_def(overview::definition())
            .window_kind_def(detail::definition())
            .window_kind_def(selection::definition())
            .interaction(puzzle2d_interaction_definition())
            .window_kind_interactions(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID, vec![InteractionRef::new(PUZZLE2D_INTERACTION_DOMAIN)])
            .window_kind_interactions(crate::editor::puzzle2d::config::PUZZLE2D_DETAIL_WINDOW_KIND_ID, vec![InteractionRef::new(PUZZLE2D_INTERACTION_DOMAIN)])
            .window_kind_interactions(crate::editor::puzzle2d::config::PUZZLE2D_SELECTION_WINDOW_KIND_ID, vec![InteractionRef::new(PUZZLE2D_INTERACTION_DOMAIN)])
            .panel_tab_def(artifact::definition())
            .panel_tab_def(catalogue::definition())
            .panel_tab_def(inspection::definition())
            .panel_tab_def(settings::definition())
            .mutation("addNode", LocalizedLabel::native("Add Node", "Knoten hinzufügen"))
            .action_with(ActionDefinition::new("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), ActionKind::Mutation, "panel-left"))
            .action_destructive("setActiveExample")
            .action_with(ActionDefinition::bounded_catalog("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"), ActionKind::Mutation).with_category("selection"))
            .action_destructive("deleteSelection")
            .keybinding("delete,backspace", "deleteSelection")
            .action_with(ActionDefinition::bounded_catalog("duplicateSelection", LocalizedLabel::native("Duplicate Selection", "Auswahl duplizieren"), ActionKind::Mutation).with_category("create"))
            .mutation("forceLayout", LocalizedLabel::native("Force Layout", "Kraftbasiertes Layout"))
            .action_with(ActionDefinition::bounded_catalog("focusSelection", LocalizedLabel::native("Focus Selection", "Auswahl fokussieren"), ActionKind::Mutation).with_category("view"))
            .action_with(ActionDefinition::bounded_catalog("translateSelection", puzzle2d_localized(|l| l.translate), ActionKind::Mutation).with_category("transform"))
            .action_with(ActionDefinition::bounded_catalog("rotateSelection", puzzle2d_localized(|l| l.rotate), ActionKind::Mutation).with_category("transform"))
            .action_with(ActionDefinition::bounded_catalog("scaleSelection", puzzle2d_localized(|l| l.scale), ActionKind::Mutation).with_category("transform"))
            .keybinding("mod+d", "duplicateSelection")
            .action_with(ActionDefinition::bounded_catalog("exportSnapshot", puzzle2d_localized(|l| l.export), ActionKind::Shell).with_category("file"))
            .action_with(ActionDefinition::bounded_catalog("openImportSnapshot", puzzle2d_localized(|l| l.import), ActionKind::Shell).with_category("file"))
            .action_with(puzzle2d_internal_action("importSnapshot", LocalizedLabel::native("Import Fixture", "Fixture importieren"), ActionKind::Mutation))
            .action_with(ActionDefinition::bounded_catalog("createEdge", puzzle2d_localized(|l| l.connect), ActionKind::Mutation).with_category("create"))
            .action_with(ActionDefinition::bounded_catalog("deleteEdge", puzzle2d_localized(|l| l.disconnect), ActionKind::Mutation).with_category("selection"))
            .action_destructive("deleteEdge")
            .action_with(ActionDefinition::bounded_catalog("proximityConnect", LocalizedLabel::native("Connect Nearby", "In der Nähe verbinden"), ActionKind::Mutation).with_category("create"))
            .action_with(puzzle2d_internal_action("setProximityRadius", puzzle2d_localized(|l| l.proximity_radius), ActionKind::View).with_category("settings"))
            .action_with(ActionDefinition::bounded_catalog("selectSameKind", LocalizedLabel::native("Select Same Kind", "Gleiche Art auswählen"), ActionKind::View).with_category("selection"))
            .action_with(puzzle2d_internal_action("setSelectionFlag", LocalizedLabel::native("Set Selection Flag", "Auswahlmarkierung festlegen"), ActionKind::Mutation).with_category("settings"))
            .action_with(puzzle2d_flag_value_action("setSelectionHidden", "settings", ActionArgDef::text_list("ids", LocalizedLabel::native("Ids", "IDs")), "hidden", LocalizedLabel::native("Set Hidden", "Verborgen festlegen"), LocalizedLabel::native("Hidden", "Verborgen")))
            .action_with(puzzle2d_flag_value_action("setSelectionLocked", "settings", ActionArgDef::text_list("ids", LocalizedLabel::native("Ids", "IDs")), "locked", LocalizedLabel::native("Set Locked", "Gesperrt festlegen"), LocalizedLabel::native("Locked", "Gesperrt")))
            .action_with(puzzle2d_internal_action("patchInspectorNodes", LocalizedLabel::native("Patch Inspector Nodes", "Inspektorknoten aktualisieren"), ActionKind::Mutation))
            .action_with(puzzle2d_internal_action("redrawHandles", LocalizedLabel::native("Redraw Handles", "Anschlüsse neu zeichnen"), ActionKind::Mutation))
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("reorganize", LocalizedLabel::native("Reorganize", "Neu anordnen"), ActionKind::Mutation, "rotate-cw") })
            .action_with(puzzle2d_internal_action("applyBoardEvents", LocalizedLabel::native("Apply Board Events", "Board-Ereignisse anwenden"), ActionKind::Mutation))
            .action_with(puzzle2d_internal_action("setFillCount", LocalizedLabel::native("Set Fill Count", "Füllanzahl festlegen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("acceptSuggestion", puzzle2d_localized(|l| l.accept_suggestion), ActionKind::Mutation))
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"), ActionKind::View, "camera") })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("engagementInput", LocalizedLabel::native("Engagement Input", "Eingabe"), ActionKind::View, "hand") })
            .action_audience("engagementInput", semio_framework_plugin::CapabilityAudience::Input)
            .action_with(puzzle2d_internal_action("engagementSubmit", LocalizedLabel::native("Engagement Submit", "Eingabe bestätigen"), ActionKind::Mutation))
            .action_audience("engagementSubmit", semio_framework_plugin::CapabilityAudience::Input)
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("engagementAbort", LocalizedLabel::native("Engagement Abort", "Eingabe abbrechen"), ActionKind::View, "hand") })
            .action_audience("engagementAbort", semio_framework_plugin::CapabilityAudience::Input)
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("engagementControlSelect", LocalizedLabel::native("Engagement Control Select", "Eingabesteuerung auswählen"), ActionKind::View, "hand") })
            .action_with(puzzle2d_internal_action("setLodModeForPane", LocalizedLabel::native("Set LOD Mode For Pane", "LOD-Modus für Bereich festlegen"), ActionKind::View))
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("setGridSnapEnabled", LocalizedLabel::native("Set Grid Snap Enabled", "Rasterfang aktivieren"), ActionKind::View, "grid-3x3") })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("setGridFactor", LocalizedLabel::native("Set Grid Factor", "Rasterfaktor festlegen"), ActionKind::View, "grid-3x3") })
            .action_with(puzzle2d_internal_action("setBrushKindWeights", LocalizedLabel::native("Set Brush Kind Weights", "Pinsel-Artgewichte festlegen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("setBrushNodeSize", LocalizedLabel::native("Set Brush Node Size", "Pinsel-Knotengröße festlegen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("setSuggestionOffset", LocalizedLabel::native("Set Suggestion Offset", "Vorschlagsversatz festlegen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("setTransformGumballFlag", LocalizedLabel::native("Set Transform Gumball Flag", "Transformationsgriff festlegen"), ActionKind::View))
            .mutation("addTargetRegion", puzzle2d_localized_phrase(|l| l.target_region, |w| format!("Add {w}"), |w| format!("{w} hinzufügen")))
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("deleteTargetRegion", LocalizedLabel::native("Delete Target Region", "Zielbereich löschen"), ActionKind::Mutation).with_category("targets") })
            .action_destructive("deleteTargetRegion")
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("setTargetRegionFlag", LocalizedLabel::native("Set Target Region Flag", "Zielbereichsmarkierung festlegen"), ActionKind::Mutation).with_category("targets") })
            .action_with(puzzle2d_flag_value_action("setTargetRegionHidden", "targets", ActionArgDef::text("id", puzzle2d_localized(|l| l.id)), "hidden", LocalizedLabel::native("Set Target Region Hidden", "Zielbereich verborgen festlegen"), LocalizedLabel::native("Hidden", "Verborgen")))
            .action_with(puzzle2d_flag_value_action("setTargetRegionLocked", "targets", ActionArgDef::text("id", puzzle2d_localized(|l| l.id)), "locked", LocalizedLabel::native("Set Target Region Locked", "Zielbereich gesperrt festlegen"), LocalizedLabel::native("Locked", "Gesperrt")))
            .action_with(puzzle2d_internal_action("relocateTargetRegion", LocalizedLabel::native("Relocate Target Region", "Zielbereich verlagern"), ActionKind::Mutation))
            .action_with(puzzle2d_internal_action("setAreaBrushSize", LocalizedLabel::native("Set Area Brush Size", "Flächenpinselgröße festlegen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("cycleBrushCandidate", puzzle2d_localized(|l| l.cycle_candidate), ActionKind::View))
            .action_with(puzzle2d_internal_action("cycleBrushCandidateBack", puzzle2d_localized(|l| l.cycle_candidate_back), ActionKind::View))
            .keybinding("tab", "cycleBrushCandidate")
            .keybinding("shift+tab", "cycleBrushCandidateBack")
            .action_with(puzzle2d_internal_action("targetBrushSuggestions", puzzle2d_localized(|l| l.target_suggestions), ActionKind::View))
            .action_with(puzzle2d_internal_action("hoverSuggestion", puzzle2d_localized(|l| l.hover_suggestion), ActionKind::View))
            .action_audience("hoverSuggestion", semio_framework_plugin::CapabilityAudience::Input)
            .action_with(puzzle2d_internal_action("openHandleSuggestions", puzzle2d_localized(|l| l.suggest_nodes), ActionKind::View))
            .action_with(puzzle2d_internal_action("closeHandleSuggestions", puzzle2d_localized(|l| l.close_suggestions), ActionKind::View))
            .action_with(puzzle2d_internal_action("lodScaleJson", LocalizedLabel::native("LOD Scale Json", "LOD-Skalierung-Json"), ActionKind::View))
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("setGridVisible", LocalizedLabel::native("Set Grid Visible", "Raster einblenden"), ActionKind::View, "layout-grid") })
            .action_with(puzzle2d_internal_action("setSelectableKind", LocalizedLabel::native("Set Selectable Kind", "Auswählbare Art festlegen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("setBrushPlacementContactTolerance", LocalizedLabel::native("Set Brush Placement Contact Tolerance", "Kontakttoleranz der Pinselplatzierung festlegen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("setBrushPlacementOverlapBudget", LocalizedLabel::native("Set Brush Placement Overlap Budget", "Überlappungsbudget der Pinselplatzierung festlegen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("engagementRepeatLast", LocalizedLabel::native("Engagement Repeat Last", "Letzte Eingabe wiederholen"), ActionKind::View))
            .action_with(ActionDefinition::bounded_catalog("openAddNodeDialog", puzzle2d_localized_phrase(|l| l.node, |w| format!("Add {w}…"), |w| format!("{w} hinzufügen…")), ActionKind::Shell).with_category("create"))
            .action_args("addNode", vec![puzzle2d_node_kind_arg()])
            .action_args("translateSelection", vec![
                ActionArgDef::number("dx", LocalizedLabel::native("Δx", "Δx")).default_value(&0.0),
                ActionArgDef::number("dy", LocalizedLabel::native("Δy", "Δy")).default_value(&0.0),
            ])
            .action_args("rotateSelection", vec![ActionArgDef::number("angle", puzzle2d_localized(|l| l.angle)).required().default_value(&90.0)])
            .action_args("scaleSelection", vec![ActionArgDef::number("factor", LocalizedLabel::native("Factor", "Faktor")).required().default_value(&1.5)])
            .action_args("createEdge", vec![
                ActionArgDef::text("source", puzzle2d_localized(|l| l.source)).required(),
                ActionArgDef::text("target", puzzle2d_localized(|l| l.target)).required(),
            ])
            .action_args("deleteEdge", vec![ActionArgDef::text("id", puzzle2d_localized(|l| l.id)).required()])
            .action_args("setProximityRadius", vec![ActionArgDef::number("value", puzzle2d_localized(|l| l.proximity_radius)).required().default_value(&crate::editor::puzzle2d::config::PUZZLE2D_DEFAULT_PROXIMITY_RADIUS)])
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![
                    ActionArgOption::new(PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID, puzzle2d_localized(|l| l.example_concrete_forest)),
                    ActionArgOption::new(PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID, LocalizedLabel::native("Nakagin Capsule Tower", "Nakagin Capsule Tower")),
                ]).required().default_value(&PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID),
            ])
            .action_interactive_job("addNode", InteractiveJobClassification::Migrated)
            .action_interactive_job("applyBoardEvents", InteractiveJobClassification::Migrated)
            .action_interactive_job("closeHandleSuggestions", InteractiveJobClassification::Migrated)
            .action_interactive_job("acceptSuggestion", InteractiveJobClassification::Migrated)
            .action_interactive_job("cycleBrushCandidate", InteractiveJobClassification::Migrated)
            .action_interactive_job("cycleBrushCandidateBack", InteractiveJobClassification::Migrated)
            .action_interactive_job("targetBrushSuggestions", InteractiveJobClassification::Migrated)
            .action_interactive_job("openHandleSuggestions", InteractiveJobClassification::Migrated)
            .action_interactive_job("hoverSuggestion", InteractiveJobClassification::Migrated)
            .action_interactive_job("deleteSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("duplicateSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementAbort", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementControlSelect", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementInput", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementSubmit", InteractiveJobClassification::Migrated)
            .action_interactive_job("focusSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("forceLayout", InteractiveJobClassification::Migrated)
            .action_interactive_job("lodScaleJson", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchInspectorNodes", InteractiveJobClassification::Migrated)
            .action_interactive_job("redrawHandles", InteractiveJobClassification::Migrated)
            .action_interactive_job("reorganize", InteractiveJobClassification::Migrated)
            .action_interactive_job("createEdge", InteractiveJobClassification::Migrated)
            .action_interactive_job("deleteEdge", InteractiveJobClassification::Migrated)
            .action_interactive_job("proximityConnect", InteractiveJobClassification::Migrated)
            .action_interactive_job("setProximityRadius", InteractiveJobClassification::Migrated)
            .action_interactive_job("selectSameKind", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
            .action_interactive_job("setBrushKindWeights", InteractiveJobClassification::Migrated)
            .action_interactive_job("setBrushNodeSize", InteractiveJobClassification::Migrated)
            .action_interactive_job("setCamera", InteractiveJobClassification::Migrated)
            .action_interactive_job("setFillCount", InteractiveJobClassification::Migrated)
            .action_interactive_job("setGridFactor", InteractiveJobClassification::Migrated)
            .action_interactive_job("setGridSnapEnabled", InteractiveJobClassification::Migrated)
            .action_interactive_job("setLodModeForPane", InteractiveJobClassification::Migrated)
            .action_interactive_job("translateSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("rotateSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("scaleSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("exportSnapshot", InteractiveJobClassification::Migrated)
            .action_destructive("exportSnapshot")
            .action_interactive_job("importSnapshot", InteractiveJobClassification::Migrated)
            .action_interactive_job("openImportSnapshot", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSelectionFlag", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSelectionHidden", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSelectionLocked", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSuggestionOffset", InteractiveJobClassification::Migrated)
            .action_interactive_job("setTransformGumballFlag", InteractiveJobClassification::Migrated)
            .action_interactive_job("addTargetRegion", InteractiveJobClassification::Migrated)
            .action_interactive_job("deleteTargetRegion", InteractiveJobClassification::Migrated)
            .action_interactive_job("relocateTargetRegion", InteractiveJobClassification::Migrated)
            .action_interactive_job("setTargetRegionFlag", InteractiveJobClassification::Migrated)
            .action_interactive_job("setTargetRegionHidden", InteractiveJobClassification::Migrated)
            .action_interactive_job("setTargetRegionLocked", InteractiveJobClassification::Migrated)
            .action_interactive_job("setAreaBrushSize", InteractiveJobClassification::Migrated)
            .action_interactive_job("setGridVisible", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSelectableKind", InteractiveJobClassification::Migrated)
            .action_interactive_job("setBrushPlacementContactTolerance", InteractiveJobClassification::Migrated)
            .action_interactive_job("setBrushPlacementOverlapBudget", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementRepeatLast", InteractiveJobClassification::Migrated)
            .action_interactive_job("openAddNodeDialog", InteractiveJobClassification::Migrated)
            .dialog(
                DialogDefinition::new(PUZZLE2D_ADD_NODE_DIALOG_ID, puzzle2d_localized_phrase(|l| l.node, |w| format!("Add {w}"), |w| format!("{w} hinzufügen")), ActionRef::new("addNode"))
                    .body(puzzle2d_localized_phrase(|l| l.node, |w| format!("Choose the kind of {w} to add to the board."), |_w| "Wählen Sie die Art zum Hinzufügen.".to_string()))
                    .args(vec![puzzle2d_node_kind_arg()])
                    .submit_label(LocalizedLabel::native("Add", "Hinzufügen")),
            )
            .utility(select_utility::definition(puzzle2d_localized(|l| l.select)))
            .utility(brush_utility::definition(puzzle2d_localized(|l| l.brush)))
            .utility(area_brush_utility::definition(puzzle2d_localized(|l| l.area_brush)))
            .tool(fill::definition(puzzle2d_localized(|l| l.fill)))
            .default_layout(edit::layout())
            .action_describe("setActiveExample", LocalizedLabel::native("Replaces the whole 2D puzzle with one of the plugin's bundled examples, by example id.", "Ersetzt das gesamte 2D-Puzzle durch eines der mitgelieferten Beispiele, anhand der Beispiel-Id."))
            .action_describe("deleteSelection", LocalizedLabel::native("Deletes every selected node from the 2D puzzle together with its connections.", "Löscht alle ausgewählten Knoten samt ihrer Verbindungen aus dem 2D-Puzzle."))
            .action_describe("duplicateSelection", LocalizedLabel::native("Adds a copy of every selected node next to the original; nothing happens without a selection.", "Fügt neben jedem ausgewählten Knoten eine Kopie hinzu; ohne Auswahl geschieht nichts."))
            .action_describe("focusSelection", LocalizedLabel::native("Frames the camera on the selected nodes; only the view changes.", "Richtet die Kamera auf die ausgewählten Knoten aus; nur die Ansicht ändert sich."))
            .action_describe("translateSelection", LocalizedLabel::native("Moves the selected nodes by dx and dy.", "Verschiebt die ausgewählten Knoten um dx und dy."))
            .action_describe("rotateSelection", LocalizedLabel::native("Rotates the selected nodes by the given angle.", "Dreht die ausgewählten Knoten um den angegebenen Winkel."))
            .action_describe("scaleSelection", LocalizedLabel::native("Scales the selected nodes by the given factor.", "Skaliert die ausgewählten Knoten um den angegebenen Faktor."))
            .action_describe("exportSnapshot", LocalizedLabel::native("Writes the whole 2D puzzle as JSON to a downloaded file named after the active example on the user's machine.", "Schreibt das gesamte 2D-Puzzle als JSON in eine heruntergeladene, nach dem aktiven Beispiel benannte Datei auf dem Rechner des Nutzers."))
            .action_describe("openImportSnapshot", LocalizedLabel::native("Opens the host's file picker for a 2D puzzle JSON file; the chosen file then replaces the whole puzzle.", "Öffnet die Dateiauswahl des Hosts für eine 2D-Puzzle-JSON-Datei; die gewählte Datei ersetzt dann das gesamte Puzzle."))
            .action_describe("importSnapshot", LocalizedLabel::native("Replaces the whole 2D puzzle with one read from an imported JSON file; the previous puzzle is discarded.", "Ersetzt das gesamte 2D-Puzzle durch eines aus einer importierten JSON-Datei; das bisherige Puzzle wird verworfen."))
            .action_describe("setSelectionFlag", LocalizedLabel::native("Sets one flag (such as hidden or locked) on the given or selected nodes.", "Setzt eine Markierung (etwa verborgen oder gesperrt) auf den angegebenen oder ausgewählten Knoten."))
            .action_describe("setSelectionHidden", LocalizedLabel::native("Sets the given or selected nodes hidden or shown, to exactly the value passed; repeating it changes nothing.", "Verbirgt die angegebenen oder ausgewählten Knoten oder zeigt sie, genau nach dem übergebenen Wert; eine Wiederholung ändert nichts."))
            .action_describe("setSelectionLocked", LocalizedLabel::native("Sets the given or selected nodes locked or unlocked, to exactly the value passed; repeating it changes nothing.", "Sperrt die angegebenen oder ausgewählten Knoten oder entsperrt sie, genau nach dem übergebenen Wert; eine Wiederholung ändert nichts."))
            .action_describe("acceptSuggestion", LocalizedLabel::native("Places the suggested piece chosen from the suggestion list (by index, or the highlighted one) at its connection point.", "Setzt das aus der Vorschlagsliste gewählte Teil (per Index oder das hervorgehobene) an seinem Anschlusspunkt."))
            .action_describe("addNode", LocalizedLabel::native("Adds a node of the given kind (a 2D block kind) to the puzzle at x, y.", "Fügt dem Puzzle an x, y einen Knoten der angegebenen Art (einer 2D-Blockart) hinzu."))
            .action_describe("forceLayout", LocalizedLabel::native("Lays out every node of the 2D puzzle with a force-directed layout, overwriting their positions.", "Ordnet alle Knoten des 2D-Puzzles mit einem kraftbasierten Layout an und überschreibt ihre Positionen."))
            .action_describe("createEdge", LocalizedLabel::native("Connects two handles of two nodes (source and target) with an edge.", "Verbindet zwei Griffe zweier Knoten (Quelle und Ziel) mit einer Kante."))
            .action_describe("deleteEdge", LocalizedLabel::native("Removes one edge by id, disconnecting its two handles.", "Entfernt eine Kante anhand ihrer Id und trennt ihre zwei Griffe."))
            .action_describe("proximityConnect", LocalizedLabel::native("Connects every pair of compatible handles that lie within the proximity radius of each other.", "Verbindet jedes Paar verträglicher Griffe, die innerhalb des Näheradius beieinander liegen."))
            .action_describe("selectSameKind", LocalizedLabel::native("Extends the selection to every node of the same kind as the selected one.", "Erweitert die Auswahl auf alle Knoten derselben Art wie der ausgewählte."))
            .action_describe("patchInspectorNodes", LocalizedLabel::native("Sets one field on the given nodes, or on handles addressed by id (such as a handle's angle or radius); an empty id list addresses every node.", "Setzt ein Feld auf den angegebenen Knoten oder per Id adressierten Griffen (etwa Winkel oder Radius eines Griffs); eine leere Id-Liste adressiert alle Knoten."))
            .action_describe("redrawHandles", LocalizedLabel::native("Re-aims every connected handle at the edge it carries, updating the handle angles.", "Richtet jeden verbundenen Griff neu auf seine Kante aus und aktualisiert die Griffwinkel."))
            .action_describe("reorganize", LocalizedLabel::native("Lays out every node of the 2D puzzle automatically, overwriting their manual positions.", "Ordnet alle Knoten des 2D-Puzzles automatisch an und überschreibt ihre manuellen Positionen."))
            .action_describe("addTargetRegion", LocalizedLabel::native("Adds a rectangular target region at the given origin and size (the area brush size by default).", "Fügt an Ursprung und Größe (standardmäßig der Flächenpinselgröße) einen rechteckigen Zielbereich hinzu."))
            .action_describe("deleteTargetRegion", LocalizedLabel::native("Deletes one target region by id.", "Löscht einen Zielbereich anhand seiner Id."))
            .action_describe("setTargetRegionFlag", LocalizedLabel::native("Sets one flag (such as hidden or locked) on the given target regions.", "Setzt eine Markierung (etwa verborgen oder gesperrt) auf den angegebenen Zielbereichen."))
            .action_describe("setTargetRegionHidden", LocalizedLabel::native("Sets the given or selected target regions hidden or shown, to exactly the value passed; repeating it changes nothing.", "Verbirgt die angegebenen oder ausgewählten Zielbereiche oder zeigt sie, genau nach dem übergebenen Wert; eine Wiederholung ändert nichts."))
            .action_describe("setTargetRegionLocked", LocalizedLabel::native("Sets the given or selected target regions locked or unlocked, to exactly the value passed; repeating it changes nothing.", "Sperrt die angegebenen oder ausgewählten Zielbereiche oder entsperrt sie, genau nach dem übergebenen Wert; eine Wiederholung ändert nichts."))
            .action_describe("relocateTargetRegion", LocalizedLabel::native("Moves or resizes one target region to the given rectangle.", "Verschiebt oder skaliert einen Zielbereich auf das angegebene Rechteck."))
            .action_describe("openAddNodeDialog", LocalizedLabel::native("Opens the Add Node dialog to pick a node kind to place.", "Öffnet den Dialog Knoten hinzufügen, um eine zu setzende Knotenart zu wählen."))
            .action_audience("applyBoardEvents", semio_framework_plugin::CapabilityAudience::Input)
            .action_destructive("importSnapshot")
            .action_destructive("forceLayout")
            .action_destructive("reorganize")
            .build_definition()
}
//#endregion 🔖️Manifest
