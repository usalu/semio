//! ⚙️ PresentationML (pptx) engine — real OPC container + slide-list/shape-tree model. Zip/OPC/XML
//! byte-level work is never reimplemented here: it is reused from the shared
//! `crate::artifacts::zip::opc` layer. `ppt/slideMasters`/`ppt/slideLayouts`/`ppt/theme` are
//! unmodeled boilerplate every real reader still needs to open the package validly: they are
//! synthesized once (fixed minimal-but-schema-shaped constants) when building a package from
//! scratch, and left verbatim (untouched) whenever they already exist in a decoded package.
//!
//! `p:spTree`'s DIRECT children (`p:sp`/`p:pic`/anything else) become one `PptxShape` each --
//! per this ticket's W0 finding, the shape tree used to be flattened away entirely (every
//! `p:txBody`'s paragraphs concatenated, shape boundaries discarded). Shapes nested inside a
//! `p:grpSp` group, `p:graphicFrame` (charts/tables/SmartArt), `p:cxnSp` connectors, and anything
//! unrecognized fall back to `PptxShape::Other{xml}` -- the exact serialized child node, verbatim
//! -- so nothing real on disk is silently dropped.

use crate::artifacts::pptx::{schema::snapshot::{PptxParagraph, PptxPresentation, PptxRun, PptxShape, PptxSlide, PptxTransform}, PptxArtifact, PptxDiff, PptxMutation, PptxSnapshot, STDIO_PPTX_DOCUMENT_SCHEMA};
use crate::artifacts::xml::schema::snapshot::{xml_document_from_text, xml_document_to_text, XmlAttr, XmlDocument, XmlNode};
use crate::artifacts::zip::opc::{self, OpcPackage, OpcRelationship, OpcTargetMode, REL_TYPE_OFFICE_DOCUMENT, RELS_CONTENT_TYPE};

//#region 🔖️Error
/// ⚠️ Typed pptx decode/encode failure — a package this engine cannot honestly interpret is
/// never fabricated into a partial/empty presentation.
#[derive(Clone, Debug, PartialEq)]
pub enum PptxError {
    Opc(opc::OpcError),
    MissingPresentationRelationship,
    MissingPart(String),
    Xml { part: String, detail: String },
    Malformed(String),
}

impl std::fmt::Display for PptxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Opc(e) => write!(f, "pptx: {e}"),
            Self::MissingPresentationRelationship => write!(f, "pptx: package root has no officeDocument relationship"),
            Self::MissingPart(p) => write!(f, "pptx: missing required part {p}"),
            Self::Xml { part, detail } => write!(f, "pptx: xml in {part}: {detail}"),
            Self::Malformed(detail) => write!(f, "pptx: {detail}"),
        }
    }
}

impl std::error::Error for PptxError {}

impl From<opc::OpcError> for PptxError {
    fn from(e: opc::OpcError) -> Self { Self::Opc(e) }
}
//#endregion 🔖️Error

//#region 🔖️Constants
const A_NS: &str = "http://schemas.openxmlformats.org/drawingml/2006/main";
const P_NS: &str = "http://schemas.openxmlformats.org/presentationml/2006/main";
const R_NS: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";

const PRESENTATION_PART: &str = "ppt/presentation.xml";
const SLIDE_MASTER_PART: &str = "ppt/slideMasters/slideMaster1.xml";
const SLIDE_LAYOUT_PART: &str = "ppt/slideLayouts/slideLayout1.xml";
const THEME_PART: &str = "ppt/theme/theme1.xml";

const PRESENTATION_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml";
const SLIDE_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.presentationml.slide+xml";
const SLIDE_LAYOUT_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml";
const SLIDE_MASTER_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml";
const THEME_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.theme+xml";

const REL_TYPE_SLIDE: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide";
const REL_TYPE_SLIDE_LAYOUT: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout";
const REL_TYPE_SLIDE_MASTER: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster";
const REL_TYPE_THEME: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme";

/// 🏅️ ISO/IEC 29500-1:2016 Strict's officeDocument relationship type -- Strict packages carry
/// this instead of `REL_TYPE_OFFICE_DOCUMENT` (see `🪆️subsets/🔣️component.json`'s "strictRelBase"
/// citation, ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES). `regenerate_presentation_parts`
/// never writes this -- this engine's own writer only ever emits Transitional -- but `decode_pptx`
/// and `sniff_pptx_bytes` must still recognize a genuine Strict-relationship-typed input package,
/// or the `✳️strict` subset's analyzer could never see real Strict bytes at all.
const REL_TYPE_OFFICE_DOCUMENT_STRICT: &str = "http://purl.oclc.org/ooxml/officeDocument/relationships/officeDocument";

/// 🧭️ Resolves the package root's officeDocument relationship regardless of whether it was
/// authored under the Transitional or the Strict relationship-type namespace -- see
/// `REL_TYPE_OFFICE_DOCUMENT_STRICT`.
pub fn resolve_office_document_relationship(opc: &OpcPackage) -> Option<String> {
    opc.resolve_relationship("", REL_TYPE_OFFICE_DOCUMENT).or_else(|| opc.resolve_relationship("", REL_TYPE_OFFICE_DOCUMENT_STRICT))
}

fn attr(name: &str, value: &str) -> XmlAttr {
    XmlAttr { name: name.into(), value: value.into() }
}

fn attr_val<'a>(attrs: &'a [XmlAttr], name: &str) -> Option<&'a str> {
    attrs.iter().find(|a| a.name == name).map(|a| a.value.as_str())
}

fn find_child<'a>(children: &'a [XmlNode], name: &str) -> Option<&'a XmlNode> {
    children.iter().find(|c| matches!(c, XmlNode::Element { name: n, .. } if n == name))
}

fn element_children(node: &XmlNode) -> &[XmlNode] {
    match node {
        XmlNode::Element { children, .. } => children,
        _ => &[],
    }
}

/// 🔤️ Serializes a single node (not a whole document) via the xml module's own document
/// serializer -- wraps it as a document root, discards the (absent) declaration/doctype.
fn node_to_text(node: &XmlNode) -> String {
    xml_document_to_text(&XmlDocument { root: Some(node.clone()), doctype: None, declaration: None })
}

/// 🔤️ Parses a single node back from `node_to_text`'s output (or any other well-formed XML
/// fragment). Falls back to a `Comment` node carrying the raw text verbatim on parse failure --
/// this can only happen for a hand-constructed `PptxShape::Other{xml}` with malformed content
/// (never for anything this engine itself produced), and a comment still round-trips the string
/// losslessly instead of panicking or silently dropping it.
fn node_from_text(text: &str) -> XmlNode {
    xml_document_from_text(text).ok().and_then(|d| d.root).unwrap_or_else(|| XmlNode::Comment { text: text.to_string() })
}

