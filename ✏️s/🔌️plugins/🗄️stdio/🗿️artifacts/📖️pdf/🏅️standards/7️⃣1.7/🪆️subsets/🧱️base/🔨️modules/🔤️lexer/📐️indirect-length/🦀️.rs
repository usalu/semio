//! 📐️ Admits an indirect stream length without reading or retaining an unrelated object body.

use super::{is_ws, CosGrant, CosLimits, CosReceipt, CosStatus, HeaderStep, IndirectHeaderCursor, ObjRef, PdfCosCursor};

enum Phase { Header, Seek { comment: bool }, Scalar, Ready, Cancelled, Closing, Closed }

/// 🔢️ Reads one exact-generation numeric object for a separately owned raw stream.
pub struct PdfIndirectLengthCursor {
    expected: ObjRef,
    header: IndirectHeaderCursor,
    value: Option<PdfCosCursor>,
    length: Option<usize>,
    position: usize,
    phase: Phase,
}

impl PdfIndirectLengthCursor {
    pub fn new(expected: ObjRef, offset: usize) -> Self {
        Self { expected, header: IndirectHeaderCursor::new(offset), value: None, length: None, position: offset, phase: Phase::Header }
    }

    pub fn advance(&mut self, data: &[u8], grant: CosGrant, cancelled: bool) -> CosReceipt {
        self.advance_bytes(|position| data.get(position).copied(), grant, cancelled)
    }

    /// 🎚️ Reads the exact indirect-length owner over segmented original bytes.
    pub fn advance_bytes(&mut self, byte_at: impl Fn(usize) -> Option<u8>, mut grant: CosGrant, cancelled: bool) -> CosReceipt {
        if cancelled && !matches!(self.phase, Phase::Closing | Phase::Closed) { self.phase = Phase::Cancelled; }
        let mut spent = 0;
        loop {
            match self.phase {
                Phase::Ready => return step(CosStatus::Complete { end: self.position }, spent),
                Phase::Cancelled => return step(CosStatus::Cancelled, spent),
                Phase::Closed => return step(CosStatus::Closed, spent),
                Phase::Closing => return step(CosStatus::More, spent),
                _ if grant.work == 0 => return step(CosStatus::More, spent),
                Phase::Header => {
                    let before = self.header.position();
                    match self.header.advance(&byte_at, grant.work) {
                        Ok((used, HeaderStep::Complete { id, body_offset })) => {
                            spent += used;
                            grant.work -= used;
                            self.position = body_offset;
                            self.phase = if id == self.expected { Phase::Seek { comment: false } } else { Phase::Ready };
                        }
                        Ok((used, HeaderStep::More)) => return step(CosStatus::More, spent + used),
                        Err(_) => {
                            spent += (self.header.position() - before).saturating_add(1).min(grant.work);
                            self.phase = Phase::Ready;
                        }
                    }
                }
                Phase::Seek { comment } => {
                    spent += 1;
                    grant.work -= 1;
                    match byte_at(self.position) {
                        Some(byte) if comment || is_ws(byte) || byte == b'%' => {
                            self.position += 1;
                            self.phase = Phase::Seek { comment: byte == b'%' || comment && !matches!(byte, b'\r' | b'\n') };
                        }
                        Some(b'0'..=b'9' | b'+' | b'-' | b'.') => {
                            self.value = Some(PdfCosCursor::new(self.position, CosLimits::default()));
                            self.phase = Phase::Scalar;
                        }
                        _ => self.phase = Phase::Ready,
                    }
                }
                Phase::Scalar => {
                    let value = self.value.as_mut().expect("active indirect length token");
                    let mut result = value.advance_bytes(&byte_at, grant, false);
                    result.work += spent;
                    if let Ok(CosStatus::Complete { end }) = result.status {
                        self.length = value.ready_value().and_then(super::PdfObject::as_i64).and_then(|length| usize::try_from(length).ok());
                        self.position = end;
                        self.phase = Phase::Ready;
                    }
                    return result;
                }
            }
        }
    }

    pub fn length(&self) -> Option<usize> { if matches!(self.phase, Phase::Ready) { self.length } else { None } }

    pub fn begin_close(&mut self) {
        if matches!(self.phase, Phase::Closed | Phase::Closing) { return; }
        self.phase = Phase::Closing;
        if let Some(value) = self.value.as_mut() { value.begin_close(); }
    }

    pub fn close_step(&mut self, grant: CosGrant) -> CosReceipt {
        if matches!(self.phase, Phase::Closed) { return step(CosStatus::Closed, 0); }
        if !matches!(self.phase, Phase::Closing) { return step(CosStatus::More, 0); }
        if let Some(value) = self.value.as_mut() {
            let result = value.close_step(grant);
            if result.status == Ok(CosStatus::Closed) {
                self.value = None;
                self.phase = Phase::Closed;
            }
            return result;
        }
        self.phase = Phase::Closed;
        step(CosStatus::Closed, 0)
    }
}

fn step(status: CosStatus, work: usize) -> CosReceipt {
    CosReceipt { status: Ok(status), work, allocated_bytes: 0, released_bytes: 0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn indirect_length_accepts_only_its_exact_numeric_owner() {
        let one = CosGrant { work: 1, allocation_bytes: 128, release_bytes: 128 };
        let expected = ObjRef { num: 40, gen: 3 };
        for (data, id, value) in [
            (&b"40 % comment\n3 obj\n+9\nendobj"[..], expected, Some(9)),
            (b"40 3 obj\n+9\nendobj", ObjRef { gen: 0, ..expected }, None),
            (b"40 3 obj\n<< /Length 10000 >>\nstream\n" , expected, None),
            (b"40 3 obj\n12 0 R\nendobj", expected, None),
        ] {
            let mut cursor = PdfIndirectLengthCursor::new(id, 0);
            let mut done = false;
            for _ in 0..100 {
                let turn = cursor.advance(data, one, false);
                assert!(turn.work <= 1 && turn.allocated_bytes <= one.allocation_bytes);
                if matches!(turn.status, Ok(CosStatus::Complete { .. })) { done = true; break; }
                assert_eq!(turn.status, Ok(CosStatus::More));
            }
            assert!(done);
            assert_eq!(cursor.length(), value);
            cursor.begin_close();
            while cursor.close_step(one).status != Ok(CosStatus::Closed) {}
        }
    }
}
