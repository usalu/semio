//! 🧾️ Produces the live and free entries and typed trailer of one classic xref section.

use super::classic::{ClassicXrefCursor, ClassicXrefEvent};
use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot::{PdfDictEntry, PdfObject};
use semio_s_artifact_stdio_pdf_lexer::{CosGrant, CosLimits, CosStatus, PdfCosCursor};

pub struct ClassicSectionCursor {
    rows: ClassicXrefCursor,
    trailer: Option<PdfCosCursor>,
    cancelled: bool,
    closing: bool,
}

pub struct ClassicSectionReceipt {
    pub event: Result<ClassicSectionEvent, &'static str>,
    pub work: usize,
    pub allocated_bytes: usize,
    pub released_bytes: usize,
}

pub enum ClassicSectionEvent {
    Row { number: u32, offset: usize, generation: u16, live: bool },
    Progress(CosStatus),
}

impl ClassicSectionCursor {
    pub fn new(offset: usize) -> Self {
        Self { rows: ClassicXrefCursor::new(offset), trailer: None, cancelled: false, closing: false }
    }

    /// 📋️ Emits one row or advances the typed trailer using the remaining allocation and work grant.
    pub fn advance(&mut self, byte_at: impl Fn(usize) -> Option<u8>, grant: CosGrant, cancelled: bool) -> ClassicSectionReceipt {
        use ClassicSectionEvent::{Progress, Row};
        use CosStatus as Status;
        let blank = |event| ClassicSectionReceipt { event: Ok(Progress(event)), work: 0, allocated_bytes: 0, released_bytes: 0 };
        self.cancelled |= cancelled;
        if self.closing { return blank(Status::Closed); }
        if self.cancelled { return blank(Status::Cancelled); }
        if let Some(trailer) = self.trailer.as_mut() {
            let receipt = trailer.advance_bytes(&byte_at, grant, false);
            let event = receipt.status.and_then(|status| {
                if matches!(status, CosStatus::Complete { .. }) && !matches!(trailer.ready_value(), Some(PdfObject::Dict(_))) {
                    Err("trailer is not a dict")
                } else {
                    Ok(Progress(status))
                }
            });
            return ClassicSectionReceipt { event, work: receipt.work, allocated_bytes: receipt.allocated_bytes, released_bytes: receipt.released_bytes };
        }
        let receipt = self.rows.advance(byte_at, grant.work);
        let event = receipt.event.map(|step| match step {
            ClassicXrefEvent::Entry { number, offset, generation, live } => Row { number, offset, generation, live },
            ClassicXrefEvent::Trailer => {
                self.trailer = Some(PdfCosCursor::new(self.rows.position(), CosLimits::default()));
                Progress(Status::More)
            }
            ClassicXrefEvent::More => Progress(Status::More),
        });
        ClassicSectionReceipt { event, work: receipt.work, allocated_bytes: 0, released_bytes: 0 }
    }

    /// 🏃️ Whether the next unit scans one row byte, before the trailer dictionary starts.
    pub fn scans_rows(&self) -> bool { self.trailer.is_none() && !self.closing && !self.cancelled }

    pub fn ready_trailer(&self) -> Option<&[PdfDictEntry]> {
        if self.closing || self.cancelled { return None; }
        self.trailer.as_ref()?.ready_value()?.as_dict()
    }

    pub fn take_trailer(&mut self) -> Option<Vec<PdfDictEntry>> {
        if self.closing || self.cancelled { return None; }
        let PdfObject::Dict(entries) = self.trailer.as_mut()?.take_value()? else { return None };
        Some(entries)
    }

    pub fn begin_close(&mut self) {
        self.closing = true;
        if let Some(trailer) = self.trailer.as_mut() { trailer.begin_close(); }
    }

    pub fn close_step(&mut self, grant: CosGrant) -> ClassicSectionReceipt {
        let mut result = ClassicSectionReceipt { event: Ok(ClassicSectionEvent::Progress(CosStatus::More)), work: 0, allocated_bytes: 0, released_bytes: 0 };
        if !self.closing { return result; }
        if let Some(trailer) = self.trailer.as_mut() {
            let receipt = trailer.close_step(grant);
            result.work = receipt.work;
            result.released_bytes = receipt.released_bytes;
            match receipt.status {
                Ok(CosStatus::Closed) => self.trailer = None,
                Ok(CosStatus::NeedsRelease { bytes }) => result.event = Ok(ClassicSectionEvent::Progress(CosStatus::NeedsRelease { bytes })),
                Ok(_) => {}
                Err(reason) => result.event = Err(reason),
            }
        }
        if self.trailer.is_none() { result.event = Ok(ClassicSectionEvent::Progress(CosStatus::Closed)); }
        result
    }
}
