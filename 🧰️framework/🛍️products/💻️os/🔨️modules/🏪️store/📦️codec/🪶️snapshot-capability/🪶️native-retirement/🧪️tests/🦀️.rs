use semio_framework_value::{ValueError,ValueRefusalKind};
use super::*;
use crate::sqlite_snapshot::{SqliteDatabase, SqliteDatabaseLimits, SqliteRow, SqliteValue, SqliteSnapshotControl, SqliteSnapshotPhase, SnapshotEncoding, artifact::NativeEncodingBound};

struct RetainedSnapshot { value: i64, retired: bool }
std::thread_local! { static RETIREMENTS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) }; }

impl Drop for RetainedSnapshot {
    fn drop(&mut self) { assert!(self.retired, "retained snapshot requires its declared owner retirement"); }
}

struct RetainedCursor { original:Option<RetainedSnapshot>, remaining:usize }
impl semio_framework_value::retirement::RetirementCursor for RetainedCursor{
 fn close_step(&mut self,grant:semio_framework_value::RetainedCloneGrant)->semio_framework_value::retirement::RetirementStep{use semio_framework_value::retirement::RetirementStep;if grant.maximum_items==0{return RetirementStep::BudgetExhausted}if self.remaining>0{let bytes=grant.maximum_copy_bytes.min(self.remaining);if bytes==0{return RetirementStep::BudgetExhausted}self.remaining-=bytes;return RetirementStep::ProcessedBytes(bytes)}if let Some(mut original)=self.original.take(){original.retired=true;RETIREMENTS.with(|count|count.set(count.get()+1));drop(original)}RetirementStep::Complete}
 fn terminal_is_empty(&self)->bool{self.original.is_none()&&self.remaining==0}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(usize::from(self.remaining>0))}
 fn next_close_byte_demand(&self)->Option<usize>{Some(0)}
 fn next_birth_bytes(&self,_:usize)->Option<usize>{Some(0)}
 fn terminal_release_bytes(&self)->Option<usize>{Some(std::mem::size_of::<Self>())}
}
impl semio_framework_value::retirement::RetireOwned for RetainedSnapshot{
 fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{Box::new(RetainedCursor{original:Some(self),remaining:std::mem::size_of::<Self>()})}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<RetainedCursor>())}
 fn controlled_retirement_supported()->bool{true}
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

