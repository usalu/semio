//! 🚪️ IO stdio.bcf (2.1/🖊️markup) — flat ZIP container of parts, reusing the real `stdio.zip`
//! codec for every byte concern, plus a typed `BcfTopic`/`BcfComment`/`BcfViewpoint` view
//! parsed/re-emitted via the real `stdio.xml` codec (never a hand-rolled parser here). `bcfzip`
//! is NOT an OPC package (no content-types/relationships apparatus) so this artifact builds its
//! own simple wrapper directly on `zip::ZipEntry` rather than reusing `zip::opc::OpcPackage` —
//! see the F5 report §1. 🦑 Codec dissolved out of the former `⚙️engine` (ticket
//! 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES); registration flows through
//! `crate::declaration()` (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE).

use crate::{
    schema::snapshot::{BcfCamera, BcfColoring, BcfComment, BcfComponents, BcfPoint3, BcfRawPart, BcfTopic, BcfViewpoint, BcfVisibility},
    BcfSnapshot, STDIO_BCF_DOCUMENT_SCHEMA,
};
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlAttr, XmlDocument, XmlNode};
use semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::text::snapshot::{xml_document_from_text, xml_document_to_text};
use semio_s_artifact_stdio_zip::schema::snapshot::ZipEntry;

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v2_1::subsets::any::io::BcfAnalyzer;
    use crate::BcfSnapshot;
    use {semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.bcf", standard: StandardId("2.1"), subset: SubsetId("*") };
    const DEP_ZIP: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId("*") };

    pub struct BcfComposerComposition;

    impl ArtifactComposition for BcfComposerComposition {
        type Snapshot = BcfSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_ZIP]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            // 🌱 Every listed read dialect's payload is raw text/bytes that this artifact's own
            // analyzer already round-trips through `store::Document{Dsl,Pack}` -- including bytes
            // claiming a dependency's dialect, since (for a single-standard DAG-adjacent dependency
            // like binary) that payload IS the same byte/text shape `analyze` already accepts.
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT || s.dialect == DEP_ZIP)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "BcfComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = BcfAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "BcfComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🔖️XmlHelpers
/// 🌳️ Narrows an `XmlNode` to its `Element` shape, if it is one.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn as_element(node: &XmlNode) -> Option<(&str, &[XmlAttr], &[XmlNode])> {
    match node {
        XmlNode::Element { name, attrs, children } => Some((name.as_str(), attrs.as_slice(), children.as_slice())),
        _ => None,
    }
}

/// 🔎️ First direct child element named `name`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn find_child<'a>(children: &'a [XmlNode], name: &str) -> Option<&'a XmlNode> {
    children.iter().find(|c| as_element(c).is_some_and(|(n, _, _)| n == name))
}

/// 🔎️ All direct child elements named `name`, in document order.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn find_children<'a>(children: &'a [XmlNode], name: &str) -> Vec<&'a XmlNode> {
    children.iter().filter(|c| as_element(c).is_some_and(|(n, _, _)| n == name)).collect()
}

/// 🏷️ Attribute value by name.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn attr<'a>(attrs: &'a [XmlAttr], name: &str) -> Option<&'a str> {
    attrs.iter().find(|a| a.name == name).map(|a| a.value.as_str())
}

/// 🔤️ Concatenated text/CDATA content of an element's direct children (BCF's leaf elements are
/// always simple text content, never mixed markup).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn text_content(node: &XmlNode) -> String {
    let Some((_, _, children)) = as_element(node) else { return String::new() };
    let mut out = String::new();
    for child in children {
        match child {
            XmlNode::Text { text } | XmlNode::CData { text } => out.push_str(text),
            _ => {}
        }
    }
    out
}

