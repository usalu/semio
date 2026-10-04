//! ✏️ `rename-layer` — changes an id-addressed layer's identity field (`name`).

pub mod mutation {
    use crate::diff::RasterDiff;
    use crate::mutations::RasterMutation;
    use crate::RasterSnapshot;

    //#region 🔖️RenameLayer
    #[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf)]
    #[mutation_leaf(contract = ::protocol)]
    #[value(rename_all = "camelCase")]
    pub struct RenameLayer {
        pub layer_id: String,
        pub new_name: String,
    }

    impl protocol::MutationKind<RasterSnapshot, RasterMutation> for RenameLayer {
        const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "layer", kind: "rename-layer", record: "RenamedLayer" };

        fn diff(&self, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
            super::super::diff::diff(self, base)
        }

        fn inverse(&self, base: &RasterSnapshot) -> Result<Vec<RasterMutation>, semio_framework_value::ValueError> {
    Ok({
            super::super::inverse::inverse(self, base)?
        
    })
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
