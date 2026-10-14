//! 👁️ `change-layer-visible` — sets an id-addressed layer's `visible` scalar.

pub mod mutation {
    use semio_s_artifact_raster_raster_diff::RasterDiff;

    use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;

    //#region 🔖️ChangeLayerVisible
    #[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
    #[canonical_json(owner = semio_framework_pack_json)]
    #[mutation_leaf(contract = ::protocol)]
    #[value(rename_all = "camelCase")]
    pub struct ChangeLayerVisible {
        pub layer_id: String,
        pub new_visible: bool,
    }

    impl<Op> protocol::MutationKind<RasterSnapshot, Op> for ChangeLayerVisible where Op: protocol::Mutation<RasterSnapshot, Diff = semio_s_artifact_raster_raster_diff::RasterDiff> + From<crate::change_layer_visible::mutation::ChangeLayerVisible> {
        const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "layer", kind: "change-layer-visible", record: "ChangedLayerVisible" };

        fn diff(&self, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
            crate::change_layer_visible::diff::diff(self, base)
        }

        fn inverse(&self, base: &RasterSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    crate::change_layer_visible::inverse::inverse(self, base)
}

        fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
            semio_framework_ui_locale::LocalizedLabel::native(&format!("Set layer {} visible to {}", self.layer_id, self.new_visible), &format!("Sichtbarkeit von Ebene {} auf {} setzen", self.layer_id, self.new_visible))
        }

        fn target(&self) -> Vec<String> {
            vec![self.layer_id.clone()]
        }
    }
    //#endregion 🔖️ChangeLayerVisible
}

pub use mutation::ChangeLayerVisible;

//#region 🪢️TaxonomyMounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
#[cfg(test)]
#[path = "🧪️tests/🙈️hides/🦀️.rs"]
mod tests_hides_the_overlay_layer;
//#endregion 🪢️TaxonomyMounts
