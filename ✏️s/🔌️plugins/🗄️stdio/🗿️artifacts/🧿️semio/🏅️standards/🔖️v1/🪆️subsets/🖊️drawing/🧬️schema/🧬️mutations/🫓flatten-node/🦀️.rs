//! 🫓️️ `flatten` — collapses the `Group` addressed by `at` down to ONE level: every descendant
//! `Group` (not `at` itself) is dissolved into its leaf (`Path`/`Text`/`Image`) descendants,
//! PROVIDED every one of those descendant groups has an identity `transform`. `Path`/`Text`/
//! `Image` carry no transform of their own (`Path`'s geometry lives entirely in absolute-
//! coordinate `segments`; `Text`/`Image` carry only a position `at`), so baking a non-identity
//! ancestor transform into their geometry is not something this facet can do honestly — a
//! non-identity descendant group makes this a real no-op (never a lossy approximation) rather
//! than silently corrupting geometry.

use semio_s_artifact_stdio_semio_base::schema::geometry::SemioTransform;
use crate::schema::diff::NodePath;
use crate::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct FlattenNode {
    pub at: NodePath,
}

impl<Op> protocol::MutationKind<SemioDrawingSnapshot, Op> for FlattenNode
where
    Op: protocol::Mutation<SemioDrawingSnapshot, Diff = crate::schema::diff::SemioDrawingDiff> + From<FlattenNode> + From<crate::schema::mutations::unflatten_node::UnflattenNode>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "flatten", entity: "node", kind: "flatten-node", record: "FlattenedNode" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Flatten node in layer #{}", self.at.layer), &format!("Knoten in Ebene #{} reduzieren", self.at.layer))
    }
    fn target(&self) -> Vec<String> {
        vec![self.at.layer.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<SemioDrawingSnapshot, Op> for FlattenNode
where
    Op: protocol::ColdMutation<SemioDrawingSnapshot, ReferenceDiff = crate::schema::diff::SemioDrawingDiff> + From<FlattenNode> + From<crate::schema::mutations::unflatten_node::UnflattenNode>,
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
