//! ⚙️ WordprocessingML (docx) engine — real OPC container + `word/document.xml` paragraph/run
//! model. Zip/OPC/XML byte-level work is never reimplemented here: it is reused from the shared
//! `crate::artifacts::zip::opc` layer and, transitively, `crate::artifacts::zip::engine` +
//! `crate::artifacts::xml::schema::snapshot`.

use crate::artifacts::docx::{schema::snapshot::{DocxDocument, DocxParagraph, DocxRun}, DocxArtifact, DocxDiff, DocxMutation, DocxSnapshot, STDIO_DOCX_DOCUMENT_SCHEMA};
use crate::artifacts::xml::schema::snapshot::{xml_document_from_text, xml_document_to_text, XmlAttr, XmlDocument, XmlNode};
use crate::artifacts::zip::opc::{self, OpcPackage, REL_TYPE_OFFICE_DOCUMENT, RELS_CONTENT_TYPE};

//#region 🔖️Error
/// ⚠️ Typed docx decode/encode failure — a package this engine cannot honestly interpret is
/// never fabricated into a partial/empty document.
#[derive(Clone, Debug, PartialEq)]
pub enum DocxError {
    Opc(opc::OpcError),
    MissingMainDocumentRelationship,
    MissingPart(String),
    Xml { part: String, detail: String },
    Malformed(String),
}

impl std::fmt::Display for DocxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Opc(e) => write!(f, "docx: {e}"),
            Self::MissingMainDocumentRelationship => write!(f, "docx: package root has no officeDocument relationship"),
            Self::MissingPart(p) => write!(f, "docx: missing required part {p}"),
            Self::Xml { part, detail } => write!(f, "docx: xml in {part}: {detail}"),
            Self::Malformed(detail) => write!(f, "docx: {detail}"),
        }
    }
}

impl std::error::Error for DocxError {}

impl From<opc::OpcError> for DocxError {
    fn from(e: opc::OpcError) -> Self { Self::Opc(e) }
}
//#endregion 🔖️Error

//#region 🔖️Constants
const W_NS: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
const R_NS: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
const MAIN_DOCUMENT_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml";
const MAIN_DOCUMENT_PART: &str = "word/document.xml";
//#endregion 🔖️Constants

//#region 🔖️XmlMapping
fn document_to_xml(doc: &DocxDocument) -> XmlDocument {
    let body_children = doc
        .paragraphs
        .iter()
        .map(|p| {
            let run_children = p
                .runs
                .iter()
                .map(|r| {
                    let mut rc = Vec::new();
                    if r.bold || r.italic {
                        let mut rpr = Vec::new();
                        if r.bold {
                            rpr.push(XmlNode::Element { name: "w:b".into(), attrs: vec![], children: vec![] });
                        }
                        if r.italic {
                            rpr.push(XmlNode::Element { name: "w:i".into(), attrs: vec![], children: vec![] });
                        }
                        rc.push(XmlNode::Element { name: "w:rPr".into(), attrs: vec![], children: rpr });
                    }
                    rc.push(XmlNode::Element {
                        name: "w:t".into(),
                        attrs: vec![XmlAttr { name: "xml:space".into(), value: "preserve".into() }],
                        children: vec![XmlNode::Text { text: r.text.clone() }],
                    });
                    XmlNode::Element { name: "w:r".into(), attrs: vec![], children: rc }
                })
                .collect();
            XmlNode::Element { name: "w:p".into(), attrs: vec![], children: run_children }
        })
        .collect();
    XmlDocument {
        root: Some(XmlNode::Element {
            name: "w:document".into(),
            attrs: vec![XmlAttr { name: "xmlns:w".into(), value: W_NS.into() }],
            children: vec![XmlNode::Element { name: "w:body".into(), attrs: vec![], children: body_children }],
        }),
        doctype: None,
        declaration: None,
    }
}

fn document_from_xml(doc: &XmlDocument) -> Result<DocxDocument, DocxError> {
    let bad = |detail: &str| DocxError::Xml { part: MAIN_DOCUMENT_PART.into(), detail: detail.into() };
    let root = doc.root.as_ref().ok_or_else(|| bad("empty document"))?;
    let XmlNode::Element { name, children, .. } = root else { return Err(bad("root is not an element")) };
    if name != "w:document" {
        return Err(DocxError::Xml { part: MAIN_DOCUMENT_PART.into(), detail: format!("expected <w:document>, got <{name}>") });
    }
    let body = children
        .iter()
        .find_map(|c| match c {
            XmlNode::Element { name, children, .. } if name == "w:body" => Some(children),
            _ => None,
        })
        .ok_or_else(|| bad("missing <w:body>"))?;

    let mut paragraphs = Vec::new();
    for node in body {
        let XmlNode::Element { name, children: p_children, .. } = node else { continue };
        if name != "w:p" {
            continue;
        }
        let mut runs = Vec::new();
        for rn in p_children {
            let XmlNode::Element { name, children: r_children, .. } = rn else { continue };
            if name != "w:r" {
                continue;
            }
            let mut bold = false;
            let mut italic = false;
            let mut text = String::new();
            for rc in r_children {
                let XmlNode::Element { name, children: inner, .. } = rc else { continue };
                match name.as_str() {
                    "w:rPr" => {
                        for prop in inner {
                            if let XmlNode::Element { name, .. } = prop {
                                match name.as_str() {
                                    "w:b" => bold = true,
                                    "w:i" => italic = true,
                                    _ => {}
                                }
                            }
                        }
                    }
                    "w:t" => {
                        for t in inner {
                            if let XmlNode::Text { text: t } = t {
                                text.push_str(t);
                            }
                        }
                    }
                    _ => {}
                }
            }
            runs.push(DocxRun { text, bold, italic });
        }
        paragraphs.push(DocxParagraph { runs });
    }
    Ok(DocxDocument { paragraphs })
}
//#endregion 🔖️XmlMapping

