//! 🧪️ Actual builtin native factories and erased semantic snapshot boundaries.
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_framework_collection_builtin_creation_and_reload_publish_io_without_manual_registration() {
    const MODE: &str = "SEMIO_SQLITE_COLLECTION_BUILTIN_MODE";
    let Ok(mode) = std::env::var(MODE) else {
        for mode in ["create", "reload", "retained"] {
            let output = std::process::Command::new(std::env::current_exe().unwrap())
                .arg("--exact")
                .arg(format!("{}::sqlite_snapshot_framework_collection_builtin_creation_and_reload_publish_io_without_manual_registration",module_path!().split_once("::").expect("owning test module includes crate").1))
                .arg("--nocapture")
                .env(MODE, mode).output().unwrap();
            assert!(String::from_utf8_lossy(&output.stdout).contains("running 1 test"), "isolated builtin selector must execute its actual test: {}\n{}",String::from_utf8_lossy(&output.stdout),String::from_utf8_lossy(&output.stderr));
            assert!(output.status.success(), "./isolated {mode} builtin I/O failed: {}\n{}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
        }
        return;
    };
    let source = fixture();
    let mut envelope = store::create_document_envelope::<CollectionSnapshot, CollectionMutation>(S_COLLECTION_SCHEMA, "sqlite-builtin", source.clone(), None);
    if mode == "retained" { envelope.dialect = Some(dialect()); }
    let mut live = if mode == "retained" {
        let saved = store::print_document_pack(&envelope).await.unwrap();
        envelope.retire_unadopted();
        let history = store::os_spr::decode_history(&saved.spr, &store::os_spr::DecodeOptions::default()).await.unwrap();
        let initial_digest = store::artifact_initial_digest_of_pack(&saved.pack);
        let mut open = store::RetainedPersistedDocumentHydration::<CollectionSnapshot, CollectionMutation>::from_decoded_pack(
            source.clone(), saved.pack, initial_digest, history,
            semio_framework_artifact_reference::ArtifactRef { artifact_id: "sqlite-builtin".into(), dialect: dialect() }, None, S_COLLECTION_SCHEMA.into(),
            store::test_support::plain_document_store_owners(), semio_framework_job::OperationId(1), semio_framework_job::Generation(1),
            u64::MAX, store::PersistedDocumentHydrationTarget::Store { generation: 0 },
            store::os_spr::ActorId("actor:sqlite-retained-registration-fixture".into()),
        );
        let cancellation = semio_framework_job::root_cancel_token();
        let mut preview_sequence = 0;
        let mut ready = None;
        for _ in 0..100_000 {
            let mut context = semio_framework_job::StepContext::new(
                semio_framework_job::OperationId(1), semio_framework_job::Generation(1),
                semio_framework_job::StepBudget::new(256, u64::MAX), cancellation.clone(),
                semio_framework_job::default_now_us, &mut preview_sequence,
            );
            match open.step(&mut context) {
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
        store::parse_document_pack::<CollectionSnapshot, CollectionMutation>(&saved.pack, &saved.spr).await.unwrap().into_envelope()
        } else { envelope };
        let mut live = store::ArtifactStore::new(envelope, store::os_spr::ActorId("actor:sqlite-retained-registration-fixture".into())).await.unwrap();
        live.install_document_store_owners_exact(store::test_support::plain_document_store_owners());
        live
    };
    assert_eq!(live.envelope().dialect, if mode == "retained" { Some(dialect()) } else { None });
    let output = store::io::io_mechanism::io_export_sqlite_snapshot(&dialect(), live.snapshot_ref(), SnapshotEncoding::Binary, SqliteDatabaseLimits::default(), &mut |_|true).await;
    store::test_support::close_plain_test_store(&mut live);
    let bytes = output.expect("actual builtin construction publishes the authored SQLite coordinate atomically").value;
    assert_eq!(store::io::io_mechanism::io_import_sqlite_snapshot::<CollectionSnapshot>(&dialect(), &bytes, SqliteDatabaseLimits::default(), &mut |_|true).await.unwrap().value, source);
}

#[test]fn sqlite_snapshot_framework_collection_same_control_retained_materialization_budget_is_cumulative(){let f=f();let case=&f["reconstructionBudget"];let mut source=fixture();source.name=case["textUnit"].as_str().unwrap().repeat(case["repeat"].as_u64().unwrap()as usize);let bytes=source.name.len();let database=<CollectionSnapshot as store::ArtifactSqliteSnapshot>::to_sqlite_database(&source,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();let limits=SqliteDatabaseLimits{max_value_bytes:bytes*case["ceilingNumerator"].as_u64().unwrap()as usize/case["ceilingDenominator"].as_u64().unwrap()as usize,..SqliteDatabaseLimits::default()};let mut progress=|_|true;let mut control=SqliteSnapshotControl::new(&mut progress,limits);let retained=<CollectionSnapshot as store::ArtifactSqliteSnapshot>::from_sqlite_database(&database,&mut control).unwrap();assert_eq!(retained.name,source.name);assert!(<CollectionSnapshot as store::ArtifactSqliteSnapshot>::from_sqlite_database(&database,&mut control).is_err(),"second live materialization must not reset the caller ownership ledger");}
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_framework_collection_actual_typed_io_file_metadata_and_route(){crate::register_sqlite_snapshot().unwrap();let source=fixture();let limits=SqliteDatabaseLimits::default();for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let bytes=store::io::io_mechanism::io_export_sqlite_snapshot(&dialect(),&source,encoding,limits,&mut |_|true).await.unwrap().value;let database=import_sqlite_database(&bytes,limits,&mut |_|true).unwrap();assert_eq!(store::io::io_mechanism::sqlite_snapshot_metadata(&database).unwrap(),(dialect(),encoding));assert_eq!(store::io::io_mechanism::io_import_sqlite_snapshot::<CollectionSnapshot>(&dialect(),&bytes,limits,&mut |_|true).await.unwrap().value,source);}let route=store::io::io_mechanism::io_route(&dialect(),&semio_framework_artifact_reference::ArtifactDialect::from(store::io_schema::SQLITE_SNAPSHOT),1).await.unwrap().value;assert_eq!(route.hops.len(),1);}
#[test]fn sqlite_snapshot_framework_collection_independent_surrogate_renumber_preserves_complete_logical_state(){let source=fixture();let codec=codec();let changed=oracle::renumber(&database(),f()["identityOffset"].as_i64().unwrap());for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let native=(codec.import)(S_COLLECTION_SCHEMA,&dialect(),changed.clone(),encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;assert_eq!(decode(native),source);}}
use crate::*;
use semio_framework_os_kernel as store;
use store::sqlite_snapshot::*;
#[path="../../../../../../🧪️tests/🪶️sqlite/🔬️oracle/🦀️.rs"]mod oracle;
const SQL:&str=include_str!("../🗄️.sql");
fn f()->serde_json::Value{serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap()}
fn fixture()->CollectionSnapshot{{let f=f();let mut value:CollectionSnapshot=serde_json::from_value(f["nativeSnapshot"].clone()).unwrap();let mut sizes=f["blobSizes"].as_array().unwrap().iter();for entry in&mut value.entries{if let ArtifactBody::Blob{blob}=entry.body.as_mut(){blob.size=sizes.next().unwrap().as_str().unwrap().parse().unwrap();}}value}}
fn database()->SqliteDatabase{oracle::database(&f(),SQL)}
fn dialect()->semio_framework_artifact_reference::ArtifactDialect{semio_framework_artifact_reference::ArtifactDialect{artifact_kind:S_COLLECTION_SCHEMA.into(),standard:"1".into(),subset:"*".into()}}
fn codec()->store::ArtifactSqliteSnapshotCodec{store::ArtifactCodec::bare::<CollectionSnapshot,CollectionMutation>(S_COLLECTION_SCHEMA).snapshot_sqlite.expect("actual builtin native factory requires its owned semantic SQLite capability")}
fn payload(value:&CollectionSnapshot,encoding:SnapshotEncoding)->store::io::IoPayload{match encoding{SnapshotEncoding::Binary=>store::io::IoPayload::Binary(store::ArtifactPack::encode_pack(value)),SnapshotEncoding::Text=>store::io::IoPayload::Text(store::ArtifactDsl::print_dsl(value))}}
fn decode(value:store::io::IoPayload)->CollectionSnapshot{match value{store::io::IoPayload::Binary(bytes)=>store::ArtifactPack::decode_pack(&bytes).unwrap(),store::io::IoPayload::Text(text)=>store::ArtifactDsl::parse_dsl(&text).unwrap()}}
#[test]fn sqlite_snapshot_framework_collection_actual_factory_capability(){let codec=codec();assert_eq!(codec.snapshot_type,Some(std::any::TypeId::of::<CollectionSnapshot>()));}
#[test]fn sqlite_snapshot_framework_collection_ordinary_native_complete_state(){let source=fixture();for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{assert_eq!(decode(payload(&source,encoding)),source);}}
#[test]fn sqlite_snapshot_framework_collection_erased_both_directions_preserve_all_semantic_cells(){let source=fixture();let codec=codec();let limits=SqliteDatabaseLimits::default();let expected=database();for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let native=payload(&source,encoding);assert_eq!((codec.export)(S_COLLECTION_SCHEMA,&dialect(),&native,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap().value,expected);let native=(codec.import)(S_COLLECTION_SCHEMA,&dialect(),expected.clone(),encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap().value;assert_eq!(decode(native),source);}}
#[test]fn sqlite_snapshot_framework_collection_independent_sql_edit_retains_literal_typed_state(){let codec=codec();let edited=oracle::edit(&database(),&f());for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let native=(codec.import)(S_COLLECTION_SCHEMA,&dialect(),edited.clone(),encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;let value=decode(native);assert!(matches!(value.entries[0].body.as_ref(),ArtifactBody::Document{document_id,..}if document_id==f()["edit"]["value"].as_str().unwrap()));}}
#[test]fn sqlite_snapshot_framework_collection_authored_schema_and_exact_semantic_row_admission(){let codec=codec();let source=fixture();let expected=database();let rows=f()["rowsTotal"].as_u64().unwrap()as usize;assert_eq!(expected.tables.iter().map(|table|table.rows.len()).sum::<usize>(),rows);for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let native=payload(&source,encoding);for maximum in[rows-1,rows]{let limits=SqliteDatabaseLimits{max_rows:maximum,..SqliteDatabaseLimits::default()};let output=(codec.export)(S_COLLECTION_SCHEMA,&dialect(),&native,&mut SqliteSnapshotControl::new(&mut |_|true,limits));let input=(codec.import)(S_COLLECTION_SCHEMA,&dialect(),expected.clone(),encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits));assert_eq!(output.is_ok(),maximum==rows);assert_eq!(input.is_ok(),maximum==rows);}let limits=SqliteDatabaseLimits{max_schema_bytes:SQL.len()-1,..SqliteDatabaseLimits::default()};assert!((codec.export)(S_COLLECTION_SCHEMA,&dialect(),&native,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());assert!((codec.import)(S_COLLECTION_SCHEMA,&dialect(),expected.clone(),encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());}}
#[test]fn sqlite_snapshot_framework_collection_all_owned_phases_cancel_and_wrong_dialect_refuses(){let codec=codec();let source=fixture();let expected=database();for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let native=payload(&source,encoding);for phase in[SqliteSnapshotPhase::DecodeNative,SqliteSnapshotPhase::ProjectSnapshot]{let mut seen=false;let result=(codec.export)(S_COLLECTION_SCHEMA,&dialect(),&native,&mut SqliteSnapshotControl::new(&mut |p|{if p.phase==phase{seen=true;false}else{true}},SqliteDatabaseLimits::default()));assert!(result.is_err()&&seen);}for phase in[SqliteSnapshotPhase::ReconstructSnapshot,SqliteSnapshotPhase::EncodeNative]{let mut seen=false;let result=(codec.import)(S_COLLECTION_SCHEMA,&dialect(),expected.clone(),encoding,&mut SqliteSnapshotControl::new(&mut |p|{if p.phase==phase{seen=true;false}else{true}},SqliteDatabaseLimits::default()));assert!(result.is_err()&&seen);}let wrong=semio_framework_artifact_reference::ArtifactDialect{subset:"invented".into(),..dialect()};assert!((codec.export)(S_COLLECTION_SCHEMA,&wrong,&native,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).is_err());}}

#[test]
fn sqlite_snapshot_framework_collection_native_input_retained_materialization_uses_same_caller_ledger(){
 let corpus=f();let sample=&corpus["reconstructionBudget"];let ratio=&corpus["nativeRetention"];assert_eq!(ratio["ledger"],"native-backing-allocation");let mut source=fixture();source.name=sample["textUnit"].as_str().unwrap().repeat(sample["repeat"].as_u64().unwrap()as usize);
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let payload=payload(&source,encoding);let limits=SqliteDatabaseLimits::default();let mut progress=|_|true;let mut control=SqliteSnapshotControl::new(&mut progress,limits);let first=<CollectionSnapshot as store::ArtifactSqliteSnapshot>::decode_sqlite_snapshot_native(&payload,&mut control).unwrap();assert_eq!(first,source);let charged=limits.max_allocation_bytes-control.allocation_remaining_bytes();assert!(charged>0,"native parser and retained typed fields must publish their actual cumulative ownership");
  let ceiling=charged.checked_mul(ratio["ceilingNumerator"].as_u64().unwrap()as usize).unwrap()/ratio["ceilingDenominator"].as_u64().unwrap()as usize;let limits=SqliteDatabaseLimits{max_allocation_bytes:ceiling,..limits};let mut progress=|_|true;let mut control=SqliteSnapshotControl::new(&mut progress,limits);let retained=<CollectionSnapshot as store::ArtifactSqliteSnapshot>::decode_sqlite_snapshot_native(&payload,&mut control).unwrap();assert_eq!(retained,source);assert!(<CollectionSnapshot as store::ArtifactSqliteSnapshot>::decode_sqlite_snapshot_native(&payload,&mut control).is_err(),"second retained native input must not reset the same caller ledger");
 }
}

