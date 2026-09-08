
use super::*;
use std::fs;

#[test]
fn segment_key_strips_emoji_prefix() {
    assert_eq!(segment_key("🌊️flow"), "flow");
    assert_eq!(segment_key("📦️packages"), "packages");
}

#[test]
fn discover_builds_verb_first_level() {
    let tmp = std::env::temp_dir().join(format!("semio-discover-test-{}", std::process::id()));
    let _ = fs::remove_dir_all(&tmp);
    fs::create_dir_all(&tmp).unwrap();
    let pkg = tmp.join("✏️s/🔌️plugins/demo/📦️packages/🦀️rust");
    fs::create_dir_all(&pkg).unwrap();
    fs::write(pkg.join("📋️project.json"), r#"{"name":"@semio-tech/demo","targets":{"test":{"executor":"nx:run-commands"}}}"#).unwrap();
    let tree = discover(&tmp);
    let verbs: Vec<&str> = tree.children.iter().map(|c| c.key.as_str()).collect();
    assert!(verbs.contains(&"test"));
}
