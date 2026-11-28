//! 🧮️ Recalculation of formula caches after an edit — the one shared helper every cell-editing path (typed draft, details pane) appends to its user mutations, so a single command carries the edit and the caches it makes stale.

use super::*;
use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::calculation::recalculate_workbook;
use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::vocabulary::{column_index, XLSX_MAX_COLUMN, XLSX_MAX_ROW};

/// 🧮️ Appends one `SetCell` per formula whose cache the `mutations` make stale, each addressed in the state its predecessors leave behind; a sequence that cannot be applied or projected is returned unchanged.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn xlsx_with_recalculated_caches(snapshot: &XlsxSnapshot, mutations: Vec<XlsxMutation>) -> Vec<XlsxMutation> {
    recalculated(snapshot, &mutations).map_or(mutations.clone(), |recalculated| [mutations, recalculated].concat())
}

fn recalculated(snapshot: &XlsxSnapshot, mutations: &[XlsxMutation]) -> Option<Vec<XlsxMutation>> {
    let mut next = snapshot.clone();
    for mutation in mutations {
        apply_checked(&mut next, mutation)?;
    }
    let mut follow_ups = Vec::new();
    for cell in recalculate_workbook(&next.project_workbook().ok()?) {
        let mutation = XlsxMutation::SetCell(set_cell::SetCell {
            address: cell_address::xlsx_cell_address(&next, &cell.sheet, cell.row, cell.col).ok()?,
            value: XlsxCellValue::Formula { expr: cell.expr, cached: Some(Box::new(cell.cached)) },
            node: None,
        });
        apply_checked(&mut next, &mutation)?;
        follow_ups.push(mutation);
    }
    Some(follow_ups)
}

fn apply_checked(snapshot: &mut XlsxSnapshot, mutation: &XlsxMutation) -> Option<()> {
    match ::semio_os_kernel::ColdMutation::diff(mutation, snapshot).admit() {
        protocol::MutationAdmission::Admitted(outcome) => {
            *snapshot = protocol::apply_diff_cold(outcome.diff(), snapshot).ok()?;
            Some(())
        }
        protocol::MutationAdmission::Refused(_) => None,
    }
}

//#region 🔖️SheetReferences
/// 🏷️ Rewrites every reference to sheet `from` in formula text `expr` the way Excel does: a rename (`to` is `Some`) swaps the sheet prefix and adds or drops its quotes as the new name requires,
/// a removal (`to` is `None`) turns the whole reference into `#REF!`. String literals and other sheets whose names merely share a prefix are left alone.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn rewrite_sheet_references(expr: &str, from: &str, to: Option<&str>) -> String {
    let chars: Vec<char> = expr.chars().collect();
    let is_word = |c: char| c.is_alphanumeric() || matches!(c, '_' | '.' | '$');
    let same = |name: &str| name.to_lowercase() == from.to_lowercase();
    let mut out = String::new();
    let mut at = 0;
    while at < chars.len() {
        let start = at;
        let (name, quoted) = match chars[at] {
            '"' => {
                at += 1;
                while at < chars.len() && (chars[at] != '"' || chars.get(at + 1) == Some(&'"')) {
                    at += if chars[at] == '"' { 2 } else { 1 };
                }
                at = (at + 1).min(chars.len());
                out.extend(&chars[start..at]);
                continue;
            }
            '\'' => {
                at += 1;
                let mut name = String::new();
                while at < chars.len() && (chars[at] != '\'' || chars.get(at + 1) == Some(&'\'')) {
                    name.push(chars[at]);
                    at += if chars[at] == '\'' { 2 } else { 1 };
                }
                at = (at + 1).min(chars.len());
                (name, true)
            }
            c if is_word(c) => {
                while at < chars.len() && is_word(chars[at]) {
                    at += 1;
                }
                (chars[start..at].iter().collect(), false)
            }
            c => {
                out.push(c);
                at += 1;
                continue;
            }
        };
        if chars.get(at) != Some(&'!') || (!quoted && name.contains('$')) || !same(&name) {
            out.extend(&chars[start..at]);
            continue;
        }
        at += 1;
        match to {
            Some(new) => out.push_str(&format!("{}!", sheet_name_token(new))),
            None => {
                while at < chars.len() && (chars[at].is_alphanumeric() || chars[at] == '$') {
                    at += 1;
                }
                if chars.get(at) == Some(&':') && chars.get(at + 1).is_some_and(|c| c.is_alphanumeric() || *c == '$') {
                    at += 1;
                    while at < chars.len() && (chars[at].is_alphanumeric() || chars[at] == '$') {
                        at += 1;
                    }
                }
                out.push_str("#REF!");
            }
        }
    }
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn sheet_name_token(name: &str) -> String {
    let lower = name.to_lowercase();
    let r1c1 = |rest: &str, marker: char| {
        let rest = rest.trim_start_matches(|c: char| c.is_ascii_digit());
        rest.is_empty() || rest.strip_prefix(marker).is_some_and(|tail| tail.chars().all(|c| c.is_ascii_digit()))
    };
    let reference_like = reference_like(name) || matches!(lower.as_str(), "true" | "false") || lower.strip_prefix('r').is_some_and(|rest| r1c1(rest, 'c')) || lower.strip_prefix('c').is_some_and(|rest| rest.chars().all(|c| c.is_ascii_digit()));
    let plain = name.chars().next().is_some_and(|c| c.is_alphabetic() || c == '_') && name.chars().all(|c| c.is_alphanumeric() || matches!(c, '_' | '.')) && !reference_like;
    if plain { name.to_string() } else { format!("'{}'", name.replace('\'', "''")) }
}


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn reference_like(name: &str) -> bool {
    let letters = name.trim_end_matches(|c: char| c.is_ascii_digit());
    let digits = &name[letters.len()..];
    letters.len() <= 3 && column_index(letters).is_some_and(|col| col <= XLSX_MAX_COLUMN) && digits.parse::<u32>().is_ok_and(|row| (1..=XLSX_MAX_ROW).contains(&row))
}

