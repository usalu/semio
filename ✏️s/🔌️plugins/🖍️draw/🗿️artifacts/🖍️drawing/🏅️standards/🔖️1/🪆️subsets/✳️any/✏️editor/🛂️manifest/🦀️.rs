//! 🛂️ Drawing editor app manifest: the actions, utilities, windows, panels and engagement the drawing editor declares.

use semio_s_artifact_draw_drawing_editor_interaction::DRAWING_DEFAULT_UTILITY;
use semio_s_artifact_draw_drawing_editor_interaction::DRAWING_INTERACTION_DOMAIN;
use semio_s_artifact_draw_drawing_editor_interaction::DRAWING_INTERACTION_GRANULARITY;
use semio_s_artifact_draw_drawing_editor_interaction::DRAWING_POINT_DOMAIN;
use semio_s_artifact_draw_drawing_editor_interaction::drawing_manifest_action;
use semio_s_artifact_draw_drawing_editor_commands::nudge_selection::{nudge_selection_left, nudge_selection_left_fast, nudge_selection_right, nudge_selection_right_fast, nudge_selection_up, nudge_selection_up_fast, nudge_selection_down, nudge_selection_down_fast};
use semio_s_artifact_draw_drawing_editor_tool::{DrawingGesturePreview, DrawingSession};
use semio_s_artifact_draw_drawing_editor_commands::{import_image, add_layer, combine_boolean, commit_document, delete_layer, delete_selection, drop_layer_kind, duplicate_layer, engagement_input, engagement_submit, export_document, edit_selection, edit_path, edit_fill, move_layer, patch_layer, patch_layers, set_active_example, set_camera, set_camera_zoom, load_document_json, set_selected_opacity, set_snapshot, toggle_layer_visible};
use semio_s_artifact_draw_drawing_editor_tool::{canvas_commit_draft, canvas_double_click, canvas_escape, canvas_pointer_move, canvas_pointer_up};
use semio_s_artifact_draw_drawing_editor_tool as canvas_pointer_down;
use semio_s_artifact_draw_drawing_editor_tool::modes::edit;
use semio_s_artifact_draw_drawing_editor_tool::modes::edit::windows::canvas as canvas_window;
use semio_s_artifact_draw_drawing_editor_tool::modes::edit::windows::canvas::config::DrawingCanvasWindowConfig;
use semio_s_artifact_draw_drawing_editor_tool::modes::edit::windows::canvas::transient::DrawingCanvasWindowTransient;
use semio_s_artifact_draw_drawing_editor_interaction::panels::{catalogue as catalogue_panel, layers as layers_panel, properties as properties_panel};
use semio_s_artifact_draw_drawing_editor_interaction::presence::{DrawingPresence, DrawingPresenceMutation};
use semio_s_artifact_draw_drawing_editor_interaction::terminology::DrawingPlayLabels;
use semio_s_artifact_draw_drawing_mutations::DrawingMutation;
use semio_s_artifact_draw_drawing_scene::identity::SceneIdentity;
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use semio_s_artifact_draw_drawing_model::DRAWING_DOCUMENT_SCHEMA;
use semio_framework_job_registry::{FixedOperationOwner};
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::ActionDescriptor;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::FaultCode;
use semio_framework_plugin::FaultOrigin;
use semio_framework_plugin::GranularityDefinition;
use semio_framework_plugin::HierarchyProvider;
use semio_framework_plugin::HoverSpec;
use semio_framework_plugin::InteractionDefinition;
use semio_framework_plugin::InteractionRef;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::Media;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaError;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaPayload;
use semio_framework_plugin::MediaType;
use semio_framework_plugin::MergeMode;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::SelectionMethod;
use semio_framework_plugin::SelectionMode;
use semio_framework_plugin::SelectionSpec;
use semio_framework_plugin::UtilityCategory;
use semio_framework_plugin::UtilityDefinition;
use semio_framework_plugin::WindowEngagement;
use semio_framework_plugin::WindowEngagementInput;
use semio_framework_plugin::WindowEngagementStatus;
use pack::snapshot::ArtifactPack;
use semio_framework_2d_compute::EngineHandles;
use std::collections::HashMap;
use semio_s_artifact_draw_drawing_editor_tool::modes::edit::windows::canvas::DRAWING_PLAY_BODY_COMPOSITE;
use semio_s_artifact_draw_drawing_editor_tool::modes::edit::windows::canvas::DRAWING_PLAY_WINDOW_CANVAS;
use semio_s_artifact_draw_drawing_editor_interaction::panels::catalogue::DRAWING_PLAY_BODY_CATALOGUE;
use semio_s_artifact_draw_drawing_editor_interaction::panels::layers::DRAWING_PLAY_BODY_LAYERS;
use semio_s_artifact_draw_drawing_editor_interaction::panels::properties::DRAWING_PLAY_BODY_PROPERTIES;
use semio_s_artifact_draw_drawing_editor::{drawing_io};
use semio_s_artifact_draw_drawing_editor_interaction::DRAWING_POINT_GRANULARITY;

