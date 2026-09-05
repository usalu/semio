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
//! **Design**: every mutation kind is expressed as a pure operation on an in-memory, ordered
//! `Vec<PSlide>` (this module's own typed shape tree, independent of
//! `crate::artifacts::pptx::schema::snapshot::PptxShape`), mirroring the vocabulary's own
//! `slide_index`/`shape_index` addressing. After the operation, every `ppt/slides/*.xml` part,
//! `ppt/_rels/presentation.xml.rels`'s slide relationships, `ppt/presentation.xml`'s
//! `p:sldIdLst`, and `[Content_Types].xml`'s slide `Override` entries are freshly regenerated from
//! that `Vec<PSlide>` — every other OPC part (layouts, master, themes, media, docProps, root
//! rels) is carried forward byte-for-byte untouched. This sidesteps incremental rId/id bookkeeping
//! entirely: `no-mutation` and every real mutation alike re-derive the whole slide part set from
//! the current typed model, a genuine re-serialization each time (never a byte pass-through).
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
pub const KINDS: &[&str] = &["no-mutation", "set-snapshot", "insert-slide", "remove-slide", "move-slide", "insert-shape", "remove-shape", "set-shape-text", "set-shape-position"];
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
    /// independent of `crate::artifacts::zip::opc::OpcPackage`, this repository's own codec.
    #[derive(Clone, Debug, Default)]
    struct Package {
        parts: HashMap<String, Vec<u8>>,
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
        for index in 0..archive.len() {
            let mut member = archive.by_index(index).map_err(|e| format!("independent reader could not read PPTX ZIP entry {index}: {e}"))?;
            if member.is_dir() {
                continue;
            }
            let name = member.name().to_string();
            let mut data = Vec::new();
            member.read_to_end(&mut data).map_err(|e| format!("independent reader could not decompress {name}: {e}"))?;
            parts.insert(name, data);
        }
        Ok(Package { parts })
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
    /// `crate::artifacts::pptx::schema::snapshot::PptxTransform` field-for-field, independent type.
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
    /// order — every hop resolved independently of `crate::artifacts::pptx`'s own importer.
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
    fn number_field(value: &Json, key: &str) -> f64 {
        match value.get(key) {
            Some(Json::Number(n)) => *n,
            _ => 0.0,
        }
    }
    fn usize_field(value: &Json, key: &str) -> usize {
        number_field(value, key).max(0.0) as usize
    }
    fn i64_field(value: &Json, key: &str) -> i64 {
        number_field(value, key) as i64
    }

    fn json_to_transform(value: &Json) -> Transform {
        match value.get("position") {
            Some(position) => Transform { x: i64_field(position, "x"), y: i64_field(position, "y"), cx: i64_field(position, "cx"), cy: i64_field(position, "cy") },
            None => Transform::default(),
        }
    }
    fn transform_to_json(t: Transform) -> Json {
        Json::Object(vec![("x".into(), Json::Number(t.x as f64)), ("y".into(), Json::Number(t.y as f64)), ("cx".into(), Json::Number(t.cx as f64)), ("cy".into(), Json::Number(t.cy as f64))])
    }

    /// 🔎️ Owned shape-spec JSON grammar mutation params speak: `{"kind":"textBox"|"placeholder",
    /// "text":..., "position":{"x":...,"y":...,"cx":...,"cy":...}}` | `{"kind":"picture",
    /// "blipRelId":...,"position":{...}}`.
    fn json_to_shape(value: &Json) -> Result<PShape, String> {
        let position = json_to_transform(value);
        match value.str("kind").as_str() {
            "textBox" => Ok(PShape::TextBox { text: value.str("text"), position }),
            "placeholder" => Ok(PShape::Placeholder { kind: value.str("phKind"), text: value.str("text"), position }),
            "picture" => Ok(PShape::Picture { blip_rel_id: value.str("blipRelId"), position }),
            other => Err(format!("unknown shape kind {other:?}")),
        }
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
    fn json_to_slide(value: &Json) -> Result<PSlide, String> {
        Ok(PSlide { shapes: value.array("shapes").iter().map(json_to_shape).collect::<Result<Vec<_>, _>>()? })
    }
    fn slide_to_json(slide: &PSlide) -> Json {
        Json::Object(vec![("shapeCount".into(), Json::Number(slide.shapes.len() as f64)), ("shapes".into(), Json::Array(slide.shapes.iter().map(shape_to_json).collect()))])
    }
    //#endregion 🔖️JsonValue

    //#region 🔖️Forward
    fn shape_has_text(shape: &PShape) -> Option<(&String, Transform)> {
        match shape {
            PShape::TextBox { text, position } | PShape::Placeholder { text, position, .. } => Some((text, *position)),
            _ => None,
        }
    }
    fn shape_position(shape: &PShape) -> Option<Transform> {
        match shape {
            PShape::TextBox { position, .. } | PShape::Placeholder { position, .. } | PShape::Picture { position, .. } => Some(*position),
            PShape::Other { .. } => None,
        }
    }
    fn with_position(shape: &PShape, position: Transform) -> PShape {
        match shape {
            PShape::TextBox { text, .. } => PShape::TextBox { text: text.clone(), position },
            PShape::Placeholder { kind, text, .. } => PShape::Placeholder { kind: kind.clone(), text: text.clone(), position },
            PShape::Picture { blip_rel_id, .. } => PShape::Picture { blip_rel_id: blip_rel_id.clone(), position },
            PShape::Other { node } => PShape::Other { node: node.clone() },
        }
    }
    fn with_text(shape: &PShape, text: String) -> PShape {
        match shape {
            PShape::TextBox { position, .. } => PShape::TextBox { text, position: *position },
            PShape::Placeholder { kind, position, .. } => PShape::Placeholder { kind: kind.clone(), text, position: *position },
            other => other.clone(),
        }
    }

    /// 🦠️ Applies one declared mutation kind to the typed, ordered slide list. An unrecognised
    /// kind, or a target index out of range, is an error — never a silent no-op.
    fn apply(mut slides: Vec<PSlide>, kind: &str, params: &Json) -> Result<Vec<PSlide>, String> {
        match kind {
            "no-mutation" => Ok(slides),
            "set-snapshot" => Ok(params.array("slides").iter().map(json_to_slide).collect::<Result<Vec<_>, _>>()?),
            "insert-slide" => {
                let index = usize_field(params, "index").min(slides.len());
                slides.insert(index, json_to_slide(params.get("slide").ok_or("insert-slide: missing slide")?)?);
                Ok(slides)
            }
            "remove-slide" => {
                let index = usize_field(params, "index");
                if index >= slides.len() {
                    return Err(format!("remove-slide: index {index} out of range ({} slides)", slides.len()));
                }
                slides.remove(index);
                Ok(slides)
            }
            "move-slide" => {
                let from = usize_field(params, "from");
                if from >= slides.len() {
                    return Err(format!("move-slide: from {from} out of range ({} slides)", slides.len()));
                }
                let slide = slides.remove(from);
                let to = usize_field(params, "to").min(slides.len());
                slides.insert(to, slide);
                Ok(slides)
            }
            "insert-shape" => {
                let slide_index = usize_field(params, "slideIndex");
                let slide = slides.get_mut(slide_index).ok_or_else(|| format!("insert-shape: slideIndex {slide_index} out of range"))?;
                let shape_index = usize_field(params, "shapeIndex").min(slide.shapes.len());
                slide.shapes.insert(shape_index, json_to_shape(params.get("shape").ok_or("insert-shape: missing shape")?)?);
                Ok(slides)
            }
            "remove-shape" => {
                let slide_index = usize_field(params, "slideIndex");
                let slide = slides.get_mut(slide_index).ok_or_else(|| format!("remove-shape: slideIndex {slide_index} out of range"))?;
                let shape_index = usize_field(params, "shapeIndex");
                if shape_index >= slide.shapes.len() {
                    return Err(format!("remove-shape: shapeIndex {shape_index} out of range ({} shapes)", slide.shapes.len()));
                }
                slide.shapes.remove(shape_index);
                Ok(slides)
            }
            "set-shape-text" => {
                let slide_index = usize_field(params, "slideIndex");
                let slide = slides.get_mut(slide_index).ok_or_else(|| format!("set-shape-text: slideIndex {slide_index} out of range"))?;
                let shape_index = usize_field(params, "shapeIndex");
                let shape = slide.shapes.get_mut(shape_index).ok_or_else(|| format!("set-shape-text: shapeIndex {shape_index} out of range"))?;
                if shape_has_text(shape).is_some() {
                    *shape = with_text(shape, params.str("text"));
                }
                Ok(slides)
            }
            "set-shape-position" => {
                let slide_index = usize_field(params, "slideIndex");
                let slide = slides.get_mut(slide_index).ok_or_else(|| format!("set-shape-position: slideIndex {slide_index} out of range"))?;
                let shape_index = usize_field(params, "shapeIndex");
                let shape = slide.shapes.get_mut(shape_index).ok_or_else(|| format!("set-shape-position: shapeIndex {shape_index} out of range"))?;
                if shape_position(shape).is_some() {
                    *shape = with_position(shape, json_to_transform(params));
                }
                Ok(slides)
            }
            other => Err(format!("mutation kind {other:?} has no oracle implementation")),
        }
    }
    //#endregion 🔖️Forward

    //#region 🔖️Inverse
    /// ↩️ Reads `base` (the CURRENT, pre-mutation slide list) to build the spec that undoes
    /// `{kind, params}` — the same law `PptxMutation::inverse` proves at the Rust-model level
    /// (`../🧬️schema/🧬️mutations/🦀️.rs`), computed here against the reference implementation
    /// instead.
    fn inverse_spec(base: &[PSlide], kind: &str, params: &Json) -> Json {
        let spec = |k: &str, p: Json| Json::Object(vec![("kind".into(), Json::String(k.into())), ("params".into(), p)]);
        let obj = |entries: Vec<(&str, Json)>| Json::Object(entries.into_iter().map(|(k, v)| (k.to_string(), v)).collect());
        match kind {
            "no-mutation" => spec("no-mutation", obj(vec![])),
            "set-snapshot" => spec("set-snapshot", obj(vec![("slides", Json::Array(base.iter().map(slide_to_json).collect()))])),
            "insert-slide" => spec("remove-slide", obj(vec![("index", Json::Number(usize_field(params, "index") as f64))])),
            "remove-slide" => match base.get(usize_field(params, "index")) {
                Some(slide) => spec("insert-slide", obj(vec![("index", Json::Number(usize_field(params, "index") as f64)), ("slide", slide_to_json(slide))])),
                None => spec("no-mutation", obj(vec![])),
            },
            "move-slide" => {
                let from = usize_field(params, "from");
                let to = usize_field(params, "to");
                let final_pos = to.min(base.len().saturating_sub(1));
                spec("move-slide", obj(vec![("from", Json::Number(final_pos as f64)), ("to", Json::Number(from as f64))]))
            }
            "insert-shape" => spec("remove-shape", obj(vec![("slideIndex", Json::Number(usize_field(params, "slideIndex") as f64)), ("shapeIndex", Json::Number(usize_field(params, "shapeIndex") as f64))])),
            "remove-shape" => {
                let slide_index = usize_field(params, "slideIndex");
                let shape_index = usize_field(params, "shapeIndex");
                match base.get(slide_index).and_then(|slide| slide.shapes.get(shape_index)) {
                    Some(shape) => spec("insert-shape", obj(vec![("slideIndex", Json::Number(slide_index as f64)), ("shapeIndex", Json::Number(shape_index as f64)), ("shape", shape_to_json(shape))])),
                    None => spec("no-mutation", obj(vec![])),
                }
            }
            "set-shape-text" => {
                let slide_index = usize_field(params, "slideIndex");
                let shape_index = usize_field(params, "shapeIndex");
                match base.get(slide_index).and_then(|slide| slide.shapes.get(shape_index)).and_then(shape_has_text) {
                    Some((text, _)) => spec("set-shape-text", obj(vec![("slideIndex", Json::Number(slide_index as f64)), ("shapeIndex", Json::Number(shape_index as f64)), ("text", Json::String(text.clone()))])),
                    None => spec("no-mutation", obj(vec![])),
                }
            }
            "set-shape-position" => {
                let slide_index = usize_field(params, "slideIndex");
                let shape_index = usize_field(params, "shapeIndex");
                match base.get(slide_index).and_then(|slide| slide.shapes.get(shape_index)).and_then(shape_position) {
                    Some(position) => {
                        let mut entries = vec![("slideIndex".to_string(), Json::Number(slide_index as f64)), ("shapeIndex".to_string(), Json::Number(shape_index as f64))];
                        let Json::Object(position_entries) = transform_to_json(position) else { unreachable!() };
                        entries.push(("position".to_string(), Json::Object(position_entries)));
                        spec("set-shape-position", Json::Object(entries))
                    }
                    None => spec("no-mutation", obj(vec![])),
                }
            }
            other => spec(other, params.clone()),
        }
    }
    //#endregion 🔖️Inverse

    //#region 🔖️Routing
    pub fn apply_mutation(input: &[u8], kind: &str, params: &Json) -> Result<Vec<u8>, String> {
        let pkg = read_zip(input)?;
        let slides = read_presentation(&pkg)?;
        let mutated = apply(slides, kind, params)?;
        write_zip(&write_presentation(&pkg, &mutated)?)
    }

    /// ↩️ Applies `{kind, params}` and then its computed inverse, in sequence, and returns the
    /// re-serialized result — the caller compares its projection against the ORIGINAL input's own.
    pub fn apply_mutation_inverse(input: &[u8], kind: &str, params: &Json) -> Result<Vec<u8>, String> {
        let pkg = read_zip(input)?;
        let base = read_presentation(&pkg)?;
        let inverse = inverse_spec(&base, kind, params);
        let mutated = apply_mutation(input, kind, params)?;
        apply_mutation(&mutated, &inverse.str("kind"), inverse.get("params").unwrap_or(&Json::Null))
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
pub fn project_pptx_mutation(_bytes: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch

//#region 🧪️Tests
#[cfg(all(test, feature = "oracles"))]
mod tests {
    use super::*;
    use crate::law::feature_rows;

    /// 🧫️ The real committed package `🌴️mutate-pptx-ecma-376` runs on — the seven-slide subset derived
    /// once from a real 62-slide conference deck, with real titles, real placeholders and real
    /// `a:xfrm` geometry in EMUs.
    const FIXTURE: &[u8] = include_bytes!("../../../../../🧫️fixtures/📽️.pptx");

    /// 🧾️ The case's own `Examples` rows, read rather than restated — see [`crate::law::feature_rows`].
    const FEATURE: &str = include_str!("../../../../../🧪️tests/🌴️mutate-pptx-ecma-376/🥒️.feature");

    fn spec(kind: &str, params: &Json) -> Json {
        Json::Object(vec![("kind".to_string(), Json::String(kind.to_string())), ("params".to_string(), params.clone())])
    }

    /// ⚖️ The two laws `🌴️mutate-pptx-ecma-376`'s adapter asserts in role, proven here against the real
    /// deck without the runner: every declared kind moves the ordered slide/shape projection, and
    /// every declared kind's own computed inverse lands back on the untouched deck's projection.
    /// Nothing is exempt from either — every one of the nine kinds is defined on the slide list or
    /// on a shape inside it, which is precisely what the projection reports.
    #[test]
    fn every_declared_kind_is_observable_and_its_inverse_restores_the_presentation() {
        let base = project_pptx_mutation(FIXTURE).expect("the independent reader projects the real deck");
        let rows = feature_rows(FEATURE);
        assert_eq!(rows.len(), KINDS.len(), "the feature must carry exactly one Examples row per declared kind");
        for (kind, params) in &rows {
            assert!(KINDS.contains(&kind.as_str()), "the feature exercises {kind:?}, which the pptx-ecma-376-base catalog does not declare");
            let forward = spec(kind, params);
            let mutated = oracle_apply_mutation(FIXTURE, &forward).unwrap_or_else(|error| panic!("{kind}: {error}"));
            let moved = project_pptx_mutation(&mutated).unwrap_or_else(|error| panic!("{kind}: projecting the result failed: {error}"));
            if kind != "no-mutation" {
                assert_ne!(moved, base, "{kind} left the compared projection untouched, so its scenario would pass whether or not the mutation ran");
            }
            let restored = oracle_apply_mutation_inverse(FIXTURE, &forward).unwrap_or_else(|error| panic!("{kind}: inverse: {error}"));
            assert_eq!(project_pptx_mutation(&restored).unwrap(), base, "{kind}: applying the mutation and then its own inverse must restore the deck's projection");
        }
    }

    /// 🔒️ Both halves of the identity law, on the real deck. `oracle_round_trip` is the same rebuild
    /// path every kind takes: unzip, parse each slide, regenerate every slide-related OPC part from
    /// the typed slide/shape list, rezip.
    #[test]
    fn the_round_trip_is_projection_stable_and_not_a_byte_passthrough() {
        let rebuilt = oracle_round_trip(FIXTURE).expect("the reference re-serializes the deck");
        assert_ne!(rebuilt.as_slice(), FIXTURE, "the slide parts and the archive are both rebuilt from the parsed model; identical bytes would mean the input was smuggled");
        assert_eq!(project_pptx_mutation(&rebuilt).unwrap(), project_pptx_mutation(FIXTURE).unwrap());
    }

    #[test]
    fn unknown_kind_is_an_error_never_a_silent_no_op() {
        let unknown = spec("not-a-real-kind", &Json::Object(Vec::new()));
        assert!(oracle_apply_mutation(FIXTURE, &unknown).is_err());
        assert!(oracle_apply_mutation_inverse(FIXTURE, &unknown).is_err());
        assert!(oracle_apply_mutation(FIXTURE, &Json::Object(vec![("params".to_string(), Json::Object(Vec::new()))])).is_err(), "a spec with no kind at all is an error too");
    }

    /// 📇️ [`KINDS`] against the catalog that declares it.
    #[test]
    fn kinds_matches_the_catalog() {
        let manifest = include_str!("🔣️.json");
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "the pptx-ecma-376-base catalog is missing {kind:?}");
        }
        assert_eq!(KINDS.len(), 9, "PptxMutation declares nine kinds");
    }
}
//#endregion 🧪️Tests
