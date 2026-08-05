//! 🗣️ Draw play app — the complete UI label set (constitutional: was `ui`'s Locale/Terminology region).

semio_framework_plugin::app_labels! {
    /// 🗣️ Complete UI label set for the draw app; one field per label makes every locale combination compile-checked.
    pub struct DrawPlayLabels {
        window_canvas: native_en "Canvas", native_de "Leinwand", reuse_en "Canvas", reuse_de "Leinwand";
        mode_edit: native_en "Edit", native_de "Bearbeiten", reuse_en "Edit", reuse_de "Bearbeiten";
        add_path: native_en "Add Path", native_de "Pfad hinzufügen", reuse_en "Add Path", reuse_de "Pfad hinzufügen";
        add_rectangle: native_en "Add Rectangle", native_de "Rechteck hinzufügen", reuse_en "Add Rectangle", reuse_de "Rechteck hinzufügen";
        add_text: native_en "Add Text", native_de "Text hinzufügen", reuse_en "Add Text", reuse_de "Text hinzufügen";
        add_group: native_en "Add Group", native_de "Gruppe hinzufügen", reuse_en "Add Group", reuse_de "Gruppe hinzufügen";
        add_boolean: native_en "Add Boolean", native_de "Boolean hinzufügen", reuse_en "Add Boolean", reuse_de "Boolean hinzufügen";
        empty_state: native_en "Drop layers here", native_de "Ebenen hier ablegen", reuse_en "Drop layers here", reuse_de "Ebenen hier ablegen";
        kind_path: native_en "Path", native_de "Pfad", reuse_en "Path", reuse_de "Pfad";
        kind_rectangle: native_en "Rectangle", native_de "Rechteck", reuse_en "Rectangle", reuse_de "Rechteck";
        kind_ellipse: native_en "Ellipse", native_de "Ellipse", reuse_en "Ellipse", reuse_de "Ellipse";
        kind_line: native_en "Line", native_de "Linie", reuse_en "Line", reuse_de "Linie";
        kind_polygon: native_en "Polygon", native_de "Polygon", reuse_en "Polygon", reuse_de "Polygon";
        kind_text: native_en "Text", native_de "Text", reuse_en "Text", reuse_de "Text";
        kind_image: native_en "Image", native_de "Bild", reuse_en "Image", reuse_de "Bild";
        kind_group: native_en "Group", native_de "Gruppe", reuse_en "Group", reuse_de "Gruppe";
        kind_boolean: native_en "Boolean", native_de "Boolean", reuse_en "Boolean", reuse_de "Boolean";
        kind_trace: native_en "Trace", native_de "Nachzeichnung", reuse_en "Trace", reuse_de "Nachzeichnung";
        boolean_operation: native_en "Boolean Operation", native_de "Boolean-Operation", reuse_en "Boolean Operation", reuse_de "Boolean-Operation";
        children: native_en "Children", native_de "Kinder", reuse_en "Children", reuse_de "Kinder";
        trace_threshold: native_en "Trace Threshold", native_de "Trace-Schwellenwert", reuse_en "Trace Threshold", reuse_de "Trace-Schwellenwert";
        simplify: native_en "Simplify", native_de "Vereinfachen", reuse_en "Simplify", reuse_de "Vereinfachen";
        source_key: native_en "Source Key", native_de "Quellschlüssel", reuse_en "Source Key", reuse_de "Quellschlüssel";
        width: native_en "Width", native_de "Breite", reuse_en "Width", reuse_de "Breite";
        height: native_en "Height", native_de "Höhe", reuse_en "Height", reuse_de "Höhe";
        content: native_en "Content", native_de "Inhalt", reuse_en "Content", reuse_de "Inhalt";
        size: native_en "Size", native_de "Größe", reuse_en "Size", reuse_de "Größe";
        segment_count: native_en "Segment Count", native_de "Segmentanzahl", reuse_en "Segment Count", reuse_de "Segmentanzahl";
        children_count: native_en "Children Count", native_de "Kinderanzahl", reuse_en "Children Count", reuse_de "Kinderanzahl";
        appearance: native_en "Appearance", native_de "Erscheinungsbild", reuse_en "Appearance", reuse_de "Erscheinungsbild";
        fill: native_en "Fill", native_de "Füllung", reuse_en "Fill", reuse_de "Füllung";
        fill_alpha: native_en "Fill Alpha", native_de "Füllung Alpha", reuse_en "Fill Alpha", reuse_de "Füllung Alpha";
        stroke_width: native_en "Stroke Width", native_de "Strichstärke", reuse_en "Stroke Width", reuse_de "Strichstärke";
        layer: native_en "Layer", native_de "Ebene", reuse_en "Layer", reuse_de "Ebene";
        name: native_en "Name", native_de "Name", reuse_en "Name", reuse_de "Name";
        opacity: native_en "Opacity", native_de "Deckkraft", reuse_en "Opacity", reuse_de "Deckkraft";
        blend_mode: native_en "Blend Mode", native_de "Mischmodus", reuse_en "Blend Mode", reuse_de "Mischmodus";
        visible: native_en "Visible", native_de "Sichtbar", reuse_en "Visible", reuse_de "Sichtbar";
        locked: native_en "Locked", native_de "Gesperrt", reuse_en "Locked", reuse_de "Gesperrt";
        orientation: native_en "Orientation", native_de "Ausrichtung", reuse_en "Orientation", reuse_de "Ausrichtung";
        position_x: native_en "Position X", native_de "Position X", reuse_en "Position X", reuse_de "Position X";
        position_y: native_en "Position Y", native_de "Position Y", reuse_en "Position Y", reuse_de "Position Y";
        scale_x: native_en "Scale X", native_de "Skalierung X", reuse_en "Scale X", reuse_de "Skalierung X";
        scale_y: native_en "Scale Y", native_de "Skalierung Y", reuse_en "Scale Y", reuse_de "Skalierung Y";
        rotation: native_en "Rotation", native_de "Rotation", reuse_en "Rotation", reuse_de "Rotation";
    }
}
