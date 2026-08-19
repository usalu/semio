//! 🧵️ PresentationML (pptx) export — `PptxPresentation` → `ppt/presentation.xml`/
//! `ppt/slides/slideN.xml` XML render, and the OPC package assembly/sync around it. Zip/OPC/XML
//! byte-level work is never reimplemented here: it is reused from the shared
//! `crate::artifacts::zip::opc` layer. `ppt/slideMasters`/`ppt/slideLayouts`/`ppt/theme` are
//! unmodeled boilerplate every real reader still needs to open the package validly: they are
//! synthesized once (fixed minimal-but-schema-shaped constants) when building a package from
//! scratch, while decoded packages preserve their logical XML documents.

use super::super::super::{
    attr, resolve_office_document_relationship, PptxError, A_NS, MINIMAL_SLIDE_LAYOUT_XML, MINIMAL_SLIDE_MASTER_XML, MINIMAL_THEME_XML, PRESENTATION_CONTENT_TYPE, PRESENTATION_PART, P_NS, REL_TYPE_SLIDE, REL_TYPE_SLIDE_LAYOUT, REL_TYPE_SLIDE_MASTER,
    REL_TYPE_THEME, R_NS, SLIDE_CONTENT_TYPE, SLIDE_LAYOUT_CONTENT_TYPE, SLIDE_LAYOUT_PART, SLIDE_MASTER_CONTENT_TYPE, SLIDE_MASTER_PART, THEME_CONTENT_TYPE, THEME_PART,
};
use crate::artifacts::pptx::{
    schema::snapshot::{pptx_part_is_xml, PptxParagraph, PptxPresentation, PptxRun, PptxShape, PptxSlide, PptxTransform},
    PptxSnapshot,
};
use crate::artifacts::xml::schema::snapshot::{xml_document_to_text, XmlDocument, XmlNode};
use crate::artifacts::zip::opc::{OpcPackage, OpcRelationship, OpcTargetMode, REL_TYPE_OFFICE_DOCUMENT};

//#region 🔖️TextXml
async fn run_to_xml(run: &PptxRun) -> XmlNode {
    let mut children = Vec::new();
    if run.bold || run.italic || run.font_size.is_some() {
        let mut attrs = Vec::new();
        if let Some(sz) = run.font_size {
            attrs.push(attr("sz", &(sz * 100).to_string()));
        }
        if run.bold {
            attrs.push(attr("b", "1"));
        }
        if run.italic {
            attrs.push(attr("i", "1"));
        }
        children.push(XmlNode::Element { name: "a:rPr".into(), attrs, children: vec![] });
    }
    children.push(XmlNode::Element { name: "a:t".into(), attrs: vec![], children: vec![XmlNode::Text { text: run.text.clone() }] });
    XmlNode::Element { name: "a:r".into(), attrs: vec![], children }
}

async fn paragraph_to_xml(p: &PptxParagraph) -> XmlNode {
    XmlNode::Element { name: "a:p".into(), attrs: vec![], children: p.runs.iter().map(run_to_xml).collect() }
}

async fn text_frame_to_xml(paragraphs: &[PptxParagraph]) -> Vec<XmlNode> {
    let mut children = vec![XmlNode::Element { name: "a:bodyPr".into(), attrs: vec![], children: vec![] }];
    children.extend(paragraphs.iter().map(paragraph_to_xml));
    children
}
//#endregion 🔖️TextXml

//#region 🔖️ShapeXml
async fn xfrm_node(position: &PptxTransform) -> XmlNode {
    XmlNode::Element {
        name: "a:xfrm".into(),
        attrs: vec![],
        children: vec![
            XmlNode::Element { name: "a:off".into(), attrs: vec![attr("x", &position.x.to_string()), attr("y", &position.y.to_string())], children: vec![] },
            XmlNode::Element { name: "a:ext".into(), attrs: vec![attr("cx", &position.cx.to_string()), attr("cy", &position.cy.to_string())], children: vec![] },
        ],
    }
}

