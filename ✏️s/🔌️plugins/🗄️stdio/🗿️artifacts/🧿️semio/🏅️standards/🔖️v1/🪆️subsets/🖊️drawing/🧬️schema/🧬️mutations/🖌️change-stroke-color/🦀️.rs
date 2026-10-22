//! 🖌️️ `change-stroke-color` — sets one scalar field (`DrawStyle.stroke`, name-keyed) on the named
//! style. SMO's binding ruling decomposes stroke into independently-set fields rather than one
//! `update`/`replace-stroke` facet: an editor sets color without touching width. Addressed by
//! `style_name` (the real name-keyed collection this snapshot's stroke lives on), not `node_id` —
//! `DrawStyle` is referenced BY NAME from `DrawNode.style`, it is not a per-node struct.

use semio_s_artifact_stdio_semio_base::schema::geometry::SemioRgba;
use crate::schema::snapshot::SemioDrawingSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeStrokeColor {
    pub style_name: String,
    pub new_color: Option<SemioRgba>,
}

impl<Op> protocol::MutationKind<SemioDrawingSnapshot, Op> for ChangeStrokeColor
where
    Op: protocol::Mutation<SemioDrawingSnapshot, Diff = crate::schema::diff::SemioDrawingDiff> + From<ChangeStrokeColor>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "stroke-color", kind: "change-stroke-color", record: "ChangedStrokeColor" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change stroke color of style \"{}\"", self.style_name), &format!("Strichfarbe von Stil \"{}\" ändern", self.style_name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.style_name.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<SemioDrawingSnapshot, Op> for ChangeStrokeColor
where
    Op: protocol::ColdMutation<SemioDrawingSnapshot, ReferenceDiff = crate::schema::diff::SemioDrawingDiff> + From<ChangeStrokeColor>,
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
