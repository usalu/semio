//! 🔀 `reorder-layers` — repositions an id-addressed layer within or across the layer tree (LIST
//! position, never spatial — the coordinator's explicit ruling; spatial reposition is `move-layer`).

pub mod mutation {
    use crate::diff::RasterDiff;
    use crate::mutations::RasterMutation;
    use crate::RasterSnapshot;

    //#region 🔖️ReorderLayers
    #[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
    #[canonical_json(owner = semio_framework_pack_json)]
    #[mutation_leaf(contract = ::protocol)]
    #[value(rename_all = "camelCase")]
    pub struct ReorderLayers {
        pub layer_id: String,
        pub parent_id: Option<String>,
        pub index: usize,
    }

    impl protocol::MutationKind<RasterSnapshot, RasterMutation> for ReorderLayers {
        const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "reorder", entity: "layer", kind: "reorder-layers", record: "ReorderedLayers" };

        fn diff(&self, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
            super::super::diff::diff(self, base)
        }

        fn inverse(&self, base: &RasterSnapshot) -> Result<Vec<RasterMutation>, semio_framework_value::ValueError> {
    super::super::inverse::inverse(self, base)
}

        fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
            semio_framework_ui_locale::LocalizedLabel::native(&format!("Reorder layer {}", self.layer_id), &format!("Reihenfolge von Ebene {} ändern", self.layer_id))
        }

        fn target(&self) -> Vec<String> {
            vec![self.layer_id.clone()]
        }
    }
    //#endregion 🔖️ReorderLayers
}

pub use mutation::ReorderLayers;