//#region 🔖️Codec
/// 🏗️ Assembles a brand-new, minimal-but-valid OPC package around `document` — correct
/// `[Content_Types].xml`, a root `_rels/.rels` pointing at `word/document.xml`, and the
/// serialized part itself. Real Office/LibreOffice-shaped readers accept this container.
pub fn build_minimal_docx(document: DocxDocument) -> DocxSnapshot {
    let mut opc = OpcPackage::empty();
    opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
    opc.content_types.set_default("xml", "application/xml");
    let bytes = xml_document_to_text(&document_to_xml(&document)).into_bytes();
    opc.set_part(MAIN_DOCUMENT_PART, MAIN_DOCUMENT_CONTENT_TYPE, bytes);
    opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, MAIN_DOCUMENT_PART);
    DocxSnapshot::from_parts(opc, document)
}

pub fn encode_docx(snap: &DocxSnapshot) -> Result<Vec<u8>, DocxError> {
    let mut opc = snap.opc.clone();
    let bytes = xml_document_to_text(&document_to_xml(&snap.document)).into_bytes();
    let content_type = opc.content_types.resolve(MAIN_DOCUMENT_PART).map(str::to_string).unwrap_or_else(|| MAIN_DOCUMENT_CONTENT_TYPE.into());
    opc.set_part(MAIN_DOCUMENT_PART, &content_type, bytes);
    if opc.relationships_for("").iter().all(|r| r.rel_type != REL_TYPE_OFFICE_DOCUMENT) {
        opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, MAIN_DOCUMENT_PART);
    }
    Ok(opc::encode_opc(&opc)?)
}

pub fn decode_docx(data: &[u8]) -> Result<DocxSnapshot, DocxError> {
    let opc = opc::decode_opc(data)?;
    let main_path = opc
        .resolve_relationship("", REL_TYPE_OFFICE_DOCUMENT)
        .ok_or(DocxError::MissingMainDocumentRelationship)?;
    let bytes = opc.part_bytes(&main_path).ok_or_else(|| DocxError::MissingPart(main_path.clone()))?;
    let text = String::from_utf8(bytes.to_vec()).map_err(|_| DocxError::Xml { part: main_path.clone(), detail: "not valid utf-8".into() })?;
    let xml = xml_document_from_text(&text).map_err(|e| DocxError::Xml { part: main_path.clone(), detail: e })?;
    let document = document_from_xml(&xml)?;
    let _ = R_NS; // documented namespace constant, not independently emitted (w: prefix carries relationship refs when needed)
    Ok(DocxSnapshot::from_parts(opc, document))
}

pub fn empty_docx_snapshot() -> DocxSnapshot { DocxSnapshot::default() }

pub fn register() {
    crate::artifacts::docx::composer::register();
    ::schema::register_artifact_schema_descriptor(crate::artifacts::docx::schema::docx_artifact_schema_descriptor());
    store::register_document_codec(store::ArtifactCodec::of::<DocxSnapshot, DocxMutation>(STDIO_DOCX_DOCUMENT_SCHEMA));
}
//#endregion 🔖️Codec

//#region 🔖️Sniff
/// 🕵️ Real docx sniff: OPC-shaped (real `[Content_Types].xml`) *and* the root officeDocument
/// relationship resolves to a part under `word/` — disambiguates from xlsx/pptx, which share the
/// same zip magic and OPC shape but point at `xl/`/`ppt/` instead.
pub fn sniff_docx_bytes(data: &[u8]) -> bool {
    let Ok(opc) = opc::decode_opc(data) else { return false };
    match opc.resolve_relationship("", REL_TYPE_OFFICE_DOCUMENT) {
        Some(path) => path.starts_with("word/"),
        None => false,
    }
}
//#endregion 🔖️Sniff

