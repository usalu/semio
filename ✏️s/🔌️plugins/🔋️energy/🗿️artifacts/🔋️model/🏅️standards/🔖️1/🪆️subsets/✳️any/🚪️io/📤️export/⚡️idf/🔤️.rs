//! 🔤️ EnergyPlus IDF lexical records, independent of physical-model semantics.

/// 🧾️ One IDF object with positional fields, including explicit empty fields.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IdfRecord {
    pub kind: String,
    pub fields: Vec<String>,
}

/// 🔤️ Reads EnergyPlus delimiters and line comments without discarding empty positions.
pub fn parse_records(text: &str) -> Result<Vec<IdfRecord>, String> {
    let mut records = Vec::new();
    let mut fields = Vec::new();
    let mut field = String::new();
    let mut comment = false;
    for (offset, ch) in text.trim_start_matches('\u{feff}').char_indices() {
        if comment {
            if ch == '\n' || ch == '\r' { comment = false; }
            continue;
        }
        match ch {
            '!' => comment = true,
            ',' | ';' => {
                fields.push(field.trim().to_string());
                field.clear();
                if ch == ';' {
                    let kind = fields.remove(0);
                    if kind.is_empty() { return Err(format!("IDF: empty object type at byte {offset}")); }
                    records.push(IdfRecord { kind, fields: std::mem::take(&mut fields) });
                }
            }
            '\0' => return Err(format!("IDF: NUL at byte {offset}")),
            _ => field.push(ch),
        }
    }
    if !field.trim().is_empty() || !fields.is_empty() { return Err("IDF: object is missing its terminating semicolon".into()); }
    Ok(records)
}

/// 🖨️ Writes canonical records and refuses values which IDF cannot represent losslessly.
pub fn write_records(records: &[IdfRecord]) -> Result<String, String> {
    let mut output = String::new();
    for record in records {
        for field in std::iter::once(&record.kind).chain(record.fields.iter()) {
            if field.chars().any(|ch| matches!(ch, ',' | ';' | '!' | '\n' | '\r' | '\0')) || field.trim() != field {
                return Err(format!("IDF: unrepresentable field {field:?} in {}", record.kind));
            }
        }
        if record.kind.is_empty() { return Err("IDF: empty object type".into()); }
        output.push_str(&record.kind);
        for field in &record.fields {
            output.push_str(",\n  ");
            output.push_str(field);
        }
        output.push_str(";\n\n");
    }
    Ok(output)
}

