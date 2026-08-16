//! 🗣️ Block 3D play app — the single `app_labels!` block plus the locale resolvers every taxonomy node
//! reaches for.

use crate::editor::block3d::config::Block3dConfig;
use semio_framework_plugin::{AppLabels, Locale, Terminology};

//#region 🔖️Labels
// 🗣️ Complete UI label set for the block3d-play app; one field per label makes every locale
// combination compile-checked. No separate reuse-terminology concept, so reuse repeats native.
semio_framework_plugin::app_labels! {
    pub struct Block3dLabels {
        window_world: native_en "Object Kind", native_de "Objektart", reuse_en "Object Kind", reuse_de "Objektart";
        name: native_en "Name", native_de "Name", reuse_en "Name", reuse_de "Name";
        label: native_en "Label", native_de "Bezeichnung", reuse_en "Label", reuse_de "Bezeichnung";
        representation: native_en "Representation", native_de "Darstellung", reuse_en "Representation", reuse_de "Darstellung";
        representations: native_en "Representations", native_de "Darstellungen", reuse_en "Representations", reuse_de "Darstellungen";
        vortex_kinds: native_en "Vortex Kinds", native_de "Wirbelarten", reuse_en "Vortex Kinds", reuse_de "Wirbelarten";
        vortices: native_en "Vortices", native_de "Wirbel", reuse_en "Vortices", reuse_de "Wirbel";
        no_representations: native_en "(no representations)", native_de "(keine Darstellungen)", reuse_en "(no representations)", reuse_de "(keine Darstellungen)";
        no_vortices: native_en "(no vortices)", native_de "(keine Wirbel)", reuse_en "(no vortices)", reuse_de "(keine Wirbel)";
        summary: native_en "Object kind", native_de "Objektart", reuse_en "Object kind", reuse_de "Objektart";
        arrangement: native_en "Arrangement", native_de "Anordnung", reuse_en "Arrangement", reuse_de "Anordnung";
        spacing: native_en "Spacing", native_de "Abstand", reuse_en "Spacing", reuse_de "Abstand";
        brush: native_en "Surface brush", native_de "Flächenpinsel", reuse_en "Surface brush", reuse_de "Flächenpinsel";
        brush_radius: native_en "Radius", native_de "Radius", reuse_en "Radius", reuse_de "Radius";
        flip_normal: native_en "Flip normal", native_de "Normale umkehren", reuse_en "Flip normal", reuse_de "Normale umkehren";
        show_all: native_en "All representations", native_de "Alle Darstellungen", reuse_en "All representations", reuse_de "Alle Darstellungen";
    }
}

/// 🗣️ B1: `cfg.locale`-driven counterpart to the deleted `ViewModel`-driven resolver.
fn block3d_is_de_locale(cfg: &Block3dConfig) -> bool {
    cfg.locale.starts_with("de")
}

fn block3d_locale(cfg: &Block3dConfig) -> Locale {
    if block3d_is_de_locale(cfg) {
        Locale::De
    } else {
        Locale::En
    }
}

/// 🗣️ Resolves the active `Block3dLabels` cell from the config-carried locale. `Block3dConfig` carries
/// no terminology field, so terminology is always `Native`.
pub fn block3d_labels(cfg: &Block3dConfig) -> &'static Block3dLabels {
    Block3dLabels::labels(block3d_locale(cfg), Terminology::Native)
}
//#endregion 🔖️Labels

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn labels_resolve_native_english_and_german_from_the_config_locale() {
        assert_eq!(block3d_labels(&Block3dConfig::default()).summary.as_str(), "Object kind");
        assert_eq!(block3d_labels(&Block3dConfig { locale: "de-DE".into(), ..Block3dConfig::default() }).summary.as_str(), "Objektart");
    }
}
//#endregion 🧪️Tests
