//! 🛤️️ `replace-path` — whole-value swap of a `Path` node's `segments` (SMO-approved; a path's
//! control points are edited piecewise by any real editor, so this is `replace`, not `change`).

use crate::schema::diff::NodePath;
use crate::schema::snapshot::{PathSegment, SemioDrawingSnapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplacePath {
    pub at: NodePath,
    pub new_segments: Vec<PathSegment>,
}

impl<Op> protocol::MutationKind<SemioDrawingSnapshot, Op> for ReplacePath
where
    Op: protocol::Mutation<SemioDrawingSnapshot, Diff = crate::schema::diff::SemioDrawingDiff> + From<ReplacePath>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "path", kind: "replace-path", record: "ReplacedPath" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Replace path in layer #{}", self.at.layer), &format!("Pfad in Ebene #{} ersetzen", self.at.layer))
    }
    fn target(&self) -> Vec<String> {
        vec![self.at.layer.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<SemioDrawingSnapshot, Op> for ReplacePath
where
    Op: protocol::ColdMutation<SemioDrawingSnapshot, ReferenceDiff = crate::schema::diff::SemioDrawingDiff> + From<ReplacePath>,
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
