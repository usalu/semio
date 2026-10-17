//! 📥️ PDF byte reader of `stdio.pdf` (1.7/🧱️base): sniff → cross-reference → decrypt → retained graph → typed lanes,
//! leniently for PDF 1.0–2.0 (`declared_version` records the header verbatim), plus page text extraction through the
//! page fonts. Depends on the model and the COS engine modules only.

use crate::standards::v1_7::subsets::base::modules::encryption::open_standard_security;
use crate::standards::v1_7::subsets::base::modules::fonts::FontCodec;
use crate::standards::v1_7::subsets::base::modules::lexer::dict_get;
use crate::standards::v1_7::subsets::base::io::foreign_artifacts;
use crate::standards::v1_7::subsets::base::io::error::{PResult, PdfEngineError};
use crate::standards::v1_7::subsets::base::modules::lift::lift_document_with;
use crate::standards::v1_7::subsets::base::modules::xref::{build_xref, startxref_offset, Resolver, WRITER_TRAILER_KEYS};
use crate::standards::v1_7::subsets::base::schema::graph_source::GraphSource;
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use std::collections::HashMap;


//#region 🔖️Sniff
/// 🔍️ Real magic + version probe: `%PDF-` header, version digits parsed and reported.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn sniff_pdf(bytes: &[u8]) -> Option<String> {
    let start = bytes.windows(5).take(1024).position(|window| window == b"%PDF-")?;
    let rest = &bytes[start + 5..];
    let end = rest.iter().take(8).position(|&b| b == b'\n' || b == b'\r' || b == b' ' || b == b'\t').unwrap_or(rest.len().min(8));
    let version = String::from_utf8_lossy(&rest[..end]).trim().to_string();
    (!version.is_empty() && version.chars().all(|c| c.is_ascii_digit() || c == '.')).then_some(version)
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
    let declared_version = sniff_pdf(data).ok_or(PdfEngineError::NotPdf)?;
    let start = data.windows(5).position(|window| window == b"%PDF-").unwrap_or(0);
    let data = &data[start..];
    let xref = build_xref(data, startxref_offset(data));
    let mut resolver = Resolver::new(data, xref.entries.clone());
    let mut encryption = None;
    let mut encrypt_object: Option<u32> = None;
    if let Some(encrypt) = dict_get(&xref.trailer, "Encrypt") {
        encrypt_object = encrypt.as_ref().map(|reference| reference.num);
        let dictionary = match encrypt {
            PdfObject::Ref(reference) => resolver.resolve(reference.num).ok_or_else(|| PdfEngineError::Malformed("/Encrypt reference does not resolve".into()))?,
            other => other.clone(),
        };
        let dictionary = dictionary.as_dict().ok_or_else(|| PdfEngineError::Malformed("/Encrypt is not a dictionary".into()))?.to_vec();
        let document_id = dict_get(&xref.trailer, "ID").and_then(PdfObject::as_array).and_then(|items| items.first()).and_then(PdfObject::as_str_bytes).unwrap_or(&[]).to_vec();
        let (decryptor, parameters) = open_standard_security(&dictionary, &document_id, password)?;
        resolver.set_decryptor(Some(decryptor));
        encryption = Some(parameters);
    }
    let mut objects = resolver.resolve_all()?;
    if let Some(number) = encrypt_object {
        objects.retain(|object| object.id.num != number);
    }
    let mut trailer: Vec<PdfDictEntry> = xref.trailer.iter().filter(|entry| !WRITER_TRAILER_KEYS.contains(&entry.key.as_str())).cloned().collect();
    if !trailer.iter().any(|entry| entry.key == "Root") {
        return Err(PdfEngineError::Malformed("no /Root in any trailer and no /Catalog object to recover one from".into()));
    }
    crate::standards::v1_7::subsets::base::io::text::snapshot::retained_text::admit_retained_text(&mut objects,&mut trailer);
    let mut source = GraphSource::new(&objects);
    let lifter = lift_document_with(&trailer, &declared_version, &mut source);
    if let Some(error)=lifter.admission_error {return Err(error);}
    let mut snapshot = lifter.snapshot;
    snapshot.schema = STDIO_PDF17_DOCUMENT_SCHEMA.into();
    snapshot.declared_version = declared_version;
    snapshot.encryption = encryption;
    for mut object in std::mem::take(&mut snapshot.objects){object.id.num=objects.iter().map(|object|object.id.num).max().unwrap_or(0).checked_add(1).ok_or_else(||PdfEngineError::Unsupported("PDF native custody exhausted".into()))?;objects.push(object);}
    snapshot.objects = objects;
    snapshot.trailer = trailer;
    Ok(snapshot)
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
