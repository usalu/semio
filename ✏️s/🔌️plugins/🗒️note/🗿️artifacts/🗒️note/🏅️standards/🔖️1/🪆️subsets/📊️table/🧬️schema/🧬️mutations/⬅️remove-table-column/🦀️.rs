//! ⬅️ Note mutation — `RemoveTableColumn`: removes a table block's last column (a table always keeps at least one column).

use crate::{NoteDiff, NoteSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ⬅️ `remove-table-column` payload — removes a table block's last column (a table always keeps at least one column).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "remove-table-column")]
pub struct RemoveTableColumn {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn remove_table_column<Op>(id: String) -> Op
where
    Op: From<crate::standards::v1::subsets::table::schema::mutations::remove_table_column::RemoveTableColumn>,
{
    Op::from(RemoveTableColumn { id })
}

impl<Op> MutationKind<NoteSnapshot, Op> for RemoveTableColumn
where
    Op: protocol::Mutation<NoteSnapshot, Diff = NoteDiff> + From<RemoveTableColumn> + From<crate::standards::v1::subsets::table::schema::mutations::insert_table_column::InsertTableColumn>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "table-column", kind: "remove-table-column", record: "RemovedTableColumn" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove column from table \"{}\"", self.id), &format!("Spalte aus Tabelle \"{}\" entfernen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<NoteSnapshot, Op> for RemoveTableColumn
where
    Op: protocol::ColdMutation<NoteSnapshot, ReferenceDiff = NoteDiff> + From<RemoveTableColumn> + From<crate::standards::v1::subsets::table::schema::mutations::insert_table_column::InsertTableColumn>,
{
    fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &NoteSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
