//! 🪟️ PDF page editor windows: the page and inspector window definitions, their rendering and the document layout.

use semio_plugin::{WindowLayoutNode, Axis};
use super::*;

//#region 🪟️Windows
/// 🎬️ One document edit verb: a bounded mutation every pane commits through the shared worker job.
fn edit_action(id: &str, en: &str, de: &str, args: Vec<ActionArgDef>) -> ActionDefinition {
    let mut definition = ActionDefinition::bounded_catalog(id, LocalizedLabel::native(en, de), ActionKind::Mutation).with_args(args);
    definition.semantics.execution.interactive_job = semio_plugin::InteractiveJobClassification::Migrated;
    definition
}

/// 🪟 The page window: a canvas that owns exactly the actions only it dispatches (its pointer gestures, the view tools and the presence lane). The document edits it publishes are [`edit_action_definitions`], declared once on the app roster.
pub fn window_definition() -> WindowKindDefinition {
    let coordinates = || vec![ActionArgDef::number("x", LocalizedLabel::native("X", "X")), ActionArgDef::number("y", LocalizedLabel::native("Y", "Y"))];
    let mut definition = WindowKindDefinition {
        initial_utility_id: None,
        id: WINDOW_KIND_ID.into(),
        label: LocalizedLabel::native("Page", "Seite"),
        body_key: BODY_KEY.into(),
        surface_kind: SurfaceKind::Canvas2d,
        icon_id: "file-text".into(),
        options: WindowOptions::default(),
        actions: vec![
            edit_action("canvasPointerDown", "Canvas Pointer Down", "Leinwandzeiger", coordinates()).input_event(),
            edit_action("canvasPointerMove", "Canvas Pointer Move", "Leinwandzeiger bewegt", coordinates()).input_event(),
            edit_action("canvasPointerUp", "Canvas Pointer Up", "Leinwandzeiger losgelassen", coordinates()).input_event(),
        ],
        utilities: Vec::new(),
        interactions: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    };
    definition.actions.extend(view_action_definitions());
    definition.actions.extend(semio_s_artifact_stdio_pdf_presence_lane::presence_action_definitions());
    definition
}

/// ✏️ Declares every document edit verb once on the app roster of `builder`, where the page canvas, the Object pane and the thumbnail strip all resolve it (`window_kind_actions`): a verb no window kind claims is dispatchable from every window of the app, so no pane copies another pane's actions into the descriptor.
pub fn edit_actions_with(builder: semio_plugin::app::EditorBuilder) -> semio_plugin::app::EditorBuilder {
    edit_action_definitions().into_iter().fold(builder, semio_plugin::app::EditorBuilder::action_with)
}

