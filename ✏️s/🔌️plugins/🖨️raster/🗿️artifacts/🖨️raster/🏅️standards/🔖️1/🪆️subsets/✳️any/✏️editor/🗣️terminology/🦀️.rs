//! 🗣️ Raster app — locale/terminology (constitutional: general). One `app_labels!` block, never split
//! (TEMPLATE.md §4).

//#region 🔖️Terminology
semio_framework_plugin::app_labels! {
    /// 🗣️ Complete UI label set for the raster app; one field per label makes every locale combination
    /// compile-checked.
    pub struct RasterPlayLabels {
        brightness: native_en "Brightness", native_de "Helligkeit", reuse_en "Brightness", reuse_de "Helligkeit";
        contrast: native_en "Contrast", native_de "Kontrast", reuse_en "Contrast", reuse_de "Kontrast";
        mask_present: native_en "Layer mask", native_de "Ebenenmaske", reuse_en "Layer mask", reuse_de "Ebenenmaske";
        mask_enabled: native_en "Enable mask", native_de "Maske aktivieren", reuse_en "Enable mask", reuse_de "Maske aktivieren";
        mask_invert: native_en "Invert mask", native_de "Maske umkehren", reuse_en "Invert mask", reuse_de "Maske umkehren";
        mask_x: native_en "Mask X position", native_de "Maskenposition X", reuse_en "Mask X position", reuse_de "Maskenposition X";
        mask_y: native_en "Mask Y position", native_de "Maskenposition Y", reuse_en "Mask Y position", reuse_de "Maskenposition Y";
        mask_scale_x: native_en "Mask horizontal scale", native_de "Horizontale Maskenskalierung", reuse_en "Mask horizontal scale", reuse_de "Horizontale Maskenskalierung";
        mask_scale_y: native_en "Mask vertical scale", native_de "Vertikale Maskenskalierung", reuse_en "Mask vertical scale", reuse_de "Vertikale Maskenskalierung";
        mask_rotation: native_en "Mask rotation (°)", native_de "Maskendrehung (°)", reuse_en "Mask rotation (°)", reuse_de "Maskendrehung (°)";
        mask_width: native_en "Mask display width", native_de "Maskenanzeigebreite", reuse_en "Mask display width", reuse_de "Maskenanzeigebreite";
        mask_height: native_en "Mask display height", native_de "Maskenanzeigehöhe", reuse_en "Mask display height", reuse_de "Maskenanzeigehöhe";
        masks: native_en "Masks", native_de "Masken", reuse_en "Masks", reuse_de "Masken";
        no_masks: native_en "No masks", native_de "Keine Masken", reuse_en "No masks", reuse_de "Keine Masken";
        mask_suffix: native_en "mask", native_de "Maske", reuse_en "mask", reuse_de "Maske";
        merge_down_hint: native_en "Combine this layer with the layer below. Both must be visible pixels or groups using Normal blend.", native_de "Diese Ebene mit der darunterliegenden vereinen. Beide müssen sichtbare Pixel- oder Gruppenebenen im normalen Mischmodus sein.", reuse_en "Combine this layer with the layer below. Both must be visible pixels or groups using Normal blend.", reuse_de "Diese Ebene mit der darunterliegenden vereinen. Beide müssen sichtbare Pixel- oder Gruppenebenen im normalen Mischmodus sein.";
        merge_down: native_en "Merge Down", native_de "Nach unten vereinen", reuse_en "Merge Down", reuse_de "Nach unten vereinen";
        flatten_image: native_en "Flatten Image", native_de "Bild reduzieren", reuse_en "Flatten Image", reuse_de "Bild reduzieren";
        flattened_image: native_en "Flattened Image", native_de "Reduziertes Bild", reuse_en "Flattened Image", reuse_de "Reduziertes Bild";
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
        blend_overlay: native_en "Overlay", native_de "Ineinanderkopieren", reuse_en "Overlay", reuse_de "Ineinanderkopieren";
        blend_color_dodge: native_en "Color dodge", native_de "Farbig abwedeln", reuse_en "Color dodge", reuse_de "Farbig abwedeln";
        blend_color_burn: native_en "Color burn", native_de "Farbig nachbelichten", reuse_en "Color burn", reuse_de "Farbig nachbelichten";
        blend_hard_light: native_en "Hard light", native_de "Hartes Licht", reuse_en "Hard light", reuse_de "Hartes Licht";
        blend_soft_light: native_en "Soft light", native_de "Weiches Licht", reuse_en "Soft light", reuse_de "Weiches Licht";
        blend_exclusion: native_en "Exclusion", native_de "Ausschluss", reuse_en "Exclusion", reuse_de "Ausschluss";
        blend_hue: native_en "Hue", native_de "Farbton", reuse_en "Hue", reuse_de "Farbton";
        blend_saturation: native_en "Saturation", native_de "Sättigung", reuse_en "Saturation", reuse_de "Sättigung";
        blend_color: native_en "Color", native_de "Farbe", reuse_en "Color", reuse_de "Farbe";
        blend_luminosity: native_en "Luminosity", native_de "Luminanz", reuse_en "Luminosity", reuse_de "Luminanz";


    }
}

/// 🗣️ Resolves the raster app's label set for a config's locale — the one call site every window/panel
/// render fn goes through.
pub fn raster_play_labels(view_state: &semio_framework_plugin::ViewModel) -> &'static RasterPlayLabels {
    semio_framework_plugin::resolve_labels::<RasterPlayLabels>(view_state)
}
//#endregion 🔖️Terminology
