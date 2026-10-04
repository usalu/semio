//! 🏛️ Dependency-free relational SQLite interchange; https://sqlite.org/fileformat.html.

use std::collections::BTreeSet;
pub use semio_framework_value::{ValueError, ValueRefusalKind};

#[path = "🧩️artifact/🦀️.rs"]
pub mod artifact;

#[path = "🔁️transfer/🦀️.rs"]
pub mod transfer;
pub use transfer::{export_controlled as export_sqlite_database_controlled,import_controlled as import_sqlite_database_controlled,validate_database_controlled as validate_sqlite_database_schema_controlled,validate_table_controlled as validate_sqlite_table_schema_controlled};

pub const SQLITE_SNAPSHOT_APPLICATION_ID: u32 = 0x534d534e;
pub const SQLITE_SNAPSHOT_USER_VERSION: u32 = 1;
const PAGE_SIZE: usize = 4096;

/// 📜️ Native representation choices used by artifact providers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SnapshotEncoding { Binary, Text }

impl SnapshotEncoding {
    pub fn as_str(self) -> &'static str { match self { Self::Binary => "binary", Self::Text => "text" } }
    pub fn parse(value: &str) -> Result<Self> { match value { "binary" => Ok(Self::Binary), "text" => Ok(Self::Text), _ => Err(ValueError::new(ValueRefusalKind::InvalidValue, "encoding requires binary or text")) } }
}

/// 🔢️ SQLite storage classes with owned, native scalar values.
#[derive(Clone, Debug, PartialEq)]
pub enum SqliteValue { Null, Integer(i64), Real(f64), Text(String), Blob(Vec<u8>) }

/// 📑️ One row with its signed SQLite rowid and semantic column values.
#[derive(Clone, Debug, PartialEq)]
pub struct SqliteRow { pub rowid: i64, pub values: Vec<SqliteValue> }

impl SqliteRow {
    pub fn integer(&self, index: usize) -> std::result::Result<i64, ValueError> { match self.values.get(index) { Some(SqliteValue::Integer(value)) => Ok(*value), _ => Err(ValueError::new(ValueRefusalKind::InvalidValue, format!("column {index} requires INTEGER"))) } }
    pub fn real(&self, index: usize) -> std::result::Result<f64, ValueError> { match self.values.get(index) { Some(SqliteValue::Real(value)) => Ok(*value), Some(SqliteValue::Integer(value)) => Ok(*value as f64), _ => Err(ValueError::new(ValueRefusalKind::InvalidValue, format!("column {index} requires REAL"))) } }
    pub fn text(&self, index: usize) -> std::result::Result<&str, ValueError> { match self.values.get(index) { Some(SqliteValue::Text(value)) => Ok(value), _ => Err(ValueError::new(ValueRefusalKind::InvalidValue, format!("column {index} requires TEXT"))) } }
    pub fn blob(&self, index: usize) -> std::result::Result<&[u8], ValueError> { match self.values.get(index) { Some(SqliteValue::Blob(value)) => Ok(value), _ => Err(ValueError::new(ValueRefusalKind::InvalidValue, format!("column {index} requires BLOB"))) } }
    pub fn optional_text(&self, index: usize) -> std::result::Result<Option<&str>, ValueError> { match self.values.get(index) { Some(SqliteValue::Text(value)) => Ok(Some(value)), Some(SqliteValue::Null) => Ok(None), _ => Err(ValueError::new(ValueRefusalKind::InvalidValue, format!("column {index} requires TEXT or NULL"))) } }
}

/// 🧱️ Handwritten domain table definition and its typed relational rows.
#[derive(Clone, Debug, PartialEq)]
pub struct SqliteTable { pub name: String, pub sql: String, pub rows: Vec<SqliteRow> }

impl SqliteTable {
    pub fn single_row(&self) -> std::result::Result<&SqliteRow, ValueError> { if self.rows.len() == 1 { Ok(&self.rows[0]) } else { Err(ValueError::new(ValueRefusalKind::InvalidValue, format!("table {} requires exactly one row", self.name))) } }
    pub fn ordered_rows(&self, ordinal_column: usize) -> std::result::Result<Vec<&SqliteRow>, ValueError> {
        let mut rows = self.rows.iter().map(|row| Ok((row.integer(ordinal_column)?, row))).collect::<std::result::Result<Vec<_>, ValueError>>()?;
        rows.sort_by_key(|(ordinal, _)| *ordinal);
        for (expected, (ordinal, _)) in rows.iter().enumerate() { if *ordinal != expected as i64 { return Err(ValueError::new(ValueRefusalKind::InvalidValue, format!("table {} requires contiguous zero-based ordinals", self.name))); } }
        Ok(rows.into_iter().map(|(_, row)| row).collect())
    }
}

/// 🏗️ A relational database projected explicitly by one artifact provider.
#[derive(Clone, Debug, PartialEq)]
pub struct SqliteDatabase { pub tables: Vec<SqliteTable> }

/// 🏛️ Validates one declared table without cloning its rows or requiring a single-table database.
pub fn validate_sqlite_table_schema(table: &SqliteTable, sql: &str, limits: SqliteDatabaseLimits) -> Result<()> {
    limit(sql.len(), limits.max_schema_bytes, ValueRefusalKind::OwnershipLimit, "schema bytes")?;
    limit(table.sql.len().checked_add(table.name.len()).ok_or(ownership_limit("schema bytes"))?, limits.max_schema_bytes, ValueRefusalKind::OwnershipLimit, "schema bytes")?;
    let actual = parse_table(&table.sql)?; let expected = parse_table(sql)?; limit(actual.columns.len(), limits.max_columns, ValueRefusalKind::WorkLimit, "columns")?;
    if !table.name.eq_ignore_ascii_case(&actual.name) || !table.name.eq_ignore_ascii_case(&expected.name) || !schema_matches(&table.sql, sql)? { return Err(invalid("artifact table schema")); }
    Ok(())
}

/// 🏛️ Validates an artifact's declared relational contract without normalizing quoted syntax into keywords.
pub fn validate_sqlite_database_schema(database: &SqliteDatabase, sql: &str, limits: SqliteDatabaseLimits) -> Result<()> {
    limit(sql.len(), limits.max_schema_bytes, ValueRefusalKind::OwnershipLimit, "schema bytes")?; limit(database.tables.len(), limits.max_tables, ValueRefusalKind::WorkLimit, "tables")?;
    let mut bytes = 0usize; for table in &database.tables { bytes = bytes.checked_add(table.sql.len()).and_then(|sum| sum.checked_add(table.name.len())).ok_or(ownership_limit("schema bytes"))?; limit(bytes, limits.max_schema_bytes, ValueRefusalKind::OwnershipLimit, "schema bytes")?; }
    let expected = SqliteDatabase::from_schema(sql)?;
    if database.tables.len() != expected.tables.len() { return Err(invalid("artifact table count")); }
    let mut names = BTreeSet::new();
    for table in &database.tables {
        if !names.insert(table.name.to_ascii_lowercase()) { return Err(invalid("artifact duplicate table name")); }
        let declared = expected.tables.iter().find(|declared| declared.name.eq_ignore_ascii_case(&table.name)).ok_or(invalid("artifact table name"))?;
        validate_sqlite_table_schema(table, &declared.sql, limits)?;
    }
    Ok(())
}

impl SqliteDatabase {
    /// 🏗️ Creates the actual declared tables under the caller's schema, backing and cancellation authority.
    pub fn from_schema_controlled(sql:&str,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<Self>{transfer::schema_controlled(sql,phase,control)}

    pub fn from_schema(sql: &str) -> Result<Self> {
        let tokens = lex(sql)?;
        let mut start = 0;
        let mut tables = Vec::new();
        for end in 0..=tokens.len() {
            if end != tokens.len() && (tokens[end].quoted || tokens[end].literal || tokens[end].text != ";") { continue; }
            if start < end {
                let statement = sql[tokens[start].start..tokens[end - 1].end].trim();
                let definition = parse_table(statement)?;
                if tables.iter().any(|table: &SqliteTable| table.name.eq_ignore_ascii_case(&definition.name)) { return Err(invalid("duplicate table name")); }
                tables.push(SqliteTable { name: definition.name, sql: statement.into(), rows: Vec::new() });
            }
            start = end + 1;
        }
        Ok(Self { tables })
    }

    pub fn table(&self, name: &str) -> std::result::Result<&SqliteTable, ValueError> { self.tables.iter().find(|table| table.name.eq_ignore_ascii_case(name)).ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue, format!("missing table {name}"))) }
    pub fn table_mut(&mut self, name: &str) -> std::result::Result<&mut SqliteTable, ValueError> { self.tables.iter_mut().find(|table| table.name.eq_ignore_ascii_case(name)).ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue, format!("missing table {name}"))) }
}

/// 📏️ Allocation and structure bounds applied before untrusted lengths allocate.
#[derive(Clone, Copy, Debug)]
pub struct SqliteDatabaseLimits {
    pub max_file_bytes: usize,
    pub max_value_bytes: usize,
    pub max_allocation_bytes: usize,
    pub max_schema_bytes: usize,
    pub max_rows: usize,
    pub max_columns: usize,
    pub max_tables: usize,
    pub max_pages: usize,
}

impl Default for SqliteDatabaseLimits {
    fn default() -> Self { Self { max_file_bytes: 272 * 1024 * 1024, max_value_bytes: 256 * 1024 * 1024, max_allocation_bytes: 512 * 1024 * 1024, max_schema_bytes: 4 * 1024 * 1024, max_rows: 1_000_000, max_columns: 1024, max_tables: 4096, max_pages: 1_000_000 } }
}

/// 🧭️ Transfer and artifact projection stages share one cancellation boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SqliteSnapshotPhase { ReadPages, WritePages, ProjectSnapshot, ReconstructSnapshot, DecodeNative, EncodeNative }

/// ⏱️ Stage-local progress; physical import counts visited pages, and false cancels.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SqliteSnapshotProgress { pub phase: SqliteSnapshotPhase, pub completed: usize, pub total: usize }

/// 🛑️ One progress and cancellation authority spanning typed projection and physical I/O.
pub struct SqliteSnapshotControl<'a> { callback: &'a mut dyn FnMut(SqliteSnapshotProgress) -> bool, limits: SqliteDatabaseLimits, reconstruction_bytes: usize, reconstruction_units: usize, reconstruction_scalar_bytes: usize, allocation_bytes: usize, physical_read_progress: Option<SqliteSnapshotProgress> }

impl<'a> SqliteSnapshotControl<'a> {
    pub fn new(callback: &'a mut dyn FnMut(SqliteSnapshotProgress) -> bool, limits: SqliteDatabaseLimits) -> Self { Self { callback, limits, reconstruction_bytes: 0, reconstruction_units: 0, reconstruction_scalar_bytes: 0, allocation_bytes: 0, physical_read_progress: None } }
    pub fn limits(&self) -> SqliteDatabaseLimits { self.limits }
    /// 📏️ Remaining cumulative backing admission, independent of semantic payload bytes.
    pub fn allocation_remaining_bytes(&self) -> usize { self.limits.max_allocation_bytes - self.allocation_bytes }
    /// 🏗️ Admits one concrete owned backing allocation before construction; retirement never refunds it.
    pub fn admit_allocation_bytes(&mut self, count: usize) -> std::result::Result<(), ValueError> {
        let next = self.allocation_bytes.checked_add(count).filter(|next| *next <= self.limits.max_allocation_bytes).ok_or_else(|| ownership_limit("allocation bytes"))?;
        self.allocation_bytes = next;
        Ok(())
    }
    /// 🪆️ Settles a child producer's admitted backing even when its typed construction refuses or cancels.
    pub fn allocation_stage<T,E>(&mut self, phase: SqliteSnapshotPhase, operation: impl FnOnce(usize,&mut dyn FnMut(usize,usize)->bool)->(std::result::Result<T,E>,usize)) -> std::result::Result<std::result::Result<T,E>,ValueError> {
        self.checkpoint(phase,0,0)?;
        let remaining = self.allocation_remaining_bytes();
        let (result,owned) = { let mut progress = |completed,total| self.accept_progress(phase,completed,total); operation(remaining,&mut progress) };
        self.admit_allocation_bytes(owned)?;
        Ok(result)
    }
    pub fn check_rows(&self, count: usize) -> std::result::Result<(), ValueError> { limit(count, self.limits.max_rows, ValueRefusalKind::WorkLimit, "rows") }
    pub fn check_value_bytes(&self, count: usize) -> std::result::Result<(), ValueError> { limit(count, self.limits.max_value_bytes, ValueRefusalKind::OwnershipLimit, "value bytes") }
    /// 📏️ Exposes remaining reconstruction ownership before a native allocator starts.
    pub fn reconstruction_remaining_bytes(&self) -> std::result::Result<usize,ValueError> {
        let used=self.reconstruction_bytes.checked_add(self.reconstruction_scalar_bytes).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "native reconstruction aggregate byte count overflow"))?;
        self.limits.max_value_bytes.checked_sub(used).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit, "native reconstruction ownership exceeds caller limit"))
    }
    /// 📥️ Retains the actual admitted native allocator ledger across domain constructions.
    pub fn admit_reconstruction_bytes(&mut self,count:usize) -> std::result::Result<(),ValueError> {
        if count>self.reconstruction_remaining_bytes()?{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit, "native reconstruction ownership exceeds caller limit"))}
        self.reconstruction_bytes=self.reconstruction_bytes.checked_add(count).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "native reconstruction value byte count overflow"))?;
        Ok(())
    }
    pub fn check_database(&mut self, database: &SqliteDatabase, phase: SqliteSnapshotPhase) -> std::result::Result<(), ValueError> {
        self.checkpoint(phase, 0, 0)?; let total = database.tables.iter().try_fold(0usize, |count, table| count.checked_add(table.rows.len()).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "SQLite row count overflow")))?; self.check_rows(total)?; let mut rows = 0usize; let mut bytes = 0usize; let mut scalar_bytes = 0usize;
        for table in &database.tables { for row in &table.rows { rows = rows.checked_add(1).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "SQLite row count overflow"))?; self.check_rows(rows)?; for value in &row.values { let size = value_size(value); bytes = bytes.checked_add(size).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "SQLite value byte count overflow"))?; if matches!(value, SqliteValue::Integer(_) | SqliteValue::Real(_)) { scalar_bytes = scalar_bytes.checked_add(size).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "SQLite scalar byte count overflow"))?; } self.check_value_bytes(bytes)?; } if rows % 256 == 0 { self.checkpoint(phase, rows, total)?; } } }
        if phase == SqliteSnapshotPhase::ReconstructSnapshot { self.reconstruction_scalar_bytes = self.reconstruction_scalar_bytes.max(scalar_bytes); }
        self.checkpoint(phase, rows, total)
    }
    fn accept_progress(&mut self, phase: SqliteSnapshotPhase, completed: usize, total: usize) -> bool { let progress=if phase==SqliteSnapshotPhase::ReadPages{self.physical_read_progress.unwrap_or(SqliteSnapshotProgress { phase, completed, total })}else{SqliteSnapshotProgress { phase, completed, total }};(self.callback)(progress) }
    pub fn checkpoint(&mut self, phase: SqliteSnapshotPhase, completed: usize, total: usize) -> std::result::Result<(), ValueError> { if self.accept_progress(phase,completed,total) { Ok(()) } else { Err(ValueError::new(ValueRefusalKind::Canceled, "relational SQLite transfer cancelled")) } }
}

type Result<T> = std::result::Result<T, ValueError>;

fn invalid(reason: &str) -> ValueError { ValueError::new(ValueRefusalKind::InvalidValue, format!("invalid relational SQLite: {reason}")) }
fn ownership_limit(reason: &str) -> ValueError { ValueError::new(ValueRefusalKind::OwnershipLimit, format!("relational SQLite resource limit: {reason}")) }
fn work_limit(reason: &str) -> ValueError { ValueError::new(ValueRefusalKind::WorkLimit, format!("relational SQLite resource limit: {reason}")) }
fn limit(value: usize, maximum: usize, kind: ValueRefusalKind, name: &str) -> Result<()> { if value > maximum { Err(ValueError::new(kind, format!("relational SQLite resource limit: {name}"))) } else { Ok(()) } }
fn checkpoint(callback: &mut dyn FnMut(SqliteSnapshotProgress) -> bool, phase: SqliteSnapshotPhase, completed: usize, total: usize) -> Result<()> { if callback(SqliteSnapshotProgress { phase, completed, total }) { Ok(()) } else { Err(ValueError::new(ValueRefusalKind::Canceled, "relational SQLite transfer cancelled")) } }
fn put_u16(bytes: &mut [u8], at: usize, value: usize) { bytes[at..at + 2].copy_from_slice(&(value as u16).to_be_bytes()); }
fn put_u32(bytes: &mut [u8], at: usize, value: u32) { bytes[at..at + 4].copy_from_slice(&value.to_be_bytes()); }
fn u16_at(bytes: &[u8], at: usize) -> Result<usize> { let value = bytes.get(at..at + 2).ok_or(invalid("truncated integer"))?; Ok(u16::from_be_bytes([value[0], value[1]]) as usize) }
fn u32_at(bytes: &[u8], at: usize) -> Result<u32> { let value = bytes.get(at..at + 4).ok_or(invalid("truncated integer"))?; Ok(u32::from_be_bytes([value[0], value[1], value[2], value[3]])) }

fn varint(mut value: u64) -> Vec<u8> {
    if value > 0x00ff_ffff_ffff_ffff { let mut bytes = vec![0; 9]; bytes[8] = value as u8; value >>= 8; for index in (0..8).rev() { bytes[index] = (value as u8 & 0x7f) | 0x80; value >>= 7; } return bytes; }
    let mut bytes = vec![value as u8 & 0x7f]; value >>= 7; while value != 0 { bytes.push((value as u8 & 0x7f) | 0x80); value >>= 7; } bytes.reverse(); bytes
}

fn read_varint(bytes: &[u8], at: &mut usize) -> Result<u64> {
    let mut value = 0; for index in 0..9 { let byte = *bytes.get(*at).ok_or(invalid("truncated varint"))?; *at += 1; if index == 8 { return Ok((value << 8) | byte as u64); } value = (value << 7) | (byte & 0x7f) as u64; if byte & 0x80 == 0 { return Ok(value); } } Err(invalid("invalid varint"))
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
        if bytes[at..].starts_with(b"/*") { at += 2; while at + 1 < bytes.len() && !bytes[at..].starts_with(b"*/") { at += 1; } if at + 1 >= bytes.len() { return Err(invalid("unterminated SQL comment")); } at += 2; continue; }
        let start = at; let mut text = String::new(); let quoted = matches!(bytes[at], b'"' | b'`' | b'['); let literal = bytes[at] == b'\'';
        if quoted || literal {
            let close = if bytes[at] == b'[' { b']' } else { bytes[at] }; at += 1; let mut segment = at;
            loop { if at >= bytes.len() { return Err(invalid("unterminated SQL quote")); } if bytes[at] == close { text.push_str(&sql[segment..at]); at += 1; if close != b']' && bytes.get(at) == Some(&close) { text.push(close as char); at += 1; segment = at; } else { break; } } else { at += 1; } }
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
    if tokens.len() < 5 || !tokens[0].keyword("CREATE") || !tokens[1].keyword("TABLE") || tokens[2].literal || tokens[2].text.is_empty() || tokens[3].text != "(" { return Err(invalid("expected CREATE TABLE")); }
    if tokens.iter().any(|token| ["UNIQUE", "AUTOINCREMENT", "WITHOUT", "DESC", "GENERATED"].iter().any(|keyword| token.keyword(keyword))) { return Err(invalid("unsupported implicit index or table feature")); }
    let name = tokens[2].text.clone();
    if name.to_ascii_lowercase().starts_with("sqlite_") { return Err(invalid("reserved table name")); }
    let mut groups = Vec::new(); let mut depth = 1; let mut start = 4; let mut close = None;
    for (at, token) in tokens.iter().enumerate().skip(4) {
        if token.quoted || token.literal { continue; }
        match token.text.as_str() { "(" => depth += 1, ")" => { depth -= 1; if depth == 0 { if start < at { groups.push(&tokens[start..at]); } close = Some(at); break; } }, "," if depth == 1 => { if start == at { return Err(invalid("empty column definition")); } groups.push(&tokens[start..at]); start = at + 1; }, _ => {} }
    }
    let close = close.ok_or(invalid("unbalanced table definition"))?;
    if tokens[close + 1..].iter().any(|token| token.text != ";") || groups.is_empty() { return Err(invalid("unsupported table suffix or empty table")); }
    let mut columns = Vec::new(); let mut primary_name = None;
    for mut group in groups {
        if group[0].keyword("CONSTRAINT") { if group.len() < 3 { return Err(invalid("incomplete named constraint")); } group = &group[2..]; }
        if group[0].keyword("PRIMARY") {
            if group.len() < 5 || !group[1].keyword("KEY") || group[2].text != "(" || group.last().is_none_or(|token| token.text != ")") || (group.len() != 5 && !(group.len() == 6 && group[4].keyword("ASC"))) || primary_name.is_some() { return Err(invalid("unsupported table primary key")); }
            primary_name = Some(group[3].text.clone()); continue;
        }
        if group[0].keyword("FOREIGN") || group[0].keyword("CHECK") { continue; }
        if group[0].literal || group[0].text.is_empty() || columns.iter().any(|column: &Column| column.name.eq_ignore_ascii_case(&group[0].text)) { return Err(invalid("invalid or duplicate column")); }
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
        if group.windows(2).any(|pair| pair[0].keyword("PRIMARY") && pair[1].keyword("KEY")) { if primary_name.is_some() { return Err(invalid("multiple primary keys")); } primary_name = Some(group[0].text.clone()); }
        columns.push(Column { name: group[0].text.clone(), integer_type, real_affinity, not_null });
    }
    if columns.is_empty() { return Err(invalid("table has no columns")); }
    let primary_key = if let Some(primary) = primary_name { let index = columns.iter().position(|column| column.name.eq_ignore_ascii_case(&primary)).ok_or(invalid("unknown primary key column"))?; if !columns[index].integer_type { return Err(invalid("non-INTEGER primary key creates implicit index")); } Some(index) } else { None };
    Ok(Definition { name, columns, primary_key })
}

fn value_size(value: &SqliteValue) -> usize { match value { SqliteValue::Null => 0, SqliteValue::Integer(_) | SqliteValue::Real(_) => 8, SqliteValue::Text(value) => value.len(), SqliteValue::Blob(value) => value.len() } }

/// 📤️ Exports handcrafted entity tables, relationships and scalar values as ordinary SQLite.
pub fn export_sqlite_database(database: &SqliteDatabase, limits: SqliteDatabaseLimits, callback: &mut dyn FnMut(SqliteSnapshotProgress) -> bool) -> Result<Vec<u8>> {
    export_sqlite_database_controlled(database, &mut SqliteSnapshotControl::new(callback, limits))
}

/// 📥️ Imports independent SQLite rowid tables with semantic primary keys and REAL affinity.
pub fn import_sqlite_database(bytes: &[u8], limits: SqliteDatabaseLimits, callback: &mut dyn FnMut(SqliteSnapshotProgress) -> bool) -> Result<SqliteDatabase> {
    import_sqlite_database_controlled(bytes, &mut SqliteSnapshotControl::new(callback, limits))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "⚠️refusal/🧪️tests/🦀️.rs"]
mod typed_refusal_tests;
