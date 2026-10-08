use semio_framework_artifact_reference::io::text::artifact_reference::DialectCoordinateText;
use semio_framework::kernel::Budget;
use semio_framework_plugin_host::{GuestRuntime,GuestRuntimes,OwnedRuntime,WasmtimeRuntime,SharedEngineConfig,PackageRef,PackageId,PackageHash,GuestCallCancellation,sqlite_wire};
use std::path::PathBuf;
fn repo_root()->PathBuf {PathBuf::from(env!("CARGO_MANIFEST_DIR")).ancestors().find(|path|path.join("nx.json").exists()).unwrap().to_path_buf()}
fn plugin_wasm()->PathBuf {
    let path=repo_root().join("🌎️hub/🧩️compositions/🗒️note/📦️packages/🦀️rust/dist/component-dev/semio_hub_note.wasm");
    assert!(path.is_file(),"the actual Note component-dev producer prerequisite is required");path
}
fn package_ref(package:&str,bytes:&[u8])->PackageRef {PackageRef {package:PackageId(package.into()),hash:PackageHash(*semio_framework_hash::hash(bytes).as_bytes())}}
fn jit_budget()->Budget {Budget {fuel:u64::MAX,deadline_ms:120_000,max_effects:64,max_patch_bytes:1<<20,max_frames:64}}
enum CodecSweepRuntime {Owned,Jit}
async fn sqlite_snapshot_note_guest_law(which:CodecSweepRuntime){
    use semio_framework_os_kernel::{ArtifactDsl,ArtifactPack,ArtifactSqliteSnapshot,sqlite_snapshot::{self,SqliteDatabaseLimits,SqliteSnapshotControl,SnapshotEncoding}};
    use semio_framework_value::FromValue;
    let root=repo_root();let source=root.join("✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧫️fixtures/🔣️.json");let snapshot=crate::NoteSnapshot::from_value(serde_json::from_str(&std::fs::read_to_string(source).unwrap()).unwrap()).unwrap();
    let path=plugin_wasm();let bytes=std::fs::read(path).unwrap();let runtime=match which{CodecSweepRuntime::Owned=>GuestRuntimes::Owned(OwnedRuntime::new()),CodecSweepRuntime::Jit=>GuestRuntimes::Wasmtime(WasmtimeRuntime::new(SharedEngineConfig::default()).await.unwrap())};let compiled=runtime.compile(&package_ref("semio:note",&bytes),&bytes).await.unwrap();let budget=jit_budget();let limits=SqliteDatabaseLimits::default();let dialect="s.note.note@1/*";
    let schema=runtime.codec_sqlite_schema(&compiled,dialect,&budget).await.unwrap();assert_eq!(schema,<crate::NoteSnapshot as ArtifactSqliteSnapshot>::SQLITE_SCHEMA);
    for(encoding,payload)in[("binary",snapshot.encode_pack()),("text",snapshot.print_dsl().into_bytes())]{
        let cancellation=GuestCallCancellation::default();let result=runtime.codec_sqlite_export(&compiled,dialect,encoding,&payload,limits,&budget,|_,_|{},&cancellation).await.unwrap();let sqlite_wire::SnapshotFileResult::Done(file)=result else{panic!("actual Note guest rejected a complete native snapshot: {result:?}")};assert!(sqlite_wire::decode_diagnostics(&file.diagnostics).unwrap().iter().all(|value|!matches!(value.severity,semio_framework::Severity::Error|semio_framework::Severity::Fatal)));
        let mut database=sqlite_snapshot::import_sqlite_database(&file.bytes,limits,&mut |_|true).unwrap();let(detected,native_encoding)=semio_framework_os_kernel::io::io_mechanism::take_sqlite_snapshot_metadata(&mut database).unwrap();assert_eq!(detected.to_coordinate(),dialect);assert_eq!(native_encoding,SnapshotEncoding::parse(encoding).unwrap());let restored=crate::NoteSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(restored,snapshot);assert_eq!(database.table("note_block").unwrap().rows.len(),8);
        let result=runtime.codec_sqlite_import(&compiled,dialect,&file.bytes,limits,&budget,|_,_|{},&cancellation).await.unwrap();let sqlite_wire::SnapshotPayloadResult::Done(payload)=result else{panic!("actual Note guest rejected its relational snapshot: {result:?}")};assert_eq!(payload.encoding,encoding);let restored=match encoding{"binary"=>crate::NoteSnapshot::decode_pack(&payload.bytes).unwrap(),_=>crate::NoteSnapshot::parse_dsl(std::str::from_utf8(&payload.bytes).unwrap()).unwrap()};assert_eq!(restored,snapshot);
        let wrong=runtime.codec_sqlite_import(&compiled,"s.note.note@1/unknown",&file.bytes,limits,&budget,|_,_|{},&cancellation).await;assert!(!matches!(wrong,Ok(sqlite_wire::SnapshotPayloadResult::Done(_))));
        let tiny=runtime.codec_sqlite_export(&compiled,dialect,encoding,&payload.bytes,SqliteDatabaseLimits{max_rows:1,..limits},&budget,|_,_|{},&cancellation).await;assert!(!matches!(tiny,Ok(sqlite_wire::SnapshotFileResult::Done(_))));
        let cancellation=GuestCallCancellation::default();cancellation.cancel();assert!(runtime.codec_sqlite_import(&compiled,dialect,&file.bytes,limits,&budget,|_,_|{},&cancellation).await.is_err());
    }
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_note_owned_component_preserves_native_relations(){sqlite_snapshot_note_guest_law(CodecSweepRuntime::Owned).await;}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_note_wasmtime_component_preserves_native_relations(){sqlite_snapshot_note_guest_law(CodecSweepRuntime::Jit).await;}
