//! 📜️ `set-doctype` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetDoctype {
    pub(crate) doctype: Option<String>,
}

impl protocol::MutationKind<HtmlSnapshot, HtmlMutation> for SetDoctype {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "doctype", kind: "set-doctype", record: "SetDoctype" };

    fn diff(&self, base: &HtmlSnapshot) -> protocol::MutationOutcome<<HtmlMutation as Mutation<HtmlSnapshot>>::Diff> {
        let Self { doctype } = self;
        protocol::MutationOutcome::new(HtmlDiff { doctype: Some(doctype.clone()), root: None })
    }
    fn inverse(&self, base: &HtmlSnapshot) -> Result<Vec<HtmlMutation>, semio_framework_value::ValueError> {
        Ok(vec![HtmlMutation::SetDoctype(set_doctype::SetDoctype { doctype: base.doctype.clone() })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set doctype", "Dokumenttyp setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
