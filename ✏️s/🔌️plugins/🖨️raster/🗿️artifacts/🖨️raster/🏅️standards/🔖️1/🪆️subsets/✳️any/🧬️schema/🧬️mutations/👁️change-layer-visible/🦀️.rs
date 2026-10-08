//! 👁️ `change-layer-visible` — sets an id-addressed layer's `visible` scalar.

pub mod mutation {
    use crate::diff::RasterDiff;
    use crate::mutations::RasterMutation;
    use crate::RasterSnapshot;

    //#region 🔖️ChangeLayerVisible
    #[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf)]
    #[mutation_leaf(contract = ::protocol)]
    #[value(rename_all = "camelCase")]
    pub struct ChangeLayerVisible {
        pub layer_id: String,
        pub new_visible: bool,
    }

    impl protocol::MutationKind<RasterSnapshot, RasterMutation> for ChangeLayerVisible {
        const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "layer", kind: "change-layer-visible", record: "ChangedLayerVisible" };

        fn diff(&self, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
            super::super::diff::diff(self, base)
        }

        fn inverse(&self, base: &RasterSnapshot) -> Result<Vec<RasterMutation>, semio_framework_value::ValueError> {
    super::super::inverse::inverse(self, base)
}

        fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
            semio_framework_ui_locale::LocalizedLabel::native(&format!("Set layer {} visible to {}", self.layer_id, self.new_visible), &format!("Sichtbarkeit von Ebene {} auf {} setzen", self.layer_id, self.new_visible))
        }

        fn target(&self) -> Vec<String> {
            vec![self.layer_id.clone()]
        }
    }
    //#endregion 🔖️ChangeLayerVisible
}

pub use mutation::ChangeLayerVisible;
