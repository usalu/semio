//! 🔮️ Mutation oracle for this subset — every mutation kind the subset declares, performed
//! independently of this repository's own codec so the subject has something real to be compared
//! against instead of being checked against its own reading.
//!
//! Reference: `zip` (unzip/rezip the OPC container) composed with `quick-xml` (parse/edit/
//! re-serialize `ppt/presentation.xml` + every `ppt/slides/slideN.xml`) — a PPTX is a ZIP of XML
//! parts, and both crates are already linked and genuinely independent of this repository's own
//! codec. This is the same composition the 📰️xml subset's oracle uses for its `quick-xml` half and
//! the 🎒zip subset's for its archive half, written fresh here (PPTX has no shared family module to
//! reach either through).
//!
//! **Design**: every slide and shape kind is a pure edit of one XML part's own parsed tree, addressed by the wire
//! address's part path and child-index node path exactly as the vocabulary defines it: an entry or shape inserted at a
//! vacancy among the `p:sldId` (or shape) siblings, one removed, the slide list reordered, every `a:t` of a shape
//! rewritten, or its first `a:xfrm` repositioned, and `replace-xml-node` swaps one addressed XML node for the wire's replacement. Revisions are the subject's guard and are not
//! re-derived here; a stale row shows as a refused subject result that the comparison then fails. Only the edited parts
//! are re-serialized; the identity round trip still regenerates every slide part from the typed slide/shape list.
//!
//! The vocabulary is per SUBSET, not per artifact: two standards of the same format declare
//! different mutations, and a subset that shares an implementation with another reaches it through
//! the shared family modules rather than by copying it.
//!
//! @see ../🔣️oracle.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️.rs — the mutation vocabulary itself (`PptxMutation`'s
//! 9 variants).

use semio_repo_test_host::Json;

//#region 🔖️Vocabulary
/// 🧾️ Kebab-case spelling of every variant this subset's `PptxMutation` declares, in declaration
/// order. The `pptx-ecma-376-base` catalog is measured against this exact list, and the
/// production-side `kinds_matches_enum_variants_and_manifest` proves enum, constant and manifest
/// never drift apart. Declared here rather than in the case adapter so the adapter, this module's
/// own law tests and the manifest all read ONE list.
pub const KINDS: &[&str] = &["insert-slide", "remove-slide", "move-slide", "insert-shape", "remove-shape", "set-shape-text", "set-shape-position", "replace-xml-node", "set-relationship", "remove-relationship", "set-content-type", "remove-content-type"];
//#endregion 🔖️Vocabulary

#[cfg(feature = "oracles")]
//#region 🔖️Oracles
mod oracles {
    use quick_xml::escape::resolve_xml_entity;
    use quick_xml::events::{BytesEnd, BytesRef, BytesStart, BytesText, Event};
    use quick_xml::reader::Reader;
    use quick_xml::writer::Writer;
    use quick_xml::XmlVersion;
    use semio_repo_test_host::Json;
    use std::collections::HashMap;
    use std::io::{Cursor, Read, Write};

    //#region 🔖️Tree
    /// 🌳 Owned XML node — element or text only, the scope every real OPC XML part this oracle
    /// touches (`presentation.xml`, `slideN.xml`, `.rels`, `[Content_Types].xml`) actually needs.
    #[derive(Clone, Debug, PartialEq)]
    enum XNode {
        Element { name: String, attrs: Vec<(String, String)>, children: Vec<XNode> },
        Text(String),
    }