fn validate_retained_subset(dialect:&semio_framework_artifact_reference::ArtifactDialect,control:&mut SqliteSnapshotControl<'_>)->crate::io_schema::IoResult<()>{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1).map_err(crate::io_schema::IoError::from_value_error)?;match dialect.subset.as_str(){"*"=>Ok(crate::io_schema::IoOutcome::clean(())),"reject"=>Err(crate::io_schema::IoError::from_value_error(ValueError::literal(ValueRefusalKind::InvalidValue,"declared semantic owner refusal"))),_=>Err(crate::io_schema::IoError::from_value_error(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"retained scalar has no declared subset validator")))}}

impl ArtifactSqliteSnapshot for RetainedSnapshot {
    const SQLITE_SCHEMA: &'static str = include_str!("../🧬️schema/🗄️.sql");
    fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,owner:&mut crate::os_store::NativeSnapshotEncodeOwner<'_,'_>)->Result<crate::io_schema::IoPayload,ValueError>{
     owner.receive::<crate::io_schema::IoPayload,crate::io_schema::IoPayload>(|slot,native,body|{
      control.checkpoint(SqliteSnapshotPhase::EncodeNative,0,1)?;if self.value==-2{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"owner encoder refusal"))}
      *slot=Some(match encoding{SnapshotEncoding::Binary=>crate::io_schema::IoPayload::Binary(Vec::new()),SnapshotEncoding::Text=>crate::io_schema::IoPayload::Text(String::new())});
      let before=native.owned_bytes();let result=match slot.as_mut().unwrap(){crate::io_schema::IoPayload::Binary(bytes)=>{body.allocate_encode_vec_into(native,8,bytes,1)?;body.admit_frontier(semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:8,maximum_depth:1,..Default::default()})?;bytes.extend_from_slice(&self.value.to_le_bytes());body.record_progress(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:8,..Default::default()})},crate::io_schema::IoPayload::Text(text)=>{let mut buffer=[0u8;20];let mut writer=std::io::Cursor::new(buffer.as_mut_slice());use std::io::Write;write!(writer,"{}",self.value).map_err(|_|ValueError::literal(ValueRefusalKind::InvalidValue,"retained integer formatting failed"))?;let length=writer.position()as usize;body.copy_encode_text_into(native,std::str::from_utf8(&buffer[..length]).unwrap(),text,1)}};
      control.admit_native_allocation_bytes(native.owned_bytes()-before)?;result?;Ok(slot.take().unwrap())
     })
    }
    fn decode_sqlite_snapshot_native(payload:&crate::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>,owner:&mut crate::os_store::NativeSnapshotDecodeOwner<'_,'_>)->Result<Self,ValueError>{
     owner.receive::<Self,Self>(|slot,native,body|{
      control.checkpoint(SqliteSnapshotPhase::DecodeNative,0,0)?;native.checkpoint()?;control.check_value_bytes(std::mem::size_of::<Self>())?;
      let value=match payload{crate::io_schema::IoPayload::Binary(bytes)=>i64::from_le_bytes(bytes.as_slice().try_into().map_err(|_|ValueError::literal(ValueRefusalKind::InvalidValue,"expected exact integer word"))?),crate::io_schema::IoPayload::Text(text)if text.len()<=20=>text.parse::<i64>().map_err(|_|ValueError::literal(ValueRefusalKind::InvalidValue,"expected bounded integer text"))?,_=>return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"expected bounded integer text"))};
      body.admit_frontier(semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:std::mem::size_of::<Self>(),maximum_depth:1,..Default::default()})?;*slot=Some(Self{value,retired:false});body.record_progress(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:std::mem::size_of::<Self>(),..Default::default()})?;Ok(slot.take().unwrap())
     })
    }
    fn to_sqlite_database_receiving(&self,control:&mut SqliteSnapshotControl<'_>,owner:&mut crate::os_store::NativeSnapshotDecodeOwner<'_,'_>)->Result<SqliteDatabase,ValueError>{
     use crate::sqlite_snapshot::{SqliteTable,artifact::receiving::{Port,Direction,Projection,Scalar,row,cell,push_row}};
     owner.receive::<Projection,SqliteDatabase>(|slot,native,body|{
      *slot=Some(Projection::new());let frame=slot.as_mut().unwrap();let mut port=Port{control,native:Direction::Decode(native),body};
      port.vector(1,&mut frame.database.tables,1)?;port.work(2,std::mem::size_of::<SqliteTable>())?;frame.database.tables.push(SqliteTable{name:String::new(),sql:String::new(),rows:Vec::new()});
      let table=&mut frame.database.tables[0];port.text("retained_value",&mut table.name,2)?;port.text(Self::SQLITE_SCHEMA.trim().trim_end_matches(';'),&mut table.sql,2)?;port.vector(1,&mut table.rows,2)?;
      row(frame,1,2,&mut port)?;cell(frame,Scalar::Integer(self.value),&mut port)?;push_row(frame,0,&mut port)?;Ok(std::mem::replace(&mut frame.database,SqliteDatabase{tables:Vec::new()}))
     })
    }
    fn from_sqlite_database_receiving(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,owner:&mut crate::os_store::NativeSnapshotEncodeOwner<'_,'_>)->Result<Self,ValueError>{
     owner.receive::<Self,Self>(|slot,native,body|{
      let mut port=crate::sqlite_snapshot::artifact::receiving::Port{control,native:crate::sqlite_snapshot::artifact::receiving::Direction::Encode(native),body};
      crate::sqlite_snapshot::artifact::receiving::validate_schema_sql(database,Self::SQLITE_SCHEMA,&mut port)?;port.work(1,std::mem::size_of::<Self>())?;
      let table=database.tables.first().unwrap();if table.rows.len()!=1{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"expected one retained scalar row"))}let value=crate::sqlite_snapshot::artifact::receiving::integer(&table.rows[0],1)?;*slot=Some(Self{value,retired:false});Ok(slot.take().unwrap())
     })
    }
    fn validate_sqlite_snapshot_subset_decoding(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,_database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,native:&mut crate::os_store::NativeSnapshotDecodeOwner<'_,'_>)->crate::io_schema::IoResult<()>{native.native().checkpoint().map_err(crate::io_schema::IoError::from_value_error)?;validate_retained_subset(dialect,control)}
    fn validate_sqlite_snapshot_subset_encoding(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,_database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,native:&mut crate::os_store::NativeSnapshotEncodeOwner<'_,'_>)->crate::io_schema::IoResult<()>{native.native().checkpoint().map_err(crate::io_schema::IoError::from_value_error)?;validate_retained_subset(dialect,control)}
    fn retire_sqlite_snapshot(mut self) { self.retired = true; RETIREMENTS.with(|count| count.set(count.get() + 1)); }
    fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase,ValueError> { control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, 1)?; Ok(value_database(self.value)) }
    fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self,ValueError> { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 0, 1)?; Ok(Self { value: database.table("retained_value")?.single_row()?.integer(1)?, retired: false }) }
    fn preflight_sqlite_snapshot_encoding(&self, _: SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<(),ValueError> { let mut bound = NativeEncodingBound::new(control)?; bound.add(32)?; bound.finish() }
    fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,_:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->crate::io_schema::IoResult<()>{validate_retained_subset(dialect,control)}
}

