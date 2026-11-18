//! 🦀️ Native twin of `jco transpile`: the same `js-component-bindgen` generator the vendored jco ships as a wasm32 component, run as a native
//! process so a component beyond the 32-bit address space and shadow stack of that build still transpiles, with the option defaults of the jco CLI.
//!
//! See <https://github.com/bytecodealliance/jco> and <https://docs.rs/js-component-bindgen/2.16.0>.

use anyhow::{anyhow, bail, Context, Result};
use js_component_bindgen::{transpile, InstantiationMode, TranspileOpts};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

const WASI_SHIMS: &[(&str, &str)] = &[
    ("wasi:cli/*", "@bytecodealliance/preview2-shim/cli#*"),
    ("wasi:clocks/*", "@bytecodealliance/preview2-shim/clocks#*"),
    ("wasi:filesystem/*", "@bytecodealliance/preview2-shim/filesystem#*"),
    ("wasi:http/*", "@bytecodealliance/preview2-shim/http#*"),
    ("wasi:io/*", "@bytecodealliance/preview2-shim/io#*"),
    ("wasi:random/*", "@bytecodealliance/preview2-shim/random#*"),
    ("wasi:sockets/*", "@bytecodealliance/preview2-shim/sockets#*"),
];

const WASI_P3_VERSIONS: &[&str] = &["0.3.0-rc-2026-03-15", "0.3.0"];

const WASI_P3_SHIMS: &[(&str, &str)] = &[
    ("cli", "@bytecodealliance/preview3-shim/cli#*"),
    ("clocks", "@bytecodealliance/preview3-shim/clocks#*"),
    ("filesystem", "@bytecodealliance/preview3-shim/filesystem#*"),
    ("http", "@bytecodealliance/preview3-shim/http#*"),
    ("random", "@bytecodealliance/preview3-shim/random#*"),
    ("sockets", "@bytecodealliance/preview3-shim/sockets#*"),
];

const NODE_CAPABILITIES: &[(&str, &str)] = &[
    ("jco:node/crypto-kdf@0.1.0", "24.x.x/crypto/kdf/host"),
    ("jco:node/timers@0.1.0", "24.x.x/timers/host"),
    ("jco:node/v8@0.1.0", "24.x.x/v8/host"),
    ("jco:node/tls@0.1.0", "24.x.x/tls/node-host"),
    ("jco:node/dgram@0.1.0", "24.x.x/dgram/host"),
    ("wasi:tls/types@0.2.0-draft", "24.x.x/tls/host"),
    ("jco:node/child-process@0.1.0", "24.x.x/child-process/host"),
    ("jco:node/worker-threads@0.1.0", "24.x.x/worker-threads/host"),
    ("jco:node/cluster@0.1.0", "24.x.x/cluster/host"),
    ("jco:node/trace-events@0.1.0", "24.x.x/trace-events/host"),
    ("jco:node/console@0.1.0", "24.x.x/console/host"),
    ("jco:node/sqlite@0.1.0", "24.x.x/sqlite/host"),
    ("jco:node/dns@0.1.0", "24.x.x/dns/host"),
    ("jco:node/fs@0.1.0", "24.x.x/fs/host"),
    ("jco:node/process@0.1.0", "24.x.x/process/host"),
    ("jco:node/os@0.1.0", "24.x.x/os/host"),
    ("jco:node/zlib@0.1.0", "24.x.x/zlib/host"),
    ("jco:node/tty@0.1.0", "24.x.x/tty/host"),
    ("jco:node/wasi@0.1.0", "24.x.x/wasi/host"),
    ("jco:node/ffi@0.1.0", "26.x.x/ffi/host"),
    ("jco:node/http@0.1.0", "24.x.x/http/host"),
    ("jco:node/inspector@0.1.0", "24.x.x/inspector/host"),
    ("jco:node/http2@0.1.0", "24.x.x/http2/host"),
];

const NODE_CAPABILITY_ROOT: &str = "@bytecodealliance/jco-std/wasi/0.2.x/node/";

#[derive(Default)]
struct Cli {
    component: PathBuf,
    out_dir: PathBuf,
    name: Option<String>,
    maps: Vec<(String, String)>,
    no_typescript: bool,
    tla_compat: bool,
    valid_lifting_optimization: bool,
    instantiation: Option<InstantiationMode>,
    base64_cutoff: usize,
    no_wasi_shim: bool,
    no_nodejs_compat: bool,
    no_namespaced_exports: bool,
    multi_memory: bool,
}

