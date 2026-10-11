//! 🎚️ `change-layer-adjustment-kind` — changes an id-addressed `Adjustment` layer's
//! `adjustment_kind` scalar. Only meaningful on the `Adjustment` variant; addressing a
//! `Pixel`/`Group` layer is a graceful no-op.

pub mod mutation {
    use crate::diff::RasterDiff;
    use crate::mutations::RasterMutation;
    use crate::RasterSnapshot;

    //#region 🔖️ChangeLayerAdjustmentKind
    #[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
    #[canonical_json(owner = semio_framework_pack_json)]
    #[mutation_leaf(contract = ::protocol)]
    #[value(rename_all = "camelCase")]
    pub struct ChangeLayerAdjustmentKind {
        pub layer_id: String,
        pub new_adjustment_kind: String,
    }

    impl protocol::MutationKind<RasterSnapshot, RasterMutation> for ChangeLayerAdjustmentKind {
        const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "layer", kind: "change-layer-adjustment-kind", record: "ChangedLayerAdjustmentKind" };

        fn diff(&self, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
            super::super::diff::diff(self, base)
        }

        fn inverse(&self, base: &RasterSnapshot) -> Result<Vec<RasterMutation>, semio_framework_value::ValueError> {
    super::super::inverse::inverse(self, base)
}

        fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
            semio_framework_ui_locale::LocalizedLabel::native(&format!("Set layer {} adjustment kind to {}", self.layer_id, self.new_adjustment_kind), &format!("Korrekturart von Ebene {} auf {} setzen", self.layer_id, self.new_adjustment_kind))
        }

        fn target(&self) -> Vec<String> {
            vec![self.layer_id.clone()]
        }
    }
    //#endregion 🔖️ChangeLayerAdjustmentKind
}

pub use mutation::ChangeLayerAdjustmentKind;