/// 🛠️ An internal (non-palette) action declaration — the pointer/gesture/inspector-bound vocabulary
/// that is dispatched by the canvas/panels, never surfaced as a standalone command palette entry.
fn drawing_internal_action(id: &str, label: impl Into<LocalizedLabel>, kind: ActionKind) -> semio_framework_plugin::ActionDefinition {
    semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::bounded_catalog(id, label, kind) }
}
/// 🖱️ A raw canvas/engagement input event — the canvas dispatches it, the MCP capability catalog
/// never publishes it (`CapabilityAudience::Input`).
fn drawing_input_event(id: &str, label: impl Into<LocalizedLabel>, kind: ActionKind) -> semio_framework_plugin::ActionDefinition {
    semio_framework_plugin::ActionDefinition::bounded_catalog(id, label, kind).input_event()
}
/// 🗂️ The `kind` argument shared by `addLayer`/`dropLayerKind` — the exact vocabulary
/// `schema::create_layer_by_kind` branches on, so the published JSON Schema enumerates what the
/// guest really accepts instead of leaving an agent to guess "rectangle".
fn drawing_layer_kind_arg() -> semio_framework_plugin::ActionArgDef {
    semio_framework_plugin::ActionArgDef::select(
        "kind",
        LocalizedLabel::native("Layer Kind", "Ebenenart"),
        vec![
            semio_framework_plugin::ActionArgOption::new("shape:rect", LocalizedLabel::native("Rectangle", "Rechteck")),
            semio_framework_plugin::ActionArgOption::new("shape:ellipse", LocalizedLabel::native("Ellipse", "Ellipse")),
            semio_framework_plugin::ActionArgOption::new("shape:line", LocalizedLabel::native("Line", "Linie")),
            semio_framework_plugin::ActionArgOption::new("shape:polygon", LocalizedLabel::native("Polygon", "Polygon")),
            semio_framework_plugin::ActionArgOption::new("path", LocalizedLabel::native("Path", "Pfad")),
            semio_framework_plugin::ActionArgOption::new("text", LocalizedLabel::native("Text", "Text")),
            semio_framework_plugin::ActionArgOption::new("image", LocalizedLabel::native("Image", "Bild")),
            semio_framework_plugin::ActionArgOption::new("group", LocalizedLabel::native("Group", "Gruppe")),
            semio_framework_plugin::ActionArgOption::new("boolean", LocalizedLabel::native("Boolean", "Boolean")),
            semio_framework_plugin::ActionArgOption::new("trace", LocalizedLabel::native("Trace", "Nachzeichnung")),
        ],
    )
    .default_value(&"path")
}
/// 🪪️ The `layerId` argument shared by every single-layer verb.
fn drawing_layer_id_arg() -> semio_framework_plugin::ActionArgDef {
    semio_framework_plugin::ActionArgDef::text("layerId", LocalizedLabel::native("Layer", "Ebene")).required()
}
/// 🎯️ The layer-tree drop target shared by `moveLayer`/`dropLayerKind`.
fn drawing_target_row_arg() -> semio_framework_plugin::ActionArgDef {
    semio_framework_plugin::ActionArgDef::text("targetRowId", LocalizedLabel::native("Target Row", "Zielzeile")).required()
}
/// ↕️ Where a moved/dropped layer lands relative to the target row.
fn drawing_drop_position_arg() -> semio_framework_plugin::ActionArgDef {
    semio_framework_plugin::ActionArgDef::select(
        "dropPosition",
        LocalizedLabel::native("Drop Position", "Ablageposition"),
        vec![
            semio_framework_plugin::ActionArgOption::new("before", LocalizedLabel::native("Before", "Davor")),
            semio_framework_plugin::ActionArgOption::new("after", LocalizedLabel::native("After", "Danach")),
            semio_framework_plugin::ActionArgOption::new("inside", LocalizedLabel::native("Inside", "Hinein")),
        ],
    )
    .default_value(&"after")
}
/// 🩹️ The patchable layer property vocabulary — the exact `field` arm set of
/// `schema::mutations::drawing_op_for_layer_field`.
fn drawing_layer_field_arg() -> semio_framework_plugin::ActionArgDef {
    semio_framework_plugin::ActionArgDef::select(
        "field",
        LocalizedLabel::native("Field", "Feld"),
        vec![
            semio_framework_plugin::ActionArgOption::new("name", LocalizedLabel::native("Name", "Name")),
            semio_framework_plugin::ActionArgOption::new("textContent", LocalizedLabel::native("Text Content", "Textinhalt")),
            semio_framework_plugin::ActionArgOption::new("textSize", LocalizedLabel::native("Text Size", "Schriftgröße")),
            semio_framework_plugin::ActionArgOption::new("fontFamily",LocalizedLabel::native("Font Family","Schriftfamilie")),
            semio_framework_plugin::ActionArgOption::new("imageKey", LocalizedLabel::native("Image Asset Key", "Bildressourcenschlüssel")),
            semio_framework_plugin::ActionArgOption::new("imageWidth", LocalizedLabel::native("Image Width", "Bildbreite")),
            semio_framework_plugin::ActionArgOption::new("imageHeight", LocalizedLabel::native("Image Height", "Bildhöhe")),
            semio_framework_plugin::ActionArgOption::new("rectX", LocalizedLabel::native("Rectangle X", "Rechteck X")),
            semio_framework_plugin::ActionArgOption::new("rectY", LocalizedLabel::native("Rectangle Y", "Rechteck Y")),
            semio_framework_plugin::ActionArgOption::new("rectWidth", LocalizedLabel::native("Rectangle Width", "Rechteckbreite")),
            semio_framework_plugin::ActionArgOption::new("rectHeight", LocalizedLabel::native("Rectangle Height", "Rechteckhöhe")),
            semio_framework_plugin::ActionArgOption::new("ellipseCx", LocalizedLabel::native("Ellipse Center X", "Ellipsenmittelpunkt X")),
            semio_framework_plugin::ActionArgOption::new("ellipseCy", LocalizedLabel::native("Ellipse Center Y", "Ellipsenmittelpunkt Y")),
            semio_framework_plugin::ActionArgOption::new("ellipseRx", LocalizedLabel::native("Ellipse Radius X", "Ellipsenradius X")),
            semio_framework_plugin::ActionArgOption::new("ellipseRy", LocalizedLabel::native("Ellipse Radius Y", "Ellipsenradius Y")),
            semio_framework_plugin::ActionArgOption::new("circleCx", LocalizedLabel::native("Circle Center X", "Kreismittelpunkt X")),
            semio_framework_plugin::ActionArgOption::new("circleCy", LocalizedLabel::native("Circle Center Y", "Kreismittelpunkt Y")),
            semio_framework_plugin::ActionArgOption::new("circleR", LocalizedLabel::native("Circle Radius", "Kreisradius")),
            semio_framework_plugin::ActionArgOption::new("lineX1", LocalizedLabel::native("Line Start X", "Linienanfang X")),
            semio_framework_plugin::ActionArgOption::new("lineY1", LocalizedLabel::native("Line Start Y", "Linienanfang Y")),
            semio_framework_plugin::ActionArgOption::new("lineX2", LocalizedLabel::native("Line End X", "Linienende X")),
            semio_framework_plugin::ActionArgOption::new("lineY2", LocalizedLabel::native("Line End Y", "Linienende Y")),
            semio_framework_plugin::ActionArgOption::new("polygonX", LocalizedLabel::native("Polygon Point X", "Polygonpunkt X")),
            semio_framework_plugin::ActionArgOption::new("polygonY", LocalizedLabel::native("Polygon Point Y", "Polygonpunkt Y")),
            semio_framework_plugin::ActionArgOption::new("opacity", LocalizedLabel::native("Opacity", "Deckkraft")),
            semio_framework_plugin::ActionArgOption::new("visible", LocalizedLabel::native("Visible", "Sichtbar")),
            semio_framework_plugin::ActionArgOption::new("locked", LocalizedLabel::native("Locked", "Gesperrt")),
            semio_framework_plugin::ActionArgOption::new("blendMode", LocalizedLabel::native("Blend Mode", "Mischmodus")),
            semio_framework_plugin::ActionArgOption::new("booleanOperation", LocalizedLabel::native("Boolean Operation", "Boolean-Operation")),
            semio_framework_plugin::ActionArgOption::new("isolation", LocalizedLabel::native("Isolate Group Blending", "Gruppenmischung isolieren")),
            semio_framework_plugin::ActionArgOption::new("fillRule", LocalizedLabel::native("Fill Rule", "Füllregel")),
            semio_framework_plugin::ActionArgOption::new("fillEnabled", LocalizedLabel::native("Fill Enabled", "Füllung aktiv")),
            semio_framework_plugin::ActionArgOption::new("strokeEnabled", LocalizedLabel::native("Stroke Enabled", "Kontur aktiv")),
            semio_framework_plugin::ActionArgOption::new("strokeColor", LocalizedLabel::native("Stroke Color", "Konturfarbe")),
            semio_framework_plugin::ActionArgOption::new("fillColor", LocalizedLabel::native("Fill Colour", "Füllfarbe")),
            semio_framework_plugin::ActionArgOption::new("strokeCap", LocalizedLabel::native("Line Caps", "Linienenden")),
            semio_framework_plugin::ActionArgOption::new("strokeJoin", LocalizedLabel::native("Line Joins", "Linienverbindungen")),
            semio_framework_plugin::ActionArgOption::new("strokeDash", LocalizedLabel::native("Dash Pattern", "Strichmuster")),
            semio_framework_plugin::ActionArgOption::new("strokeWidth", LocalizedLabel::native("Stroke Width", "Strichstärke")),
            semio_framework_plugin::ActionArgOption::new("transformX", LocalizedLabel::native("Transform X", "Transformation X")),
            semio_framework_plugin::ActionArgOption::new("transformY", LocalizedLabel::native("Transform Y", "Transformation Y")),
            semio_framework_plugin::ActionArgOption::new("transformScaleX", LocalizedLabel::native("Scale X", "Skalierung X")),
            semio_framework_plugin::ActionArgOption::new("transformScaleY", LocalizedLabel::native("Scale Y", "Skalierung Y")),
            semio_framework_plugin::ActionArgOption::new("transformRotation", LocalizedLabel::native("Rotation", "Drehung")),
            semio_framework_plugin::ActionArgOption::new("transformShear", LocalizedLabel::native("Shear", "Scherung")),
            semio_framework_plugin::ActionArgOption::new("rotationDegrees", LocalizedLabel::native("Rotation (°)", "Drehung (°)")),
            semio_framework_plugin::ActionArgOption::new("traceThreshold", LocalizedLabel::native("Trace Threshold", "Schwellenwert")),
            semio_framework_plugin::ActionArgOption::new("traceSimplify", LocalizedLabel::native("Trace Simplify", "Vereinfachung")),
        ],
    )
    .required()
}
/// 🔤️ The patch `value` — one `String` wire field carrying JSON text, so one verb covers bool,
/// number and string properties (see `patch_layer::patch_value_json`).
fn drawing_layer_value_arg() -> semio_framework_plugin::ActionArgDef {
    semio_framework_plugin::ActionArgDef::json_text("value", LocalizedLabel::native("Value", "Wert")).describe(LocalizedLabel::native("JSON text: a quoted string for name/blendMode/fillColor, a number for opacity/strokeWidth/transforms, true/false for visible/locked.", "JSON-Text: eine Zeichenkette in Anführungszeichen für name/blendMode/fillColor, eine Zahl für opacity/strokeWidth/Transformationen, true/false für visible/locked.")).required()
}
/// 🧰️ One canvas utility declaration (id/label/icon reused verbatim from the retired `utilities()` impl).
fn drawing_utility(id: &str, label: impl Into<LocalizedLabel>, icon: &str, group: &str, category: UtilityCategory) -> UtilityDefinition {
    UtilityDefinition { group: Some(group.into()), category: Some(category), allows_actions_while_active: true, ..UtilityDefinition::new(id, label, icon) }
}
pub fn create_drawing_app() -> semio_framework_plugin::AppDefinition {
    let engagement = WindowEngagement {
        session_active: Some(false),
        options: None,
        input: Some(WindowEngagementInput {
            id: Some("drawing-canvas-engagement".into()),
            value: Some(String::new()),
            placeholder: Some("Layer name".into()),
            on_change: Some(drawing_manifest_action("engagementInput")),
            on_submit: Some(drawing_manifest_action("engagementSubmit")),
            disabled: None,
            on_repeat_last: None,
            on_abort: None,
        }),
        control: None,
        controls: None,
        status: Some(vec![WindowEngagementStatus { id: "drawing-layer-count".into(), text: "0 layers · 0 selected".into() }]),
        possible_engagements: None,
    };
    let builder = Editor::builder(semio_s_artifact_draw_drawing_model::DRAWING_DIALECT);
    let builder = create_drawing_app_part_1(builder, engagement);
    let builder = create_drawing_app_part_2(builder);
    let builder = create_drawing_app_part_3(builder);
    let builder = create_drawing_app_part_4(builder);
    let builder = create_drawing_app_part_5(builder);
    builder.build_definition()
}

