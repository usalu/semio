//! ⚙️ PresentationML (pptx) engine — real OPC container + slide-list/text model. Zip/OPC/XML
//! byte-level work is never reimplemented here: it is reused from the shared
//! `crate::artifacts::zip::opc` layer. `ppt/slideMasters`/`ppt/slideLayouts`/`ppt/theme` are
//! unmodeled boilerplate every real reader still needs to open the package validly: they are
//! synthesized once (fixed minimal-but-schema-shaped constants) when building a package from
//! scratch, and left verbatim (untouched) whenever they already exist in a decoded package.

use crate::artifacts::pptx::{schema::snapshot::{PptxParagraph, PptxPresentation, PptxRun, PptxSlide}, PptxArtifact, PptxDiff, PptxMutation, PptxSnapshot, STDIO_PPTX_DOCUMENT_SCHEMA};
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

fn attr(name: &str, value: &str) -> XmlAttr {
    XmlAttr { name: name.into(), value: value.into() }
}

fn attr_val<'a>(attrs: &'a [XmlAttr], name: &str) -> Option<&'a str> {
    attrs.iter().find(|a| a.name == name).map(|a| a.value.as_str())
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

//#region 🔖️SlideXml
fn run_to_xml(run: &PptxRun) -> XmlNode {
    let mut children = Vec::new();
    if run.bold || run.italic {
        let mut attrs = Vec::new();
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

fn slide_to_xml(slide: &PptxSlide) -> XmlDocument {
    let mut text_body_children = vec![XmlNode::Element { name: "a:bodyPr".into(), attrs: vec![], children: vec![] }];
    text_body_children.extend(slide.paragraphs.iter().map(paragraph_to_xml));

    let sp = XmlNode::Element {
        name: "p:sp".into(),
        attrs: vec![],
        children: vec![
            XmlNode::Element {
                name: "p:nvSpPr".into(),
                attrs: vec![],
                children: vec![
                    XmlNode::Element { name: "p:cNvPr".into(), attrs: vec![attr("id", "2"), attr("name", "TextBox 1")], children: vec![] },
                    XmlNode::Element { name: "p:cNvSpPr".into(), attrs: vec![attr("txBox", "1")], children: vec![] },
                    XmlNode::Element { name: "p:nvPr".into(), attrs: vec![], children: vec![] },
                ],
            },
            XmlNode::Element { name: "p:spPr".into(), attrs: vec![], children: vec![] },
            XmlNode::Element { name: "p:txBody".into(), attrs: vec![], children: text_body_children },
        ],
    };

    XmlDocument {
        root: Some(XmlNode::Element {
            name: "p:sld".into(),
            attrs: vec![attr("xmlns:a", A_NS), attr("xmlns:p", P_NS)],
            children: vec![XmlNode::Element {
                name: "p:cSld".into(),
                attrs: vec![],
                children: vec![XmlNode::Element {
                    name: "p:spTree".into(),
                    attrs: vec![],
                    children: vec![
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
                        sp,
                    ],
                }],
            }],
        }),
        doctype: None,
    }
}

fn paragraph_from_xml(node: &XmlNode) -> PptxParagraph {
    let mut runs = Vec::new();
    if let XmlNode::Element { children, .. } = node {
        for c in children {
            let XmlNode::Element { name, children: r_children, .. } = c else { continue };
            if name != "a:r" {
                continue;
            }
            let mut bold = false;
            let mut italic = false;
            let mut text = String::new();
            for rc in r_children {
                let XmlNode::Element { name, attrs, children: inner } = rc else { continue };
                match name.as_str() {
                    "a:rPr" => {
                        if let Some(b) = attr_val(attrs, "b") {
                            bold = b == "1";
                        }
                        if let Some(i) = attr_val(attrs, "i") {
                            italic = i == "1";
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
            runs.push(PptxRun { text, bold, italic });
        }
    }
    PptxParagraph { runs }
}

/// 🔎️ Recursively collects every `p:txBody`'s paragraphs, in document order, across the whole
/// shape tree (shapes may nest inside `p:grpSp` groups) — "shape -> text body -> paragraphs" per
/// the D2 plan, not just the first shape found.
fn collect_paragraphs(node: &XmlNode, out: &mut Vec<PptxParagraph>) {
    let XmlNode::Element { name, children, .. } = node else { return };
    if name == "p:txBody" {
        for c in children {
            if let XmlNode::Element { name, .. } = c {
                if name == "a:p" {
                    out.push(paragraph_from_xml(c));
                }
            }
        }
    } else {
        for c in children {
            collect_paragraphs(c, out);
        }
    }
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
    opc.parts.retain(|p| !p.path.starts_with("ppt/slides/"));
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
    let presentation_path = opc.resolve_relationship("", REL_TYPE_OFFICE_DOCUMENT).ok_or(PptxError::MissingPresentationRelationship)?;
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
        let mut paragraphs = Vec::new();
        if let Some(root) = &slide_xml.root {
            collect_paragraphs(root, &mut paragraphs);
        }
        slides.push(PptxSlide { paragraphs });
    }

    Ok(PptxSnapshot::from_parts(opc, PptxPresentation { slides }))
}

pub fn empty_pptx_snapshot() -> PptxSnapshot { PptxSnapshot::default() }

pub fn register() {
    crate::artifacts::pptx::composer::register();
    ::schema::register_artifact_schema_descriptor(crate::artifacts::pptx::schema::pptx_artifact_schema_descriptor());
    store::register_document_codec(store::ArtifactCodec::of::<PptxSnapshot, PptxMutation>(STDIO_PPTX_DOCUMENT_SCHEMA));
}
//#endregion 🔖️Codec

//#region 🔖️Sniff
/// 🕵️ Real pptx sniff: OPC-shaped bytes whose root officeDocument relationship resolves under
/// `ppt/` — disambiguates from docx/xlsx sharing the same zip magic and OPC shape.
pub fn sniff_pptx_bytes(data: &[u8]) -> bool {
    let Ok(opc) = opc::decode_opc(data) else { return false };
    match opc.resolve_relationship("", REL_TYPE_OFFICE_DOCUMENT) {
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
impl protocol::ArtifactEngine for PptxEngine {
    type Artifact = PptxArtifact; type Snapshot = PptxSnapshot; type Mutation = PptxMutation; type Diff = PptxDiff;
    fn artifact(&self) -> &Self::Artifact { &self.artifact_state }
    fn snapshot(&self) -> &Self::Snapshot { &self.snapshot_state }
    fn apply(&mut self, mutation: &Self::Mutation) -> Result<Self::Diff, protocol::EngineFault> {
        let diff = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(mutation, &self.snapshot_state);
        self.snapshot_state = <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(&diff, &self.snapshot_state);
        self.artifact_state.set_snapshot(self.snapshot_state.clone());
        Ok(diff)
    }
    fn inverse(&self, mutation: &Self::Mutation) -> Vec<Self::Mutation> {
        <Self::Mutation as protocol::Mutation<Self::Snapshot>>::inverse(mutation, &self.snapshot_state)
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
                PptxSlide { paragraphs: vec![PptxParagraph { runs: vec![PptxRun { text: "Title Slide".into(), bold: true, italic: false }] }] },
                PptxSlide { paragraphs: vec![PptxParagraph::text("Second slide, plain."), PptxParagraph { runs: vec![PptxRun { text: "italic note".into(), bold: false, italic: true }] }] },
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
    fn decode_resolves_real_hand_built_package_with_nested_group_shapes() {
        // Hand-built OOXML: real presentation.xml/rels + slide with a shape nested inside a
        // group (p:grpSp) — exercises the recursive shape-tree walk, not just the first shape.
        let mut opc = OpcPackage::empty();
        opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
        opc.content_types.set_default("xml", "application/xml");

        let slide_xml = concat!(
            r#"<p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">"#,
            "<p:cSld><p:spTree>",
            r#"<p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr><p:grpSpPr/>"#,
            "<p:grpSp><p:nvGrpSpPr><p:cNvPr id=\"3\" name=\"Group\"/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr><p:grpSpPr/>",
            "<p:sp><p:nvSpPr><p:cNvPr id=\"4\" name=\"Nested\"/><p:cNvSpPr txBox=\"1\"/><p:nvPr/></p:nvSpPr><p:spPr/>",
            r#"<p:txBody><a:bodyPr/><a:p><a:r><a:rPr b="1" i="1"/><a:t>Nested &amp; bold-italic</a:t></a:r></a:p></p:txBody>"#,
            "</p:sp></p:grpSp>",
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
        let paragraphs = &decoded.presentation.slides[0].paragraphs;
        assert_eq!(paragraphs.len(), 1);
        assert_eq!(paragraphs[0].runs[0].text, "Nested & bold-italic");
        assert!(paragraphs[0].runs[0].bold);
        assert!(paragraphs[0].runs[0].italic);
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
}
//#endregion 🧪️Tests
