//! ➕️ `insert-tiny-element` — authored as its own mutation leaf. The aggregate's original `diff`/
//! `inverse` bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its
//! aggregate value and delegates, so the semantics are preserved by construction rather than
//! re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertTinyElement {
    pub(crate) parent: NodePath,
    pub(crate) index: usize,
    pub(crate) node: SvgNode,
}

impl protocol::MutationKind<SvgSnapshot, SvgTinyMutation> for InsertTinyElement {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "tiny-element", kind: "insert-tiny-element", record: "InsertTinyElement" };

    fn diff(&self, base: &SvgSnapshot) -> protocol::MutationOutcome<<SvgTinyMutation as Mutation<SvgSnapshot>>::Diff> {
        agg_diff(&SvgTinyMutation::InsertTinyElement(self.clone()), base)
    }
    fn inverse(&self, base: &SvgSnapshot) -> Result<Vec<SvgTinyMutation>, semio_framework_value::ValueError> {
    Ok({
        agg_inverse(&SvgTinyMutation::InsertTinyElement(self.clone()), base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert tiny element", "Tiny-Element einfügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
