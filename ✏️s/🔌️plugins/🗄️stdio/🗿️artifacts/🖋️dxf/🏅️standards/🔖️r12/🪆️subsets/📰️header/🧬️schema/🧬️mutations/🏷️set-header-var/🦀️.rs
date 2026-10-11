//! 🏷️ `set-header-var` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetHeaderVar {
    pub name: String,
    pub header_var: DxfHeaderVar,
    pub index: Option<usize>,
}

impl protocol::MutationKind<DxfSnapshot, DxfMutation> for SetHeaderVar {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "header-var", kind: "set-header-var", record: "SetHeaderVar" };

    fn diff(&self, base: &DxfSnapshot) -> protocol::MutationOutcome<<DxfMutation as Mutation<DxfSnapshot>>::Diff> {
        let Self { name, header_var, index } = self;
        protocol::MutationOutcome::new({
            let existed = base.header_vars.iter().any(|v| &v.name == name);
            diff_set_header_var(index.unwrap_or(base.header_vars.len()).min(base.header_vars.len()), name, header_var.clone(), existed)
        })
    }
    fn inverse(&self, base: &DxfSnapshot) -> Result<Vec<DxfMutation>, semio_framework_value::ValueError> {
        let Self { name, .. } = self;
        Ok({
            match base.header_vars.iter().find(|v| &v.name == name) {
                Some(v) => vec![DxfMutation::SetHeaderVar(set_header_var::SetHeaderVar { name: name.clone(), header_var: v.clone(), index: None })],
                None => vec![DxfMutation::RemoveHeaderVar(remove_header_var::RemoveHeaderVar { name: name.clone() })],
            }
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set header var", "Header-Variable setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
