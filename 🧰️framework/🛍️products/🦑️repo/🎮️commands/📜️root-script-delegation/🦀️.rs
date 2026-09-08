use crate::args::ParsedArgs;
use crate::proc::spawn_inherit;
use std::path::Path;

// #region 🔖️Command
/// 📜️ Delegates an otherwise unhandled Semio CLI invocation to the root script.
pub fn run(root: &Path, parsed: &ParsedArgs) -> i32 {
    let forwarded = forwarded_segments(parsed);
    let forwarded_refs = forwarded.iter().map(String::as_str).collect::<Vec<_>>();
    spawn_inherit("bun", &forwarded_refs, root, &[])
}

fn forwarded_segments(parsed: &ParsedArgs) -> Vec<String> {
    let mut forwarded = vec!["./📜️script.ts".to_string(), parsed.verb.clone()];
    forwarded.extend(parsed.segments.clone());
    forwarded
}
// #endregion 🔖️Command

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
