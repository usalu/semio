//! 🌐 Puzzle3d mutation — `ChangeDomain`: changes the document's design domain classification.
use semio_s_artifact_puzzle_3d_diff::Puzzle3dDiff;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

//#region 🔖️Mutation
/// 🌐 `change-domain` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testing"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[dsl(keyword = "change-domain")]
pub struct ChangeDomain {
    pub new_domain: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_domain<Op: From<ChangeDomain>>(new_domain: String) -> Op {
    Op::from(ChangeDomain { new_domain })
}

impl<Op> protocol::MutationKind<Puzzle3dSnapshot, Op> for ChangeDomain
where
    Op: protocol::Mutation<Puzzle3dSnapshot, Diff = Puzzle3dDiff> + From<ChangeDomain>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "domain", kind: "change-domain", record: "ChangedDomain" };

    fn diff(&self, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change domain to \"{}\"", self.new_domain), &format!("Domäne auf \"{}\" ändern", self.new_domain))
    }
}
//#endregion 🔖️Mutation