//#region 🔖️ArtifactEngine
pub struct DocxEngine { artifact_state: DocxArtifact, snapshot_state: DocxSnapshot }
impl DocxEngine {
    pub fn new(snapshot: DocxSnapshot) -> Self {
        Self { artifact_state: DocxArtifact::from_snapshot(snapshot.clone()), snapshot_state: snapshot }
    }
}
//#endregion 🔖️ArtifactEngine

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn sample_document() -> DocxDocument {
        DocxDocument {
            paragraphs: vec![
                DocxParagraph { runs: vec![DocxRun { text: "Hello, ".into(), bold: true, italic: false }, DocxRun { text: "world!".into(), bold: false, italic: true }] },
                DocxParagraph::text("Second paragraph, plain."),
            ],
        }
    }

    #[test]
    fn builder_produces_minimal_valid_package_that_decodes_back() {
        let snap = build_minimal_docx(sample_document());
        let bytes = encode_docx(&snap).expect("encode minimal package");
        assert!(opc::sniff_opc_bytes(&bytes));
        assert!(sniff_docx_bytes(&bytes));
        let decoded = decode_docx(&bytes).expect("decode minimal package");
        assert_eq!(decoded.document, sample_document());
    }

    #[test]
    fn decode_resolves_real_hand_built_package_with_formatting() {
        // Hand-built OOXML: correct Content_Types/.rels/part structure, not just "a zip with xml".
        let mut opc = OpcPackage::empty();
        opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
        opc.content_types.set_default("xml", "application/xml");
        let xml = concat!(
            r#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">"#,
            "<w:body>",
            r#"<w:p><w:r><w:rPr><w:b/></w:rPr><w:t xml:space="preserve">Bold run</w:t></w:r></w:p>"#,
            r#"<w:p><w:r><w:rPr><w:i/></w:rPr><w:t xml:space="preserve">Italic run</w:t></w:r></w:p>"#,
            r#"<w:p><w:r><w:t xml:space="preserve">Plain &amp; escaped</w:t></w:r></w:p>"#,
            "</w:body>",
            "</w:document>",
        );
        opc.set_part(MAIN_DOCUMENT_PART, MAIN_DOCUMENT_CONTENT_TYPE, xml.as_bytes().to_vec());
        opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, MAIN_DOCUMENT_PART);
        let bytes = crate::artifacts::zip::opc::encode_opc(&opc).expect("encode opc");

        let decoded = decode_docx(&bytes).expect("decode hand-built docx");
        assert_eq!(decoded.document.paragraphs.len(), 3);
        assert!(decoded.document.paragraphs[0].runs[0].bold);
        assert!(decoded.document.paragraphs[1].runs[0].italic);
        assert_eq!(decoded.document.paragraphs[2].runs[0].text, "Plain & escaped");
    }

    #[test]
    fn unmodeled_parts_survive_decode_encode_verbatim() {
        let mut opc = OpcPackage::empty();
        opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
        opc.content_types.set_default("xml", "application/xml");
        opc.set_part(MAIN_DOCUMENT_PART, MAIN_DOCUMENT_CONTENT_TYPE, xml_document_to_text(&document_to_xml(&sample_document())).into_bytes());
        opc.set_part("word/styles.xml", "application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml", b"<w:styles/>".to_vec());
        opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, MAIN_DOCUMENT_PART);
        let bytes = crate::artifacts::zip::opc::encode_opc(&opc).expect("encode");

        let decoded = decode_docx(&bytes).expect("decode");
        assert_eq!(decoded.opc.part_bytes("word/styles.xml"), Some(b"<w:styles/>".as_slice()));
        let re_encoded = encode_docx(&decoded).expect("re-encode");
        let re_decoded = decode_docx(&re_encoded).expect("re-decode");
        assert_eq!(re_decoded.opc.part_bytes("word/styles.xml"), Some(b"<w:styles/>".as_slice()));
        assert_eq!(re_decoded.document, sample_document());
    }

    #[test]
    fn decode_rejects_missing_main_document_relationship() {
        let mut opc = OpcPackage::empty();
        opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
        let bytes = crate::artifacts::zip::opc::encode_opc(&opc).expect("encode");
        let err = decode_docx(&bytes).expect_err("must reject a package with no officeDocument relationship");
        assert_eq!(err, DocxError::MissingMainDocumentRelationship);
    }

    #[test]
    fn analyzer_builder_round_trip() {
        let original = build_minimal_docx(sample_document());
        // Analyzer: real decode of the encoded bytes.
        let bytes = encode_docx(&original).expect("encode");
        let analyzed = decode_docx(&bytes).expect("decode");
        // Builder: reconstruct an equivalent document from the analyzed parts.
        let rebuilt = build_minimal_docx(analyzed.document.clone());
        let rebuilt_bytes = encode_docx(&rebuilt).expect("encode rebuilt");
        let reanalyzed = decode_docx(&rebuilt_bytes).expect("decode rebuilt");
        assert_eq!(reanalyzed.document, analyzed.document);
    }
}
//#endregion 🧪️Tests
