//! 🌐 Puzzle5d mutation — `ChangeDomain`: changes the document's design domain classification.
use crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
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
pub fn change_domain(new_domain: String) -> Puzzle5dMutation {
    Puzzle5dMutation::ChangeDomain(ChangeDomain { new_domain })
}

impl protocol::MutationKind<Puzzle5dSnapshot, Puzzle5dMutation> for ChangeDomain {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "domain", kind: "change-domain", record: "ChangedDomain" };

    fn diff(&self, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle5dSnapshot) -> Result<Vec<Puzzle5dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change domain to \"{}\"", self.new_domain), &format!("Domäne auf \"{}\" ändern", self.new_domain))
    }
}
//#endregion 🔖️Mutation
