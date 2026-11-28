//! 📋️ Shared-formula expansion: a dependent `<f t="shared" si>` cell has the formula of its master translated by the offset between the two cells.
//! Relative reference parts shift, `$`-anchored parts stay, and a reference pushed off the grid becomes `#REF!`, exactly as Excel and LibreOffice expand a fill-down group.
//! See [ECMA-376 §18.3.1.40](https://ecma-international.org/publications-and-standards/standards/ecma-376/).

use crate::standards::v_ecma_376::subsets::base::schema::vocabulary::{column_index, column_letter, XLSX_MAX_COLUMN, XLSX_MAX_ROW};

/// 🧭️ The master of one shared-formula group: its formula text and its own coordinates (1-based row, 0-based column).
#[derive(Clone, Debug, PartialEq)]
pub struct SharedMaster {
    pub expr: String,
    pub row: u32,
    pub col: u32,
}

/// 📋️ The formula `master` spells at the cell (`row`, `col`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn expand_shared_formula(master: &SharedMaster, row: u32, col: u32) -> String {
    translate_formula(&master.expr, i64::from(row) - i64::from(master.row), i64::from(col) - i64::from(master.col))
}

/// ↔️ Moves every relative A1 reference (and column-only range) of `expr` by (`d_row`, `d_col`), leaving string literals, quoted sheet names, function names and `$` parts alone.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn translate_formula(expr: &str, d_row: i64, d_col: i64) -> String {
    let chars: Vec<char> = expr.chars().collect();
    let mut out = String::with_capacity(expr.len());
    let mut at = 0;
    while at < chars.len() {
        let c = chars[at];
        if c == '"' || c == '\'' {
            let end = quoted_end(&chars, at, c);
            out.extend(&chars[at..end]);
            at = end;
            continue;
        }
        let boundary = at == 0 || !(chars[at - 1].is_alphanumeric() || matches!(chars[at - 1], '_' | '.' | '$'));
        if boundary && (c == '$' || c.is_ascii_alphabetic()) {
            if let Some((text, end)) = reference_at(&chars, at, d_row, d_col) {
                out.push_str(&text);
                at = end;
                continue;
            }
        }
        if c.is_alphanumeric() || matches!(c, '_' | '.') {
            while at < chars.len() && (chars[at].is_alphanumeric() || matches!(chars[at], '_' | '.')) {
                out.push(chars[at]);
                at += 1;
            }
            continue;
        }
        out.push(c);
        at += 1;
    }
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn quoted_end(chars: &[char], start: usize, quote: char) -> usize {
    let mut at = start + 1;
    while at < chars.len() {
        if chars[at] == quote {
            if chars.get(at + 1) == Some(&quote) {
                at += 2;
                continue;
            }
            return at + 1;
        }
        at += 1;
    }
    chars.len()
}

struct Part {
    absolute: bool,
    value: String,
    end: usize,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn part_at(chars: &[char], at: usize, digits: bool) -> Option<Part> {
    let absolute = chars.get(at) == Some(&'$');
    let start = at + usize::from(absolute);
    let mut end = start;
    while end < chars.len() && if digits { chars[end].is_ascii_digit() } else { chars[end].is_ascii_alphabetic() } {
        end += 1;
    }
    (end > start).then(|| Part { absolute, value: chars[start..end].iter().collect(), end })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn continues_identifier(chars: &[char], at: usize) -> bool {
    chars.get(at).is_some_and(|c| c.is_alphanumeric() || matches!(c, '_' | '(' | '!' | '.'))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn shifted_column(part: &Part, d_col: i64) -> Option<String> {
    let index = column_index(&part.value).filter(|index| *index <= XLSX_MAX_COLUMN && part.value.len() <= 3)?;
    if part.absolute {
        return Some(format!("${}", column_letter(index)));
    }
    let moved = i64::from(index) + d_col;
    (0..=i64::from(XLSX_MAX_COLUMN)).contains(&moved).then(|| column_letter(moved as u32))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn reference_at(chars: &[char], at: usize, d_row: i64, d_col: i64) -> Option<(String, usize)> {
    let column = part_at(chars, at, false)?;
    let moved_column = shifted_column(&column, d_col);
    let (row_absolute, row_start) = if chars.get(column.end) == Some(&'$') { (true, column.end + 1) } else { (false, column.end) };
    let mut row_end = row_start;
    while row_end < chars.len() && chars[row_end].is_ascii_digit() {
        row_end += 1;
    }
    if row_end > row_start {
        if continues_identifier(chars, row_end) {
            return None;
        }
        let number: i64 = chars[row_start..row_end].iter().collect::<String>().parse().ok()?;
        let moved_row = if row_absolute { Some(number) } else { Some(number + d_row).filter(|row| (1..=i64::from(XLSX_MAX_ROW)).contains(row)) };
        let text = match (moved_column, moved_row) {
            (Some(col), Some(row)) => format!("{col}{}{row}", if row_absolute { "$" } else { "" }),
            _ => "#REF!".to_string(),
        };
        return Some((text, row_end));
    }
    if chars.get(column.end) == Some(&':') {
        let other = part_at(chars, column.end + 1, false)?;
        if continues_identifier(chars, other.end) || part_at(chars, other.end, true).is_some() {
            return None;
        }
        let text = match (moved_column, shifted_column(&other, d_col)) {
            (Some(left), Some(right)) => format!("{left}:{right}"),
            _ => "#REF!".to_string(),
        };
        return Some((text, other.end));
    }
    None
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