/// 🔤️ Wraps a leaf text element `<name>text</name>` (only emitted when `text` is non-empty,
/// mirroring how real BCF writers omit optional leaf elements rather than emit them empty).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn text_element(name: &str, text: &str) -> Option<XmlNode> {
    if text.is_empty() {
        return None;
    }
    Some(XmlNode::Element { name: name.into(), attrs: Vec::new(), children: vec![XmlNode::Text { text: text.into() }] })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_f64(s: &str) -> f64 {
    s.parse::<f64>().unwrap_or(0.0)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn xml_bytes(root: XmlNode) -> Vec<u8> {
    let doc = XmlDocument { root: Some(root), doctype: None, declaration: None, prolog: Vec::new(), epilog: Vec::new() };
    let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    out.push_str(&xml_document_to_text(&doc));
    out.into_bytes()
}
//#endregion 🔖️XmlHelpers

//#region 🔖️VersionXml
/// 🧩️ Parses `bcf.version`'s `<Version VersionId="...">` root attribute.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_bcf_version(data: &[u8]) -> Option<String> {
    let text = std::str::from_utf8(data).ok()?;
    let doc = xml_document_from_text(text).ok()?;
    let root = doc.root.as_ref()?;
    let (name, attrs, _) = as_element(root)?;
    if name != "Version" {
        return None;
    }
    Some(attr(attrs, "VersionId").unwrap_or_default().to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn bcf_version_bytes(version: &str) -> Vec<u8> {
    let mut children = Vec::new();
    if let Some(n) = text_element("DetailedVersion", version) {
        children.push(n);
    }
    xml_bytes(XmlNode::Element { name: "Version".into(), attrs: vec![XmlAttr { name: "VersionId".into(), value: version.to_string() }], children })
}
//#endregion 🔖️VersionXml

//#region 🔖️MarkupXml
/// 🧩 One `markup.bcf` `<Viewpoints>` reference entry: the guid plus the referenced `.bcfv`/
/// snapshot filenames (as actually written in the file — used only to locate the sibling zip
/// entries during decode; the typed `BcfViewpoint` itself carries no filename).
struct ViewpointRef {
    guid: String,
    viewpoint_file: Option<String>,
    snapshot_file: Option<String>,
}

/// 🧩 Everything `parse_markup_bcf` recovers from one topic folder's `markup.bcf`, before the
/// sibling `.bcfv`/snapshot files have been resolved into full `BcfViewpoint`s.
struct RawTopicMarkup {
    topic: BcfTopic,
    viewpoint_refs: Vec<ViewpointRef>,
}

/// 🧩️ Parses one topic folder's `markup.bcf` XML bytes (BCF-XML 2.1 `markup.xsd`: root
/// `<Markup>` with a required `<Topic Guid="..." TopicStatus="...">` carrying `<Title>` plus
/// optional `<Priority>`/`<Labels>`*/`<CreationDate>`/`<CreationAuthor>`/`<Description>` CHILD
/// elements -- not attributes, a defect this rewrite fixes -- zero-or-more sibling `<Comment
/// Guid="...">` elements each with `<Date>`/`<Author>`/`<Comment>`/optional `<Viewpoint Guid=
/// "...">`, and zero-or-more `<Viewpoints Guid="...">` reference entries).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_markup_bcf(data: &[u8]) -> Option<RawTopicMarkup> {
    let text = std::str::from_utf8(data).ok()?;
    let doc = xml_document_from_text(text).ok()?;
    let root = doc.root.as_ref()?;
    let (root_name, _, root_children) = as_element(root)?;
    if root_name != "Markup" {
        return None;
    }
    let topic_node = find_child(root_children, "Topic")?;
    let (_, topic_attrs, topic_children) = as_element(topic_node)?;
    let guid = attr(topic_attrs, "Guid").unwrap_or_default().to_string();
    let status = attr(topic_attrs, "TopicStatus").unwrap_or_default().to_string();
    let title = find_child(topic_children, "Title").map(text_content).unwrap_or_default();
    let priority = find_child(topic_children, "Priority").map(text_content).unwrap_or_default();
    let description = find_child(topic_children, "Description").map(text_content).unwrap_or_default();
    let creation_date = find_child(topic_children, "CreationDate").map(text_content).unwrap_or_default();
    let creation_author = find_child(topic_children, "CreationAuthor").map(text_content).unwrap_or_default();
    let labels: Vec<String> = find_children(topic_children, "Labels").into_iter().map(text_content).collect();

    let comments = find_children(root_children, "Comment")
        .into_iter()
        .map(|c| {
            let (_, c_attrs, c_children) = as_element(c).unwrap_or(("Comment", &[], &[]));
            let viewpoint_ref = find_child(c_children, "Viewpoint").and_then(as_element).and_then(|(_, vattrs, _)| attr(vattrs, "Guid")).map(|s| s.to_string());
            BcfComment {
                guid: attr(c_attrs, "Guid").unwrap_or_default().to_string(),
                date: find_child(c_children, "Date").map(text_content).unwrap_or_default(),
                author: find_child(c_children, "Author").map(text_content).unwrap_or_default(),
                text: find_child(c_children, "Comment").map(text_content).unwrap_or_default(),
                viewpoint_ref,
            }
        })
        .collect();

    let viewpoint_refs = find_children(root_children, "Viewpoints")
        .into_iter()
        .map(|v| {
            let (_, v_attrs, v_children) = as_element(v).unwrap_or(("Viewpoints", &[], &[]));
            ViewpointRef {
                guid: attr(v_attrs, "Guid").unwrap_or_default().to_string(),
                viewpoint_file: find_child(v_children, "Viewpoint").map(text_content).filter(|s| !s.is_empty()),
                snapshot_file: find_child(v_children, "Snapshot").map(text_content).filter(|s| !s.is_empty()),
            }
        })
        .collect();

    Some(RawTopicMarkup { topic: BcfTopic { guid, title, description, status, priority, labels, creation_date, creation_author, comments, viewpoints: Vec::new() }, viewpoint_refs })
}

/// 🧩️ Re-emits a `BcfTopic` as a full `markup.bcf` XML document (the inverse of
/// `parse_markup_bcf`), via the real `stdio.xml` text codec. Viewpoint references always point at
/// this artifact's canonical `<guid>.bcfv`/`<guid>.png` filenames (documented normal form).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn markup_bcf_bytes(topic: &BcfTopic) -> Vec<u8> {
    let mut topic_children = Vec::new();
    if let Some(n) = text_element("Title", &topic.title) {
        topic_children.push(n);
    }
    if let Some(n) = text_element("Priority", &topic.priority) {
        topic_children.push(n);
    }
    for label in &topic.labels {
        if let Some(n) = text_element("Labels", label) {
            topic_children.push(n);
        }
    }
    if let Some(n) = text_element("CreationDate", &topic.creation_date) {
        topic_children.push(n);
    }
    if let Some(n) = text_element("CreationAuthor", &topic.creation_author) {
        topic_children.push(n);
    }
    if let Some(n) = text_element("Description", &topic.description) {
        topic_children.push(n);
    }

    let mut markup_children = vec![XmlNode::Element { name: "Topic".into(), attrs: vec![XmlAttr { name: "Guid".into(), value: topic.guid.clone() }, XmlAttr { name: "TopicStatus".into(), value: topic.status.clone() }], children: topic_children }];

    for comment in &topic.comments {
        let mut children = Vec::new();
        if let Some(n) = text_element("Date", &comment.date) {
            children.push(n);
        }
        if let Some(n) = text_element("Author", &comment.author) {
            children.push(n);
        }
        if let Some(n) = text_element("Comment", &comment.text) {
            children.push(n);
        }
        if let Some(vref) = &comment.viewpoint_ref {
            children.push(XmlNode::Element { name: "Viewpoint".into(), attrs: vec![XmlAttr { name: "Guid".into(), value: vref.clone() }], children: Vec::new() });
        }
        markup_children.push(XmlNode::Element { name: "Comment".into(), attrs: vec![XmlAttr { name: "Guid".into(), value: comment.guid.clone() }], children });
    }

    for vp in &topic.viewpoints {
        let mut children = Vec::new();
        children.push(XmlNode::Element { name: "Viewpoint".into(), attrs: Vec::new(), children: vec![XmlNode::Text { text: format!("{}.bcfv", vp.guid) }] });
        if vp.snapshot.is_some() {
            children.push(XmlNode::Element { name: "Snapshot".into(), attrs: Vec::new(), children: vec![XmlNode::Text { text: format!("{}.png", vp.guid) }] });
        }
        markup_children.push(XmlNode::Element { name: "Viewpoints".into(), attrs: vec![XmlAttr { name: "Guid".into(), value: vp.guid.clone() }], children });
    }

    xml_bytes(XmlNode::Element { name: "Markup".into(), attrs: Vec::new(), children: markup_children })
}
//#endregion 🔖️MarkupXml

