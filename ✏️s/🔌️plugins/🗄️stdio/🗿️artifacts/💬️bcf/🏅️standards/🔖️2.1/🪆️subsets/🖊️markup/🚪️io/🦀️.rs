//! 🚪️ IO stdio.bcf (2.1/🖊️markup) — flat ZIP container of parts, reusing the real `stdio.zip`
//! codec for every byte concern, plus a typed `BcfTopic`/`BcfComment`/`BcfViewpoint` view
//! parsed/re-emitted via the real `stdio.xml` codec (never a hand-rolled parser here). `bcfzip`
//! is NOT an OPC package (no content-types/relationships apparatus) so this artifact builds its
//! own simple wrapper directly on `zip::ZipEntry` rather than reusing `zip::opc::OpcPackage` —
//! see the F5 report §1. 🦑 Codec dissolved out of the former `⚙️engine` (ticket
//! 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES); registration flows through
//! `crate::artifacts::bcf::declaration()` (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE).

use crate::artifacts::bcf::{
    schema::snapshot::{BcfCamera, BcfColoring, BcfComment, BcfComponents, BcfPoint3, BcfRawPart, BcfTopic, BcfViewpoint, BcfVisibility},
    BcfSnapshot, STDIO_BCF_DOCUMENT_SCHEMA,
};
use crate::artifacts::xml::schema::snapshot::{xml_document_from_text, xml_document_to_text, XmlAttr, XmlDocument, XmlNode};
use crate::artifacts::zip::schema::snapshot::ZipEntry;

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::bcf::standards::v2_1::subsets::any::schema::BcfAnalyzer;
    use crate::artifacts::bcf::BcfSnapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.bcf", standard: StandardId("2.1"), subset: SubsetId("*") };
    const DEP_ZIP: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId("*") };
    const DEP_XML: Dialect = Dialect { artifact_kind: "s.stdio.xml", standard: StandardId("1.0"), subset: SubsetId("*") };

    pub struct BcfComposerComposition;

    impl ArtifactComposition for BcfComposerComposition {
        type Snapshot = BcfSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_ZIP, DEP_XML]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            // 🌱 Every listed read dialect's payload is raw text/bytes that this artifact's own
            // analyzer already round-trips through `store::Document{Dsl,Pack}` -- including bytes
            // claiming a dependency's dialect, since (for a single-standard DAG-adjacent dependency
            // like binary) that payload IS the same byte/text shape `analyze` already accepts.
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT || s.dialect == DEP_ZIP || s.dialect == DEP_XML)
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
    children.iter().find(|c| as_element(c).map(|(n, _, _)| n == name).unwrap_or(false))
}