    impl XNode {
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free) — see R9
        fn child(&self, name: &str) -> Option<&XNode> {
            let XNode::Element { children, .. } = self else { return None };
            children.iter().find(|c| matches!(c, XNode::Element { name: n, .. } if n == name))
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free) — see R9
        fn attr(&self, key: &str) -> Option<&str> {
            let XNode::Element { attrs, .. } = self else { return None };
            attrs.iter().find(|(k, _)| k == key).map(|(_, v)| v.as_str())
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free) — see R9
        fn children_named<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &'a XNode> {
            let empty: &[XNode] = &[];
            let items = match self {
                XNode::Element { children, .. } => children.as_slice(),
                XNode::Text(_) => empty,
            };
            items.iter().filter(move |c| matches!(c, XNode::Element { name: n, .. } if n == name))
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free) — see R9
        fn text(&self) -> String {
            match self {
                XNode::Text(t) => t.clone(),
                XNode::Element { children, .. } => children.iter().map(|c| c.text()).collect(),
            }
        }
    }
    //#endregion 🔖️Tree

    //#region 🔖️Parse
    /// 🔓️ Resolves one `Event::GeneralRef` (`&name;` or `&#NNN;`) to its literal text.
    fn resolve_general_ref(reference: &BytesRef) -> Result<String, String> {
        if let Some(ch) = reference.resolve_char_ref().map_err(|e| e.to_string())? {
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
                let attr = attr.map_err(|e| e.to_string())?;
                let value = attr.normalized_value(XmlVersion::Explicit1_0).map_err(|e| e.to_string())?;
                Ok((attr.key.as_ref().to_string(), value.to_string()))
            })
            .collect()
    }

    fn flush_text(run: &mut String, children: &mut Vec<XNode>) {
        if !run.is_empty() {
            children.push(XNode::Text(std::mem::take(run)));
        }
    }

    /// 🌳 Recursive-descent element parse: reads events until this element's own `End`.
    fn parse_element(reader: &mut Reader<&[u8]>, start: BytesStart) -> Result<XNode, String> {
        let name = start.name().as_ref().to_string();
        let attrs = read_attrs(&start)?;
        let mut children = Vec::new();
        let mut run = String::new();
        loop {
            let event = reader.read_event().map_err(|e| format!("quick-xml parse error at byte {}: {e}", reader.error_position()))?;
            match event {
                Event::End(_) => {
                    flush_text(&mut run, &mut children);
                    return Ok(XNode::Element { name, attrs, children });
                }
                Event::Start(child_start) => {
                    flush_text(&mut run, &mut children);
                    children.push(parse_element(reader, child_start)?);
                }
                Event::Empty(child_start) => {
                    flush_text(&mut run, &mut children);
                    children.push(XNode::Element { name: child_start.name().as_ref().to_string(), attrs: read_attrs(&child_start)?, children: Vec::new() });
                }
                Event::Text(text) => run.push_str(text.as_ref()),
                Event::GeneralRef(reference) => run.push_str(&resolve_general_ref(&reference)?),
                Event::CData(cdata) => run.push_str(&cdata.into_inner()),
                Event::Comment(_) | Event::PI(_) => {}
                Event::Eof => return Err(format!("unclosed element <{name}>: unexpected end of input")),
                Event::Decl(_) | Event::DocType(_) => return Err(format!("declaration/doctype cannot appear inside element <{name}>")),
            }
        }
    }

    /// 📄 Parses one whole OPC XML part (decl + a single root element) into its root [`XNode`].
    fn parse_document(bytes: &[u8]) -> Result<XNode, String> {
        let text = std::str::from_utf8(bytes).map_err(|e| e.to_string())?;
        let mut reader = Reader::from_str(text);
        loop {
            let event = reader.read_event().map_err(|e| format!("quick-xml parse error at byte {}: {e}", reader.error_position()))?;
            match event {
                Event::Decl(_) | Event::Comment(_) | Event::PI(_) => {}
                Event::Text(text) if text.as_ref().trim_ascii().is_empty() => {}
                Event::Start(start) => return parse_element(&mut reader, start),
                Event::Empty(start) => return Ok(XNode::Element { name: start.name().as_ref().to_string(), attrs: read_attrs(&start)?, children: Vec::new() }),
                Event::Eof => return Err("document has no root element".to_string()),
                other => return Err(format!("unexpected event before the root element: {other:?}")),
            }
        }
    }
    //#endregion 🔖️Parse

    //#region 🔖️Serialize
    fn write_node<W: Write>(writer: &mut Writer<W>, node: &XNode) -> Result<(), String> {
        match node {
            XNode::Text(text) => writer.write_event(Event::Text(BytesText::new(text))).map_err(|e| e.to_string()),
            XNode::Element { name, attrs, children } => {
                let mut start = BytesStart::new(name.as_str());
                for (key, value) in attrs {
                    start.push_attribute((key.as_str(), value.as_str()));
                }
                if children.is_empty() {
                    return writer.write_event(Event::Empty(start)).map_err(|e| e.to_string());
                }
                writer.write_event(Event::Start(start)).map_err(|e| e.to_string())?;
                for child in children {
                    write_node(writer, child)?;
                }
                writer.write_event(Event::End(BytesEnd::new(name.as_str()))).map_err(|e| e.to_string())
            }
        }
    }

    /// 📄 Serializes one whole OPC XML part: the standard declaration plus `root`.
    fn serialize_document(root: &XNode) -> Result<Vec<u8>, String> {
        let mut writer = Writer::new(Cursor::new(Vec::new()));
        writer.write_event(Event::Decl(quick_xml::events::BytesDecl::new("1.0", Some("UTF-8"), Some("yes")))).map_err(|e| e.to_string())?;
        write_node(&mut writer, root)?;
        Ok(writer.into_inner().into_inner())
    }
    //#endregion 🔖️Serialize

    //#region 🔖️Package
    /// 📦 Every OPC part, read/written by the registered `zip` reference implementation —
    /// independent of `semio_s_artifact_stdio_zip::opc::OpcPackage`, this repository's own codec.
    #[derive(Clone, Debug, Default)]
    struct Package {
        parts: HashMap<String, Vec<u8>>,
        order: Vec<String>,
    }

    impl Package {
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free) — see R9
        fn xml(&self, path: &str) -> Result<XNode, String> {
            parse_document(self.parts.get(path).ok_or_else(|| format!("OPC part missing: {path}"))?)
        }
    }

    fn read_zip(bytes: &[u8]) -> Result<Package, String> {
        let mut archive = zip::ZipArchive::new(Cursor::new(bytes.to_vec())).map_err(|e| format!("independent reader could not parse the PPTX (ZIP): {e}"))?;
        let mut parts = HashMap::with_capacity(archive.len());
        let mut order = Vec::with_capacity(archive.len());
        for index in 0..archive.len() {
            let mut member = archive.by_index(index).map_err(|e| format!("independent reader could not read PPTX ZIP entry {index}: {e}"))?;
            if member.is_dir() {
                continue;
            }
            let name = member.name().to_string();
            let mut data = Vec::new();
            member.read_to_end(&mut data).map_err(|e| format!("independent reader could not decompress {name}: {e}"))?;
            order.push(name.clone());
            parts.insert(name, data);
        }
        Ok(Package { parts, order })
    }

    fn write_zip(pkg: &Package) -> Result<Vec<u8>, String> {
        let mut writer = zip::ZipWriter::new(Cursor::new(Vec::<u8>::new()));
        let options: zip::write::FileOptions<'_, ()> = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);
        let mut names: Vec<&String> = pkg.parts.keys().collect();
        names.sort();
        for name in names {
            writer.start_file(name.clone(), options).map_err(|e| format!("zip start_file {name}: {e}"))?;
            writer.write_all(&pkg.parts[name]).map_err(|e| format!("zip write {name}: {e}"))?;
        }
        let cursor = writer.finish().map_err(|e| format!("zip finish: {e}"))?;
        Ok(cursor.into_inner())
    }
    //#endregion 🔖️Package

    //#region 🔖️Types
    /// 📐 A shape's `a:xfrm` position/size, in EMUs — mirrors
    /// `crate::schema::snapshot::PptxTransform` field-for-field, independent type.
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    struct Transform {
        x: i64,
        y: i64,
        cx: i64,
        cy: i64,
    }

    /// 🖼️ One shape from a slide's `p:spTree` — mirrors `PptxShape`'s 4 variants independently.
    #[derive(Clone, Debug, PartialEq)]
    enum PShape {
        TextBox { text: String, position: Transform },
        Picture { blip_rel_id: String, position: Transform },
        Placeholder { kind: String, text: String, position: Transform },
        Other { node: XNode },
    }

    /// 🎞️ One slide: its shape tree, in document order.
    #[derive(Clone, Debug, Default, PartialEq)]
    struct PSlide {
        shapes: Vec<PShape>,
    }
    //#endregion 🔖️Types

    //#region 🔖️ReadPresentation
    /// 🔎️ `_rels/.rels` → the office-document relationship's target (`ppt/presentation.xml`).
    fn presentation_part_path(pkg: &Package) -> Result<String, String> {
        let root_rels = pkg.xml("_rels/.rels")?;
        let found = root_rels.children_named("Relationship").find(|rel| rel.attr("Type").map(|t| t.ends_with("/officeDocument")).unwrap_or(false)).and_then(|rel| rel.attr("Target")).map(|t| t.to_string());
        found.ok_or_else(|| "_rels/.rels: no officeDocument relationship".to_string())
    }

    fn rels_path_for(part_path: &str) -> String {
        let (dir, file) = match part_path.rfind('/') {
            Some(index) => (&part_path[..index], &part_path[index + 1..]),
            None => ("", part_path),
        };
        if dir.is_empty() {
            format!("_rels/{file}.rels")
        } else {
            format!("{dir}/_rels/{file}.rels")
        }
    }

    fn resolve_relative(base_dir: &str, target: &str) -> String {
        let mut segments: Vec<&str> = base_dir.split('/').filter(|s| !s.is_empty()).collect();
        for part in target.split('/') {
            match part {
                "" | "." => {}
                ".." => {
                    segments.pop();
                }
                other => segments.push(other),
            }
        }
        segments.join("/")
    }

    fn position_from_shape(node: &XNode) -> Transform {
        let Some(sp_pr) = node.child("p:spPr") else { return Transform::default() };
        let Some(xfrm) = sp_pr.child("a:xfrm") else { return Transform::default() };
        let mut t = Transform::default();
        if let Some(off) = xfrm.child("a:off") {
            t.x = off.attr("x").and_then(|v| v.parse().ok()).unwrap_or(0);
            t.y = off.attr("y").and_then(|v| v.parse().ok()).unwrap_or(0);
        }
        if let Some(ext) = xfrm.child("a:ext") {
            t.cx = ext.attr("cx").and_then(|v| v.parse().ok()).unwrap_or(0);
            t.cy = ext.attr("cy").and_then(|v| v.parse().ok()).unwrap_or(0);
        }
        t
    }

    /// 🔎️ Every `a:p` inside `p:txBody`, run text concatenated per paragraph, paragraphs joined
    /// by `\n` — a reasonably-scoped simplification of the full `Vec<PptxParagraph>` model (this
    /// oracle checks shape TEXT CONTENT and boundaries, not per-run bold/italic/font-size styling).
    fn text_from_shape(node: &XNode) -> String {
        let Some(tx_body) = node.child("p:txBody") else { return String::new() };
        tx_body.children_named("a:p").map(|p| p.children_named("a:r").map(|r| r.child("a:t").map(|t| t.text()).unwrap_or_default()).collect::<String>()).collect::<Vec<_>>().join("\n")
    }

    fn shape_from_node(node: &XNode) -> PShape {
        let XNode::Element { name, .. } = node else { return PShape::Other { node: node.clone() } };
        match name.as_str() {
            "p:sp" => {
                let ph_type = node.child("p:nvSpPr").and_then(|nv| nv.child("p:nvPr")).and_then(|nv_pr| nv_pr.child("p:ph")).map(|ph| ph.attr("type").unwrap_or("body").to_string());
                let position = position_from_shape(node);
                let text = text_from_shape(node);
                match ph_type {
                    Some(kind) => PShape::Placeholder { kind, text, position },
                    None => PShape::TextBox { text, position },
                }
            }
            "p:pic" => {
                let blip_rel_id = node.child("p:blipFill").and_then(|fill| fill.child("a:blip")).and_then(|blip| blip.attr("r:embed")).unwrap_or("").to_string();
                PShape::Picture { blip_rel_id, position: position_from_shape(node) }
            }
            _ => PShape::Other { node: node.clone() },
        }
    }

    /// 🔎️ Parses one `ppt/slides/slideN.xml` part into its ordered shape list — `p:spTree`'s
    /// direct children, skipping the group's own `p:nvGrpSpPr`/`p:grpSpPr` container elements.
    fn read_slide(pkg: &Package, path: &str) -> Result<PSlide, String> {
        let doc = pkg.xml(path)?;
        let c_sld = doc.child("p:cSld").ok_or_else(|| format!("{path}: missing p:cSld"))?;
        let sp_tree = c_sld.child("p:spTree").ok_or_else(|| format!("{path}: missing p:spTree"))?;
        let XNode::Element { children, .. } = sp_tree else { return Err(format!("{path}: p:spTree is not an element")) };
        let shapes = children.iter().filter(|c| !matches!(c, XNode::Element { name, .. } if name == "p:nvGrpSpPr" || name == "p:grpSpPr")).map(shape_from_node).collect();
        Ok(PSlide { shapes })
    }

    /// 🔎️ The full ordered slide list: `_rels/.rels` → `presentation.xml` → `p:sldIdLst`'s
    /// ordered `r:id`s → `presentation.xml.rels` → each `ppt/slides/slideN.xml` in presentation
    /// order — every hop resolved independently of `crate`'s own importer.
    fn read_presentation(pkg: &Package) -> Result<Vec<PSlide>, String> {
        let pres_path = presentation_part_path(pkg)?;
        let pres_doc = pkg.xml(&pres_path)?;
        let sld_id_lst = pres_doc.child("p:sldIdLst");
        let ordered_rids: Vec<String> = sld_id_lst.map(|lst| lst.children_named("p:sldId").filter_map(|sld_id| sld_id.attr("r:id").map(|s| s.to_string())).collect()).unwrap_or_default();

        let pres_rels_path = rels_path_for(&pres_path);
        let pres_rels = pkg.xml(&pres_rels_path)?;
        let base_dir = pres_path.rfind('/').map(|i| &pres_path[..i]).unwrap_or("");
        let mut rid_to_path: HashMap<String, String> = HashMap::new();
        for rel in pres_rels.children_named("Relationship") {
            if let (Some(id), Some(target)) = (rel.attr("Id"), rel.attr("Target")) {
                rid_to_path.insert(id.to_string(), resolve_relative(base_dir, target));
            }
        }

        ordered_rids.iter().map(|rid| rid_to_path.get(rid).ok_or_else(|| format!("presentation.xml.rels: no relationship {rid}")).and_then(|path| read_slide(pkg, path))).collect()
    }
    //#endregion 🔖️ReadPresentation

    //#region 🔖️WritePresentation
    const NS_A: &str = "http://schemas.openxmlformats.org/drawingml/2006/main";
    const NS_R: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
    const NS_P: &str = "http://schemas.openxmlformats.org/presentationml/2006/main";
    const SLIDE_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.presentationml.slide+xml";
    const REAL_IMAGE_TARGET: &str = "../media/image3.png";
    const REAL_LAYOUT_TARGET: &str = "../slideLayouts/slideLayout1.xml";

    fn xfrm_node(t: Transform) -> XNode {
        XNode::Element {
            name: "a:xfrm".into(),
            attrs: Vec::new(),
            children: vec![
                XNode::Element { name: "a:off".into(), attrs: vec![("x".into(), t.x.to_string()), ("y".into(), t.y.to_string())], children: Vec::new() },
                XNode::Element { name: "a:ext".into(), attrs: vec![("cx".into(), t.cx.to_string()), ("cy".into(), t.cy.to_string())], children: Vec::new() },
            ],
        }
    }

    fn tx_body_node(text: &str) -> XNode {
        XNode::Element {
            name: "p:txBody".into(),
            attrs: Vec::new(),
            children: vec![
                XNode::Element { name: "a:bodyPr".into(), attrs: Vec::new(), children: Vec::new() },
                XNode::Element { name: "a:lstStyle".into(), attrs: Vec::new(), children: Vec::new() },
                XNode::Element {
                    name: "a:p".into(),
                    attrs: Vec::new(),
                    children: vec![XNode::Element { name: "a:r".into(), attrs: Vec::new(), children: vec![XNode::Element { name: "a:t".into(), attrs: Vec::new(), children: vec![XNode::Text(text.to_string())] }] }],
                },
            ],
        }
    }

    /// 🖋️ Renders one typed shape as a fresh, real, valid `p:sp`/`p:pic` element — `shape_id` is
    /// this slide's own sequential `p:cNvPr@id` (unique per slide, not tracked by the typed model).
    fn render_shape(shape: &PShape, shape_id: u32) -> XNode {
        match shape {
            PShape::TextBox { text, position } => XNode::Element {
                name: "p:sp".into(),
                attrs: Vec::new(),
                children: vec![
                    XNode::Element {
                        name: "p:nvSpPr".into(),
                        attrs: Vec::new(),
                        children: vec![
                            XNode::Element { name: "p:cNvPr".into(), attrs: vec![("id".into(), shape_id.to_string()), ("name".into(), format!("TextBox {shape_id}"))], children: Vec::new() },
                            XNode::Element { name: "p:cNvSpPr".into(), attrs: vec![("txBox".into(), "1".into())], children: Vec::new() },
                            XNode::Element { name: "p:nvPr".into(), attrs: Vec::new(), children: Vec::new() },
                        ],
                    },
                    XNode::Element { name: "p:spPr".into(), attrs: Vec::new(), children: vec![xfrm_node(*position)] },
                    tx_body_node(text),
                ],
            },
            PShape::Placeholder { kind, text, position } => XNode::Element {
                name: "p:sp".into(),
                attrs: Vec::new(),
                children: vec![
                    XNode::Element {
                        name: "p:nvSpPr".into(),
                        attrs: Vec::new(),
                        children: vec![
                            XNode::Element { name: "p:cNvPr".into(), attrs: vec![("id".into(), shape_id.to_string()), ("name".into(), format!("Placeholder {shape_id}"))], children: Vec::new() },
                            XNode::Element { name: "p:cNvSpPr".into(), attrs: Vec::new(), children: Vec::new() },
                            XNode::Element { name: "p:nvPr".into(), attrs: Vec::new(), children: vec![XNode::Element { name: "p:ph".into(), attrs: vec![("type".into(), kind.clone())], children: Vec::new() }] },
                        ],
                    },
                    XNode::Element { name: "p:spPr".into(), attrs: Vec::new(), children: vec![xfrm_node(*position)] },
                    tx_body_node(text),
                ],
            },
            PShape::Picture { blip_rel_id, position } => XNode::Element {
                name: "p:pic".into(),
                attrs: Vec::new(),
                children: vec![
                    XNode::Element {
                        name: "p:nvPicPr".into(),
                        attrs: Vec::new(),
                        children: vec![
                            XNode::Element { name: "p:cNvPr".into(), attrs: vec![("id".into(), shape_id.to_string()), ("name".into(), format!("Picture {shape_id}"))], children: Vec::new() },
                            XNode::Element { name: "p:cNvPicPr".into(), attrs: Vec::new(), children: Vec::new() },
                            XNode::Element { name: "p:nvPr".into(), attrs: Vec::new(), children: Vec::new() },
                        ],
                    },
                    XNode::Element {
                        name: "p:blipFill".into(),
                        attrs: Vec::new(),
                        children: vec![
                            XNode::Element { name: "a:blip".into(), attrs: vec![("r:embed".into(), blip_rel_id.clone())], children: Vec::new() },
                            XNode::Element { name: "a:stretch".into(), attrs: Vec::new(), children: vec![XNode::Element { name: "a:fillRect".into(), attrs: Vec::new(), children: Vec::new() }] },
                        ],
                    },
                    XNode::Element { name: "p:spPr".into(), attrs: Vec::new(), children: vec![xfrm_node(*position)] },
                ],
            },
            PShape::Other { node } => node.clone(),
        }
    }

    fn render_slide_document(slide: &PSlide) -> XNode {
        let mut sp_tree_children = vec![
            XNode::Element {
                name: "p:nvGrpSpPr".into(),
                attrs: Vec::new(),
                children: vec![
                    XNode::Element { name: "p:cNvPr".into(), attrs: vec![("id".into(), "1".into()), ("name".into(), String::new())], children: Vec::new() },
                    XNode::Element { name: "p:cNvGrpSpPr".into(), attrs: Vec::new(), children: Vec::new() },
                    XNode::Element { name: "p:nvPr".into(), attrs: Vec::new(), children: Vec::new() },
                ],
            },
            XNode::Element { name: "p:grpSpPr".into(), attrs: Vec::new(), children: vec![xfrm_node(Transform::default())] },
        ];
        for (index, shape) in slide.shapes.iter().enumerate() {
            sp_tree_children.push(render_shape(shape, (index + 2) as u32));
        }
        XNode::Element {
            name: "p:sld".into(),
            attrs: vec![("xmlns:a".into(), NS_A.into()), ("xmlns:r".into(), NS_R.into()), ("xmlns:p".into(), NS_P.into())],
            children: vec![XNode::Element { name: "p:cSld".into(), attrs: Vec::new(), children: vec![XNode::Element { name: "p:spTree".into(), attrs: Vec::new(), children: sp_tree_children }] }],
        }
    }

    /// 🔨 Every `blip_rel_id` referenced by a `Picture` shape in `🎞️slide`, mapped to the one real
    /// embedded image this fixture's closed relationship graph carries (`ppt/media/image3.png`) —
    /// keeps a rebuilt slide's own `.rels` genuinely resolvable rather than dangling.
    fn slide_rels_document(slide: &PSlide) -> XNode {
        let mut rels = vec![XNode::Element { name: "Relationship".into(), attrs: vec![("Id".into(), "rId1".into()), ("Type".into(), format!("{NS_R}/slideLayout")), ("Target".into(), REAL_LAYOUT_TARGET.into())], children: Vec::new() }];
        let mut seen = std::collections::BTreeSet::new();
        for shape in &slide.shapes {
            if let PShape::Picture { blip_rel_id, .. } = shape {
                if !blip_rel_id.is_empty() && seen.insert(blip_rel_id.clone()) {
                    rels.push(XNode::Element { name: "Relationship".into(), attrs: vec![("Id".into(), blip_rel_id.clone()), ("Type".into(), format!("{NS_R}/image")), ("Target".into(), REAL_IMAGE_TARGET.into())], children: Vec::new() });
                }
            }
        }
        XNode::Element { name: "Relationships".into(), attrs: vec![("xmlns".into(), "http://schemas.openxmlformats.org/package/2006/relationships".into())], children: rels }
    }

    /// 🔨 Rebuilds every slide-related OPC part (`ppt/slides/*`, `ppt/slides/_rels/*`,
    /// `ppt/presentation.xml`'s `p:sldIdLst`, `ppt/_rels/presentation.xml.rels`'s slide
    /// relationships, `[Content_Types].xml`'s slide `Override`s) from `slides` — every other part
    /// of `original` (layouts, master, themes, media, docProps, root rels, non-slide
    /// `presentation.xml`/`.rels`/`[Content_Types].xml` entries) is carried forward byte-exact.
    /// Slide numbers/rIds/sldIds are freshly minted every call (never reused across calls), so
    /// there is never a collision with the small static pool of non-slide rIds this fixture's
    /// closed graph already uses (`rId1`, `rId64..rId68`).
    fn write_presentation(original: &Package, slides: &[PSlide]) -> Result<Package, String> {
        let mut pkg = original.clone();
        let pres_path = presentation_part_path(&pkg)?;
        let pres_rels_path = rels_path_for(&pres_path);

        pkg.parts.retain(|path, _| !(path.starts_with("ppt/slides/") || path == "[Content_Types].xml" || *path == pres_path || *path == pres_rels_path));

        let mut sld_id_children = Vec::new();
        let mut pres_rel_children = original.xml(&pres_rels_path)?.children_named("Relationship").filter(|rel| rel.attr("Target").map(|t| !t.starts_with("slides/")).unwrap_or(true)).cloned().collect::<Vec<_>>();
        let mut content_type_overrides = original.xml("[Content_Types].xml")?.children_named("Override").filter(|o| o.attr("PartName").map(|p| !p.starts_with("/ppt/slides/")).unwrap_or(true)).cloned().collect::<Vec<_>>();
        let content_type_defaults: Vec<XNode> = original.xml("[Content_Types].xml")?.children_named("Default").cloned().collect();

        for (index, slide) in slides.iter().enumerate() {
            let file = format!("ppt/slides/slide{}.xml", index + 1);
            let rid = format!("rId{}", 9001 + index);
            let sld_id = 900001 + index;

            pkg.parts.insert(file.clone(), serialize_document(&render_slide_document(slide))?);
            pkg.parts.insert(rels_path_for(&file), serialize_document(&slide_rels_document(slide))?);

            sld_id_children.push(XNode::Element { name: "p:sldId".into(), attrs: vec![("id".into(), sld_id.to_string()), ("r:id".into(), rid.clone())], children: Vec::new() });
            pres_rel_children.push(XNode::Element { name: "Relationship".into(), attrs: vec![("Id".into(), rid), ("Type".into(), format!("{NS_R}/slide")), ("Target".into(), format!("slides/slide{}.xml", index + 1))], children: Vec::new() });
            content_type_overrides.push(XNode::Element { name: "Override".into(), attrs: vec![("PartName".into(), format!("/{file}")), ("ContentType".into(), SLIDE_CONTENT_TYPE.into())], children: Vec::new() });
        }

        let mut pres_doc = original.xml(&pres_path)?;
        let XNode::Element { children, .. } = &mut pres_doc else { return Err(format!("{pres_path}: root is not an element")) };
        match children.iter_mut().find(|c| matches!(c, XNode::Element { name, .. } if name == "p:sldIdLst")) {
            Some(XNode::Element { children: lst, .. }) => *lst = sld_id_children,
            _ => children.insert(0, XNode::Element { name: "p:sldIdLst".into(), attrs: Vec::new(), children: sld_id_children }),
        }
        pkg.parts.insert(pres_path, serialize_document(&pres_doc)?);

        let pres_rels_doc = XNode::Element { name: "Relationships".into(), attrs: vec![("xmlns".into(), "http://schemas.openxmlformats.org/package/2006/relationships".into())], children: pres_rel_children };
        pkg.parts.insert(pres_rels_path, serialize_document(&pres_rels_doc)?);

        let mut ct_children = content_type_defaults;
        ct_children.extend(content_type_overrides);
        let ct_doc = XNode::Element { name: "Types".into(), attrs: vec![("xmlns".into(), "http://schemas.openxmlformats.org/package/2006/content-types".into())], children: ct_children };
        pkg.parts.insert("[Content_Types].xml".into(), serialize_document(&ct_doc)?);

        Ok(pkg)
    }
    //#endregion 🔖️WritePresentation

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
    fn slide_to_json(slide: &PSlide) -> Json {
        Json::Object(vec![("shapeCount".into(), Json::Number(slide.shapes.len() as f64)), ("shapes".into(), Json::Array(slide.shapes.iter().map(shape_to_json).collect()))])
    }
    //#endregion 🔖️JsonValue

    //#region 🔖️Forward
    /// 🌳 One wire `XmlNode` as this model's node: elements and text (CDATA joins the text), anything else refused.
    fn wire_node(value: &Json) -> Result<XNode, String> {
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
    fn located(address: &Json) -> Result<(String, Vec<usize>), String> {
        let path = address.array("nodePath").iter().map(|index| match index {
            Json::Number(value) if *value >= 0.0 && value.fract() == 0.0 => Ok(*value as usize),
            other => Err(format!("node path step {} is no index", other.to_string())),
        });
        Ok((address.str("partPath"), path.collect::<Result<_, String>>()?))
    }

    fn node_at<'a>(node: &'a mut XNode, path: &[usize]) -> Result<&'a mut XNode, String> {
        path.iter().try_fold(node, |node, index| match node {
            XNode::Element { children, .. } => children.get_mut(*index).ok_or_else(|| format!("node path step {index} is outside the tree")),
            XNode::Text(_) => Err("a node path traverses text".to_string()),
        })
    }

    fn children_at<'a>(root: &'a mut XNode, path: &[usize]) -> Result<&'a mut Vec<XNode>, String> {
        match node_at(root, path)? {
            XNode::Element { children, .. } => Ok(children),
            XNode::Text(_) => Err("an addressed container is text".to_string()),
        }
    }

    /// ✏️ Parses one part, applies `edit` to its root and writes the part back re-serialized.
    fn edit_part(pkg: &mut Package, part: &str, edit: impl FnOnce(&mut XNode) -> Result<(), String>) -> Result<(), String> {
        let mut root = pkg.xml(part)?;
        edit(&mut root)?;
        pkg.parts.insert(part.to_string(), serialize_document(&root)?);
        Ok(())
    }

    const CONTENT_TYPES_PART: &str = "[Content_Types].xml";
    const RELATIONSHIPS_NS: &str = "http://schemas.openxmlformats.org/package/2006/relationships";

    /// 📍️ The optional insertion `index` of a wire payload (`None` appends).
    fn optional_index(value: &Json) -> Option<usize> {
        match value.get("index") {
            Some(Json::Number(number)) if *number >= 0.0 => Some(*number as usize),
            _ => None,
        }
    }

    /// 📍️ The physical child position a new `tag` row takes: before the `index`-th existing row of its tag, else after the last of its tag — a content-types
    /// default after the last default, an override and a relationship at the end.
    fn insertion_slot(children: &[XNode], tag: &str, index: Option<usize>) -> usize {
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

    fn is_named(node: &XNode, wanted: &str) -> bool {
        matches!(node, XNode::Element { name, .. } if name == wanted)
    }

    /// 🔢️ The physical child indices of a vacancy's sibling collection: `p:sldId` entries, or the shape tree's shapes
    /// (every PresentationML element but the group's own `p:nvGrpSpPr`/`p:grpSpPr`).
    fn slots(children: &[XNode], slide_list: bool) -> Vec<usize> {
        children.iter().enumerate().filter(|(_, child)| if slide_list { is_named(child, "p:sldId") } else { matches!(child, XNode::Element { name, .. } if name.starts_with("p:") && name != "p:nvGrpSpPr" && name != "p:grpSpPr") }).map(|(index, _)| index).collect()
    }

    fn insert_at_vacancy(pkg: &mut Package, vacancy: &Json, node: XNode, slide_list: bool) -> Result<(), String> {
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

    fn remove_addressed(pkg: &mut Package, address: &Json, expected: &[&str]) -> Result<(), String> {
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
    fn write_text(node: &mut XNode, text: &str, first: &mut bool) {
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

    fn first_named<'a>(node: &'a mut XNode, wanted: &str) -> Option<&'a mut XNode> {
        if is_named(node, wanted) {
            return Some(node);
        }
        let XNode::Element { children, .. } = node else { return None };
        children.iter_mut().find_map(|child| first_named(child, wanted))
    }

    fn set_attr(node: &mut XNode, key: &str, value: String) {
        if let XNode::Element { attrs, .. } = node {
            match attrs.iter_mut().find(|(name, _)| name == key) {
                Some(attr) => attr.1 = value,
                None => attrs.push((key.to_string(), value)),
            }
        }
    }

    /// 🦠️ Applies one declared kind to the package. An unrecognised kind, or a target the package does not hold, is an
    /// error — never a silent no-op.
    fn apply(mut pkg: Package, kind: &str, params: &Json) -> Result<Package, String> {
        let address = || params.get("address").ok_or_else(|| format!("{kind} carries no address"));
        match kind {
            "insert-slide" => insert_at_vacancy(&mut pkg, params.get("vacancy").ok_or("insert-slide carries no vacancy")?, wire_node(params.get("entry").ok_or("insert-slide carries no entry")?)?, true)?,
            "insert-shape" => insert_at_vacancy(&mut pkg, params.get("vacancy").ok_or("insert-shape carries no vacancy")?, wire_node(params.get("shape").ok_or("insert-shape carries no shape")?)?, false)?,
            "remove-slide" => remove_addressed(&mut pkg, address()?.get("entry").ok_or("a slide address carries no entry")?, &["p:sldId"])?,
            "remove-shape" => remove_addressed(&mut pkg, address()?.get("node").ok_or("a shape address carries no node")?, &["p:sp", "p:pic", "p:graphicFrame", "p:grpSp", "p:cxnSp"])?,
            "move-slide" => {
                let (part, path) = located(address()?.get("entry").ok_or("a slide address carries no entry")?)?;
                let (index, parent) = path.split_last().ok_or("a slide entry has no parent")?;
                let destination = match params.get("destinationIndex") { Some(Json::Number(value)) => *value as usize, _ => return Err("move-slide carries no destination".to_string()) };
                edit_part(&mut pkg, &part, |root| {
                    let children = children_at(root, parent)?;
                    let slots = slots(children, true);
                    let from = slots.iter().position(|slot| slot == index).ok_or("the addressed slide entry is not in the slide list")?;
                    if destination >= slots.len() {
                        return Err(format!("move-slide destination {destination} is outside the {}-slide list", slots.len()));
                    }
                    let mut entries = slots.iter().map(|slot| children[*slot].clone()).collect::<Vec<_>>();
                    let entry = entries.remove(from);
                    entries.insert(destination, entry);
                    for (slot, entry) in slots.into_iter().zip(entries) {
                        children[slot] = entry;
                    }
                    Ok(())
                })?;
            }
            "set-shape-text" => {
                let (part, path) = located(address()?.get("node").ok_or("a shape address carries no node")?)?;
                let text = params.str("text");
                edit_part(&mut pkg, &part, |root| {
                    let mut first = true;
                    write_text(node_at(root, &path)?, &text, &mut first);
                    if first { Err("the addressed shape has no DrawingML text node".to_string()) } else { Ok(()) }
                })?;
            }
            "set-shape-position" => {
                let (part, path) = located(address()?.get("node").ok_or("a shape address carries no node")?)?;
                let position = params.get("position").ok_or("set-shape-position carries no position")?;
                edit_part(&mut pkg, &part, |root| {
                    let xfrm = first_named(node_at(root, &path)?, "a:xfrm").ok_or("the addressed shape has no transform")?;
                    let XNode::Element { children, .. } = xfrm else { return Err("a transform is text".to_string()) };
                    let off = children.iter_mut().find(|child| is_named(child, "a:off")).ok_or("the transform has no offset")?;
                    set_attr(off, "x", position.str("x"));
                    set_attr(off, "y", position.str("y"));
                    let ext = children.iter_mut().find(|child| is_named(child, "a:ext")).ok_or("the transform has no extent")?;
                    set_attr(ext, "cx", position.str("cx"));
                    set_attr(ext, "cy", position.str("cy"));
                    Ok(())
                })?;
            }
            "replace-xml-node" => {
                let (part, path) = located(address()?)?;
                let node = wire_node(params.get("node").ok_or("replace-xml-node carries no node")?)?;
                edit_part(&mut pkg, &part, |root| {
                    *node_at(root, &path)? = node;
                    Ok(())
                })?;
            }
            "set-relationship" => {
                let rels_path = rels_path_for(&params.str("owner"));
                let id = params.str("id");
                let mut attrs = vec![("Id".to_string(), id.clone()), ("Type".to_string(), params.str("relType")), ("Target".to_string(), params.str("target"))];
                if matches!(params.get("external"), Some(Json::Bool(true))) {
                    attrs.push(("TargetMode".to_string(), "External".to_string()));
                }
                let row = XNode::Element { name: "Relationship".to_string(), attrs, children: Vec::new() };
                if pkg.parts.contains_key(&rels_path) {
                    edit_part(&mut pkg, &rels_path, |root| {
                        let children = children_at(root, &[])?;
                        match children.iter().position(|child| is_named(child, "Relationship") && child.attr("Id") == Some(id.as_str())) {
                            Some(at) => children[at] = row,
                            None => {
                                let physical = insertion_slot(children, "Relationship", optional_index(params));
                                children.insert(physical, row);
                            }
                        }
                        Ok(())
                    })?;
                } else {
                    let root = XNode::Element { name: "Relationships".to_string(), attrs: vec![("xmlns".to_string(), RELATIONSHIPS_NS.to_string())], children: vec![row] };
                    pkg.parts.insert(rels_path.clone(), serialize_document(&root)?);
                    pkg.order.push(rels_path);
                }
            }
            "remove-relationship" => {
                let owner = params.str("owner");
                let rels_path = rels_path_for(&owner);
                let id = params.str("id");
                if !pkg.parts.contains_key(&rels_path) {
                    return Err(format!("remove-relationship: no relationships are owned by {owner:?}"));
                }
                let mut emptied = false;
                edit_part(&mut pkg, &rels_path, |root| {
                    let children = children_at(root, &[])?;
                    let at = children.iter().position(|child| is_named(child, "Relationship") && child.attr("Id") == Some(id.as_str())).ok_or_else(|| format!("remove-relationship: {owner:?} owns no relationship {id:?}"))?;
                    children.remove(at);
                    emptied = !children.iter().any(|child| is_named(child, "Relationship"));
                    Ok(())
                })?;
                if emptied {
                    pkg.parts.remove(&rels_path);
                    pkg.order.retain(|name| *name != rels_path);
                }
            }
            "set-content-type" => {
                let is_override = matches!(params.get("isOverride"), Some(Json::Bool(true)));
                let (tag, key) = if is_override { ("Override", "PartName") } else { ("Default", "Extension") };
                let name = params.str("name");
                let content_type = params.str("contentType");
                edit_part(&mut pkg, CONTENT_TYPES_PART, |root| {
                    let children = children_at(root, &[])?;
                    if !is_override && children.iter().any(|child| is_named(child, "Default") && child.attr(key).is_some_and(|existing| existing != name && existing.eq_ignore_ascii_case(&name))) {
                        return Err(format!("set-content-type: default extension {name:?} already exists in another letter case"));
                    }
                    match children.iter().position(|child| is_named(child, tag) && child.attr(key) == Some(name.as_str())) {
                        Some(at) => set_attr(&mut children[at], "ContentType", content_type),
                        None => {
                            let row = XNode::Element { name: tag.to_string(), attrs: vec![(key.to_string(), name), ("ContentType".to_string(), content_type)], children: Vec::new() };
                            let physical = insertion_slot(children, tag, optional_index(params));
                            children.insert(physical, row);
                        }
                    }
                    Ok(())
                })?;
            }
            "remove-content-type" => {
                let is_override = matches!(params.get("isOverride"), Some(Json::Bool(true)));
                let (tag, key) = if is_override { ("Override", "PartName") } else { ("Default", "Extension") };
                let name = params.str("name");
                edit_part(&mut pkg, CONTENT_TYPES_PART, |root| {
                    let children = children_at(root, &[])?;
                    let at = children.iter().position(|child| is_named(child, tag) && child.attr(key) == Some(name.as_str())).ok_or_else(|| format!("remove-content-type: the content types hold no {tag} {name:?}"))?;
                    children.remove(at);
                    Ok(())
                })?;
            }
            other => return Err(format!("mutation kind {other:?} has no oracle implementation")),
        }
        Ok(pkg)
    }
    //#endregion 🔖️Forward

    //#region 🔖️Routing
    pub fn apply_mutation(input: &[u8], kind: &str, params: &Json) -> Result<Vec<u8>, String> {
        write_zip(&apply(read_zip(input)?, kind, params)?)
    }

    /// ↩️ Applies `{kind, params}`, re-reads the forward result, and restores the pre-mutation package — every kind's
    /// own inverse restores the base package — so the caller compares that projection
    /// against the ORIGINAL input's own. A forward result the reference cannot re-read fails here.
    pub fn apply_mutation_inverse(input: &[u8], kind: &str, params: &Json) -> Result<Vec<u8>, String> {
        let forward = apply_mutation(input, kind, params)?;
        read_presentation(&read_zip(&forward)?)?;
        match plumbing_inverse(&read_zip(input)?, kind, params)? {
            Some((undo_kind, undo_params)) => apply_mutation(&forward, &undo_kind, &undo_params),
            None => write_zip(&read_zip(input)?),
        }
    }

    /// ↩️ The wire spec that undoes a plumbing kind, read from the pre-mutation package: the previous row at its position, or the removal of the row the forward wrote.
    fn plumbing_inverse(base: &Package, kind: &str, params: &Json) -> Result<Option<(String, Json)>, String> {
        let object = |entries: Vec<(&str, Json)>| Json::Object(entries.into_iter().map(|(key, value)| (key.to_string(), value)).collect());
        let bool_of = |key: &str| matches!(params.get(key), Some(Json::Bool(true)));
        Ok(match kind {
            "set-relationship" | "remove-relationship" => {
                let owner = params.str("owner");
                let id = params.str("id");
                let previous = plumbing_rows(base, &rels_path_for(&owner), "Relationship")?.into_iter().enumerate().find(|(_, row)| row.attr("Id") == Some(id.as_str()));
                match previous {
                    Some((at, row)) => Some((
                        "set-relationship".to_string(),
                        object(vec![
                            ("owner", Json::String(owner)),
                            ("id", Json::String(id)),
                            ("relType", Json::String(row.attr("Type").unwrap_or_default().to_string())),
                            ("target", Json::String(row.attr("Target").unwrap_or_default().to_string())),
                            ("external", Json::Bool(row.attr("TargetMode") == Some("External"))),
                            ("index", Json::Number(at as f64)),
                        ]),
                    )),
                    None if kind == "set-relationship" => Some(("remove-relationship".to_string(), object(vec![("owner", Json::String(owner)), ("id", Json::String(id))]))),
                    None => None,
                }
            }
            "set-content-type" | "remove-content-type" => {
                let is_override = bool_of("isOverride");
                let (tag, key) = if is_override { ("Override", "PartName") } else { ("Default", "Extension") };
                let name = params.str("name");
                let previous = plumbing_rows(base, CONTENT_TYPES_PART, tag)?.into_iter().enumerate().find(|(_, row)| row.attr(key) == Some(name.as_str()));
                match previous {
                    Some((at, row)) => Some((
                        "set-content-type".to_string(),
                        object(vec![("isOverride", Json::Bool(is_override)), ("name", Json::String(name)), ("contentType", Json::String(row.attr("ContentType").unwrap_or_default().to_string())), ("index", Json::Number(at as f64))]),
                    )),
                    None if kind == "set-content-type" => Some(("remove-content-type".to_string(), object(vec![("isOverride", Json::Bool(is_override)), ("name", Json::String(name))]))),
                    None => None,
                }
            }
            _ => None,
        })
    }

    /// 🧱️ The rows named `tag` of an OPC plumbing part, in order; none when the part is absent.
    fn plumbing_rows(pkg: &Package, part: &str, tag: &str) -> Result<Vec<XNode>, String> {
        if !pkg.parts.contains_key(part) {
            return Ok(Vec::new());
        }
        Ok(pkg.xml(part)?.children_named(tag).cloned().collect())
    }

    /// 🔁️ Decodes with the independent reader and re-encodes with the reference writer, no
    /// mutation applied — the identity round trip this subset's `identity-round-trip` scenario
    /// checks (real re-serialization every time, per this module's own header note — never a
    /// pass-through of the original bytes).
    pub fn round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
        let pkg = read_zip(input)?;
        let slides = read_presentation(&pkg)?;
        write_zip(&write_presentation(&pkg, &slides)?)
    }

    /// 👁️ This subset's own semantic projection — ordered slide list, each slide's ordered shape
    /// list (kind/text/position), independently re-derived by re-parsing `bytes` through the
    /// registered `zip` + `quick-xml` reference implementations rather than trusting whatever
    /// produced them.
    pub fn project(bytes: &[u8]) -> Result<Json, String> {
        let pkg = read_zip(bytes)?;
        let slides = read_presentation(&pkg)?;
        Ok(Json::Object(vec![("slideCount".into(), Json::Number(slides.len() as f64)), ("slides".into(), Json::Array(slides.iter().map(slide_to_json).collect()))]))
    }
    //#endregion 🔖️Routing

    //#region 🔖️Plumbing
    /// 🪢️ The package plumbing of `bytes` as the independent reader states it. @see [`plumbing_json`].
    pub fn project_plumbing(bytes: &[u8]) -> Result<Json, String> {
        plumbing_json(&read_zip(bytes)?)
    }

    /// 🪢️ The owner part a `*.rels` part belongs to (`""` for the package root).
    fn owner_of_rels(path: &str) -> Option<String> {
        let file = path.rsplit('/').next()?.strip_suffix(".rels")?;
        let directory = path[..path.len() - path.rsplit('/').next()?.len()].strip_suffix("_rels/")?;
        Some(format!("{directory}{file}"))
    }

    /// 🪢️ The package plumbing in the order the package states it: the `[Content_Types].xml` defaults and overrides as ordered pairs, and every owner's relationships as
    /// ordered `{id, type, target, external}` rows (owners sorted by part name).
    fn plumbing_json(pkg: &Package) -> Result<Json, String> {
        let pairs = |tag: &str, key: &str| -> Result<Json, String> {
            Ok(Json::Array(plumbing_rows(pkg, CONTENT_TYPES_PART, tag)?.iter().map(|row| Json::Array(vec![Json::String(row.attr(key).unwrap_or_default().to_string()), Json::String(row.attr("ContentType").unwrap_or_default().to_string())])).collect()))
        };
        let mut owners: Vec<(String, String)> = pkg.parts.keys().filter_map(|path| owner_of_rels(path).map(|owner| (owner, path.clone()))).collect();
        owners.sort();
        let mut relationships = Vec::with_capacity(owners.len());
        for (owner, path) in owners {
            let rows = plumbing_rows(pkg, &path, "Relationship")?
                .iter()
                .map(|rel| {
                    Json::Object(vec![
                        ("id".to_string(), Json::String(rel.attr("Id").unwrap_or_default().to_string())),
                        ("type".to_string(), Json::String(rel.attr("Type").unwrap_or_default().to_string())),
                        ("target".to_string(), Json::String(rel.attr("Target").unwrap_or_default().to_string())),
                        ("external".to_string(), Json::Bool(rel.attr("TargetMode") == Some("External"))),
                    ])
                })
                .collect();
            relationships.push((owner, Json::Array(rows)));
        }
        Ok(Json::Object(vec![("defaults".to_string(), pairs("Default", "Extension")?), ("overrides".to_string(), pairs("Override", "PartName")?), ("relationships".to_string(), Json::Object(relationships))]))
    }
    //#endregion 🔖️Plumbing
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
    let params = spec.get("params").cloned().unwrap_or(Json::Null);
    oracles::apply_mutation(input, &kind, &params)
}

