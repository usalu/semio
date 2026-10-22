//! 🖊️️ `edit-design` — replaces the matching DESIGN's `pieces`/`connections` wholesale (BASE-state
//! addressing by `id`; `name` untouched). A design's arrangement is one authored unit — `edit`
//! replaces an authored content body per `📓️taxonomy.md`, the same shape `🔤️text`'s `edit-run`
//! uses one level down for a run's `content`.

use crate::schema::snapshot::{SemioKitConnection, SemioKitPiece, SemioKitSnapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct EditDesign {
    pub id: String,
    pub pieces: Vec<SemioKitPiece>,
    pub connections: Vec<SemioKitConnection>,
}

impl<Op> protocol::MutationKind<SemioKitSnapshot, Op> for EditDesign
where
    Op: protocol::Mutation<SemioKitSnapshot, Diff = crate::schema::diff::SemioKitDiff> + From<EditDesign>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "edit", entity: "design", kind: "edit-design", record: "EditedDesign" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Edit design {}", self.id), &format!("Entwurf {} bearbeiten", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<SemioKitSnapshot, Op> for EditDesign
where
    Op: protocol::ColdMutation<SemioKitSnapshot, ReferenceDiff = crate::schema::diff::SemioKitDiff> + From<EditDesign>,
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
