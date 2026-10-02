//! 🏛️ Dependency-free relational SQLite interchange; https://sqlite.org/fileformat.html.

use std::collections::BTreeSet;
use std::fmt;

#[path = "🧩️artifact/🦀️.rs"]
pub mod artifact;

pub const SQLITE_SNAPSHOT_APPLICATION_ID: u32 = 0x534d534e;
pub const SQLITE_SNAPSHOT_USER_VERSION: u32 = 1;
const PAGE_SIZE: usize = 4096;

/// 📜️ Native representation choices used by artifact providers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SnapshotEncoding { Binary, Text }

impl SnapshotEncoding {
    pub fn as_str(self) -> &'static str { match self { Self::Binary => "binary", Self::Text => "text" } }
    pub fn parse(value: &str) -> Result<Self> { match value { "binary" => Ok(Self::Binary), "text" => Ok(Self::Text), _ => Err(SqliteSnapshotError::InvalidEncoding) } }
}

/// 🔢️ SQLite storage classes with owned, native scalar values.
#[derive(Clone, Debug, PartialEq)]
pub enum SqliteValue { Null, Integer(i64), Real(f64), Text(String), Blob(Vec<u8>) }

/// 📑️ One row with its signed SQLite rowid and semantic column values.
#[derive(Clone, Debug, PartialEq)]
pub struct SqliteRow { pub rowid: i64, pub values: Vec<SqliteValue> }

impl SqliteRow {
    pub fn integer(&self, index: usize) -> std::result::Result<i64, String> { match self.values.get(index) { Some(SqliteValue::Integer(value)) => Ok(*value), _ => Err(format!("column {index} requires INTEGER")) } }
    pub fn real(&self, index: usize) -> std::result::Result<f64, String> { match self.values.get(index) { Some(SqliteValue::Real(value)) => Ok(*value), Some(SqliteValue::Integer(value)) => Ok(*value as f64), _ => Err(format!("column {index} requires REAL")) } }
    pub fn text(&self, index: usize) -> std::result::Result<&str, String> { match self.values.get(index) { Some(SqliteValue::Text(value)) => Ok(value), _ => Err(format!("column {index} requires TEXT")) } }
    pub fn blob(&self, index: usize) -> std::result::Result<&[u8], String> { match self.values.get(index) { Some(SqliteValue::Blob(value)) => Ok(value), _ => Err(format!("column {index} requires BLOB")) } }
    pub fn optional_text(&self, index: usize) -> std::result::Result<Option<&str>, String> { match self.values.get(index) { Some(SqliteValue::Text(value)) => Ok(Some(value)), Some(SqliteValue::Null) => Ok(None), _ => Err(format!("column {index} requires TEXT or NULL")) } }
}

/// 🧱️ Handwritten domain table definition and its typed relational rows.
#[derive(Clone, Debug, PartialEq)]
pub struct SqliteTable { pub name: String, pub sql: String, pub rows: Vec<SqliteRow> }

impl SqliteTable {
    pub fn single_row(&self) -> std::result::Result<&SqliteRow, String> { if self.rows.len() == 1 { Ok(&self.rows[0]) } else { Err(format!("table {} requires exactly one row", self.name)) } }
    pub fn ordered_rows(&self, ordinal_column: usize) -> std::result::Result<Vec<&SqliteRow>, String> {
        let mut rows = self.rows.iter().map(|row| Ok((row.integer(ordinal_column)?, row))).collect::<std::result::Result<Vec<_>, String>>()?;
        rows.sort_by_key(|(ordinal, _)| *ordinal);
        for (expected, (ordinal, _)) in rows.iter().enumerate() { if *ordinal != expected as i64 { return Err(format!("table {} requires contiguous zero-based ordinals", self.name)); } }
        Ok(rows.into_iter().map(|(_, row)| row).collect())
    }
}

/// 🏗️ A relational database projected explicitly by one artifact provider.
#[derive(Clone, Debug, PartialEq)]
pub struct SqliteDatabase { pub tables: Vec<SqliteTable> }

/// 🏛️ Validates one declared table without cloning its rows or requiring a single-table database.
pub fn validate_sqlite_table_schema(table: &SqliteTable, sql: &str, limits: SqliteDatabaseLimits) -> Result<()> {
    limit(sql.len(), limits.max_schema_bytes, "schema bytes")?;
    limit(table.sql.len().checked_add(table.name.len()).ok_or(SqliteSnapshotError::Limit("schema bytes"))?, limits.max_schema_bytes, "schema bytes")?;
    let actual = parse_table(&table.sql)?; let expected = parse_table(sql)?; limit(actual.columns.len(), limits.max_columns, "columns")?;
    if !table.name.eq_ignore_ascii_case(&actual.name) || !table.name.eq_ignore_ascii_case(&expected.name) || !schema_matches(&table.sql, sql)? { return Err(SqliteSnapshotError::Malformed("artifact table schema")); }
    Ok(())
}

/// 🏛️ Validates an artifact's declared relational contract without normalizing quoted syntax into keywords.
pub fn validate_sqlite_database_schema(database: &SqliteDatabase, sql: &str, limits: SqliteDatabaseLimits) -> Result<()> {
    limit(sql.len(), limits.max_schema_bytes, "schema bytes")?; limit(database.tables.len(), limits.max_tables, "tables")?;
    let mut bytes = 0usize; for table in &database.tables { bytes = bytes.checked_add(table.sql.len()).and_then(|sum| sum.checked_add(table.name.len())).ok_or(SqliteSnapshotError::Limit("schema bytes"))?; limit(bytes, limits.max_schema_bytes, "schema bytes")?; }
    let expected = SqliteDatabase::from_schema(sql)?;
    if database.tables.len() != expected.tables.len() { return Err(SqliteSnapshotError::Malformed("artifact table count")); }
    let mut names = BTreeSet::new();
    for table in &database.tables {
        if !names.insert(table.name.to_ascii_lowercase()) { return Err(SqliteSnapshotError::Malformed("artifact duplicate table name")); }
        let declared = expected.tables.iter().find(|declared| declared.name.eq_ignore_ascii_case(&table.name)).ok_or(SqliteSnapshotError::Malformed("artifact table name"))?;
        validate_sqlite_table_schema(table, &declared.sql, limits)?;
    }
    Ok(())
}

