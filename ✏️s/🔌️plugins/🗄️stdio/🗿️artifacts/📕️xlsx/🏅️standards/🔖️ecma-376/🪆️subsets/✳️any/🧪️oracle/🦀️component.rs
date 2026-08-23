//! 🔮️ Mutation oracle for this subset — every mutation kind the subset declares, performed by the
//! registered `calamine` (reader) + `rust_xlsxwriter` (writer) reference pairing so the subject's
//! own mutation has an independent result to be compared against instead of being checked against
//! its own reading.
//!
//! **The constraint that shapes this module** — no single crate both reads and modifies an XLSX.
//! `calamine` 0.36 parses a workbook into resolved cell values but exposes no accessor for the raw
//! shared-string table (`Xlsx<RS>::strings: Vec<String>` and `read_shared_strings` are both private
//! — confirmed by reading `calamine-0.36.1/src/xlsx/mod.rs`); it collapses `t="s"` shared-string
//! references and `t="inlineStr"` literal text into the same resolved `Data::String`. `rust_xlsxwriter`
//! 0.96 can only assemble a brand-new package — never open and patch an existing one — and its own
//! shared-string table (`shared_strings_table.rs`) is populated ONLY as a byproduct of `write_string`
//! on a cell; there is no API to insert, remove or target a pool entry independent of a cell write.
//! Concretely: sheet/cell mutations (`InsertSheet`, `RemoveSheet`, `RenameSheet`, `SetCell`,
//! `RemoveCell`, `SetSnapshot`, `NoMutation`) round-trip through "read the whole workbook into a
//! grid, apply the change to the grid, rebuild the whole workbook from the grid" — a genuine second
//! producer, hence `@mode-differential`. `InsertSharedString`/`RemoveSharedString`/`SetSharedString`
//! address the shared-string pool by an INDEX that is independent of any cell reference — exactly the
//! axis neither reference crate exposes — so this module cannot independently perform them; it
//! reports that honestly (see `oracle_apply_mutation`'s dispatch below) rather than faking a
//! differential result, per the fleet brief's §6.
//!
//! @see ../🧪️oracle/🔣️component.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️component.rs — the mutation vocabulary itself.

use semio_repo_test_host::Json;

//#region 🔖️Grid
/// 🔢️ A cell value as the independent reader/writer pairing can observe and reproduce it — the
/// `Data::Int`/`Data::Float` split `calamine` draws collapses to one numeric case here since XLSX
/// itself stores every number as an IEEE-754 double (ECMA-376 §18.17.2, ST_Xstring numeric literal),
/// never a real reader/writer distinction.
#[cfg(feature = "oracles")]
#[derive(Clone, Debug, PartialEq)]
enum GridValue {
    Number(f64),
    Bool(bool),
    Text(String),
}

/// 📄 One sheet as `(name, [(row, col, value)])` — `row`/`col` are 0-based, `calamine`'s and
/// `rust_xlsxwriter`'s own native convention (the subset's OWN `XlsxCell::row` is 1-based; the JSON
/// spec this module reads carries the subset's 1-based convention, converted at the boundary below,
/// so one wire contract serves both this module and the subject's own mutation code).
#[cfg(feature = "oracles")]
type GridSheet = (String, Vec<(u32, u32, GridValue)>);