/// 📐️ Minimal-but-schema-shaped `slideMaster1.xml` — synthesized once when a package has no
/// existing slide master, never regenerated over a decoded one.
const MINIMAL_SLIDE_MASTER_XML: &str = concat!(
    r#"<p:sldMaster xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">"#,
    "<p:cSld><p:spTree>",
    r#"<p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr><p:grpSpPr/>"#,
    "</p:spTree></p:cSld>",
    r#"<p:clrMap bg1="lt1" tx1="dk1" bg2="lt2" tx2="dk2" accent1="accent1" accent2="accent2" accent3="accent3" accent4="accent4" accent5="accent5" accent6="accent6" hlink="hlink" folHlink="folHlink"/>"#,
    r#"<p:sldLayoutIdLst><p:sldLayoutId id="2147483649" r:id="rId1"/></p:sldLayoutIdLst>"#,
    "</p:sldMaster>",
);

/// 📐️ Minimal-but-schema-shaped `slideLayout1.xml`.
const MINIMAL_SLIDE_LAYOUT_XML: &str = concat!(
    r#"<p:sldLayout xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" type="blank" preserve="1">"#,
    "<p:cSld><p:spTree>",
    r#"<p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr><p:grpSpPr/>"#,
    "</p:spTree></p:cSld>",
    "<p:clrMapOvr><a:masterClrMapping/></p:clrMapOvr>",
    "</p:sldLayout>",
);

/// 🎨️ Minimal-but-schema-shaped `theme1.xml` (all required color/font/format-scheme slots).
const MINIMAL_THEME_XML: &str = concat!(
    r#"<a:theme xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" name="Minimal">"#,
    "<a:themeElements>",
    r#"<a:clrScheme name="Minimal">"#,
    r#"<a:dk1><a:sysClr val="windowText" lastClr="000000"/></a:dk1>"#,
    r#"<a:lt1><a:sysClr val="window" lastClr="FFFFFF"/></a:lt1>"#,
    r#"<a:dk2><a:srgbClr val="1F497D"/></a:dk2>"#,
    r#"<a:lt2><a:srgbClr val="EEECE1"/></a:lt2>"#,
    r#"<a:accent1><a:srgbClr val="4F81BD"/></a:accent1>"#,
    r#"<a:accent2><a:srgbClr val="C0504D"/></a:accent2>"#,
    r#"<a:accent3><a:srgbClr val="9BBB59"/></a:accent3>"#,
    r#"<a:accent4><a:srgbClr val="8064A2"/></a:accent4>"#,
    r#"<a:accent5><a:srgbClr val="4BACC6"/></a:accent5>"#,
    r#"<a:accent6><a:srgbClr val="F79646"/></a:accent6>"#,
    r#"<a:hlink><a:srgbClr val="0000FF"/></a:hlink>"#,
    r#"<a:folHlink><a:srgbClr val="800080"/></a:folHlink>"#,
    "</a:clrScheme>",
    r#"<a:fontScheme name="Minimal"><a:majorFont><a:latin typeface="Calibri"/></a:majorFont><a:minorFont><a:latin typeface="Calibri"/></a:minorFont></a:fontScheme>"#,
    r#"<a:fmtScheme name="Minimal">"#,
    r#"<a:fillStyleLst><a:solidFill><a:schemeClr val="phClr"/></a:solidFill><a:solidFill><a:schemeClr val="phClr"/></a:solidFill><a:solidFill><a:schemeClr val="phClr"/></a:solidFill></a:fillStyleLst>"#,
    r#"<a:lnStyleLst><a:ln><a:solidFill><a:schemeClr val="phClr"/></a:solidFill></a:ln><a:ln><a:solidFill><a:schemeClr val="phClr"/></a:solidFill></a:ln><a:ln><a:solidFill><a:schemeClr val="phClr"/></a:solidFill></a:ln></a:lnStyleLst>"#,
    r#"<a:effectStyleLst><a:effectStyle><a:effectLst/></a:effectStyle><a:effectStyle><a:effectLst/></a:effectStyle><a:effectStyle><a:effectLst/></a:effectStyle></a:effectStyleLst>"#,
    r#"<a:bgFillStyleLst><a:solidFill><a:schemeClr val="phClr"/></a:solidFill><a:solidFill><a:schemeClr val="phClr"/></a:solidFill><a:solidFill><a:schemeClr val="phClr"/></a:solidFill></a:bgFillStyleLst>"#,
    "</a:fmtScheme>",
    "</a:themeElements>",
    "</a:theme>",
);
//#endregion 🔖️Constants

//#region 🔖️TextXml
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

fn paragraph_to_xml(p: &PptxParagraph) -> XmlNode {
    XmlNode::Element { name: "a:p".into(), attrs: vec![], children: p.runs.iter().map(run_to_xml).collect() }
}

fn run_from_xml(node: &XmlNode) -> Option<PptxRun> {
    let XmlNode::Element { name, children, .. } = node else { return None };
    if name != "a:r" {
        return None;
    }
    let mut bold = false;
    let mut italic = false;
    let mut font_size = None;
    let mut text = String::new();
    for rc in children {
        let XmlNode::Element { name, attrs, children: inner } = rc else { continue };
        match name.as_str() {
            "a:rPr" => {
                if let Some(b) = attr_val(attrs, "b") {
                    bold = b == "1";
                }
                if let Some(i) = attr_val(attrs, "i") {
                    italic = i == "1";
                }
                if let Some(sz) = attr_val(attrs, "sz").and_then(|v| v.parse::<u32>().ok()) {
                    font_size = Some(sz / 100);
                }
            }
            "a:t" => {
                for t in inner {
                    if let XmlNode::Text { text: t } = t {
                        text.push_str(t);
                    }
                }
            }
            _ => {}
        }
    }
    Some(PptxRun { text, bold, italic, font_size })
}

fn paragraph_from_xml(node: &XmlNode) -> PptxParagraph {
    let mut runs = Vec::new();
    for c in element_children(node) {
        if let Some(r) = run_from_xml(c) {
            runs.push(r);
        }
    }
    PptxParagraph { runs }
}

fn text_frame_to_xml(paragraphs: &[PptxParagraph]) -> Vec<XmlNode> {
    let mut children = vec![XmlNode::Element { name: "a:bodyPr".into(), attrs: vec![], children: vec![] }];
    children.extend(paragraphs.iter().map(paragraph_to_xml));
    children
}

/// 🔎️ Every `a:p` directly inside `p:txBody` (direct children only -- a real `p:txBody` never
/// nests paragraphs inside anything else).
fn text_frame_from_xml(tx_body: &XmlNode) -> Vec<PptxParagraph> {
    element_children(tx_body).iter().filter(|c| matches!(c, XmlNode::Element { name, .. } if name == "a:p")).map(paragraph_from_xml).collect()
}
//#endregion 🔖️TextXml

//#region 🔖️ShapeXml
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

/// 🔎️ Reads `p:spPr/a:xfrm`'s `a:off`/`a:ext` (defaulting each missing field to `0`, same
/// convention the rest of this engine uses for "not present in the XML").
fn position_from_xml(shape_children: &[XmlNode]) -> PptxTransform {
    let Some(sp_pr) = find_child(shape_children, "p:spPr") else { return PptxTransform::default() };
    let Some(xfrm) = find_child(element_children(sp_pr), "a:xfrm") else { return PptxTransform::default() };
    let xfrm_children = element_children(xfrm);
    let (mut x, mut y, mut cx, mut cy) = (0i64, 0i64, 0i64, 0i64);
    if let Some(XmlNode::Element { attrs, .. }) = find_child(xfrm_children, "a:off") {
        x = attr_val(attrs, "x").and_then(|v| v.parse().ok()).unwrap_or(0);
        y = attr_val(attrs, "y").and_then(|v| v.parse().ok()).unwrap_or(0);
    }
    if let Some(XmlNode::Element { attrs, .. }) = find_child(xfrm_children, "a:ext") {
        cx = attr_val(attrs, "cx").and_then(|v| v.parse().ok()).unwrap_or(0);
        cy = attr_val(attrs, "cy").and_then(|v| v.parse().ok()).unwrap_or(0);
    }
    PptxTransform { x, y, cx, cy }
}

