//! 🧬️ SvgMutation — document mutation dispatch.

use crate::artifacts::svg::schema::diff::{diff_set_snapshot, SvgDiff};
use crate::artifacts::svg::schema::snapshot::{
    element_attr, node_at, node_at_mut, parse_transform_list, parse_view_box, set_element_attr, transform_list_to_string, view_box_to_string, NodePath, TransformOp, ViewBox,
};
use crate::artifacts::svg::SvgSnapshot;
use crate::artifacts::xml::schema::snapshot::XmlNode;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.svg`. Beyond the baseline `{NoMutation, SetSnapshot}`,
/// this is a flagship mutation vocabulary (plan D2) addressing nodes in the persisted
/// `SvgSnapshot.doc` tree by `NodePath` (child-index chain from the root `<svg>` element).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum SvgMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: SvgSnapshot,
    },
    /// ➕️ Inserts `node` as child `index` of the element at `parent`.
    InsertElement {
        parent: NodePath,
        index: usize,
        node: XmlNode,
    },
    /// ➖️ Removes child `index` of the element at `parent`.
    RemoveElement {
        parent: NodePath,
        index: usize,
    },
    /// 🏷️ Sets (or, with `value: None`, removes) attribute `name` on the element at `path`.
    SetAttribute {
        path: NodePath,
        name: String,
        value: Option<String>,
    },
    /// ✍️ Replaces the literal text of the `Text` node at `path`.
    SetText {
        path: NodePath,
        text: String,
    },
    /// 🖼️ Sets (or clears) the typed `viewBox` of the element at `path`.
    SetViewBox {
        path: NodePath,
        view_box: Option<ViewBox>,
    },
    /// 🔄 Sets (or clears) the typed `transform` list of the element at `path`.
    SetTransform {
        path: NodePath,
        transform: Option<Vec<TransformOp>>,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`. Malformed `NodePath`s (produced only by the engine itself,
/// never by document parsing) are treated as no-ops -- this function has no `Result` in its
/// signature (matches `ArtifactBuilder::mutate`'s infallible contract), so a bad path can't be
/// surfaced as a typed error here; callers that need failure feedback should validate the path via
/// `node_at`/`node_at_mut` first.
pub fn apply_svg_mutation(snapshot: &mut SvgSnapshot, mutation: &SvgMutation) -> SvgDiff {
    // 🚧️ Not `<SvgMutation as protocol::Mutation<SvgSnapshot>>::diff(mutation, snapshot)` here --
    // `Mutation::diff`'s apply-and-capture fallback arm (below) itself calls `apply_svg_mutation`
    // on a clone, so computing the diff via `.diff()` up front would recurse infinitely. Instead
    // the diff is derived the same way `.diff()` does: mutate in place, then read the result back.
    match mutation {
        SvgMutation::NoMutation => {}
        SvgMutation::SetSnapshot { snapshot: next } => *snapshot = next.clone(),
        SvgMutation::InsertElement { parent, index, node } => {
            if let Ok(XmlNode::Element { children, .. }) = node_at_mut(&mut snapshot.doc, parent) {
                let idx = (*index).min(children.len());
                children.insert(idx, node.clone());
            }
        }
        SvgMutation::RemoveElement { parent, index } => {
            if let Ok(XmlNode::Element { children, .. }) = node_at_mut(&mut snapshot.doc, parent) {
                if *index < children.len() {
                    children.remove(*index);
                }
            }
        }
        SvgMutation::SetAttribute { path, name, value } => {
            if let Ok(node) = node_at_mut(&mut snapshot.doc, path) {
                set_element_attr(node, name, value.clone());
            }
        }
        SvgMutation::SetText { path, text } => {
            if let Ok(node) = node_at_mut(&mut snapshot.doc, path) {
                if let XmlNode::Text { text: t } = node {
                    *t = text.clone();
                }
            }
        }
        SvgMutation::SetViewBox { path, view_box } => {
            if let Ok(node) = node_at_mut(&mut snapshot.doc, path) {
                set_element_attr(node, "viewBox", view_box.as_ref().map(view_box_to_string));
            }
        }
        SvgMutation::SetTransform { path, transform } => {
            if let Ok(node) = node_at_mut(&mut snapshot.doc, path) {
                set_element_attr(node, "transform", transform.as_ref().map(|ops| transform_list_to_string(ops)));
            }
        }
    }

    match mutation {
        SvgMutation::NoMutation => SvgDiff::default(),
        SvgMutation::SetSnapshot { snapshot: next } => diff_set_snapshot(next),
        _ => diff_set_snapshot(snapshot),
    }
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<SvgSnapshot> for SvgMutation {
    type Diff = SvgDiff;

    /// 🔺️ Every mutation resolves to a full-snapshot-replacement diff (matching the existing
    /// `SvgDiff` shape: a sparse "replace whole snapshot" diff, not a per-field patch) -- computed
    /// generically by applying the mutation to `base` and capturing the result, so new mutation
    /// variants never need bespoke diff-construction logic.
    fn diff(&self, base: &SvgSnapshot) -> Self::Diff {
        match self {
            SvgMutation::NoMutation => SvgDiff::default(),
            SvgMutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
            other => {
                let mut next = base.clone();
                apply_svg_mutation(&mut next, other);
                diff_set_snapshot(&next)
            }
        }
    }

    fn inverse(&self, base: &SvgSnapshot) -> Vec<Self> {
        match self {
            SvgMutation::NoMutation => vec![SvgMutation::NoMutation],
            SvgMutation::SetSnapshot { .. } => vec![SvgMutation::SetSnapshot { snapshot: base.clone() }],
            SvgMutation::InsertElement { parent, index, .. } => vec![SvgMutation::RemoveElement { parent: parent.clone(), index: *index }],
            SvgMutation::RemoveElement { parent, index } => match node_at(&base.doc, parent) {
                Ok(XmlNode::Element { children, .. }) => match children.get(*index) {
                    Some(node) => vec![SvgMutation::InsertElement { parent: parent.clone(), index: *index, node: node.clone() }],
                    None => vec![SvgMutation::NoMutation],
                },
                _ => vec![SvgMutation::NoMutation],
            },
            SvgMutation::SetAttribute { path, name, .. } => {
                let old = node_at(&base.doc, path).ok().and_then(|n| element_attr(n, name)).map(|s| s.to_string());
                vec![SvgMutation::SetAttribute { path: path.clone(), name: name.clone(), value: old }]
            }
            SvgMutation::SetText { path, .. } => {
                let old = match node_at(&base.doc, path) {
                    Ok(XmlNode::Text { text }) => text.clone(),
                    _ => String::new(),
                };
                vec![SvgMutation::SetText { path: path.clone(), text: old }]
            }
            SvgMutation::SetViewBox { path, .. } => {
                let old = node_at(&base.doc, path).ok().and_then(|n| element_attr(n, "viewBox")).and_then(|v| parse_view_box(v).ok());
                vec![SvgMutation::SetViewBox { path: path.clone(), view_box: old }]
            }
            SvgMutation::SetTransform { path, .. } => {
                let old = node_at(&base.doc, path).ok().and_then(|n| element_attr(n, "transform")).and_then(|v| parse_transform_list(v).ok());
                vec![SvgMutation::SetTransform { path: path.clone(), transform: old }]
            }
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
impl protocol::OpText for SvgMutation {
    fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".into())
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line.trim()).map_err(|e| {
            store::TextError::new(format!("op parse: {e}"), dsl::TextSpan::at(1, 1))
        })
    }
}

impl protocol::OpBinary for SvgMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        serde_json::to_vec(self).map_err(|e| protocol::ProtocolError::Malformed {
            what: "op encode",
            offset: 0,
            detail: e.to_string(),
        })
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        serde_json::from_slice(bytes).map_err(|e| protocol::ProtocolError::Malformed {
            what: "op decode",
            offset: 0,
            detail: e.to_string(),
        })
    }
}
//#endregion OpCodecs

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::svg::schema::snapshot::write_svg_xml;
    use crate::artifacts::xml::schema::snapshot::XmlAttr;

    fn fixture() -> SvgSnapshot {
        <SvgSnapshot as store::ArtifactDsl>::parse_dsl(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 10 10"><rect x="0" y="0" width="5" height="5"/></svg>"#,
        )
        .unwrap()
    }

    #[test]
    fn insert_then_remove_element_apply_and_inverse() {
        let base = fixture();
        let insert = SvgMutation::InsertElement {
            parent: vec![],
            index: 1,
            node: XmlNode::Element { name: "circle".into(), attrs: vec![XmlAttr { name: "r".into(), value: "1".into() }], children: vec![] },
        };
        let mut after = base.clone();
        apply_svg_mutation(&mut after, &insert);
        match &after.doc.root {
            Some(XmlNode::Element { children, .. }) => assert_eq!(children.len(), 2),
            other => panic!("unexpected root {other:?}"),
        }
        let inverses = Mutation::inverse(&insert, &base);
        let mut restored = after.clone();
        for inv in &inverses {
            apply_svg_mutation(&mut restored, inv);
        }
        assert_eq!(restored, base);
    }

    #[test]
    fn set_attribute_apply_and_inverse_round_trip() {
        let base = fixture();
        let mutation = SvgMutation::SetAttribute { path: vec![0], name: "width".into(), value: Some("99".into()) };
        let diff = Mutation::diff(&mutation, &base);
        let after = <SvgDiff as protocol::MutationDiff<SvgSnapshot>>::apply(&diff, &base);
        assert_eq!(element_attr(node_at(&after.doc, &[0]).unwrap(), "width"), Some("99"));

        let inverses = Mutation::inverse(&mutation, &base);
        let mut restored = after.clone();
        for inv in &inverses {
            apply_svg_mutation(&mut restored, inv);
        }
        assert_eq!(write_svg_xml(&restored.doc), write_svg_xml(&base.doc));
    }

    #[test]
    fn set_view_box_and_set_transform_apply_and_inverse() {
        let base = fixture();
        let vb = SvgMutation::SetViewBox { path: vec![], view_box: Some(ViewBox { min_x: 1.0, min_y: 2.0, width: 3.0, height: 4.0 }) };
        let mut after = base.clone();
        apply_svg_mutation(&mut after, &vb);
        assert_eq!(element_attr(node_at(&after.doc, &[]).unwrap(), "viewBox"), Some("1 2 3 4"));
        for inv in Mutation::inverse(&vb, &base) {
            apply_svg_mutation(&mut after, &inv);
        }
        assert_eq!(write_svg_xml(&after.doc), write_svg_xml(&base.doc));

        let tf = SvgMutation::SetTransform { path: vec![0], transform: Some(vec![TransformOp::Translate { x: 2.0, y: None }]) };
        let mut after2 = base.clone();
        apply_svg_mutation(&mut after2, &tf);
        assert_eq!(element_attr(node_at(&after2.doc, &[0]).unwrap(), "transform"), Some("translate(2)"));
        for inv in Mutation::inverse(&tf, &base) {
            apply_svg_mutation(&mut after2, &inv);
        }
        assert_eq!(write_svg_xml(&after2.doc), write_svg_xml(&base.doc));
    }

    #[test]
    fn remove_element_inverse_restores_removed_node() {
        let base = fixture();
        let remove = SvgMutation::RemoveElement { parent: vec![], index: 0 };
        let mut after = base.clone();
        apply_svg_mutation(&mut after, &remove);
        match &after.doc.root {
            Some(XmlNode::Element { children, .. }) => assert!(children.is_empty()),
            other => panic!("unexpected root {other:?}"),
        }
        for inv in Mutation::inverse(&remove, &base) {
            apply_svg_mutation(&mut after, &inv);
        }
        assert_eq!(write_svg_xml(&after.doc), write_svg_xml(&base.doc));
    }
}
//#endregion 🧪️Tests
