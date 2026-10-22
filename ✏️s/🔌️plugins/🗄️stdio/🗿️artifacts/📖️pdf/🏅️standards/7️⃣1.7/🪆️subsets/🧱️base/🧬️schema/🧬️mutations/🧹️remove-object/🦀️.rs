//! 🧹️ Authoritative PDF mutation payload, diff, inverse, and tests for `remove-object`.

use crate::standards::v1_7::subsets::base::schema::mutations::insert_object::InsertObject;
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::{ObjRef, PdfSnapshot};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveObject {
    pub id: ObjRef,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub admitted_stream_roles: Option<diff::PdfIndexedDiff<crate::standards::v1_7::subsets::base::schema::stream_roles::PdfAdmittedStreamRole>>,
}

impl<Op> MutationKind<PdfSnapshot, Op> for RemoveObject
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<RemoveObject> + From<crate::standards::v1_7::subsets::base::schema::mutations::insert_object::InsertObject>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "object", kind: "remove-object", record: "Remove" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove object {} {}", self.id.num, self.id.gen), &format!("Objekt {} {} entfernen", self.id.num, self.id.gen))
    }

    fn target(&self) -> Vec<String> {
        vec![format!("{} {}", self.id.num, self.id.gen)]
    }
}

impl<Op> protocol::ColdMutationKind<PdfSnapshot, Op> for RemoveObject
where
    Op: protocol::ColdMutation<PdfSnapshot, ReferenceDiff = PdfDiff> + From<RemoveObject> + From<crate::standards::v1_7::subsets::base::schema::mutations::insert_object::InsertObject>,
{
    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        if !base.objects.iter().any(|object| object.id == self.id) {
            return MutationOutcome::error("mutation.target-missing", format!("Object {} {} does not exist.", self.id.num, self.id.gen), [format!("{} {}", self.id.num, self.id.gen)]);
        }
        MutationOutcome::new(diff::graph_edit_with_roles(diff::diff_remove_object(self.id), self.admitted_stream_roles.as_ref()))
    }
    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
        base.objects.iter().position(|object| object.id == self.id).map(|position| Op::from(InsertObject { id: self.id, value: base.objects[position].value.clone(), index: Some(position), admitted_stream_roles: self.admitted_stream_roles.as_ref().map(|roles| roles.inverse(&base.admitted_stream_roles)) })).into_iter().collect()
    
    })())
}
}

//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests

