//! 📦️ Authoritative PDF mutation payload, diff, inverse, and tests for `insert-object`.

use super::remove_object::RemoveObject;
use super::PdfMutation;
use crate::standards::v1_7::subsets::base::schema::{
    diff::{self, PdfDiff},
    snapshot::{ObjRef, PdfObject, PdfSnapshot},
};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct InsertObject {
    pub id: ObjRef,
    pub value: PdfObject,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<usize>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub admitted_stream_roles: Option<diff::PdfIndexedDiff<crate::standards::v1_7::subsets::base::schema::stream_roles::PdfAdmittedStreamRole>>,
}

impl MutationKind<PdfSnapshot, PdfMutation> for InsertObject {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "insert", entity: "object", kind: "insert-object", record: "Insert" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        if base.objects.iter().any(|object| object.id == self.id) {
            return MutationOutcome::fatal("mutation.duplicate-id", format!("Object {} {} already exists.", self.id.num, self.id.gen), [format!("{} {}", self.id.num, self.id.gen)]);
        }
        MutationOutcome::new(diff::graph_edit_with_roles(diff::diff_insert_object(self.id, self.index.map_or(base.objects.len(), |at| at.min(base.objects.len())), self.value.clone()), self.admitted_stream_roles.as_ref()))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<PdfMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        if base.objects.iter().any(|object| object.id == self.id) {
            Vec::new()
        } else {
            vec![PdfMutation::RemoveObject(RemoveObject { id: self.id, admitted_stream_roles: self.admitted_stream_roles.as_ref().map(|roles| roles.inverse(&base.admitted_stream_roles)) })]
        }
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Insert object {} {}", self.id.num, self.id.gen), &format!("Objekt {} {} einfügen", self.id.num, self.id.gen))
    }

    fn target(&self) -> Vec<String> {
        vec![format!("{} {}", self.id.num, self.id.gen)]
    }
}

//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

