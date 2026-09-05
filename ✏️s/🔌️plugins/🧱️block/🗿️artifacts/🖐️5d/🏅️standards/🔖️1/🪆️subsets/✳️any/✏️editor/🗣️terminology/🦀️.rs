//! 🗣️ Block 5D play app — the single `app_labels!` block plus the locale resolver every taxonomy node
//! reaches for.

//#region 🔖️Labels
semio_framework_plugin::app_labels! {
    /// 🗣️ Complete UI label set for the block-5d app; one field per label makes every locale×terminology combination compile-checked.
    pub struct Block5dLabels {
        window_board: native_en "Board", native_de "Board", reuse_en "Board", reuse_de "Board";
        window_world: native_en "World", native_de "Welt", reuse_en "World", reuse_de "Welt";
        name: native_en "Name", native_de "Name", reuse_en "Name", reuse_de "Name";
        label: native_en "Label", native_de "Bezeichnung", reuse_en "Label", reuse_de "Bezeichnung";
        grip_kinds: native_en "Grip Kinds", native_de "Griffarten", reuse_en "Grip Kinds", reuse_de "Griffarten";
        grips: native_en "Grips", native_de "Griffe", reuse_en "Grips", reuse_de "Griffe";
        no_grip_kinds: native_en "(no grip kinds)", native_de "(keine Griffarten)", reuse_en "(no grip kinds)", reuse_de "(keine Griffarten)";
        no_grips: native_en "(no grips)", native_de "(keine Griffe)", reuse_en "(no grips)", reuse_de "(keine Griffe)";
        summary: native_en "Part kind", native_de "Teilart", reuse_en "Part kind", reuse_de "Teilart";
    }
}

/// 🗣️ Resolves the active label set from `cfg.locale`; falls back to native English.
pub fn block5d_labels(locale: &str) -> &'static Block5dLabels {
    semio_framework_plugin::resolve_labels_for_locale::<Block5dLabels>(locale)
}
//#endregion 🔖️Labels

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn labels_resolve_native_english_and_german_from_the_config_locale() {
        assert_eq!(block5d_labels("en-US").summary.as_str(), "Part kind");
        assert_eq!(block5d_labels("de-DE").summary.as_str(), "Teilart");
    }
}
//#endregion 🧪️Tests
