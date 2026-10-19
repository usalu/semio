//! 🕸️ CLI: `bun ./📜️script.ts os run <bundle>.studio [--node <id>] [--watch] [--dry] [--param k=v]*
//! [--policy <laissez-faire|normal|vigilant>]` shells out to `cargo run -p semio-framework-os-run
//! --release -- <same args>`. This binary owns argv
//! parsing and studio-bundle plumbing only — all the actual dirty/clean and execution logic lives in
//! the library. Non-destructive (W5 Lane A): every invocation produces/updates a `RunArtifact` under
//! `runs/<RUN_ID>.run.pack|.spr` (sealed on success, sealed `Failed` if the run itself errored) — it
//! never mutates a node's source `artifacts/<artifact_ref>`/`artifacts/<config_ref>` bytes.

// 🔁️ ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W1: a `[[bin]]`
// target is its own separate crate root — it does NOT inherit the `extern crate ... as store;`/
// `as workflow;` aliases the lib's own `🦀️.rs` declares, so this binary needs its own copies
// (this was always true; it was only ever masked because the lib itself failed to compile first,
// so cargo never got far enough to check this file — see the lib's own W1 fix history).
extern crate semio_framework as store;
extern crate semio_framework_artifact_workflow_workflow as workflow;
extern crate semio_framework_os_kernel as protocol;

use semio_framework_process_routing_workspace as workspace_identity;

/// 🏠️ Admits only a declared package workspace with regular build authority.
fn is_repository_root(path: &Path) -> Result<bool, String> {
    workspace_identity::is_workspace_root(path, |source| semio_framework_pack_json::parse(source, semio_framework_pack_json::JsonMemberPolicy::Reject).map(|value| matches!(value.get("workspaces"), Some(semio_framework_pack_json::Value::Array(_) | semio_framework_pack_json::Value::Object(_)))).map_err(|error| error.to_string()))
}

use semio_framework_os_run::{plan, register_builtin_converters, MediaCache, NodeDocumentLoadProgress, RunSink, SpaceBundle, SpaceRunner, WasmtimeNodeHost};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use store::{Media, MediaFingerprint};

//#region 🔖️FileMediaCache
/// 💾️ Disk-backed run-only `MediaCache` under `<space>/cache/media/<fingerprint>.json`.
struct FileMediaCache {
    root: PathBuf,
}

impl FileMediaCache {
    fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    fn entry_path(&self, fingerprint: &MediaFingerprint) -> PathBuf {
        self.root.join(format!("{}.json", fingerprint.0))
    }
}

impl MediaCache for FileMediaCache {
    fn get(&self, fingerprint: &MediaFingerprint) -> Option<Media> {
        let text = std::fs::read_to_string(self.entry_path(fingerprint)).ok()?;
        semio_framework_pack_json::from_json_str(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).ok()
    }

    fn put(&mut self, fingerprint: &MediaFingerprint, media: &Media) {
        if std::fs::create_dir_all(&self.root).is_err() {
            return;
        }
        let _ = std::fs::write(self.entry_path(fingerprint), semio_framework_pack_json::to_json_string(media));
    }
}
//#endregion 🔖️FileMediaCache

//#region 🔖️PluginArtifacts
// 🤖️ The registry generate target owns the selected-mode artifact lookup and plugin roster.
use protocol::native_catalog::{configured_native_catalog_v1,plugin_component_directory};

/// 🧭️ Locates the repo root: `SEMIO_REPO_ROOT` (already plumbed by `📜️script.ts`'s `os run` region)
/// when present and valid, else walks up from this crate's own compiled-in `CARGO_MANIFEST_DIR`
/// looking for paired build and package workspace markers — so a direct `cargo run -p semio-framework-os-run` (bypassing the script,
/// from an arbitrary working directory) still resolves correctly.
fn find_repo_root() -> Result<PathBuf, String> {
    if let Ok(value) = std::env::var("SEMIO_REPO_ROOT") {
        let candidate = PathBuf::from(value);
        if is_repository_root(&candidate)? {
            return Ok(candidate);
        }
    }
    let mut candidate = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    loop {
        if is_repository_root(&candidate)? {
            return Ok(candidate);
        }
        if !candidate.pop() {
            return Err("could not locate the repo root (no declared package workspace with build authority above SEMIO_REPO_ROOT or CARGO_MANIFEST_DIR)".to_string());
        }
    }
}

