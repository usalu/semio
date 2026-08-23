//! 🦀️ XLSX ECMA-376 exhaustive mutation round-trip case — Rust adapter.
//!
//! Every scenario copies the immutable real fixture into the case work directory first; the
//! committed file is never written to. `oracle` handlers drive the registered `calamine` +
//! `rust_xlsxwriter` reference pairing (via this subset's own `🧪️oracle/🦀️component.rs`), `subject`
//! handlers drive this repository's own decode/mutate/encode round trip, and both results are read
//! back by the SAME independent reader (`project_xlsx_workbook`) before the `semantic-spreadsheet-v1`
//! profile compares them. The subject half is gated behind the generated host's `sut` feature so the
//! oracle-only run never compiles the local implementation — see §5.3 of the fleet brief.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::xlsx::standards::v_ecma_376::subsets::any::{oracle_apply_mutation, oracle_round_trip, project_xlsx_workbook};

//#region 🔖️Kinds
/// 🧾️ Test-case-local mirror of the `xlsx-ecma-376-any` catalog. Duplicated, not imported, from
/// `../../🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs::KINDS` — that
/// module lives in the SUBJECT crate, and the oracle role must not link the subject crate at all
/// (fleet brief §5.3), while this loop registers handlers for both roles from one list. That other
/// `KINDS` carries its own test proving it matches the enum AND the catalog manifest; a mismatch
/// HERE against either one is caught structurally instead — the contract phase fails with
/// `mutation-kind-uncovered`/`mutation-kind-undeclared` if this list omits or invents a kind, and the
/// runner fails every unregistered scenario id outright (`adapter has no {role} registration`).
const KINDS: &[&str] = &["no-mutation", "set-snapshot", "insert-sheet", "remove-sheet", "rename-sheet", "set-cell", "remove-cell", "insert-shared-string", "remove-shared-string", "set-shared-string"];
//#endregion 🔖️Kinds

//#region 🔖️Input
const INPUT: &str = "shared://📕️reuse-marketplaces.xlsx";
/// 📑️ The real fixture's own baseline: `xl/sharedStrings.xml` reports `uniqueCount="229"` —
/// confirmed by unzipping the committed file, not assumed. `calamine` cannot re-derive this number
/// (see the oracle module's doc comment), so it is tracked here instead.
const BASELINE_SHARED_STRING_COUNT: usize = 229;

