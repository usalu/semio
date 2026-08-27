//! 🔮️ Mutation oracle for this subset — every mutation kind the subset declares, performed
//! independently of this repository's own codec so the subject has something real to be compared
//! against instead of being checked against its own reading.
//!
//! Reference: `zip` 6 (the bcfzip container itself) composed with `quick-xml` 0.42 (every XML part
//! inside it: `bcf.version`, `markup.bcf`, `.bcfv`). A BCF file IS a ZIP of XML markup, viewpoint
//! and snapshot files — no standalone BCF crate exists in the Rust ecosystem (BCF support only
//! appears bundled inside much larger MPL-licensed IFC toolkits), so composing two already-linked,
//! genuinely independent crates over the real BCF-XML 2.1 `markup.xsd`/`visinfo.xsd` shapes IS the
//! right oracle here, not a gap to fill with a weaker substitute. Every type, parser and writer
//! below is a fresh, independent implementation of that shape — it never imports
//! `semio-s-plugin-stdio`'s own `BcfSnapshot`/`XmlNode`/`ZipEntry` types (see this crate's own
//! purity gate).
//!
//! The vocabulary is per SUBSET, not per artifact: two standards of the same format declare
//! different mutations, and a subset that shares an implementation with another reaches it through
//! the shared family modules rather than by copying it. BCF has no shared family helper (unlike
//! `document`/`raster`/`archive`/...): the `zip`+`quick-xml` composition it needs is specific to
//! this one subset's container shape, so it lives here rather than in a shared module.
//!
//! Two entry points mirror the `📰xml`/`🎨️svg` precedent: [`oracle_apply_mutation`] performs the
//! FORWARD mutation (the `mutate-<kind>` scenarios), [`oracle_apply_mutation_inverse`] performs the
//! forward mutation and then its computed inverse in sequence (the `inverse-<kind>` scenarios) —
//! the same "apply, then apply the inverse, land back on the start" law `BcfMutation::inverse`
//! proves at the Rust-model level, proven here independently against the registered reference
//! libraries. [`project_bcf_2_1`] is the shared independent-reader projection both this module's own
//! handlers AND the case's subject handlers read their results back through before comparison.
//!
//! Binary payloads (a viewpoint's PNG snapshot, a raw retained part's bytes) travel through mutation
//! params as lowercase hex — the same convention `BcfSnapshot::parse_dsl`/`print_dsl` already use
//! for this artifact's own whole-document DSL encoding, kept here rather than reaching for a new
//! base64 dependency this artifact does not otherwise need.
//!
//! @see ../🧪️oracle/🔣️.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️component.rs — the mutation vocabulary itself (`BcfMutation::KINDS`).

use semio_repo_test_host::Json;

#[cfg(feature = "oracles")]
//#region 🔖️Oracles
mod oracles {
    use quick_xml::escape::resolve_xml_entity;
    use quick_xml::events::{BytesDecl, BytesEnd, BytesRef, BytesStart, BytesText, Event};
    use quick_xml::reader::Reader;
    use quick_xml::writer::Writer;
    use quick_xml::XmlVersion;
    use semio_repo_test_host::{digest, Json};
    use std::collections::{BTreeMap, HashSet};
    use std::io::Cursor;

    //#region 🔖️Model
    /// 📐 A 3D point/vector — the independent mirror of `visinfo.xsd`'s `Point`/`Direction`
    /// (`{X,Y,Z}` triples), unrelated to `semio-s-plugin-stdio`'s own `BcfPoint3`.
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    struct OPoint3 {
        x: f64,
        y: f64,
        z: f64,
    }

    /// 📷 A viewpoint's camera — `visinfo.xsd`'s `PerspectiveCamera`/`OrthogonalCamera` `xs:choice`.
    #[derive(Clone, Debug, PartialEq)]
    enum OCamera {
        Perspective { view_point: OPoint3, direction: OPoint3, up_vector: OPoint3, field_of_view: f64 },
        Orthogonal { view_point: OPoint3, direction: OPoint3, up_vector: OPoint3, view_to_world_scale: f64 },
    }

    #[derive(Clone, Debug, Default, PartialEq)]
    struct OVisibility {
        default_visibility: bool,
        exceptions: Vec<String>,
    }

    #[derive(Clone, Debug, Default, PartialEq)]
    struct OColoring {
        color: String,
        components: Vec<String>,
    }

    #[derive(Clone, Debug, Default, PartialEq)]
    struct OComponents {
        selection: Vec<String>,
        visibility: OVisibility,
        coloring: Vec<OColoring>,
    }

    #[derive(Clone, Debug, Default, PartialEq)]
    struct OComment {
        guid: String,
        date: String,
        author: String,
        text: String,
        viewpoint_ref: Option<String>,
    }

    #[derive(Clone, Debug, Default, PartialEq)]
    struct OViewpoint {
        guid: String,
        camera: Option<OCamera>,
        components: Option<OComponents>,
        snapshot: Option<Vec<u8>>,
    }

    #[derive(Clone, Debug, Default, PartialEq)]
    struct OTopic {
        guid: String,
        title: String,
        description: String,
        status: String,
        priority: String,
        labels: Vec<String>,
        creation_date: String,
        creation_author: String,
        comments: Vec<OComment>,
        viewpoints: Vec<OViewpoint>,
    }

    #[derive(Clone, Debug, Default, PartialEq)]
    struct ORawPart {
        name: String,
        data: Vec<u8>,
    }

    #[derive(Clone, Debug, Default, PartialEq)]
    struct ODoc {
        version: String,
        topics: Vec<OTopic>,
        parts: Vec<ORawPart>,
    }
    //#endregion 🔖️Model

    //#region 🔖️Hex
    /// 🔤️ Lowercase hex, the same binary-in-text convention `BcfSnapshot::print_dsl` already uses
    /// for this artifact's whole-document DSL form.
    fn hex_encode(bytes: &[u8]) -> String {
        bytes.iter().map(|byte| format!("{byte:02x}")).collect()
    }