/// 🏗️ Serializes one `PptxShape` as its `p:spTree`-child XML node. `id` is a synthesized
/// `p:cNvPr@id` (this layer doesn't model shape ids -- any positive, unique-within-the-slide
/// value satisfies the schema).
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
        PptxShape::Other { xml } => node_from_text(xml),
    }
}

/// 🧭️ Classifies one `p:spTree` DIRECT child into a typed `PptxShape` (`p:sp`/`p:pic` get real
/// per-kind typing; everything else -- `p:graphicFrame`, `p:grpSp`, `p:cxnSp`, unrecognized --
/// falls back to `Other{xml}`, the exact serialized node, verbatim).
fn shape_from_xml_node(node: &XmlNode) -> PptxShape {
    let XmlNode::Element { name, children, .. } = node else { return PptxShape::Other { xml: node_to_text(node) } };
    match name.as_str() {
        "p:sp" => {
            let ph_type = find_child(children, "p:nvSpPr")
                .and_then(|nv| find_child(element_children(nv), "p:nvPr"))
                .and_then(|nvpr| find_child(element_children(nvpr), "p:ph"))
                .map(|ph| match ph {
                    XmlNode::Element { attrs, .. } => attr_val(attrs, "type").unwrap_or("body").to_string(),
                    _ => "body".to_string(),
                });
            let position = position_from_xml(children);
            let text_frame = find_child(children, "p:txBody").map(text_frame_from_xml).unwrap_or_default();
            match ph_type {
                Some(kind) => PptxShape::Placeholder { kind, text_frame, position },
                None => PptxShape::TextBox { text_frame, position },
            }
        }
        "p:pic" => {
            let blip_rel_id = find_child(children, "p:blipFill")
                .and_then(|fill| find_child(element_children(fill), "a:blip"))
                .and_then(|blip| match blip { XmlNode::Element { attrs, .. } => attr_val(attrs, "r:embed"), _ => None })
                .unwrap_or_default()
                .to_string();
            let position = position_from_xml(children);
            PptxShape::Picture { blip_rel_id, position }
        }
        _ => PptxShape::Other { xml: node_to_text(node) },
    }
}
//#endregion 🔖️ShapeXml

//#region 🔖️SlideXml
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
        root: Some(XmlNode::Element {
            name: "p:sld".into(),
            attrs: vec![attr("xmlns:a", A_NS), attr("xmlns:p", P_NS)],
            children: vec![XmlNode::Element { name: "p:cSld".into(), attrs: vec![], children: vec![XmlNode::Element { name: "p:spTree".into(), attrs: vec![], children: sp_tree_children }] }],
        }),
        doctype: None,
        declaration: None,
    }
}

/// 🔎️ Collects every `p:spTree` DIRECT child (skipping the group's own `p:nvGrpSpPr`/
/// `p:grpSpPr` container elements) into one `PptxShape` each, in document order -- shape
/// BOUNDARIES are preserved (not flattened away like the pre-migration model).
fn collect_shapes(root: &XmlNode) -> Vec<PptxShape> {
    let XmlNode::Element { children, .. } = root else { return Vec::new() };
    let Some(c_sld) = find_child(children, "p:cSld") else { return Vec::new() };
    let Some(sp_tree) = find_child(element_children(c_sld), "p:spTree") else { return Vec::new() };
    element_children(sp_tree)
        .iter()
        .filter(|c| !matches!(c, XmlNode::Element { name, .. } if name == "p:nvGrpSpPr" || name == "p:grpSpPr"))
        .map(shape_from_xml_node)
        .collect()
}
//#endregion 🔖️SlideXml

//#region 🔖️PresentationXml
fn presentation_to_xml(master_rid: &str, sld_id_entries: &[(u32, String)]) -> XmlDocument {
    let sld_ids = sld_id_entries.iter().map(|(id, rid)| XmlNode::Element { name: "p:sldId".into(), attrs: vec![attr("id", &id.to_string()), attr("r:id", rid)], children: vec![] }).collect();
    XmlDocument {
        root: Some(XmlNode::Element {
            name: "p:presentation".into(),
            attrs: vec![attr("xmlns:a", A_NS), attr("xmlns:p", P_NS), attr("xmlns:r", R_NS)],
            children: vec![
                XmlNode::Element {
                    name: "p:sldMasterIdLst".into(),
                    attrs: vec![],
                    children: vec![XmlNode::Element { name: "p:sldMasterId".into(), attrs: vec![attr("id", "2147483648"), attr("r:id", master_rid)], children: vec![] }],
                },
                XmlNode::Element { name: "p:sldIdLst".into(), attrs: vec![], children: sld_ids },
            ],
        }),
        doctype: None,
        declaration: None,
    }
}

fn presentation_slide_rids_from_xml(doc: &XmlDocument, part: &str) -> Result<Vec<String>, PptxError> {
    let bad = |detail: String| PptxError::Xml { part: part.into(), detail };
    let root = doc.root.as_ref().ok_or_else(|| bad("empty document".into()))?;
    let XmlNode::Element { name, children, .. } = root else { return Err(bad("root is not an element".into())) };
    if name != "p:presentation" {
        return Err(bad(format!("expected <p:presentation>, got <{name}>")));
    }
    let sld_id_lst = children.iter().find_map(|c| match c { XmlNode::Element { name, children, .. } if name == "p:sldIdLst" => Some(children), _ => None });
    let mut out = Vec::new();
    if let Some(children) = sld_id_lst {
        for c in children {
            let XmlNode::Element { name, attrs, .. } = c else { continue };
            if name != "p:sldId" {
                continue;
            }
            let rid = attr_val(attrs, "r:id").ok_or_else(|| bad("<p:sldId> missing r:id".into()))?.to_string();
            out.push(rid);
        }
    }
    Ok(out)
}
//#endregion 🔖️PresentationXml

