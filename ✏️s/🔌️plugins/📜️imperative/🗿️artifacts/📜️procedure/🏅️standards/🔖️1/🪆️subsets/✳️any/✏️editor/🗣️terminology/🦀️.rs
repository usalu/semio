//! 🗣️ Imperative play app — the single `app_labels!` block plus the locale resolver every taxonomy node
//! reaches for. Deliberately ONE block for the whole app (never split per window/panel): the macro's
//! value is that every locale×terminology combination is compile-checked in one place. No separate
//! reuse-terminology concept (pure control-flow vocabulary), so `reuse_*` mirrors `native_*` throughout —
//! `ImperativeConfig` carries no terminology axis.

use crate::editor::procedure::config::ImperativeConfig;

//#region 🔖️Labels
semio_framework_plugin::app_labels! {
    /// 🗣️ Complete UI label set for the imperative app; one field per label makes every locale
    /// combination compile-checked.
    pub struct ImperativeLabels {
        window_main: native_en "Imperative", native_de "Imperativ", reuse_en "Imperative", reuse_de "Imperativ";
        window_script: native_en "Script", native_de "Skript", reuse_en "Script", reuse_de "Skript";
        col_index: native_en "#", native_de "#", reuse_en "#", reuse_de "#";
        col_id: native_en "Id", native_de "ID", reuse_en "Id", reuse_de "ID";
        col_kind: native_en "Kind", native_de "Art", reuse_en "Kind", reuse_de "Art";
        action_state_set: native_en "Set state", native_de "Zustand setzen", reuse_en "Set state", reuse_de "Zustand setzen";
        action_log_print: native_en "Print log", native_de "Log ausgeben", reuse_en "Print log", reuse_de "Log ausgeben";
        action_control_if: native_en "If", native_de "Wenn", reuse_en "If", reuse_de "Wenn";
        action_control_while: native_en "While", native_de "Solange", reuse_en "While", reuse_de "Solange";
        action_math_add: native_en "Add", native_de "Addieren", reuse_en "Add", reuse_de "Addieren";
        document_empty: native_en "(none)", native_de "(keine)", reuse_en "(none)", reuse_de "(keine)";
        inspector_steps: native_en "Steps", native_de "Schritte", reuse_en "Steps", reuse_de "Schritte";
    }
}
//#endregion 🔖️Labels

//#region 🔖️Resolvers
/// 🗣️ Resolves the active label set from `cfg.locale`; falls back to native English.
pub fn imperative_labels(cfg: &ImperativeConfig) -> &'static ImperativeLabels {
    semio_framework_plugin::resolve_labels_for_locale::<ImperativeLabels>(&cfg.locale)
}
//#endregion 🔖️Resolvers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn labels_resolve_native_english_and_german_from_the_config_locale() {
        assert_eq!(imperative_labels(&ImperativeConfig::default()).action_control_while.as_str(), "While");
        assert_eq!(imperative_labels(&ImperativeConfig { locale: "de-DE".into(), ..ImperativeConfig::default() }).action_control_while.as_str(), "Solange");
    }
}
//#endregion 🧪️Tests