    fn hex_decode(text: &str) -> Result<Vec<u8>, String> {
        if text.len() % 2 != 0 {
            return Err(format!("odd hex length ({} chars)", text.len()));
        }
        (0..text.len()).step_by(2).map(|i| u8::from_str_radix(&text[i..i + 2], 16).map_err(|error| format!("invalid hex {:?}: {error}", &text[i..i + 2]))).collect()
    }
    //#endregion 🔖️Hex

    //#region 🔖️XmlTree
    /// 🌳 Minimal owned XML node — element-with-attributes-and-children or leaf text — sufficient
    /// for BCF-XML 2.1's `markup.xsd`/`visinfo.xsd` shapes (no CDATA/comment/PI content anywhere in
    /// a real BCF part). Independent of every other subset's own tree type (each subset that builds
    /// on `quick-xml` owns its own, per the wave 7 fleet brief's XML precedent).
    #[derive(Clone, Debug, PartialEq)]
    enum BNode {
        Element { name: String, attrs: Vec<(String, String)>, children: Vec<BNode> },
        Text(String),
    }

    fn as_element(node: &BNode) -> Option<(&str, &[(String, String)], &[BNode])> {
        match node {
            BNode::Element { name, attrs, children } => Some((name.as_str(), attrs.as_slice(), children.as_slice())),
            BNode::Text(_) => None,
        }
    }

