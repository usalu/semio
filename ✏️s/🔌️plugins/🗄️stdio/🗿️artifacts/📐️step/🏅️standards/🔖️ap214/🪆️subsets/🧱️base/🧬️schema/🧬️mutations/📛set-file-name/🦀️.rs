//! 📛️ `set-file-name` — authored as its own mutation leaf. The aggregate's original `diff`/
//! `inverse` bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its
//! aggregate value and delegates, so the semantics are preserved by construction rather than
//! re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetFileName {
    pub file_name: StepFileName,
}

impl protocol::MutationKind<StepSnapshot, StepMutation> for SetFileName {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "file-name", kind: "set-file-name", record: "SetFileName" };

    fn diff(&self, base: &StepSnapshot) -> protocol::MutationOutcome<<StepMutation as Mutation<StepSnapshot>>::Diff> {
        agg_diff(&StepMutation::SetFileName(self.clone()), base)
    }
    fn inverse(&self, base: &StepSnapshot) -> Result<Vec<StepMutation>, semio_framework_value::ValueError> {
    Ok({
        agg_inverse(&StepMutation::SetFileName(self.clone()), base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set file name", "Dateiname setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
