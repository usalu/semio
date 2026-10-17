//! 🎨 `change-layer-blend-mode` — sets an id-addressed layer's `blend_mode` scalar.

pub mod mutation {
    use semio_s_artifact_raster_raster_diff::RasterDiff;

    use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;

    //#region 🔖️ChangeLayerBlendMode
    #[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
    #[canonical_json(owner = semio_framework_pack_json)]
    #[mutation_leaf(contract = ::protocol)]
    #[value(rename_all = "camelCase")]
    pub struct ChangeLayerBlendMode {
        pub layer_id: String,
        pub new_blend_mode: String,
    }

    impl<Op> protocol::MutationKind<RasterSnapshot, Op> for ChangeLayerBlendMode where Op: protocol::Mutation<RasterSnapshot, Diff = semio_s_artifact_raster_raster_diff::RasterDiff> + From<crate::change_layer_blend_mode::mutation::ChangeLayerBlendMode> {
        const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "layer", kind: "change-layer-blend-mode", record: "ChangedLayerBlendMode" };

        fn diff(&self, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
            crate::change_layer_blend_mode::diff::diff(self, base)
        }

        fn inverse(&self, base: &RasterSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    crate::change_layer_blend_mode::inverse::inverse(self, base)
}

        fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
            semio_framework_ui_locale::LocalizedLabel::native(&format!("Set layer {} blend mode to {}", self.layer_id, self.new_blend_mode), &format!("Füllmethode von Ebene {} auf {} setzen", self.layer_id, self.new_blend_mode))
        }

        fn target(&self) -> Vec<String> {
            vec![self.layer_id.clone()]
        }
    }
    //#endregion 🔖️ChangeLayerBlendMode
}

pub use mutation::ChangeLayerBlendMode;

//#region 🪢️TaxonomyMounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️TaxonomyMounts
