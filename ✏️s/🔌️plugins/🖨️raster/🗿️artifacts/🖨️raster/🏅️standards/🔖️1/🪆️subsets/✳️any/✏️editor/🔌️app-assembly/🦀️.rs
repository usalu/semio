//! 🔌️ Raster editor app assembly: the typed media io ports, the composite media, the internal action and utility ids and the raster app definition.

use super::*;
use semio_s_artifact_raster_raster_editor_panels::panels;
use crate::modes::edit;
use crate::modes::edit::windows::{composite, navigator};
use semio_s_artifact_raster_raster_model::RASTER_DOCUMENT_SCHEMA;
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;
use semio_framework::InteractiveJobClassification;
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::AppDefinition;
use semio_framework_plugin::ArtifactKindSpec;
use semio_framework_plugin::Editor;
use semio_framework_plugin::GranularityDefinition;
use semio_framework_plugin::HierarchyProvider;
use semio_framework_plugin::HoverSpec;
use semio_framework_plugin::InteractionDefinition;
use semio_framework_plugin::InteractionRef;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::Media;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaError;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaPayload;
use semio_framework_plugin::MediaType;
use semio_framework_plugin::MergeMode;
use semio_framework_plugin::OsMediaCapability;
use semio_framework_plugin::SelectionMethod;
use semio_framework_plugin::SelectionMode;
use semio_framework_plugin::SelectionSpec;
use semio_framework_plugin::UtilityCategory;
use semio_framework_plugin::UtilityDefinition;

//#region 🔖️Io
/// 🔌️ Relocated verbatim from `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES,
/// rule 4: anything returning `AppIo` or referencing an app type lives in `🎛️apps/<app>/`). This app's
/// typed media I/O surface (`AppDefinition.io`) — mirrors the `2d.raster` `ArtifactKindSpec` literal
/// `crate::artifact_kind` already declares, plus the app-specific `image:in`/
/// `image:out` ports (see below).
pub fn raster_io() -> semio_framework::AppIo {
    semio_framework::AppIo {
        artifact_schema: RASTER_DOCUMENT_SCHEMA.into(),
        artifact_media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster },
        ports: vec![raster_image_in_port(), raster_image_out_port()],
        export_formats: Vec::new(),
        import_formats: Vec::new(),
        artifact: semio_framework::ArtifactPresentation { id: "2d.raster".into(), name: "2D Raster".into(), dimension: "2d".into(), component_kind: "raster".into() },
    }
}

/// 🔌️ `image:in` — accepts raster imagery from upstream producers (e.g. draw's `vector:out`,
/// converted Vector→Raster) as a new composited layer. `Many`/optional: several upstream images may
/// feed in, and the port may sit unconnected.
pub fn raster_image_in_port() -> semio_framework::MediaPortSpec {
    semio_framework::MediaPortSpec {
        id: "image:in".into(),
        label: "Image".into(),
        direction: semio_framework::MediaPortDirection::In,
        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster },
        kind_id: None,
        required: false,
        multiplicity: semio_framework::PortMultiplicity::Many,
    }
}

/// 🔌️ `image:out` — the raster document's current composited raster, as `2d.image` media (workflow
/// port surface; WORKFLOWS-END-TO-END-TYPED-PORTS Wave 2 port recipe). `kind_id: "2d.image"` — the
/// shared framework-builtin interchange kind (declared on this app's `.artifact_kind(...)` below;
/// `shooting`'s `photos:out` declares the identical shape, harmless duplicate registrations).
pub fn raster_image_out_port() -> semio_framework::MediaPortSpec {
    semio_framework::MediaPortSpec {
        id: "image:out".into(),
        label: "Image".into(),
        direction: semio_framework::MediaPortDirection::Out,
        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster },
        kind_id: Some("2d.image".into()),
        required: false,
        multiplicity: semio_framework::PortMultiplicity::Many,
    }
}

