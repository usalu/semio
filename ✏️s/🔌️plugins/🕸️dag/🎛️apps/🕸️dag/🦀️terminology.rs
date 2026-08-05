//! 🗣️ DAG play app — the single `app_labels!` block plus the locale resolver every taxonomy node reaches
//! for. Deliberately ONE block for the whole app (never split per window/panel): the macro's value is
//! that every locale combination is compile-checked in one place.

use crate::apps::dag::config::DagConfig;

//#region 🔖️Labels
semio_framework_plugin::app_labels! {
    /// 🗣️ Complete UI label set for the DAG app; one field per label makes every locale combination
    /// compile-checked. This app has no separate reuse-terminology concept, so the `reuse_*` cells
    /// repeat the `native_*` text verbatim.
    pub struct DagPlayLabels {
        nodes: native_en "Nodes", native_de "Knoten", reuse_en "Nodes", reuse_de "Knoten";
        edges: native_en "Edges", native_de "Kanten", reuse_en "Edges", reuse_de "Kanten";
        empty: native_en "(none)", native_de "(keine)", reuse_en "(none)", reuse_de "(keine)";
        kind_computation: native_en "Computation", native_de "Berechnung", reuse_en "Computation", reuse_de "Berechnung";
        kind_slider: native_en "Slider", native_de "Schieberegler", reuse_en "Slider", reuse_de "Schieberegler";
        kind_select: native_en "Select", native_de "Auswahl", reuse_en "Select", reuse_de "Auswahl";
        kind_note: native_en "Note", native_de "Notiz", reuse_en "Note", reuse_de "Notiz";
        kind_preview: native_en "Preview", native_de "Vorschau", reuse_en "Preview", reuse_de "Vorschau";
        kind_screen: native_en "Screen", native_de "Bildschirm", reuse_en "Screen", reuse_de "Bildschirm";
        select_a_node: native_en "Select a node in the document.", native_de "Wählen Sie einen Knoten im Dokument aus.", reuse_en "Select a node in the document.", reuse_de "Wählen Sie einen Knoten im Dokument aus.";
        node_not_found: native_en "Node not found", native_de "Knoten nicht gefunden", reuse_en "Node not found", reuse_de "Knoten nicht gefunden";
        slider_group: native_en "slider", native_de "schieberegler", reuse_en "slider", reuse_de "schieberegler";
        node_group: native_en "Node", native_de "Knoten", reuse_en "Node", reuse_de "Knoten";
        field_value: native_en "Value", native_de "Wert", reuse_en "Value", reuse_de "Wert";
        field_min: native_en "Min", native_de "Min", reuse_en "Min", reuse_de "Min";
        field_max: native_en "Max", native_de "Max", reuse_en "Max", reuse_de "Max";
        field_name: native_en "Name", native_de "Name", reuse_en "Name", reuse_de "Name";
        field_kind: native_en "Kind", native_de "Typ", reuse_en "Kind", reuse_de "Typ";
        field_id: native_en "Id", native_de "Id", reuse_en "Id", reuse_de "Id";
        selected_suffix: native_en "selected", native_de "ausgewählt", reuse_en "selected", reuse_de "ausgewählt";
        delete_selection: native_en "Delete selection", native_de "Auswahl löschen", reuse_en "Delete selection", reuse_de "Auswahl löschen";
    }
}
//#endregion 🔖️Labels

//#region 🔖️Resolvers
/// 🗣️ `cfg.locale`-driven counterpart to the deleted `ViewState`-driven locale read.
pub fn is_de_locale(cfg: &DagConfig) -> bool {
    cfg.locale.starts_with("de")
}

/// 🗣️ Derives the compile-time-checked `Locale` from the BCP-47 `cfg.locale` tag.
pub fn dag_locale(cfg: &DagConfig) -> semio_framework_plugin::Locale {
    if is_de_locale(cfg) {
        semio_framework_plugin::Locale::De
    } else {
        semio_framework_plugin::Locale::En
    }
}

/// 🗣️ Resolves the active label set from `cfg.locale`; this app has no terminology variant, so
/// `Terminology` is always `Native`.
pub fn dag_play_labels(cfg: &DagConfig) -> &'static DagPlayLabels {
    DagPlayLabels::labels(dag_locale(cfg), semio_framework_plugin::Terminology::Native)
}
//#endregion 🔖️Resolvers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn labels_resolve_native_english_and_german_from_the_config_locale() {
        assert_eq!(dag_play_labels(&DagConfig::default()).nodes.as_str(), "Nodes");
        assert_eq!(dag_play_labels(&DagConfig { locale: "de-DE".into(), ..DagConfig::default() }).nodes.as_str(), "Knoten");
    }

    #[test]
    fn is_de_locale_matches_any_de_prefixed_tag() {
        assert!(is_de_locale(&DagConfig { locale: "de".into(), ..DagConfig::default() }));
        assert!(is_de_locale(&DagConfig { locale: "de-DE".into(), ..DagConfig::default() }));
        assert!(!is_de_locale(&DagConfig { locale: "en-US".into(), ..DagConfig::default() }));
    }
}
//#endregion 🧪️Tests