/// 🔎️ All direct child elements named `name`, in document order.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn find_children<'a>(children: &'a [XmlNode], name: &str) -> Vec<&'a XmlNode> {
    children.iter().filter(|c| as_element(c).map(|(n, _, _)| n == name).unwrap_or(false)).collect()
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
    let doc = XmlDocument { root: Some(root), doctype: None, declaration: None, prolog: Vec::new() };
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
    let attrs = as_element(node).map(|(_, a, _)| a).unwrap_or(&[]);
    BcfPoint3 { x: attr(attrs, "X").map(parse_f64).unwrap_or(0.0), y: attr(attrs, "Y").map(parse_f64).unwrap_or(0.0), z: attr(attrs, "Z").map(parse_f64).unwrap_or(0.0) }
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
        let field_of_view = find_child(pc, "FieldOfView").map(|n| parse_f64(&text_content(n))).unwrap_or(0.0);
        return Some(BcfCamera::Perspective { view_point, direction, up_vector, field_of_view });
    }
    if let Some(ortho) = find_child(children, "OrthogonalCamera") {
        let (_, _, oc) = as_element(ortho)?;
        let view_point = find_child(oc, "CameraViewPoint").map(parse_point).unwrap_or_default();
        let direction = find_child(oc, "CameraDirection").map(parse_point).unwrap_or_default();
        let up_vector = find_child(oc, "CameraUpVector").map(parse_point).unwrap_or_default();
        let view_to_world_scale = find_child(oc, "ViewToWorldScale").map(|n| parse_f64(&text_content(n))).unwrap_or(0.0);
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
            let default_visibility = attr(vattrs, "DefaultVisibility").map(|s| s != "false").unwrap_or(true);
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
    let zip_snap = crate::artifacts::zip::ZipSnapshot { schema: crate::artifacts::zip::STDIO_ZIP_DOCUMENT_SCHEMA.into(), entries, comment: String::new() };
    crate::artifacts::zip::standards::v2_0::subsets::base::io::encode_zip(&zip_snap).map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_bcf(data: &[u8]) -> Result<BcfSnapshot, String> {
    let zip = crate::artifacts::zip::standards::v2_0::subsets::base::io::decode_zip(data).map_err(|e| e.to_string())?;

    let mut version = String::new();
    let mut consumed: std::collections::HashSet<String> = std::collections::HashSet::new();
    if let Some(e) = zip.entries.iter().find(|e| e.name.eq_ignore_ascii_case("bcf.version")) {
        version = parse_bcf_version(&e.data).unwrap_or_default();
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
        let Some(raw) = parse_markup_bcf(&markup_entry.data) else { continue };
        consumed.insert(markup_entry.name.clone());

        let mut viewpoints = Vec::new();
        for vref in &raw.viewpoint_refs {
            let mut camera = None;
            let mut components = None;
            if let Some(vp_file) = &vref.viewpoint_file {
                let full = format!("{folder}/{vp_file}");
                if let Some(vp_entry) = folder_entries.iter().find(|e| e.name.eq_ignore_ascii_case(&full)) {
                    if let Some((c, comp)) = parse_visualization_info(&vp_entry.data) {
                        camera = c;
                        components = comp;
                    }
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
mod tests {
    use super::*;
    use crate::artifacts::bcf::schema::diff::BcfDiff;
    use crate::artifacts::bcf::schema::mutations::{
        apply_bcf_mutation, insert_comment, insert_topic, insert_viewpoint, remove_comment, remove_topic, remove_viewpoint, set_comment, set_snapshot, set_topic_markup, set_version, set_viewpoint_camera, set_viewpoint_components, set_viewpoint_snapshot, BcfMutation,
    };
    use crate::artifacts::bcf::standards::v2_1::subsets::any::schema::snapshot::{demo_bcf_snapshot, empty_bcf_snapshot};
    use protocol::command::DiffAlgebra;
    use protocol::{DiffCodec, Mutation, MutationDiff, OpBinary, OpText};

    //#region Fixtures
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn perspective_camera() -> BcfCamera {
        BcfCamera::Perspective { view_point: BcfPoint3 { x: 1.0, y: 2.0, z: 3.0 }, direction: BcfPoint3 { x: 0.0, y: 0.0, z: -1.0 }, up_vector: BcfPoint3 { x: 0.0, y: 1.0, z: 0.0 }, field_of_view: 60.0 }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn orthogonal_camera() -> BcfCamera {
        BcfCamera::Orthogonal { view_point: BcfPoint3 { x: 4.0, y: 5.0, z: 6.0 }, direction: BcfPoint3 { x: 1.0, y: 0.0, z: 0.0 }, up_vector: BcfPoint3 { x: 0.0, y: 0.0, z: 1.0 }, view_to_world_scale: 2.5 }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sample_components() -> BcfComponents {
        BcfComponents {
            selection: vec!["2O2Fr$t4X7Zf8NOew3FLOH".into()],
            visibility: BcfVisibility { default_visibility: false, exceptions: vec!["1yQBoo7d5EEBLiyMxGgTLc".into()] },
            coloring: vec![BcfColoring { color: "FFFF0000".into(), components: vec!["0BTBFw6f90Nfh9rP1dl_3n".into()] }],
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sample_viewpoint(guid: &str) -> BcfViewpoint {
        BcfViewpoint { guid: guid.into(), camera: Some(perspective_camera()), components: Some(sample_components()), snapshot: Some(vec![0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a]) }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sample_comment(guid: &str, viewpoint_ref: Option<&str>) -> BcfComment {
        BcfComment { guid: guid.into(), date: "2024-01-01T00:00:00+00:00".into(), author: "ueli@example.com".into(), text: "Please review this clash.".into(), viewpoint_ref: viewpoint_ref.map(|s| s.to_string()) }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sample_topic(guid: &str) -> BcfTopic {
        BcfTopic {
            guid: guid.into(),
            title: "Clash on Level 2".into(),
            description: "MEP duct clashes with structural beam.".into(),
            status: "Open".into(),
            priority: "High".into(),
            labels: vec!["Clash".into(), "MEP".into()],
            creation_date: "2024-01-01T00:00:00+00:00".into(),
            creation_author: "ueli@example.com".into(),
            comments: vec![sample_comment("c1", Some("vp1"))],
            viewpoints: vec![sample_viewpoint("vp1")],
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sample_snapshot() -> BcfSnapshot {
        BcfSnapshot { schema: STDIO_BCF_DOCUMENT_SCHEMA.into(), version: "2.1".into(), topics: vec![sample_topic("t1")], parts: vec![BcfRawPart { name: "project.bcfp".into(), data: b"<ProjectExtension/>".to_vec() }] }
    }
    //#endregion Fixtures

    /// 🧪️ Full round trip through the real zip+xml codecs: version, topic markup (incl. the
    /// previously-mismodeled `Priority`/`Description`/`Labels`/`CreationDate`/`CreationAuthor`
    /// child elements), comments (incl. `viewpoint_ref`), and a viewpoint's camera/components/
    /// snapshot all survive.
    #[semio_framework_async_macros::async_test]
    async fn decode_of_encode_recovers_full_typed_model() {
        let snap = sample_snapshot();
        let bytes = encode_bcf(&snap).expect("encode");
        let decoded = decode_bcf(&bytes).expect("decode");

        assert_eq!(decoded.version, "2.1");
        assert_eq!(decoded.topics.len(), 1);
        let topic = &decoded.topics[0];
        assert_eq!(topic.guid, "t1");
        assert_eq!(topic.title, "Clash on Level 2");
        assert_eq!(topic.description, "MEP duct clashes with structural beam.");
        assert_eq!(topic.status, "Open");
        assert_eq!(topic.priority, "High");
        assert_eq!(topic.labels, vec!["Clash".to_string(), "MEP".to_string()]);
        assert_eq!(topic.creation_date, "2024-01-01T00:00:00+00:00");
        assert_eq!(topic.creation_author, "ueli@example.com");

        assert_eq!(topic.comments.len(), 1);
        assert_eq!(topic.comments[0].guid, "c1");
        assert_eq!(topic.comments[0].viewpoint_ref.as_deref(), Some("vp1"));

        assert_eq!(topic.viewpoints.len(), 1);
        let vp = &topic.viewpoints[0];
        assert_eq!(vp.guid, "vp1");
        assert_eq!(vp.camera, Some(perspective_camera()));
        assert_eq!(vp.components, Some(sample_components()));
        assert_eq!(vp.snapshot, Some(vec![0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a]));

        assert!(decoded.parts.iter().any(|p| p.name == "project.bcfp"));
    }

    /// 🧪️ Orthogonal camera round-trips too (the `xs:choice` sibling of `PerspectiveCamera`).
    #[semio_framework_async_macros::async_test]
    async fn orthogonal_camera_round_trips() {
        let mut snap = sample_snapshot();
        snap.topics[0].viewpoints[0].camera = Some(orthogonal_camera());
        let decoded = decode_bcf(&encode_bcf(&snap).unwrap()).unwrap();
        assert_eq!(decoded.topics[0].viewpoints[0].camera, Some(orthogonal_camera()));
    }

    /// 🧪️ A topic folder with no `markup.bcf` (only stray files) is retained verbatim as raw
    /// parts, never fabricated into a bogus topic.
    #[semio_framework_async_macros::async_test]
    async fn folder_without_markup_becomes_raw_parts() {
        let zip_snap = crate::artifacts::zip::ZipSnapshot {
            schema: crate::artifacts::zip::STDIO_ZIP_DOCUMENT_SCHEMA.into(),
            entries: vec![ZipEntry { name: "bcf.version".into(), data: bcf_version_bytes("2.1"), ..Default::default() }, ZipEntry { name: "stray/notes.txt".into(), data: b"not a topic".to_vec(), ..Default::default() }],
            comment: String::new(),
        };
        let bytes = crate::artifacts::zip::standards::v2_0::subsets::base::io::encode_zip(&zip_snap).unwrap();
        let decoded = decode_bcf(&bytes).unwrap();
        assert!(decoded.topics.is_empty());
        assert!(decoded.parts.iter().any(|p| p.name == "stray/notes.txt"));
    }

    #[semio_framework_async_macros::async_test]
    async fn empty_snapshot_matches_schema() {
        let snapshot = empty_bcf_snapshot();
        assert_eq!(snapshot.schema, STDIO_BCF_DOCUMENT_SCHEMA);
        assert!(snapshot.topics.is_empty());
        assert!(snapshot.parts.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn codec_round_trip() {
        let snap = decode_bcf(&encode_bcf(&sample_snapshot()).unwrap()).unwrap();

        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <BcfSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(parsed, snap);

        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <BcfSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }

    //#region 🧪️Law1_MutationDiffLaw
    /// ⚖️ Law 1 — `mutation_diff_law`: for every mutation variant, applying via
    /// `apply_bcf_mutation` matches `m.diff(base).diff().apply(base)`, and the returned diff equals
    /// `m.diff(base)`.
    #[semio_framework_async_macros::async_test]
    async fn mutation_diff_law() {
        let base = decode_bcf(&encode_bcf(&sample_snapshot()).unwrap()).unwrap();
        let mutations = vec![
            BcfMutation::SetVersion(set_version::SetVersion { version: "2.2".into() }),
            BcfMutation::InsertTopic(insert_topic::InsertTopic { topic: sample_topic("t2") }),
            BcfMutation::RemoveTopic(remove_topic::RemoveTopic { guid: "t1".into() }),
            BcfMutation::SetTopicMarkup(set_topic_markup::SetTopicMarkup { guid: "t1".into(), title: Some("Renamed".into()), description: None, status: Some("Closed".into()), priority: None, labels: Some(vec!["Renamed".into()]), creation_date: None, creation_author: None }),
            BcfMutation::InsertComment(insert_comment::InsertComment { topic_guid: "t1".into(), comment: sample_comment("c2", None) }),
            BcfMutation::RemoveComment(remove_comment::RemoveComment { topic_guid: "t1".into(), guid: "c1".into() }),
            BcfMutation::SetComment(set_comment::SetComment { topic_guid: "t1".into(), guid: "c1".into(), date: None, author: None, text: Some("Updated".into()), viewpoint_ref: Some(None) }),
            BcfMutation::InsertViewpoint(insert_viewpoint::InsertViewpoint { topic_guid: "t1".into(), viewpoint: sample_viewpoint("vp2") }),
            BcfMutation::RemoveViewpoint(remove_viewpoint::RemoveViewpoint { topic_guid: "t1".into(), guid: "vp1".into() }),
            BcfMutation::SetViewpointCamera(set_viewpoint_camera::SetViewpointCamera { topic_guid: "t1".into(), guid: "vp1".into(), camera: Some(orthogonal_camera()) }),
            BcfMutation::SetViewpointComponents(set_viewpoint_components::SetViewpointComponents { topic_guid: "t1".into(), guid: "vp1".into(), components: None }),
            BcfMutation::SetViewpointSnapshot(set_viewpoint_snapshot::SetViewpointSnapshot { topic_guid: "t1".into(), guid: "vp1".into(), snapshot: None }),
        ];
        for m in mutations {
            let mut snap = base.clone();
            let returned = apply_bcf_mutation(&mut snap, &m);
            let expected_diff = m.diff(&base);
            assert_eq!(returned, expected_diff, "returned diff mismatch for {m:?}");
            assert_eq!(snap, expected_diff.diff().apply(&base).expect("diff must apply to base"), "apply mismatch for {m:?}");
        }
    }
    //#endregion

    //#region 🧪️Law2_InverseLaw
    /// ⚖️ Law 2 — `inverse_law`: every mutation round-trips (mutation-level) and every diff
    /// round-trips (diff-level `d.diff().inverse(base).apply(&d.diff().apply(base)) == base`).
    #[semio_framework_async_macros::async_test]
    async fn inverse_law() {
        let base = decode_bcf(&encode_bcf(&sample_snapshot()).unwrap()).unwrap();
        let mutations = vec![
            BcfMutation::SetVersion(set_version::SetVersion { version: "2.2".into() }),
            BcfMutation::InsertTopic(insert_topic::InsertTopic { topic: sample_topic("t2") }),
            BcfMutation::RemoveTopic(remove_topic::RemoveTopic { guid: "t1".into() }),
            BcfMutation::SetTopicMarkup(set_topic_markup::SetTopicMarkup { guid: "t1".into(), title: Some("Renamed".into()), description: Some("New desc".into()), status: None, priority: None, labels: None, creation_date: None, creation_author: None }),
            BcfMutation::InsertComment(insert_comment::InsertComment { topic_guid: "t1".into(), comment: sample_comment("c2", None) }),
            BcfMutation::RemoveComment(remove_comment::RemoveComment { topic_guid: "t1".into(), guid: "c1".into() }),
            BcfMutation::SetComment(set_comment::SetComment { topic_guid: "t1".into(), guid: "c1".into(), date: Some("2025-01-01T00:00:00+00:00".into()), author: None, text: None, viewpoint_ref: None }),
            BcfMutation::InsertViewpoint(insert_viewpoint::InsertViewpoint { topic_guid: "t1".into(), viewpoint: sample_viewpoint("vp2") }),
            BcfMutation::RemoveViewpoint(remove_viewpoint::RemoveViewpoint { topic_guid: "t1".into(), guid: "vp1".into() }),
            BcfMutation::SetViewpointCamera(set_viewpoint_camera::SetViewpointCamera { topic_guid: "t1".into(), guid: "vp1".into(), camera: None }),
            BcfMutation::SetViewpointComponents(set_viewpoint_components::SetViewpointComponents { topic_guid: "t1".into(), guid: "vp1".into(), components: Some(sample_components()) }),
            BcfMutation::SetViewpointSnapshot(set_viewpoint_snapshot::SetViewpointSnapshot { topic_guid: "t1".into(), guid: "vp1".into(), snapshot: Some(vec![1, 2, 3]) }),
        ];
        for m in mutations {
            let mut snap = base.clone();
            apply_bcf_mutation(&mut snap, &m);
            for inv in m.inverse(&base) {
                let mut undone = snap.clone();
                apply_bcf_mutation(&mut undone, &inv);
                assert_eq!(undone, base, "mutation-level inverse mismatch for {m:?}");
            }

            let d = m.diff(&base);
            let after = d.diff().apply(&base).expect("diff must apply to base");
            let d_inv = d.diff().inverse(&base);
            assert_eq!(d_inv.apply(&after).expect("inverse diff must apply to after"), base, "diff-level inverse mismatch for {m:?}");
        }
    }
    //#endregion

    //#region 🧪️Law3_AbsorbLaw
    /// ⚖️ Law 3 — `absorb_law`: sequential-coalesce over a curated op list incl. the canonical
    /// cases (Insert+Remove-before, Insert+Insert-same-key-both-survive [name-keyed: no
    /// same-key clash needed, tested via disjoint-then-merge], Add+SetField patches into added,
    /// Modify+Remove annihilates) plus associativity.
    #[semio_framework_async_macros::async_test]
    async fn absorb_law() {
        let base = decode_bcf(&encode_bcf(&sample_snapshot()).unwrap()).unwrap();

        // Insert+Remove-before: insert t2, then remove t1 -- both survive independently (name-keyed,
        // no interaction), net effect must match sequential application.
        let d1 = BcfMutation::InsertTopic(insert_topic::InsertTopic { topic: sample_topic("t2") }).diff(&base);
        let mid = d1.diff().apply(&base).expect("d1 must apply to base");
        let d2 = BcfMutation::RemoveTopic(remove_topic::RemoveTopic { guid: "t1".into() }).diff(&mid);
        assert_absorb_matches_sequential(&base, d1.clone(), d2.clone());

        // Add+SetField: insert a comment, then immediately edit that SAME comment -- the edit must
        // patch into the carried `added` payload, not become a dangling `modified` entry.
        let comment = sample_comment("c9", None);
        let d1 = BcfMutation::InsertComment(insert_comment::InsertComment { topic_guid: "t1".into(), comment: comment.clone() }).diff(&base);
        let mid = d1.diff().apply(&base).expect("d1 must apply to base");
        let d2 = BcfMutation::SetComment(set_comment::SetComment { topic_guid: "t1".into(), guid: "c9".into(), date: None, author: None, text: Some("edited after insert".into()), viewpoint_ref: None }).diff(&mid);
        let absorbed = assert_absorb_matches_sequential(&base, d1, d2);
        let topics_diff = absorbed.topics.as_ref().expect("topics diff");
        let t1_diff = &topics_diff.modified.iter().find(|m| m.key == "t1").expect("t1 modified").diff;
        let comments_diff = t1_diff.comments.as_ref().expect("comments diff");
        assert!(comments_diff.modified.is_empty(), "edit-after-insert must patch into added, not appear as modified");
        let added_comment = comments_diff.added.iter().find(|c| c.guid == "c9").expect("c9 still in added");
        assert_eq!(added_comment.text, "edited after insert");

        // Modify+Remove: edit a viewpoint's camera, then remove that same viewpoint -- must
        // annihilate to a plain removal, not a dangling modify+remove pair.
        let d1 = BcfMutation::SetViewpointCamera(set_viewpoint_camera::SetViewpointCamera { topic_guid: "t1".into(), guid: "vp1".into(), camera: Some(orthogonal_camera()) }).diff(&base);
        let mid = d1.diff().apply(&base).expect("d1 must apply to base");
        let d2 = BcfMutation::RemoveViewpoint(remove_viewpoint::RemoveViewpoint { topic_guid: "t1".into(), guid: "vp1".into() }).diff(&mid);
        let absorbed = assert_absorb_matches_sequential(&base, d1, d2);
        let topics_diff = absorbed.topics.as_ref().expect("topics diff");
        let t1_diff = &topics_diff.modified.iter().find(|m| m.key == "t1").expect("t1 modified").diff;
        let viewpoints_diff = t1_diff.viewpoints.as_ref().expect("viewpoints diff");
        assert_eq!(viewpoints_diff.removed, vec!["vp1".to_string()]);
        assert!(viewpoints_diff.modified.is_empty());

        // Associativity: absorb(absorb(d1,d2),d3) == absorb(d1,absorb(d2,d3)).
        let d1 = BcfMutation::SetVersion(set_version::SetVersion { version: "2.2".into() }).diff(&base);
        let mid1 = d1.diff().apply(&base).expect("d1 must apply to base");
        let d2 = BcfMutation::InsertTopic(insert_topic::InsertTopic { topic: sample_topic("t3") }).diff(&mid1);
        let mid2 = d2.diff().apply(&mid1).expect("d2 must apply to mid1");
        let d3 = BcfMutation::SetTopicMarkup(set_topic_markup::SetTopicMarkup { guid: "t3".into(), title: Some("Renamed t3".into()), description: None, status: None, priority: None, labels: None, creation_date: None, creation_author: None }).diff(&mid2);

        let mut left = d1.diff().clone();
        MutationDiff::absorb(&mut left, d2.diff().clone());
        MutationDiff::absorb(&mut left, d3.diff().clone());

        let mut d2_d3 = d2.diff().clone();
        MutationDiff::absorb(&mut d2_d3, d3.diff().clone());
        let mut right = d1.diff().clone();
        MutationDiff::absorb(&mut right, d2_d3);

        assert_eq!(left.apply(&base).expect("left must apply to base"), right.apply(&base).expect("right must apply to base"), "absorb must be associative");
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn assert_absorb_matches_sequential(base: &BcfSnapshot, d1: protocol::MutationOutcome<BcfDiff>, d2: protocol::MutationOutcome<BcfDiff>) -> BcfDiff {
        let mid = d1.diff().apply(base).expect("d1 must apply to base");
        let sequential = d2.diff().apply(&mid).expect("d2 must apply to mid");
        let mut absorbed = d1.diff().clone();
        MutationDiff::absorb(&mut absorbed, d2.diff().clone());
        assert_eq!(absorbed.apply(base).expect("absorbed diff must apply to base"), sequential, "absorb(d1,d2).apply(base) must equal sequential application");
        absorbed
    }
    //#endregion

    //#region 🧪️Law4_BetweenRoundtripLaw
    /// ⚖️ Law 4 — `between_roundtrip_law`: `between(a,b).apply(a) == b` on real fixtures.
    #[semio_framework_async_macros::async_test]
    async fn between_roundtrip_law() {
        let a = decode_bcf(&encode_bcf(&sample_snapshot()).unwrap()).unwrap();
        let mut b = a.clone();
        b.version = "2.2".into();
        b.topics[0].title = "Renamed via between".into();
        b.topics[0].comments.push(sample_comment("c2", None));
        b.topics.push(sample_topic("t2"));
        b.parts.push(BcfRawPart { name: "extra.txt".into(), data: b"stray".to_vec() });

        let d = BcfDiff::between(&a, &b);
        assert_eq!(d.apply(&a).expect("d must apply to a"), b);
        let d_back = BcfDiff::between(&b, &a);
        assert_eq!(d_back.apply(&b).expect("d_back must apply to b"), a);
        assert!(BcfDiff::between(&a, &a).is_empty());
    }
    //#endregion

    //#region 🧪️Law5_CodecRetentionLaw
    /// ⚖️ Law 5 — `codec_retention_law`: decode(encode(x)) == x (this artifact's documented
    /// normal form for viewpoint/snapshot filenames -- see the snapshot module's `BcfViewpoint`
    /// doc comment).
    #[semio_framework_async_macros::async_test]
    async fn codec_retention_law() {
        let snap = decode_bcf(&encode_bcf(&sample_snapshot()).unwrap()).unwrap();
        let re_encoded = encode_bcf(&snap).unwrap();
        let re_decoded = decode_bcf(&re_encoded).unwrap();
        assert_eq!(re_decoded, snap);
    }
    //#endregion

    //#region 🧪️Law6_FieldSweep
    /// ⚖️ Law 6 — `field_sweep` (the acceptance criterion): `sweep_a`/`sweep_b` differ in EVERY
    /// mutable field, incl. per guid-keyed collection one removed/one modified-in-every-field/one
    /// added, and every tri-state field exercising `Some(None)`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sweep_a() -> BcfSnapshot {
        BcfSnapshot {
            schema: STDIO_BCF_DOCUMENT_SCHEMA.into(),
            version: "2.1".into(),
            topics: vec![
                BcfTopic {
                    guid: "keep".into(),
                    title: "Keep-topic before".into(),
                    description: "before desc".into(),
                    status: "Open".into(),
                    priority: "Low".into(),
                    labels: vec!["before".into()],
                    creation_date: "2024-01-01T00:00:00+00:00".into(),
                    creation_author: "a@example.com".into(),
                    // 🩹 Comment/viewpoint order here is deliberate, not arbitrary: `apply_named`
                    // reconstructs a collection as "surviving items in THIS snapshot's original
                    // relative order, then added items appended" (docx's own `f4-docx-report.md`
                    // §5 documents the identical order-sensitivity gotcha for its `overrides`
                    // list). `between(b,a).apply(b)` must reproduce `a`'s exact order, so the
                    // survivor (`c-keep`/`vp-keep`) is listed FIRST here, matching where it sits
                    // in `sweep_b` below -- otherwise the law would spuriously "fail" on order
                    // alone despite every field being correct.
                    comments: vec![
                        BcfComment { guid: "c-keep".into(), date: "2024-01-01T00:00:00+00:00".into(), author: "a@example.com".into(), text: "before text".into(), viewpoint_ref: Some("vp-remove".into()) },
                        BcfComment { guid: "c-remove".into(), date: "2024-01-01T00:00:00+00:00".into(), author: "a@example.com".into(), text: "will be removed".into(), viewpoint_ref: Some("vp-keep".into()) },
                    ],
                    viewpoints: vec![
                        BcfViewpoint { guid: "vp-keep".into(), camera: Some(perspective_camera()), components: Some(sample_components()), snapshot: Some(vec![2]) },
                        BcfViewpoint { guid: "vp-remove".into(), camera: Some(perspective_camera()), components: Some(sample_components()), snapshot: Some(vec![1]) },
                    ],
                },
                BcfTopic {
                    guid: "topic-remove".into(),
                    title: "Will be removed".into(),
                    description: String::new(),
                    status: "Open".into(),
                    priority: String::new(),
                    labels: Vec::new(),
                    creation_date: String::new(),
                    creation_author: String::new(),
                    comments: Vec::new(),
                    viewpoints: Vec::new(),
                },
            ],
            // 🩹 Same order-sensitivity as above: `part-keep.txt` (the survivor) listed first.
            parts: vec![BcfRawPart { name: "part-keep.txt".into(), data: b"before".to_vec() }, BcfRawPart { name: "part-remove.txt".into(), data: b"gone".to_vec() }],
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sweep_b() -> BcfSnapshot {
        BcfSnapshot {
            schema: STDIO_BCF_DOCUMENT_SCHEMA.into(),
            version: "2.2".into(),
            topics: vec![
                BcfTopic {
                    guid: "keep".into(),
                    title: "Keep-topic after".into(),
                    description: "after desc".into(),
                    status: "Closed".into(),
                    priority: "High".into(),
                    labels: vec!["after".into(), "second".into()],
                    creation_date: "2024-02-02T00:00:00+00:00".into(),
                    creation_author: "b@example.com".into(),
                    comments: vec![
                        BcfComment { guid: "c-keep".into(), date: "2024-02-02T00:00:00+00:00".into(), author: "b@example.com".into(), text: "after text".into(), viewpoint_ref: None },
                        BcfComment { guid: "c-add".into(), date: "2024-02-02T00:00:00+00:00".into(), author: "b@example.com".into(), text: "newly added".into(), viewpoint_ref: Some("vp-keep".into()) },
                    ],
                    viewpoints: vec![
                        BcfViewpoint { guid: "vp-keep".into(), camera: Some(orthogonal_camera()), components: None, snapshot: None },
                        BcfViewpoint { guid: "vp-add".into(), camera: None, components: Some(sample_components()), snapshot: Some(vec![9]) },
                    ],
                },
                BcfTopic {
                    guid: "topic-add".into(),
                    title: "Freshly added".into(),
                    description: "added desc".into(),
                    status: "Open".into(),
                    priority: "Medium".into(),
                    labels: vec!["fresh".into()],
                    creation_date: "2024-03-03T00:00:00+00:00".into(),
                    creation_author: "c@example.com".into(),
                    comments: Vec::new(),
                    viewpoints: Vec::new(),
                },
            ],
            parts: vec![BcfRawPart { name: "part-keep.txt".into(), data: b"after".to_vec() }, BcfRawPart { name: "part-add.txt".into(), data: b"new".to_vec() }],
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn field_sweep() {
        let a = sweep_a();
        let b = sweep_b();

        let forward = BcfDiff::between(&a, &b);
        assert_eq!(forward.apply(&a).expect("forward must apply to a"), b, "between(a,b).apply(a) must equal b");
        let backward = BcfDiff::between(&b, &a);
        assert_eq!(backward.apply(&b).expect("backward must apply to b"), a, "between(b,a).apply(b) must equal a");
        assert!(BcfDiff::between(&a, &a).is_empty(), "between(a,a) must be empty");

        // Every top-level field patched.
        assert!(forward.version.is_some(), "version field not swept");
        let topics_diff = forward.topics.as_ref().expect("topics diff present");
        assert!(!topics_diff.removed.is_empty(), "topics.removed not swept");
        assert!(!topics_diff.added.is_empty(), "topics.added not swept");
        let keep_diff = &topics_diff.modified.iter().find(|m| m.key == "keep").expect("keep topic modified").diff;

        // Every scalar field on the modified topic patched.
        assert!(keep_diff.title.is_some(), "topic.title not swept");
        assert!(keep_diff.description.is_some(), "topic.description not swept");
        assert!(keep_diff.status.is_some(), "topic.status not swept");
        assert!(keep_diff.priority.is_some(), "topic.priority not swept");
        assert!(keep_diff.labels.is_some(), "topic.labels not swept");
        assert!(keep_diff.creation_date.is_some(), "topic.creation_date not swept");
        assert!(keep_diff.creation_author.is_some(), "topic.creation_author not swept");

        let comments_diff = keep_diff.comments.as_ref().expect("comments diff present");
        assert!(!comments_diff.removed.is_empty(), "comments.removed not swept");
        assert!(!comments_diff.added.is_empty(), "comments.added not swept");
        let kept_comment_diff = &comments_diff.modified.iter().find(|m| m.key == "c-keep").expect("c-keep modified").diff;
        assert!(kept_comment_diff.date.is_some());
        assert!(kept_comment_diff.author.is_some());
        assert!(kept_comment_diff.text.is_some());
        assert_eq!(kept_comment_diff.viewpoint_ref, Some(None), "comment.viewpoint_ref tri-state Some(None) not swept");

        let viewpoints_diff = keep_diff.viewpoints.as_ref().expect("viewpoints diff present");
        assert!(!viewpoints_diff.removed.is_empty(), "viewpoints.removed not swept");
        assert!(!viewpoints_diff.added.is_empty(), "viewpoints.added not swept");
        let kept_vp_diff = &viewpoints_diff.modified.iter().find(|m| m.key == "vp-keep").expect("vp-keep modified").diff;
        assert!(kept_vp_diff.camera.is_some(), "viewpoint.camera not swept");
        assert_eq!(kept_vp_diff.components, Some(None), "viewpoint.components tri-state Some(None) not swept");
        assert_eq!(kept_vp_diff.snapshot, Some(None), "viewpoint.snapshot tri-state Some(None) not swept");

        let parts_diff = forward.parts.as_ref().expect("parts diff present");
        assert!(!parts_diff.removed.is_empty(), "parts.removed not swept");
        assert!(!parts_diff.added.is_empty(), "parts.added not swept");
        let kept_part_diff = &parts_diff.modified.iter().find(|m| m.key == "part-keep.txt").expect("part-keep modified").diff;
        assert!(kept_part_diff.data.is_some(), "part.data not swept");
    }
    //#endregion

    //#region 🧪️Law7_OpTextBinaryRoundtripLaw
    /// ⚖️ Law 7 — `op_text_binary_roundtrip_law` (F6): `OpText`/`OpBinary` round-trip laws for the
    /// hand-rolled `BcfMutation` grammar (`f6-bcf-report.md`) — exercises every variant incl.
    /// `SetViewpointCamera`'s `BcfCamera` enum payload (both `Perspective`/`Orthogonal`, plus
    /// `None`) and `SetComment`'s tri-state `viewpoint_ref` (both `Some(None)` and
    /// `Some(Some(_))`).
    #[semio_framework_async_macros::async_test]
    async fn op_text_binary_roundtrip_law() {
        let mutations = vec![
            BcfMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: sample_snapshot() }),
            BcfMutation::SetVersion(set_version::SetVersion { version: "2.2".into() }),
            BcfMutation::InsertTopic(insert_topic::InsertTopic { topic: sample_topic("t2") }),
            BcfMutation::RemoveTopic(remove_topic::RemoveTopic { guid: "t1".into() }),
            BcfMutation::SetTopicMarkup(set_topic_markup::SetTopicMarkup {
                guid: "t1".into(),
                title: Some("Renamed".into()),
                description: None,
                status: Some("Closed".into()),
                priority: None,
                labels: Some(vec!["Renamed".into(), "Second".into()]),
                creation_date: None,
                creation_author: None,
            }),
            BcfMutation::InsertComment(insert_comment::InsertComment { topic_guid: "t1".into(), comment: sample_comment("c2", Some("vp1")) }),
            BcfMutation::RemoveComment(remove_comment::RemoveComment { topic_guid: "t1".into(), guid: "c1".into() }),
            BcfMutation::SetComment(set_comment::SetComment { topic_guid: "t1".into(), guid: "c1".into(), date: None, author: None, text: Some("Updated".into()), viewpoint_ref: Some(None) }),
            BcfMutation::SetComment(set_comment::SetComment { topic_guid: "t1".into(), guid: "c1".into(), date: Some("2025-01-01T00:00:00+00:00".into()), author: Some("a@example.com".into()), text: None, viewpoint_ref: Some(Some("vp2".into())) }),
            BcfMutation::InsertViewpoint(insert_viewpoint::InsertViewpoint { topic_guid: "t1".into(), viewpoint: sample_viewpoint("vp2") }),
            BcfMutation::RemoveViewpoint(remove_viewpoint::RemoveViewpoint { topic_guid: "t1".into(), guid: "vp1".into() }),
            BcfMutation::SetViewpointCamera(set_viewpoint_camera::SetViewpointCamera { topic_guid: "t1".into(), guid: "vp1".into(), camera: Some(perspective_camera()) }),
            BcfMutation::SetViewpointCamera(set_viewpoint_camera::SetViewpointCamera { topic_guid: "t1".into(), guid: "vp1".into(), camera: Some(orthogonal_camera()) }),
            BcfMutation::SetViewpointCamera(set_viewpoint_camera::SetViewpointCamera { topic_guid: "t1".into(), guid: "vp1".into(), camera: None }),
            BcfMutation::SetViewpointComponents(set_viewpoint_components::SetViewpointComponents { topic_guid: "t1".into(), guid: "vp1".into(), components: Some(sample_components()) }),
            BcfMutation::SetViewpointComponents(set_viewpoint_components::SetViewpointComponents { topic_guid: "t1".into(), guid: "vp1".into(), components: None }),
            BcfMutation::SetViewpointSnapshot(set_viewpoint_snapshot::SetViewpointSnapshot { topic_guid: "t1".into(), guid: "vp1".into(), snapshot: Some(vec![1, 2, 3]) }),
            BcfMutation::SetViewpointSnapshot(set_viewpoint_snapshot::SetViewpointSnapshot { topic_guid: "t1".into(), guid: "vp1".into(), snapshot: None }),
        ];
        for m in mutations {
            let printed = m.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = BcfMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, m, "print_op/parse_op round-trip mismatch for {m:?} (printed {printed:?})");

            let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op({m:?}) failed: {e}"));
            let decoded = BcfMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, m, "encode_op/decode_op round-trip mismatch for {m:?}");
        }
    }
    //#endregion

    //#region 🧪️Law8_DiffCodecTextBinaryRoundtripLaw
    /// ⚖️ Law 8 — `diff_codec_text_binary_roundtrip_law` (F6): `DiffCodec` round-trip laws for the
    /// hand-rolled `BcfDiff` grammar — exercises every collection triple (`topics`/`comments`/
    /// `viewpoints`/`parts`, all guid/name-keyed) plus every tri-state field's `Some(None)`
    /// transition, via `sweep_a`/`sweep_b`'s `between()` result (the same fixtures `field_sweep`
    /// uses, incl. `BcfCamera`'s `Perspective`->`Orthogonal` transition inside `vp-keep`'s diff).
    #[semio_framework_async_macros::async_test]
    async fn diff_codec_text_binary_roundtrip_law() {
        let a = sweep_a();
        let b = sweep_b();
        let cases = vec![BcfDiff::default(), BcfDiff::between(&a, &b), BcfDiff::between(&b, &a), BcfDiff::between(&a, &a)];
        for d in cases {
            let printed = d.print_diff();
            assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
            let parsed = BcfDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
            assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

            let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
            let decoded = BcfDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
            assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
        }
    }
    //#endregion

    //#region 🔖️ConformanceLaws
    /// 🧪️ FG-wave: per-artifact conformance laws (`📖️grammar-recipe.md` §4's checklist item) --
    /// grammar/protocol parseability, `Recognizer` against real fixtures AND real `print_op`/
    /// `print_diff` output, `walk_protocol` against real `encode_pack`/`encode_op`/`encode_diff`
    /// bytes, and the fixture-honesty round-trip. Lives here (the engine's own test region), not
    /// any framework file -- same placement `📜️docx/…/⚙️engine/🦀️.rs`'s own
    /// `conformance_laws` module uses; these tests are this artifact's OWN early-warning, plus
    /// direct coverage of the mutations/diff facets the framework's `m5` auto-discovery does not
    /// reach at all.
    mod conformance_laws {
        use super::*;
        use crate::artifacts::bcf::schema::{diff, mutations, snapshot};
        use protocol::{DiffCodec, OpBinary, OpText};

        /// ✅️ "committed files parse": all 6 handcrafted `.grammar.semio`/`.protocol.semio` files
        /// parse under the real dialect -- independent of, and cheaper than, the two
        /// `recognize`/`walk_protocol` laws below (a parse failure here fails fast with a clearer
        /// message).
        #[semio_framework_async_macros::async_test]
        async fn committed_facet_files_parse() {
            for (label, text) in [("snapshot grammar", snapshot::text::COMPONENT_GRAMMAR_SEMIO), ("mutations grammar", mutations::text::COMPONENT_GRAMMAR_SEMIO), ("diff grammar", diff::text::COMPONENT_GRAMMAR_SEMIO)] {
                let grammar = dsl::parse_grammar(text).unwrap_or_else(|e| panic!("{label}: parse_grammar failed: {e:?}"));
                assert_eq!(grammar.dialect, dsl::SemioDialect::Grammar, "{label}: expected grammar dialect");
            }
            for (label, text) in [("snapshot protocol", snapshot::binary::COMPONENT_PROTOCOL_SEMIO), ("mutations protocol", mutations::binary::COMPONENT_PROTOCOL_SEMIO), ("diff protocol", diff::binary::COMPONENT_PROTOCOL_SEMIO)] {
                dsl::parse_protocol(text).unwrap_or_else(|e| panic!("{label}: parse_protocol failed: {e:?}"));
            }
        }

        /// ✅️ `grammar_conformance_law`: the snapshot grammar models the real TEXT syntax of bcf's
        /// own plain-zip container's XML parts (`📸️snapshot/📝️text/📖️.grammar.semio`'s own
        /// doc comment explains why -- this artifact's `ArtifactDsl::print_dsl` hex-dumps the WHOLE
        /// binary zip container, matching this facet's SIBLING binary protocol, not this text
        /// grammar; the two facets describe different LAYERS of the same real artifact). So, UNLIKE
        /// a binary-native pilot's `grammar_conformance_law` (which feeds `print_dsl` output
        /// straight to the recognizer), this law decodes the REAL zip entries `encode_bcf`
        /// genuinely produces (via `zip::engine::decode_zip`, the same real codec this artifact's
        /// own `encode_bcf`/`decode_bcf` delegate to directly) and recognizes EACH real part's own
        /// text against the grammar -- direct proof the grammar matches this artifact's own real
        /// per-part XML bytes, not an invented approximation.
        #[semio_framework_async_macros::async_test]
        async fn grammar_conformance_law() {
            let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);

            let demo = demo_bcf_snapshot();
            let bytes = encode_bcf(&demo).expect("encode demo bcf");
            let zip = crate::artifacts::zip::standards::v2_0::subsets::base::io::decode_zip(&bytes).expect("decode zip");

            let mut checked = 0;
            for entry in &zip.entries {
                let is_version = entry.name.eq_ignore_ascii_case("bcf.version");
                let is_markup = entry.name.ends_with("/markup.bcf");
                let is_visinfo = entry.name.ends_with(".bcfv");
                if !(is_version || is_markup || is_visinfo) {
                    continue;
                }
                let text = String::from_utf8(entry.data.clone()).unwrap_or_else(|e| panic!("part {:?}: not valid utf-8: {e}", entry.name));
                assert!(recognizer.recognize(&text).unwrap_or(false), "grammar did not recognize real part {:?}:\n{text}", entry.name);
                checked += 1;
            }
            assert_eq!(checked, 3, "not every modeled part kind (version/markup/visinfo) was present in the real zip entries");
        }

        /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op`
        /// output for every `BcfMutation` variant (`mutations::demo_mutation_cases()`).
        #[semio_framework_async_macros::async_test]
        async fn ops_grammar_conformance_law() {
            let grammar = dsl::parse_grammar(mutations::text::COMPONENT_GRAMMAR_SEMIO).expect("parse mutations grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            for mutation in mutations::demo_mutation_cases() {
                let printed = mutation.print_op();
                assert!(recognizer.recognize(&printed).unwrap_or(false), "mutations grammar did not recognize {printed:?} (from {mutation:?})");
            }
        }

        /// ✅️ `diff_grammar_conformance_law`: the diff grammar recognizes real `print_diff` output
        /// for every representative `BcfDiff` (`diff::demo_diff_cases()`).
        #[semio_framework_async_macros::async_test]
        async fn diff_grammar_conformance_law() {
            let grammar = dsl::parse_grammar(diff::text::COMPONENT_GRAMMAR_SEMIO).expect("parse diff grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            for d in diff::demo_diff_cases() {
                let printed = d.print_diff();
                assert!(recognizer.recognize(&printed).unwrap_or(false), "diff grammar did not recognize {printed:?} (from {d:?})");
            }
        }

        /// ✅️ `protocol_walk_law`: `walk_protocol` against REAL bytes for all three facets --
        /// snapshot pack (`encode_pack`, envelope-unwrapped first, matching how
        /// `m5_handcrafted_protocol_conformance` itself feeds `walk_protocol`), every demo
        /// mutation's `encode_op`, and every demo diff's `encode_diff`. The snapshot protocol
        /// declares `backward`/`jump` (restated from zip's own real ZIP layout), so `walk_protocol`
        /// correctly does NOT require landing on exactly `bytes.len()` (M2's own documented
        /// exception, `📖️grammar-recipe.md` §2.3) -- assert a sane in-range `consumed` there
        /// instead, same as zip's/docx's own `protocol_walk_law` does; the op/diff protocols have
        /// no such exception and must consume every byte.
        #[semio_framework_async_macros::async_test]
        async fn protocol_walk_law() {
            let pack_spec = dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
            let demo = demo_bcf_snapshot();
            let packed = store::ArtifactPack::encode_pack(&demo);
            let (_, inner) = store::semio_format::unwrap_binary(&packed).expect("unwrap semio envelope");
            let trace = dsl::walk_protocol(&pack_spec, &inner).unwrap_or_else(|e| panic!("walk_protocol(pack) failed @{}: {}", e.offset, e.message));
            assert!(trace.consumed > 0 && trace.consumed <= inner.len(), "pack walk consumed an out-of-range span");

            let op_spec = dsl::parse_protocol(mutations::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse mutations protocol");
            for mutation in mutations::demo_mutation_cases() {
                let bytes = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op failed for {mutation:?}: {e:?}"));
                let trace = dsl::walk_protocol(&op_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(op) failed for {mutation:?} @{}: {}", e.offset, e.message));
                assert_eq!(trace.consumed, bytes.len(), "op walk did not consume every byte for {mutation:?}");
            }

            let diff_spec = dsl::parse_protocol(diff::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse diff protocol");
            for d in diff::demo_diff_cases() {
                let bytes = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed for {d:?}: {e:?}"));
                let trace = dsl::walk_protocol(&diff_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(diff) failed for {d:?} @{}: {}", e.offset, e.message));
                assert_eq!(trace.consumed, bytes.len(), "diff walk did not consume every byte for {d:?}");
            }
        }

        /// ✅️ `fixture_honesty_law`: the shipped `.dsl.semio`/`.pack.semio` fixtures are GENUINE
        /// `print_dsl`/`encode_pack` output of `demo_bcf_snapshot()` -- `parse_dsl(fixture) ==
        /// demo()`, `print_dsl(demo()) == fixture` (byte-for-byte), and the pack twin -- so the
        /// fixtures can never silently drift back to a fake `"68656c6c6f"`-style placeholder again
        /// (this ticket's own recon note on the pre-FG-wave state of these two files -- the
        /// `.dsl.semio` fixture WAS exactly that placeholder before this wave).
        #[semio_framework_async_macros::async_test]
        async fn fixture_honesty_law() {
            const FIXTURE_DSL: &str = include_str!("../📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio");
            const FIXTURE_PACK: &[u8] = include_bytes!("../📚️examples/🎬️demo/🖼️assets/🎒️.pack.semio");

            let demo = demo_bcf_snapshot();

            let parsed = <BcfSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
            assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_bcf_snapshot()");
            assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_bcf_snapshot()) drifted from the shipped .dsl.semio fixture");

            let decoded = <BcfSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
            assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_bcf_snapshot()");
            assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_bcf_snapshot()) drifted from the shipped .pack.semio fixture");
        }
    }
    //#endregion 🔖️ConformanceLaws
}
//#endregion 🧪️Tests

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::artifacts::bcf::standards::v2_1::subsets::any::schema::BcfComposer as BcfRawAnyComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<BcfRawAnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
