//! 💬️ Markup and viewpoint XML of the BCF oracle: the markup.bcf and visualization info parse and write. See [`super`].

use super::*;

//#region 🔖️MarkupXml
struct ViewpointRefRaw {
    pub(super) guid: String,
    pub(super) viewpoint_file: Option<String>,
    pub(super) snapshot_file: Option<String>,
}

/// 🧩️ Parses one topic folder's `markup.bcf` (BCF-XML 2.1 `markup.xsd`): root `<Markup>` with a
/// required `<Topic Guid="..." TopicStatus="...">` carrying `<Title>`/optional `<Priority>`/
/// `<Labels>`*/`<CreationDate>`/`<CreationAuthor>`/`<Description>` CHILD elements, zero-or-more
/// sibling `<Comment Guid="...">` elements each with `<Date>`/`<Author>`/`<Comment>`/optional
/// `<Viewpoint Guid="...">`, and zero-or-more `<Viewpoints Guid="...">` reference entries.
pub(super) fn parse_markup(data: &[u8]) -> Result<(OTopic, Vec<ViewpointRefRaw>), String> {
    let root = parse_xml(data)?;
    let (root_name, _, root_children) = as_element(&root).ok_or("markup.bcf: root is not an element")?;
    if root_name != "Markup" {
        return Err(format!("markup.bcf: expected root <Markup>, got <{root_name}>"));
    }
    let topic_node = find_child(root_children, "Topic").ok_or("markup.bcf: missing <Topic>")?;
    let (_, topic_attrs, topic_children) = as_element(topic_node).ok_or("markup.bcf: <Topic> is not an element")?;
    let topic = OTopic {
        guid: attr(topic_attrs, "Guid").unwrap_or_default().to_string(),
        title: find_child(topic_children, "Title").map(text_content).unwrap_or_default(),
        description: find_child(topic_children, "Description").map(text_content).unwrap_or_default(),
        status: attr(topic_attrs, "TopicStatus").unwrap_or_default().to_string(),
        priority: find_child(topic_children, "Priority").map(text_content).unwrap_or_default(),
        labels: find_children(topic_children, "Labels").into_iter().map(text_content).collect(),
        creation_date: find_child(topic_children, "CreationDate").map(text_content).unwrap_or_default(),
        creation_author: find_child(topic_children, "CreationAuthor").map(text_content).unwrap_or_default(),
        comments: find_children(root_children, "Comment")
            .into_iter()
            .map(|comment_node| {
                let (_, c_attrs, c_children) = as_element(comment_node).unwrap_or(("Comment", &[], &[]));
                let viewpoint_ref = find_child(c_children, "Viewpoint").and_then(as_element).and_then(|(_, v_attrs, _)| attr(v_attrs, "Guid")).map(|guid| guid.to_string());
                OComment {
                    guid: attr(c_attrs, "Guid").unwrap_or_default().to_string(),
                    date: find_child(c_children, "Date").map(text_content).unwrap_or_default(),
                    author: find_child(c_children, "Author").map(text_content).unwrap_or_default(),
                    text: find_child(c_children, "Comment").map(text_content).unwrap_or_default(),
                    viewpoint_ref,
                }
            })
            .collect(),
        viewpoints: Vec::new(),
    };
    let viewpoint_refs = find_children(root_children, "Viewpoints")
        .into_iter()
        .map(|viewpoints_node| {
            let (_, v_attrs, v_children) = as_element(viewpoints_node).unwrap_or(("Viewpoints", &[], &[]));
            ViewpointRefRaw {
                guid: attr(v_attrs, "Guid").unwrap_or_default().to_string(),
                viewpoint_file: find_child(v_children, "Viewpoint").map(text_content).filter(|value| !value.is_empty()),
                snapshot_file: find_child(v_children, "Snapshot").map(text_content).filter(|value| !value.is_empty()),
            }
        })
        .collect();
    Ok((topic, viewpoint_refs))
}

