//! 🔮️ Mutation oracle for this subset — every mutation kind the subset declares, performed by the
//! registered reference implementation so the subject's own mutation has an independent result to
//! be compared against instead of being checked against its own reading.
//!
//! The vocabulary is per SUBSET, not per artifact: two standards of the same format declare
//! different mutations, and a subset that shares an implementation with another reaches it through
//! the shared `tabular` module rather than by copying it. This subset does not — RFC 4180's own
//! optional-header convention means `has_header` never has to be reconciled against `has_headers`
//! assumptions the shared `tabular` module bakes in — so its independent reading lives here.
//!
//! @see ../🧪️oracle/🔣️.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️component.rs — the mutation vocabulary itself.

use semio_repo_test_host::Json;

//#region 🔖️Grid
/// 📥️ Independent RFC 4180 read: every record as a flat string grid, with NO header/data
/// distinction — the format draws none; `has_header` is pure metadata a caller tracks apart from
/// the bytes, exactly as `../🧬️schema/📸️snapshot/🦀️component.rs` documents its own codec.
/// `flexible(true)` because field COUNT is real per-record information, not a rectangular grid the
/// format guarantees.
#[cfg(feature = "oracles")]
pub fn read_grid(input: &[u8]) -> Result<Vec<Vec<String>>, String> {
    let mut reader = csv::ReaderBuilder::new().has_headers(false).flexible(true).from_reader(input);
    reader.records().map(|result| result.map(|record| record.iter().map(|cell| cell.to_string()).collect())).collect::<Result<Vec<_>, _>>().map_err(|error| format!("independent reader could not read a CSV record: {error}"))
}

/// 📤️ Independent RFC 4180 write: the reference writer decides its own minimal quoting, so a
/// mutated grid round-trips through it rather than through any hand-rolled escaping.
#[cfg(feature = "oracles")]
pub fn write_grid(grid: &[Vec<String>]) -> Result<Vec<u8>, String> {
    let mut writer = csv::WriterBuilder::new().flexible(true).from_writer(Vec::new());
    for record in grid {
        writer.write_record(record).map_err(|error| format!("csv row: {error}"))?;
    }
    writer.into_inner().map_err(|error| format!("csv finish: {error}"))
}
//#endregion 🔖️Grid

//#region 🔖️SpecReaders
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
fn strings(value: &Json, key: &str) -> Vec<String> {
    value
        .array(key)
        .iter()
        .map(|entry| match entry {
            Json::String(text) => text.clone(),
            _ => String::new(),
        })
        .collect()
}
#[cfg(feature = "oracles")]
fn rows(value: &Json, key: &str) -> Vec<Vec<String>> {
    value
        .array(key)
        .iter()
        .map(|row| match row {
            Json::Array(cells) => cells
                .iter()
                .map(|cell| match cell {
                    Json::String(text) => text.clone(),
                    _ => String::new(),
                })
                .collect(),
            _ => Vec::new(),
        })
        .collect()
}
//#endregion 🔖️SpecReaders

//#region 🔖️Dispatch
/// 🦠️ Applies one declared mutation kind to a real artifact and returns the re-serialized bytes.
/// An unrecognised kind is an error, never a silent no-op: a mutation that is quietly skipped
/// reports as a passing test. `set-has-header` deliberately returns `input` unchanged — RFC 4180
/// carries no header/data distinction on the wire, so toggling the convention never touches a byte;
/// the caller (this case's adapter) carries the toggled flag into the comparison projection instead.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    let params = mutation_params(spec);
    match spec.str("kind").as_str() {
        "" => Err("mutation spec carries no `kind`".to_string()),
        "no-mutation" => Ok(input.to_vec()),
        "set-has-header" => Ok(input.to_vec()),
        "set-snapshot" => write_grid(&rows(&params, "rows")),
        "insert-record" => {
            let mut grid = read_grid(input)?;
            let index = number(&params, "index").ok_or("insert-record: missing `index`")? as usize;
            let record = strings(&params, "fields");
            grid.insert(index.min(grid.len()), record);
            write_grid(&grid)
        }
        "remove-record" => {
            let mut grid = read_grid(input)?;
            let index = number(&params, "index").ok_or("remove-record: missing `index`")? as usize;
            if index >= grid.len() {
                return Err(format!("remove-record: index {index} out of bounds ({} record(s))", grid.len()));
            }
            grid.remove(index);
            write_grid(&grid)
        }
        "set-field" => {
            let mut grid = read_grid(input)?;
            let record_index = number(&params, "recordIndex").ok_or("set-field: missing `recordIndex`")? as usize;
            let field_index = number(&params, "fieldIndex").ok_or("set-field: missing `fieldIndex`")? as usize;
            let value = params.str("value");
            let record_count = grid.len();
            let record = grid.get_mut(record_index).ok_or_else(|| format!("set-field: record index {record_index} out of bounds ({record_count} record(s))"))?;
            if field_index >= record.len() {
                record.resize(field_index + 1, String::new());
            }
            record[field_index] = value;
            write_grid(&grid)
        }
        kind => Err(format!("mutation kind {kind:?} has no oracle implementation ({} input byte(s))", input.len())),
    }
}

