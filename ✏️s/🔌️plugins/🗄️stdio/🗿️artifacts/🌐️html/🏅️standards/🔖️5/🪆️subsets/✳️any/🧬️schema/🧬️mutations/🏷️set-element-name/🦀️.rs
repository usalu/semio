//! 🏷️ `set-element-name` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetElementName {
    pub(crate) path: NodePath,
    pub(crate) name: String,
}

impl protocol::MutationKind<HtmlSnapshot, HtmlMutation> for SetElementName {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "element-name", kind: "set-element-name", record: "SetElementName" };

    fn diff(&self, base: &HtmlSnapshot) -> protocol::MutationOutcome<<HtmlMutation as Mutation<HtmlSnapshot>>::Diff> {
        let Self { path, name } = self;
        protocol::MutationOutcome::new(diff_at_path(path, HtmlNodeDiff::Element(HtmlElementDiff { name: Some(name.clone()), attributes: None, children: None })))
    }
    fn inverse(&self, base: &HtmlSnapshot) -> Result<Vec<HtmlMutation>, semio_framework_value::ValueError> {
        let Self { path, .. } = self;
        Ok((|| {
            let prior = match node_at(base, path) {
                Ok(HtmlNode::Element { name, .. }) => name.clone(),
                _ => return Vec::new(),
            };
            vec![HtmlMutation::SetElementName(set_element_name::SetElementName { path: path.clone(), name: prior })]
        })())
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set element name", "Elementname setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
