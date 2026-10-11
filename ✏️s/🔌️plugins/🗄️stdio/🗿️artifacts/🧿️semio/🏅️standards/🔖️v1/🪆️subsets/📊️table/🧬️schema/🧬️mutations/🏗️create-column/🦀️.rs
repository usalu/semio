//! 🏗️ `create-column` — brings a new named column into existence at an optional FINAL-state
//! index, per `📓️taxonomy.md`'s `create` row ("full initial payload (+ optional `index`)").
//! Inserting `SemioValue::Null` at the same index into every row keeps the CRITICAL row/column
//! alignment invariant (see `📸️snapshot/🦀️.rs`'s own doc comment).

use crate::standards::v1::subsets::table::schema::mutations::SemioTableMutation;
use crate::standards::v1::subsets::table::schema::snapshot::{SemioTableCellKind, SemioTableSnapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateColumn {
    pub name: String,
    pub kind: SemioTableCellKind,
    pub index: Option<usize>,
}

impl protocol::MutationKind<SemioTableSnapshot, SemioTableMutation> for CreateColumn {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "column", kind: "create-column", record: "CreatedColumn" };

    fn diff(&self, base: &SemioTableSnapshot) -> protocol::MutationOutcome<<SemioTableMutation as protocol::Mutation<SemioTableSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioTableSnapshot) -> Result<Vec<SemioTableMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create column {}", self.name), &format!("Spalte {} erstellen", self.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.name.clone()]
    }
}
//#endregion 🔖️Payload
