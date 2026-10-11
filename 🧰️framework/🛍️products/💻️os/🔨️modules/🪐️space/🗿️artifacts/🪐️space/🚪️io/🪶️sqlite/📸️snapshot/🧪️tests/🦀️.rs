//! 🧪️ Actual builtin native factories and erased semantic snapshot boundaries.
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_framework_space_builtin_creation_and_reload_publish_io_without_manual_registration() {
    const MODE: &str = "SEMIO_SQLITE_SPACE_BUILTIN_MODE";
    let Ok(mode) = std::env::var(MODE) else {
        for mode in ["create", "reload", "retained"] {
            let output = std::process::Command::new(std::env::current_exe().unwrap())
                .arg("--exact")
                .arg(format!("{}::sqlite_snapshot_framework_space_builtin_creation_and_reload_publish_io_without_manual_registration",module_path!().split_once("::").expect("owning test module includes crate").1))
                .arg("--nocapture")
                .env(MODE, mode).output().unwrap();
            assert!(String::from_utf8_lossy(&output.stdout).contains("running 1 test"), "isolated builtin selector must execute its actual test: {}\n{}",String::from_utf8_lossy(&output.stdout),String::from_utf8_lossy(&output.stderr));
            assert!(output.status.success(), "./isolated {mode} builtin I/O failed: {}\n{}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
        }
        return;
    };
    let source = fixture();
    let mut envelope = store::create_document_envelope::<SpaceSnapshot, SpaceMutation>(S_SPACE_SCHEMA, "sqlite-builtin", source.clone(), None);
    if mode == "retained" { envelope.dialect = Some(dialect()); }
    let mut live = if mode == "retained" {
        let saved = store::print_document_pack(&envelope).await.unwrap();
        envelope.retire_unadopted();
        let history = store::os_spr::decode_history(&saved.spr, &store::os_spr::DecodeOptions::default()).await.unwrap();
        let initial_digest = store::artifact_initial_digest_of_pack(&saved.pack);
        let mut open = store::RetainedPersistedDocumentHydration::<SpaceSnapshot, SpaceMutation>::from_decoded_pack(
            source.clone(), saved.pack, initial_digest, history,
            semio_framework_artifact_reference::ArtifactRef { artifact_id: "sqlite-builtin".into(), dialect: dialect() }, None, S_SPACE_SCHEMA.into(),
            store::test_support::plain_document_store_owners(), semio_framework_job::OperationId(1), semio_framework_job::Generation(1),
            u64::MAX, store::PersistedDocumentHydrationTarget::Store { generation: 0 },
            store::os_spr::ActorId("actor:sqlite-retained-registration-fixture".into()),
        );
        let cancellation = semio_framework_job::root_cancel_token();
        let mut preview_sequence = 0;
        let mut ready = None;
        let grant = serde_json::from_value::<semio_framework_job::RetainedCloneGrant>(native_caller_policy()["bodyGrant"].clone()).unwrap();
        for _ in 0..100_000 {
            let mut retained_progress = semio_framework_job::RetainedCloneProgress::default();
            let mut context = semio_framework_job::StepContext::new(
                semio_framework_job::OperationId(1), semio_framework_job::Generation(1),
                semio_framework_job::StepBudget::new(256, u64::MAX, grant), cancellation.clone(),
                semio_framework_job::default_now_us, &mut preview_sequence, &mut retained_progress,
            );
            match open.step(&mut context, grant) {
                store::PersistedDocumentHydrationStep::Pending(_) => {},
                store::PersistedDocumentHydrationStep::Ready(store::PersistedDocumentHydrationOutput::Store(member)) => { ready = Some(*member); break; },
                _ => panic!("valid builtin retained history must hand off its exact store"),
            }
        }
        ready.expect("bounded retained opening must finish")
    } else {
        let envelope = if mode == "reload" {
        let saved = store::print_document_pack(&envelope).await.unwrap();
        envelope.retire_unadopted();
        store::parse_document_pack::<SpaceSnapshot, SpaceMutation>(&saved.pack, &saved.spr).await.unwrap().into_envelope()
        } else { envelope };
        let mut live = store::ArtifactStore::new(envelope, store::os_spr::ActorId("actor:sqlite-retained-registration-fixture".into())).await.unwrap();
        live.install_document_store_owners_exact(store::test_support::plain_document_store_owners());
        live
    };
    assert_eq!(live.envelope().dialect, if mode == "retained" { Some(dialect()) } else { None });
    let output = store::io::io_mechanism::io_export_sqlite_snapshot(&dialect(), live.snapshot_ref(), SnapshotEncoding::Binary, SqliteDatabaseLimits::default(), &mut |_|true).await;
    store::test_support::close_plain_test_store(&mut live);
    let bytes = output.expect("actual builtin construction publishes the authored SQLite coordinate atomically").value;
    assert_eq!(store::io::io_mechanism::io_import_sqlite_snapshot::<SpaceSnapshot>(&dialect(), &bytes, SqliteDatabaseLimits::default(), &mut |_|true).await.unwrap().value, source);
}

#[test]fn sqlite_snapshot_framework_space_same_control_retained_materialization_budget_is_cumulative(){let f=f();let case=&f["reconstructionBudget"];let mut source=fixture();source.name=case["textUnit"].as_str().unwrap().repeat(case["repeat"].as_u64().unwrap()as usize);let bytes=source.name.len();let database=<SpaceSnapshot as store::ArtifactSqliteSnapshot>::to_sqlite_database(&source,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();let limits=SqliteDatabaseLimits{max_value_bytes:bytes*case["ceilingNumerator"].as_u64().unwrap()as usize/case["ceilingDenominator"].as_u64().unwrap()as usize,..SqliteDatabaseLimits::default()};let mut progress=|_|true;let mut control=SqliteSnapshotControl::new(&mut progress,limits);let retained=<SpaceSnapshot as store::ArtifactSqliteSnapshot>::from_sqlite_database(&database,&mut control).unwrap();assert_eq!(retained.name,source.name);assert!(<SpaceSnapshot as store::ArtifactSqliteSnapshot>::from_sqlite_database(&database,&mut control).is_err(),"second live materialization must not reset the caller ownership ledger");}
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_framework_space_actual_typed_io_file_metadata_and_route(){crate::register_sqlite_snapshot().unwrap();let source=fixture();let limits=SqliteDatabaseLimits::default();for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let bytes=store::io::io_mechanism::io_export_sqlite_snapshot(&dialect(),&source,encoding,limits,&mut |_|true).await.unwrap().value;let database=import_sqlite_database(&bytes,limits,&mut |_|true).unwrap();assert_eq!(store::io::io_mechanism::sqlite_snapshot_metadata(&database).unwrap(),(dialect(),encoding));assert_eq!(store::io::io_mechanism::io_import_sqlite_snapshot::<SpaceSnapshot>(&dialect(),&bytes,limits,&mut |_|true).await.unwrap().value,source);}let route=store::io::io_mechanism::io_route(&dialect(),&semio_framework_artifact_reference::ArtifactDialect::from(store::io_schema::SQLITE_SNAPSHOT),1).await.unwrap().value;assert_eq!(route.hops.len(),1);}
#[test]fn sqlite_snapshot_framework_space_independent_surrogate_renumber_preserves_complete_logical_state(){let source=fixture();let codec=codec();let changed=oracle::renumber(&database(),f()["identityOffset"].as_i64().unwrap());for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let native={let mut original_input=Some(changed.clone());with_original_encode(&native_caller_policy(),SqliteDatabaseLimits::default(),&mut |_|true,&mut |_|true,|_|{},|original_sql,original_owner|(codec.import)(S_SPACE_SCHEMA,&dialect(),&mut original_input,encoding,original_sql,original_owner))}.unwrap().value;assert_eq!(decode(native),source);}}
use crate::*;
use semio_framework_os_kernel as store;
use store::sqlite_snapshot::*;
#[path="../../../../../../🧪️tests/🪶️sqlite/🔬️oracle/🦀️.rs"]mod oracle;
#[path="../../../../../../🧪️tests/🪶️sqlite/🫴️native-caller/🦀️.rs"]mod native_caller;
use native_caller::*;
const SQL:&str=include_str!("../🗄️.sql");
fn f()->serde_json::Value{serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap()}
fn fixture()->SpaceSnapshot{serde_json::from_value(f()["nativeSnapshot"].clone()).unwrap()}
fn database()->SqliteDatabase{oracle::database(&f(),SQL)}
fn dialect()->semio_framework_artifact_reference::ArtifactDialect{semio_framework_artifact_reference::ArtifactDialect{artifact_kind:S_SPACE_SCHEMA.into(),standard:"1".into(),subset:"*".into()}}
fn codec()->store::ArtifactSqliteSnapshotCodec{store::ArtifactCodec::bare::<SpaceSnapshot,SpaceMutation>(S_SPACE_SCHEMA).snapshot_sqlite.expect("actual builtin native factory requires its owned semantic SQLite capability")}
fn payload(value:&SpaceSnapshot,encoding:SnapshotEncoding)->store::io::IoPayload{match encoding{SnapshotEncoding::Binary=>store::io::IoPayload::Binary(store::ArtifactPack::encode_pack(value)),SnapshotEncoding::Text=>store::io::IoPayload::Text(store::ArtifactDsl::print_dsl(value))}}
fn decode(value:store::io::IoPayload)->SpaceSnapshot{match value{store::io::IoPayload::Binary(bytes)=>store::ArtifactPack::decode_pack(&bytes).unwrap(),store::io::IoPayload::Text(text)=>store::ArtifactDsl::parse_dsl(&text).unwrap()}}
#[test]fn sqlite_snapshot_framework_space_actual_factory_capability(){let codec=codec();assert_eq!(codec.snapshot_type,Some(std::any::TypeId::of::<SpaceSnapshot>()));}
#[test]fn sqlite_snapshot_framework_space_ordinary_native_complete_state(){let source=fixture();for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{assert_eq!(decode(payload(&source,encoding)),source);}}
#[test]fn sqlite_snapshot_framework_space_erased_both_directions_preserve_all_semantic_cells(){let source=fixture();let codec=codec();let limits=SqliteDatabaseLimits::default();let expected=database();for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let native=payload(&source,encoding);assert_eq!(with_original_decode(&native_caller_policy(),limits,&mut |_|true,&mut |_|true,|_|{},|original_sql,original_owner|(codec.export)(S_SPACE_SCHEMA,&dialect(),&native,original_sql,original_owner)).unwrap().value,expected);let native={let mut original_input=Some(expected.clone());with_original_encode(&native_caller_policy(),limits,&mut |_|true,&mut |_|true,|_|{},|original_sql,original_owner|(codec.import)(S_SPACE_SCHEMA,&dialect(),&mut original_input,encoding,original_sql,original_owner))}.unwrap().value;assert_eq!(decode(native),source);}}
#[test]fn sqlite_snapshot_framework_space_independent_sql_edit_retains_literal_typed_state(){let codec=codec();let edited=oracle::edit(&database(),&f());for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let native={let mut original_input=Some(edited.clone());with_original_encode(&native_caller_policy(),SqliteDatabaseLimits::default(),&mut |_|true,&mut |_|true,|_|{},|original_sql,original_owner|(codec.import)(S_SPACE_SCHEMA,&dialect(),&mut original_input,encoding,original_sql,original_owner))}.unwrap().value;let value=decode(native);assert!(value.users.iter().all(|row|row.name==f()["edit"]["value"].as_str().unwrap()));}}
#[test]fn sqlite_snapshot_framework_space_authored_schema_and_exact_semantic_row_admission(){let codec=codec();let source=fixture();let expected=database();let rows=f()["rowsTotal"].as_u64().unwrap()as usize;assert_eq!(expected.tables.iter().map(|table|table.rows.len()).sum::<usize>(),rows);for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let native=payload(&source,encoding);for maximum in[rows-1,rows]{let limits=SqliteDatabaseLimits{max_rows:maximum,..SqliteDatabaseLimits::default()};let output=with_original_decode(&native_caller_policy(),limits,&mut |_|true,&mut |_|true,|_|{},|original_sql,original_owner|(codec.export)(S_SPACE_SCHEMA,&dialect(),&native,original_sql,original_owner));let input={let mut original_input=Some(expected.clone());with_original_encode(&native_caller_policy(),limits,&mut |_|true,&mut |_|true,|_|{},|original_sql,original_owner|(codec.import)(S_SPACE_SCHEMA,&dialect(),&mut original_input,encoding,original_sql,original_owner))};assert_eq!(output.is_ok(),maximum==rows);assert_eq!(input.is_ok(),maximum==rows);}let limits=SqliteDatabaseLimits{max_schema_bytes:SQL.len()-1,..SqliteDatabaseLimits::default()};assert!(with_original_decode(&native_caller_policy(),limits,&mut |_|true,&mut |_|true,|_|{},|original_sql,original_owner|(codec.export)(S_SPACE_SCHEMA,&dialect(),&native,original_sql,original_owner)).is_err());assert!({let mut original_input=Some(expected.clone());with_original_encode(&native_caller_policy(),limits,&mut |_|true,&mut |_|true,|_|{},|original_sql,original_owner|(codec.import)(S_SPACE_SCHEMA,&dialect(),&mut original_input,encoding,original_sql,original_owner))}.is_err());}}
#[test]fn sqlite_snapshot_framework_space_all_owned_phases_cancel_and_wrong_dialect_refuses(){let codec=codec();let source=fixture();let expected=database();for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let native=payload(&source,encoding);for phase in[SqliteSnapshotPhase::DecodeNative,SqliteSnapshotPhase::ProjectSnapshot]{let mut seen=false;let result=with_original_decode(&native_caller_policy(),SqliteDatabaseLimits::default(),&mut |p|{if p.phase==phase{seen=true;false}else{true}},&mut |_|true,|_|{},|original_sql,original_owner|(codec.export)(S_SPACE_SCHEMA,&dialect(),&native,original_sql,original_owner));assert!(result.is_err()&&seen);}for phase in[SqliteSnapshotPhase::ReconstructSnapshot,SqliteSnapshotPhase::EncodeNative]{let mut seen=false;let result={let mut original_input=Some(expected.clone());with_original_encode(&native_caller_policy(),SqliteDatabaseLimits::default(),&mut |p|{if p.phase==phase{seen=true;false}else{true}},&mut |_|true,|_|{},|original_sql,original_owner|(codec.import)(S_SPACE_SCHEMA,&dialect(),&mut original_input,encoding,original_sql,original_owner))};assert!(result.is_err()&&seen);}let wrong=semio_framework_artifact_reference::ArtifactDialect{subset:"invented".into(),..dialect()};assert!(with_original_decode(&native_caller_policy(),SqliteDatabaseLimits::default(),&mut |_|true,&mut |_|true,|_|{},|original_sql,original_owner|(codec.export)(S_SPACE_SCHEMA,&wrong,&native,original_sql,original_owner)).is_err());}}

