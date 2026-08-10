//! 🧬️ CsvSnapshot schema — persistent fields + real codecs.

use crate::artifacts::csv::STDIO_CSV_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted `stdio.csv` snapshot (RFC4180-ish table).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.csv")]
pub struct CsvSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub headers: Vec<String>,
    #[state(persistent)]
    #[serde(default)]
    pub rows: Vec<Vec<String>>,
}

impl Default for CsvSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_CSV_DOCUMENT_SCHEMA.into(),
            headers: Vec::new(),
            rows: Vec::new(),
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️CsvTextCodec
fn csv_escape_row(cells: &[String]) -> String {
    cells
        .iter()
        .map(|c| {
            if c.contains(',') || c.contains('"') || c.contains('\n') || c.contains('\r') {
                format!("\"{}\"", c.replace('"', "\"\""))
            } else {
                c.clone()
            }
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn csv_parse_row(line: &str) -> Vec<String> {
    let mut cells = Vec::new();
    let mut cur = String::new();
    let mut in_q = false;
    let mut chars = line.chars().peekable();
    while let Some(ch) = chars.next() {
        if in_q {
            if ch == '"' {
                if chars.peek() == Some(&'"') {
                    chars.next();
                    cur.push('"');
                } else {
                    in_q = false;
                }
            } else {
                cur.push(ch);
            }
        } else if ch == '"' {
            in_q = true;
        } else if ch == ',' {
            cells.push(cur.clone());
            cur.clear();
        } else {
            cur.push(ch);
        }
    }
    cells.push(cur);
    cells
}

pub fn csv_table_to_text(headers: &[String], rows: &[Vec<String>]) -> String {
    let mut out = csv_escape_row(headers);
    out.push('\n');
    for row in rows {
        out.push_str(&csv_escape_row(row));
        out.push('\n');
    }
    out
}

pub fn csv_table_from_text(text: &str) -> (Vec<String>, Vec<Vec<String>>) {
    let mut lines = text.lines();
    let headers = csv_parse_row(lines.next().unwrap_or(""));
    let rows = lines.filter(|l| !l.is_empty()).map(|l| csv_parse_row(l)).collect();
    (headers, rows)
}
//#endregion 🔖️CsvTextCodec

//#region 🔖️HandcraftedDocumentCodecs
impl store::DocumentDsl for CsvSnapshot {
    const EXTENSION: &'static str = "csv";
    fn envelope_id() -> &'static str { "stdio.csv" }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let (headers, rows) = csv_table_from_text(body);
        Ok(Self { schema: STDIO_CSV_DOCUMENT_SCHEMA.into(), headers, rows })
    }
    fn print_dsl(&self) -> String {
        let body = csv_table_to_text(&self.headers, &self.rows);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::DocumentPack for CsvSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = csv_table_to_text(&self.headers, &self.rows).into_bytes();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::DocumentDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::DocumentDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let _ = options;
        let text = String::from_utf8(inner).map_err(|e| store::PackError::Schema(e.to_string()))?;
        let (headers, rows) = csv_table_from_text(&text);
        Ok(Self { schema: STDIO_CSV_DOCUMENT_SCHEMA.into(), headers, rows })
    }
}
//#endregion 🔖️HandcraftedDocumentCodecs
