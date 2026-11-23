//! 🔭️ Third-party witnesses of the XLSX oracle: laws read back through `calamine`, a reader this module neither wrote nor drives. See [`super`].

use super::*;

//#region 🔖️SharedStringWitness
/// 🔭️ The index-addressed shared-string kinds judged by the one reader that never saw their code path: `calamine` resolves every `t="s"` cell through the pool it
/// parses out of the package itself, so a pool edit is observable exactly where a worksheet cell references the edited entry.
///
/// - `set-shared-string` must change every resolved cell whose text is the replaced entry, to the new text, and nothing else.
/// - `insert-shared-string` and `remove-shared-string` (the arranged, unreferenced entry) must leave every resolved cell exactly as it was: an entry no cell
///   references is invisible to a reader that resolves cells, so any visible movement is a leak.
///
/// 🪞️ What this cannot see is an entry no cell references, which is the one thing the insert and remove kinds move; `calamine` keeps that table private. That residue
/// is the committed openpyxl string table of `🕳️unreferenced-shared-string`: `read_string_table` reads every `si`, and the pool reader is held to that list.
pub fn calamine_shared_string_law(before: &[u8], after: &[u8], forward: &Json) -> Result<(), String> {
    let kind = forward.str("kind");
    let seen_before = read_workbook_grid(before)?;
    let seen_after = read_workbook_grid(after)?;
    let expected = match kind.as_str() {
        "insert-shared-string" | "remove-shared-string" => seen_before,
        "set-shared-string" => {
            let pool = shared_strings::read_pool(before)?;
            let params = mutation_params(forward);
            let index = number(&params, "index").ok_or("set-shared-string: missing `index`")?.max(0.0) as usize;
            let old = pool.get(index).ok_or_else(|| format!("set-shared-string: index {index} is outside the {}-entry pool", pool.len()))?.clone();
            if pool.iter().filter(|entry| **entry == old).count() != 1 {
                return Err(format!("set-shared-string: {old:?} is held by more than one pool entry, so a resolved cell cannot be attributed to index {index}"));
            }
            let new = string(&params, "value");
            seen_before.into_iter().map(|(name, cells)| (name, cells.into_iter().map(|(row, col, value)| (row, col, if value == GridValue::Text(old.clone()) { GridValue::Text(new.clone()) } else { value })).collect())).collect()
        }
        other => return Err(format!("{other:?} is not one of this catalog's shared-string kinds")),
    };
    if expected != seen_after {
        let first = expected.iter().zip(seen_after.iter()).find_map(|(left, right)| (left != right).then(|| format!("sheet {:?} reads differently through calamine", left.0)));
        return Err(format!("{kind}: the resolved cells calamine reads are not what the pool edit implies — {}", first.unwrap_or_else(|| format!("{} sheet(s) became {}", expected.len(), seen_after.len()))));
    }
    Ok(())
}

/// 👀️ How many resolved cells `calamine` reads from `bytes` that carry `text` — the observable a `set-shared-string` is judged by.
pub fn calamine_cells_reading(bytes: &[u8], text: &str) -> Result<usize, String> {
    Ok(read_workbook_grid(bytes)?.iter().map(|(_, cells)| cells.iter().filter(|(_, _, value)| *value == GridValue::Text(text.to_string())).count()).sum())
}
//#endregion 🔖️SharedStringWitness
