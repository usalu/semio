//! 🔧️ Authoritative PDF mutation payload, diff, inverse, and tests for `set-object-value`.

use super::remove_object::RemoveObject;
use super::PdfMutation;
use crate::standards::v1_7::subsets::base::schema::{
    diff::{self, PdfDiff},
    snapshot::{ObjRef, PdfObject, PdfSnapshot},
};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetObjectValue {
    pub id: ObjRef,
    pub value: PdfObject,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<usize>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub admitted_stream_roles: Option<diff::PdfIndexedDiff<crate::standards::v1_7::subsets::base::schema::stream_roles::PdfAdmittedStreamRole>>,
}

impl MutationKind<PdfSnapshot, PdfMutation> for SetObjectValue {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "object-value", kind: "set-object-value", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::graph_edit_with_roles(diff::diff_set_object_value(base, self.id, self.value.clone(), self.index), self.admitted_stream_roles.as_ref()))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<PdfMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        match base.objects.iter().find(|object| object.id == self.id) {
            Some(object) => vec![PdfMutation::SetObjectValue(SetObjectValue { id: self.id, value: object.value.clone(), index: None, admitted_stream_roles: self.admitted_stream_roles.as_ref().map(|roles| roles.inverse(&base.admitted_stream_roles)) })],
            None => vec![PdfMutation::RemoveObject(RemoveObject { id: self.id, admitted_stream_roles: self.admitted_stream_roles.as_ref().map(|roles| roles.inverse(&base.admitted_stream_roles)) })],
        }
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set object {} {} value", self.id.num, self.id.gen), &format!("Wert von Objekt {} {} setzen", self.id.num, self.id.gen))
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