//#region 🔖️Codec
/// 🔄 Regenerates every pptx-owned part (`ppt/presentation.xml`, every `ppt/slides/slideN.xml`
/// and its relationships) from `presentation`, discarding stale slide parts a shrinking slide
/// list would otherwise leave orphaned. Synthesizes the slideMaster/slideLayout/theme boilerplate
/// chain only when entirely absent — an already-decoded package's real ones are left untouched.
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
    opc.relationships.retain(|owner, _| !owner.starts_with("ppt/slides/"));

    opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
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

    let master_rel = opc
        .relationships_for(PRESENTATION_PART)
        .iter()
        .find(|r| r.rel_type == REL_TYPE_SLIDE_MASTER)
        .cloned()
        .unwrap_or(OpcRelationship { id: "rId1".into(), rel_type: REL_TYPE_SLIDE_MASTER.into(), target: "slideMasters/slideMaster1.xml".into(), target_mode: OpcTargetMode::Internal });
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
        opc.relationships.insert(
            path,
            vec![OpcRelationship { id: "rId1".into(), rel_type: REL_TYPE_SLIDE_LAYOUT.into(), target: "../slideLayouts/slideLayout1.xml".into(), target_mode: OpcTargetMode::Internal }],
        );
    }
    opc.relationships.insert(PRESENTATION_PART.to_string(), pres_rels);

    let presentation_bytes = xml_document_to_text(&presentation_to_xml(&master_rid, &sld_id_entries)).into_bytes();
    opc.set_part(PRESENTATION_PART, PRESENTATION_CONTENT_TYPE, presentation_bytes);

    if opc.relationships_for("").iter().all(|r| r.rel_type != REL_TYPE_OFFICE_DOCUMENT) {
        opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, PRESENTATION_PART);
    }
}

/// 🏗️ Assembles a brand-new, minimal-but-valid OPC package around `presentation` — correct
/// `[Content_Types].xml`, root `_rels/.rels`, `ppt/presentation.xml` + its relationships, every
/// slide, and a synthesized slideMaster/slideLayout/theme chain real readers expect to exist.
pub fn build_minimal_pptx(presentation: PptxPresentation) -> PptxSnapshot {
    let mut opc = OpcPackage::empty();
    regenerate_presentation_parts(&mut opc, &presentation);
    PptxSnapshot::from_parts(opc, presentation)
}

pub fn encode_pptx(snap: &PptxSnapshot) -> Result<Vec<u8>, PptxError> {
    let mut opc = snap.opc.clone();
    regenerate_presentation_parts(&mut opc, &snap.presentation);
    Ok(opc::encode_opc(&opc)?)
}

pub fn decode_pptx(data: &[u8]) -> Result<PptxSnapshot, PptxError> {
    let opc = opc::decode_opc(data)?;
    let presentation_path = resolve_office_document_relationship(&opc).ok_or(PptxError::MissingPresentationRelationship)?;
    let bytes = opc.part_bytes(&presentation_path).ok_or_else(|| PptxError::MissingPart(presentation_path.clone()))?;
    let text = String::from_utf8(bytes.to_vec()).map_err(|_| PptxError::Xml { part: presentation_path.clone(), detail: "not valid utf-8".into() })?;
    let xml = xml_document_from_text(&text).map_err(|e| PptxError::Xml { part: presentation_path.clone(), detail: e })?;
    let slide_rids = presentation_slide_rids_from_xml(&xml, &presentation_path)?;

    let pres_rels = opc.relationships_for(&presentation_path);
    let mut slides = Vec::with_capacity(slide_rids.len());
    for rid in &slide_rids {
        let rel = pres_rels
            .iter()
            .find(|r| &r.id == rid)
            .ok_or_else(|| PptxError::Malformed(format!("presentation references unknown relationship id {rid}")))?;
        let path = opc::resolve_relationship_target(&presentation_path, &rel.target);
        let bytes = opc.part_bytes(&path).ok_or_else(|| PptxError::MissingPart(path.clone()))?;
        let text = String::from_utf8(bytes.to_vec()).map_err(|_| PptxError::Xml { part: path.clone(), detail: "not valid utf-8".into() })?;
        let slide_xml = xml_document_from_text(&text).map_err(|e| PptxError::Xml { part: path.clone(), detail: e })?;
        let shapes = slide_xml.root.as_ref().map(collect_shapes).unwrap_or_default();
        slides.push(PptxSlide { shapes });
    }

    Ok(PptxSnapshot::from_parts(opc, PptxPresentation { slides }))
}

pub fn empty_pptx_snapshot() -> PptxSnapshot { PptxSnapshot::default() }

