//! 🛤️️ `replace-path` — whole-value swap of a `Path` node's `segments` (SMO-approved; a path's
//! control points are edited piecewise by any real editor, so this is `replace`, not `change`).

use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::NodePath;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{PathSegment, SemioDrawingSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplacePath {
    pub at: NodePath,
    pub new_segments: Vec<PathSegment>,
}

impl protocol::MutationKind<SemioDrawingSnapshot, SemioDrawingMutation> for ReplacePath {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "path", kind: "replace-path", record: "ReplacedPath" };

    fn diff(&self, base: &SemioDrawingSnapshot) -> <SemioDrawingMutation as protocol::Mutation<SemioDrawingSnapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace path in layer #{}", self.at.layer)
    }
    fn target(&self) -> Vec<String> {
        vec![self.at.layer.to_string()]
    }
}
//#endregion 🔖️Payload
