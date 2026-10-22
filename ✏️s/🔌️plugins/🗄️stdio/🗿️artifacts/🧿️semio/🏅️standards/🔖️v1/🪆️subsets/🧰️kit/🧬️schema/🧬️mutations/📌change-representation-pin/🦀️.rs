//! 📌️ `change-representation-pin` — re-pins the link at `index` (target/role stay put; only
//! `pin` changes) — `change`, not `update`: a single narrow field, not an inseparable ≥2-field
//! facet rewritten atomically (📌️important.md's `change-link-pin` ruling).

use crate::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeRepresentationPin {
    pub index: usize,
    pub pin: store::LinkPin,
}

impl<Op> protocol::MutationKind<SemioKitSnapshot, Op> for ChangeRepresentationPin
where
    Op: protocol::Mutation<SemioKitSnapshot, Diff = crate::schema::diff::SemioKitDiff> + From<ChangeRepresentationPin>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "representation-pin", kind: "change-representation-pin", record: "ChangedRepresentationPin" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Re-pin representation at #{}", self.index), &format!("Repräsentation bei #{} neu anheften", self.index))
    }
    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<SemioKitSnapshot, Op> for ChangeRepresentationPin
where
    Op: protocol::ColdMutation<SemioKitSnapshot, ReferenceDiff = crate::schema::diff::SemioKitDiff> + From<ChangeRepresentationPin>,
{
    fn diff(&self, base: &SemioKitSnapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<SemioKitSnapshot>>::ReferenceDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioKitSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Payload