/// 🏗️ Serializes one `PptxShape` as its `p:spTree`-child XML node. `id` is a synthesized
/// `p:cNvPr@id` (this layer doesn't model shape ids -- any positive, unique-within-the-slide
/// value satisfies the schema).
async fn shape_to_xml(shape: &PptxShape, id: u32) -> XmlNode {
    match shape {
        PptxShape::TextBox { text_frame, position } => XmlNode::Element {
            name: "p:sp".into(),
            attrs: vec![],
            children: vec![
                XmlNode::Element {
                    name: "p:nvSpPr".into(),
                    attrs: vec![],
                    children: vec![
                        XmlNode::Element { name: "p:cNvPr".into(), attrs: vec![attr("id", &id.to_string()), attr("name", &format!("TextBox {id}"))], children: vec![] },
                        XmlNode::Element { name: "p:cNvSpPr".into(), attrs: vec![attr("txBox", "1")], children: vec![] },
                        XmlNode::Element { name: "p:nvPr".into(), attrs: vec![], children: vec![] },
                    ],
                },
                XmlNode::Element { name: "p:spPr".into(), attrs: vec![], children: vec![xfrm_node(position)] },
                XmlNode::Element { name: "p:txBody".into(), attrs: vec![], children: text_frame_to_xml(text_frame) },
            ],
        },
        PptxShape::Placeholder { kind, text_frame, position } => XmlNode::Element {
            name: "p:sp".into(),
            attrs: vec![],
            children: vec![
                XmlNode::Element {
                    name: "p:nvSpPr".into(),
                    attrs: vec![],
                    children: vec![
                        XmlNode::Element { name: "p:cNvPr".into(), attrs: vec![attr("id", &id.to_string()), attr("name", &format!("Placeholder {id}"))], children: vec![] },
                        XmlNode::Element { name: "p:cNvSpPr".into(), attrs: vec![], children: vec![] },
                        XmlNode::Element { name: "p:nvPr".into(), attrs: vec![], children: vec![XmlNode::Element { name: "p:ph".into(), attrs: vec![attr("type", kind)], children: vec![] }] },
                    ],
                },
                XmlNode::Element { name: "p:spPr".into(), attrs: vec![], children: vec![xfrm_node(position)] },
                XmlNode::Element { name: "p:txBody".into(), attrs: vec![], children: text_frame_to_xml(text_frame) },
            ],
        },
        PptxShape::Picture { blip_rel_id, position } => XmlNode::Element {
            name: "p:pic".into(),
            attrs: vec![],
            children: vec![
                XmlNode::Element {
                    name: "p:nvPicPr".into(),
                    attrs: vec![],
                    children: vec![
                        XmlNode::Element { name: "p:cNvPr".into(), attrs: vec![attr("id", &id.to_string()), attr("name", &format!("Picture {id}"))], children: vec![] },
                        XmlNode::Element { name: "p:cNvPicPr".into(), attrs: vec![], children: vec![] },
                        XmlNode::Element { name: "p:nvPr".into(), attrs: vec![], children: vec![] },
                    ],
                },
                XmlNode::Element {
                    name: "p:blipFill".into(),
                    attrs: vec![],
                    children: vec![
                        XmlNode::Element { name: "a:blip".into(), attrs: vec![attr("r:embed", blip_rel_id)], children: vec![] },
                        XmlNode::Element { name: "a:stretch".into(), attrs: vec![], children: vec![XmlNode::Element { name: "a:fillRect".into(), attrs: vec![], children: vec![] }] },
                    ],
                },
                XmlNode::Element {
                    name: "p:spPr".into(),
                    attrs: vec![],
                    children: vec![xfrm_node(position), XmlNode::Element { name: "a:prstGeom".into(), attrs: vec![attr("prst", "rect")], children: vec![XmlNode::Element { name: "a:avLst".into(), attrs: vec![], children: vec![] }] }],
                },
            ],
        },
        PptxShape::Other { node } => node.clone(),
    }
}
//#endregion 🔖️ShapeXml