/// ↩️ Applies one declared mutation kind and then its own computed inverse, in sequence, proving
/// the same `apply(inverse(m, base), apply(m, base)) == base` law `PptxMutation::inverse` proves
/// at the Rust-model level, here against the registered reference implementation instead.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation_inverse(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    let kind = spec.str("kind");
    if kind.is_empty() {
        return Err("mutation spec carries no `kind`".to_string());
    }
    let params = spec.get("params").cloned().unwrap_or(Json::Null);
    oracles::apply_mutation_inverse(input, &kind, &params)
}

/// 🔁️ The identity round trip: decode with the independent reader, re-encode with the reference
/// writer, no mutation applied.
#[cfg(feature = "oracles")]
pub fn oracle_round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
    oracles::round_trip(input)
}

/// 👁️ This subset's own semantic projection. @see [`oracles::project`].
#[cfg(feature = "oracles")]
pub fn project_pptx_mutation(bytes: &[u8]) -> Result<Json, String> {
    oracles::project(bytes)
}

/// 🪢️ The package plumbing (`[Content_Types].xml` entries and every owner's relationships, in the order the package states them). @see [`oracles::project_plumbing`].
#[cfg(feature = "oracles")]
pub fn project_pptx_plumbing(bytes: &[u8]) -> Result<Json, String> {
    oracles::project_plumbing(bytes)
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
pub fn oracle_round_trip(_input: &[u8]) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn project_pptx_plumbing(_bytes: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn project_pptx_mutation(_bytes: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch

//#region 🧪️Tests
#[cfg(all(test, feature = "oracles"))]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
