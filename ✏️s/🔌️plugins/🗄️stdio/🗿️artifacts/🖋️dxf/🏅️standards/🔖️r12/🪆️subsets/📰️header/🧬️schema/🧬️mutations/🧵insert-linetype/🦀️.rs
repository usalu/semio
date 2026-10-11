//! 🧵️ `insert-linetype` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertLinetype {
    pub index: usize,
    pub linetype: DxfLinetype,
}

impl protocol::MutationKind<DxfSnapshot, DxfMutation> for InsertLinetype {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "linetype", kind: "insert-linetype", record: "InsertLinetype" };

    fn diff(&self, base: &DxfSnapshot) -> protocol::MutationOutcome<<DxfMutation as Mutation<DxfSnapshot>>::Diff> {
        let Self { index, linetype } = self;
        protocol::MutationOutcome::new(diff_insert_linetype(*index, linetype.clone()))
    }
    fn inverse(&self, base: &DxfSnapshot) -> Result<Vec<DxfMutation>, semio_framework_value::ValueError> {
        let Self { linetype, .. } = self;
        Ok({ vec![DxfMutation::RemoveLinetype(remove_linetype::RemoveLinetype { name: linetype.name.clone() })] })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert linetype", "Linientyp einfügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
