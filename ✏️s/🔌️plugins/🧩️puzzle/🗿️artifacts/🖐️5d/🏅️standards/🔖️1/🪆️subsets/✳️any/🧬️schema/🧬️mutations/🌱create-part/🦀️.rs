//! 🌱 Puzzle5d mutation — `CreatePart`: brings a new id-keyed part into existence.
use crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
use crate::{Puzzle5dSnapshot};
use crate::standards::v1::subsets::any::schema::document_records::Puzzle5dPart;


//#region 🔖️Mutation
/// 🌱 `create-part` payload — full initial payload at an optional FINAL-state `index` (`None`
/// appends). A duplicate `part.id` is a no-op.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "create-part")]
pub struct CreatePart {
    #[dsl(block)]
    pub part: Puzzle5dPart,
    pub index: Option<usize>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_part<Op>(part: Puzzle5dPart, index: Option<usize>) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::create_part::CreatePart>,
{
    Op::from(CreatePart { part, index })
}

impl<Op> protocol::MutationKind<Puzzle5dSnapshot, Op> for CreatePart
where
    Op: protocol::Mutation<Puzzle5dSnapshot, Diff = Puzzle5dDiff> + From<CreatePart> + From<crate::standards::v1::subsets::any::schema::mutations::delete_part::DeletePart>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "part", kind: "create-part", record: "CreatedPart" };



    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create part \"{}\"", self.part.id), &format!("Bauteil \"{}\" erstellen", self.part.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.part.id.clone()]
    }
}

impl protocol::ColdMutationKind<Puzzle5dSnapshot, Puzzle5dMutation> for CreatePart {


}

impl protocol::ColdMutationKind<Puzzle5dSnapshot, Puzzle5dMutation> for CreatePart {
    fn diff(&self, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