#[test]
fn sqlite_snapshot_framework_space_native_input_retained_materialization_uses_same_caller_ledger(){
 let corpus=f();let sample=&corpus["reconstructionBudget"];let ratio=&corpus["nativeRetention"];let mut source=fixture();source.name=sample["textUnit"].as_str().unwrap().repeat(sample["repeat"].as_u64().unwrap()as usize);
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let payload=payload(&source,encoding);let limits=SqliteDatabaseLimits::default();let mut charged=0usize;
  let first=with_original_decode(&native_caller_policy(),limits,&mut |_|true,&mut |_|true,|_|{},|original_sql,original_owner|{let value=<SpaceSnapshot as store::ArtifactSqliteSnapshot>::decode_sqlite_snapshot_native(&payload,original_sql,original_owner);charged=limits.max_value_bytes-original_sql.reconstruction_remaining_bytes().unwrap();value}).unwrap();assert_eq!(first,source);<SpaceSnapshot as store::ArtifactSqliteSnapshot>::retire_sqlite_snapshot(first);assert!(charged>0,"native parser and retained typed fields must publish their actual cumulative ownership");
  let ceiling=charged.checked_mul(ratio["ceilingNumerator"].as_u64().unwrap()as usize).unwrap()/ratio["ceilingDenominator"].as_u64().unwrap()as usize;let limits=SqliteDatabaseLimits{max_value_bytes:ceiling,..limits};
  let (retained,second)=with_original_decode(&native_caller_policy(),limits,&mut |_|true,&mut |_|true,|_|{},|original_sql,original_owner|Ok::<_,semio_framework_value::ValueError>((<SpaceSnapshot as store::ArtifactSqliteSnapshot>::decode_sqlite_snapshot_native(&payload,original_sql,original_owner),<SpaceSnapshot as store::ArtifactSqliteSnapshot>::decode_sqlite_snapshot_native(&payload,original_sql,original_owner)))).unwrap();let retained=retained.unwrap();assert_eq!(retained,source);<SpaceSnapshot as store::ArtifactSqliteSnapshot>::retire_sqlite_snapshot(retained);assert!(second.is_err(),"second retained native input must not reset the same caller ledger");
 }
}

