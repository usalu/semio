//! 🧩️ Puzzle5d mutation — `ChangeFastenerKind`: changes a fastener's `fastener_kind` catalog reference.
use crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
use crate::Puzzle5dSnapshot;

//#region 🔖️Mutation
/// 🔩️ `change-fastener-kind` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "change-fastener-kind")]
pub struct ChangeFastenerKind {
    pub id: String,
    pub new_fastener_kind: Option<String>,
}

impl<Op> protocol::MutationKind<Puzzle5dSnapshot, Op> for ChangeFastenerKind
where
    Op: protocol::Mutation<Puzzle5dSnapshot, Diff = Puzzle5dDiff> + From<ChangeFastenerKind>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "fastener", kind: "change-fastener-kind", record: "ChangedFastenerKind" };



    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change fastener \"{}\" kind", self.id), &format!("Art von Verbindungsmittel \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Puzzle5dSnapshot, Op> for ChangeFastenerKind
where
    Op: protocol::ColdMutation<Puzzle5dSnapshot, ReferenceDiff = Puzzle5dDiff> + From<ChangeFastenerKind>,
{
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

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_fastener_kind<Op>(id: String, new_fastener_kind: Option<String>) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_fastener_kind::ChangeFastenerKind>,
{
    Op::from(ChangeFastenerKind { id, new_fastener_kind })
}
