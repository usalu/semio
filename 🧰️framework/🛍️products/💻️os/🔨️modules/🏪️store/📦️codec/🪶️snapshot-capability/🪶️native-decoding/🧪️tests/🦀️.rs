use super::*;
use crate::sqlite_snapshot::{SqliteDatabase, SqliteDatabaseLimits, SqliteRow, SqliteValue, SqliteSnapshotControl, SqliteSnapshotPhase};
use crate::os_dsl::schema::{FieldSpec, FieldValue, RecordLayout, RecordSpec, RecordValue, Shape};

struct DecodedBuffers { buffers: Vec<Vec<u8>> }
std::thread_local! { static COMPLETED_DECODERS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) }; }

fn buffer_spec() -> RecordSpec {
    RecordSpec::new(Some("buffers"), RecordLayout::Lines, vec![FieldSpec::new(1, "buffers", Shape::List(Box::new(Shape::Bytes64)))])
}

impl ArtifactDsl for DecodedBuffers {
    const EXTENSION: &'static str = "buffers";
    fn parse_dsl(_: &str) -> Result<Self, TextError> { Err(TextError { message: "binary admission fixture".into(), span: Default::default(), expected: None }) }
    fn print_dsl(&self) -> String { String::new() }
}

impl ArtifactPack for DecodedBuffers {
    fn encode_pack_with(&self, options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        let mut record = RecordValue::default();
        record.fields.insert(1, FieldValue::List(self.buffers.iter().cloned().map(FieldValue::Bytes64).collect()));
        pack_rt::encode_document(&buffer_spec(), &record, options)
    }
    fn decode_pack_with(bytes: &[u8], options: &PackDecodeOptions) -> Result<Self, PackError> {
        let (mut record, _) = pack_rt::decode_document(bytes, &buffer_spec(), options)?;
        let Some(FieldValue::List(fields)) = record.fields.remove(&1) else { return Err(PackError::Schema("expected buffers".into())); };
        let buffers = fields.into_iter().map(|field| match field { FieldValue::Bytes64(bytes) => Ok(bytes), _ => Err(PackError::Schema("expected intrinsic octets".into())) }).collect::<Result<Vec<_>, _>>()?;
        COMPLETED_DECODERS.with(|count| count.set(count.get() + 1));
        Ok(Self { buffers })
    }
}

impl ArtifactSqliteSnapshot for DecodedBuffers {
    const SQLITE_SCHEMA: &'static str = include_str!("../🧬️schema/🗄️.sql");
    fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, String> {
        let count = self.buffers.iter().try_fold(0usize, |count, bytes| count.checked_add(bytes.len()).ok_or("octet count overflow"))?;
        control.check_value_bytes(count)?;
        control.check_rows(self.buffers.len())?;
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, self.buffers.len())?;
        let mut database = SqliteDatabase::from_schema(Self::SQLITE_SCHEMA).map_err(|error| error.to_string())?;
        for (index, bytes) in self.buffers.iter().enumerate() {
            let id = i64::try_from(index + 1).map_err(|error| error.to_string())?;
            database.table_mut("decoded_buffer")?.rows.push(SqliteRow { rowid: id, values: vec![SqliteValue::Integer(id), SqliteValue::Blob(bytes.clone())] });
        }
        Ok(database)
    }
    fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, String> {
        control.check_database(database, SqliteSnapshotPhase::ReconstructSnapshot)?;
        Ok(Self { buffers: database.table("decoded_buffer")?.rows.iter().map(|row| row.blob(1).map(<[u8]>::to_vec)).collect::<Result<_, _>>()? })
    }
}

#[test]
fn sqlite_snapshot_native_decoding_admits_compressed_aggregate_before_projection() {
    use std::{io::Write, process::{Command, Stdio}};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let schema: serde_json::Value = serde_json::from_str(include_str!("../🧬️schema/🔣️.json")).unwrap();
    let dialect = crate::io_schema::ArtifactDialect { artifact_kind: "fixture.buffers".into(), standard: "1".into(), subset: "*".into() };
    for case in fixture["cases"].as_array().unwrap() {
        let snapshot = DecodedBuffers { buffers: case["bufferBytes"].as_array().unwrap().iter().map(|value| vec![0; value.as_u64().unwrap() as usize]).chain(std::iter::repeat_with(Vec::new).take(case["emptyBuffers"].as_u64().unwrap() as usize)).collect() };
        let mut options = PackEncodeOptions::default();
        options.chunk_threshold = 1;
        options.chunk_size = 16384;
        let bytes = snapshot.encode_pack_with(&options).unwrap();
        let maximum = case["maximumBytes"].as_u64().unwrap() as usize;
        assert!(bytes.len() < maximum, "fixture must exercise decoded expansion");
        let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
        let sqlite = crate::sqlite_snapshot::export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
        let script = "import {Database} from 'bun:sqlite';import Ajv from 'ajv/dist/2020.js';const input=JSON.parse(await Bun.stdin.text());if(!new Ajv({strict:true}).validate(input.schema,input.fixture))throw Error('fixture');const db=Database.deserialize(Buffer.from(input.sqlite,'base64'));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok')throw Error('integrity');await Bun.write(Bun.stdout,String(db.query('SELECT SUM(length(octets)) AS n FROM decoded_buffer').get().n));db.close();";
        let mut child = Command::new("bun").args(["-e", script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
        let input = serde_json::json!({"schema": schema, "fixture": fixture, "sqlite": protocol::bytes::encode_base64(&sqlite)});
        child.stdin.take().unwrap().write_all(input.to_string().as_bytes()).unwrap();
        let result = child.wait_with_output().unwrap();
        assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
        assert_eq!(String::from_utf8(result.stdout).unwrap(), snapshot.buffers.iter().map(Vec::len).sum::<usize>().to_string());
        let mut limits = SqliteDatabaseLimits::default();
        limits.max_value_bytes = maximum;
        COMPLETED_DECODERS.with(|count| count.set(0));
        let result = (DecodedBuffers::sqlite_codec().export)("fixture.buffers/v1", &dialect, &crate::io_schema::IoPayload::Binary(bytes.clone()), &mut SqliteSnapshotControl::new(&mut |_| true, limits));
        let decoded = case["decoded"].as_bool().unwrap();
        assert_eq!(result.is_ok(), decoded, "{}: {result:?}", case["id"]);
        COMPLETED_DECODERS.with(|count| assert_eq!(count.get(), usize::from(decoded), "{}", case["id"]));
        COMPLETED_DECODERS.with(|count| count.set(0));
        let result = (DecodedBuffers::sqlite_codec().export)("fixture.buffers/v1", &dialect, &crate::io_schema::IoPayload::Binary(bytes), &mut SqliteSnapshotControl::new(&mut |event| event.phase != SqliteSnapshotPhase::DecodeNative, limits));
        assert!(result.is_err());
        COMPLETED_DECODERS.with(|count| assert_eq!(count.get(), 0));
    }
}

#[test]
fn sqlite_snapshot_native_decoding_admits_inline_empty_collection_storage() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let case = fixture["cases"].as_array().unwrap().iter().find(|case| case["emptyBuffers"].as_u64().unwrap() > 0).unwrap();
    let mut record = RecordValue::default();
    record.fields.insert(1, FieldValue::List(std::iter::repeat_with(|| FieldValue::Bytes64(Vec::new())).take(case["emptyBuffers"].as_u64().unwrap() as usize).collect()));
    let bytes = pack_rt::encode_record_body(&buffer_spec(), &record, &PackEncodeOptions::default()).unwrap();
    let mut options = PackDecodeOptions::default();
    options.limits.max_total_alloc = case["maximumBytes"].as_u64().unwrap();
    assert!((bytes.len() as u64) < options.limits.max_total_alloc);
    assert!(pack_rt::decode_record_body(&bytes, &buffer_spec(), &options).is_err(), "empty intrinsic buffers still require owned list slots");
}
