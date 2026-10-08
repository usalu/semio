//! 🧰 Shared attribute diff construction for direct SVG mutations.
use crate::schema::diff::{diff_at_path, SvgAttrAdded, SvgAttrModified, SvgAttributesDiff, SvgDiff, SvgElementDiff, SvgNodeDiff};
use crate::schema::snapshot::{element_attr, node_at, NodePath, SvgAttr, SvgAttributeValue, SvgNode};
use crate::SvgSnapshot;
use semio_framework_value::FromValue;
use semio_s_artifact_stdio_contract::editing::{SnapshotEditError, SnapshotEditEvent};

/// 🏷️ One attribute transition: the name, the value to hold (`None` removes it) and the position it takes when it is added.
pub type AttributeChange<'a> = (&'a str, Option<SvgAttributeValue>, Option<usize>);

/// 🏷️ The sparse diff that carries the element at `path` through `changes`: unchanged values contribute nothing, a new
/// attribute is added at its position (last by default), and the empty diff results when nothing changes.
pub fn attributes_diff_at_path(base: &SvgSnapshot, path: &[usize], changes: &[AttributeChange<'_>]) -> SvgDiff {
    let existing: &[SvgAttr] = match node_at(&base.doc, path) {
        Ok(SvgNode::Element { attrs, .. }) => attrs.as_slice(),
        _ => &[],
    };
    let present = |name: &str| existing.iter().find(|attribute| attribute.name == name);
    let removed_count = changes.iter().filter(|(name, value, _)| value.is_none() && present(name).is_some()).count();
    let added_count = changes.iter().filter(|(name, value, _)| value.is_some() && present(name).is_none()).count();
    let final_length = existing.len() - removed_count + added_count;
    let mut append_at = existing.len() - removed_count;
    let mut diff = SvgAttributesDiff::default();
    for (name, value, index) in changes {
        match (present(name), value) {
            (Some(current), Some(value)) if current.value != *value => diff.modified.push(SvgAttrModified { name: (*name).to_string(), value: value.clone() }),
            (Some(_), None) => diff.removed.push((*name).to_string()),
            (None, Some(value)) => {
                let at = index.map_or(append_at, |index| index.min(final_length - 1));
                append_at += 1;
                diff.added.push(SvgAttrAdded { index: at, name: (*name).to_string(), value: value.clone() });
            }
            _ => {}
        }
    }
    if diff.removed.is_empty() && diff.modified.is_empty() && diff.added.is_empty() {
        return SvgDiff::default();
    }
    diff_at_path(path, SvgNodeDiff::Element(SvgElementDiff { name: None, attributes: Some(diff), children: None }))
}

/// 🏷️ The sparse diff that sets, adds (at `index`, default last) or removes attribute `name` on the element at `path`.
pub fn attribute_diff_at_path(base: &SvgSnapshot, path: &[usize], name: &str, value: Option<SvgAttributeValue>, index: Option<usize>) -> SvgDiff {
    attributes_diff_at_path(base, path, &[(name, value, index)])
}

/// 🔎️ The current value and position of attribute `name` on the element at `path`, as the `(value, index)` pair a restoring
/// mutation sets back.
pub fn prior_attribute(base: &SvgSnapshot, path: &[usize], name: &str) -> (Option<SvgAttributeValue>, Option<usize>) {
    match node_at(&base.doc, path) {
        Ok(SvgNode::Element { attrs, .. }) => match attrs.iter().position(|attribute| attribute.name == name) {
            Some(position) => (Some(attrs[position].value.clone()), Some(position)),
            None => (None, None),
        },
        _ => (None, None),
    }
}

/// 🧰️ The kinds one subset raises for a gesture on the element tree: each subset names its own attribute and element leaves.
pub trait TreeEditKit {
    type Mutation;

    /// 🏷️ Sets, adds at `index` or removes (`None`) attribute `name` of the element at `path`.
    fn set_attribute(path: NodePath, name: String, value: Option<SvgAttributeValue>, index: Option<usize>) -> Self::Mutation;
    /// ➕ Inserts `node` at child position `index` of the element at `parent`.
    fn insert_element(parent: NodePath, index: usize, node: SvgNode) -> Self::Mutation;
    /// ➖ Removes the child at `index` of the element at `parent`.
    fn remove_element(parent: NodePath, index: usize) -> Self::Mutation;
    /// ✍️ Sets the text of the text node at `path`.
    fn set_text(path: NodePath, text: String) -> Self::Mutation;
    /// 🔤️ Renames the element at `path`; `None` when the subset has no such kind.
    fn set_element_name(_path: NodePath, _name: String) -> Option<Self::Mutation> {
        None
    }
    /// 📣️ Sets the XML declaration; `None` when the subset has no such kind.
    fn set_declaration(_declaration: Option<semio_s_artifact_stdio_xml::schema::snapshot::XmlDeclaration>) -> Option<Self::Mutation> {
        None
    }
    /// 📜️ Sets the doctype; `None` when the subset has no such kind.
    fn set_doctype(_doctype: Option<semio_s_artifact_stdio_xml::schema::snapshot::XmlDoctype>) -> Option<Self::Mutation> {
        None
    }
}

fn tree_refusal(path: &str, message: impl Into<String>) -> SnapshotEditError {
    SnapshotEditError::new("snapshot-edit.schema-invalid", path, message)
}

fn tree_pointer(path: &str) -> Option<Vec<&str>> {
    let mut segments = path.strip_prefix("/doc/root")?.split('/');
    (segments.next()? == "").then_some(())?;
    Some(segments.collect())
}

/// 📍 The element path a pointer navigates through `children/<n>` pairs and what remains below it.
fn locate<'a>(segments: &'a [&'a str], path: &str) -> Result<(NodePath, &'a [&'a str]), SnapshotEditError> {
    let mut node = NodePath::new();
    let mut rest = segments;
    while rest.len() >= 3 && rest[0] == "children" {
        node.push(rest[1].parse().map_err(|_| SnapshotEditError::new("snapshot-edit.index-out-of-bounds", path, format!("'{}' is no child position", rest[1])))?);
        rest = &rest[2..];
    }
    Ok((node, rest))
}

fn position(segment: &str, length: usize, path: &str, insert: bool) -> Result<usize, SnapshotEditError> {
    let at = if insert && segment == "-" { length } else { segment.parse().map_err(|_| SnapshotEditError::new("snapshot-edit.index-out-of-bounds", path, format!("'{segment}' is no list position")))? };
    if at > length || (!insert && at == length) {
        return Err(SnapshotEditError::new("snapshot-edit.index-out-of-bounds", path, format!("position {at} is outside the {length} rows")));
    }
    Ok(at)
}

fn element_parts<'a>(base: &'a SvgSnapshot, path: &[usize], pointer: &str) -> Result<(&'a [SvgAttr], &'a [SvgNode]), SnapshotEditError> {
    match node_at(&base.doc, path) {
        Ok(SvgNode::Element { attrs, children, .. }) => Ok((attrs, children)),
        Ok(_) => Err(tree_refusal(pointer, "the addressed node is not an element")),
        Err(message) => Err(SnapshotEditError::new("snapshot-edit.path-missing", pointer, message)),
    }
}

fn decode<T: FromValue>(value: &semio_framework_value::DslValue, pointer: &str) -> Result<T, SnapshotEditError> {
    T::from_value(value.clone()).map_err(|error| tree_refusal(pointer, error.to_string()))
}

fn moved_attribute<K: TreeEditKit>(base: &SvgSnapshot, from: &str, path: &str) -> Result<Option<Vec<K::Mutation>>, SnapshotEditError> {
    let (Some(origin), Some(destination)) = (tree_pointer(from), tree_pointer(path)) else { return Ok(None) };
    let ((node, from_rest), (target, rest)) = (locate(&origin, from)?, locate(&destination, path)?);
    if node != target || from_rest.len() != 2 || rest.len() != 2 || from_rest[0] != rest[0] {
        return Ok(None);
    }
    match from_rest[0] {
        "attrs" => {
            let (attrs, _) = element_parts(base, &node, from)?;
            let origin = position(from_rest[1], attrs.len(), from, false)?;
            let destination = position(rest[1], attrs.len(), path, false)?;
            if origin == destination {
                return Ok(Some(Vec::new()));
            }
            let moved = &attrs[origin];
            Ok(Some(vec![K::set_attribute(node.clone(), moved.name.clone(), None, None), K::set_attribute(node, moved.name.clone(), Some(moved.value.clone()), Some(destination))]))
        }
        "children" => {
            let (_, children) = element_parts(base, &node, from)?;
            let origin = position(from_rest[1], children.len(), from, false)?;
            let destination = position(rest[1], children.len(), path, false)?;
            if origin == destination {
                return Ok(Some(Vec::new()));
            }
            Ok(Some(vec![K::remove_element(node.clone(), origin), K::insert_element(node, destination, children[origin].clone())]))
        }
        _ => Ok(None),
    }
}

/// 🌳 The kinds one edit of the element tree below `/doc/root` raises, addressed by node path (`children/<n>` pairs), attribute position and text;
/// `None` for a pointer outside the tree (the document's own fields go through the rules table).
pub fn tree_edit<K: TreeEditKit>(event: &SnapshotEditEvent, base: &SvgSnapshot) -> Result<Option<Vec<K::Mutation>>, SnapshotEditError> {
    if let SnapshotEditEvent::MoveValue { from, path } = event {
        return moved_attribute::<K>(base, from, path);
    }
    let (path, value) = match event {
        SnapshotEditEvent::SetValue { path, value } | SnapshotEditEvent::InsertValue { path, value } => (path.as_str(), Some(value)),
        SnapshotEditEvent::RemoveValue { path } => (path.as_str(), None),
        SnapshotEditEvent::RenameKey { .. } | SnapshotEditEvent::MoveValue { .. } | SnapshotEditEvent::ReplaceSource { .. } => return Ok(None),
    };
    let Some(segments) = tree_pointer(path) else { return Ok(None) };
    let (node, rest) = locate(&segments, path)?;
    let setting = matches!(event, SnapshotEditEvent::SetValue { .. });
    let inserting = matches!(event, SnapshotEditEvent::InsertValue { .. });
    let attrs = element_parts(base, &node, path).map_or(&[][..], |(attrs, _)| attrs);
    Ok(match (rest, value) {
        (["name"], Some(value)) if setting => K::set_element_name(node, decode(value, path)?).map(|mutation| vec![mutation]),
        (["text"], Some(value)) if setting => Some(vec![K::set_text(node, decode(value, path)?)]),
        (["children", row], _) => {
            let (_, children) = element_parts(base, &node, path)?;
            match value {
                Some(value) if inserting => Some(vec![K::insert_element(node, position(row, children.len(), path, true)?, decode(value, path)?)]),
                Some(value) => {
                    let at = position(row, children.len(), path, false)?;
                    let replacement: SvgNode = decode(value, path)?;
                    Some(if children[at] == replacement { Vec::new() } else { vec![K::remove_element(node.clone(), at), K::insert_element(node, at, replacement)] })
                }
                None => Some(vec![K::remove_element(node, position(row, children.len(), path, false)?)]),
            }
        }
        (["attrs", row], _) => match value {
            Some(value) if inserting => {
                let attribute: SvgAttr = decode(value, path)?;
                Some(vec![K::set_attribute(node, attribute.name, Some(attribute.value), Some(position(row, attrs.len(), path, true)?))])
            }
            Some(value) => {
                let at = position(row, attrs.len(), path, false)?;
                let attribute: SvgAttr = decode(value, path)?;
                let current = &attrs[at];
                Some(if *current == attribute {
                    Vec::new()
                } else if current.name == attribute.name {
                    vec![K::set_attribute(node, attribute.name, Some(attribute.value), None)]
                } else {
                    vec![K::set_attribute(node.clone(), current.name.clone(), None, None), K::set_attribute(node, attribute.name, Some(attribute.value), Some(at))]
                })
            }
            None => Some(vec![K::set_attribute(node, attrs[position(row, attrs.len(), path, false)?].name.clone(), None, None)]),
        },
        (["attrs", row, field @ ("name" | "value")], Some(value)) if setting => {
            let at = position(row, attrs.len(), path, false)?;
            let current = &attrs[at];
            Some(match *field {
                "value" => {
                    let next: SvgAttributeValue = decode(value, path)?;
                    if current.value == next { Vec::new() } else { vec![K::set_attribute(node, current.name.clone(), Some(next), None)] }
                }
                _ => {
                    let name: String = decode(value, path)?;
                    if current.name == name { Vec::new() } else { vec![K::set_attribute(node.clone(), current.name.clone(), None, None), K::set_attribute(node, name, Some(current.value.clone()), Some(at))] }
                }
            })
        }
        _ => None,
    })
}

/// 🖼️ The kinds that make the root element of `base` hold the content of the root element of `region` — the gesture of replacing the drawing by a
/// described one: every attribute of `region` is set (new ones at their position), every other attribute of `base` is removed, then the children of `base`
/// are removed last to first and the children of `region` are inserted in order. Each row is the concrete kind of its part, so the gesture undoes row by row.
/// The XML declaration and the doctype follow `region` where the subset has a kind for them.
pub fn region_edit<K: TreeEditKit>(base: &SvgSnapshot, region: &SvgSnapshot) -> Result<Vec<K::Mutation>, String> {
    let (Some(SvgNode::Element { name: base_name, attrs: base_attrs, children: base_children }), Some(SvgNode::Element { name, attrs, children })) = (&base.doc.root, &region.doc.root) else {
        return Err("a drawing region replaces the content of an element root".into());
    };
    let mut rows = Vec::new();
    if base.doc.declaration != region.doc.declaration {
        rows.extend(K::set_declaration(region.doc.declaration.clone()));
    }
    if base.doc.doctype != region.doc.doctype {
        rows.extend(K::set_doctype(region.doc.doctype.clone()));
    }
    if base_name != name {
        rows.push(K::set_element_name(NodePath::new(), name.clone()).ok_or_else(|| format!("this profile cannot rename the root element {base_name:?} to {name:?}"))?);
    }
    rows.extend(base_attrs.iter().filter(|attribute| !attrs.iter().any(|kept| kept.name == attribute.name)).map(|attribute| K::set_attribute(NodePath::new(), attribute.name.clone(), None, None)));
    rows.extend(attrs.iter().enumerate().map(|(index, attribute)| {
        let known = base_attrs.iter().any(|existing| existing.name == attribute.name);
        K::set_attribute(NodePath::new(), attribute.name.clone(), Some(attribute.value.clone()), (!known).then_some(index))
    }));
    rows.extend((0..base_children.len()).rev().map(|index| K::remove_element(NodePath::new(), index)));
    rows.extend(children.iter().enumerate().map(|(index, child)| K::insert_element(NodePath::new(), index, child.clone())));
    Ok(rows)
}
