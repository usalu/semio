//! 🏗️ Pure PresentationML construction and package relationships.
use crate::standards::v_ecma_376::subsets::base::schema::{snapshot::*,vocabulary::*};
use crate::PptxSnapshot;
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlDocument,XmlNode};
use semio_s_artifact_stdio_zip::opc::{OpcPackage,OpcRelationship,OpcTargetMode,REL_TYPE_OFFICE_DOCUMENT};
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


fn element(name:&str,attrs:Vec<semio_s_artifact_stdio_xml::schema::snapshot::XmlAttr>,children:Vec<XmlNode>)->XmlNode{XmlNode::Element{name:name.into(),attrs,children}}
fn document(root:XmlNode)->XmlDocument{XmlDocument{root:Some(root),prolog:Vec::new(),epilog:Vec::new(),doctype:None,declaration:None}}
fn group_shape_tree()->XmlNode{element("p:spTree",vec![],vec![element("p:nvGrpSpPr",vec![],vec![element("p:cNvPr",vec![attr("id","1"),attr("name","")],vec![]),element("p:cNvGrpSpPr",vec![],vec![]),element("p:nvPr",vec![],vec![])]),element("p:grpSpPr",vec![],vec![])])}
fn master_document()->XmlDocument{document(element("p:sldMaster",vec![attr("xmlns:a",A_NS),attr("xmlns:p",P_NS),attr("xmlns:r",R_NS)],vec![element("p:cSld",vec![],vec![group_shape_tree()]),element("p:clrMap",["bg1","tx1","bg2","tx2","accent1","accent2","accent3","accent4","accent5","accent6","hlink","folHlink"].into_iter().zip(["lt1","dk1","lt2","dk2","accent1","accent2","accent3","accent4","accent5","accent6","hlink","folHlink"]).map(|(key,value)|attr(key,value)).collect(),vec![]),element("p:sldLayoutIdLst",vec![],vec![element("p:sldLayoutId",vec![attr("id","2147483649"),attr("r:id","rId1")],vec![])])]))}
fn layout_document()->XmlDocument{document(element("p:sldLayout",vec![attr("xmlns:a",A_NS),attr("xmlns:p",P_NS),attr("type","blank"),attr("preserve","1")],vec![element("p:cSld",vec![],vec![group_shape_tree()]),element("p:clrMapOvr",vec![],vec![element("a:masterClrMapping",vec![],vec![])])]))}
fn scheme_fill()->XmlNode{element("a:solidFill",vec![],vec![element("a:schemeClr",vec![attr("val","phClr")],vec![])])}
fn font_family(name:&str)->XmlNode{element(name,vec![],vec![element("a:latin",vec![attr("typeface","Calibri")],vec![])])}
fn theme_document()->XmlDocument{
 let colors=[("dk1","windowText","000000"),("lt1","window","FFFFFF")].into_iter().map(|(name,value,last)|element(&format!("a:{name}"),vec![],vec![element("a:sysClr",vec![attr("val",value),attr("lastClr",last)],vec![])])).chain([("dk2","1F497D"),("lt2","EEECE1"),("accent1","4F81BD"),("accent2","C0504D"),("accent3","9BBB59"),("accent4","8064A2"),("accent5","4BACC6"),("accent6","F79646"),("hlink","0000FF"),("folHlink","800080")].into_iter().map(|(name,value)|element(&format!("a:{name}"),vec![],vec![element("a:srgbClr",vec![attr("val",value)],vec![])]))).collect();
 let format=element("a:fmtScheme",vec![attr("name","Minimal")],vec![element("a:fillStyleLst",vec![],(0..3).map(|_|scheme_fill()).collect()),element("a:lnStyleLst",vec![],(0..3).map(|_|element("a:ln",vec![],vec![scheme_fill()])).collect()),element("a:effectStyleLst",vec![],(0..3).map(|_|element("a:effectStyle",vec![],vec![element("a:effectLst",vec![],vec![])])).collect()),element("a:bgFillStyleLst",vec![],(0..3).map(|_|scheme_fill()).collect())]);
 document(element("a:theme",vec![attr("xmlns:a",A_NS),attr("name","Minimal")],vec![element("a:themeElements",vec![],vec![element("a:clrScheme",vec![attr("name","Minimal")],colors),element("a:fontScheme",vec![attr("name","Minimal")],vec![font_family("a:majorFont"),font_family("a:minorFont")]),format])]))
}
/// 🏗️ Builds complete authoritative logical PresentationML without serializing an intermediate package.
pub fn build_minimal_pptx(presentation:PptxPresentation)->PptxSnapshot{
 let mut opc=OpcPackage::empty();opc.content_types.set_default("rels",semio_s_artifact_stdio_zip::opc::RELS_CONTENT_TYPE);opc.content_types.set_default("xml","application/xml");
 let mut xml_parts=vec![PptxXmlPart{path:SLIDE_MASTER_PART.into(),content_type:SLIDE_MASTER_CONTENT_TYPE.into(),document:master_document()},PptxXmlPart{path:SLIDE_LAYOUT_PART.into(),content_type:SLIDE_LAYOUT_CONTENT_TYPE.into(),document:layout_document()},PptxXmlPart{path:THEME_PART.into(),content_type:THEME_CONTENT_TYPE.into(),document:theme_document()}];
 opc.add_relationship(SLIDE_MASTER_PART,"rId1",REL_TYPE_SLIDE_LAYOUT,"../slideLayouts/slideLayout1.xml");opc.add_relationship(SLIDE_MASTER_PART,"rId2",REL_TYPE_THEME,"../theme/theme1.xml");opc.add_relationship(SLIDE_LAYOUT_PART,"rId1",REL_TYPE_SLIDE_MASTER,"../slideMasters/slideMaster1.xml");
 let(master_rid,slide_rids,relationships)=presentation_relationships(&[],presentation.slides.len());let mut entries=Vec::with_capacity(presentation.slides.len());
 for(index,slide)in presentation.slides.iter().enumerate(){let path=format!("ppt/slides/slide{}.xml",index+1);xml_parts.push(PptxXmlPart{path:path.clone(),content_type:SLIDE_CONTENT_TYPE.into(),document:slide_to_xml(slide)});entries.push((256+index as u32,slide_rids[index].clone()));opc.relationships.replace_owner(path,vec![OpcRelationship{id:"rId1".into(),rel_type:REL_TYPE_SLIDE_LAYOUT.into(),target:"../slideLayouts/slideLayout1.xml".into(),target_mode:OpcTargetMode::Internal}]);}
 opc.relationships.replace_owner(PRESENTATION_PART.to_string(),relationships);opc.add_generated_relationship("",REL_TYPE_OFFICE_DOCUMENT,PRESENTATION_PART);
 xml_parts.push(PptxXmlPart{path:PRESENTATION_PART.into(),content_type:PRESENTATION_CONTENT_TYPE.into(),document:presentation_to_xml(&master_rid,&entries)});xml_parts.sort_unstable_by(|a,b|a.path.cmp(&b.path));for part in &xml_parts{opc.content_types.set_override(&part.path,&part.content_type);}
 PptxSnapshot::from_parts(opc,xml_parts)
}