#[path="../../../../../../🧪️tests/🪶️sqlite/📏️preflight/🦀️.rs"]
mod public_preflight;
#[test]
fn sqlite_snapshot_framework_space_public_borrowed_native_preflight_admits_actual_owner(){
 let facet:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/📏️preflight/🔣️.json")).unwrap();
 let mut short=fixture();short.name=facet["textUnit"].as_str().unwrap().into();
 let mut long=short.clone();long.name=facet["textUnit"].as_str().unwrap().repeat(usize::try_from(facet["repeat"].as_u64().unwrap()).unwrap());
 let grant:semio_framework_value::RetainedCloneGrant=serde_json::from_value(facet["callerGrant"].clone()).unwrap();let maximum=facet["nativeMaximumBytes"].as_u64().unwrap()as usize;let mut receive=|event:semio_framework_value::native_decoding::NativeDecodeProgress|{assert!(event.owned_bytes<=maximum);true};let mut emit=|event:semio_framework_value::native_encoding::NativeEncodeProgress|{assert!(event.owned_bytes<=maximum);true};let mut receive_recipient=semio_framework_value::native_decoding::NativeDecodeRetirementRecipient::new();let mut emit_recipient=semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();let mut decode=semio_framework_value::NativeDecodeControl::new(maximum,&mut receive);let mut encode=semio_framework_value::NativeEncodeControl::new(maximum,&mut emit);decode.install_retirement_recipient(&mut receive_recipient).unwrap();encode.install_retirement_recipient(&mut emit_recipient).unwrap();let mut original_io=store::io::io_mechanism::IoRunControl::new(&mut decode,&mut encode,grant);
 public_preflight::verify(&short,&long,SQL,usize::try_from(facet["rows"].as_u64().unwrap()).unwrap(),long.name.len(),usize::try_from(facet["cancelAt"].as_u64().unwrap()).unwrap(),|operation|crate::test_allocation::observe(operation),&mut original_io);
}

