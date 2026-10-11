//! 🧬️ HtmlMutation — document mutation dispatch. Every variant's `diff()` is handcrafted (never
//! apply-and-capture) and every variant's `inverse()` is handcrafted, path/key-aware. Structural
//! pattern borrowed from `🎨️svg`'s `SvgMutation` (own types throughout).
//!
//! Leaf-per-variant shape mirrored from `🖼️tiff`'s `TiffBaselineMutation` (ticket
//! `26/08/29/S-END-TO-END`): `NoMutation` was dropped (`#[derive(dsl::Mutations)]` requires every
//! variant to wrap exactly one leaf payload and a unit variant wraps none; `no` is not an approved
//! semantic verb either), and every remaining variant now wraps its own `dsl::MutationLeaf` struct
//! instead of carrying its fields as a struct-literal directly. `#[value(tag = "mutation", ...)]`
//! is kept (unlike tiff, which has none) so the wire shape this artifact's committed fixtures and
//! `OpText`/`OpBinary` already depend on stays byte-for-byte identical — serde's internally-tagged
//! representation supports a newtype variant wrapping a plain struct.









use crate::standards::v5::subsets::any::schema::diff::{diff_at_path, HtmlAttrAdded, HtmlAttrModified, HtmlAttributesDiff, HtmlChildAdded, HtmlChildrenDiff, HtmlDiff, HtmlElementDiff, HtmlNodeDiff};
use crate::standards::v5::subsets::any::schema::snapshot::{element_attr, node_at, HtmlNode, HtmlSnapshot, NodePath};

use protocol::{Mutation};
use semio_s_artifact_stdio_contract::deserialize_double_option;

//#region 🔖️Mutations
#[path = "➕insert-node/🦀️.rs"]
pub mod insert_node;
#[path = "➖remove-node/🦀️.rs"]
pub mod remove_node;
#[path = "🔖set-attribute/🦀️.rs"]
pub mod set_attribute;
#[path = "💬set-comment/🦀️.rs"]
pub mod set_comment;
#[path = "📜set-doctype/🦀️.rs"]
pub mod set_doctype;
#[path = "🏷️set-element-name/🦀️.rs"]
pub mod set_element_name;
#[path = "⌨️set-raw-text/🦀️.rs"]
pub mod set_raw_text;
/// 📐️ Typed content mutation for `stdio.html`. It addresses
/// nodes in the persisted `HtmlSnapshot.root` tree by `NodePath` (child-index chain from the root
/// element). `InsertNode`/`RemoveNode`'s `parent` addresses the PARENT element (`index` is the
/// position among the parent's children); every other path-carrying variant's `path` addresses the
/// target node itself.
//#region 🔖️Leaves
#[path = "✍️set-text/🦀️.rs"]
pub mod set_text;
//#endregion 🔖️Leaves

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = HtmlSnapshot, diff = HtmlDiff, schema = "HtmlMutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum HtmlMutation {
    /// 📜️ Sets (or, if `None`, clears) the document's raw `<!DOCTYPE ...>` declaration content.
    SetDoctype(set_doctype::SetDoctype),
    /// ➕️ Inserts `node` as child `index` of the element at `parent`.
    InsertNode(insert_node::InsertNode),
    /// ➖️ Removes child `index` of the element at `parent`.
    RemoveNode(remove_node::RemoveNode),
    /// 🏷️ Renames the element at `path`.
    SetElementName(set_element_name::SetElementName),
    /// 🏷️ Sets attribute `name` on the element at `path`. Tri-state `value`: `None` = remove the
    /// attribute entirely, `Some(None)` = set/keep it VALUELESS (e.g. `disabled`), `Some(Some(v))`
    /// = set it to `v`.
    SetAttribute(set_attribute::SetAttribute),
    /// ✍️ Replaces the literal text of the `Text` node at `path`.
    SetText(set_text::SetText),
    /// 💬️ Replaces the literal text of the `Comment` node at `path`.
    SetComment(set_comment::SetComment),
    /// 📄️ Replaces the literal text of the `RawText` node at `path` (its `parent_kind` — whether
    /// it belongs to a `<script>` or `<style>` element — is left unchanged).
    SetRawText(set_raw_text::SetRawText),
}

/// 📇️ Kebab-case spelling of every `HtmlMutation` variant, in declaration order -- the exhaustive
/// mutation catalog `../🔣️oracle.json`'s `kinds` array is required to match verbatim
/// (`kinds_const_matches_enum_variants_in_declaration_order` below is what keeps that honest; the
/// framework never parses Rust to check it itself).
pub const KINDS: &[&str] = &["set-doctype", "insert-node", "remove-node", "set-element-name", "set-attribute", "set-text", "set-comment", "set-raw-text"];
//#endregion 🔖️Mutations



//#endregion 🔖️Apply

//#region 🔖️AttributeHelper
/// 🏷️ Shared diff-construction for `SetAttribute`. Resolves the PRIOR tri-state of `name` on the
/// element addressed by `path` against `base`, then builds the exact `HtmlAttributesDiff` entry
/// the transition requires, lowered through `diff_at_path`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn attribute_diff_at_path(base: &HtmlSnapshot, path: &[usize], name: &str, value: Option<Option<String>>) -> HtmlDiff {
    let target = node_at(base, path).ok();
    let existing: Option<&Option<String>> = target.and_then(|n| element_attr(n, name));
    let attrs_diff = match (existing, value) {
        (Some(_), None) => HtmlAttributesDiff { removed: vec![name.to_string()], modified: Vec::new(), added: Vec::new() },
        (Some(_), Some(v)) => HtmlAttributesDiff { removed: Vec::new(), modified: vec![HtmlAttrModified { name: name.to_string(), value: v }], added: Vec::new() },
        (None, Some(v)) => {
            let next_index = match target {
                Some(HtmlNode::Element { attributes, .. }) => attributes.len(),
                _ => 0,
            };
            HtmlAttributesDiff { removed: Vec::new(), modified: Vec::new(), added: vec![HtmlAttrAdded { index: next_index, name: name.to_string(), value: v }] }
        }
        (None, None) => HtmlAttributesDiff::default(),
    };
    diff_at_path(path, HtmlNodeDiff::Element(HtmlElementDiff { name: None, attributes: Some(attrs_diff), children: None }))
}

/// 🔎 Reads the PRIOR tri-state of attribute `name` on the element addressed by `path` in `base`:
/// `None` = attribute absent, `Some(None)` = present and valueless, `Some(Some(v))` = present with
/// value `v`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn prior_attribute(base: &HtmlSnapshot, path: &[usize], name: &str) -> Option<Option<String>> {
    node_at(base, path).ok().and_then(|n| element_attr(n, name)).cloned()
}
//#endregion 🔖️AttributeHelper

//#endregion 🔖️MutationTrait

//#region OpCodecs















//#endregion OpCodecs

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests



#[cfg(test)]
use protocol::{OpBinary,OpText};
