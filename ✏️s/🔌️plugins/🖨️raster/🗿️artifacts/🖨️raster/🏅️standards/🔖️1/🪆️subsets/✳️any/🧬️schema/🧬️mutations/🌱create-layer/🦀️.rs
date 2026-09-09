//! 🌱 `create-layer` — brings a new `RasterLayerNode` into existence at a tree address.

pub mod mutation {
    use crate::diff::RasterDiff;
    use crate::mutations::RasterMutation;
    use crate::{RasterLayerNode, RasterSnapshot};

    //#region 🔖️CreateLayer
    #[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::MutationLeaf)]
    #[mutation_leaf(contract = ::protocol)]
    #[value(rename_all = "camelCase")]
    pub struct CreateLayer {
        pub parent_id: Option<String>,
        pub index: usize,
        pub layer: Box<RasterLayerNode>,
    }

    impl protocol::MutationKind<RasterSnapshot, RasterMutation> for CreateLayer {
        const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "layer", kind: "create-layer", record: "CreatedLayer" };

        fn diff(&self, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
            super::super::diff::diff(self, base)
        }

        fn inverse(&self, base: &RasterSnapshot) -> Vec<RasterMutation> {
            super::super::inverse::inverse(self, base)
        }

        fn label(&self) -> String {
            format!("Create layer \"{}\"", crate::standards::v1::subsets::any::schema::layer_name(&self.layer))
        }

        fn target(&self) -> Vec<String> {
            vec![crate::standards::v1::subsets::any::schema::layer_node_id(&self.layer).to_string()]
        }
    }
    //#endregion 🔖️CreateLayer
}

pub use mutation::CreateLayer;
