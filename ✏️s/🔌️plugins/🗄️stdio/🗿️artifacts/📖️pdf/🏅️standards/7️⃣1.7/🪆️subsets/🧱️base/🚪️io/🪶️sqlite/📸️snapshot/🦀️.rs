//! 📖️ Explicit PDF 1.7 document, COS, rendering and resource relations.
use semio_framework_value::{ValueError,ValueRefusalKind};
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use store::sqlite_snapshot::{self, SqliteDatabase as Db, SqliteRow as RawRow, SqliteValue as V, SqliteSnapshotControl as Control, SqliteSnapshotPhase as Phase, artifact::{Cell as C}};
use std::collections::{BTreeMap, BTreeSet};
#[path = "🔢️number/🦀️.rs"]
mod number;
use number::{Row,Projection};

#[path = "🧩️cos/🦀️.rs"]
mod cos;
#[path = "🌈️color/🦀️.rs"]
mod color;
#[path = "🔤️font/🦀️.rs"]
mod font;
#[path = "🖋️content/🦀️.rs"]
mod content;
#[path = "🖼️resource/🦀️.rs"]
mod resource;
#[path = "📇️metadata/🦀️.rs"]
mod metadata;
#[path = "🎯️navigation/🦀️.rs"]
mod navigation;
#[path = "📌️annotation/🦀️.rs"]
mod annotation;
#[path = "📝️form/🦀️.rs"]
mod form;
#[path = "📄️document/🦀️.rs"]
mod document;
#[path = "🛂️admission/🦀️.rs"]
mod admission;

fn integer<T: TryFrom<i64>>(row:Row<'_>, column: usize) -> Result<T,ValueError> { T::try_from(row.integer(column)?).map_err(|_| ValueError::new(ValueRefusalKind::InvalidValue,"PDF integer is outside its declared range")) }
fn boolean(row:Row<'_>, column: usize) -> Result<bool,ValueError> { match row.integer(column)? { 0 => Ok(false), 1 => Ok(true), _ => Err(ValueError::new(ValueRefusalKind::InvalidValue,"PDF boolean must be zero or one")) } }
fn optional_integer(row:Row<'_>, column: usize) -> Result<Option<i64>,ValueError> { if row.values.get(column) == Some(&V::Null) { Ok(None) } else { row.integer(column).map(Some) } }
fn optional_typed_integer<T: TryFrom<i64>>(row:Row<'_>, column: usize) -> Result<Option<T>,ValueError> { if row.values.get(column) == Some(&V::Null) { Ok(None) } else { integer(row, column).map(Some) } }
fn optional_real(row:Row<'_>, column: usize) -> Result<Option<f64>,ValueError> { if row.is_null(column)? { Ok(None) } else { row.real(column).map(Some) } }
fn optional_boolean(row:Row<'_>, column: usize) -> Result<Option<bool>,ValueError> { if row.values.get(column) == Some(&V::Null) { Ok(None) } else { boolean(row, column).map(Some) } }
fn real_cell(value: Option<f64>) -> C<'static> { value.map_or(C::Null, C::Real) }
fn integer_cell(value: Option<u32>) -> C<'static> { value.map_or(C::Null, |value| C::Integer(i64::from(value))) }
fn boolean_cell(value: Option<bool>) -> C<'static> { value.map_or(C::Null, |value| C::Integer(i64::from(value))) }
fn text_cell(value: &Option<String>) -> C<'_> { value.as_deref().map_or(C::Null, C::Text) }
fn optional_array<const N: usize>(row:Row<'_>, start: usize) -> Result<Option<[f64; N]>,ValueError> { let mut values = [0.0; N]; let mut present = 0; for (index, value) in values.iter_mut().enumerate() { if let Some(number) = optional_real(row, start + index)? { *value = number; present += 1; } } match present { 0 => Ok(None), count if count == N => Ok(Some(values)), _ => Err(ValueError::new(ValueRefusalKind::InvalidValue,"PDF optional array has partially absent components")) } }
fn array_cells<const N: usize>(value: &Option<[f64; N]>) -> [C<'static>; N] { value.map_or([C::Null; N], |value| value.map(C::Real)) }
fn write_sequence(out: &mut Projection<'_, '_>, table: &str, owner: i64, values: &[f64]) -> Result<(),ValueError> { for (ordinal, value) in values.iter().enumerate() { out.insert(table, &[C::Integer(owner), C::Integer(ordinal as i64), C::Real(*value)])?; } Ok(()) }
fn read_sequence(reader: &mut Reader<'_, '_, '_>, table: &'static str, owner: i64) -> Result<Vec<f64>,ValueError> { let mut values = Vec::new(); for row in reader.children(table, 1, 2, owner)? { let row = reader.take(table, row.rowid, 4)?; values.push(row.real(3)?); } Ok(values) }
fn null_except(row:Row<'_>, columns: std::ops::Range<usize>, present: &[usize]) -> Result<(),ValueError> { for column in columns { if !present.contains(&column) && !row.is_null(column)? { return Err(ValueError::new(ValueRefusalKind::InvalidValue,"PDF variant contains a property belonging to another kind")); } } Ok(()) }

struct Reader<'a, 'c, 'p> {
    db: &'a Db,
    rows: BTreeMap<(&'a str, i64), Row<'a>>,
    groups: BTreeMap<(&'a str, usize, usize, Option<(usize, &'a str)>), BTreeMap<i64, Vec<&'a RawRow>>>,
    used: BTreeSet<(&'a str, i64)>,
    control: &'c mut Control<'p>,
    total: usize,
}

impl<'a, 'c, 'p> Reader<'a, 'c, 'p> {
    fn new(db: &'a Db, control: &'c mut Control<'p>) -> Result<Self,ValueError> {
        control.check_database(db, Phase::ReconstructSnapshot)?;
        let mut reader = Self { db, rows: BTreeMap::new(), groups: BTreeMap::new(), used: BTreeSet::new(), control, total: 0 };
        for table in &db.tables { for row in &table.rows {
            if row.rowid <= 0 || row.integer(0)? != row.rowid || reader.rows.insert((&table.name, row.rowid), sqlite_snapshot::artifact::FloatRow::new(row,number::columns(&table.name))?).is_some() { return Err(ValueError::new(ValueRefusalKind::InvalidValue,"PDF row identity must be positive and unique within its entity table")); }
            reader.total += 1;
            if reader.total % 256 == 0 { reader.control.checkpoint(Phase::ReconstructSnapshot, 0, reader.total)?; }
        } }
        Ok(reader)
    }
    fn take(&mut self, table: &'a str, key: i64, width: usize) -> Result<Row<'a>,ValueError> {
        let row = self.rows.get(&(table, key)).copied().ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue,format!("missing {table} entity {key}")))?;
        if row.values.len() != width || !self.used.insert((table, key)) { return Err(ValueError::new(ValueRefusalKind::InvalidValue,format!("{table} has an invalid row width, duplicate ownership or containment cycle"))); }
        if self.used.len() % 256 == 0 { self.control.checkpoint(Phase::ReconstructSnapshot, self.used.len(), self.total)?; }
        Ok(row)
    }
    fn children(&mut self, table: &'a str, parent_column: usize, ordinal_column: usize, parent: i64) -> Result<Vec<&'a RawRow>,ValueError> {
        self.children_with_role(table, parent_column, ordinal_column, parent, None)
    }
    fn children_with_role(&mut self, table: &'a str, parent_column: usize, ordinal_column: usize, parent: i64, role: Option<(usize, &'a str)>) -> Result<Vec<&'a RawRow>,ValueError> {
        let key = (table, parent_column, ordinal_column, role);
        if !self.groups.contains_key(&key) {
            let mut groups: BTreeMap<i64, Vec<&RawRow>> = BTreeMap::new();
            for (index, row) in self.db.table(table)?.rows.iter().enumerate() {
                if index % 256 == 0 { self.control.checkpoint(Phase::ReconstructSnapshot, self.used.len(), self.total)?; }
                if let Some((column, role)) = role { if row.text(column)? != role { continue; } }
                row.integer(ordinal_column)?;
                groups.entry(row.integer(parent_column)?).or_default().push(row);
            }
            for rows in groups.values_mut() {
                let mut comparisons = 0; let mut cancelled = None;
                rows.sort_by_key(|row| { comparisons += 1; if comparisons % 1024 == 0 && cancelled.is_none() { cancelled = self.control.checkpoint(Phase::ReconstructSnapshot, self.used.len(), self.total).err(); } row.integer(ordinal_column).unwrap_or(i64::MIN) });
                if let Some(error) = cancelled { return Err(error); }
                for (ordinal, row) in rows.iter().enumerate() { if row.integer(ordinal_column)? != ordinal as i64 { return Err(ValueError::new(ValueRefusalKind::InvalidValue,format!("{table} order must be contiguous and unique"))); } }
            }
            self.groups.insert(key, groups);
        }
        Ok(self.groups.get_mut(&key).unwrap().remove(&parent).unwrap_or_default())
    }
    fn copy_text(&mut self,value:&str)->Result<String,ValueError>{sqlite_snapshot::artifact::Reconstruction::new(self.control)?.text(value)}
    fn copy_blob(&mut self,value:&[u8])->Result<Vec<u8>,ValueError>{sqlite_snapshot::artifact::Reconstruction::new(self.control)?.blob(value)}
    fn text(&mut self,row:Row<'_>,column:usize)->Result<String,ValueError>{self.copy_text(row.text(column)?)}
    fn optional_text(&mut self,row:Row<'_>,column:usize)->Result<Option<String>,ValueError>{row.optional_text(column)?.map(|value|self.copy_text(value)).transpose()}
    fn blob(&mut self,row:Row<'_>,column:usize)->Result<Vec<u8>,ValueError>{self.copy_blob(row.blob(column)?)}
    fn has(&self, table: &str, key: i64) -> bool { self.rows.contains_key(&(table, key)) }
    fn finish(self) -> Result<(),ValueError> {
        if self.used.len() != self.total { return Err(ValueError::new(ValueRefusalKind::InvalidValue,"PDF database contains orphaned or mismatched semantic entities")); }
        self.control.checkpoint(Phase::ReconstructSnapshot, self.total, self.total)
    }
}

#[cfg(test)]
mod tests {
    use crate::standards::v1_7::subsets::base::io::sqlite::snapshot::*;
    use store::ArtifactSqliteSnapshot;
    use semio_framework_value::FromValue;
    fn fixture()->PdfSnapshot { PdfSnapshot::from_value(serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap()).unwrap() }
    fn domain_schema()->String{include_str!("🗄️.sql").into()}
    fn independent_semantic_extent(snapshot:&PdfSnapshot)->serde_json::Value{
        use std::{io::Write,process::{Command,Stdio}};let limits=sqlite_snapshot::SqliteDatabaseLimits::default();let database=snapshot.to_sqlite_database(&mut Control::new(&mut |_|true,limits)).unwrap();let bytes=sqlite_snapshot::export_sqlite_database(&database,limits,&mut |_|true).unwrap();let script=r#"import{Database}from'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));let rows=0,bytes=0;const counts={};for(const table of db.query("SELECT name FROM sqlite_schema WHERE type='table' ORDER BY name").all()){const fields=db.query('PRAGMA table_info("'+table.name+'")').all();const sum=fields.map(f=>'CASE typeof("'+f.name+'") WHEN \'integer\' THEN 8 WHEN \'real\' THEN 8 WHEN \'text\' THEN length(CAST("'+f.name+'" AS BLOB)) WHEN \'blob\' THEN length("'+f.name+'") ELSE 0 END').join('+');const n=db.query('SELECT COUNT(*) AS rows,COALESCE(SUM('+sum+'),0) AS bytes FROM "'+table.name+'"').get();rows+=n.rows;bytes+=n.bytes;if(n.rows)counts[table.name]=n.rows;}db.close();await Bun.write(Bun.stdout,JSON.stringify({rows,bytes,counts}));"#;let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));serde_json::from_slice(&output.stdout).unwrap()
    }


    #[test]
    fn sqlite_snapshot_pdf17_controlled_owned_value_projection_preserves_fields_and_cancels_inside_copy(){
        use pack::value::{ToValue,NativeEncodeControl};
        use semio_framework_value::native_encoding::NativeEncodeProgress;
        let plan:serde_json::Value=serde_json::from_str(include_str!("../../../🧬️schema/📸️snapshot/🌱️value/🧬️octets/🧫️fixtures/🛫️encoding.json")).unwrap();let maximum=plan["maximumBytes"].as_u64().unwrap()as usize;let mut snapshot=fixture();snapshot.schema=plan["text"].as_str().unwrap().repeat(plan["repeatCount"].as_u64().unwrap()as usize);snapshot.document_id=Some([vec![0,1,255],Vec::new()]);
        let mut accept=|_|true;let output=snapshot.to_value_controlled(&mut NativeEncodeControl::new(maximum,&mut accept)).unwrap();let pack::value::DslValue::Object(fields)=&output else{panic!("PDF output must preserve its declared fields")};assert_eq!(fields.len(),plan["fieldCount"].as_u64().unwrap()as usize);assert_eq!(output,snapshot.to_value());assert_eq!(PdfSnapshot::from_value(output).unwrap(),snapshot);
        let mut interior=false;let mut cancel=|event:NativeEncodeProgress|if event.total==snapshot.schema.len()&&event.completed>=plan["cancelAt"].as_u64().unwrap()as usize&&event.completed<event.total{interior=true;false}else{true};assert!(snapshot.to_value_controlled(&mut NativeEncodeControl::new(maximum,&mut cancel)).is_err());assert!(interior);let mut accept=|_|true;let mut control=NativeEncodeControl::new(1,&mut accept);assert!(snapshot.to_value_controlled(&mut control).is_err());assert_eq!(control.owned_bytes(),0);
        use std::{io::Write,process::{Command,Stdio}};let script="import{Database}from'bun:sqlite';const x=JSON.parse(await Bun.stdin.text());const d=new Database(':memory:');d.run('CREATE TABLE literal(id INTEGER PRIMARY KEY,text TEXT NOT NULL,left_id BLOB NOT NULL,right_id BLOB NOT NULL)');d.run('INSERT INTO literal VALUES(1,?,?,?)',x.plan.text.repeat(x.plan.repeatCount),Uint8Array.from(x.plan.documentId.left),Uint8Array.from(x.plan.documentId.right));const row=d.query('SELECT text,hex(left_id) AS left_id,hex(right_id) AS right_id FROM literal').get();await Bun.write(Bun.stdout,JSON.stringify(row));d.close();";let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(serde_json::json!({"plan":plan}).to_string().as_bytes()).unwrap();let result=child.wait_with_output().unwrap();assert!(result.status.success(),"{}",String::from_utf8_lossy(&result.stderr));let oracle:serde_json::Value=serde_json::from_slice(&result.stdout).unwrap();assert_eq!(oracle["text"].as_str().unwrap(),snapshot.schema);assert_eq!(oracle["left_id"],"0001FF");assert_eq!(oracle["right_id"],"");
    }

    #[test]
    fn sqlite_snapshot_pdf17_controlled_native_output_preserves_full_fixture_and_stops_inside_text(){
        let plan:serde_json::Value=serde_json::from_str(include_str!("../../../🧬️schema/📸️snapshot/🌱️value/🧬️octets/🧫️fixtures/🛫️encoding.json")).unwrap();let mut snapshot=fixture();snapshot.schema=plan["text"].as_str().unwrap().repeat(plan["repeatCount"].as_u64().unwrap()as usize);snapshot.document_id=Some([vec![0,1,255],Vec::new()]);let limits=sqlite_snapshot::SqliteDatabaseLimits::default();
        for encoding in [sqlite_snapshot::SnapshotEncoding::Binary,sqlite_snapshot::SnapshotEncoding::Text]{
            let payload=snapshot.encode_sqlite_snapshot_native(encoding,&mut Control::new(&mut |_|true,limits)).unwrap();assert_eq!(PdfSnapshot::decode_sqlite_snapshot_native(&payload,&mut Control::new(&mut |_|true,limits)).unwrap(),snapshot);
            let mut interior=false;let mut callback=|event:sqlite_snapshot::SqliteSnapshotProgress|if event.phase==Phase::EncodeNative&&event.total==snapshot.schema.len()&&event.completed>=plan["cancelAt"].as_u64().unwrap()as usize&&event.completed<event.total{interior=true;false}else{true};assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut Control::new(&mut callback,limits)).is_err());assert!(interior);
            let tiny=sqlite_snapshot::SqliteDatabaseLimits{max_value_bytes:128,..limits};assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut Control::new(&mut |_|true,tiny)).is_err());
        }
    }

    #[test]
    fn sqlite_snapshot_pdf17_controlled_native_owner_preserves_full_fixture_and_stops_inside_text(){
        use store::{ArtifactDsl,ArtifactPack};use semio_framework_os_kernel::io_schema::IoPayload;
        let mut snapshot=fixture();snapshot.schema="owned PDF 世界\0".repeat(16384);let limits=sqlite_snapshot::SqliteDatabaseLimits::default();
        for encoding in [sqlite_snapshot::SnapshotEncoding::Binary,sqlite_snapshot::SnapshotEncoding::Text]{
            let payload=match encoding{sqlite_snapshot::SnapshotEncoding::Binary=>IoPayload::Binary(snapshot.encode_pack()),sqlite_snapshot::SnapshotEncoding::Text=>IoPayload::Text(snapshot.print_dsl())};
            let decoded=PdfSnapshot::decode_sqlite_snapshot_native(&payload,&mut Control::new(&mut |_|true,limits)).unwrap();assert_eq!(decoded,snapshot);
            let mut reached=false;let result=PdfSnapshot::decode_sqlite_snapshot_native(&payload,&mut Control::new(&mut |event|{if event.phase==sqlite_snapshot::SqliteSnapshotPhase::DecodeNative&&event.completed>=65536&&event.completed<event.total{reached=true;false}else{true}},limits));assert!(result.is_err());assert!(reached,"cancel inside long native field");
            let small=sqlite_snapshot::SqliteDatabaseLimits{max_value_bytes:128,..limits};assert!(PdfSnapshot::decode_sqlite_snapshot_native(&payload,&mut Control::new(&mut |_|true,small)).is_err());assert!(PdfSnapshot::decode_sqlite_snapshot_native(&payload,&mut Control::new(&mut |_|false,limits)).is_err());
        }
        let full:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🛂️semantic/📖️full/🔣️.json")).unwrap();
        assert_eq!(snapshot.schema,full["schema"].as_str().unwrap().repeat(full["repeat"].as_u64().unwrap()as usize));
        let observed=independent_semantic_extent(&snapshot);assert_eq!(observed["rows"],full["rows"]);assert_eq!(observed["bytes"],full["bytes"]);assert_eq!(observed["counts"],full["counts"]);
        let exact=sqlite_snapshot::SqliteDatabaseLimits{max_rows:full["rows"].as_u64().unwrap()as usize,max_value_bytes:full["bytes"].as_u64().unwrap()as usize,..limits};
        for encoding in [sqlite_snapshot::SnapshotEncoding::Binary,sqlite_snapshot::SnapshotEncoding::Text]{
            let payload=snapshot.encode_sqlite_snapshot_native(encoding,&mut Control::new(&mut |_|true,exact)).unwrap();let decoded=PdfSnapshot::decode_sqlite_snapshot_native(&payload,&mut Control::new(&mut |_|true,exact)).unwrap();assert_eq!(decoded,snapshot);decoded.retire_sqlite_snapshot();
            let short=sqlite_snapshot::SqliteDatabaseLimits{max_rows:exact.max_rows-1,..exact};assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut Control::new(&mut |_|true,short)).is_err());assert!(PdfSnapshot::decode_sqlite_snapshot_native(&payload,&mut Control::new(&mut |_|true,short)).is_err());
            let short=sqlite_snapshot::SqliteDatabaseLimits{max_value_bytes:exact.max_value_bytes-1,..exact};assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut Control::new(&mut |_|true,short)).is_err());assert!(PdfSnapshot::decode_sqlite_snapshot_native(&payload,&mut Control::new(&mut |_|true,short)).is_err());
        }
        let plan:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🛂️semantic/🔣️.json")).unwrap();
        for case in plan["cases"].as_array().unwrap(){
            let snapshot=PdfSnapshot{schema:case["schema"].as_str().unwrap().repeat(case["repeat"].as_u64().unwrap_or(1)as usize),declared_version:case["declaredVersion"].as_str().unwrap().into(),..Default::default()};
            let rows=case["rows"].as_u64().unwrap()as usize;let bytes=case["valueBytes"].as_u64().unwrap()as usize;
            let exact=sqlite_snapshot::SqliteDatabaseLimits{max_rows:rows,max_value_bytes:bytes,..limits};
            let database=snapshot.to_sqlite_database(&mut Control::new(&mut |_|true,limits)).unwrap();
            let file=sqlite_snapshot::export_sqlite_database(&database,limits,&mut |_|true).unwrap();
            use std::{io::Write,process::{Command,Stdio}};
            let script=r#"import{Database}from'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));let rows=0,bytes=0;const counts={};for(const table of db.query("SELECT name FROM sqlite_schema WHERE type='table' ORDER BY name").all()){const fields=db.query('PRAGMA table_info("'+table.name+'")').all();const sum=fields.map(f=>'CASE typeof("'+f.name+'") WHEN \'integer\' THEN 8 WHEN \'real\' THEN 8 WHEN \'text\' THEN length(CAST("'+f.name+'" AS BLOB)) WHEN \'blob\' THEN length("'+f.name+'") ELSE 0 END').join('+');const n=db.query('SELECT COUNT(*) AS rows,COALESCE(SUM('+sum+'),0) AS bytes FROM "'+table.name+'"').get();rows+=n.rows;bytes+=n.bytes;if(n.rows)counts[table.name]=n.rows;}db.close();await Bun.write(Bun.stdout,JSON.stringify({rows,bytes,counts}));"#;
            let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&file).unwrap();let oracle=child.wait_with_output().unwrap();assert!(oracle.status.success(),"{}",String::from_utf8_lossy(&oracle.stderr));let observed:serde_json::Value=serde_json::from_slice(&oracle.stdout).unwrap();assert_eq!(observed["rows"],case["rows"]);assert_eq!(observed["bytes"],case["valueBytes"]);assert_eq!(observed["counts"],case["counts"]);
            for encoding in [sqlite_snapshot::SnapshotEncoding::Binary,sqlite_snapshot::SnapshotEncoding::Text]{
                let payload=snapshot.encode_sqlite_snapshot_native(encoding,&mut Control::new(&mut |_|true,exact)).unwrap();
                let decoded=PdfSnapshot::decode_sqlite_snapshot_native(&payload,&mut Control::new(&mut |_|true,exact)).unwrap();assert_eq!(decoded,snapshot);decoded.retire_sqlite_snapshot();
                let short=sqlite_snapshot::SqliteDatabaseLimits{max_value_bytes:bytes-1,..exact};assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut Control::new(&mut |_|true,short)).is_err());assert!(PdfSnapshot::decode_sqlite_snapshot_native(&payload,&mut Control::new(&mut |_|true,short)).is_err());
                let short=sqlite_snapshot::SqliteDatabaseLimits{max_rows:rows-1,..exact};assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut Control::new(&mut |_|true,short)).is_err());assert!(PdfSnapshot::decode_sqlite_snapshot_native(&payload,&mut Control::new(&mut |_|true,short)).is_err());
            }
            snapshot.retire_sqlite_snapshot();
        }

    }

    #[test]
    fn sqlite_snapshot_pdf17_direct_domain_rows_do_not_reserve_file_metadata(){
        let snapshot=fixture();let limits=sqlite_snapshot::SqliteDatabaseLimits::default();let database=snapshot.to_sqlite_database(&mut Control::new(&mut |_|true,limits)).unwrap();let rows=database.tables.iter().map(|table|table.rows.len()).sum::<usize>();let exact=sqlite_snapshot::SqliteDatabaseLimits{max_rows:rows,..limits};assert_eq!(snapshot.to_sqlite_database(&mut Control::new(&mut |_|true,exact)).unwrap(),database);assert_eq!(PdfSnapshot::from_sqlite_database(&database,&mut Control::new(&mut |_|true,exact)).unwrap(),snapshot);let short=sqlite_snapshot::SqliteDatabaseLimits{max_rows:rows-1,..limits};assert!(snapshot.to_sqlite_database(&mut Control::new(&mut |_|true,short)).is_err());assert!(PdfSnapshot::from_sqlite_database(&database,&mut Control::new(&mut |_|true,short)).is_err());
    }

    #[test]
    fn sqlite_snapshot_pdf17_deep_function_and_color_domains(){
        use std::{io::Write,process::{Command,Stdio}};let plan:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🌲️deep.json")).unwrap();let depth=plan["depth"].as_u64().unwrap() as usize;let mut function=PdfFunction::PostScript{domain:vec![0.0,1.0],range:vec![0.0,1.0],code:"{dup}".into()};let mut color=PdfColorSpace::DeviceRgb;for _ in 0..depth{function=PdfFunction::Array{functions:vec![function]};color=PdfColorSpace::Pattern{base:Some(Box::new(color))};}
        let limits=sqlite_snapshot::SqliteDatabaseLimits::default();let mut progress=|_|true;let mut control=Control::new(&mut progress,limits);let mut projection=Projection::new(&domain_schema(),&mut control).unwrap();let function_key=color::write_function(&mut projection,&function).unwrap();let color_key=color::write_color(&mut projection,&color).unwrap();let db=projection.finish().unwrap();let bytes=sqlite_snapshot::export_sqlite_database(&db,limits,&mut |_|true).unwrap();let script="import {Database} from 'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');await Bun.write(Bun.stdout,JSON.stringify(db.query('SELECT (SELECT COUNT(*) FROM pdf_function) AS functions,(SELECT COUNT(*) FROM pdf_color_space) AS colors').get()));db.close();";let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));let actual:serde_json::Value=serde_json::from_slice(&output.stdout).unwrap();for name in["functions","colors"]{assert_eq!(actual[name],plan["expectedValues"]);}
        let loaded=sqlite_snapshot::import_sqlite_database(&bytes,limits,&mut |_|true).unwrap();let mut reader=Reader::new(&loaded,&mut control).unwrap();let restored_function=color::read_function(&mut reader,function_key).unwrap();let restored_color=color::read_color(&mut reader,color_key).unwrap();reader.finish().unwrap();let mut function=&restored_function;let mut color=&restored_color;for _ in 0..depth{let PdfFunction::Array{functions}=function else{panic!("function array");};assert_eq!(functions.len(),1);function=&functions[0];let PdfColorSpace::Pattern{base:Some(base)}=color else{panic!("pattern");};color=base;}assert!(matches!(function,PdfFunction::PostScript{code,..}if code=="{dup}"));assert!(matches!(color,PdfColorSpace::DeviceRgb));
    }

    #[test]
    fn sqlite_snapshot_pdf17_deep_actions_outlines_and_form_fields(){
        use std::{io::Write,process::{Command,Stdio}};let plan:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🌲️deep.json")).unwrap();let depth=plan["depth"].as_u64().unwrap() as usize;let mut action=PdfAction{kind:PdfActionKind::Named{name:"leaf-action".into()},next:Vec::new()};let mut outline=PdfOutlineItem::to_page("leaf-outline",0);let mut field=PdfFormField{name:"leaf-field".into(),kind:PdfFormFieldKind::Container,flags:0,alternate_name:None,mapping_name:None,default_appearance:None,quadding:None,widgets:Vec::new(),children:Vec::new(),additional_actions:Vec::new(),extra:Vec::new()};for level in 0..depth{action=PdfAction{kind:PdfActionKind::Named{name:format!("action-{level}")},next:vec![action]};let mut parent=PdfOutlineItem::to_page(format!("outline-{level}"),0);parent.children=vec![outline];outline=parent;field=PdfFormField{name:format!("field-{level}"),children:vec![field],kind:PdfFormFieldKind::Container,flags:0,alternate_name:None,mapping_name:None,default_appearance:None,quadding:None,widgets:Vec::new(),additional_actions:Vec::new(),extra:Vec::new()};}
        let limits=sqlite_snapshot::SqliteDatabaseLimits::default();let snapshot=PdfSnapshot{open_action:Some(PdfOpenAction::Action{action}),outlines:vec![outline],acro_form:Some(PdfAcroForm{fields:vec![field],need_appearances:false,signature_flags:0,default_appearance:None,quadding:None,default_fonts:Vec::new(),extra:Vec::new()}),..Default::default()};let db=snapshot.to_sqlite_database(&mut Control::new(&mut |_|true,limits)).unwrap();let bytes=sqlite_snapshot::export_sqlite_database(&db,limits,&mut |_|true).unwrap();let script="import {Database} from 'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');await Bun.write(Bun.stdout,JSON.stringify(db.query('SELECT (SELECT COUNT(*) FROM pdf_action) AS actions,(SELECT COUNT(*) FROM pdf_outline) AS outlines,(SELECT COUNT(*) FROM pdf_form_field) AS fields').get()));db.close();";let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));let actual:serde_json::Value=serde_json::from_slice(&output.stdout).unwrap();for name in["actions","outlines","fields"]{assert_eq!(actual[name],plan["expectedValues"]);}
        let loaded=sqlite_snapshot::import_sqlite_database(&bytes,limits,&mut |_|true).unwrap();let restored=PdfSnapshot::from_sqlite_database(&loaded,&mut Control::new(&mut |_|true,limits)).unwrap();let PdfOpenAction::Action{action} = restored.open_action.as_ref().unwrap()else{panic!("action");};let mut action=action;let mut outline=&restored.outlines[0];let mut field=&restored.acro_form.as_ref().unwrap().fields[0];for _ in 0..depth{assert_eq!(action.next.len(),1);action=&action.next[0];assert_eq!(outline.children.len(),1);outline=&outline.children[0];assert_eq!(field.children.len(),1);field=&field.children[0];}assert!(matches!(&action.kind,PdfActionKind::Named{name}if name=="leaf-action"));assert_eq!(outline.title,"leaf-outline");assert_eq!(field.name,"leaf-field");
    }

    #[test]
    fn sqlite_snapshot_pdf17_iterative_cos_exceeds512_and_sqlite_walks_containment(){
        use std::{io::Write,process::{Command,Stdio}};let plan:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🌲️deep.json")).unwrap();let depth=plan["depth"].as_u64().unwrap() as usize;let mut object=PdfObject::Name(plan["leafName"].as_str().unwrap().into());for _ in 0..depth{object=PdfObject::Array(vec![object]);}
        let limits=sqlite_snapshot::SqliteDatabaseLimits::default();let mut callback=|_|true;let mut control=Control::new(&mut callback,limits);let mut projection=Projection::new(&domain_schema(),&mut control).unwrap();let key=cos::write_object(&mut projection,&object).unwrap();let db=projection.finish().unwrap();let bytes=sqlite_snapshot::export_sqlite_database(&db,limits,&mut |_|true).unwrap();
        let script="import {Database} from 'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const row=db.query('WITH RECURSIVE values_tree(id,depth) AS (SELECT id,0 FROM pdf_cos_value WHERE id NOT IN (SELECT value_id FROM pdf_cos_array_element) UNION ALL SELECT e.value_id,t.depth+1 FROM pdf_cos_array_element e JOIN values_tree t ON e.array_id=t.id) SELECT COUNT(*) AS count,MAX(depth) AS depth FROM values_tree').get();await Bun.write(Bun.stdout,JSON.stringify(row));db.close();";let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));let actual:serde_json::Value=serde_json::from_slice(&output.stdout).unwrap();assert_eq!(actual["count"],plan["expectedValues"]);assert_eq!(actual["depth"],plan["depth"]);
        let loaded=sqlite_snapshot::import_sqlite_database(&bytes,limits,&mut |_|true).unwrap();let mut reader=Reader::new(&loaded,&mut control).unwrap();let restored=cos::read_object(&mut reader,key).unwrap();reader.finish().unwrap();let mut node=&restored;for _ in 0..depth{let PdfObject::Array(values)=node else{panic!("array");};assert_eq!(values.len(),1);node=&values[0];}let PdfObject::Name(name)=node else{panic!("name");};assert_eq!(name,plan["leafName"].as_str().unwrap());let mut calls=0;let mut cancel=|_|{calls+=1;calls<8};let mut cancelled=Control::new(&mut cancel,limits);assert!(Reader::new(&loaded,&mut cancelled).and_then(|mut reader|cos::read_object(&mut reader,key)).is_err());
    }
    fn register_declaration(){crate::register_sqlite_test_declaration();}

    async fn erased_owned_snapshot(snapshot:&PdfSnapshot,encoding:sqlite_snapshot::SnapshotEncoding)->PdfSnapshot{
        use {semio_framework_artifact_reference::ArtifactDialect,semio_framework_os_kernel::io::IoPayload,semio_framework_os_kernel::io::io_mechanism::io_route,semio_framework_os_kernel::io::io_mechanism::io_run};
        register_declaration();let native=ArtifactDialect{artifact_kind:"s.stdio.pdf".into(),standard:"1.7".into(),subset:"*".into()};let sqlite:ArtifactDialect=semio_framework_os_kernel::io_schema::SQLITE_SNAPSHOT.into();let payload=match encoding{sqlite_snapshot::SnapshotEncoding::Binary=>IoPayload::Binary(<PdfSnapshot as store::ArtifactPack>::encode_pack(snapshot)),sqlite_snapshot::SnapshotEncoding::Text=>IoPayload::Text(<PdfSnapshot as store::ArtifactDsl>::print_dsl(snapshot))};let export=io_route(&native,&sqlite,1).await.unwrap().value;let file=io_run(&export,payload).await.unwrap().value;let import=io_route(&sqlite,&native,1).await.unwrap().value;let restored=io_run(&import,file).await.unwrap().value;match restored{IoPayload::Binary(bytes)=><PdfSnapshot as store::ArtifactPack>::decode_pack(&bytes).unwrap(),IoPayload::Text(text)=><PdfSnapshot as store::ArtifactDsl>::parse_dsl(&text).unwrap()}
    }
    async fn erased_snapshot_roundtrip(encoding:sqlite_snapshot::SnapshotEncoding){let snapshot=fixture();assert_eq!(erased_owned_snapshot(&snapshot,encoding).await,snapshot);}
    #[semio_framework_async_macros::async_test]
    async fn sqlite_snapshot_pdf_native_full_entity_row_admission() {
        use store::ArtifactSqliteSnapshot;
        use std::{io::Write,process::{Command,Stdio}};
        let plan:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🛫️row-admission.json")).unwrap();
        let mut snapshot=PdfSnapshot::default();
        snapshot.objects.push(PdfIndirectObject{id:ObjRef{num:1,gen:0},value:PdfObject::Array((0..plan["arrayLength"].as_u64().unwrap()).map(|_|PdfObject::Null).collect())});
        let limits=sqlite_snapshot::SqliteDatabaseLimits::default();
        let database=snapshot.to_sqlite_database(&mut Control::new(&mut |_|true,limits)).unwrap();
        let bytes=sqlite_snapshot::export_sqlite_database(&database,limits,&mut |_|true).unwrap();
        let script="import{Database}from'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));const counts={};let total=0;for(const{name}of db.query(\"SELECT name FROM sqlite_schema WHERE type='table'\").all()){const n=db.query('SELECT COUNT(*) AS n FROM \"'+name+'\"').get().n;if(n)counts[name]=n;total+=n;}await Bun.write(Bun.stdout,JSON.stringify({counts,total}));db.close();";
        let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));let actual:serde_json::Value=serde_json::from_slice(&output.stdout).unwrap();assert_eq!(actual["counts"],plan["counts"]);assert_eq!(actual["total"],plan["totalRows"]);
        for encoding in[sqlite_snapshot::SnapshotEncoding::Binary,sqlite_snapshot::SnapshotEncoding::Text]{
            let tight=sqlite_snapshot::SqliteDatabaseLimits{max_rows:plan["refusedRows"].as_u64().unwrap()as usize,..limits};
            let mut metadata=false;let mut observe=|event:sqlite_snapshot::SqliteSnapshotProgress|{metadata|=event.phase==Phase::EncodeNative&&event.total==32;true};
            let Err(error)=snapshot.encode_sqlite_snapshot_native(encoding,&mut Control::new(&mut observe,tight))else{panic!("all authored COS entities and relationships require row admission for {encoding:?}")};assert!(error.message.contains("row"),"{error}");assert!(!metadata,"row refusal precedes native metadata ownership");
            let mut interior=false;let mut metadata=false;let mut cancel=|event:sqlite_snapshot::SqliteSnapshotProgress|{metadata|=event.phase==Phase::EncodeNative&&event.total==32;if event.phase==Phase::EncodeNative&&event.total==0&&event.completed==plan["cancelAt"].as_u64().unwrap()as usize{interior=true;false}else{true}};assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut Control::new(&mut cancel,limits)).is_err());assert!(interior);assert!(!metadata,"forecast cancellation precedes native metadata ownership");
            let exact=sqlite_snapshot::SqliteDatabaseLimits{max_rows:plan["totalRows"].as_u64().unwrap()as usize,..limits};assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut Control::new(&mut |_|true,exact)).is_ok());
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn sqlite_snapshot_pdf17_actual_erased_binary_preserves_all_owned_fields(){erased_snapshot_roundtrip(sqlite_snapshot::SnapshotEncoding::Binary).await;}
    #[semio_framework_async_macros::async_test]
    async fn sqlite_snapshot_pdf17_actual_erased_text_preserves_all_owned_fields(){erased_snapshot_roundtrip(sqlite_snapshot::SnapshotEncoding::Text).await;}
    #[semio_framework_async_macros::async_test]
    async fn sqlite_snapshot_pdf17_actual_erased_routes_preserve_ieee_and_u64_indices(){
        let cases:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔢ieee.json")).unwrap();
        for encoding in [sqlite_snapshot::SnapshotEncoding::Binary,sqlite_snapshot::SnapshotEncoding::Text]{for case in cases["binary64"].as_array().unwrap(){
            let bits=u64::from_str_radix(case["bits"].as_str().unwrap(),16).unwrap();let number=f64::from_bits(bits);let mut snapshot=fixture();
            snapshot.schema="complete-owned-erased-domain".into();snapshot.pages[0].media_box=[number;4];snapshot.pages[0].crop_box=Some([number;4]);snapshot.pages[0].user_unit=Some(number);snapshot.pages[0].duration=Some(number);
            snapshot.pages[0].annotations[0].markup=Some(PdfMarkupAnnotation{popup:Some(u64::MAX),in_reply_to:Some(1u64<<32),opacity:Some(number),..Default::default()});snapshot.pages[0].annotations[15].kind=PdfAnnotationKind::Popup{parent:Some(u64::MAX),open:true};
            let restored=erased_owned_snapshot(&snapshot,encoding).await;assert_eq!(restored.schema,snapshot.schema);assert_eq!(restored.pages[0].media_box.map(f64::to_bits),[bits;4]);assert_eq!(restored.pages[0].crop_box.unwrap().map(f64::to_bits),[bits;4]);assert_eq!(restored.pages[0].user_unit.unwrap().to_bits(),bits);assert_eq!(restored.pages[0].duration.unwrap().to_bits(),bits);
            let markup=restored.pages[0].annotations[0].markup.as_ref().unwrap();assert_eq!(markup.popup,Some(u64::MAX));assert_eq!(markup.in_reply_to,Some(1u64<<32));assert_eq!(markup.opacity.unwrap().to_bits(),bits);assert!(matches!(restored.pages[0].annotations[15].kind,PdfAnnotationKind::Popup{parent:Some(u64::MAX),open:true}));
        }}
    }
    #[semio_framework_async_macros::async_test]
    async fn sqlite_snapshot_pdf_profiles_use_exact_declared_native_provider_and_validator(){
        use {semio_framework_artifact_reference::ArtifactDialect,semio_framework_os_kernel::io::io_mechanism::io_route,semio_framework_os_kernel::io::io_mechanism::io_export_sqlite_snapshot,semio_framework_os_kernel::io::io_mechanism::io_import_sqlite_snapshot};use semio_framework_plugin::ArtifactBuilder;
        register_declaration();let sqlite:ArtifactDialect=semio_framework_os_kernel::io_schema::SQLITE_SNAPSHOT.into();for(standard,subset)in[("1.4","*"),("1.4","a"),("1.4","x"),("1.7","*"),("1.7","a"),("1.7","x"),("1.7","e"),("1.7","ua"),("1.7","vt"),("1.7","h")]{let dialect=ArtifactDialect{artifact_kind:"s.stdio.pdf".into(),standard:standard.into(),subset:subset.into()};assert_eq!(io_route(&dialect,&sqlite,1).await.unwrap().value.hops.len(),1);assert_eq!(io_route(&sqlite,&dialect,1).await.unwrap().value.hops.len(),1);}
        let dialect=ArtifactDialect{artifact_kind:"s.stdio.pdf".into(),standard:"1.7".into(),subset:"a".into()};let mut snapshot=crate::standards::v1_7::subsets::a::io::PdfABuilderConstruction::new("sRGB IEC61966-2.1").add_page(PdfPage::new(100.0,100.0)).build().unwrap();snapshot.schema="full-owned-PDF-A-schema".into();let native=store::ArtifactPack::encode_pack(&snapshot);assert_eq!(<PdfSnapshot as store::ArtifactPack>::decode_pack(&native).unwrap(),snapshot);let limits=sqlite_snapshot::SqliteDatabaseLimits::default();let mut phases=Vec::new();let file=io_export_sqlite_snapshot(&dialect,&snapshot,sqlite_snapshot::SnapshotEncoding::Binary,limits,&mut |event|{phases.push(event.phase);true}).await.unwrap().value;assert_eq!(io_import_sqlite_snapshot::<PdfSnapshot>(&dialect,&file,limits,&mut |event|{phases.push(event.phase);true}).await.unwrap().value,snapshot);assert!(!phases.iter().any(|phase|matches!(phase,Phase::EncodeNative|Phase::DecodeNative)));
        let invalid=PdfSnapshot{open_action:Some(PdfOpenAction::Action{action:PdfAction{kind:PdfActionKind::JavaScript{script:"app.alert(1)".into()},next:Vec::new()}}),..PdfSnapshot::default()};assert!(io_export_sqlite_snapshot(&dialect,&invalid,sqlite_snapshot::SnapshotEncoding::Binary,limits,&mut |_|true).await.is_err());let base=ArtifactDialect{artifact_kind:"s.stdio.pdf".into(),standard:"1.7".into(),subset:"*".into()};let file=io_export_sqlite_snapshot(&base,&invalid,sqlite_snapshot::SnapshotEncoding::Binary,limits,&mut |_|true).await.unwrap().value;assert!(io_import_sqlite_snapshot::<PdfSnapshot>(&dialect,&file,limits,&mut |_|true).await.is_err());let mut forged=sqlite_snapshot::import_sqlite_database(&file,limits,&mut |_|true).unwrap();semio_framework_os_kernel::io::io_mechanism::take_sqlite_snapshot_metadata(&mut forged).unwrap();semio_framework_os_kernel::io::io_mechanism::attach_sqlite_snapshot_metadata(&mut forged,&dialect,sqlite_snapshot::SnapshotEncoding::Binary,&mut sqlite_snapshot::SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();let forged=sqlite_snapshot::export_sqlite_database(&forged,limits,&mut |_|true).unwrap();assert!(io_import_sqlite_snapshot::<PdfSnapshot>(&dialect,&forged,limits,&mut |_|true).await.is_err());
    }
    #[semio_framework_async_macros::async_test]
    async fn sqlite_snapshot_pdf17_owned_io_preserves_full_snapshot(){
        use {semio_framework_artifact_reference::ArtifactDialect,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId,semio_framework_os_kernel::io::io_mechanism::io_export_sqlite_snapshot,semio_framework_os_kernel::io::io_mechanism::io_import_sqlite_snapshot};
        register_declaration();let native=Dialect{artifact_kind:"s.stdio.pdf",standard:StandardId("1.7"),subset:SubsetId("*")};let dialect:ArtifactDialect=native.into();let snapshot=fixture();let limits=sqlite_snapshot::SqliteDatabaseLimits::default();let mut phases=Vec::new();
        let file=io_export_sqlite_snapshot(&dialect,&snapshot,sqlite_snapshot::SnapshotEncoding::Binary,limits,&mut |event|{phases.push(event.phase);true}).await.unwrap().value;let loaded=io_import_sqlite_snapshot::<PdfSnapshot>(&dialect,&file,limits,&mut |_|true).await.unwrap().value;assert_eq!(loaded,snapshot);assert!(!phases.iter().any(|phase|matches!(phase,Phase::DecodeNative|Phase::EncodeNative)));
        let other=ArtifactDialect{artifact_kind:"s.stdio.pdf".into(),standard:"1.4".into(),subset:"*".into()};assert!(io_import_sqlite_snapshot::<PdfSnapshot>(&other,&file,limits,&mut |_|true).await.is_err());
    }
    #[semio_framework_async_macros::async_test]
    async fn sqlite_snapshot_pdf17_owned_ieee_fields_preserve_presence_and_exact_bits(){
        use {semio_framework_artifact_reference::ArtifactDialect,semio_framework_os_kernel::io::io_mechanism::io_export_sqlite_snapshot,semio_framework_os_kernel::io::io_mechanism::io_import_sqlite_snapshot};use std::{io::Write,process::{Command,Stdio}};register_declaration();let cases:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔢ieee.json")).unwrap();let dialect=ArtifactDialect{artifact_kind:"s.stdio.pdf".into(),standard:"1.7".into(),subset:"*".into()};let limits=sqlite_snapshot::SqliteDatabaseLimits::default();
        for case in cases["binary64"].as_array().unwrap(){
            let bits=u64::from_str_radix(case["bits"].as_str().unwrap(),16).unwrap();let value=f64::from_bits(bits);let mut page=PdfPage::new(value,10.0);page.crop_box=Some([value,0.0,1.0,2.0]);page.user_unit=Some(value);page.content=vec![PdfOp::SetLineWidth{width:value},PdfOp::SetDash{array:vec![value],phase:value}];let snapshot=PdfSnapshot{schema:"every-owned-IEEE-field".into(),pages:vec![page],ext_g_states:vec![PdfExtGState{id:"state".into(),line_width:Some(value),..Default::default()}],..Default::default()};let mut phases=Vec::new();let file=io_export_sqlite_snapshot(&dialect,&snapshot,sqlite_snapshot::SnapshotEncoding::Binary,limits,&mut |event|{phases.push(event.phase);true}).await.unwrap().value;let restored=io_import_sqlite_snapshot::<PdfSnapshot>(&dialect,&file,limits,&mut |event|{phases.push(event.phase);true}).await.unwrap().value;assert_eq!(restored.pages[0].media_box[2].to_bits(),bits);assert_eq!(restored.pages[0].crop_box.unwrap()[0].to_bits(),bits);assert_eq!(restored.pages[0].user_unit.unwrap().to_bits(),bits);assert_eq!(restored.ext_g_states[0].line_width.unwrap().to_bits(),bits);assert!(restored.ext_g_states[0].font.is_none());let PdfOp::SetLineWidth{width}=restored.pages[0].content[0] else{panic!("line width");};assert_eq!(width.to_bits(),bits);let PdfOp::SetDash{array,phase}=&restored.pages[0].content[1] else{panic!("dash");};assert_eq!(array[0].to_bits(),bits);assert_eq!(phase.to_bits(),bits);assert!(!phases.iter().any(|phase|matches!(phase,Phase::EncodeNative|Phase::DecodeNative)));
            let script="import {Database} from 'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');await Bun.write(Bun.stdout,JSON.stringify(db.query('SELECT CAST(user_unit_bits AS TEXT) AS bits,user_unit_class AS class,user_unit IS NULL AS nullQuery FROM pdf_page').get()));db.close();";let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&file).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));let independent:serde_json::Value=serde_json::from_slice(&output.stdout).unwrap();assert_eq!(independent["bits"].as_str().unwrap(),(bits as i64).to_string());assert_eq!(independent["class"],case["class"]);assert_eq!(independent["nullQuery"].as_i64().unwrap(),i64::from(case["class"]=="nan"));
        }
    }
    #[test]
    fn sqlite_snapshot_pdf17_retained_text_dates_are_direct_owned_rows(){
        let cases:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪪️retained-text/🔣️.json")).unwrap();let limits=sqlite_snapshot::SqliteDatabaseLimits::default();
        for case in cases["cases"].as_array().unwrap(){
            let date=PdfDate::from_value(serde_json::from_value(case["ownedDate"].clone()).unwrap()).unwrap();
            let snapshot=PdfSnapshot {objects:vec![PdfIndirectObject {id:ObjRef {num:7,gen:2},value:PdfObject::Array(vec![PdfObject::Text(case["text"].as_str().unwrap().into()),PdfObject::Date(date)])}],..Default::default()};
            let database=snapshot.to_sqlite_database(&mut Control::new(&mut |_|true,limits)).unwrap();
            assert_eq!(database.table("pdf_date").unwrap().rows.len(),1);
            assert_eq!(PdfSnapshot::from_sqlite_database(&database,&mut Control::new(&mut |_|true,limits)).unwrap(),snapshot);
            let file=sqlite_snapshot::export_sqlite_database(&database,limits,&mut |_|true).unwrap();
            let restored=sqlite_snapshot::import_sqlite_database(&file,limits,&mut |_|true).unwrap();
            assert_eq!(PdfSnapshot::from_sqlite_database(&restored,&mut Control::new(&mut |_|true,limits)).unwrap(),snapshot);
        }
    }
    #[test]
    fn sqlite_snapshot_pdf17_independent_sql_queries_and_annotation_edits(){
        use std::{io::Write,process::{Command,Stdio}};let mut snapshot=fixture();let limits=sqlite_snapshot::SqliteDatabaseLimits::default();let db=snapshot.to_sqlite_database(&mut Control::new(&mut |_|true,limits)).unwrap();let bytes=sqlite_snapshot::export_sqlite_database(&db,limits,&mut |_|true).unwrap();
        let script="import {Database} from 'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const rows=db.query('SELECT a.contents,k.kind FROM pdf_page_annotation p JOIN pdf_annotation a ON a.id=p.annotation_id JOIN pdf_annotation_detail k ON k.id=a.detail_id ORDER BY p.ordinal').all();if(rows.length!==27||rows[0].kind!=='text'||rows[26].kind!=='unknown')throw Error('annotation query');db.query(\"UPDATE pdf_annotation SET contents='independent edit' WHERE id=(SELECT annotation_id FROM pdf_page_annotation WHERE ordinal=0)\").run();await Bun.write(Bun.stdout,db.serialize());db.close();";
        let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));let edited=sqlite_snapshot::import_sqlite_database(&output.stdout,limits,&mut |_|true).unwrap();snapshot.pages[0].annotations[0].contents=Some("independent edit".into());assert_eq!(PdfSnapshot::from_sqlite_database(&edited,&mut Control::new(&mut |_|true,limits)).unwrap(),snapshot);
    }
    #[test]
    fn sqlite_snapshot_pdf17_preserves_complete_document_and_sql_edits(){
        let snapshot=fixture();let limits=sqlite_snapshot::SqliteDatabaseLimits::default();let mut callback=|_|true;let mut control=Control::new(&mut callback,limits);
        let db=snapshot.to_sqlite_database(&mut control).unwrap();assert_eq!(db.table("pdf_annotation").unwrap().rows.len(),27);assert_eq!(db.table("pdf_form_field").unwrap().rows.len(),6);
        let file=sqlite_snapshot::export_sqlite_database(&db,limits,&mut |_|true).unwrap();let mut loaded=sqlite_snapshot::import_sqlite_database(&file,limits,&mut |_|true).unwrap();assert_eq!(PdfSnapshot::from_sqlite_database(&loaded,&mut control).unwrap(),snapshot);
        loaded.table_mut("pdf_annotation").unwrap().rows[0].values[5]=V::Text("edited annotation".into());let mut expected=snapshot.clone();expected.pages[0].annotations[0].contents=Some("edited annotation".into());assert_eq!(PdfSnapshot::from_sqlite_database(&loaded,&mut control).unwrap(),expected);
        let empty=PdfSnapshot::default();let db=empty.to_sqlite_database(&mut control).unwrap();assert_eq!(PdfSnapshot::from_sqlite_database(&db,&mut control).unwrap(),empty);
    }
    #[test]
    fn sqlite_snapshot_pdf17_rejects_bad_relations_schema_and_respects_limits(){
        let snapshot=fixture();let limits=sqlite_snapshot::SqliteDatabaseLimits::default();let mut callback=|_|true;let mut control=Control::new(&mut callback,limits);let db=snapshot.to_sqlite_database(&mut control).unwrap();
        let mut bad=db.clone();bad.table_mut("pdf_page_annotation").unwrap().rows[0].values[3]=V::Integer(i64::MAX);assert!(PdfSnapshot::from_sqlite_database(&bad,&mut control).is_err());
        let mut bad=db.clone();bad.table_mut("pdf_document_page").unwrap().rows[0].values[2]=V::Integer(1);assert!(PdfSnapshot::from_sqlite_database(&bad,&mut control).is_err());
        let mut bad=db.clone();bad.table_mut("pdf_document").unwrap().sql=bad.table("pdf_document").unwrap().sql.replace("NOT NULL","\"NOT NULL\"");assert!(PdfSnapshot::from_sqlite_database(&bad,&mut control).is_err());
        assert!(snapshot.to_sqlite_database(&mut Control::new(&mut |_|false,limits)).is_err());assert!(PdfSnapshot::from_sqlite_database(&db,&mut Control::new(&mut |_|false,limits)).is_err());
        assert!(snapshot.to_sqlite_database(&mut Control::new(&mut |_|true,sqlite_snapshot::SqliteDatabaseLimits{max_rows:5,..limits})).is_err());
        assert!(PdfSnapshot::from_sqlite_database(&db,&mut Control::new(&mut |_|true,sqlite_snapshot::SqliteDatabaseLimits{max_value_bytes:16,..limits})).is_err());
        let mut calls=0;assert!(snapshot.to_sqlite_database(&mut Control::new(&mut |_|{calls+=1;calls<4},limits)).is_err());
    }
    #[test]
    fn sqlite_snapshot_pdf17_cos_retains_exact_values_filters_and_colors(){
        let snapshot=fixture();let limits=sqlite_snapshot::SqliteDatabaseLimits::default();let mut progress=|_|true;let mut control=Control::new(&mut progress,limits);
        let schema=domain_schema();let mut projection=Projection::new(&schema,&mut control).unwrap();let mut objects=Vec::new();let mut colors=Vec::new();
        for object in &snapshot.objects {objects.push(cos::write_object(&mut projection,&object.value).unwrap());}
        for color in &snapshot.color_spaces {colors.push(color::write_color(&mut projection,&color.color_space).unwrap());}
        let db=projection.finish().unwrap();assert_eq!(db.table("pdf_stream_filter").unwrap().rows.len(),10);
        let file=sqlite_snapshot::export_sqlite_database(&db,limits,&mut |_|true).unwrap();let loaded=sqlite_snapshot::import_sqlite_database(&file,limits,&mut |_|true).unwrap();
        let mut reader=Reader::new(&loaded,&mut control).unwrap();
        for(object,key)in snapshot.objects.iter().zip(objects){assert_eq!(cos::read_object(&mut reader,key).unwrap(),object.value);}
        for(color,key)in snapshot.color_spaces.iter().zip(colors){assert_eq!(color::read_color(&mut reader,key).unwrap(),color.color_space);}
        reader.finish().unwrap();
    }
    #[test]
    fn sqlite_snapshot_pdf17_preserves_every_content_operator_and_font_field(){
        let snapshot=fixture();let limits=sqlite_snapshot::SqliteDatabaseLimits::default();let mut progress=|_|true;let mut control=Control::new(&mut progress,limits);let schema=domain_schema();
        let mut projection=Projection::new(&schema,&mut control).unwrap();let content_key=content::write_ops(&mut projection,&snapshot.pages[0].content).unwrap();let mut fonts=Vec::new();for value in &snapshot.fonts{fonts.push(font::write_font(&mut projection,value).unwrap());}
        let db=projection.finish().unwrap();let file=sqlite_snapshot::export_sqlite_database(&db,limits,&mut |_|true).unwrap();let loaded=sqlite_snapshot::import_sqlite_database(&file,limits,&mut |_|true).unwrap();let mut reader=Reader::new(&loaded,&mut control).unwrap();
        assert_eq!(content::read_ops(&mut reader,content_key).unwrap(),snapshot.pages[0].content);
        for(value,key)in snapshot.fonts.iter().zip(fonts){assert_eq!(font::read_font(&mut reader,key).unwrap(),*value);}
        reader.finish().unwrap();
    }
    #[test]
    fn sqlite_snapshot_pdf17_preserves_every_graphics_resource_field(){
        let snapshot=fixture();let limits=sqlite_snapshot::SqliteDatabaseLimits::default();let mut progress=|_|true;let mut control=Control::new(&mut progress,limits);let schema=domain_schema();let mut projection=Projection::new(&schema,&mut control).unwrap();
        let images=snapshot.images.iter().map(|value|resource::write_image(&mut projection,value).unwrap()).collect::<Vec<_>>();let forms=snapshot.forms.iter().map(|value|resource::write_form(&mut projection,value).unwrap()).collect::<Vec<_>>();let states=snapshot.ext_g_states.iter().map(|value|resource::write_state(&mut projection,value).unwrap()).collect::<Vec<_>>();let shadings=snapshot.shadings.iter().map(|value|resource::write_shading(&mut projection,value).unwrap()).collect::<Vec<_>>();let patterns=snapshot.patterns.iter().map(|value|resource::write_pattern(&mut projection,value).unwrap()).collect::<Vec<_>>();
        let db=projection.finish().unwrap();let file=sqlite_snapshot::export_sqlite_database(&db,limits,&mut |_|true).unwrap();let loaded=sqlite_snapshot::import_sqlite_database(&file,limits,&mut |_|true).unwrap();let mut reader=Reader::new(&loaded,&mut control).unwrap();
        for(value,key)in snapshot.images.iter().zip(images){assert_eq!(resource::read_image(&mut reader,key).unwrap(),*value);}for(value,key)in snapshot.forms.iter().zip(forms){assert_eq!(resource::read_form(&mut reader,key).unwrap(),*value);}for(value,key)in snapshot.ext_g_states.iter().zip(states){assert_eq!(resource::read_state(&mut reader,key).unwrap(),*value);}for(value,key)in snapshot.shadings.iter().zip(shadings){assert_eq!(resource::read_shading(&mut reader,key).unwrap(),*value);}for(value,key)in snapshot.patterns.iter().zip(patterns){assert_eq!(resource::read_pattern(&mut reader,key).unwrap(),*value);}
        reader.finish().unwrap();
    }
    #[test]
    fn sqlite_snapshot_pdf17_cos_rejects_ambiguous_payloads_and_graph_cycles(){
        let limits=sqlite_snapshot::SqliteDatabaseLimits::default();let mut progress=|_|true;let mut control=Control::new(&mut progress,limits);let schema=domain_schema();
        let object=PdfObject::Array(vec![PdfObject::Null]);let mut projection=Projection::new(&schema,&mut control).unwrap();let key=cos::write_object(&mut projection,&object).unwrap();let db=projection.finish().unwrap();
        let mut bad=db.clone();bad.table_mut("pdf_cos_value").unwrap().rows[0].values[3]=V::Integer(12);assert!(cos::read_object(&mut Reader::new(&bad,&mut control).unwrap(),key).is_err());
        let mut bad=db.clone();bad.table_mut("pdf_cos_array_element").unwrap().rows[0].values[3]=V::Integer(key);assert!(cos::read_object(&mut Reader::new(&bad,&mut control).unwrap(),key).is_err());
        assert!(Reader::new(&db,&mut Control::new(&mut |_|false,limits)).is_err());
    }    #[semio_framework_async_macros::async_test]
    async fn sqlite_snapshot_pdf17_ua_initial_retains_genuine_typed_raw_profile_and_both_public_forms() {
        use semio_framework_plugin::ArtifactBuilder;
        use {semio_framework_artifact_reference::ArtifactDialect,semio_framework_os_kernel::io::io_mechanism::io_export_sqlite_snapshot,semio_framework_os_kernel::io::io_mechanism::io_import_sqlite_snapshot};
        fn agree(snapshot: &PdfSnapshot) {
            let reference = |num| ObjRef { num, gen: 0 };
            assert_eq!(snapshot.trailer, vec![PdfDictEntry { key: "Root".into(), value: PdfObject::Ref(reference(1)) }]);
            assert_eq!(snapshot.objects.iter().map(|object| object.id.num).collect::<Vec<_>>(), vec![1,2,3,4,5]);
            let object = |num| &snapshot.objects.iter().find(|object| object.id == reference(num)).expect("literal declared object").value;
            assert_eq!(object(1).dict_get("Type"), Some(&PdfObject::Name("Catalog".into())));
            for (key,num) in [("Pages",5),("MarkInfo",2),("StructTreeRoot",3),("ViewerPreferences",4)] { assert_eq!(object(1).dict_get(key), Some(&PdfObject::Ref(reference(num)))); }
            assert_eq!(object(1).dict_get("Lang"), Some(&PdfObject::Str(b"und".to_vec())));
            assert_eq!(object(5).dict_get("Type"), Some(&PdfObject::Name("Pages".into())));
            assert_eq!(object(5).dict_get("Kids"), Some(&PdfObject::Array(Vec::new())));
            assert_eq!(object(5).dict_get("Count"), Some(&PdfObject::Int(0)));
            assert_eq!(object(2).dict_get("Marked"), Some(&PdfObject::Bool(true)));
            assert_eq!(object(3).dict_get("Type"), Some(&PdfObject::Name("StructTreeRoot".into())));
            assert_eq!(object(4).dict_get("DisplayDocTitle"), Some(&PdfObject::Bool(true)));
            assert_eq!(snapshot.language.as_deref(), Some("und"));
            assert_eq!(snapshot.mark_info, Some(PdfMarkInfo { marked:true,user_properties:false,suspects:false }));
            assert!(snapshot.viewer_preferences.as_ref().is_some_and(|preferences| preferences.display_doc_title));
            assert_eq!(snapshot.catalog_extra, vec![PdfDictEntry { key:"StructTreeRoot".into(),value:PdfObject::Ref(reference(3)) }]);
            for (key,code) in [("MarkInfo","stdio.pdf.ua.missing-markinfo-marked"),("StructTreeRoot","stdio.pdf.ua.missing-structtreeroot")] {
                let mut refused=snapshot.clone();
                let catalog=refused.objects.iter_mut().find(|object| object.id==reference(1)).unwrap();
                let PdfObject::Dict(entries)=&mut catalog.value else { panic!("literal Catalog dictionary") };
                entries.retain(|entry| entry.key!=key);
                assert!(crate::standards::v1_7::subsets::ua::io::check_ua_conformance(&refused).iter().any(|diagnostic| diagnostic.code.0==code && matches!(diagnostic.severity,semio_framework_diagnostic::Severity::Error|semio_framework_diagnostic::Severity::Fatal)));
                <PdfSnapshot as store::ArtifactSqliteSnapshot>::retire_sqlite_snapshot(refused);
            }
            assert!(crate::standards::v1_7::subsets::ua::io::check_ua_conformance(snapshot).iter().all(|diagnostic| !matches!(diagnostic.severity,semio_framework_diagnostic::Severity::Error|semio_framework_diagnostic::Severity::Fatal)));
        }
        let contract=semio_repo_test_host::parse_json(include_str!("🧫️fixtures/♿️ua-initial/🔣️.json")).unwrap();
        let owner=crate::standards::v1_7::subsets::ua::io::PdfUaBuilderConstruction::new("und").build().unwrap();
        agree(&owner);
        let native=crate::standards::v1_7::subsets::base::io::encode_pdf(&owner).unwrap();
        let independent=semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::ua::project_conformance(&native).unwrap();
        assert_eq!(&independent,contract.get("conformance").unwrap());
        let decoded=crate::standards::v1_7::subsets::base::io::decode_pdf(&native).unwrap();
        agree(&decoded);
        register_declaration();
        let dialect=ArtifactDialect{artifact_kind:"s.stdio.pdf".into(),standard:"1.7".into(),subset:"ua".into()};
        let limits=sqlite_snapshot::SqliteDatabaseLimits::default();
        for encoding in [sqlite_snapshot::SnapshotEncoding::Binary,sqlite_snapshot::SnapshotEncoding::Text] {
            let file=io_export_sqlite_snapshot(&dialect,&owner,encoding,limits,&mut |_|true).await.unwrap().value;
            let restored=io_import_sqlite_snapshot::<PdfSnapshot>(&dialect,&file,limits,&mut |_|true).await.unwrap().value;
            assert_eq!(restored,owner);
            agree(&restored);
            let native=crate::standards::v1_7::subsets::base::io::encode_pdf(&restored).unwrap();
            assert_eq!(semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::ua::project_conformance(&native).unwrap(),independent);
            <PdfSnapshot as store::ArtifactSqliteSnapshot>::retire_sqlite_snapshot(restored);
        }
        <PdfSnapshot as store::ArtifactSqliteSnapshot>::retire_sqlite_snapshot(decoded);
        <PdfSnapshot as store::ArtifactSqliteSnapshot>::retire_sqlite_snapshot(owner);
    }

    #[semio_framework_async_macros::async_test]
    async fn sqlite_snapshot_pdf17_x_initial_retains_real_profile_typed_raw_and_both_public_forms() {
        use semio_framework_plugin::ArtifactBuilder;
        use {semio_framework_artifact_reference::ArtifactDialect,semio_framework_os_kernel::io::io_mechanism::io_export_sqlite_snapshot,semio_framework_os_kernel::io::io_mechanism::io_import_sqlite_snapshot};
        const ICC: &[u8] = include_bytes!("🧫️fixtures/🌈️x-vt-initial/🌈️sRGB2014.icc");
        fn agree(owner: &PdfSnapshot) {
            let reference=|num|ObjRef{num,gen:0};
            let object=|num|&owner.objects.iter().find(|object|object.id==reference(num)).expect("declared object").value;
            assert_eq!(owner.trailer,vec![PdfDictEntry{key:"Root".into(),value:PdfObject::Ref(reference(1))}]);
            assert_eq!(owner.objects.iter().map(|object|object.id.num).collect::<Vec<_>>(),vec![1,2,3,4]);
            assert_eq!(object(1).dict_get("Type"),Some(&PdfObject::Name("Catalog".into())));
            assert_eq!(object(1).dict_get("Pages"),Some(&PdfObject::Ref(reference(4))));
            assert_eq!(object(1).dict_get("OutputIntents"),Some(&PdfObject::Array(vec![PdfObject::Ref(reference(2))])));
            assert_eq!(object(2).dict_get("Type"),Some(&PdfObject::Name("OutputIntent".into())));
            assert_eq!(object(2).dict_get("S"),Some(&PdfObject::Name("GTS_PDFX".into())));
            assert_eq!(object(2).dict_get("OutputConditionIdentifier"),Some(&PdfObject::Str(b"sRGB2014".to_vec())));
            assert_eq!(object(2).dict_get("DestOutputProfile"),Some(&PdfObject::Ref(reference(3))));
            let PdfObject::Stream{dict,data,filters}=object(3) else {panic!("real profile stream")};
            assert_eq!(dict, &vec![PdfDictEntry{key:"N".into(),value:PdfObject::Int(3)}]);
            assert_eq!(data.as_slice(),ICC);assert!(filters.is_empty());
            assert_eq!(object(4).dict_get("Type"),Some(&PdfObject::Name("Pages".into())));
            assert_eq!(object(4).dict_get("Kids"),Some(&PdfObject::Array(Vec::new())));
            assert_eq!(object(4).dict_get("Count"),Some(&PdfObject::Int(0)));assert!(owner.pages.is_empty());
            assert_eq!(owner.output_intents,vec![PdfOutputIntent{subtype:"GTS_PDFX".into(),condition_identifier:"sRGB2014".into(),condition:None,registry_name:None,info:None,profile:Some(ICC.to_vec())}]);
            assert!(crate::standards::v1_7::subsets::x::io::check_x_conformance(owner).iter().all(|diagnostic|!matches!(diagnostic.severity,semio_framework_diagnostic::Severity::Error|semio_framework_diagnostic::Severity::Fatal)));
        }
        let contract=semio_repo_test_host::parse_json(include_str!("🧫️fixtures/🌈️x-vt-initial/🖨️x.json")).unwrap();
        let owner=crate::standards::v1_7::subsets::x::io::PdfXBuilderConstruction::empty().build().unwrap();agree(&owner);
        let native=crate::standards::v1_7::subsets::base::io::encode_pdf(&owner).unwrap();
        let project=|bytes:&[u8]|semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::x::project_conformance(bytes).unwrap();
        let profiles=|bytes:&[u8]|semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::x::output_profiles(bytes).unwrap();
        assert_eq!(&project(&native),contract.get("conformance").unwrap());assert_eq!(profiles(&native),vec![(3,ICC.to_vec())]);
        let decoded=crate::standards::v1_7::subsets::base::io::decode_pdf(&native).unwrap();agree(&decoded);assert_eq!(decoded,owner);
        register_declaration();let dialect=ArtifactDialect{artifact_kind:"s.stdio.pdf".into(),standard:"1.7".into(),subset:"x".into()};let limits=sqlite_snapshot::SqliteDatabaseLimits::default();
        for encoding in [sqlite_snapshot::SnapshotEncoding::Binary,sqlite_snapshot::SnapshotEncoding::Text] {
            let file=io_export_sqlite_snapshot(&dialect,&owner,encoding,limits,&mut |_|true).await.unwrap().value;
            let restored=io_import_sqlite_snapshot::<PdfSnapshot>(&dialect,&file,limits,&mut |_|true).await.unwrap().value;assert_eq!(restored,owner);agree(&restored);
            let bytes=crate::standards::v1_7::subsets::base::io::encode_pdf(&restored).unwrap();assert_eq!(&project(&bytes),contract.get("conformance").unwrap());assert_eq!(profiles(&bytes),vec![(3,ICC.to_vec())]);
            <PdfSnapshot as store::ArtifactSqliteSnapshot>::retire_sqlite_snapshot(restored);
        }
        <PdfSnapshot as store::ArtifactSqliteSnapshot>::retire_sqlite_snapshot(decoded);<PdfSnapshot as store::ArtifactSqliteSnapshot>::retire_sqlite_snapshot(owner);
        eprintln!("[DEBUG] pdf17 X genuine profile typed/raw native and both public forms");
    }

    #[semio_framework_async_macros::async_test]
    async fn sqlite_snapshot_pdf17_vt_initial_retains_real_profile_typed_raw_and_both_public_forms() {
        use semio_framework_plugin::ArtifactBuilder;
        use {semio_framework_artifact_reference::ArtifactDialect,semio_framework_os_kernel::io::io_mechanism::io_export_sqlite_snapshot,semio_framework_os_kernel::io::io_mechanism::io_import_sqlite_snapshot};
        const ICC: &[u8] = include_bytes!("🧫️fixtures/🌈️x-vt-initial/🌈️sRGB2014.icc");
        fn agree(owner: &PdfSnapshot) {
            let reference=|num|ObjRef{num,gen:0};
            let object=|num|&owner.objects.iter().find(|object|object.id==reference(num)).expect("declared object").value;
            assert_eq!(owner.trailer,vec![PdfDictEntry{key:"Root".into(),value:PdfObject::Ref(reference(1))}]);
            assert_eq!(owner.objects.iter().map(|object|object.id.num).collect::<Vec<_>>(),vec![1,2,3,4,5,10,11]);
            assert_eq!(object(1).dict_get("Type"),Some(&PdfObject::Name("Catalog".into())));
            assert_eq!(object(1).dict_get("Pages"),Some(&PdfObject::Ref(reference(4))));
            assert_eq!(object(1).dict_get("OutputIntents"),Some(&PdfObject::Array(vec![PdfObject::Ref(reference(2))])));
            assert_eq!(object(2).dict_get("Type"),Some(&PdfObject::Name("OutputIntent".into())));
            assert_eq!(object(2).dict_get("S"),Some(&PdfObject::Name("GTS_PDFX".into())));
            assert_eq!(object(2).dict_get("OutputConditionIdentifier"),Some(&PdfObject::Str(b"sRGB2014".to_vec())));
            assert_eq!(object(2).dict_get("DestOutputProfile"),Some(&PdfObject::Ref(reference(3))));
            let PdfObject::Stream{dict,data,filters}=object(3) else {panic!("real profile stream")};
            assert_eq!(dict, &vec![PdfDictEntry{key:"N".into(),value:PdfObject::Int(3)}]);
            assert_eq!(data.as_slice(),ICC);assert!(filters.is_empty());
            assert_eq!(object(4).dict_get("Type"),Some(&PdfObject::Name("Pages".into())));
            assert_eq!(object(4).dict_get("Kids"),Some(&PdfObject::Array(vec![PdfObject::Ref(reference(5))])));
            assert_eq!(object(4).dict_get("Count"),Some(&PdfObject::Int(1)));assert_eq!(owner.pages.len(),1);
            assert_eq!(object(1).dict_get("DPartRoot"),Some(&PdfObject::Ref(reference(10))));
            assert_eq!(object(10).dict_get("Type"),Some(&PdfObject::Name("DPartRoot".into())));
            assert_eq!(object(10).dict_get("DPartRootNode"),Some(&PdfObject::Ref(reference(11))));assert!(object(10).dict_get("DParts").is_none());
            assert_eq!(object(11).dict_get("Type"),Some(&PdfObject::Name("DPart".into())));
            assert_eq!(object(11).dict_get("Parent"),Some(&PdfObject::Ref(reference(10))));assert_eq!(object(11).dict_get("Start"),Some(&PdfObject::Ref(reference(5))));assert!(object(11).dict_get("DParts").is_none());
            assert_eq!(object(11).dict_get("DPM"),Some(&PdfObject::Dict(Vec::new())));
            assert_eq!(object(5).dict_get("Type"),Some(&PdfObject::Name("Page".into())));assert_eq!(object(5).dict_get("Parent"),Some(&PdfObject::Ref(reference(4))));
            let page=PdfPage::new(612.0,792.0);let mut expected=page;expected.trim_box=Some([0.0,0.0,612.0,792.0]);assert_eq!(owner.pages,vec![expected]);
            assert_eq!(owner.output_intents,vec![PdfOutputIntent{subtype:"GTS_PDFX".into(),condition_identifier:"sRGB2014".into(),condition:None,registry_name:None,info:None,profile:Some(ICC.to_vec())}]);
            assert!(crate::standards::v1_7::subsets::vt::io::check_vt_conformance(owner).iter().all(|diagnostic|!matches!(diagnostic.severity,semio_framework_diagnostic::Severity::Error|semio_framework_diagnostic::Severity::Fatal)));
        }
        let contract=semio_repo_test_host::parse_json(include_str!("🧫️fixtures/🌈️x-vt-initial/🧾️vt.json")).unwrap();
        let owner=crate::standards::v1_7::subsets::vt::io::PdfVtBuilderConstruction::empty().build().unwrap();agree(&owner);
        let native=crate::standards::v1_7::subsets::base::io::encode_pdf(&owner).unwrap();
        let project=|bytes:&[u8]|semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::vt::project_conformance(bytes).unwrap();
        let profiles=|bytes:&[u8]|semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::x::output_profiles(bytes).unwrap();
        assert_eq!(&project(&native),contract.get("conformance").unwrap());assert_eq!(profiles(&native),vec![(3,ICC.to_vec())]);
        let decoded=crate::standards::v1_7::subsets::base::io::decode_pdf(&native).unwrap();agree(&decoded);assert_eq!(decoded,owner);
        register_declaration();let dialect=ArtifactDialect{artifact_kind:"s.stdio.pdf".into(),standard:"1.7".into(),subset:"vt".into()};let limits=sqlite_snapshot::SqliteDatabaseLimits::default();
        for encoding in [sqlite_snapshot::SnapshotEncoding::Binary,sqlite_snapshot::SnapshotEncoding::Text] {
            let file=io_export_sqlite_snapshot(&dialect,&owner,encoding,limits,&mut |_|true).await.unwrap().value;
            let restored=io_import_sqlite_snapshot::<PdfSnapshot>(&dialect,&file,limits,&mut |_|true).await.unwrap().value;assert_eq!(restored,owner);agree(&restored);
            let bytes=crate::standards::v1_7::subsets::base::io::encode_pdf(&restored).unwrap();assert_eq!(&project(&bytes),contract.get("conformance").unwrap());assert_eq!(profiles(&bytes),vec![(3,ICC.to_vec())]);
            <PdfSnapshot as store::ArtifactSqliteSnapshot>::retire_sqlite_snapshot(restored);
        }
        <PdfSnapshot as store::ArtifactSqliteSnapshot>::retire_sqlite_snapshot(decoded);<PdfSnapshot as store::ArtifactSqliteSnapshot>::retire_sqlite_snapshot(owner);
        eprintln!("[DEBUG] pdf17 VT genuine profile typed/raw native and both public forms");
    }

    #[test]
    fn sqlite_snapshot_pdf17_vt_initial_graph_refuses_each_malformed_terminal_relation() {
        let r=|num|ObjRef{num,gen:0};
        let dictionary=|entries:Vec<(&str,PdfObject)>|PdfObject::Dict(entries.into_iter().map(|(key,value)|PdfDictEntry{key:key.into(),value}).collect());
        let mut page=PdfPage::new(612.0,792.0);page.trim_box=Some(page.media_box);
        let profile=include_bytes!("🧫️fixtures/🌈️x-vt-initial/🌈️sRGB2014.icc").to_vec();
        let owner=PdfSnapshot{
            pages:vec![page],output_intents:vec![PdfOutputIntent{subtype:"GTS_PDFX".into(),condition_identifier:"sRGB2014".into(),condition:None,registry_name:None,info:None,profile:Some(profile.clone())}],
            catalog_extra:vec![PdfDictEntry{key:"DPartRoot".into(),value:PdfObject::Ref(r(10))}],
            trailer:vec![PdfDictEntry{key:"Root".into(),value:PdfObject::Ref(r(1))}],
            objects:vec![
                PdfIndirectObject{id:r(1),value:dictionary(vec![("Type",PdfObject::name("Catalog")),("Pages",PdfObject::Ref(r(4))),("OutputIntents",PdfObject::Array(vec![PdfObject::Ref(r(2))])),("DPartRoot",PdfObject::Ref(r(10)))])},
                PdfIndirectObject{id:r(2),value:dictionary(vec![("Type",PdfObject::name("OutputIntent")),("S",PdfObject::name("GTS_PDFX")),("OutputConditionIdentifier",PdfObject::Str(b"sRGB2014".to_vec())),("DestOutputProfile",PdfObject::Ref(r(3)))])},
                PdfIndirectObject{id:r(3),value:PdfObject::Stream{dict:vec![PdfDictEntry{key:"N".into(),value:PdfObject::Int(3)}],data:profile,filters:Vec::new()}},
                PdfIndirectObject{id:r(4),value:dictionary(vec![("Type",PdfObject::name("Pages")),("Kids",PdfObject::Array(vec![PdfObject::Ref(r(5))])),("Count",PdfObject::Int(1))])},
                PdfIndirectObject{id:r(5),value:dictionary(vec![("Type",PdfObject::name("Page")),("Parent",PdfObject::Ref(r(4))),("MediaBox",PdfObject::numbers(&[0.0,0.0,612.0,792.0])),("TrimBox",PdfObject::numbers(&[0.0,0.0,612.0,792.0]))])},
                PdfIndirectObject{id:r(10),value:dictionary(vec![("Type",PdfObject::name("DPartRoot")),("DPartRootNode",PdfObject::Ref(r(11)))])},
                PdfIndirectObject{id:r(11),value:dictionary(vec![("Type",PdfObject::name("DPart")),("Parent",PdfObject::Ref(r(10))),("Start",PdfObject::Ref(r(5))),("DPM",dictionary(Vec::new()))])},
            ],..PdfSnapshot::default()
        };
        let check=|snapshot:&PdfSnapshot|crate::standards::v1_7::subsets::vt::io::check_vt_conformance(snapshot);
        assert!(check(&owner).iter().all(|d|!matches!(d.severity,semio_framework_diagnostic::Severity::Error|semio_framework_diagnostic::Severity::Fatal)));
        let contract:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🌈️x-vt-initial/🧾️vt-negatives.json")).unwrap();
        for case in contract["cases"].as_array().unwrap(){
            let name=case.as_str().unwrap();let mut malformed=owner.clone();
            let replace=|owner:&mut PdfSnapshot,num:u32,key:&str,value:Option<PdfObject>|{let object=owner.objects.iter_mut().find(|o|o.id.num==num).unwrap();let PdfObject::Dict(entries)=&mut object.value else{panic!("declared dictionary")};entries.retain(|entry|entry.key!=key);if let Some(value)=value{entries.push(PdfDictEntry{key:key.into(),value});}};
            match name {
                "root-node-missing"=>replace(&mut malformed,10,"DPartRootNode",None),
                "root-node-direct"=>replace(&mut malformed,10,"DPartRootNode",Some(dictionary(Vec::new()))),
                "root-type-wrong"=>replace(&mut malformed,10,"Type",Some(PdfObject::name("DPart"))),
                "leaf-parent-missing"=>replace(&mut malformed,11,"Parent",None),
                "leaf-parent-wrong"=>replace(&mut malformed,11,"Parent",Some(PdfObject::Ref(r(4)))),
                "leaf-start-missing"=>replace(&mut malformed,11,"Start",None),
                "leaf-start-dangling"=>replace(&mut malformed,11,"Start",Some(PdfObject::Ref(r(99)))),
                "leaf-start-outside-page-tree"=>{replace(&mut malformed,11,"Start",Some(PdfObject::Ref(r(6))));malformed.objects.push(PdfIndirectObject{id:r(6),value:dictionary(vec![("Type",PdfObject::name("Page"))])});},
                "child-cycle"=>replace(&mut malformed,11,"DParts",Some(PdfObject::Array(vec![PdfObject::Ref(r(11))]))),
                "duplicate-object-id"=>malformed.objects.push(malformed.objects.last().unwrap().clone()),
                other=>panic!("unknown neutral refusal {other}")
            }
            assert!(check(&malformed).iter().any(|d|d.code.0=="stdio.pdf.vt.invalid-dpart-graph"&&d.severity==semio_framework_diagnostic::Severity::Error),"genuine VT relation must refuse {name}");
        }
        eprintln!("[DEBUG] VT ten independently authored terminal graph refusals");
    }

}
