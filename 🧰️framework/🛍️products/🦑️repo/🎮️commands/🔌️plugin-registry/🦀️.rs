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
mod tests {
    use super::*;
    use std::path::PathBuf;

    /// 🧪️ Creates an isolated root for generated-registry command tests.
    fn temp_root(name: &str) -> PathBuf {
        let nanos = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
        let root = std::env::temp_dir().join(format!("semio-plugin-registry-command-{name}-{nanos}"));
        std::fs::create_dir_all(&root).expect("create temp root");
        root
    }

    #[test]
    fn check_reports_missing_generated_files() {
        let root = temp_root("missing");
        let problems = check_generated_plugin_registry(&root);
        assert_eq!(problems.len(), 2);
        assert!(problems.iter().all(|problem| problem.contains("is missing")));
        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn check_reports_invalid_json_and_passes_when_valid() {
        let root = temp_root("invalid-then-valid");
        let output = generated_dir(&root);
        std::fs::create_dir_all(&output).unwrap();
        std::fs::write(output.join("🔣️plugins.json"), "not json").unwrap();
        std::fs::write(output.join("🔣️playgrounds.json"), "not json").unwrap();
        let problems = check_generated_plugin_registry(&root);
        assert_eq!(problems.len(), 2);
        assert!(problems.iter().all(|problem| problem.contains("invalid JSON")));

        std::fs::write(output.join("🔣️plugins.json"), "[]\n").unwrap();
        std::fs::write(output.join("🔣️playgrounds.json"), "[]\n").unwrap();
        assert!(check_generated_plugin_registry(&root).is_empty());
        std::fs::remove_dir_all(&root).ok();
    }
}
// #endregion 🔖️Tests
