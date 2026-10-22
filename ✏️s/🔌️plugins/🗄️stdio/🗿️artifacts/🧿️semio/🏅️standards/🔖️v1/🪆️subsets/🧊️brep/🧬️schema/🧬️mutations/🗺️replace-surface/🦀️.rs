//! 🗺️ `replace-surface` — whole-value swap of `face_id`'s underlying `BrepSurface`. Same
//! structured-payload reasoning as `replace-curve`: a NURBS surface's control-point grid is edited
//! piecewise by the editor, so `replace`, never `change`.

use crate::schema::snapshot::{BrepSurface, SemioBrepSnapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplaceSurface {
    pub face_id: String,
    pub new_surface: BrepSurface,
}

impl<Op> protocol::MutationKind<SemioBrepSnapshot, Op> for ReplaceSurface
where
    Op: protocol::Mutation<SemioBrepSnapshot, Diff = crate::schema::diff::SemioBrepDiff> + From<ReplaceSurface>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "surface", kind: "replace-surface", record: "ReplacedSurface" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Replace surface on face \"{}\"", self.face_id), &format!("Oberfläche auf Fläche \"{}\" ersetzen", self.face_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.face_id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<SemioBrepSnapshot, Op> for ReplaceSurface
where
    Op: protocol::ColdMutation<SemioBrepSnapshot, ReferenceDiff = crate::schema::diff::SemioBrepDiff> + From<ReplaceSurface>,
{
    fn diff(&self, base: &SemioBrepSnapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<SemioBrepSnapshot>>::ReferenceDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioBrepSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Payload
