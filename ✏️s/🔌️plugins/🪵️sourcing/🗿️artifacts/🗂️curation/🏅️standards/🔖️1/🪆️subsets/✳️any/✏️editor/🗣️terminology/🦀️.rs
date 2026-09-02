//! 🗣️ Sourcing curation app — the single `app_labels!` block plus the locale resolver every taxonomy node
//! reaches for. Deliberately ONE block for the whole app (never split per window): the macro's value is
//! that every locale combination is compile-checked in one place.

use crate::editor::sourcing::config::SourcingCurationConfig;

//#region 🔖️Labels
semio_framework_plugin::app_labels! {
    /// 🗣️ Complete UI label set for the curation app; one field per label makes every locale combination compile-checked.
    pub struct SourcingLabels {
        window_pool: native_en "Pool", native_de "Pool", reuse_en "Pool", reuse_de "Pool";
        window_curated: native_en "Curated", native_de "Kuratiert", reuse_en "Curated", reuse_de "Kuratiert";
        window_preview: native_en "Preview", native_de "Vorschau", reuse_en "Preview", reuse_de "Vorschau";
        window_grid: native_en "Grid", native_de "Raster", reuse_en "Grid", reuse_de "Raster";
        mode_curation: native_en "Curation", native_de "Kuratierung", reuse_en "Curation", reuse_de "Kuratierung";
        search_placeholder: native_en "Search…", native_de "Suchen…", reuse_en "Search…", reuse_de "Suchen…";
        all_typologies: native_en "All Typologies", native_de "Alle Typologien", reuse_en "All Typologies", reuse_de "Alle Typologien";
        col_name: native_en "Name", native_de "Name", reuse_en "Name", reuse_de "Name";
        col_module: native_en "Module", native_de "Modul", reuse_en "Module", reuse_de "Modul";
        col_typology: native_en "Typology", native_de "Typologie", reuse_en "Typology", reuse_de "Typologie";
        col_availability: native_en "Availability", native_de "Verfügbarkeit", reuse_en "Availability", reuse_de "Verfügbarkeit";
        col_curated: native_en "Curated", native_de "Kuratiert", reuse_en "Curated", reuse_de "Kuratiert";
        col_count: native_en "Count", native_de "Anzahl", reuse_en "Count", reuse_de "Anzahl";
        remove: native_en "Remove", native_de "Entfernen", reuse_en "Remove", reuse_de "Entfernen";
        no_selection: native_en "No selection", native_de "Keine Auswahl", reuse_en "No selection", reuse_de "Keine Auswahl";
    }
}
//#endregion 🔖️Labels

//#region 🔖️Resolvers
/// 🗣️ Resolves the active label set from `cfg.locale`; falls back to native English.
pub fn sourcing_curation_labels(cfg: &SourcingCurationConfig) -> &'static SourcingLabels {
    semio_framework_plugin::resolve_labels_for_locale::<SourcingLabels>(&cfg.locale)
}
//#endregion 🔖️Resolvers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn labels_resolve_native_english_and_german_from_the_config_locale() {
        assert_eq!(sourcing_curation_labels(&SourcingCurationConfig::default()).window_pool.as_str(), "Pool");
        assert_eq!(sourcing_curation_labels(&SourcingCurationConfig { locale: "de-DE".into(), ..SourcingCurationConfig::default() }).col_curated.as_str(), "Kuratiert");
    }
}
//#endregion 🧪️Tests