/// 🗺️ Resolves each plugin component using the exact build mode tracked by rustc.
fn resolve_plugin_paths(repo_root: &Path, plugin_ids: impl Iterator<Item = String>) -> Result<HashMap<String, PathBuf>, String> {
    let component_directory = plugin_component_directory(env!("SEMIO_BUILD_MODE")).map_err(str::to_owned)?;
    let catalog=configured_native_catalog_v1()?;
    let artifact_by_plugin: HashMap<&str, (&str, &str)> = catalog.plugins.iter().map(|row| (row.plugin_id.as_str(), (row.crate_path.as_str(), row.wasm_out.as_str()))).collect();
    let mut resolved = HashMap::new();
    for plugin_id in plugin_ids.collect::<BTreeSet<_>>() {
        let (crate_path, wasm_out) = artifact_by_plugin.get(plugin_id.as_str()).ok_or_else(|| format!("plugin `{plugin_id}` is not in the plugin registry — regenerate it: `bun repo run @semio-tech/plugin-registry:generate`"))?;
        let path = repo_root.join(crate_path).join(component_directory).join(wasm_out);
        if !path.is_file() {
            return Err(format!("plugin `{plugin_id}`'s shipped component is missing at {}; build it: `bun repo run <its crate project>:describe`", path.display()));
        }
        resolved.insert(plugin_id, path);
    }
    Ok(resolved)
}

/// 🪪️ One row of the registry's `🔣️plugins.json` (`bun repo run @semio-tech/plugin-registry:generate`
/// output) — only the two fields `resolve_descriptor_paths` needs; every other field (capabilities,
/// contributes, …) is ignored by ordinary serde (no `deny_unknown_fields` here).
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct RegistryPluginEntry {
    plugin_id: String,
    crate_path: String,
}

/// ✅️ R1-native-manifest: maps every registered plugin/extension id to its committed
/// `🛂️.descriptor.semio` path — `<cratePath>/../../🛂️.descriptor.semio` (`cratePath` ends in
/// `📦️packages/🦀️rust`; the descriptor sits two levels up, at the plugin crate's own root — see
/// `describe_component`'s doc, `🔌️plugin/🖨️describe/📦️packages/🦀️rust/🦀️.rs`). Reads the SAME
/// generated `🔣️plugins.json` `resolve_plugin_paths` above reads `🦀️artifacts.rs` from (both
/// produced by the one `plugin-registry:generate` build target); a plugin id absent here, or whose
/// file does not exist on disk, simply has no committed descriptor yet — `WasmtimeNodeHost` treats
/// that as an honest per-plugin load failure, not a hard error at startup.
fn resolve_descriptor_paths(repo_root: &Path) -> Result<HashMap<String, PathBuf>, String> {
    let entries=configured_native_catalog_v1()?.plugins;
    Ok(entries
        .into_iter()
        .filter_map(|entry| {
            let crate_root = repo_root.join(&entry.crate_path);
            let crate_root = crate_root.parent()?.parent()?;
            Some((entry.plugin_id, crate_root.join("🛂️.descriptor.semio")))
        })
        .collect())
}
//#endregion 🔖️PluginArtifacts

/// 🆔️ Single canonical run slot per bundle for this wave — see `SpaceBundle`'s own doc comment on
/// `runs/<run id>.run.pack`. A real multi-run history/listing is later-wave `space::DraftCatalog`
/// integration work; every invocation of this CLI updates/overwrites the SAME `RunArtifact`.
const RUN_ID: &str = "default";

struct Args {
    host_driver_policy: PathBuf,
    bundle: PathBuf,
    dry: bool,
    watch: bool,
    only_node: Option<String>,
    /// 🎛️ `--param <parameter_id>=<value>` (repeatable) — resolved into `semio_framework_artifact_workflow_run::RunParameterValue`s
    /// applied as a config-fingerprint overlay on bound node configs before dirty-checking (see
    /// `semio_framework_os_run::node_parameter_overlay_bytes`'s doc for why this affects the
    /// fingerprint rather than the raw config bytes this crate sends to the app).
    params: Vec<(String, String)>,
    /// ⚖️ `--policy <laissez-faire|normal|vigilant>` — this run's local/authority `MergePolicy`
    /// (contract-freeze.md §C3), sent to every node instance at handshake time (see
    /// `SpaceRunner::compute_node`). Defaults to `Normal`.
    policy: protocol::MergePolicy,
}