impl SqliteDatabase {
    pub fn from_schema(sql: &str) -> Result<Self> {
        let tokens = lex(sql)?;
        let mut start = 0;
        let mut tables = Vec::new();
        for end in 0..=tokens.len() {
            if end != tokens.len() && (tokens[end].quoted || tokens[end].literal || tokens[end].text != ";") { continue; }
            if start < end {
                let statement = sql[tokens[start].start..tokens[end - 1].end].trim();
                let definition = parse_table(statement)?;
                if tables.iter().any(|table: &SqliteTable| table.name.eq_ignore_ascii_case(&definition.name)) { return Err(SqliteSnapshotError::Malformed("duplicate table name")); }
                tables.push(SqliteTable { name: definition.name, sql: statement.into(), rows: Vec::new() });
            }
            start = end + 1;
        }
        Ok(Self { tables })
    }

    pub fn table(&self, name: &str) -> std::result::Result<&SqliteTable, String> { self.tables.iter().find(|table| table.name.eq_ignore_ascii_case(name)).ok_or_else(|| format!("missing table {name}")) }
    pub fn table_mut(&mut self, name: &str) -> std::result::Result<&mut SqliteTable, String> { self.tables.iter_mut().find(|table| table.name.eq_ignore_ascii_case(name)).ok_or_else(|| format!("missing table {name}")) }
}

/// 📏️ Allocation and structure bounds applied before untrusted lengths allocate.
#[derive(Clone, Copy, Debug)]
pub struct SqliteDatabaseLimits {
    pub max_file_bytes: usize,
    pub max_value_bytes: usize,
    pub max_schema_bytes: usize,
    pub max_rows: usize,
    pub max_columns: usize,
    pub max_tables: usize,
    pub max_pages: usize,
}

impl Default for SqliteDatabaseLimits {
    fn default() -> Self { Self { max_file_bytes: 272 * 1024 * 1024, max_value_bytes: 256 * 1024 * 1024, max_schema_bytes: 4 * 1024 * 1024, max_rows: 1_000_000, max_columns: 1024, max_tables: 4096, max_pages: 1_000_000 } }
}

/// 🧭️ Transfer and artifact projection stages share one cancellation boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SqliteSnapshotPhase { ReadPages, WritePages, ProjectSnapshot, ReconstructSnapshot, DecodeNative, EncodeNative }

/// ⏱️ Stage-local progress; validation can repeat its boundary, and false cancels.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SqliteSnapshotProgress { pub phase: SqliteSnapshotPhase, pub completed: usize, pub total: usize }

/// 🛑️ One progress and cancellation authority spanning typed projection and physical I/O.
pub struct SqliteSnapshotControl<'a> { callback: &'a mut dyn FnMut(SqliteSnapshotProgress) -> bool, limits: SqliteDatabaseLimits, reconstruction_bytes: usize, reconstruction_units: usize, reconstruction_scalar_bytes: usize }

impl<'a> SqliteSnapshotControl<'a> {
    pub fn new(callback: &'a mut dyn FnMut(SqliteSnapshotProgress) -> bool, limits: SqliteDatabaseLimits) -> Self { Self { callback, limits, reconstruction_bytes: 0, reconstruction_units: 0, reconstruction_scalar_bytes: 0 } }
    pub fn limits(&self) -> SqliteDatabaseLimits { self.limits }
    pub fn check_rows(&self, count: usize) -> std::result::Result<(), String> { limit(count, self.limits.max_rows, "rows").map_err(|error| error.to_string()) }
    pub fn check_value_bytes(&self, count: usize) -> std::result::Result<(), String> { limit(count, self.limits.max_value_bytes, "value bytes").map_err(|error| error.to_string()) }
    pub fn check_database(&mut self, database: &SqliteDatabase, phase: SqliteSnapshotPhase) -> std::result::Result<(), String> {
        self.checkpoint(phase, 0, 0)?; let total = database.tables.iter().try_fold(0usize, |count, table| count.checked_add(table.rows.len()).ok_or("SQLite row count overflow"))?; self.check_rows(total)?; let mut rows = 0usize; let mut bytes = 0usize; let mut scalar_bytes = 0usize;
        for table in &database.tables { for row in &table.rows { rows = rows.checked_add(1).ok_or("SQLite row count overflow")?; self.check_rows(rows)?; for value in &row.values { let size = value_size(value); bytes = bytes.checked_add(size).ok_or("SQLite value byte count overflow")?; if matches!(value, SqliteValue::Integer(_) | SqliteValue::Real(_)) { scalar_bytes = scalar_bytes.checked_add(size).ok_or("SQLite scalar byte count overflow")?; } self.check_value_bytes(bytes)?; } if rows % 256 == 0 { self.checkpoint(phase, rows, total)?; } } }
        if phase == SqliteSnapshotPhase::ReconstructSnapshot { self.reconstruction_scalar_bytes = self.reconstruction_scalar_bytes.max(scalar_bytes); }
        self.checkpoint(phase, rows, total)
    }
    pub fn checkpoint(&mut self, phase: SqliteSnapshotPhase, completed: usize, total: usize) -> std::result::Result<(), String> { if (self.callback)(SqliteSnapshotProgress { phase, completed, total }) { Ok(()) } else { Err(SqliteSnapshotError::Cancelled.to_string()) } }
}

/// 🚫️ Structural, representation, resource or cancellation failure.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SqliteSnapshotError { Malformed(&'static str), Limit(&'static str), InvalidEncoding, Cancelled }

impl fmt::Display for SqliteSnapshotError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self { Self::Malformed(reason) => write!(formatter, "invalid relational SQLite: {reason}"), Self::Limit(reason) => write!(formatter, "relational SQLite resource limit: {reason}"), Self::InvalidEncoding => formatter.write_str("encoding requires binary or text"), Self::Cancelled => formatter.write_str("relational SQLite transfer cancelled") }
    }
}

impl std::error::Error for SqliteSnapshotError {}
type Result<T> = std::result::Result<T, SqliteSnapshotError>;