#[test]
fn sqlite_snapshot_codec_native_retirement_covers_success_cancellation_and_refusal() {
 let snapshot_policy:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let snapshot_caller_grant:semio_framework_value::RetainedCloneGrant=serde_json::from_value(snapshot_policy["original"]["grant"].clone()).unwrap();let snapshot_native_maximum=snapshot_policy["original"]["nativeMaximumBytes"].as_u64().unwrap()as usize;let snapshot_original_live=std::cell::Cell::new(true);
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
                (codec.import)("fixture.retained/v1", &dialect, &mut Some(value_database(value)), encoding, &mut control,&mut snapshot_encoding_owner).map(|_| ())
            }
        }));
        assert!(result.is_ok(), "{} skipped owner retirement", case["id"]);
        let result=result.unwrap();if let Err(error)=&result{eprintln!("[DEBUG] Original SQLite codec semantic case={} kind={:?} cause={}",case["id"],error.cause.kind,error.cause.message)}assert_eq!(result.is_ok(), case["succeeds"].as_bool().unwrap(), "{}: {:?}", case["id"],result);
        RETIREMENTS.with(|count| assert_eq!(count.get() as u64, case["retirements"].as_u64().unwrap(), "{}", case["id"]));
    }

drop(snapshot_decoding_owner);while snapshot_decoding_native.has_retirement_owner(){snapshot_decoding_native.close_retirement_recipient(snapshot_caller_grant).unwrap();}
drop(snapshot_encoding_owner);while snapshot_encoding_native.has_retirement_owner(){snapshot_encoding_native.close_retirement_recipient(snapshot_caller_grant).unwrap();}
 eprintln!("[DEBUG] Original SQLite codec completed twelve binary/text success, cancellation, subset and encoding refusal laws with genuine declared scalar retirement and independent Bun SQLite output");
}

