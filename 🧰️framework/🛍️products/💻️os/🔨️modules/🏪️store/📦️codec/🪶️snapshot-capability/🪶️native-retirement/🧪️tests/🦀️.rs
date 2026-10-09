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
    fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,snapshot_encoding_owner:&mut crate::os_store::NativeSnapshotEncodeOwner<'_,'_>)->Result<crate::io_schema::IoPayload,ValueError>{
        use semio_framework_value::native_encoding::NativeEncodeProgress;
        let before=snapshot_encoding_owner.native().owned_bytes();let grant=snapshot_encoding_owner.remaining_grant();let mut performed=Default::default();
        let result=control.allocation_stage_native(SqliteSnapshotPhase::EncodeNative,|remaining,progress|{let mut callback=|event:NativeEncodeProgress|progress(event.completed,event.total);let result=snapshot_encoding_owner.native().scoped_observer(&mut callback,|native|native.scoped_maximum(before.saturating_add(remaining),|native|{let mut child=crate::os_store::NativeSnapshotEncodeOwner::new(native,grant);let result=child.receive::<crate::io_schema::IoPayload,crate::io_schema::IoPayload>(|slot,native,body|{
         if self.value==-2{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"owner encoder refusal"))}
         let extent=match encoding{SnapshotEncoding::Binary=>8,SnapshotEncoding::Text=>20};body.admit_frontier(semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:extent,maximum_capacity_bytes:extent,maximum_release_bytes:0,maximum_depth:1})?;
         *slot=Some(match encoding{SnapshotEncoding::Binary=>crate::io_schema::IoPayload::Binary(Vec::new()),SnapshotEncoding::Text=>crate::io_schema::IoPayload::Text(String::new())});
         let copied=native.scoped_stage(|native|{native.begin_stage(1)?;match slot.as_mut().unwrap(){crate::io_schema::IoPayload::Binary(output)=>{native.charge(8)?;output.try_reserve_exact(8).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"retained scalar output allocation failed"))?;output.extend_from_slice(&self.value.to_le_bytes());},crate::io_schema::IoPayload::Text(output)=>{native.charge(20)?;output.try_reserve_exact(20).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"retained scalar text allocation failed"))?;use std::fmt::Write;write!(output,"{}",self.value).map_err(|_|ValueError::literal(ValueRefusalKind::InvalidValue,"retained scalar formatting failed"))?;}}native.step()});let(length,capacity)=match slot.as_ref().unwrap(){crate::io_schema::IoPayload::Text(text)=>(text.len(),text.capacity()),crate::io_schema::IoPayload::Binary(bytes)=>(bytes.len(),bytes.capacity())};body.record_progress(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:length,retained_capacity_bytes:capacity,released_bytes:0})?;copied?;Ok(slot.take().unwrap())
        });performed=child.progress();result}));(result,snapshot_encoding_owner.native().owned_bytes().saturating_sub(before))});snapshot_encoding_owner.record_progress(performed)?;result?
    }
    fn decode_sqlite_snapshot_native(payload:&crate::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>,snapshot_decoding_owner:&mut crate::os_store::NativeSnapshotDecodeOwner<'_,'_>)->Result<Self,ValueError>{
        control.checkpoint(SqliteSnapshotPhase::DecodeNative,0,0)?;snapshot_decoding_owner.native().checkpoint()?;
        control.check_value_bytes(std::mem::size_of::<Self>())?;
        let length=match payload{crate::io_schema::IoPayload::Binary(bytes)=>bytes.len(),crate::io_schema::IoPayload::Text(text)=>text.len()};
        if length>control.limits().max_file_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"retained scalar exceeds file byte limit"));}
        let value=match payload{
            crate::io_schema::IoPayload::Binary(bytes)=>i64::from_le_bytes(bytes.as_slice().try_into().map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"expected exact integer word"))?),
            crate::io_schema::IoPayload::Text(text)if text.len()<=20=>text.parse::<i64>().map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"expected bounded integer text"))?,
            _=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"expected bounded integer text")),
        };
        snapshot_decoding_owner.native().checkpoint()?;Ok(Self{value,retired:false})
    }
    fn retire_sqlite_snapshot(mut self) { self.retired = true; RETIREMENTS.with(|count| count.set(count.get() + 1)); }
    fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase,ValueError> { control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, 1)?; Ok(value_database(self.value)) }
    fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self,ValueError> { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 0, 1)?; Ok(Self { value: database.table("retained_value")?.single_row()?.integer(1)?, retired: false }) }
    fn preflight_sqlite_snapshot_encoding(&self, _: SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<(),ValueError> { let mut bound = NativeEncodingBound::new(control)?; bound.add(32)?; bound.finish() }
    fn validate_sqlite_snapshot_subset(&self, dialect: &semio_framework_artifact_reference::ArtifactDialect, _: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> crate::io_schema::IoResult<()> { control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, 1).map_err(crate::io_schema::IoError::from_value_error)?; if dialect.subset == "reject" { return Err(crate::io_schema::IoError::from_value_error(ValueError::new(ValueRefusalKind::InvalidValue,"declared semantic owner refusal"))); } Ok(crate::io_schema::IoOutcome::clean(())) }
}