fn limit(value: usize, maximum: usize, name: &'static str) -> Result<()> { if value > maximum { Err(SqliteSnapshotError::Limit(name)) } else { Ok(()) } }
fn checkpoint(callback: &mut dyn FnMut(SqliteSnapshotProgress) -> bool, phase: SqliteSnapshotPhase, completed: usize, total: usize) -> Result<()> { if callback(SqliteSnapshotProgress { phase, completed, total }) { Ok(()) } else { Err(SqliteSnapshotError::Cancelled) } }
fn put_u16(bytes: &mut [u8], at: usize, value: usize) { bytes[at..at + 2].copy_from_slice(&(value as u16).to_be_bytes()); }
fn put_u32(bytes: &mut [u8], at: usize, value: u32) { bytes[at..at + 4].copy_from_slice(&value.to_be_bytes()); }
fn u16_at(bytes: &[u8], at: usize) -> Result<usize> { let value = bytes.get(at..at + 2).ok_or(SqliteSnapshotError::Malformed("truncated integer"))?; Ok(u16::from_be_bytes([value[0], value[1]]) as usize) }
fn u32_at(bytes: &[u8], at: usize) -> Result<u32> { let value = bytes.get(at..at + 4).ok_or(SqliteSnapshotError::Malformed("truncated integer"))?; Ok(u32::from_be_bytes([value[0], value[1], value[2], value[3]])) }

fn varint(mut value: u64) -> Vec<u8> {
    if value > 0x00ff_ffff_ffff_ffff { let mut bytes = vec![0; 9]; bytes[8] = value as u8; value >>= 8; for index in (0..8).rev() { bytes[index] = (value as u8 & 0x7f) | 0x80; value >>= 7; } return bytes; }
    let mut bytes = vec![value as u8 & 0x7f]; value >>= 7; while value != 0 { bytes.push((value as u8 & 0x7f) | 0x80); value >>= 7; } bytes.reverse(); bytes
}

fn read_varint(bytes: &[u8], at: &mut usize) -> Result<u64> {
    let mut value = 0; for index in 0..9 { let byte = *bytes.get(*at).ok_or(SqliteSnapshotError::Malformed("truncated varint"))?; *at += 1; if index == 8 { return Ok((value << 8) | byte as u64); } value = (value << 7) | (byte & 0x7f) as u64; if byte & 0x80 == 0 { return Ok(value); } } Err(SqliteSnapshotError::Malformed("invalid varint"))
}

fn local_payload(length: usize, usable: usize) -> usize { if length <= usable - 35 { return length; } let minimum = (usable - 12) * 32 / 255 - 23; let candidate = minimum + (length - minimum) % (usable - 4); if candidate <= usable - 35 { candidate } else { minimum } }

#[derive(Clone)]
struct Token { text: String, quoted: bool, literal: bool, start: usize, end: usize }

impl Token { fn keyword(&self, value: &str) -> bool { !self.quoted && !self.literal && self.text.eq_ignore_ascii_case(value) } }

fn lex(sql: &str) -> Result<Vec<Token>> {
    let bytes = sql.as_bytes(); let mut at = 0; let mut tokens = Vec::new();
    while at < bytes.len() {
        if bytes[at].is_ascii_whitespace() { at += 1; continue; }
        if bytes[at..].starts_with(b"--") { at += 2; while at < bytes.len() && bytes[at] != b'\n' { at += 1; } continue; }
        if bytes[at..].starts_with(b"/*") { at += 2; while at + 1 < bytes.len() && !bytes[at..].starts_with(b"*/") { at += 1; } if at + 1 >= bytes.len() { return Err(SqliteSnapshotError::Malformed("unterminated SQL comment")); } at += 2; continue; }
        let start = at; let mut text = String::new(); let quoted = matches!(bytes[at], b'"' | b'`' | b'['); let literal = bytes[at] == b'\'';
        if quoted || literal {
            let close = if bytes[at] == b'[' { b']' } else { bytes[at] }; at += 1; let mut segment = at;
            loop { if at >= bytes.len() { return Err(SqliteSnapshotError::Malformed("unterminated SQL quote")); } if bytes[at] == close { text.push_str(&sql[segment..at]); at += 1; if close != b']' && bytes.get(at) == Some(&close) { text.push(close as char); at += 1; segment = at; } else { break; } } else { at += 1; } }
        } else if bytes[at].is_ascii_alphanumeric() || bytes[at] == b'_' || bytes[at] >= 128 {
            at += 1; while at < bytes.len() && (bytes[at].is_ascii_alphanumeric() || bytes[at] == b'_' || bytes[at] >= 128) { at += 1; } text.push_str(&sql[start..at]);
        } else { text.push(bytes[at] as char); at += 1; }
        tokens.push(Token { text, quoted, literal, start, end: at });
    }
    Ok(tokens)
}

fn schema_matches(actual: &str, expected: &str) -> Result<bool> {
    let trim = |mut tokens: Vec<Token>| { while tokens.last().is_some_and(|token| token.keyword(";")) { tokens.pop(); } tokens };
    let actual_tokens = trim(lex(actual)?); let tokens = trim(lex(expected)?); if actual_tokens.len() != tokens.len() { return Ok(false); }
    let definition = parse_table(expected)?; let mut identifiers = BTreeSet::from([2usize]); let mut depth = 0usize; let mut start = false;
    let syntax = ["NOT", "NULL", "IS", "IN", "BETWEEN", "AND", "OR", "LIKE", "GLOB", "MATCH", "REGEXP", "ESCAPE", "CASE", "WHEN", "THEN", "ELSE", "END", "COLLATE", "AS"];
    for (position, token) in tokens.iter().enumerate().skip(3) {
        if !token.quoted && !token.literal && token.text == "(" { depth += 1; if depth == 1 { start = true; } continue; }
        if !token.quoted && !token.literal && token.text == ")" { depth = depth.saturating_sub(1); continue; }
        if depth == 1 && !token.quoted && !token.literal && token.text == "," { start = true; continue; }
        if start { if token.keyword("CONSTRAINT") { identifiers.insert(position + 1); } else if !["PRIMARY", "FOREIGN", "CHECK"].iter().any(|keyword| token.keyword(keyword)) { identifiers.insert(position); } start = false; }
        if token.keyword("REFERENCES") {
            identifiers.insert(position + 1);
            if tokens.get(position + 2).is_some_and(|token| token.keyword("(")) { let mut at = position + 3; while let Some(token) = tokens.get(at) { if token.keyword(")") { break; } if !token.keyword(",") { identifiers.insert(at); } at += 1; } }
        }
        if token.keyword("COLLATE") { identifiers.insert(position + 1); }
        if !token.literal && (token.quoted || !syntax.iter().any(|keyword| token.keyword(keyword))) && definition.columns.iter().any(|column| column.name.eq_ignore_ascii_case(&token.text)) { identifiers.insert(position); }
    }
    Ok(actual_tokens.iter().zip(&tokens).enumerate().all(|(position, (actual_token, expected_token))| {
        if actual_token.literal || expected_token.literal { actual_token.literal == expected_token.literal && actual[actual_token.start..actual_token.end] == expected[expected_token.start..expected_token.end] }
        else { actual_token.text.eq_ignore_ascii_case(&expected_token.text) && (identifiers.contains(&position) || actual_token.quoted == expected_token.quoted) }
    }))
}

struct Column { name: String, integer_type: bool, real_affinity: bool, not_null: bool }
struct Definition { name: String, columns: Vec<Column>, primary_key: Option<usize> }

fn parse_table(sql: &str) -> Result<Definition> {
    let tokens = lex(sql)?;
    if tokens.len() < 5 || !tokens[0].keyword("CREATE") || !tokens[1].keyword("TABLE") || tokens[2].literal || tokens[2].text.is_empty() || tokens[3].text != "(" { return Err(SqliteSnapshotError::Malformed("expected CREATE TABLE")); }
    if tokens.iter().any(|token| ["UNIQUE", "AUTOINCREMENT", "WITHOUT", "DESC", "GENERATED"].iter().any(|keyword| token.keyword(keyword))) { return Err(SqliteSnapshotError::Malformed("unsupported implicit index or table feature")); }
    let name = tokens[2].text.clone();
    if name.to_ascii_lowercase().starts_with("sqlite_") { return Err(SqliteSnapshotError::Malformed("reserved table name")); }
    let mut groups = Vec::new(); let mut depth = 1; let mut start = 4; let mut close = None;
    for (at, token) in tokens.iter().enumerate().skip(4) {
        if token.quoted || token.literal { continue; }
        match token.text.as_str() { "(" => depth += 1, ")" => { depth -= 1; if depth == 0 { if start < at { groups.push(&tokens[start..at]); } close = Some(at); break; } }, "," if depth == 1 => { if start == at { return Err(SqliteSnapshotError::Malformed("empty column definition")); } groups.push(&tokens[start..at]); start = at + 1; }, _ => {} }
    }
    let close = close.ok_or(SqliteSnapshotError::Malformed("unbalanced table definition"))?;
    if tokens[close + 1..].iter().any(|token| token.text != ";") || groups.is_empty() { return Err(SqliteSnapshotError::Malformed("unsupported table suffix or empty table")); }
    let mut columns = Vec::new(); let mut primary_name = None;
    for mut group in groups {
        if group[0].keyword("CONSTRAINT") { if group.len() < 3 { return Err(SqliteSnapshotError::Malformed("incomplete named constraint")); } group = &group[2..]; }
        if group[0].keyword("PRIMARY") {
            if group.len() < 5 || !group[1].keyword("KEY") || group[2].text != "(" || group.last().is_none_or(|token| token.text != ")") || (group.len() != 5 && !(group.len() == 6 && group[4].keyword("ASC"))) || primary_name.is_some() { return Err(SqliteSnapshotError::Malformed("unsupported table primary key")); }
            primary_name = Some(group[3].text.clone()); continue;
        }
        if group[0].keyword("FOREIGN") || group[0].keyword("CHECK") { continue; }
        if group[0].literal || group[0].text.is_empty() || columns.iter().any(|column: &Column| column.name.eq_ignore_ascii_case(&group[0].text)) { return Err(SqliteSnapshotError::Malformed("invalid or duplicate column")); }
        let constraints = ["PRIMARY", "NOT", "NULL", "DEFAULT", "CHECK", "REFERENCES", "COLLATE", "CONSTRAINT"];
        let type_end = (1..group.len()).find(|index| constraints.iter().any(|keyword| group[*index].keyword(keyword))).unwrap_or(group.len());
        let declared = group[1..type_end].iter().map(|token| token.text.as_str()).collect::<Vec<_>>().join(" ").to_ascii_uppercase();
        let integer_type = declared == "INTEGER";
        let real_affinity = !declared.contains("INT") && !declared.contains("CHAR") && !declared.contains("CLOB") && !declared.contains("TEXT") && !declared.contains("BLOB") && ["REAL", "FLOA", "DOUB"].iter().any(|name| declared.contains(name));
        let mut depth = 0; let mut not_null = false;
        for (index, token) in group.iter().enumerate() {
            if token.quoted || token.literal { continue; }
            if token.text == "(" { depth += 1; } else if token.text == ")" { depth -= 1; } else if depth == 0 && token.keyword("NOT") && group.get(index + 1).is_some_and(|token| token.keyword("NULL")) { not_null = true; }
        }
        if group.windows(2).any(|pair| pair[0].keyword("PRIMARY") && pair[1].keyword("KEY")) { if primary_name.is_some() { return Err(SqliteSnapshotError::Malformed("multiple primary keys")); } primary_name = Some(group[0].text.clone()); }
        columns.push(Column { name: group[0].text.clone(), integer_type, real_affinity, not_null });
    }
    if columns.is_empty() { return Err(SqliteSnapshotError::Malformed("table has no columns")); }
    let primary_key = if let Some(primary) = primary_name { let index = columns.iter().position(|column| column.name.eq_ignore_ascii_case(&primary)).ok_or(SqliteSnapshotError::Malformed("unknown primary key column"))?; if !columns[index].integer_type { return Err(SqliteSnapshotError::Malformed("non-INTEGER primary key creates implicit index")); } Some(index) } else { None };
    Ok(Definition { name, columns, primary_key })
}

fn serial(value: &SqliteValue) -> Result<(u64, Vec<u8>)> {
    Ok(match value {
        SqliteValue::Null => (0, Vec::new()),
        SqliteValue::Integer(value) if *value == 0 || *value == 1 => (8 + *value as u64, Vec::new()),
        SqliteValue::Integer(value) => { let (serial, length) = if (-128..=127).contains(value) { (1, 1) } else if (-32768..=32767).contains(value) { (2, 2) } else if (-8_388_608..=8_388_607).contains(value) { (3, 3) } else if (i32::MIN as i64..=i32::MAX as i64).contains(value) { (4, 4) } else if (-140_737_488_355_328..=140_737_488_355_327).contains(value) { (5, 6) } else { (6, 8) }; (serial, value.to_be_bytes()[8 - length..].to_vec()) },
        SqliteValue::Real(value) => { if value.is_nan() { return Err(SqliteSnapshotError::Malformed("NaN is not a SQLite REAL")); } (7, value.to_be_bytes().to_vec()) },
        SqliteValue::Text(value) => (13 + 2 * value.len() as u64, value.as_bytes().to_vec()),
        SqliteValue::Blob(value) => (12 + 2 * value.len() as u64, value.clone()),
    })
}

fn value_size(value: &SqliteValue) -> usize { match value { SqliteValue::Null => 0, SqliteValue::Integer(_) | SqliteValue::Real(_) => 8, SqliteValue::Text(value) => value.len(), SqliteValue::Blob(value) => value.len() } }

fn encode_record(values: &[SqliteValue], limits: SqliteDatabaseLimits) -> Result<Vec<u8>> {
    limit(values.len(), limits.max_columns.max(5), "columns")?;
    let mut total = 0usize;
    for value in values { let size = value_size(value); limit(size, limits.max_value_bytes, "value bytes")?; total = total.checked_add(size).ok_or(SqliteSnapshotError::Limit("record bytes"))?; limit(total, limits.max_file_bytes, "record bytes")?; }
    let fields = values.iter().map(serial).collect::<Result<Vec<_>>>()?;
    let serials: Vec<u8> = fields.iter().flat_map(|(serial, _)| varint(*serial)).collect(); let mut header = serials.len() + 1;
    while varint(header as u64).len() + serials.len() != header { header = varint(header as u64).len() + serials.len(); }
    let mut bytes = varint(header as u64); bytes.extend(serials); for (_, field) in fields { bytes.extend(field); } limit(bytes.len(), limits.max_file_bytes, "record bytes")?; Ok(bytes)
}

fn read_record(bytes: &[u8], maximum_columns: usize, maximum_value: usize) -> Result<Vec<SqliteValue>> {
    let mut cursor = 0; let header = usize::try_from(read_varint(bytes, &mut cursor)?).map_err(|_| SqliteSnapshotError::Malformed("header length"))?;
    if header < cursor || header > bytes.len() { return Err(SqliteSnapshotError::Malformed("record header boundary")); }
    let mut body = header; let mut fields = Vec::new(); let mut semantic_bytes = 0usize;
    while cursor < header {
        if fields.len() >= maximum_columns { return Err(SqliteSnapshotError::Limit("columns")); }
        let serial = read_varint(&bytes[..header], &mut cursor)?;
        let length = match serial { 0 | 8 | 9 => 0, 1..=4 => serial, 5 => 6, 6 | 7 => 8, 10 | 11 => return Err(SqliteSnapshotError::Malformed("reserved serial type")), _ => (serial - 12) / 2 };
        let length = usize::try_from(length).map_err(|_| SqliteSnapshotError::Limit("value bytes"))?; limit(length, maximum_value, "value bytes")?;
        semantic_bytes = semantic_bytes.checked_add(if (1..=9).contains(&serial) { 8 } else { length }).ok_or(SqliteSnapshotError::Limit("value bytes"))?; limit(semantic_bytes, maximum_value, "value bytes")?;
        let end = body.checked_add(length).filter(|end| *end <= bytes.len()).ok_or(SqliteSnapshotError::Malformed("record body boundary"))?; let value = &bytes[body..end];
        let field = match serial {
            0 => SqliteValue::Null,
            1..=6 => { let mut number = if value[0] & 0x80 != 0 { -1_i64 } else { 0 }; for byte in value { number = (number << 8) | *byte as i64; } SqliteValue::Integer(number) },
            7 => { let mut number = [0; 8]; number.copy_from_slice(value); let number = f64::from_be_bytes(number); if number.is_nan() { return Err(SqliteSnapshotError::Malformed("NaN REAL storage")); } SqliteValue::Real(number) },
            8 | 9 => SqliteValue::Integer((serial - 8) as i64),
            _ if serial & 1 == 0 => SqliteValue::Blob(value.to_vec()),
            _ => SqliteValue::Text(std::str::from_utf8(value).map_err(|_| SqliteSnapshotError::Malformed("invalid UTF-8"))?.into()),
        };
        fields.push(field); body = end;
    }
    if body != bytes.len() { return Err(SqliteSnapshotError::Malformed("trailing record bytes")); } Ok(fields)
}

struct Writer<'a> { pages: Vec<Vec<u8>>, limits: SqliteDatabaseLimits, callback: &'a mut dyn FnMut(SqliteSnapshotProgress) -> bool, completed: usize }