/// ⚖️ Parses `--policy`'s value — the three `MergePolicy` variants, kebab-cased.
fn parse_merge_policy(raw: &str) -> Result<protocol::MergePolicy, String> {
    match raw {
        "laissez-faire" => Ok(protocol::MergePolicy::LaissezFaire),
        "normal" => Ok(protocol::MergePolicy::Normal),
        "vigilant" => Ok(protocol::MergePolicy::Vigilant),
        other => Err(format!("--policy `{other}` must be one of: laissez-faire, normal, vigilant")),
    }
}

fn parse_args() -> Result<Args, String> {
    let mut host_driver_policy: Option<PathBuf> = None;
    let mut bundle: Option<PathBuf> = None;
    let mut dry = false;
    let mut watch = false;
    let mut only_node: Option<String> = None;
    let mut params: Vec<(String, String)> = Vec::new();
    let mut policy = protocol::MergePolicy::default();
    let mut argv = std::env::args().skip(1);
    while let Some(arg) = argv.next() {
        match arg.as_str() {
            "--host-driver-policy" => host_driver_policy = Some(PathBuf::from(argv.next().ok_or("--host-driver-policy requires an original full-policy JSON path")?)),
            "--dry" => dry = true,
            "--watch" => watch = true,
            "--node" => only_node = Some(argv.next().ok_or("--node requires a value")?),
            "--param" => {
                let raw = argv.next().ok_or("--param requires a value of the form <parameter_id>=<value>")?;
                let (parameter_id, value) = raw.split_once('=').ok_or_else(|| format!("--param `{raw}` must be of the form <parameter_id>=<value>"))?;
                params.push((parameter_id.to_string(), value.to_string()));
            }
            "--policy" => {
                let raw = argv.next().ok_or("--policy requires a value of laissez-faire, normal, or vigilant")?;
                policy = parse_merge_policy(&raw)?;
            }
            other if !other.starts_with("--") => bundle = Some(PathBuf::from(other)),
            other => return Err(format!("unknown flag {other}")),
        }
    }
    let bundle = bundle.ok_or_else(|| "usage: os run <bundle>.studio [--node <id>] [--watch] [--dry] [--param <parameter_id>=<value>]* [--policy <laissez-faire|normal|vigilant>]".to_string())?;
    let direct_identity=semio_framework_plugin_host::shard::native_identity_issuer(identity_policy,move |_|{let original=direct_cancel.clone();Box::new(move |_|!original.is_cancelled_now())});
    let host_driver_policy = host_driver_policy.ok_or("--host-driver-policy is required")?;
    Ok(Args { host_driver_policy, bundle, dry, watch, only_node, params, policy })
}

fn main() {
    let args = match parse_args() {
        Ok(args) => args,
        Err(message) => {
            eprintln!("[os run] {message}");
            std::process::exit(1);
        }
    };
    if args.watch {
        eprintln!("[os run] --watch is not implemented yet — re-run the command instead for now (see HEADLESS-MEDIA-CONTRACT follow-up)");
        std::process::exit(1);
    }
    if let Err(error) = run(&args) {
        eprintln!("[os run] {error}");
        std::process::exit(1);
    }
}

/// 🔒️ Replays `sink.mutations` through a fresh `store::ArtifactStore` (the same "build an envelope,
/// `Apply`, `snapshot_pack`" pattern every other document in this codebase persists through) and
/// writes the resulting pack+spr to `runs/<RUN_ID>.run.pack|.spr`, plus every node document/config
/// `sink` accumulated — the ONLY two places this CLI ever writes bytes for a run.
async fn persist_run(bundle: &SpaceBundle, sink: &RunSink) -> Result<(), Box<dyn std::error::Error>> {
    let envelope = protocol::os_store::document_materialize::create_document_envelope::<semio_framework_artifact_workflow_run::RunArtifact, semio_framework_artifact_workflow_run::RunMutation>(semio_framework_artifact_workflow_run::S_RUN_SCHEMA, RUN_ID, semio_framework_artifact_workflow_run::empty_run_document().await, None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut document_store = protocol::ArtifactStore::new::<::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication>(envelope, protocol::os_spr::ActorId(protocol::os_spr::LOCAL_ACTOR_ID.into())).await.map_err(|error| error.to_string())?;
    protocol::install_unscheduled_catalog(&mut document_store, protocol::os_store::document_store_owners::funded_bounded_artifact_store_owners::<semio_framework_artifact_workflow_run::RunArtifact, semio_framework_artifact_workflow_run::RunMutation>()).map_err(|error| error.to_string())?;
    let applied = async {
        if !sink.mutations.is_empty() {
            let mut observer = |_: semio_framework_value::native_encoding::NativeEncodeProgress| true;
            let mut identity = protocol::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1usize << 24, &mut observer).map_err(|error| error.to_string())?;
            document_store.dispatch(protocol::os_store::store_schema::ArtifactCommand::Apply { mutations: sink.mutations.clone(), transaction: None }, &mut identity).await.map_err(|error| error.to_string())?;
            drop(identity.pause().map_err(|error| error.to_string())?);
        }
        document_store.snapshot_pack().await.map_err(|error| error.to_string())
    }.await;
    let closed = document_store.close_owned_unscheduled().map_err(|error| error.to_string());
    let snapshot = applied?;
    closed?;
    bundle.write_run_document(RUN_ID, &snapshot.pack, &snapshot.spr)?;
    bundle.write_run_nodes(RUN_ID, sink)?;
    Ok(())
}

