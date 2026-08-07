//! 🧩️ Sourcing slabs module — contributes the slabs typology and demo catalogue kinds to the sourcing app.

use semio_framework::Contribution;
use semio_framework_plugin::ExtensionBundle;
use sourcing_curate::artifacts::curate::engine::{slabs::SlabsModule, SourcingModule};

//#region 🔖️Bundle
const EXTENSION_ID: &str = "sourcing-module-slabs";
const HOST_APP_ID: &str = "sourcing-curate";

fn bundle() -> ExtensionBundle {
    let module = SlabsModule;
    ExtensionBundle::new(EXTENSION_ID, "Sourcing Module Slabs", "0.1.0")
        .extends("sourcing")
        .contributes(Contribution::SourcingModule {
            app_id: HOST_APP_ID.into(),
            module_id: module.module_id().into(),
            label: module.label().into(),
            icon_id: "slab".into(),
            typology_json: serde_json::to_string(&module.typology()).unwrap_or_default(),
            kinds_json: serde_json::to_string(&module.demo_kinds()).unwrap_or_default(),
        })
}

semio_framework_plugin::extension_exports!(bundle);
//#endregion 🔖️Bundle

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bundle_contributes_module_for_sourcing_curate() {
        let manifest = bundle().manifest;
        assert_eq!(manifest.extension_id, EXTENSION_ID);
        assert_eq!(manifest.extends, "sourcing");
        assert_eq!(manifest.contributions.len(), 1);
        let Contribution::SourcingModule { app_id, module_id, typology_json, kinds_json, .. } = &manifest.contributions[0] else {
            panic!("expected a SourcingModule contribution");
        };
        assert_eq!(app_id, HOST_APP_ID);
        assert_eq!(module_id, "slabs");
        assert!(serde_json::from_str::<sourcing_curate::artifacts::curate::engine::TypologyNode>(typology_json).is_ok());
        assert!(serde_json::from_str::<Vec<sourcing_curate::artifacts::curate::ObjectKind>>(kinds_json).is_ok());
    }
}
//#endregion 🔖️Tests
