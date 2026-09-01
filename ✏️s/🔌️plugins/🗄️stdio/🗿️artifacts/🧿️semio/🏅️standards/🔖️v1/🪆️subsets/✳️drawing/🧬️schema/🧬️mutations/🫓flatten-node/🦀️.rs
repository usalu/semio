//! 🫓️️ `flatten` — collapses the `Group` addressed by `at` down to ONE level: every descendant
//! `Group` (not `at` itself) is dissolved into its leaf (`Path`/`Text`/`Image`) descendants,
//! PROVIDED every one of those descendant groups has an identity `transform`. `Path`/`Text`/
//! `Image` carry no transform of their own (`Path`'s geometry lives entirely in absolute-
//! coordinate `segments`; `Text`/`Image` carry only a position `at`), so baking a non-identity
//! ancestor transform into their geometry is not something this facet can do honestly — a
//! non-identity descendant group makes this a real no-op (never a lossy approximation) rather
//! than silently corrupting geometry.

use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioTransform;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{DrawNodeDiff, NodePath, SemioDrawingDiff, diff_at_path, node_at};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::{SemioDrawingMutation, unflatten_node};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct FlattenNode {
    pub at: NodePath,
}

impl protocol::MutationKind<SemioDrawingSnapshot, SemioDrawingMutation> for FlattenNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "flatten", entity: "node", kind: "flatten-node", record: "FlattenedNode" };

    fn diff(&self, base: &SemioDrawingSnapshot) -> protocol::MutationOutcome<<SemioDrawingMutation as protocol::Mutation<SemioDrawingSnapshot>>::Diff> {
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

//#region 🔖️CollectLeaves
/// 🍃️️ Depth-first leaf collection through any run of identity-transform descendant `Group`s;
/// `None` the moment a non-identity transform is found (refuse rather than approximate).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn collect_flattened_leaves(children: &[DrawNode]) -> Option<Vec<DrawNode>> {
    let mut out = Vec::new();
    for child in children {
        match child {
            DrawNode::Group { transform, children: nested } => {
                if *transform != SemioTransform::identity() {
                    return None;
                }
                out.extend(collect_flattened_leaves(nested)?);
            }
            leaf => out.push(leaf.clone()),
        }
    }
    Some(out)
}
//#endregion 🔖️CollectLeaves