#[path="../../../../../../🧪️tests/🪶️sqlite/📏️preflight/🦀️.rs"]
mod public_preflight;
#[test]
fn sqlite_snapshot_framework_collection_public_borrowed_native_preflight_admits_actual_owner(){
 let facet:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/📏️preflight/🔣️.json")).unwrap();
 let mut short=fixture();short.name=facet["textUnit"].as_str().unwrap().into();
 let mut long=short.clone();long.name=facet["textUnit"].as_str().unwrap().repeat(usize::try_from(facet["repeat"].as_u64().unwrap()).unwrap());
 public_preflight::verify(&short,&long,SQL,usize::try_from(facet["rows"].as_u64().unwrap()).unwrap(),long.name.len(),usize::try_from(facet["cancelAt"].as_u64().unwrap()).unwrap(),|operation|crate::test_allocation::observe(operation));
}


#[test]
fn sqlite_snapshot_framework_collection_original_borrowed_variants_match_serde_owner(){
    use semio_framework_dsl_record::{DslVariants,native_encoding::FieldProjectionView as V};
    let schema:serde_json::Value=serde_json::from_str(include_str!("../../../../\u{1f9eb}\u{fe0f}fixtures/\u{1f3ed}\u{fe0f}native-schema/\u{1f523}\u{fe0f}.json")).unwrap();
    let neutral=f();let source=fixture();let mut sizes=neutral["blobSizes"].as_array().unwrap().iter();let mut witnessed=std::collections::BTreeSet::new();
    let authored_entries=neutral["nativeSnapshot"]["entries"].as_array().unwrap();assert_eq!(source.entries.len(),authored_entries.len());
    for(entry,authored)in source.entries.iter().zip(authored_entries){
        let body=entry.body.as_ref();let(keyword,ordinal,producer)=body.projected_variant_identity();witnessed.insert(keyword);
        assert_eq!(schema["variants"][ordinal]["keyword"],keyword);let spec=(producer.ordinary)();let mut expected=authored["body"].clone();
        if keyword=="blob"{expected["blob"]["size"]=serde_json::json!(sizes.next().unwrap().as_str().unwrap().parse::<u64>().unwrap());}
        let third_party=serde_json::to_value(body).unwrap();assert_eq!(third_party,expected);
        let ids=match body.projected_variant_view(&[]).unwrap(){V::Record(ids)=>ids,_=>panic!("borrowed variant is an original Record")};
        assert_eq!(ids,spec.fields.iter().map(|field|field.id).collect::<Vec<_>>());
        let mut projected=Vec::new();
        for index in 0..ids.len(){match body.projected_variant_view(&[index]).unwrap(){
            V::Text(text)=>{let original=match(body,index){(ArtifactBody::Document{schema,..},0)=>schema,(ArtifactBody::Document{document_id,..},1)=>document_id,(ArtifactBody::Blob{blob},0)=>&blob.hash,(ArtifactBody::Blob{blob},2)=>&blob.media_type,_=>panic!("declared borrowed text ordinal")};assert_eq!((text.as_ptr(),text.len()),(original.as_ptr(),original.len()));projected.push(serde_json::json!(text));},
            V::UInt(value)=>projected.push(serde_json::json!(value)),_=>panic!("complete authored scalar variant")
        }}
        let independently_serialized=match keyword{"document"=>serde_json::json!([third_party["schema"],third_party["document_id"]]),"blob"=>serde_json::json!([third_party["blob"]["hash"],third_party["blob"]["size"],third_party["blob"]["mediaType"]]),_=>panic!("closed authored variant")};
        assert_eq!(serde_json::Value::Array(projected),independently_serialized);assert!(body.projected_variant_view(&[ids.len()]).is_err());assert!(body.projected_variant_view(&[0,0]).is_err());assert!(body.projected_variant_key(&[],0).is_err());assert!(body.projected_variant_key(&[0],0).is_err());
    }
    assert_eq!(witnessed,std::collections::BTreeSet::from(["document","blob"]));assert!(sizes.next().is_none());
    eprintln!("[DEBUG] Collection original borrowed ArtifactBody variants=2 exact_scalar_fields=true original_text_pointers=true serde_full_owner=true");
}

