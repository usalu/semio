//! 🧬️ XmlMutation — document mutation dispatch. Every variant's `diff()` is handcrafted (never
//! apply-and-capture) and every variant's `inverse()` is handcrafted, key/index-aware.

use crate::artifacts::xml::schema::diff::{
    diff_set_snapshot, XmlAttrAdded, XmlAttrModified, XmlAttributesDiff, XmlChildAdded, XmlChildrenDiff, XmlDiff,
    XmlElementDiff, XmlNodeDiff,
};
use crate::artifacts::xml::schema::snapshot::{XmlDeclaration, XmlNode};
use crate::artifacts::xml::XmlSnapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️NodePath
/// 🧭️ Path from the document root to a node: chain of child indices at each nesting level.
/// `XmlNodePath(vec![])` addresses the root itself. Mutation-level only (never appears inside
/// `XmlDiff` -- diffs nest via `XmlChildModified` chains instead, built by `diff_at_path`).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct XmlNodePath(pub Vec<usize>);

impl XmlNodePath {
    /// 🌳 The empty path -- addresses the document root.
    pub fn root() -> Self {
        Self(Vec::new())
    }

    /// 🔎️ Walks `self` from `root`, returning the addressed node if it exists and every
    /// intermediate segment is itself an `Element` (any other shape or an out-of-range index is a
    /// graceful `None`, never a panic).
    pub fn resolve<'a>(&self, root: Option<&'a XmlNode>) -> Option<&'a XmlNode> {
        let mut current = root?;
        for &index in &self.0 {
            let XmlNode::Element { children, .. } = current else { return None };
            current = children.get(index)?;
        }
        Some(current)
    }
}
//#endregion 🔖️NodePath

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.xml`. `InsertElement`/`RemoveElement`'s `path` addresses
/// the PARENT element (`index` is the position among the parent's children); every other
/// path-carrying variant's `path` addresses the target node itself.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum XmlMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: XmlSnapshot,
    },
    SetDeclaration {
        declaration: Option<XmlDeclaration>,
    },
    SetDoctype {
        doctype: Option<String>,
    },
    /// ➕️ Inserts `node` at `index` among the children of the element addressed by `path`.
    InsertElement {
        path: XmlNodePath,
        index: usize,
        node: XmlNode,
    },
    /// ➖️ Removes the child at `index` among the children of the element addressed by `path`.
    RemoveElement {
        path: XmlNodePath,
        index: usize,
    },
    /// 🏷️ Sets (or, if `value` is `None`, removes) the attribute `name` on the element addressed
    /// by `path`.
    SetAttribute {
        path: XmlNodePath,
        name: String,
        value: Option<String>,
    },
    /// 🔤️ Sets the literal text of the `Text` node addressed by `path`.
    SetText {
        path: XmlNodePath,
        text: String,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`: `let d = mutation.diff(&*snapshot); *snapshot = d.apply(snapshot); d`