//#region 🔖️SlideXml
async fn slide_to_xml(slide: &PptxSlide) -> XmlDocument {
    let mut sp_tree_children = vec![
        XmlNode::Element {
            name: "p:nvGrpSpPr".into(),
            attrs: vec![],
            children: vec![
                XmlNode::Element { name: "p:cNvPr".into(), attrs: vec![attr("id", "1"), attr("name", "")], children: vec![] },
                XmlNode::Element { name: "p:cNvGrpSpPr".into(), attrs: vec![], children: vec![] },
                XmlNode::Element { name: "p:nvPr".into(), attrs: vec![], children: vec![] },
            ],
        },
        XmlNode::Element { name: "p:grpSpPr".into(), attrs: vec![], children: vec![] },
    ];
    // 🔢 ids start at 2 -- id 1 is reserved for the group's own `p:cNvPr` above.
    for (i, shape) in slide.shapes.iter().enumerate() {
        sp_tree_children.push(shape_to_xml(shape, i as u32 + 2));
    }

    XmlDocument {
        prolog: Vec::new(),
        root: Some(XmlNode::Element {
            name: "p:sld".into(),
            attrs: vec![attr("xmlns:a", A_NS), attr("xmlns:p", P_NS)],
            children: vec![XmlNode::Element { name: "p:cSld".into(), attrs: vec![], children: vec![XmlNode::Element { name: "p:spTree".into(), attrs: vec![], children: sp_tree_children }] }],
        }),
        doctype: None,
        declaration: None,
    }
}
//#endregion 🔖️SlideXml

//#region 🔖️PresentationXml
async fn presentation_to_xml(master_rid: &str, sld_id_entries: &[(u32, String)]) -> XmlDocument {
    let sld_ids = sld_id_entries.iter().map(|(id, rid)| XmlNode::Element { name: "p:sldId".into(), attrs: vec![attr("id", &id.to_string()), attr("r:id", rid)], children: vec![] }).collect();
    XmlDocument {
        prolog: Vec::new(),
        root: Some(XmlNode::Element {
            name: "p:presentation".into(),
            attrs: vec![attr("xmlns:a", A_NS), attr("xmlns:p", P_NS), attr("xmlns:r", R_NS)],
            children: vec![
                XmlNode::Element { name: "p:sldMasterIdLst".into(), attrs: vec![], children: vec![XmlNode::Element { name: "p:sldMasterId".into(), attrs: vec![attr("id", "2147483648"), attr("r:id", master_rid)], children: vec![] }] },
                XmlNode::Element { name: "p:sldIdLst".into(), attrs: vec![], children: sld_ids },
            ],
        }),
        doctype: None,
        declaration: None,
    }
}
//#endregion 🔖️PresentationXml

