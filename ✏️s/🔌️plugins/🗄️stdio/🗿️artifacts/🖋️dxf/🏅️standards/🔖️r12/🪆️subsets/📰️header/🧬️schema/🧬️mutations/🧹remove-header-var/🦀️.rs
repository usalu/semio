//! 🧹️ `remove-header-var` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveHeaderVar {
    pub name: String,
}

impl protocol::MutationKind<DxfSnapshot, DxfMutation> for RemoveHeaderVar {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "header-var", kind: "remove-header-var", record: "RemoveHeaderVar" };

    fn diff(&self, base: &DxfSnapshot) -> protocol::MutationOutcome<<DxfMutation as Mutation<DxfSnapshot>>::Diff> {
        let Self { name } = self;
        protocol::MutationOutcome::new(diff_remove_header_var(name))
    }
    fn inverse(&self, base: &DxfSnapshot) -> Result<Vec<DxfMutation>, semio_framework_value::ValueError> {
        let Self { name } = self;
        Ok({
            match base.header_vars.iter().position(|v| &v.name == name) {
                Some(at) => vec![DxfMutation::SetHeaderVar(set_header_var::SetHeaderVar { name: name.clone(), header_var: base.header_vars[at].clone(), index: Some(at) })],
                None => Vec::new(),
            }
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove header var", "Header-Variable entfernen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
