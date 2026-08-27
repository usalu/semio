//! 🧰 Shared canonical text-carrier representability check.
use crate::artifacts::txt::TxtSnapshot;
use crate::artifacts::txt::schema::snapshot::LineEnding;

//#region 🔖️Shape
pub fn non_canonical_shape(line_count: usize, last_line_is_empty: bool, trailing_newline: bool) -> Option<String> {
    if trailing_newline && line_count == 0 {
        return Some("a document with no lines cannot carry a trailing terminator".to_string());
    }
    if !trailing_newline && last_line_is_empty {
        return Some("an unterminated document cannot end with an empty line".to_string());
    }
    None
}
//#endregion 🔖️Shape

//#region 🔖️Carrier
pub fn native_text_error(text: &str, line_ending: LineEnding, followed_by_separator: bool) -> Option<String> {
    match line_ending {
        LineEnding::Lf if text.contains('\n') => Some("an LF document line cannot contain LF".to_string()),
        LineEnding::Lf if followed_by_separator && text.ends_with('\r') => Some("a bare CR cannot precede an LF separator".to_string()),
        LineEnding::CrLf if text.contains("\r\n") => Some("a CRLF document line cannot contain CRLF".to_string()),
        _ => None,
    }
}

pub fn native_shape_error(line_count: usize, last_line_is_empty: bool, trailing_newline: bool, line_ending: LineEnding) -> Option<String> {
    if let Some(reason) = non_canonical_shape(line_count, last_line_is_empty, trailing_newline) {
        return Some(reason);
    }
    if line_ending == LineEnding::CrLf && line_count <= 1 && !trailing_newline {
        return Some("CRLF requires a visible separator".to_string());
    }
    None
}

pub fn native_lines_error(lines: &[String], trailing_newline: bool, line_ending: LineEnding) -> Option<String> {
    if let Some(reason) = native_shape_error(lines.len(), lines.last().is_some_and(|line| line.is_empty()), trailing_newline, line_ending) {
        return Some(reason);
    }
    lines.iter().enumerate().find_map(|(index, text)| native_text_error(text, line_ending, index + 1 < lines.len() || trailing_newline))
}

pub fn native_snapshot_error(snapshot: &TxtSnapshot) -> Option<String> {
    native_lines_error(&snapshot.lines, snapshot.trailing_newline, snapshot.line_ending)
}
//#endregion 🔖️Carrier

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn carrier_product_matrix_matches_the_production_snapshot_codec() {
        let texts = ["", "a", "a\nb", "a\rb", "a\r", "a\r\nb"];
        for line_ending in [LineEnding::Lf, LineEnding::CrLf] {
            for trailing_newline in [false, true] {
                for first in texts {
                    for second in texts {
                        for lines in [Vec::new(), vec![first.to_string()], vec![first.to_string(), second.to_string()]] {
                            let snapshot = TxtSnapshot { lines, trailing_newline, line_ending, ..Default::default() };
                            let production_round_trip = TxtSnapshot::from_body(&snapshot.to_body()) == snapshot;
                            assert_eq!(native_snapshot_error(&snapshot).is_none(), production_round_trip, "{snapshot:?}");
                        }
                    }
                }
            }
        }
    }
}
//#endregion 🧪️Tests
