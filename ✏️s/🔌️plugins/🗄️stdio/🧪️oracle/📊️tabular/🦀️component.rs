//! 📊️ Tabular oracles: RFC 4180 CSV creation, mutation and projection.
//!
//! The `semantic-tabular-v1` profile compares the header and the cell grid. Line terminator, quote
//! style and the presence of a trailing newline are writer choices, not normative content.
//!
//! @see 📇️registry/🔣️component.json — the approved oracle registry these functions implement.

use semio_repo_test_host::Json;

//#region 🔖️TableSpec
/// 📊️ Owned description of a table — the one input both producers are given.
#[derive(Debug, Clone)]
pub struct TableSpec {
    pub header: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

impl TableSpec {
    /// 📊️ Reads a spec out of a scenario's owned JSON payload.
    pub fn from_json(value: &Json) -> TableSpec {
        let strings = |entry: &Json| match entry {
            Json::Array(items) => items
                .iter()
                .map(|cell| match cell {
                    Json::String(text) => text.clone(),
                    Json::Number(number) => format!("{}", number),
                    _ => String::new(),
                })
                .collect(),
            _ => Vec::new(),
        };
        TableSpec {
            header: value.get("header").map(strings).unwrap_or_default(),
            rows: value.array("rows").iter().map(strings).collect(),
        }
    }

    /// 🔁️ The projection every tabular producer is compared through.
    pub fn projection(&self) -> Json {
        Json::Object(vec![
            ("format".to_string(), Json::String("csv".to_string())),
            ("header".to_string(), Json::Array(self.header.iter().map(|cell| Json::String(cell.clone())).collect())),
            ("rowCount".to_string(), Json::Number(self.rows.len() as f64)),
            ("columnCount".to_string(), Json::Number(self.header.len() as f64)),
            ("rows".to_string(), Json::Array(self.rows.iter().map(|row| Json::Array(row.iter().map(|cell| Json::String(cell.clone())).collect())).collect())),
        ])
    }
}
//#endregion 🔖️TableSpec

//#region 🔖️Csv
/// 🔮️ Writes a table as RFC 4180 CSV with the registered `csv` reference implementation.
/// @see https://github.com/BurntSushi/rust-csv
#[cfg(feature = "oracles")]
pub fn oracle_create_csv(spec: &TableSpec) -> Result<Vec<u8>, String> {
    let mut writer = csv::Writer::from_writer(Vec::new());
    writer.write_record(&spec.header).map_err(|error| format!("csv header: {}", error))?;
    for row in &spec.rows {
        writer.write_record(row).map_err(|error| format!("csv row: {}", error))?;
    }
    writer.into_inner().map_err(|error| format!("csv finish: {}", error))
}

/// 🔮️ Appends one row to an existing CSV with the registered reference implementation.
#[cfg(feature = "oracles")]
pub fn oracle_append_csv_row(input: &[u8], row: &[String]) -> Result<Vec<u8>, String> {
    let mut spec = read_csv(input)?;
    spec.rows.push(row.to_vec());
    oracle_create_csv(&spec)
}

/// 👁️ Projects CSV bytes with the INDEPENDENT reader onto the owned `semantic-tabular-v1` shape.
#[cfg(feature = "oracles")]
pub fn project_csv(input: &[u8]) -> Result<Json, String> {
    Ok(read_csv(input)?.projection())
}

#[cfg(feature = "oracles")]
fn read_csv(input: &[u8]) -> Result<TableSpec, String> {
    let mut reader = csv::ReaderBuilder::new().has_headers(true).from_reader(input);
    let header = reader.headers().map_err(|error| format!("independent reader could not read the CSV header: {}", error))?.iter().map(|cell| cell.to_string()).collect();
    let mut rows = Vec::new();
    for record in reader.records() {
        rows.push(record.map_err(|error| format!("independent reader could not read a CSV record: {}", error))?.iter().map(|cell| cell.to_string()).collect());
    }
    Ok(TableSpec { header, rows })
}
//#endregion 🔖️Csv

//#region 🔖️Unavailable
/// 🚫️ Without the `oracles` feature nothing here is linked, and every entry point fails loudly.
#[cfg(not(feature = "oracles"))]
mod unavailable {
    use super::{Json, TableSpec};
    const MESSAGE: &str = "the `oracles` feature is disabled — this host was not built with the registered reference implementations";

    pub fn create_csv(_spec: &TableSpec) -> Result<Vec<u8>, String> {
        Err(MESSAGE.to_string())
    }
    pub fn append_csv_row(_input: &[u8], _row: &[String]) -> Result<Vec<u8>, String> {
        Err(MESSAGE.to_string())
    }
    pub fn project_csv(_input: &[u8]) -> Result<Json, String> {
        Err(MESSAGE.to_string())
    }
}

#[cfg(not(feature = "oracles"))]
pub use unavailable::{append_csv_row as oracle_append_csv_row, create_csv as oracle_create_csv, project_csv};
//#endregion 🔖️Unavailable