#[test]
fn sqlite_snapshot_native_retirement_covers_success_cancellation_and_refusal() {
 let snapshot_policy:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../🔨️modules/🚪️io/⏱️control/🛫️snapshot/🧫️fixtures/🔣️.json")).unwrap();let snapshot_caller_grant:semio_framework_value::RetainedCloneGrant=serde_json::from_value(snapshot_policy["cases"][0]["grant"].clone()).unwrap();let snapshot_native_maximum=snapshot_policy["nativeMaximumBytes"].as_u64().unwrap()as usize;let snapshot_original_live=std::cell::Cell::new(true);
let mut snapshot_decoding_progress=|_|snapshot_original_live.get();let mut snapshot_decoding_recipient=semio_framework_value::native_decoding::NativeDecodeRetirementRecipient::new();let mut snapshot_decoding_native=semio_framework_value::NativeDecodeControl::new(snapshot_native_maximum,&mut snapshot_decoding_progress);snapshot_decoding_native.install_retirement_recipient(&mut snapshot_decoding_recipient).unwrap();let mut snapshot_decoding_owner=crate::os_store::NativeSnapshotDecodeOwner::new(&mut snapshot_decoding_native,snapshot_caller_grant);
let mut snapshot_encoding_progress=|_|snapshot_original_live.get();let mut snapshot_encoding_recipient=semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();let mut snapshot_encoding_native=semio_framework_value::NativeEncodeControl::new(snapshot_native_maximum,&mut snapshot_encoding_progress);snapshot_encoding_native.install_retirement_recipient(&mut snapshot_encoding_recipient).unwrap();let mut snapshot_encoding_owner=crate::os_store::NativeSnapshotEncodeOwner::new(&mut snapshot_encoding_native,snapshot_caller_grant);

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
                (codec.export)("fixture.retained/v1", &dialect, &payload, &mut control,&mut snapshot_decoding_owner).map(|_| ())
            } else {
                let encoding = if operation.ends_with("binary") { SnapshotEncoding::Binary } else { SnapshotEncoding::Text };
                (codec.import)("fixture.retained/v1", &dialect, value_database(value), encoding, &mut control,&mut snapshot_encoding_owner).map(|_| ())
            }
        }));
        assert!(result.is_ok(), "{} skipped owner retirement", case["id"]);
        assert_eq!(result.unwrap().is_ok(), case["succeeds"].as_bool().unwrap(), "{}", case["id"]);
        RETIREMENTS.with(|count| assert_eq!(count.get() as u64, case["retirements"].as_u64().unwrap(), "{}", case["id"]));
    }

drop(snapshot_decoding_owner);while snapshot_decoding_native.has_retirement_owner(){snapshot_decoding_native.close_retirement_recipient(snapshot_caller_grant).unwrap();}
drop(snapshot_encoding_owner);while snapshot_encoding_native.has_retirement_owner(){snapshot_encoding_native.close_retirement_recipient(snapshot_caller_grant).unwrap();}
}
