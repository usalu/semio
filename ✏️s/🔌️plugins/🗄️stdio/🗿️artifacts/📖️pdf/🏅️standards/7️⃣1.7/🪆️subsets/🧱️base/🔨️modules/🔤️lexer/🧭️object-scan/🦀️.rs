//! 🧭️ Bounded recovery scan of indirect PDF headers when structured xref cannot be read.

use super::{HeaderStep, IndirectHeaderCursor, is_delim, is_ws};
use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot::ObjRef;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ObjectScanStep {
    More,
    Found { id: ObjRef, offset: usize },
    Complete,
}

pub struct ObjectScanCursor {
    position: usize,
    boundary: bool,
    candidate: Option<IndirectHeaderCursor>,
}

impl ObjectScanCursor {
    pub fn new() -> Self { Self { position: 0, boundary: true, candidate: None } }

    /// ⏭️ Resumes beyond bytes already recognized as an indirect object's body.
    pub fn resume_after(&mut self, source: &[u8], resume: usize) {
        if resume <= self.position || resume > source.len() { return; }
        self.position = resume;
        self.boundary = is_ws(source[resume - 1]) || is_delim(source[resume - 1]);
        self.candidate = None;
    }

    /// 🔎️ Scan the immutable original without retaining any bytes or constructing an object map.
    pub fn advance(&mut self, source: &[u8], grant: usize) -> (usize, ObjectScanStep) {
        let mut work = 0;
        while work < grant {
            if let Some(header) = self.candidate.as_mut() {
                let previous = header.position();
                let remaining = grant - work;
                match header.advance(|position| source.get(position).copied(), remaining) {
                    Ok((used, HeaderStep::Complete { id, body_offset })) => {
                        let offset = self.position;
                        self.position = body_offset;
                        self.boundary = false;
                        self.candidate = None;
                        return (work + used, ObjectScanStep::Found { id, offset });
                    }
                    Ok((used, HeaderStep::More)) => return (work + used, ObjectScanStep::More),
                    Err(_) => {
                        work += (header.position() - previous).saturating_add(1).min(remaining);
                        self.position += 1;
                        self.boundary = false;
                        self.candidate = None;
                    }
                }
                continue;
            }
            let Some(&byte) = source.get(self.position) else { return (work, ObjectScanStep::Complete) };
            work += 1;
            if self.boundary && byte.is_ascii_digit() {
                self.candidate = Some(IndirectHeaderCursor::new(self.position));
            } else {
                self.boundary = is_ws(byte) || is_delim(byte);
                self.position += 1;
            }
        }
        (work, ObjectScanStep::More)
    }
}

impl Default for ObjectScanCursor {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recovery_scan_yields_fixture_headers_in_source_order_with_one_byte_work() {
        let pdf = include_bytes!("../../../🧪️tests/🧫️fixtures/🔗️indirect-length.pdf");
        let mut cursor = ObjectScanCursor::new();
        let mut ids = Vec::new();
        loop {
            let (work, step) = cursor.advance(pdf, 1);
            assert!(work <= 1);
            match step {
                ObjectScanStep::More => assert_eq!(work, 1),
                ObjectScanStep::Found { id, offset } => {
                    assert!(pdf[offset..].starts_with(format!("{} {} obj", id.num, id.gen).as_bytes()));
                    ids.push(id.num);
                }
                ObjectScanStep::Complete => break,
            }
        }
        assert_eq!(ids, [1, 2, 3, 4, 5, 6]);
    }
}
