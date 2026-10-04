//! ↔️ `move-layer` — absolute spatial reposition of an id-addressed layer's `transform.x`/`.y`.
//! Distinct from `reorder-layers` (list position, never spatial).

pub mod mutation {
    use crate::diff::RasterDiff;
    use crate::mutations::RasterMutation;
    use crate::RasterSnapshot;

    //#region 🔖️MoveLayer
    #[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf)]
    #[mutation_leaf(contract = ::protocol)]
    #[value(rename_all = "camelCase")]
    pub struct MoveLayer {
        pub layer_id: String,
        pub new_x: f64,
        pub new_y: f64,
    }

    impl protocol::MutationKind<RasterSnapshot, RasterMutation> for MoveLayer {
        const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "layer", kind: "move-layer", record: "MovedLayer" };

        fn diff(&self, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
            super::super::diff::diff(self, base)
        }

        fn inverse(&self, base: &RasterSnapshot) -> Result<Vec<RasterMutation>, semio_framework_value::ValueError> {
    Ok({
            super::super::inverse::inverse(self, base)?
        
    })
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
