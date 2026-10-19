//! 🚀️ App assembly: the fault notice, flag value, part kind and interaction definition helpers and the app creation with its manifest stitch. See [`super`].

use super::*;

//#region 🔖️Manifest
/// 📢️ The localized notice of every refusal code the puzzle5d guest names (design §20.12): a set-verb's missing flag value.
pub(super) fn puzzle5d_fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
    static NOTICES: LazyLock<[(&str, LocalizedLabel); 1]> = LazyLock::new(|| {
        [
            ("puzzle5d.action.flag-value-required", LocalizedLabel::native("Choose on or off for this setting.", "Für diese Einstellung Ein oder Aus wählen.")),
        ]
    });
    &*NOTICES
}

/// 🗨️ The one `partKind` select both the standalone `addPartKind` arg form and the "Add Part" dialog
/// declare — built twice from the same catalog so the two forms can never drift apart.

/// 🙈️ The flag a set-verb sets to exactly the boolean its arguments carry (`setSelectionHidden{hidden}`,
/// `setTargetVolumeLocked{locked}`, …) — the row target's explicit next state, so a stale view sets a value and never flips one.
pub(super) fn puzzle5d_flag_value_argument(action: &str) -> Option<&'static str> {
    match action {
        "setSelectionHidden" | "setTargetVolumeHidden" => Some("hidden"),
        "setSelectionLocked" | "setTargetVolumeLocked" => Some("locked"),
        _ => None,
    }
}

/// 🙈️ A set-verb's definition: the explicit identity a row names (or, left empty, the live selection) and the REQUIRED
/// boolean `flag` it sets — a missing value is refused, never defaulted.
fn puzzle5d_flag_value_action(id: &str, identity: Vec<ActionArgDef>, flag: &str, label: LocalizedLabel, value: LocalizedLabel) -> ActionDefinition {
    let mut args = identity;
    args.push(ActionArgDef::toggle(flag, value).required());
    ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog(id, label, ActionKind::Mutation).with_category("settings").with_args(args) }
}

fn puzzle5d_part_kind_arg() -> ActionArgDef {
    let options = puzzle5d_part_kind_options();
    let default = puzzle5d_default_part_kind(&options);
    ActionArgDef::select("partKind", puzzle5d_localized(|l| l.kind), options).default_value(&default)
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the `vortex` domain declaration —
/// one granularity per previously-distinct `Puzzle5dSelection` bag (part/grip/fastener). `Topology`
/// hierarchy (see `Puzzle5dPlayApp::interaction_topology`) exposes the part→grip nesting.
fn puzzle5d_interaction_definition() -> InteractionDefinition {
    let granularity = |id: &str, label: LocalizedLabel, icon: &str| GranularityDefinition { id: id.into(), label, icon_id: icon.into() };
    InteractionDefinition {
        id: PUZZLE5D_INTERACTION_DOMAIN.into(),
        label: LocalizedLabel::native("Vortex", "Vortex"),
        granularities: vec![
            granularity(PUZZLE5D_GRANULARITY_PART, puzzle5d_localized(|l| l.part), "box"),
            granularity(PUZZLE5D_GRANULARITY_GRIP, puzzle5d_localized(|l| l.grip), "circle-dot"),
            granularity(PUZZLE5D_GRANULARITY_FASTENER, LocalizedLabel::native("Fastener", "Verbinder"), "link"),
            granularity(PUZZLE5D_GRANULARITY_TARGET_VOLUME, LocalizedLabel::native("Target Volume", "Zielvolumen"), "box-select"),
        ],
        hierarchy: HierarchyProvider::Topology,
        hover: HoverSpec { enabled: true, transitive: false, channels: vec![PUZZLE5D_HOVER_CHANNEL.into()], broadcast: true },
        selection: SelectionSpec {
            modes: vec![SelectionMode::Multiple, SelectionMode::Single],
            methods: vec![SelectionMethod::Pick, SelectionMethod::Rectangle],
            merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive],
            transitive: false,
            broadcast: true,
        },
    }
}

