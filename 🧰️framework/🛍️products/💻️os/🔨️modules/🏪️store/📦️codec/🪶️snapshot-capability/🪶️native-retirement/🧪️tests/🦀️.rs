use semio_framework_value::{ValueError,ValueRefusalKind};
use super::*;
use crate::sqlite_snapshot::{SqliteDatabase, SqliteDatabaseLimits, SqliteRow, SqliteValue, SqliteSnapshotControl, SqliteSnapshotPhase, SnapshotEncoding, artifact::NativeEncodingBound};

struct RetainedSnapshot { value: i64, retired: bool }
std::thread_local! { static RETIREMENTS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) }; }

impl Drop for RetainedSnapshot {
    fn drop(&mut self) { assert!(self.retired, "retained snapshot requires its declared owner retirement"); }
}

impl ArtifactDsl for RetainedSnapshot {
    const EXTENSION: &'static str = "retained";
    fn parse_dsl(text: &str) -> Result<Self, TextError> { Ok(Self { value: text.parse().map_err(|error: std::num::ParseIntError| TextError { kind: semio_framework_value::ValueRefusalKind::InvalidValue, message: error.to_string(), span: Default::default(), expected: None })?, retired: false }) }
    fn print_dsl(&self) -> String { self.value.to_string() }
}

impl ArtifactPack for RetainedSnapshot {
    fn encode_pack_with(&self, _: &PackEncodeOptions) -> Result<Vec<u8>, PackError> { if self.value == -2 { return Err(PackError::from(ValueError::new(ValueRefusalKind::InvalidValue, "owner encoder refusal"))); } Ok(self.value.to_le_bytes().to_vec()) }
    fn decode_pack_with(bytes: &[u8], _: &PackDecodeOptions) -> Result<Self, PackError> { Ok(Self { value: i64::from_le_bytes(bytes.try_into().map_err(|_| PackError::from(ValueError::new(ValueRefusalKind::InvalidValue, "expected integer word")))?), retired: false }) }
}

fn value_database(value: i64) -> SqliteDatabase {
    let mut database = SqliteDatabase::from_schema(RetainedSnapshot::SQLITE_SCHEMA).unwrap();
    database.table_mut("retained_value").unwrap().rows.push(SqliteRow { rowid: 1, values: vec![SqliteValue::Integer(1), SqliteValue::Integer(value)] });
    database
}

impl ArtifactSqliteSnapshot for RetainedSnapshot {
    const SQLITE_SCHEMA: &'static str = include_str!("../🧬️schema/🗄️.sql");
    fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<crate::io_schema::IoPayload,ValueError>{
        use semio_framework_value::native_encoding::{NativeEncodeControl,NativeEncodeProgress};
        let limits=control.limits();let mut callback=|event:NativeEncodeProgress|control.checkpoint(SqliteSnapshotPhase::EncodeNative,event.completed,event.total).is_ok();let mut native=NativeEncodeControl::new(limits.max_value_bytes,&mut callback);native.begin_stage(1)?;let output=match encoding{SnapshotEncoding::Binary=>{if self.value == -2{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"owner encoder refusal"))}crate::io_schema::IoPayload::Binary(native.copy_bytes(&self.value.to_le_bytes())?)},SnapshotEncoding::Text=>{native.charge(20)?;crate::io_schema::IoPayload::Text(self.value.to_string())}};native.step()?;Ok(output)
    }
    fn decode_sqlite_snapshot_native(payload:&crate::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{
        control.checkpoint(SqliteSnapshotPhase::DecodeNative,0,0)?;
        control.check_value_bytes(std::mem::size_of::<Self>())?;
        let length=match payload{crate::io_schema::IoPayload::Binary(bytes)=>bytes.len(),crate::io_schema::IoPayload::Text(text)=>text.len()};
        if length>control.limits().max_file_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"retained scalar exceeds file byte limit"));}
        let value=match payload{
            crate::io_schema::IoPayload::Binary(bytes)=>i64::from_le_bytes(bytes.as_slice().try_into().map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"expected exact integer word"))?),
            crate::io_schema::IoPayload::Text(text)if text.len()<=20=>text.parse::<i64>().map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"expected bounded integer text"))?,
            _=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"expected bounded integer text")),
        };
        Ok(Self{value,retired:false})
    }
    fn retire_sqlite_snapshot(mut self) { self.retired = true; RETIREMENTS.with(|count| count.set(count.get() + 1)); }
    fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase,ValueError> { control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, 1)?; Ok(value_database(self.value)) }
    fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self,ValueError> { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 0, 1)?; Ok(Self { value: database.table("retained_value")?.single_row()?.integer(1)?, retired: false }) }
    fn preflight_sqlite_snapshot_encoding(&self, _: SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<(),ValueError> { let mut bound = NativeEncodingBound::new(control)?; bound.add(32)?; bound.finish() }
    fn validate_sqlite_snapshot_subset(&self, dialect: &semio_framework_artifact_reference::ArtifactDialect, _: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> crate::io_schema::IoResult<()> { control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, 1).map_err(crate::io_schema::IoError::from_value_error)?; if dialect.subset == "reject" { return Err(crate::io_schema::IoError::from_value_error(ValueError::new(ValueRefusalKind::InvalidValue,"declared semantic owner refusal"))); } Ok(crate::io_schema::IoOutcome::clean(())) }
}