#[test]
fn sqlite_snapshot_framework_space_from_sqlite_full_owned_backing_is_paid(){
 use semio_framework_value::ValueRefusalKind;
 let demand:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/💰️reconstruction/🔣️.json")).unwrap();
 let text=demand["textUnit"].as_str().unwrap().repeat(usize::try_from(demand["repeat"].as_u64().unwrap()).unwrap());
 assert_eq!(text.len(),usize::try_from(demand["utf8Bytes"].as_u64().unwrap()).unwrap());
 let mut source=fixture();source.name=text.clone();
 let mut database=database();
 let table=database.tables.iter_mut().find(|table|table.name==demand["table"].as_str().unwrap()).unwrap();
 table.rows[0].values[2]=SqliteValue::Text(text);
 let unchanged=database.clone();
 let defaults=SqliteDatabaseLimits::default();
 let mut callback=|_|true;
 let mut control=SqliteSnapshotControl::new(&mut callback,defaults);
 let(result,requested)=crate::test_allocation::observe(||<SpaceSnapshot as store::ArtifactSqliteSnapshot>::from_sqlite_database(&database,&mut control));
 assert_eq!(result.unwrap(),source);
 assert!(requested>0,"the complete actual owner requires real backing");
 assert_eq!(defaults.max_allocation_bytes-control.allocation_remaining_bytes(),requested,"./full SQL schema/frontier/retained field requests must be admitted by the actual caller");
 let mut callback=|_|true;
 let mut exact=SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits{max_allocation_bytes:requested,..defaults});
 let(result,exact_requests)=crate::test_allocation::observe(||<SpaceSnapshot as store::ArtifactSqliteSnapshot>::from_sqlite_database(&database,&mut exact));
 assert_eq!(result.unwrap(),source);assert_eq!(exact_requests,requested);assert_eq!(exact.allocation_remaining_bytes(),0);
 for maximum in[requested-1,0]{
  let mut callback=|_|true;let mut denied=SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits{max_allocation_bytes:maximum,..defaults});
  let(result,observed)=crate::test_allocation::observe(||<SpaceSnapshot as store::ArtifactSqliteSnapshot>::from_sqlite_database(&database,&mut denied));
  let error=result.unwrap_err();assert_eq!(error.kind,ValueRefusalKind::OwnershipLimit);
  assert_eq!(observed,maximum-denied.allocation_remaining_bytes()+diagnostic_capacity(&error),"a refused producer retains only paid requests and its exact returned diagnostic backing");
 }
 let maximum=requested.checked_mul(2).unwrap();let mut callback=|_|true;
 let mut cumulative=SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits{max_allocation_bytes:maximum,..defaults});
 for _ in 0..2{assert_eq!(<SpaceSnapshot as store::ArtifactSqliteSnapshot>::from_sqlite_database(&database,&mut cumulative).unwrap(),source);}
 assert_eq!(cumulative.allocation_remaining_bytes(),0);
 assert_eq!(<SpaceSnapshot as store::ArtifactSqliteSnapshot>::from_sqlite_database(&database,&mut cumulative).unwrap_err().kind,ValueRefusalKind::OwnershipLimit);
 assert_eq!(cumulative.allocation_remaining_bytes(),0,"retired transient scratch must not refund admitted backing");
 let mut start=false;let mut callback=|_:SqliteSnapshotProgress|{start=true;false};let mut canceled=SqliteSnapshotControl::new(&mut callback,defaults);
 let(result,observed)=crate::test_allocation::observe(||<SpaceSnapshot as store::ArtifactSqliteSnapshot>::from_sqlite_database(&database,&mut canceled));
 let error=result.unwrap_err();assert_eq!(error.kind,ValueRefusalKind::Canceled);assert_eq!(canceled.allocation_remaining_bytes(),defaults.max_allocation_bytes);assert_eq!(observed,diagnostic_capacity(&error));drop(canceled);assert!(start);
 let mut interior=false;let field_bytes=source.name.len();let cancel_at=usize::try_from(demand["cancelAt"].as_u64().unwrap()).unwrap();
 let mut callback=|event:SqliteSnapshotProgress|{let stop=event.phase==SqliteSnapshotPhase::ReconstructSnapshot&&event.total==field_bytes&&event.completed>=cancel_at&&event.completed<event.total;interior|=stop;!stop};
 let mut canceled=SqliteSnapshotControl::new(&mut callback,defaults);
 let(result,observed)=crate::test_allocation::observe(||<SpaceSnapshot as store::ArtifactSqliteSnapshot>::from_sqlite_database(&database,&mut canceled));
 let error=result.unwrap_err();assert_eq!(error.kind,ValueRefusalKind::Canceled);
 let paid=defaults.max_allocation_bytes-canceled.allocation_remaining_bytes();assert!(paid>=field_bytes,"interior literal cancellation keeps its admitted full backing");
 assert_eq!(observed,paid+diagnostic_capacity(&error),"canceled owner construction retains its actual paid ledger");drop(canceled);assert!(interior);
 assert_eq!(database,unchanged,"borrowed full SQL input must remain unchanged for all outcomes");
}

