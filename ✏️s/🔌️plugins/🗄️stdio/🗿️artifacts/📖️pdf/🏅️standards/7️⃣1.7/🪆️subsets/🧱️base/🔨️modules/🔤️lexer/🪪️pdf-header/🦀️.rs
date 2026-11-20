//! 🪪️ Discovers the PDF header and version without allocating from source bytes.

const MAGIC: &[u8] = b"%PDF-";
const MAXIMUM_OFFSET: usize = 1023;
const MAXIMUM_VERSION_BYTES: usize = 8;

/// 📍 The original PDF header location and its validated ASCII version.
#[derive(Clone, Copy)]
pub struct PdfHeader {
    offset: usize,
    bytes: [u8; MAXIMUM_VERSION_BYTES],
    length: usize,
}

impl PdfHeader {
    pub fn offset(&self) -> usize { self.offset }

    pub fn version(&self) -> &str {
        std::str::from_utf8(&self.bytes[..self.length]).expect("PDF version only contains ASCII digits and periods")
    }
}

pub enum PdfHeaderStep {
    More,
    Complete(Option<PdfHeader>),
}

#[derive(Default)]
pub struct PdfHeaderCursor {
    position: usize,
    matched: usize,
    header: Option<PdfHeader>,
    result: Option<Option<PdfHeader>>,
}

impl PdfHeaderCursor {
    /// 🔎️ Consumes one bounded source transition per work unit until a printable version settles.
    pub fn advance(&mut self, source: &[u8], grant: usize) -> (usize, PdfHeaderStep) {
        let mut work = 0;
        while work < grant && self.result.is_none() {
            work += 1;
            let next = source.get(self.position).copied();
            if let Some(header) = self.header.as_mut() {
                let finishes = next.is_none_or(|byte| matches!(byte, b' ' | b'\t' | b'\r' | b'\n'));
                if finishes {
                    self.result = Some((header.length != 0).then_some(*header));
                } else if let Some(byte) = next.filter(|byte| byte.is_ascii_digit() || *byte == b'.') {
                    header.bytes[header.length] = byte;
                    header.length += 1;
                    self.position += 1;
                    if header.length == MAXIMUM_VERSION_BYTES { self.result = Some(Some(*header)); }
                } else {
                    self.result = Some(None);
                }
                continue;
            }
            if self.matched == 0 && self.position > MAXIMUM_OFFSET {
                self.result = Some(None);
                continue;
            }
            let Some(byte) = next else {
                self.result = Some(None);
                continue;
            };
            self.matched = if byte == MAGIC[self.matched] {
                self.matched + 1
            } else {
                usize::from(self.position <= MAXIMUM_OFFSET && byte == MAGIC[0])
            };
            self.position += 1;
            if self.matched == MAGIC.len() {
                self.header = Some(PdfHeader {
                    offset: self.position - MAGIC.len(),
                    bytes: [0; MAXIMUM_VERSION_BYTES],
                    length: 0,
                });
            }
        }
        (work, self.result.map_or(PdfHeaderStep::More, PdfHeaderStep::Complete))
    }
}

/// 🗺️ Drains the bounded probe for a reference reader.
pub fn pdf_header(source: &[u8]) -> Option<PdfHeader> {
    let mut cursor = PdfHeaderCursor::default();
    match cursor.advance(source, MAXIMUM_OFFSET + MAGIC.len() + MAXIMUM_VERSION_BYTES + 1).1 {
        PdfHeaderStep::Complete(header) => header,
        PdfHeaderStep::More => None,
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