/// 🧱️ The manifest slice 1 of 5 of `create_drawing_app`: its builder calls.
fn create_drawing_app_part_1(builder: semio_framework_plugin::app::EditorBuilder, engagement: WindowEngagement) -> semio_framework_plugin::app::EditorBuilder {
    builder.document(["semio", "drawing"])
            .artifact_kind(semio_s_artifact_draw_drawing_model::artifact_kind())
            .icon_id("drawing")
            .mode("edit", LocalizedLabel::native("Edit", "Bearbeiten"), "pencil")
            .default_mode_id("edit")
            .window_kind_with_engagement(DRAWING_PLAY_WINDOW_CANVAS, LocalizedLabel::native("Canvas", "Leinwand"), DRAWING_PLAY_BODY_COMPOSITE, semio_framework_ui_contract::SurfaceKind::Canvas2d, engagement, "pen-tool")
            .panel_tab_def(layers_panel::definition())
            .panel_tab_def(catalogue_panel::definition())
            .panel_tab_def(properties_panel::definition())
            .action_with(
                semio_framework_plugin::ActionDefinition::bounded_catalog("addLayer", LocalizedLabel::native("Add Layer", "Ebene hinzufügen"), ActionKind::Mutation)
                    .describe(LocalizedLabel::native(
                        "Appends a new layer of the given kind (a rectangle, ellipse, line, polygon, freehand path, text, image, group, boolean or trace) to the top of the drawing.",
                        "Fügt der Zeichnung oben eine neue Ebene der angegebenen Art hinzu (Rechteck, Ellipse, Linie, Polygon, Pfad, Text, Bild, Gruppe, Boolean oder Nachzeichnung).",
                    ))
                    .use_when(["draw a rectangle", "add a rectangle", "draw an ellipse or circle", "add a line", "add a polygon", "add a text layer", "add a new layer"])
                    .with_args([drawing_layer_kind_arg()]),
            )
            .action_interactive_job("addLayer", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(semio_framework_plugin::ActionDefinition { in_palette:false, ..semio_framework_plugin::ActionDefinition::bounded_catalog("importImage",LocalizedLabel::native("Import PNG Image","PNG-Bild importieren"),ActionKind::Mutation) })
            .action_interactive_job("importImage", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(
                semio_framework_plugin::ActionDefinition::bounded_catalog("combineBoolean", LocalizedLabel::native("Combine Boolean", "Boolean kombinieren"), ActionKind::Mutation)
                    .describe(LocalizedLabel::native(
                        "Combines two or more existing layers into one boolean layer (union, intersect, subtract or exclude). Defaults to the current selection when no ids are given.",
                        "Verbindet zwei oder mehr vorhandene Ebenen zu einer Boolean-Ebene (Vereinigung, Schnitt, Differenz oder Ausschluss). Ohne Ids wird die aktuelle Auswahl verwendet.",
                    ))
                    .use_when(["union these shapes", "subtract one shape from another", "intersect the selection", "combine shapes"])
                    .with_args([
                        semio_framework_plugin::ActionArgDef::select(
                            "operation",
                            LocalizedLabel::native("Operation", "Operation"),
                            vec![
                                semio_framework_plugin::ActionArgOption::new("union", LocalizedLabel::native("Union", "Vereinigung")),
                                semio_framework_plugin::ActionArgOption::new("intersection", LocalizedLabel::native("Intersect", "Schnitt")),
                                semio_framework_plugin::ActionArgOption::new("difference", LocalizedLabel::native("Subtract", "Differenz")),
                                semio_framework_plugin::ActionArgOption::new("xor", LocalizedLabel::native("Exclude", "Ausschluss")),
                            ],
                        )
                        .default_value(&"union")
                        .required(),
                        semio_framework_plugin::ActionArgDef::text_list("ids", LocalizedLabel::native("Layer Ids", "Ebenen-Ids")).describe(LocalizedLabel::native("Ids of the layers to combine; empty uses the current selection.", "Ids der zu kombinierenden Ebenen; leer verwendet die aktuelle Auswahl.")),
                    ]),
            )
            .action_interactive_job("combineBoolean", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(semio_framework_plugin::ActionDefinition::bounded_catalog("editSelection", LocalizedLabel::native("Arrange Selection", "Auswahl anordnen"), ActionKind::Mutation)
                .describe(LocalizedLabel::native(
                    "Arranges the given layers, or the current selection when none are given, in one undoable step: groups or ungroups them, duplicates or deletes them, moves them one step or all the way to the front or back, aligns their edges or centres (two or more layers), or distributes them evenly (three or more). Every layer must be unlocked; grouping, duplicating, deleting and reordering need layers of one parent group.",
                    "Ordnet die angegebenen Ebenen, oder die aktuelle Auswahl, wenn keine angegeben sind, in einem rückgängig machbaren Schritt an: gruppiert sie oder hebt ihre Gruppierung auf, dupliziert oder löscht sie, verschiebt sie eine Ebene oder ganz nach vorne oder hinten, richtet ihre Kanten oder Mitten aus (ab zwei Ebenen) oder verteilt sie gleichmäßig (ab drei Ebenen). Jede Ebene muss entsperrt sein; Gruppieren, Duplizieren, Löschen und Umordnen verlangen Ebenen derselben Gruppe.",
                ))
                .use_when(["group the selected shapes", "ungroup the selection", "duplicate the selection", "delete the selected layers", "bring the selection forward", "send the selection to the back", "align the selected layers", "distribute the selection evenly"])
                .with_args(vec![semio_framework_plugin::ActionArgDef::select("operation", LocalizedLabel::native("Operation", "Aktion"), vec![
                    semio_framework_plugin::ActionArgOption::new("group", LocalizedLabel::native("Group", "Gruppieren")),
                    semio_framework_plugin::ActionArgOption::new("duplicate", LocalizedLabel::native("Duplicate", "Duplizieren")),
                    semio_framework_plugin::ActionArgOption::new("delete", LocalizedLabel::native("Delete", "Löschen")),
                    semio_framework_plugin::ActionArgOption::new("ungroup", LocalizedLabel::native("Ungroup", "Gruppierung aufheben")),
                    semio_framework_plugin::ActionArgOption::new("bringForward", LocalizedLabel::native("Bring Forward", "Eine Ebene nach vorne")),
                    semio_framework_plugin::ActionArgOption::new("sendBackward", LocalizedLabel::native("Send Backward", "Eine Ebene nach hinten")),
                    semio_framework_plugin::ActionArgOption::new("bringToFront", LocalizedLabel::native("Bring to Front", "In den Vordergrund")),
                    semio_framework_plugin::ActionArgOption::new("sendToBack", LocalizedLabel::native("Send to Back", "In den Hintergrund")),
                    semio_framework_plugin::ActionArgOption::new("alignLeft", LocalizedLabel::native("Align Left", "Links ausrichten")),
                    semio_framework_plugin::ActionArgOption::new("alignCenter", LocalizedLabel::native("Align Center", "Horizontal zentrieren")),
                    semio_framework_plugin::ActionArgOption::new("alignRight", LocalizedLabel::native("Align Right", "Rechts ausrichten")),
                    semio_framework_plugin::ActionArgOption::new("alignTop", LocalizedLabel::native("Align Top", "Oben ausrichten")),
                    semio_framework_plugin::ActionArgOption::new("alignMiddle", LocalizedLabel::native("Align Middle", "Vertikal zentrieren")),
                    semio_framework_plugin::ActionArgOption::new("alignBottom", LocalizedLabel::native("Align Bottom", "Unten ausrichten")),
                    semio_framework_plugin::ActionArgOption::new("distributeHorizontal", LocalizedLabel::native("Distribute Horizontally", "Horizontal verteilen")),
                    semio_framework_plugin::ActionArgOption::new("distributeVertical", LocalizedLabel::native("Distribute Vertically", "Vertikal verteilen")),
                ]).required(), semio_framework_plugin::ActionArgDef::text_list("ids", LocalizedLabel::native("Layers", "Ebenen"))]))
            .action_interactive_job("editSelection", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(semio_framework_plugin::ActionDefinition::bounded_catalog("editPath", LocalizedLabel::native("Edit Path", "Pfad bearbeiten"), ActionKind::Mutation)
                .describe(LocalizedLabel::native(
                    "Edits one unlocked path layer's geometry in one undoable step, given as a JSON node edit: moves or deletes chosen anchors and control handles, sets one point's position or coordinate, splits, deletes or converts a segment between line and cubic curve, opens, closes or joins the path at a node, or reverses its direction.",
                    "Bearbeitet die Geometrie einer entsperrten Pfadebene in einem rückgängig machbaren Schritt, angegeben als JSON-Knotenbearbeitung: verschiebt oder löscht gewählte Anker- und Steuerpunkte, setzt die Position oder eine Koordinate eines Punkts, teilt, löscht oder wandelt ein Segment zwischen Linie und kubischer Kurve um, öffnet, schließt oder verbindet den Pfad an einem Knoten oder kehrt seine Richtung um.",
                ))
                .use_when(["move a path node", "drag a bezier handle", "delete path points", "split a path segment", "convert a segment to a curve", "close the path", "reverse the path direction"])
                .with_args([semio_framework_plugin::ActionArgDef::text("layerId", LocalizedLabel::native("Path", "Pfad")).required(), semio_framework_plugin::ActionArgDef::json_text("edit", LocalizedLabel::native("Node Edit", "Knotenbearbeitung")).required()]))
            .action_interactive_job("editPath", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(semio_framework_plugin::ActionDefinition::bounded_catalog("editFill", LocalizedLabel::native("Edit Fill", "Füllung bearbeiten"), ActionKind::Mutation)
                .describe(LocalizedLabel::native(
                    "Edits one unlocked layer's fill in one undoable step, given as a JSON fill edit: sets its type (none, solid, linear or radial gradient), a colour or its opacity, a gradient coordinate or stop offset, or adds or removes a gradient stop.",
                    "Bearbeitet die Füllung einer entsperrten Ebene in einem rückgängig machbaren Schritt, angegeben als JSON-Füllungsbearbeitung: setzt ihren Typ (keine, einfarbig, linearer oder radialer Verlauf), eine Farbe oder deren Deckkraft, eine Verlaufskoordinate oder den Versatz eines Farbstopps, oder fügt einen Farbstopp hinzu oder entfernt ihn.",
                ))
                .use_when(["change the fill colour", "make the fill a gradient", "add a gradient stop", "set the fill opacity", "remove the fill"])
                .with_args([semio_framework_plugin::ActionArgDef::text("layerId", LocalizedLabel::native("Layer", "Ebene")).required(), semio_framework_plugin::ActionArgDef::json_text("edit", LocalizedLabel::native("Fill Edit", "Füllungsbearbeitung")).required()]))
            .action_interactive_job("editFill", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(semio_framework_plugin::ActionDefinition::bounded_catalog("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"), ActionKind::Mutation))
            
}

/// 🪵️ The manifest slice 2 of 5 of `create_drawing_app`: its builder calls.
fn create_drawing_app_part_2(builder: semio_framework_plugin::app::EditorBuilder) -> semio_framework_plugin::app::EditorBuilder {
    builder.action_audience("deleteSelection", semio_framework_plugin::CapabilityAudience::Input)
            .action_interactive_job("deleteSelection", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(semio_framework_plugin::ActionDefinition::bounded_catalog("nudgeSelectionLeft", LocalizedLabel::native("Nudge Left", "Auswahl nach links verschieben"), ActionKind::Mutation))
            .action_audience("nudgeSelectionLeft", semio_framework_plugin::CapabilityAudience::Input)
            .action_interactive_job("nudgeSelectionLeft", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(semio_framework_plugin::ActionDefinition::bounded_catalog("nudgeSelectionLeftFast", LocalizedLabel::native("Nudge Left Fast", "Auswahl schnell nach links verschieben"), ActionKind::Mutation))
            .action_audience("nudgeSelectionLeftFast", semio_framework_plugin::CapabilityAudience::Input)
            .action_interactive_job("nudgeSelectionLeftFast", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(semio_framework_plugin::ActionDefinition::bounded_catalog("nudgeSelectionRight", LocalizedLabel::native("Nudge Right", "Auswahl nach rechts verschieben"), ActionKind::Mutation))
            .action_audience("nudgeSelectionRight", semio_framework_plugin::CapabilityAudience::Input)
            .action_interactive_job("nudgeSelectionRight", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(semio_framework_plugin::ActionDefinition::bounded_catalog("nudgeSelectionRightFast", LocalizedLabel::native("Nudge Right Fast", "Auswahl schnell nach rechts verschieben"), ActionKind::Mutation))
            .action_audience("nudgeSelectionRightFast", semio_framework_plugin::CapabilityAudience::Input)
            .action_interactive_job("nudgeSelectionRightFast", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(semio_framework_plugin::ActionDefinition::bounded_catalog("nudgeSelectionUp", LocalizedLabel::native("Nudge Up", "Auswahl nach oben verschieben"), ActionKind::Mutation))
            .action_audience("nudgeSelectionUp", semio_framework_plugin::CapabilityAudience::Input)
            .action_interactive_job("nudgeSelectionUp", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(semio_framework_plugin::ActionDefinition::bounded_catalog("nudgeSelectionUpFast", LocalizedLabel::native("Nudge Up Fast", "Auswahl schnell nach oben verschieben"), ActionKind::Mutation))
            .action_audience("nudgeSelectionUpFast", semio_framework_plugin::CapabilityAudience::Input)
            .action_interactive_job("nudgeSelectionUpFast", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(semio_framework_plugin::ActionDefinition::bounded_catalog("nudgeSelectionDown", LocalizedLabel::native("Nudge Down", "Auswahl nach unten verschieben"), ActionKind::Mutation))
            .action_audience("nudgeSelectionDown", semio_framework_plugin::CapabilityAudience::Input)
            .action_interactive_job("nudgeSelectionDown", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(semio_framework_plugin::ActionDefinition::bounded_catalog("nudgeSelectionDownFast", LocalizedLabel::native("Nudge Down Fast", "Auswahl schnell nach unten verschieben"), ActionKind::Mutation))
            .action_audience("nudgeSelectionDownFast", semio_framework_plugin::CapabilityAudience::Input)
            .action_interactive_job("nudgeSelectionDownFast", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(
                semio_framework_plugin::ActionDefinition::bounded_catalog("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), ActionKind::Mutation)
                    .describe(LocalizedLabel::native(
                        "Replaces the whole drawing with one of the plugin's declared playground examples, by example id.",
                        "Ersetzt die gesamte Zeichnung durch eines der deklarierten Beispiele des Plugins, anhand der Beispiel-Id.",
                    ))
                    .use_when(["load the demo drawing", "open an example"])
                    .with_args([semio_framework_plugin::ActionArgDef::text("exampleId", LocalizedLabel::native("Example", "Beispiel")).required()]),
            )
            .action_interactive_job("setActiveExample", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(
                semio_framework_plugin::ActionDefinition { icon_id: "download".into(), ..semio_framework_plugin::ActionDefinition::bounded_catalog("exportDocument", LocalizedLabel::native("Export Drawing", "Zeichnung exportieren"), ActionKind::View) }
                    .describe(LocalizedLabel::native(
                        "Renders the drawing to a downloadable PNG image, vector-painted PDF page or SVG document.",
                        "Rendert die Zeichnung als PNG-Bild, vektorgezeichnete PDF-Seite oder SVG-Dokument in eine herunterladbare Datei.",
                    ))
                    .use_when(["export the document as png", "export the document as pdf", "save this drawing as an svg", "download the drawing"])
                    .with_args([semio_framework_plugin::ActionArgDef::select(
                        "format",
                        LocalizedLabel::native("Format", "Format"),
                        vec![
                            semio_framework_plugin::ActionArgOption::new("png", LocalizedLabel::native("PNG", "PNG")),
                            semio_framework_plugin::ActionArgOption::new("pdf", LocalizedLabel::native("PDF", "PDF")),
                            semio_framework_plugin::ActionArgOption::new("svg", LocalizedLabel::native("SVG", "SVG")),
                        ],
                    )
                    .default_value(&export_document::DEFAULT_EXPORT_FORMAT),
                    semio_framework_plugin::ActionArgDef {schema:semio_framework::ArgSchema::number(Some(1.0),Some(16384.0),Some(1.0),true),..semio_framework_plugin::ActionArgDef::index("width",LocalizedLabel::native("PNG Width in Pixels","PNG-Breite in Pixeln"))},
                    semio_framework_plugin::ActionArgDef {schema:semio_framework::ArgSchema::number(Some(1.0),Some(16384.0),Some(1.0),true),..semio_framework_plugin::ActionArgDef::index("height",LocalizedLabel::native("PNG Height in Pixels","PNG-Höhe in Pixeln"))},
                    semio_framework_plugin::ActionArgDef::toggle("transparent",LocalizedLabel::native("Transparent PNG Background","Transparenter PNG-Hintergrund")).default_value(&true)]),
            )
            .action_interactive_job("exportDocument", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(
                drawing_internal_action("setSnapshot", LocalizedLabel::native("Set Document", "Dokument festlegen"), ActionKind::Mutation)
                    .describe(LocalizedLabel::native("Replaces the entire drawing document with a supplied snapshot.", "Ersetzt das gesamte Zeichnungsdokument durch einen übergebenen Snapshot."))
                    .use_when(["replace the whole drawing"]),
            )
            .action_interactive_job("setSnapshot", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(
                drawing_internal_action("commitDocument", LocalizedLabel::native("Commit Document", "Dokument übernehmen"), ActionKind::Mutation)
                    .describe(LocalizedLabel::native("Commits a supplied snapshot as the drawing's next revision.", "Schreibt einen übergebenen Snapshot als nächste Revision der Zeichnung fest."))
                    .use_when(["commit this document state"]),
            )
            .action_interactive_job("commitDocument", semio_framework_plugin::InteractiveJobClassification::Migrated)
            
}

/// 🔩️ The manifest slice 3 of 5 of `create_drawing_app`: its builder calls.
fn create_drawing_app_part_3(builder: semio_framework_plugin::app::EditorBuilder) -> semio_framework_plugin::app::EditorBuilder {
    builder.action_with(
                drawing_internal_action("loadDocumentJson", LocalizedLabel::native("Load Document JSON", "Dokument-JSON laden"), ActionKind::Mutation)
                    .describe(LocalizedLabel::native("Loads a whole drawing document from JSON text.", "Lädt ein vollständiges Zeichnungsdokument aus JSON-Text."))
                    .use_when(["load this drawing from json"])
                    .with_args([semio_framework_plugin::ActionArgDef::json_text("json", LocalizedLabel::native("Document JSON", "Dokument-JSON")).required()]),
            )
            .action_interactive_job("loadDocumentJson", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(
                drawing_internal_action("setSelectedOpacity", LocalizedLabel::native("Set Selected Opacity", "Deckkraft der Auswahl festlegen"), ActionKind::Mutation)
                    .describe(LocalizedLabel::native("Sets the opacity of every currently selected layer, from 0 (invisible) to 1 (opaque).", "Setzt die Deckkraft aller ausgewählten Ebenen, von 0 (unsichtbar) bis 1 (deckend)."))
                    .use_when(["make the selection half transparent", "change the opacity"])
                    .with_args([semio_framework_plugin::ActionArgDef::slider("value", LocalizedLabel::native("Opacity", "Deckkraft"), 0.0, 1.0).required()]),
            )
            .action_interactive_job("setSelectedOpacity", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(drawing_input_event("engagementSubmit", LocalizedLabel::native("Engagement Submit", "Eingabe bestätigen"), ActionKind::Mutation))
            .action_interactive_job("engagementSubmit", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(
                drawing_internal_action("dropLayerKind", LocalizedLabel::native("Drop Layer Kind", "Ebenenart ablegen"), ActionKind::Mutation)
                    .describe(LocalizedLabel::native("Creates a layer of the given kind at a drop target in the layer tree.", "Erzeugt eine Ebene der angegebenen Art an einer Ablagestelle im Ebenenbaum."))
                    .with_args([drawing_layer_kind_arg(), drawing_target_row_arg(), drawing_drop_position_arg()]),
            )
            .action_interactive_job("dropLayerKind", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(
                drawing_internal_action("moveLayer", LocalizedLabel::native("Move Layer", "Ebene verschieben"), ActionKind::Mutation)
                    .describe(LocalizedLabel::native("Reorders one layer within the layer tree, before or after a target row or inside a group.", "Ordnet eine Ebene im Ebenenbaum um — vor oder nach einer Zielzeile oder in eine Gruppe hinein."))
                    .use_when(["move a layer up", "reorder the layers", "put this layer in the group"])
                    .with_args([drawing_layer_id_arg(), drawing_target_row_arg(), drawing_drop_position_arg()]),
            )
            .action_interactive_job("moveLayer", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(
                drawing_internal_action("deleteLayer", LocalizedLabel::native("Delete Layer", "Ebene löschen"), ActionKind::Mutation)
                    .describe(LocalizedLabel::native("Removes one layer from the drawing by id.", "Entfernt eine Ebene anhand ihrer Id aus der Zeichnung."))
                    .use_when(["delete a layer", "remove this shape"])
                    .with_args([drawing_layer_id_arg()]),
            )
            .action_interactive_job("deleteLayer", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(
                drawing_internal_action("duplicateLayer", LocalizedLabel::native("Duplicate Layer", "Ebene duplizieren"), ActionKind::Mutation)
                    .describe(LocalizedLabel::native("Copies one layer and inserts the copy next to the original.", "Kopiert eine Ebene und fügt die Kopie neben dem Original ein."))
                    .use_when(["duplicate this layer", "copy the shape"])
                    .with_args([drawing_layer_id_arg()]),
            )
            .action_interactive_job("duplicateLayer", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(
                drawing_internal_action("toggleLayerVisible", LocalizedLabel::native("Toggle Layer Visible", "Ebenensichtbarkeit umschalten"), ActionKind::Mutation)
                    .describe(LocalizedLabel::native("Flips one layer between visible and hidden.", "Schaltet eine Ebene zwischen sichtbar und ausgeblendet um."))
                    .use_when(["hide this layer", "show the layer again"])
                    .with_args([drawing_layer_id_arg()]),
            )
            .action_interactive_job("toggleLayerVisible", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(
                drawing_internal_action("patchLayer", LocalizedLabel::native("Patch Layer", "Ebene aktualisieren"), ActionKind::Mutation)
                    .describe(LocalizedLabel::native(
                        "Sets one named property of one layer — its name, opacity, visibility, lock, blend mode, boolean operation, fill colour, stroke width, transform or trace parameters.",
                        "Setzt eine benannte Eigenschaft einer Ebene — Name, Deckkraft, Sichtbarkeit, Sperre, Mischmodus, Boolean-Operation, Füllfarbe, Strichstärke, Transformation oder Nachzeichnungsparameter.",
                    ))
                    .use_when(["rename a layer", "change the fill colour", "set the stroke width", "rotate or move a layer", "lock a layer"])
                    .with_args([drawing_layer_id_arg(), drawing_layer_field_arg(), drawing_layer_value_arg()]),
            )
            .action_interactive_job("patchLayer", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(
                drawing_internal_action("patchLayers", LocalizedLabel::native("Patch Layers", "Ebenen aktualisieren"), ActionKind::Mutation)
                    .describe(LocalizedLabel::native("Sets the same named property on several layers at once — see Patch Layer for the field vocabulary.", "Setzt dieselbe benannte Eigenschaft auf mehreren Ebenen gleichzeitig — siehe Ebene aktualisieren für die Feldliste."))
                    .use_when(["set the colour of all selected layers", "hide several layers"])
                    .with_args([semio_framework_plugin::ActionArgDef::text_list("layerIds", LocalizedLabel::native("Layer Ids", "Ebenen-Ids")).required(), drawing_layer_field_arg(), drawing_layer_value_arg()]),
            )
            .action_interactive_job("patchLayers", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(semio_framework_plugin::ActionDefinition::new("canvasPointerDown", LocalizedLabel::native("Canvas Pointer Down", "Leinwand-Zeiger gedrückt"), ActionKind::Mutation, "mouse-pointer").input_event())
            .action_interactive_job("canvasPointerDown", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(semio_framework_plugin::ActionDefinition::new("canvasPointerUp", LocalizedLabel::native("Canvas Pointer Up", "Leinwand-Zeiger losgelassen"), ActionKind::Mutation, "mouse-pointer").input_event())
            .action_interactive_job("canvasPointerUp", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(drawing_input_event("canvasDoubleClick", LocalizedLabel::native("Canvas Double Click", "Leinwand-Doppelklick"), ActionKind::Mutation))
            .action_interactive_job("canvasDoubleClick", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(drawing_input_event("canvasCommitDraft", LocalizedLabel::native("Canvas Commit Draft", "Leinwand-Entwurf übernehmen"), ActionKind::Mutation))
            .action_interactive_job("canvasCommitDraft", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(semio_framework_plugin::ActionDefinition::new("canvasPointerMove", LocalizedLabel::native("Canvas Pointer Move", "Leinwand-Zeiger bewegen"), ActionKind::View, "mouse-pointer").input_event())
            .action_interactive_job("canvasPointerMove", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(drawing_input_event("canvasEscape", LocalizedLabel::native("Canvas Escape", "Leinwand abbrechen"), ActionKind::View))
            
}

/// ⚙️ The manifest slice 4 of 5 of `create_drawing_app`: its builder calls.
fn create_drawing_app_part_4(builder: semio_framework_plugin::app::EditorBuilder) -> semio_framework_plugin::app::EditorBuilder {
    builder.action_interactive_job("canvasEscape", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(semio_framework_plugin::ActionDefinition::new("engagementInput", LocalizedLabel::native("Engagement Input", "Eingabe"), ActionKind::View, "hand").input_event())
            .action_interactive_job("engagementInput", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::new("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"), ActionKind::View, "camera") })
            .action_interactive_job("setCamera", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(drawing_internal_action("setCameraZoom", LocalizedLabel::native("Set Camera Zoom", "Kamerazoom festlegen"), ActionKind::View))
            .action_interactive_job("setCameraZoom", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_audience("dropLayerKind", semio_framework_plugin::CapabilityAudience::Agent)
            .action_destructive("deleteLayer")
            .action_destructive("setSnapshot")
            .action_destructive("loadDocumentJson")
            .action_destructive("setActiveExample")
            .action_destructive("commitDocument")
            .action_destructive("exportDocument")
            .utility(drawing_utility("selectMarquee", LocalizedLabel::native("Marquee Select", "Rahmenauswahl"), "square-dashed", "Select", UtilityCategory::Selection))
            .utility(drawing_utility("selectLasso", LocalizedLabel::native("Lasso Select", "Lasso-Auswahl"), "lasso", "Select", UtilityCategory::Selection))
            .utility(drawing_utility("selectDirect", LocalizedLabel::native("Direct Select", "Direktauswahl"), "mouse-pointer-2", "Select", UtilityCategory::Selection))
            .utility(drawing_utility("editNodes", LocalizedLabel::native("Edit Nodes", "Knoten bearbeiten"), "spline", "Select", UtilityCategory::Selection))
            .utility(drawing_utility("pen", LocalizedLabel::native("Pen", "Stift"), "pen-tool", "Drawing", UtilityCategory::Utilities))
            .utility(drawing_utility("shapeRect", LocalizedLabel::native("Rectangle", "Rechteck"), "rectangle-tool", "Drawing", UtilityCategory::Utilities))
            .utility(drawing_utility("shapeEllipse", LocalizedLabel::native("Ellipse", "Ellipse"), "circle", "Drawing", UtilityCategory::Utilities))
            .utility(drawing_utility("shapeLine", LocalizedLabel::native("Line", "Linie"), "minus", "Drawing", UtilityCategory::Utilities))
            .utility(drawing_utility("shapePolygon", LocalizedLabel::native("Polygon", "Polygon"), "hexagon", "Drawing", UtilityCategory::Utilities))
            .utility(drawing_utility("booleanCombine", LocalizedLabel::native("Boolean", "Boolean"), "combine", "Combine", UtilityCategory::Utilities))
            .utility(drawing_utility("trace", LocalizedLabel::native("Trace", "Nachzeichnen"), "scan-line", "Combine", UtilityCategory::Utilities))
            .utility(drawing_utility("transformMove", LocalizedLabel::native("Pan", "Verschieben"), "move", "View", UtilityCategory::Utilities))
            .window_kind_utilities(DRAWING_PLAY_WINDOW_CANVAS, vec![
                "selectMarquee".into(), "selectLasso".into(), "selectDirect".into(), "editNodes".into(),
                "pen".into(), "shapeRect".into(), "shapeEllipse".into(), "shapeLine".into(), "shapePolygon".into(),
                "booleanCombine".into(), "trace".into(), "transformMove".into(),
            ])
            .window_kind_initial_utility(DRAWING_PLAY_WINDOW_CANVAS, "selectDirect")
            .interaction(InteractionDefinition {
                id: DRAWING_INTERACTION_DOMAIN.into(),
                label: LocalizedLabel::native("Strokes", "Striche"),
                granularities: vec![GranularityDefinition { id: DRAWING_INTERACTION_GRANULARITY.into(), label: LocalizedLabel::native("Stroke", "Strich"), icon_id: "pen-tool".into() }],
                hierarchy: HierarchyProvider::Topology,
                hover: HoverSpec::default(),
                selection: SelectionSpec {
                    modes: vec![SelectionMode::Multiple, SelectionMode::Single],
                    methods: vec![SelectionMethod::Pick, SelectionMethod::Rectangle, SelectionMethod::Lasso],
                    merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive],
                    transitive: false,
                    broadcast: true,
                },
            })
            .interaction(InteractionDefinition {
                id:DRAWING_POINT_DOMAIN.into(),
                label:LocalizedLabel::native("Path Points","Pfadpunkte"),
                granularities:vec![GranularityDefinition {id:DRAWING_POINT_GRANULARITY.into(),label:LocalizedLabel::native("Point","Punkt"),icon_id:"spline".into()}],
                hierarchy:HierarchyProvider::Topology,
                hover:HoverSpec::default(),
                selection:SelectionSpec {modes:vec![SelectionMode::Multiple],methods:vec![SelectionMethod::Pick],merges:vec![MergeMode::Replace,MergeMode::Additive,MergeMode::Subtractive,MergeMode::Invertive],transitive:false,broadcast:true},
            })
            .window_kind_interactions(DRAWING_PLAY_WINDOW_CANVAS, vec![InteractionRef::new(DRAWING_INTERACTION_DOMAIN),InteractionRef::new(DRAWING_POINT_DOMAIN)])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .keybinding("escape", "canvasEscape")
            .keybinding("enter", "canvasCommitDraft")
            .keybinding("delete", "deleteSelection")
            .keybinding("backspace", "deleteSelection")
            .keybinding("arrowleft", "nudgeSelectionLeft")
            .keybinding("shift+arrowleft", "nudgeSelectionLeftFast")
            .keybinding("arrowright", "nudgeSelectionRight")
            .keybinding("shift+arrowright", "nudgeSelectionRightFast")
            .keybinding("arrowup", "nudgeSelectionUp")
            .keybinding("shift+arrowup", "nudgeSelectionUpFast")
            
}

/// 🧲️ The manifest slice 5 of 5 of `create_drawing_app`: its builder calls.
fn create_drawing_app_part_5(builder: semio_framework_plugin::app::EditorBuilder) -> semio_framework_plugin::app::EditorBuilder {
    builder.keybinding("arrowdown", "nudgeSelectionDown")
            .keybinding("shift+arrowdown", "nudgeSelectionDownFast")
            .default_layout(edit::layout())
            .io(drawing_io())
            
}