/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch

//#region 🔖️Projection
/// 👁️ Projects CSV bytes plus the caller-tracked `has_header` flag (pure metadata the bytes alone
/// never carry, see `read_grid`) with the INDEPENDENT reader onto the `semantic-tabular-v1` shape
/// this case's oracle and subject are both compared through.
#[cfg(feature = "oracles")]
pub fn project_csv_grid(bytes: &[u8], has_header: bool) -> Result<Json, String> {
    let grid = read_grid(bytes)?;
    Ok(Json::Object(vec![
        ("format".to_string(), Json::String("csv".to_string())),
        ("hasHeader".to_string(), Json::Bool(has_header)),
        ("recordCount".to_string(), Json::Number(grid.len() as f64)),
        ("records".to_string(), Json::Array(grid.into_iter().map(|record| Json::Array(record.into_iter().map(Json::String).collect())).collect())),
    ]))
}

#[cfg(not(feature = "oracles"))]
pub fn project_csv_grid(_bytes: &[u8], _has_header: bool) -> Result<Json, String> {
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

    #[test]
    fn no_mutation_is_a_true_byte_identity() {
        let input = b"a,b\n1,2\n";
        let output = oracle_apply_mutation(input, &spec("no-mutation", Json::Object(vec![]))).unwrap();
        assert_eq!(output, input);
    }

    #[test]
    fn insert_and_remove_record_are_inverse_on_a_real_shaped_grid() {
        let input = b"id,name\n1,Alpha\n2,Beta\n";
        let inserted =
            oracle_apply_mutation(input, &spec("insert-record", Json::Object(vec![("index".to_string(), Json::Number(1.0)), ("fields".to_string(), Json::Array(vec![Json::String("9".to_string()), Json::String("Neu".to_string())]))]))).unwrap();
        assert_eq!(read_grid(&inserted).unwrap(), vec![vec!["id".to_string(), "name".to_string()], vec!["9".to_string(), "Neu".to_string()], vec!["1".to_string(), "Alpha".to_string()], vec!["2".to_string(), "Beta".to_string()]]);

        let removed = oracle_apply_mutation(&inserted, &spec("remove-record", Json::Object(vec![("index".to_string(), Json::Number(1.0))]))).unwrap();
        assert_eq!(read_grid(&removed).unwrap(), read_grid(input).unwrap());
    }

    #[test]
    fn set_field_patches_a_single_cell_and_requotes_when_needed() {
        let input = b"id,note\n1,plain\n";
        let output =
            oracle_apply_mutation(input, &spec("set-field", Json::Object(vec![("recordIndex".to_string(), Json::Number(1.0)), ("fieldIndex".to_string(), Json::Number(1.0)), ("value".to_string(), Json::String("has, comma".to_string()))]))).unwrap();
        let text = String::from_utf8(output.clone()).unwrap();
        assert!(text.contains("\"has, comma\""), "a value containing a comma must come back quoted, got {text:?}");
        assert_eq!(read_grid(&output).unwrap()[1][1], "has, comma");
    }

    #[test]
    fn set_has_header_is_a_true_byte_identity() {
        let input = b"a,b\n1,2\n";
        let output = oracle_apply_mutation(input, &spec("set-has-header", Json::Object(vec![("hasHeader".to_string(), Json::Bool(false))]))).unwrap();
        assert_eq!(output, input);
    }

    #[test]
    fn project_csv_grid_carries_the_caller_tracked_header_flag() {
        let bytes = b"a,b\n1,2\n";
        let with_header = project_csv_grid(bytes, true).unwrap();
        let without_header = project_csv_grid(bytes, false).unwrap();
        assert_eq!(with_header.str("format"), "csv");
        assert_ne!(with_header, without_header, "the two hasHeader states must project differently even though the bytes are identical");
    }

    #[test]
    fn unknown_kind_is_an_error_never_a_silent_no_op() {
        let input = b"a,b\n1,2\n";
        let result = oracle_apply_mutation(input, &spec("not-a-real-kind", Json::Object(vec![])));
        assert!(result.is_err(), "an unrecognised kind must fail loudly");
    }
}
//#endregion 🧪️Tests
