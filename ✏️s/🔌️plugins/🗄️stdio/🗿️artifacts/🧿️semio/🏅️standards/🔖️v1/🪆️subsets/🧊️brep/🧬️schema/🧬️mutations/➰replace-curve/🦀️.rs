//! ➰ `replace-curve` — whole-value swap of `edge_id`'s underlying `BrepCurve`. SMO's corrected
//! ruling: a NURBS curve has control points the editor edits individually, so this is `replace`,
//! never `change` — the discriminator is whether the editor ever manipulates the value's interior
//! piecewise, and it does here.

use crate::schema::snapshot::{BrepCurve, SemioBrepSnapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplaceCurve {
    pub edge_id: String,
    pub new_curve: BrepCurve,
}

impl<Op> protocol::MutationKind<SemioBrepSnapshot, Op> for ReplaceCurve
where
    Op: protocol::Mutation<SemioBrepSnapshot, Diff = crate::schema::diff::SemioBrepDiff> + From<ReplaceCurve>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "curve", kind: "replace-curve", record: "ReplacedCurve" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Replace curve on edge \"{}\"", self.edge_id), &format!("Kurve auf Kante \"{}\" ersetzen", self.edge_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.edge_id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<SemioBrepSnapshot, Op> for ReplaceCurve
where
    Op: protocol::ColdMutation<SemioBrepSnapshot, ReferenceDiff = crate::schema::diff::SemioBrepDiff> + From<ReplaceCurve>,
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
