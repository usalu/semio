use crate::args::ParsedArgs;
use crate::catalog::{load_playground_catalog, playgrounds_json_text, PlaygroundEntry};
use std::path::Path;

// #region 🔖️Command
/// 📇️ Lists generated playground registrations as JSON or tabular terminal text.
pub fn run(root: &Path, parsed: &ParsedArgs) -> i32 {
    if parsed.has_flag("json") {
        print!("{}", playgrounds_json_text(root));
        return 0;
    }
    print!("{}", table_text(&load_playground_catalog(root)));
    0
}
// #endregion 🔖️Command

// #region 🔖️Presentation
fn table_text(catalog: &[PlaygroundEntry]) -> String {
    catalog.iter().map(|row| format!("{}\t{}\treact:{}\twgpu:{}\n", row.variant, row.plugin_id, row.ports.react, row.ports.wgpu)).collect()
}
// #endregion 🔖️Presentation

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
