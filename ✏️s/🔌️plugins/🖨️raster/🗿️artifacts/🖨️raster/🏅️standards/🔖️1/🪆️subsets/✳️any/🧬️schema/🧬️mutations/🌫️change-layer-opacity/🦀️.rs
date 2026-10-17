//! 🌫️ `change-layer-opacity` — sets an id-addressed layer's `opacity` scalar.

pub mod mutation {
    use semio_s_artifact_raster_raster_diff::RasterDiff;

    use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;

    //#region 🔖️ChangeLayerOpacity
    #[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
    #[canonical_json(owner = semio_framework_pack_json)]
    #[mutation_leaf(contract = ::protocol)]
    #[value(rename_all = "camelCase")]
    pub struct ChangeLayerOpacity {
        pub layer_id: String,
        pub new_opacity: f32,
    }

    impl<Op> protocol::MutationKind<RasterSnapshot, Op> for ChangeLayerOpacity where Op: protocol::Mutation<RasterSnapshot, Diff = semio_s_artifact_raster_raster_diff::RasterDiff> + From<crate::change_layer_opacity::mutation::ChangeLayerOpacity> {
        const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "layer", kind: "change-layer-opacity", record: "ChangedLayerOpacity" };

        fn diff(&self, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
            crate::change_layer_opacity::diff::diff(self, base)
        }

        fn inverse(&self, base: &RasterSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    crate::change_layer_opacity::inverse::inverse(self, base)
}

        fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
            semio_framework_ui_locale::LocalizedLabel::native(&format!("Set layer {} opacity to {}", self.layer_id, self.new_opacity), &format!("Deckkraft von Ebene {} auf {} setzen", self.layer_id, self.new_opacity))
        }

        fn target(&self) -> Vec<String> {
            vec![self.layer_id.clone()]
        }
    }
    //#endregion 🔖️ChangeLayerOpacity
}

pub use mutation::ChangeLayerOpacity;

//#region 🪢️TaxonomyMounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️TaxonomyMounts
