//! 🖇️ `add-layer-asset` — attaches a real `SemioImageSnapshot` (event-log content) to the document's
//! id-keyed asset collection. NOT one of the coordinator's ten mandated derivations; added so
//! `image:in` media import (`crate::editor::raster::wasm`/the app's `import_media`) can stay a real,
//! undoable operation now that whole-document replace is gone — `assets: BTreeMap<String,
//! store::os_store::artifact_child::ArtifactChild<SemioImageSnapshot>>` (ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`:
//! the persisted document field holds only a composed-child HANDLE; this mutation's own payload still
//! carries the real bytes, minted into a handle + working-scene cache entry at apply time via
//! `crate::mint_raster_asset_child`) is itself an id-keyed root collection per
//! `📓️derivation-rules.md` rule 2, and `add`/`remove` is its taxonomy-correct verb pair (`add`:
//! "Attach a set-like member … inverse: remove").

pub mod mutation {
    use semio_s_artifact_raster_raster_diff::RasterDiff;

    use semio_s_artifact_raster_raster_model::RasterImagePayload;
    use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;

    //#region 🔖️AddLayerAsset
    #[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::NativeReceive)]
    #[canonical_json(owner = semio_framework_pack_json)]
    #[mutation_leaf(contract = ::protocol)]
    #[value(rename_all = "camelCase")]
    pub struct AddLayerAsset {
        pub asset_id: String,
        pub asset: RasterImagePayload,
    }

    impl<Op> protocol::MutationKind<RasterSnapshot, Op> for AddLayerAsset
where
    Op: protocol::Mutation<RasterSnapshot, Diff = semio_s_artifact_raster_raster_diff::RasterDiff> + From<crate::add_layer_asset::mutation::AddLayerAsset> + From<crate::remove_layer_asset::mutation::RemoveLayerAsset> {
const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "asset", kind: "add-layer-asset", record: "AddedLayerAsset" };
    fn label(&self) -> semio_ui_locale::LocalizedLabel {
            semio_ui_locale::LocalizedLabel::native(&format!("Add asset {}", self.asset_id), &format!("Asset {} hinzufügen", self.asset_id))
        }
    fn target(&self) -> Vec<String> {
            vec![self.asset_id.clone()]
        }
}
impl<Op> protocol::ColdMutationKind<RasterSnapshot, Op> for AddLayerAsset
where
    Op: protocol::ColdMutation<RasterSnapshot, ReferenceDiff = semio_s_artifact_raster_raster_diff::RasterDiff> + From<crate::add_layer_asset::mutation::AddLayerAsset> + From<crate::remove_layer_asset::mutation::RemoveLayerAsset> {
    fn diff(&self, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
            crate::add_layer_asset::diff::diff(self, base)
        }
    fn inverse(&self, base: &RasterSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    crate::add_layer_asset::inverse::inverse(self, base)
}
}
    //#endregion 🔖️AddLayerAsset
}

pub use mutation::AddLayerAsset;

//#region 🪢️TaxonomyMounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️TaxonomyMounts
