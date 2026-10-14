//! ✏️ `rename-layer` — changes an id-addressed layer's identity field (`name`).

pub mod mutation {
    use semio_s_artifact_raster_raster_diff::RasterDiff;

    use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;

    //#region 🔖️RenameLayer
    #[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
    #[canonical_json(owner = semio_framework_pack_json)]
    #[mutation_leaf(contract = ::protocol)]
    #[value(rename_all = "camelCase")]
    pub struct RenameLayer {
        pub layer_id: String,
        pub new_name: String,
    }

    impl<Op> protocol::MutationKind<RasterSnapshot, Op> for RenameLayer where Op: protocol::Mutation<RasterSnapshot, Diff = semio_s_artifact_raster_raster_diff::RasterDiff> + From<crate::rename_layer::mutation::RenameLayer> {
        const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "layer", kind: "rename-layer", record: "RenamedLayer" };

        fn diff(&self, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
            crate::rename_layer::diff::diff(self, base)
        }

        fn inverse(&self, base: &RasterSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    crate::rename_layer::inverse::inverse(self, base)
}

        fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
            semio_framework_ui_locale::LocalizedLabel::native(&format!("Rename layer {} to \"{}\"", self.layer_id, self.new_name), &format!("Ebene {} in \"{}\" umbenennen", self.layer_id, self.new_name))
        }

        fn target(&self) -> Vec<String> {
            vec![self.layer_id.clone()]
        }
    }
    //#endregion 🔖️RenameLayer
}

pub use mutation::RenameLayer;

//#region 🪢️TaxonomyMounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
#[cfg(test)]
#[path = "🧪️tests/✏️renames/🦀️.rs"]
mod tests_renames_the_sketch_layer_to_final_linework;
//#endregion 🪢️TaxonomyMounts