fn parse(args: Vec<String>) -> Result<Cli> {
    let mut cli = Cli { base64_cutoff: 5000, ..Cli::default() };
    let mut iter = args.into_iter().peekable();
    if iter.next().as_deref() != Some("transpile") {
        bail!("usage: semio-native-transpile transpile <component.wasm> -o <dir> [--name N] [--map from=to]... [--instantiation [async|sync]] [--base64-cutoff N] [--no-typescript] [--tla-compat] [--valid-lifting-optimization] [--no-wasi-shim] [--no-nodejs-compat] [--no-namespaced-exports] [--multi-memory]");
    }
    let mut component = None;
    let mut out_dir = None;
    while let Some(arg) = iter.next() {
        let mut value = |flag: &str| iter.next().ok_or_else(|| anyhow!("{flag} needs a value"));
        match arg.as_str() {
            "-o" | "--out-dir" => out_dir = Some(PathBuf::from(value(&arg)?)),
            "--name" => cli.name = Some(value(&arg)?),
            "-b" | "--base64-cutoff" => cli.base64_cutoff = value(&arg)?.parse().context("--base64-cutoff")?,
            "-M" | "--map" => {
                let pair = value(&arg)?;
                let (from, to) = pair.split_once('=').ok_or_else(|| anyhow!("--map needs from=to, got {pair}"))?;
                cli.maps.push((from.to_owned(), to.to_owned()));
            }
            "--no-typescript" => cli.no_typescript = true,
            "--tla-compat" => cli.tla_compat = true,
            "--valid-lifting-optimization" => cli.valid_lifting_optimization = true,
            "--no-wasi-shim" => cli.no_wasi_shim = true,
            "--no-nodejs-compat" => cli.no_nodejs_compat = true,
            "--no-namespaced-exports" => cli.no_namespaced_exports = true,
            "--multi-memory" => cli.multi_memory = true,
            "-I" | "--instantiation" => {
                cli.instantiation = Some(match iter.peek().map(String::as_str) {
                    Some("sync") => { iter.next(); InstantiationMode::Sync }
                    Some("async") => { iter.next(); InstantiationMode::Async }
                    _ => InstantiationMode::Async,
                });
            }
            flag if flag.starts_with('-') => bail!("unsupported option {flag}"),
            _ => component = Some(PathBuf::from(arg)),
        }
    }
    cli.component = component.ok_or_else(|| anyhow!("component path required"))?;
    cli.out_dir = out_dir.ok_or_else(|| anyhow!("-o <dir> required"))?;
    Ok(cli)
}

fn effective_map(cli: &Cli) -> HashMap<String, String> {
    let mut map = HashMap::new();
    if cli.instantiation.is_none() && !cli.no_wasi_shim {
        for (from, to) in WASI_SHIMS {
            map.insert((*from).to_owned(), (*to).to_owned());
        }
        for version in WASI_P3_VERSIONS {
            for (interface, to) in WASI_P3_SHIMS {
                map.insert(format!("wasi:{interface}/*@{version}"), (*to).to_owned());
            }
        }
    }
    if cli.instantiation.is_none() {
        for (from, to) in NODE_CAPABILITIES {
            map.insert((*from).to_owned(), format!("{NODE_CAPABILITY_ROOT}{to}"));
        }
    }
    for (from, to) in &cli.maps {
        map.insert(from.clone(), to.clone());
    }
    map
}

fn run(cli: Cli) -> Result<()> {
    let bytes = std::fs::read(&cli.component).with_context(|| format!("read {}", cli.component.display()))?;
    let name = cli.name.clone().unwrap_or_else(|| {
        cli.component.file_stem().map(|stem| stem.to_string_lossy().into_owned()).unwrap_or_else(|| "component".to_owned())
    });
    let emit_conditional_suspending_wasm = cli.instantiation.is_some();
    let opts = TranspileOpts {
        name,
        no_typescript: cli.no_typescript,
        map: Some(effective_map(&cli)),
        nodejs_compat_disabled: cli.no_nodejs_compat,
        base64_cutoff: cli.base64_cutoff,
        tla_compat: cli.tla_compat,
        valid_lifting_optimization: cli.valid_lifting_optimization,
        no_namespaced_exports: cli.no_namespaced_exports,
        multi_memory: cli.multi_memory,
        instantiation_mode: cli.instantiation,
        emit_conditional_suspending_wasm,
        ..TranspileOpts::default()
    };
    let transpiled = transpile(&bytes, opts)?;
    drop(bytes);
    for (file, contents) in transpiled.files {
        let target: &Path = &cli.out_dir.join(&file);
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(target, contents).with_context(|| format!("write {}", target.display()))?;
    }
    Ok(())
}

fn main() {
    let outcome = std::thread::Builder::new()
        .stack_size(512 * 1024 * 1024)
        .spawn(|| parse(std::env::args().skip(1).collect()).and_then(run))
        .map_err(anyhow::Error::from)
        .and_then(|handle| handle.join().map_err(|_| anyhow!("transpile thread panicked"))?);
    if let Err(error) = outcome {
        eprintln!("semio-native-transpile: {error:#}");
        std::process::exit(1);
    }
}
