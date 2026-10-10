//! 🪡️ `set-linetype` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetLinetype {
    pub name: String,
    pub linetype: DxfLinetype,
}

impl protocol::MutationKind<DxfSnapshot, DxfMutation> for SetLinetype {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "linetype", kind: "set-linetype", record: "SetLinetype" };

    fn diff(&self, base: &DxfSnapshot) -> protocol::MutationOutcome<<DxfMutation as Mutation<DxfSnapshot>>::Diff> {
        let Self { name, linetype } = self;
        protocol::MutationOutcome::new({
            let old = base.tables.linetypes.iter().find(|l| &l.name == name).cloned().unwrap_or_default();
            diff_set_linetype(name, linetype_field_changes(&old, linetype))
        })
    }
    fn inverse(&self, base: &DxfSnapshot) -> Result<Vec<DxfMutation>, semio_framework_value::ValueError> {
        let Self { name, .. } = self;
        Ok({
            match base.tables.linetypes.iter().find(|l| &l.name == name) {
                Some(l) => vec![DxfMutation::SetLinetype(set_linetype::SetLinetype { name: name.clone(), linetype: l.clone() })],
                None => vec![DxfMutation::RemoveLinetype(remove_linetype::RemoveLinetype { name: name.clone() })],
            }
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set linetype", "Linientyp setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