/// 🧩️ Re-emits an `OTopic` as a `markup.bcf` document — the inverse of [`parse_markup`].
/// Viewpoint references always point at this module's own canonical `<guid>.bcfv`/`<guid>.png`
/// filenames (documented normal form, matching the production codec's own convention).
pub(super) fn markup_bytes(topic: &OTopic) -> Vec<u8> {
    let mut topic_children = Vec::new();
    for (name, text) in [("Title", &topic.title), ("Priority", &topic.priority)] {
        if let Some(node) = text_element(name, text) {
            topic_children.push(node);
        }
    }
    for label in &topic.labels {
        if let Some(node) = text_element("Labels", label) {
            topic_children.push(node);
        }
    }
    for (name, text) in [("CreationDate", &topic.creation_date), ("CreationAuthor", &topic.creation_author), ("Description", &topic.description)] {
        if let Some(node) = text_element(name, text) {
            topic_children.push(node);
        }
    }
    let mut markup_children = vec![BNode::Element { name: "Topic".to_string(), attrs: vec![("Guid".to_string(), topic.guid.clone()), ("TopicStatus".to_string(), topic.status.clone())], children: topic_children }];
    for comment in &topic.comments {
        let mut children = Vec::new();
        for (name, text) in [("Date", &comment.date), ("Author", &comment.author), ("Comment", &comment.text)] {
            if let Some(node) = text_element(name, text) {
                children.push(node);
            }
        }
        if let Some(reference) = &comment.viewpoint_ref {
            children.push(BNode::Element { name: "Viewpoint".to_string(), attrs: vec![("Guid".to_string(), reference.clone())], children: Vec::new() });
        }
        markup_children.push(BNode::Element { name: "Comment".to_string(), attrs: vec![("Guid".to_string(), comment.guid.clone())], children });
    }
    for viewpoint in &topic.viewpoints {
        let mut children = vec![BNode::Element { name: "Viewpoint".to_string(), attrs: Vec::new(), children: vec![BNode::Text(format!("{}.bcfv", viewpoint.guid))] }];
        if viewpoint.snapshot.is_some() {
            children.push(BNode::Element { name: "Snapshot".to_string(), attrs: Vec::new(), children: vec![BNode::Text(format!("{}.png", viewpoint.guid))] });
        }
        markup_children.push(BNode::Element { name: "Viewpoints".to_string(), attrs: vec![("Guid".to_string(), viewpoint.guid.clone())], children });
    }
    write_xml(&BNode::Element { name: "Markup".to_string(), attrs: Vec::new(), children: markup_children })
}

pub(super) fn bcf_version_bytes(version: &str) -> Vec<u8> {
    let mut children = Vec::new();
    if let Some(node) = text_element("DetailedVersion", version) {
        children.push(node);
    }
    write_xml(&BNode::Element { name: "Version".to_string(), attrs: vec![("VersionId".to_string(), version.to_string())], children })
}
//#endregion 🔖️MarkupXml

//#region 🔖️VisualizationInfoXml
fn parse_f64(text: &str) -> f64 {
    text.parse::<f64>().unwrap_or(0.0)
}

fn parse_point(node: &BNode) -> OPoint3 {
    let attrs = as_element(node).map(|(_, a, _)| a).unwrap_or(&[]);
    OPoint3 { x: attr(attrs, "X").map(parse_f64).unwrap_or(0.0), y: attr(attrs, "Y").map(parse_f64).unwrap_or(0.0), z: attr(attrs, "Z").map(parse_f64).unwrap_or(0.0) }
}

fn point_element(name: &str, point: &OPoint3) -> BNode {
    BNode::Element { name: name.to_string(), attrs: vec![("X".to_string(), point.x.to_string()), ("Y".to_string(), point.y.to_string()), ("Z".to_string(), point.z.to_string())], children: Vec::new() }
}

fn parse_camera(children: &[BNode]) -> Option<OCamera> {
    if let Some(perspective) = find_child(children, "PerspectiveCamera") {
        let (_, _, pc) = as_element(perspective)?;
        return Some(OCamera::Perspective {
            view_point: find_child(pc, "CameraViewPoint").map(parse_point).unwrap_or_default(),
            direction: find_child(pc, "CameraDirection").map(parse_point).unwrap_or_default(),
            up_vector: find_child(pc, "CameraUpVector").map(parse_point).unwrap_or_default(),
            field_of_view: find_child(pc, "FieldOfView").map(|node| parse_f64(&text_content(node))).unwrap_or(0.0),
        });
    }
    if let Some(orthogonal) = find_child(children, "OrthogonalCamera") {
        let (_, _, oc) = as_element(orthogonal)?;
        return Some(OCamera::Orthogonal {
            view_point: find_child(oc, "CameraViewPoint").map(parse_point).unwrap_or_default(),
            direction: find_child(oc, "CameraDirection").map(parse_point).unwrap_or_default(),
            up_vector: find_child(oc, "CameraUpVector").map(parse_point).unwrap_or_default(),
            view_to_world_scale: find_child(oc, "ViewToWorldScale").map(|node| parse_f64(&text_content(node))).unwrap_or(0.0),
        });
    }
    None
}

fn camera_element(camera: &OCamera) -> BNode {
    match camera {
        OCamera::Perspective { view_point, direction, up_vector, field_of_view } => BNode::Element {
            name: "PerspectiveCamera".to_string(),
            attrs: Vec::new(),
            children: vec![
                point_element("CameraViewPoint", view_point),
                point_element("CameraDirection", direction),
                point_element("CameraUpVector", up_vector),
                BNode::Element { name: "FieldOfView".to_string(), attrs: Vec::new(), children: vec![BNode::Text(field_of_view.to_string())] },
            ],
        },
        OCamera::Orthogonal { view_point, direction, up_vector, view_to_world_scale } => BNode::Element {
            name: "OrthogonalCamera".to_string(),
            attrs: Vec::new(),
            children: vec![
                point_element("CameraViewPoint", view_point),
                point_element("CameraDirection", direction),
                point_element("CameraUpVector", up_vector),
                BNode::Element { name: "ViewToWorldScale".to_string(), attrs: Vec::new(), children: vec![BNode::Text(view_to_world_scale.to_string())] },
            ],
        },
    }
}