/// 🚧️ SDK note (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.4): `EditorBuilder`
/// has no `.example(...)`/`.workflow(...)` methods — `.editor::<E>(def: AppDefinition)` only takes the
/// bare definition, `App.examples` is discarded by the plugin-root builder. The three examples this
/// app used to register (`PUZZLE5D_EXAMPLE_CONCRETE_FOREST`/`_NAKAGIN`/`_CAPSULE_DREAM`) and the
/// `"puzzle5d"` workflow tag are DROPPED here, not silently ported — the constants/JSON statics stay
/// live for `setActiveExample`'s own dispatch path (`🎮️commands/🛍️set-active-example`), only the
/// manifest-level registration is gone.
pub fn create_puzzle5d_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(Puzzle5dPlayApp::DIALECT)
            .document(["semio", "puzzle", "5d"])
            .artifact_kind(crate::artifact_kind())
            .icon_id("puzzle")
            .terminology("reuse")
            .terminology_document("reuse", ["Entwerfen mit Bestand", "puzzle", "5d"])
            .mode_def(edit::definition())
            .default_mode_id(edit::PUZZLE5D_PLAY_MODE_EDIT)
            .window_kind_def(board2d::definition())
            .window_kind_def(world3d::definition())
            .interaction(puzzle5d_interaction_definition())
            .window_kind_interactions(crate::editor::puzzle5d::entities::PUZZLE5D_BOARD_WINDOW_KIND_ID, vec![InteractionRef::new(PUZZLE5D_INTERACTION_DOMAIN)])
            .window_kind_interactions(crate::editor::puzzle5d::entities::PUZZLE5D_WORLD_WINDOW_KIND_ID, vec![InteractionRef::new(PUZZLE5D_INTERACTION_DOMAIN)])
            .window_kind_action_refs(crate::editor::puzzle5d::entities::PUZZLE5D_BOARD_WINDOW_KIND_ID, vec![board2d::actions::apply_board_events::reference(), board2d::actions::set_camera::reference()])
            .window_kind_action_refs(
                crate::editor::puzzle5d::entities::PUZZLE5D_WORLD_WINDOW_KIND_ID,
                vec![
                    world3d::actions::translate_selection::reference(),
                    world3d::actions::rotate_selection::reference(),
                    world3d::actions::scale_selection::reference(),
                    world3d::actions::world_relocate::reference(),
                    world3d::actions::set_camera::reference(),
                ],
            )
            .default_layout(edit::layout())
            .panel_tab_def(artifact_panel::definition())
            .panel_tab_def(catalogue::definition())
            .panel_tab_def(inspection::definition())
            .panel_tab_def(settings_panel::definition())
            .keybinding("escape", "engagementAbort")
            .keybinding("delete", "deleteSelection")
            .keybinding("backspace", "deleteSelection")
            .keybinding("mod+d", "duplicateSelection")
            .keybinding("tab", "cycleBrushCandidate")
            .keybinding("shift+tab", "cycleBrushCandidateBack")
            .keybinding("f", "focusSelection")
            .action_with(ActionDefinition::bounded_catalog("exportSnapshot", puzzle5d_localized(|l| l.export), ActionKind::Shell).with_category("file"))
            .action_with(ActionDefinition::bounded_catalog("openImportSnapshot", puzzle5d_localized(|l| l.import), ActionKind::Shell).with_category("file"))
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("importSnapshot", puzzle5d_localized(|l| l.import), ActionKind::Mutation) })
            .action_with(ActionDefinition::new("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), ActionKind::Mutation, "panel-left"))
            .action_destructive("setActiveExample")
            .mutation("addNode", LocalizedLabel::native("Add Node", "Knoten hinzufügen"))
            .action_with(ActionDefinition::bounded_catalog("openAddPartDialog", puzzle5d_localized(|l| l.add_part_prompt), ActionKind::Shell).with_category("create"))
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("addPartKind", puzzle5d_localized(|l| l.add_part), ActionKind::Mutation).with_category("create") })
            .mutation("addBrushPart", LocalizedLabel::native("Add Brush Part", "Pinselteil hinzufügen"))
            .action_with(ActionDefinition::bounded_catalog("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"), ActionKind::Mutation).with_category("selection"))
            .action_destructive("deleteSelection")
            .action_with(ActionDefinition::bounded_catalog("duplicateSelection", LocalizedLabel::native("Duplicate Selection", "Auswahl duplizieren"), ActionKind::Mutation).with_category("create"))
            .action_with(ActionDefinition::bounded_catalog("setSelectionFlag", LocalizedLabel::native("Set Selection Flag", "Auswahlmarkierung festlegen"), ActionKind::Mutation).with_category("settings"))
            .action_with(puzzle5d_flag_value_action("setSelectionHidden", vec![ActionArgDef::text("entity", LocalizedLabel::native("Entity", "Entität")), ActionArgDef::text_list("ids", LocalizedLabel::native("Ids", "IDs"))], "hidden", LocalizedLabel::native("Set Hidden", "Verborgen festlegen"), LocalizedLabel::native("Hidden", "Verborgen")))
            .action_with(puzzle5d_flag_value_action("setSelectionLocked", vec![ActionArgDef::text("entity", LocalizedLabel::native("Entity", "Entität")), ActionArgDef::text_list("ids", LocalizedLabel::native("Ids", "IDs"))], "locked", LocalizedLabel::native("Set Locked", "Gesperrt festlegen"), LocalizedLabel::native("Locked", "Gesperrt")))
            .action_with(ActionDefinition::bounded_catalog("focusSelection", LocalizedLabel::native("Focus Selection", "Auswahl fokussieren"), ActionKind::Mutation).with_category("view"))
            .mutation("engagementSubmit", LocalizedLabel::native("Engagement Submit", "Eingabe bestätigen"))
            .action_audience("engagementSubmit", semio_framework_plugin::CapabilityAudience::Input)
            .mutation("engagementRepeatLast", LocalizedLabel::native("Engagement Repeat Last", "Letzte Eingabe wiederholen"))
            .mutation("patchPart", LocalizedLabel::native("Patch Part", "Teil aktualisieren"))
            .mutation("patchGrip", LocalizedLabel::native("Patch Grip", "Griff aktualisieren"))
            .mutation("patchFastener", LocalizedLabel::native("Patch Fastener", "Verbinder aktualisieren"))
            .mutation("createFastener", LocalizedLabel::native("Create Fastener", "Verbinder erstellen"))
            .mutation("deleteFastener", LocalizedLabel::native("Delete Fastener", "Verbinder löschen"))
            .action_destructive("deleteFastener")
            .mutation("retargetFastener", LocalizedLabel::native("Retarget Fastener", "Verbinder umhängen"))
            .mutation("editFastener", LocalizedLabel::native("Edit Fastener", "Verbinder bearbeiten"))
            .mutation("proximityConnect", LocalizedLabel::native("Proximity Connect", "Näherungsverbinden"))
            .mutation("translateSelection", LocalizedLabel::native("Translate Selection", "Auswahl verschieben"))
            .mutation("rotateSelection", LocalizedLabel::native("Rotate Selection", "Auswahl drehen"))
            .mutation("scaleSelection", LocalizedLabel::native("Scale Selection", "Auswahl skalieren"))
            .mutation("worldRelocate", LocalizedLabel::native("Relocate Part", "Teil verlagern"))
            .mutation("addTargetVolume", LocalizedLabel::native("Add Target Volume", "Zielvolumen hinzufügen"))
            .action_with(ActionDefinition::bounded_catalog("deleteTargetVolume", LocalizedLabel::native("Delete Target Volume", "Zielvolumen löschen"), ActionKind::Mutation).with_category("selection"))
            .action_destructive("deleteTargetVolume")
            .action_with(ActionDefinition::bounded_catalog("setTargetVolumeFlag", LocalizedLabel::native("Set Target Volume Flag", "Zielvolumen-Markierung festlegen"), ActionKind::Mutation).with_category("settings"))
            .action_with(puzzle5d_flag_value_action("setTargetVolumeHidden", vec![ActionArgDef::text("id", LocalizedLabel::native("Id", "ID"))], "hidden", LocalizedLabel::native("Set Target Volume Hidden", "Zielvolumen verborgen festlegen"), LocalizedLabel::native("Hidden", "Verborgen")))
            .action_with(puzzle5d_flag_value_action("setTargetVolumeLocked", vec![ActionArgDef::text("id", LocalizedLabel::native("Id", "ID"))], "locked", LocalizedLabel::native("Set Target Volume Locked", "Zielvolumen gesperrt festlegen"), LocalizedLabel::native("Locked", "Gesperrt")))
            .mutation("relocateTargetVolume", LocalizedLabel::native("Relocate Target Volume", "Zielvolumen verlagern"))
            .mutation("applyBoardEvents", LocalizedLabel::native("Apply Board Events", "Board-Ereignisse anwenden"))
            .action_with(ActionDefinition::new("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"), ActionKind::View, "camera"))
            .action_with(ActionDefinition::new("setCamera2d", LocalizedLabel::native("Set Camera 2D", "Kamera 2D festlegen"), ActionKind::View, "camera"))
            .action_with(ActionDefinition::new("setCamera3d", LocalizedLabel::native("Set Camera 3D", "Kamera 3D festlegen"), ActionKind::View, "camera"))
            .action_with(ActionDefinition::bounded_catalog("selectSameKindSelection", LocalizedLabel::native("Select Same Kind", "Gleiche Art auswählen"), ActionKind::View).with_category("selection"))
            .action_with(ActionDefinition::new("toggleSun", LocalizedLabel::native("Toggle Sun", "Sonne umschalten"), ActionKind::View, "sun"))
            .action_with(ActionDefinition::new("setSunAzimuth", LocalizedLabel::native("Set Sun Azimuth", "Sonnenazimut festlegen"), ActionKind::View, "sun"))
            .action_with(ActionDefinition::new("setSunElevation", LocalizedLabel::native("Set Sun Elevation", "Sonnenhöhe festlegen"), ActionKind::View, "sun"))
            .action_with(ActionDefinition::new("setSunIntensity", LocalizedLabel::native("Set Sun Intensity", "Sonnenintensität festlegen"), ActionKind::View, "sun"))
            .action_with(ActionDefinition::new("engagementInput", LocalizedLabel::native("Engagement Input", "Eingabe"), ActionKind::View, "hand"))
            .action_audience("engagementInput", semio_framework_plugin::CapabilityAudience::Input)
            .action_with(ActionDefinition::new("engagementAbort", LocalizedLabel::native("Engagement Abort", "Eingabe abbrechen"), ActionKind::View, "hand"))
            .action_audience("engagementAbort", semio_framework_plugin::CapabilityAudience::Input)
            .action_with(ActionDefinition::new("engagementControlSelect", LocalizedLabel::native("Engagement Control Select", "Eingabesteuerung auswählen"), ActionKind::View, "hand"))
            .view_action("cycleBrushCandidate", LocalizedLabel::native("Cycle Brush Candidate", "Pinselkandidat wechseln"))
            .view_action("cycleBrushCandidateBack", LocalizedLabel::native("Cycle Brush Candidate Back", "Pinselkandidat rückwärts wechseln"))
            .view_action("targetBrushSuggestions", LocalizedLabel::native("Target Brush Suggestions", "Pinselvorschläge ausrichten"))
            .view_action("openVortexSuggestions", LocalizedLabel::native("Open Grip Suggestions", "Griff-Vorschläge öffnen"))
            .view_action("closeVortexSuggestions", LocalizedLabel::native("Close Grip Suggestions", "Griff-Vorschläge schließen"))
            .view_action("hoverSuggestion", LocalizedLabel::native("Hover Suggestion", "Vorschlag überfahren"))
            .action_audience("hoverSuggestion", semio_framework_plugin::CapabilityAudience::Input)
            .mutation("acceptSuggestion", LocalizedLabel::native("Accept Suggestion", "Vorschlag annehmen"))
            .view_action("registerBrushMesh", LocalizedLabel::native("Register Brush Mesh", "Pinsel-Mesh registrieren"))
            .view_action("setBrushPlacementContactTolerance", LocalizedLabel::native("Set Brush Placement Contact Tolerance", "Pinsel-Kontakttoleranz festlegen"))
            .view_action("setProximityRadius", LocalizedLabel::native("Set Proximity Radius", "Näherungsradius festlegen"))
            .view_action("setChunkSize", LocalizedLabel::native("Set Chunk Size", "Blockgröße festlegen"))
            .view_action("setPartKindWeight", LocalizedLabel::native("Set Part Kind Weight", "Teileart-Gewicht festlegen"))
            .view_action("setGripKindWeight", LocalizedLabel::native("Set Grip Kind Weight", "Griffart-Gewicht festlegen"))
            .action_with(ActionDefinition::new("setLodMode", LocalizedLabel::native("Set Lod Mode", "LOD-Modus festlegen"), ActionKind::View, "layers"))
            .view_action("setFillCount", LocalizedLabel::native("Set Fill Count", "Füllanzahl festlegen"))
            .view_action("setSuggestionOffset", LocalizedLabel::native("Set Suggestion Offset", "Vorschlagsversatz festlegen"))
            .action_with(ActionDefinition::new("setGridSnapEnabled", LocalizedLabel::native("Set Grid Snap Enabled", "Rasterfang aktivieren"), ActionKind::View, "grid-3x3"))
            .action_with(ActionDefinition::new("setGridFactor", LocalizedLabel::native("Set Grid Factor", "Rasterfaktor festlegen"), ActionKind::View, "grid-3x3"))
            .action_with(ActionDefinition::new("setVoxelDims", LocalizedLabel::native("Set Voxel Dims", "Voxelmaße festlegen"), ActionKind::View, "box"))
            .action_with(ActionDefinition::new("setGridVisible", LocalizedLabel::native("Set Grid Visible", "Raster anzeigen"), ActionKind::View, "layout-grid"))
            .action_with(ActionDefinition::new("setGridSpacing", LocalizedLabel::native("Set Grid Spacing", "Rasterabstand festlegen"), ActionKind::View, "grid-3x3"))
            .action_with(ActionDefinition::new("setProjection", LocalizedLabel::native("Set Projection", "Projektion festlegen"), ActionKind::View, "video"))
            .action_with(ActionDefinition::new("setProjectionParam", LocalizedLabel::native("Set Projection Parameter", "Projektionsparameter festlegen"), ActionKind::View, "video"))
            .action_with(ActionDefinition::new("setGripShow", LocalizedLabel::native("Set Grip Markers", "Griffmarken festlegen"), ActionKind::View, "circle-dot"))
            .action_with(ActionDefinition::new("setGripDirection", LocalizedLabel::native("Set Grip Direction", "Griffrichtung festlegen"), ActionKind::View, "compass"))
            .action_with(ActionDefinition::new("setSelectableKind", LocalizedLabel::native("Set Selectable Kind", "Auswählbare Art festlegen"), ActionKind::View, "mouse-pointer").with_category("selection"))
            .action_with(ActionDefinition::new("setLodAutomatic", LocalizedLabel::native("Set Lod Automatic", "LOD automatisch"), ActionKind::View, "zoom-in"))
            .action_with(ActionDefinition::new("setLodDepthVariable", LocalizedLabel::native("Set Lod Depth Variable", "LOD tiefenabhängig"), ActionKind::View, "layers"))
            .action_with(ActionDefinition::new("setLodManual", LocalizedLabel::native("Set Lod Manual", "LOD manuell festlegen"), ActionKind::View, "layers"))
            .action_with(ActionDefinition::new("setTransformGumballFlag", LocalizedLabel::native("Set Transform Gumball Flag", "Transformationsgriff umschalten"), ActionKind::View, "move-3d"))
            .action_audience("setTransformGumballFlag", semio_framework_plugin::CapabilityAudience::Chrome)
            .action_with(ActionDefinition::new("worldPointerDown", LocalizedLabel::native("World Pointer Down", "Welt-Zeiger gedrückt"), ActionKind::View, "mouse-pointer"))
            .action_audience("worldPointerDown", semio_framework_plugin::CapabilityAudience::Input)
            .action_with(ActionDefinition::new("canvasPointerDown", LocalizedLabel::native("Canvas Pointer Down", "Leinwand-Zeiger gedrückt"), ActionKind::View, "mouse-pointer"))
            .action_audience("canvasPointerDown", semio_framework_plugin::CapabilityAudience::Input)
            .action_interactive_job("addBrushPart", InteractiveJobClassification::Migrated)
            .action_interactive_job("addNode", InteractiveJobClassification::Migrated)
            .action_interactive_job("addPartKind", InteractiveJobClassification::Migrated)
            .action_interactive_job("applyBoardEvents", InteractiveJobClassification::Migrated)
            .action_interactive_job("canvasPointerDown", InteractiveJobClassification::Migrated)
            .action_interactive_job("deleteSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("duplicateSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("createFastener", InteractiveJobClassification::Migrated)
            .action_interactive_job("cycleBrushCandidate", InteractiveJobClassification::Migrated)
            .action_interactive_job("cycleBrushCandidateBack", InteractiveJobClassification::Migrated)
            .action_interactive_job("deleteFastener", InteractiveJobClassification::Migrated)
            .action_interactive_job("editFastener", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementAbort", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementControlSelect", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementInput", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementRepeatLast", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementSubmit", InteractiveJobClassification::Migrated)
            .action_interactive_job("focusSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("exportSnapshot", InteractiveJobClassification::Migrated)
            .action_destructive("exportSnapshot")
            .action_interactive_job("importSnapshot", InteractiveJobClassification::Migrated)
            .action_interactive_job("openAddPartDialog", InteractiveJobClassification::Migrated)
            .action_interactive_job("openImportSnapshot", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchFastener", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchGrip", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchPart", InteractiveJobClassification::Migrated)
            .action_interactive_job("proximityConnect", InteractiveJobClassification::Migrated)
            .action_interactive_job("registerBrushMesh", InteractiveJobClassification::Migrated)
            .action_interactive_job("retargetFastener", InteractiveJobClassification::Migrated)
            .action_interactive_job("rotateSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("scaleSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("selectSameKindSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
            .action_interactive_job("setBrushPlacementContactTolerance", InteractiveJobClassification::Migrated)
            .action_interactive_job("setProximityRadius", InteractiveJobClassification::Migrated)
            .action_interactive_job("setChunkSize", InteractiveJobClassification::Migrated)
            .action_interactive_job("setCamera", InteractiveJobClassification::Migrated)
            .action_interactive_job("setCamera2d", InteractiveJobClassification::Migrated)
            .action_interactive_job("setCamera3d", InteractiveJobClassification::Migrated)
            .action_interactive_job("setFillCount", InteractiveJobClassification::Migrated)
            .action_interactive_job("setGridFactor", InteractiveJobClassification::Migrated)
            .action_interactive_job("setGridSnapEnabled", InteractiveJobClassification::Migrated)
            .action_interactive_job("setGridVisible", InteractiveJobClassification::Migrated)
            .action_interactive_job("setGridSpacing", InteractiveJobClassification::Migrated)
            .action_interactive_job("setProjection", InteractiveJobClassification::Migrated)
            .action_interactive_job("setProjectionParam", InteractiveJobClassification::Migrated)
            .action_interactive_job("setGripShow", InteractiveJobClassification::Migrated)
            .action_interactive_job("setGripDirection", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSelectableKind", InteractiveJobClassification::Migrated)
            .action_interactive_job("setLodAutomatic", InteractiveJobClassification::Migrated)
            .action_interactive_job("setLodDepthVariable", InteractiveJobClassification::Migrated)
            .action_interactive_job("setLodManual", InteractiveJobClassification::Migrated)
            .action_interactive_job("setTransformGumballFlag", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSelectionFlag", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSelectionHidden", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSelectionLocked", InteractiveJobClassification::Migrated)
            .action_interactive_job("setLodMode", InteractiveJobClassification::Migrated)
            .action_interactive_job("setPartKindWeight", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSuggestionOffset", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSunAzimuth", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSunElevation", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSunIntensity", InteractiveJobClassification::Migrated)
            .action_interactive_job("setGripKindWeight", InteractiveJobClassification::Migrated)
            .action_interactive_job("targetBrushSuggestions", InteractiveJobClassification::Migrated)
            .action_interactive_job("openVortexSuggestions", InteractiveJobClassification::Migrated)
            .action_interactive_job("closeVortexSuggestions", InteractiveJobClassification::Migrated)
            .action_interactive_job("hoverSuggestion", InteractiveJobClassification::Migrated)
            .action_interactive_job("acceptSuggestion", InteractiveJobClassification::Migrated)
            .action_interactive_job("toggleSun", InteractiveJobClassification::Migrated)
            .action_interactive_job("translateSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("worldPointerDown", InteractiveJobClassification::Migrated)
            .action_interactive_job("addTargetVolume", InteractiveJobClassification::Migrated)
            .action_interactive_job("deleteTargetVolume", InteractiveJobClassification::Migrated)
            .action_interactive_job("relocateTargetVolume", InteractiveJobClassification::Migrated)
            .action_interactive_job("setTargetVolumeFlag", InteractiveJobClassification::Migrated)
            .action_interactive_job("setTargetVolumeHidden", InteractiveJobClassification::Migrated)
            .action_interactive_job("setTargetVolumeLocked", InteractiveJobClassification::Migrated)
            .action_interactive_job("setVoxelDims", InteractiveJobClassification::Migrated)
            .action_interactive_job("worldRelocate", InteractiveJobClassification::Migrated)
            .action_args("addPartKind", vec![puzzle5d_part_kind_arg()])
            .dialog(
                DialogDefinition::new(open_add_part_dialog::PUZZLE5D_ADD_PART_DIALOG, puzzle5d_localized(|l| l.add_part), semio_framework_plugin::ActionRef::new("addPartKind"))
                    .body(puzzle5d_localized(|l| l.add_part_body))
                    .args(vec![puzzle5d_part_kind_arg().required()])
                    .submit_label(puzzle5d_localized(|l| l.add)),
            )
            .action_args("addBrushPart", vec![
                ActionArgDef::select("partKind", puzzle5d_localized(|l| l.kind), vec![ActionArgOption::new("Part", puzzle5d_localized(|l| l.part))]).default_value(&"Part"),
            ])
            .utility(board2d::utilities::select::definition(puzzle5d_localized(|l| l.select)))
            .utility(crate::editor::puzzle5d::modes::edit::windows::world3d::utilities::transform::definition())
            .utility(board2d::utilities::brush::definition(puzzle5d_localized(|l| l.brush)))
            .utility(world3d::utilities::volume_brush::definition(puzzle5d_localized(|l| l.volume_brush)))
            .utility(world3d::utilities::world_relocate::definition())
            .tool(fill_tool::definition(puzzle5d_localized(|l| l.fill)))
            .mode_tools(edit::PUZZLE5D_PLAY_MODE_EDIT, vec![::semio_framework_async::poll::resolve_ready(ToolRef::new(fill_tool::TOOL_ID))])
    .action_describe("setActiveExample", LocalizedLabel::native("Replaces the whole 5D puzzle with one of the plugin's bundled examples, by example id.", "Ersetzt das gesamte 5D-Puzzle durch eines der mitgelieferten Beispiele, anhand der Beispiel-Id."))
    .action_describe("deleteSelection", LocalizedLabel::native("Deletes every selected part from the 5D puzzle together with its connections.", "Löscht alle ausgewählten Teile samt ihrer Verbindungen aus dem 5D-Puzzle."))
    .action_describe("duplicateSelection", LocalizedLabel::native("Adds a copy of every selected part next to the original; nothing happens without a selection.", "Fügt neben jedem ausgewählten Teil eine Kopie hinzu; ohne Auswahl geschieht nichts."))
    .action_describe("focusSelection", LocalizedLabel::native("Frames the camera on the selected parts; only the view changes.", "Richtet die Kamera auf die ausgewählten Teile aus; nur die Ansicht ändert sich."))
    .action_describe("translateSelection", LocalizedLabel::native("Moves the selected parts by dx and dy.", "Verschiebt die ausgewählten Teile um dx und dy."))
    .action_describe("rotateSelection", LocalizedLabel::native("Rotates the selected parts by the given angle.", "Dreht die ausgewählten Teile um den angegebenen Winkel."))
    .action_describe("scaleSelection", LocalizedLabel::native("Scales the selected parts by the given factor.", "Skaliert die ausgewählten Teile um den angegebenen Faktor."))
    .action_describe("exportSnapshot", LocalizedLabel::native("Writes the whole 5D puzzle as JSON to a downloaded file named after the active example on the user's machine.", "Schreibt das gesamte 5D-Puzzle als JSON in eine heruntergeladene, nach dem aktiven Beispiel benannte Datei auf dem Rechner des Nutzers."))
    .action_describe("openImportSnapshot", LocalizedLabel::native("Opens the host's file picker for a 5D puzzle JSON file; the chosen file then replaces the whole puzzle.", "Öffnet die Dateiauswahl des Hosts für eine 5D-Puzzle-JSON-Datei; die gewählte Datei ersetzt dann das gesamte Puzzle."))
    .action_describe("importSnapshot", LocalizedLabel::native("Replaces the whole 5D puzzle with one read from an imported JSON file; the previous puzzle is discarded.", "Ersetzt das gesamte 5D-Puzzle durch eines aus einer importierten JSON-Datei; das bisherige Puzzle wird verworfen."))
    .action_describe("setSelectionFlag", LocalizedLabel::native("Sets one flag (such as hidden or locked) on the given or selected parts.", "Setzt eine Markierung (etwa verborgen oder gesperrt) auf den angegebenen oder ausgewählten Teile."))
    .action_describe("setSelectionHidden", LocalizedLabel::native("Sets the given or selected parts hidden or shown, to exactly the value passed; repeating it changes nothing.", "Verbirgt die angegebenen oder ausgewählten Teile oder zeigt sie, genau nach dem übergebenen Wert; eine Wiederholung ändert nichts."))
    .action_describe("setSelectionLocked", LocalizedLabel::native("Sets the given or selected parts locked or unlocked, to exactly the value passed; repeating it changes nothing.", "Sperrt die angegebenen oder ausgewählten Teile oder entsperrt sie, genau nach dem übergebenen Wert; eine Wiederholung ändert nichts."))
    .action_describe("acceptSuggestion", LocalizedLabel::native("Places the suggested piece chosen from the suggestion list (by index, or the highlighted one) at its connection point.", "Setzt das aus der Vorschlagsliste gewählte Teil (per Index oder das hervorgehobene) an seinem Anschlusspunkt."))
    .action_describe("selectSameKindSelection", LocalizedLabel::native("Extends the selection to every piece of the same kind as the selected one.", "Erweitert die Auswahl auf alle Teile derselben Art wie das ausgewählte."))
    .action_describe("toggleSun", LocalizedLabel::native("Switches the 3D view's sun light on or off; only the view changes.", "Schaltet das Sonnenlicht der 3D-Ansicht ein oder aus; nur die Ansicht ändert sich."))
    .action_describe("setSunAzimuth", LocalizedLabel::native("Sets the compass direction the 3D view's sun shines from; only the view changes.", "Legt die Himmelsrichtung fest, aus der die Sonne der 3D-Ansicht scheint; nur die Ansicht ändert sich."))
    .action_describe("setSunElevation", LocalizedLabel::native("Sets how high the 3D view's sun stands above the horizon; only the view changes.", "Legt fest, wie hoch die Sonne der 3D-Ansicht über dem Horizont steht; nur die Ansicht ändert sich."))
    .action_describe("setSunIntensity", LocalizedLabel::native("Sets the brightness of the 3D view's sun; only the view changes.", "Legt die Helligkeit der Sonne der 3D-Ansicht fest; nur die Ansicht ändert sich."))
    .action_describe("setLodAutomatic", LocalizedLabel::native("Turns automatic level of detail in the 3D view on or off; only the view changes.", "Schaltet die automatische Detailstufe der 3D-Ansicht ein oder aus; nur die Ansicht ändert sich."))
    .action_describe("setLodDepthVariable", LocalizedLabel::native("Turns depth-dependent level of detail in the 3D view on or off; only the view changes.", "Schaltet die tiefenabhängige Detailstufe der 3D-Ansicht ein oder aus; nur die Ansicht ändert sich."))
    .action_describe("setLodManual", LocalizedLabel::native("Sets a fixed level of detail for the 3D view; only the view changes.", "Legt eine feste Detailstufe für die 3D-Ansicht fest; nur die Ansicht ändert sich."))
    .action_describe("setGridVisible", LocalizedLabel::native("Shows or hides the placement grid in the 3D view; only the view changes.", "Blendet das Platzierungsraster in der 3D-Ansicht ein oder aus; nur die Ansicht ändert sich."))
    .action_describe("setGridSnapEnabled", LocalizedLabel::native("Turns snapping to the placement grid on or off for moves in the 3D view.", "Schaltet das Fangen am Platzierungsraster für Verschiebungen in der 3D-Ansicht ein oder aus."))
    .action_describe("setGridSpacing", LocalizedLabel::native("Sets the spacing of the placement grid.", "Legt den Abstand des Platzierungsrasters fest."))
    .action_describe("setProximityRadius", LocalizedLabel::native("Sets how close two pieces must come for a move or Connect Nearby to join their connection points.", "Legt fest, wie nahe sich zwei Teile kommen müssen, damit ein Verschieben oder In der Nähe verbinden ihre Anschlusspunkte verbindet."))
    .action_describe("setChunkSize", LocalizedLabel::native("Sets the size of the spatial chunks the 3D view loads and draws pieces in; only the view changes.", "Legt die Größe der räumlichen Blöcke fest, in denen die 3D-Ansicht Teile lädt und zeichnet; nur die Ansicht ändert sich."))
    .action_describe("setSelectableKind", LocalizedLabel::native("Sets whether pieces, connection points or attractions can be picked in the 3D view.", "Legt fest, ob Teile, Anschlusspunkte oder Anziehungen in der 3D-Ansicht gewählt werden können."))
    .action_describe("setTransformGumballFlag", LocalizedLabel::native("Switches one option of the transform gumball (such as translate, rotate or scale handles) on or off.", "Schaltet eine Option des Transformationsgriffs (etwa Verschiebe-, Dreh- oder Skaliergriffe) ein oder aus."))
    .action_describe("setVoxelDims", LocalizedLabel::native("Sets one dimension (width, depth or height) of the voxel volume the fill tool packs pieces into.", "Legt eine Abmessung (Breite, Tiefe oder Höhe) des Voxelvolumens fest, in das das Füllwerkzeug Teile packt."))
    .action_describe("relocateTargetVolume", LocalizedLabel::native("Moves one target volume to a new position.", "Verschiebt ein Zielvolumen an eine neue Position."))
    .action_describe("setFillCount", LocalizedLabel::native("Sets how many pieces the fill tool places into the target volume.", "Legt fest, wie viele Teile das Füllwerkzeug in das Zielvolumen setzt."))
    .action_describe("setBrushPlacementContactTolerance", LocalizedLabel::native("Sets how much overlap (0 to 1) the placement brush tolerates between a suggested piece and existing ones.", "Legt fest, wie viel Überlappung (0 bis 1) der Platzierungspinsel zwischen einem vorgeschlagenen und vorhandenen Teilen zulässt."))
    .action_describe("cycleBrushCandidate", LocalizedLabel::native("Switches the placement brush to the next suggested piece.", "Schaltet den Platzierungspinsel zum nächsten vorgeschlagenen Teil."))
    .action_describe("cycleBrushCandidateBack", LocalizedLabel::native("Switches the placement brush to the previous suggested piece.", "Schaltet den Platzierungspinsel zum vorherigen vorgeschlagenen Teil."))
    .action_describe("openVortexSuggestions", LocalizedLabel::native("Opens the list of pieces that fit at one connection point.", "Öffnet die Liste der Teile, die an einen Anschlusspunkt passen."))
    .action_describe("closeVortexSuggestions", LocalizedLabel::native("Closes the list of suggested pieces.", "Schließt die Liste der vorgeschlagenen Teile."))
    .action_describe("addTargetVolume", LocalizedLabel::native("Adds a target volume, a box region the fill tool packs pieces into.", "Fügt ein Zielvolumen hinzu, einen Quaderbereich, in den das Füllwerkzeug Teile packt."))
    .action_describe("deleteTargetVolume", LocalizedLabel::native("Deletes one target volume by id.", "Löscht ein Zielvolumen anhand seiner Id."))
    .action_describe("setTargetVolumeFlag", LocalizedLabel::native("Sets one flag (such as hidden or locked) on the given target volumes.", "Setzt eine Markierung (etwa verborgen oder gesperrt) auf den angegebenen Zielvolumen."))
    .action_describe("setTargetVolumeHidden", LocalizedLabel::native("Sets the given target volume hidden or shown, to exactly the value passed; repeating it changes nothing.", "Verbirgt das angegebene Zielvolumen oder zeigt es, genau nach dem übergebenen Wert; eine Wiederholung ändert nichts."))
    .action_describe("setTargetVolumeLocked", LocalizedLabel::native("Sets the given target volume locked or unlocked, to exactly the value passed; repeating it changes nothing.", "Sperrt das angegebene Zielvolumen oder entsperrt es, genau nach dem übergebenen Wert; eine Wiederholung ändert nichts."))
    .action_describe("addNode", LocalizedLabel::native("Adds a part of the given kind (a 5D block kind) to the puzzle at x, y.", "Fügt dem Puzzle an x, y ein Teil der angegebenen Art (einer 5D-Blockart) hinzu."))
    .action_describe("proximityConnect", LocalizedLabel::native("Fastens every pair of compatible grips that lie within the proximity radius of each other.", "Verbindet jedes Paar verträglicher Griffe, die innerhalb des Näheradius beieinander liegen, mit Verbindern."))
    .action_describe("worldRelocate", LocalizedLabel::native("Moves one part to a new position and fastens it to nearby compatible grips.", "Verschiebt ein Teil an eine neue Position und verbindet es mit verträglichen Griffen in der Nähe."))
    .action_describe("openAddPartDialog", LocalizedLabel::native("Opens the Add Part dialog to pick a part kind to place.", "Öffnet den Dialog Teil hinzufügen, um eine zu setzende Teileart zu wählen."))
    .action_describe("addPartKind", LocalizedLabel::native("Adds a part of the given kind to the puzzle.", "Fügt dem Puzzle ein Teil der angegebenen Art hinzu."))
    .action_describe("addBrushPart", LocalizedLabel::native("Places a part of the given kind with the placement brush where the brush suggests.", "Setzt mit dem Platzierungspinsel ein Teil der angegebenen Art dort, wo der Pinsel es vorschlägt."))
    .action_describe("patchPart", LocalizedLabel::native("Sets one field on the given or selected parts.", "Setzt ein Feld auf den angegebenen oder ausgewählten Teilen."))
    .action_describe("patchGrip", LocalizedLabel::native("Sets one field on the given or selected grips.", "Setzt ein Feld auf den angegebenen oder ausgewählten Griffen."))
    .action_describe("patchFastener", LocalizedLabel::native("Sets one field on the given or selected fasteners.", "Setzt ein Feld auf den angegebenen oder ausgewählten Verbindern."))
    .action_describe("createFastener", LocalizedLabel::native("Creates a fastener joining two compatible grips of different parts.", "Erstellt einen Verbinder, der zwei verträgliche Griffe verschiedener Teile verbindet."))
    .action_describe("deleteFastener", LocalizedLabel::native("Deletes one fastener by id, releasing its two grips.", "Löscht einen Verbinder anhand seiner Id und gibt seine zwei Griffe frei."))
    .action_describe("retargetFastener", LocalizedLabel::native("Moves one end of a fastener to another compatible grip.", "Hängt ein Ende eines Verbinders an einen anderen verträglichen Griff um."))
    .action_describe("editFastener", LocalizedLabel::native("Sets the gap, shift or rise of one fastener, absolutely or by a delta.", "Setzt Abstand, Versatz oder Anhebung eines Verbinders, absolut oder um eine Differenz."))
    .action_describe("setPartKindWeight", LocalizedLabel::native("Sets how often the fill tool and brush pick one part kind relative to the others.", "Legt fest, wie oft Füllwerkzeug und Pinsel eine Teileart im Verhältnis zu den anderen wählen."))
    .action_describe("setGripKindWeight", LocalizedLabel::native("Sets how strongly one grip kind is preferred when suggesting placements.", "Legt fest, wie stark eine Griffart bei Platzierungsvorschlägen bevorzugt wird."))
    .action_describe("setLodMode", LocalizedLabel::native("Sets the level-of-detail mode of the views; only the view changes.", "Legt den Detailstufenmodus der Ansichten fest; nur die Ansicht ändert sich."))
    .action_describe("setSuggestionOffset", LocalizedLabel::native("Sets how far suggested parts are offset from the grip they would attach to.", "Legt fest, wie weit vorgeschlagene Teile vom Griff, an den sie ansetzen würden, versetzt werden."))
    .action_describe("setGridFactor", LocalizedLabel::native("Sets the spacing factor of the 2D view's snap grid; only the view setting changes.", "Legt den Abstandsfaktor des Fangrasters der 2D-Ansicht fest; nur die Ansichtseinstellung ändert sich."))
    .action_describe("setGripShow", LocalizedLabel::native("Sets whether grip markers are shown always or only on selected parts; only the view changes.", "Legt fest, ob Griffmarken immer oder nur an ausgewählten Teilen gezeigt werden; nur die Ansicht ändert sich."))
    .action_describe("setGripDirection", LocalizedLabel::native("Sets how grip directions are drawn; only the view changes.", "Legt fest, wie Griffrichtungen gezeichnet werden; nur die Ansicht ändert sich."))
    .action_audience("setCamera", semio_framework_plugin::CapabilityAudience::Chrome)
    .action_audience("setProjection", semio_framework_plugin::CapabilityAudience::Chrome)
    .action_audience("setProjectionParam", semio_framework_plugin::CapabilityAudience::Chrome)
    .action_audience("engagementRepeatLast", semio_framework_plugin::CapabilityAudience::Input)
    .action_audience("engagementControlSelect", semio_framework_plugin::CapabilityAudience::Input)
    .action_audience("targetBrushSuggestions", semio_framework_plugin::CapabilityAudience::Input)
    .action_audience("registerBrushMesh", semio_framework_plugin::CapabilityAudience::Input)
    .action_audience("setCamera2d", semio_framework_plugin::CapabilityAudience::Chrome)
    .action_audience("setCamera3d", semio_framework_plugin::CapabilityAudience::Chrome)
    .action_audience("applyBoardEvents", semio_framework_plugin::CapabilityAudience::Input)
    .action_destructive("importSnapshot")
    .build_definition()
}
//#endregion 🔖️Manifest
