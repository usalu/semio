//! 📄 PDF page editor — paints every page and addresses text, vectors, images, forms, shadings,
//! and annotations as objects the end user can insert, move, resize, restyle, and delete.
//! Document info and page structure use the same command channel. A canvas click hit-tests a page
//! object and redispatches `interactionSelect`; it does not mutate the PDF. Raw fields that have no
//! page geometry stay on the snapshot details window.

use crate::standards::v1_7::subsets::base::schema::diff::PdfPageBox;
use crate::standards::v1_7::subsets::base::schema::mutations::{
    insert_object::InsertObject, insert_page::InsertPage, move_page::MovePage, remove_catalog_entry::RemoveCatalogEntry, remove_object::RemoveObject, remove_embedded_file::RemoveEmbeddedFile, remove_named_destination::RemoveNamedDestination, remove_page::RemovePage, remove_trailer_entry::RemoveTrailerEntry, set_annotation::SetAnnotation, set_embedded_file::SetEmbeddedFile, set_acro_form::SetAcroForm, set_document_id::SetDocumentId, set_color_space::SetColorSpace, set_catalog_entry::SetCatalogEntry, set_encryption::SetEncryption, set_ext_g_state::SetExtGState, set_font::SetFont, set_form::SetForm, set_snapshot::SetSnapshot, set_trailer_entry::SetTrailerEntry, set_open_action::SetOpenAction, set_output_intents::SetOutputIntents, set_pattern::SetPattern, set_properties::SetProperties, set_image::SetImage, set_info::SetInfo, set_language::SetLanguage, set_mark_info::SetMarkInfo, set_metadata::SetMetadata, set_named_destination::SetNamedDestination, set_object_value::SetObjectValue, set_optional_content::SetOptionalContent, set_outlines::SetOutlines, set_page_box::SetPageBox, set_page_content::SetPageContent, set_page_labels::SetPageLabels, set_page_layout::SetPageLayout, set_page_media_box::SetPageMediaBox, set_page_mode::SetPageMode, set_page_rotation::SetPageRotation, set_page_user_unit::SetPageUserUnit, set_shading::SetShading, set_viewer_preferences::SetViewerPreferences, PdfMutation,
};
use crate::standards::v1_7::subsets::base::schema::snapshot::{ObjRef, PdfAction, PdfAnnotation, PdfAppearance, PdfAppearanceEntry, PdfBaseEncoding, PdfCharProc, PdfColorSpace, PdfDate, PdfDestination, PdfDestinationFit, PdfDictEntry, PdfEmbeddedFile, PdfEncryption, PdfEncryptionAlgorithm, PdfExtGState, PdfFont, PdfFontDescriptor, PdfFontKind, PdfFontProgram, PdfFormField, PdfFormFieldKind, PdfFunction, PdfFormXObject, PdfImage, PdfImageCodec, PdfImageMask, PdfMarkInfo, PdfMatrix, PdfNamedColorSpace, PdfNamedDestination, PdfNamedProperties, PdfObject, PdfOpenAction, PdfOutputIntent, PdfPattern, PdfPatternKind, PdfSimpleEncoding, PdfOp, PdfOptionalContentGroup, PdfOutlineItem, PdfPage, PdfPageLabelRange, PdfPageLabelStyle, PdfPageLayout, PdfPageMode, PdfShadingKind, PdfTextArrayItem, PdfTextString, PdfTransparencyGroup, PdfViewerPreferences, PDF_IDENTITY_MATRIX};
use crate::PdfSnapshot;
use semio_framework_plugin::{ActionArgDef, ActionDefinition, ActionKind, Canvas2dScene, Fault, FaultCode, FaultOrigin, LocalizedLabel, SurfaceKind, WindowKindDefinition, WindowOptions};
use serde_json::{json, Value};

pub const WINDOW_KIND_ID: &str = "pdf.page";
pub const BODY_KEY: &str = "pdf.page";

pub const OBJECT_DOMAIN: &str = "objects";

const PAGE_ACTIONS: [&str; 60] = [
    "set-text",
    "move",
    "resize",
    "delete",
    "set-fill",
    "set-stroke",
    "insert-text",
    "insert-rectangle",
    "insert-line",
    "insert-image",
    "insert-page",
    "remove-page",
    "move-page",
    "set-page-size",
    "set-info",
    "set-annotation",
    "set-image",
    "set-font",
    "set-outline",
    "set-page-rotation",
    "set-page-box",
    "set-page-user-unit",
    "set-language",
    "set-page-layout",
    "set-page-mode",
    "set-optional-content",
    "set-embedded-file",
    "remove-embedded-file",
    "set-named-destination",
    "remove-named-destination",
    "set-page-label",
    "set-mark-info",
    "set-metadata",
    "set-viewer-preferences",
    "set-encryption",
    "set-output-intent",
    "set-form-field",
    "set-open-action",
    "set-document-id",
    "set-font-program",
    "set-graphics-state",
    "set-pattern",
    "set-color-space",
    "set-properties",
    "set-font-metrics",
    "set-image-mask",
    "set-form-content",
    "set-page-transition",
    "set-catalog-entry",
    "set-trailer-entry",
    "set-annotation-appearance",
    "set-glyph",
    "set-indirect-object",
    "set-mesh-data",
    "set-info-field",
    "set-page-extra",
    "set-annotation-style",
    "canvasPointerDown",
    "canvasPointerMove",
    "canvasPointerUp",
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ObjectKind {
    Text,
    Vector,
    Image,
    Form,
    Shading,
    Annotation,
}

impl ObjectKind {
    fn letter(self) -> char {
        match self {
            Self::Text => 't',
            Self::Vector => 'v',
            Self::Image => 'i',
            Self::Form => 'f',
            Self::Shading => 's',
            Self::Annotation => 'a',
        }
    }
}

/// 🎯 One paintable, addressable detail on a page.
#[derive(Clone, Debug, PartialEq)]
pub struct PageObject {
    pub id: String,
    pub page: usize,
    pub kind: ObjectKind,
    pub start: usize,
    pub end: usize,
    pub position_op: Option<usize>,
    pub transform_op: Option<usize>,
    pub ctm: PdfMatrix,
    pub parent_ctm: PdfMatrix,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub text: String,
    pub font_size: f64,
    pub fill: [f64; 3],
    pub stroke: [f64; 3],
    pub line_width: f64,
}

/// ✏️ One end-user edit, already checked against the action's required arguments.
#[derive(Clone, Debug, PartialEq)]
pub struct PdfPageEdit {
    pub action: String,
    pub payload: String,
}

/// 🏷️ Stable action id for the command log.
pub fn static_action(action: &str) -> &'static str {
    PAGE_ACTIONS.into_iter().find(|candidate| *candidate == action).unwrap_or("page-edit")
}

/// 🧭 Whether `action` is a page-editor command rather than snapshot details or `set-page`.
pub fn is_page_action(action: &str) -> bool {
    PAGE_ACTIONS.contains(&action)
}