/// 🚫️async: E5 executor bridge — this binary's ONE thread-root bridge (R2: at most one per crate).
/// MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (packet `run-kernel-wiring`): drives `run_async`'s whole
/// body — not just `SpaceRunner::run` any more — because `WasmtimeNodeHost::new` (and `open`, called
/// deep inside `SpaceRunner::run`) are now `async fn` themselves, building/driving the real
/// `NativeKernelRuntime` (`semio-framework-os`'s `🎠️activation/🦀️.rs`) instead of the old ad hoc actor-id
/// minting. Uses `semio_framework_async::block_on` — the canonical bridge — rather than
/// `semio_framework_async::block_on` (what this file used before this packet): a single genuine
/// thread-root bridge should be the one this repo ships, not an incidental external crate; this
/// crate's own tests (and `SpaceRunner`'s internal test module) keep `semio_framework_async::block_on`
/// as their own SEPARATE, `#[cfg(test)]`-sanctioned bridge (R4 clause 5), so nothing there changes.
fn run(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    semio_framework_async::block_on(run_async(args))
}

async fn run_async(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    register_builtin_converters();
    let bundle = SpaceBundle::open(&args.bundle);
    let (space_pack, space_spr) = bundle.read_space_document()?;
    let parsed: protocol::os_store::text_format::ParsedDocumentText<workflow::WorkflowSnapshot, workflow::WorkflowMutation> = protocol::os_store::text_format::parse_document_pack(&space_pack, &space_spr, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.map_err(|error| error.to_string())?;
    let snapshot = parsed.snapshot;

    let mut graph = snapshot.graph.clone();
    if let Some(node_id) = &args.only_node {
        graph.nodes.retain(|node| &node.id == node_id);
        graph.edges.retain(|edge| &edge.source_node_id == node_id || &edge.target_node_id == node_id);
    }

    let mut documents: BTreeMap<String, (Vec<u8>, Vec<u8>)> = BTreeMap::new();
    let mut configs: BTreeMap<String, (Vec<u8>, Vec<u8>)> = BTreeMap::new();
    for node in &graph.nodes {
        documents.insert(node.artifact_ref.clone(), bundle.read_artifact(&node.artifact_ref)?);
        configs.insert(node.config_ref.clone(), bundle.read_config(&node.config_ref)?);
    }

    let parameter_values: Vec<semio_framework_artifact_workflow_run::RunParameterValue> = args.params.iter().map(|(parameter_id, value)| semio_framework_artifact_workflow_run::RunParameterValue { parameter_id: parameter_id.clone(), value: value.clone() }).collect();

    let prior_node_records: BTreeMap<String, semio_framework_artifact_workflow_run::RunNodeRecord> = {
        let (run_pack, run_spr) = bundle.read_run_document(RUN_ID)?;
        if run_pack.is_empty() {
            BTreeMap::new()
        } else {
            let parsed: protocol::os_store::text_format::ParsedDocumentText<semio_framework_artifact_workflow_run::RunArtifact, semio_framework_artifact_workflow_run::RunMutation> = protocol::os_store::text_format::parse_document_pack(&run_pack, &run_spr, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.map_err(|error| error.to_string())?;
            parsed.snapshot.node_records.into_iter().map(|record| (record.node_id.clone(), record)).collect()
        }
    };

    if args.dry {
        let report = plan(&graph, &documents, &configs, &parameter_values, &snapshot.parameter_bindings, &prior_node_records).await?;
        println!("recompute: {:?}", report.recomputed);
        println!("clean:     {:?}", report.clean);
        return Ok(());
    }

    let relevant_plugin_ids: BTreeSet<String> = graph.nodes.iter().map(|node| node.plugin_id.clone()).collect();

    let repo_root = find_repo_root()?;
    let plugin_paths = resolve_plugin_paths(&repo_root, relevant_plugin_ids.into_iter())?;
    let descriptor_paths = resolve_descriptor_paths(&repo_root)?;
    let blob_store = Arc::new(bundle.blob_store());
    let identity_policy=semio_framework_pack_json::from_json_str(include_str!("../../🔌️plugin/🖥️host/🧵️shard/🪪️identity/⚙️configuration/🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error|error.to_string())?;
    let original_cancel=semio_framework_async::CancelToken::root_now();
    let identity_cancel=original_cancel.clone();
    let direct_cancel=original_cancel.clone();
    let identity=semio_framework_plugin_host::shard::native_identity_issuer(identity_policy,move |_|{let original=identity_cancel.clone();Box::new(move |_|!original.is_cancelled_now())});
    let host_driver_policy = semio_framework_pack_json::from_json_str(&std::fs::read_to_string(&args.host_driver_policy)?, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error|error.to_string())?;
    let host = WasmtimeNodeHost::new(plugin_paths, descriptor_paths, Arc::clone(&blob_store),identity,host_driver_policy,semio_framework_job::default_now_us,original_cancel.clone(),direct_identity).await?;
    let mut runner = SpaceRunner::new(host, blob_store, args.policy,original_cancel).with_document_load_progress(document_load_reporter());
    let mut cache = FileMediaCache::new(bundle.media_cache_dir());

    let mut sink = RunSink::new(semio_framework_artifact_workflow_run::empty_run_document().await);
    sink.record(semio_framework_artifact_workflow_run::RunMutation::StartRun(semio_framework_artifact_workflow_run::StartRun {
        workflow_ref: args.bundle.display().to_string(),
        workflow_checkpoint_id: String::new(),
        input_collection_ref: String::new(),
        input_snapshot_id: String::new(),
        parameter_values: parameter_values.clone(),
        output_collection_ref: String::new(),
        trigger: semio_framework_artifact_workflow_run::RunTrigger::Manual { actor: "cli".into() },
    })).await
    .map_err(|error| error.to_string())?;

    let run_result = runner.run(&graph, &documents, &configs, &parameter_values, &snapshot.parameter_bindings, &prior_node_records, &mut cache, &mut sink).await;
    let (io_router_plugins, io_router_keys) = runner.into_host().io_router_stats().await;
    eprintln!("[os run] io-router: {io_router_plugins} plugins / {io_router_keys} keys");
    eprintln!("[os run] merge-policy: {:?}", args.policy);
    match run_result {
        Ok(report) => {
            println!("recomputed: {:?}", report.recomputed);
            println!("clean:      {:?}", report.clean);
            sink.record(semio_framework_artifact_workflow_run::RunMutation::SealRun(semio_framework_artifact_workflow_run::SealRun { status: semio_framework_artifact_workflow_run::RunStatus::Succeeded })).await.map_err(|error| error.to_string())?;
            persist_run(&bundle, &sink).await?;
            Ok(())
        }
        Err(error) => {
            let _ = sink.record(semio_framework_artifact_workflow_run::RunMutation::AppendRunLog(semio_framework_artifact_workflow_run::AppendRunLog { node_id: String::new(), level: "error".into(), message: error.to_string(), at: protocol::os_store::document_materialize::now_iso() })).await;
            let _ = sink.record(semio_framework_artifact_workflow_run::RunMutation::SealRun(semio_framework_artifact_workflow_run::SealRun { status: semio_framework_artifact_workflow_run::RunStatus::Failed })).await;
            persist_run(&bundle, &sink).await?;
            Err(error.into())
        }
    }
}

/// 📊️ The CLI's document-load progress: one `[os run]` line per node whenever its polled load crosses another tenth, so a
/// long history shows it is moving without printing every poll.
fn document_load_reporter() -> impl FnMut(&NodeDocumentLoadProgress) + Send + 'static {
    let mut reported: Option<(String, u64)> = None;
    move |progress| {
        let tenth = if progress.total == 0 { 0 } else { progress.completed.saturating_mul(10) / progress.total };
        if reported.as_ref().is_none_or(|(node_id, last)| node_id != &progress.node_id || *last != tenth) {
            eprintln!("[os run] {}: loading document {}/{}", progress.node_id, progress.completed, progress.total);
            reported = Some((progress.node_id.clone(), tenth));
        }
    }
}
