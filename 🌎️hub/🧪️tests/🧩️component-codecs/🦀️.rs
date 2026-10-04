//! 🧩️ Actual Hub-owned component codec integration across caller selected runtimes.
use semio_framework_plugin_host::{GuestRuntime,GuestRuntimes,OwnedRuntime,WasmtimeRuntime,SharedEngineConfig,PackageRef,PackageId,PackageHash};
use semio_framework::kernel::Budget;
const MINTED_DOCUMENT_ID:&str="artifact-0123456789abcdef0123456789abcdef";
fn repo_root()->std::path::PathBuf {std::path::Path::new(env!("CARGO_MANIFEST_DIR")).ancestors().find(|p|p.join("nx.json").is_file()).unwrap().to_path_buf()}
fn package_ref(package:&str,bytes:&[u8])->PackageRef {PackageRef {package:PackageId(package.into()),hash:PackageHash(*semio_framework_hash::hash(bytes).as_bytes())}}
fn codec_budget()->Budget {Budget {fuel:4_000_000_000,deadline_ms:30_000,max_effects:0,max_patch_bytes:0,max_frames:0}}
fn jit_budget()->Budget {Budget {deadline_ms:120_000,..codec_budget()}}
#[derive(Clone,Copy,Debug)]
enum CodecSweepRuntime {Owned,Jit}
#[derive(Clone,Copy)]
enum GenesisMirrorText {Structured,CarrierRaw}
async fn codec_sweep_one_component(package: &str, file_name: &str, kind: &str, schema: &str, which: CodecSweepRuntime, text: GenesisMirrorText) -> Result<(), String> {
    let path=repo_root().join(file_name);
    if !path.is_file() {return Err(format!("required component producer output is absent: {}",path.display()))}
    let bytes = std::fs::read(&path).map_err(|error| format!("read {}: {error}", path.display()))?;
    let runtime = match which {
        CodecSweepRuntime::Owned => GuestRuntimes::Owned(OwnedRuntime::new()),
        CodecSweepRuntime::Jit => GuestRuntimes::Wasmtime(WasmtimeRuntime::new(SharedEngineConfig::default()).await.map_err(|error| format!("engine: {error:?}"))?),
    };
    let budget = match which {
        CodecSweepRuntime::Owned => codec_budget(),
        CodecSweepRuntime::Jit => jit_budget(),
    };
    let compiled = runtime.compile(&package_ref(package, &bytes), &bytes).await.map_err(|error| format!("compile {}: {error:?}", path.display()))?;
    let hash = runtime.codec_pack_schema_hash(&compiled, schema, &budget).await.map_err(|error| format!("codec.pack-schema-hash({schema}): {error:?}"))?;
    if hash == [0; 32] {
        return Err(format!("codec.pack-schema-hash({schema}) answered the zero fingerprint"));
    }
    let pair = runtime.codec_genesis(&compiled, schema, MINTED_DOCUMENT_ID, &budget).await.map_err(|error| format!("codec.genesis({schema}): {error:?}"))?;
    if pair.pack.is_empty() || pair.spr.is_empty() {
        return Err(format!("codec.genesis({schema}) produced an empty pair"));
    }
    let by_kind = runtime.codec_genesis(&compiled, kind, MINTED_DOCUMENT_ID, &budget).await.map_err(|error| format!("codec.genesis({kind}): {error:?}"))?;
    if by_kind != pair {
        return Err(format!("codec.genesis({kind}) and codec.genesis({schema}) selected different apps"));
    }
    let mirror = runtime.codec_print_mirror(&compiled, schema, &pair.pack, &pair.spr, &budget).await.map_err(|error| format!("codec.print-mirror({schema}): {error:?}"))?;
    match text {
        GenesisMirrorText::Structured if mirror.dsl.is_empty() => return Err(format!("codec.print-mirror({schema}) printed no dsl at all")),
        GenesisMirrorText::CarrierRaw if !mirror.dsl.is_empty() => return Err(format!("codec.print-mirror({schema}) printed {} bytes for an empty carrier document", mirror.dsl.len())),
        _ => {}
    }
    if !mirror.ops.contains(MINTED_DOCUMENT_ID) || !mirror.ops.contains(schema) {
        return Err(format!(
            "codec.print-mirror({schema}) lost the minted identity: its ops log must open on a doc header carrying {MINTED_DOCUMENT_ID} and {schema}, got {} bytes beginning {:?}",
            mirror.ops.len(),
            mirror.ops.chars().take(320).collect::<String>()
        ));
    }
    let applied = runtime.codec_apply_ops(&compiled, schema, &pair.pack, &pair.spr, &[], &budget).await.map_err(|error| format!("codec.apply-ops({schema}): {error:?}"))?;
    if applied.pack.is_empty() || applied.spr.is_empty() {
        return Err(format!("codec.apply-ops({schema}) returned an empty baseline"));
    }
    Ok(())
}


mod quick {
use super::*;
#[tokio::test]
async fn actual_component_codec_exports_preserve_owner_resolver_and_identity() {
    let source=include_str!("🧫️fixtures/🔣️.json");
    let fixture:serde_json::Value=serde_json::from_str(source).unwrap();
    let firstparty:directory::DslValue=semio_framework_pack_json::from_json_str(source, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&firstparty)).unwrap(),fixture);
    let mut failures=Vec::new();
    for row in fixture["components"].as_array().unwrap() {
        let runtime=match row["runtime"].as_str().unwrap() {"owned"=>CodecSweepRuntime::Owned,"wasmtime"=>CodecSweepRuntime::Jit,_=>panic!("unknown authored runtime")};
        let text=match row["text"].as_str().unwrap() {"structured"=>GenesisMirrorText::Structured,"carrier"=>GenesisMirrorText::CarrierRaw,_=>panic!("unknown authored text contract")};
        if let Err(error)=codec_sweep_one_component(row["package"].as_str().unwrap(),row["producerOutput"].as_str().unwrap(),row["artifactKind"].as_str().unwrap(),row["documentSchema"].as_str().unwrap(),runtime,text).await {failures.push(error)}
    }
    assert!(failures.is_empty(),"{}",failures.join("\n"));
}
}
mod exhaustive {
use super::*;
#[tokio::test]
async fn actual_gis_owned_codec_honors_the_callers_finite_fuel_budget() {
    let path=repo_root().join("🌎️hub/🧩️compositions/🌍️gis/📦️packages/🦀️rust/dist/component-release/semio_hub_gis.wasm");
    let bytes=std::fs::read(&path).expect("required actual GIS component producer output");
    let runtime=OwnedRuntime::new();let compiled=runtime.compile(&package_ref("semio:gis",&bytes),&bytes).await.unwrap();
    let pair=runtime.codec_genesis(&compiled,"gis.map",MINTED_DOCUMENT_ID,codec_budget()).await.unwrap();
    assert!(!pair.pack.is_empty()&&!pair.spr.is_empty());
    assert_ne!(runtime.codec_pack_schema_hash(&compiled,"gis.map",codec_budget()).await.unwrap(),[0;32]);
}

}
