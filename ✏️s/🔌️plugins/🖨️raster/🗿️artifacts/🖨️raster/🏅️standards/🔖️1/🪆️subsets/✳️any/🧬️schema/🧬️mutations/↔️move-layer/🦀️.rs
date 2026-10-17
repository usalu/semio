//! ↔️ `move-layer` — absolute spatial reposition of an id-addressed layer's `transform.x`/`.y`.
//! Distinct from `reorder-layers` (list position, never spatial).

pub mod mutation {
    use semio_s_artifact_raster_raster_diff::RasterDiff;

    use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;

    //#region 🔖️MoveLayer
    #[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
    #[canonical_json(owner = semio_framework_pack_json)]
    #[mutation_leaf(contract = ::protocol)]
    #[value(rename_all = "camelCase")]
    pub struct MoveLayer {
        pub layer_id: String,
        pub new_x: f64,
        pub new_y: f64,
    }

    impl<Op> protocol::MutationKind<RasterSnapshot, Op> for MoveLayer where Op: protocol::Mutation<RasterSnapshot, Diff = semio_s_artifact_raster_raster_diff::RasterDiff> + From<crate::move_layer::mutation::MoveLayer> {
        const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "layer", kind: "move-layer", record: "MovedLayer" };

        fn diff(&self, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
            crate::move_layer::diff::diff(self, base)
        }

        fn inverse(&self, base: &RasterSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    crate::move_layer::inverse::inverse(self, base)
}

        fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
            semio_framework_ui_locale::LocalizedLabel::native(&format!("Move layer {} to ({}, {})", self.layer_id, self.new_x, self.new_y), &format!("Ebene {} nach ({}, {}) verschieben", self.layer_id, self.new_x, self.new_y))
        }

        fn target(&self) -> Vec<String> {
            vec![self.layer_id.clone()]
        }
    }
    //#endregion 🔖️MoveLayer
}

pub use mutation::MoveLayer;

//#region 🪢️TaxonomyMounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️TaxonomyMounts
