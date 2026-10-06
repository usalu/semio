use super::*;
use semio_framework_value::DslValue;
use semio_framework_dsl_record::FieldSpec;
use semio_framework_dsl_record::FieldValue;
use semio_framework_dsl_record::JoinMode;
use semio_framework_value::NativeDecodeControl;
use semio_framework_value::Number;
use semio_framework_dsl_record::ParseOptions;
use semio_framework_dsl_record::RecordLayout;
use semio_framework_dsl_record::RecordSpec;
use semio_framework_dsl_record::RecordValue;
use semio_framework_dsl_record::Shape;
use semio_framework_dsl_record::SourceMode;
use crate::sqlite_snapshot::{SnapshotEncoding, SqliteDatabase, SqliteDatabaseLimits, SqliteRow, SqliteSnapshotControl, SqliteSnapshotPhase, SqliteValue, artifact::NativeEncodingBound};
use semio_framework_diagnostic::Limits;
use semio_framework_value::{ValueError, ValueRefusalKind};

struct MissingEncodingGuard {
    output_bytes: usize,
}
struct MissingControlledEncoding {
    output_bytes: usize,
}
struct GuardedEncoding {
    output_bytes: usize,
}
struct EstimatedEncoding {
    output_bytes: usize,
}
std::thread_local! { static ENCODER_CALLS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) }; static CONTROLLED_ENCODER_CALLS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) }; }

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
            fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase,ValueError> {
                control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, 1)?;
                let mut database = SqliteDatabase::from_schema(Self::SQLITE_SCHEMA)?;
                database.table_mut("encoding_expansion")?.rows.push(SqliteRow { rowid: 1, values: vec![SqliteValue::Integer(1), SqliteValue::Integer(i64::try_from(self.output_bytes).map_err(|_|ValueError::new(ValueRefusalKind::WorkLimit,"output size exceeds i64"))?)] });
                Ok(database)
            }
            fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self,ValueError> {
                control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 0, 1)?;
                Ok(Self { output_bytes: usize::try_from(database.table("encoding_expansion")?.rows[0].integer(1)?).map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"stored output size must fit usize"))? })
            }
            $($guard)*
        }
    };
}
expansion_owner!(MissingEncodingGuard,);
expansion_owner!(
    MissingControlledEncoding,
    fn preflight_sqlite_snapshot_encoding(&self, _: SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<(), ValueError> {
        let mut bound = NativeEncodingBound::new(control)?;
        bound.add(self.output_bytes)?;
        bound.finish()
    }
);
expansion_owner!(
    GuardedEncoding,
    fn encode_sqlite_snapshot_native(&self, encoding: SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<crate::io_schema::IoPayload, ValueError> {
        use semio_framework_value::native_encoding::{NativeEncodeControl, NativeEncodeProgress};
        let limits = control.limits();
        if self.output_bytes > limits.max_file_bytes {
            return Err(ValueError::new(ValueRefusalKind::OwnershipLimit, "native fixture exceeds file ceiling"));
        }
        let mut callback = |event: NativeEncodeProgress| control.checkpoint(SqliteSnapshotPhase::EncodeNative, event.completed, event.total).is_ok();
        let mut native = NativeEncodeControl::new(limits.max_value_bytes, &mut callback);
        native.begin_stage(self.output_bytes)?;
        let mut bytes = native.allocate_vec(self.output_bytes)?;
        CONTROLLED_ENCODER_CALLS.with(|count| count.set(count.get() + 1));
        while bytes.len() < self.output_bytes {
            let count = 65536.min(self.output_bytes - bytes.len());
            bytes.resize(bytes.len() + count, b'x');
            native.advance(count)?;
        }
        match encoding {
            SnapshotEncoding::Binary => Ok(crate::io_schema::IoPayload::Binary(bytes)),
            SnapshotEncoding::Text => Ok(crate::io_schema::IoPayload::Text(String::from_utf8(bytes).map_err(|_| ValueError::new(ValueRefusalKind::InvariantViolated, "generated fixture text must be UTF-8"))?)),
        }
    }
    fn preflight_sqlite_snapshot_encoding(&self, _: SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<(), ValueError> {
        let mut bound = NativeEncodingBound::new(control)?;
        bound.add(self.output_bytes)?;
        bound.finish()
    }
);
expansion_owner!(
    EstimatedEncoding,
    fn encode_sqlite_snapshot_native(&self, encoding: SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<crate::io_schema::IoPayload, ValueError> {
        GuardedEncoding { output_bytes: self.output_bytes }.encode_sqlite_snapshot_native(encoding, control)
    }
    fn preflight_sqlite_snapshot_encoding(&self, _: SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<(), ValueError> {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
        let mut bound = NativeEncodingBound::new(control)?;
        bound.repeated(self.output_bytes, fixture["exactEncodingAdmission"]["estimateMultiplier"].as_u64().unwrap() as usize)?;
        bound.finish()
    }
);

#[test]
fn sqlite_snapshot_native_exact_output_admission_ignores_approximate_preflight() {
    use std::{
        io::Write,
        process::{Command, Stdio},
    };
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    
    let case = &fixture["exactEncodingAdmission"];
    let output_bytes = case["outputBytes"].as_u64().unwrap() as usize;
    let snapshot = EstimatedEncoding { output_bytes };
    let mut limits = SqliteDatabaseLimits::default();
    limits.max_value_bytes = case["maximumBytes"].as_u64().unwrap() as usize;
    limits.max_file_bytes = case["maximumFileBytes"].as_u64().unwrap() as usize;
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    let bytes = crate::sqlite_snapshot::export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let script = "import{Database}from'bun:sqlite';const x=JSON.parse(await Bun.stdin.text());const d=Database.deserialize(Buffer.from(x.bytes,'base64'));const n=d.query('SELECT output_bytes FROM encoding_expansion WHERE id=1').get().output_bytes;await Bun.write(Bun.stdout,Uint8Array.from({length:n},()=>120));d.close();";
    let input = serde_json::json!({"fixture":fixture,"bytes":protocol::bytes::encode_base64(&bytes)});
    let mut child = Command::new("bun").args(["-e", script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(input.to_string().as_bytes()).unwrap();
    let oracle = child.wait_with_output().unwrap();
    assert!(oracle.status.success(), "{}", String::from_utf8_lossy(&oracle.stderr));
    assert_eq!(oracle.stdout.len(), output_bytes);
    let dialect = crate::io_schema::ArtifactDialect { artifact_kind: "fixture.expansion".into(), standard: "1".into(), subset: "*".into() };
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        assert!(snapshot.preflight_sqlite_snapshot_encoding(encoding, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_err());
        ENCODER_CALLS.with(|count| count.set(0));
        let result = (EstimatedEncoding::sqlite_codec().import)("fixture.expansion/v1", &dialect, database.clone(), encoding, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
        let actual = match result.value {
            crate::io_schema::IoPayload::Binary(bytes) => bytes,
            crate::io_schema::IoPayload::Text(text) => text.into_bytes(),
        };
        assert_eq!(actual, oracle.stdout);
        ENCODER_CALLS.with(|count| assert_eq!(count.get(), 0));
        let mut short = limits;
        short.max_file_bytes -= 1;
        assert!((EstimatedEncoding::sqlite_codec().import)("fixture.expansion/v1", &dialect, database.clone(), encoding, &mut SqliteSnapshotControl::new(&mut |_| true, short)).is_err());
    }
}

#[test]
fn sqlite_snapshot_native_encoding_admission_rejects_before_either_encoder() {
    use std::{
        io::Write,
        process::{Command, Stdio},
    };
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let dialect = crate::io_schema::ArtifactDialect { artifact_kind: "fixture.expansion".into(), standard: "1".into(), subset: "*".into() };
    for case in fixture["cases"].as_array().unwrap() {
        let output_bytes = case["outputBytes"].as_u64().unwrap() as usize;
        let guarded = case["guarded"].as_bool().unwrap();
        let codec = if !guarded {
            MissingEncodingGuard::sqlite_codec()
        } else if case["controlled"].as_bool().unwrap() {
            GuardedEncoding::sqlite_codec()
        } else {
            MissingControlledEncoding::sqlite_codec()
        };
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
            CONTROLLED_ENCODER_CALLS.with(|count| count.set(0));
            let mut limits = SqliteDatabaseLimits::default();
            limits.max_value_bytes = case["maximumBytes"].as_u64().unwrap() as usize;
            limits.max_file_bytes = limits.max_value_bytes;
            let cancel = case["cancelEncoding"].as_bool().unwrap();
            let mut callback = |event: crate::sqlite_snapshot::SqliteSnapshotProgress| !cancel || event.phase != SqliteSnapshotPhase::EncodeNative;
            let result = (codec.import)("fixture.expansion/v1", &dialect, database.clone(), encoding, &mut SqliteSnapshotControl::new(&mut callback, limits));
            let encoded = case["encoded"].as_bool().unwrap();
            assert_eq!(result.is_ok(), encoded, "{} {encoding:?}: {result:?}", case["id"]);
            ENCODER_CALLS.with(|count| assert_eq!(count.get(), 0, "ordinary encoders must never be used by SQLite import"));
            CONTROLLED_ENCODER_CALLS.with(|count| assert_eq!(count.get(), usize::from(encoded)));
            if let Ok(outcome) = result {
                let actual = match outcome.value {
                    crate::io_schema::IoPayload::Binary(bytes) => bytes.len(),
                    crate::io_schema::IoPayload::Text(text) => text.len(),
                };
                assert_eq!(actual, output_bytes);
            }
        }
    }
}

fn encoding_stops_inside_owned_output(encoding: SnapshotEncoding) {
    use std::{
        io::Write,
        process::{Command, Stdio},
    };
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    
    let case = &fixture["interiorEncoding"];
    let output_bytes = case["outputBytes"].as_u64().unwrap() as usize;
    let cancel_at = case["cancelAt"].as_u64().unwrap() as usize;
    let snapshot = GuardedEncoding { output_bytes };
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    let bytes = crate::sqlite_snapshot::export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let input = serde_json::json!({"fixture":fixture,"bytes":protocol::bytes::encode_base64(&bytes)});
    let script = "import{Database}from'bun:sqlite';const x=JSON.parse(await Bun.stdin.text());const db=Database.deserialize(Buffer.from(x.bytes,'base64'));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok')throw Error('integrity');await Bun.write(Bun.stdout,String(db.query('SELECT output_bytes FROM encoding_expansion WHERE id=1').get().output_bytes));db.close();";
    let mut child = Command::new("bun").args(["-e", script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(input.to_string().as_bytes()).unwrap();
    let oracle = child.wait_with_output().unwrap();
    assert!(oracle.status.success(), "{}", String::from_utf8_lossy(&oracle.stderr));
    assert_eq!(String::from_utf8(oracle.stdout).unwrap(), output_bytes.to_string());
    let dialect = crate::io_schema::ArtifactDialect { artifact_kind: "fixture.expansion".into(), standard: "1".into(), subset: "*".into() };
    let mut limits = SqliteDatabaseLimits::default();
    limits.max_value_bytes = case["maximumBytes"].as_u64().unwrap() as usize;
    limits.max_file_bytes = limits.max_value_bytes;
    let mut reached = false;
    ENCODER_CALLS.with(|count| count.set(0));
    let mut callback = |event: crate::sqlite_snapshot::SqliteSnapshotProgress| {
        if event.phase == SqliteSnapshotPhase::EncodeNative && event.total == output_bytes && event.completed >= cancel_at && event.completed < event.total {
            reached = true;
            false
        } else {
            true
        }
    };
    let result = (GuardedEncoding::sqlite_codec().import)("fixture.expansion/v1", &dialect, database, encoding, &mut SqliteSnapshotControl::new(&mut callback, limits));
    assert_eq!(result.is_err(), case["expectedCanceled"].as_bool().unwrap(), "{encoding:?} encoding must stop during real output ownership");
    assert!(reached, "expected an interior EncodeNative byte checkpoint");
    ENCODER_CALLS.with(|count| assert_eq!(count.get(), 0, "ordinary printer/packer must not be called"));
}
#[test]
fn sqlite_snapshot_native_binary_encoding_stops_inside_owned_output() {
    encoding_stops_inside_owned_output(SnapshotEncoding::Binary)
}
#[test]
fn sqlite_snapshot_native_text_encoding_stops_inside_owned_output() {
    encoding_stops_inside_owned_output(SnapshotEncoding::Text)
}

#[derive(semio_framework_dsl_record_derive::DslRecord)]
struct ControlledOutputFields {
    items: Vec<String>,
}

#[test]
fn sqlite_snapshot_native_record_owner_output_uses_one_control_through_both_envelopes() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let case = &fixture["typedProjection"];
    let count = case["count"].as_u64().unwrap() as usize;
    let source = ControlledOutputFields { items: (0..count).map(|index| index.to_string()).collect() };
    let limits = SqliteDatabaseLimits::default();
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        let output = super::encode_sqlite_snapshot_record_native(
            encoding,
            "fixture.output",
            ControlledOutputFields::__dsl_spec_producer(),
            |native| source.__dsl_to_record_controlled(native),
            &mut SqliteSnapshotControl::new(&mut |_| true, limits),
        )
        .unwrap();
        let decoded = super::decode_sqlite_snapshot_record_native(
            &output,
            "fixture.output",
            ControlledOutputFields::__dsl_spec_producer(),
            |record, native| ControlledOutputFields::__dsl_from_record_controlled(record, native),
            &mut SqliteSnapshotControl::new(&mut |_| true, limits),
        )
        .unwrap();
        assert_eq!(decoded.items, source.items);
        let length = match &output {
            crate::io_schema::IoPayload::Binary(bytes) => bytes.len(),
            crate::io_schema::IoPayload::Text(text) => text.len(),
        };
        let mut exact = limits;
        exact.max_file_bytes = length;
        assert_eq!(
            super::encode_sqlite_snapshot_record_native(
                encoding,
                "fixture.output",
                ControlledOutputFields::__dsl_spec_producer(),
                |native| source.__dsl_to_record_controlled(native),
                &mut SqliteSnapshotControl::new(&mut |_| true, exact)
            )
            .unwrap(),
            output
        );
        exact.max_file_bytes -= 1;
        assert!(
            super::encode_sqlite_snapshot_record_native(
                encoding,
                "fixture.output",
                ControlledOutputFields::__dsl_spec_producer(),
                |native| source.__dsl_to_record_controlled(native),
                &mut SqliteSnapshotControl::new(&mut |_| true, exact)
            )
            .is_err()
        );
        let mut reached = false;
        let mut callback = |event: crate::sqlite_snapshot::SqliteSnapshotProgress| {
            if event.phase == SqliteSnapshotPhase::EncodeNative && event.total == count && event.completed >= 256 && event.completed < event.total {
                reached = true;
                false
            } else {
                true
            }
        };
        assert!(
            super::encode_sqlite_snapshot_record_native(
                encoding,
                "fixture.output",
                ControlledOutputFields::__dsl_spec_producer(),
                |native| source.__dsl_to_record_controlled(native),
                &mut SqliteSnapshotControl::new(&mut callback, limits)
            )
            .is_err()
        );
        assert!(reached);
        let mut tiny = limits;
        tiny.max_allocation_bytes = 1;
        assert!(
            super::encode_sqlite_snapshot_record_native(
                encoding,
                "fixture.output",
                ControlledOutputFields::__dsl_spec_producer(),
                |native| source.__dsl_to_record_controlled(native),
                &mut SqliteSnapshotControl::new(&mut |_| true, tiny)
            )
            .is_err()
        );
    }
}

#[test]
fn sqlite_snapshot_native_typed_projection_stops_inside_known_collection_before_ordinary_copy() {
    use semio_framework_value::native_encoding::{NativeEncodeControl, NativeEncodeProgress};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let case = &fixture["typedProjection"];
    let count = case["count"].as_u64().unwrap() as usize;
    let cancel_at = case["cancelAt"].as_u64().unwrap() as usize;
    let source = ControlledOutputFields { items: (0..count).map(|index| index.to_string()).collect() };
    let mut interior = false;
    let mut callback = |event: NativeEncodeProgress| {
        if event.total == count && event.completed >= cancel_at && event.completed < event.total {
            interior = true;
            false
        } else {
            true
        }
    };
    let mut control = NativeEncodeControl::new(case["maximumBytes"].as_u64().unwrap() as usize, &mut callback);
    assert!(source.__dsl_to_record_controlled(&mut control).is_err());
    assert!(interior);
    let mut admit = |_| true;
    let mut control = NativeEncodeControl::new(1, &mut admit);
    assert!(source.__dsl_to_record_controlled(&mut control).is_err());
    assert_eq!(control.owned_bytes(), 0);
    let record = source.__dsl_to_record_controlled(&mut NativeEncodeControl::new(case["maximumBytes"].as_u64().unwrap() as usize, &mut |_| true)).unwrap();
    let actual = ControlledOutputFields::__dsl_from_record(&record).unwrap();
    assert_eq!(actual.items, source.items);
    use std::{
        io::Write,
        process::{Command, Stdio},
    };
    let script = "import{Database}from'bun:sqlite';const x=JSON.parse(await Bun.stdin.text());const d=new Database(':memory:');d.run('CREATE TABLE item(id INTEGER PRIMARY KEY,text TEXT NOT NULL)');for(let i=0;i<x.count;i++)d.run('INSERT INTO item VALUES(?,?)',i,String(i));await Bun.write(Bun.stdout,JSON.stringify(d.query('SELECT text FROM item ORDER BY id').all().map(r=>r.text)));d.close();";
    let mut child = Command::new("bun").args(["-e", script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(case.to_string().as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(actual.items, serde_json::from_slice::<Vec<String>>(&output.stdout).unwrap());
}

#[test]
fn sqlite_snapshot_native_physical_text_preserves_escaped_utf8_and_cancels_inside_emission() {
    use semio_framework_value::native_encoding::{NativeEncodeControl, NativeEncodeProgress};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let case = &fixture["physicalText"];
    let expected = case["seed"].as_str().unwrap().repeat(case["repeatCount"].as_u64().unwrap() as usize);
    let maximum = case["maximumBytes"].as_u64().unwrap() as usize;
    let cancel_at = case["cancelAt"].as_u64().unwrap() as usize;
    let spec = semio_framework_dsl_record::RecordSpec::new(Some("output"), semio_framework_dsl_record::RecordLayout::Inline, vec![FieldSpec::new(1, "text", Shape::Text)]);
    let mut record = semio_framework_dsl_record::RecordValue::default();
    record.fields.insert(1, semio_framework_dsl_record::FieldValue::Text(expected.clone()));
    let output = semio_framework_dsl_record::print_controlled(&record, &spec, semio_framework_dsl_record::JoinMode::Document, usize::MAX, &mut NativeEncodeControl::new(maximum, &mut |_| true)).unwrap();
    let options = ParseOptions { limits: Limits { max_bytes: maximum, ..Limits::default() }, mode: semio_framework_dsl_record::SourceMode::Document };
    let actual = semio_framework_dsl_record::parse_exact_controlled(&output, &spec, &options, &mut semio_framework_value::NativeDecodeControl::new(maximum, &mut |_| true)).unwrap();
    assert_eq!(actual, record);
    assert_eq!(semio_framework_dsl_record::parse_exact(&output, &spec, &options).unwrap(), record);
    assert_eq!(output, semio_framework_dsl_record::print(&record, &spec, JoinMode::Document));
    let mut reached = false;
    let mut cancel = |event: NativeEncodeProgress| {
        if event.owned_bytes >= output.len() && event.total == expected.len() && event.completed >= cancel_at && event.completed < event.total {
            reached = true;
            false
        } else {
            true
        }
    };
    assert!(semio_framework_dsl_record::print_controlled(&record, &spec, JoinMode::Document, usize::MAX, &mut NativeEncodeControl::new(maximum, &mut cancel)).is_err());
    assert!(reached);
    let mut admit = |_| true;
    let mut control = NativeEncodeControl::new(1, &mut admit);
    assert!(semio_framework_dsl_record::print_controlled(&record, &spec, JoinMode::Document, usize::MAX, &mut control).is_err());
    assert_eq!(control.owned_bytes(), 0);
    let mut accept = |_| true;
    let mut file_control = NativeEncodeControl::new(maximum, &mut accept);
    assert!(semio_framework_dsl_record::print_controlled(&record, &spec, JoinMode::Document, case["maximumFileBytes"].as_u64().unwrap() as usize, &mut file_control).is_err());
    assert!(file_control.owned_bytes() < case["maximumFileBytes"].as_u64().unwrap() as usize, "only the small borrowed measurement frontier may be admitted before file refusal");
    use std::{
        io::Write,
        process::{Command, Stdio},
    };
    let script = "import{Database}from'bun:sqlite';const x=JSON.parse(await Bun.stdin.text());const d=new Database(':memory:');d.run('CREATE TABLE text_literal(id INTEGER PRIMARY KEY,text TEXT NOT NULL)');d.run('INSERT INTO text_literal VALUES(1,?)',x.seed.repeat(x.repeatCount));await Bun.write(Bun.stdout,d.query('SELECT text FROM text_literal').get().text);d.close();";
    let mut child = Command::new("bun").args(["-e", script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(case.to_string().as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(output.stdout, expected.as_bytes());
}

#[test]
fn sqlite_snapshot_native_physical_text_keeps_literal_lexer_boundaries() {
    use semio_framework_value::native_encoding::NativeEncodeControl;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let spec = semio_framework_dsl_record::RecordSpec::new(Some("literal"), semio_framework_dsl_record::RecordLayout::Inline, vec![FieldSpec::new(0, "text", Shape::Text)]);
    for source in fixture["physicalLiteralTexts"].as_array().unwrap() {
        let mut record = semio_framework_dsl_record::RecordValue::default();
        record.fields.insert(0, semio_framework_dsl_record::FieldValue::Text(source.as_str().unwrap().into()));
        for mode in [semio_framework_dsl_record::JoinMode::Document, semio_framework_dsl_record::JoinMode::Inline] {
            let mut accept = |_| true;
            let mut control = NativeEncodeControl::new(65536, &mut accept);
            let text = semio_framework_dsl_record::print_controlled(&record, &spec, mode, usize::MAX, &mut control).unwrap();
            assert_eq!(text, semio_framework_dsl_record::print(&record, &spec, mode), "{source}");
            assert_eq!(semio_framework_dsl_record::parse_exact(&text, &spec, &ParseOptions::default()).unwrap(), record, "{source}");
            let mut accept = |_| true;
            let mut control = semio_framework_value::NativeDecodeControl::new(65536, &mut accept);
            assert_eq!(semio_framework_dsl_record::parse_exact_controlled(&text, &spec, &ParseOptions::default(), &mut control).unwrap(), record, "{source}");
        }
    }
    use std::{
        io::Write,
        process::{Command, Stdio},
    };
    let script = "import{Database}from'bun:sqlite';const x=JSON.parse(await Bun.stdin.text());const d=new Database(':memory:');d.run('CREATE TABLE literal(id INTEGER PRIMARY KEY,text TEXT NOT NULL)');x.forEach((text,id)=>d.run('INSERT INTO literal VALUES(?,?)',id,text));await Bun.write(Bun.stdout,JSON.stringify(d.query('SELECT text FROM literal ORDER BY id').all().map(r=>r.text)));d.close();";
    let mut child = Command::new("bun").args(["-e", script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(fixture["physicalLiteralTexts"].to_string().as_bytes()).unwrap();
    let result = child.wait_with_output().unwrap();
    assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
    assert_eq!(serde_json::from_slice::<serde_json::Value>(&result.stdout).unwrap(), fixture["physicalLiteralTexts"]);
}

fn physical_pack_output_controlled(document: bool) {
    use semio_framework_value::native_encoding::{NativeEncodeControl, NativeEncodeProgress};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let case = &fixture["physicalBytes"];
    let length = case["length"].as_u64().unwrap() as usize;
    let maximum = case["maximumBytes"].as_u64().unwrap() as usize;
    let cancel_at = case["cancelAt"].as_u64().unwrap() as usize;
    let expected: Vec<u8> = (0..length).map(|index| (index % 256) as u8).collect();
    let mut record = semio_framework_dsl_record::RecordValue::default();
    record.fields.insert(0, semio_framework_dsl_record::FieldValue::Bytes64(expected.clone()));
    let spec = semio_framework_dsl_record::RecordSpec::new(Some("octets"), semio_framework_dsl_record::RecordLayout::Inline, vec![FieldSpec::new(0, "bytes", Shape::Bytes64)]);
    let emit = |control: &mut NativeEncodeControl<'_>| {
        if document { crate::os_pack::encode_document_controlled(&spec, &record, &PackEncodeOptions::default(), control) } else { crate::os_pack::encode_record_body_controlled(&spec, &record, &PackEncodeOptions::default(), control) }
    };
    let mut accept = |_| true;
    let output = emit(&mut NativeEncodeControl::new(maximum, &mut accept)).unwrap();
    let actual = if document { crate::os_pack::decode_document(&output, &spec, &PackDecodeOptions::default()).unwrap().0 } else { crate::os_pack::decode_record_body_exact(&output, &spec, &PackDecodeOptions::default()).unwrap() };
    assert_eq!(actual, record);
    let ordinary = if document { crate::os_pack::encode_document(&spec, &record, &PackEncodeOptions::default()).unwrap() } else { crate::os_pack::encode_record_body(&spec, &record, &PackEncodeOptions::default()).unwrap() };
    assert_eq!(output, ordinary);
    let mut reached = false;
    let mut callback = |event: NativeEncodeProgress| {
        if event.total == length && event.completed >= cancel_at && event.completed < event.total {
            reached = true;
            false
        } else {
            true
        }
    };
    assert!(emit(&mut NativeEncodeControl::new(maximum, &mut callback)).is_err());
    assert!(reached);
    let mut accept = |_| true;
    let mut control = NativeEncodeControl::new(1, &mut accept);
    assert!(emit(&mut control).is_err());
    assert_eq!(control.owned_bytes(), 0);
    use std::{
        io::Write,
        process::{Command, Stdio},
    };
    let script = "import{Database}from'bun:sqlite';const x=JSON.parse(await Bun.stdin.text());const bytes=Uint8Array.from({length:x.length},(_,i)=>i%256);const d=new Database(':memory:');d.run('CREATE TABLE octets(id INTEGER PRIMARY KEY,data BLOB NOT NULL)');d.run('INSERT INTO octets VALUES(1,?)',bytes);await Bun.write(Bun.stdout,d.query('SELECT data FROM octets').get().data);d.close();";
    let mut child = Command::new("bun").args(["-e", script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(case.to_string().as_bytes()).unwrap();
    let result = child.wait_with_output().unwrap();
    assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
    assert_eq!(result.stdout, expected);
}

#[test]
fn sqlite_snapshot_native_physical_pack_record_controls_intrinsic_octet_emission() {
    physical_pack_output_controlled(false)
}

#[test]
fn sqlite_snapshot_native_physical_pack_document_controls_intrinsic_octet_emission() {
    physical_pack_output_controlled(true)
}

#[test]
fn sqlite_snapshot_native_physical_pack_document_preserves_chunk_indexes_frames_and_caller_ceilings() {
    use semio_framework_value::native_encoding::NativeEncodeControl;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let case = &fixture["physicalBytes"];
    let length = case["length"].as_u64().unwrap() as usize;
    let maximum = case["maximumBytes"].as_u64().unwrap() as usize;
    let bytes: Vec<u8> = (0..length).map(|index| (index % 256) as u8).collect();
    let mut record = semio_framework_dsl_record::RecordValue::default();
    record.fields.insert(0, semio_framework_dsl_record::FieldValue::Bytes64(bytes.clone()));
    let spec = semio_framework_dsl_record::RecordSpec::new(Some("octets"), semio_framework_dsl_record::RecordLayout::Inline, vec![FieldSpec::new(0, "bytes", Shape::Bytes64)]);
    for codec in [crate::os_pack::CodecId(0), crate::os_pack::CodecId(1)] {
        let options = PackEncodeOptions { codec, chunk_threshold: 1, chunk_size: 8192, frame_size: 16, ..PackEncodeOptions::default() };
        let output = crate::os_pack::encode_document_controlled(&spec, &record, &options, &mut NativeEncodeControl::new(maximum, &mut |_| true)).unwrap();
        assert_eq!(output, crate::os_pack::encode_document(&spec, &record, &options).unwrap());
        assert_eq!(crate::os_pack::decode_document(&output, &spec, &PackDecodeOptions::default()).unwrap().0, record);
        let footer = ::semio_framework_async::poll::resolve_ready(crate::os_pack::read_footer_only(&output.as_slice())).unwrap();
        let mut body = vec![1, 0, 9, 16];
        body.extend(0..16);
        assert_eq!(footer.content_hash.0, *blake3::hash(&body).as_bytes());
        let mut limits = options.clone();
        limits.limits.max_file_len = output.len() as u64 - 1;
        assert!(crate::os_pack::encode_document_controlled(&spec, &record, &limits, &mut NativeEncodeControl::new(maximum, &mut |_| true)).is_err());
        limits = options.clone();
        limits.limits.max_total_alloc = 1;
        let mut accept = |_| true;
        let mut control = NativeEncodeControl::new(maximum, &mut accept);
        assert!(crate::os_pack::encode_document_controlled(&spec, &record, &limits, &mut control).is_err());
        assert_eq!(control.owned_bytes(), 0);
    }
}

#[derive(semio_framework_dsl_record_derive::DslRecord)]
struct PackControlRow {
    signed: i64,
    unsigned: u64,
    float: f64,
    flag: bool,
    text: Option<String>,
}

#[test]
fn sqlite_snapshot_native_physical_pack_expression_has_controlled_literal_printer() {
    use semio_framework_value::native_encoding::{NativeEncodeControl, NativeEncodeProgress};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let case = &fixture["physicalBytes"];
    let count = case["length"].as_u64().unwrap() as usize;
    let maximum = case["maximumBytes"].as_u64().unwrap() as usize;
    let name = "x".repeat(count);
    let expression = semio_framework_dsl_record::ExprValue::Var(name);
    let mut record = semio_framework_dsl_record::RecordValue::default();
    record.fields.insert(0, semio_framework_dsl_record::FieldValue::Expr(expression));
    let spec = semio_framework_dsl_record::RecordSpec::new(Some("expression"), semio_framework_dsl_record::RecordLayout::Inline, vec![FieldSpec::new(0, "value", Shape::Expr)]);
    let options = PackEncodeOptions::default();
    let output = crate::os_pack::encode_record_body_controlled(&spec, &record, &options, &mut NativeEncodeControl::new(maximum, &mut |_| true)).unwrap();
    assert_eq!(output, crate::os_pack::encode_record_body(&spec, &record, &options).unwrap());
    assert_eq!(crate::os_pack::decode_record_body_exact(&output, &spec, &PackDecodeOptions::default()).unwrap(), record);
    let mut reached = false;
    let mut cancel = |event: NativeEncodeProgress| {
        if event.owned_bytes >= count && event.total == count && event.completed >= 65536 && event.completed < event.total {
            reached = true;
            false
        } else {
            true
        }
    };
    assert!(crate::os_pack::encode_record_body_controlled(&spec, &record, &options, &mut NativeEncodeControl::new(maximum, &mut cancel)).is_err());
    assert!(reached);
}

#[test]
fn sqlite_snapshot_native_physical_pack_expression_preserves_all_literal_variable_and_call_names() {
    use semio_framework_value::native_encoding::NativeEncodeControl;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let spec = semio_framework_dsl_record::RecordSpec::new(Some("expression"), semio_framework_dsl_record::RecordLayout::Inline, vec![FieldSpec::new(0, "value", Shape::Expr)]);
    let options = PackEncodeOptions::default();
    for name in fixture["physicalLiteralTexts"].as_array().unwrap() {
        let name = name.as_str().unwrap();
        for expression in [semio_framework_dsl_record::ExprValue::Var(name.into()), semio_framework_dsl_record::ExprValue::Call(name.into(), vec![semio_framework_dsl_record::ExprValue::Var(name.into())])] {
            let printed = semio_framework_dsl_record::print_expr(&expression);
            assert_eq!(semio_framework_dsl_record::parse_expr_text(&printed).unwrap(), expression, "literal expression name {name:?}");
            let mut record = semio_framework_dsl_record::RecordValue::default();
            record.fields.insert(0, semio_framework_dsl_record::FieldValue::Expr(expression));
            let output = crate::os_pack::encode_record_body_controlled(&spec, &record, &options, &mut NativeEncodeControl::new(1048576, &mut |_| true)).unwrap();
            assert_eq!(output, crate::os_pack::encode_record_body(&spec, &record, &options).unwrap());
            assert_eq!(crate::os_pack::decode_record_body_exact(&output, &spec, &PackDecodeOptions::default()).unwrap(), record);
            assert_eq!(crate::os_pack::decode_record_body_exact_controlled(&output, &spec, &PackDecodeOptions::default(), &mut NativeDecodeControl::new(1048576, &mut |_| true)).unwrap(), record);
        }
    }
}

#[test]
fn sqlite_snapshot_native_physical_pack_record_preserves_literal_table_and_numeric_tags() {
    use semio_framework_value::native_encoding::NativeEncodeControl;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let texts: Vec<String> = fixture["physicalLiteralTexts"].as_array().unwrap().iter().map(|value| value.as_str().unwrap().to_string()).collect();
    let rows = [PackControlRow { signed: i64::MIN, unsigned: u64::MAX, float: f64::from_bits(0xfff0000000000001), flag: true, text: Some(texts[13].clone()) }, PackControlRow { signed: i64::MAX, unsigned: 0, float: -0.0, flag: false, text: None }];
    let mut accept = |_| true;
    let mut control = NativeEncodeControl::new(16777216, &mut accept);
    let producer = PackControlRow::__dsl_spec_producer();
    let shape = semio_framework_dsl_record::Shape::Table(producer);
    let spec = semio_framework_dsl_record::RecordSpec::new(
        Some("table"),
        semio_framework_dsl_record::RecordLayout::Inline,
        vec![FieldSpec::new(0, "rows", shape), FieldSpec::new(1, "texts", Shape::List(Box::new(Shape::Text))), FieldSpec::new(2, "numbers", Shape::List(Box::new(Shape::UInt))), FieldSpec::new(3, "values", Shape::Value)],
    );
    let mut record = semio_framework_dsl_record::RecordValue::default();
    record.fields.insert(0, semio_framework_dsl_record::FieldValue::List(rows.iter().map(|row| row.__dsl_to_record_controlled(&mut control).map(semio_framework_dsl_record::FieldValue::Record).unwrap()).collect()));
    record.fields.insert(1, semio_framework_dsl_record::FieldValue::List(texts.iter().map(|text| semio_framework_dsl_record::FieldValue::Text(text.clone())).collect()));
    record.fields.insert(2, semio_framework_dsl_record::FieldValue::List(vec![FieldValue::UInt(u64::MAX), FieldValue::UInt(0)]));
    record.fields.insert(3, semio_framework_dsl_record::FieldValue::Value(semio_framework_value::DslValue::Object(texts.iter().enumerate().map(|(index, key)| (key.clone(), semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(index as u64)))).collect())));
    let options = PackEncodeOptions::default();
    let output = crate::os_pack::encode_record_body_controlled(&spec, &record, &options, &mut control).unwrap();
    assert_eq!(output, crate::os_pack::encode_record_body(&spec, &record, &options).unwrap());
    let decoded = crate::os_pack::decode_record_body_exact(&output, &spec, &PackDecodeOptions::default()).unwrap();
    let semio_framework_dsl_record::FieldValue::List(actual) = decoded.get(0).unwrap() else { panic!("table") };
    for (row, expected) in actual.iter().zip(&rows) {
        let semio_framework_dsl_record::FieldValue::Record(row) = row else { panic!("row") };
        let row = PackControlRow::__dsl_from_record(row).unwrap();
        assert_eq!(row.signed, expected.signed);
        assert_eq!(row.unsigned, expected.unsigned);
        assert_eq!(row.float.to_bits(), expected.float.to_bits());
        assert_eq!(row.flag, expected.flag);
        assert_eq!(row.text, expected.text);
    }
    assert_eq!(decoded.get(1), record.get(1));
    assert_eq!(decoded.get(2), record.get(2));
    let semio_framework_dsl_record::FieldValue::Value(semio_framework_value::DslValue::Object(entries)) = decoded.get(3).unwrap() else { panic!("object") };
    let mut expected: Vec<_> = texts.iter().enumerate().map(|(index, key)| (key.as_str(), index as u64)).collect();
    expected.sort_by(|a, b| a.0.as_bytes().cmp(b.0.as_bytes()));
    for ((key, value), (expected_key, number)) in entries.iter().zip(expected) {
        assert_eq!(key, expected_key);
        assert_eq!(value, &DslValue::Number(Number::UInt(number)));
    }
    use std::{
        io::Write,
        process::{Command, Stdio},
    };
    let script = "import{Database}from'bun:sqlite';const x=JSON.parse(await Bun.stdin.text());const db=new Database(':memory:');db.run('CREATE TABLE exact_key(id INTEGER PRIMARY KEY,text TEXT NOT NULL)');x.forEach((text,id)=>db.run('INSERT INTO exact_key VALUES(?,?)',id,text));await Bun.write(Bun.stdout,JSON.stringify(db.query('SELECT text FROM exact_key ORDER BY id').all().map(row=>row.text)));db.close();";
    let mut child = Command::new("bun").args(["-e", script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(fixture["physicalLiteralTexts"].to_string().as_bytes()).unwrap();
    let result = child.wait_with_output().unwrap();
    assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
    assert_eq!(serde_json::from_slice::<Vec<String>>(&result.stdout).unwrap(), texts);
}

#[path = "../../../../🪆️child/🧪️tests/🛫️encoding/🦀️.rs"]
mod child_encoding_tests;

#[test]
fn sqlite_snapshot_native_protocol_refusals_retain_control_and_text_authority_at_public_terminals() {
    use semio_framework_value::{NativeDecodeControl, NativeEncodeControl};
    let spec = semio_framework_dsl_record::RecordSpec::new(Some("expression"), semio_framework_dsl_record::RecordLayout::Inline, vec![FieldSpec::new(0, "value", Shape::Expr)]);
    let mut record = semio_framework_dsl_record::RecordValue::default();
    record.fields.insert(0, semio_framework_dsl_record::FieldValue::Expr(semio_framework_dsl_record::ExprValue::Var("x".into())));
    let bytes = crate::os_pack::encode_record_body(&spec, &record, &PackEncodeOptions::default()).unwrap();
    for kind in [ValueRefusalKind::Canceled, ValueRefusalKind::OwnershipLimit] {
        let mut callback = |_| kind != ValueRefusalKind::Canceled;
        let mut control = NativeEncodeControl::new(if kind == ValueRefusalKind::OwnershipLimit { 0 } else { 65536 }, &mut callback);
        let error = crate::os_pack::encode_record_body_controlled(&spec, &record, &PackEncodeOptions::default(), &mut control).unwrap_err();
        assert!(matches!(&error,pack::PackRefusal::ValueRefusal(error)if error.kind==kind));
        let message = error.to_string();
        let terminal = crate::io_schema::IoError::from_value_error(error.into_value_error());
        assert_eq!(terminal.cause.kind, kind);
        assert_eq!(format!("schema error: {}", terminal.cause.message), message);
        assert_eq!(control.owned_bytes(), 0);
        let mut callback = |_| kind != ValueRefusalKind::Canceled;
        let mut control = NativeDecodeControl::new(if kind == ValueRefusalKind::OwnershipLimit { 0 } else { 65536 }, &mut callback);
        let error = crate::os_pack::decode_record_body_exact_controlled(&bytes, &spec, &PackDecodeOptions::default(), &mut control).unwrap_err();
        assert!(matches!(&error,pack::PackRefusal::ValueRefusal(error)if error.kind==kind));
        let terminal = crate::io_schema::IoError::from_value_error(error.into_value_error());
        assert_eq!(terminal.cause.kind, kind);
        assert_eq!(control.owned_bytes(), 0);
    }
    let mut malformed = bytes;
    assert_eq!(malformed.last(), Some(&b'x'));
    *malformed.last_mut().unwrap() = b'(';
    let error = crate::os_pack::decode_record_body_exact_controlled(&malformed, &spec, &PackDecodeOptions::default(), &mut NativeDecodeControl::new(65536, &mut |_| true)).unwrap_err();
    let pack::PackRefusal::TextRefusal(text) = error else { panic!("actual expression parser lost source authority") };
    let span = text.span;
    let message = text.message.clone();
    let expected = text.expected.clone();
    assert_eq!(text.kind, ValueRefusalKind::InvalidValue);
    let mut yes = |_| true;
    let mut control = NativeEncodeControl::new(65536, &mut yes);
    let terminal = crate::io_schema::IoError::from_text_error_controlled(text, &mut control).unwrap();
    assert_eq!(terminal.cause.kind, ValueRefusalKind::InvalidValue);
    assert_eq!(terminal.cause.message, message);
    assert_eq!(terminal.diagnostics.len(), 1);
    assert_eq!(terminal.diagnostics[0].span, span);
    assert_eq!(terminal.diagnostics[0].message, message);
    assert_eq!(terminal.diagnostics[0].expected.as_ref().map(|set| set.tokens.as_slice()), expected.as_ref().map(std::slice::from_ref));
    assert!(control.owned_bytes() > 0);
}
