//! 📥️ PDF byte reader of `stdio.pdf` (1.7/🧱️base): sniff → cross-reference → decrypt → retained graph → typed lanes,
//! leniently for PDF 1.0–2.0 (`declared_version` records the header verbatim), plus page text extraction through the
//! page fonts. Depends on the model and the COS engine modules only.

use crate::standards::v1_7::subsets::base::modules::encryption::open_standard_security;
use crate::standards::v1_7::subsets::base::modules::fonts::FontCodec;
use crate::standards::v1_7::subsets::base::modules::lexer::{dict_get, pdf_header, CosGrant};
use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::modules::filters::declares_logical_codec;
use crate::standards::v1_7::subsets::base::io::foreign_artifacts;
use crate::standards::v1_7::subsets::base::io::error::{PResult, PdfEngineError};
use crate::standards::v1_7::subsets::base::modules::lift::lift_document_with;
use crate::standards::v1_7::subsets::base::modules::xref::{normalize_pdf_object, read_file_xref, ResolvedGraph, Resolver, XrefEntry, WRITER_TRAILER_KEYS};
use crate::standards::v1_7::subsets::base::schema::graph_source::GraphSource;
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use std::collections::HashMap;

#[path = "../../🔨️modules/🔗️xref/📥️retained-graph/🦀️.rs"]
pub mod retained_graph;
use retained_graph::{PdfByteGraph, PdfByteGraphCursor, PdfByteGraphEvent};

#[path = "🗃️original-custody/🦀️.rs"]
pub mod original_custody;

#[path = "📖️interpreted-streams/🦀️.rs"]
pub mod interpreted_streams;

#[path = "🔁️controlled/🦀️.rs"]
mod controlled;
pub use controlled::{admit_pdf_natural_decode, admit_pdf_paged_natural_decode, build_pdf_natural_decode, PDF_PAGED_NATURAL_SOURCE_MAXIMUM_BYTES};


//#region 🔖️Sniff
/// 🔍️ Real magic + version probe: `%PDF-` header, version digits parsed and reported.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn sniff_pdf(bytes: &[u8]) -> Option<String> {
    pdf_header(bytes).map(|header| header.version().to_owned())
}
//#endregion 🔖️Sniff
//#region 🔖️Decode
/// 📥️ Decodes a file with the empty user password.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_pdf(data: &[u8]) -> PResult<PdfSnapshot> {
    decode_pdf_with_password(data, "")
}
/// 📥️ Decodes a file, opening the standard security handler with `password` when it is
/// encrypted (`Unsupported` when the password does not open it or the handler is not the
/// standard one — never garbage).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_pdf_with_password(data: &[u8], password: &str) -> PResult<PdfSnapshot> {
    let OpenedPdf { declared_version, graph: ResolvedGraph { mut objects, positions }, mut trailer, encryption } =
        match read_clear_pdf_graph(data)? {
            Some(file) => OpenedPdf {
                declared_version: file.header.version().to_owned(),
                graph: ResolvedGraph { objects: file.graph.objects, positions: file.graph.positions },
                trailer: file.trailer.into_entries(),
                encryption: None,
            },
            None => read_pdf_reference(data, password)?,
        };
    trailer.retain(|entry| !WRITER_TRAILER_KEYS.contains(&entry.key.as_str()));
    let Some(root) = dict_get(&trailer, "Root").and_then(PdfObject::as_ref) else {
        return Err(PdfEngineError::Malformed("no /Root in any trailer and no /Catalog object to recover one from".into()));
    };
    let position = |reference: ObjRef| match positions.get(&reference.num) {
        Some(XrefEntry::Normal { offset, gen }) if *gen == reference.gen => Some(*offset),
        _ => None,
    };
    let Some(catalog) = position(root).and_then(|index| objects.get(index)) else {
        return Err(PdfEngineError::Malformed("PDF /Root reference does not resolve to a live object".into()));
    };
    if catalog.value.dict_get("Type").and_then(PdfObject::as_name) != Some("Catalog") {
        return Err(PdfEngineError::Malformed("PDF /Root does not identify a catalog".into()));
    }
    crate::standards::v1_7::subsets::base::io::text::snapshot::retained_text::admit_retained_text(&mut objects, &mut trailer, &position).map_err(|reason| PdfEngineError::Malformed(reason.into()))?;
    interpreted_streams::admit_interpreted_streams(&mut objects, &trailer, &position).map_err(|reason| PdfEngineError::Malformed(reason.into()))?;
    let mut source = GraphSource::with_positions(&objects, &position);
    let lifter = lift_document_with(&trailer, &declared_version, &mut source);
    if let Some(error)=lifter.admission_error {return Err(error);}
    let mut snapshot = lifter.snapshot;
    snapshot.schema = STDIO_PDF17_DOCUMENT_SCHEMA.into();
    snapshot.declared_version = declared_version;
    snapshot.encryption = encryption;
    if !snapshot.objects.is_empty() {
        let mut number = objects.iter().map(|object| object.id.num).max().unwrap_or(0);
        for mut object in std::mem::take(&mut snapshot.objects) {
            if objects.iter().any(|existing| existing.value == object.value) { continue; }
            number = number.checked_add(1).ok_or_else(|| PdfEngineError::Unsupported("PDF native custody exhausted".into()))?;
            object.id.num = number;
            objects.push(object);
        }
    }
    snapshot.objects = objects;
    snapshot.trailer = trailer;
    Ok(snapshot)
}
struct OpenedPdf {
    declared_version: String,
    graph: ResolvedGraph,
    trailer: Vec<PdfDictEntry>,
    encryption: Option<PdfEncryption>,
}