/// 📄️ FG-wave: the demo `stdio.pptx` presentation — a genuinely non-trivial `PptxSnapshot`
/// exercising a title `Placeholder` (bold run), a `Picture`, a `TextBox` with mixed bold/italic
/// runs across two paragraphs, and one raw-retained `Other` shape (`p:graphicFrame`, round-tripped
/// verbatim), plus one unmodeled raw OPC part (`ppt/media/image1.png`, verbatim-retained). The
/// single source of truth for `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`/
/// `🎒️example.pack.semio` (both are literally this snapshot's `print_dsl`/`encode_pack` output,
/// asserted equal by `fixture_honesty_law` below) — same shape `📜️docx/…/⚙️engine/🦀️component.rs`'s
/// own `demo_docx_snapshot()` establishes.
pub fn demo_pptx_snapshot() -> PptxSnapshot {
    let presentation = PptxPresentation {
        slides: vec![
            PptxSlide {
                shapes: vec![
                    PptxShape::Placeholder {
                        kind: "title".into(),
                        text_frame: vec![PptxParagraph { runs: vec![PptxRun { text: "Semio Demo".into(), bold: true, italic: false, font_size: Some(44) }] }],
                        position: PptxTransform { x: 685800, y: 457200, cx: 7772400, cy: 1143000 },
                    },
                    PptxShape::Picture { blip_rel_id: "rId2".into(), position: PptxTransform { x: 685800, y: 1600200, cx: 2286000, cy: 1714500 } },
                ],
            },
            PptxSlide {
                shapes: vec![
                    PptxShape::TextBox {
                        text_frame: vec![
                            PptxParagraph {
                                runs: vec![
                                    PptxRun { text: "Bold and ".into(), bold: true, italic: false, font_size: None },
                                    PptxRun { text: "italic".into(), bold: false, italic: true, font_size: None },
                                ],
                            },
                            PptxParagraph::text("second paragraph"),
                        ],
                        position: PptxTransform { x: 685800, y: 457200, cx: 7772400, cy: 2286000 },
                    },
                    // 🩹 Deliberately no `<a:graphic/>` child here: an UNATTRIBUTED self-closing
                    // element (real bytes `<a:graphic/>`, no space) would hit the SAME lexer
                    // identifier-fusion property this file's own grammar documents for `p:nvPr`/
                    // `p:grpSpPr`/etc (`"cNvGrpSpPr/"` fuses into ONE token) -- but the GENERIC
                    // `x-elem` raw-retention fallback (unlike this artifact's own TYPED shape
                    // productions, which model every real fused case with an explicit literal
                    // token) has no way to disambiguate "bare self-close" from "open tag, more
                    // content follows" using only same-shape `LT x-name GT` lookahead -- a
                    // genuine, documented limitation of the x-elem restatement (same one docx's
                    // own snapshot grammar's `x-elem` inherits), not something this demo fixture
                    // should paper over by accident. Keeping every attr non-empty here keeps the
                    // conformance law honest without exercising that known gap.
                    PptxShape::Other { xml: r#"<p:graphicFrame><p:nvGraphicFramePr><p:cNvPr id="9" name="Table 1"/></p:nvGraphicFramePr></p:graphicFrame>"#.into() },
                ],
            },
        ],
    };
    let mut snap = build_minimal_pptx(presentation);
    snap.opc.set_part("ppt/media/image1.png", "image/png", b"\x89PNG\r\n\x1a\n".to_vec());
    // 🩹 Canonicalize `opc.parts` ORDER by round-tripping through one real encode/decode pass --
    // `regenerate_presentation_parts` (invoked again inside `encode_pptx`) retains-away and
    // re-appends `ppt/slides/*`/`ppt/presentation.xml` on EVERY call (see
    // `double_regenerate_keeps_opc_parts_order_stable`'s own regression note above); since THIS
    // demo snapshot manually appends an EXTRA raw part (`ppt/media/image1.png`) AFTER
    // `build_minimal_pptx`'s own regen pass, a LATER `encode_pptx` call (invoked by `print_dsl`/
    // `encode_pack`/every conformance law below) would otherwise reorder `opc.parts` differently
    // from whatever order this function returns -- exactly the failure mode
    // `fixture_honesty_law`'s `print_dsl`/`parse_dsl` round trip exists to catch. Round-tripping
    // once here means every LATER `encode_pptx` call on this snapshot is a stable no-op reorder.
    let canonical_bytes = encode_pptx(&snap).expect("encode demo pptx for order canonicalization");
    decode_pptx(&canonical_bytes).expect("decode demo pptx for order canonicalization")
}

pub fn register() {
    crate::artifacts::pptx::io_registry::register();
    ::schema::register_artifact_schema_descriptor(crate::artifacts::pptx::schema::pptx_artifact_schema_descriptor());
    register_artifact_inferences();
    register_pilot_languages();
    store::register_document_codec(store::ArtifactCodec::of::<PptxSnapshot, PptxMutation>(STDIO_PPTX_DOCUMENT_SCHEMA));
    // 🛡️ D5's generic validate-on-build hook: registers the ✳️strict/✳️transitional subsets'
    // SubsetValidators so `io_dispatch`/`wire_artifact_compose` re-check them for free. Their
    // ComposerEntrys themselves are registered separately via this standard's own
    // `composer::entries()` aggregation.
    crate::artifacts::pptx::standards::v_ecma_376::subsets::strict::io::register();
    crate::artifacts::pptx::standards::v_ecma_376::subsets::transitional::io::register();
}

/// 💡️ Registers `s.stdio.pptx.inference`'s facet leaves into the OS-wide inference catalog —
/// sibling to the artifact schema descriptor registration above (separate registry, ticket
/// 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
pub fn register_artifact_inferences() {
    ::schema::register_artifact_inference_descriptor(crate::artifacts::pptx::standards::v_ecma_376::subsets::any::schema::inferences::pptx_artifact_inference_descriptor());
}

/// 📌️ FG-wave: 5-role `LanguageSpec` registration (Document/Ops/Diff/Pack/Spr), per
/// `📷️png/…/⚙️engine/🦀️component.rs`'s own `register_pilot_languages` exemplar pattern —
/// `stdio.pptx`/`.op`/`.diff`/`.pack`/`.spr`, all `dsl::passthrough_hooks`. `diff`'s `protocol`
/// slot stays `None`, matching the exemplar's own shape exactly (the 5-role scheme has no
/// dedicated "diff binary" role, even though `🔺️diff/💾️binary/📡️component.protocol.semio` is a
/// real, conformance-tested file — its binary form is exercised directly by `protocol_walk_law`
/// below), same precedent docx's own `register_pilot_languages` already established for this
/// OPC-family shape.
///
/// `register_schema_spec` (P2-M3's `FullResolver` insertion API) is deliberately NOT called here —
/// filed as this wave's own `mechanism_gaps` entry: it requires `fn() -> RecordSpec`, and
/// `PptxSnapshot`/`PptxDiff`/`PptxMutation` have none (all three are hand-rolled — see
/// `📸️snapshot/🦀️component.rs`'s `ArtifactDsl`/`ArtifactPack` and `🔺️diff/🦀️component.rs`/
/// `🧬️mutations/🦀️component.rs`'s own F6-verification doc comments confirming
/// `#[derive(dsl::Dsl*)]` fails to compile on every one of these types), same root cause docx's
/// own `register_pilot_languages` doc comment already documents for the identical OPC shape.
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.pptx", extension: Some("pptx"), role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::pptx::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::pptx::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::pptx::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::pptx::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.pptx"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.pptx.op", extension: None, role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::pptx::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::pptx::schema::mutations::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::pptx::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::pptx::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.pptx.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.pptx.diff", extension: None, role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::pptx::schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::pptx::schema::diff::text::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("stdio.pptx.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.pptx.pack", extension: None, role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::pptx::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::pptx::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.pptx.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.pptx.spr", extension: None, role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::pptx::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::pptx::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.pptx.spr"),
    });
}
//#endregion 🔖️Codec

//#region 🔖️Sniff
/// 🕵️ Real pptx sniff: OPC-shaped bytes whose root officeDocument relationship (Transitional or
/// Strict) resolves under `ppt/` — disambiguates from docx/xlsx sharing the same zip magic and
/// OPC shape.
pub fn sniff_pptx_bytes(data: &[u8]) -> bool {
    let Ok(opc) = opc::decode_opc(data) else { return false };
    match resolve_office_document_relationship(&opc) {
        Some(path) => path.starts_with("ppt/"),
        None => false,
    }
}
//#endregion 🔖️Sniff