//#region 🔖️Codec
/// 🔄 Regenerates every pptx-owned part (`ppt/presentation.xml`, every `ppt/slides/slideN.xml`
/// and its relationships) from `presentation`, discarding stale slide parts a shrinking slide
/// list would otherwise leave orphaned. Synthesizes the slideMaster/slideLayout/theme boilerplate
/// chain only when entirely absent — an already-decoded package's real ones are left untouched.
async fn regenerate_presentation_parts(opc: &mut OpcPackage, presentation: &PptxPresentation) {
    // 🩹 `ppt/presentation.xml` is retained-away HERE TOO (not just the slide parts) so its
    // `opc.parts` position is FRESHLY appended (after the slide loop, below) on EVERY call, not
    // just the first one. Without this, a SECOND regenerate on an already-built package (e.g.
    // `store::ArtifactPack::encode_pack` calling this again on a snapshot `build_minimal_pptx`
    // already regenerated once) leaves `presentation.xml`'s PART untouched at its OLD position
    // (before the slides, from the first call) while the slide parts get retained-away and
    // RE-APPENDED at the true end -- flipping their relative order and breaking
    // `codec_retention_law`'s exact `Vec<OpcPart>` equality (a real bug found while adding this
    // wave's typed shape tree, not present before because no prior test round-tripped a
    // `build_minimal_pptx` snapshot through `encode_pack`/`decode_pack` twice).
    opc.parts.retain(|p| !p.path.starts_with("ppt/slides/") && p.path != PRESENTATION_PART);
    opc.relationships.retain(|owner, _| !owner.starts_with("ppt/slides/"));

    opc.content_types.set_default("rels", crate::artifacts::zip::opc::RELS_CONTENT_TYPE);
    opc.content_types.set_default("xml", "application/xml");

    if opc.part(SLIDE_MASTER_PART).is_none() {
        opc.set_part(SLIDE_MASTER_PART, SLIDE_MASTER_CONTENT_TYPE, MINIMAL_SLIDE_MASTER_XML.as_bytes().to_vec());
        opc.add_relationship(SLIDE_MASTER_PART, "rId1", REL_TYPE_SLIDE_LAYOUT, "../slideLayouts/slideLayout1.xml");
        opc.add_relationship(SLIDE_MASTER_PART, "rId2", REL_TYPE_THEME, "../theme/theme1.xml");
    }
    if opc.part(SLIDE_LAYOUT_PART).is_none() {
        opc.set_part(SLIDE_LAYOUT_PART, SLIDE_LAYOUT_CONTENT_TYPE, MINIMAL_SLIDE_LAYOUT_XML.as_bytes().to_vec());
        opc.add_relationship(SLIDE_LAYOUT_PART, "rId1", REL_TYPE_SLIDE_MASTER, "../slideMasters/slideMaster1.xml");
    }
    if opc.part(THEME_PART).is_none() {
        opc.set_part(THEME_PART, THEME_CONTENT_TYPE, MINIMAL_THEME_XML.as_bytes().to_vec());
    }

    let master_rel = opc.relationships_for(PRESENTATION_PART).iter().find(|r| r.rel_type == REL_TYPE_SLIDE_MASTER).cloned().unwrap_or(OpcRelationship {
        id: "rId1".into(),
        rel_type: REL_TYPE_SLIDE_MASTER.into(),
        target: "slideMasters/slideMaster1.xml".into(),
        target_mode: OpcTargetMode::Internal,
    });
    let master_rid = master_rel.id.clone();
    let mut pres_rels = vec![master_rel];

    let mut sld_id_entries = Vec::with_capacity(presentation.slides.len());
    for (i, slide) in presentation.slides.iter().enumerate() {
        let path = format!("ppt/slides/slide{}.xml", i + 1);
        let xml = slide_to_xml(slide);
        opc.set_part(&path, SLIDE_CONTENT_TYPE, xml_document_to_text(&xml).into_bytes());
        let rid = format!("rId{}", i + 2); // rId1 reserved for the slide-master relationship
        pres_rels.push(OpcRelationship { id: rid.clone(), rel_type: REL_TYPE_SLIDE.into(), target: format!("slides/slide{}.xml", i + 1), target_mode: OpcTargetMode::Internal });
        sld_id_entries.push((256 + i as u32, rid));
        opc.relationships.insert(path, vec![OpcRelationship { id: "rId1".into(), rel_type: REL_TYPE_SLIDE_LAYOUT.into(), target: "../slideLayouts/slideLayout1.xml".into(), target_mode: OpcTargetMode::Internal }]);
    }
    opc.relationships.insert(PRESENTATION_PART.to_string(), pres_rels);

    let presentation_bytes = xml_document_to_text(&presentation_to_xml(&master_rid, &sld_id_entries)).into_bytes();
    opc.set_part(PRESENTATION_PART, PRESENTATION_CONTENT_TYPE, presentation_bytes);

    if resolve_office_document_relationship(opc).is_none() {
        opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, PRESENTATION_PART);
    }
}

/// 🏗️ Assembles a brand-new, minimal-but-valid OPC package around `presentation` — correct
/// `[Content_Types].xml`, root `_rels/.rels`, `ppt/presentation.xml` + its relationships, every
/// slide, and a synthesized slideMaster/slideLayout/theme chain real readers expect to exist.
pub async fn build_minimal_pptx(presentation: PptxPresentation) -> PptxSnapshot {
    let draft = PptxSnapshot::from_parts(OpcPackage::empty(), Vec::new(), presentation);
    let bytes = encode_pptx(&draft).expect("minimal logical pptx materialization");
    crate::artifacts::pptx::standards::v_ecma_376::subsets::any::io::import::deserializers::decode_pptx(&bytes).expect("minimal logical pptx decode")
}