#[test]
fn sqlite_snapshot_native_retirement_covers_success_cancellation_and_refusal() {
    use std::{io::Write, process::{Command, Stdio}};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    
    let script = "import {Database} from 'bun:sqlite';const x=JSON.parse(await Bun.stdin.text());const db=Database.deserialize(Buffer.from(x.sqlite,'base64'));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok')throw Error('integrity');if(db.query('SELECT value FROM retained_value WHERE id=1').get().value!==7)throw Error('owner value');await Bun.write(Bun.stdout,'ok');db.close();";
    let sqlite = crate::sqlite_snapshot::export_sqlite_database(&value_database(7), SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let mut child = Command::new("bun").args(["-e", script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(serde_json::json!({"fixture":fixture,"sqlite":protocol::bytes::encode_base64(&sqlite)}).to_string().as_bytes()).unwrap();
    let result = child.wait_with_output().unwrap();
    assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
    let codec = RetainedSnapshot::sqlite_codec();
    for case in fixture["cases"].as_array().unwrap() {
        RETIREMENTS.with(|count| count.set(0));
        let value = case["value"].as_i64().unwrap();
        let operation = case["operation"].as_str().unwrap();
        let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "fixture.retained".into(), standard: "1".into(), subset: case["subset"].as_str().unwrap().into() };
        let mut callback = |event: crate::sqlite_snapshot::SqliteSnapshotProgress| case["cancelPhase"].as_str().is_none_or(|phase| format!("{:?}", event.phase) != phase);
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut control = SqliteSnapshotControl::new(&mut callback, SqliteDatabaseLimits::default());
            if operation.starts_with("export") {
                let payload = if operation.ends_with("binary") { crate::io_schema::IoPayload::Binary(value.to_le_bytes().to_vec()) } else { crate::io_schema::IoPayload::Text(value.to_string()) };
                (codec.export)("fixture.retained/v1", &dialect, &payload, &mut control).map(|_| ())
            } else {
                let encoding = if operation.ends_with("binary") { SnapshotEncoding::Binary } else { SnapshotEncoding::Text };
                (codec.import)("fixture.retained/v1", &dialect, value_database(value), encoding, &mut control).map(|_| ())
            }
        }));
        assert!(result.is_ok(), "{} skipped owner retirement", case["id"]);
        assert_eq!(result.unwrap().is_ok(), case["succeeds"].as_bool().unwrap(), "{}", case["id"]);
        RETIREMENTS.with(|count| assert_eq!(count.get() as u64, case["retirements"].as_u64().unwrap(), "{}", case["id"]));
    }
}
