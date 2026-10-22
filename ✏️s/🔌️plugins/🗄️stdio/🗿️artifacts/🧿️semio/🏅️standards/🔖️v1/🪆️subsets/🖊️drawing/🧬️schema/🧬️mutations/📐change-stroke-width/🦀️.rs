//! 📐️️ `change-stroke-width` — sets one scalar field (`DrawStyle.stroke_width`, name-keyed) on
//! the named style, independently of `change-stroke-color` per SMO's stroke decomposition ruling.

use crate::schema::snapshot::SemioDrawingSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeStrokeWidth {
    pub style_name: String,
    pub new_width: Option<f64>,
}

impl<Op> protocol::MutationKind<SemioDrawingSnapshot, Op> for ChangeStrokeWidth
where
    Op: protocol::Mutation<SemioDrawingSnapshot, Diff = crate::schema::diff::SemioDrawingDiff> + From<ChangeStrokeWidth>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "stroke-width", kind: "change-stroke-width", record: "ChangedStrokeWidth" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change stroke width of style \"{}\"", self.style_name), &format!("Strichbreite von Stil \"{}\" ändern", self.style_name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.style_name.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<SemioDrawingSnapshot, Op> for ChangeStrokeWidth
where
    Op: protocol::ColdMutation<SemioDrawingSnapshot, ReferenceDiff = crate::schema::diff::SemioDrawingDiff> + From<ChangeStrokeWidth>,
{
    fn diff(&self, base: &SemioDrawingSnapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<SemioDrawingSnapshot>>::ReferenceDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioDrawingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Payload
