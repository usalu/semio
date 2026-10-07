//! 📅️ Physical PDF date spelling over the owned date value.
use crate::schema::snapshot::PdfDate;

/// 📥️ Decodes a native PDF date.
pub fn parse_pdf_date(text: &str) -> Option<PdfDate> {
        let body = text.strip_prefix("D:").unwrap_or(text);
        let digits = |from: usize, len: usize| body.get(from..from + len).filter(|s| s.len() == len && s.bytes().all(|b| b.is_ascii_digit())).and_then(|s| s.parse::<u32>().ok());
        let year = digits(0, 4)? as i32;
        let month = digits(4, 2).unwrap_or(1).clamp(1, 12);
        let day = digits(6, 2).unwrap_or(1).clamp(1, 31);
        let hour = digits(8, 2).unwrap_or(0).min(23);
        let minute = digits(10, 2).unwrap_or(0).min(59);
        let second = digits(12, 2).unwrap_or(0).min(59);
        let marker = body.bytes().position(|b| !b.is_ascii_digit()).unwrap_or(body.len()).min(14);
        let offset_minutes = match body.as_bytes().get(marker) {
            Some(b'Z') => Some(0),
            Some(sign @ (b'+' | b'-')) => {
                let hours = digits(marker + 1, 2).unwrap_or(0) as i32;
                let minutes = digits(marker + 4, 2).unwrap_or(0) as i32;
                let total = hours * 60 + minutes;
                Some(if *sign == b'-' { -total } else { total })
            }
            _ => None,
        };
        Some(PdfDate { year, month, day, hour, minute, second, offset_minutes })
    }

struct NativePdfDate<'a>(&'a PdfDate);
impl std::fmt::Display for NativePdfDate<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "D:{:04}{:02}{:02}{:02}{:02}{:02}", self.0.year, self.0.month, self.0.day, self.0.hour, self.0.minute, self.0.second)?;
        match self.0.offset_minutes {
            Some(0) => formatter.write_str("Z"),
            Some(offset) => write!(formatter, "{}{:02}'{:02}'", if offset < 0 { '-' } else { '+' }, offset.abs() / 60, offset.abs() % 60),
            None => Ok(()),
        }
    }
}

/// 📤️ Formats a native PDF date.
pub fn print_pdf_date(date:&PdfDate)->String { NativePdfDate(date).to_string() }

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
