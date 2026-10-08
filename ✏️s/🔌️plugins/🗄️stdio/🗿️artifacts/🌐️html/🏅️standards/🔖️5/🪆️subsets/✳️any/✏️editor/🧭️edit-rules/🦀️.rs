//! 🧭️ The details-pane vocabulary of the html editor: a snapshot pointer walks the tree (`/root`, then `children/<i>` per level) to the node
//! it addresses and raises that node's kind: a node insert and remove at its parent, a text / comment / raw-text set, an element rename,
//! an attribute set / remove by name, the doctype whole, and every other node edit as a remove then an insert at the same position.
//! The table itself is empty because the kinds are addressed by a recursive path, not by a fixed pointer shape.

use crate::standards::v5::subsets::any::schema::mutations::{insert_node, remove_node, set_attribute, set_comment, set_doctype, set_element_name, set_raw_text, set_text, HtmlMutation};
use crate::standards::v5::subsets::any::schema::snapshot::{HtmlAttr, HtmlNode, HtmlSnapshot};
use semio_framework_plugin::Fault;
use semio_framework_value::{DslValue, FromValue, ToValue};
use semio_s_artifact_stdio_contract::editing::{edited_subtree, EditRules, SnapshotEditEvent};

/// 📚 No fixed pointer shape: see [`resolve`].
pub const EDIT_RULES: EditRules = EditRules { entities: &[], inserts: &[], removes: &[] };

fn fail(message: impl Into<String>) -> Fault {
    Fault::from(message.into())
}

fn segments_of(pointer: &str) -> Result<Vec<String>, Fault> {
    if pointer.is_empty() {
        return Ok(Vec::new());
    }
    pointer.strip_prefix('/').map(|rest| rest.split('/').map(|raw| raw.replace("~1", "/").replace("~0", "~")).collect()).ok_or_else(|| fail(format!("'{pointer}' is not a pointer")))
}

fn index_of(segment: &str, length: usize, insertion: bool) -> Result<usize, Fault> {
    let index = if insertion && segment == "-" { length } else { segment.parse::<usize>().map_err(|error| fail(error.to_string()))? };
    (index < length || (insertion && index == length)).then_some(index).ok_or_else(|| fail(format!("index {index} is outside 0..{length}")))
}

struct Located<'a> {
    path: Vec<usize>,
    node: &'a HtmlNode,
    pointer: String,
    consumed: usize,
}

