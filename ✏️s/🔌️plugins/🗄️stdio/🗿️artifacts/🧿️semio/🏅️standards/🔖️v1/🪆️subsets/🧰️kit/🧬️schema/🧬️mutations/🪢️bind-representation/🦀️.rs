//! 🔗️ `bind-representation` — appends a new independent-lifecycle LINK to the kit's
//! `representations` pool (FINAL-state addressing). `role` joins this link to the
//! `SemioKitType.id` it represents (module doc comment on `📸️snapshot/🦀️.rs`).

use crate::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct BindRepresentation {
    pub target: semio_framework_artifact_reference::ArtifactRef,
    pub pin: store::LinkPin,
    pub role: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub at: Option<usize>,
}

impl<Op> protocol::MutationKind<SemioKitSnapshot, Op> for BindRepresentation
where
    Op: protocol::Mutation<SemioKitSnapshot, Diff = crate::schema::diff::SemioKitDiff> + From<BindRepresentation> + From<crate::schema::mutations::unbind_representation::UnbindRepresentation>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "bind", entity: "representation", kind: "bind-representation", record: "BoundRepresentation" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Bind representation for {}", self.role), &format!("Repräsentation für {} binden", self.role))
    }
    fn target(&self) -> Vec<String> {
        vec![self.role.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<SemioKitSnapshot, Op> for BindRepresentation
where
    Op: protocol::ColdMutation<SemioKitSnapshot, ReferenceDiff = crate::schema::diff::SemioKitDiff> + From<BindRepresentation> + From<crate::schema::mutations::unbind_representation::UnbindRepresentation>,
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
