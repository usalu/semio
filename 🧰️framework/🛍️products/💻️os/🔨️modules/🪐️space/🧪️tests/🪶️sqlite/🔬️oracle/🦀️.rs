//! 🔬️ Independent typed SQLite fixture and Bun edit oracle for builtin persisted owners.
use super::store;
use store::sqlite_snapshot::*;
pub(super) fn renumber(database: &SqliteDatabase, offset: i64) -> SqliteDatabase {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let bytes = export_sqlite_database(database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let script = r#"import{Database}from'bun:sqlite';const d=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));const offset=Number(process.argv.at(-1));const q=s=>'"'+s.replaceAll('"','""')+'"';d.exec('PRAGMA foreign_keys=OFF');for(const{name}of d.query("SELECT name FROM sqlite_schema WHERE type='table'").all()){const columns=new Set(d.query('PRAGMA foreign_key_list('+q(name)+')').all().map(row=>row.from).filter(name=>name!=='id'));d.exec('UPDATE '+q(name)+' SET '+['id=id+'+offset,...[...columns].map(name=>q(name)+'='+q(name)+'+'+offset)].join(','));}if(d.query('PRAGMA integrity_check').get().integrity_check!=='ok'||d.query('PRAGMA foreign_key_check').all().length)throw Error('renumber integrity');await Bun.write(Bun.stdout,d.serialize());d.close();"#;
    let mut child = Command::new("bun").args(["-e", script, &offset.to_string()]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(&bytes).unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    import_sqlite_database(&output.stdout, SqliteDatabaseLimits::default(), &mut |_| true).unwrap()
}
pub(super) fn database(f: &serde_json::Value, sql: &str) -> SqliteDatabase {
    let mut database = SqliteDatabase::from_schema(sql).unwrap();
    for table in &mut database.tables {
        table.rows = f["rows"][&table.name]
            .as_array()
            .unwrap()
            .iter()
            .map(|row| {
                let values = row
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|cell| match cell {
                        serde_json::Value::Null => SqliteValue::Null,
                        serde_json::Value::String(value) => SqliteValue::Text(value.clone()),
                        serde_json::Value::Number(value) => SqliteValue::Integer(value.as_i64().unwrap()),
                        _ => panic!("fixture SQLite cell"),
                    })
                    .collect::<Vec<_>>();
                SqliteRow { rowid: row[0].as_i64().unwrap(), values }
            })
            .collect();
    }
    database
}
pub(super) fn edit(database: &SqliteDatabase, f: &serde_json::Value) -> SqliteDatabase {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let bytes = export_sqlite_database(database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let script = r#"import{Database}from'bun:sqlite';const input=Buffer.from(await Bun.stdin.arrayBuffer());const head=input.readUInt32LE(0);const edit=JSON.parse(input.subarray(4,4+head).toString('utf8'));const d=Database.deserialize(new Uint8Array(input.subarray(4+head)));const q=s=>'"'+s.replaceAll('"','""')+'"';d.query('UPDATE '+q(edit.table)+' SET '+q(edit.column)+'=?').run(edit.value);if(d.query('PRAGMA integrity_check').get().integrity_check!=='ok'||d.query('PRAGMA foreign_key_check').all().length)throw Error('edit integrity');await Bun.write(Bun.stdout,d.serialize());d.close();"#;
    let mut child = Command::new("bun").args(["-e", script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    let head = f["edit"].to_string();
    let mut stdin = child.stdin.take().unwrap();
    stdin.write_all(&(head.len() as u32).to_le_bytes()).unwrap();
    stdin.write_all(head.as_bytes()).unwrap();
    stdin.write_all(&bytes).unwrap();
    drop(stdin);
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    import_sqlite_database(&output.stdout, SqliteDatabaseLimits::default(), &mut |_| true).unwrap()
}
