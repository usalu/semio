use super::*;
use serde_json::Value;

#[test]
fn explicit_repository_root_uses_the_provided_directory_and_refuses_invalid_paths() {
    let fixture: Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).expect("root cases");
    for row in fixture["prefixCases"].as_array().expect("prefix cases") { assert_eq!(strip_verbatim(Path::new(row["path"].as_str().expect("path"))), PathBuf::from(row["expected"].as_str().expect("expected"))); }
    let parent = PathBuf::from(std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").expect("owned artifacts"));
    assert!(parent.is_absolute() && parent.to_string_lossy().encode_utf16().count() <= 256);
    fs::create_dir_all(&parent).expect("artifact parent");
    let root = (0..128).find_map(|serial| { let root = parent.join(format!("rr-{}-{serial}", std::process::id())); assert!(root.to_string_lossy().encode_utf16().count() <= 256); match fs::create_dir(&root) { Ok(()) => Some(root), Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => None, Err(error) => panic!("fixture: {error}") } }).expect("finite fixture root");
    let before = std::env::current_dir().expect("cwd");
    for directory in ["first/child", "second"] { let path = root.join(directory); assert!(path.to_string_lossy().encode_utf16().count() <= 256); fs::create_dir_all(path).expect("fixture directory"); }
    let file = root.join("first/file.rs");
    assert!(file.to_string_lossy().encode_utf16().count() <= 256);
    fs::write(file, "pub fn fixture() {}\n").expect("fixture file");
    for row in fixture["cases"].as_array().expect("cases") {
        let start = root.join(row["cwd"].as_str().expect("cwd case"));
        let explicit = row["explicit"].as_str().map(|value| match value { "{second}" => root.join("second"), "{long}" => PathBuf::from("x".repeat(300)), value => PathBuf::from(value) });
        let result = select_repo_root(&start, explicit.as_deref());
        if let Some(expected) = row["expected"].as_str() { let expected = if expected == "discovery" { find_repo_root(&start) } else { root.join(expected) }; assert_eq!(result.expect("valid root"), strip_verbatim(&fs::canonicalize(expected).expect("expected root")), "{}", row["name"]); } else { assert!(result.is_err(), "{}", row["name"]); }
        assert_eq!(std::env::current_dir().expect("unchanged cwd"), before);
    }
    fs::remove_dir_all(root).expect("owned fixture cleanup");
}