/// 🖼️ Publishes the canonical layer composite as a PNG image port payload.
pub fn raster_composite_media(document: &RasterSnapshot) -> Result<Media, MediaError> {
    let image=semio_s_artifact_raster_raster_io::raster_composite_image(document).map_err(|error|MediaError::Payload("image:out".into(),error))?;
    let bytes=semio_s_artifact_raster_raster_io::png_bytes_from_semio_image(&image).map_err(|error|MediaError::Payload("image:out".into(),error))?;
    let png_base64=base64_codec::base64_standard_encode(bytes);
    Ok(Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster }, payload: MediaPayload::Structured { schema: "2d.image".into(), json: png_base64 } })
}
//#endregion 🔖️Io

//#region 🔖️Manifest
/// 🛠️ An internal (non-palette) action declaration — the panel/pointer/gesture-bound vocabulary
/// dispatched by the layer tree, catalogue drops and inspector, never a palette command.
fn raster_internal_action(id: &str, label: impl Into<LocalizedLabel>, kind: ActionKind) -> semio_framework_plugin::ActionDefinition {
    semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::bounded_catalog(id, label, kind) }
}

/// 🧰️ One composite-window utility declaration; ids must stay host-compatible (`paint*` prefix paints,
/// `paintEraser` erases, `paintBucket` fills, `selectMarquee` selects) because the scene's active utility feeds `RasterHost`.
fn raster_utility(id: &str, label: impl Into<LocalizedLabel>, icon: &str, group: &str, category: UtilityCategory) -> UtilityDefinition {
    UtilityDefinition { group: Some(group.into()), category: Some(category), ..UtilityDefinition::new(id, label, icon) }
}

