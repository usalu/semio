//! 🧮️ Mutation dispatch of the XLSX oracle: the forward mutation, the arrange, the inverse and the round trip. See [`super`].

use super::*;

//#region 🔖️Dispatch
/// 🦠️ Applies one declared mutation kind to a real artifact and returns the re-serialized bytes.
/// An unrecognised kind is an error, never a silent no-op: a mutation that is quietly skipped
/// reports as a passing test.
///
/// `insert-shared-string`/`remove-shared-string`/`set-shared-string` do NOT go through
/// `calamine`/`rust_xlsxwriter`: the raw pool those three address is invisible to the first's read
/// model and unreachable by index through the second's write API. They go through the `zip` +
/// `quick-xml` pairing instead (see the [`shared_strings`] module), which reads and rewrites
/// `xl/sharedStrings.xml` as the OPC PART it is.
pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    let params = mutation_params(spec);
    match spec.str("kind").as_str() {
        "" => Err("mutation spec carries no `kind`".to_string()),
        "insert-sheet" => {
            let mut sheets = read_workbook_grid(input)?;
            sheets.push(sheet_of(params.get("sheet").ok_or("insert-sheet: missing `sheet`")?)?);
            write_workbook_grid(&sheets)
        }
        "remove-sheet" => {
            let mut sheets = read_workbook_grid(input)?;
            let name = string(&params, "name");
            let before = sheets.len();
            sheets.retain(|(sheet_name, _)| sheet_name != &name);
            if sheets.len() == before {
                return Err(format!("remove-sheet: no sheet named {name:?}"));
            }
            write_workbook_grid(&sheets)
        }
        "rename-sheet" => {
            let mut sheets = read_workbook_grid(input)?;
            let name = string(&params, "name");
            let sheet = sheets.iter_mut().find(|(sheet_name, _)| sheet_name == &name).ok_or_else(|| format!("rename-sheet: no sheet named {name:?}"))?;
            sheet.0 = string(&params, "newName");
            write_workbook_grid(&sheets)
        }
        "set-cell" => {
            let (sheet_name, row, col) = cell_address::addressed_cell(input, params.get("address").ok_or("set-cell: missing `address`")?)?;
            let value = cell_value(params.get("value").ok_or("set-cell: missing `value`")?)?;
            let mut sheets = read_workbook_grid(input)?;
            put_cell(&mut sheets, &sheet_name, row, col, Some(value))?;
            write_workbook_grid(&sheets)
        }
        "remove-cell" => {
            let (sheet_name, row, col) = cell_address::addressed_cell(input, params.get("address").ok_or("remove-cell: missing `address`")?)?;
            let mut sheets = read_workbook_grid(input)?;
            put_cell(&mut sheets, &sheet_name, row, col, None)?;
            write_workbook_grid(&sheets)
        }
        "insert-shared-string" => {
            let mut pool = shared_strings::read_pool(input)?;
            pool.push(string(&params, "value"));
            shared_strings::write_pool(input, &pool)
        }
        "remove-shared-string" => {
            let mut pool = shared_strings::read_pool(input)?;
            let index = number(&params, "index").ok_or("remove-shared-string: missing `index`")?.max(0.0) as usize;
            if index >= pool.len() {
                return Err(format!("remove-shared-string: index {index} is outside the {}-entry pool", pool.len()));
            }
            if shared_strings::is_referenced(input, index)? {
                return Err(format!("remove-shared-string: pool entry {index} is still referenced by a cell"));
            }
            pool.remove(index);
            shared_strings::write_pool(input, &pool)
        }
        "set-shared-string" => {
            let mut pool = shared_strings::read_pool(input)?;
            let index = number(&params, "index").ok_or("set-shared-string: missing `index`")?.max(0.0) as usize;
            let value = string(&params, "value");
            let slot = pool.get_mut(index).ok_or_else(|| format!("set-shared-string: index {index} is outside the pool"))?;
            if *slot == value {
                return Err(format!("set-shared-string: the pool already holds {value:?} at index {index} — the mutation would be unobservable"));
            }
            *slot = value;
            shared_strings::write_pool(input, &pool)
        }
        kind @ ("set-relationship" | "remove-relationship" | "set-content-type" | "remove-content-type") => plumbing::apply(input, kind, &params),
        kind => Err(format!("mutation kind {kind:?} has no oracle implementation ({} input byte(s))", input.len())),
    }
}

/// 🎬️ The real pre-state a kind runs on. `remove-shared-string` needs a pool entry no cell references, and every entry of
/// the real workbook's pool is referenced, so for that kind this is the real package after the reference has appended one
/// unreferenced entry — the removal under test is still the reference's own, on genuine OPC. Every other kind reads the
/// committed bytes untouched.
pub fn oracle_arrange(input: &[u8], forward: &Json) -> Result<Vec<u8>, String> {
    match forward.str("kind").as_str() {
        "remove-shared-string" => {
            let mut pool = shared_strings::read_pool(input)?;
            pool.push(UNREFERENCED_ENTRY.to_string());
            shared_strings::write_pool(input, &pool)
        }
        _ => Ok(input.to_vec()),
    }
}

/// 🧾️ The pool entry [`oracle_arrange`] appends: text no cell of the real workbook carries.
const UNREFERENCED_ENTRY: &str = "Nicht referenzierter Eintrag";

/// ✍️ Writes (`Some`) or clears (`None`) one grid cell of the named sheet.
fn put_cell(sheets: &mut [GridSheet], sheet_name: &str, row: u32, col: u32, value: Option<GridValue>) -> Result<(), String> {
    let (_, cells) = sheets.iter_mut().find(|(name, _)| name == sheet_name).ok_or_else(|| format!("no sheet named {sheet_name:?}"))?;
    cells.retain(|(r, c, _)| !(*r == row && *c == col));
    if let Some(value) = value {
        cells.push((row, col, value));
    }
    Ok(())
}

fn applied<T>(mut value: T, step: impl FnOnce(&mut T) -> Result<(), String>) -> Result<T, String> {
    step(&mut value)?;
    Ok(value)
}

/// ↩️ Undoes `forward` on `mutated`, sourcing whatever it discarded from `original` (the package the forward kind ran on) —
/// the algebra `XlsxMutation::inverse` defines, computed independently by the reference pairing. A cell address is
/// lineage-bound to `original`, so it is resolved there and the undo applied to the rebuilt grid by coordinate; the three pool kinds go through
/// [`shared_string_inverse_spec`].
pub fn oracle_apply_inverse(original: &[u8], mutated: &[u8], forward: &Json) -> Result<Vec<u8>, String> {
    let params = mutation_params(forward);
    let kind = forward.str("kind");
    let cell_undo = |field: &str| -> Result<Vec<u8>, String> {
        let (sheet_name, row, col) = cell_address::addressed_cell(original, params.get(field).ok_or_else(|| format!("{kind}: missing `{field}`"))?)?;
        let before = read_workbook_grid(original)?.into_iter().find(|(name, _)| name == &sheet_name).and_then(|(_, cells)| cells.into_iter().find(|(r, c, _)| *r == row && *c == col)).map(|(_, _, value)| value);
        let sheets = applied(read_workbook_grid(mutated)?, |sheets| put_cell(sheets, &sheet_name, row, col, before))?;
        write_workbook_grid(&sheets)
    };
    match kind.as_str() {
        "insert-sheet" => {
            let name = params.get("sheet").map(|sheet| string(sheet, "name")).unwrap_or_default();
            let mut sheets = read_workbook_grid(mutated)?;
            sheets.retain(|(sheet_name, _)| sheet_name != &name);
            write_workbook_grid(&sheets)
        }
        "remove-sheet" => {
            let name = string(&params, "name");
            let originals = read_workbook_grid(original)?;
            let index = originals.iter().position(|(sheet_name, _)| sheet_name == &name).ok_or_else(|| format!("remove-sheet inverse: no sheet named {name:?} in the original"))?;
            let mut sheets = read_workbook_grid(mutated)?;
            sheets.insert(index.min(sheets.len()), originals[index].clone());
            write_workbook_grid(&sheets)
        }
        "rename-sheet" => {
            let new_name = string(&params, "newName");
            let mut sheets = read_workbook_grid(mutated)?;
            let sheet = sheets.iter_mut().find(|(sheet_name, _)| sheet_name == &new_name).ok_or_else(|| format!("rename-sheet inverse: no sheet named {new_name:?}"))?;
            sheet.0 = string(&params, "name");
            write_workbook_grid(&sheets)
        }
        "set-cell" | "remove-cell" => cell_undo("address"),
        "insert-shared-string" | "remove-shared-string" | "set-shared-string" => oracle_apply_mutation(mutated, &shared_string_inverse_spec(original, forward)?),
        "set-relationship" | "remove-relationship" | "set-content-type" | "remove-content-type" => plumbing::undo(original, mutated, &kind, &params),
        other => Err(format!("no inverse rule for kind {other:?}")),
    }
}

/// 🔁️ The oracle's own decode/re-encode, through the SAME independent `calamine` + `rust_xlsxwriter`
/// pairing every grid mutation above uses — proves the reference pairing itself is stable on the real
/// fixture before the subject's own codec is asked to be. Genuinely rebuilds the package (never a
/// literal byte passthrough): `rust_xlsxwriter` cannot reproduce another writer's object layout.
pub fn oracle_round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
    write_workbook_grid(&read_workbook_grid(input)?)
}
//#endregion 🔖️Dispatch
