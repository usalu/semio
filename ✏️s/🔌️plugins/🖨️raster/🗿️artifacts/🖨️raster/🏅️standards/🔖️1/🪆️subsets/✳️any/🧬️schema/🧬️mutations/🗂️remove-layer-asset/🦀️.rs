//! 🗂️ `remove-layer-asset` — detaches an id-addressed `RasterImageAsset` from the document's asset
//! map. Inverse partner of `add-layer-asset` (see that leaf for why this pair exists).

pub mod mutation {
    use semio_s_artifact_raster_raster_diff::RasterDiff;

    use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;

    //#region 🔖️RemoveLayerAsset
    #[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::NativeReceive)]
    #[canonical_json(owner = semio_framework_pack_json)]
    #[mutation_leaf(contract = ::protocol)]
    #[value(rename_all = "camelCase")]
    pub struct RemoveLayerAsset {
        pub asset_id: String,
    }

    impl<Op> protocol::MutationKind<RasterSnapshot, Op> for RemoveLayerAsset
where
    Op: protocol::Mutation<RasterSnapshot, Diff = semio_s_artifact_raster_raster_diff::RasterDiff> + From<crate::add_layer_asset::mutation::AddLayerAsset> {
const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "asset", kind: "remove-layer-asset", record: "RemovedLayerAsset" };
    fn label(&self) -> semio_ui_locale::LocalizedLabel {
            semio_ui_locale::LocalizedLabel::native(&format!("Remove asset {}", self.asset_id), &format!("Asset {} entfernen", self.asset_id))
        }
    fn target(&self) -> Vec<String> {
            vec![self.asset_id.clone()]
        }
}
impl<Op> protocol::ColdMutationKind<RasterSnapshot, Op> for RemoveLayerAsset
where
    Op: protocol::ColdMutation<RasterSnapshot, ReferenceDiff = semio_s_artifact_raster_raster_diff::RasterDiff> + From<crate::add_layer_asset::mutation::AddLayerAsset> {
    fn diff(&self, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
            crate::remove_layer_asset::diff::diff(self, base)
        }
    fn inverse(&self, base: &RasterSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    crate::remove_layer_asset::inverse::inverse(self, base)
}
}
    //#endregion 🔖️RemoveLayerAsset
}

pub use mutation::RemoveLayerAsset;

//#region 🪢️TaxonomyMounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️TaxonomyMounts
