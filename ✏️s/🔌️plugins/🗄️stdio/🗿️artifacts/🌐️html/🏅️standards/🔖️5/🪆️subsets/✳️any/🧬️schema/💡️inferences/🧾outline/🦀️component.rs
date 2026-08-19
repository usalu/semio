//! 🧾 `outline` — one named inference: this HTML5 document's own tree shape. `elementCount` is a
//! real recursive walk counting every `HtmlNode::Element` from `root` (always >= 1, the root
//! itself); `maxDepth` is the deepest element nesting level (`root` alone is depth 1);
//! `textLength` sums every `Text`/`RawText` node's character count anywhere in the tree.

use crate::artifacts::html::standards::v5::subsets::any::schema::snapshot::HtmlNode;
use crate::artifacts::html::HtmlSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Outline
/// 🧾️ `Html` document outline.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HtmlOutline {
    pub element_count: u32,
    pub max_depth: u32,
    pub text_length: u32,
}

/// 🌳️ Recursively walks `node`, returning `(element_count, max_depth, text_length)` — `depth` is
/// the caller's own nesting level (the root element call passes `1`).
async fn walk(node: &HtmlNode, depth: u32) -> (u32, u32, u32) {
    match node {
        HtmlNode::Element { children, .. } => {
            let mut count = 1u32;
            let mut max_depth = depth;
            let mut text_length = 0u32;
            for child in children {
                let (c, d, t) = walk(child, depth + 1);
                count += c;
                max_depth = max_depth.max(d);
                text_length += t;
            }
            (count, max_depth, text_length)
        }
        HtmlNode::Text { text } => (0, depth.saturating_sub(1), text.chars().count() as u32),
        HtmlNode::RawText { text, .. } => (0, depth.saturating_sub(1), text.chars().count() as u32),
        HtmlNode::Comment { .. } => (0, depth.saturating_sub(1), 0),
    }
}

impl HtmlOutline {
    pub async fn compute(snapshot: &HtmlSnapshot) -> Self {
        let (element_count, max_depth, text_length) = walk(&snapshot.root, 1);
        Self { element_count, max_depth, text_length }
    }
}
//#endregion 🔖️Outline

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn counts_elements_depth_and_text_over_nested_structure() {
        let root = HtmlNode::Element { name: "html".into(), attributes: vec![], children: vec![HtmlNode::Element { name: "body".into(), attributes: vec![], children: vec![HtmlNode::Text { text: "hello".into() }] }] };
        let snapshot = HtmlSnapshot { schema: "stdio.html".into(), doctype: None, root };
        let outline = HtmlOutline::compute(&snapshot);
        assert_eq!(outline.element_count, 2);
        assert_eq!(outline.max_depth, 2);
        assert_eq!(outline.text_length, 5);
    }

    #[semio_framework_async_macros::async_test]
    async fn default_snapshot_has_one_element_and_no_text() {
        let outline = HtmlOutline::compute(&HtmlSnapshot::default());
        assert_eq!(outline.element_count, 1);
        assert_eq!(outline.max_depth, 1);
        assert_eq!(outline.text_length, 0);
    }

    #[semio_framework_async_macros::async_test]
    async fn outline_is_deterministic() {
        let snapshot = HtmlSnapshot::default();
        assert_eq!(HtmlOutline::compute(&snapshot), HtmlOutline::compute(&snapshot));
    }
}
//#endregion 🧪️Tests