/// ✏️ The document edit verbs: one action per edit the page canvas, the Object pane or the thumbnail strip can publish.
pub fn edit_action_definitions() -> Vec<ActionDefinition> {
    let page = || ActionArgDef::number("page", LocalizedLabel::native("Page number", "Seitenzahl")).required();
    let page_index = || ActionArgDef::index("page", LocalizedLabel::native("Page index", "Seitenindex")).required();
    let object = || ActionArgDef::text("object", LocalizedLabel::native("Object", "Objekt")).min_length(1).required();
    let text = || ActionArgDef::text("text", LocalizedLabel::native("Text", "Text")).min_length(0).required();
    let x = || ActionArgDef::number("x", LocalizedLabel::native("X", "X")).required();
    let y = || ActionArgDef::number("y", LocalizedLabel::native("Y", "Y")).required();
    let width = || ActionArgDef::number("width", LocalizedLabel::native("Width", "Breite")).required();
    let height = || ActionArgDef::number("height", LocalizedLabel::native("Height", "Höhe")).required();
    let red = || ActionArgDef::number("red", LocalizedLabel::native("Red", "Rot")).required();
    let green = || ActionArgDef::number("green", LocalizedLabel::native("Green", "Grün")).required();
    let blue = || ActionArgDef::number("blue", LocalizedLabel::native("Blue", "Blau")).required();
    let action = edit_action;
    vec![
        action(
            "set-page",
            "Set Page Text",
            "Seitentext setzen",
            vec![page_index(), ActionArgDef::number("item", LocalizedLabel::native("Text Item", "Textelement")).required(), ActionArgDef::target_revision("revision", LocalizedLabel::native("Revision", "Revision")).min_length(1), text()],
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
        action("move-page", "Move Page", "Seite verschieben", vec![ActionArgDef::number("page", LocalizedLabel::native("Page number", "Seitenzahl")), ActionArgDef::number("x", LocalizedLabel::native("Target page number", "Zielseitenzahl")).required()]),
        action("set-page-size", "Set Page Size", "Seitengröße setzen", vec![page(), width(), height()]),
        action("set-info", "Set Document Info", "Dokumentinfo setzen", vec![ActionArgDef::text("text", LocalizedLabel::native("Title", "Titel")).min_length(0), ActionArgDef::text("extra", LocalizedLabel::native("Author", "Autor")).min_length(0), ActionArgDef::text("object", LocalizedLabel::native("Subject", "Thema")).min_length(0)]),
        action("set-info-field", "Set Info Field", "Infofeld setzen", vec![object(), text()]),
        action("set-annotation", "Set Annotation", "Anmerkung setzen", vec![page(), object(), text(), x(), y(), width(), height()]),
        action("set-annotation-style", "Set Annotation Style", "Anmerkung gestalten", vec![page(), object(), text(), ActionArgDef::text("extra", LocalizedLabel::native("Value", "Wert")).min_length(0), x(), red(), green(), blue()]),
        action("set-annotation-border", "Set Annotation Border", "Rahmen setzen", vec![page(), object(), ActionArgDef::text("text", LocalizedLabel::native("Style", "Stil")).min_length(0), ActionArgDef::text("extra", LocalizedLabel::native("Dash", "Strich")).min_length(0), x(), y(), width()]),
        action("set-annotation-markup", "Set Annotation Markup", "Markierung setzen", vec![page(), object(), text(), ActionArgDef::text("extra", LocalizedLabel::native("Value", "Wert")).min_length(0), x()]),
        action("set-form-settings", "Set Form Settings", "Formular setzen", vec![object(), ActionArgDef::text("text", LocalizedLabel::native("Value", "Wert")).min_length(0), ActionArgDef::text("extra", LocalizedLabel::native("Key", "Schlüssel")).min_length(0), x()]),
        action("set-extra-entry", "Set Extra Entry", "Zusatzeintrag setzen", vec![object(), ActionArgDef::text("extra", LocalizedLabel::native("Key", "Schlüssel")).min_length(1).required(), ActionArgDef::text("text", LocalizedLabel::native("Value", "Wert")).min_length(0)]),
        action("set-annotation-kind", "Set Annotation Kind", "Anmerkungsart setzen", vec![page(), object(), text(), ActionArgDef::text("extra", LocalizedLabel::native("Value", "Wert")).min_length(0), x(), y(), width(), height(), red(), green(), blue()]),
        action("set-field-data", "Set Field Data", "Felddaten setzen", vec![object(), text(), ActionArgDef::text("extra", LocalizedLabel::native("Value", "Wert")).min_length(0), x()]),
        action("set-resource-detail", "Set Resource Detail", "Ressource setzen", vec![object(), text(), ActionArgDef::text("extra", LocalizedLabel::native("Value", "Wert")).min_length(0), x(), y(), width(), height(), red(), green(), blue()]),
        action("set-page-extra", "Set Page Extra", "Seite ergänzen", vec![page(), object(), text(), ActionArgDef::text("extra", LocalizedLabel::native("Key", "Schlüssel")).min_length(0), x(), y()]),
        action("set-font", "Set Font", "Schrift setzen", vec![page(), object(), text()]),
        action("set-font-size", "Set Font Size", "Schriftgröße setzen", vec![page(), object(), ActionArgDef::number("size", LocalizedLabel::native("Font size", "Schriftgröße")).required()]),
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
        action("set-font-reference", "Set Font Reference", "Schriftreferenz setzen", vec![object(), text(), ActionArgDef::text("extra", LocalizedLabel::native("Artifact Identity", "Artefaktidentität")).min_length(1).required()]),
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
        action("set-mesh-reference", "Set Mesh Reference", "Netzreferenz setzen", vec![object(), ActionArgDef::text("text", LocalizedLabel::native("Decode", "Dekodierung")).min_length(0), ActionArgDef::text("extra", LocalizedLabel::native("Artifact Identity", "Artefaktidentität")).min_length(0)]),
        action("set-entry", "Set Entry", "Eintrag setzen", vec![ActionArgDef::text("object", LocalizedLabel::native("Family (catalog, trailer, info, viewer)", "Familie (catalog, trailer, info, viewer)")).min_length(1).required(), ActionArgDef::text("text", LocalizedLabel::native("Value (key=value to add)", "Wert (key=value zum Hinzufügen)")).min_length(0), ActionArgDef::text("extra", LocalizedLabel::native("Key", "Schlüssel")).min_length(0)]),
        action("create-annotation", "Create Annotation", "Anmerkung erstellen", vec![page(), ActionArgDef::text("object", LocalizedLabel::native("Kind (note, highlight, underline, strikeOut, squiggly, ink, link)", "Art (note, highlight, underline, strikeOut, squiggly, ink, link)")).min_length(1).required(), ActionArgDef::text("text", LocalizedLabel::native("Contents or link", "Inhalt oder Link")).min_length(0), ActionArgDef::text("extra", LocalizedLabel::native("Quad points or ink strokes", "Quadpunkte oder Freihandstriche")).min_length(0), x(), y(), ActionArgDef::number("width", LocalizedLabel::native("Width", "Breite")), ActionArgDef::number("height", LocalizedLabel::native("Height", "Höhe")), ActionArgDef::number("red", LocalizedLabel::native("Red", "Rot")), ActionArgDef::number("green", LocalizedLabel::native("Green", "Grün")), ActionArgDef::number("blue", LocalizedLabel::native("Blue", "Blau"))]),
        action("fill-form-field", "Fill Form Field", "Formularfeld ausfüllen", vec![ActionArgDef::text("object", LocalizedLabel::native("Field name", "Feldname")).min_length(1).required(), ActionArgDef::text("text", LocalizedLabel::native("Value", "Wert")).min_length(0)]),
        action("replace-page", "Replace Page", "Seite ersetzen", vec![page(), ActionArgDef::text("payload", LocalizedLabel::native("PDF Data", "PDF-Daten")).min_length(0), ActionArgDef::text("extra", LocalizedLabel::native("Source Page", "Quellseite")).min_length(0), ActionArgDef::number("width", LocalizedLabel::native("Width", "Breite")), ActionArgDef::number("height", LocalizedLabel::native("Height", "Höhe"))]),
        action("replace-page-pick", "Replace Page from PDF", "Seite aus PDF ersetzen", vec![page(), ActionArgDef::text("extra", LocalizedLabel::native("Source Page", "Quellseite")).min_length(0)]),
        action("merge-document", "Merge PDF Pages", "PDF-Seiten zusammenführen", vec![page(), ActionArgDef::text("payload", LocalizedLabel::native("PDF Data", "PDF-Daten")).min_length(1).required(), ActionArgDef::text("extra", LocalizedLabel::native("Source Pages", "Quellseiten")).min_length(0)]),
        action("merge-pick", "Insert Pages from PDF", "Seiten aus PDF einfügen", vec![page(), ActionArgDef::text("extra", LocalizedLabel::native("Source Pages", "Quellseiten")).min_length(0)]),
        action("extract-pages", "Extract Pages to PDF", "Seiten als PDF extrahieren", vec![ActionArgDef::text("extra", LocalizedLabel::native("Pages (1-3,5; next file after ;)", "Seiten (1-3,5; weitere Datei nach ;)")).min_length(1).required()]),
    ]
}

/// 🖼️ Paints every page, stacked top to bottom, with one layer per object.
pub fn render_window(snapshot: &PdfSnapshot, locale: Locale) -> UiAssemblyResult<semio_ui_contract::BuiltNode> {
    render_window_with_selection(snapshot, None, locale)
}

/// 🖼️ Paints every page and outlines `selected` when the host has one.
pub fn render_window_with_selection(snapshot: &PdfSnapshot, selected: Option<&str>, locale: Locale) -> UiAssemblyResult<semio_ui_contract::BuiltNode> {
    let labels = pdf_editor_labels(locale);
    let paint = canvas_paint(snapshot, selected, locale);
    semio_plugin::scene_surface(BODY_KEY, semio_ui_contract::SurfaceKind::Canvas2d, &Canvas2dScene::base(0.0, 0.0, 1.0, paint.layers_json()), labels.canvas_name.as_str(), Some(paint.description(locale).as_str()))
}

/// 🖼️ Paints the page window from the framework-owned object selection.
pub fn render_selected(snapshot: &PdfSnapshot, interaction: &semio_plugin::app::InteractionView<'_>, locale: Locale) -> UiAssemblyResult<semio_ui_contract::BuiltNode> {
    render_window_with_selection(snapshot, interaction.selection(OBJECT_DOMAIN).ids.first().map(String::as_str), locale)
}

/// 🪟 Fields for the object selected on the page. The canvas window stays a canvas.
/// The object window owns no action of its own: every field, page button, entry form and authoring control commits a document edit verb from [`edit_action_definitions`], which the app roster declares once for every window, so a commit addressed to the focused Object pane resolves without the pane copying the page window's actions.
pub fn inspector_window_definition() -> WindowKindDefinition {
    WindowKindDefinition {
        initial_utility_id: None,
        id: INSPECTOR_WINDOW_KIND_ID.into(),
        label: LocalizedLabel::native("Object", "Objekt"),
        body_key: INSPECTOR_BODY_KEY.into(),
        surface_kind: SurfaceKind::BlockList,
        icon_id: "square".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        interactions: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}

/// 🪟 Page, the selected object's fields, and snapshot details side by side.
pub fn document_layout(main_window_kind_id: &str) -> WindowLayout {
    let stack = |size: f64, window_kind_id: &str| {
        WindowLayoutNode::Stack { size: Some(size),
            active_window_id: None,
            children: vec![WindowLayoutNode::Window { window_kind_id: window_kind_id.into(), title: None, instance_id: None, template_id: None, corner: None }],
        }
    };
    WindowLayout {
        root: WindowLayoutNode::Split { axis: Axis::Horizontal, size: None,
            children: vec![stack(0.12, THUMBNAILS_WINDOW_KIND_ID), stack(0.34, main_window_kind_id), stack(0.18, TEXT_LAYER_WINDOW_KIND_ID), stack(0.18, INSPECTOR_WINDOW_KIND_ID), stack(0.18, semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_WINDOW_KIND_ID)],
        },
    }
}

/// 🎛️ The selected page object's fields. A commit names `field` and lets the host supply `value`.
pub fn render_inspector(
    snapshot: &PdfSnapshot,
    selected: Option<&str>,
    controller_id: &str,
    publication_revision: semio_plugin::UiPublicationRevision,
    locale: Locale,
) -> UiAssemblyResult<semio_ui_contract::BuiltNode> {
    let page_index = selected.and_then(|id| id.strip_prefix('p')?.strip_suffix(":paper")?.parse::<usize>().ok()).filter(|page| *page < snapshot.pages.len());
    let object = selected.filter(|_| page_index.is_none()).and_then(|id| find_object(snapshot, id));
    let mut column = ui::column().try_id("pdf-object").map_err(|_| inspector_error("pdf.object.root-id"))?;
    let Some(object) = object else {
        let prompt = match locale {
            Locale::De => "Wählen Sie ein Objekt auf der Seite.",
            Locale::En => "Select an object on the page.",
        };
        let note = ui::text(ui::Label(UiText::clipped(prompt))).try_id("pdf-object-empty").map_err(|_| inspector_error("pdf.object.empty-id"))?.try_build().map_err(|_| inspector_error("pdf.object.empty"))?;
        column = column.try_child(note).map_err(|_| inspector_error("pdf.object.empty"))?;
        let page_index = page_index.unwrap_or(0);
        let heading = format!("{} {}", pick(locale, "Page", "Seite"), page_number(page_index));
        column = column.try_child(ui::heading(ui::Label(UiText::clipped(&heading)), 2).try_id("pdf-object-kind").map_err(|_| inspector_error("pdf.object.kind-id"))?.try_build().map_err(|_| inspector_error("pdf.object.kind"))?).map_err(|_| inspector_error("pdf.object.kind"))?;
        for field in page_fields(snapshot, page_index) {
            column = column.try_child(field_input(page_index, "", &field, controller_id, publication_revision, locale)?).map_err(|_| inspector_error("pdf.object.field"))?;
        }
        if let Some(page) = snapshot.pages.get(page_index) {
            let help = match locale {
                Locale::De => "Rahmen als Koordinaten in der Reihenfolge links, unten, rechts, oben eingeben, getrennt durch Leerzeichen. Ein leerer optionaler Rahmen verwendet den Standardwert.",
                Locale::En => "Enter box coordinates in left, bottom, right, top order, separated by spaces. Clear an optional box to use its default.",
            };
            let note = ui::text(ui::Label(UiText::clipped(help))).try_id("pdf-page-boundary-help").map_err(|_| inspector_error("pdf.object.boundary-help-id"))?.try_build().map_err(|_| inspector_error("pdf.object.boundary-help"))?;
            column = column.try_child(note).map_err(|_| inspector_error("pdf.object.boundary-help"))?;
            for field in page_boundary_fields(page) {
                column = column.try_child(field_input(page_index, "", &field, controller_id, publication_revision, locale)?).map_err(|_| inspector_error("pdf.object.field"))?;
            }
        }
        let size = snapshot.pages.get(page_index).map_or((612.0, 792.0), |page| (page.width(), page.height()));
        let label = |en: &'static str, de: &'static str| pick_label(locale, en, de);
        let mut actions = vec![("insert-page", label("Insert page after", "Seite danach einfügen"), page_index.saturating_add(1).min(snapshot.pages.len()), vec![("width", size.0), ("height", size.1)])];
        if page_index > 0 {
            actions.push(("move-page-earlier", label("Move page earlier", "Seite nach vorne"), page_index, vec![("x", page_number(page_index - 1) as f64)]));
        }
        if page_index + 1 < snapshot.pages.len() {
            actions.push(("move-page-later", label("Move page later", "Seite nach hinten"), page_index, vec![("x", page_number(page_index + 1) as f64)]));
        }
        if snapshot.pages.len() > 1 {
            actions.push(("remove-page", label("Remove page", "Seite entfernen"), page_index, Vec::new()));
        }
        for (id, label, page, args) in actions {
            let action = if id.starts_with("move-page-") { "move-page" } else { id };
            let node = ui::button(ui::Label(UiText::clipped(label))).try_id(format!("pdf-{id}")).map_err(|_| inspector_error("pdf.object.page-action-id"))?
                .try_on_with(ui::Trigger::Activate, inspector_action(controller_id, action)?, object_args(page, "", "", &args)?)
                .map_err(|_| inspector_error("pdf.object.page-action-binding"))?.try_build().map_err(|_| inspector_error("pdf.object.page-action"))?;
            column = column.try_child(node).map_err(|_| inspector_error("pdf.object.page-action"))?;
        }
        for node in entry_forms(snapshot, controller_id, publication_revision, locale)? {
            column = column.try_child(node).map_err(|_| inspector_error("pdf.object.entries"))?;
        }
        for node in authoring_controls(page_index, size, controller_id, publication_revision, locale)? {
            column = column.try_child(node).map_err(|_| inspector_error("pdf.object.authoring"))?;
        }
        return column.try_build().map_err(|_| inspector_error("pdf.object.root"));
    };
    let heading = match locale {
        Locale::De => match object.kind {
            ObjectKind::Text => "Text",
            ObjectKind::Vector => "Vektor",
            ObjectKind::Image => "Bild",
            ObjectKind::Form => "Formular",
            ObjectKind::Shading => "Verlauf",
            ObjectKind::Annotation => "Anmerkung",
        },
        Locale::En => match object.kind {
            ObjectKind::Text => "Text",
            ObjectKind::Vector => "Vector",
            ObjectKind::Image => "Image",
            ObjectKind::Form => "Form",
            ObjectKind::Shading => "Shading",
            ObjectKind::Annotation => "Annotation",
        },
    };
    column = column.try_child(ui::heading(ui::Label(UiText::clipped(heading)), 2).try_id("pdf-object-kind").map_err(|_| inspector_error("pdf.object.kind-id"))?.try_build().map_err(|_| inspector_error("pdf.object.kind"))?).map_err(|_| inspector_error("pdf.object.kind"))?;
    if object.kind == ObjectKind::Image && controller_id == "s.stdio.pdf@1.7/*#editor" {
        let button = image_replacement_button(controller_id, object.page, object.end, publication_revision.0, locale)?;
        column = column.try_child(button).map_err(|_| inspector_error("pdf.object.image-action"))?;
    }
    for field in inspector_fields(snapshot, &object) {
        column = column.try_child(field_input(object.page, &object.id, &field, controller_id, publication_revision, locale)?).map_err(|_| inspector_error("pdf.object.field"))?;
    }
    let delete_label = match locale {
        Locale::De => "Löschen",
        Locale::En => "Delete",
    };
    let delete = ui::button(ui::Label(UiText::clipped(delete_label))).try_id("pdf-object-delete").map_err(|_| inspector_error("pdf.object.delete-id"))?;
    let delete = delete.try_on_with(ui::Trigger::Activate, inspector_action(controller_id, "delete")?, object_args(object.page, &object.id, "delete", &[])?).map_err(|_| inspector_error("pdf.object.delete-binding"))?.try_build().map_err(|_| inspector_error("pdf.object.delete"))?;
    column.try_child(delete).map_err(|_| inspector_error("pdf.object.delete"))?.try_build().map_err(|_| inspector_error("pdf.object.root"))
}

//#region 🔭️ViewGlue
/// 🖼️ Paints the page window through the ephemeral local view with `overlay` layers on top.
pub fn render_view(snapshot: &PdfSnapshot, selected: Option<&str>, view: &ViewState, viewport: (f64, f64), overlay: Vec<OverlayLayer>, locale: Locale) -> UiAssemblyResult<semio_ui_contract::BuiltNode> {
    let labels = pdf_editor_labels(locale);
    let paint = canvas_paint(snapshot, selected, locale);
    semio_plugin::scene_surface(BODY_KEY, semio_ui_contract::SurfaceKind::Canvas2d, &scene_for_view(snapshot, &paint.layers_json(), view, viewport, overlay), labels.canvas_name.as_str(), Some(paint.description(locale).as_str()))
}

/// 🔎 Hits of `query` over the text of every page object.
pub fn search(snapshot: &PdfSnapshot, query: &str, match_case: bool, control: &mut SearchControl<'_>) -> SearchOutcome {
    let texts: Vec<_> = objects(snapshot).into_iter().filter(|o| o.kind == ObjectKind::Text).map(|o| SearchText { page: o.page, object: o.id, text: o.text }).collect();
    find_hits(&texts, snapshot.pages.len(), query, match_case, control)
}

/// 🖍️ Highlight layers over the objects holding `hits`.
pub fn hit_layers(snapshot: &PdfSnapshot, hits: &[SearchHit], active: Option<usize>) -> Vec<OverlayLayer> {
    let placed = placement(snapshot, &page_viewports(snapshot).collect::<Vec<_>>(), &mut FontMeasurements::new(snapshot));
    highlight_hits(hits, active, &|id| placed.iter().find(|p| p.object.id == id).map(|p| [p.canvas_x, p.canvas_y, p.canvas_w, p.canvas_h]))
}

/// 👥️ The page every object lives on, by object id.
pub fn object_page_of(snapshot: &PdfSnapshot) -> impl Fn(&str) -> Option<u32> {
    let pages: std::collections::BTreeMap<String, u32> = objects(snapshot).into_iter().map(|o| (o.id, o.page as u32)).collect();
    move |id| pages.get(id).copied().or_else(|| semio_s_artifact_stdio_pdf_presence_lane::page_of_object(id))
}

/// 👥️ Collaborator layers over the page `overlay` was projected for: selection frames, visible-area frames, carets and pointers.
pub fn peer_overlay_layers(snapshot: &PdfSnapshot, overlay: &semio_s_artifact_stdio_pdf_presence_lane::PeerOverlay) -> Vec<OverlayLayer> {
    if overlay.selections.is_empty() && overlay.pointers.is_empty() && overlay.viewports.is_empty() && overlay.carets.is_empty() {
        return Vec::new();
    }
    let Some(page) = page_viewports(snapshot).nth(overlay.page as usize).map(|viewport| viewport.clip_rect()) else { return Vec::new() };
    let placed = placement(snapshot, &page_viewports(snapshot).collect::<Vec<_>>(), &mut FontMeasurements::new(snapshot));
    semio_s_artifact_stdio_pdf_presence_lane::peer_layers(overlay, page, &|id| placed.iter().find(|p| p.object.id == id).map(|p| [p.canvas_x, p.canvas_y, p.canvas_w, p.canvas_h]))
}
//#endregion 🔭️ViewGlue