fn parse_component_list(container: &BNode) -> Vec<String> {
    let Some((_, _, children)) = as_element(container) else { return Vec::new() };
    find_children(children, "Component").into_iter().filter_map(|node| as_element(node).and_then(|(_, a, _)| attr(a, "IfcGuid")).map(|guid| guid.to_string())).collect()
}

fn component_list_elements(guids: &[String]) -> Vec<BNode> {
    guids.iter().map(|guid| BNode::Element { name: "Component".to_string(), attrs: vec![("IfcGuid".to_string(), guid.clone())], children: Vec::new() }).collect()
}

fn parse_components(components_node: &BNode) -> OComponents {
    let (_, _, children) = as_element(components_node).unwrap_or(("Components", &[], &[]));
    let selection = find_child(children, "Selection").map(parse_component_list).unwrap_or_default();
    let visibility = match find_child(children, "Visibility") {
        Some(node) => {
            let (_, v_attrs, v_children) = as_element(node).unwrap_or(("Visibility", &[], &[]));
            OVisibility { default_visibility: attr(v_attrs, "DefaultVisibility").map(|value| value != "false").unwrap_or(true), exceptions: find_child(v_children, "Exceptions").map(parse_component_list).unwrap_or_default() }
        }
        None => OVisibility { default_visibility: true, exceptions: Vec::new() },
    };
    let coloring = match find_child(children, "Coloring") {
        Some(node) => {
            let (_, _, c_children) = as_element(node).unwrap_or(("Coloring", &[], &[]));
            find_children(c_children, "Color")
                .into_iter()
                .map(|color_node| {
                    let (_, c_attrs, _) = as_element(color_node).unwrap_or(("Color", &[], &[]));
                    OColoring { color: attr(c_attrs, "Color").unwrap_or_default().to_string(), components: parse_component_list(color_node) }
                })
                .collect()
        }
        None => Vec::new(),
    };
    OComponents { selection, visibility, coloring }
}

fn components_element(components: &OComponents) -> BNode {
    let mut children = Vec::new();
    if !components.selection.is_empty() {
        children.push(BNode::Element { name: "Selection".to_string(), attrs: Vec::new(), children: component_list_elements(&components.selection) });
    }
    let mut visibility_children = Vec::new();
    if !components.visibility.exceptions.is_empty() {
        visibility_children.push(BNode::Element { name: "Exceptions".to_string(), attrs: Vec::new(), children: component_list_elements(&components.visibility.exceptions) });
    }
    children.push(BNode::Element { name: "Visibility".to_string(), attrs: vec![("DefaultVisibility".to_string(), components.visibility.default_visibility.to_string())], children: visibility_children });
    if !components.coloring.is_empty() {
        let color_nodes = components.coloring.iter().map(|coloring| BNode::Element { name: "Color".to_string(), attrs: vec![("Color".to_string(), coloring.color.clone())], children: component_list_elements(&coloring.components) }).collect();
        children.push(BNode::Element { name: "Coloring".to_string(), attrs: Vec::new(), children: color_nodes });
    }
    BNode::Element { name: "Components".to_string(), attrs: Vec::new(), children }
}

/// 🧩️ Parses one `.bcfv` `<VisualizationInfo Guid="...">` (BCF-XML 2.1 `visinfo.xsd`) into
/// `(camera, components)` — the guid itself is already known from the `markup.bcf` `<Viewpoints>`
/// reference entry.
pub(super) fn parse_visualization_info(data: &[u8]) -> Result<(Option<OCamera>, Option<OComponents>), String> {
    let root = parse_xml(data)?;
    let (name, _, children) = as_element(&root).ok_or("bcfv: root is not an element")?;
    if name != "VisualizationInfo" {
        return Err(format!("bcfv: expected root <VisualizationInfo>, got <{name}>"));
    }
    Ok((parse_camera(children), find_child(children, "Components").map(parse_components)))
}

pub(super) fn visualization_info_bytes(viewpoint: &OViewpoint) -> Vec<u8> {
    let mut children = Vec::new();
    if let Some(components) = &viewpoint.components {
        children.push(components_element(components));
    }
    if let Some(camera) = &viewpoint.camera {
        children.push(camera_element(camera));
    }
    write_xml(&BNode::Element { name: "VisualizationInfo".to_string(), attrs: vec![("Guid".to_string(), viewpoint.guid.clone())], children })
}
//#endregion 🔖️VisualizationInfoXml