fn read_clear_pdf_graph(data: &[u8]) -> PResult<Option<PdfByteGraph>> {
    let grant = CosGrant { work: 4096, allocation_bytes: usize::MAX, release_bytes: usize::MAX };
    let mut cursor = PdfByteGraphCursor::new(usize::MAX, usize::MAX);
    let result = loop {
        match cursor.advance(data, grant, false).event {
            Ok(PdfByteGraphEvent::Object | PdfByteGraphEvent::More) => {}
            Ok(PdfByteGraphEvent::Complete) => {
                let (mut file, _) = cursor.take(grant).expect("completed PDF byte graph under unbounded reference grants");
                for object in &mut file.graph.objects {
                    if matches!(&object.value, PdfObject::Stream { dict, filters, .. }
                        if filters.is_empty()
                            && dict_get(dict, "Subtype").and_then(PdfObject::as_name) == Some("Image")
                            && dict.iter().any(|entry| matches!(entry.key.as_str(), "Filter" | "F"))
                            && !declares_logical_codec(dict)) {
                        object.value = normalize_pdf_object(std::mem::replace(&mut object.value, PdfObject::Null))?;
                    }
                }
                return Ok(Some(file));
            }
            Ok(PdfByteGraphEvent::NotPdf) => break Err(PdfEngineError::NotPdf),
            _ => break Ok(None),
        }
    };
    cursor.begin_close();
    while matches!(cursor.close_step(grant).event, Ok(PdfByteGraphEvent::More)) {}
    result
}

fn read_pdf_reference(data: &[u8], password: &str) -> PResult<OpenedPdf> {
    let (header, origin, xref) = read_file_xref(data)?;
    let declared_version = header.version().to_owned();
    let data = &data[origin..];
    let mut resolver = Resolver::new(data, xref.entries);
    let mut encryption = None;
    let mut encrypt_object: Option<u32> = None;
    if let Some(encrypt) = dict_get(&xref.trailer, "Encrypt") {
        encrypt_object = encrypt.as_ref().map(|reference| reference.num);
        let dictionary = match encrypt {
            PdfObject::Ref(reference) => resolver.resolve_reference(*reference).ok_or_else(|| PdfEngineError::Malformed("/Encrypt reference does not resolve".into()))?,
            other => other.clone(),
        };
        let dictionary = dictionary.as_dict().ok_or_else(|| PdfEngineError::Malformed("/Encrypt is not a dictionary".into()))?.to_vec();
        let document_id = dict_get(&xref.trailer, "ID").and_then(PdfObject::as_array).and_then(|items| items.first()).and_then(PdfObject::as_str_bytes).unwrap_or(&[]).to_vec();
        let (decryptor, parameters) = open_standard_security(&dictionary, &document_id, password)?;
        resolver.set_decryptor(Some(decryptor));
        encryption = Some(parameters);
    }
    let graph = resolver.resolve_all(encrypt_object)?;
    Ok(OpenedPdf { declared_version, graph, trailer: xref.trailer, encryption })
}
//#endregion 🔖️Decode

//#region 🔖️Text
/// 🔤 The Unicode text page `index` of `snapshot` shows, raw codes decoded through the page's fonts.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn page_text(snapshot: &PdfSnapshot, index: usize) -> PResult<String> {
    let Some(page) = snapshot.pages.get(index) else { return Ok(String::new()) };
    let artifacts = foreign_artifacts::NativePdfArtifactResources::from_objects(&snapshot.objects);
    let codecs: HashMap<String, FontCodec> = snapshot.fonts.iter().map(|font| Ok((font.id.clone(), FontCodec::new(font, &artifacts)?))).collect::<PResult<_>>()?;
    Ok(crate::standards::v1_7::subsets::base::modules::content::extract_text(&page.content, &codecs))
}
//#endregion 🔖️Text

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn document_root_must_identify_a_catalog_after_retained_resolution() {
        let fixture = include_bytes!("../../🧪️tests/🧫️fixtures/📦️object-stream/🧪️compressed.pdf");
        let mut bytes = fixture.to_vec();
        let from = fixture.windows(8).position(|window| window == b"/Catalog").unwrap();
        bytes[from..from + 8].copy_from_slice(b"/BadType");
        assert!(matches!(decode_pdf(&bytes), Err(PdfEngineError::Malformed(reason)) if reason.contains("catalog")));
    }
}
