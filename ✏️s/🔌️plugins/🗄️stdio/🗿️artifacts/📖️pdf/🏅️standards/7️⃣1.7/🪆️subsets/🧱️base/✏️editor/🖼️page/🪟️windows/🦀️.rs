//! 🪟️ PDF page editor windows: the page and inspector window definitions, their rendering and the document layout.

use super::*;

//#region 🪟️Windows
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
        initial_utility_id: None,
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
            action("set-annotation-border", "Set Annotation Border", "Rahmen setzen", vec![page(), object(), ActionArgDef::text("text", LocalizedLabel::native("Style", "Stil")).min_length(0), ActionArgDef::text("extra", LocalizedLabel::native("Dash", "Strich")).min_length(0), x(), y(), width()]),
            action("set-annotation-markup", "Set Annotation Markup", "Markierung setzen", vec![page(), object(), text(), ActionArgDef::text("extra", LocalizedLabel::native("Value", "Wert")).min_length(0), x()]),
            action("set-form-settings", "Set Form Settings", "Formular setzen", vec![object(), ActionArgDef::text("text", LocalizedLabel::native("Value", "Wert")).min_length(0), ActionArgDef::text("extra", LocalizedLabel::native("Key", "Schlüssel")).min_length(0), x()]),
            action("set-extra-entry", "Set Extra Entry", "Zusatzeintrag setzen", vec![object(), ActionArgDef::text("extra", LocalizedLabel::native("Key", "Schlüssel")).min_length(1).required(), ActionArgDef::text("text", LocalizedLabel::native("Value", "Wert")).min_length(0)]),
            action("set-annotation-kind", "Set Annotation Kind", "Anmerkungsart setzen", vec![page(), object(), text(), ActionArgDef::text("extra", LocalizedLabel::native("Value", "Wert")).min_length(0), x(), y(), width(), height(), red(), green(), blue()]),
            action("set-field-data", "Set Field Data", "Felddaten setzen", vec![object(), text(), ActionArgDef::text("extra", LocalizedLabel::native("Value", "Wert")).min_length(0), x()]),
            action("set-resource-detail", "Set Resource Detail", "Ressource setzen", vec![object(), text(), ActionArgDef::text("extra", LocalizedLabel::native("Value", "Wert")).min_length(0), x(), y(), width(), height(), red(), green(), blue()]),
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
pub fn render_window(snapshot: &PdfSnapshot) -> UiAssemblyResult<semio_framework_ui_contract::BuiltNode> {
    render_window_with_selection(snapshot, None)
}

/// 🖼️ Paints every page and outlines `selected` when the host has one.
pub fn render_window_with_selection(snapshot: &PdfSnapshot, selected: Option<&str>) -> UiAssemblyResult<semio_framework_ui_contract::BuiltNode> {
    semio_framework_plugin::scene_surface(BODY_KEY, semio_framework_ui_contract::SurfaceKind::Canvas2d, &Canvas2dScene::base(0.0, 0.0, 1.0, canvas_layers(snapshot, selected)))
}

/// 🖼️ Paints the page window from the framework-owned object selection.
pub fn render_selected(snapshot: &PdfSnapshot, interaction: &semio_framework_plugin::app::InteractionView<'_>) -> UiAssemblyResult<semio_framework_ui_contract::BuiltNode> {
    render_window_with_selection(snapshot, interaction.selection(OBJECT_DOMAIN).ids.first().map(String::as_str))
}

/// 🪟 Fields for the object selected on the page. The canvas window stays a canvas.
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
    let stack = |size: f64, window_kind_id: &str, title: &str| {
        WindowLayoutChild::Stack(WindowLayoutStackNode {
            kind: "stack".into(),
            size: Some(size),
            active_window_kind_id: None,
            children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: window_kind_id.into(), title: Some(title.into()), instance_id: None, template_id: None, corner: None }],
        })
    };
    WindowLayout {
        root: WindowLayoutRoot::Axis(WindowLayoutAxisNode {
            kind: "row".into(),
            size: None,
            children: vec![stack(0.56, main_window_kind_id, "Document"), stack(0.22, INSPECTOR_WINDOW_KIND_ID, "Object"), stack(0.22, semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_WINDOW_KIND_ID, "Details")],
        }),
    }
}

/// 🎛️ The selected page object's fields. A commit names `field` and lets the host supply `value`.
pub fn render_inspector(
    snapshot: &PdfSnapshot,
    selected: Option<&str>,
    controller_id: &str,
    publication_revision: semio_framework_plugin::UiPublicationRevision,
    locale: Locale,
) -> UiAssemblyResult<semio_framework_ui_contract::BuiltNode> {
    let object = selected.and_then(|id| objects(snapshot).into_iter().find(|object| object.id == id));
    let mut column = ui::column().try_id("pdf-object").map_err(|_| inspector_error("pdf.object.root-id"))?;
    let Some(object) = object else {
        let prompt = match locale {
            Locale::De => "Wählen Sie ein Objekt auf der Seite.",
            Locale::En => "Select an object on the page.",
        };
        let note = ui::text(ui::Label(UiText::clipped(prompt))).try_id("pdf-object-empty").map_err(|_| inspector_error("pdf.object.empty-id"))?.try_build().map_err(|_| inspector_error("pdf.object.empty"))?;
        column = column.try_child(note).map_err(|_| inspector_error("pdf.object.empty"))?;
        let heading = match locale {
            Locale::De => "Seite",
            Locale::En => "Page",
        };
        column = column.try_child(ui::text(ui::Label(UiText::clipped(heading))).try_id("pdf-object-kind").map_err(|_| inspector_error("pdf.object.kind-id"))?.try_build().map_err(|_| inspector_error("pdf.object.kind"))?).map_err(|_| inspector_error("pdf.object.kind"))?;
        for field in page_fields(snapshot) {
            column = column.try_child(field_input(0, "", &field, controller_id, publication_revision, locale)?).map_err(|_| inspector_error("pdf.object.field"))?;
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
    column = column.try_child(ui::text(ui::Label(UiText::clipped(heading))).try_id("pdf-object-kind").map_err(|_| inspector_error("pdf.object.kind-id"))?.try_build().map_err(|_| inspector_error("pdf.object.kind"))?).map_err(|_| inspector_error("pdf.object.kind"))?;
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
//#endregion 🪟️Windows