//#region 🔖️VisualizationInfoXml
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_point(node: &XmlNode) -> BcfPoint3 {
    let attrs = as_element(node).map_or(&[][..], |(_, a, _)| a);
    BcfPoint3 { x: attr(attrs, "X").map_or(0.0, parse_f64), y: attr(attrs, "Y").map_or(0.0, parse_f64), z: attr(attrs, "Z").map_or(0.0, parse_f64) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn point_element(name: &str, p: &BcfPoint3) -> XmlNode {
    XmlNode::Element { name: name.into(), attrs: vec![XmlAttr { name: "X".into(), value: p.x.to_string() }, XmlAttr { name: "Y".into(), value: p.y.to_string() }, XmlAttr { name: "Z".into(), value: p.z.to_string() }], children: Vec::new() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_camera(children: &[XmlNode]) -> Option<BcfCamera> {
    if let Some(persp) = find_child(children, "PerspectiveCamera") {
        let (_, _, pc) = as_element(persp)?;
        let view_point = find_child(pc, "CameraViewPoint").map(parse_point).unwrap_or_default();
        let direction = find_child(pc, "CameraDirection").map(parse_point).unwrap_or_default();
        let up_vector = find_child(pc, "CameraUpVector").map(parse_point).unwrap_or_default();
        let field_of_view = find_child(pc, "FieldOfView").map_or(0.0, |n| parse_f64(&text_content(n)));
        return Some(BcfCamera::Perspective { view_point, direction, up_vector, field_of_view });
    }
    if let Some(ortho) = find_child(children, "OrthogonalCamera") {
        let (_, _, oc) = as_element(ortho)?;
        let view_point = find_child(oc, "CameraViewPoint").map(parse_point).unwrap_or_default();
        let direction = find_child(oc, "CameraDirection").map(parse_point).unwrap_or_default();
        let up_vector = find_child(oc, "CameraUpVector").map(parse_point).unwrap_or_default();
        let view_to_world_scale = find_child(oc, "ViewToWorldScale").map_or(0.0, |n| parse_f64(&text_content(n)));
        return Some(BcfCamera::Orthogonal { view_point, direction, up_vector, view_to_world_scale });
    }
    None
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn camera_element(camera: &BcfCamera) -> XmlNode {
    match camera {
        BcfCamera::Perspective { view_point, direction, up_vector, field_of_view } => XmlNode::Element {
            name: "PerspectiveCamera".into(),
            attrs: Vec::new(),
            children: vec![
                point_element("CameraViewPoint", view_point),
                point_element("CameraDirection", direction),
                point_element("CameraUpVector", up_vector),
                XmlNode::Element { name: "FieldOfView".into(), attrs: Vec::new(), children: vec![XmlNode::Text { text: field_of_view.to_string() }] },
            ],
        },
        BcfCamera::Orthogonal { view_point, direction, up_vector, view_to_world_scale } => XmlNode::Element {
            name: "OrthogonalCamera".into(),
            attrs: Vec::new(),
            children: vec![
                point_element("CameraViewPoint", view_point),
                point_element("CameraDirection", direction),
                point_element("CameraUpVector", up_vector),
                XmlNode::Element { name: "ViewToWorldScale".into(), attrs: Vec::new(), children: vec![XmlNode::Text { text: view_to_world_scale.to_string() }] },
            ],
        },
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_component_list(container: &XmlNode) -> Vec<String> {
    let Some((_, _, children)) = as_element(container) else { return Vec::new() };
    find_children(children, "Component").into_iter().filter_map(|c| as_element(c).and_then(|(_, a, _)| attr(a, "IfcGuid")).map(|s| s.to_string())).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn component_list_elements(guids: &[String]) -> Vec<XmlNode> {
    guids.iter().map(|g| XmlNode::Element { name: "Component".into(), attrs: vec![XmlAttr { name: "IfcGuid".into(), value: g.clone() }], children: Vec::new() }).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_components(components_node: &XmlNode) -> BcfComponents {
    let (_, _, children) = as_element(components_node).unwrap_or(("Components", &[], &[]));
    let selection = find_child(children, "Selection").map(parse_component_list).unwrap_or_default();
    let visibility = match find_child(children, "Visibility") {
        Some(v) => {
            let (_, vattrs, vchildren) = as_element(v).unwrap_or(("Visibility", &[], &[]));
            let default_visibility = attr(vattrs, "DefaultVisibility") != Some("false");
            let exceptions = find_child(vchildren, "Exceptions").map(parse_component_list).unwrap_or_default();
            BcfVisibility { default_visibility, exceptions }
        }
        None => BcfVisibility { default_visibility: true, exceptions: Vec::new() },
    };
    let coloring = match find_child(children, "Coloring") {
        Some(c) => {
            let (_, _, cchildren) = as_element(c).unwrap_or(("Coloring", &[], &[]));
            find_children(cchildren, "Color")
                .into_iter()
                .map(|color_node| {
                    let (_, cattrs, _) = as_element(color_node).unwrap_or(("Color", &[], &[]));
                    BcfColoring { color: attr(cattrs, "Color").unwrap_or_default().to_string(), components: parse_component_list(color_node) }
                })
                .collect()
        }
        None => Vec::new(),
    };
    BcfComponents { selection, visibility, coloring }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn components_element(components: &BcfComponents) -> XmlNode {
    let mut children = Vec::new();
    if !components.selection.is_empty() {
        children.push(XmlNode::Element { name: "Selection".into(), attrs: Vec::new(), children: component_list_elements(&components.selection) });
    }
    let mut visibility_children = Vec::new();
    if !components.visibility.exceptions.is_empty() {
        visibility_children.push(XmlNode::Element { name: "Exceptions".into(), attrs: Vec::new(), children: component_list_elements(&components.visibility.exceptions) });
    }
    children.push(XmlNode::Element { name: "Visibility".into(), attrs: vec![XmlAttr { name: "DefaultVisibility".into(), value: components.visibility.default_visibility.to_string() }], children: visibility_children });
    if !components.coloring.is_empty() {
        let color_nodes = components.coloring.iter().map(|c| XmlNode::Element { name: "Color".into(), attrs: vec![XmlAttr { name: "Color".into(), value: c.color.clone() }], children: component_list_elements(&c.components) }).collect();
        children.push(XmlNode::Element { name: "Coloring".into(), attrs: Vec::new(), children: color_nodes });
    }
    XmlNode::Element { name: "Components".into(), attrs: Vec::new(), children }
}

/// 🧩️ Parses one `.bcfv` `<VisualizationInfo Guid="...">` document (BCF-XML 2.1 `visinfo.xsd`)
/// into `(camera, components)` — the guid itself is already known from the `markup.bcf`
/// `<Viewpoints>` reference entry, so it isn't re-extracted here.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_visualization_info(data: &[u8]) -> Option<(Option<BcfCamera>, Option<BcfComponents>)> {
    let text = std::str::from_utf8(data).ok()?;
    let doc = xml_document_from_text(text).ok()?;
    let root = doc.root.as_ref()?;
    let (name, _, children) = as_element(root)?;
    if name != "VisualizationInfo" {
        return None;
    }
    let components = find_child(children, "Components").map(parse_components);
    let camera = parse_camera(children);
    Some((camera, components))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn visualization_info_bytes(vp: &BcfViewpoint) -> Vec<u8> {
    let mut children = Vec::new();
    if let Some(components) = &vp.components {
        children.push(components_element(components));
    }
    if let Some(camera) = &vp.camera {
        children.push(camera_element(camera));
    }
    xml_bytes(XmlNode::Element { name: "VisualizationInfo".into(), attrs: vec![XmlAttr { name: "Guid".into(), value: vp.guid.clone() }], children })
}
//#endregion 🔖️VisualizationInfoXml

//#region 🔖️Codec
/// 🚫️ The refusal for a part the archive names but no well-formed `<root>` document fills. Dropping it instead lost a
/// viewpoint's camera and components (or a whole topic) without a word, and re-encoding then wrote the loss back.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn malformed(part: &str, root: &str) -> String {
    format!("{part} is not a well-formed BCF 2.1 <{root}> document")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_bcf(snap: &BcfSnapshot) -> Result<Vec<u8>, String> {
    let mut entries = Vec::new();
    entries.push(ZipEntry { name: "bcf.version".into(), data: bcf_version_bytes(&snap.version), ..Default::default() });
    for topic in &snap.topics {
        entries.push(ZipEntry { name: format!("{}/markup.bcf", topic.guid), data: markup_bcf_bytes(topic), ..Default::default() });
        for vp in &topic.viewpoints {
            entries.push(ZipEntry { name: format!("{}/{}.bcfv", topic.guid, vp.guid), data: visualization_info_bytes(vp), ..Default::default() });
            if let Some(bytes) = &vp.snapshot {
                entries.push(ZipEntry { name: format!("{}/{}.png", topic.guid, vp.guid), data: bytes.clone(), ..Default::default() });
            }
        }
    }
    for part in &snap.parts {
        entries.push(ZipEntry { name: part.name.clone(), data: part.data.clone(), ..Default::default() });
    }
    let zip_snap = semio_s_artifact_stdio_zip::ZipSnapshot { schema: semio_s_artifact_stdio_zip::STDIO_ZIP_DOCUMENT_SCHEMA.into(), entries, comment: String::new(), ..Default::default() };
    semio_s_artifact_stdio_zip::standards::v2_0::subsets::base::io::encode_zip(&zip_snap).map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_bcf(data: &[u8]) -> Result<BcfSnapshot, String> {
    let zip = semio_s_artifact_stdio_zip::standards::v2_0::subsets::base::io::decode_zip(data).map_err(|e| e.to_string())?;

    let mut version = String::new();
    let mut consumed: std::collections::HashSet<String> = std::collections::HashSet::new();
    if let Some(e) = zip.entries.iter().find(|e| e.name.eq_ignore_ascii_case("bcf.version")) {
        version = parse_bcf_version(&e.data).ok_or_else(|| malformed(&e.name, "Version"))?;
        consumed.insert(e.name.clone());
    }

    let mut folders: std::collections::BTreeMap<&str, Vec<&ZipEntry>> = Default::default();
    for e in &zip.entries {
        if let Some((folder, _)) = e.name.split_once('/') {
            folders.entry(folder).or_default().push(e);
        }
    }

    let mut topics = Vec::new();
    for (folder, folder_entries) in &folders {
        let markup_name = format!("{folder}/markup.bcf");
        let Some(markup_entry) = folder_entries.iter().find(|e| e.name.eq_ignore_ascii_case(&markup_name)) else { continue };
        let raw = parse_markup_bcf(&markup_entry.data).ok_or_else(|| malformed(&markup_entry.name, "Markup"))?;
        consumed.insert(markup_entry.name.clone());

        let mut viewpoints = Vec::new();
        for vref in &raw.viewpoint_refs {
            let mut camera = None;
            let mut components = None;
            if let Some(vp_file) = &vref.viewpoint_file {
                let full = format!("{folder}/{vp_file}");
                if let Some(vp_entry) = folder_entries.iter().find(|e| e.name.eq_ignore_ascii_case(&full)) {
                    (camera, components) = parse_visualization_info(&vp_entry.data).ok_or_else(|| malformed(&vp_entry.name, "VisualizationInfo"))?;
                    consumed.insert(vp_entry.name.clone());
                }
            }
            let mut snapshot = None;
            if let Some(snap_file) = &vref.snapshot_file {
                let full = format!("{folder}/{snap_file}");
                if let Some(snap_entry) = folder_entries.iter().find(|e| e.name.eq_ignore_ascii_case(&full)) {
                    snapshot = Some(snap_entry.data.clone());
                    consumed.insert(snap_entry.name.clone());
                }
            }
            viewpoints.push(BcfViewpoint { guid: vref.guid.clone(), camera, components, snapshot });
        }

        let mut topic = raw.topic;
        topic.viewpoints = viewpoints;
        topics.push(topic);
    }

    let parts = zip.entries.iter().filter(|e| !consumed.contains(&e.name)).map(|e| BcfRawPart { name: e.name.clone(), data: e.data.clone() }).collect();

    Ok(BcfSnapshot { schema: STDIO_BCF_DOCUMENT_SCHEMA.into(), version, topics, parts })
}
//#endregion 🔖️Codec

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::standards::v2_1::subsets::any::io::BcfComposer as BcfRawAnyComposer;
    use semio_framework_plugin::{composer_entry_of, io::ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<BcfRawAnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry

#[path = "💾️binary/🦀️.rs"]
pub mod binary;

#[path = "📝️text/🦀️.rs"]
pub mod text;

#[path = "🪶️sqlite/🦀️.rs"]
pub mod sqlite;

pub mod derived_construction {
    use crate::{BcfDiff, BcfMutation, BcfSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.bcf` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct BcfBuilderConstruction {
        snapshot: BcfSnapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
    }

    impl ArtifactBuilder for BcfBuilderConstruction {
        type Snapshot = BcfSnapshot;
        type Mutation = BcfMutation;
        type Diff = BcfDiff;
        fn empty() -> Self {
            Self { snapshot: BcfSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<BcfSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<BcfSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::schema::mutations::apply_bcf_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = protocol::apply_diff(&diff, &self.snapshot)?;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>> {
            if self.diagnostics.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(self.diagnostics)
            }
        }
    }
    //#endregion 🔖️Builder
}
pub use derived_construction::*;

pub mod derived_analysis {
    use crate::BcfSnapshot;
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.bcf` parts.
    #[derive(Clone, Debug, Default)]
    pub struct BcfParts {
        pub snapshot: Option<BcfSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.bcf` (2.1/🖊️markup) sources.
    pub struct BcfAnalyzerAnalysis;

    impl ArtifactAnalysis for BcfAnalyzerAnalysis {
        type Parts = BcfParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.bcf", standard: StandardId("2.1"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            // 🕵️ Real sniff: BCF is a zip container that additionally carries a root `bcf.version`
            // entry. Reuses the zip artifact's own byte-level magic+EOCD check (never reimplemented
            // here) for the base confidence, then cheaply corroborates the `bcf.version` entry name
            // via a substring scan of the raw bytes -- filenames are stored as literal bytes in both
            // the local and central-directory headers, so this finds a real entry name without
            // paying for a full `decode_zip` (which would also inflate every snapshot PNG payload
            // just to read names -- the same cost tradeoff the zip analyzer's own sniff makes by
            // stopping at "does a well-formed EOCD exist" rather than parsing every entry).
            use semio_s_artifact_stdio_zip::standards::v2_0::subsets::base::io::{sniff_zip_bytes, SniffConfidence};
            match source {
                AnalyzeSource::Binary(bytes) => match sniff_zip_bytes(bytes) {
                    SniffConfidence::Low => semio_framework_plugin::io::Confidence::Low,
                    zip_confidence => {
                        let needle = b"bcf.version";
                        let has_bcf_version_name = bytes.len() >= needle.len() && bytes.windows(needle.len()).any(|w| w == needle);
                        match (zip_confidence, has_bcf_version_name) {
                            (SniffConfidence::High, true) => semio_framework_plugin::io::Confidence::High,
                            (SniffConfidence::High, false) => semio_framework_plugin::io::Confidence::Medium,
                            (SniffConfidence::Medium, _) => semio_framework_plugin::io::Confidence::Medium,
                            (SniffConfidence::Low, _) => unreachable!("Low was matched above"),
                        }
                    }
                },
                // The DSL envelope (hex-wrapped text) preamble is what actually recognizes the text
                // form, not this byte-magic sniff.
                AnalyzeSource::Text(_) => semio_framework_plugin::io::Confidence::Low,
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = BcfParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = semio_framework_plugin::io::Confidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <BcfSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <BcfSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.binary", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
    //#endregion 🔖️Analyzer

    //#region 🧪️Tests
    #[cfg(test)]
    include!("../🧬️schema/🧪️tests/🔬️derived-analysis-unit/🦀️.rs");
    //#endregion 🧪️Tests
}
pub use derived_analysis::*;

semio_framework_plugin::derive_artifact_facets!(
    pub spec BcfBuilderFacets {
        construction: BcfBuilderConstruction,
        analysis: BcfAnalyzerAnalysis,
        composition: crate::standards::v2_1::subsets::any::io::derived_composition::BcfComposerComposition,
    }
    builder: BcfBuilder,
    analyzer: BcfAnalyzer,
    composer: BcfComposer,
);
