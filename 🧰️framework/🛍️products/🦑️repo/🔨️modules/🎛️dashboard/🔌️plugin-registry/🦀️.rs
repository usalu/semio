//! 🔌️ Verifies and regenerates the canonical plugin and playground registry.

use crate::proc::spawn_inherit;
use dashboard_core::catalog::configured_catalog_root;
use std::path::Path;

// #region 🔖️Command
/// 🔧️ Verifies and regenerates the canonical plugin and playground registry.
pub fn run(root: &Path, subcommand: &str) -> i32 {
    match subcommand {
        "check" => {
            let problems = match configured_catalog_root(){Ok(output)=>check_generated_plugin_registry(output.as_deref()),Err(error)=>vec![error]};
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
        _ => {let project=std::env::var("SEMIO_CATALOG_PUBLISHER_PROJECT").unwrap_or_else(|_|"@semio-tech/plugin-registry".into());if project.is_empty()||project.starts_with("-")||project.contains(char::is_whitespace){eprintln!("Catalog publisher project refused");return 1;}spawn_inherit("bun", &["repo", "run", &format!("{project}:generate")], root, &[])},
    }
}
// #endregion 🔖️Command

// #region 🔖️GeneratedCatalog
fn check_generated_plugin_registry(publication:Option<&Path>) -> Vec<String> {
    let Some(output)=publication else{return Vec::new();};
    ["🔌️plugins.json", "🎠️playgrounds.json"]
        .iter()
        .filter_map(|name| match std::fs::read_to_string(output.join(name)) {
            Ok(text) if serde_json::from_str::<serde_json::Value>(&text).is_ok() => None,
            Ok(_) => Some(format!("plugin registry catalog is invalid JSON: generated/{name}")),
            Err(_) => Some(format!("plugin registry catalog is missing: generated/{name} (run `bun repo run @semio-tech/plugin-registry:generate`)")),
        })
        .collect()
}
// #endregion 🔖️GeneratedCatalog

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
