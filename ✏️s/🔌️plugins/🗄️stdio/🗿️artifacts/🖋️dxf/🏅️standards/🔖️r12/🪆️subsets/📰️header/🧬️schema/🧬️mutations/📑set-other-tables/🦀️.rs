//! 📑 `set-other-tables` — replaces the raw-retained unmodeled tables. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;
use crate::schema::snapshot::DxfOtherTable;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetOtherTables {
    pub other_tables: Vec<DxfOtherTable>,
}

impl protocol::MutationKind<DxfSnapshot, DxfMutation> for SetOtherTables {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "other-tables", kind: "set-other-tables", record: "SetOtherTables" };

    fn diff(&self, base: &DxfSnapshot) -> protocol::MutationOutcome<<DxfMutation as Mutation<DxfSnapshot>>::Diff> {
        protocol::MutationOutcome::new(DxfDiff { other_tables: (base.other_tables != self.other_tables).then(|| self.other_tables.clone()), ..DxfDiff::default() })
    }
    fn inverse(&self, base: &DxfSnapshot) -> Result<Vec<DxfMutation>, semio_framework_value::ValueError> {
        Ok((base.other_tables != self.other_tables).then(|| DxfMutation::SetOtherTables(set_other_tables::SetOtherTables { other_tables: base.other_tables.clone() })).into_iter().collect())
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set other tables", "Weitere Tabellen setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
