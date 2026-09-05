//! 🗣️ Block 2D play app — the single `app_labels!` block plus the locale resolver every taxonomy node
//! reaches for.

//#region 🔖️Labels
semio_framework_plugin::app_labels! {
    /// 🗣️ Complete UI label set for the block-2d app; one field per label makes every locale×terminology combination compile-checked.
    pub struct Block2dLabels {
        window_board: native_en "Node Kind", native_de "Knotenart", reuse_en "Node Kind", reuse_de "Knotenart";
        name: native_en "Name", native_de "Name", reuse_en "Name", reuse_de "Name";
        label: native_en "Label", native_de "Bezeichnung", reuse_en "Label", reuse_de "Bezeichnung";
        variant: native_en "Variant", native_de "Variante", reuse_en "Variant", reuse_de "Variante";
        description: native_en "Description", native_de "Beschreibung", reuse_en "Description", reuse_de "Beschreibung";
        handle_kinds: native_en "Handle Kinds", native_de "Griffarten", reuse_en "Handle Kinds", reuse_de "Griffarten";
        handles: native_en "Handles", native_de "Griffe", reuse_en "Handles", reuse_de "Griffe";
        no_handle_kinds: native_en "(no handle kinds)", native_de "(keine Griffarten)", reuse_en "(no handle kinds)", reuse_de "(keine Griffarten)";
        no_handles: native_en "(no handles)", native_de "(keine Griffe)", reuse_en "(no handles)", reuse_de "(keine Griffe)";
        summary: native_en "Node kind", native_de "Knotenart", reuse_en "Node kind", reuse_de "Knotenart";
    }
}

/// 🗣️ Resolves the active label set from `cfg.locale`; falls back to native English.
pub fn block2d_labels(locale: &str) -> &'static Block2dLabels {
    semio_framework_plugin::resolve_labels_for_locale::<Block2dLabels>(locale)
}
//#endregion 🔖️Labels

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn labels_resolve_native_english_and_german_from_the_config_locale() {
        assert_eq!(block2d_labels("en-US").summary.as_str(), "Node kind");
        assert_eq!(block2d_labels("de-DE").summary.as_str(), "Knotenart");
    }
}
//#endregion 🧪️Tests
