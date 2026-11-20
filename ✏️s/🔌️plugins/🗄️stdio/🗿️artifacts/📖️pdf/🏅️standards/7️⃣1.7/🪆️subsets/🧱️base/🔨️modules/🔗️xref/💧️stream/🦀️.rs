//! 💧️ Incremental big-endian row decoding for a PDF cross-reference stream.

#[derive(Clone, Debug)]
pub struct XrefStreamRowCursor {
    widths: [usize; 3],
    values: [u64; 3],
    field: usize,
    position: usize,
}

impl XrefStreamRowCursor {
    pub fn new(widths: [usize; 3]) -> Result<Self, &'static str> {
        if widths == [0; 3] {
            return Err("xref stream /W is all zero");
        }
        Ok(Self { widths, values: [u64::from(widths[0] == 0), 0, 0], field: 0, position: 0 })
    }

    /// 🧾️ Consume the granted prefix and return at most one row, retaining a partially read field.
    pub fn advance(&mut self, grant: &[u8]) -> Result<(usize, Option<[u64; 3]>), &'static str> {
        let mut consumed = 0;
        loop {
            if self.field == self.widths.len() {
                let row = self.values;
                self.field = 0;
                self.values = [u64::from(self.widths[0] == 0), 0, 0];
                return Ok((consumed, Some(row)));
            }
            if self.position == self.widths[self.field] {
                self.field += 1;
                self.position = 0;
                continue;
            }
            let Some(byte) = grant.get(consumed) else { return Ok((consumed, None)) };
            let value = self.values[self.field].checked_mul(256).ok_or("xref stream field exceeds 64 bits")?;
            self.values[self.field] = value + u64::from(*byte);
            self.position += 1;
            consumed += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn incremental_update_rows_match_across_every_input_grant() {
        let pdf = include_bytes!("../../../🧪️tests/🧫️fixtures/🔗️incremental-free/💧️stream.pdf");
        let start = pdf.windows(8).rposition(|w| w == b"\nstream\n").unwrap() + 8;
        let info = pdf.windows(8).position(|w| w == b"\n9 0 obj").unwrap() + 1;
        let rows = [[0, 3, 65535], [0, 0, 1], [1, info as u64, 0], [1, 1002, 0]];
        for budget in 1..=28 {
            let mut reader = XrefStreamRowCursor::new([1, 4, 2]).unwrap();
            let mut pos = start;
            for expected in rows {
                loop {
                    let (used, row) = reader.advance(&pdf[pos..pos + budget.min(start + 28 - pos)]).unwrap();
                    assert!(used > 0 && used <= budget);
                    pos += used;
                    if let Some(got) = row { assert_eq!(got, expected); break }
                }
            }
            assert_eq!(pos, start + 28);
        }
    }

    #[test]
    fn omitted_widths_use_pdf_defaults_and_overflow_is_reported() {
        let mut cursor = XrefStreamRowCursor::new([0, 2, 0]).unwrap();
        assert_eq!(cursor.advance(&[]), Ok((0, None)));
        assert_eq!(cursor.advance(&[1, 2, 3]), Ok((2, Some([1, 258, 0]))));
        assert_eq!(cursor.advance(&[3, 4]), Ok((2, Some([1, 772, 0]))));
        assert!(XrefStreamRowCursor::new([0; 3]).is_err());
        let mut wide = XrefStreamRowCursor::new([0, 9, 0]).unwrap();
        assert!(wide.advance(&[255; 9]).is_err());
    }
}
