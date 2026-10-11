//! 🗑️ `delete-layer` — removes an id-addressed `RasterLayerNode` (and, if it is a `Group`, its
//! whole subtree cascade — `remove_layer_from_tree` deletes the node it finds, children included).

pub mod mutation {
    use crate::diff::RasterDiff;
    use crate::mutations::RasterMutation;
    use crate::RasterSnapshot;

    //#region 🔖️DeleteLayer
    #[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
    #[canonical_json(owner = semio_framework_pack_json)]
    #[mutation_leaf(contract = ::protocol)]
    #[value(rename_all = "camelCase")]
    pub struct DeleteLayer {
        pub layer_id: String,
    }

    impl protocol::MutationKind<RasterSnapshot, RasterMutation> for DeleteLayer {
        const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "layer", kind: "delete-layer", record: "DeletedLayer" };

        fn diff(&self, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
            super::super::diff::diff(self, base)
        }

        fn inverse(&self, base: &RasterSnapshot) -> Result<Vec<RasterMutation>, semio_framework_value::ValueError> {
    super::super::inverse::inverse(self, base)
}

        fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
            semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete layer {}", self.layer_id), &format!("Ebene {} löschen", self.layer_id))
        }

        fn target(&self) -> Vec<String> {
            vec![self.layer_id.clone()]
        }
    }
    //#endregion 🔖️DeleteLayer
}

pub use mutation::DeleteLayer;