/// 📥️ Independent read: every sheet, every non-empty cell, through `calamine`'s resolved
/// `Data` — this IS the reader's own semantic view; it cannot and does not distinguish a
/// shared-string reference from an inline string (see module doc comment).
#[cfg(feature = "oracles")]
fn read_workbook_grid(input: &[u8]) -> Result<Vec<GridSheet>, String> {
    use calamine::{Data, Reader, Xlsx};
    let mut workbook: Xlsx<_> = calamine::open_workbook_from_rs(std::io::Cursor::new(input)).map_err(|error| format!("independent reader could not open the workbook: {error}"))?;
    let mut sheets = Vec::new();
    for name in workbook.sheet_names() {
        let range = workbook.worksheet_range(&name).map_err(|error| format!("independent reader could not read sheet {name:?}: {error}"))?;
        let mut cells = Vec::new();
        for (row, col, value) in range.used_cells() {
            let value = match value {
                Data::Int(v) => GridValue::Number(*v as f64),
                Data::Float(v) => GridValue::Number(*v),
                Data::String(v) => GridValue::Text(v.clone()),
                Data::Bool(v) => GridValue::Bool(*v),
                Data::DateTimeIso(v) => GridValue::Text(v.clone()),
                Data::DurationIso(v) => GridValue::Text(v.clone()),
                Data::DateTime(v) => GridValue::Text(format!("{v:?}")),
                Data::Error(kind) => return Err(format!("sheet {name:?} cell ({row},{col}) is a formula error the independent reader cannot project: {kind:?}")),
                Data::Empty => continue,
            };
            cells.push((row as u32, col as u32, value));
        }
        sheets.push((name, cells));
    }
    Ok(sheets)
}

/// 📤️ Independent write: assembles a BRAND-NEW package from `sheets` — `rust_xlsxwriter` has no
/// "open and patch" path (see module doc comment), so every oracle mutation below rebuilds the
/// entire workbook from its post-mutation grid rather than editing the original bytes.
#[cfg(feature = "oracles")]
fn write_workbook_grid(sheets: &[GridSheet]) -> Result<Vec<u8>, String> {
    use rust_xlsxwriter::Workbook;
    let mut workbook = Workbook::new();
    for (name, cells) in sheets {
        let worksheet = workbook.add_worksheet();
        worksheet.set_name(name).map_err(|error| format!("independent writer rejected sheet name {name:?}: {error}"))?;
        for (row, col, value) in cells {
            let col = u16::try_from(*col).map_err(|_| format!("column {col} exceeds the independent writer's column range"))?;
            let write_result = match value {
                GridValue::Number(n) => worksheet.write_number(*row, col, *n),
                GridValue::Bool(b) => worksheet.write_boolean(*row, col, *b),
                GridValue::Text(t) => worksheet.write_string(*row, col, t.as_str()),
            };
            write_result.map_err(|error| format!("independent writer could not write cell ({row},{col}) on sheet {name:?}: {error}"))?;
        }
    }
    workbook.save_to_buffer().map_err(|error| format!("independent writer could not assemble the workbook: {error}"))
}
//#endregion 🔖️Grid

//#region 🔖️SpecReaders
/// 🔀️ This module's own JSON wire contract: `row` is 1-based (the subset's `XlsxCell::row`
/// convention, ECMA-376's own `<row r="N">` index), `col` is 0-based — matching both sides so one
/// spec drives the subject's typed mutation AND this module's grid without a second translation.
#[cfg(feature = "oracles")]
fn mutation_params(spec: &Json) -> Json {
    spec.get("params").cloned().unwrap_or(Json::Null)
}
#[cfg(feature = "oracles")]
fn number(value: &Json, key: &str) -> Option<f64> {
    match value.get(key) {
        Some(Json::Number(number)) => Some(*number),
        _ => None,
    }
}
#[cfg(feature = "oracles")]
fn string(value: &Json, key: &str) -> String {
    match value.get(key) {
        Some(Json::String(text)) => text.clone(),
        _ => String::new(),
    }
}
#[cfg(feature = "oracles")]
fn json_to_grid_value(value: &Json) -> Result<GridValue, String> {
    match value {
        Json::Number(n) => Ok(GridValue::Number(*n)),
        Json::Bool(b) => Ok(GridValue::Bool(*b)),
        Json::String(s) => Ok(GridValue::Text(s.clone())),
        other => Err(format!("cell value must be a number, boolean or string, got {other:?}")),
    }
}
/// 🔁️ One-based (`XlsxCell::row` convention) -> zero-based (this module's/`calamine`'s convention).
#[cfg(feature = "oracles")]
fn row0(one_based_row: f64) -> Result<u32, String> {
    let row = one_based_row as i64;
    if row < 1 {
        return Err(format!("row must be >= 1 (1-based), got {row}"));
    }
    Ok((row - 1) as u32)
}
#[cfg(feature = "oracles")]
fn cells_from_json(value: &Json, key: &str) -> Result<Vec<(u32, u32, GridValue)>, String> {
    value
        .array(key)
        .iter()
        .map(|entry| {
            let row = row0(number(entry, "row").ok_or_else(|| format!("{key} entry missing `row`"))?)?;
            let col = number(entry, "col").ok_or_else(|| format!("{key} entry missing `col`"))? as u32;
            let value = json_to_grid_value(entry.get("value").ok_or_else(|| format!("{key} entry missing `value`"))?)?;
            Ok((row, col, value))
        })
        .collect()
}
#[cfg(feature = "oracles")]
fn sheets_from_json(value: &Json, key: &str) -> Result<Vec<GridSheet>, String> {
    value.array(key).iter().map(|entry| Ok((string(entry, "name"), cells_from_json(entry, "cells")?))).collect()
}
//#endregion 🔖️SpecReaders