fn semantic_role_source(index:usize,text:&str)->CollectionSnapshot{let mut source=fixture();match index{0=>{source.schema=text.into();},1=>{source.name=text.into();},2=>{source.folders[0].id=text.into();},3=>{source.folders[0].parent_id=Some(text.into());},4=>{source.folders[0].name=text.into();},5=>{source.entries[0].id=text.into();},6=>{source.entries[0].folder_id=Some(text.into());},7=>{source.entries[0].name=text.into();},8=>{source.entries[0].kind_id=text.into();},9=>{if let ArtifactBody::Document{schema,..}=source.entries[0].body.as_mut(){*schema=text.into();}else{panic!("original document body required")}},10=>{if let ArtifactBody::Document{document_id,..}=source.entries[0].body.as_mut(){*document_id=text.into();}else{panic!("original document body required")}},11=>{if let ArtifactBody::Blob{blob}=source.entries[1].body.as_mut(){blob.hash=text.into();}else{panic!("original blob body required")}},12=>{if let ArtifactBody::Blob{blob}=source.entries[1].body.as_mut(){blob.media_type=text.into();}else{panic!("original blob body required")}},_=>panic!("closed thirteen authored text roles")}source}

#[test]
fn sqlite_snapshot_framework_collection_complete_native_semantic_text_roles_and_columns(){
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
   let retained=CollectionSnapshot::decode_sqlite_snapshot_native(&input,&mut SqliteSnapshotControl::new(&mut |_|true,defaults)).unwrap();assert_eq!(retained,source);retained.retire_sqlite_snapshot();
   let short=SqliteDatabaseLimits{max_columns:8-1,..defaults};
   assert!(CollectionSnapshot::decode_sqlite_snapshot_native(&input,&mut SqliteSnapshotControl::new(&mut |_|true,short)).is_err(),"{} borrowed complete native columns must refuse before typed materialization",sample["id"]);
   for maximum in[bytes-1,bytes]{
    let limits=SqliteDatabaseLimits{max_value_bytes:maximum,..defaults};
    let output=source.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits));
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
fn sqlite_snapshot_framework_collection_actual_borrowed_wire_matches_public_file_for_every_text_role(){
 use semio_framework_value::NativeEncodeControl;
 let plan:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🛂️semantic/🔣️.json")).unwrap();
 for(index,_)in plan["cases"].as_array().unwrap().iter().enumerate(){
  let source=semantic_role_source(index,plan["text"].as_str().unwrap());
  for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
   let actual=payload(&source,encoding);let length=match actual{store::io::IoPayload::Binary(bytes)=>bytes.len(),store::io::IoPayload::Text(text)=>text.len()};
   let component=match encoding{SnapshotEncoding::Binary=>store::semio_format::Component::Pack,SnapshotEncoding::Text=>store::semio_format::Component::Dsl};
   let prefix=store::semio_format::declared_envelope_prefix_len(CollectionSnapshot::__DSL_ENVELOPE_ID,component,1).unwrap();
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
  <CollectionSnapshot as store::ArtifactSqliteSnapshot>::retire_sqlite_snapshot(source);
 }
}

