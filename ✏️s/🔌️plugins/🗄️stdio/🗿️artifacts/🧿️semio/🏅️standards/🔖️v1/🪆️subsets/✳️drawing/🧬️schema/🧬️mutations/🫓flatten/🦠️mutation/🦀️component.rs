//! 🫓️️ `flatten` — collapses the `Group` addressed by `at` down to ONE level: every descendant
//! `Group` (not `at` itself) is dissolved into its leaf (`Path`/`Text`/`Image`) descendants,
//! PROVIDED every one of those descendant groups has an identity `transform`. `Path`/`Text`/
//! `Image` carry no transform of their own (`Path`'s geometry lives entirely in absolute-
//! coordinate `segments`; `Text`/`Image` carry only a position `at`), so baking a non-identity
//! ancestor transform into their geometry is not something this facet can do honestly — a
//! non-identity descendant group makes this a real no-op (never a lossy approximation) rather
//! than silently corrupting geometry.

use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::NodePath;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FlattenNode {
    pub at: NodePath,
}

impl protocol::MutationKind<SemioDrawingSnapshot, SemioDrawingMutation> for FlattenNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "flatten", entity: "node", kind: "flatten", record: "FlattenedNode" };

    fn diff(&self, base: &SemioDrawingSnapshot) -> <SemioDrawingMutation as protocol::Mutation<SemioDrawingSnapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Flatten node in layer #{}", self.at.layer)
    }
    fn target(&self) -> Vec<String> {
        vec![self.at.layer.to_string()]
    }
}
//#endregion 🔖️Payload
