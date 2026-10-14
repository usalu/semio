//! 🌱 `create-layer` — brings a new `RasterLayerNode` into existence at a tree address.

pub mod mutation {
    use semio_s_artifact_raster_raster_diff::RasterDiff;

    use semio_s_artifact_raster_raster_model::RasterLayerNode;
    use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;

    //#region 🔖️CreateLayer
    #[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
    #[canonical_json(owner = semio_framework_pack_json)]
    #[mutation_leaf(contract = ::protocol)]
    #[value(rename_all = "camelCase")]
    pub struct CreateLayer {
        pub parent_id: Option<String>,
        pub index: usize,
        pub layer: Box<RasterLayerNode>,
    }

    impl<Op> protocol::MutationKind<RasterSnapshot, Op> for CreateLayer where Op: protocol::Mutation<RasterSnapshot, Diff = semio_s_artifact_raster_raster_diff::RasterDiff> + From<crate::delete_layer::mutation::DeleteLayer> {
        const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "layer", kind: "create-layer", record: "CreatedLayer" };

        fn diff(&self, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
            crate::create_layer::diff::diff(self, base)
        }

        fn inverse(&self, base: &RasterSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    crate::create_layer::inverse::inverse(self, base)
}

        fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
            semio_framework_ui_locale::LocalizedLabel::native(&format!("Create layer \"{}\"", semio_s_artifact_raster_raster_model::layer_name(&self.layer)), &format!("Ebene \"{}\" erstellen", semio_s_artifact_raster_raster_model::layer_name(&self.layer)))
        }

        fn target(&self) -> Vec<String> {
            vec![semio_s_artifact_raster_raster_model::layer_node_id(&self.layer).to_string()]
        }
    }
    //#endregion 🔖️CreateLayer
}

pub use mutation::CreateLayer;

//#region 🪢️TaxonomyMounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
#[cfg(test)]
#[path = "🧪️tests/🖋️creates-an-ink-8a1bf9/🦀️.rs"]
mod tests_creates_an_ink_layer_inside_the_artwork_group;
//#endregion 🪢️TaxonomyMounts
