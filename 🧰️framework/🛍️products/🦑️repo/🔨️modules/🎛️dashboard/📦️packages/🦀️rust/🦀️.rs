//! 🎛️ The semio terminal dashboard: the wizard command tree, the pseudo-terminal windows it runs
//! commands in, the session daemon behind them, and the repo-domain leaves the Rust domain crates
//! answer in process.
//!
//! Every command of the dashboard is one domain sub-folder of `🔨️modules/🎛️dashboard`, pulled in
//! with `#[path]` so the taxonomy tree — not the crate layout — states what the dashboard is made
//! of. This crate owns the `semio` binary: `🚪️entrypoint` hands its argv to [`run`], which
//! dispatches each verb to its command module.
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🔣️.json
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🚪️entrypoint/🦀️.rs

#[path = "../../🔌️plugin-registry/🦀️.rs"]
pub mod plugin_registry;

#[path = "../../🌀️daemon/🦀️.rs"]
pub mod daemon;

#[path = "../../⌨️usage/🦀️.rs"]
pub mod usage;

#[path = "../../📇️playground-catalog/🦀️.rs"]
pub mod playground_catalog;

#[path = "../../🌳️command-tree/🦀️.rs"]
pub mod command_tree;

#[path = "../../⚙️preferences/🦀️.rs"]
pub mod preferences;

#[path = "../../📚️inventory/🦀️.rs"]
pub mod inventory;

#[path = "../../🎮️registry/🦀️.rs"]
pub mod registry;

#[path = "../../🏛️repo-domain/🦀️.rs"]
pub mod repo_domain;

#[path = "../../🧭️cli/🦀️.rs"]
pub mod cli;

#[path = "../../🖥️terminal/🦀️.rs"]
pub mod terminal;

/// ✉️ The framed dashboard transport, owned by the daemon sub-folder.
pub use daemon::ipc;

// #region 🔖️Dispatch
/// 🗂️ The verbs `run` answers itself, in dispatch order. The usage text and the README verb table list every
/// one of them except the internal ones; a test fails when the three drift apart.
pub const NATIVE_VERBS: &[&str] = &["dashboard", "preferences", "repo-view", "daemon", "catalog", "command-tree", "plugin", "commands", "run", "tasks", "logs", "stop", "restart", "kill", "open"];
/// 🔒️ Native verbs that exist for the dashboard's own child processes and are not documented as commands.
pub const INTERNAL_VERBS: &[&str] = &["repo-view"];

/// 🚦️ Runs one `semio` invocation and returns its process exit code.
pub fn run(argv: &[String]) -> i32 {
    let root = semio_framework_repo_workspace::find_repo_root(&std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from(".")));
    if argv.first().is_some_and(|first| matches!(first.as_str(), "--help" | "-h" | "help")) { usage::help(); return 0; }
    let parsed = invocation(argv);
    let rest = argv.get(1..).unwrap_or_default();
    match parsed.verb.as_str() {
        "dashboard" => terminal::run_with(&root, &parsed),
        "preferences" => preferences::run(&root, &parsed),
        "repo-view" => repo_domain::run_action(&root, &parsed),
        "daemon" => daemon::run(&root, &parsed),
        "catalog" => playground_catalog::run(&root, &parsed),
        "command-tree" => command_tree::run(&root, &parsed),
        "plugin" if parsed.segments.first().map(String::as_str) == Some("registry") => plugin_registry::run(&root, parsed.segments.get(1).map_or("generate", String::as_str)),
        "commands" => cli::commands(&root, rest),
        "run" => cli::run(&root, rest),
        "tasks" => cli::tasks(&root, rest),
        "logs" => cli::logs(&root, rest),
        "stop" | "restart" | "kill" => cli::act(&root, &parsed.verb, rest),
        "open" => cli::open(&root, rest),
        _ => {
            eprintln!("[semio] unknown verb {:?}", argv.first().map_or("", String::as_str));
            usage::print();
            2
        }
    }
}

