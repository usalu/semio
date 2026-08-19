//! 🚪️ S Space index artifact — IO facet. No stdio import/export composer registered this wave (the
//! index is a hub-shared control-plane document, not a file a user imports/exports directly) — this
//! facet exists to satisfy the taxonomy's `subsetChildDirs`/`subsetComponentDirs` shape and is a
//! deliberate placeholder for a follow-up wave, flagged in `$T/📓️w1-e-report.md`.

//#region 🔖️IoRegistry
/// 📭️ No composer entries registered this wave.
pub async fn io_registry_entries() -> &'static [semio_framework_plugin::ComposerEntry] {
    &[]
}
//#endregion 🔖️IoRegistry
