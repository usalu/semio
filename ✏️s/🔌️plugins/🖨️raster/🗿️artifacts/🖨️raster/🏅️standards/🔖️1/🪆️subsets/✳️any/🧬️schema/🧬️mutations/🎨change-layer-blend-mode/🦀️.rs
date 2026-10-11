//! 🎨 `change-layer-blend-mode` — sets an id-addressed layer's `blend_mode` scalar.

pub mod mutation {
    use crate::diff::RasterDiff;
    use crate::mutations::RasterMutation;
    use crate::RasterSnapshot;

    //#region 🔖️ChangeLayerBlendMode
    #[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
    #[canonical_json(owner = semio_framework_pack_json)]
    #[mutation_leaf(contract = ::protocol)]
    #[value(rename_all = "camelCase")]
    pub struct ChangeLayerBlendMode {
        pub layer_id: String,
        pub new_blend_mode: String,
    }

    impl protocol::MutationKind<RasterSnapshot, RasterMutation> for ChangeLayerBlendMode {
        const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "layer", kind: "change-layer-blend-mode", record: "ChangedLayerBlendMode" };

        fn diff(&self, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
            super::super::diff::diff(self, base)
        }

        fn inverse(&self, base: &RasterSnapshot) -> Result<Vec<RasterMutation>, semio_framework_value::ValueError> {
    super::super::inverse::inverse(self, base)
}

        fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
            semio_framework_ui_locale::LocalizedLabel::native(&format!("Set layer {} blend mode to {}", self.layer_id, self.new_blend_mode), &format!("Füllmethode von Ebene {} auf {} setzen", self.layer_id, self.new_blend_mode))
        }

        fn target(&self) -> Vec<String> {
            vec![self.layer_id.clone()]
        }
    }
    //#endregion 🔖️ChangeLayerBlendMode
}

pub use mutation::ChangeLayerBlendMode;
