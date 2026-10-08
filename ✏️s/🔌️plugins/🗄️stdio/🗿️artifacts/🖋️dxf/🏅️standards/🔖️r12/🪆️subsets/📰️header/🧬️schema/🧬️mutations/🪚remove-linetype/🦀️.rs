//! 🪚️ `remove-linetype` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveLinetype {
    pub name: String,
}

impl protocol::MutationKind<DxfSnapshot, DxfMutation> for RemoveLinetype {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "linetype", kind: "remove-linetype", record: "RemoveLinetype" };

    fn diff(&self, base: &DxfSnapshot) -> protocol::MutationOutcome<<DxfMutation as Mutation<DxfSnapshot>>::Diff> {
        let Self { name } = self;
        protocol::MutationOutcome::new(diff_remove_linetype(name))
    }
    fn inverse(&self, base: &DxfSnapshot) -> Result<Vec<DxfMutation>, semio_framework_value::ValueError> {
        let Self { name } = self;
        Ok({
            match base.tables.linetypes.iter().find(|l| &l.name == name) {
                Some(l) => vec![DxfMutation::InsertLinetype(insert_linetype::InsertLinetype { index: base.tables.linetypes.iter().position(|x| &x.name == name).unwrap_or(base.tables.linetypes.len()), linetype: l.clone() })],
                None => Vec::new(),
            }
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove linetype", "Linientyp entfernen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