#[test]
fn sqlite_snapshot_codec_original_input_and_retained_result_have_exact_custody(){
 let policy:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 let full:semio_framework_value::RetainedCloneGrant=serde_json::from_value(policy["original"]["grant"].clone()).unwrap();
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let codec=RetainedSnapshot::sqlite_codec();
 let dialect=semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"fixture.retained".into(),standard:"1".into(),subset:"*".into()};
 for case in fixture["custodyCases"].as_array().unwrap().iter().filter(|case|case["id"].as_str().unwrap().starts_with("import")){
  let mut input=Some(value_database(7));let pointer=input.as_ref().unwrap().tables.as_ptr();let original=std::cell::Cell::new(true);let mut observer=|_|original.get();
  let mut recipient=semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();let mut native=semio_framework_value::NativeEncodeControl::new(policy["original"]["nativeMaximumBytes"].as_u64().unwrap()as usize,&mut observer);native.install_retirement_recipient(&mut recipient).unwrap();
  let grant=semio_framework_value::RetainedCloneGrant{maximum_items:case["maximumItems"].as_u64().map(|value|value as usize).unwrap_or(full.maximum_items),maximum_release_bytes:case["maximumReleaseBytes"].as_u64().map(|value|value as usize).unwrap_or(full.maximum_release_bytes),maximum_copy_bytes:case["maximumCopyBytes"].as_u64().map(|value|value as usize).unwrap_or(full.maximum_copy_bytes),maximum_capacity_bytes:case["maximumCapacityBytes"].as_u64().map(|value|value as usize).unwrap_or(full.maximum_capacity_bytes),maximum_depth:case["maximumDepth"].as_u64().map(|value|value as usize).unwrap_or(full.maximum_depth),..full};
  let mut sql_observer=|_|true;let mut control=SqliteSnapshotControl::new(&mut sql_observer,Default::default());let mut owner=crate::os_store::NativeSnapshotEncodeOwner::new(&mut native,grant);
  let(result,born,released)=crate::test_allocation::observe_backing(||(codec.import)("fixture.retained/v1",&dialect,&mut input,SnapshotEncoding::Binary,&mut control,&mut owner));
  assert!(result.is_err());assert_eq!(born,owner.progress().retained_capacity_bytes,"{} actual frame and producer births",case["id"]);assert_eq!(released,owner.progress().released_bytes,"{} actual receiving releases",case["id"]);
  assert_eq!(input.is_some(),case["callerRetainsDatabase"].as_bool().unwrap());if let Some(database)=input.as_ref(){assert_eq!(database.tables.as_ptr(),pointer)}
  drop(owner);assert_eq!(native.has_retirement_owner(),case["recipientOccupied"].as_bool().unwrap());
  if native.has_retirement_owner(){let (_,denied_birth,denied_release)=crate::test_allocation::observe_backing(||native.close_retirement_recipient(semio_framework_value::RetainedCloneGrant{maximum_items:0,..full}).unwrap());assert_eq!((denied_birth,denied_release),(0,0));assert!(native.has_retirement_owner());while native.has_retirement_owner(){let(step,born,released)=crate::test_allocation::observe_backing(||native.close_retirement_recipient(full).unwrap());let progress=step.progress();assert_eq!(born,progress.retained_capacity_bytes);assert_eq!(released,progress.released_bytes);assert_ne!(progress,Default::default());}}
 }
 let case=fixture["custodyCases"].as_array().unwrap().iter().find(|case|case["id"]=="export-refused-release").unwrap();
 let mut callback=|_|true;let mut recipient=semio_framework_value::native_decoding::NativeDecodeRetirementRecipient::new();let mut native=semio_framework_value::NativeDecodeControl::new(policy["original"]["nativeMaximumBytes"].as_u64().unwrap()as usize,&mut callback);native.install_retirement_recipient(&mut recipient).unwrap();
 let mut sql_observer=|_|true;let mut control=SqliteSnapshotControl::new(&mut sql_observer,Default::default());let mut owner=crate::os_store::NativeSnapshotDecodeOwner::new(&mut native,semio_framework_value::RetainedCloneGrant{maximum_release_bytes:case["maximumReleaseBytes"].as_u64().unwrap()as usize,..full});let payload=crate::io_schema::IoPayload::Text("7".into());
 let(result,born,released)=crate::test_allocation::observe_backing(||(codec.export)("fixture.retained/v1",&dialect,&payload,&mut control,&mut owner));assert!(result.is_err());assert_eq!(born,owner.progress().retained_capacity_bytes);assert_eq!(released,owner.progress().released_bytes);drop(owner);assert!(native.has_retirement_owner());
 while native.has_retirement_owner(){let(step,born,released)=crate::test_allocation::observe_backing(||native.close_retirement_recipient(full).unwrap());assert_eq!(born,step.progress().retained_capacity_bytes);assert_eq!(released,step.progress().released_bytes);assert_ne!(step.progress(),Default::default());}
 let case=fixture["custodyCases"].as_array().unwrap().iter().find(|case|case["id"]=="original-semantic-error").unwrap();let dialect=semio_framework_artifact_reference::ArtifactDialect{subset:case["subset"].as_str().unwrap().into(),..dialect};let mut owner=crate::os_store::NativeSnapshotDecodeOwner::new(&mut native,full);
 let error=(codec.export)("fixture.retained/v1",&dialect,&payload,&mut control,&mut owner).unwrap_err();assert_eq!(error.cause.message,case["cause"].as_str().unwrap());assert!(matches!(error.cause.message,std::borrow::Cow::Borrowed("declared semantic owner refusal")));drop(owner);assert!(!native.has_retirement_owner());
 eprintln!("[DEBUG] Original SQLite codec retains original caller database backing on frame admission refusal and retains admitted typed frames/results under zero release authority; real allocator births/releases equal authentic receipts on every funded terminal turn");
}

