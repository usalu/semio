
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