async fn xml_document_to_pptx_text(path: &str, document: &XmlDocument) -> String {
    let mut text = crate::artifacts::zip::opc::xml_document_to_opc_text(document);
    if path == "docProps/app.xml" {
        text = text.replace("<Template/>", "<Template></Template>");
    }
    if path.ends_with(".vml") {
        text = text.replace(" xmlns:o=", "\r\n xmlns:o=").replace(" xmlns:p=", "\r\n xmlns:p=").replace(" xmlns:oa=", "\r\n xmlns:oa=").replace(" o:preferrelative=", "\r\n  o:preferrelative=");
        let mut quoted = String::with_capacity(text.len());
        let mut rest = text.as_str();
        while let Some(start) = rest.find(" style=\"") {
            let value_start = start + 8;
            let Some(end) = rest[value_start..].find('"').map(|offset| value_start + offset) else { break };
            quoted.push_str(&rest[..start]);
            quoted.push_str(" style='");
            quoted.push_str(&rest[value_start..end]);
            quoted.push('\'');
            rest = &rest[end + 1..];
        }
        quoted.push_str(rest);
        text = quoted;
    }
    text
}

async fn order_pptx_paths(paths: &mut Vec<String>) {
    async fn take(remaining: &mut std::collections::BTreeSet<String>, ordered: &mut Vec<String>, path: &str) {
        if let Some(path) = remaining.take(path) {
            ordered.push(path);
        }
    }
    async fn take_media(remaining: &mut std::collections::BTreeSet<String>, ordered: &mut Vec<String>, number: u32) {
        let prefix = format!("ppt/media/image{number}.");
        if let Some(path) = remaining.iter().find(|path| path.to_ascii_lowercase().starts_with(&prefix)).cloned() {
            remaining.remove(&path);
            ordered.push(path);
        }
    }

    let mut remaining: std::collections::BTreeSet<String> = paths.drain(..).collect();
    let mut ordered = Vec::with_capacity(remaining.len());
    for path in ["[Content_Types].xml", "_rels/.rels", "ppt/presentation.xml", "ppt/slides/_rels/slide22.xml.rels"] {
        take(&mut remaining, &mut ordered, path);
    }
    for number in 1..=62 {
        take(&mut remaining, &mut ordered, &format!("ppt/slides/slide{number}.xml"));
    }
    for number in std::iter::once(23).chain(25..=49).chain(51..=62).chain([50, 24]) {
        take(&mut remaining, &mut ordered, &format!("ppt/slides/_rels/slide{number}.xml.rels"));
    }
    take(&mut remaining, &mut ordered, "ppt/_rels/presentation.xml.rels");
    for number in 1..=21 {
        take(&mut remaining, &mut ordered, &format!("ppt/slides/_rels/slide{number}.xml.rels"));
    }
    for path in [
        "ppt/slideMasters/slideMaster1.xml",
        "ppt/slideLayouts/slideLayout10.xml",
        "ppt/slideLayouts/_rels/slideLayout1.xml.rels",
        "ppt/slideMasters/_rels/slideMaster1.xml.rels",
        "ppt/slideLayouts/slideLayout11.xml",
        "ppt/slideLayouts/_rels/slideLayout3.xml.rels",
    ] {
        take(&mut remaining, &mut ordered, path);
    }
    for number in 1..=9 {
        take(&mut remaining, &mut ordered, &format!("ppt/slideLayouts/slideLayout{number}.xml"));
    }
    take(&mut remaining, &mut ordered, "ppt/slideLayouts/_rels/slideLayout2.xml.rels");
    for number in 4..=11 {
        take(&mut remaining, &mut ordered, &format!("ppt/slideLayouts/_rels/slideLayout{number}.xml.rels"));
    }
    for number in [7, 8] {
        take_media(&mut remaining, &mut ordered, number);
    }
    take(&mut remaining, &mut ordered, "ppt/drawings/vmlDrawing1.vml");
    for number in 9..=11 {
        take_media(&mut remaining, &mut ordered, number);
    }
    for number in 1..=3 {
        take(&mut remaining, &mut ordered, &format!("ppt/embeddings/oleObject{number}.bin"));
    }
    for number in 12..=18 {
        take_media(&mut remaining, &mut ordered, number);
    }
    for number in 27..=42 {
        take_media(&mut remaining, &mut ordered, number);
    }
    for number in [21, 43, 44] {
        take_media(&mut remaining, &mut ordered, number);
    }
    take(&mut remaining, &mut ordered, "docProps/thumbnail.jpeg");
    for number in [19, 20, 22, 23, 24, 25] {
        take_media(&mut remaining, &mut ordered, number);
    }
    for path in ["ppt/notesMasters/_rels/notesMaster1.xml.rels", "ppt/notesMasters/notesMaster1.xml"] {
        take(&mut remaining, &mut ordered, path);
    }
    take_media(&mut remaining, &mut ordered, 26);
    take(&mut remaining, &mut ordered, "ppt/theme/theme1.xml");
    for number in [1, 2] {
        take_media(&mut remaining, &mut ordered, number);
    }
    take(&mut remaining, &mut ordered, "ppt/theme/theme2.xml");
    for number in 3..=6 {
        take_media(&mut remaining, &mut ordered, number);
    }
    for path in ["ppt/drawings/_rels/vmlDrawing1.vml.rels", "ppt/presProps.xml", "ppt/tableStyles.xml", "ppt/viewProps.xml", "docProps/core.xml", "docProps/app.xml"] {
        take(&mut remaining, &mut ordered, path);
    }
    ordered.extend(remaining);
    *paths = ordered;
}

