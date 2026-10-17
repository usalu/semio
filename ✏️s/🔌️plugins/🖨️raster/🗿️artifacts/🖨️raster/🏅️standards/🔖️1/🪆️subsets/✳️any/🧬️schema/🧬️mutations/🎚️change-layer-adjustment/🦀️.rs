//! 🎚️ `change-layer-adjustment-kind` — changes an id-addressed `Adjustment` layer's
//! `adjustment_kind` scalar. Only meaningful on the `Adjustment` variant; addressing a
//! `Pixel`/`Group` layer is a graceful no-op.

pub mod mutation {
    use semio_s_artifact_raster_raster_diff::RasterDiff;

    use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;

    //#region 🔖️ChangeLayerAdjustmentKind
    #[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
    #[canonical_json(owner = semio_framework_pack_json)]
    #[mutation_leaf(contract = ::protocol)]
    #[value(rename_all = "camelCase")]
    pub struct ChangeLayerAdjustmentKind {
        pub layer_id: String,
        pub new_adjustment_kind: String,
    }

    impl<Op> protocol::MutationKind<RasterSnapshot, Op> for ChangeLayerAdjustmentKind where Op: protocol::Mutation<RasterSnapshot, Diff = semio_s_artifact_raster_raster_diff::RasterDiff> + From<crate::change_layer_adjustment_kind::mutation::ChangeLayerAdjustmentKind> {
        const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "layer", kind: "change-layer-adjustment-kind", record: "ChangedLayerAdjustmentKind" };

        fn diff(&self, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
            crate::change_layer_adjustment_kind::diff::diff(self, base)
        }

        fn inverse(&self, base: &RasterSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    crate::change_layer_adjustment_kind::inverse::inverse(self, base)
}

        fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
            semio_framework_ui_locale::LocalizedLabel::native(&format!("Set layer {} adjustment kind to {}", self.layer_id, self.new_adjustment_kind), &format!("Korrekturart von Ebene {} auf {} setzen", self.layer_id, self.new_adjustment_kind))
        }

        fn target(&self) -> Vec<String> {
            vec![self.layer_id.clone()]
        }
    }
    //#endregion 🔖️ChangeLayerAdjustmentKind
}

pub use mutation::ChangeLayerAdjustmentKind;

//#region 🪢️TaxonomyMounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️TaxonomyMounts