fn semantic_role_source(index:usize,text:&str)->SpaceSnapshot{let mut source=fixture();match index{0=>{source.schema=text.into();},1=>{source.name=text.into();},2=>{source.users[0].id=text.into();},3=>{source.users[0].name=text.into();},4=>{source.users[1].avatar=Some(text.into());},5=>{source.collections[0].id=text.into();},6=>{source.collections[0].name=text.into();},7=>{source.collections[0].document_id=text.into();},8=>{source.programs[0]=text.into();},9=>{source.extensions[0].extension_id=text.into();},10=>{source.extensions[0].version=text.into();},11=>{source.extensions[0].source_uri=text.into();},12=>{source.extensions[0].package_hash=text.into();},_=>panic!("closed thirteen authored text roles")}source}

#[test]
fn sqlite_snapshot_framework_space_complete_native_semantic_text_roles_and_columns(){
 use store::ArtifactSqliteSnapshot;use std::io::Write;use std::process::{Command,Stdio};
 let plan:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🛂️semantic/🔣️.json")).unwrap();
 
 let defaults=SqliteDatabaseLimits::default();let text=plan["text"].as_str().unwrap();assert_eq!(text.len(),plan["textBytes"].as_u64().unwrap()as usize);
 for(index,sample)in plan["cases"].as_array().unwrap().iter().enumerate(){
  let source=semantic_role_source(index,text);let bytes=sample["semanticBytes"].as_u64().unwrap()as usize;
  let database=source.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,defaults)).unwrap();
  let file=export_sqlite_database(&database,defaults,&mut |_|true).unwrap();
  let script=r###"import{Database}from "bun:sqlite";const plan=JSON.parse(process.argv[1]),sample=JSON.parse(process.argv[2]);if(Buffer.byteLength(plan.text,"utf8")!==plan.textBytes)throw Error("independent UTF8 extent");const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));const quote=v=>'"'+v.replaceAll('"','""')+'"';try{if(db.query("PRAGMA integrity_check").get().integrity_check!=="ok"||db.query("PRAGMA foreign_key_check").all().length)throw Error("independent physical integrity");let rows=0,bytes=0;for(const[table,count]of Object.entries(plan.tableRows)){const fields=db.query("PRAGMA table_info("+quote(table)+")").all().map(v=>v.name);const cells=fields.map(v=>{const f=quote(v);return "CASE typeof("+f+") WHEN 'integer' THEN 8 WHEN 'real' THEN 8 WHEN 'text' THEN length(CAST("+f+" AS BLOB)) WHEN 'blob' THEN length("+f+") ELSE 0 END";}).join("+");const actual=db.query("SELECT COUNT(*) AS rows,COALESCE(SUM("+cells+"),0) AS bytes FROM "+quote(table)).get();if(actual.rows!==count)throw Error("authored row extent "+table);rows+=actual.rows;bytes+=actual.bytes;}if(rows!==plan.rows||bytes!==sample.semanticBytes)throw Error("independent complete cell extent "+JSON.stringify({rows,bytes}));}finally{db.close();}"###;
  let mut child=Command::new("bun").args(["-e",script]).arg(plan.to_string()).arg(sample.to_string()).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
  child.stdin.take().unwrap().write_all(&file).unwrap();let result=child.wait_with_output().unwrap();assert!(result.status.success(),"{}: {}",sample["id"],String::from_utf8_lossy(&result.stderr));
  for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
   let input=payload(&source,encoding);
   let retained=with_original_decode(&native_caller_policy(),defaults,&mut |_|true,&mut |_|true,|_|{},|original_sql,original_owner|SpaceSnapshot::decode_sqlite_snapshot_native(&input,original_sql,original_owner)).unwrap();assert_eq!(retained,source);retained.retire_sqlite_snapshot();
   let short=SqliteDatabaseLimits{max_columns:8-1,..defaults};
   assert!(with_original_decode(&native_caller_policy(),short,&mut |_|true,&mut |_|true,|_|{},|original_sql,original_owner|SpaceSnapshot::decode_sqlite_snapshot_native(&input,original_sql,original_owner)).is_err(),"{} borrowed complete native columns must refuse before typed materialization",sample["id"]);
   for maximum in[bytes-1,bytes]{
    let limits=SqliteDatabaseLimits{max_value_bytes:maximum,..defaults};
    let output=with_original_encode(&native_caller_policy(),limits,&mut |_|true,&mut |_|true,|_|{},|original_sql,original_owner|source.encode_sqlite_snapshot_native(encoding,original_sql,original_owner));
    assert_eq!(output.is_ok(),maximum==bytes,"{} complete typed semantic output cells",sample["id"]);
    let preflight=source.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits));
    assert_eq!(preflight.is_ok(),maximum==bytes,"{} complete borrowed semantic preflight cells",sample["id"]);
   }
  }
  source.retire_sqlite_snapshot();
 }
}

