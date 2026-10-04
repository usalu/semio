//! 🧵️ PresentationML (pptx) export — `PptxPresentation` → `ppt/presentation.xml`/
//! `ppt/slides/slideN.xml` XML render, and the OPC package assembly/sync around it. Zip/OPC/XML
//! byte-level work is never reimplemented here: it is reused from the shared
//! `semio_s_artifact_stdio_zip::opc` layer. `ppt/slideMasters`/`ppt/slideLayouts`/`ppt/theme` are
//! unmodeled boilerplate every real reader still needs to open the package validly: they are
//! synthesized once (fixed minimal-but-schema-shaped constants) when building a package from
//! scratch, while decoded packages preserve their logical XML documents.

use super::super::super::{
    attr, resolve_office_document_relationship, PptxError, A_NS, MINIMAL_SLIDE_LAYOUT_XML, MINIMAL_SLIDE_MASTER_XML, MINIMAL_THEME_XML, PRESENTATION_CONTENT_TYPE, PRESENTATION_PART, P_NS, REL_TYPE_SLIDE, REL_TYPE_SLIDE_LAYOUT, REL_TYPE_SLIDE_MASTER,
    REL_TYPE_THEME, R_NS, SLIDE_CONTENT_TYPE, SLIDE_LAYOUT_CONTENT_TYPE, SLIDE_LAYOUT_PART, SLIDE_MASTER_CONTENT_TYPE, SLIDE_MASTER_PART, THEME_CONTENT_TYPE, THEME_PART,
};
use crate::{
    schema::snapshot::{pptx_part_is_xml, PptxParagraph, PptxPresentation, PptxRun, PptxShape, PptxSlide, PptxTransform},
    PptxSnapshot,
};
use semio_s_artifact_stdio_xml::schema::snapshot::{xml_document_to_text, XmlDocument, XmlNode};
use semio_s_artifact_stdio_zip::opc::{OpcPackage, OpcRelationship, OpcTargetMode, REL_TYPE_OFFICE_DOCUMENT};

