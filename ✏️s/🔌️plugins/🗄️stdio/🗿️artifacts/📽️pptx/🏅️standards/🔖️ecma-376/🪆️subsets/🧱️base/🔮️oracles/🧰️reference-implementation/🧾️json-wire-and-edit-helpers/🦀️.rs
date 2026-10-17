//! 🧾️ Wire and edit helpers of the PPTX oracle: the JSON projections, the wire node codec, the node addressing, the vacancy and removal edits and the text and attribute setters. See [`super`].

use super::*;

//#region 🔖️JsonValue
fn transform_to_json(t: Transform) -> Json {
    Json::Object(vec![("x".into(), Json::String(t.x.to_string())), ("y".into(), Json::String(t.y.to_string())), ("cx".into(), Json::String(t.cx.to_string())), ("cy".into(), Json::String(t.cy.to_string()))])
}

fn shape_to_json(shape: &PShape) -> Json {
    match shape {
        PShape::TextBox { text, position } => Json::Object(vec![("kind".into(), Json::String("textBox".into())), ("text".into(), Json::String(text.clone())), ("position".into(), transform_to_json(*position))]),
        PShape::Placeholder { kind, text, position } => {
            Json::Object(vec![("kind".into(), Json::String("placeholder".into())), ("phKind".into(), Json::String(kind.clone())), ("text".into(), Json::String(text.clone())), ("position".into(), transform_to_json(*position))])
        }
        PShape::Picture { blip_rel_id, position } => Json::Object(vec![("kind".into(), Json::String("picture".into())), ("blipRelId".into(), Json::String(blip_rel_id.clone())), ("position".into(), transform_to_json(*position))]),
        PShape::Other { .. } => Json::Object(vec![("kind".into(), Json::String("other".into()))]),
    }
}

pub(super) fn slide_to_json(slide: &PSlide) -> Json {
    Json::Object(vec![("shapeCount".into(), Json::Number(slide.shapes.len() as f64)), ("shapes".into(), Json::Array(slide.shapes.iter().map(shape_to_json).collect()))])
}
//#endregion 🔖️JsonValue

//#region 🔖️Forward
/// 🌳 One wire `XmlNode` as this model's node: elements and text (CDATA joins the text), anything else refused.
pub(super) fn wire_node(value: &Json) -> Result<XNode, String> {
    match value.str("kind").as_str() {
        "element" => Ok(XNode::Element {
            name: value.str("name"),
            attrs: value.array("attrs").iter().map(|attr| (attr.str("name"), attr.str("value"))).collect(),
            children: value.array("children").iter().map(wire_node).collect::<Result<_, _>>()?,
        }),
        "text" | "cData" => Ok(XNode::Text(value.str("text"))),
        other => Err(format!("wire XML node kind {other:?} is outside what this oracle models")),
    }
}

/// 🌳 This model's node as the wire `XmlNode` the subject's snapshot carries.
fn node_wire(node: &XNode) -> Json {
    match node {
        XNode::Text(text) => Json::Object(vec![("kind".into(), Json::String("text".into())), ("text".into(), Json::String(text.clone()))]),
        XNode::Element { name, attrs, children } => Json::Object(vec![
            ("kind".into(), Json::String("element".into())),
            ("name".into(), Json::String(name.clone())),
            ("attrs".into(), Json::Array(attrs.iter().map(|(key, value)| Json::Object(vec![("name".into(), Json::String(key.clone())), ("value".into(), Json::String(value.clone()))])).collect())),
            ("children".into(), Json::Array(children.iter().map(node_wire).collect())),
        ]),
    }
}

/// 🧭️ A wire `PptxXmlAddress` as its part path and child-index node path.
pub(super) fn located(address: &Json) -> Result<(String, Vec<usize>), String> {
    let path = address.array("nodePath").iter().map(|index| match index {
        Json::Number(value) if *value >= 0.0 && value.fract() == 0.0 => Ok(*value as usize),
        other => Err(format!("node path step {} is no index", other.to_string())),
    });
    Ok((address.str("partPath"), path.collect::<Result<_, String>>()?))
}

pub(super) fn node_at<'a>(node: &'a mut XNode, path: &[usize]) -> Result<&'a mut XNode, String> {
    path.iter().try_fold(node, |node, index| match node {
        XNode::Element { children, .. } => children.get_mut(*index).ok_or_else(|| format!("node path step {index} is outside the tree")),
        XNode::Text(_) => Err("a node path traverses text".to_string()),
    })
}

pub(super) fn children_at<'a>(root: &'a mut XNode, path: &[usize]) -> Result<&'a mut Vec<XNode>, String> {
    match node_at(root, path)? {
        XNode::Element { children, .. } => Ok(children),
        XNode::Text(_) => Err("an addressed container is text".to_string()),
    }
}

