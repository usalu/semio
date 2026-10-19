//! 🗑️ `delete-column` — removes a named column and its aligned cell from every row (captures the
//! full cascade for its inverse, per `📓️taxonomy.md`'s `delete` row).

use crate::schema::mutations::SemioTableOp;
use semio_s_artifact_stdio_semio_table_model::schema::diff::SemioTableDiff;
use semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteColumn {
    pub name: String,
}

impl<Op: SemioTableOp> protocol::MutationKind<SemioTableSnapshot, Op> for DeleteColumn {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "column", kind: "delete-column", record: "DeletedColumn" };

    fn diff(&self, base: &SemioTableSnapshot) -> protocol::MutationOutcome<SemioTableDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioTableSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete column {}", self.name), &format!("Spalte {} löschen", self.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.name.clone()]
    }
}
//#endregion 🔖️Payload
