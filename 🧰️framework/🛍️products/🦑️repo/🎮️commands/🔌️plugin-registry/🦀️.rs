use crate::catalog::generated_dir;
use crate::proc::spawn_inherit;
use std::path::Path;

// #region 🔖️Command
/// 🔌️ Verifies and regenerates the canonical plugin and playground registry.
pub fn run(root: &Path, subcommand: &str) -> i32 {
    match subcommand {
        "check" => {
            let problems = check_generated_plugin_registry(root);
            if problems.is_empty() {
                println!("plugin registry catalog is fresh.");
                0
            } else {
                for problem in &problems {
                    eprintln!("{problem}");
                }
                1
            }
        }
        _ => spawn_inherit("bun", &["nx", "run", "@semio-tech/plugin-registry:generate"], root, &[]),
    }
}
// #endregion 🔖️Command

// #region 🔖️GeneratedCatalog
fn check_generated_plugin_registry(root: &Path) -> Vec<String> {
    let output = generated_dir(root);
    ["🔣️plugins.json", "🔣️playgrounds.json"]
        .iter()
        .filter_map(|name| match std::fs::read_to_string(output.join(name)) {
            Ok(text) if serde_json::from_str::<serde_json::Value>(&text).is_ok() => None,
            Ok(_) => Some(format!("plugin registry catalog is invalid JSON: generated/{name}")),
            Err(_) => Some(format!("plugin registry catalog is missing: generated/{name} (run `bun nx run @semio-tech/plugin-registry:generate`)")),
        })
        .collect()
}
// #endregion 🔖️GeneratedCatalog

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