//#region 🔖️TextXml
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn run_to_xml(run: &PptxRun) -> XmlNode {
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn paragraph_to_xml(p: &PptxParagraph) -> XmlNode {
    XmlNode::Element { name: "a:p".into(), attrs: vec![], children: p.runs.iter().map(run_to_xml).collect() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn text_frame_to_xml(paragraphs: &[PptxParagraph]) -> Vec<XmlNode> {
    let mut children = vec![XmlNode::Element { name: "a:bodyPr".into(), attrs: vec![], children: vec![] }];
    children.extend(paragraphs.iter().map(paragraph_to_xml));
    children
}
//#endregion 🔖️TextXml

//#region 🔖️ShapeXml
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn xfrm_node(position: &PptxTransform) -> XmlNode {
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn shape_to_xml(shape: &PptxShape, id: u32) -> XmlNode {
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn slide_to_xml(slide: &PptxSlide) -> XmlDocument {
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
        epilog: Vec::new(),
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn presentation_to_xml(master_rid: &str, sld_id_entries: &[(u32, String)]) -> XmlDocument {
    let sld_ids = sld_id_entries.iter().map(|(id, rid)| XmlNode::Element { name: "p:sldId".into(), attrs: vec![attr("id", &id.to_string()), attr("r:id", rid)], children: vec![] }).collect();
    XmlDocument {
        prolog: Vec::new(),
        epilog: Vec::new(),
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

//#region 🔖️PresentationRelationships

/// 🔗️ `ppt/presentation.xml`'s relationship list, regenerated for the slide list this snapshot
/// carries while PRESERVING every relationship the package was read with that this codec does not
/// own (`presProps`, `viewProps`, `tableStyles`, `notesMaster`, `theme`, …). Only the `🎞️slide`
/// pointers are rebuilt, each reusing the id and the declared type URI of the pointer it replaces,
/// so the list keeps its original order and a Strict package keeps its `purl.oclc.org/ooxml` types.
/// Returns the slide-master relationship id, the per-slide relationship ids in slide order, and the
/// rebuilt list.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn presentation_relationships(existing: &[OpcRelationship], slide_count: usize) -> (String, Vec<String>, Vec<OpcRelationship>) {
    let is_slide = |rel: &OpcRelationship| rel.rel_type.ends_with("/slide");
    let slide_type = existing.iter().find(|r| is_slide(r)).map_or_else(|| REL_TYPE_SLIDE.to_string(), |r| r.rel_type.clone());
    let mut taken: Vec<String> = existing.iter().map(|r| r.id.clone()).collect();

    let master = existing.iter().find(|r| r.rel_type.ends_with("/slideMaster")).cloned();
    let master_rel = master.clone().unwrap_or_else(|| OpcRelationship {
        id: semio_s_artifact_stdio_zip::opc::fresh_relationship_id(&mut taken),
        rel_type: REL_TYPE_SLIDE_MASTER.into(),
        target: "slideMasters/slideMaster1.xml".into(),
        target_mode: OpcTargetMode::Internal,
    });
    let master_rid = master_rel.id.clone();

    let prior_slide_ids: Vec<String> = existing.iter().filter(|r| is_slide(r)).map(|r| r.id.clone()).collect();
    let slide_rids: Vec<String> = (0..slide_count).map(|i| prior_slide_ids.get(i).cloned().unwrap_or_else(|| semio_s_artifact_stdio_zip::opc::fresh_relationship_id(&mut taken))).collect();
    let mut slides = slide_rids.iter().enumerate().map(|(i, id)| OpcRelationship { id: id.clone(), rel_type: slide_type.clone(), target: format!("slides/slide{}.xml", i + 1), target_mode: OpcTargetMode::Internal });

    let mut rebuilt: Vec<OpcRelationship> = Vec::with_capacity(existing.len().max(slide_count + 1));
    if master.is_none() {
        rebuilt.push(master_rel);
    }
    for relationship in existing {
        if is_slide(relationship) {
            if let Some(next) = slides.next() {
                rebuilt.push(next);
            }
            continue;
        }
        rebuilt.push(relationship.clone());
    }
    rebuilt.extend(slides);
    (master_rid, slide_rids, rebuilt)
}
//#endregion 🔖️PresentationRelationships

//#region 🔖️Codec
/// 🔄 Regenerates every pptx-owned part (`ppt/presentation.xml`, every `ppt/slides/slideN.xml`
/// and its relationships) from `presentation`, discarding stale slide parts a shrinking slide
/// list would otherwise leave orphaned. Synthesizes the slideMaster/slideLayout/theme boilerplate
/// chain only when entirely absent — an already-decoded package's real ones are left untouched.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn regenerate_presentation_parts(opc: &mut OpcPackage, presentation: &PptxPresentation) {
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
    let slide_relationship_owners = opc.relationships.groups().filter(|(owner, _)| owner.starts_with("ppt/slides/")).map(|(owner, _)| owner.clone()).collect::<Vec<_>>();
    for owner in slide_relationship_owners {
        opc.relationships.remove_owner(&owner);
    }

    opc.content_types.set_default("rels", semio_s_artifact_stdio_zip::opc::RELS_CONTENT_TYPE);
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

    let (master_rid, slide_rids, pres_rels) = presentation_relationships(opc.relationships_for(PRESENTATION_PART), presentation.slides.len());

    let mut sld_id_entries = Vec::with_capacity(presentation.slides.len());
    for (i, slide) in presentation.slides.iter().enumerate() {
        let path = format!("ppt/slides/slide{}.xml", i + 1);
        let xml = slide_to_xml(slide);
        opc.set_part(&path, SLIDE_CONTENT_TYPE, xml_document_to_text(&xml).into_bytes());
        sld_id_entries.push((256 + i as u32, slide_rids[i].clone()));
        opc.relationships.replace_owner(path, vec![OpcRelationship { id: "rId1".into(), rel_type: REL_TYPE_SLIDE_LAYOUT.into(), target: "../slideLayouts/slideLayout1.xml".into(), target_mode: OpcTargetMode::Internal }]);
    }
    opc.relationships.replace_owner(PRESENTATION_PART.to_string(), pres_rels);

    let presentation_bytes = xml_document_to_text(&presentation_to_xml(&master_rid, &sld_id_entries)).into_bytes();
    opc.set_part(PRESENTATION_PART, PRESENTATION_CONTENT_TYPE, presentation_bytes);

    if resolve_office_document_relationship(opc).is_none() {
        opc.add_generated_relationship("", REL_TYPE_OFFICE_DOCUMENT, PRESENTATION_PART);
    }
}

/// 🏗️ Assembles a brand-new, minimal-but-valid OPC package around `presentation` — correct
/// `[Content_Types].xml`, root `_rels/.rels`, `ppt/presentation.xml` + its relationships, every
/// slide, and a synthesized slideMaster/slideLayout/theme chain real readers expect to exist.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn build_minimal_pptx(presentation: PptxPresentation) -> PptxSnapshot {
    let mut opc = OpcPackage::empty();
    regenerate_presentation_parts(&mut opc, &presentation);
    let bytes = semio_s_artifact_stdio_zip::opc::encode_opc_with_package_order(&opc).expect("minimal logical pptx materialization");
    crate::standards::v_ecma_376::subsets::base::io::import::deserializers::decode_pptx(&bytes).expect("minimal logical pptx decode")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_pptx(snap: &PptxSnapshot) -> Result<Vec<u8>, PptxError> {
    let mut opc = snap.opc.clone();
    let mut xml_paths = std::collections::HashSet::new();
    for part in &snap.xml_parts {
        if !pptx_part_is_xml(&part.path, &part.content_type) {
            return Err(PptxError::Malformed(format!("logical XML part {} has a non-XML content type", part.path)));
        }
        if !xml_paths.insert(part.path.as_str()) {
            return Err(PptxError::Malformed(format!("duplicate logical XML part {}", part.path)));
        }
        let bytes = semio_s_artifact_stdio_zip::opc::xml_document_to_opc_text_checked(&part.document).map_err(|detail| PptxError::Xml { part: part.path.clone(), detail })?.into_bytes();
        if opc.content_types.resolve(&part.path) == Some(part.content_type.as_str()) {
            if let Some(existing) = opc.parts.iter_mut().find(|candidate| candidate.path == part.path) {
                existing.content_type = part.content_type.clone();
                existing.bytes = bytes;
            } else {
                opc.parts.push(semio_s_artifact_stdio_zip::opc::OpcPart { path: part.path.clone(), content_type: part.content_type.clone(), bytes });
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
    Ok(semio_s_artifact_stdio_zip::opc::encode_opc_with_package_order(&opc)?)
}
//#endregion 🔖️Codec

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