fn locate<'a>(root: &'a HtmlNode, segments: &[String], container_edit: bool) -> Result<Located<'a>, Fault> {
    let mut located = Located { path: Vec::new(), node: root, pointer: "/root".to_string(), consumed: 0 };
    loop {
        let rest = &segments[located.consumed..];
        match (located.node, rest) {
            (HtmlNode::Element { children, .. }, [key, index, ..]) if key == "children" && !(container_edit && rest.len() == 2) => {
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

fn replace_node(path: &[usize], edited: HtmlNode) -> Result<Vec<HtmlMutation>, Fault> {
    let (index, parent) = path.split_last().ok_or_else(|| fail("no kind rewrites the root node into another node kind"))?;
    Ok(vec![
        HtmlMutation::RemoveNode(remove_node::RemoveNode { parent: parent.to_vec(), index: *index }),
        HtmlMutation::InsertNode(insert_node::InsertNode { parent: parent.to_vec(), index: *index, node: edited }),
    ])
}

/// 🎯 The concrete kinds an edit of the html document denotes, or `None` when the pointer is outside `/root` and `/doctype`.
pub fn resolve(snapshot: &HtmlSnapshot, event: &SnapshotEditEvent) -> Result<Option<Vec<HtmlMutation>>, Fault> {
    let (path, container_edit) = match event {
        SnapshotEditEvent::SetValue { path, .. } | SnapshotEditEvent::RenameKey { path, .. } | SnapshotEditEvent::MoveValue { path, .. } => (path, false),
        SnapshotEditEvent::InsertValue { path, .. } | SnapshotEditEvent::RemoveValue { path } => (path, true),
        SnapshotEditEvent::ReplaceSource { .. } => return Ok(None),
    };
    let segments = segments_of(path)?;
    match segments.as_slice() {
        [key] if key == "doctype" => {
            let SnapshotEditEvent::SetValue { value, .. } = event else { return Err(fail("a doctype is set, not inserted or removed")) };
            let doctype = <Option<String>>::from_value(value.clone()).map_err(|error| fail(error.to_string()))?;
            Ok(Some(if doctype == snapshot.doctype { Vec::new() } else { vec![HtmlMutation::SetDoctype(set_doctype::SetDoctype { doctype })] }))
        }
        [key, tail @ ..] if key == "root" => {
            let located = locate(&snapshot.root, tail, container_edit)?;
            let rest = &tail[located.consumed..];
            let at = located.path.clone();
            let leaves = match (event, located.node, rest) {
                (SnapshotEditEvent::InsertValue { value, .. }, HtmlNode::Element { children, .. }, [key, index]) if key == "children" => {
                    let node = HtmlNode::from_value(value.clone()).map_err(|error| fail(error.to_string()))?;
                    vec![HtmlMutation::InsertNode(insert_node::InsertNode { parent: at, index: index_of(index, children.len(), true)?, node })]
                }
                (SnapshotEditEvent::RemoveValue { .. }, HtmlNode::Element { children, .. }, [key, index]) if key == "children" => {
                    vec![HtmlMutation::RemoveNode(remove_node::RemoveNode { parent: at, index: index_of(index, children.len(), false)? })]
                }
                (SnapshotEditEvent::SetValue { value: DslValue::String(text), .. }, HtmlNode::Text { .. }, [field]) if field == "text" => vec![HtmlMutation::SetText(set_text::SetText { path: at, text: text.clone() })],
                (SnapshotEditEvent::SetValue { value: DslValue::String(text), .. }, HtmlNode::Comment { .. }, [field]) if field == "text" => vec![HtmlMutation::SetComment(set_comment::SetComment { path: at, text: text.clone() })],
                (SnapshotEditEvent::SetValue { value: DslValue::String(text), .. }, HtmlNode::RawText { .. }, [field]) if field == "text" => vec![HtmlMutation::SetRawText(set_raw_text::SetRawText { path: at, text: text.clone() })],
                (SnapshotEditEvent::SetValue { value: DslValue::String(name), .. }, HtmlNode::Element { .. }, [field]) if field == "name" => vec![HtmlMutation::SetElementName(set_element_name::SetElementName { path: at, name: name.clone() })],
                (SnapshotEditEvent::SetValue { value, .. }, HtmlNode::Element { attributes, .. }, [key, index, field]) if key == "attributes" && field == "value" => {
                    let attribute = &attributes[index_of(index, attributes.len(), false)?];
                    let new_value = <Option<String>>::from_value(value.clone()).map_err(|error| fail(error.to_string()))?;
                    vec![HtmlMutation::SetAttribute(set_attribute::SetAttribute { path: at, name: attribute.name.clone(), value: Some(new_value) })]
                }
                (SnapshotEditEvent::InsertValue { value, .. }, HtmlNode::Element { attributes, .. }, [key, index]) if key == "attributes" && index_of(index, attributes.len(), true)? == attributes.len() => {
                    let attribute = HtmlAttr::from_value(value.clone()).map_err(|error| fail(error.to_string()))?;
                    vec![HtmlMutation::SetAttribute(set_attribute::SetAttribute { path: at, name: attribute.name, value: Some(attribute.value) })]
                }
                (SnapshotEditEvent::RemoveValue { .. }, HtmlNode::Element { attributes, .. }, [key, index]) if key == "attributes" => {
                    let attribute = &attributes[index_of(index, attributes.len(), false)?];
                    vec![HtmlMutation::SetAttribute(set_attribute::SetAttribute { path: at, name: attribute.name.clone(), value: None })]
                }
                _ => {
                    let edited = HtmlNode::from_value(edited_subtree(&located.node.to_value(), &located.pointer, event).map_err(|error| fail(error.to_string()))?).map_err(|error| fail(error.to_string()))?;
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