impl Writer<'_> {
    fn allocate(&mut self) -> Result<u32> {
        let pages = self.pages.len() + 1; limit(pages, self.limits.max_pages.min(u32::MAX as usize - 1), "pages")?; limit(pages.checked_mul(PAGE_SIZE).ok_or(SqliteSnapshotError::Limit("file bytes"))?, self.limits.max_file_bytes, "file bytes")?;
        self.pages.push(vec![0; PAGE_SIZE]); Ok(pages as u32)
    }

    fn done(&mut self) -> Result<()> { self.completed += 1; checkpoint(self.callback, SqliteSnapshotPhase::WritePages, self.completed, self.pages.len()) }

    fn cell(&mut self, rowid: i64, payload: &[u8]) -> Result<Vec<u8>> {
        let local = local_payload(payload.len(), PAGE_SIZE); let mut cell = varint(payload.len() as u64); cell.extend(varint(rowid as u64)); cell.extend_from_slice(&payload[..local]);
        let mut previous: Option<usize> = None;
        for bytes in payload[local..].chunks(PAGE_SIZE - 4) {
            let number = self.allocate()?; if let Some(previous) = previous { put_u32(&mut self.pages[previous], 0, number); } else { cell.extend_from_slice(&number.to_be_bytes()); }
            let page = &mut self.pages[number as usize - 1]; page[4..4 + bytes.len()].copy_from_slice(bytes); previous = Some(number as usize - 1); self.done()?;
        }
        Ok(cell)
    }

    fn leaf_page(&mut self, number: u32, cells: &[(i64, Vec<u8>)]) -> Result<()> {
        let page = &mut self.pages[number as usize - 1]; let header = if number == 1 { 100 } else { 0 }; let mut end = PAGE_SIZE; page[header] = 13; put_u16(page, header + 3, cells.len());
        for (index, (_, cell)) in cells.iter().enumerate() { end -= cell.len(); put_u16(page, header + 8 + index * 2, end); page[end..end + cell.len()].copy_from_slice(cell); }
        put_u16(page, header + 5, end); self.done()
    }

    fn interior_page(&mut self, number: u32, children: &[(u32, i64)]) -> Result<()> {
        if children.is_empty() || children.len() < 2 && number != 1 { return Err(SqliteSnapshotError::Malformed("interior page requires children")); }
        let page = &mut self.pages[number as usize - 1]; let header = if number == 1 { 100 } else { 0 }; let mut end = PAGE_SIZE; page[header] = 5; put_u16(page, header + 3, children.len() - 1); put_u32(page, header + 8, children.last().unwrap().0);
        for (index, (child, key)) in children[..children.len() - 1].iter().enumerate() { let key = varint(*key as u64); end -= 4 + key.len(); put_u16(page, header + 12 + index * 2, end); put_u32(page, end, *child); page[end + 4..end + 4 + key.len()].copy_from_slice(&key); }
        put_u16(page, header + 5, end); self.done()
    }

    fn interior_capacity(children: &[(u32, i64)], header: usize) -> bool { header + 12 + children.len().saturating_sub(1) * 2 + children[..children.len().saturating_sub(1)].iter().map(|(_, key)| 4 + varint(*key as u64).len()).sum::<usize>() <= PAGE_SIZE }

    fn tree(&mut self, root: u32, rows: &[(i64, Vec<u8>)]) -> Result<()> {
        let mut groups = Vec::new(); let mut cells = Vec::new(); let mut used = 8;
        for (rowid, payload) in rows {
            let cell = self.cell(*rowid, payload)?;
            if !cells.is_empty() && used + 2 + cell.len() > PAGE_SIZE { groups.push(cells); cells = Vec::new(); used = 8; }
            used += 2 + cell.len(); if used > PAGE_SIZE { return Err(SqliteSnapshotError::Malformed("leaf cell does not fit")); } cells.push((*rowid, cell));
        }
        if !cells.is_empty() || groups.is_empty() { groups.push(cells); }
        let root_header = if root == 1 { 100 } else { 0 };
        if groups.len() == 1 && root_header + 8 + groups[0].len() * 2 + groups[0].iter().map(|(_, cell)| cell.len()).sum::<usize>() <= PAGE_SIZE { return self.leaf_page(root, &groups[0]); }
        let mut level = Vec::new(); for group in groups { let page = self.allocate()?; let key = group.last().ok_or(SqliteSnapshotError::Malformed("empty non-root leaf"))?.0; self.leaf_page(page, &group)?; level.push((page, key)); }
        while !Self::interior_capacity(&level, root_header) {
            let mut parents = Vec::new(); let mut start = 0;
            while start < level.len() { let mut end = start + 2; while end < level.len() && Self::interior_capacity(&level[start..=end], 0) { end += 1; } if level.len() - end == 1 { end -= 1; } let group = &level[start..end]; let page = self.allocate()?; self.interior_page(page, group)?; parents.push((page, group.last().unwrap().1)); start = end; }
            level = parents;
        }
        self.interior_page(root, &level)
    }
}

/// 📤️ Exports handcrafted entity tables, relationships and scalar values as ordinary SQLite.
pub fn export_sqlite_database(database: &SqliteDatabase, limits: SqliteDatabaseLimits, callback: &mut dyn FnMut(SqliteSnapshotProgress) -> bool) -> Result<Vec<u8>> {
    checkpoint(callback, SqliteSnapshotPhase::WritePages, 0, 0)?; limit(database.tables.len(), limits.max_tables, "tables")?;
    let mut names = BTreeSet::new(); let mut definitions = Vec::new(); let mut schema_bytes = 0usize; let mut row_count = 0usize; let mut value_bytes = 0usize;
    for table in &database.tables {
        schema_bytes = schema_bytes.checked_add(table.sql.len()).and_then(|sum| sum.checked_add(table.name.len())).ok_or(SqliteSnapshotError::Limit("schema bytes"))?; limit(schema_bytes, limits.max_schema_bytes, "schema bytes")?;
        let definition = parse_table(&table.sql)?; if !definition.name.eq_ignore_ascii_case(&table.name) || !names.insert(table.name.to_ascii_lowercase()) { return Err(SqliteSnapshotError::Malformed("table name mismatch or duplicate")); }
        limit(definition.columns.len(), limits.max_columns, "columns")?; row_count = row_count.checked_add(table.rows.len()).ok_or(SqliteSnapshotError::Limit("rows"))?; limit(row_count, limits.max_rows, "rows")?; definitions.push(definition);
        for row in &table.rows { for value in &row.values { value_bytes = value_bytes.checked_add(value_size(value)).ok_or(SqliteSnapshotError::Limit("value bytes"))?; limit(value_bytes, limits.max_value_bytes, "value bytes")?; } checkpoint(callback, SqliteSnapshotPhase::WritePages, 0, database.tables.len() + 1)?; }
    }
    let mut writer = Writer { pages: Vec::new(), limits, callback, completed: 0 }; writer.allocate()?; for _ in &database.tables { writer.allocate()?; }
    let mut schema_rows = Vec::new();
    for (index, (table, definition)) in database.tables.iter().zip(&definitions).enumerate() {
        let root = index as u32 + 2; let values = vec![SqliteValue::Text("table".into()), SqliteValue::Text(table.name.clone()), SqliteValue::Text(table.name.clone()), SqliteValue::Integer(root as i64), SqliteValue::Text(table.sql.trim_end_matches(';').into())];
        let schema_limits = SqliteDatabaseLimits { max_value_bytes: limits.max_schema_bytes, ..limits }; schema_rows.push((index as i64 + 1, encode_record(&values, schema_limits)?));
        let mut rows: Vec<_> = table.rows.iter().collect(); rows.sort_by_key(|row| row.rowid); if rows.windows(2).any(|rows| rows[0].rowid == rows[1].rowid) { return Err(SqliteSnapshotError::Malformed("duplicate rowid")); }
        let mut encoded = Vec::new(); let mut encoded_bytes = 0usize;
        for row in rows {
            if row.values.len() != definition.columns.len() { return Err(SqliteSnapshotError::Malformed("row column count")); }
            let mut values = row.values.clone();
            for (column, value) in definition.columns.iter().zip(&values) { if column.not_null && matches!(value, SqliteValue::Null) { return Err(SqliteSnapshotError::Malformed("NOT NULL violation")); } }
            if let Some(primary) = definition.primary_key { if values[primary] != SqliteValue::Integer(row.rowid) { return Err(SqliteSnapshotError::Malformed("INTEGER PRIMARY KEY must equal rowid")); } values[primary] = SqliteValue::Null; }
            let payload = encode_record(&values, limits)?; encoded_bytes = encoded_bytes.checked_add(payload.len()).ok_or(SqliteSnapshotError::Limit("table bytes"))?; limit(encoded_bytes, limits.max_file_bytes, "table bytes")?; encoded.push((row.rowid, payload));
            checkpoint(writer.callback, SqliteSnapshotPhase::WritePages, writer.completed, writer.pages.len())?;
        }
        writer.tree(root, &encoded)?;
    }
    writer.tree(1, &schema_rows)?;
    let pages = writer.pages.len(); let header = &mut writer.pages[0]; header[..16].copy_from_slice(b"SQLite format 3\0"); put_u16(header, 16, PAGE_SIZE); header[18..24].copy_from_slice(&[1, 1, 0, 64, 32, 32]);
    for (offset, value) in [(24, 1), (28, pages as u32), (40, 1), (44, 4), (56, 1), (60, SQLITE_SNAPSHOT_USER_VERSION), (68, SQLITE_SNAPSHOT_APPLICATION_ID), (92, 1), (96, 3_046_000)] { put_u32(header, offset, value); }
    let mut bytes = Vec::with_capacity(pages * PAGE_SIZE); for page in writer.pages { bytes.extend(page); checkpoint(writer.callback, SqliteSnapshotPhase::WritePages, pages, pages)?; } Ok(bytes)
}

struct Reader<'a, 'b> { bytes: &'a [u8], page_size: usize, usable: usize, pages: usize, visited: BTreeSet<u32>, callback: &'b mut dyn FnMut(SqliteSnapshotProgress) -> bool }

impl<'a> Reader<'a, '_> {
    fn page(&mut self, number: u32) -> Result<&'a [u8]> { if number == 0 || number as usize > self.pages || !self.visited.insert(number) { return Err(SqliteSnapshotError::Malformed("invalid or repeated page reference")); } checkpoint(self.callback, SqliteSnapshotPhase::ReadPages, self.visited.len(), self.pages)?; let start = (number as usize - 1) * self.page_size; Ok(&self.bytes[start..start + self.usable]) }

    fn payload(&mut self, page: &[u8], cursor: &mut usize, length: usize) -> Result<Vec<u8>> {
        let local = local_payload(length, self.usable); let end = cursor.checked_add(local).filter(|end| *end <= page.len()).ok_or(SqliteSnapshotError::Malformed("truncated local payload"))?;
        let mut bytes = Vec::with_capacity(length); bytes.extend_from_slice(&page[*cursor..end]); *cursor = end; if local == length { return Ok(bytes); }
        let mut next = u32_at(page, *cursor)?; *cursor += 4;
        while bytes.len() < length { let overflow = self.page(next)?; next = u32_at(overflow, 0)?; let count = (length - bytes.len()).min(self.usable - 4); bytes.extend_from_slice(&overflow[4..4 + count]); }
        if next != 0 { return Err(SqliteSnapshotError::Malformed("overflow exceeds payload")); } Ok(bytes)
    }

    fn freelist(&mut self, mut next: u32, expected: usize) -> Result<()> {
        let mut count = 0usize;
        while next != 0 {
            let trunk = self.page(next)?; next = u32_at(trunk, 0)?; let leaves = u32_at(trunk, 4)? as usize;
            if leaves > self.usable / 4 - 2 { return Err(SqliteSnapshotError::Malformed("freelist trunk capacity")); }
            count = count.checked_add(leaves + 1).ok_or(SqliteSnapshotError::Malformed("freelist count"))?;
            if count > expected { return Err(SqliteSnapshotError::Malformed("freelist count")); }
            for index in 0..leaves { self.page(u32_at(trunk, 8 + index * 4)?)?; }
        }
        if count != expected { return Err(SqliteSnapshotError::Malformed("freelist count")); } Ok(())
    }

    fn table<F>(&mut self, root: u32, mut budget: usize, multiplier: usize, overhead: usize, max_rows: usize, consume: &mut F) -> Result<()> where F: FnMut(i64, &[u8], usize) -> Result<usize> {
        let mut stack = vec![(root, None::<i64>, None::<i64>)]; let mut rows = 0usize;
        while let Some((number, lower, upper)) = stack.pop() {
            let page = self.page(number)?; let header = if number == 1 { 100 } else { 0 }; let kind = page[header]; if kind != 5 && kind != 13 { return Err(SqliteSnapshotError::Malformed("expected table B-tree")); }
            let count = u16_at(page, header + 3)?; let pointers = header + if kind == 5 { 12 } else { 8 }; let pointer_end = pointers + count * 2; let raw_start = u16_at(page, header + 5)?; let content_start = if raw_start == 0 { 65536 } else { raw_start };
            if pointer_end > content_start || content_start > self.usable || page[header + 7] > 60 { return Err(SqliteSnapshotError::Malformed("B-tree content boundary")); }
            let mut ranges = Vec::new(); let mut children = Vec::new(); let mut prior = lower;
            for index in 0..count {
                let start = u16_at(page, pointers + index * 2)?; if start < content_start || start >= self.usable { return Err(SqliteSnapshotError::Malformed("cell pointer boundary")); } let mut cursor = start;
                if kind == 5 {
                    let child = u32_at(page, cursor)?; cursor += 4; let key = read_varint(page, &mut cursor)? as i64; if prior.is_some_and(|prior| key <= prior) || upper.is_some_and(|upper| key > upper) { return Err(SqliteSnapshotError::Malformed("unordered B-tree keys")); } children.push((child, prior, Some(key))); prior = Some(key);
                } else {
                    if rows >= max_rows { return Err(SqliteSnapshotError::Limit("rows")); } let length = usize::try_from(read_varint(page, &mut cursor)?).map_err(|_| SqliteSnapshotError::Limit("record bytes"))?; limit(length, budget.saturating_mul(multiplier).saturating_add(overhead).min(self.bytes.len()), "record bytes")?;
                    let key = read_varint(page, &mut cursor)? as i64; if prior.is_some_and(|prior| key <= prior) || upper.is_some_and(|upper| key > upper) { return Err(SqliteSnapshotError::Malformed("unordered B-tree keys")); } prior = Some(key); let payload = self.payload(page, &mut cursor, length)?; let cost = consume(key, &payload, budget)?; budget = budget.checked_sub(cost).ok_or(SqliteSnapshotError::Limit("value bytes"))?; rows += 1;
                }
                ranges.push((start, cursor));
            }
            let mut free = u16_at(page, header + 1)?;
            while free != 0 { if free < content_start { return Err(SqliteSnapshotError::Malformed("freeblock content boundary")); } let next = u16_at(page, free)?; let size = u16_at(page, free + 2)?; let end = free.checked_add(size).filter(|end| *end <= self.usable).ok_or(SqliteSnapshotError::Malformed("freeblock boundary"))?; if size < 4 || next != 0 && next < end { return Err(SqliteSnapshotError::Malformed("freeblock chain")); } ranges.push((free, end)); free = next; }
            ranges.sort_unstable(); if ranges.windows(2).any(|pair| pair[0].1 > pair[1].0) { return Err(SqliteSnapshotError::Malformed("overlapping cells")); }
            let mut end = content_start; let mut fragments = 0; for (start, next) in ranges { let gap = start - end; if gap > 3 { return Err(SqliteSnapshotError::Malformed("untracked free space")); } fragments += gap; end = next; }
            if self.usable - end > 3 || fragments + self.usable - end != page[header + 7] as usize { return Err(SqliteSnapshotError::Malformed("fragment accounting")); }
            if kind == 5 { children.push((u32_at(page, header + 8)?, prior, upper)); stack.extend(children.into_iter().rev()); }
        }
        Ok(())
    }
}

/// 📥️ Imports independent SQLite rowid tables with semantic primary keys and REAL affinity.
pub fn import_sqlite_database(bytes: &[u8], limits: SqliteDatabaseLimits, callback: &mut dyn FnMut(SqliteSnapshotProgress) -> bool) -> Result<SqliteDatabase> {
    limit(bytes.len(), limits.max_file_bytes, "file bytes")?;
    if bytes.len() < 100 || bytes.get(..16) != Some(b"SQLite format 3\0") { return Err(SqliteSnapshotError::Malformed("SQLite header")); }
    let raw = u16_at(bytes, 16)?; let page_size = if raw == 1 { 65536 } else { raw }; if !(512..=65536).contains(&page_size) || !page_size.is_power_of_two() || bytes.len() % page_size != 0 { return Err(SqliteSnapshotError::Malformed("page size or truncation")); }
    let pages = bytes.len() / page_size; limit(pages, limits.max_pages, "pages")?; let usable = page_size - bytes[20] as usize; let declared = u32_at(bytes, 28)? as usize; let authoritative = declared != 0 && u32_at(bytes, 24)? == u32_at(bytes, 92)?;
    if usable < 480 || bytes[18..20] != [1, 1] || bytes[21..24] != [64, 32, 32] || authoritative && declared != pages || u32_at(bytes, 44)? != 4 || u32_at(bytes, 56)? != 1 || bytes[72..92].iter().any(|byte| *byte != 0) { return Err(SqliteSnapshotError::Malformed("unsupported database header")); }
    if u32_at(bytes, 68)? != SQLITE_SNAPSHOT_APPLICATION_ID || u32_at(bytes, 60)? != SQLITE_SNAPSHOT_USER_VERSION { return Err(SqliteSnapshotError::Malformed("application identity or version")); }
    if u32_at(bytes, 32)? as usize > pages || u32_at(bytes, 36)? as usize > pages || u32_at(bytes, 52)? as usize > pages { return Err(SqliteSnapshotError::Malformed("page metadata")); }
    checkpoint(callback, SqliteSnapshotPhase::ReadPages, 0, pages)?; let mut reader = Reader { bytes, page_size, usable, pages, visited: BTreeSet::new(), callback }; let mut tables = Vec::new(); let mut schema_bytes = 0usize; let mut names = BTreeSet::new();
    reader.freelist(u32_at(bytes, 32)?, u32_at(bytes, 36)? as usize)?;
    reader.table(1, limits.max_schema_bytes, 2, 67, limits.max_tables, &mut |_, schema, remaining| {
        let fields = read_record(schema, 5, remaining.saturating_mul(2).saturating_add(67))?; let cost;
        match fields.as_slice() {
            [SqliteValue::Text(kind), SqliteValue::Text(name), SqliteValue::Text(table), SqliteValue::Integer(page), SqliteValue::Text(sql)] if kind == "table" && name.eq_ignore_ascii_case(table) && *page > 1 && *page as u64 <= pages as u64 => {
                cost = sql.len().checked_add(name.len()).ok_or(SqliteSnapshotError::Limit("schema bytes"))?;
                schema_bytes = schema_bytes.checked_add(sql.len()).and_then(|sum| sum.checked_add(name.len())).ok_or(SqliteSnapshotError::Limit("schema bytes"))?; limit(schema_bytes, limits.max_schema_bytes, "schema bytes")?;
                let definition = parse_table(sql)?; if !definition.name.eq_ignore_ascii_case(name) || !names.insert(name.to_ascii_lowercase()) { return Err(SqliteSnapshotError::Malformed("schema name mismatch or duplicate")); } limit(definition.columns.len(), limits.max_columns, "columns")?;
                tables.push((SqliteTable { name: name.clone(), sql: sql.clone(), rows: Vec::new() }, definition, *page as u32));
            }
            _ => return Err(SqliteSnapshotError::Malformed("unsupported schema object or record")),
        }
        Ok(cost)
    })?;
    let mut total_rows = 0usize; let mut value_bytes = 0usize;
    for (table, definition, root) in &mut tables {
        reader.table(*root, limits.max_value_bytes.saturating_sub(value_bytes), 1, definition.columns.len().saturating_mul(9).saturating_add(9), limits.max_rows.saturating_sub(total_rows), &mut |rowid, payload, remaining| {
            let alias_bytes = if definition.primary_key.is_some() { 8 } else { 0 }; let remaining = remaining.checked_sub(alias_bytes).ok_or(SqliteSnapshotError::Limit("value bytes"))?;
            let mut values = read_record(payload, limits.max_columns, remaining)?; if values.len() != definition.columns.len() { return Err(SqliteSnapshotError::Malformed("row column count")); }
            if let Some(primary) = definition.primary_key { if values[primary] != SqliteValue::Null { return Err(SqliteSnapshotError::Malformed("INTEGER PRIMARY KEY storage must be NULL")); } values[primary] = SqliteValue::Integer(rowid); }
            for (column, value) in definition.columns.iter().zip(&mut values) { if column.not_null && matches!(value, SqliteValue::Null) { return Err(SqliteSnapshotError::Malformed("NOT NULL violation")); } if column.real_affinity { if let SqliteValue::Integer(number) = value { *value = SqliteValue::Real(*number as f64); } } }
            let mut cost = 0usize; for value in &values { cost = cost.checked_add(value_size(value)).ok_or(SqliteSnapshotError::Limit("value bytes"))?; value_bytes = value_bytes.checked_add(value_size(value)).ok_or(SqliteSnapshotError::Limit("value bytes"))?; limit(value_bytes, limits.max_value_bytes, "value bytes")?; }
            table.rows.push(SqliteRow { rowid, values }); total_rows += 1; Ok(cost)
        })?;
    }
    if reader.visited.len() < pages { checkpoint(reader.callback, SqliteSnapshotPhase::ReadPages, pages, pages)?; }
    Ok(SqliteDatabase { tables: tables.into_iter().map(|(table, _, _)| table).collect() })
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