/// 🧭️ Reads argv as one verb invocation; a bare or flag-first invocation is the dashboard itself.
fn invocation(argv: &[String]) -> args::ParsedArgs {
    if argv.first().is_none_or(|argument| argument.starts_with("--")) {
        return args::parse(&std::iter::once("dashboard".into()).chain(argv.iter().cloned()).collect::<Vec<_>>());
    }
    args::parse(argv)
}
// #endregion 🔖️Dispatch

// #region 🔖️Args
pub mod args {
    use std::collections::HashMap;

    /// ✂️ One `semio <verb> [segments…] [--flag [value]]` invocation, split into its parts.
    #[derive(Debug, Clone, PartialEq, Eq, Default)]
    pub struct ParsedArgs {
        pub verb: String,
        pub segments: Vec<String>,
        pub flags: HashMap<String, Option<String>>,
    }

    impl ParsedArgs {
        pub fn flag(&self, name: &str) -> Option<&str> {
            self.flags.get(name).and_then(|v| v.as_deref())
        }

        pub fn has_flag(&self, name: &str) -> bool {
            self.flags.contains_key(name)
        }
    }

    /// 🔪️ Splits raw argv into a verb, positional segments, and `--flag [value]` pairs.
    ///
    /// A flag consumes the next token as its value unless that token is itself a `--flag` or
    /// there is no next token, in which case the flag is boolean (`has_flag` only).
    pub fn parse(argv: &[String]) -> ParsedArgs {
        let mut iter = argv.iter().peekable();
        let verb = iter.next().cloned().unwrap_or_default();
        let mut segments = Vec::new();
        let mut flags = HashMap::new();
        while let Some(tok) = iter.next() {
            if let Some(name) = tok.strip_prefix("--") {
                let value = match iter.peek() {
                    Some(next) if !next.starts_with("--") => iter.next().cloned(),
                    _ => None,
                };
                flags.insert(name.to_string(), value);
            } else {
                segments.push(tok.clone());
            }
        }
        ParsedArgs { verb, segments, flags }
    }
}
// #endregion 🔖️Args

// #region 🔖️Proc
pub mod proc {
    use std::path::Path;
    use std::process::Command;

    /// 🏃️ Spawns `cmd` with inherited stdio, extending (not replacing) the current environment.
    pub fn spawn_inherit(cmd: &str, args: &[&str], cwd: &Path, env: &[(String, String)]) -> i32 {
        let status = Command::new(cmd).args(args).current_dir(cwd).envs(env.iter().cloned()).status();
        match status {
            Ok(s) => s.code().unwrap_or(1),
            Err(e) => {
                let text = crate::terminal::labels::labels(crate::preferences::cli_locale(cwd, &crate::args::ParsedArgs::default()));
                eprintln!("{}", text.cli_run_failed.fill(&[("command", cmd), ("error", &e.to_string())]).into_string());
                1
            }
        }
    }
}
// #endregion 🔖️Proc