#[path="../📏️preflight/🫳️borrowed/🦀️.rs"]mod exact_borrowed_wire;
#[test]
fn sqlite_snapshot_framework_space_actual_borrowed_wire_matches_public_file_for_every_text_role(){
 use semio_framework_value::NativeEncodeControl;
 let plan:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🛂️semantic/🔣️.json")).unwrap();
 for(index,_)in plan["cases"].as_array().unwrap().iter().enumerate(){
  let source=semantic_role_source(index,plan["text"].as_str().unwrap());
  for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
   let actual=payload(&source,encoding);let length=match actual{store::io::IoPayload::Binary(bytes)=>bytes.len(),store::io::IoPayload::Text(text)=>text.len()};
   let component=match encoding{SnapshotEncoding::Binary=>store::semio_format::Component::Pack,SnapshotEncoding::Text=>store::semio_format::Component::Dsl};
   let prefix=store::semio_format::declared_envelope_prefix_len(SpaceSnapshot::__DSL_ENVELOPE_ID,component,1).unwrap();
   let body=length.checked_sub(prefix).unwrap();let mut callback=|_|true;let mut native=NativeEncodeControl::new(SqliteDatabaseLimits::default().max_allocation_bytes,&mut callback);
   let measured=match encoding{
    SnapshotEncoding::Text=>semio_framework_dsl_record::measure_print_borrowed(&source,&exact_borrowed_wire::spec(),body,&mut native),
    SnapshotEncoding::Binary=>{let mut options=store::os_pack::record::EncodeOptions::default();options.limits.max_file_len=body as u64;store::os_pack::record::measure_document_borrowed(&source,&exact_borrowed_wire::spec(),&options,&mut native)},
   }.expect("./same authored Table/Statements metadata must measure the actual ordinary public native file");
   assert_eq!(measured,body,"each authored role uses the canonical actual producer");
   let mut callback=|_|true;let mut native=NativeEncodeControl::new(SqliteDatabaseLimits::default().max_allocation_bytes,&mut callback);
   let short=match encoding{
    SnapshotEncoding::Text=>semio_framework_dsl_record::measure_print_borrowed(&source,&exact_borrowed_wire::spec(),body-1,&mut native),
    SnapshotEncoding::Binary=>{let mut options=store::os_pack::record::EncodeOptions::default();options.limits.max_file_len=(body-1)as u64;store::os_pack::record::measure_document_borrowed(&source,&exact_borrowed_wire::spec(),&options,&mut native)},
   };assert!(short.is_err(),"one canonical output octet short must refuse");
  }
  <SpaceSnapshot as store::ArtifactSqliteSnapshot>::retire_sqlite_snapshot(source);
 }
}