/// ✏️ Parses one part, applies `edit` to its root and writes the part back re-serialized.
pub(super) fn edit_part(pkg: &mut Package, part: &str, edit: impl FnOnce(&mut XNode) -> Result<(), String>) -> Result<(), String> {
    let mut root = pkg.xml(part)?;
    edit(&mut root)?;
    pkg.parts.insert(part.to_string(), serialize_document(&root)?);
    Ok(())
}

pub(super) const CONTENT_TYPES_PART: &str = "[Content_Types].xml";

pub(super) const RELATIONSHIPS_NS: &str = "http://schemas.openxmlformats.org/package/2006/relationships";

/// 📍️ The optional insertion `index` of a wire payload (`None` appends).
pub(super) fn optional_index(value: &Json) -> Option<usize> {
    match value.get("index") {
        Some(Json::Number(number)) if *number >= 0.0 => Some(*number as usize),
        _ => None,
    }
}

/// 📍️ The physical child position a new `tag` row takes: before the `index`-th existing row of its tag, else after the last of its tag — a content-types
/// default after the last default, an override and a relationship at the end.
pub(super) fn insertion_slot(children: &[XNode], tag: &str, index: Option<usize>) -> usize {
    let slots: Vec<usize> = children.iter().enumerate().filter(|(_, child)| is_named(child, tag)).map(|(at, _)| at).collect();
    match index.and_then(|at| slots.get(at).copied()) {
        Some(physical) => physical,
        None => match slots.last() {
            Some(last) => last + 1,
            None if tag == "Default" => 0,
            None => children.len(),
        },
    }
}

pub(super) fn is_named(node: &XNode, wanted: &str) -> bool {
    matches!(node, XNode::Element { name, .. } if name == wanted)
}

/// 🔢️ The physical child indices of a vacancy's sibling collection: `p:sldId` entries, or the shape tree's shapes
/// (every PresentationML element but the group's own `p:nvGrpSpPr`/`p:grpSpPr`).
pub(super) fn slots(children: &[XNode], slide_list: bool) -> Vec<usize> {
    children.iter().enumerate().filter(|(_, child)| if slide_list { is_named(child, "p:sldId") } else { matches!(child, XNode::Element { name, .. } if name.starts_with("p:") && name != "p:nvGrpSpPr" && name != "p:grpSpPr") }).map(|(index, _)| index).collect()
}

pub(super) fn insert_at_vacancy(pkg: &mut Package, vacancy: &Json, node: XNode, slide_list: bool) -> Result<(), String> {
    let (part, path) = located(vacancy.get("container").ok_or("a vacancy carries no container")?)?;
    let index = match vacancy.get("index") { Some(Json::Number(value)) => *value as usize, _ => return Err("a vacancy carries no index".to_string()) };
    edit_part(pkg, &part, |root| {
        let children = children_at(root, &path)?;
        let slots = slots(children, slide_list);
        let physical = slots.get(index).copied().unwrap_or(children.len());
        children.insert(physical, node);
        Ok(())
    })
}

pub(super) fn remove_addressed(pkg: &mut Package, address: &Json, expected: &[&str]) -> Result<(), String> {
    let (part, path) = located(address)?;
    let (index, parent) = path.split_last().ok_or("a part root cannot be removed")?;
    edit_part(pkg, &part, |root| {
        let children = children_at(root, parent)?;
        if !children.get(*index).is_some_and(|child| expected.iter().any(|name| is_named(child, name))) {
            return Err(format!("the addressed node is not one of {expected:?}"));
        }
        children.remove(*index);
        Ok(())
    })
}

/// 🔤️ Every `a:t` of a shape in document order: the first carries the text, the rest are emptied.
pub(super) fn write_text(node: &mut XNode, text: &str, first: &mut bool) {
    let XNode::Element { name, children, .. } = node else { return };
    if name == "a:t" {
        let value = if std::mem::take(first) { text.to_string() } else { String::new() };
        match children.iter_mut().find_map(|child| match child { XNode::Text(existing) => Some(existing), XNode::Element { .. } => None }) {
            Some(existing) => *existing = value,
            None => children.insert(0, XNode::Text(value)),
        }
        return;
    }
    for child in children {
        write_text(child, text, first);
    }
}

pub(super) fn first_named<'a>(node: &'a mut XNode, wanted: &str) -> Option<&'a mut XNode> {
    if is_named(node, wanted) {
        return Some(node);
    }
    let XNode::Element { children, .. } = node else { return None };
    children.iter_mut().find_map(|child| first_named(child, wanted))
}

pub(super) fn set_attr(node: &mut XNode, key: &str, value: String) {
    if let XNode::Element { attrs, .. } = node {
        match attrs.iter_mut().find(|(name, _)| name == key) {
            Some(attr) => attr.1 = value,
            None => attrs.push((key.to_string(), value)),
        }
    }
}
//#endregion 🔖️Forward