/// -- the diff is the single semantics source, never a separate imperative apply path.
pub fn apply_xml_mutation(snapshot: &mut XmlSnapshot, mutation: &XmlMutation) -> XmlDiff {
    let diff = Mutation::diff(mutation, snapshot);
    *snapshot = protocol::MutationDiff::apply(&diff, snapshot);
    diff
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<XmlSnapshot> for XmlMutation {
    type Diff = XmlDiff;

    fn diff(&self, base: &XmlSnapshot) -> Self::Diff {
        match self {
            XmlMutation::NoMutation => XmlDiff::default(),
            XmlMutation::SetSnapshot { snapshot } => diff_set_snapshot(base, snapshot),
            XmlMutation::SetDeclaration { declaration } => {
                XmlDiff { declaration: Some(declaration.clone()), doctype: None, root: None }
            }
            XmlMutation::SetDoctype { doctype } => XmlDiff { declaration: None, doctype: Some(doctype.clone()), root: None },
            XmlMutation::InsertElement { path, index, node } => diff_at_path(
                &path.0,
                XmlNodeDiff::Element(XmlElementDiff {
                    name: None,
                    attributes: None,
                    children: Some(XmlChildrenDiff {
                        removed: Vec::new(),
                        modified: Vec::new(),
                        added: vec![XmlChildAdded { index: *index, item: node.clone() }],
                    }),
                }),
            ),
            XmlMutation::RemoveElement { path, index } => diff_at_path(
                &path.0,
                XmlNodeDiff::Element(XmlElementDiff {
                    name: None,
                    attributes: None,
                    children: Some(XmlChildrenDiff { removed: vec![*index], modified: Vec::new(), added: Vec::new() }),
                }),
            ),
            XmlMutation::SetAttribute { path, name, value } => {
                let target = path.resolve(base.doc.root.as_ref());
                let existing = target.and_then(|n| match n {
                    XmlNode::Element { attrs, .. } => attrs.iter().find(|a| &a.name == name),
                    _ => None,
                });
                let attrs_diff = match (existing, value) {
                    (Some(_), Some(v)) => XmlAttributesDiff {
                        removed: Vec::new(),
                        modified: vec![XmlAttrModified { name: name.clone(), value: v.clone() }],
                        added: Vec::new(),
                    },
                    (Some(_), None) => {
                        XmlAttributesDiff { removed: vec![name.clone()], modified: Vec::new(), added: Vec::new() }
                    }
                    (None, Some(v)) => {
                        let next_index = match target {
                            Some(XmlNode::Element { attrs, .. }) => attrs.len(),
                            _ => 0,
                        };
                        XmlAttributesDiff {
                            removed: Vec::new(),
                            modified: Vec::new(),
                            added: vec![XmlAttrAdded { index: next_index, name: name.clone(), value: v.clone() }],
                        }
                    }
                    (None, None) => XmlAttributesDiff::default(),
                };
                diff_at_path(&path.0, XmlNodeDiff::Element(XmlElementDiff { name: None, attributes: Some(attrs_diff), children: None }))
            }
            XmlMutation::SetText { path, text } => diff_at_path(&path.0, XmlNodeDiff::Text { text: Some(text.clone()) }),
        }
    }

    fn inverse(&self, base: &XmlSnapshot) -> Vec<Self> {
        match self {
            XmlMutation::NoMutation => vec![XmlMutation::NoMutation],
            XmlMutation::SetSnapshot { .. } => vec![XmlMutation::SetSnapshot { snapshot: base.clone() }],
            XmlMutation::SetDeclaration { .. } => vec![XmlMutation::SetDeclaration { declaration: base.doc.declaration.clone() }],
            XmlMutation::SetDoctype { .. } => vec![XmlMutation::SetDoctype { doctype: base.doc.doctype.clone() }],
            XmlMutation::InsertElement { path, index, .. } => {
                vec![XmlMutation::RemoveElement { path: path.clone(), index: *index }]
            }
            XmlMutation::RemoveElement { path, index } => {
                let parent = path.resolve(base.doc.root.as_ref());
                let node = parent
                    .and_then(|n| match n {
                        XmlNode::Element { children, .. } => children.get(*index).cloned(),
                        _ => None,
                    })
                    .unwrap_or(XmlNode::Text { text: String::new() });
                vec![XmlMutation::InsertElement { path: path.clone(), index: *index, node }]
            }
            XmlMutation::SetAttribute { path, name, .. } => {
                let target = path.resolve(base.doc.root.as_ref());
                let prior = target.and_then(|n| match n {
                    XmlNode::Element { attrs, .. } => attrs.iter().find(|a| &a.name == name).map(|a| a.value.clone()),
                    _ => None,
                });
                vec![XmlMutation::SetAttribute { path: path.clone(), name: name.clone(), value: prior }]
            }
            XmlMutation::SetText { path, .. } => {
                let prior = path
                    .resolve(base.doc.root.as_ref())
                    .and_then(|n| match n {
                        XmlNode::Text { text } => Some(text.clone()),
                        _ => None,
                    })
                    .unwrap_or_default();
                vec![XmlMutation::SetText { path: path.clone(), text: prior }]
            }
        }
    }
}

/// 🧭️ `path`-addressing convenience over `crate::artifacts::xml::schema::diff::diff_at_path`
/// (which takes a bare `&[usize]` so the diff module never needs to depend on this one).
fn diff_at_path(path: &[usize], leaf: XmlNodeDiff) -> XmlDiff {
    crate::artifacts::xml::schema::diff::diff_at_path(path, leaf)
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
impl protocol::OpText for XmlMutation {
    fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".into())
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line.trim()).map_err(|e| {
            store::TextError::new(format!("op parse: {e}"), dsl::TextSpan::at(1, 1))
        })
    }
}

impl protocol::OpBinary for XmlMutation {
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
