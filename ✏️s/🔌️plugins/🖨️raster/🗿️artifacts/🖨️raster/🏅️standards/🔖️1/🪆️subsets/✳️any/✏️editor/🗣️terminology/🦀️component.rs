//! 🗣️ Raster app — locale/terminology (constitutional: general). One `app_labels!` block, never split
//! (TEMPLATE.md §4).

use crate::editor::raster::config::RasterConfig;

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
    }
}

/// 🗣️ Resolves the raster app's label set for a config's locale — the one call site every window/panel
/// render fn goes through.
pub async fn raster_play_labels(cfg: &RasterConfig) -> &'static RasterPlayLabels {
    semio_framework_plugin::resolve_labels_for_locale::<RasterPlayLabels>(&cfg.locale)
}
//#endregion 🔖️Terminology
