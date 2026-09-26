//! 🗣️ Raster app — locale/terminology (constitutional: general). One `app_labels!` block, never split
//! (TEMPLATE.md §4).

//#region 🔖️Terminology
semio_framework_plugin::app_labels! {
    /// 🗣️ Complete UI label set for the raster app; one field per label makes every locale combination
    /// compile-checked.
    pub struct RasterPlayLabels {
        masks: native_en "Masks", native_de "Masken", reuse_en "Masks", reuse_de "Masken";
        no_masks: native_en "No masks", native_de "Keine Masken", reuse_en "No masks", reuse_de "Keine Masken";
        mask_suffix: native_en "mask", native_de "Maske", reuse_en "mask", reuse_de "Maske";
        add_pixel: native_en "Add Pixel", native_de "Pixel hinzufügen", reuse_en "Add Pixel", reuse_de "Pixel hinzufügen";
        add_group: native_en "Add Group", native_de "Gruppe hinzufügen", reuse_en "Add Group", reuse_de "Gruppe hinzufügen";
        layer_kinds: native_en "Layer kinds", native_de "Ebenenarten", reuse_en "Layer kinds", reuse_de "Ebenenarten";
        layer: native_en "Layer", native_de "Ebene", reuse_en "Layer", reuse_de "Ebene";
        pixel_layer: native_en "Pixel layer", native_de "Pixelebene", reuse_en "Pixel layer", reuse_de "Pixelebene";
        group_layer: native_en "Group", native_de "Gruppe", reuse_en "Group", reuse_de "Gruppe";
        adjustment_layer: native_en "Adjustment", native_de "Anpassung", reuse_en "Adjustment", reuse_de "Anpassung";
        catalogue_pixel: native_en "pixel — paintable bitmap layer", native_de "pixel — bearbeitbare Bitmap-Ebene", reuse_en "pixel — paintable bitmap layer", reuse_de "pixel — bearbeitbare Bitmap-Ebene";
        catalogue_group: native_en "group — nested layer stack", native_de "group — verschachtelter Ebenenstapel", reuse_en "group — nested layer stack", reuse_de "group — verschachtelter Ebenenstapel";
        catalogue_adjustment: native_en "adjustment — non-destructive filter", native_de "adjustment — zerstörungsfreier Filter", reuse_en "adjustment — non-destructive filter", reuse_de "adjustment — zerstörungsfreier Filter";
        window_composite: native_en "Composite", native_de "Komposit", reuse_en "Composite", reuse_de "Komposit";
        window_navigator: native_en "Navigator", native_de "Navigator", reuse_en "Navigator", reuse_de "Navigator";
        name: native_en "Name", native_de "Name", reuse_en "Name", reuse_de "Name";
        opacity: native_en "Opacity", native_de "Deckkraft", reuse_en "Opacity", reuse_de "Deckkraft";
        mixed: native_en "Mixed", native_de "Gemischt", reuse_en "Mixed", reuse_de "Gemischt";
        schema_prefix: native_en "Schema", native_de "Schema", reuse_en "Schema", reuse_de "Schema";
        brush_prefix: native_en "Brush", native_de "Pinsel", reuse_en "Brush", reuse_de "Pinsel";
        eraser: native_en "Eraser", native_de "Radiergummi", reuse_en "Eraser", reuse_de "Radiergummi";
        brush_size: native_en "Size", native_de "Größe", reuse_en "Size", reuse_de "Größe";
        brush_hardness: native_en "Hardness", native_de "Härte", reuse_en "Hardness", reuse_de "Härte";
        foreground: native_en "Foreground", native_de "Vordergrund", reuse_en "Foreground", reuse_de "Vordergrund";
        inspection: native_en "Inspection", native_de "Inspektion", reuse_en "Inspection", reuse_de "Inspektion";
        visible: native_en "Visible", native_de "Sichtbar", reuse_en "Visible", reuse_de "Sichtbar";
        blend_mode: native_en "Blend mode", native_de "Mischmodus", reuse_en "Blend mode", reuse_de "Mischmodus";
        position_x: native_en "X position", native_de "X-Position", reuse_en "X position", reuse_de "X-Position";
        position_y: native_en "Y position", native_de "Y-Position", reuse_en "Y position", reuse_de "Y-Position";
        width: native_en "Display width", native_de "Anzeigebreite", reuse_en "Display width", reuse_de "Anzeigebreite";
        height: native_en "Display height", native_de "Anzeigehöhe", reuse_en "Display height", reuse_de "Anzeigehöhe";
        blend_normal: native_en "Normal", native_de "Normal", reuse_en "Normal", reuse_de "Normal";
        blend_multiply: native_en "Multiply", native_de "Multiplizieren", reuse_en "Multiply", reuse_de "Multiplizieren";
        blend_screen: native_en "Screen", native_de "Negativ multiplizieren", reuse_en "Screen", reuse_de "Negativ multiplizieren";
        blend_darken: native_en "Darken", native_de "Abdunkeln", reuse_en "Darken", reuse_de "Abdunkeln";
        blend_lighten: native_en "Lighten", native_de "Aufhellen", reuse_en "Lighten", reuse_de "Aufhellen";
        blend_difference: native_en "Difference", native_de "Differenz", reuse_en "Difference", reuse_de "Differenz";


    }
}

/// 🗣️ Resolves the raster app's label set for a config's locale — the one call site every window/panel
/// render fn goes through.
pub fn raster_play_labels(view_state: &semio_framework_plugin::ViewModel) -> &'static RasterPlayLabels {
    semio_framework_plugin::resolve_labels::<RasterPlayLabels>(view_state)
}
//#endregion 🔖️Terminology