#[test]
fn sqlite_snapshot_framework_collection_borrowed_semantic_gate_uses_closed_exact_cells_before_typed_construction(){
 use store::ArtifactSqliteSnapshot;
 let plan:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🛂️semantic/🔣️.json")).unwrap();let defaults=SqliteDatabaseLimits::default();
 for(index,sample)in plan["cases"].as_array().unwrap().iter().enumerate(){
  let source=semantic_role_source(index,plan["text"].as_str().unwrap());let bytes=sample["semanticBytes"].as_u64().unwrap()as usize;let rows=plan["rows"].as_u64().unwrap()as usize;let schema=CollectionSnapshot::SQLITE_SCHEMA.len();
  for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let input=payload(&source,encoding);
   for(field,maximum,success)in[("bytes",bytes,true),("bytes",bytes-1,false),("rows",rows,true),("rows",rows-1,false),("columns",8,true),("columns",7,false),("schema",schema,true),("schema",schema-1,false)]{
    let limits=match field{"bytes"=>SqliteDatabaseLimits{max_value_bytes:maximum,..defaults},"rows"=>SqliteDatabaseLimits{max_rows:maximum,..defaults},"columns"=>SqliteDatabaseLimits{max_columns:maximum,..defaults},"schema"=>SqliteDatabaseLimits{max_schema_bytes:maximum,..defaults},_=>unreachable!()};
    let mut callback=|_|true;let mut caller=SqliteSnapshotControl::new(&mut callback,defaults);let admitted=store::decode_sqlite_snapshot_record_native(&input,CollectionSnapshot::__DSL_ENVELOPE_ID,CollectionSnapshot::__dsl_spec_producer(),|record,native|crate::io::sqlite::snapshot::admission::record(record,native,limits),&mut caller);assert_eq!(admitted.is_ok(),success,"{} isolated borrowed {} grant before typed construction",sample["id"],field);
   }
  }source.retire_sqlite_snapshot();
 }
}
