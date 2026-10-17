//! 🖼️ Presentation writer of the PPTX oracle: the namespace and content-type constants, the shape, slide and relationship renderers and the presentation writer. See [`super`].

use super::*;

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
pub(super) fn write_presentation(original: &Package, slides: &[PSlide]) -> Result<Package, String> {
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