// #region 🔖️Catalog
/// 🧾️ Reads the plugin/playground registry catalog generated by
/// `framework/plugin/registry`'s `script.ts` (single source of truth for the Cargo.toml scan/parse/emit
/// pipeline — see `26/08/05/REGISTRY-DISCOVERY-CONTRACT-TOLERANCE-AND-RUST-TWIN-COLLAPSE`). This crate
/// only *consumes* the generated JSON; it no longer reimplements the scanner in Rust, since `bun` is a
/// hard repo dependency and a second implementation was pure liability (drifted discovery regexes,
/// double the surface to update per taxonomy-contract change).
pub mod catalog {
    use std::fs;
    use std::path::{Path, PathBuf};

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct Ports {
        pub react: u32,
        pub wgpu: u32,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Default)]
    pub struct PlaygroundEntry {
        pub variant: String,
        pub plugin_id: String,
        pub crate_path: String,
        pub app: Option<String>,
        pub aliases: Vec<String>,
        pub ports: Ports,
        pub examples: Vec<String>,
        /// 🔌️ Crate paths (e.g. `framework/surface/tiled-map/rs`) whose `wasm` build target must run
        /// for this playground variant, read from `engines = […]` on its `[[…playground]]` row.
        pub engines: Vec<String>,
        /// 🗃️ Dev-time asset serving needs for this variant, read from the crate's
        /// `[[package.metadata.semio.assets]]` rows (see [`AssetSpec`]).
        pub assets: Vec<AssetSpec>,
    }

    /// 📎️ One `[[package.metadata.semio.assets]]` row: a dev-time asset-serving need declared by a
    /// plugin crate (tile proxy, static directory, or mesh collection). `app` optionally scopes the
    /// row to one playground variant of a multi-app crate (unset ⇒ every variant of the crate).
    #[derive(Debug, Clone, PartialEq, Eq, Default)]
    pub struct AssetSpec {
        pub kind: String,
        pub route: String,
        pub app: Option<String>,
        pub upstream: Option<String>,
        pub cache: Option<String>,
        pub root: Option<String>,
        pub roots: Vec<String>,
        pub placeholder: Option<String>,
        pub filter_from_examples: bool,
    }

    /// 📁️ Resolves the canonical generated plugin-registry output directory.
    pub(crate) fn generated_dir(root: &Path) -> PathBuf {
        root.join("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated")
    }

    /// 📄️ Raw text of the generated playgrounds catalog (`"[]\n"` if it has never been generated) —
    /// used by `semio catalog --json`, which passes the TS-emitted JSON straight through rather than
    /// re-serializing the parsed `PlaygroundEntry` (single source of truth for the wire shape stays
    /// with the TS emitter).
    pub fn playgrounds_json_text(root: &Path) -> String {
        fs::read_to_string(generated_dir(root).join("🚀️playgrounds.json")).unwrap_or_else(|_| "[]\n".to_string())
    }

    /// 📖️ Reads the committed catalog (empty if it has never been generated).
    pub fn load_playground_catalog(root: &Path) -> Vec<PlaygroundEntry> {
        let path = generated_dir(root).join("🚀️playgrounds.json");
        let Ok(text) = fs::read_to_string(path) else { return Vec::new() };
        let Ok(raw) = serde_json::from_str::<Vec<serde_json::Value>>(&text) else { return Vec::new() };
        raw.into_iter()
            .filter_map(|v| {
                Some(PlaygroundEntry {
                    variant: v.get("variant")?.as_str()?.to_string(),
                    plugin_id: v.get("pluginId")?.as_str()?.to_string(),
                    crate_path: v.get("cratePath")?.as_str()?.to_string(),
                    app: v.get("app").and_then(|a| a.as_str()).map(str::to_string),
                    aliases: v.get("aliases")?.as_array()?.iter().filter_map(|a| a.as_str().map(str::to_string)).collect(),
                    ports: Ports { react: v.get("ports")?.get("react")?.as_u64()? as u32, wgpu: v.get("ports")?.get("wgpu")?.as_u64()? as u32 },
                    examples: v.get("examples").and_then(|e| e.as_array()).map(|a| a.iter().filter_map(|e| e.as_str().map(str::to_string)).collect()).unwrap_or_default(),
                    engines: v.get("engines").and_then(|e| e.as_array()).map(|a| a.iter().filter_map(|e| e.as_str().map(str::to_string)).collect()).unwrap_or_default(),
                    assets: v.get("assets").and_then(|e| e.as_array()).map(|a| a.iter().filter_map(asset_spec_from_json).collect()).unwrap_or_default(),
                })
            })
            .collect()
    }

    fn asset_spec_from_json(v: &serde_json::Value) -> Option<AssetSpec> {
        Some(AssetSpec {
            kind: v.get("kind")?.as_str()?.to_string(),
            route: v.get("route")?.as_str()?.to_string(),
            app: None,
            upstream: v.get("upstream").and_then(|x| x.as_str()).map(str::to_string),
            cache: v.get("cache").and_then(|x| x.as_str()).map(str::to_string),
            root: v.get("root").and_then(|x| x.as_str()).map(str::to_string),
            roots: v.get("roots").and_then(|x| x.as_array()).map(|a| a.iter().filter_map(|e| e.as_str().map(str::to_string)).collect()).unwrap_or_default(),
            placeholder: v.get("placeholder").and_then(|x| x.as_str()).map(str::to_string),
            filter_from_examples: v.get("filterFromExamples").and_then(|x| x.as_bool()).unwrap_or(false),
        })
    }
}
// #endregion 🔖️Catalog