    fn find_child<'a>(children: &'a [BNode], name: &str) -> Option<&'a BNode> {
        children.iter().find(|child| as_element(child).map(|(n, _, _)| n == name).unwrap_or(false))
    }

    fn find_children<'a>(children: &'a [BNode], name: &str) -> Vec<&'a BNode> {
        children.iter().filter(|child| as_element(child).map(|(n, _, _)| n == name).unwrap_or(false)).collect()
    }

    fn attr<'a>(attrs: &'a [(String, String)], name: &str) -> Option<&'a str> {
        attrs.iter().find(|(key, _)| key == name).map(|(_, value)| value.as_str())
    }

    fn text_content(node: &BNode) -> String {
        let Some((_, _, children)) = as_element(node) else { return String::new() };
        children.iter().filter_map(|child| if let BNode::Text(text) = child { Some(text.as_str()) } else { None }).collect()
    }

    fn text_element(name: &str, text: &str) -> Option<BNode> {
        if text.is_empty() {
            return None;
        }
        Some(BNode::Element { name: name.to_string(), attrs: Vec::new(), children: vec![BNode::Text(text.to_string())] })
    }
    //#endregion 🔖️XmlTree

    //#region 🔖️XmlParse
    /// 🔓️ Resolves one `Event::GeneralRef` (`&name;` or `&#NNN;`) to its literal text — this
    /// `quick-xml` version splits every entity reference out of `Text` into its own event, exactly
    /// the same `📰xml`/`🎨️svg` precedent already established.
    fn resolve_general_ref(reference: &BytesRef) -> Result<String, String> {
        if let Some(ch) = reference.resolve_char_ref().map_err(|error| error.to_string())? {
            return Ok(ch.to_string());
        }
        match resolve_xml_entity(reference.as_ref()) {
            Some(resolved) => Ok(resolved.to_string()),
            None => Err(format!("unknown entity &{};", reference.as_ref())),
        }
    }

    fn read_attrs(start: &BytesStart) -> Result<Vec<(String, String)>, String> {
        start
            .attributes()
            .map(|attr| {
                let attr = attr.map_err(|error| error.to_string())?;
                let value = attr.normalized_value(XmlVersion::Explicit1_0).map_err(|error| error.to_string())?;
                Ok((attr.key.as_ref().to_string(), value.to_string()))
            })
            .collect()
    }

    fn flush_text(text_run: &mut String, children: &mut Vec<BNode>) {
        if !text_run.is_empty() {
            children.push(BNode::Text(std::mem::take(text_run)));
        }
    }

    fn parse_element(reader: &mut Reader<&[u8]>, start: BytesStart) -> Result<BNode, String> {
        let name = start.name().as_ref().to_string();
        let attrs = read_attrs(&start)?;
        let mut children = Vec::new();
        let mut text_run = String::new();
        loop {
            let event = reader.read_event().map_err(|error| format!("quick-xml parse error at byte {}: {error}", reader.error_position()))?;
            match event {
                Event::End(_) => {
                    flush_text(&mut text_run, &mut children);
                    return Ok(BNode::Element { name, attrs, children });
                }
                Event::Start(child_start) => {
                    flush_text(&mut text_run, &mut children);
                    children.push(parse_element(reader, child_start)?);
                }
                Event::Empty(child_start) => {
                    flush_text(&mut text_run, &mut children);
                    children.push(BNode::Element { name: child_start.name().as_ref().to_string(), attrs: read_attrs(&child_start)?, children: Vec::new() });
                }
                Event::Text(text) => text_run.push_str(text.as_ref()),
                Event::GeneralRef(reference) => text_run.push_str(&resolve_general_ref(&reference)?),
                Event::CData(_) => return Err(format!("unexpected CDATA inside <{name}> — BCF-XML 2.1 parts never carry it")),
                Event::Comment(_) | Event::PI(_) => {}
                Event::Eof => return Err(format!("unclosed element <{name}>: unexpected end of input")),
                Event::Decl(_) | Event::DocType(_) => return Err(format!("declaration/doctype cannot appear inside element <{name}>")),
            }
        }
    }

    fn parse_xml(bytes: &[u8]) -> Result<BNode, String> {
        let text = std::str::from_utf8(bytes).map_err(|error| error.to_string())?;
        let mut reader = Reader::from_str(text);
        let mut root: Option<BNode> = None;
        loop {
            let event = reader.read_event().map_err(|error| format!("quick-xml parse error at byte {}: {error}", reader.error_position()))?;
            match event {
                Event::Eof => break,
                Event::Decl(_) | Event::PI(_) | Event::Comment(_) | Event::DocType(_) => {}
                Event::Text(text) => {
                    if !text.as_ref().trim().is_empty() {
                        return Err("unexpected text outside the root element".to_string());
                    }
                }
                Event::GeneralRef(reference) => return Err(format!("unexpected entity reference &{}; outside the root element", reference.as_ref())),
                Event::CData(_) => return Err("unexpected CDATA outside the root element".to_string()),
                Event::Start(start) => {
                    if root.is_some() {
                        return Err("multiple root elements".to_string());
                    }
                    root = Some(parse_element(&mut reader, start)?);
                }
                Event::Empty(start) => {
                    if root.is_some() {
                        return Err("multiple root elements".to_string());
                    }
                    root = Some(BNode::Element { name: start.name().as_ref().to_string(), attrs: read_attrs(&start)?, children: Vec::new() });
                }
                Event::End(_) => return Err("unexpected closing tag before the root element".to_string()),
            }
        }
        root.ok_or_else(|| "document has no root element".to_string())
    }
    //#endregion 🔖️XmlParse

    //#region 🔖️XmlWrite
    fn write_node<W: std::io::Write>(writer: &mut Writer<W>, node: &BNode) -> Result<(), String> {
        match node {
            BNode::Text(text) => writer.write_event(Event::Text(BytesText::new(text))).map_err(|error| error.to_string()),
            BNode::Element { name, attrs, children } => {
                let mut start = BytesStart::new(name.as_str());
                for (key, value) in attrs {
                    start.push_attribute((key.as_str(), value.as_str()));
                }
                if children.is_empty() {
                    return writer.write_event(Event::Empty(start)).map_err(|error| error.to_string());
                }
                writer.write_event(Event::Start(start)).map_err(|error| error.to_string())?;
                for child in children {
                    write_node(writer, child)?;
                }
                writer.write_event(Event::End(BytesEnd::new(name.as_str()))).map_err(|error| error.to_string())
            }
        }
    }

    /// 📝️ Serializes one XML part (`bcf.version`, a `markup.bcf`, a `.bcfv`) with a leading
    /// `<?xml version="1.0" encoding="UTF-8"?>` declaration — every write to an in-memory
    /// `Cursor<Vec<u8>>` here, so failure is not a real possibility for a tree this module built
    /// itself.
    fn write_xml(root: &BNode) -> Vec<u8> {
        let mut writer = Writer::new(Cursor::new(Vec::new()));
        writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None))).expect("declaration write to an in-memory buffer cannot fail");
        write_node(&mut writer, root).expect("element write to an in-memory buffer cannot fail");
        writer.into_inner().into_inner()
    }
    //#endregion 🔖️XmlWrite

    //#region 🔖️Archive
    /// 🎒️ Reads every non-directory ZIP member with the registered `zip` reference reader.
    fn read_entries(input: &[u8]) -> Result<Vec<(String, Vec<u8>)>, String> {
        use std::io::Read;
        let mut archive = zip::ZipArchive::new(Cursor::new(input.to_vec())).map_err(|error| format!("independent reader could not parse the ZIP: {error}"))?;
        let mut entries = Vec::with_capacity(archive.len());
        for index in 0..archive.len() {
            let mut member = archive.by_index(index).map_err(|error| format!("independent reader could not read ZIP entry {index}: {error}"))?;
            if member.is_dir() {
                continue;
            }
            let name = member.name().to_string();
            let mut data = Vec::new();
            member.read_to_end(&mut data).map_err(|error| format!("independent reader could not decompress {name}: {error}"))?;
            entries.push((name, data));
        }
        Ok(entries)
    }

    /// 🎒️ Writes every ZIP member with the registered `zip` reference writer, deflated — the same
    /// flat bcfzip shape `bcfzip` uses (no OPC apparatus, per the format's own definition).
    fn write_entries(entries: &[(String, Vec<u8>)]) -> Result<Vec<u8>, String> {
        use std::io::Write;
        let mut writer = zip::ZipWriter::new(Cursor::new(Vec::<u8>::new()));
        let options: zip::write::FileOptions<'_, ()> = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);
        for (name, data) in entries {
            writer.start_file(name.clone(), options).map_err(|error| format!("zip start_file {name}: {error}"))?;
            writer.write_all(data).map_err(|error| format!("zip write {name}: {error}"))?;
        }
        Ok(writer.finish().map_err(|error| format!("zip finish: {error}"))?.into_inner())
    }
    //#endregion 🔖️Archive

    //#region 🔖️MarkupXml
    struct ViewpointRefRaw {
        guid: String,
        viewpoint_file: Option<String>,
        snapshot_file: Option<String>,
    }

    /// 🧩️ Parses one topic folder's `markup.bcf` (BCF-XML 2.1 `markup.xsd`): root `<Markup>` with a
    /// required `<Topic Guid="..." TopicStatus="...">` carrying `<Title>`/optional `<Priority>`/
    /// `<Labels>`*/`<CreationDate>`/`<CreationAuthor>`/`<Description>` CHILD elements, zero-or-more
    /// sibling `<Comment Guid="...">` elements each with `<Date>`/`<Author>`/`<Comment>`/optional
    /// `<Viewpoint Guid="...">`, and zero-or-more `<Viewpoints Guid="...">` reference entries.
    fn parse_markup(data: &[u8]) -> Result<(OTopic, Vec<ViewpointRefRaw>), String> {
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
    fn markup_bytes(topic: &OTopic) -> Vec<u8> {
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

    fn bcf_version_bytes(version: &str) -> Vec<u8> {
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
    fn parse_visualization_info(data: &[u8]) -> Result<(Option<OCamera>, Option<OComponents>), String> {
        let root = parse_xml(data)?;
        let (name, _, children) = as_element(&root).ok_or("bcfv: root is not an element")?;
        if name != "VisualizationInfo" {
            return Err(format!("bcfv: expected root <VisualizationInfo>, got <{name}>"));
        }
        Ok((parse_camera(children), find_child(children, "Components").map(parse_components)))
    }

    fn visualization_info_bytes(viewpoint: &OViewpoint) -> Vec<u8> {
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

    //#region 🔖️Codec
    /// 🔮️ Decodes a real bcfzip: `bcf.version`, every topic folder's `markup.bcf` + referenced
    /// `.bcfv` + snapshot, and every unconsumed entry retained as a raw part — the same shape
    /// `semio-s-plugin-stdio`'s own `decode_bcf` implements, composed here independently.
    fn decode(bytes: &[u8]) -> Result<ODoc, String> {
        let entries = read_entries(bytes)?;
        let mut version = String::new();
        let mut consumed: HashSet<String> = HashSet::new();
        if let Some((name, data)) = entries.iter().find(|(entry_name, _)| entry_name.eq_ignore_ascii_case("bcf.version")) {
            let root = parse_xml(data)?;
            if let Some(("Version", attrs, _)) = as_element(&root) {
                version = attr(attrs, "VersionId").unwrap_or_default().to_string();
            }
            consumed.insert(name.clone());
        }

        let mut folders: BTreeMap<&str, Vec<&(String, Vec<u8>)>> = BTreeMap::new();
        for entry in &entries {
            if let Some((folder, _)) = entry.0.split_once('/') {
                folders.entry(folder).or_default().push(entry);
            }
        }

        let mut topics = Vec::new();
        for (folder, folder_entries) in &folders {
            let markup_name = format!("{folder}/markup.bcf");
            let Some((markup_entry_name, markup_data)) = folder_entries.iter().find(|(name, _)| name.eq_ignore_ascii_case(&markup_name)) else { continue };
            let (mut topic, viewpoint_refs) = parse_markup(markup_data)?;
            consumed.insert(markup_entry_name.clone());

            let mut viewpoints = Vec::new();
            for reference in &viewpoint_refs {
                let mut camera = None;
                let mut components = None;
                if let Some(viewpoint_file) = &reference.viewpoint_file {
                    let full = format!("{folder}/{viewpoint_file}");
                    if let Some((name, data)) = folder_entries.iter().find(|(name, _)| name.eq_ignore_ascii_case(&full)) {
                        let (c, comp) = parse_visualization_info(data)?;
                        camera = c;
                        components = comp;
                        consumed.insert(name.clone());
                    }
                }
                let mut snapshot = None;
                if let Some(snapshot_file) = &reference.snapshot_file {
                    let full = format!("{folder}/{snapshot_file}");
                    if let Some((name, data)) = folder_entries.iter().find(|(name, _)| name.eq_ignore_ascii_case(&full)) {
                        snapshot = Some(data.clone());
                        consumed.insert(name.clone());
                    }
                }
                viewpoints.push(OViewpoint { guid: reference.guid.clone(), camera, components, snapshot });
            }
            topic.viewpoints = viewpoints;
            topics.push(topic);
        }

        let parts = entries.iter().filter(|(name, _)| !consumed.contains(name)).map(|(name, data)| ORawPart { name: name.clone(), data: data.clone() }).collect();
        Ok(ODoc { version, topics, parts })
    }

    /// 🔮️ Re-encodes a full `ODoc` into a real bcfzip.
    fn encode(doc: &ODoc) -> Result<Vec<u8>, String> {
        let mut entries: Vec<(String, Vec<u8>)> = vec![("bcf.version".to_string(), bcf_version_bytes(&doc.version))];
        for topic in &doc.topics {
            entries.push((format!("{}/markup.bcf", topic.guid), markup_bytes(topic)));
            for viewpoint in &topic.viewpoints {
                entries.push((format!("{}/{}.bcfv", topic.guid, viewpoint.guid), visualization_info_bytes(viewpoint)));
                if let Some(snapshot) = &viewpoint.snapshot {
                    entries.push((format!("{}/{}.png", topic.guid, viewpoint.guid), snapshot.clone()));
                }
            }
        }
        for part in &doc.parts {
            entries.push((part.name.clone(), part.data.clone()));
        }
        write_entries(&entries)
    }
    //#endregion 🔖️Codec

    //#region 🔖️JsonValue
    fn obj(entries: Vec<(&str, Json)>) -> Json {
        Json::Object(entries.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
    }

    fn strings(items: Vec<Json>) -> Vec<String> {
        items.into_iter().filter_map(|item| if let Json::String(text) = item { Some(text) } else { None }).collect()
    }

    fn strings_json(items: &[String]) -> Json {
        Json::Array(items.iter().map(|item| Json::String(item.clone())).collect())
    }

    fn number_field(value: &Json, key: &str) -> f64 {
        match value.get(key) {
            Some(Json::Number(number)) => *number,
            _ => 0.0,
        }
    }

    fn point_from_json(value: &Json) -> OPoint3 {
        OPoint3 { x: number_field(value, "x"), y: number_field(value, "y"), z: number_field(value, "z") }
    }

    fn point_to_json(point: &OPoint3) -> Json {
        obj(vec![("x", Json::Number(point.x)), ("y", Json::Number(point.y)), ("z", Json::Number(point.z))])
    }

    /// 🔎️ `{"kind":"perspective","viewPoint":{x,y,z},"direction":{...},"upVector":{...},
    /// "fieldOfView":...}` | `{"kind":"orthogonal",...,"viewToWorldScale":...}` — mirrors
    /// `BcfCamera`'s own `#[serde(tag="kind")]` shape field-for-field.
    fn camera_from_json(value: &Json) -> Result<OCamera, String> {
        let view_point = value.get("viewPoint").map(point_from_json).unwrap_or_default();
        let direction = value.get("direction").map(point_from_json).unwrap_or_default();
        let up_vector = value.get("upVector").map(point_from_json).unwrap_or_default();
        match value.str("kind").as_str() {
            "perspective" => Ok(OCamera::Perspective { view_point, direction, up_vector, field_of_view: number_field(value, "fieldOfView") }),
            "orthogonal" => Ok(OCamera::Orthogonal { view_point, direction, up_vector, view_to_world_scale: number_field(value, "viewToWorldScale") }),
            other => Err(format!("unknown camera kind {other:?}")),
        }
    }

    fn camera_to_json(camera: &OCamera) -> Json {
        match camera {
            OCamera::Perspective { view_point, direction, up_vector, field_of_view } => {
                obj(vec![("kind", Json::String("perspective".to_string())), ("viewPoint", point_to_json(view_point)), ("direction", point_to_json(direction)), ("upVector", point_to_json(up_vector)), ("fieldOfView", Json::Number(*field_of_view))])
            }
            OCamera::Orthogonal { view_point, direction, up_vector, view_to_world_scale } => obj(vec![
                ("kind", Json::String("orthogonal".to_string())),
                ("viewPoint", point_to_json(view_point)),
                ("direction", point_to_json(direction)),
                ("upVector", point_to_json(up_vector)),
                ("viewToWorldScale", Json::Number(*view_to_world_scale)),
            ]),
        }
    }

    /// 🔎️ `{"selection":[guid,...],"visibility":{"defaultVisibility":bool,"exceptions":[guid,...]},
    /// "coloring":[{"color":"FFRRGGBB","components":[guid,...]}]}` — mirrors `BcfComponents`.
    fn components_from_json(value: &Json) -> OComponents {
        let visibility = match value.get("visibility") {
            Some(node) => OVisibility {
                default_visibility: match node.get("defaultVisibility") {
                    Some(Json::Bool(flag)) => *flag,
                    _ => true,
                },
                exceptions: strings(node.array("exceptions")),
            },
            None => OVisibility { default_visibility: true, exceptions: Vec::new() },
        };
        OComponents { selection: strings(value.array("selection")), visibility, coloring: value.array("coloring").iter().map(|entry| OColoring { color: entry.str("color"), components: strings(entry.array("components")) }).collect() }
    }

    fn components_to_json(components: &OComponents) -> Json {
        obj(vec![
            ("selection", strings_json(&components.selection)),
            ("visibility", obj(vec![("defaultVisibility", Json::Bool(components.visibility.default_visibility)), ("exceptions", strings_json(&components.visibility.exceptions))])),
            ("coloring", Json::Array(components.coloring.iter().map(|coloring| obj(vec![("color", Json::String(coloring.color.clone())), ("components", strings_json(&coloring.components))])).collect())),
        ])
    }

    fn comment_from_json(value: &Json) -> OComment {
        OComment {
            guid: value.str("guid"),
            date: value.str("date"),
            author: value.str("author"),
            text: value.str("text"),
            viewpoint_ref: match value.get("viewpointRef") {
                Some(Json::String(text)) if !text.is_empty() => Some(text.clone()),
                _ => None,
            },
        }
    }

    fn comment_to_json(comment: &OComment) -> Json {
        obj(vec![
            ("guid", Json::String(comment.guid.clone())),
            ("date", Json::String(comment.date.clone())),
            ("author", Json::String(comment.author.clone())),
            ("text", Json::String(comment.text.clone())),
            (
                "viewpointRef",
                match &comment.viewpoint_ref {
                    Some(reference) => Json::String(reference.clone()),
                    None => Json::Null,
                },
            ),
        ])
    }

    fn viewpoint_from_json(value: &Json) -> Result<OViewpoint, String> {
        let camera = match value.get("camera") {
            Some(Json::Null) | None => None,
            Some(node) => Some(camera_from_json(node)?),
        };
        let components = match value.get("components") {
            Some(Json::Null) | None => None,
            Some(node) => Some(components_from_json(node)),
        };
        let snapshot = match value.get("snapshot") {
            Some(Json::String(hex)) if !hex.is_empty() => Some(hex_decode(hex)?),
            _ => None,
        };
        Ok(OViewpoint { guid: value.str("guid"), camera, components, snapshot })
    }

    fn viewpoint_to_json(viewpoint: &OViewpoint) -> Json {
        obj(vec![
            ("guid", Json::String(viewpoint.guid.clone())),
            (
                "camera",
                match &viewpoint.camera {
                    Some(camera) => camera_to_json(camera),
                    None => Json::Null,
                },
            ),
            (
                "components",
                match &viewpoint.components {
                    Some(components) => components_to_json(components),
                    None => Json::Null,
                },
            ),
            (
                "snapshot",
                match &viewpoint.snapshot {
                    Some(bytes) => Json::String(hex_encode(bytes)),
                    None => Json::Null,
                },
            ),
        ])
    }

    fn topic_from_json(value: &Json) -> Result<OTopic, String> {
        Ok(OTopic {
            guid: value.str("guid"),
            title: value.str("title"),
            description: value.str("description"),
            status: value.str("status"),
            priority: value.str("priority"),
            labels: strings(value.array("labels")),
            creation_date: value.str("creationDate"),
            creation_author: value.str("creationAuthor"),
            comments: value.array("comments").iter().map(comment_from_json).collect(),
            viewpoints: value.array("viewpoints").iter().map(viewpoint_from_json).collect::<Result<_, _>>()?,
        })
    }

    fn topic_to_json(topic: &OTopic) -> Json {
        obj(vec![
            ("guid", Json::String(topic.guid.clone())),
            ("title", Json::String(topic.title.clone())),
            ("description", Json::String(topic.description.clone())),
            ("status", Json::String(topic.status.clone())),
            ("priority", Json::String(topic.priority.clone())),
            ("labels", strings_json(&topic.labels)),
            ("creationDate", Json::String(topic.creation_date.clone())),
            ("creationAuthor", Json::String(topic.creation_author.clone())),
            ("comments", Json::Array(topic.comments.iter().map(comment_to_json).collect())),
            ("viewpoints", Json::Array(topic.viewpoints.iter().map(viewpoint_to_json).collect())),
        ])
    }

    fn doc_from_json(value: &Json) -> Result<ODoc, String> {
        Ok(ODoc {
            version: value.str("version"),
            topics: value.array("topics").iter().map(topic_from_json).collect::<Result<_, _>>()?,
            parts: value
                .array("parts")
                .iter()
                .map(|entry| {
                    let data = match entry.get("content") {
                        Some(Json::String(hex)) if !hex.is_empty() => hex_decode(hex)?,
                        _ => Vec::new(),
                    };
                    Ok(ORawPart { name: entry.str("name"), data })
                })
                .collect::<Result<_, String>>()?,
        })
    }

    fn doc_to_json(doc: &ODoc) -> Json {
        obj(vec![
            ("version", Json::String(doc.version.clone())),
            ("topics", Json::Array(doc.topics.iter().map(topic_to_json).collect())),
            ("parts", Json::Array(doc.parts.iter().map(|part| obj(vec![("name", Json::String(part.name.clone())), ("content", Json::String(hex_encode(&part.data)))])).collect())),
        ])
    }
    //#endregion 🔖️JsonValue

    //#region 🔖️Forward
    fn find_viewpoint_mut<'a>(doc: &'a mut ODoc, topic_guid: &str, guid: &str) -> Result<&'a mut OViewpoint, String> {
        let topic = doc.topics.iter_mut().find(|topic| topic.guid == topic_guid).ok_or_else(|| format!("no topic named {topic_guid:?}"))?;
        topic.viewpoints.iter_mut().find(|viewpoint| viewpoint.guid == guid).ok_or_else(|| format!("no viewpoint named {guid:?}"))
    }

    /// 🦠️ Applies one declared mutation kind, described by `spec` (`{"kind": ..., "params": {...}}`),
    /// to an already-decoded document. An unrecognised kind, or a named topic/comment/viewpoint that
    /// does not exist, is an error — never a silent no-op.
    fn apply_kind(doc: &mut ODoc, kind: &str, params: &Json) -> Result<(), String> {
        match kind {
            "no-mutation" => {}
            "set-snapshot" => *doc = doc_from_json(params)?,
            "set-version" => doc.version = params.str("version"),
            "insert-topic" => {
                let topic = topic_from_json(&params.get("topic").cloned().unwrap_or(Json::Null))?;
                if doc.topics.iter().any(|existing| existing.guid == topic.guid) {
                    return Err(format!("insert-topic: a topic named {:?} already exists", topic.guid));
                }
                doc.topics.push(topic);
            }
            "remove-topic" => {
                let guid = params.str("guid");
                let before = doc.topics.len();
                doc.topics.retain(|topic| topic.guid != guid);
                if doc.topics.len() == before {
                    return Err(format!("remove-topic: no topic named {guid:?}"));
                }
            }
            "set-topic-markup" => {
                let guid = params.str("guid");
                let topic = doc.topics.iter_mut().find(|topic| topic.guid == guid).ok_or_else(|| format!("set-topic-markup: no topic named {guid:?}"))?;
                if let Some(Json::String(value)) = params.get("title") {
                    topic.title = value.clone();
                }
                if let Some(Json::String(value)) = params.get("description") {
                    topic.description = value.clone();
                }
                if let Some(Json::String(value)) = params.get("status") {
                    topic.status = value.clone();
                }
                if let Some(Json::String(value)) = params.get("priority") {
                    topic.priority = value.clone();
                }
                if let Some(Json::Array(_)) = params.get("labels") {
                    topic.labels = strings(params.array("labels"));
                }
                if let Some(Json::String(value)) = params.get("creationDate") {
                    topic.creation_date = value.clone();
                }
                if let Some(Json::String(value)) = params.get("creationAuthor") {
                    topic.creation_author = value.clone();
                }
            }
            "insert-comment" => {
                let topic_guid = params.str("topicGuid");
                let comment = comment_from_json(&params.get("comment").cloned().unwrap_or(Json::Null));
                let topic = doc.topics.iter_mut().find(|topic| topic.guid == topic_guid).ok_or_else(|| format!("insert-comment: no topic named {topic_guid:?}"))?;
                if topic.comments.iter().any(|existing| existing.guid == comment.guid) {
                    return Err(format!("insert-comment: a comment named {:?} already exists", comment.guid));
                }
                topic.comments.push(comment);
            }
            "remove-comment" => {
                let topic_guid = params.str("topicGuid");
                let guid = params.str("guid");
                let topic = doc.topics.iter_mut().find(|topic| topic.guid == topic_guid).ok_or_else(|| format!("remove-comment: no topic named {topic_guid:?}"))?;
                let before = topic.comments.len();
                topic.comments.retain(|comment| comment.guid != guid);
                if topic.comments.len() == before {
                    return Err(format!("remove-comment: no comment named {guid:?}"));
                }
            }
            "set-comment" => {
                let topic_guid = params.str("topicGuid");
                let guid = params.str("guid");
                let topic = doc.topics.iter_mut().find(|topic| topic.guid == topic_guid).ok_or_else(|| format!("set-comment: no topic named {topic_guid:?}"))?;
                let comment = topic.comments.iter_mut().find(|comment| comment.guid == guid).ok_or_else(|| format!("set-comment: no comment named {guid:?}"))?;
                if let Some(Json::String(value)) = params.get("date") {
                    comment.date = value.clone();
                }
                if let Some(Json::String(value)) = params.get("author") {
                    comment.author = value.clone();
                }
                if let Some(Json::String(value)) = params.get("text") {
                    comment.text = value.clone();
                }
                if let Some(value) = params.get("viewpointRef") {
                    comment.viewpoint_ref = match value {
                        Json::String(reference) if !reference.is_empty() => Some(reference.clone()),
                        _ => None,
                    };
                }
            }
            "insert-viewpoint" => {
                let topic_guid = params.str("topicGuid");
                let viewpoint = viewpoint_from_json(&params.get("viewpoint").cloned().unwrap_or(Json::Null))?;
                let topic = doc.topics.iter_mut().find(|topic| topic.guid == topic_guid).ok_or_else(|| format!("insert-viewpoint: no topic named {topic_guid:?}"))?;
                if topic.viewpoints.iter().any(|existing| existing.guid == viewpoint.guid) {
                    return Err(format!("insert-viewpoint: a viewpoint named {:?} already exists", viewpoint.guid));
                }
                topic.viewpoints.push(viewpoint);
            }
            "remove-viewpoint" => {
                let topic_guid = params.str("topicGuid");
                let guid = params.str("guid");
                let topic = doc.topics.iter_mut().find(|topic| topic.guid == topic_guid).ok_or_else(|| format!("remove-viewpoint: no topic named {topic_guid:?}"))?;
                let before = topic.viewpoints.len();
                topic.viewpoints.retain(|viewpoint| viewpoint.guid != guid);
                if topic.viewpoints.len() == before {
                    return Err(format!("remove-viewpoint: no viewpoint named {guid:?}"));
                }
            }
            "set-viewpoint-camera" => {
                let viewpoint = find_viewpoint_mut(doc, &params.str("topicGuid"), &params.str("guid"))?;
                viewpoint.camera = match params.get("camera") {
                    Some(Json::Null) | None => None,
                    Some(node) => Some(camera_from_json(node)?),
                };
            }
            "set-viewpoint-components" => {
                let viewpoint = find_viewpoint_mut(doc, &params.str("topicGuid"), &params.str("guid"))?;
                viewpoint.components = match params.get("components") {
                    Some(Json::Null) | None => None,
                    Some(node) => Some(components_from_json(node)),
                };
            }
            "set-viewpoint-snapshot" => {
                let viewpoint = find_viewpoint_mut(doc, &params.str("topicGuid"), &params.str("guid"))?;
                viewpoint.snapshot = match params.get("snapshot") {
                    Some(Json::String(hex)) if !hex.is_empty() => Some(hex_decode(hex)?),
                    _ => None,
                };
            }
            other => return Err(format!("mutation kind {other:?} has no oracle implementation")),
        }
        Ok(())
    }
    //#endregion 🔖️Forward

    //#region 🔖️Inverse
    /// ↩️ Reads `base` (the CURRENT, pre-mutation document) to build the spec that undoes `{kind,
    /// params}` — same law `BcfMutation::inverse` proves at the Rust-model level, computed here
    /// against the reference libraries instead.
    fn inverse_spec(base: &ODoc, kind: &str, params: &Json) -> Json {
        let spec = |inverse_kind: &str, inverse_params: Json| obj(vec![("kind", Json::String(inverse_kind.to_string())), ("params", inverse_params)]);
        let find_topic = |guid: &str| base.topics.iter().find(|topic| topic.guid == guid);
        let find_comment = |topic_guid: &str, guid: &str| find_topic(topic_guid).and_then(|topic| topic.comments.iter().find(|comment| comment.guid == guid));
        let find_viewpoint = |topic_guid: &str, guid: &str| find_topic(topic_guid).and_then(|topic| topic.viewpoints.iter().find(|viewpoint| viewpoint.guid == guid));

        match kind {
            "no-mutation" => spec("no-mutation", obj(vec![])),
            "set-snapshot" => spec("set-snapshot", doc_to_json(base)),
            "set-version" => spec("set-version", obj(vec![("version", Json::String(base.version.clone()))])),
            "insert-topic" => spec("remove-topic", obj(vec![("guid", Json::String(params.get("topic").map(|topic| topic.str("guid")).unwrap_or_default()))])),
            "remove-topic" => match find_topic(&params.str("guid")) {
                Some(topic) => spec("insert-topic", obj(vec![("topic", topic_to_json(topic))])),
                None => spec("no-mutation", obj(vec![])),
            },
            "set-topic-markup" => {
                let guid = params.str("guid");
                let mut inverse_params = vec![("guid", Json::String(guid.clone()))];
                if let Some(topic) = find_topic(&guid) {
                    if params.get("title").is_some() {
                        inverse_params.push(("title", Json::String(topic.title.clone())));
                    }
                    if params.get("description").is_some() {
                        inverse_params.push(("description", Json::String(topic.description.clone())));
                    }
                    if params.get("status").is_some() {
                        inverse_params.push(("status", Json::String(topic.status.clone())));
                    }
                    if params.get("priority").is_some() {
                        inverse_params.push(("priority", Json::String(topic.priority.clone())));
                    }
                    if params.get("labels").is_some() {
                        inverse_params.push(("labels", strings_json(&topic.labels)));
                    }
                    if params.get("creationDate").is_some() {
                        inverse_params.push(("creationDate", Json::String(topic.creation_date.clone())));
                    }
                    if params.get("creationAuthor").is_some() {
                        inverse_params.push(("creationAuthor", Json::String(topic.creation_author.clone())));
                    }
                }
                spec("set-topic-markup", obj(inverse_params))
            }
            "insert-comment" => spec("remove-comment", obj(vec![("topicGuid", Json::String(params.str("topicGuid"))), ("guid", Json::String(params.get("comment").map(|comment| comment.str("guid")).unwrap_or_default()))])),
            "remove-comment" => match find_comment(&params.str("topicGuid"), &params.str("guid")) {
                Some(comment) => spec("insert-comment", obj(vec![("topicGuid", Json::String(params.str("topicGuid"))), ("comment", comment_to_json(comment))])),
                None => spec("no-mutation", obj(vec![])),
            },
            "set-comment" => {
                let topic_guid = params.str("topicGuid");
                let guid = params.str("guid");
                let mut inverse_params = vec![("topicGuid", Json::String(topic_guid.clone())), ("guid", Json::String(guid.clone()))];
                if let Some(comment) = find_comment(&topic_guid, &guid) {
                    if params.get("date").is_some() {
                        inverse_params.push(("date", Json::String(comment.date.clone())));
                    }
                    if params.get("author").is_some() {
                        inverse_params.push(("author", Json::String(comment.author.clone())));
                    }
                    if params.get("text").is_some() {
                        inverse_params.push(("text", Json::String(comment.text.clone())));
                    }
                    if params.get("viewpointRef").is_some() {
                        inverse_params.push((
                            "viewpointRef",
                            match &comment.viewpoint_ref {
                                Some(reference) => Json::String(reference.clone()),
                                None => Json::Null,
                            },
                        ));
                    }
                }
                spec("set-comment", obj(inverse_params))
            }
            "insert-viewpoint" => spec("remove-viewpoint", obj(vec![("topicGuid", Json::String(params.str("topicGuid"))), ("guid", Json::String(params.get("viewpoint").map(|viewpoint| viewpoint.str("guid")).unwrap_or_default()))])),
            "remove-viewpoint" => match find_viewpoint(&params.str("topicGuid"), &params.str("guid")) {
                Some(viewpoint) => spec("insert-viewpoint", obj(vec![("topicGuid", Json::String(params.str("topicGuid"))), ("viewpoint", viewpoint_to_json(viewpoint))])),
                None => spec("no-mutation", obj(vec![])),
            },
            "set-viewpoint-camera" => {
                let camera = find_viewpoint(&params.str("topicGuid"), &params.str("guid")).and_then(|viewpoint| viewpoint.camera.as_ref());
                spec(
                    "set-viewpoint-camera",
                    obj(vec![
                        ("topicGuid", Json::String(params.str("topicGuid"))),
                        ("guid", Json::String(params.str("guid"))),
                        (
                            "camera",
                            match camera {
                                Some(camera) => camera_to_json(camera),
                                None => Json::Null,
                            },
                        ),
                    ]),
                )
            }
            "set-viewpoint-components" => {
                let components = find_viewpoint(&params.str("topicGuid"), &params.str("guid")).and_then(|viewpoint| viewpoint.components.as_ref());
                spec(
                    "set-viewpoint-components",
                    obj(vec![
                        ("topicGuid", Json::String(params.str("topicGuid"))),
                        ("guid", Json::String(params.str("guid"))),
                        (
                            "components",
                            match components {
                                Some(components) => components_to_json(components),
                                None => Json::Null,
                            },
                        ),
                    ]),
                )
            }
            "set-viewpoint-snapshot" => {
                let snapshot = find_viewpoint(&params.str("topicGuid"), &params.str("guid")).and_then(|viewpoint| viewpoint.snapshot.as_ref());
                spec(
                    "set-viewpoint-snapshot",
                    obj(vec![
                        ("topicGuid", Json::String(params.str("topicGuid"))),
                        ("guid", Json::String(params.str("guid"))),
                        (
                            "snapshot",
                            match snapshot {
                                Some(bytes) => Json::String(hex_encode(bytes)),
                                None => Json::Null,
                            },
                        ),
                    ]),
                )
            }
            other => spec(other, params.clone()),
        }
    }
    //#endregion 🔖️Inverse

    //#region 🔖️Routing
    pub fn apply_mutation(input: &[u8], kind: &str, params: &Json) -> Result<Vec<u8>, String> {
        let mut doc = decode(input)?;
        apply_kind(&mut doc, kind, params)?;
        encode(&doc)
    }

    /// ↩️ Applies `{kind, params}` and then its computed inverse, in sequence — the caller compares
    /// its projection against the ORIGINAL input's own.
    pub fn apply_mutation_inverse(input: &[u8], kind: &str, params: &Json) -> Result<Vec<u8>, String> {
        let base = decode(input)?;
        let inverse = inverse_spec(&base, kind, params);
        let mutated = apply_mutation(input, kind, params)?;
        apply_mutation(&mutated, &inverse.str("kind"), &inverse.get("params").cloned().unwrap_or(Json::Null))
    }

    //#region 🔖️Projection
    fn topic_projection(topic: &OTopic) -> Json {
        obj(vec![
            ("title", Json::String(topic.title.clone())),
            ("description", Json::String(topic.description.clone())),
            ("status", Json::String(topic.status.clone())),
            ("priority", Json::String(topic.priority.clone())),
            ("labels", strings_json(&topic.labels)),
            ("creationDate", Json::String(topic.creation_date.clone())),
            ("creationAuthor", Json::String(topic.creation_author.clone())),
            ("comments", Json::Object(topic.comments.iter().map(|comment| (comment.guid.clone(), comment_to_json(comment))).collect())),
            ("viewpoints", Json::Object(topic.viewpoints.iter().map(|viewpoint| (viewpoint.guid.clone(), viewpoint_projection(viewpoint))).collect())),
        ])
    }

    /// 👁️ A viewpoint's snapshot projects as a size+digest pair, not raw bytes — the same
    /// content-addressed treatment the `🎒️zip`/`🎨️svg` precedents already give opaque binary
    /// payloads, so the comparison mechanism's per-number tolerance never has to look inside a PNG.
    fn viewpoint_projection(viewpoint: &OViewpoint) -> Json {
        obj(vec![
            (
                "camera",
                match &viewpoint.camera {
                    Some(camera) => camera_to_json(camera),
                    None => Json::Null,
                },
            ),
            (
                "components",
                match &viewpoint.components {
                    Some(components) => components_to_json(components),
                    None => Json::Null,
                },
            ),
            (
                "snapshotSize",
                match &viewpoint.snapshot {
                    Some(bytes) => Json::Number(bytes.len() as f64),
                    None => Json::Null,
                },
            ),
            (
                "snapshotDigest",
                match &viewpoint.snapshot {
                    Some(bytes) => Json::String(digest(bytes)),
                    None => Json::Null,
                },
            ),
        ])
    }

    /// 👁️ This subset's own semantic projection — version, every topic keyed by guid (comments and
    /// viewpoints keyed by guid in turn), and every raw retained part keyed by name — independently
    /// re-derived by re-decoding `bytes` through this module's own `zip`+`quick-xml` composition.
    pub fn project(bytes: &[u8]) -> Result<Json, String> {
        let doc = decode(bytes)?;
        Ok(obj(vec![
            ("version", Json::String(doc.version)),
            ("topics", Json::Object(doc.topics.iter().map(|topic| (topic.guid.clone(), topic_projection(topic))).collect())),
            ("parts", Json::Object(doc.parts.iter().map(|part| (part.name.clone(), obj(vec![("size", Json::Number(part.data.len() as f64)), ("digest", Json::String(digest(&part.data)))]))).collect())),
        ]))
    }
    //#endregion 🔖️Projection
    //#endregion 🔖️Routing
}
//#endregion 🔖️Oracles

//#region 🔖️Dispatch
/// 🦠️ Applies one declared mutation kind to a real artifact and returns the re-serialized bytes.
/// An unrecognised kind is an error, never a silent no-op: a mutation that is quietly skipped
/// reports as a passing test.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    let kind = spec.str("kind");
    if kind.is_empty() {
        return Err("mutation spec carries no `kind`".to_string());
    }
    oracles::apply_mutation(input, &kind, &spec.get("params").cloned().unwrap_or(Json::Null))
}

/// ↩️ Applies one declared mutation kind and then its own computed inverse, in sequence, proving the
/// same `apply(inverse(m, base), apply(m, base)) == base` law `BcfMutation::inverse` proves at the
/// Rust-model level, here against the registered `zip`+`quick-xml` reference composition instead.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation_inverse(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    let kind = spec.str("kind");
    if kind.is_empty() {
        return Err("mutation spec carries no `kind`".to_string());
    }
    oracles::apply_mutation_inverse(input, &kind, &spec.get("params").cloned().unwrap_or(Json::Null))
}

/// 👁️ This subset's own semantic projection. @see [`oracles::project`].
#[cfg(feature = "oracles")]
pub fn project_bcf_2_1(bytes: &[u8]) -> Result<Json, String> {
    oracles::project(bytes)
}

/// 🚫️ Without the `oracles` feature the reference implementations are not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation_inverse(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn project_bcf_2_1(_bytes: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch
