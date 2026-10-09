use super::*;
use semio_framework_value::native_decoding::NativeDecodeControl;
use crate::os_store::NativeSnapshotDecodeOwner;
use semio_framework_dsl_record::FieldSpec;
use semio_framework_dsl_record::FieldValue;
use semio_framework_dsl_record::RecordLayout;
use semio_framework_dsl_record::RecordSpec;
use semio_framework_dsl_record::RecordValue;
use semio_framework_dsl_record::Shape;
use crate::sqlite_snapshot::{SqliteDatabase, SqliteDatabaseLimits, SqliteRow, SqliteSnapshotControl, SqliteSnapshotPhase, SqliteValue};
use semio_framework_value::{ValueError, ValueRefusalKind,RetainedCloneGrant,retirement::{RetireOwned,RetirementCursor}};

struct DecodedBuffers {
    buffers: Vec<Vec<u8>>,
}
impl RetireOwned for DecodedBuffers {
    fn retirement(self)->Box<dyn RetirementCursor>{self.buffers.retirement()}
    fn retirement_birth_bytes(&self)->Option<usize>{self.buffers.retirement_birth_bytes()}
    fn controlled_retirement_supported()->bool{<Vec<Vec<u8>> as RetireOwned>::controlled_retirement_supported()}
}
fn caller_grant()->RetainedCloneGrant{
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    serde_json::from_value(fixture["callerGrant"].clone()).unwrap()
}
fn close_original_recipient(native:&mut NativeDecodeControl<'_>){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for _ in 0..fixture["maximumCloseTurns"].as_u64().unwrap(){
        if !native.has_retirement_owner(){return}
        let step=native.close_retirement_recipient(caller_grant()).unwrap();
        assert!(step.progress()!=Default::default()||!native.has_retirement_owner(),"original authored close turn must advance actual custody");
    }
    assert!(!native.has_retirement_owner(),"original bounded close supervisor did not finish");
}
std::thread_local! { static COMPLETED_DECODERS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) }; }
std::thread_local! { static ORDINARY_DECODER_CALLS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) }; static CONTROLLED_DECODER_CALLS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) }; }

fn buffer_spec() -> RecordSpec {
    semio_framework_dsl_record::RecordSpec::new(Some("buffers"), semio_framework_dsl_record::RecordLayout::Lines, vec![FieldSpec::new(1, "buffers", Shape::List(Box::new(Shape::Bytes64)))])
}
fn buffer_spec_controlled<C: semio_framework_dsl_record::NativeSchemaControl>(control: &mut C) -> Result<RecordSpec, semio_framework_value::ValueError> {
    control.scoped_stage(|control| {
        control.begin_stage(1)?;
        let mut fields = control.allocate_vec(1)?;
        let shape = semio_framework_dsl_record::Shape::List(semio_framework_dsl_record::producer::boxed(semio_framework_dsl_record::Shape::Bytes64, control)?);
        fields.push(semio_framework_dsl_record::producer::field(1, "buffers", shape, control)?);
        control.step()?;
        semio_framework_dsl_record::producer::record(Some("buffers"), semio_framework_dsl_record::RecordLayout::Lines, fields, control)
    })
}
fn buffer_spec_producer() -> semio_framework_dsl_record::RecordSpecProducer {
    semio_framework_dsl_record::RecordSpecProducer { ordinary: buffer_spec, decoding: |control| buffer_spec_controlled(control), encoding: |control| buffer_spec_controlled(control) }
}
#[test]
fn sqlite_snapshot_erased_export_dispatches_only_the_declared_controlled_native_owner() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let case = &fixture["erasedDispatch"];
    let snapshot = DecodedBuffers { buffers: vec![vec![1; case["bufferBytes"].as_u64().unwrap() as usize]] };
    let bytes = snapshot.encode_pack();
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "fixture.buffers".into(), standard: "1".into(), subset: "*".into() };
    ORDINARY_DECODER_CALLS.with(|count| count.set(0));
    CONTROLLED_DECODER_CALLS.with(|count| count.set(0));
    let mut recipient=semio_framework_value::native_decoding::NativeDecodeRetirementRecipient::new();
    let mut callback=|_|true;
    let mut native=NativeDecodeControl::new(fixture["nativeMaximumBytes"].as_u64().unwrap()as usize,&mut callback);
    native.install_retirement_recipient(&mut recipient).unwrap();
    let outcome = (DecodedBuffers::sqlite_codec().export)("fixture.buffers/v1", &dialect, &crate::io_schema::IoPayload::Binary(bytes.clone()), &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default()),&mut NativeSnapshotDecodeOwner::new(&mut native,caller_grant())).unwrap();
    assert_eq!(outcome.value.table("decoded_buffer").unwrap().single_row().unwrap().blob(1).unwrap(), snapshot.buffers[0]);
    ORDINARY_DECODER_CALLS.with(|count| assert_eq!(count.get() as u64, case["ordinaryCalls"].as_u64().unwrap()));
    CONTROLLED_DECODER_CALLS.with(|count| assert_eq!(count.get() as u64, case["controlledCalls"].as_u64().unwrap()));
    let mut reached = false;
    let result = (DecodedBuffers::sqlite_codec().export)(
        "fixture.buffers/v1",
        &dialect,
        &crate::io_schema::IoPayload::Binary(bytes),
        &mut SqliteSnapshotControl::new(
            &mut |event| {
                if event.phase == SqliteSnapshotPhase::DecodeNative && event.completed >= case["cancelAt"].as_u64().unwrap() as usize {
                    reached = true;
                    false
                } else {
                    true
                }
            },
            SqliteDatabaseLimits::default(),
        ),
        &mut NativeSnapshotDecodeOwner::new(&mut native,caller_grant()),
    );
    assert!(result.is_err());
    assert!(reached);
    close_original_recipient(&mut native);
    drop(native);
    assert!(!recipient.has_owner());
}

impl ArtifactDsl for DecodedBuffers {
    const EXTENSION: &'static str = "buffers";
    fn envelope_id() -> &'static str {
        "fixture.buffers"
    }
    fn parse_dsl(_: &str) -> Result<Self, TextError> {
        Err(TextError { kind: semio_framework_value::ValueRefusalKind::UnsupportedOwner, message: "binary admission fixture".into(), span: Default::default(), expected: None })
    }
    fn print_dsl(&self) -> String {
        String::new()
    }
}

impl ArtifactPack for DecodedBuffers {
    fn encode_pack_with(&self, options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        let mut record = semio_framework_dsl_record::RecordValue::default();
        record.fields.insert(1, semio_framework_dsl_record::FieldValue::List(self.buffers.iter().cloned().map(semio_framework_dsl_record::FieldValue::Bytes64).collect()));
        let body = pack_rt::encode_document(&buffer_spec(), &record, options)?;
        let envelope = semio_format::SemioEnvelope::from_envelope_id(Self::envelope_id(), semio_format::Component::Pack, 1).unwrap();
        Ok(semio_format::wrap_binary(&envelope, &body))
    }
    fn decode_pack_with(bytes: &[u8], options: &PackDecodeOptions) -> Result<Self, PackError> {
        ORDINARY_DECODER_CALLS.with(|count| count.set(count.get() + 1));
        let (envelope, body) = semio_format::unwrap_binary(bytes).map_err(|error| PackError::from(error.into_value_error()))?;
        if !envelope.matches_identity(Self::envelope_id(), semio_format::Component::Pack, 1) {
            return Err(PackError::from(ValueError::new(ValueRefusalKind::UnsupportedOwner,"fixture envelope identity")));
        }
        let (mut record, _) = pack_rt::decode_document(&body, &buffer_spec(), options)?;
        let Some(semio_framework_dsl_record::FieldValue::List(fields)) = record.fields.remove(&1) else {
            return Err(PackError::from(ValueError::new(ValueRefusalKind::InvalidValue,"expected buffers")));
        };
        let buffers = fields
            .into_iter()
            .map(|field| match field {
                semio_framework_dsl_record::FieldValue::Bytes64(bytes) => Ok(bytes),
                _ => Err(PackError::from(ValueError::new(ValueRefusalKind::InvalidValue,"expected intrinsic octets"))),
            })
            .collect::<Result<Vec<_>, _>>()?;
        COMPLETED_DECODERS.with(|count| count.set(count.get() + 1));
        Ok(Self { buffers })
    }
}

fn bind_original_buffers(record: &mut RecordValue, snapshot_output: &mut Option<DecodedBuffers>, native: &mut NativeDecodeControl<'_>, body: &mut crate::os_store::NativeSnapshotBodyWallet) -> Result<(), ValueError> {
    let Some(FieldValue::List(fields)) = record.fields.get_mut(&1) else {
        return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "expected buffers"));
    };
    body.admit_frontier(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: std::mem::size_of::<Option<DecodedBuffers>>(), maximum_capacity_bytes: 0, maximum_release_bytes: 0, maximum_depth: 1 })?;
    *snapshot_output = Some(DecodedBuffers { buffers: Vec::new() });
    body.record_progress(semio_framework_value::RetainedCloneProgress { copied_items: 1, copied_bytes: std::mem::size_of::<Option<DecodedBuffers>>(), ..Default::default() })?;
    let buffers = &mut snapshot_output.as_mut().unwrap().buffers;
    body.allocate_vec_into(native, fields.len(), buffers, 2)?;
    for field in fields {
        let FieldValue::Bytes64(bytes) = field else {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "expected intrinsic octets"));
        };
        let copy_bytes = 2 * std::mem::size_of::<Vec<u8>>();
        body.admit_frontier(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy_bytes, maximum_capacity_bytes: 0, maximum_release_bytes: 0, maximum_depth: 3 })?;
        native.step()?;
        buffers.push(std::mem::take(bytes));
        body.record_progress(semio_framework_value::RetainedCloneProgress { copied_items: 1, copied_bytes: copy_bytes, ..Default::default() })?;
    }
    COMPLETED_DECODERS.with(|count| count.set(count.get() + 1));
    Ok(())
}

impl ArtifactSqliteSnapshot for DecodedBuffers {
    const SQLITE_SCHEMA: &'static str = include_str!("../🧬️schema/🗄️.sql");
    fn decode_sqlite_snapshot_native(payload: &crate::io_schema::IoPayload, control: &mut SqliteSnapshotControl<'_>, native: &mut NativeSnapshotDecodeOwner<'_, '_>) -> Result<Self, ValueError> {
        CONTROLLED_DECODER_CALLS.with(|count| count.set(count.get() + 1));
        decode_sqlite_snapshot_record_native(
            payload,
            Self::envelope_id(),
            buffer_spec_producer(),
            bind_original_buffers,
            control,
            native,
        )
    }
    fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, ValueError> {
        let count = self.buffers.iter().try_fold(0usize, |count, bytes| count.checked_add(bytes.len()).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "octet count overflow")))?;
        control.check_value_bytes(count)?;
        control.check_rows(self.buffers.len())?;
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, self.buffers.len())?;
        let mut database = SqliteDatabase::from_schema(Self::SQLITE_SCHEMA)?;
        for (index, bytes) in self.buffers.iter().enumerate() {
            let id = i64::try_from(index + 1).map_err(|_| ValueError::new(ValueRefusalKind::WorkLimit, "buffer ordinal exceeds i64"))?;
            database.table_mut("decoded_buffer")?.rows.push(SqliteRow { rowid: id, values: vec![SqliteValue::Integer(id), SqliteValue::Blob(bytes.clone())] });
        }
        Ok(database)
    }
    fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, ValueError> {
        control.check_database(database, SqliteSnapshotPhase::ReconstructSnapshot)?;
        Ok(Self { buffers: database.table("decoded_buffer")?.rows.iter().map(|row| row.blob(1).map(<[u8]>::to_vec)).collect::<Result<_, _>>()? })
    }
}

#[test]
fn sqlite_snapshot_binding_moves_original_octet_allocations_with_exact_structural_receipt() {
    let policy = caller_grant();
    let mut record = RecordValue::default();
    record.fields.insert(1, FieldValue::List(vec![FieldValue::Bytes64(vec![7; 16384]), FieldValue::Bytes64(vec![9; 16384])]));
    let original = match record.get(1).unwrap() {
        FieldValue::List(fields) => match (&fields[0], &fields[1]) {
            (FieldValue::Bytes64(a), FieldValue::Bytes64(b)) => [a.as_ptr(), b.as_ptr()],
            _ => unreachable!(),
        },
        _ => unreachable!(),
    };
    let mut output = None;
    let mut callback = |_| true;
    let mut native = NativeDecodeControl::new(1048576, &mut callback);
    let mut denied = crate::os_store::NativeSnapshotBodyWallet::new(RetainedCloneGrant { maximum_copy_bytes: 0, ..policy });
    let (refusal, allocated, released) = crate::test_allocation::observe_backing(|| bind_original_buffers(&mut record, &mut output, &mut native, &mut denied));
    assert_eq!(refusal.unwrap_err().kind, ValueRefusalKind::OwnershipLimit);
    assert_eq!((allocated, released), (0, 0));
    assert!(output.is_none());
    let mut wallet = crate::os_store::NativeSnapshotBodyWallet::new(policy);
    let (result, allocated, released) = crate::test_allocation::observe_backing(|| bind_original_buffers(&mut record, &mut output, &mut native, &mut wallet));
    result.unwrap();
    assert_eq!((allocated, released), (2 * std::mem::size_of::<Vec<u8>>(), 0));
    assert_eq!(wallet.progress().retained_capacity_bytes, allocated);
    assert_eq!(wallet.progress().copied_bytes, std::mem::size_of::<Option<DecodedBuffers>>() + 5 * std::mem::size_of::<Vec<u8>>());
    let buffers = &output.as_ref().unwrap().buffers;
    assert_eq!([buffers[0].as_ptr(), buffers[1].as_ptr()], original);
    assert!(buffers[0].iter().all(|byte| *byte == 7) && buffers[1].iter().all(|byte| *byte == 9));
    let FieldValue::List(fields) = record.get(1).unwrap() else { unreachable!() };
    assert!(fields.iter().all(|field| matches!(field, FieldValue::Bytes64(bytes) if bytes.is_empty() && bytes.capacity() == 0)));
    let mut retirement = semio_framework_value::retirement::controlled::ControlledRetirement::new((record, output)).unwrap_or_else(|(error, _)| panic!("{error}"));
    for _ in 0..131072 {
        if retirement.terminal_is_empty() { break; }
        let (step, allocated, released) = crate::test_allocation::observe_backing(|| retirement.step(policy).unwrap());
        assert!(step.progress().fits(policy));
        assert_eq!((allocated, released), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
    }
    assert!(retirement.terminal_is_empty());
    let (_, allocated, released) = crate::test_allocation::observe_backing(|| drop(retirement));
    assert_eq!((allocated, released), (0, 0));
    println!("[DEBUG] SQLite binding original two32768-byte payload allocations moved without duplication; fixed65536 copy authority; structural receipt/System equality; denied original retained and terminalDrop0");
}

#[test]
fn sqlite_snapshot_native_decoding_admits_compressed_aggregate_before_projection() {
    use std::{
        io::Write,
        process::{Command, Stdio},
    };
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "fixture.buffers".into(), standard: "1".into(), subset: "*".into() };
    for case in fixture["cases"].as_array().unwrap() {
        let snapshot =
            DecodedBuffers { buffers: case["bufferBytes"].as_array().unwrap().iter().map(|value| vec![0; value.as_u64().unwrap() as usize]).chain(std::iter::repeat_with(Vec::new).take(case["emptyBuffers"].as_u64().unwrap() as usize)).collect() };
        let mut options = PackEncodeOptions::default();
        options.codec = crate::os_pack::CodecId(1);
        options.chunk_threshold = u64::MAX;
        options.frame_size = 32768;
        let bytes = snapshot.encode_pack_with(&options).unwrap();
        let maximum = case["maximumBytes"].as_u64().unwrap() as usize;
        assert!(bytes.len() < maximum, "fixture must exercise decoded expansion");
        let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
        let sqlite = crate::sqlite_snapshot::export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
        let script = "import {Database} from 'bun:sqlite';const input=JSON.parse(await Bun.stdin.text());const db=Database.deserialize(Buffer.from(input.sqlite,'base64'));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok')throw Error('integrity');await Bun.write(Bun.stdout,String(db.query('SELECT SUM(length(octets)) AS n FROM decoded_buffer').get().n));db.close();";
        let mut child = Command::new("bun").args(["-e", script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
        let input = serde_json::json!({"sqlite": protocol::bytes::encode_base64(&sqlite)});
        child.stdin.take().unwrap().write_all(input.to_string().as_bytes()).unwrap();
        let result = child.wait_with_output().unwrap();
        assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
        assert_eq!(String::from_utf8(result.stdout).unwrap(), snapshot.buffers.iter().map(Vec::len).sum::<usize>().to_string());
        let mut limits = SqliteDatabaseLimits::default();
        limits.max_allocation_bytes = maximum;
        COMPLETED_DECODERS.with(|count| count.set(0));
        let mut recipient=semio_framework_value::native_decoding::NativeDecodeRetirementRecipient::new();
        let mut callback=|_|true;
        let mut native=NativeDecodeControl::new(fixture["nativeMaximumBytes"].as_u64().unwrap()as usize,&mut callback);
        native.install_retirement_recipient(&mut recipient).unwrap();
        let result = (DecodedBuffers::sqlite_codec().export)("fixture.buffers/v1", &dialect, &crate::io_schema::IoPayload::Binary(bytes.clone()), &mut SqliteSnapshotControl::new(&mut |_| true, limits),&mut NativeSnapshotDecodeOwner::new(&mut native,caller_grant()));
        let decoded = case["decoded"].as_bool().unwrap();
        assert_eq!(result.is_ok(), decoded, "{}: {result:?}", case["id"]);
        COMPLETED_DECODERS.with(|count| assert_eq!(count.get(), usize::from(decoded), "{}", case["id"]));
        close_original_recipient(&mut native);
        COMPLETED_DECODERS.with(|count| count.set(0));
        let result = (DecodedBuffers::sqlite_codec().export)("fixture.buffers/v1", &dialect, &crate::io_schema::IoPayload::Binary(bytes), &mut SqliteSnapshotControl::new(&mut |event| event.phase != SqliteSnapshotPhase::DecodeNative, limits),&mut NativeSnapshotDecodeOwner::new(&mut native,caller_grant()));
        assert!(result.is_err());
        COMPLETED_DECODERS.with(|count| assert_eq!(count.get(), 0));
        close_original_recipient(&mut native);
        drop(native);
        assert!(!recipient.has_owner());
    }
}

#[test]
fn sqlite_snapshot_native_decoding_admits_inline_empty_collection_storage() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let case = fixture["cases"].as_array().unwrap().iter().find(|case| case["emptyBuffers"].as_u64().unwrap() > 0).unwrap();
    let mut record = semio_framework_dsl_record::RecordValue::default();
    record.fields.insert(1, semio_framework_dsl_record::FieldValue::List(std::iter::repeat_with(|| semio_framework_dsl_record::FieldValue::Bytes64(Vec::new())).take(case["emptyBuffers"].as_u64().unwrap() as usize).collect()));
    let bytes = pack_rt::encode_record_body(&buffer_spec(), &record, &PackEncodeOptions::default()).unwrap();
    let mut options = PackDecodeOptions::default();
    options.limits.max_total_alloc = case["maximumBytes"].as_u64().unwrap();
    assert!((bytes.len() as u64) < options.limits.max_total_alloc);
    assert!(pack_rt::decode_record_body(&bytes, &buffer_spec(), &options).is_err(), "empty intrinsic buffers still require owned list slots");
}

#[test]
fn sqlite_snapshot_metadata_admits_final_file_before_mutating_domain_database() {
    use std::{
        io::Write,
        process::{Command, Stdio},
    };
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    
    let rows = fixture["metadataAdmission"]["domainRows"].as_u64().unwrap() as usize;
    let file_rows = fixture["metadataAdmission"]["fileRows"].as_u64().unwrap() as usize;
    let snapshot = DecodedBuffers { buffers: std::iter::repeat_with(Vec::new).take(rows).collect() };
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits { max_rows: rows, ..Default::default() })).unwrap();
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "fixture.buffers".into(), standard: "1".into(), subset: "*".into() };
    for limits in [
        SqliteDatabaseLimits { max_rows: rows, ..Default::default() },
        SqliteDatabaseLimits { max_value_bytes: 1, ..Default::default() },
        SqliteDatabaseLimits { max_tables: 1, ..Default::default() },
        SqliteDatabaseLimits { max_columns: 5, ..Default::default() },
        SqliteDatabaseLimits { max_schema_bytes: 1, ..Default::default() },
    ] {
        let mut bounded = database.clone();
        assert!(semio_framework_os_kernel::io::io_mechanism::attach_sqlite_snapshot_metadata(&mut bounded, &dialect, crate::sqlite_snapshot::SnapshotEncoding::Binary, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_err());
        assert_eq!(bounded, database);
    }
    let mut canceled = database.clone();
    assert!(semio_framework_os_kernel::io::io_mechanism::attach_sqlite_snapshot_metadata(&mut canceled, &dialect, crate::sqlite_snapshot::SnapshotEncoding::Binary, &mut SqliteSnapshotControl::new(&mut |_| false, Default::default())).is_err());
    assert_eq!(canceled, database);
    let mut admitted = database;
    let limits = SqliteDatabaseLimits { max_rows: file_rows, ..Default::default() };
    semio_framework_os_kernel::io::io_mechanism::attach_sqlite_snapshot_metadata(&mut admitted, &dialect, crate::sqlite_snapshot::SnapshotEncoding::Binary, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
    let bytes = crate::sqlite_snapshot::export_sqlite_database(&admitted, limits, &mut |_| true).unwrap();
    let script = "import {Database} from 'bun:sqlite';const x=JSON.parse(await Bun.stdin.text());const db=Database.deserialize(Buffer.from(x.bytes,'base64'));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok')throw Error('integrity');await Bun.write(Bun.stdout,String(db.query('SELECT (SELECT COUNT(*) FROM decoded_buffer)+(SELECT COUNT(*) FROM semio_snapshot) AS n').get().n));db.close();";
    let mut child = Command::new("bun").args(["-e", script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(serde_json::json!({"fixture":fixture,"bytes":protocol::bytes::encode_base64(&bytes)}).to_string().as_bytes()).unwrap();
    let result = child.wait_with_output().unwrap();
    assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
    assert_eq!(String::from_utf8(result.stdout).unwrap(), file_rows.to_string());
}