// #region 🔖️Options
pub mod options {
    /// 🔀️ One CLI option pick: switchable at runtime, or locked to one value at boot.
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[derive(Default)]
    pub enum Lock {
        #[default]
        All,
        Individual(String),
    }
}
// #endregion 🔖️Options

// #region 🔖️EnvContract
pub mod env_contract {
    use crate::catalog::PlaygroundEntry;
    use crate::options::Lock;

    #[derive(Debug, Clone, Default)]
    pub struct DevOptions {
        pub renderer: String,
        pub port: Option<u16>,
        pub example: Lock,
        pub language: Lock,
        pub terminology: Lock,
        pub theme: Lock,
        pub appearance: Lock,
    }

    

    /// 🚪️ Resolves the dev-server port: `--port`, else the catalog's port for this renderer, else 6066.
    pub fn resolve_port(playground: Option<&PlaygroundEntry>, renderer: &str, explicit: Option<u16>) -> u16 {
        if let Some(port) = explicit {
            return port;
        }
        match playground {
            Some(row) if renderer == "wgpu" => row.ports.wgpu as u16,
            Some(row) => row.ports.react as u16,
            None => 6066,
        }
    }

    /// 📡️ Builds the env vars a `framework-os-dev` dev session (or its browser) needs to boot,
    /// including the `SEMIO_LOCKED_*` shell locks for any `Individual` option pick.
    pub fn build_dev_env(variant: &str, playground: Option<&PlaygroundEntry>, opts: &DevOptions) -> Vec<(String, String)> {
        let mut env = Vec::new();
        let renderer = if opts.renderer.starts_with("wgpu") { "wgpu" } else { "react" };
        let port = resolve_port(playground, renderer, opts.port);
        env.push(("SEMIO_PLUGIN".to_string(), variant.to_string()));
        env.push(("SEMIO_RENDERER".to_string(), renderer.to_string()));
        env.push(("S_OS_PORT".to_string(), port.to_string()));
        env.push(("VITE_SEMIO_PLUGIN".to_string(), playground.map_or_else(|| variant.to_string(), |p| p.plugin_id.clone())));
        env.push(("VITE_SEMIO_RENDERER".to_string(), renderer.to_string()));
        if let Some(app) = playground.and_then(|p| p.app.as_ref()) {
            env.push(("VITE_SEMIO_APP_ID".to_string(), app.clone()));
            env.push(("SEMIO_APP_ID".to_string(), app.clone()));
        }
        if let Lock::Individual(id) = &opts.example {
            env.push(("PLAYGROUND_LOCKED_EXAMPLE_ID".to_string(), id.clone()));
            env.push(("VITE_SEMIO_LOCKED_EXAMPLE".to_string(), id.clone()));
        }
        if let Lock::Individual(v) = &opts.language {
            env.push(("SEMIO_LOCKED_LOCALE".to_string(), v.clone()));
        }
        if let Lock::Individual(v) = &opts.terminology {
            env.push(("SEMIO_LOCKED_TERMINOLOGY".to_string(), v.clone()));
        }
        if let Lock::Individual(v) = &opts.theme {
            env.push(("SEMIO_LOCKED_THEME".to_string(), v.clone()));
        }
        if let Lock::Individual(v) = &opts.appearance {
            env.push(("SEMIO_LOCKED_APPEARANCE".to_string(), v.clone()));
        }
        env.push(("NX_TASKS_RUNNER_DYNAMIC_OUTPUT".to_string(), "false".to_string()));
        env.extend(crate::registry::RUNNER_ENV.iter().map(|(key, value)| ((*key).to_string(), (*value).to_string())));
        env
    }
}
// #endregion 🔖️EnvContract

// #region 🔖️Tests
#[cfg(test)]
#[path = "../../🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests

