//! 🧭️ The details-pane vocabulary of the xml editors: a snapshot pointer walks the document (`/doc/root`, then `children/<i>` per level) to
//! the node it addresses and raises that node's kind: an element insert and remove at its parent, an attribute set / insert / remove by name,
//! a text set, the declaration and the doctype whole, and every other node edit as a remove then an insert at the same position.
//! The table itself is empty because the kinds are addressed by a recursive path, not by a fixed pointer shape.

use crate::schema::mutations::{InsertElementPayload, RemoveElementPayload, SetAttributePayload, SetDeclarationPayload, SetDoctypePayload, SetTextPayload, XmlMutation, XmlNodePath};
use crate::schema::snapshot::{XmlAttr, XmlDeclaration, XmlDoctype, XmlNode};
use crate::XmlSnapshot;
use semio_framework_plugin::Fault;
use semio_framework_value::{DslValue, FromValue, ToValue};
use semio_s_artifact_stdio_contract::editing::{edited_subtree, EditRules, SnapshotEditEvent};

/// 📚 No fixed pointer shape: see [`resolve`].
pub const EDIT_RULES: EditRules = EditRules { entities: &[], inserts: &[], removes: &[] };

fn fail(message: impl Into<String>) -> Fault {
    Fault::from(message.into())
}

/// 🧭️ The pointer's segments, unescaped.
pub fn segments_of(pointer: &str) -> Result<Vec<String>, Fault> {
    if pointer.is_empty() {
        return Ok(Vec::new());
    }
    pointer.strip_prefix('/').map(|rest| rest.split('/').map(|raw| raw.replace("~1", "/").replace("~0", "~")).collect()).ok_or_else(|| fail(format!("'{pointer}' is not a pointer")))
}

fn index_of(segment: &str, length: usize, insertion: bool) -> Result<usize, Fault> {
    let index = if insertion && segment == "-" { length } else { segment.parse::<usize>().map_err(|error| fail(error.to_string()))? };
    (index < length || (insertion && index == length)).then_some(index).ok_or_else(|| fail(format!("index {index} is outside 0..{length}")))
}

/// 📍 The node a pointer reaches below the root: its child-index path, the node, its own pointer and how many segments were consumed.
pub struct Located<'a> {
    pub path: Vec<usize>,
    pub node: &'a XmlNode,
    pub pointer: String,
    pub consumed: usize,
}

/// 🧭️ Walks `children/<i>` per level from the root; a child insert or remove stops at its parent (`container_edit`).
pub fn locate<'a>(root: &'a XmlNode, segments: &[String], container_edit: bool) -> Result<Located<'a>, Fault> {
    let mut located = Located { path: Vec::new(), node: root, pointer: "/doc/root".to_string(), consumed: 0 };
    loop {
        let rest = &segments[located.consumed..];
        match (located.node, rest) {
            (XmlNode::Element { children, .. }, [key, index, ..]) if key == "children" && !(container_edit && rest.len() == 2) => {
                let child = index_of(index, children.len(), false)?;
                located.path.push(child);
                located.pointer = format!("{}/children/{child}", located.pointer);
                located.node = &children[child];
                located.consumed += 2;
            }
            _ => return Ok(located),
        }
    }
}

fn replace_node(path: &[usize], edited: XmlNode) -> Result<Vec<XmlMutation>, Fault> {
    let (index, parent) = path.split_last().ok_or_else(|| fail("no kind rewrites the document element itself"))?;
    Ok(vec![
        XmlMutation::RemoveElement(RemoveElementPayload { path: XmlNodePath(parent.to_vec()), index: *index }),
        XmlMutation::InsertElement(InsertElementPayload { path: XmlNodePath(parent.to_vec()), index: *index, node: edited }),
    ])
}

/// 🎯 The concrete kinds an edit of the xml document denotes, or `None` when the pointer is outside `/doc`.
pub fn resolve(snapshot: &XmlSnapshot, event: &SnapshotEditEvent) -> Result<Option<Vec<XmlMutation>>, Fault> {
    let (path, container_edit) = match event {
        SnapshotEditEvent::SetValue { path, .. } | SnapshotEditEvent::RenameKey { path, .. } | SnapshotEditEvent::MoveValue { path, .. } => (path, false),
        SnapshotEditEvent::InsertValue { path, .. } | SnapshotEditEvent::RemoveValue { path } => (path, true),
        SnapshotEditEvent::ReplaceSource { .. } => return Ok(None),
    };
    let segments = segments_of(path)?;
    let edit_of = |subtree: DslValue, prefix: &str| edited_subtree(&subtree, prefix, event).map_err(|error| fail(error.to_string()));
    match segments.as_slice() {
        [doc, key, ..] if doc == "doc" && key == "declaration" => {
            let edited = edit_of(snapshot.doc.declaration.as_ref().map_or(DslValue::Null, ToValue::to_value), "/doc/declaration")?;
            let declaration = <Option<XmlDeclaration>>::from_value(edited).map_err(|error| fail(error.to_string()))?;
            Ok(Some(if declaration == snapshot.doc.declaration { Vec::new() } else { vec![XmlMutation::SetDeclaration(SetDeclarationPayload { declaration })] }))
        }
        [doc, key, ..] if doc == "doc" && key == "doctype" => {
            let edited = edit_of(snapshot.doc.doctype.as_ref().map_or(DslValue::Null, ToValue::to_value), "/doc/doctype")?;
            let doctype = <Option<XmlDoctype>>::from_value(edited).map_err(|error| fail(error.to_string()))?;
            Ok(Some(if doctype == snapshot.doc.doctype { Vec::new() } else { vec![XmlMutation::SetDoctype(SetDoctypePayload { doctype })] }))
        }
        [doc, key, tail @ ..] if doc == "doc" && key == "root" => {
            let root = snapshot.doc.root.as_ref().ok_or_else(|| fail("the document has no root element"))?;
            let located = locate(root, tail, container_edit)?;
            let rest = &tail[located.consumed..];
            let at = XmlNodePath(located.path.clone());
            let leaves = match (event, located.node, rest) {
                (SnapshotEditEvent::InsertValue { value, .. }, XmlNode::Element { children, .. }, [key, index]) if key == "children" => {
                    let node = XmlNode::from_value(value.clone()).map_err(|error| fail(error.to_string()))?;
                    vec![XmlMutation::InsertElement(InsertElementPayload { path: at, index: index_of(index, children.len(), true)?, node })]
                }
                (SnapshotEditEvent::RemoveValue { .. }, XmlNode::Element { children, .. }, [key, index]) if key == "children" => {
                    vec![XmlMutation::RemoveElement(RemoveElementPayload { path: at, index: index_of(index, children.len(), false)? })]
                }
                (SnapshotEditEvent::SetValue { value: DslValue::String(text), .. }, XmlNode::Text { .. }, [key]) if key == "text" => vec![XmlMutation::SetText(SetTextPayload { path: at, text: text.clone() })],
                (SnapshotEditEvent::SetValue { value: DslValue::String(value), .. }, XmlNode::Element { attrs, .. }, [key, index, field]) if key == "attrs" && field == "value" => {
                    let attribute = &attrs[index_of(index, attrs.len(), false)?];
                    vec![XmlMutation::SetAttribute(SetAttributePayload { path: at, name: attribute.name.clone(), value: Some(value.clone()), index: None })]
                }
                (SnapshotEditEvent::SetValue { value: DslValue::String(renamed), .. }, XmlNode::Element { attrs, .. }, [key, index, field]) if key == "attrs" && field == "name" => {
                    let position = index_of(index, attrs.len(), false)?;
                    let attribute = &attrs[position];
                    vec![
                        XmlMutation::SetAttribute(SetAttributePayload { path: at.clone(), name: attribute.name.clone(), value: None, index: None }),
                        XmlMutation::SetAttribute(SetAttributePayload { path: at, name: renamed.clone(), value: Some(attribute.value.clone()), index: Some(position) }),
                    ]
                }
                (SnapshotEditEvent::InsertValue { value, .. }, XmlNode::Element { attrs, .. }, [key, index]) if key == "attrs" => {
                    let attribute = XmlAttr::from_value(value.clone()).map_err(|error| fail(error.to_string()))?;
                    vec![XmlMutation::SetAttribute(SetAttributePayload { path: at, name: attribute.name, value: Some(attribute.value), index: Some(index_of(index, attrs.len(), true)?) })]
                }
                (SnapshotEditEvent::RemoveValue { .. }, XmlNode::Element { attrs, .. }, [key, index]) if key == "attrs" => {
                    let attribute = &attrs[index_of(index, attrs.len(), false)?];
                    vec![XmlMutation::SetAttribute(SetAttributePayload { path: at, name: attribute.name.clone(), value: None, index: None })]
                }
                _ => {
                    let edited = XmlNode::from_value(edit_of(located.node.to_value(), &located.pointer)?).map_err(|error| fail(error.to_string()))?;
                    if &edited == located.node {
                        Vec::new()
                    } else {
                        replace_node(&located.path, edited)?
                    }
                }
            };
            Ok(Some(leaves))
        }
        _ => Ok(None),
    }
}
