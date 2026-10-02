//! 🎬️ `set-main-header` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse`
//! bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate
//! value and delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetMainHeader {
    pub main_header: AviMainHeader,
}

impl protocol::MutationKind<AviSnapshot, AviMutation> for SetMainHeader {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "main-header", kind: "set-main-header", record: "SetMainHeader" };

    fn diff(&self, base: &AviSnapshot) -> protocol::MutationOutcome<<AviMutation as Mutation<AviSnapshot>>::Diff> {
        agg_diff(&AviMutation::SetMainHeader(self.clone()), base)
    }
    fn inverse(&self, base: &AviSnapshot) -> Vec<AviMutation> {
        agg_inverse(&AviMutation::SetMainHeader(self.clone()), base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set main header", "Haupt-Header setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