#[derive(semio_framework_value::RetireOwned)]
struct OriginalRelocation{original:Vec<RetainedSnapshot>,replacement:Vec<RetainedSnapshot>}

struct DefaultReceivingSnapshot;
impl ArtifactSqliteSnapshot for DefaultReceivingSnapshot{
 const SQLITE_SCHEMA:&'static str="";
 fn to_sqlite_database(&self,_:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"no declared direct schema"))}
 fn from_sqlite_database(_:&SqliteDatabase,_:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"no declared direct schema"))}
}

#[test]
fn sqlite_snapshot_codec_default_refusals_preserve_zero_original_authority(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let grant:semio_framework_value::RetainedCloneGrant=serde_json::from_value(fixture["original"]["defaultRefusalGrant"].clone()).unwrap();let database=SqliteDatabase{tables:Vec::new()};let payload=crate::io_schema::IoPayload::Binary(Vec::new());let dialect=semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"default.receiving".into(),standard:"1".into(),subset:"*".into()};let snapshot=DefaultReceivingSnapshot;
 for case in fixture["defaultRefusalCases"].as_array().unwrap(){
  let live=!case["nativeCancelled"].as_bool().unwrap();let mut decode_observer=|_|live;let mut encode_observer=|_|live;let mut sql_observer=|_|true;let mut decode=semio_framework_value::NativeDecodeControl::new(case["nativeMaximumBytes"].as_u64().unwrap()as usize,&mut decode_observer);let mut encode=semio_framework_value::NativeEncodeControl::new(case["nativeMaximumBytes"].as_u64().unwrap()as usize,&mut encode_observer);let mut decoding=crate::os_store::NativeSnapshotDecodeOwner::new(&mut decode,grant);let mut encoding=crate::os_store::NativeSnapshotEncodeOwner::new(&mut encode,grant);let mut control=SqliteSnapshotControl::new(&mut sql_observer,Default::default());
  let(result,born,released)=crate::test_allocation::observe_backing(||match case["operation"].as_str().unwrap(){
   "projection"=>snapshot.to_sqlite_database_receiving(&mut control,&mut decoding).map(|_|()),"reconstruction"=>DefaultReceivingSnapshot::from_sqlite_database_receiving(&database,&mut control,&mut encoding).map(|_|()),"decode"=>DefaultReceivingSnapshot::decode_sqlite_snapshot_native(&payload,&mut control,&mut decoding).map(|_|()),"encode"=>snapshot.encode_sqlite_snapshot_native(SnapshotEncoding::Binary,&mut control,&mut encoding).map(|_|()),"preflight"=>snapshot.preflight_sqlite_snapshot_encoding(SnapshotEncoding::Binary,&mut control),"validate-decode"=>snapshot.validate_sqlite_snapshot_subset_decoding(&dialect,&database,&mut control,&mut decoding).map(|_|()).map_err(|error|error.cause),"validate-encode"=>snapshot.validate_sqlite_snapshot_subset_encoding(&dialect,&database,&mut control,&mut encoding).map(|_|()).map_err(|error|error.cause),_=>unreachable!()
  });
  let error=result.unwrap_err();assert_eq!(born,case["allocatedBytes"].as_u64().unwrap()as usize,"{}",case["operation"]);assert_eq!(released,case["releasedBytes"].as_u64().unwrap()as usize);assert_eq!(format!("{:?}",error.kind),case["kind"].as_str().unwrap());assert_eq!(error.message,case["cause"].as_str().unwrap());assert!(matches!(error.message,std::borrow::Cow::Borrowed(_)));assert_eq!(decoding.progress(),Default::default());assert_eq!(encoding.progress(),Default::default());drop(decoding);drop(encoding);assert_eq!(decode.owned_bytes(),case["ownedBytes"].as_u64().unwrap()as usize);assert_eq!(encode.owned_bytes(),case["ownedBytes"].as_u64().unwrap()as usize);
 }
 eprintln!("[DEBUG] Original SQLite codec default receiving refusals preserve nine declared exact literal/canceled outcomes with zero actual allocation/release and unchanged original native/body authority");
}

