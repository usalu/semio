//! 🌐 Puzzle5d mutation — `ChangeDomain`: changes the document's design domain classification.
use crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
use crate::Puzzle5dSnapshot;

//#region 🔖️Mutation
/// 🌐 `change-domain` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "change-domain")]
pub struct ChangeDomain {
    pub new_domain: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_domain<Op>(new_domain: String) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_domain::ChangeDomain>,
{
    Op::from(ChangeDomain { new_domain })
}

impl<Op> protocol::MutationKind<Puzzle5dSnapshot, Op> for ChangeDomain
where
    Op: protocol::Mutation<Puzzle5dSnapshot, Diff = Puzzle5dDiff> + From<ChangeDomain>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "domain", kind: "change-domain", record: "ChangedDomain" };



    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change domain to \"{}\"", self.new_domain), &format!("Domäne auf \"{}\" ändern", self.new_domain))
    }
}

impl protocol::ColdMutationKind<Puzzle5dSnapshot, Puzzle5dMutation> for ChangeDomain {
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