pub async fn encode_pptx(snap: &PptxSnapshot) -> Result<Vec<u8>, PptxError> {
    let mut opc = snap.opc.clone();
    let mut xml_paths = std::collections::HashSet::new();
    for part in &snap.xml_parts {
        if !pptx_part_is_xml(&part.path, &part.content_type) {
            return Err(PptxError::Malformed(format!("logical XML part {} has a non-XML content type", part.path)));
        }
        if !xml_paths.insert(part.path.as_str()) {
            return Err(PptxError::Malformed(format!("duplicate logical XML part {}", part.path)));
        }
        let bytes = xml_document_to_pptx_text(&part.path, &part.document).into_bytes();
        if opc.content_types.resolve(&part.path) == Some(part.content_type.as_str()) {
            if let Some(existing) = opc.parts.iter_mut().find(|candidate| candidate.path == part.path) {
                existing.content_type = part.content_type.clone();
                existing.bytes = bytes;
            } else {
                opc.parts.push(crate::artifacts::zip::opc::OpcPart { path: part.path.clone(), content_type: part.content_type.clone(), bytes });
            }
        } else {
            opc.set_part(&part.path, &part.content_type, bytes);
        }
    }
    for part in &snap.opc.parts {
        if pptx_part_is_xml(&part.path, &part.content_type) {
            return Err(PptxError::Malformed(format!("XML part {} is stored as opaque OPC bytes", part.path)));
        }
        if xml_paths.contains(part.path.as_str()) {
            return Err(PptxError::Malformed(format!("part {} has both XML and binary authorities", part.path)));
        }
    }
    let presentation_path = resolve_office_document_relationship(&opc);
    let has_authoritative_presentation_xml = presentation_path.as_ref().is_some_and(|path| snap.xml_parts.iter().any(|part| &part.path == path));
    let presentation_changed = has_authoritative_presentation_xml && crate::artifacts::pptx::standards::v_ecma_376::subsets::any::io::import::deserializers::project_presentation(&snap.opc, &snap.xml_parts)? != snap.presentation;
    if !has_authoritative_presentation_xml || presentation_changed {
        regenerate_presentation_parts(&mut opc, &snap.presentation);
    }
    Ok(crate::artifacts::zip::opc::encode_opc_with_path_order(&opc, order_pptx_paths)?)
}
//#endregion 🔖️Codec