#[test]
fn sqlite_snapshot_codec_relocation_retains_every_original_across_partial_moves(){
 use crate::sqlite_snapshot::artifact::receiving::{Port,Direction,relocate_vector};
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let full:semio_framework_value::RetainedCloneGrant=serde_json::from_value(fixture["original"]["grant"].clone()).unwrap();
 for case in fixture["relocationCases"].as_array().unwrap(){
  RETIREMENTS.with(|count|count.set(0));let values=case["values"].as_array().unwrap();let mut input=Some(OriginalRelocation{original:values.iter().map(|value|RetainedSnapshot{value:value.as_i64().unwrap(),retired:false}).collect(),replacement:Vec::new()});
  let work=std::cell::Cell::new(0usize);let completed=std::cell::Cell::new(false);let mut sql_observer=|event:crate::sqlite_snapshot::SqliteSnapshotProgress|{if event.phase==SqliteSnapshotPhase::ProjectSnapshot&&event.total==0{work.set(work.get()+1);return case["refuseWorkAt"].as_u64().is_none_or(|limit|work.get()!=limit as usize)}true};let mut native_observer=|_|true;
  let mut recipient=semio_framework_value::native_decoding::NativeDecodeRetirementRecipient::new();let mut native=semio_framework_value::NativeDecodeControl::new(fixture["original"]["nativeMaximumBytes"].as_u64().unwrap()as usize,&mut native_observer);native.install_retirement_recipient(&mut recipient).unwrap();let mut owner=crate::os_store::NativeSnapshotDecodeOwner::new(&mut native,semio_framework_value::RetainedCloneGrant{maximum_release_bytes:0,..full});let mut control=SqliteSnapshotControl::new(&mut sql_observer,Default::default());
  let(result,born,released)=crate::test_allocation::observe_backing(||owner.receive::<OriginalRelocation,()>(|slot,native,body|{
   let transfer=semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:std::mem::size_of::<Option<OriginalRelocation>>(),maximum_depth:1,..Default::default()};body.admit_frontier(transfer)?;native.checkpoint()?;*slot=input.take();body.record_progress(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:transfer.maximum_copy_bytes,..Default::default()})?;
   let frame=slot.as_mut().unwrap();let result=relocate_vector(&mut frame.original,&mut frame.replacement,values.len()+1,1,&mut Port{control:&mut control,native:Direction::Decode(native),body});completed.set(result.is_ok());
   for(actual,expected)in[(&frame.original,&case["original"]),(&frame.replacement,&case["replacement"]) ]{let expected=expected.as_array().unwrap();assert_eq!(actual.len(),expected.len(),"{}",case["id"]);for(actual,expected)in actual.iter().zip(expected){assert_eq!(actual.value,expected.as_i64().unwrap(),"{}",case["id"]);assert!(!actual.retired)}}result
  }));
  assert!(result.is_err());assert_eq!(completed.get(),case["completed"].as_bool().unwrap());assert!(input.is_none());assert_eq!(born,owner.progress().retained_capacity_bytes);assert_eq!(released,owner.progress().released_bytes);assert_eq!(released,0);drop(owner);assert!(native.has_retirement_owner());
  while native.has_retirement_owner(){let(step,born,released)=crate::test_allocation::observe_backing(||native.close_retirement_recipient(full).unwrap());assert_eq!(born,step.progress().retained_capacity_bytes);assert_eq!(released,step.progress().released_bytes);assert_ne!(step.progress(),Default::default());}RETIREMENTS.with(|count|assert_eq!(count.get(),values.len()));
 }
 eprintln!("[DEBUG] Original SQLite vector relocation preserves every original scalar and both backing fields on partial move/reversal refusal; allocator births/releases equal original receipts and funded terminal turns retire all original owners");
}
