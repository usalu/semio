//! 🔎️ Checks cross-reference offsets against their exact indirect object identities.

use super::{XrefEntry, XrefIndex, XrefIndexScan, XrefRow};
use semio_s_artifact_stdio_pdf_lexer::{HeaderStep, IndirectHeaderCursor};
use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot::ObjRef;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XrefOffsetStatus {
    More,
    Complete,
    Invalid { id: ObjRef, offset: usize },
    Cancelled,
}

/// 🧭️ Walks the immutable cross-reference table and resumes within a partial object header.
#[derive(Default)]
pub struct XrefOffsetCursor {
    rows: XrefIndexScan,
    header: Option<(ObjRef, usize, IndirectHeaderCursor)>,
    terminal: Option<XrefOffsetStatus>,
}

impl XrefOffsetCursor {
    /// ▶️ Charges one work unit per visited table slot or header byte.
    pub fn advance(&mut self, byte_at: impl Fn(usize) -> Option<u8>, entries: &XrefIndex, grant: usize, cancelled: bool) -> (usize, XrefOffsetStatus) {
        if cancelled && self.terminal.is_none() { self.terminal = Some(XrefOffsetStatus::Cancelled); }
        if let Some(status) = self.terminal { return (0, status); }

        let mut work = 0;
        while work < grant {
            if let Some((expected, offset, cursor)) = self.header.as_mut() {
                let start = cursor.position();
                let remaining = grant - work;
                let status = match cursor.advance(&byte_at, remaining) {
                    Ok((used, HeaderStep::Complete { id, .. })) => {
                        work += used;
                        if id == *expected {
                            self.header = None;
                            continue;
                        }
                        XrefOffsetStatus::Invalid { id: *expected, offset: *offset }
                    }
                    Ok((used, HeaderStep::More)) => return (work + used, XrefOffsetStatus::More),
                    Err(_) => {
                        work += (cursor.position() - start).saturating_add(1).min(remaining);
                        XrefOffsetStatus::Invalid { id: *expected, offset: *offset }
                    }
                };
                self.terminal = Some(status);
                return (work, status);
            }

            let (inspected, done, row) = self.rows.advance(entries, grant - work);
            work += inspected;
            if let Some(XrefRow { number, entry: Some(XrefEntry::Normal { offset, gen }) }) = row {
                self.header = Some((ObjRef { num: number, gen }, offset, IndirectHeaderCursor::new(offset)));
            } else if done {
                self.terminal = Some(XrefOffsetStatus::Complete);
                return (work, XrefOffsetStatus::Complete);
            }
        }
        (work, XrefOffsetStatus::More)
    }
}
