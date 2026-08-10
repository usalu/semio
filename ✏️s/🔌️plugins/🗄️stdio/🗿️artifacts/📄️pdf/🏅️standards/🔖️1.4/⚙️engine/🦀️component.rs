//! ⚙️ PdfEngine — minimal PDF 1.4 with FlateDecode stream.

// 🔀️ S-6 twin: `crate::artifacts::pdf::schema` now shims to 1.7 (canonical) -- 1.4's own engine
// uses its own standard-local schema path directly rather than the shared root re-export.
use crate::artifacts::pdf::STDIO_PDF_DOCUMENT_SCHEMA;
use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::{diff::PdfDiff, mutations::PdfMutation, snapshot::{PageDoc, PdfSnapshot}, PdfArtifact};

pub fn encode_pdf(snap: &PdfSnapshot) -> Result<Vec<u8>, String> {
    let page = &snap.page;
    let w = page.width.max(1.0);
    let h = page.height.max(1.0);
    let stream = format!("BT /F1 12 Tf 72 {} Td ({}) Tj ET", h - 72.0, escape_pdf(&page.text));
    let compressed = crate::artifacts::deflate::engine::zlib_compress(stream.as_bytes())?;
    let mut body: Vec<u8> = Vec::new();
    body.extend_from_slice(b"%PDF-1.4\n");
    let o1 = body.len();
    body.extend_from_slice(b"1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n");
    let o2 = body.len();
    body.extend_from_slice(b"2 0 obj\n<< /Type /Pages /Kids [3 0 R] /Count 1 >>\nendobj\n");
    let o3 = body.len();
    body.extend_from_slice(format!("3 0 obj\n<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {w} {h}] /Contents 4 0 R /Resources << /Font << /F1 5 0 R >> >> >>\nendobj\n").as_bytes());
    let o4 = body.len();
    body.extend_from_slice(format!("4 0 obj\n<< /Length {} /Filter /FlateDecode >>\nstream\n", compressed.len()).as_bytes());
    body.extend_from_slice(&compressed);
    body.extend_from_slice(b"\nendstream\nendobj\n");
    let o5 = body.len();
    body.extend_from_slice(b"5 0 obj\n<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>\nendobj\n");
    let xref = body.len();
    body.extend_from_slice(b"xref\n0 6\n0000000000 65535 f \n");
    for off in [o1, o2, o3, o4, o5] {
        body.extend_from_slice(format!("{:010} 00000 n \n", off).as_bytes());
    }
    body.extend_from_slice(format!("trailer\n<< /Size 6 /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n").as_bytes());
    Ok(body)
}

fn escape_pdf(s: &str) -> String {
    s.replace('\\', "\\\\").replace('(', "\\(").replace(')', "\\)")
}

pub fn decode_pdf(data: &[u8]) -> Result<PdfSnapshot, String> {
    let text = String::from_utf8_lossy(data);
    if !text.starts_with("%PDF") { return Err("not pdf".into()); }
    let w = 612.0f64;
    let h = 792.0f64;
    let mut content = String::new();
    if let Some(i) = text.find("stream") {
        let rest = &text[i + 6..];
        if let Some(j) = rest.find("endstream") {
            let raw = rest[..j].trim().as_bytes();
            if let Ok(dec) = crate::artifacts::deflate::engine::zlib_decompress(raw) {
                content = String::from_utf8_lossy(&dec).into_owned();
            }
        }
    }
    let label = content.split('(').nth(1).and_then(|s| s.split(')').next()).unwrap_or("").to_string();
    Ok(PdfSnapshot {
        schema: STDIO_PDF_DOCUMENT_SCHEMA.into(),
        page: PageDoc { width: w, height: h, text: label },
    })
}

pub fn empty_pdf_snapshot() -> PdfSnapshot { PdfSnapshot::default() }

pub fn register() {
    crate::artifacts::pdf::composer::register();
    ::schema::register_artifact_schema_descriptor(crate::artifacts::pdf::standards::v1_4::subsets::any::schema::pdf_artifact_schema_descriptor());
    store::register_document_codec(store::ArtifactCodec::of::<PdfSnapshot, PdfMutation>(STDIO_PDF_DOCUMENT_SCHEMA));
}

pub struct PdfEngine { artifact_state: PdfArtifact, snapshot_state: PdfSnapshot }
impl PdfEngine {
    pub fn new(snapshot: PdfSnapshot) -> Self {
        Self { artifact_state: PdfArtifact::from_snapshot(snapshot.clone()), snapshot_state: snapshot }
    }
}
