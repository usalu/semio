use crate::args::ParsedArgs;
use crate::catalog::{load_playground_catalog, PlaygroundEntry};
use crate::env_contract::{build_dev_env, DevOptions};
use crate::options::{parse_lock, Lock};
use crate::proc::spawn_inherit;
use std::path::Path;

// #region 🔖️Command
/// 🛝️ Resolves a registered playground variant and starts its Framework OS development session.
pub fn run(root: &Path, args: &ParsedArgs) -> i32 {
    let catalog = load_playground_catalog(root);
    let Some((playground, _rest)) = resolve_playground(&catalog, &args.segments) else {
        eprintln!("[semio dev] unknown plugin/variant {:?} — run `semio catalog` to list playgrounds", args.segments.join(" "));
        return 1;
    };
    let opts = dev_options_from_args(args);
    let env = build_dev_env(&playground.variant, Some(playground), &opts);
    println!("[semio dev] {} via {} on port {}", playground.variant, opts.renderer, env.iter().find(|(key, _)| key == "S_OS_PORT").map(|(_, value)| value.as_str()).unwrap_or("?"));
    spawn_inherit("bun", &["nx", "run", "@semio-tech/framework-os-dev:dev"], root, &env)
}
// #endregion 🔖️Command

// #region 🔖️Resolution
/// 🎯️ Resolves the longest multi-word catalog alias before any trailing command segments.
fn resolve_playground<'a>(catalog: &'a [PlaygroundEntry], segments: &[String]) -> Option<(&'a PlaygroundEntry, Vec<String>)> {
    for length in (1..=segments.len()).rev() {
        let alias = segments[..length].join(" ");
        if let Some(row) = catalog.iter().find(|row| row.variant == alias || row.aliases.iter().any(|candidate| candidate == &alias)) {
            return Some((row, segments[length..].to_vec()));
        }
    }
    None
}

fn dev_options_from_args(args: &ParsedArgs) -> DevOptions {
    DevOptions {
        renderer: args.flag("renderer").unwrap_or("react").to_string(),
        port: args.flag("port").and_then(|port| port.parse().ok()),
        example: args.flag("example").map(parse_lock).unwrap_or(Lock::All),
        language: args.flag("language").map(parse_lock).unwrap_or(Lock::All),
        terminology: args.flag("terminology").map(parse_lock).unwrap_or(Lock::All),
        theme: args.flag("theme").map(parse_lock).unwrap_or(Lock::All),
        appearance: args.flag("appearance").map(parse_lock).unwrap_or(Lock::All),
        skip_plugin_build: args.has_flag("skip-plugin-build"),
        skip_engine_build: args.has_flag("skip-engine-build"),
        skip_wgpu_build: args.has_flag("skip-wgpu-build"),
    }
}
// #endregion 🔖️Resolution

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
