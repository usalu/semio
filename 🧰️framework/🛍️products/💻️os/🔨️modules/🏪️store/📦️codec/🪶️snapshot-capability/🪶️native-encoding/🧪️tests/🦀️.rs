use super::*;
use crate::sqlite_snapshot::{SqliteDatabase, SqliteDatabaseLimits, SqliteRow, SqliteValue, SqliteSnapshotControl, SqliteSnapshotPhase, SnapshotEncoding, artifact::NativeEncodingBound};

struct MissingEncodingGuard { output_bytes: usize }
struct GuardedEncoding { output_bytes: usize }
std::thread_local! { static ENCODER_CALLS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) }; }

macro_rules! expansion_owner {
    ($owner:ty, $($guard:item)*) => {
        impl ArtifactDsl for $owner {
            const EXTENSION: &'static str = "expansion";
            fn parse_dsl(text: &str) -> Result<Self, TextError> { Ok(Self { output_bytes: text.len() }) }
            fn print_dsl(&self) -> String { ENCODER_CALLS.with(|count| count.set(count.get() + 1)); "x".repeat(self.output_bytes) }
        }
        impl ArtifactPack for $owner {
            fn encode_pack_with(&self, _: &PackEncodeOptions) -> Result<Vec<u8>, PackError> { ENCODER_CALLS.with(|count| count.set(count.get() + 1)); Ok(vec![120; self.output_bytes]) }
            fn decode_pack_with(bytes: &[u8], _: &PackDecodeOptions) -> Result<Self, PackError> { Ok(Self { output_bytes: bytes.len() }) }
        }
        impl ArtifactSqliteSnapshot for $owner {
            const SQLITE_SCHEMA: &'static str = include_str!("../🧬️schema/🗄️.sql");
            fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, String> {
                control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, 1)?;
                let mut database = SqliteDatabase::from_schema(Self::SQLITE_SCHEMA).map_err(|error| error.to_string())?;
                database.table_mut("encoding_expansion")?.rows.push(SqliteRow { rowid: 1, values: vec![SqliteValue::Integer(1), SqliteValue::Integer(i64::try_from(self.output_bytes).map_err(|error| error.to_string())?)] });
                Ok(database)
            }
            fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, String> {
                control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 0, 1)?;
                Ok(Self { output_bytes: usize::try_from(database.table("encoding_expansion")?.rows[0].integer(1)?).map_err(|error| error.to_string())? })
            }
            $($guard)*
        }
    };
}
expansion_owner!(MissingEncodingGuard,);
expansion_owner!(GuardedEncoding,
    fn preflight_sqlite_snapshot_encoding(&self, _: SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<(), String> {
        let mut bound = NativeEncodingBound::new(control)?;
        bound.add(self.output_bytes)?;
        bound.finish()
    }
);

#[test]
fn sqlite_snapshot_native_encoding_admission_rejects_before_either_encoder() {
    use std::{io::Write, process::{Command, Stdio}};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let dialect = crate::io_schema::ArtifactDialect { artifact_kind: "fixture.expansion".into(), standard: "1".into(), subset: "*".into() };
    for case in fixture["cases"].as_array().unwrap() {
        let output_bytes = case["outputBytes"].as_u64().unwrap() as usize;
        let guarded = case["guarded"].as_bool().unwrap();
        let codec = if guarded { GuardedEncoding::sqlite_codec() } else { MissingEncodingGuard::sqlite_codec() };
        let database = GuardedEncoding { output_bytes }.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
        let bytes = crate::sqlite_snapshot::export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
        let script = "import {Database} from 'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok')throw Error('integrity');await Bun.write(Bun.stdout,String(db.query('SELECT output_bytes FROM encoding_expansion WHERE id=1').get().output_bytes));db.close();";
        let mut child = Command::new("bun").args(["-e", script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
        child.stdin.take().unwrap().write_all(&bytes).unwrap();
        let result = child.wait_with_output().unwrap();
        assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
        assert_eq!(String::from_utf8(result.stdout).unwrap(), output_bytes.to_string());
        for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
            ENCODER_CALLS.with(|count| count.set(0));
            let mut limits = SqliteDatabaseLimits::default();
            limits.max_value_bytes = case["maximumBytes"].as_u64().unwrap() as usize;
            limits.max_file_bytes = limits.max_value_bytes;
            let cancel = case["cancelEncoding"].as_bool().unwrap();
            let mut callback = |event: crate::sqlite_snapshot::SqliteSnapshotProgress| !cancel || event.phase != SqliteSnapshotPhase::EncodeNative;
            let result = (codec.import)("fixture.expansion/v1", &dialect, database.clone(), encoding, &mut SqliteSnapshotControl::new(&mut callback, limits));
            let encoded = case["encoded"].as_bool().unwrap();
            assert_eq!(result.is_ok(), encoded, "{} {encoding:?}: {result:?}", case["id"]);
            ENCODER_CALLS.with(|count| assert_eq!(count.get(), usize::from(encoded)));
            if let Ok(outcome) = result { let actual = match outcome.value { crate::io_schema::IoPayload::Binary(bytes) => bytes.len(), crate::io_schema::IoPayload::Text(text) => text.len() }; assert_eq!(actual, output_bytes); }
        }
    }
}