/// 🏷️ Rewrites, one `SetCell` per changed formula and each addressed in the state its predecessors leave behind, every formula of `next` that names sheet `from`; formulas on `skip` are left alone.
fn reference_rewrites(next: &mut XlsxSnapshot, from: &str, to: Option<&str>, skip: Option<&str>) -> Option<Vec<XlsxMutation>> {
    let mut rewrites = Vec::new();
    for sheet in next.project_workbook().ok()?.sheets {
        if skip == Some(sheet.name.as_str()) {
            continue;
        }
        for cell in &sheet.cells {
            let XlsxCellValue::Formula { expr, cached } = &cell.value else { continue };
            let rewritten = rewrite_sheet_references(expr, from, to);
            if rewritten == *expr {
                continue;
            }
            let mutation = XlsxMutation::SetCell(set_cell::SetCell { address: cell_address::xlsx_cell_address(next, &sheet.name, cell.row, cell.col).ok()?, value: XlsxCellValue::Formula { expr: rewritten, cached: cached.clone() }, node: None });
            apply_checked(next, &mutation)?;
            rewrites.push(mutation);
        }
    }
    Some(rewrites)
}

/// 🏷️ The rename of sheet `name` to `new_name` followed by one `SetCell` for each formula of the workbook that named the old sheet, so a single command renames and re-points and its inverse restores both.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn xlsx_rename_sheet_mutations(snapshot: &XlsxSnapshot, name: &str, new_name: &str) -> Result<Vec<XlsxMutation>, String> {
    let rename = XlsxMutation::RenameSheet(rename_sheet::RenameSheet { name: name.into(), new_name: new_name.into() });
    let mut next = snapshot.clone();
    apply_checked(&mut next, &rename).ok_or_else(|| format!("worksheet {name:?} cannot be renamed to {new_name:?}"))?;
    let rewrites = reference_rewrites(&mut next, name, Some(new_name), None).ok_or_else(|| "the workbook formulas cannot be re-pointed".to_string())?;
    Ok([vec![rename], rewrites].concat())
}

/// ➖️ One `SetCell` for each formula that named sheet `name` (its references become `#REF!`) followed by the removal of the sheet, so a single command removes and its inverse restores both.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn xlsx_remove_sheet_mutations(snapshot: &XlsxSnapshot, name: &str) -> Result<Vec<XlsxMutation>, String> {
    let remove = XlsxMutation::RemoveSheet(remove_sheet::RemoveSheet { name: name.into() });
    let mut next = snapshot.clone();
    let rewrites = reference_rewrites(&mut next, name, None, Some(name)).ok_or_else(|| "the workbook formulas cannot be re-pointed".to_string())?;
    apply_checked(&mut next, &remove).ok_or_else(|| format!("worksheet {name:?} cannot be removed"))?;
    Ok([rewrites, vec![remove]].concat())
}
//#endregion 🔖️SheetReferences
