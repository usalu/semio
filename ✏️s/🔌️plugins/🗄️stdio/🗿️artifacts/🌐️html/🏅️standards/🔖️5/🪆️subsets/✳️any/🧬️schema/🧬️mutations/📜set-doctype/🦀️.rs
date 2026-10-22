//! 📜️ `set-doctype` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;
use semio_s_artifact_stdio_html_snapshot::standards::v5::subsets::any::schema::snapshot::HtmlSnapshot;
use semio_s_artifact_stdio_html_inferences::standards::v5::subsets::any::schema::diff::HtmlDiff;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetDoctype {
    pub(crate) doctype: Option<String>,
}

impl protocol::MutationKind<HtmlSnapshot, HtmlMutation> for SetDoctype {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "doctype", kind: "set-doctype", record: "SetDoctype" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set doctype", "Dokumenttyp setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

impl protocol::ColdMutationKind<HtmlSnapshot, HtmlMutation> for SetDoctype {
    fn diff(&self, _base: &HtmlSnapshot) -> protocol::MutationOutcome<<HtmlMutation as protocol::ColdMutation<HtmlSnapshot>>::ReferenceDiff> {
        let Self { doctype } = self;
        protocol::MutationOutcome::new(HtmlDiff { doctype: Some(doctype.clone()), root: None })
    }
    fn inverse(&self, base: &HtmlSnapshot) -> Result<Vec<HtmlMutation>, semio_framework_value::ValueError> {
        Ok(vec![HtmlMutation::SetDoctype(SetDoctype { doctype: base.doctype.clone() })])
    }
}
//#endregion 🔖️Payload