/// 🧫️ Copies the immutable fixture into the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("input.xlsx"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️SpecHelpers
fn json_object(pairs: Vec<(&str, Json)>) -> Json {
    Json::Object(pairs.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
}
fn kind_spec(kind: &str, params: Json) -> Json {
    json_object(vec![("kind", Json::String(kind.to_string())), ("params", params)])
}

/// 🔢️ The `sharedStringCount` this catalog's three shared-string kinds move the pool to, and the
/// value every OTHER declared kind leaves untouched — real arithmetic on the real fixture's baseline,
/// not a placeholder: `insert-sheet`/`remove-sheet`/`rename-sheet`/`set-cell`/`remove-cell` never
/// touch `shared_strings` (their `diff_*` functions all set `shared_strings: None`, confirmed by
/// reading `../../🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`); every
/// cell value this case's scenarios write is a literal (`XlsxCellValue::InlineString`/`Number`/
/// `Boolean`), never a `SharedString` reference, so `set-cell`/`insert-sheet` cannot grow the pool
/// either. `set-snapshot`'s target is ALWAYS built with an empty `shared_strings` (this case never
/// carries the pool through a JSON `sheets` replacement, matching the `@mode-differential` group's
/// own honest limit — see the Feature description), so it moves the count to exactly 0, both as the
/// forward mutation and as its own inverse.
fn shared_string_count_after(current: usize, kind: &str) -> usize {
    match kind {
        "set-snapshot" => 0,
        "insert-shared-string" => current + 1,
        "remove-shared-string" => current.saturating_sub(1),
        _ => current,
    }
}

fn sheets_array(projection: &Json) -> Vec<Json> {
    projection.array("sheets")
}
fn sheet_cells(projection: &Json, name: &str) -> Vec<Json> {
    sheets_array(projection).into_iter().find(|sheet| sheet.str("name") == name).map(|sheet| sheet.array("cells")).unwrap_or_default()
}
fn cell_json(cells: &[Json], row: f64, col: f64) -> Option<Json> {
    cells
        .iter()
        .find(|cell| matches!(cell.get("row"), Some(Json::Number(r)) if *r == row) && matches!(cell.get("col"), Some(Json::Number(c)) if *c == col))
        .cloned()
}

/// ↩️ The inverse mutation's OWN spec, computed by reading whatever pre-mutation state it needs
/// straight out of `original` through the SAME independent reader the oracle mutates with (via
/// `project_xlsx_workbook`) — never by calling this repository's own `XlsxMutation::inverse`, which
/// would defeat the point of an independently-computed oracle. Mirrors that method's documented rule
/// exactly (index-aware, reading the pre-state it needs from the ORIGINAL document), just derived
/// from real bytes instead of a typed snapshot. `insert-shared-string`/`remove-shared-string`/
/// `set-shared-string` carry a placeholder `value`/`index` — this reference pairing cannot observe
/// the raw pool either way (see the oracle module's doc comment), so only `sharedStringCount`
/// (computed by `shared_string_count_after`, chained across both steps) is genuinely asserted for
/// those three.
fn inverse_spec(original: &[u8], forward: &Json) -> Result<Json, String> {
    let params = forward.get("params").cloned().unwrap_or(Json::Null);
    let number = |key: &str| match params.get(key) {
        Some(Json::Number(value)) => Some(*value),
        _ => None,
    };
    match forward.str("kind").as_str() {
        "no-mutation" => Ok(kind_spec("no-mutation", json_object(vec![]))),
        "set-snapshot" => {
            let projection = project_xlsx_workbook(original, 0)?;
            let sheets = projection.get("sheets").cloned().unwrap_or(Json::Array(vec![]));
            Ok(kind_spec("set-snapshot", json_object(vec![("sheets", sheets)])))
        }
        "insert-sheet" => Ok(kind_spec("remove-sheet", json_object(vec![("name", Json::String(params.str("name")))]))),
        "remove-sheet" => {
            let name = params.str("name");
            let projection = project_xlsx_workbook(original, 0)?;
            if !sheets_array(&projection).iter().any(|sheet| sheet.str("name") == name) {
                return Err(format!("remove-sheet inverse: no sheet named {name:?} in the original"));
            }
            let cells = sheet_cells(&projection, &name);
            Ok(kind_spec("insert-sheet", json_object(vec![("name", Json::String(name)), ("cells", Json::Array(cells))])))
        }
        "rename-sheet" => Ok(kind_spec("rename-sheet", json_object(vec![("name", Json::String(params.str("newName"))), ("newName", Json::String(params.str("name")))]))),
        "set-cell" => {
            let sheet_name = params.str("sheetName");
            let row = number("row").ok_or("set-cell inverse: missing `row`")?;
            let col = number("col").ok_or("set-cell inverse: missing `col`")?;
            let projection = project_xlsx_workbook(original, 0)?;
            let cells = sheet_cells(&projection, &sheet_name);
            match cell_json(&cells, row, col).and_then(|cell| cell.get("value").cloned()) {
                Some(value) => Ok(kind_spec("set-cell", json_object(vec![("sheetName", Json::String(sheet_name)), ("row", Json::Number(row)), ("col", Json::Number(col)), ("value", value)]))),
                None => Ok(kind_spec("remove-cell", json_object(vec![("sheetName", Json::String(sheet_name)), ("row", Json::Number(row)), ("col", Json::Number(col))]))),
            }
        }
        "remove-cell" => {
            let sheet_name = params.str("sheetName");
            let row = number("row").ok_or("remove-cell inverse: missing `row`")?;
            let col = number("col").ok_or("remove-cell inverse: missing `col`")?;
            let projection = project_xlsx_workbook(original, 0)?;
            let cells = sheet_cells(&projection, &sheet_name);
            let value = cell_json(&cells, row, col).and_then(|cell| cell.get("value").cloned()).ok_or_else(|| format!("remove-cell inverse: no cell at ({row},{col}) on {sheet_name:?} in the original"))?;
            Ok(kind_spec("set-cell", json_object(vec![("sheetName", Json::String(sheet_name)), ("row", Json::Number(row)), ("col", Json::Number(col)), ("value", value)])))
        }
        "insert-shared-string" => Ok(kind_spec("remove-shared-string", json_object(vec![("index", Json::Number(0.0))]))),
        "remove-shared-string" => Ok(kind_spec("insert-shared-string", json_object(vec![("value", Json::String(String::new()))]))),
        "set-shared-string" => Ok(kind_spec("set-shared-string", json_object(vec![("index", Json::Number(number("index").unwrap_or(0.0))), ("value", Json::String(String::new()))]))),
        other => Err(format!("no inverse rule for kind {other:?}")),
    }
}
//#endregion 🔖️SpecHelpers

//#region 🔖️Oracle
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let output = oracle_apply_mutation(&input, &spec)?;
    let count = shared_string_count_after(BASELINE_SHARED_STRING_COUNT, &spec.str("kind"));
    let projection = project_xlsx_workbook(&output, count)?;
    Ok(Outcome::with_raw(output, projection))
}

fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let mutated = oracle_apply_mutation(&input, &spec)?;
    let undo = inverse_spec(&input, &spec)?;
    let restored = oracle_apply_mutation(&mutated, &undo)?;
    let count_after_forward = shared_string_count_after(BASELINE_SHARED_STRING_COUNT, &spec.str("kind"));
    let count_after_inverse = shared_string_count_after(count_after_forward, &undo.str("kind"));
    let projection = project_xlsx_workbook(&restored, count_after_inverse)?;
    Ok(Outcome::with_raw(restored, projection))
}

/// 🔁️ The oracle's own decode/re-encode, through the SAME independent `calamine` + `rust_xlsxwriter`
/// pairing this subset's mutations use — proves the reference pairing itself is stable on the real
/// fixture before the subject's own codec is asked to be.
fn round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let output = oracle_round_trip(&input)?;
    let projection = project_xlsx_workbook(&output, BASELINE_SHARED_STRING_COUNT)?;
    Ok(Outcome::with_raw(output, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{inverse_spec, mutable_input};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::xlsx::standards::v_ecma_376::subsets::any::io::export::serializers::{build_minimal_xlsx, encode_xlsx};
    use semio_s_plugin_stdio::artifacts::xlsx::standards::v_ecma_376::subsets::any::io::import::deserializers::decode_xlsx;
    use semio_s_plugin_stdio::artifacts::xlsx::standards::v_ecma_376::subsets::any::schema::mutations::apply_xlsx_mutation;
    use semio_s_plugin_stdio::artifacts::xlsx::standards::v_ecma_376::subsets::any::schema::snapshot::{XlsxCell, XlsxCellValue, XlsxSheet, XlsxWorkbook};
    use semio_s_plugin_stdio::artifacts::xlsx::{XlsxMutation, XlsxSnapshot};
    use semio_s_plugin_stdio_test_oracle::artifacts::xlsx::standards::v_ecma_376::subsets::any::project_xlsx_workbook;

    /// 🔀️ The same JSON mutation spec the oracle reads, turned into this repository's own typed
    /// `XlsxMutation` — the only channel between the feature's parameters and the subject's codec.
    /// Cell values are always `InlineString`/`Number`/`Boolean` literals (never a raw `SharedString`
    /// index) — this case never asks the subject to address the shared-string pool through a cell,
    /// matching the oracle's own honest limit (see the Feature description).
    fn xlsx_cell_value_from_json(value: &Json) -> Result<XlsxCellValue, String> {
        match value {
            Json::Number(n) => Ok(XlsxCellValue::Number(*n)),
            Json::Bool(b) => Ok(XlsxCellValue::Boolean(*b)),
            Json::String(s) => Ok(XlsxCellValue::InlineString(s.clone())),
            other => Err(format!("cell value must be a number, boolean or string, got {other:?}")),
        }
    }
    fn xlsx_cells_from_json(value: &Json, key: &str) -> Result<Vec<XlsxCell>, String> {
        value
            .array(key)
            .iter()
            .map(|entry| {
                let row = match entry.get("row") {
                    Some(Json::Number(n)) => *n as u32,
                    _ => return Err(format!("{key} entry missing `row`")),
                };
                let col = match entry.get("col") {
                    Some(Json::Number(n)) => *n as u32,
                    _ => return Err(format!("{key} entry missing `col`")),
                };
                let value = xlsx_cell_value_from_json(entry.get("value").ok_or_else(|| format!("{key} entry missing `value`"))?)?;
                Ok(XlsxCell { row, col, value })
            })
            .collect()
    }
    fn xlsx_sheets_from_json(value: &Json, key: &str) -> Result<Vec<XlsxSheet>, String> {
        value.array(key).iter().map(|entry| Ok(XlsxSheet { name: entry.str("name"), cells: xlsx_cells_from_json(entry, "cells")? })).collect()
    }

    fn mutation_from_spec(spec: &Json) -> Result<XlsxMutation, String> {
        let params = spec.get("params").cloned().unwrap_or(Json::Null);
        let number = |key: &str| match params.get(key) {
            Some(Json::Number(value)) => Some(*value),
            _ => None,
        };
        Ok(match spec.str("kind").as_str() {
            "no-mutation" => XlsxMutation::NoMutation,
            "set-snapshot" => {
                let sheets = xlsx_sheets_from_json(&params, "sheets")?;
                // 🩹 Always an EMPTY shared-string pool — this case's `set-snapshot` targets never
                // carry one through the JSON `sheets` shape (see `shared_string_count_after`'s doc
                // comment), matching the oracle's own honest limit.
                XlsxMutation::SetSnapshot { snapshot: build_minimal_xlsx(XlsxWorkbook { sheets, shared_strings: vec![] }) }
            }
            "insert-sheet" => XlsxMutation::InsertSheet { sheet: XlsxSheet { name: params.str("name"), cells: xlsx_cells_from_json(&params, "cells")? } },
            "remove-sheet" => XlsxMutation::RemoveSheet { name: params.str("name") },
            "rename-sheet" => XlsxMutation::RenameSheet { name: params.str("name"), new_name: params.str("newName") },
            "set-cell" => XlsxMutation::SetCell { sheet_name: params.str("sheetName"), row: number("row").ok_or("set-cell: missing `row`")? as u32, col: number("col").ok_or("set-cell: missing `col`")? as u32, value: xlsx_cell_value_from_json(params.get("value").ok_or("set-cell: missing `value`")?)? },
            "remove-cell" => XlsxMutation::RemoveCell { sheet_name: params.str("sheetName"), row: number("row").ok_or("remove-cell: missing `row`")? as u32, col: number("col").ok_or("remove-cell: missing `col`")? as u32 },
            "insert-shared-string" => XlsxMutation::InsertSharedString { value: params.str("value") },
            "remove-shared-string" => XlsxMutation::RemoveSharedString { index: number("index").ok_or("remove-shared-string: missing `index`")? as usize },
            "set-shared-string" => XlsxMutation::SetSharedString { index: number("index").ok_or("set-shared-string: missing `index`")? as usize, value: params.str("value") },
            other => return Err(format!("no subject rule for kind {other:?}")),
        })
    }

    fn decode(bytes: &[u8]) -> Result<XlsxSnapshot, String> {
        decode_xlsx(bytes).map_err(|error| error.to_string())
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let mut snapshot = decode(&mutable_input(ctx)?)?;
        let mutation = mutation_from_spec(&ctx.doc_json()?)?;
        apply_xlsx_mutation(&mut snapshot, &mutation);
        let output = encode_xlsx(&snapshot).map_err(|error| error.to_string())?;
        let projection = project_xlsx_workbook(&output, snapshot.workbook.shared_strings.len())?;
        Ok(Outcome::with_raw(output, projection))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let spec = ctx.doc_json()?;
        let mut snapshot = decode(&input)?;
        apply_xlsx_mutation(&mut snapshot, &mutation_from_spec(&spec)?);
        apply_xlsx_mutation(&mut snapshot, &mutation_from_spec(&inverse_spec(&input, &spec)?)?);
        let output = encode_xlsx(&snapshot).map_err(|error| error.to_string())?;
        let projection = project_xlsx_workbook(&output, snapshot.workbook.shared_strings.len())?;
        Ok(Outcome::with_raw(output, projection))
    }

    /// 🔁️ Full semantic parse, re-serialized from the model alone — copying, splicing or patching
    /// source bytes is cheating (fleet brief, "the point of this wave") and this tripwire catches it:
    /// this repository's encoder always rebuilds every xlsx-owned part from `XlsxWorkbook`
    /// (`regenerate_workbook_parts`), so a genuine re-encode can never coincidentally reproduce a
    /// `rust_xlsxwriter`-authored input's exact object layout/compression.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let snapshot = decode(&input)?;
        let output = encode_xlsx(&snapshot).map_err(|error| error.to_string())?;
        if output == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_xlsx_workbook(&output, snapshot.workbook.shared_strings.len())?;
        Ok(Outcome::with_raw(output, projection))
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle).oracle(&format!("inverse-{kind}"), inverse_oracle);
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate).subject(&format!("inverse-{kind}"), subject::inverse);
        }
    }
    built = built.oracle("identity-round-trip", round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