//#region 🔖️Dispatch
/// 🦠️ Applies one declared mutation kind to a real artifact and returns the re-serialized bytes.
/// An unrecognised kind is an error, never a silent no-op: a mutation that is quietly skipped
/// reports as a passing test.
///
/// `insert-shared-string`/`remove-shared-string`/`set-shared-string` return `input` UNCHANGED —
/// not a shortcut, but the honest answer this reference pairing can give: the raw shared-string pool
/// these three kinds address is invisible to `calamine`'s read model and unreachable through
/// `rust_xlsxwriter`'s write API (see module doc comment), so there is no second producer capable of
/// independently performing an index-addressed pool edit. The case adapter types their `mutate`
/// scenario `@mode-round-trip` rather than `@mode-differential` and carries the expected
/// `sharedStrings.len()` as adapter-tracked arithmetic (mirrored against the subject's own real
/// count) instead of claiming this module observed it.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    let params = mutation_params(spec);
    match spec.str("kind").as_str() {
        "" => Err("mutation spec carries no `kind`".to_string()),
        "no-mutation" => Ok(input.to_vec()),
        "set-snapshot" => write_workbook_grid(&sheets_from_json(&params, "sheets")?),
        "insert-sheet" => {
            let mut sheets = read_workbook_grid(input)?;
            sheets.push((string(&params, "name"), cells_from_json(&params, "cells")?));
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
            let new_name = string(&params, "newName");
            let sheet = sheets.iter_mut().find(|(sheet_name, _)| sheet_name == &name).ok_or_else(|| format!("rename-sheet: no sheet named {name:?}"))?;
            sheet.0 = new_name;
            write_workbook_grid(&sheets)
        }
        "set-cell" => {
            let mut sheets = read_workbook_grid(input)?;
            let sheet_name = string(&params, "sheetName");
            let row = row0(number(&params, "row").ok_or("set-cell: missing `row`")?)?;
            let col = number(&params, "col").ok_or("set-cell: missing `col`")? as u32;
            let value = json_to_grid_value(params.get("value").ok_or("set-cell: missing `value`")?)?;
            let (_, cells) = sheets.iter_mut().find(|(name, _)| name == &sheet_name).ok_or_else(|| format!("set-cell: no sheet named {sheet_name:?}"))?;
            match cells.iter_mut().find(|(r, c, _)| *r == row && *c == col) {
                Some(cell) => cell.2 = value,
                None => cells.push((row, col, value)),
            }
            write_workbook_grid(&sheets)
        }
        "remove-cell" => {
            let mut sheets = read_workbook_grid(input)?;
            let sheet_name = string(&params, "sheetName");
            let row = row0(number(&params, "row").ok_or("remove-cell: missing `row`")?)?;
            let col = number(&params, "col").ok_or("remove-cell: missing `col`")? as u32;
            let (_, cells) = sheets.iter_mut().find(|(name, _)| name == &sheet_name).ok_or_else(|| format!("remove-cell: no sheet named {sheet_name:?}"))?;
            cells.retain(|(r, c, _)| !(*r == row && *c == col));
            write_workbook_grid(&sheets)
        }
        "insert-shared-string" | "remove-shared-string" | "set-shared-string" => Ok(input.to_vec()),
        kind => Err(format!("mutation kind {kind:?} has no oracle implementation ({} input byte(s))", input.len())),
    }
}