/// 🪟 The page window: a canvas, with one action per edit the surface can publish.
pub fn window_definition() -> WindowKindDefinition {
    let page = || ActionArgDef::number("page", LocalizedLabel::native("Page", "Seite")).required();
    let object = || ActionArgDef::text("object", LocalizedLabel::native("Object", "Objekt")).min_length(1).required();
    let text = || ActionArgDef::text("text", LocalizedLabel::native("Text", "Text")).min_length(0).required();
    let x = || ActionArgDef::number("x", LocalizedLabel::native("X", "X")).required();
    let y = || ActionArgDef::number("y", LocalizedLabel::native("Y", "Y")).required();
    let width = || ActionArgDef::number("width", LocalizedLabel::native("Width", "Breite")).required();
    let height = || ActionArgDef::number("height", LocalizedLabel::native("Height", "Höhe")).required();
    let red = || ActionArgDef::number("red", LocalizedLabel::native("Red", "Rot")).required();
    let green = || ActionArgDef::number("green", LocalizedLabel::native("Green", "Grün")).required();
    let blue = || ActionArgDef::number("blue", LocalizedLabel::native("Blue", "Blau")).required();
    let action = |id, en, de, args| {
        let mut definition = ActionDefinition::bounded_catalog(id, LocalizedLabel::native(en, de), ActionKind::Mutation).with_args(args);
        definition.semantics.execution.interactive_job = semio_framework_plugin::InteractiveJobClassification::Migrated;
        definition
    };
    WindowKindDefinition {
        id: WINDOW_KIND_ID.into(),
        label: LocalizedLabel::native("Page", "Seite"),
        body_key: BODY_KEY.into(),
        surface_kind: SurfaceKind::Canvas2d,
        icon_id: "file-text".into(),
        options: WindowOptions::default(),
        actions: vec![
            action("canvasPointerDown", "Canvas Pointer Down", "Leinwandzeiger", vec![ActionArgDef::number("x", LocalizedLabel::native("X", "X")), ActionArgDef::number("y", LocalizedLabel::native("Y", "Y"))]).input_event(),
            action("canvasPointerMove", "Canvas Pointer Move", "Leinwandzeiger bewegt", vec![ActionArgDef::number("x", LocalizedLabel::native("X", "X")), ActionArgDef::number("y", LocalizedLabel::native("Y", "Y"))]).input_event(),
            action("canvasPointerUp", "Canvas Pointer Up", "Leinwandzeiger losgelassen", vec![ActionArgDef::number("x", LocalizedLabel::native("X", "X")), ActionArgDef::number("y", LocalizedLabel::native("Y", "Y"))]).input_event(),
            action(
                "set-page",
                "Set Page Text",
                "Seitentext setzen",
                vec![page(), ActionArgDef::number("item", LocalizedLabel::native("Text Item", "Textelement")).required(), ActionArgDef::target_revision("revision", LocalizedLabel::native("Revision", "Revision")).min_length(1), text()],
            )
            .describe(LocalizedLabel::native(
                "Replaces the addressed page text when its revision still matches; the previous text remains available through undo.",
                "Ersetzt den adressierten Seitentext, wenn seine Revision noch übereinstimmt; der bisherige Text bleibt über Rückgängig verfügbar.",
            ))
            .in_palette(false),
            action("set-text", "Set Text", "Text setzen", vec![page(), object(), text()]),
            action("move", "Move", "Verschieben", vec![page(), object(), x(), y()]),
            action("resize", "Resize", "Größe ändern", vec![page(), object(), x(), y(), width(), height()]),
            action("delete", "Delete", "Löschen", vec![page(), object()]),
            action("set-fill", "Set Fill", "Füllung setzen", vec![page(), object(), red(), green(), blue()]),
            action("set-stroke", "Set Stroke", "Linie setzen", vec![page(), object(), red(), green(), blue(), width()]),
            action("insert-text", "Insert Text", "Text einfügen", vec![page(), x(), y(), text(), height()]),
            action("insert-rectangle", "Insert Rectangle", "Rechteck einfügen", vec![page(), x(), y(), width(), height(), red(), green(), blue()]),
            action("insert-line", "Insert Line", "Linie einfügen", vec![page(), x(), y(), width(), height(), red(), green(), blue()]),
            action("insert-image", "Insert Image", "Bild einfügen", vec![page(), x(), y(), width(), height()]),
            action("set-image", "Set Image Samples", "Bildpunkte setzen", vec![page(), object(), width(), height(), text()]),
            action("insert-page", "Insert Page", "Seite einfügen", vec![page(), width(), height()]),
            action("remove-page", "Remove Page", "Seite entfernen", vec![page()]),
            action("move-page", "Move Page", "Seite verschieben", vec![page(), x()]),
            action("set-page-size", "Set Page Size", "Seitengröße setzen", vec![page(), width(), height()]),
            action("set-info", "Set Document Info", "Dokumentinfo setzen", vec![ActionArgDef::text("text", LocalizedLabel::native("Title", "Titel")).min_length(0), ActionArgDef::text("extra", LocalizedLabel::native("Author", "Autor")).min_length(0), ActionArgDef::text("object", LocalizedLabel::native("Subject", "Thema")).min_length(0)]),
            action("set-info-field", "Set Info Field", "Infofeld setzen", vec![object(), text()]),
            action("set-annotation", "Set Annotation", "Anmerkung setzen", vec![page(), object(), text(), x(), y(), width(), height()]),
            action("set-annotation-style", "Set Annotation Style", "Anmerkung gestalten", vec![page(), object(), text(), ActionArgDef::text("extra", LocalizedLabel::native("Value", "Wert")).min_length(0), x(), red(), green(), blue()]),
            action("set-page-extra", "Set Page Extra", "Seite ergänzen", vec![page(), object(), text(), ActionArgDef::text("extra", LocalizedLabel::native("Key", "Schlüssel")).min_length(0), x(), y()]),
            action("set-font", "Set Font", "Schrift setzen", vec![page(), object(), text()]),
            action("set-outline", "Set Outline", "Lesezeichen setzen", vec![page(), x(), text()]),
            action("set-page-rotation", "Set Page Rotation", "Seitendrehung setzen", vec![page(), x()]),
            action("set-page-box", "Set Page Box", "Seitenbox setzen", vec![page(), text(), x(), y(), width(), height()]),
            action("set-page-user-unit", "Set Page User Unit", "Seiteneinheit setzen", vec![page(), x()]),
            action("set-language", "Set Language", "Sprache setzen", vec![ActionArgDef::text("text", LocalizedLabel::native("Language", "Sprache")).min_length(0)]),
            action("set-page-layout", "Set Page Layout", "Seitenlayout setzen", vec![ActionArgDef::text("text", LocalizedLabel::native("Layout", "Layout")).min_length(0)]),
            action("set-page-mode", "Set Page Mode", "Seitenmodus setzen", vec![ActionArgDef::text("text", LocalizedLabel::native("Mode", "Modus")).min_length(0)]),
            action("set-optional-content", "Set Optional Content", "Ebene setzen", vec![object(), text(), x()]),
            action("set-embedded-file", "Set Embedded File", "Datei einbetten", vec![object(), ActionArgDef::text("text", LocalizedLabel::native("File Name", "Dateiname")).min_length(0), ActionArgDef::text("extra", LocalizedLabel::native("Contents", "Inhalt")).min_length(0)]),
            action("remove-embedded-file", "Remove Embedded File", "Datei entfernen", vec![object()]),
            action("set-named-destination", "Set Named Destination", "Ziel setzen", vec![page(), object()]),
            action("remove-named-destination", "Remove Named Destination", "Ziel entfernen", vec![object()]),
            action("set-page-label", "Set Page Label", "Seitenbeschriftung setzen", vec![page(), ActionArgDef::text("text", LocalizedLabel::native("Style", "Stil")).min_length(0), ActionArgDef::text("extra", LocalizedLabel::native("Prefix", "Präfix")).min_length(0), x()]),
            action("set-mark-info", "Set Mark Info", "Markierung setzen", vec![x(), y(), width()]),
            action("set-metadata", "Set Metadata", "Metadaten setzen", vec![ActionArgDef::text("text", LocalizedLabel::native("XMP", "XMP")).min_length(0)]),
            action("set-viewer-preferences", "Set Viewer Preference", "Betrachter setzen", vec![object(), x()]),
            action("set-encryption", "Set Encryption", "Verschlüsselung setzen", vec![ActionArgDef::text("text", LocalizedLabel::native("Algorithm", "Algorithmus")).min_length(0), ActionArgDef::text("extra", LocalizedLabel::native("User Password", "Benutzerpasswort")).min_length(0), ActionArgDef::text("object", LocalizedLabel::native("Owner Password", "Besitzerpasswort")).min_length(0), x()]),
            action("set-output-intent", "Set Output Intent", "Ausgabebedingung setzen", vec![ActionArgDef::text("object", LocalizedLabel::native("Subtype", "Untertyp")).min_length(0), ActionArgDef::text("text", LocalizedLabel::native("Identifier", "Kennung")).min_length(0), ActionArgDef::text("extra", LocalizedLabel::native("Info", "Info")).min_length(0)]),
            action("set-form-field", "Set Form Field", "Formularfeld setzen", vec![ActionArgDef::text("object", LocalizedLabel::native("Name", "Name")).min_length(0), ActionArgDef::text("text", LocalizedLabel::native("Value", "Wert")).min_length(0), ActionArgDef::text("extra", LocalizedLabel::native("Kind", "Art")).min_length(0)]),
            action("set-open-action", "Set Open Action", "Öffnen-Aktion setzen", vec![page(), ActionArgDef::text("object", LocalizedLabel::native("Kind", "Art")).min_length(0), ActionArgDef::text("text", LocalizedLabel::native("URI", "URI")).min_length(0)]),
            action("set-document-id", "Set Document Id", "Dokumentkennung setzen", vec![ActionArgDef::text("text", LocalizedLabel::native("Permanent", "Dauerhaft")).min_length(0), ActionArgDef::text("extra", LocalizedLabel::native("Changing", "Wechselnd")).min_length(0)]),
            action("set-font-program", "Set Font Program", "Schriftprogramm setzen", vec![object(), text(), ActionArgDef::text("extra", LocalizedLabel::native("Program", "Programm")).min_length(1).required()]),
            action("set-graphics-state", "Set Graphics State", "Grafikzustand setzen", vec![object(), ActionArgDef::text("text", LocalizedLabel::native("Blend Mode", "Mischmodus")).min_length(0), x(), y()]),
            action("set-pattern", "Set Pattern", "Muster setzen", vec![object(), ActionArgDef::text("text", LocalizedLabel::native("Shading", "Verlauf")).min_length(0), x(), y(), width(), height()]),
            action("set-color-space", "Set Color Space", "Farbraum setzen", vec![object(), text(), ActionArgDef::text("extra", LocalizedLabel::native("Separation", "Sonderfarbe")).min_length(0)]),
            action("set-properties", "Set Properties", "Eigenschaften setzen", vec![object(), text(), ActionArgDef::text("extra", LocalizedLabel::native("Value", "Wert")).min_length(0)]),
            action("set-font-metrics", "Set Font Metrics", "Schriftmetrik setzen", vec![object(), ActionArgDef::text("text", LocalizedLabel::native("Encoding", "Kodierung")).min_length(0), ActionArgDef::text("extra", LocalizedLabel::native("Widths", "Breiten")).min_length(0), x(), y()]),
            action("set-image-mask", "Set Image Mask", "Bildmaske setzen", vec![object(), ActionArgDef::text("text", LocalizedLabel::native("Kind", "Art")).min_length(0), ActionArgDef::text("extra", LocalizedLabel::native("Mask", "Maske")).min_length(0)]),
            action("set-form-content", "Set Form Content", "Formularinhalt setzen", vec![object(), ActionArgDef::text("text", LocalizedLabel::native("Text", "Text")).min_length(0), x(), y(), width(), height()]),
            action("set-page-transition", "Set Page Transition", "Seitenübergang setzen", vec![page(), ActionArgDef::text("text", LocalizedLabel::native("Style", "Stil")).min_length(0), ActionArgDef::text("extra", LocalizedLabel::native("Direction", "Richtung")).min_length(0), x()]),
            action("set-catalog-entry", "Set Catalog Entry", "Katalogeintrag setzen", vec![object(), ActionArgDef::text("text", LocalizedLabel::native("Value", "Wert")).min_length(0)]),
            action("set-trailer-entry", "Set Trailer Entry", "Anhang setzen", vec![object(), ActionArgDef::text("text", LocalizedLabel::native("Value", "Wert")).min_length(0)]),
            action("set-annotation-appearance", "Set Annotation Appearance", "Erscheinungsbild setzen", vec![page(), object(), ActionArgDef::text("text", LocalizedLabel::native("Form", "Formular")).min_length(0)]),
            action("set-glyph", "Set Glyph", "Glyphe setzen", vec![object(), text(), x(), y(), width(), height()]),
            action("set-indirect-object", "Set Indirect Object", "Objekt setzen", vec![x(), y(), ActionArgDef::text("text", LocalizedLabel::native("Name", "Name")).min_length(0)]),
            action("set-mesh-data", "Set Mesh Data", "Netzdaten setzen", vec![object(), ActionArgDef::text("text", LocalizedLabel::native("Decode", "Dekodierung")).min_length(0), ActionArgDef::text("extra", LocalizedLabel::native("Vertices", "Knoten")).min_length(0)]),
        ],
        utilities: Vec::new(),
        interactions: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}

/// 🖼️ Paints every page, stacked top to bottom, with one layer per object.
pub fn render_window(snapshot: &PdfSnapshot) -> semio_framework_plugin::UiAssemblyResult<semio_framework_ui_contract::BuiltNode> {
    render_window_with_selection(snapshot, None)
}

/// 🖼️ Paints every page and outlines `selected` when the host has one.
pub fn render_window_with_selection(snapshot: &PdfSnapshot, selected: Option<&str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_ui_contract::BuiltNode> {
    semio_framework_plugin::scene_surface(BODY_KEY, semio_framework_ui_contract::SurfaceKind::Canvas2d, &Canvas2dScene::base(0.0, 0.0, 1.0, canvas_layers(snapshot, selected)))
}

/// 🖼️ Paints the page window from the framework-owned object selection.
pub fn render_selected(snapshot: &PdfSnapshot, interaction: &semio_framework_plugin::app::InteractionView<'_>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_ui_contract::BuiltNode> {
    render_window_with_selection(snapshot, interaction.selection(OBJECT_DOMAIN).ids.first().map(String::as_str))
}

/// 🧾 Reads a page-edit action into a replayable payload.
pub fn edit_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<PdfPageEdit, Fault> {
    if !is_page_action(action) {
        return Err(fault(format!("unknown pdf page action '{action}'")));
    }
    let page_index = || req_index(args, "page");
    let object_id = || req_text(args, "object");
    let text_value = || req_text(args, "text");
    if action == "move-page" {
        return Ok(payload_edit(action, page_index()?, String::new(), String::new(), String::new(), req_num(args, "x")?, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0));
    }
    let (page, object, text, extra, x, y, width, height, red, green, blue) = match action {
        "set-text" => (page_index()?, object_id()?, text_value()?, String::new(), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "move" => (page_index()?, object_id()?, String::new(), String::new(), req_num(args, "x")?, req_num(args, "y")?, 0.0, 0.0, 0.0, 0.0, 0.0),
        "delete" => (page_index()?, object_id()?, String::new(), String::new(), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "remove-page" => (page_index()?, String::new(), String::new(), String::new(), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "resize" => (page_index()?, object_id()?, String::new(), String::new(), req_num(args, "x")?, req_num(args, "y")?, req_num(args, "width")?, req_num(args, "height")?, 0.0, 0.0, 0.0),
        "insert-rectangle" | "insert-line" => (page_index()?, String::new(), String::new(), String::new(), req_num(args, "x")?, req_num(args, "y")?, req_num(args, "width")?, req_num(args, "height")?, opt_num(args, "red", 0.0), opt_num(args, "green", 0.0), opt_num(args, "blue", 0.0)),
        "set-fill" => (page_index()?, object_id()?, String::new(), String::new(), 0.0, 0.0, 0.0, 0.0, req_num(args, "red")?, req_num(args, "green")?, req_num(args, "blue")?),
        "set-stroke" => (page_index()?, object_id()?, String::new(), String::new(), 0.0, 0.0, req_num(args, "width")?, 0.0, req_num(args, "red")?, req_num(args, "green")?, req_num(args, "blue")?),
        "insert-text" => (page_index()?, String::new(), text_value()?, String::new(), req_num(args, "x")?, req_num(args, "y")?, 0.0, req_num(args, "height")?, opt_num(args, "red", 0.0), opt_num(args, "green", 0.0), opt_num(args, "blue", 0.0)),
        "insert-image" | "insert-page" | "set-page-size" => (page_index()?, String::new(), String::new(), String::new(), opt_num(args, "x", 0.0), opt_num(args, "y", 0.0), req_num(args, "width")?, req_num(args, "height")?, 0.0, 0.0, 0.0),
        "set-info" => (0, text_arg(args, "object"), text_arg(args, "text"), text_arg(args, "extra"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-annotation" => (page_index()?, object_id()?, text_value()?, String::new(), req_num(args, "x")?, req_num(args, "y")?, req_num(args, "width")?, req_num(args, "height")?, 0.0, 0.0, 0.0),
        "set-image" => (page_index()?, object_id()?, text_value()?, String::new(), 0.0, 0.0, req_num(args, "width")?, req_num(args, "height")?, 0.0, 0.0, 0.0),
        "set-font" => (page_index()?, object_id()?, text_value()?, String::new(), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-outline" => (page_index()?, String::new(), text_value()?, String::new(), opt_num(args, "x", -1.0), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-page-rotation" | "set-page-user-unit" => (page_index()?, String::new(), String::new(), String::new(), req_num(args, "x")?, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-page-box" => (page_index()?, text_value()?, String::new(), String::new(), req_num(args, "x")?, req_num(args, "y")?, req_num(args, "width")?, req_num(args, "height")?, 0.0, 0.0, 0.0),
        "set-language" | "set-page-layout" | "set-page-mode" | "set-metadata" => (0, String::new(), text_arg(args, "text"), String::new(), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-optional-content" => (0, object_id()?, text_value()?, String::new(), opt_num(args, "x", 1.0), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-embedded-file" => (0, object_id()?, text_arg(args, "text"), text_arg(args, "extra"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "remove-embedded-file" | "remove-named-destination" => (0, object_id()?, String::new(), String::new(), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-named-destination" => (page_index()?, object_id()?, String::new(), String::new(), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-page-label" => (page_index()?, String::new(), text_arg(args, "text"), text_arg(args, "extra"), opt_num(args, "x", 1.0), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-mark-info" => (0, String::new(), String::new(), String::new(), opt_num(args, "x", 0.0), opt_num(args, "y", 0.0), opt_num(args, "width", 0.0), 0.0, 0.0, 0.0, 0.0),
        "set-viewer-preferences" => (0, object_id()?, text_arg(args, "text"), String::new(), req_num(args, "x")?, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-info-field" => (0, text_arg(args, "object"), text_arg(args, "text"), String::new(), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-page-extra" => (page_index()?, text_arg(args, "object"), text_arg(args, "text"), text_arg(args, "extra"), opt_num(args, "x", 0.0), opt_num(args, "y", 0.0), 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-annotation-style" => (page_index()?, object_id()?, text_value()?, text_arg(args, "extra"), opt_num(args, "x", 0.0), 0.0, 0.0, 0.0, opt_num(args, "red", 0.0), opt_num(args, "green", 0.0), opt_num(args, "blue", 0.0)),
        "set-encryption" => (0, text_arg(args, "object"), text_arg(args, "text"), text_arg(args, "extra"), opt_num(args, "x", -1.0), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-output-intent" | "set-form-field" | "set-document-id" => (0, text_arg(args, "object"), text_arg(args, "text"), text_arg(args, "extra"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-open-action" => (opt_index(args, "page"), text_arg(args, "object"), text_arg(args, "text"), String::new(), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-font-program" => (0, object_id()?, text_value()?, text_arg(args, "extra"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-graphics-state" => (0, object_id()?, text_arg(args, "text"), String::new(), req_num(args, "x")?, req_num(args, "y")?, 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-pattern" => (0, object_id()?, text_arg(args, "text"), String::new(), opt_num(args, "x", 0.0), opt_num(args, "y", 0.0), opt_num(args, "width", 8.0), opt_num(args, "height", 8.0), 0.0, 0.0, 0.0),
        "set-color-space" => (0, object_id()?, text_value()?, text_arg(args, "extra"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-properties" => (0, object_id()?, text_value()?, text_arg(args, "extra"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-font-metrics" => (0, object_id()?, text_arg(args, "text"), text_arg(args, "extra"), opt_num(args, "x", -1.0), opt_num(args, "y", 0.0), 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-image-mask" | "set-catalog-entry" | "set-trailer-entry" => (0, object_id()?, text_arg(args, "text"), text_arg(args, "extra"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-form-content" => (0, object_id()?, text_arg(args, "text"), String::new(), opt_num(args, "x", 0.0), opt_num(args, "y", 0.0), opt_num(args, "width", 8.0), opt_num(args, "height", 8.0), 0.0, 0.0, 0.0),
        "set-page-transition" => (page_index()?, String::new(), text_arg(args, "text"), text_arg(args, "extra"), opt_num(args, "x", 0.0), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-annotation-appearance" => (page_index()?, object_id()?, text_arg(args, "text"), String::new(), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-glyph" => (0, object_id()?, text_value()?, String::new(), opt_num(args, "x", 0.0), opt_num(args, "y", 0.0), opt_num(args, "width", 10.0), opt_num(args, "height", 10.0), 0.0, 0.0, 0.0),
        "set-indirect-object" => (0, String::new(), text_arg(args, "text"), String::new(), req_num(args, "x")?, opt_num(args, "y", 0.0), 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-mesh-data" => (0, object_id()?, text_arg(args, "text"), text_arg(args, "extra"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "canvasPointerDown" | "canvasPointerMove" | "canvasPointerUp" => (0, String::new(), String::new(), if pointer_extend(args) { "shift".into() } else { String::new() }, opt_num(args, "x", 0.0), opt_num(args, "y", 0.0), 0.0, 0.0, 0.0, 0.0, 0.0),
        _ => return Err(fault(format!("unknown pdf page action '{action}'"))),
    };
    Ok(payload_edit(action, page, object, text, extra, x, y, width, height, red, green, blue))
}

/// 🧬 Applies a replayable page edit to the typed document.
pub fn apply_payload(snapshot: &PdfSnapshot, action: &str, payload: &str) -> Result<Vec<PdfMutation>, Fault> {
    let value: Value = serde_json::from_str(payload).map_err(|error| fault(error.to_string()))?;
    let page = value.get("page").and_then(Value::as_u64).unwrap_or(0) as usize;
    let object = value.get("object").and_then(Value::as_str).unwrap_or("").to_string();
    let text = value.get("text").and_then(Value::as_str).unwrap_or("").to_string();
    let extra = value.get("extra").and_then(Value::as_str).unwrap_or("").to_string();
    let x = json_f64(&value, "x");
    let y = json_f64(&value, "y");
    let width = json_f64(&value, "width");
    let height = json_f64(&value, "height");
    let red = json_f64(&value, "red");
    let green = json_f64(&value, "green");
    let blue = json_f64(&value, "blue");
    match action {
        "set-info" => {
            let mut info = snapshot.info.clone();
            if !text.is_empty() {
                info.title = Some(text);
            }
            if !extra.is_empty() {
                info.author = Some(extra);
            }
            if !object.is_empty() {
                info.subject = Some(object);
            }
            Ok(vec![PdfMutation::SetInfo(SetInfo { info })])
        }
        "insert-page" => Ok(vec![PdfMutation::InsertPage(InsertPage { index: page, page: PdfPage::new(width.max(1.0), height.max(1.0)) })]),
        "remove-page" => Ok(vec![PdfMutation::RemovePage(RemovePage { index: page })]),
        "move-page" => Ok(vec![PdfMutation::MovePage(MovePage { from: page, to: x.max(0.0) as usize })]),
        "set-page-size" => Ok(vec![PdfMutation::SetPageMediaBox(SetPageMediaBox { index: page, media_box: [0.0, 0.0, width.max(1.0), height.max(1.0)] })]),
        "insert-text" | "insert-rectangle" | "insert-line" | "insert-image" => insert_object(snapshot, action, page, x, y, width, height, red, green, blue, &text),
        "set-image" => replace_image_samples(snapshot, &object, width, height, &text),
        "set-font" => set_font_of_text(snapshot, &object, &text),
        "set-outline" => set_outline(snapshot, page, x, &text),
        "set-page-rotation" => set_page_rotation(snapshot, page, x),
        "set-page-box" => set_page_box(snapshot, page, &object, x, y, width, height),
        "set-page-user-unit" => set_page_user_unit(snapshot, page, x),
        "set-language" => Ok(vec![PdfMutation::SetLanguage(SetLanguage { language: none_if_empty(&text) })]),
        "set-page-layout" => Ok(vec![PdfMutation::SetPageLayout(SetPageLayout { layout: page_layout(&text)? })]),
        "set-page-mode" => Ok(vec![PdfMutation::SetPageMode(SetPageMode { mode: page_mode(&text)? })]),
        "set-optional-content" => set_optional_content(snapshot, &object, &text, x),
        "set-embedded-file" => set_embedded_file(snapshot, &object, &text, &extra),
        "remove-embedded-file" => remove_named(&object, "an embedded file needs an id").map(|id| vec![PdfMutation::RemoveEmbeddedFile(RemoveEmbeddedFile { id })]),
        "set-named-destination" => set_named_destination(&object, page),
        "remove-named-destination" => remove_named(&object, "a named destination needs a name").map(|name| vec![PdfMutation::RemoveNamedDestination(RemoveNamedDestination { name })]),
        "set-page-label" => set_page_label(snapshot, page, &text, &extra, x),
        "set-mark-info" => Ok(set_mark_info(x, y, width)),
        "set-metadata" => Ok(vec![PdfMutation::SetMetadata(SetMetadata { xmp: none_if_empty(&text) })]),
        "set-viewer-preferences" => set_viewer_preference(snapshot, &object, &text, x),
        "set-info-field" => set_info_field(snapshot, &object, &text),
        "set-page-extra" => set_page_extra(snapshot, page, &object, &text, &extra, x, y),
        "set-annotation-style" => set_annotation_style(snapshot, page, &object, &text, &extra, red, green, blue, x),
        "set-encryption" => set_encryption(&text, &extra, &object, x),
        "set-output-intent" => set_output_intent(snapshot, &object, &text, &extra),
        "set-form-field" => set_form_field(snapshot, &object, &text, &extra),
        "set-open-action" => set_open_action(&object, &text, page),
        "set-document-id" => Ok(set_document_id(&text, &extra)),
        "set-font-program" => set_font_program(snapshot, &object, &text, &extra),
        "set-graphics-state" => set_graphics_state(snapshot, &object, &text, x, y),
        "set-pattern" => set_pattern(snapshot, &object, &text, x, y, width, height),
        "set-color-space" => set_color_space(snapshot, &object, &text, &extra),
        "set-properties" => set_properties(snapshot, &object, &text, &extra),
        "set-font-metrics" => set_font_metrics(snapshot, &object, &text, &extra, x, y),
        "set-image-mask" => set_image_mask(snapshot, &object, &text, &extra),
        "set-form-content" => set_form_content(snapshot, &object, &text, x, y, width, height),
        "set-page-transition" => set_page_transition(snapshot, page, &text, &extra, x),
        "set-catalog-entry" => set_named_object(&object, &text, "a catalog entry needs a key", |key| PdfMutation::RemoveCatalogEntry(RemoveCatalogEntry { key }), |key, value| PdfMutation::SetCatalogEntry(SetCatalogEntry { key, value })),
        "set-trailer-entry" => set_named_object(&object, &text, "a trailer entry needs a key", |key| PdfMutation::RemoveTrailerEntry(RemoveTrailerEntry { key }), |key, value| PdfMutation::SetTrailerEntry(SetTrailerEntry { key, value })),
        "set-annotation-appearance" => set_annotation_appearance(snapshot, page, &object, &text),
        "set-glyph" => set_glyph(snapshot, &object, &text, x, y, width, height),
        "set-indirect-object" => set_indirect_object(snapshot, x, y, &text),
        "set-mesh-data" => set_mesh_data(snapshot, &object, &text, &extra),
        "set-annotation" => edit_annotation(snapshot, page, &object, &text, x, y, width, height),
        "canvasPointerDown" | "canvasPointerMove" | "canvasPointerUp" => Ok(Vec::new()),
        _ => edit_object(snapshot, action, &object, x, y, width, height, red, green, blue, &text),
    }
}

/// 🖱️ A page edit, or a canvas hit that redispatches selection and leaves the PDF unchanged.
pub fn emit_page_edit(snapshot: &PdfSnapshot, action: &str, payload: &str) -> Result<semio_framework_plugin::Emit<PdfMutation>, Fault> {
    match action {
        "canvasPointerMove" | "canvasPointerUp" => Ok(semio_framework_plugin::Emit::default()),
        "canvasPointerDown" => {
            let effect = pointer_down_effect(snapshot, payload)?;
            Ok(semio_framework_plugin::Emit { description: Some("Select".into()), ..semio_framework_plugin::Emit::effect(effect) })
        }
        _ => {
            let mutations = apply_payload(snapshot, action, payload)?;
            Ok(semio_framework_plugin::Emit { artifact_mutations: mutations, description: Some(action.to_string()), ..Default::default() })
        }
    }
}

/// 🎯 Declares the page-object selection domain and points the page window at it.
pub fn install_object_interaction(definition: &mut semio_framework_plugin::AppDefinition) {
    definition.interactions.push(semio_framework_plugin::InteractionDefinition {
        id: OBJECT_DOMAIN.into(),
        label: LocalizedLabel::native("Objects", "Objekte"),
        granularities: vec![semio_framework_plugin::GranularityDefinition { id: "object".into(), label: LocalizedLabel::native("Object", "Objekt"), icon_id: "square".into() }],
        hierarchy: semio_framework_plugin::HierarchyProvider::Flat,
        hover: semio_framework_plugin::HoverSpec::default(),
        selection: semio_framework_plugin::SelectionSpec {
            modes: vec![semio_framework_plugin::SelectionMode::Multiple, semio_framework_plugin::SelectionMode::Single],
            methods: vec![semio_framework_plugin::SelectionMethod::Pick],
            merges: vec![semio_framework_plugin::MergeMode::Replace, semio_framework_plugin::MergeMode::Invertive],
            transitive: false,
            broadcast: true,
        },
    });
    for window in definition.window_kinds.iter_mut() {
        if window.id == WINDOW_KIND_ID {
            window.interactions = vec![semio_framework_plugin::InteractionRef::new(OBJECT_DOMAIN)];
        }
    }
}

fn pointer_down_effect(snapshot: &PdfSnapshot, payload: &str) -> Result<semio_framework_plugin::Effect, Fault> {
    let value: Value = serde_json::from_str(payload).map_err(|error| fault(error.to_string()))?;
    let extend = value.get("extra").and_then(Value::as_str) == Some("shift") || value.get("shift").and_then(Value::as_bool).unwrap_or(false) || json_f64(&value, "shift") >= 0.5;
    let merge = if extend { "invertive" } else { "replace" };
    Ok(match hit_test(snapshot, json_f64(&value, "x"), json_f64(&value, "y")) {
        Some(id) => select_effect(&id, merge),
        None => semio_framework_plugin::Effect::DispatchAction { req: semio_framework_plugin::RequestId(113), action: semio_framework_plugin::CLEAR_SELECTION_ACTION_ID.into(), args: None, delay_ms: 0 },
    })
}

fn select_effect(id: &str, merge: &str) -> semio_framework_plugin::Effect {
    semio_framework_plugin::Effect::DispatchAction {
        req: semio_framework_plugin::RequestId(115),
        action: semio_framework_plugin::INTERACTION_SELECT_ACTION_ID.into(),
        args: Some(dsl::DslValue::from(json!({ "domainId": OBJECT_DOMAIN, "targets": json!([{ "granularity": "object", "id": id }]).to_string(), "merge": merge, "method": "pick" }))),
        delay_ms: 0,
    }
}

fn pointer_extend(args: Option<&dsl::DslValue>) -> bool {
    let Some(dsl::DslValue::Object(entries)) = args else { return false };
    entries.iter().any(|(key, value)| {
        (key == "shift" || key == "ctrl" || key == "meta") && match value {
            dsl::DslValue::Bool(flag) => *flag,
            dsl::DslValue::Number(number) => number.as_f64() >= 0.5,
            dsl::DslValue::String(text) => text == "true" || text == "1",
            _ => false,
        }
    })
}

/// 👁 Every addressable object in painting order.
pub fn objects(snapshot: &PdfSnapshot) -> Vec<PageObject> {
    let mut found = Vec::new();
    for (page_index, page) in snapshot.pages.iter().enumerate() {
        found.extend(page_objects(snapshot, page_index, page));
    }
    found
}

/// 🖼️ Host canvas layers for the whole document.
pub fn canvas_layers(snapshot: &PdfSnapshot, selected: Option<&str>) -> String {
    let placed = placement(snapshot);
    let mut layers = Vec::new();
    let mut top = 0.0;
    for (page_index, page) in snapshot.pages.iter().enumerate() {
        let (width, height) = displayed_size(page);
        layers.push(json!({ "id": format!("p{page_index}:paper"), "segments": rect_segments(0.0, top, width, height), "fill": { "color": [1.0, 1.0, 1.0, 1.0] }, "stroke": { "color": [0.75, 0.75, 0.78, 1.0], "width": 1.0 } }));
        top += height + 16.0;
    }
    for item in &placed {
        let color = [item.object.fill[0] as f32, item.object.fill[1] as f32, item.object.fill[2] as f32, 1.0];
        let stroke = [item.object.stroke[0] as f32, item.object.stroke[1] as f32, item.object.stroke[2] as f32, 1.0];
        match item.object.kind {
            ObjectKind::Text => {
                layers.push(json!({
                    "id": item.object.id,
                    "kind": "text",
                    "x": item.canvas_x,
                    "y": item.canvas_y,
                    "width": item.canvas_w.max(1.0),
                    "height": item.canvas_h.max(1.0),
                    "text": { "content": item.object.text, "size": item.object.font_size.max(1.0) },
                    "fill": { "color": color },
                }));
            }
            ObjectKind::Vector => {
                let page = &snapshot.pages[item.object.page];
                let segments = vector_segments(page, item.top, &page.content[item.object.start..=item.object.end.min(page.content.len().saturating_sub(1))]);
                layers.push(json!({ "id": item.object.id, "segments": segments, "fill": { "color": color }, "stroke": { "color": stroke, "width": item.object.line_width.max(0.5) } }));
            }
            ObjectKind::Image | ObjectKind::Form | ObjectKind::Shading | ObjectKind::Annotation => {
                layers.push(json!({ "id": item.object.id, "segments": rect_segments(item.canvas_x, item.canvas_y, item.canvas_w.max(1.0), item.canvas_h.max(1.0)), "fill": { "color": if item.object.kind == ObjectKind::Image { [0.85, 0.88, 0.92, 1.0] } else { color } }, "stroke": { "color": stroke, "width": 1.0 } }));
                if !item.object.text.is_empty() {
                    layers.push(json!({ "id": format!("{}.label", item.object.id), "kind": "text", "x": item.canvas_x + 4.0, "y": item.canvas_y + 4.0, "width": item.canvas_w.max(1.0), "height": 12.0, "text": { "content": item.object.text, "size": 12.0 }, "fill": { "color": [0.1, 0.1, 0.12, 1.0] } }));
                }
            }
        }
        if selected == Some(item.object.id.as_str()) {
            layers.push(json!({ "id": format!("{}.selection", item.object.id), "segments": rect_segments(item.canvas_x, item.canvas_y, item.canvas_w.max(1.0), item.canvas_h.max(1.0)), "stroke": { "color": [0.1, 0.45, 0.95, 1.0], "width": 2.0 } }));
        }
    }
    serde_json::to_string(&layers).unwrap_or_else(|_| "[]".into())
}

/// 🎯 Top-most object whose displayed bounds contain the canvas point.
pub fn hit_test(snapshot: &PdfSnapshot, x: f64, y: f64) -> Option<String> {
    placement(snapshot).into_iter().rev().find(|item| x >= item.canvas_x && y >= item.canvas_y && x <= item.canvas_x + item.canvas_w && y <= item.canvas_y + item.canvas_h).map(|item| item.object.id)
}

fn payload_edit(action: &str, page: u32, object: String, text: String, extra: String, x: f64, y: f64, width: f64, height: f64, red: f64, green: f64, blue: f64) -> PdfPageEdit {
    PdfPageEdit { action: action.to_string(), payload: json!({ "page": page, "object": object, "text": text, "extra": extra, "x": x, "y": y, "width": width, "height": height, "red": red, "green": green, "blue": blue }).to_string() }
}

fn edit_object(snapshot: &PdfSnapshot, action: &str, object_id: &str, x: f64, y: f64, width: f64, height: f64, red: f64, green: f64, blue: f64, text: &str) -> Result<Vec<PdfMutation>, Fault> {
    let object = objects(snapshot).into_iter().find(|item| item.id == object_id).ok_or_else(|| fault(format!("pdf object '{object_id}' is not on the page")))?;
    if object.kind == ObjectKind::Annotation {
        return edit_annotation(snapshot, object.page, object_id, text, x, y, width, height);
    }
    if object.kind == ObjectKind::Shading && (action == "move" || action == "resize") {
        return edit_shading(snapshot, &object, action, x, y, width, height);
    }
    let page = snapshot.pages.get(object.page).ok_or_else(|| fault("pdf page is gone"))?;
    let mut content = page.content.clone();
    match action {
        "set-text" => replace_shown_text(&mut content, object.end, text),
        "delete" => {
            let end = object.end.min(content.len().saturating_sub(1));
            if object.start <= end && end < content.len() {
                content.drain(object.start..=end);
            }
        }
        "move" => translate_object(&mut content, &object, x - object.x, y - object.y),
        "resize" => resize_object(&mut content, &object, x, y, width, height),
        "set-fill" => set_fill(&mut content, &object, red, green, blue),
        "set-stroke" => set_stroke(&mut content, &object, red, green, blue, width),
        other => return Err(fault(format!("unknown pdf page action '{other}'"))),
    }
    Ok(vec![PdfMutation::SetPageContent(SetPageContent { index: object.page, content })])
}

fn insert_object(snapshot: &PdfSnapshot, action: &str, page_index: usize, x: f64, y: f64, width: f64, height: f64, red: f64, green: f64, blue: f64, text: &str) -> Result<Vec<PdfMutation>, Fault> {
    let page = snapshot.pages.get(page_index).ok_or_else(|| fault(format!("pdf page {page_index} does not exist")))?;
    let mut content = page.content.clone();
    let mut extra = Vec::new();
    match action {
        "insert-text" => {
            let (font_id, font_mutation) = ensure_font(snapshot);
            if let Some(mutation) = font_mutation {
                extra.push(mutation);
            }
            let size = if height > 0.0 { height } else { 12.0 };
            content.extend([
                PdfOp::BeginText,
                PdfOp::SetFont { name: font_id, size },
                PdfOp::SetFillRgb { r: red, g: green, b: blue },
                PdfOp::SetTextMatrix { matrix: [1.0, 0.0, 0.0, 1.0, x, y] },
                PdfOp::ShowText { text: PdfTextString::text(text) },
                PdfOp::EndText,
            ]);
        }
        "insert-rectangle" => content.extend([PdfOp::SetFillRgb { r: red, g: green, b: blue }, PdfOp::Rectangle { x, y, width, height }, PdfOp::Fill]),
        "insert-line" => content.extend([
            PdfOp::SetStrokeRgb { r: red, g: green, b: blue },
            PdfOp::SetLineWidth { width: 1.0 },
            PdfOp::MoveTo { x, y },
            PdfOp::LineTo { x: x + width, y: y + height },
            PdfOp::Stroke,
        ]),
        "insert-image" => {
            let id = format!("Im{}", snapshot.images.len() + 1);
            let image = PdfImage::rgb8(&id, 8, 8, vec![180; 8 * 8 * 3]);
            extra.push(PdfMutation::SetImage(SetImage { image }));
            content.extend([PdfOp::Save, PdfOp::Transform { matrix: [width.max(1.0), 0.0, 0.0, height.max(1.0), x, y] }, PdfOp::PaintXObject { name: id }, PdfOp::Restore]);
        }
        other => return Err(fault(format!("unknown pdf page action '{other}'"))),
    }
    extra.push(PdfMutation::SetPageContent(SetPageContent { index: page_index, content }));
    Ok(extra)
}

fn edit_annotation(snapshot: &PdfSnapshot, page_index: usize, object_id: &str, text: &str, x: f64, y: f64, width: f64, height: f64) -> Result<Vec<PdfMutation>, Fault> {
    let index = object_id.rsplit_once(":a").and_then(|(_, raw)| raw.parse::<usize>().ok()).ok_or_else(|| fault(format!("pdf annotation '{object_id}' is not addressable")))?;
    let annotation = snapshot.pages.get(page_index).and_then(|page| page.annotations.get(index)).cloned().ok_or_else(|| fault(format!("pdf annotation '{object_id}' is gone")))?;
    let mut annotation: PdfAnnotation = annotation;
    annotation.contents = Some(text.to_string());
    if width > 0.0 && height > 0.0 {
        annotation.rect = [x, y, x + width, y + height];
    }
    Ok(vec![PdfMutation::SetAnnotation(SetAnnotation { index: page_index, at: index, annotation })])
}

fn replace_image_samples(snapshot: &PdfSnapshot, object_id: &str, width: f64, height: f64, hex: &str) -> Result<Vec<PdfMutation>, Fault> {
    let object = objects(snapshot).into_iter().find(|item| item.id == object_id).ok_or_else(|| fault(format!("pdf object '{object_id}' is not on the page")))?;
    if object.kind != ObjectKind::Image {
        return Err(fault(format!("pdf object '{object_id}' is not an image")));
    }
    let pixel_width = width.max(1.0).round() as u32;
    let pixel_height = height.max(1.0).round() as u32;
    let data = decode_hex(hex)?;
    let color_space = image_color_space(pixel_width, pixel_height, data.len())?;
    if let Some(image) = snapshot.images.iter().find(|image| image.id == object.text) {
        let mut image = image.clone();
        image.width = pixel_width;
        image.height = pixel_height;
        image.data = data;
        image.codec = PdfImageCodec::Raw;
        image.bits_per_component = 8;
        image.color_space = Some(color_space);
        return Ok(vec![PdfMutation::SetImage(SetImage { image })]);
    }
    let page = snapshot.pages.get(object.page).ok_or_else(|| fault("pdf page is gone"))?;
    let mut content = page.content.clone();
    let Some(PdfOp::InlineImage { image }) = content.get_mut(object.end) else { return Err(fault(format!("pdf image '{object_id}' is not a stored image"))) };
    image.width = pixel_width;
    image.height = pixel_height;
    image.data = data;
    image.bits_per_component = 8;
    image.color_space = Some(color_space);
    Ok(vec![PdfMutation::SetPageContent(SetPageContent { index: object.page, content })])
}

fn set_font_of_text(snapshot: &PdfSnapshot, object_id: &str, base_font: &str) -> Result<Vec<PdfMutation>, Fault> {
    if base_font.is_empty() {
        return Err(fault("a text run needs a font name"));
    }
    let object = objects(snapshot).into_iter().find(|item| item.id == object_id).ok_or_else(|| fault(format!("pdf object '{object_id}' is not on the page")))?;
    if object.kind != ObjectKind::Text {
        return Err(fault(format!("pdf object '{object_id}' is not text")));
    }
    let (font_id, font_mutation) = font_for_base(snapshot, base_font);
    let page = snapshot.pages.get(object.page).ok_or_else(|| fault("pdf page is gone"))?;
    let mut content = page.content.clone();
    content.insert(object.end, PdfOp::SetFont { name: font_id, size: object.font_size.max(1.0) });
    let mut mutations = Vec::new();
    if let Some(mutation) = font_mutation {
        mutations.push(mutation);
    }
    mutations.push(PdfMutation::SetPageContent(SetPageContent { index: object.page, content }));
    Ok(mutations)
}

fn font_for_base(snapshot: &PdfSnapshot, base_font: &str) -> (String, Option<PdfMutation>) {
    if let Some(font) = snapshot.fonts.iter().find(|font| font.base_font() == base_font) {
        return (font.id.clone(), None);
    }
    let id = format!("F{}", snapshot.fonts.len() + 1);
    (id.clone(), Some(PdfMutation::SetFont(SetFont { font: PdfFont::standard(id, base_font) })))
}

fn set_outline(snapshot: &PdfSnapshot, page: usize, index: f64, title: &str) -> Result<Vec<PdfMutation>, Fault> {
    if title.is_empty() {
        return Err(fault("an outline needs a title"));
    }
    let mut outlines = snapshot.outlines.clone();
    let item_index = if index < 0.0 { outlines.len() } else { index.round() as usize };
    if item_index < outlines.len() {
        outlines[item_index].title = title.to_string();
        outlines[item_index].destination = PdfOutlineItem::to_page(title, page as u32).destination;
    } else {
        outlines.push(PdfOutlineItem::to_page(title, page as u32));
    }
    Ok(vec![PdfMutation::SetOutlines(SetOutlines { outlines })])
}

fn set_page_rotation(snapshot: &PdfSnapshot, page: usize, degrees: f64) -> Result<Vec<PdfMutation>, Fault> {
    require_page(snapshot, page)?;
    let rotation = degrees.round() as i32;
    if !matches!(rotation, 0 | 90 | 180 | 270) {
        return Err(fault("page rotation must be 0, 90, 180, or 270"));
    }
    Ok(vec![PdfMutation::SetPageRotation(SetPageRotation { index: page, rotation: rotation as u16 })])
}

fn set_page_box(snapshot: &PdfSnapshot, page: usize, kind: &str, x: f64, y: f64, width: f64, height: f64) -> Result<Vec<PdfMutation>, Fault> {
    require_page(snapshot, page)?;
    let kind = match kind {
        "crop" => PdfPageBox::Crop,
        "bleed" => PdfPageBox::Bleed,
        "trim" => PdfPageBox::Trim,
        "art" => PdfPageBox::Art,
        _ => return Err(fault("page box must be crop, bleed, trim, or art")),
    };
    let rect = if width == 0.0 && height == 0.0 {
        None
    } else if width > 0.0 && height > 0.0 {
        Some([x, y, x + width, y + height])
    } else {
        return Err(fault("a page box needs a positive width and height"));
    };
    Ok(vec![PdfMutation::SetPageBox(SetPageBox { index: page, kind, rect })])
}

fn set_page_user_unit(snapshot: &PdfSnapshot, page: usize, unit: f64) -> Result<Vec<PdfMutation>, Fault> {
    require_page(snapshot, page)?;
    let user_unit = if unit <= 0.0 { None } else { Some(unit) };
    Ok(vec![PdfMutation::SetPageUserUnit(SetPageUserUnit { index: page, user_unit })])
}

fn page_layout(name: &str) -> Result<Option<PdfPageLayout>, Fault> {
    Ok(Some(match name {
        "" => return Ok(None),
        "singlePage" => PdfPageLayout::SinglePage,
        "oneColumn" => PdfPageLayout::OneColumn,
        "twoColumnLeft" => PdfPageLayout::TwoColumnLeft,
        "twoColumnRight" => PdfPageLayout::TwoColumnRight,
        "twoPageLeft" => PdfPageLayout::TwoPageLeft,
        "twoPageRight" => PdfPageLayout::TwoPageRight,
        other => return Err(fault(format!("unknown page layout '{other}'"))),
    }))
}

fn page_mode(name: &str) -> Result<Option<PdfPageMode>, Fault> {
    Ok(Some(match name {
        "" => return Ok(None),
        "useNone" => PdfPageMode::UseNone,
        "useOutlines" => PdfPageMode::UseOutlines,
        "useThumbs" => PdfPageMode::UseThumbs,
        "fullScreen" => PdfPageMode::FullScreen,
        "useOc" => PdfPageMode::UseOc,
        "useAttachments" => PdfPageMode::UseAttachments,
        other => return Err(fault(format!("unknown page mode '{other}'"))),
    }))
}

fn require_page(snapshot: &PdfSnapshot, page: usize) -> Result<(), Fault> {
    if page >= snapshot.pages.len() {
        Err(fault("pdf page is gone"))
    } else {
        Ok(())
    }
}

fn none_if_empty(text: &str) -> Option<String> {
    if text.is_empty() { None } else { Some(text.to_string()) }
}

fn set_optional_content(snapshot: &PdfSnapshot, id: &str, name: &str, on: f64) -> Result<Vec<PdfMutation>, Fault> {
    if id.is_empty() || name.is_empty() {
        return Err(fault("an optional content group needs an id and a name"));
    }
    let mut content = snapshot.optional_content.clone().unwrap_or_default();
    if let Some(group) = content.groups.iter_mut().find(|group| group.id == id) {
        group.name = name.to_string();
    } else {
        content.groups.push(PdfOptionalContentGroup { id: id.to_string(), name: name.to_string(), intent: Vec::new(), usage: Vec::new() });
    }
    content.on.retain(|item| item != id);
    content.off.retain(|item| item != id);
    if on >= 0.5 { content.on.push(id.to_string()) } else { content.off.push(id.to_string()) }
    Ok(vec![PdfMutation::SetOptionalContent(SetOptionalContent { content: Some(content) })])
}

fn set_embedded_file(snapshot: &PdfSnapshot, id: &str, file_name: &str, contents: &str) -> Result<Vec<PdfMutation>, Fault> {
    if id.is_empty() {
        return Err(fault("an embedded file needs an id"));
    }
    if file_name.is_empty() {
        return Ok(vec![PdfMutation::RemoveEmbeddedFile(RemoveEmbeddedFile { id: id.to_string() })]);
    }
    let previous = snapshot.embedded_files.iter().find(|file| file.id == id);
    Ok(vec![PdfMutation::SetEmbeddedFile(SetEmbeddedFile { file: PdfEmbeddedFile { id: id.to_string(), file_name: file_name.to_string(), description: previous.and_then(|file| file.description.clone()), mime_type: previous.and_then(|file| file.mime_type.clone()), data: contents.as_bytes().to_vec(), creation_date: previous.and_then(|file| file.creation_date.clone()), modification_date: previous.and_then(|file| file.modification_date.clone()), relationship: previous.and_then(|file| file.relationship.clone()), listed: true } })])
}

fn set_named_destination(name: &str, page: usize) -> Result<Vec<PdfMutation>, Fault> {
    if name.is_empty() {
        return Err(fault("a named destination needs a name"));
    }
    Ok(vec![PdfMutation::SetNamedDestination(SetNamedDestination { destination: PdfNamedDestination { name: name.to_string(), destination: PdfDestination::Page { page: page as u32, fit: PdfDestinationFit::Fit } } })])
}

fn set_page_label(snapshot: &PdfSnapshot, page: usize, style: &str, prefix: &str, start: f64) -> Result<Vec<PdfMutation>, Fault> {
    let style = match style {
        "" => None,
        "decimal" => Some(PdfPageLabelStyle::Decimal),
        "romanUpper" => Some(PdfPageLabelStyle::RomanUpper),
        "romanLower" => Some(PdfPageLabelStyle::RomanLower),
        "lettersUpper" => Some(PdfPageLabelStyle::LettersUpper),
        "lettersLower" => Some(PdfPageLabelStyle::LettersLower),
        other => return Err(fault(format!("unknown page label style '{other}'"))),
    };
    let range = PdfPageLabelRange { start_index: page as u32, style, prefix: none_if_empty(prefix), start: start.max(1.0).round() as u32 };
    let mut labels = snapshot.page_labels.clone();
    if let Some(existing) = labels.iter_mut().find(|item| item.start_index == range.start_index) {
        *existing = range;
    } else {
        labels.push(range);
    }
    Ok(vec![PdfMutation::SetPageLabels(SetPageLabels { labels })])
}

fn set_mark_info(marked: f64, user_properties: f64, suspects: f64) -> Vec<PdfMutation> {
    let info = PdfMarkInfo { marked: marked >= 0.5, user_properties: user_properties >= 0.5, suspects: suspects >= 0.5 };
    vec![PdfMutation::SetMarkInfo(SetMarkInfo { info: (info != PdfMarkInfo::default()).then_some(info) })]
}

fn set_info_field(snapshot: &PdfSnapshot, field: &str, value: &str) -> Result<Vec<PdfMutation>, Fault> {
    let mut info = snapshot.info.clone();
    match field {
        "keywords" => info.keywords = none_if_empty(value),
        "creator" => info.creator = none_if_empty(value),
        "producer" => info.producer = none_if_empty(value),
        "trapped" => info.trapped = none_if_empty(value),
        "creationDate" => info.creation_date = parse_optional_date(value)?,
        "modificationDate" => info.modification_date = parse_optional_date(value)?,
        other => return Err(fault(format!("unknown info field '{other}'"))),
    }
    Ok(vec![PdfMutation::SetInfo(SetInfo { info })])
}

fn parse_optional_date(value: &str) -> Result<Option<PdfDate>, Fault> {
    if value.is_empty() {
        return Ok(None);
    }
    PdfDate::parse(value).map(Some).ok_or_else(|| fault("a document date needs D:YYYYMMDDHHmmSS"))
}

fn set_viewer_preference(snapshot: &PdfSnapshot, flag: &str, text: &str, value: f64) -> Result<Vec<PdfMutation>, Fault> {
    let mut preferences = snapshot.viewer_preferences.clone().unwrap_or_default();
    let on = value >= 0.5;
    match flag {
        "hideToolbar" => preferences.hide_toolbar = on,
        "hideMenubar" => preferences.hide_menubar = on,
        "hideWindowUi" => preferences.hide_window_ui = on,
        "fitWindow" => preferences.fit_window = on,
        "centerWindow" => preferences.center_window = on,
        "displayDocTitle" => preferences.display_doc_title = on,
        "pickTrayByPdfSize" => preferences.pick_tray_by_pdf_size = on,
        "numCopies" => preferences.num_copies = (value >= 0.0).then_some(value.round() as u32),
        "nonFullScreenPageMode" => preferences.non_full_screen_page_mode = page_mode(text)?,
        "printPageRange" => preferences.print_page_range = if text.is_empty() { Vec::new() } else { text.split(',').map(str::trim).filter(|item| !item.is_empty()).map(|item| item.parse::<u32>().map_err(|_| fault("a print range needs whole page numbers"))).collect::<Result<Vec<_>, _>>()? },
        "direction" => preferences.direction = none_if_empty(text),
        "viewArea" => preferences.view_area = none_if_empty(text),
        "viewClip" => preferences.view_clip = none_if_empty(text),
        "printArea" => preferences.print_area = none_if_empty(text),
        "printClip" => preferences.print_clip = none_if_empty(text),
        "printScaling" => preferences.print_scaling = none_if_empty(text),
        "duplex" => preferences.duplex = none_if_empty(text),
        other => return Err(fault(format!("unknown viewer preference '{other}'"))),
    }
    Ok(vec![PdfMutation::SetViewerPreferences(SetViewerPreferences { preferences: (preferences != PdfViewerPreferences::default()).then_some(preferences) })])
}

fn set_encryption(algorithm: &str, user_password: &str, owner_password: &str, permissions: f64) -> Result<Vec<PdfMutation>, Fault> {
    if algorithm.is_empty() {
        return Ok(vec![PdfMutation::SetEncryption(SetEncryption { encryption: None })]);
    }
    let algorithm = match algorithm {
        "rc4-40" => PdfEncryptionAlgorithm::Rc4_40,
        "rc4-128" => PdfEncryptionAlgorithm::Rc4_128,
        "aes128" => PdfEncryptionAlgorithm::Aes128,
        "aes256" => PdfEncryptionAlgorithm::Aes256,
        other => return Err(fault(format!("unknown encryption '{other}'"))),
    };
    Ok(vec![PdfMutation::SetEncryption(SetEncryption { encryption: Some(PdfEncryption { algorithm, permissions: permissions.round() as i32, user_password: user_password.to_string(), owner_password: none_if_empty(owner_password), encrypt_metadata: true }) })])
}

fn set_output_intent(snapshot: &PdfSnapshot, subtype: &str, identifier: &str, info: &str) -> Result<Vec<PdfMutation>, Fault> {
    if subtype.is_empty() {
        return Ok(vec![PdfMutation::SetOutputIntents(SetOutputIntents { intents: Vec::new() })]);
    }
    if identifier.is_empty() {
        return Err(fault("an output intent needs a condition identifier"));
    }
    let mut intents = snapshot.output_intents.clone();
    let profile = intents.iter().find(|item| item.subtype == subtype).and_then(|item| item.profile.clone());
    let intent = PdfOutputIntent { subtype: subtype.to_string(), condition_identifier: identifier.to_string(), condition: None, registry_name: None, info: none_if_empty(info), profile };
    if let Some(existing) = intents.iter_mut().find(|item| item.subtype == subtype) {
        *existing = intent;
    } else {
        intents.push(intent);
    }
    Ok(vec![PdfMutation::SetOutputIntents(SetOutputIntents { intents })])
}

fn set_form_field(snapshot: &PdfSnapshot, name: &str, value: &str, kind: &str) -> Result<Vec<PdfMutation>, Fault> {
    if name.is_empty() {
        return Ok(vec![PdfMutation::SetAcroForm(SetAcroForm { form: None })]);
    }
    let field_kind = match kind {
        "" | "text" => PdfFormFieldKind::Text { value: none_if_empty(value), default_value: None, max_length: None, rich_value: None },
        "button" => PdfFormFieldKind::Button { value: none_if_empty(value), default_value: None, options: Vec::new() },
        "choice" => PdfFormFieldKind::Choice { values: none_if_empty(value).into_iter().collect(), default_values: Vec::new(), options: Vec::new(), top_index: None },
        other => return Err(fault(format!("unknown form field kind '{other}'"))),
    };
    let mut form = snapshot.acro_form.clone().unwrap_or_default();
    form.need_appearances = true;
    if let Some(field) = form.fields.iter_mut().find(|field| field.name == name) {
        field.kind = field_kind;
    } else {
        form.fields.push(PdfFormField { name: name.to_string(), kind: field_kind, flags: 0, alternate_name: None, mapping_name: None, default_appearance: None, quadding: None, widgets: Vec::new(), children: Vec::new(), additional_actions: Vec::new(), extra: Vec::new() });
    }
    Ok(vec![PdfMutation::SetAcroForm(SetAcroForm { form: Some(form) })])
}

fn set_open_action(kind: &str, uri: &str, page: usize) -> Result<Vec<PdfMutation>, Fault> {
    let action = match kind {
        "" => None,
        "uri" if !uri.is_empty() => Some(PdfOpenAction::Action { action: PdfAction::uri(uri) }),
        "uri" => return Err(fault("an open uri needs an address")),
        "page" => Some(PdfOpenAction::Destination { destination: PdfDestination::Page { page: page as u32, fit: PdfDestinationFit::Fit } }),
        other => return Err(fault(format!("unknown open action '{other}'"))),
    };
    Ok(vec![PdfMutation::SetOpenAction(SetOpenAction { action })])
}

fn set_document_id(permanent: &str, changing: &str) -> Vec<PdfMutation> {
    let id = if permanent.is_empty() && changing.is_empty() {
        None
    } else {
        let first = if permanent.is_empty() { changing.as_bytes().to_vec() } else { permanent.as_bytes().to_vec() };
        let second = if changing.is_empty() { first.clone() } else { changing.as_bytes().to_vec() };
        Some([first, second])
    };
    vec![PdfMutation::SetDocumentId(SetDocumentId { id })]
}

fn set_font_program(snapshot: &PdfSnapshot, id: &str, kind: &str, hex: &str) -> Result<Vec<PdfMutation>, Fault> {
    let mut font = snapshot.fonts.iter().find(|font| font.id == id).cloned().ok_or_else(|| fault(format!("pdf font '{id}' is gone")))?;
    let data = decode_hex(hex)?;
    let program = font_program(kind, data)?;
    font.kind = place_font_program(font.kind, program)?;
    Ok(vec![PdfMutation::SetFont(SetFont { font })])
}

fn font_program(kind: &str, data: Vec<u8>) -> Result<PdfFontProgram, Fault> {
    let length1 = data.len() as u32;
    Ok(match kind {
        "type1" => PdfFontProgram::Type1 { data, length1, length2: 0, length3: 0 },
        "truetype" => PdfFontProgram::TrueType { data },
        "cff" => PdfFontProgram::Cff { data },
        "cidcff" => PdfFontProgram::CidCff { data },
        "opentype" => PdfFontProgram::OpenType { data },
        other => return Err(fault(format!("unknown font program '{other}'"))),
    })
}

fn place_font_program(kind: PdfFontKind, program: PdfFontProgram) -> Result<PdfFontKind, Fault> {
    let type1 = matches!(program, PdfFontProgram::Type1 { .. } | PdfFontProgram::Cff { .. });
    let outlined = matches!(program, PdfFontProgram::TrueType { .. } | PdfFontProgram::OpenType { .. });
    let cid = matches!(program, PdfFontProgram::CidCff { .. }) || outlined;
    match kind {
        PdfFontKind::Type1 { base_font, encoding, first_char, widths, descriptor, .. } if type1 => Ok(PdfFontKind::Type1 { base_font, encoding, first_char, widths, descriptor, program: Some(program) }),
        PdfFontKind::Type1 { base_font, encoding, first_char, widths, descriptor, .. } if outlined => Ok(PdfFontKind::TrueType { base_font, encoding, first_char, widths, descriptor, program: Some(program) }),
        PdfFontKind::TrueType { base_font, encoding, first_char, widths, descriptor, .. } if outlined => Ok(PdfFontKind::TrueType { base_font, encoding, first_char, widths, descriptor, program: Some(program) }),
        PdfFontKind::Type0 { base_font, cmap, mut descendant } if cid => {
            descendant.program = Some(program);
            Ok(PdfFontKind::Type0 { base_font, cmap, descendant })
        }
        _ => Err(fault("that font program does not fit this font")),
    }
}

fn set_image_mask(snapshot: &PdfSnapshot, address: &str, kind: &str, value: &str) -> Result<Vec<PdfMutation>, Fault> {
    let mut image = image_by_address(snapshot, address)?;
    match kind {
        "" => {
            image.mask = None;
            image.soft_mask = None;
        }
        "stencil" if !value.is_empty() => image.mask = Some(PdfImageMask::Stencil { image: value.to_string() }),
        "soft" if !value.is_empty() => image.soft_mask = Some(value.to_string()),
        "colorKey" => {
            let ranges = value.split(',').map(|item| item.trim().parse::<u32>().map_err(|_| fault("a color key is comma-separated integers"))).collect::<Result<Vec<_>, _>>()?;
            if ranges.is_empty() {
                return Err(fault("a color key needs ranges"));
            }
            image.mask = Some(PdfImageMask::ColorKey { ranges });
        }
        "stencil" | "soft" => return Err(fault("a mask needs an image id")),
        other => return Err(fault(format!("unknown image mask '{other}'"))),
    }
    Ok(vec![PdfMutation::SetImage(SetImage { image })])
}

fn image_by_address(snapshot: &PdfSnapshot, address: &str) -> Result<PdfImage, Fault> {
    if let Some(image) = snapshot.images.iter().find(|image| image.id == address) {
        return Ok(image.clone());
    }
    let object = objects(snapshot).into_iter().find(|item| item.id == address).ok_or_else(|| fault(format!("pdf image '{address}' is gone")))?;
    if object.kind != ObjectKind::Image {
        return Err(fault(format!("pdf object '{address}' is not an image")));
    }
    snapshot.images.iter().find(|image| image.id == object.text).cloned().ok_or_else(|| fault(format!("pdf image '{}' is not a stored image", object.text)))
}

fn set_form_content(snapshot: &PdfSnapshot, id: &str, text: &str, x: f64, y: f64, width: f64, height: f64) -> Result<Vec<PdfMutation>, Fault> {
    if id.is_empty() {
        return Err(fault("a form needs an id"));
    }
    let font = snapshot.fonts.first().map(|font| font.id.clone()).unwrap_or_else(|| "F1".into());
    let content = if text.is_empty() {
        vec![PdfOp::Rectangle { x, y, width: width.max(1.0), height: height.max(1.0) }, PdfOp::Fill]
    } else {
        vec![PdfOp::BeginText, PdfOp::SetFont { name: font, size: height.max(1.0) }, PdfOp::SetTextMatrix { matrix: [1.0, 0.0, 0.0, 1.0, x, y] }, PdfOp::ShowText { text: PdfTextString::text(text) }, PdfOp::EndText]
    };
    let form = match snapshot.forms.iter().find(|form| form.id == id) {
        Some(form) => {
            let mut form = form.clone();
            form.content = content;
            form
        }
        None => PdfFormXObject::new(id, [x, y, x + width.max(1.0), y + height.max(1.0)], content),
    };
    Ok(vec![PdfMutation::SetForm(SetForm { form })])
}

fn set_page_extra(snapshot: &PdfSnapshot, page: usize, kind: &str, text: &str, extra: &str, x: f64, y: f64) -> Result<Vec<PdfMutation>, Fault> {
    require_page(snapshot, page)?;
    let mut next = snapshot.clone();
    let target = &mut next.pages[page];
    match kind {
        "thumbnail" => target.thumbnail = none_if_empty(text),
        "metadata" => target.metadata = none_if_empty(text),
        "structParents" => target.struct_parents = (x >= 0.0).then_some(x.round() as u32),
        "group" if text == "clear" => target.group = None,
        "group" => {
            let color_space = match text {
                "" => None,
                "deviceGray" => Some(PdfColorSpace::DeviceGray),
                "deviceRgb" => Some(PdfColorSpace::DeviceRgb),
                "deviceCmyk" => Some(PdfColorSpace::DeviceCmyk),
                other => return Err(fault(format!("unknown group color space '{other}'"))),
            };
            target.group = Some(PdfTransparencyGroup { color_space, isolated: x >= 0.5, knockout: y >= 0.5 });
        }
        "action" | "entry" => {
            if extra.is_empty() {
                return Err(fault("a page entry needs a key"));
            }
            let entries = if kind == "action" { &mut target.additional_actions } else { &mut target.extra };
            if text.is_empty() {
                entries.retain(|item| item.key != extra);
            } else if let Some(existing) = entries.iter_mut().find(|item| item.key == extra) {
                *existing = PdfDictEntry::new(extra, PdfObject::Name(text.to_string()));
            } else {
                entries.push(PdfDictEntry::new(extra, PdfObject::Name(text.to_string())));
            }
        }
        other => return Err(fault(format!("unknown page extra '{other}'"))),
    }
    Ok(vec![PdfMutation::SetSnapshot(SetSnapshot { snapshot: next })])
}

fn set_annotation_style(snapshot: &PdfSnapshot, page: usize, object_id: &str, aspect: &str, value: &str, red: f64, green: f64, blue: f64, flags: f64) -> Result<Vec<PdfMutation>, Fault> {
    let index = object_id.rsplit_once(":a").and_then(|(_, raw)| raw.parse::<usize>().ok()).ok_or_else(|| fault(format!("pdf annotation '{object_id}' is not addressable")))?;
    let mut annotation = snapshot.pages.get(page).and_then(|item| item.annotations.get(index)).cloned().ok_or_else(|| fault(format!("pdf annotation '{object_id}' is gone")))?;
    match aspect {
        "color" => annotation.color = if value == "clear" { Vec::new() } else { vec![red, green, blue] },
        "flags" => annotation.flags = flags.max(0.0).round() as u32,
        "name" => annotation.name = none_if_empty(value),
        "state" => annotation.appearance_state = none_if_empty(value),
        "modified" => annotation.modified = none_if_empty(value),
        other => return Err(fault(format!("unknown annotation style '{other}'"))),
    }
    Ok(vec![PdfMutation::SetAnnotation(SetAnnotation { index: page, at: index, annotation })])
}

fn set_page_transition(snapshot: &PdfSnapshot, page: usize, style: &str, direction: &str, duration: f64) -> Result<Vec<PdfMutation>, Fault> {
    require_page(snapshot, page)?;
    let mut next = snapshot.clone();
    let target = &mut next.pages[page];
    target.transition = if style.is_empty() {
        None
    } else {
        let mut entries = vec![PdfDictEntry::new("S", PdfObject::Name(style.to_string()))];
        if !direction.is_empty() {
            entries.push(PdfDictEntry::new("Di", PdfObject::Name(direction.to_string())));
        }
        Some(entries)
    };
    if duration < 0.0 {
        target.duration = None;
    } else if duration > 0.0 {
        target.duration = Some(duration);
    }
    Ok(vec![PdfMutation::SetSnapshot(SetSnapshot { snapshot: next })])
}

fn set_named_object(key: &str, value: &str, missing: &str, remove: impl FnOnce(String) -> PdfMutation, set: impl FnOnce(String, PdfObject) -> PdfMutation) -> Result<Vec<PdfMutation>, Fault> {
    if key.is_empty() {
        return Err(fault(missing));
    }
    Ok(vec![if value.is_empty() { remove(key.to_string()) } else { set(key.to_string(), PdfObject::Name(value.to_string())) }])
}

fn set_annotation_appearance(snapshot: &PdfSnapshot, page: usize, object_id: &str, form: &str) -> Result<Vec<PdfMutation>, Fault> {
    let index = object_id.rsplit_once(":a").and_then(|(_, raw)| raw.parse::<usize>().ok()).ok_or_else(|| fault(format!("pdf annotation '{object_id}' is not addressable")))?;
    let mut annotation = snapshot.pages.get(page).and_then(|item| item.annotations.get(index)).cloned().ok_or_else(|| fault(format!("pdf annotation '{object_id}' is gone")))?;
    annotation.appearance = if form.is_empty() { None } else { Some(PdfAppearance { normal: PdfAppearanceEntry::Single { form: form.to_string() }, rollover: None, down: None }) };
    Ok(vec![PdfMutation::SetAnnotation(SetAnnotation { index: page, at: index, annotation })])
}

fn set_glyph(snapshot: &PdfSnapshot, id: &str, name: &str, x: f64, y: f64, width: f64, height: f64) -> Result<Vec<PdfMutation>, Fault> {
    if name.is_empty() {
        return Err(fault("a glyph needs a name"));
    }
    let mut font = snapshot.fonts.iter().find(|font| font.id == id).cloned().ok_or_else(|| fault(format!("pdf font '{id}' is gone")))?;
    let content = vec![PdfOp::Rectangle { x, y, width: width.max(1.0), height: height.max(1.0) }, PdfOp::Fill];
    match &mut font.kind {
        PdfFontKind::Type3 { char_procs, .. } => {
            if let Some(procedure) = char_procs.iter_mut().find(|procedure| procedure.name == name) {
                procedure.content = content;
            } else {
                char_procs.push(PdfCharProc { name: name.to_string(), content });
            }
        }
        _ => return Err(fault("glyph procedures belong to a Type 3 font")),
    }
    Ok(vec![PdfMutation::SetFont(SetFont { font })])
}

fn set_indirect_object(snapshot: &PdfSnapshot, number: f64, generation: f64, name: &str) -> Result<Vec<PdfMutation>, Fault> {
    let id = ObjRef { num: number.max(0.0).round() as u32, gen: generation.max(0.0).round() as u16 };
    if name.is_empty() {
        if snapshot.objects.iter().any(|object| object.id == id) {
            return Ok(vec![PdfMutation::RemoveObject(RemoveObject { id })]);
        }
        return Err(fault("pdf object is gone"));
    }
    let value = PdfObject::Name(name.to_string());
    let mutation = if snapshot.objects.iter().any(|object| object.id == id) { PdfMutation::SetObjectValue(SetObjectValue { id, value }) } else { PdfMutation::InsertObject(InsertObject { id, value }) };
    Ok(vec![mutation])
}

fn set_mesh_data(snapshot: &PdfSnapshot, id: &str, decode_text: &str, hex: &str) -> Result<Vec<PdfMutation>, Fault> {
    let mut shading = snapshot.shadings.iter().find(|item| item.id == id).cloned().ok_or_else(|| fault(format!("pdf shading '{id}' is gone")))?;
    let PdfShadingKind::Mesh { data, decode, .. } = &mut shading.kind else { return Err(fault(format!("pdf shading '{id}' is not a mesh"))) };
    if !hex.is_empty() {
        *data = decode_hex(hex)?;
    }
    if !decode_text.is_empty() {
        *decode = decode_text.split(',').map(|item| item.trim().parse::<f64>().map_err(|_| fault("mesh decode is comma-separated numbers"))).collect::<Result<Vec<_>, _>>()?;
    }
    Ok(vec![PdfMutation::SetShading(SetShading { shading })])
}

fn set_graphics_state(snapshot: &PdfSnapshot, id: &str, blend: &str, fill_alpha: f64, stroke_alpha: f64) -> Result<Vec<PdfMutation>, Fault> {
    if id.is_empty() {
        return Err(fault("a graphics state needs an id"));
    }
    let mut state = snapshot.ext_g_states.iter().find(|item| item.id == id).cloned().unwrap_or_else(|| PdfExtGState { id: id.to_string(), ..PdfExtGState::default() });
    state.fill_alpha = Some(fill_alpha.clamp(0.0, 1.0));
    state.stroke_alpha = Some(stroke_alpha.clamp(0.0, 1.0));
    if !blend.is_empty() {
        state.blend_mode = Some(vec![blend.to_string()]);
    }
    Ok(vec![PdfMutation::SetExtGState(SetExtGState { state })])
}

fn set_pattern(snapshot: &PdfSnapshot, id: &str, shading: &str, x: f64, y: f64, width: f64, height: f64) -> Result<Vec<PdfMutation>, Fault> {
    if id.is_empty() {
        return Err(fault("a pattern needs an id"));
    }
    let width = if width <= 0.0 { 8.0 } else { width };
    let height = if height <= 0.0 { 8.0 } else { height };
    let kind = if shading.is_empty() {
        PdfPatternKind::Tiling { paint_type: 1, tiling_type: 1, bbox: [x, y, x + width, y + height], x_step: width, y_step: height, content: Vec::new() }
    } else if snapshot.shadings.iter().any(|item| item.id == shading) {
        PdfPatternKind::Shading { shading: shading.to_string(), ext_g_state: None }
    } else {
        return Err(fault(format!("pdf shading '{shading}' is gone")));
    };
    let matrix = snapshot.patterns.iter().find(|item| item.id == id).map(|item| item.matrix).unwrap_or(PDF_IDENTITY_MATRIX);
    Ok(vec![PdfMutation::SetPattern(SetPattern { pattern: PdfPattern { id: id.to_string(), matrix, kind, extra: Vec::new() } })])
}

fn set_color_space(snapshot: &PdfSnapshot, name: &str, kind: &str, separation: &str) -> Result<Vec<PdfMutation>, Fault> {
    if name.is_empty() {
        return Err(fault("a color space needs a name"));
    }
    let color_space = match kind {
        "deviceGray" => PdfColorSpace::DeviceGray,
        "deviceRgb" => PdfColorSpace::DeviceRgb,
        "deviceCmyk" => PdfColorSpace::DeviceCmyk,
        "separation" if !separation.is_empty() => PdfColorSpace::Separation { name: separation.to_string(), alternate: Box::new(PdfColorSpace::DeviceRgb), tint_transform: PdfFunction::Exponential { domain: vec![0.0, 1.0], range: None, c0: vec![0.0, 0.0, 0.0], c1: vec![1.0, 0.0, 0.0], n: 1.0 } },
        "separation" => return Err(fault("a separation needs a name")),
        other => return Err(fault(format!("unknown color space '{other}'"))),
    };
    let _ = snapshot;
    Ok(vec![PdfMutation::SetColorSpace(SetColorSpace { color_space: PdfNamedColorSpace { name: name.to_string(), color_space } })])
}

fn set_properties(snapshot: &PdfSnapshot, name: &str, key: &str, value: &str) -> Result<Vec<PdfMutation>, Fault> {
    if name.is_empty() || key.is_empty() {
        return Err(fault("a property list needs a name and a key"));
    }
    let mut properties = snapshot.properties.iter().find(|item| item.name == name).cloned().unwrap_or(PdfNamedProperties { name: name.to_string(), entries: Vec::new() });
    let entry = PdfDictEntry::new(key, PdfObject::Name(value.to_string()));
    if let Some(existing) = properties.entries.iter_mut().find(|item| item.key == key) {
        *existing = entry;
    } else {
        properties.entries.push(entry);
    }
    Ok(vec![PdfMutation::SetProperties(SetProperties { properties })])
}

fn set_font_metrics(snapshot: &PdfSnapshot, id: &str, encoding_name: &str, widths_text: &str, first: f64, ascent: f64) -> Result<Vec<PdfMutation>, Fault> {
    let mut font = snapshot.fonts.iter().find(|font| font.id == id).cloned().ok_or_else(|| fault(format!("pdf font '{id}' is gone")))?;
    let encoding = font_encoding(encoding_name)?;
    let widths = font_widths(widths_text)?;
    match &mut font.kind {
        PdfFontKind::Type1 { base_font, encoding: slot, first_char, widths: slot_widths, descriptor, .. } | PdfFontKind::TrueType { base_font, encoding: slot, first_char, widths: slot_widths, descriptor, .. } => {
            write_simple_metrics(base_font, slot, first_char, slot_widths, descriptor, encoding, widths, first, ascent);
        }
        _ => return Err(fault("font metrics apply to a simple font")),
    }
    Ok(vec![PdfMutation::SetFont(SetFont { font })])
}

fn write_simple_metrics(base_font: &str, slot: &mut PdfSimpleEncoding, first_char: &mut u32, slot_widths: &mut Vec<f64>, descriptor: &mut Option<PdfFontDescriptor>, encoding: Option<PdfSimpleEncoding>, widths: Option<Vec<f64>>, first: f64, ascent: f64) {
    if let Some(encoding) = encoding {
        *slot = encoding;
    }
    if first >= 0.0 {
        *first_char = first.round() as u32;
    }
    if let Some(widths) = widths {
        *slot_widths = widths;
    }
    if ascent != 0.0 {
        descriptor.get_or_insert_with(|| PdfFontDescriptor { font_name: base_font.to_string(), ..PdfFontDescriptor::default() }).ascent = ascent;
    }
}

fn font_encoding(name: &str) -> Result<Option<PdfSimpleEncoding>, Fault> {
    let base = match name {
        "" => return Ok(None),
        "winAnsi" => PdfBaseEncoding::WinAnsi,
        "macRoman" => PdfBaseEncoding::MacRoman,
        "macExpert" => PdfBaseEncoding::MacExpert,
        "standard" => PdfBaseEncoding::Standard,
        other => return Err(fault(format!("unknown font encoding '{other}'"))),
    };
    Ok(Some(PdfSimpleEncoding { base: Some(base), differences: Vec::new() }))
}

fn font_widths(text: &str) -> Result<Option<Vec<f64>>, Fault> {
    if text.is_empty() {
        return Ok(None);
    }
    text.split(',').map(|item| item.trim().parse::<f64>().map_err(|_| fault("font widths are comma-separated numbers"))).collect::<Result<Vec<_>, _>>().map(Some)
}

fn remove_named(name: &str, message: &str) -> Result<String, Fault> {
    if name.is_empty() { Err(fault(message)) } else { Ok(name.to_string()) }
}

fn edit_shading(snapshot: &PdfSnapshot, object: &PageObject, action: &str, x: f64, y: f64, width: f64, height: f64) -> Result<Vec<PdfMutation>, Fault> {
    let mut shading = snapshot.shadings.iter().find(|item| item.id == object.text).cloned().ok_or_else(|| fault(format!("pdf shading '{}' is gone", object.text)))?;
    let (dx, dy) = user_delta(object.ctm, x - object.x, y - object.y);
    let (x0, y0) = unapply(object.ctm, x, y);
    let (x1, y1) = unapply(object.ctm, x + width, y + height);
    match &mut shading.kind {
        PdfShadingKind::Axial { coords, .. } => {
            if action == "move" {
                coords[0] += dx;
                coords[2] += dx;
                coords[1] += dy;
                coords[3] += dy;
            } else {
                *coords = [x0, y0, x1, y1];
            }
        }
        PdfShadingKind::Radial { coords, .. } => {
            if action == "move" {
                coords[0] += dx;
                coords[3] += dx;
                coords[1] += dy;
                coords[4] += dy;
            } else {
                *coords = [x0, y0, width.max(1.0) / 2.0, x1, y1, height.max(1.0) / 2.0];
            }
        }
        PdfShadingKind::FunctionBased { matrix, .. } => {
            let mut placed = matrix.unwrap_or(PDF_IDENTITY_MATRIX);
            if action == "move" {
                placed[4] += dx;
                placed[5] += dy;
            } else {
                placed[4] = x0;
                placed[5] = y0;
            }
            *matrix = Some(placed);
        }
        PdfShadingKind::Mesh { decode, .. } if decode.len() >= 4 => {
            if action == "move" {
                decode[0] += dx;
                decode[1] += dx;
                decode[2] += dy;
                decode[3] += dy;
            } else {
                decode[0] = x0.min(x1);
                decode[1] = x0.max(x1);
                decode[2] = y0.min(y1);
                decode[3] = y0.max(y1);
            }
        }
        PdfShadingKind::Mesh { .. } => {}
    }
    if let Some(bbox) = &mut shading.bbox {
        if action == "move" {
            bbox[0] += dx;
            bbox[2] += dx;
            bbox[1] += dy;
            bbox[3] += dy;
        } else {
            *bbox = [x0, y0, x1, y1];
        }
    }
    Ok(vec![PdfMutation::SetShading(SetShading { shading })])
}

fn image_color_space(width: u32, height: u32, bytes: usize) -> Result<PdfColorSpace, Fault> {
    let pixels = width as usize * height as usize;
    if bytes == pixels * 3 {
        Ok(PdfColorSpace::DeviceRgb)
    } else if bytes == pixels {
        Ok(PdfColorSpace::DeviceGray)
    } else {
        Err(fault(format!("image samples are {bytes} bytes; expected {pixels} gray or {} rgb", pixels * 3)))
    }
}

fn decode_hex(text: &str) -> Result<Vec<u8>, Fault> {
    let hex: String = text.chars().filter(|char| !char.is_whitespace()).collect();
    if hex.is_empty() || hex.len() % 2 != 0 {
        return Err(fault("image samples must be non-empty even-length hex"));
    }
    (0..hex.len()).step_by(2).map(|index| u8::from_str_radix(&hex[index..index + 2], 16).map_err(|_| fault("image samples must be hexadecimal"))).collect()
}

fn ensure_font(snapshot: &PdfSnapshot) -> (String, Option<PdfMutation>) {
    if let Some(font) = snapshot.fonts.first() {
        return (font.id.clone(), None);
    }
    let font = PdfFont::standard("F1", "Helvetica");
    ("F1".into(), Some(PdfMutation::SetFont(SetFont { font })))
}

fn replace_shown_text(content: &mut [PdfOp], index: usize, text: &str) {
    let Some(op) = content.get_mut(index) else { return };
    match op {
        PdfOp::ShowText { text: shown } | PdfOp::NextLineShowText { text: shown } | PdfOp::NextLineShowTextSpaced { text: shown, .. } => *shown = PdfTextString::text(text),
        PdfOp::ShowTextArray { items } => *items = vec![PdfTextArrayItem::Text { text: text.to_string() }],
        _ => {}
    }
}

fn translate_object(content: &mut [PdfOp], object: &PageObject, dx: f64, dy: f64) {
    let delta = user_delta(object.ctm, dx, dy);
    match object.kind {
        ObjectKind::Text => {
            if let Some(index) = object.position_op {
                shift_position(&mut content[index], delta);
            }
        }
        ObjectKind::Image | ObjectKind::Form => {
            if let Some(index) = object.transform_op {
                if let PdfOp::Transform { matrix } = &mut content[index] {
                    let delta = user_delta(object.parent_ctm, dx, dy);
                    matrix[4] += delta.0;
                    matrix[5] += delta.1;
                }
            }
        }
        ObjectKind::Vector => shift_path(&mut content[object.start..=object.end], object.ctm, dx, dy),
        ObjectKind::Shading | ObjectKind::Annotation => {}
    }
}

fn resize_object(content: &mut [PdfOp], object: &PageObject, x: f64, y: f64, width: f64, height: f64) {
    match object.kind {
        ObjectKind::Vector => {
            for op in &mut content[object.start..=object.end] {
                if let PdfOp::Rectangle { x: rx, y: ry, width: rw, height: rh } = op {
                    *rx = x;
                    *ry = y;
                    *rw = width;
                    *rh = height;
                }
            }
            shift_path_bounds(&mut content[object.start..=object.end], object, x, y, width, height);
        }
        ObjectKind::Image | ObjectKind::Form => {
            if let Some(index) = object.transform_op {
                if let PdfOp::Transform { matrix } = &mut content[index] {
                    matrix[0] = width;
                    matrix[3] = height;
                    matrix[4] = x;
                    matrix[5] = y;
                }
            }
        }
        ObjectKind::Text => {
            if let Some(index) = object.position_op {
                shift_position(&mut content[index], (x - object.x, y - object.y));
            }
            let size = height.max(1.0);
            let font = content.iter().rev().find_map(|op| match op {
                PdfOp::SetFont { name, .. } => Some(name.clone()),
                _ => None,
            });
            if let Some(name) = font {
                let show = object.end;
                if show <= content.len() {
                    // Font size is applied by rewriting the nearest SetFont before this show.
                    if let Some(op) = content[..show].iter_mut().rev().find(|op| matches!(op, PdfOp::SetFont { name: current, .. } if current == &name)) {
                        if let PdfOp::SetFont { size: font_size, .. } = op {
                            *font_size = size;
                        }
                    }
                }
            }
        }
        ObjectKind::Shading | ObjectKind::Annotation => {}
    }
}

fn set_fill(content: &mut Vec<PdfOp>, object: &PageObject, red: f64, green: f64, blue: f64) {
    let found = content[..=object.end].iter_mut().rev().find(|op| matches!(op, PdfOp::SetFillRgb { .. } | PdfOp::SetFillGray { .. }));
    if let Some(op) = found {
        *op = PdfOp::SetFillRgb { r: red, g: green, b: blue };
        return;
    }
    content.insert(object.start, PdfOp::SetFillRgb { r: red, g: green, b: blue });
}

fn set_stroke(content: &mut Vec<PdfOp>, object: &PageObject, red: f64, green: f64, blue: f64, width: f64) {
    if let Some(op) = content.get_mut(object.end) {
        match op {
            PdfOp::Fill => *op = PdfOp::FillStroke,
            PdfOp::FillEvenOdd => *op = PdfOp::FillStrokeEvenOdd,
            _ => {}
        }
    }
    content.insert(object.start, PdfOp::SetLineWidth { width: if width > 0.0 { width } else { 1.0 } });
    content.insert(object.start, PdfOp::SetStrokeRgb { r: red, g: green, b: blue });
}

fn shift_position(op: &mut PdfOp, delta: (f64, f64)) {
    match op {
        PdfOp::SetTextMatrix { matrix } => {
            matrix[4] += delta.0;
            matrix[5] += delta.1;
        }
        PdfOp::MoveText { tx, ty } | PdfOp::MoveTextSetLeading { tx, ty } => {
            *tx += delta.0;
            *ty += delta.1;
        }
        _ => {}
    }
}

fn shift_path(ops: &mut [PdfOp], ctm: PdfMatrix, dx: f64, dy: f64) {
    let inverse = invert(ctm).unwrap_or(PDF_IDENTITY_MATRIX);
    let map = |x: f64, y: f64| {
        let (px, py) = apply(ctm, x, y);
        apply(inverse, px + dx, py + dy)
    };
    for op in ops {
        match op {
            PdfOp::MoveTo { x, y } | PdfOp::LineTo { x, y } => {
                let (nx, ny) = map(*x, *y);
                *x = nx;
                *y = ny;
            }
            PdfOp::Rectangle { x, y, .. } => {
                let (nx, ny) = map(*x, *y);
                *x = nx;
                *y = ny;
            }
            PdfOp::CurveTo { x1, y1, x2, y2, x3, y3 } => {
                let (a, b) = map(*x1, *y1);
                let (c, d) = map(*x2, *y2);
                let (e, f) = map(*x3, *y3);
                *x1 = a;
                *y1 = b;
                *x2 = c;
                *y2 = d;
                *x3 = e;
                *y3 = f;
            }
            PdfOp::CurveToInitial { x2, y2, x3, y3 } => {
                let (c, d) = map(*x2, *y2);
                let (e, f) = map(*x3, *y3);
                *x2 = c;
                *y2 = d;
                *x3 = e;
                *y3 = f;
            }
            PdfOp::CurveToFinal { x1, y1, x3, y3 } => {
                let (a, b) = map(*x1, *y1);
                let (e, f) = map(*x3, *y3);
                *x1 = a;
                *y1 = b;
                *x3 = e;
                *y3 = f;
            }
            _ => {}
        }
    }
}

fn shift_path_bounds(ops: &mut [PdfOp], object: &PageObject, x: f64, y: f64, width: f64, height: f64) {
    if ops.iter().any(|op| matches!(op, PdfOp::Rectangle { .. })) {
        return;
    }
    let inverse = invert(object.ctm).unwrap_or(PDF_IDENTITY_MATRIX);
    let map = |px: f64, py: f64| {
        let (ux, uy) = apply(object.ctm, px, py);
        let nx = if object.width.abs() < 1.0e-6 { x } else { x + (ux - object.x) / object.width * width };
        let ny = if object.height.abs() < 1.0e-6 { y } else { y + (uy - object.y) / object.height * height };
        apply(inverse, nx, ny)
    };
    for op in ops {
        match op {
            PdfOp::MoveTo { x, y } | PdfOp::LineTo { x, y } => {
                let (nx, ny) = map(*x, *y);
                *x = nx;
                *y = ny;
            }
            _ => {}
        }
    }
}

fn page_objects(snapshot: &PdfSnapshot, page_index: usize, page: &PdfPage) -> Vec<PageObject> {
    let mut found = Vec::new();
    let mut stack = vec![Graphics::default()];
    let mut path_start = None;
    let mut path_points = Vec::new();
    for (index, op) in page.content.iter().enumerate() {
        match op {
            PdfOp::Save => {
                let state = stack.last().expect("graphics stack").clone();
                stack.push(state);
            }
            PdfOp::Restore => {
                if stack.len() > 1 {
                    stack.pop();
                }
            }
            PdfOp::Transform { matrix } => {
                let state = stack.last_mut().expect("graphics stack");
                state.parent_ctm = state.ctm;
                state.ctm = multiply(*matrix, state.ctm);
                state.transform_op = Some(index);
            }
            PdfOp::SetFillRgb { r, g, b } => stack.last_mut().expect("graphics").fill = [*r, *g, *b],
            PdfOp::SetStrokeRgb { r, g, b } => stack.last_mut().expect("graphics").stroke = [*r, *g, *b],
            PdfOp::SetFillGray { gray } => stack.last_mut().expect("graphics").fill = [*gray, *gray, *gray],
            PdfOp::SetStrokeGray { gray } => stack.last_mut().expect("graphics").stroke = [*gray, *gray, *gray],
            PdfOp::SetLineWidth { width } => stack.last_mut().expect("graphics").line_width = *width,
            PdfOp::SetFont { name, size } => {
                let state = stack.last_mut().expect("graphics");
                state.font = name.clone();
                state.font_size = *size;
            }
            PdfOp::SetLeading { leading } => stack.last_mut().expect("graphics").leading = *leading,
            PdfOp::SetCharSpacing { spacing } => stack.last_mut().expect("graphics").char_spacing = *spacing,
            PdfOp::SetWordSpacing { spacing } => stack.last_mut().expect("graphics").word_spacing = *spacing,
            PdfOp::SetHorizontalScale { scale } => stack.last_mut().expect("graphics").horizontal_scale = *scale,
            PdfOp::BeginText => {
                let state = stack.last_mut().expect("graphics");
                state.text_matrix = PDF_IDENTITY_MATRIX;
                state.line_matrix = PDF_IDENTITY_MATRIX;
                state.position_op = None;
            }
            PdfOp::SetTextMatrix { matrix } => {
                let state = stack.last_mut().expect("graphics");
                state.text_matrix = *matrix;
                state.line_matrix = *matrix;
                state.position_op = Some(index);
            }
            PdfOp::MoveText { tx, ty } => move_text(stack.last_mut().expect("graphics"), *tx, *ty, index, false),
            PdfOp::MoveTextSetLeading { tx, ty } => move_text(stack.last_mut().expect("graphics"), *tx, *ty, index, true),
            PdfOp::NextLine => {
                let leading = stack.last().expect("graphics").leading;
                move_text(stack.last_mut().expect("graphics"), 0.0, -leading, index, false);
            }
            PdfOp::MoveTo { x, y } => {
                if path_start.is_none() {
                    path_start = Some(index);
                    path_points.clear();
                }
                path_points.push(apply(stack.last().expect("graphics").ctm, *x, *y));
            }
            PdfOp::LineTo { x, y } | PdfOp::CurveTo { x3: x, y3: y, .. } | PdfOp::CurveToInitial { x3: x, y3: y, .. } | PdfOp::CurveToFinal { x3: x, y3: y, .. } => {
                if path_start.is_none() {
                    path_start = Some(index);
                }
                path_points.push(apply(stack.last().expect("graphics").ctm, *x, *y));
            }
            PdfOp::Rectangle { x, y, width, height } => {
                if path_start.is_none() {
                    path_start = Some(index);
                    path_points.clear();
                }
                let ctm = stack.last().expect("graphics").ctm;
                path_points.extend([apply(ctm, *x, *y), apply(ctm, *x + *width, *y), apply(ctm, *x + *width, *y + *height), apply(ctm, *x, *y + *height)]);
            }
            PdfOp::Stroke | PdfOp::CloseStroke | PdfOp::Fill | PdfOp::FillEvenOdd | PdfOp::FillStroke | PdfOp::FillStrokeEvenOdd | PdfOp::CloseFillStroke | PdfOp::CloseFillStrokeEvenOdd | PdfOp::EndPath => {
                if let Some(start) = path_start.take() {
                    let state = stack.last().expect("graphics");
                    found.push(bounded_object(page_index, ObjectKind::Vector, start, index, None, None, state, &path_points, String::new(), state.font_size));
                    path_points.clear();
                }
            }
            PdfOp::ShowText { text } => found.push(text_object(page_index, index, stack.last_mut().expect("graphics"), shown(text))),
            PdfOp::ShowTextArray { items } => {
                let text = items.iter().filter_map(|item| match item { PdfTextArrayItem::Text { text } => Some(text.as_str()), _ => None }).collect::<String>();
                found.push(text_object(page_index, index, stack.last_mut().expect("graphics"), text));
            }
            PdfOp::NextLineShowText { text } => {
                let leading = stack.last().expect("graphics").leading;
                move_text(stack.last_mut().expect("graphics"), 0.0, -leading, index, false);
                found.push(text_object(page_index, index, stack.last_mut().expect("graphics"), shown(text)));
            }
            PdfOp::NextLineShowTextSpaced { word_spacing, char_spacing, text } => {
                let state = stack.last_mut().expect("graphics");
                state.word_spacing = *word_spacing;
                state.char_spacing = *char_spacing;
                let leading = state.leading;
                move_text(state, 0.0, -leading, index, false);
                found.push(text_object(page_index, index, stack.last_mut().expect("graphics"), shown(text)));
            }
            PdfOp::PaintXObject { name } => {
                let state = stack.last().expect("graphics");
                let (kind, label) = if snapshot.images.iter().any(|image| image.id == *name) {
                    (ObjectKind::Image, name.clone())
                } else if snapshot.forms.iter().any(|form| form.id == *name) {
                    (ObjectKind::Form, name.clone())
                } else {
                    (ObjectKind::Form, name.clone())
                };
                let (start, end, transform_op) = paint_span(&page.content, index);
                let corners = unit_square(state.ctm);
                found.push(bounded_object(page_index, kind, start, end, None, transform_op, state, &corners, label, 0.0));
            }
            PdfOp::InlineImage { image } => {
                let state = stack.last().expect("graphics");
                let corners = unit_square(state.ctm);
                found.push(bounded_object(page_index, ObjectKind::Image, index, index, None, state.transform_op, state, &corners, format!("{}×{}", image.width, image.height), 0.0));
            }
            PdfOp::PaintShading { name } => {
                let state = stack.last().expect("graphics");
                let corners = shading_corners(snapshot, name, page, state.ctm);
                found.push(bounded_object(page_index, ObjectKind::Shading, index, index, None, None, state, &corners, name.clone(), 0.0));
            }
            _ => {}
        }
    }
    for (index, annotation) in page.annotations.iter().enumerate() {
        let state = Graphics::default();
        let corners = [(annotation.rect[0], annotation.rect[1]), (annotation.rect[2], annotation.rect[3])];
        let mut object = bounded_object(page_index, ObjectKind::Annotation, index, index, None, None, &state, &corners, annotation.contents.clone().unwrap_or_default(), 12.0);
        object.id = format!("p{page_index}:a{index}");
        found.push(object);
    }
    found
}

fn text_object(page_index: usize, index: usize, state: &mut Graphics, text: String) -> PageObject {
    let origin = apply(multiply(state.text_matrix, state.ctm), 0.0, 0.0);
    let width = text_width(&text, state);
    let advance = width;
    state.text_matrix[4] += state.text_matrix[0] * advance;
    state.text_matrix[5] += state.text_matrix[1] * advance;
    let corners = [origin, (origin.0 + width, origin.1), (origin.0 + width, origin.1 + state.font_size), (origin.0, origin.1 + state.font_size)];
    bounded_object(page_index, ObjectKind::Text, index, index, state.position_op, None, state, &corners, text, state.font_size)
}

fn bounded_object(page_index: usize, kind: ObjectKind, start: usize, end: usize, position_op: Option<usize>, transform_op: Option<usize>, state: &Graphics, points: &[(f64, f64)], text: String, font_size: f64) -> PageObject {
    let (x, y, width, height) = bounds(points);
    PageObject {
        id: format!("p{page_index}:{}{end}", kind.letter()),
        page: page_index,
        kind,
        start,
        end,
        position_op,
        transform_op,
        ctm: state.ctm,
        parent_ctm: state.parent_ctm,
        x,
        y,
        width,
        height,
        text,
        font_size,
        fill: state.fill,
        stroke: state.stroke,
        line_width: state.line_width,
    }
}

fn shading_corners(snapshot: &PdfSnapshot, name: &str, page: &PdfPage, ctm: PdfMatrix) -> Vec<(f64, f64)> {
    let media = vec![(page.media_box[0], page.media_box[1]), (page.media_box[2], page.media_box[3])];
    let Some(shading) = snapshot.shadings.iter().find(|item| item.id == name) else { return media };
    if let Some(bbox) = shading.bbox {
        return vec![apply(ctm, bbox[0], bbox[1]), apply(ctm, bbox[2], bbox[3])];
    }
    match &shading.kind {
        PdfShadingKind::Axial { coords, .. } => vec![apply(ctm, coords[0], coords[1]), apply(ctm, coords[2], coords[3])],
        PdfShadingKind::Radial { coords, .. } => vec![apply(ctm, coords[0] - coords[2], coords[1] - coords[2]), apply(ctm, coords[3] + coords[5], coords[4] + coords[5])],
        PdfShadingKind::FunctionBased { domain, matrix, .. } => {
            let domain = domain.unwrap_or([0.0, 1.0, 0.0, 1.0]);
            let placed = multiply(ctm, matrix.unwrap_or(PDF_IDENTITY_MATRIX));
            vec![apply(placed, domain[0], domain[2]), apply(placed, domain[1], domain[3])]
        }
        PdfShadingKind::Mesh { decode, .. } if decode.len() >= 4 => vec![apply(ctm, decode[0], decode[2]), apply(ctm, decode[1], decode[3])],
        PdfShadingKind::Mesh { .. } => media,
    }
}

fn paint_span(ops: &[PdfOp], paint: usize) -> (usize, usize, Option<usize>) {
    let transform_op = paint.checked_sub(1).filter(|index| matches!(ops.get(*index), Some(PdfOp::Transform { .. })));
    let mut start = transform_op.unwrap_or(paint);
    let mut end = paint;
    if let Some(transform) = transform_op {
        if transform > 0 && matches!(ops.get(transform - 1), Some(PdfOp::Save)) && matches!(ops.get(paint + 1), Some(PdfOp::Restore)) {
            start = transform - 1;
            end = paint + 1;
        }
    }
    (start, end, transform_op)
}

fn move_text(state: &mut Graphics, tx: f64, ty: f64, index: usize, set_leading: bool) {
    if set_leading {
        state.leading = -ty;
    }
    let matrix = state.line_matrix;
    state.line_matrix[4] = matrix[0] * tx + matrix[2] * ty + matrix[4];
    state.line_matrix[5] = matrix[1] * tx + matrix[3] * ty + matrix[5];
    state.text_matrix = state.line_matrix;
    state.position_op = Some(index);
}

struct Placed {
    object: PageObject,
    top: f64,
    canvas_x: f64,
    canvas_y: f64,
    canvas_w: f64,
    canvas_h: f64,
}

fn placement(snapshot: &PdfSnapshot) -> Vec<Placed> {
    let mut tops = Vec::new();
    let mut top = 0.0;
    for page in &snapshot.pages {
        tops.push(top);
        top += displayed_size(page).1 + 16.0;
    }
    objects(snapshot)
        .into_iter()
        .map(|object| {
            let page = &snapshot.pages[object.page];
            let page_top = tops[object.page];
            let (canvas_x, canvas_y, canvas_w, canvas_h) = display_rect(page, page_top, object.x, object.y, object.width, object.height);
            Placed { object, top: page_top, canvas_x, canvas_y, canvas_w, canvas_h }
        })
        .collect()
}

fn display_rect(page: &PdfPage, top: f64, x: f64, y: f64, width: f64, height: f64) -> (f64, f64, f64, f64) {
    let corners = [display_point(page, top, x, y), display_point(page, top, x + width, y), display_point(page, top, x + width, y + height), display_point(page, top, x, y + height)];
    let min_x = corners.iter().map(|point| point.0).fold(f64::INFINITY, f64::min);
    let min_y = corners.iter().map(|point| point.1).fold(f64::INFINITY, f64::min);
    let max_x = corners.iter().map(|point| point.0).fold(f64::NEG_INFINITY, f64::max);
    let max_y = corners.iter().map(|point| point.1).fold(f64::NEG_INFINITY, f64::max);
    (min_x, min_y, (max_x - min_x).max(0.0), (max_y - min_y).max(0.0))
}

fn display_point(page: &PdfPage, top: f64, x: f64, y: f64) -> (f64, f64) {
    let left = page.media_box[0];
    let bottom = page.media_box[1];
    let width = page.width();
    let height = page.height();
    let x = x - left;
    let y = y - bottom;
    let (dx, dy) = match page.rotate.rem_euclid(360) {
        90 => (y, x),
        180 => (width - x, y),
        270 => (height - y, width - x),
        _ => (x, height - y),
    };
    (dx, top + dy)
}

fn displayed_size(page: &PdfPage) -> (f64, f64) {
    match page.rotate.rem_euclid(360) {
        90 | 270 => (page.height(), page.width()),
        _ => (page.width(), page.height()),
    }
}

fn vector_segments(page: &PdfPage, top: f64, ops: &[PdfOp]) -> Value {
    let mut segments = Vec::new();
    let mut current = (0.0, 0.0);
    let mut start = (0.0, 0.0);
    let push = |segments: &mut Vec<Value>, kind: &str, point: Option<(f64, f64)>| {
        if let Some((x, y)) = point {
            let shown = display_point(page, top, x, y);
            segments.push(json!({ "kind": kind, "to": [shown.0, shown.1] }));
        } else {
            segments.push(json!({ "kind": kind }));
        }
    };
    for op in ops {
        match op {
            PdfOp::MoveTo { x, y } => {
                current = (*x, *y);
                start = current;
                push(&mut segments, "move", Some(current));
            }
            PdfOp::LineTo { x, y } => {
                current = (*x, *y);
                push(&mut segments, "line", Some(current));
            }
            PdfOp::Rectangle { x, y, width, height } => {
                push(&mut segments, "move", Some((*x, *y)));
                push(&mut segments, "line", Some((*x + *width, *y)));
                push(&mut segments, "line", Some((*x + *width, *y + *height)));
                push(&mut segments, "line", Some((*x, *y + *height)));
                push(&mut segments, "close", None);
                current = (*x, *y);
            }
            PdfOp::CurveTo { x1, y1, x2, y2, x3, y3 } => {
                for step in 1..=8 {
                    let point = cubic(current, (*x1, *y1), (*x2, *y2), (*x3, *y3), step as f64 / 8.0);
                    push(&mut segments, "line", Some(point));
                }
                current = (*x3, *y3);
            }
            PdfOp::ClosePath | PdfOp::CloseStroke | PdfOp::CloseFillStroke | PdfOp::CloseFillStrokeEvenOdd => {
                push(&mut segments, "close", None);
                current = start;
            }
            _ => {}
        }
    }
    Value::Array(segments)
}

fn rect_segments(x: f64, y: f64, width: f64, height: f64) -> Value {
    json!([{ "kind": "move", "to": [x, y] }, { "kind": "line", "to": [x + width, y] }, { "kind": "line", "to": [x + width, y + height] }, { "kind": "line", "to": [x, y + height] }, { "kind": "close" }])
}

fn cubic(p0: (f64, f64), p1: (f64, f64), p2: (f64, f64), p3: (f64, f64), t: f64) -> (f64, f64) {
    let u = 1.0 - t;
    let a = u * u * u;
    let b = 3.0 * u * u * t;
    let c = 3.0 * u * t * t;
    let d = t * t * t;
    (a * p0.0 + b * p1.0 + c * p2.0 + d * p3.0, a * p0.1 + b * p1.1 + c * p2.1 + d * p3.1)
}

fn shown(text: &PdfTextString) -> String {
    match text {
        PdfTextString::Text { text } => text.clone(),
        PdfTextString::Codes { bytes } => String::from_utf8_lossy(bytes).into_owned(),
    }
}

fn text_width(text: &str, state: &Graphics) -> f64 {
    let chars = text.chars().count() as f64;
    let spaces = text.chars().filter(|char| *char == ' ').count() as f64;
    (chars * state.font_size * 0.5 + chars * state.char_spacing + spaces * state.word_spacing) * state.horizontal_scale / 100.0
}

fn unit_square(ctm: PdfMatrix) -> [(f64, f64); 4] {
    [apply(ctm, 0.0, 0.0), apply(ctm, 1.0, 0.0), apply(ctm, 1.0, 1.0), apply(ctm, 0.0, 1.0)]
}

fn bounds(points: &[(f64, f64)]) -> (f64, f64, f64, f64) {
    if points.is_empty() {
        return (0.0, 0.0, 0.0, 0.0);
    }
    let min_x = points.iter().map(|point| point.0).fold(f64::INFINITY, f64::min);
    let min_y = points.iter().map(|point| point.1).fold(f64::INFINITY, f64::min);
    let max_x = points.iter().map(|point| point.0).fold(f64::NEG_INFINITY, f64::max);
    let max_y = points.iter().map(|point| point.1).fold(f64::NEG_INFINITY, f64::max);
    (min_x, min_y, (max_x - min_x).max(0.0), (max_y - min_y).max(0.0))
}

fn user_delta(ctm: PdfMatrix, dx: f64, dy: f64) -> (f64, f64) {
    let inverse = invert(ctm).unwrap_or(PDF_IDENTITY_MATRIX);
    (inverse[0] * dx + inverse[2] * dy, inverse[1] * dx + inverse[3] * dy)
}

fn unapply(matrix: PdfMatrix, x: f64, y: f64) -> (f64, f64) {
    apply(invert(matrix).unwrap_or(PDF_IDENTITY_MATRIX), x, y)
}

fn apply(matrix: PdfMatrix, x: f64, y: f64) -> (f64, f64) {
    (matrix[0] * x + matrix[2] * y + matrix[4], matrix[1] * x + matrix[3] * y + matrix[5])
}

fn multiply(left: PdfMatrix, right: PdfMatrix) -> PdfMatrix {
    [
        left[0] * right[0] + left[2] * right[1],
        left[1] * right[0] + left[3] * right[1],
        left[0] * right[2] + left[2] * right[3],
        left[1] * right[2] + left[3] * right[3],
        left[0] * right[4] + left[2] * right[5] + left[4],
        left[1] * right[4] + left[3] * right[5] + left[5],
    ]
}

fn invert(matrix: PdfMatrix) -> Option<PdfMatrix> {
    let det = matrix[0] * matrix[3] - matrix[1] * matrix[2];
    if det.abs() < 1.0e-12 {
        return None;
    }
    let inv = 1.0 / det;
    Some([matrix[3] * inv, -matrix[1] * inv, -matrix[2] * inv, matrix[0] * inv, (matrix[2] * matrix[5] - matrix[3] * matrix[4]) * inv, (matrix[1] * matrix[4] - matrix[0] * matrix[5]) * inv])
}

fn fault(message: impl Into<String>) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new("stdio.pdf.page-edit"), message)
}

fn text_arg(args: Option<&dsl::DslValue>, key: &str) -> String {
    semio_s_artifact_stdio_contract::window_kit_text_argument(args, &[key], "")
}

fn req_text(args: Option<&dsl::DslValue>, key: &str) -> Result<String, Fault> {
    semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, key).map_err(|error| fault(error.message))
}

fn req_index(args: Option<&dsl::DslValue>, key: &str) -> Result<u32, Fault> {
    semio_s_artifact_stdio_contract::window_kit_required_index_argument(args, key).map_err(|error| fault(error.message))
}

fn opt_index(args: Option<&dsl::DslValue>, key: &str) -> u32 {
    text_arg(args, key).parse::<u32>().unwrap_or(0)
}

fn req_num(args: Option<&dsl::DslValue>, key: &str) -> Result<f64, Fault> {
    let raw = text_arg(args, key);
    raw.parse::<f64>().map_err(|_| fault(format!("the action requires argument '{key}'"))).and_then(|value| value.is_finite().then_some(value).ok_or_else(|| fault(format!("argument '{key}' must be finite"))))
}

fn opt_num(args: Option<&dsl::DslValue>, key: &str, fallback: f64) -> f64 {
    text_arg(args, key).parse::<f64>().ok().filter(|value| value.is_finite()).unwrap_or(fallback)
}

fn json_f64(value: &Value, key: &str) -> f64 {
    value.get(key).and_then(Value::as_f64).unwrap_or(0.0)
}

#[derive(Clone, Debug)]
struct Graphics {
    ctm: PdfMatrix,
    parent_ctm: PdfMatrix,
    fill: [f64; 3],
    stroke: [f64; 3],
    line_width: f64,
    font: String,
    font_size: f64,
    leading: f64,
    char_spacing: f64,
    word_spacing: f64,
    horizontal_scale: f64,
    text_matrix: PdfMatrix,
    line_matrix: PdfMatrix,
    position_op: Option<usize>,
    transform_op: Option<usize>,
}

impl Default for Graphics {
    fn default() -> Self {
        Self {
            ctm: PDF_IDENTITY_MATRIX,
            parent_ctm: PDF_IDENTITY_MATRIX,
            fill: [0.0, 0.0, 0.0],
            stroke: [0.0, 0.0, 0.0],
            line_width: 1.0,
            font: String::new(),
            font_size: 12.0,
            leading: 0.0,
            char_spacing: 0.0,
            word_spacing: 0.0,
            horizontal_scale: 100.0,
            text_matrix: PDF_IDENTITY_MATRIX,
            line_matrix: PDF_IDENTITY_MATRIX,
            position_op: None,
            transform_op: None,
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