/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/keybinding/utility declarations (which have no dedicated `_def` passthrough) are
/// written out inline.
///
/// 📚️ The navbar dropdown reads `RasterPlayApp::examples()`, which `.editor` stamps onto the
/// manifest. The catalogue lives on the artifact, not on the plugin root.
pub fn create_raster_app() -> AppDefinition {
    Editor::builder(semio_s_artifact_raster_raster_model::RASTER_DIALECT).document(["semio", "raster"])
            .artifact_kind(semio_s_artifact_raster_raster_io::artifact_kind())
            .artifact_kind(ArtifactKindSpec {
                id: "2d.image".into(),
                label: LocalizedLabel::native("2D Image", "2D-Bild"),
                source_format: "2d.image".into(),
                component_kind: "image".into(),
                dimension: "2d".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster },
                schema: "2d.image".into(),
                export_formats: vec![],
                import_formats: vec![],
                    export_stdio_kinds: vec!["stdio.png".into()],
        import_stdio_kinds: vec!["stdio.png".into()],
    })
            .icon_id("raster")
            .mode_def(edit::definition())
            .default_mode_id(edit::RASTER_PLAY_MODE_EDIT)
            .window_kind_def(composite::definition())
            .window_kind_def(navigator::definition())
            .default_layout(edit::layout())
            .panel_tab_def(crate::panels::document::definition())
            .panel_tab_def(crate::panels::catalogue::definition())
            .panel_tab_def(crate::panels::masks::definition())
            .panel_tab_def(crate::panels::inspection::definition())
            .shell_action("exportPng", LocalizedLabel::native("Export PNG", "PNG exportieren"))
            .action_describe("exportPng", LocalizedLabel::native("Downloads the visible image as PNG with transparency. Layers remain editable.", "Lädt das sichtbare Bild als PNG mit Transparenz herunter. Die Ebenen bleiben bearbeitbar."))
            .action_interactive_job("exportPng", InteractiveJobClassification::Migrated)
            .mutation("addLayer", LocalizedLabel::native("Add Layer", "Ebene hinzufügen"))
            .action_with(raster_internal_action("flattenLayers", LocalizedLabel::native("Flatten Image", "Bild reduzieren"), ActionKind::Mutation))
            .action_describe("flattenLayers", LocalizedLabel::native("Replaces all layers with the visible image. Undo restores the original layers.", "Ersetzt alle Ebenen durch das sichtbare Bild. Rückgängig stellt die ursprünglichen Ebenen wieder her."))
            .action_interactive_job("flattenLayers", InteractiveJobClassification::Migrated)
            .action_with(raster_internal_action("mergeDown", LocalizedLabel::native("Merge Down", "Nach unten vereinen"), ActionKind::Mutation))
            .action_describe("mergeDown", LocalizedLabel::native("Merges the selected visible layer with its lower normal-blend sibling. Undo restores both layers.", "Vereint die ausgewählte sichtbare Ebene mit der darunterliegenden Ebene im normalen Mischmodus. Rückgängig stellt beide Ebenen wieder her."))
            .action_interactive_job("mergeDown", InteractiveJobClassification::Migrated)
            .action_with(raster_internal_action("paintStroke", LocalizedLabel::native("Paint Stroke", "Strich malen"), ActionKind::Mutation))
            .action_describe("paintStroke", LocalizedLabel::native("Paints one brush or eraser stroke on the selected layer's pixels or mask with the session brush. The whole stroke is one undoable edit whose brush and points can be edited in history.", "Malt einen Pinsel- oder Radierstrich mit dem Sitzungspinsel auf die Pixel oder die Maske der gewählten Ebene. Der ganze Strich ist ein rückgängig machbarer Schritt, dessen Pinsel und Punkte im Verlauf bearbeitet werden können."))
            .action_interactive_job("paintStroke", InteractiveJobClassification::Migrated)
            .action_with(raster_internal_action("fillRegion", LocalizedLabel::native("Fill Region", "Region füllen"), ActionKind::Mutation))
            .action_describe("fillRegion", LocalizedLabel::native("Fills the connected region around the clicked pixel of the selected layer's pixels or mask with the session colour, within the colour tolerance and the pixel selection. The fill is one undoable edit whose seed, tolerance and colour can be edited in history.", "Füllt die zusammenhängende Region um das angeklickte Pixel der Pixel oder der Maske der gewählten Ebene mit der Sitzungsfarbe, innerhalb der Farbtoleranz und der Pixelauswahl. Die Füllung ist ein rückgängig machbarer Schritt, dessen Startpixel, Toleranz und Farbe im Verlauf bearbeitet werden können."))
            .action_interactive_job("fillRegion", InteractiveJobClassification::Migrated)
            .action_with(raster_internal_action("applyFilter", LocalizedLabel::native("Apply Filter", "Filter anwenden"), ActionKind::Mutation))
            .action_describe("applyFilter", LocalizedLabel::native("Runs one image filter (invert, grayscale, clear, flip, brightness, contrast, saturation, gamma, threshold, posterize, blur or sharpen, with its amount) over the selected layer's pixels, within the pixel selection. The filter is one undoable edit whose filter and amount stay editable in history.", "Wendet einen Bildfilter (Invertieren, Graustufen, Leeren, Spiegeln, Helligkeit, Kontrast, Sättigung, Gamma, Schwellenwert, Tontrennung, Weichzeichnen oder Schärfen, mit seiner Stärke) auf die Pixel der gewählten Ebene innerhalb der Pixelauswahl an. Der Filter ist eine rückgängig machbare Bearbeitung, deren Filter und Stärke im Verlauf bearbeitbar bleiben."))
            .action_interactive_job("applyFilter", InteractiveJobClassification::Migrated)
            .action_with(raster_internal_action("transformImage", LocalizedLabel::native("Transform Image", "Bild umformen"), ActionKind::Mutation))
            .action_describe("transformImage", LocalizedLabel::native("Rotates the selected layer's image a quarter turn, resizes it or crops it to a window. The layer keeps its place and scale on the canvas; the change is one undoable edit whose operation and extent can be edited in history.", "Dreht das Bild der gewählten Ebene um eine Vierteldrehung, skaliert es oder schneidet es auf ein Fenster zu. Die Ebene behält Lage und Maßstab auf der Leinwand; die Änderung ist ein rückgängig machbarer Schritt, dessen Operation und Ausmaß im Verlauf bearbeitet werden können."))
            .action_interactive_job("transformImage", InteractiveJobClassification::Migrated)
            .action_with(raster_internal_action("fillSelection", LocalizedLabel::native("Fill Selection", "Auswahl füllen"), ActionKind::Mutation))
            .action_describe("fillSelection", LocalizedLabel::native("Fills the pixel selection of the selected layer's pixels or mask with the session colour (the mask value at the brush opacity), or the whole image without a selection. The fill is one undoable edit whose colour and selection can be edited in history.", "Füllt die Pixelauswahl der Pixel oder der Maske der gewählten Ebene mit der Sitzungsfarbe (dem Maskenwert mit der Pinseldeckkraft), ohne Auswahl das ganze Bild. Die Füllung ist ein rückgängig machbarer Schritt, dessen Farbe und Auswahl im Verlauf bearbeitet werden können."))
            .action_interactive_job("fillSelection", InteractiveJobClassification::Migrated)
            .action_with(raster_internal_action("maskFromSelection", LocalizedLabel::native("Mask From Selection", "Maske aus Auswahl"), ActionKind::Mutation))
            .action_describe("maskFromSelection", LocalizedLabel::native("Creates an undoable layer mask from the current pixel selection.", "Erstellt eine rückgängig machbare Ebenenmaske aus der aktuellen Pixelauswahl."))
            .action_interactive_job("maskFromSelection", InteractiveJobClassification::Migrated)
            .action_with(semio_framework_plugin::ActionDefinition::new("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), ActionKind::Mutation, "panel-left"))
            .action_with(raster_internal_action("setLayerVisible", LocalizedLabel::native("Set Layer Visible", "Ebenensichtbarkeit festlegen"), ActionKind::Mutation))
            .action_with(raster_internal_action("toggleLayerVisible", LocalizedLabel::native("Toggle Layer Visible", "Ebenensichtbarkeit umschalten"), ActionKind::Mutation))
            .action_with(raster_internal_action("dropLayerKind", LocalizedLabel::native("Drop Layer Kind", "Ebenenart ablegen"), ActionKind::Mutation))
            .action_audience("dropLayerKind", semio_framework_plugin::CapabilityAudience::Agent)
            .action_with(raster_internal_action("deleteLayer", LocalizedLabel::native("Delete Layer", "Ebene löschen"), ActionKind::Mutation))
            .action_with(raster_internal_action("duplicateLayer", LocalizedLabel::native("Duplicate Layer", "Ebene duplizieren"), ActionKind::Mutation))
            .action_with(raster_internal_action("patchLayer", LocalizedLabel::native("Patch Layer", "Ebene aktualisieren"), ActionKind::Mutation))
            .action_with(raster_internal_action("patchLayers", LocalizedLabel::native("Patch Layers", "Ebenen aktualisieren"), ActionKind::Mutation))
            .action_with(raster_internal_action("moveLayer", LocalizedLabel::native("Move Layer", "Ebene verschieben"), ActionKind::Mutation))
            .interaction(InteractionDefinition {
                id: RASTER_INTERACTION_DOMAIN.into(),
                label: LocalizedLabel::native("Layers", "Ebenen"),
                granularities: vec![GranularityDefinition { id: RASTER_INTERACTION_GRANULARITY.into(), label: LocalizedLabel::native("Layer", "Ebene"), icon_id: "image".into() }],
                hierarchy: HierarchyProvider::Topology,
                hover: HoverSpec::default(),
                selection: SelectionSpec {
                    modes: vec![SelectionMode::Multiple, SelectionMode::Single],
                    methods: vec![SelectionMethod::Pick],
                    merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive],
                    transitive: false,
                    broadcast: true,
                },
            })
            .window_kind_interactions(composite::RASTER_PLAY_WINDOW_COMPOSITE, vec![InteractionRef::new(RASTER_INTERACTION_DOMAIN)])
            .action_with(raster_internal_action("setBrushSize", LocalizedLabel::native("Set Brush Size", "Pinselgröße festlegen"), ActionKind::View))
            .action_with(raster_internal_action("setBrushOpacity", LocalizedLabel::native("Set Brush Opacity", "Pinseldeckkraft festlegen"), ActionKind::View))
            .action_with(raster_internal_action("setBrushColor", LocalizedLabel::native("Set Foreground Color", "Vordergrundfarbe festlegen"), ActionKind::View))
            .action_with(raster_internal_action("setBrushHardness", LocalizedLabel::native("Set Brush Hardness", "Pinselhärte festlegen"), ActionKind::View))
            .action_with(raster_internal_action("setPaintTarget",LocalizedLabel::native("Set Paint Target","Bearbeitungsziel festlegen"),ActionKind::View))
            .action_with(raster_internal_action("setPixelSelection",LocalizedLabel::native("Set Pixel Selection","Pixelauswahl festlegen"),ActionKind::View))
            .action_with(raster_internal_action("setMaskValue",LocalizedLabel::native("Set Mask Value","Maskenwert festlegen"),ActionKind::View))
            .action_with(raster_internal_action("setFillTolerance", LocalizedLabel::native("Set Fill Tolerance", "Fülltoleranz festlegen"), ActionKind::View))
            .action_with(raster_internal_action("setCompositeViewport", LocalizedLabel::native("Set Composite Viewport", "Komposit-Ansichtsfenster festlegen"), ActionKind::View))
            .action_with(semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::new("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"), ActionKind::View, "camera") })
            .action_with(raster_internal_action("setCameraZoom", LocalizedLabel::native("Set Camera Zoom", "Kamerazoom festlegen"), ActionKind::View))
            .action_args("addLayer", vec![
                ActionArgDef::select("kind", LocalizedLabel::native("Layer Kind", "Ebenenart"), vec![
                    ActionArgOption::new("pixel", LocalizedLabel::native("Pixel", "Pixel")),
                    ActionArgOption::new("group", LocalizedLabel::native("Group", "Gruppe")),
                    ActionArgOption::new("adjustment", LocalizedLabel::native("Adjustment", "Anpassung")),
                ]).required().default_value(&"pixel"),
            ])
            .action_describe("addLayer", LocalizedLabel::native("Adds a new raster layer to the image — a pixel layer, a group, or an adjustment layer.", "Fügt dem Bild eine neue Rasterebene hinzu — Pixelebene, Gruppe oder Anpassungsebene."))
            .action_use_when("addLayer", vec!["add a pixel layer".into(), "add an adjustment layer".into(), "new layer in the image".into()])
            .action_describe("setActiveExample", LocalizedLabel::native("Replaces the whole image with one of the plugin's declared playground examples.", "Ersetzt das gesamte Bild durch eines der deklarierten Beispiele des Plugins."))
            .action_describe("setLayerVisible", LocalizedLabel::native("Shows or hides one raster layer explicitly.", "Blendet eine Rasterebene gezielt ein oder aus."))
            .action_describe("toggleLayerVisible", LocalizedLabel::native("Flips one raster layer between visible and hidden.", "Schaltet eine Rasterebene zwischen sichtbar und ausgeblendet um."))
            .action_describe("dropLayerKind", LocalizedLabel::native("Creates a raster layer of the given kind at a drop target in the layer tree.", "Erzeugt eine Rasterebene der angegebenen Art an einer Ablagestelle im Ebenenbaum."))
            .action_describe("deleteLayer", LocalizedLabel::native("Removes one raster layer from the image by id.", "Entfernt eine Rasterebene anhand ihrer Id aus dem Bild."))
            .action_describe("duplicateLayer", LocalizedLabel::native("Copies one raster layer and inserts the copy above the original.", "Kopiert eine Rasterebene und fügt die Kopie über dem Original ein."))
            .action_describe("patchLayer", LocalizedLabel::native("Sets one named property of one raster layer — its name, opacity, blend mode or visibility.", "Setzt eine benannte Eigenschaft einer Rasterebene — Name, Deckkraft, Mischmodus oder Sichtbarkeit."))
            .action_use_when("patchLayer", vec!["rename a layer".into(), "change the layer opacity".into(), "set the blend mode".into()])
            .action_describe("patchLayers", LocalizedLabel::native("Sets the same named property on several raster layers at once.", "Setzt dieselbe benannte Eigenschaft auf mehreren Rasterebenen gleichzeitig."))
            .action_describe("moveLayer", LocalizedLabel::native("Reorders one raster layer within the layer stack.", "Ordnet eine Rasterebene im Ebenenstapel um."))
            .action_describe("setBrushSize", LocalizedLabel::native("Sets the painting brush diameter for this session.", "Legt den Pinseldurchmesser für diese Sitzung fest."))
            .action_describe("setFillTolerance", LocalizedLabel::native("Sets how far a pixel's colour may differ from the clicked pixel for the bucket to fill it, for this session.", "Legt fest, wie weit die Farbe eines Pixels vom angeklickten Pixel abweichen darf, damit der Farbeimer ihn füllt, für diese Sitzung."))
            .action_describe("setBrushOpacity", LocalizedLabel::native("Sets the painting brush opacity for this session.", "Legt die Pinseldeckkraft für diese Sitzung fest."))
            .action_destructive("deleteLayer")
            .action_destructive("setActiveExample")
            .action_interactive_job("addLayer", InteractiveJobClassification::Migrated)
            .action_interactive_job("dropLayerKind", InteractiveJobClassification::Migrated)
            .action_interactive_job("setLayerVisible", InteractiveJobClassification::Migrated)
            .action_interactive_job("toggleLayerVisible", InteractiveJobClassification::Migrated)
            .action_interactive_job("deleteLayer", InteractiveJobClassification::Migrated)
            .action_interactive_job("duplicateLayer", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchLayer", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchLayers", InteractiveJobClassification::Migrated)
            .action_interactive_job("moveLayer", InteractiveJobClassification::Migrated)
            .action_interactive_job("setBrushSize", InteractiveJobClassification::Migrated)
            .action_interactive_job("setBrushOpacity", InteractiveJobClassification::Migrated)
            .action_interactive_job("setBrushColor", InteractiveJobClassification::Migrated)
            .action_interactive_job("setBrushHardness", InteractiveJobClassification::Migrated)
            .action_interactive_job("setPaintTarget",InteractiveJobClassification::Migrated)
            .action_interactive_job("setPixelSelection",InteractiveJobClassification::Migrated)
            .action_interactive_job("setMaskValue",InteractiveJobClassification::Migrated)
            .action_interactive_job("setFillTolerance", InteractiveJobClassification::Migrated)
            .action_interactive_job("setCompositeViewport", InteractiveJobClassification::Migrated)
            .action_interactive_job("setCamera", InteractiveJobClassification::Migrated)
            .action_interactive_job("setCameraZoom", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
            .utility(raster_utility("selectMarquee", LocalizedLabel::native("Marquee Select", "Rahmenauswahl"), "square-dashed", "Select", UtilityCategory::Selection))
            .utility(raster_utility("paintBrush", LocalizedLabel::native("Brush", "Pinsel"), "paintbrush", "Paint", UtilityCategory::Utilities))
            .utility(raster_utility("paintEraser", LocalizedLabel::native("Eraser", "Radiergummi"), "eraser", "Paint", UtilityCategory::Utilities))
            .utility(raster_utility("paintBucket", LocalizedLabel::native("Bucket", "Farbeimer"), "paint-bucket", "Paint", UtilityCategory::Utilities))
            .window_kind_utilities(composite::RASTER_PLAY_WINDOW_COMPOSITE, vec![
                "selectMarquee".into(), "paintBrush".into(), "paintEraser".into(), "paintBucket".into(),
            ])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .build_definition()
}
//#endregion 🔖️Manifest