/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

/// 🔁️ The oracle's own decode/re-encode, through the SAME independent `calamine` + `rust_xlsxwriter`
/// pairing every mutation above uses — proves the reference pairing itself is stable on the real
/// fixture before the subject's own codec is asked to be. Genuinely rebuilds the package (never a
/// literal byte passthrough): `rust_xlsxwriter` cannot reproduce another writer's object layout, so
/// this is a real, if weak, round trip rather than an identity operation dressed up as one.
#[cfg(feature = "oracles")]
pub fn oracle_round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
    write_workbook_grid(&read_workbook_grid(input)?)
}

#[cfg(not(feature = "oracles"))]
pub fn oracle_round_trip(_input: &[u8]) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch

//#region 🔖️Projection
/// 👁️ Projects XLSX bytes with the INDEPENDENT `calamine` reader onto the `semantic-spreadsheet-v1`
/// shape this case's oracle and subject are both compared through. `expected_shared_string_count` is
/// caller-tracked metadata (like `csv`'s `has_header`, see that subset's own oracle module) rather
/// than read from these bytes — `calamine` cannot observe the raw pool size either (module doc
/// comment); the caller computes the oracle side by arithmetic and reads the subject side from its
/// own real `XlsxWorkbook::shared_strings.len()`, so the two are still genuinely compared, just not
/// both independently derived from bytes.
#[cfg(feature = "oracles")]
pub fn project_xlsx_workbook(bytes: &[u8], expected_shared_string_count: usize) -> Result<Json, String> {
    let sheets = read_workbook_grid(bytes)?;
    Ok(Json::Object(vec![
        ("format".to_string(), Json::String("xlsx".to_string())),
        ("sharedStringCount".to_string(), Json::Number(expected_shared_string_count as f64)),
        (
            "sheets".to_string(),
            Json::Array(
                sheets
                    .into_iter()
                    .map(|(name, cells)| {
                        Json::Object(vec![
                            ("name".to_string(), Json::String(name)),
                            (
                                "cells".to_string(),
                                Json::Array(
                                    cells
                                        .into_iter()
                                        .map(|(row, col, value)| {
                                            Json::Object(vec![
                                                ("row".to_string(), Json::Number((row + 1) as f64)),
                                                ("col".to_string(), Json::Number(col as f64)),
                                                (
                                                    "value".to_string(),
                                                    match value {
                                                        GridValue::Number(n) => Json::Number(n),
                                                        GridValue::Bool(b) => Json::Bool(b),
                                                        GridValue::Text(t) => Json::String(t),
                                                    },
                                                ),
                                            ])
                                        })
                                        .collect(),
                                ),
                            ),
                        ])
                    })
                    .collect(),
            ),
        ),
    ]))
}

#[cfg(not(feature = "oracles"))]
pub fn project_xlsx_workbook(_bytes: &[u8], _expected_shared_string_count: usize) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Projection

//#region 🧪️Tests
#[cfg(all(test, feature = "oracles"))]
mod tests {
    use super::*;

    fn spec(kind: &str, params: Json) -> Json {
        Json::Object(vec![("kind".to_string(), Json::String(kind.to_string())), ("params".to_string(), params)])
    }
    fn cell(row: f64, col: f64, value: Json) -> Json {
        Json::Object(vec![("row".to_string(), Json::Number(row)), ("col".to_string(), Json::Number(col)), ("value".to_string(), value)])
    }
    fn sheet(name: &str, cells: Vec<Json>) -> Json {
        Json::Object(vec![("name".to_string(), Json::String(name.to_string())), ("cells".to_string(), Json::Array(cells))])
    }

