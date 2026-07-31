//! 🕸️ CLI: `bun ./📜️script.ts os run <bundle>.studio [--node <id>] [--watch] [--dry]` shells out to
//! `cargo run -p semio-framework-os-run --release -- <same args>`. This binary owns argv parsing and
//! studio-bundle plumbing only — all the actual dirty/clean and execution logic lives in the library.

use semio_framework_os_run::{plan, SpaceBundle, SpaceRunner, WasmtimeNodeHost};
use store::{decode_document_pack_bytes, encode_document_pack_bytes};
use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;

struct Args {
    bundle: PathBuf,
    dry: bool,
    watch: bool,
    only_node: Option<String>,
}

fn parse_args() -> Result<Args, String> {
    let mut bundle: Option<PathBuf> = None;
    let mut dry = false;
    let mut watch = false;
    let mut only_node: Option<String> = None;
    let mut argv = std::env::args().skip(1);
    while let Some(arg) = argv.next() {
        match arg.as_str() {
            "--dry" => dry = true,
            "--watch" => watch = true,
            "--node" => only_node = Some(argv.next().ok_or("--node requires a value")?),
            other if !other.starts_with("--") => bundle = Some(PathBuf::from(other)),
            other => return Err(format!("unknown flag {other}")),
        }
    }
    let bundle = bundle.ok_or_else(|| "usage: os run <bundle>.studio [--node <id>] [--watch] [--dry]".to_string())?;
    Ok(Args { bundle, dry, watch, only_node })
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

fn run(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    let bundle = SpaceBundle::open(&args.bundle);
    let (space_pack, space_spr) = bundle.read_space_document()?;
    let parsed: store::ParsedDocumentText<semio_framework_os::OsProjection, semio_framework_os::OsOperation> =
        store::parse_document_pack(&space_pack, &space_spr).map_err(|error| error.to_string())?;
    let projection = parsed.projection;

    let mut documents: BTreeMap<String, Vec<u8>> = BTreeMap::new();
    for instance in &projection.app_instances {
        let (pack, spr) = bundle.read_document(&instance.document.document_id).unwrap_or_default();
        documents.insert(instance.id.clone(), encode_document_pack_bytes(&pack, &spr));
    }

    let mut graph = projection.workflow.clone();
    if let Some(node_id) = &args.only_node {
        graph.nodes.retain(|node| &node.id == node_id);
        graph.edges.retain(|edge| &edge.source_node_id == node_id || &edge.target_node_id == node_id);
    }

    let mut state = bundle.load_run_state()?;

    if args.dry {
        let report = plan(&graph, &documents, &state)?;
        println!("recompute: {:?}", report.recomputed);
        println!("clean:     {:?}", report.clean);
        return Ok(());
    }

    // 🩹️ Resolved from the dev shell's compiled `plugin-modules/<app>/*.wasm` in a follow-up ticket —
    // empty today, so any node that actually needs a real program instantiated fails loudly with a
    // named-app error instead of silently producing nothing.
    let plugin_paths: HashMap<String, PathBuf> = HashMap::new();
    let host = WasmtimeNodeHost::new(plugin_paths);
    let mut runner = SpaceRunner::new(host);
    let mut cache = bundle.media_cache();

    let (documents_out, report) = runner.run(&graph, &projection.app_instances, &documents, &mut state, &mut cache)?;
    println!("recomputed: {:?}", report.recomputed);
    println!("clean:      {:?}", report.clean);

    for (instance_id, document_bytes) in &documents_out {
        if documents.get(instance_id) != Some(document_bytes) {
            let (pack, spr) = decode_document_pack_bytes(document_bytes).map_err(|error| error.to_string())?;
            bundle.write_document(instance_id, &pack, &spr)?;
        }
    }
    bundle.save_run_state(&state)?;
    Ok(())
}