//#region 🔖️ArtifactEngine
pub struct PptxEngine { artifact_state: PptxArtifact, snapshot_state: PptxSnapshot }
impl PptxEngine {
    pub fn new(snapshot: PptxSnapshot) -> Self {
        Self { artifact_state: PptxArtifact::from_snapshot(snapshot.clone()), snapshot_state: snapshot }
    }
}
//#endregion 🔖️ArtifactEngine

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn sample_presentation() -> PptxPresentation {
        PptxPresentation {
            slides: vec![
                PptxSlide {
                    shapes: vec![PptxShape::Placeholder {
                        kind: "title".into(),
                        text_frame: vec![PptxParagraph { runs: vec![PptxRun { text: "Title Slide".into(), bold: true, italic: false, font_size: Some(44) }] }],
                        position: PptxTransform { x: 100, y: 200, cx: 300, cy: 400 },
                    }],
                },
                PptxSlide {
                    shapes: vec![
                        PptxShape::TextBox { text_frame: vec![PptxParagraph::text("Second slide, plain.")], position: PptxTransform::default() },
                        PptxShape::TextBox {
                            text_frame: vec![PptxParagraph { runs: vec![PptxRun { text: "italic note".into(), bold: false, italic: true, font_size: None }] }],
                            position: PptxTransform { x: 1, y: 2, cx: 3, cy: 4 },
                        },
                        PptxShape::Picture { blip_rel_id: "rId5".into(), position: PptxTransform { x: 10, y: 20, cx: 30, cy: 40 } },
                    ],
                },
            ],
        }
    }

    #[test]
    fn builder_produces_minimal_valid_package_that_decodes_back() {
        let snap = build_minimal_pptx(sample_presentation());
        let bytes = encode_pptx(&snap).expect("encode minimal package");
        assert!(opc::sniff_opc_bytes(&bytes));
        assert!(sniff_pptx_bytes(&bytes));
        let decoded = decode_pptx(&bytes).expect("decode minimal package");
        assert_eq!(decoded.presentation, sample_presentation());
        // The synthesized boilerplate chain must actually be present — a real reader needs it.
        assert!(decoded.opc.part(SLIDE_MASTER_PART).is_some());
        assert!(decoded.opc.part(SLIDE_LAYOUT_PART).is_some());
        assert!(decoded.opc.part(THEME_PART).is_some());
    }

    #[test]
    fn decode_resolves_real_hand_built_package_with_shape_boundaries_and_position() {
        // Hand-built OOXML: a slide with TWO real shapes -- a positioned placeholder title and a
        // positioned picture -- exercising real shape-BOUNDARY recovery (not flattened text) and
        // real `a:xfrm` position decoding.
        let mut opc = OpcPackage::empty();
        opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
        opc.content_types.set_default("xml", "application/xml");

        let slide_xml = concat!(
            r#"<p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">"#,
            "<p:cSld><p:spTree>",
            r#"<p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr><p:grpSpPr/>"#,
            "<p:sp><p:nvSpPr><p:cNvPr id=\"2\" name=\"Title\"/><p:cNvSpPr/><p:nvPr><p:ph type=\"title\"/></p:nvPr></p:nvSpPr>",
            r#"<p:spPr><a:xfrm><a:off x="111" y="222"/><a:ext cx="333" cy="444"/></a:xfrm></p:spPr>"#,
            r#"<p:txBody><a:bodyPr/><a:p><a:r><a:rPr b="1" i="1" sz="4400"/><a:t>Nested &amp; bold-italic</a:t></a:r></a:p></p:txBody>"#,
            "</p:sp>",
            r#"<p:pic><p:nvPicPr><p:cNvPr id="3" name="Pic"/><p:cNvPicPr/><p:nvPr/></p:nvPicPr>"#,
            r#"<p:blipFill><a:blip r:embed="rId9"/><a:stretch><a:fillRect/></a:stretch></p:blipFill>"#,
            r#"<p:spPr><a:xfrm><a:off x="5" y="6"/><a:ext cx="7" cy="8"/></a:xfrm></p:spPr></p:pic>"#,
            "</p:spTree></p:cSld></p:sld>",
        );
        opc.set_part("ppt/slides/slide1.xml", SLIDE_CONTENT_TYPE, slide_xml.as_bytes().to_vec());
        opc.add_relationship("ppt/slides/slide1.xml", "rId1", REL_TYPE_SLIDE_LAYOUT, "../slideLayouts/slideLayout1.xml");

        let presentation_xml = concat!(
            r#"<p:presentation xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">"#,
            r#"<p:sldMasterIdLst><p:sldMasterId id="2147483648" r:id="rId1"/></p:sldMasterIdLst>"#,
            r#"<p:sldIdLst><p:sldId id="256" r:id="rId2"/></p:sldIdLst>"#,
            "</p:presentation>",
        );
        opc.set_part(PRESENTATION_PART, PRESENTATION_CONTENT_TYPE, presentation_xml.as_bytes().to_vec());
        opc.add_relationship(PRESENTATION_PART, "rId1", REL_TYPE_SLIDE_MASTER, "slideMasters/slideMaster1.xml");
        opc.add_relationship(PRESENTATION_PART, "rId2", REL_TYPE_SLIDE, "slides/slide1.xml");
        opc.set_part(SLIDE_MASTER_PART, SLIDE_MASTER_CONTENT_TYPE, MINIMAL_SLIDE_MASTER_XML.as_bytes().to_vec());
        opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, PRESENTATION_PART);

        let bytes = opc::encode_opc(&opc).expect("encode hand-built package");
        let decoded = decode_pptx(&bytes).expect("decode hand-built pptx");

        assert_eq!(decoded.presentation.slides.len(), 1);
        let shapes = &decoded.presentation.slides[0].shapes;
        assert_eq!(shapes.len(), 2, "two DIRECT shapes must be recovered as two distinct PptxShape entries, not flattened");
        let PptxShape::Placeholder { kind, text_frame, position } = &shapes[0] else { panic!("expected placeholder shape") };
        assert_eq!(kind, "title");
        assert_eq!(*position, PptxTransform { x: 111, y: 222, cx: 333, cy: 444 });
        assert_eq!(text_frame[0].runs[0].text, "Nested & bold-italic");
        assert!(text_frame[0].runs[0].bold && text_frame[0].runs[0].italic);
        assert_eq!(text_frame[0].runs[0].font_size, Some(44));
        let PptxShape::Picture { blip_rel_id, position } = &shapes[1] else { panic!("expected picture shape") };
        assert_eq!(blip_rel_id, "rId9");
        assert_eq!(*position, PptxTransform { x: 5, y: 6, cx: 7, cy: 8 });
    }

    #[test]
    fn decode_preserves_unmodeled_shape_kinds_as_other_verbatim() {
        // A `p:graphicFrame` (chart/table/SmartArt) direct child -- not typed by this layer --
        // must survive decode->encode->decode verbatim via `PptxShape::Other`.
        let mut opc = OpcPackage::empty();
        opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
        opc.content_types.set_default("xml", "application/xml");
        let graphic_frame = r#"<p:graphicFrame><p:nvGraphicFramePr><p:cNvPr id="9" name="Table 1"/></p:nvGraphicFramePr><a:graphic/></p:graphicFrame>"#;
        let slide_xml = format!(
            concat!(
                r#"<p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">"#,
                "<p:cSld><p:spTree>",
                r#"<p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr><p:grpSpPr/>"#,
                "{}",
                "</p:spTree></p:cSld></p:sld>",
            ),
            graphic_frame,
        );
        opc.set_part("ppt/slides/slide1.xml", SLIDE_CONTENT_TYPE, slide_xml.into_bytes());
        opc.add_relationship("ppt/slides/slide1.xml", "rId1", REL_TYPE_SLIDE_LAYOUT, "../slideLayouts/slideLayout1.xml");
        let presentation_xml = concat!(
            r#"<p:presentation xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">"#,
            r#"<p:sldMasterIdLst><p:sldMasterId id="2147483648" r:id="rId1"/></p:sldMasterIdLst>"#,
            r#"<p:sldIdLst><p:sldId id="256" r:id="rId2"/></p:sldIdLst>"#,
            "</p:presentation>",
        );
        opc.set_part(PRESENTATION_PART, PRESENTATION_CONTENT_TYPE, presentation_xml.as_bytes().to_vec());
        opc.add_relationship(PRESENTATION_PART, "rId1", REL_TYPE_SLIDE_MASTER, "slideMasters/slideMaster1.xml");
        opc.add_relationship(PRESENTATION_PART, "rId2", REL_TYPE_SLIDE, "slides/slide1.xml");
        opc.set_part(SLIDE_MASTER_PART, SLIDE_MASTER_CONTENT_TYPE, MINIMAL_SLIDE_MASTER_XML.as_bytes().to_vec());
        opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, PRESENTATION_PART);

        let bytes = opc::encode_opc(&opc).expect("encode");
        let decoded = decode_pptx(&bytes).expect("decode");
        assert_eq!(decoded.presentation.slides[0].shapes.len(), 1);
        let PptxShape::Other { xml } = &decoded.presentation.slides[0].shapes[0] else { panic!("expected Other shape") };
        assert!(xml.contains("p:graphicFrame") && xml.contains("Table 1"));

        // Re-encode -> re-decode: the raw xml must survive the round trip verbatim.
        let re_encoded = encode_pptx(&decoded).expect("re-encode");
        let re_decoded = decode_pptx(&re_encoded).expect("re-decode");
        assert_eq!(re_decoded.presentation, decoded.presentation);
    }

    #[test]
    fn decode_resolves_strict_office_document_relationship_too() {
        // 🏅️ A genuine ISO/IEC 29500-1 Strict package's root relationship carries
        // `REL_TYPE_OFFICE_DOCUMENT_STRICT`, never the Transitional type this engine's own writer
        // emits -- `decode_pptx`/`sniff_pptx_bytes` must still recognize it (ticket
        // 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES, so the `✳️strict` subset's
        // analyzer can ever see real Strict bytes).
        let mut opc = OpcPackage::empty();
        opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
        opc.content_types.set_default("xml", "application/xml");
        let presentation_xml = concat!(
            r#"<p:presentation xmlns:a="http://purl.oclc.org/ooxml/drawingml/main" xmlns:p="http://purl.oclc.org/ooxml/presentationml/main" xmlns:r="http://purl.oclc.org/ooxml/officeDocument/relationships">"#,
            r#"<p:sldMasterIdLst><p:sldMasterId id="2147483648" r:id="rId1"/></p:sldMasterIdLst>"#,
            r#"<p:sldIdLst/>"#,
            "</p:presentation>",
        );
        opc.set_part(PRESENTATION_PART, PRESENTATION_CONTENT_TYPE, presentation_xml.as_bytes().to_vec());
        opc.set_part(SLIDE_MASTER_PART, SLIDE_MASTER_CONTENT_TYPE, MINIMAL_SLIDE_MASTER_XML.as_bytes().to_vec());
        opc.add_relationship(PRESENTATION_PART, "rId1", REL_TYPE_SLIDE_MASTER, "slideMasters/slideMaster1.xml");
        opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT_STRICT, PRESENTATION_PART);

        let bytes = opc::encode_opc(&opc).expect("encode hand-built Strict package");
        assert!(sniff_pptx_bytes(&bytes), "Strict-relationship-typed package must still sniff as pptx");
        let decoded = decode_pptx(&bytes).expect("decode Strict-relationship-typed package");
        assert_eq!(decoded.presentation.slides.len(), 0);
    }

    #[test]
    fn decode_rejects_missing_presentation_relationship() {
        let mut opc = OpcPackage::empty();
        opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
        let bytes = opc::encode_opc(&opc).expect("encode");
        let err = decode_pptx(&bytes).expect_err("must reject a package with no officeDocument relationship");
        assert_eq!(err, PptxError::MissingPresentationRelationship);
    }

    #[test]
    fn unmodeled_slide_master_survives_decode_encode_verbatim() {
        let snap = build_minimal_pptx(sample_presentation());
        // Replace the synthesized slide master with a distinguishable "real" one before encoding.
        let mut opc = snap.opc.clone();
        opc.set_part(SLIDE_MASTER_PART, SLIDE_MASTER_CONTENT_TYPE, b"<p:sldMaster marker=\"real-file\"/>".to_vec());
        let bytes = opc::encode_opc(&opc).expect("encode");

        let decoded = decode_pptx(&bytes).expect("decode");
        assert_eq!(decoded.opc.part_bytes(SLIDE_MASTER_PART), Some(b"<p:sldMaster marker=\"real-file\"/>".as_slice()));
        let re_encoded = encode_pptx(&decoded).expect("re-encode must not clobber an already-present slide master");
        let re_decoded = decode_pptx(&re_encoded).expect("re-decode");
        assert_eq!(re_decoded.opc.part_bytes(SLIDE_MASTER_PART), Some(b"<p:sldMaster marker=\"real-file\"/>".as_slice()));
        assert_eq!(re_decoded.presentation, sample_presentation());
    }

    #[test]
    fn analyzer_builder_round_trip() {
        let original = build_minimal_pptx(sample_presentation());
        let bytes = encode_pptx(&original).expect("encode");
        let analyzed = decode_pptx(&bytes).expect("decode");
        let rebuilt = build_minimal_pptx(analyzed.presentation.clone());
        let rebuilt_bytes = encode_pptx(&rebuilt).expect("encode rebuilt");
        let reanalyzed = decode_pptx(&rebuilt_bytes).expect("decode rebuilt");
        assert_eq!(reanalyzed.presentation, analyzed.presentation);
    }

    #[test]
    fn shrinking_slide_count_drops_stale_slide_parts_and_relationships() {
        let mut wide = sample_presentation();
        let snap_wide = build_minimal_pptx(wide.clone());
        assert!(snap_wide.opc.part("ppt/slides/slide2.xml").is_some());
        assert!(!snap_wide.opc.relationships_for("ppt/slides/slide2.xml").is_empty());

        wide.slides.truncate(1);
        let bytes = encode_pptx(&PptxSnapshot::from_parts(snap_wide.opc, wide)).expect("encode narrower presentation");
        let decoded = decode_pptx(&bytes).expect("decode");
        assert!(decoded.opc.part("ppt/slides/slide2.xml").is_none(), "stale second slide part must be dropped");
        assert!(decoded.opc.relationships_for("ppt/slides/slide2.xml").is_empty(), "stale second slide's relationships must be dropped too");
        assert_eq!(decoded.presentation.slides.len(), 1);
    }

    #[test]
    fn double_regenerate_keeps_opc_parts_order_stable() {
        // 🐛 Regression: `regenerate_presentation_parts` runs TWICE in a real round trip
        // (`build_minimal_pptx` once, then `encode_pptx`/`store::ArtifactPack::encode_pack`
        // again on the ALREADY-built snapshot). The slide parts get retained-away + re-appended
        // on EVERY call, but `ppt/presentation.xml` didn't, so on the SECOND call it stayed at
        // its OLD position (before the slides, from the first call) while the slides moved to
        // the true end -- flipping their relative `opc.parts` order and breaking exact
        // `Vec<OpcPart>` equality (caught by `codec_retention_law`). Asserts the FIX: two
        // `regenerate` passes produce the IDENTICAL parts order as one.
        let snap = build_minimal_pptx(sample_presentation());
        let once = snap.opc.parts.iter().map(|p| p.path.clone()).collect::<Vec<_>>();
        let twice_bytes = encode_pptx(&snap).expect("encode (second regenerate pass)");
        let twice = decode_pptx(&twice_bytes).expect("decode").opc.parts.iter().map(|p| p.path.clone()).collect::<Vec<_>>();
        assert_eq!(once, twice, "opc.parts order must be stable across repeated regenerate passes");
        // `ppt/presentation.xml` must always sort AFTER every `ppt/slides/*` part specifically
        // (the exact symptom the bug produced).
        let pres_idx = once.iter().position(|p| p == PRESENTATION_PART).expect("presentation.xml present");
        for (i, p) in once.iter().enumerate() {
            if p.starts_with("ppt/slides/") {
                assert!(i < pres_idx, "slide part {p} must precede presentation.xml in opc.parts");
            }
        }
    }

    //#region 🔖️ConformanceLaws
    /// 🧪️ FG-wave: per-artifact conformance laws (`📖️grammar-recipe.md` §4's checklist item) --
    /// grammar/protocol parseability, `Recognizer` against real fixtures AND real `print_op`/
    /// `print_diff` output, `walk_protocol` against real `encode_pack`/`encode_op`/`encode_diff`
    /// bytes, and the fixture-honesty round-trip. Lives here (the engine's own test region), not
    /// any framework file -- same placement `📜️docx/…/⚙️engine/🦀️component.rs`'s own
    /// `conformance_laws` module uses; these tests are this artifact's OWN early-warning, plus
    /// direct coverage of the mutations/diff facets the framework's `m5` auto-discovery does not
    /// reach at all.
    mod conformance_laws {
        use super::*;
        use crate::artifacts::pptx::schema::{diff, mutations, snapshot};
        use protocol::{DiffCodec, OpBinary, OpText};

        /// ✅️ "committed files parse": all 6 handcrafted `.grammar.semio`/`.protocol.semio` files
        /// parse under the real dialect -- independent of, and cheaper than, the two
        /// `recognize`/`walk_protocol` laws below (a parse failure here fails fast with a clearer
        /// message).
        #[test]
        fn committed_facet_files_parse() {
            for (label, text) in [
                ("snapshot grammar", snapshot::text::COMPONENT_GRAMMAR_SEMIO),
                ("mutations grammar", mutations::text::COMPONENT_GRAMMAR_SEMIO),
                ("diff grammar", diff::text::COMPONENT_GRAMMAR_SEMIO),
            ] {
                let grammar = dsl::parse_grammar(text).unwrap_or_else(|e| panic!("{label}: parse_grammar failed: {e:?}"));
                assert_eq!(grammar.dialect, dsl::SemioDialect::Grammar, "{label}: expected grammar dialect");
            }
            for (label, text) in [
                ("snapshot protocol", snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                ("mutations protocol", mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                ("diff protocol", diff::binary::COMPONENT_PROTOCOL_SEMIO),
            ] {
                dsl::parse_protocol(text).unwrap_or_else(|e| panic!("{label}: parse_protocol failed: {e:?}"));
            }
        }

        /// ✅️ `grammar_conformance_law`: the snapshot grammar models the real TEXT syntax of the
        /// XML parts a pptx OPC package carries (`📸️snapshot/📝️text/📖️component.grammar.semio`'s
        /// own doc comment explains why -- this artifact's `ArtifactDsl::print_dsl` hex-dumps the
        /// WHOLE binary OPC package, matching this facet's SIBLING binary protocol, not this text
        /// grammar; the two facets describe different LAYERS of the same real artifact, same as
        /// every OPC-family member's own container/contained-parts split). So this law decodes the
        /// REAL zip entries `encode_pptx` genuinely produces (via `zip::engine::decode_zip`, the
        /// same real codec `opc::decode_opc` itself delegates to) and recognizes EACH real
        /// modeled part's own text against the grammar -- direct proof the grammar matches this
        /// artifact's own real per-part XML bytes, not an invented approximation.
        #[test]
        fn grammar_conformance_law() {
            let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);

            let demo = demo_pptx_snapshot();
            let bytes = encode_pptx(&demo).expect("encode demo pptx");
            let zip = crate::artifacts::zip::engine::decode_zip(&bytes).expect("decode zip");

            let fixed_parts = ["[Content_Types].xml", "_rels/.rels", "ppt/presentation.xml"];
            let mut checked = 0;
            for entry in &zip.entries {
                let is_slide = entry.name.starts_with("ppt/slides/slide") && entry.name.ends_with(".xml");
                if !fixed_parts.contains(&entry.name.as_str()) && !is_slide {
                    continue;
                }
                let text = String::from_utf8(entry.data.clone()).unwrap_or_else(|e| panic!("part {:?}: not valid utf-8: {e}", entry.name));
                assert!(recognizer.recognize(&text).unwrap_or(false), "grammar did not recognize real part {:?}:\n{text}", entry.name);
                checked += 1;
            }
            assert_eq!(checked, fixed_parts.len() + demo.presentation.slides.len(), "not every modeled part was present in the real zip entries");
        }

        /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op`
        /// output for every `PptxMutation` variant (`mutations::demo_mutation_cases()`).
        #[test]
        fn ops_grammar_conformance_law() {
            let grammar = dsl::parse_grammar(mutations::text::COMPONENT_GRAMMAR_SEMIO).expect("parse mutations grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            for mutation in mutations::demo_mutation_cases() {
                let printed = mutation.print_op();
                assert!(recognizer.recognize(&printed).unwrap_or(false), "mutations grammar did not recognize {printed:?} (from {mutation:?})");
            }
        }

        /// ✅️ `diff_grammar_conformance_law`: the diff grammar recognizes real `print_diff` output
        /// for every representative `PptxDiff` (`diff::demo_diff_cases()`).
        #[test]
        fn diff_grammar_conformance_law() {
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
        #[test]
        fn protocol_walk_law() {
            let pack_spec = dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
            let demo = demo_pptx_snapshot();
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

        #[test]
        #[ignore]
        fn zzz_generate_p2p1_fixtures() {
            let demo = demo_pptx_snapshot();
            let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🗿️artifacts/🎞️pptx/📚️examples/🎬️demo/🖼️assets");
            std::fs::write(dir.join("🗣️example.dsl.semio"), store::ArtifactDsl::print_dsl(&demo)).unwrap();
            std::fs::write(dir.join("🎒️example.pack.semio"), store::ArtifactPack::encode_pack(&demo)).unwrap();
        }

        /// ✅️ `fixture_honesty_law`: the shipped `.dsl.semio`/`.pack.semio` fixtures are GENUINE
        /// `print_dsl`/`encode_pack` output of `demo_pptx_snapshot()` -- `parse_dsl(fixture) ==
        /// demo()`, `print_dsl(demo()) == fixture` (byte-for-byte), and the pack twin -- so the
        /// fixtures can never silently drift back to a fake `"68656c6c6f"`-style placeholder again
        /// (see this ticket's own recon note on the pre-FG-wave state of these two files).
        #[test]
        fn fixture_honesty_law() {
            const FIXTURE_DSL: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");
            const FIXTURE_PACK: &[u8] = include_bytes!("../../../📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio");

            let demo = demo_pptx_snapshot();

            let parsed = <PptxSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
            assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_pptx_snapshot()");
            assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_pptx_snapshot()) drifted from the shipped .dsl.semio fixture");

            let decoded = <PptxSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
            assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_pptx_snapshot()");
            assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_pptx_snapshot()) drifted from the shipped .pack.semio fixture");
        }
    }
    //#endregion 🔖️ConformanceLaws
}
//#endregion 🧪️Tests
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, composer_entry_of};
    use crate::artifacts::pptx::standards::v_ecma_376::subsets::any::schema::PptxComposer as PptxRawAnyComposer;
    use crate::artifacts::pptx::standards::v_ecma_376::subsets::strict::schema::PptxStrictComposer;
    use crate::artifacts::pptx::standards::v_ecma_376::subsets::transitional::schema::PptxTransitionalComposer;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<PptxRawAnyComposer>(), composer_entry_of::<PptxStrictComposer>(), composer_entry_of::<PptxTransitionalComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