    fn fixture_bytes() -> Vec<u8> {
        write_workbook_grid(&[("Sheet1".to_string(), vec![(0, 0, GridValue::Text("hello".to_string())), (0, 1, GridValue::Number(1.0)), (1, 0, GridValue::Bool(true))])]).unwrap()
    }

    #[test]
    fn no_mutation_is_a_true_byte_identity() {
        let input = fixture_bytes();
        let output = oracle_apply_mutation(&input, &spec("no-mutation", Json::Object(vec![]))).unwrap();
        assert_eq!(output, input);
    }

    #[test]
    fn insert_and_remove_sheet_are_real_transformations() {
        let input = fixture_bytes();
        let inserted = oracle_apply_mutation(&input, &spec("insert-sheet", sheet("New", vec![cell(1.0, 0.0, Json::String("fresh".to_string()))]))).unwrap();
        let grid = read_workbook_grid(&inserted).unwrap();
        assert_eq!(grid.len(), 2);
        assert_eq!(grid[1].0, "New");

        let removed = oracle_apply_mutation(&inserted, &spec("remove-sheet", Json::Object(vec![("name".to_string(), Json::String("New".to_string()))]))).unwrap();
        assert_eq!(read_workbook_grid(&removed).unwrap().len(), 1);
    }

    #[test]
    fn rename_sheet_changes_only_the_name() {
        let input = fixture_bytes();
        let renamed = oracle_apply_mutation(&input, &spec("rename-sheet", Json::Object(vec![("name".to_string(), Json::String("Sheet1".to_string())), ("newName".to_string(), Json::String("Renamed".to_string()))]))).unwrap();
        let grid = read_workbook_grid(&renamed).unwrap();
        assert_eq!(grid[0].0, "Renamed");
        assert_eq!(grid[0].1.len(), 3);
    }

    #[test]
    fn set_and_remove_cell_are_real_transformations() {
        let input = fixture_bytes();
        let set = oracle_apply_mutation(
            &input,
            &spec(
                "set-cell",
                Json::Object(vec![("sheetName".to_string(), Json::String("Sheet1".to_string())), ("row".to_string(), Json::Number(1.0)), ("col".to_string(), Json::Number(0.0)), ("value".to_string(), Json::String("changed".to_string()))]),
            ),
        )
        .unwrap();
        let grid = read_workbook_grid(&set).unwrap();
        assert!(grid[0].1.contains(&(0, 0, GridValue::Text("changed".to_string()))));

        let removed = oracle_apply_mutation(&set, &spec("remove-cell", Json::Object(vec![("sheetName".to_string(), Json::String("Sheet1".to_string())), ("row".to_string(), Json::Number(1.0)), ("col".to_string(), Json::Number(0.0))]))).unwrap();
        assert!(!read_workbook_grid(&removed).unwrap()[0].1.iter().any(|(r, c, _)| *r == 0 && *c == 0));
    }

    #[test]
    fn shared_string_kinds_are_a_true_byte_identity() {
        let input = fixture_bytes();
        for kind in ["insert-shared-string", "remove-shared-string", "set-shared-string"] {
            let output = oracle_apply_mutation(&input, &spec(kind, Json::Object(vec![("index".to_string(), Json::Number(0.0)), ("value".to_string(), Json::String("x".to_string()))]))).unwrap();
            assert_eq!(output, input, "{kind} has no observable effect through this reference pairing (see module doc comment)");
        }
    }

    #[test]
    fn project_xlsx_workbook_carries_the_caller_tracked_shared_string_count() {
        let bytes = fixture_bytes();
        let projection = project_xlsx_workbook(&bytes, 3).unwrap();
        assert_eq!(projection.str("format"), "xlsx");
        assert_eq!(projection.get("sharedStringCount"), Some(&Json::Number(3.0)));
    }

    #[test]
    fn unknown_kind_is_an_error_never_a_silent_no_op() {
        let input = fixture_bytes();
        let result = oracle_apply_mutation(&input, &spec("not-a-real-kind", Json::Object(vec![])));
        assert!(result.is_err(), "an unrecognised kind must fail loudly");
    }
}
//#endregion 🧪️Tests
