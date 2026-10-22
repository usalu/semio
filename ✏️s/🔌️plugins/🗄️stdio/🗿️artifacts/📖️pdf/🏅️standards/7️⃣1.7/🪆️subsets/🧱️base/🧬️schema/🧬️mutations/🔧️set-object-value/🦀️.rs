//! 🔧️ Authoritative PDF mutation payload, diff, inverse, and tests for `set-object-value`.

use crate::standards::v1_7::subsets::base::schema::mutations::remove_object::RemoveObject;
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::{ObjRef, PdfObject, PdfSnapshot};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
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

impl<Op> MutationKind<PdfSnapshot, Op> for SetObjectValue
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<SetObjectValue> + From<crate::standards::v1_7::subsets::base::schema::mutations::remove_object::RemoveObject>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "object-value", kind: "set-object-value", record: "Set" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set object {} {} value", self.id.num, self.id.gen), &format!("Wert von Objekt {} {} setzen", self.id.num, self.id.gen))
    }

    fn target(&self) -> Vec<String> {
        vec![format!("{} {}", self.id.num, self.id.gen)]
    }
}

impl<Op> protocol::ColdMutationKind<PdfSnapshot, Op> for SetObjectValue
where
    Op: protocol::ColdMutation<PdfSnapshot, ReferenceDiff = PdfDiff> + From<SetObjectValue> + From<crate::standards::v1_7::subsets::base::schema::mutations::remove_object::RemoveObject>,
{
    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::graph_edit_with_roles(diff::diff_set_object_value(base, self.id, self.value.clone(), self.index), self.admitted_stream_roles.as_ref()))
    }
    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
        match base.objects.iter().find(|object| object.id == self.id) {
            Some(object) => vec![Op::from(SetObjectValue { id: self.id, value: object.value.clone(), index: None, admitted_stream_roles: self.admitted_stream_roles.as_ref().map(|roles| roles.inverse(&base.admitted_stream_roles)) })],
            None => vec![Op::from(RemoveObject { id: self.id, admitted_stream_roles: self.admitted_stream_roles.as_ref().map(|roles| roles.inverse(&base.admitted_stream_roles)) })],
        }
    
    })())
}
}

//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests
