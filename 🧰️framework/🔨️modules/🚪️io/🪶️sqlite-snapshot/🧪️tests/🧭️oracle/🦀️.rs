//! 🧭️ Std-only tagged-row interchange for independent relational SQLite validation.

use semio_framework_io_sqlite_snapshot::*;

fn unhex(value: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    if value.len() % 2 != 0 { return Err("odd hexadecimal length".into()); }
    value.as_bytes().chunks_exact(2).map(|pair| Ok(u8::from_str_radix(std::str::from_utf8(pair)?, 16)?)).collect()
}

fn hex(value: &[u8]) -> String { value.iter().map(|value| format!("{value:02x}")).collect() }

fn value(field: &str) -> Result<SqliteValue, Box<dyn std::error::Error>> {
    if field == "n" { return Ok(SqliteValue::Null); }
    let (tag, value) = field.split_once(':').ok_or("missing value tag")?;
    Ok(match tag { "i" => SqliteValue::Integer(value.parse()?), "r" => SqliteValue::Real(value.parse()?), "t" => SqliteValue::Text(String::from_utf8(unhex(value)?)?), "b" => SqliteValue::Blob(unhex(value)?), _ => return Err("invalid value tag".into()) })
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.as_slice() {
        [operation, database, schema, rows] if operation == "export" => {
            let mut data = SqliteDatabase::from_schema(&std::fs::read_to_string(schema)?)?;
            for row in std::fs::read_to_string(rows)?.lines() {
                if row.is_empty() { continue; }
                let mut fields = row.split('\t'); let table = String::from_utf8(unhex(fields.next().ok_or("missing table")?)?)?; let rowid = fields.next().ok_or("missing rowid")?.parse()?;
                let values = fields.map(value).collect::<Result<Vec<_>, _>>()?;
                data.table_mut(&table)?.rows.push(SqliteRow { rowid, values });
            }
            std::fs::write(database, export_sqlite_database(&data, SqliteDatabaseLimits::default(), &mut |_| true)?)?;
        }
        [operation, database, rows] if operation == "import" => {
            let mut data = import_sqlite_database(&std::fs::read(database)?, SqliteDatabaseLimits::default(), &mut |_| true)?;
            data.tables.sort_by(|left, right| left.name.cmp(&right.name)); let mut output = String::new();
            for table in data.tables { let mut rows = table.rows; rows.sort_by_key(|row| row.rowid); for row in rows {
                output.push_str(&hex(table.name.as_bytes())); output.push('\t'); output.push_str(&row.rowid.to_string());
                for value in row.values { output.push('\t'); output.push_str(&match value { SqliteValue::Null => "n".into(), SqliteValue::Integer(value) => format!("i:{value}"), SqliteValue::Real(value) => format!("r:{value}"), SqliteValue::Text(value) => format!("t:{}", hex(value.as_bytes())), SqliteValue::Blob(value) => format!("b:{}", hex(&value)) }); } output.push('\n');
            } }
            std::fs::write(rows, output)?;
        }
        _ => return Err("usage: export DATABASE SCHEMAFILE ROWSFILE | import DATABASE ROWSFILE".into()),
    }
    Ok(())
}

fn main() { if let Err(error) = run() { eprintln!("{error}"); std::process::exit(1); } }