#[test]
fn sqlite_snapshot_framework_space_borrowed_semantic_gate_uses_closed_exact_cells_before_typed_construction(){
 use store::ArtifactSqliteSnapshot;
 let plan:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🛂️semantic/🔣️.json")).unwrap();let defaults=SqliteDatabaseLimits::default();
 for(index,sample)in plan["cases"].as_array().unwrap().iter().enumerate(){
  let source=semantic_role_source(index,plan["text"].as_str().unwrap());let bytes=sample["semanticBytes"].as_u64().unwrap()as usize;let rows=plan["rows"].as_u64().unwrap()as usize;let schema=SpaceSnapshot::SQLITE_SCHEMA.len();
  for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let input=payload(&source,encoding);
   for(field,maximum,success)in[("bytes",bytes,true),("bytes",bytes-1,false),("rows",rows,true),("rows",rows-1,false),("columns",8,true),("columns",7,false),("schema",schema,true),("schema",schema-1,false)]{
    let limits=match field{"bytes"=>SqliteDatabaseLimits{max_value_bytes:maximum,..defaults},"rows"=>SqliteDatabaseLimits{max_rows:maximum,..defaults},"columns"=>SqliteDatabaseLimits{max_columns:maximum,..defaults},"schema"=>SqliteDatabaseLimits{max_schema_bytes:maximum,..defaults},_=>unreachable!()};
    let admitted=with_original_decode(&native_caller_policy(),defaults,&mut |_|true,&mut |_|true,|_|{},|original_sql,original_owner|store::decode_sqlite_snapshot_record_native(&input,SpaceSnapshot::__DSL_ENVELOPE_ID,SpaceSnapshot::__dsl_spec_producer(),|record, snapshot_output, native,_body| { let constructed: Result<_, semio_framework_value::ValueError> = (|| {crate::io::sqlite::snapshot::admission::record(record,native,limits)})(); *snapshot_output = Some(constructed?); Ok(()) },original_sql,original_owner));assert_eq!(admitted.is_ok(),success,"{} isolated borrowed {} grant before typed construction",sample["id"],field);
   }
  }source.retire_sqlite_snapshot();
 }
}
